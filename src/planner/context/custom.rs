use super::{PlanContext, RegistryAccess};
use crate::plan::{
    CustomConstruction, CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
    CustomConstructorRefinement, CustomExpr, CustomFieldAccess, CustomFieldDefinition, CustomType,
    CustomTypeDefinition, CustomTypeParameterId, CustomTypeTemplate, CustomValueShape,
    FunctionShape, FunctionType, ValueShape, ValueType,
};
use crate::planner::error::{
    InvalidCustomTypeReason, InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
};
use ecow::EcoString;
use gleam_core::type_::{
    PRELUDE_MODULE_NAME, PatternConstructor, Type, ValueConstructor, ValueConstructorVariant,
};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::planner) struct ResolvedCustomConstructor {
    constructor: CustomConstructor,
    constructor_count: usize,
    source_shape: CustomValueShape,
}

impl ResolvedCustomConstructor {
    pub(in crate::planner) fn constructor_count(&self) -> usize {
        self.constructor_count
    }

    pub(in crate::planner) fn source_shape(&self) -> &CustomValueShape {
        &self.source_shape
    }

    pub(in crate::planner) fn into_constructor(self) -> CustomConstructor {
        self.constructor
    }
}

enum MatchedTemplateShape<'a> {
    Parameter(usize),
    Tuple {
        templates: &'a [CustomTypeTemplate],
        shapes: &'a [ValueShape],
    },
    List {
        template: &'a CustomTypeTemplate,
        shape: &'a ValueShape,
    },
    Function {
        argument_templates: &'a [CustomTypeTemplate],
        return_template: &'a CustomTypeTemplate,
        shape: &'a FunctionShape,
    },
    Custom {
        templates: &'a [CustomTypeTemplate],
        shapes: &'a [ValueShape],
    },
    External {
        templates: &'a [CustomTypeTemplate],
        shapes: &'a [ValueShape],
    },
    Scalar,
}

fn instantiate_custom_type_template(
    template: &CustomTypeTemplate,
    custom_type: &CustomType,
) -> Result<ValueType, PlanError> {
    let type_ = match template {
        CustomTypeTemplate::Int => ValueType::Int,
        CustomTypeTemplate::Float => ValueType::Float,
        CustomTypeTemplate::String => ValueType::String,
        CustomTypeTemplate::BitArray => ValueType::BitArray,
        CustomTypeTemplate::UtfCodepoint => ValueType::UtfCodepoint,
        CustomTypeTemplate::Bool => ValueType::Bool,
        CustomTypeTemplate::Nil => ValueType::Nil,
        CustomTypeTemplate::Tuple(elements) => ValueType::Tuple(
            elements
                .iter()
                .map(|element| instantiate_custom_type_template(element, custom_type))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        CustomTypeTemplate::List(element) => ValueType::List(Box::new(
            instantiate_custom_type_template(element, custom_type)?,
        )),
        CustomTypeTemplate::Function { arguments, return_ } => {
            ValueType::Function(Box::new(FunctionType::new(
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_type_template(argument, custom_type))
                    .collect::<Result<Vec<_>, _>>()?,
                instantiate_custom_type_template(return_, custom_type)?,
            )))
        }
        CustomTypeTemplate::Custom { name, arguments } => ValueType::Custom(CustomType::new(
            name.clone(),
            arguments
                .iter()
                .map(|argument| instantiate_custom_type_template(argument, custom_type))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        CustomTypeTemplate::External { name, arguments } => {
            ValueType::External(crate::plan::ExternalType::new(
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_type_template(argument, custom_type))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
        CustomTypeTemplate::Parameter(parameter) => {
            let index = validate_template_parameter(
                custom_type,
                *parameter,
                custom_type.arguments().len(),
            )?;
            custom_type.arguments()[index].clone()
        }
    };
    Ok(type_)
}

fn instantiate_custom_shape_template(
    template: &CustomTypeTemplate,
    custom_shape: &CustomValueShape,
) -> Result<ValueShape, PlanError> {
    let shape = match template {
        CustomTypeTemplate::Int => ValueShape::Int,
        CustomTypeTemplate::Float => ValueShape::Float,
        CustomTypeTemplate::String => ValueShape::String,
        CustomTypeTemplate::BitArray => ValueShape::BitArray,
        CustomTypeTemplate::UtfCodepoint => ValueShape::UtfCodepoint,
        CustomTypeTemplate::Bool => ValueShape::Bool,
        CustomTypeTemplate::Nil => ValueShape::Nil,
        CustomTypeTemplate::Tuple(elements) => ValueShape::Tuple(
            elements
                .iter()
                .map(|element| instantiate_custom_shape_template(element, custom_shape))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        CustomTypeTemplate::List(item) => ValueShape::List(Box::new(
            instantiate_custom_shape_template(item, custom_shape)?,
        )),
        CustomTypeTemplate::Function { arguments, return_ } => {
            ValueShape::Function(Box::new(FunctionShape::new(
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_shape_template(argument, custom_shape))
                    .collect::<Result<Vec<_>, _>>()?,
                instantiate_custom_shape_template(return_, custom_shape)?,
            )))
        }
        CustomTypeTemplate::Custom { name, arguments } => {
            ValueShape::Custom(CustomValueShape::new(
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_shape_template(argument, custom_shape))
                    .collect::<Result<Vec<_>, _>>()?,
                CustomConstructorRefinement::Any,
            ))
        }
        CustomTypeTemplate::External { name, arguments } => {
            ValueShape::External(crate::plan::ExternalValueShape::new(
                name.clone(),
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_shape_template(argument, custom_shape))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
        CustomTypeTemplate::Parameter(parameter) => {
            let index = validate_template_parameter(
                custom_shape.type_(),
                *parameter,
                custom_shape.arguments().len(),
            )?;
            custom_shape.arguments()[index].clone()
        }
    };
    Ok(shape)
}

fn collect_parameter_shapes(
    template: &CustomTypeTemplate,
    shape: &ValueShape,
    arguments: &mut [Option<ValueShape>],
    owner: &CustomType,
) -> Result<(), PlanError> {
    match match_template_shape(template, shape, owner)? {
        MatchedTemplateShape::Parameter(parameter) => {
            let index = validate_template_parameter(
                owner,
                CustomTypeParameterId(parameter),
                arguments.len(),
            )?;
            let argument = &mut arguments[index];
            *argument = Some(match argument.take() {
                Some(previous) => merge_parameter_shape(owner, index, previous, shape)?,
                None => shape.clone(),
            });
        }
        MatchedTemplateShape::Tuple { templates, shapes } => {
            for (template, shape) in templates.iter().zip(shapes) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
        }
        MatchedTemplateShape::List { template, shape } => {
            collect_parameter_shapes(template, shape, arguments, owner)?;
        }
        MatchedTemplateShape::Function {
            argument_templates,
            return_template,
            shape,
        } => {
            for (template, shape) in argument_templates.iter().zip(shape.argument_shapes()) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
            collect_parameter_shapes(return_template, shape.return_shape(), arguments, owner)?;
        }
        MatchedTemplateShape::Custom { templates, shapes } => {
            for (template, shape) in templates.iter().zip(shapes) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
        }
        MatchedTemplateShape::External { templates, shapes } => {
            for (template, shape) in templates.iter().zip(shapes) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
        }
        MatchedTemplateShape::Scalar => {}
    }
    Ok(())
}

fn validate_template_parameter(
    type_: &CustomType,
    parameter: CustomTypeParameterId,
    available: usize,
) -> Result<usize, PlanError> {
    if parameter.0 >= available {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::TemplateParameterIndex {
                    index: parameter.0,
                    available,
                }),
            },
        });
    }
    Ok(parameter.0)
}

fn match_template_shape<'a>(
    template: &'a CustomTypeTemplate,
    shape: &'a ValueShape,
    owner: &CustomType,
) -> Result<MatchedTemplateShape<'a>, PlanError> {
    let matched = match (template, shape) {
        (CustomTypeTemplate::Parameter(parameter), _) => {
            Some(MatchedTemplateShape::Parameter(parameter.0))
        }
        (CustomTypeTemplate::Tuple(templates), ValueShape::Tuple(shapes))
            if templates.len() == shapes.len() =>
        {
            Some(MatchedTemplateShape::Tuple { templates, shapes })
        }
        (CustomTypeTemplate::List(template), ValueShape::List(shape)) => {
            Some(MatchedTemplateShape::List { template, shape })
        }
        (CustomTypeTemplate::Function { arguments, return_ }, ValueShape::Function(shape))
            if arguments.len() == shape.argument_shapes().len() =>
        {
            Some(MatchedTemplateShape::Function {
                argument_templates: arguments,
                return_template: return_,
                shape,
            })
        }
        (
            CustomTypeTemplate::Custom {
                name,
                arguments: templates,
            },
            ValueShape::Custom(shape),
        ) if name == shape.type_name() && templates.len() == shape.arguments().len() => {
            Some(MatchedTemplateShape::Custom {
                templates,
                shapes: shape.arguments(),
            })
        }
        (
            CustomTypeTemplate::External {
                name,
                arguments: templates,
            },
            ValueShape::External(shape),
        ) if name == shape.type_name() && templates.len() == shape.arguments().len() => {
            Some(MatchedTemplateShape::External {
                templates,
                shapes: shape.arguments(),
            })
        }
        (CustomTypeTemplate::Int, ValueShape::Int)
        | (CustomTypeTemplate::Float, ValueShape::Float)
        | (CustomTypeTemplate::String, ValueShape::String)
        | (CustomTypeTemplate::BitArray, ValueShape::BitArray)
        | (CustomTypeTemplate::UtfCodepoint, ValueShape::UtfCodepoint)
        | (CustomTypeTemplate::Bool, ValueShape::Bool)
        | (CustomTypeTemplate::Nil, ValueShape::Nil) => Some(MatchedTemplateShape::Scalar),
        _ => None,
    };
    let Some(matched) = matched else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: owner.type_name().package().clone(),
                module: owner.type_name().module().clone(),
                name: owner.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::TemplateShapeMismatch {
                    expected: instantiate_custom_type_template(template, owner)?,
                    actual: shape.value_type(),
                }),
            },
        });
    };
    Ok(matched)
}

fn merge_parameter_shape(
    owner: &CustomType,
    parameter: usize,
    previous: ValueShape,
    actual: &ValueShape,
) -> Result<ValueShape, PlanError> {
    let previous_type = previous.value_type();
    previous
        .merge(actual)
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: owner.type_name().package().clone(),
                module: owner.type_name().module().clone(),
                name: owner.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::ConflictingParameterShape {
                    parameter,
                    previous: previous_type,
                    actual: actual.value_type(),
                }),
            },
        })
}

impl PlanContext<'_> {
    pub(in crate::planner) fn custom_constructor(
        &self,
        constructor: &ValueConstructor,
    ) -> Result<CustomConstructor, PlanError> {
        let ValueConstructorVariant::Record {
            name,
            module,
            variant_index,
            arity,
            ..
        } = &constructor.variant
        else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            });
        };

        if module != PRELUDE_MODULE_NAME {
            let _linked_module = self.resolve_module_reference(module, name)?;
        }
        let constructor = self.custom_constructor_from_type(
            constructor.type_.as_ref(),
            name.clone(),
            module,
            usize::from(*variant_index),
        )?;
        validate_constructor_arity(
            constructor.constructor.type_(),
            constructor.constructor.fields().len(),
            usize::from(*arity),
        )?;
        Ok(constructor.into_constructor())
    }

    pub(in crate::planner) fn module_custom_constructor(
        &self,
        type_: &Type,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
        arity: usize,
    ) -> Result<CustomConstructor, PlanError> {
        let constructor = self.custom_constructor_from_type(type_, name, module, variant_index)?;
        validate_constructor_arity(
            constructor.constructor.type_(),
            constructor.constructor.fields().len(),
            arity,
        )?;
        Ok(constructor.into_constructor())
    }

    pub(in crate::planner) fn custom_construction_shape(
        &self,
        construction: &CustomConstruction,
    ) -> Result<CustomValueShape, PlanError> {
        let constructor = construction.constructor();
        let fields = construction.fields();
        let type_ = constructor.type_();
        let (definition, _) = self.resolve_constructor_metadata(type_, constructor.index())?;
        validate_constructor_arity(type_, definition.fields().len(), fields.len())?;

        let mut arguments = vec![None; type_.arguments().len()];
        for (index, (template, field)) in definition.fields().iter().zip(fields).enumerate() {
            let expected = instantiate_custom_type_template(template.type_(), type_)?;
            validate_field_type(type_, index, &expected, &field.value_type())?;
            collect_parameter_shapes(template.type_(), field.value_shape(), &mut arguments, type_)?;
        }
        let arguments = arguments
            .into_iter()
            .zip(type_.arguments())
            .map(|(shape, type_)| match shape {
                Some(shape) => shape,
                None => ValueShape::from_value_type(type_.clone()),
            })
            .collect();
        Ok(CustomValueShape::new(
            type_.type_name().clone(),
            arguments,
            CustomConstructorRefinement::Exact(constructor.index()),
        ))
    }

    pub(in crate::planner) fn custom_expr_from_construction(
        &self,
        construction: CustomConstruction,
    ) -> Result<CustomExpr, PlanError> {
        let shape = self.custom_construction_shape(&construction)?;
        Ok(CustomExpr::from_construction(shape, construction))
    }

    pub(in crate::planner) fn custom_pattern_constructor(
        &self,
        type_: &Type,
        constructor: &PatternConstructor,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        let source_shape =
            self.custom_constructor_source_shape(type_, &constructor.module, &constructor.name)?;
        let mut constructor = self.custom_constructor_from_parts(
            source_shape.type_().clone(),
            constructor.name.clone(),
            &constructor.module,
            usize::from(constructor.constructor_index),
            field_types,
        )?;
        constructor.source_shape = source_shape;
        Ok(constructor)
    }

    pub(in crate::planner) fn custom_field_access(
        &self,
        source: CustomExpr,
        index: usize,
        label: Option<EcoString>,
        expected: &ValueType,
    ) -> Result<(CustomFieldAccess, ValueShape), PlanError> {
        let custom_type = source.type_();
        let custom_shape = source.shape();
        let type_definition = self.resolve_custom_type_definition(custom_type)?;
        let constructors = match custom_shape.constructor() {
            CustomConstructorRefinement::Exact(index) => vec![resolve_constructor_index(
                custom_type,
                type_definition.constructors(),
                index,
            )?],
            CustomConstructorRefinement::Any => type_definition.constructors().iter().collect(),
        };
        let fields = resolve_shared_fields(custom_type, &constructors, index)?;
        let (first_constructor, first_field) = fields.first;
        validate_field_label(custom_type, index, first_field.label(), label.as_ref())?;
        let mut result_shape =
            instantiate_custom_shape_template(first_field.type_(), custom_shape)?;
        validate_field_type(custom_type, index, expected, &result_shape.value_type())?;
        validate_constructor_templates(first_constructor, custom_type)?;

        for (constructor, field) in fields.rest {
            validate_field_label(custom_type, index, field.label(), label.as_ref())?;
            let field_shape = instantiate_custom_shape_template(field.type_(), custom_shape)?;
            validate_field_type(custom_type, index, expected, &field_shape.value_type())?;
            result_shape = merge_field_shape(custom_type, index, result_shape, &field_shape)?;
            validate_constructor_templates(constructor, custom_type)?;
        }
        Ok((CustomFieldAccess::new(source, index, label), result_shape))
    }

    fn custom_constructor_from_type(
        &self,
        constructor_type: &Type,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        let signature = constructor_type.fn_types();
        let return_type = match &signature {
            Some((_, return_type)) => return_type.as_ref(),
            None => constructor_type,
        };
        let type_ = self
            .custom_constructor_source_shape(return_type, module, &name)?
            .type_()
            .clone();
        let field_types = match signature {
            Some((field_types, _)) => field_types,
            None => Vec::new(),
        };
        let mut type_parameters = self.type_parameters.clone();
        let field_types = field_types
            .into_iter()
            .map(|field| {
                ValueShape::from_gleam_in_with_external(
                    field.as_ref(),
                    &mut type_parameters,
                    &|name| self.registry.is_external_type(name),
                )
                .value_type()
            })
            .collect();

        self.custom_constructor_from_parts(type_, name, module, variant_index, field_types)
    }

    fn custom_constructor_source_shape(
        &self,
        type_: &Type,
        module: &EcoString,
        name: &EcoString,
    ) -> Result<CustomValueShape, PlanError> {
        let mut type_parameters = self.type_parameters.clone();
        let actual =
            ValueShape::from_gleam_in_with_external(type_, &mut type_parameters, &|name| {
                self.registry.is_external_type(name)
            });
        let ValueShape::Custom(shape) = actual else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: EcoString::new(),
                    module: module.clone(),
                    name: name.clone(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: actual.value_type(),
                    }),
                },
            });
        };
        Ok(shape)
    }

    fn custom_constructor_from_parts(
        &self,
        type_: CustomType,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        validate_constructor_module(&type_, module)?;
        let (definition, constructor_count) =
            self.resolve_constructor_metadata(&type_, variant_index)?;
        validate_constructor_name(&type_, variant_index, definition.name(), &name)?;
        validate_constructor_arity(&type_, definition.fields().len(), field_types.len())?;

        let mut fields = Vec::with_capacity(definition.fields().len());
        for (index, (field, actual)) in definition.fields().iter().zip(&field_types).enumerate() {
            let expected = instantiate_custom_type_template(field.type_(), &type_)?;
            validate_field_type(&type_, index, &expected, actual)?;
            fields.push(CustomConstructorField::new(
                field.label().cloned(),
                expected,
            ));
        }

        let source_shape = CustomValueShape::any(type_.clone());
        Ok(ResolvedCustomConstructor {
            constructor: CustomConstructor::new(type_, name, variant_index, fields),
            constructor_count,
            source_shape,
        })
    }

    fn resolve_constructor_metadata(
        &self,
        type_: &CustomType,
        index: usize,
    ) -> Result<(CustomConstructorDefinition, usize), PlanError> {
        let constructors = if is_prelude_result(type_) {
            validate_type_argument_count(type_, 2)?;
            vec![
                CustomConstructorDefinition::new(
                    "Ok".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    )],
                ),
                CustomConstructorDefinition::new(
                    "Error".into(),
                    1,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                    )],
                ),
            ]
        } else {
            self.resolve_custom_type_definition(type_)?
                .constructors()
                .to_vec()
        };
        let count = constructors.len();
        Ok((
            resolve_constructor_index(type_, &constructors, index)?.clone(),
            count,
        ))
    }

    fn resolve_custom_type_definition(
        &self,
        type_: &CustomType,
    ) -> Result<&CustomTypeDefinition, PlanError> {
        let definition = match self.registry {
            RegistryAccess::Program { registry } => registry.custom_type(type_.type_name()),
            #[cfg(test)]
            RegistryAccess::Local { custom_types, .. } => custom_types
                .iter()
                .find(|definition| definition.name() == type_.type_name()),
        };
        let Some(definition) = definition else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: type_.type_name().package().clone(),
                    module: type_.type_name().module().clone(),
                    name: type_.type_name().name().clone(),
                    reason: Box::new(InvalidCustomTypeReason::MissingDefinition),
                },
            });
        };
        validate_type_argument_count(type_, definition.parameters().len())?;
        Ok(definition)
    }
}

fn is_prelude_result(type_: &CustomType) -> bool {
    type_.type_name().package().is_empty()
        && type_.type_name().module() == PRELUDE_MODULE_NAME
        && type_.type_name().name() == "Result"
}

fn validate_type_argument_count(type_: &CustomType, expected: usize) -> Result<(), PlanError> {
    let actual = type_.arguments().len();
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::TypeArgumentCount { expected, actual }),
            },
        });
    }
    Ok(())
}

fn resolve_constructor_index<'a>(
    type_: &CustomType,
    constructors: &'a [CustomConstructorDefinition],
    index: usize,
) -> Result<&'a CustomConstructorDefinition, PlanError> {
    constructors
        .get(index)
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::ConstructorIndex {
                    index,
                    available: constructors.len(),
                }),
            },
        })
}

fn validate_constructor_name(
    type_: &CustomType,
    index: usize,
    expected: &EcoString,
    actual: &EcoString,
) -> Result<(), PlanError> {
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::ConstructorName {
                    index,
                    expected: expected.clone(),
                    actual: actual.clone(),
                }),
            },
        });
    }
    Ok(())
}

fn validate_constructor_module(type_: &CustomType, actual: &EcoString) -> Result<(), PlanError> {
    let expected = type_.type_name().module();
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::ConstructorModule {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }),
            },
        });
    }
    Ok(())
}

fn validate_constructor_arity(
    type_: &CustomType,
    expected: usize,
    actual: usize,
) -> Result<(), PlanError> {
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::ConstructorArity { expected, actual }),
            },
        });
    }
    Ok(())
}

struct SharedFields<'a> {
    first: (&'a CustomConstructorDefinition, &'a CustomFieldDefinition),
    rest: Vec<(&'a CustomConstructorDefinition, &'a CustomFieldDefinition)>,
}

fn resolve_shared_fields<'a>(
    type_: &CustomType,
    constructors: &[&'a CustomConstructorDefinition],
    index: usize,
) -> Result<SharedFields<'a>, PlanError> {
    let mut fields = constructors.iter().map(|constructor| {
        constructor
            .fields()
            .get(index)
            .map(|field| (*constructor, field))
    });
    let first = fields.next().flatten();
    let rest = fields.collect::<Option<Vec<_>>>();
    let (Some(first), Some(rest)) = (first, rest) else {
        let available = constructors
            .iter()
            .map(|constructor| constructor.fields().len())
            .min()
            .unwrap_or(0);
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::FieldIndex { index, available }),
            },
        });
    };
    Ok(SharedFields { first, rest })
}

fn validate_constructor_templates(
    constructor: &CustomConstructorDefinition,
    type_: &CustomType,
) -> Result<(), PlanError> {
    for field in constructor.fields() {
        instantiate_custom_type_template(field.type_(), type_)?;
    }
    Ok(())
}

fn validate_field_label(
    type_: &CustomType,
    index: usize,
    expected: Option<&EcoString>,
    actual: Option<&EcoString>,
) -> Result<(), PlanError> {
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::FieldLabel {
                    index,
                    expected: expected.cloned(),
                    actual: actual.cloned(),
                }),
            },
        });
    }
    Ok(())
}

fn validate_field_type(
    type_: &CustomType,
    index: usize,
    expected: &ValueType,
    actual: &ValueType,
) -> Result<(), PlanError> {
    if actual != expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::FieldType {
                    index,
                    expected: expected.clone(),
                    actual: actual.clone(),
                }),
            },
        });
    }
    Ok(())
}

fn merge_field_shape(
    type_: &CustomType,
    index: usize,
    previous: ValueShape,
    actual: &ValueShape,
) -> Result<ValueShape, PlanError> {
    let value_type = previous.value_type();
    previous
        .merge(actual)
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(InvalidCustomTypeReason::FieldShapeConflict {
                    index,
                    type_: value_type,
                }),
            },
        })
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{
        CustomExpr, ResolvedCustomConstructor, collect_parameter_shapes,
        instantiate_custom_shape_template, instantiate_custom_type_template,
    };
    use crate::plan::{
        CustomConstruction, CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
        CustomConstructorRefinement, CustomFieldDefinition, CustomLocal, CustomLocalId, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypeParameterId, CustomTypePublicity,
        CustomTypeTemplate, CustomValueShape, Expr, ExternalTypeName, ExternalValueShape,
        FunctionExpr, FunctionReference, FunctionShape, FunctionType, IntExpr, IntLocalId,
        ParamLocal, StringExpr, TypeParameterId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::{InvalidCustomTypeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::Publicity;
    use gleam_core::type_::{
        self, Deprecation, PatternConstructor, ValueConstructor, ValueConstructorVariant,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn recursive_custom_shape_templates_reject_incompatible_parameter_shapes() {
        let owner = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Generic".into()),
            vec![ValueType::Int],
        );
        let owner_shape = CustomValueShape::new(
            owner.type_name().clone(),
            vec![ValueShape::Int],
            CustomConstructorRefinement::Any,
        );
        let error = |reason| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: "geam".into(),
                module: "main".into(),
                name: "Generic".into(),
                reason: Box::new(reason),
            },
        };

        assert_eq!(
            instantiate_custom_shape_template(
                &CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                &owner_shape,
            ),
            Err(error(InvalidCustomTypeReason::TemplateParameterIndex {
                index: 1,
                available: 1,
            })),
        );
        for template in [
            CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(
                1,
            ))]),
            CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
                CustomTypeParameterId(1),
            ))),
            CustomTypeTemplate::Function {
                arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
                return_: Box::new(CustomTypeTemplate::Int),
            },
            CustomTypeTemplate::Function {
                arguments: Vec::new(),
                return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(1))),
            },
            CustomTypeTemplate::Custom {
                name: CustomTypeName::new("geam".into(), "main".into(), "Nested".into()),
                arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
            },
            CustomTypeTemplate::External {
                name: ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
                arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
            },
        ] {
            assert_eq!(
                instantiate_custom_shape_template(&template, &owner_shape),
                Err(error(InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 1,
                    available: 1,
                })),
            );
        }

        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(
                &CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                &ValueShape::Int,
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateParameterIndex {
                index: 1,
                available: 1,
            })),
        );
        let mut arguments = vec![Some(ValueShape::Int)];
        assert_eq!(
            collect_parameter_shapes(
                &CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                &ValueShape::String,
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::ConflictingParameterShape {
                parameter: 0,
                previous: ValueType::Int,
                actual: ValueType::String,
            },)),
        );

        let tuple = CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
            CustomTypeParameterId(0),
        )]);
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&tuple, &ValueShape::Int, &mut arguments, &owner),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Int,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &tuple,
                &ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            })),
        );
        let owner_without_arguments = CustomType::new(owner.type_name().clone(), Vec::new());
        assert_eq!(
            collect_parameter_shapes(
                &tuple,
                &ValueShape::Int,
                &mut Vec::new(),
                &owner_without_arguments,
            ),
            Err(error(InvalidCustomTypeReason::TemplateParameterIndex {
                index: 0,
                available: 0,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &tuple,
                &ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                &mut arguments,
                &owner,
            ),
            Ok(()),
        );
        assert_eq!(arguments, vec![Some(ValueShape::Int)]);

        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        assert_eq!(
            context.define_existing_param(
                "value".into(),
                &ParamLocal::int(IntLocalId(0)),
                ValueShape::String,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );

        for (template, shape) in [
            (
                CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(1),
                )]),
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
            ),
            (
                CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(1),
                ))),
                ValueShape::List(Box::new(ValueShape::Int)),
            ),
            (
                CustomTypeTemplate::Function {
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
                    return_: Box::new(CustomTypeTemplate::Int),
                },
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::Int],
                    ValueShape::Int,
                ))),
            ),
            (
                CustomTypeTemplate::Function {
                    arguments: Vec::new(),
                    return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(1))),
                },
                ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int))),
            ),
            (
                CustomTypeTemplate::Custom {
                    name: CustomTypeName::new("geam".into(), "main".into(), "Nested".into()),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
                },
                ValueShape::Custom(CustomValueShape::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Nested".into()),
                    vec![ValueShape::Int],
                    CustomConstructorRefinement::Any,
                )),
            ),
            (
                CustomTypeTemplate::External {
                    name: ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(1))],
                },
                ValueShape::External(ExternalValueShape::new(
                    ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
                    vec![ValueShape::Int],
                )),
            ),
        ] {
            let mut arguments = vec![None];
            assert_eq!(
                collect_parameter_shapes(&template, &shape, &mut arguments, &owner),
                Err(error(InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 1,
                    available: 1,
                })),
            );
        }

        let list = CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
            CustomTypeParameterId(0),
        )));
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&list, &ValueShape::Int, &mut arguments, &owner),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::Int,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &list,
                &ValueShape::List(Box::new(ValueShape::Int)),
                &mut arguments,
                &owner,
            ),
            Ok(()),
        );
        assert_eq!(arguments, vec![Some(ValueShape::Int)]);

        let external_name = ExternalTypeName::new("geam".into(), "main".into(), "Resource".into());
        let external = CustomTypeTemplate::External {
            name: external_name.clone(),
            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
        };
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&external, &ValueShape::Int, &mut arguments, &owner),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::External(crate::plan::ExternalType::new(
                    external_name.clone(),
                    vec![ValueType::Int],
                )),
                actual: ValueType::Int,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &external,
                &ValueShape::External(ExternalValueShape::new(
                    ExternalTypeName::new("geam".into(), "main".into(), "Other".into()),
                    vec![ValueShape::Int],
                )),
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::External(crate::plan::ExternalType::new(
                    external_name.clone(),
                    vec![ValueType::Int],
                )),
                actual: ValueType::External(crate::plan::ExternalType::new(
                    ExternalTypeName::new("geam".into(), "main".into(), "Other".into()),
                    vec![ValueType::Int],
                )),
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &external,
                &ValueShape::External(ExternalValueShape::new(
                    external_name.clone(),
                    vec![ValueShape::Int, ValueShape::String],
                )),
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::External(crate::plan::ExternalType::new(
                    external_name.clone(),
                    vec![ValueType::Int],
                )),
                actual: ValueType::External(crate::plan::ExternalType::new(
                    external_name.clone(),
                    vec![ValueType::Int, ValueType::String],
                )),
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &external,
                &ValueShape::External(ExternalValueShape::new(
                    external_name,
                    vec![ValueShape::Int],
                )),
                &mut arguments,
                &owner,
            ),
            Ok(()),
        );
        assert_eq!(arguments, vec![Some(ValueShape::Int)]);

        let function = CustomTypeTemplate::Function {
            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
            return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(0))),
        };
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&function, &ValueShape::Int, &mut arguments, &owner),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                actual: ValueType::Int,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &function,
                &ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int,))),
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                actual: ValueType::Function(Box::new(FunctionType::new(
                    Vec::new(),
                    ValueType::Int,
                ))),
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &function,
                &ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::Int],
                    ValueShape::Int,
                ))),
                &mut arguments,
                &owner,
            ),
            Ok(()),
        );
        assert_eq!(arguments, vec![Some(ValueShape::Int)]);

        let nested_name = CustomTypeName::new("geam".into(), "main".into(), "Nested".into());
        let custom = CustomTypeTemplate::Custom {
            name: nested_name.clone(),
            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
        };
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&custom, &ValueShape::Int, &mut arguments, &owner),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Custom(CustomType::new(
                    nested_name.clone(),
                    vec![ValueType::Int],
                )),
                actual: ValueType::Int,
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &custom,
                &ValueShape::Custom(CustomValueShape::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
                    vec![ValueShape::Int],
                    CustomConstructorRefinement::Any,
                )),
                &mut arguments,
                &owner,
            ),
            Err(error(InvalidCustomTypeReason::TemplateShapeMismatch {
                expected: ValueType::Custom(CustomType::new(
                    nested_name.clone(),
                    vec![ValueType::Int],
                )),
                actual: ValueType::Custom(CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
                    vec![ValueType::Int],
                )),
            })),
        );
        assert_eq!(
            collect_parameter_shapes(
                &custom,
                &ValueShape::Custom(CustomValueShape::new(
                    nested_name,
                    vec![ValueShape::Int],
                    CustomConstructorRefinement::Exact(0),
                )),
                &mut arguments,
                &owner,
            ),
            Ok(()),
        );
        assert_eq!(arguments, vec![Some(ValueShape::Int)]);
    }

    #[test]
    fn custom_shape_templates_preserve_every_primitive_shape() {
        let owner_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Owner".into()),
            vec![ValueShape::Int],
            CustomConstructorRefinement::Any,
        );

        for (template, expected) in [
            (CustomTypeTemplate::Int, ValueShape::Int),
            (CustomTypeTemplate::Float, ValueShape::Float),
            (CustomTypeTemplate::String, ValueShape::String),
            (CustomTypeTemplate::BitArray, ValueShape::BitArray),
            (CustomTypeTemplate::UtfCodepoint, ValueShape::UtfCodepoint),
            (CustomTypeTemplate::Bool, ValueShape::Bool),
            (CustomTypeTemplate::Nil, ValueShape::Nil),
        ] {
            assert_eq!(
                instantiate_custom_shape_template(&template, &owner_shape),
                Ok(expected),
            );
        }

        let external_name = ExternalTypeName::new("geam".into(), "main".into(), "Resource".into());
        assert_eq!(
            instantiate_custom_shape_template(
                &CustomTypeTemplate::External {
                    name: external_name.clone(),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                &owner_shape,
            ),
            Ok(ValueShape::External(ExternalValueShape::new(
                external_name,
                vec![ValueShape::Int],
            ))),
        );
    }

    #[test]
    fn custom_construction_shape_rejects_malformed_constructor_metadata() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let boxed_name = CustomTypeName::new("geam".into(), module.clone(), "Boxed".into());
        let broken_name = CustomTypeName::new("geam".into(), module.clone(), "Broken".into());
        let repeated_name = CustomTypeName::new("geam".into(), module.clone(), "Repeated".into());
        let definitions = vec![
            CustomTypeDefinition::new(
                boxed_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Boxed".into(),
                    0,
                    vec![CustomFieldDefinition::new(None, CustomTypeTemplate::Int)],
                )],
            ),
            CustomTypeDefinition::new(
                broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Broken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                repeated_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Repeated".into(),
                    0,
                    vec![
                        CustomFieldDefinition::new(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        ),
                        CustomFieldDefinition::new(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        ),
                    ],
                )],
            ),
        ];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);

        let result = CustomType::new(
            CustomTypeName::new(
                "".into(),
                type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        let invalid_result = CustomConstruction::try_new(
            CustomConstructor::new(
                result.clone(),
                "Invalid".into(),
                2,
                vec![CustomConstructorField::new(None, ValueType::Int)],
            ),
            vec![Expr::int(IntExpr::value(1.into()))],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&invalid_result),
            Err(invalid_custom_constructor_error(
                &result,
                InvalidCustomTypeReason::ConstructorIndex {
                    index: 2,
                    available: 2,
                },
            )),
        );
        assert_eq!(
            context
                .custom_expr_from_construction(invalid_result)
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &result,
                InvalidCustomTypeReason::ConstructorIndex {
                    index: 2,
                    available: 2,
                },
            )),
        );

        let error = CustomConstruction::try_new(
            CustomConstructor::new(
                result.clone(),
                "Error".into(),
                1,
                vec![CustomConstructorField::new(None, ValueType::String)],
            ),
            vec![Expr::string(StringExpr::value("error".into()))],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&error),
            Ok(CustomValueShape::new(
                result.type_name().clone(),
                vec![ValueShape::Int, ValueShape::String],
                CustomConstructorRefinement::Exact(1),
            )),
        );

        let missing = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
            Vec::new(),
        );
        let unknown = CustomConstruction::try_new(
            CustomConstructor::new(missing.clone(), "Missing".into(), 0, Vec::new()),
            Vec::new(),
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&unknown),
            Err(invalid_custom_constructor_error(
                &missing,
                InvalidCustomTypeReason::MissingDefinition,
            )),
        );

        let boxed = CustomType::new(boxed_name, Vec::new());
        let missing_field = CustomConstruction::try_new(
            CustomConstructor::new(boxed.clone(), "Boxed".into(), 0, Vec::new()),
            Vec::new(),
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&missing_field),
            Err(invalid_custom_constructor_error(
                &boxed,
                InvalidCustomTypeReason::ConstructorArity {
                    expected: 1,
                    actual: 0,
                },
            )),
        );
        let invalid_constructor = CustomConstruction::try_new(
            CustomConstructor::new(
                boxed.clone(),
                "Invalid".into(),
                1,
                vec![CustomConstructorField::new(None, ValueType::Int)],
            ),
            vec![Expr::int(IntExpr::value(1.into()))],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&invalid_constructor),
            Err(invalid_custom_constructor_error(
                &boxed,
                InvalidCustomTypeReason::ConstructorIndex {
                    index: 1,
                    available: 1,
                },
            )),
        );

        let invalid_field = CustomConstruction::try_new(
            CustomConstructor::new(
                boxed.clone(),
                "Boxed".into(),
                0,
                vec![CustomConstructorField::new(None, ValueType::String)],
            ),
            vec![Expr::string(StringExpr::value("wrong".into()))],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&invalid_field),
            Err(invalid_custom_field_error(
                &boxed,
                0,
                ValueType::Int,
                ValueType::String,
            )),
        );

        let broken = CustomType::new(broken_name, vec![ValueType::Int]);
        let invalid_template = CustomConstruction::try_new(
            CustomConstructor::new(
                broken.clone(),
                "Broken".into(),
                0,
                vec![CustomConstructorField::new(None, ValueType::Int)],
            ),
            vec![Expr::int(IntExpr::value(1.into()))],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&invalid_template),
            Err(invalid_custom_constructor_error(
                &broken,
                InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 1,
                    available: 1,
                },
            )),
        );

        let choice = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Choice".into()),
            Vec::new(),
        );
        let function_type =
            FunctionType::new(vec![ValueType::Custom(choice.clone())], ValueType::Int);
        let repeated = CustomType::new(
            repeated_name,
            vec![ValueType::Function(Box::new(function_type.clone()))],
        );
        let field = |local, constructor| {
            let parameter_shape = CustomValueShape::new(
                choice.type_name().clone(),
                Vec::new(),
                CustomConstructorRefinement::Exact(constructor),
            );
            let shape = FunctionShape::new(
                vec![ValueShape::Custom(parameter_shape.clone())],
                ValueShape::Int,
            );
            Expr::function(
                FunctionExpr::reference(FunctionReference::new(
                    crate::plan::monomorphic_function_instantiation(local, shape.clone()),
                ))
                .with_resolved_shape(shape)
                .expect("function shape has the same nominal type"),
            )
        };
        let conflicting_fields = CustomConstruction::try_new(
            CustomConstructor::new(
                repeated.clone(),
                "Repeated".into(),
                0,
                vec![
                    CustomConstructorField::new(
                        None,
                        ValueType::Function(Box::new(function_type.clone())),
                    ),
                    CustomConstructorField::new(
                        None,
                        ValueType::Function(Box::new(function_type.clone())),
                    ),
                ],
            ),
            vec![field(0, 0), field(1, 1)],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&conflicting_fields),
            Err(invalid_custom_constructor_error(
                &repeated,
                InvalidCustomTypeReason::ConflictingParameterShape {
                    parameter: 0,
                    previous: ValueType::Function(Box::new(function_type.clone())),
                    actual: ValueType::Function(Box::new(function_type)),
                },
            )),
        );
    }

    #[test]
    fn field_access_validates_every_possible_constructor() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let shared_name = CustomTypeName::new("geam".into(), module.clone(), "Shared".into());
        let labels_name = CustomTypeName::new("geam".into(), module.clone(), "Labels".into());
        let types_name = CustomTypeName::new("geam".into(), module.clone(), "Types".into());
        let parameter_name = CustomTypeName::new("geam".into(), module.clone(), "Parameter".into());
        let trailing_name = CustomTypeName::new("geam".into(), module.clone(), "Trailing".into());
        let merge_name = CustomTypeName::new("geam".into(), module.clone(), "Merge".into());
        let choice_name = CustomTypeName::new("geam".into(), module.clone(), "Choice".into());
        let field = |label, type_| CustomFieldDefinition::new(label, type_);
        let constructor =
            |name, index, fields| CustomConstructorDefinition::new(name, index, fields);
        let definitions = vec![
            CustomTypeDefinition::new(
                shared_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(None, CustomTypeTemplate::Int)],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![field(None, CustomTypeTemplate::Int)],
                    ),
                ],
            ),
            CustomTypeDefinition::new(
                labels_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(Some("value".into()), CustomTypeTemplate::Int)],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![field(Some("other".into()), CustomTypeTemplate::Int)],
                    ),
                ],
            ),
            CustomTypeDefinition::new(
                types_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(None, CustomTypeTemplate::Int)],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![field(None, CustomTypeTemplate::String)],
                    ),
                ],
            ),
            CustomTypeDefinition::new(
                parameter_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(None, CustomTypeTemplate::Int)],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![field(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        )],
                    ),
                ],
            ),
            CustomTypeDefinition::new(
                trailing_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(None, CustomTypeTemplate::Int)],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![
                            field(None, CustomTypeTemplate::Int),
                            field(
                                None,
                                CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                            ),
                        ],
                    ),
                ],
            ),
            CustomTypeDefinition::new(
                merge_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0), CustomTypeParameterId(1)],
                vec![
                    constructor(
                        "First".into(),
                        0,
                        vec![field(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        )],
                    ),
                    constructor(
                        "Second".into(),
                        1,
                        vec![field(
                            None,
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                        )],
                    ),
                ],
            ),
        ];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);
        let source = |shape: CustomValueShape| {
            CustomExpr::local_get(
                CustomLocal::from_shape(CustomLocalId(0), shape),
                "source".into(),
            )
        };

        let shared = CustomType::new(shared_name, Vec::new());
        assert_eq!(
            context
                .custom_field_access(
                    source(CustomValueShape::any(shared)),
                    0,
                    None,
                    &ValueType::Int,
                )
                .map(|(_, shape)| shape),
            Ok(ValueShape::Int),
        );

        let labels = CustomType::new(labels_name, Vec::new());
        assert_eq!(
            context
                .custom_field_access(
                    source(CustomValueShape::any(labels.clone())),
                    0,
                    Some("value".into()),
                    &ValueType::Int,
                )
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &labels,
                InvalidCustomTypeReason::FieldLabel {
                    index: 0,
                    expected: Some("other".into()),
                    actual: Some("value".into()),
                },
            )),
        );

        let types = CustomType::new(types_name, Vec::new());
        assert_eq!(
            context
                .custom_field_access(
                    source(CustomValueShape::any(types.clone())),
                    0,
                    None,
                    &ValueType::Int,
                )
                .map(|_| ()),
            Err(invalid_custom_field_error(
                &types,
                0,
                ValueType::Int,
                ValueType::String,
            )),
        );

        let parameter = CustomType::new(parameter_name, Vec::new());
        assert_eq!(
            context
                .custom_field_access(
                    source(CustomValueShape::any(parameter.clone())),
                    0,
                    None,
                    &ValueType::Int,
                )
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &parameter,
                InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 0,
                    available: 0,
                },
            )),
        );

        let choice = CustomType::new(choice_name.clone(), Vec::new());
        let function_type =
            FunctionType::new(vec![ValueType::Custom(choice.clone())], ValueType::Int);
        let merge = CustomType::new(
            merge_name,
            vec![
                ValueType::Function(Box::new(function_type.clone())),
                ValueType::Function(Box::new(function_type.clone())),
            ],
        );
        let merge_shape = CustomValueShape::new(
            merge.type_name().clone(),
            vec![
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::Custom(CustomValueShape::new(
                        choice_name.clone(),
                        Vec::new(),
                        CustomConstructorRefinement::Exact(0),
                    ))],
                    ValueShape::Int,
                ))),
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::Custom(CustomValueShape::new(
                        choice_name,
                        Vec::new(),
                        CustomConstructorRefinement::Exact(1),
                    ))],
                    ValueShape::Int,
                ))),
            ],
            CustomConstructorRefinement::Any,
        );
        assert_eq!(
            context
                .custom_field_access(
                    source(merge_shape),
                    0,
                    None,
                    &ValueType::Function(Box::new(function_type.clone())),
                )
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &merge,
                InvalidCustomTypeReason::FieldShapeConflict {
                    index: 0,
                    type_: ValueType::Function(Box::new(function_type)),
                },
            )),
        );

        let trailing = CustomType::new(trailing_name, Vec::new());
        assert_eq!(
            context
                .custom_field_access(
                    source(CustomValueShape::any(trailing.clone())),
                    0,
                    None,
                    &ValueType::Int,
                )
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &trailing,
                InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 0,
                    available: 0,
                },
            )),
        );
    }

    #[test]
    fn custom_constructor_and_equality_metadata_reject_invalid_typed_ast_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let generic_name = CustomTypeName::new("geam".into(), module.clone(), "Generic".into());
        let function_name =
            CustomTypeName::new("geam".into(), module.clone(), "FunctionBox".into());
        let tuple_function_name =
            CustomTypeName::new("geam".into(), module.clone(), "TupleFunctionBox".into());
        let list_function_name =
            CustomTypeName::new("geam".into(), module.clone(), "ListFunctionBox".into());
        let broken_name = CustomTypeName::new("geam".into(), module.clone(), "Broken".into());
        let tuple_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "TupleBroken".into());
        let custom_argument_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "CustomArgumentBroken".into());
        let nested_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "NestedBroken".into());
        let missing_name = CustomTypeName::new("geam".into(), module.clone(), "Missing".into());
        let recursive_name = CustomTypeName::new("geam".into(), module.clone(), "Recursive".into());
        let definitions = vec![
            CustomTypeDefinition::new(
                generic_name.clone(),
                CustomTypePublicity::Public,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Generic".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        Some("value".into()),
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "FunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Function {
                            arguments: vec![CustomTypeTemplate::Int],
                            return_: Box::new(CustomTypeTemplate::String),
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                tuple_function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "TupleFunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Tuple(vec![
                            CustomTypeTemplate::Int,
                            CustomTypeTemplate::Function {
                                arguments: Vec::new(),
                                return_: Box::new(CustomTypeTemplate::Nil),
                            },
                        ]),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                list_function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "ListFunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Function {
                            arguments: Vec::new(),
                            return_: Box::new(CustomTypeTemplate::Nil),
                        })),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Broken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                tuple_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "TupleBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
                            CustomTypeParameterId(0),
                        )]),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                custom_argument_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "CustomArgumentBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: generic_name.clone(),
                            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(
                                0,
                            ))],
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                nested_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "NestedBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: missing_name.clone(),
                            arguments: Vec::new(),
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                recursive_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Recursive".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: recursive_name.clone(),
                            arguments: Vec::new(),
                        },
                    )],
                )],
            ),
        ];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);
        let generic_int = CustomType::new(generic_name.clone(), vec![ValueType::Int]);

        for (template, expected) in [
            (CustomTypeTemplate::Float, ValueType::Float),
            (CustomTypeTemplate::BitArray, ValueType::BitArray),
            (CustomTypeTemplate::UtfCodepoint, ValueType::UtfCodepoint),
            (CustomTypeTemplate::Bool, ValueType::Bool),
            (CustomTypeTemplate::Nil, ValueType::Nil),
            (
                CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Int)),
                ValueType::List(Box::new(ValueType::Int)),
            ),
        ] {
            assert_eq!(
                instantiate_custom_type_template(&template, &generic_int),
                Ok(expected),
            );
        }

        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Ok(ResolvedCustomConstructor {
                constructor: CustomConstructor::new(
                    generic_int.clone(),
                    "Generic".into(),
                    0,
                    vec![CustomConstructorField::new(
                        Some("value".into()),
                        ValueType::Int,
                    )],
                ),
                constructor_count: 1,
                source_shape: CustomValueShape::any(generic_int.clone()),
            }),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::String],
            ),
            Err(invalid_custom_field_error(
                &generic_int,
                0,
                ValueType::Int,
                ValueType::String,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Wrong".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorName {
                    index: 0,
                    expected: "Generic".into(),
                    actual: "Wrong".into(),
                },
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &"other".into(),
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorModule {
                    expected: "main".into(),
                    actual: "other".into(),
                },
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                Vec::new(),
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorArity {
                    expected: 1,
                    actual: 0,
                },
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                1,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorIndex {
                    index: 1,
                    available: 1,
                },
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                CustomType::new(
                    CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
                    Vec::new(),
                ),
                "Missing".into(),
                &module,
                0,
                Vec::new(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Missing".into(),
                    reason: Box::new(InvalidCustomTypeReason::MissingDefinition),
                },
            }),
        );
        let generic_without_arguments = CustomType::new(generic_name.clone(), Vec::new());
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_without_arguments.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_without_arguments,
                InvalidCustomTypeReason::TypeArgumentCount {
                    expected: 1,
                    actual: 0,
                },
            )),
        );

        let result_type = CustomType::new(
            CustomTypeName::new(
                "".into(),
                type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        assert_eq!(
            ValueType::from_gleam(type_::result(type_::int(), type_::string()).as_ref()),
            Some(ValueType::Custom(result_type.clone())),
        );
        let prelude = result_type.type_name().module().clone();
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                1,
                vec![ValueType::String],
            ),
            Ok(ResolvedCustomConstructor {
                constructor: CustomConstructor::new(
                    result_type.clone(),
                    "Error".into(),
                    1,
                    vec![CustomConstructorField::new(None, ValueType::String)],
                ),
                constructor_count: 2,
                source_shape: CustomValueShape::any(result_type.clone()),
            }),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                1,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_field_error(
                &result_type,
                0,
                ValueType::String,
                ValueType::Int,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &result_type,
                InvalidCustomTypeReason::ConstructorName {
                    index: 0,
                    expected: "Ok".into(),
                    actual: "Error".into(),
                },
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Ok".into(),
                &prelude,
                2,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &result_type,
                InvalidCustomTypeReason::ConstructorIndex {
                    index: 2,
                    available: 2,
                },
            )),
        );
        let malformed_result_type =
            CustomType::new(result_type.type_name().clone(), vec![ValueType::Int]);
        assert_eq!(
            context.custom_constructor_from_parts(
                malformed_result_type.clone(),
                "Ok".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &malformed_result_type,
                InvalidCustomTypeReason::TypeArgumentCount {
                    expected: 2,
                    actual: 1,
                },
            )),
        );
        let non_prelude_result = CustomType::new(
            CustomTypeName::new(
                "other".into(),
                type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                non_prelude_result.clone(),
                "Ok".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &non_prelude_result,
                InvalidCustomTypeReason::MissingDefinition,
            )),
        );
        let broken = CustomType::new(broken_name, vec![ValueType::Int]);
        assert_eq!(
            context.custom_constructor_from_parts(
                broken.clone(),
                "Broken".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Broken".into(),
                    reason: Box::new(InvalidCustomTypeReason::TemplateParameterIndex {
                        index: 1,
                        available: 1,
                    }),
                },
            }),
        );

        let missing_parameter = CustomType::new(generic_name, Vec::new());
        let parameter_error = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: "geam".into(),
                module: "main".into(),
                name: "Generic".into(),
                reason: Box::new(InvalidCustomTypeReason::TemplateParameterIndex {
                    index: 0,
                    available: 0,
                }),
            },
        });
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(0),
                )]),
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(0),
                ))),
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Function {
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                    return_: Box::new(CustomTypeTemplate::Int),
                },
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Function {
                    arguments: Vec::new(),
                    return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(0))),
                },
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Custom {
                    name: missing_parameter.type_name().clone(),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::External {
                    name: crate::plan::ExternalTypeName::new(
                        "dependency".into(),
                        "dependency/resource".into(),
                        "Resource".into(),
                    ),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                &missing_parameter,
            ),
            parameter_error,
        );
    }

    #[test]
    fn custom_constructor_typed_ast_margins_are_exact() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let local = ValueConstructor::local_variable(
            crate::planner::support::dummy_span(),
            gleam_core::type_::error::VariableOrigin::generated(),
            type_::int(),
        );
        assert_eq!(
            context.custom_constructor(&local),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        let invalid_field = ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_: Arc::new(gleam_core::type_::Type::Fn {
                arguments: vec![type_::generic_var(0)],
                return_: type_::result(type_::int(), type_::string()),
            }),
            variant: ValueConstructorVariant::Record {
                name: "Ok".into(),
                arity: 1,
                field_map: None,
                location: crate::planner::support::dummy_span(),
                module: type_::PRELUDE_MODULE_NAME.into(),
                variants_count: 2,
                variant_index: 0,
                documentation: None,
            },
        };
        assert_eq!(
            context.custom_constructor(&invalid_field),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: type_::PRELUDE_MODULE_NAME.into(),
                    name: "Result".into(),
                    reason: Box::new(InvalidCustomTypeReason::FieldType {
                        index: 0,
                        expected: ValueType::Int,
                        actual: ValueType::Parameter(TypeParameterId(0)),
                    }),
                },
            }),
        );
        let invalid_constructor_type = ValueConstructor {
            type_: type_::int(),
            ..invalid_field
        };
        assert_eq!(
            context.custom_constructor(&invalid_constructor_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: type_::PRELUDE_MODULE_NAME.into(),
                    name: "Ok".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: ValueType::Int,
                    }),
                },
            }),
        );

        let pattern = PatternConstructor {
            name: "Invalid".into(),
            field_map: None,
            documentation: None,
            module: module.clone(),
            location: crate::planner::support::dummy_span(),
            constructor_index: 0,
        };
        assert_eq!(
            context.custom_pattern_constructor(
                type_::generic_var(0).as_ref(),
                &pattern,
                Vec::new()
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "main".into(),
                    name: "Invalid".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: ValueType::Parameter(TypeParameterId(0)),
                    }),
                },
            }),
        );
        assert_eq!(
            context.custom_pattern_constructor(type_::int().as_ref(), &pattern, Vec::new()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "main".into(),
                    name: "Invalid".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: ValueType::Int,
                    }),
                },
            }),
        );
    }

    fn invalid_custom_constructor_error(
        type_: &CustomType,
        reason: InvalidCustomTypeReason,
    ) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                package: type_.type_name().package().clone(),
                module: type_.type_name().module().clone(),
                name: type_.type_name().name().clone(),
                reason: Box::new(reason),
            },
        }
    }

    fn invalid_custom_field_error(
        type_: &CustomType,
        index: usize,
        expected: ValueType,
        actual: ValueType,
    ) -> PlanError {
        invalid_custom_constructor_error(
            type_,
            InvalidCustomTypeReason::FieldType {
                index,
                expected,
                actual,
            },
        )
    }
}
