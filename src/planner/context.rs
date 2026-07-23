use crate::plan::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CaptureArg, CustomConstructor, CustomConstructorField,
    CustomExpr, CustomFieldAccess, CustomFunctionLocal, CustomFunctionLocalId, CustomFunctionType,
    CustomListLocalId, CustomLocalId, CustomTypeDefinition, CustomTypeTemplate,
    FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionFunctionLocalId, FunctionFunctionType, FunctionListLocalId, FunctionReference,
    FunctionTemplate, FunctionTemplateSignature, FunctionType, IntFunctionLocalId, IntListLocalId,
    IntLocalId, ListExpr, ListFunctionLocal, ListListLocalId, ListLocal, ListLocalExpr, LocalId,
    NilFunctionLocalId, NilListLocalId, NilLocalId, PanicSite, ParamBinding, ParamLocal, ParamSlot,
    StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId, ValueType,
};
use crate::plan::{
    CustomConstructorRefinement, CustomType, CustomValueShape, FunctionShape, ValueShape,
};
use crate::planner::error::{
    InvalidCustomTypeReason, InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
};
use ecow::EcoString;
use gleam_core::type_::{
    PRELUDE_MODULE_NAME, PatternConstructor, Type, ValueConstructor, ValueConstructorVariant,
};
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) signature: FunctionTemplateSignature,
    pub(super) type_parameters: super::type_parameter::TypeParameterScope,
    pub(super) return_shape: ValueShape,
    pub(super) params: Vec<FunctionParam>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    slot: crate::plan::ParamSlot,
    pub(super) binding: ParamBinding,
    pub(super) label: Option<EcoString>,
}

impl FunctionParam {
    pub(super) fn new(
        local: ParamLocal,
        shape: ValueShape,
        binding: ParamBinding,
        label: Option<EcoString>,
    ) -> Self {
        Self {
            slot: crate::plan::ParamSlot::new(local, shape),
            binding,
            label,
        }
    }

    pub(super) fn local(&self) -> &ParamLocal {
        self.slot.local()
    }

    pub(super) fn shape(&self) -> &ValueShape {
        self.slot.shape()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ResolvedCustomConstructor {
    constructor: CustomConstructor,
    constructor_count: usize,
}

impl ResolvedCustomConstructor {
    pub(super) fn constructor_count(&self) -> usize {
        self.constructor_count
    }

    pub(super) fn into_constructor(self) -> CustomConstructor {
        self.constructor
    }
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    current_function: EcoString,
    registry: RegistryAccess<'a>,
    anonymous_functions: &'a mut AnonymousFunctions,
    bindings: HashMap<EcoString, LocalBinding>,
    next_generic_local: usize,
    next_int_local: usize,
    next_float_local: usize,
    next_string_local: usize,
    next_bit_array_local: usize,
    next_utf_codepoint_local: usize,
    next_custom_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
    next_tuple_local: usize,
    next_int_list_local: usize,
    next_string_list_local: usize,
    next_bit_array_list_local: usize,
    next_utf_codepoint_list_local: usize,
    next_custom_list_local: usize,
    next_float_list_local: usize,
    next_bool_list_local: usize,
    next_nil_list_local: usize,
    next_tuple_list_local: usize,
    next_list_list_local: usize,
    next_function_list_local: usize,
    next_generic_list_local: usize,
    next_int_function_local: usize,
    next_float_function_local: usize,
    next_string_function_local: usize,
    next_bit_array_function_local: usize,
    next_utf_codepoint_function_local: usize,
    next_custom_function_local: usize,
    next_bool_function_local: usize,
    next_nil_function_local: usize,
    next_tuple_function_local: usize,
    next_list_function_local: usize,
    next_function_function_local: usize,
    next_generic_function_local: usize,
    type_parameters: super::type_parameter::TypeParameterScope,
}

#[derive(Clone, Copy)]
enum RegistryAccess<'a> {
    Program {
        registry: &'a crate::planner::module::registry::ProgramRegistry,
    },
    #[cfg(test)]
    Local {
        functions: &'a HashMap<EcoString, FunctionInfo>,
        custom_types: &'a [CustomTypeDefinition],
    },
}

#[derive(Clone)]
enum LocalBinding {
    Primitive(LocalId),
    Custom(crate::plan::CustomLocal),
    Tuple {
        local: TupleLocalId,
        shape: Box<[ValueShape]>,
    },
    List {
        local: ListLocal,
        item_shape: ValueShape,
    },
    Function {
        binding: FunctionLocalBinding,
        shape: FunctionShape,
    },
}

pub(super) struct CaptureBinding {
    name: EcoString,
    binding: LocalBinding,
}

pub(super) struct PlannedCaptures {
    captures: Box<[PlannedCapture]>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct PlannedCapture {
    slot: ParamSlot,
    source: CaptureArg,
}

impl PlannedCaptures {
    pub(super) fn into_parts(self) -> (Vec<ParamSlot>, Vec<CaptureArg>) {
        let mut slots = Vec::with_capacity(self.captures.len());
        let mut sources = Vec::with_capacity(self.captures.len());
        for capture in self.captures {
            slots.push(capture.slot);
            sources.push(capture.source);
        }
        (slots, sources)
    }
}

fn invalid_local_shape() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::Invalid,
        },
    }
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
        CustomTypeTemplate::Parameter(parameter) => {
            let Some(type_) = custom_type.arguments().get(parameter.0).cloned() else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: custom_type.type_name().name().clone(),
                        reason: InvalidCustomTypeReason::ParameterType,
                    },
                });
            };
            type_
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
        CustomTypeTemplate::Parameter(parameter) => custom_shape
            .arguments()
            .get(parameter.0)
            .cloned()
            .ok_or_else(|| {
                invalid_custom_type(custom_shape.type_(), InvalidCustomTypeReason::ParameterType)
            })?,
    };
    Ok(shape)
}

fn invalid_custom_type(type_: &CustomType, reason: InvalidCustomTypeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CustomType {
            name: type_.type_name().name().clone(),
            reason,
        },
    }
}

fn collect_parameter_shapes(
    template: &CustomTypeTemplate,
    shape: &ValueShape,
    arguments: &mut [Option<ValueShape>],
    owner: &CustomType,
) -> Result<(), PlanError> {
    match template {
        CustomTypeTemplate::Parameter(parameter) => {
            let Some(argument) = arguments.get_mut(parameter.0) else {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            };
            *argument = Some(match argument.take() {
                Some(previous) => previous.merge(shape).ok_or_else(|| {
                    invalid_custom_type(owner, InvalidCustomTypeReason::ParameterType)
                })?,
                None => shape.clone(),
            });
        }
        CustomTypeTemplate::Tuple(templates) => {
            let ValueShape::Tuple(shapes) = shape else {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            };
            if templates.len() != shapes.len() {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            }
            for (template, shape) in templates.iter().zip(shapes.iter()) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
        }
        CustomTypeTemplate::List(template) => {
            let ValueShape::List(shape) = shape else {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            };
            collect_parameter_shapes(template, shape, arguments, owner)?;
        }
        CustomTypeTemplate::Function {
            arguments: templates,
            return_: return_template,
        } => {
            let ValueShape::Function(shape) = shape else {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            };
            if templates.len() != shape.argument_shapes().len() {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            }
            for (template, shape) in templates.iter().zip(shape.argument_shapes()) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
            collect_parameter_shapes(return_template, shape.return_shape(), arguments, owner)?;
        }
        CustomTypeTemplate::Custom {
            name,
            arguments: templates,
        } => {
            let ValueShape::Custom(shape) = shape else {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            };
            if shape.type_name() != name || templates.len() != shape.arguments().len() {
                return Err(invalid_custom_type(
                    owner,
                    InvalidCustomTypeReason::ParameterType,
                ));
            }
            for (template, shape) in templates.iter().zip(shape.arguments()) {
                collect_parameter_shapes(template, shape, arguments, owner)?;
            }
        }
        CustomTypeTemplate::Int
        | CustomTypeTemplate::Float
        | CustomTypeTemplate::String
        | CustomTypeTemplate::BitArray
        | CustomTypeTemplate::UtfCodepoint
        | CustomTypeTemplate::Bool
        | CustomTypeTemplate::Nil => {}
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionLocalBinding {
    Generic(crate::plan::GenericFunctionLocal),
    Int {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    String {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArray {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepoint {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    Custom(CustomFunctionLocal),
    Float {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    Bool {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    Nil {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    Tuple {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    List(ListFunctionLocal),
    Function(FunctionFunctionLocal),
}

impl<'a> PlanContext<'a> {
    #[cfg(test)]
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_custom_types(module_name, functions, &[], anonymous_functions)
    }

    #[cfg(test)]
    pub(super) fn new_with_custom_types(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        custom_types: &'a [CustomTypeDefinition],
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_registry(
            module_name,
            RegistryAccess::Local {
                functions,
                custom_types,
            },
            anonymous_functions,
        )
    }

    pub(super) fn new_in_program(
        module: crate::plan::ModuleId,
        registry: &'a crate::planner::module::registry::ProgramRegistry,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_registry(
            registry.module_name(module),
            RegistryAccess::Program { registry },
            anonymous_functions,
        )
    }

    fn new_with_registry(
        module_name: &'a EcoString,
        registry: RegistryAccess<'a>,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self {
            module_name,
            current_function: "main".into(),
            registry,
            anonymous_functions,
            bindings: HashMap::new(),
            next_generic_local: 0,
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_generic_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
            next_generic_function_local: 0,
            type_parameters: super::type_parameter::TypeParameterScope::default(),
        }
    }

    pub(super) fn set_current_function(&mut self, name: EcoString) {
        self.current_function = name;
    }

    pub(super) fn set_type_parameters(
        &mut self,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) {
        self.type_parameters = type_parameters;
    }

    pub(super) fn type_parameters(&self) -> &super::type_parameter::TypeParameterScope {
        &self.type_parameters
    }

    pub(super) fn value_shape(&mut self, type_: &Type) -> ValueShape {
        ValueShape::from_gleam_in(type_, &mut self.type_parameters)
    }

    pub(super) fn value_shape_in_scope(&self, type_: &Type) -> ValueShape {
        let mut type_parameters = self.type_parameters.clone();
        ValueShape::from_gleam_in(type_, &mut type_parameters)
    }

    pub(super) fn value_type(&mut self, type_: &Type) -> ValueType {
        self.value_shape(type_).value_type()
    }

    pub(super) fn panic_site(&self, location: gleam_core::ast::SrcSpan) -> PanicSite {
        PanicSite::new(
            self.module_name.clone(),
            self.current_function.clone(),
            location.into(),
        )
    }

    pub(super) fn define_existing_local(&mut self, name: EcoString, local: LocalId) {
        match local {
            LocalId::Generic(local) => {
                self.next_generic_local = self.next_generic_local.max(local.id().0 + 1);
            }
            LocalId::Int(local) => {
                self.next_int_local = self.next_int_local.max(local.0 + 1);
            }
            LocalId::Float(local) => {
                self.next_float_local = self.next_float_local.max(local.0 + 1);
            }
            LocalId::String(local) => {
                self.next_string_local = self.next_string_local.max(local.0 + 1);
            }
            LocalId::BitArray(local) => {
                self.next_bit_array_local = self.next_bit_array_local.max(local.0 + 1);
            }
            LocalId::UtfCodepoint(local) => {
                self.next_utf_codepoint_local = self.next_utf_codepoint_local.max(local.0 + 1);
            }
            LocalId::Bool(local) => {
                self.next_bool_local = self.next_bool_local.max(local.0 + 1);
            }
            LocalId::Nil(local) => {
                self.next_nil_local = self.next_nil_local.max(local.0 + 1);
            }
        }
        self.bindings.insert(name, LocalBinding::Primitive(local));
    }

    pub(super) fn define_param_local_shape(
        &mut self,
        name: EcoString,
        shape: ValueShape,
    ) -> ParamLocal {
        match shape {
            ValueShape::Parameter(parameter) => {
                ParamLocal::generic(self.define_generic_local(name, parameter))
            }
            ValueShape::Int => ParamLocal::int(self.define_int_local(name)),
            ValueShape::Float => ParamLocal::float(self.define_float_local(name)),
            ValueShape::String => ParamLocal::string(self.define_string_local(name)),
            ValueShape::BitArray => ParamLocal::bit_array(self.define_bit_array_local(name)),
            ValueShape::UtfCodepoint => {
                ParamLocal::utf_codepoint(self.define_utf_codepoint_local(name))
            }
            ValueShape::Custom(shape) => {
                let local = self.define_custom_local_shape(name, shape.clone());
                ParamLocal::custom_shape(local, shape)
            }
            ValueShape::Bool => ParamLocal::bool(self.define_bool_local(name)),
            ValueShape::Nil => ParamLocal::nil(self.define_nil_local(name)),
            ValueShape::Tuple(shape) => {
                let type_ = shape.iter().map(ValueShape::value_type).collect::<Vec<_>>();
                ParamLocal::tuple(self.define_tuple_local_shape(name, shape), type_)
            }
            ValueShape::List(element) => {
                ParamLocal::list(self.define_list_local_shape(name, *element))
            }
            ValueShape::Function(shape) => {
                let shape = *shape;
                let type_ = shape.type_();
                let return_shape = shape.return_shape().clone();
                match return_shape {
                    ValueShape::Parameter(parameter) => {
                        let type_ = crate::plan::GenericFunctionType::new(
                            shape.argument_shapes().to_vec(),
                            parameter,
                        );
                        ParamLocal::generic_function(
                            self.define_generic_function_local_shape(name, type_, shape),
                        )
                    }
                    ValueShape::Int => ParamLocal::int_function(
                        self.define_int_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Float => ParamLocal::float_function(
                        self.define_float_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::String => ParamLocal::string_function(
                        self.define_string_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::BitArray => ParamLocal::bit_array_function(
                        self.define_bit_array_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::UtfCodepoint => ParamLocal::utf_codepoint_function(
                        self.define_utf_codepoint_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Custom(return_) => {
                        let type_ = CustomFunctionType::from_shapes(
                            shape.argument_shapes().to_vec(),
                            return_.clone(),
                        );
                        ParamLocal::custom_function(
                            self.define_custom_function_local_shape(name, type_, shape),
                        )
                    }
                    ValueShape::Bool => ParamLocal::bool_function(
                        self.define_bool_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Nil => ParamLocal::nil_function(
                        self.define_nil_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Tuple(_) => ParamLocal::tuple_function(
                        self.define_tuple_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::List(item_shape) => {
                        ParamLocal::list_function(self.define_list_function_local_shape(
                            name,
                            type_.clone(),
                            item_shape.value_type(),
                            shape,
                        ))
                    }
                    ValueShape::Function(return_) => {
                        let type_ = FunctionFunctionType::from_shapes(
                            shape.argument_shapes().to_vec(),
                            return_.as_ref().clone(),
                        );
                        ParamLocal::function_function(
                            self.define_function_function_local_shape(name, type_, shape),
                        )
                    }
                }
            }
        }
    }

    pub(super) fn define_existing_param(
        &mut self,
        name: EcoString,
        local: &ParamLocal,
        shape: ValueShape,
    ) -> Result<(), PlanError> {
        match (local, shape) {
            (ParamLocal::Generic(local), ValueShape::Parameter(parameter))
                if local.parameter() == parameter =>
            {
                self.define_existing_local(name, LocalId::Generic(*local));
            }
            (ParamLocal::Int(local), ValueShape::Int) => {
                self.define_existing_local(name, LocalId::Int(*local));
            }
            (ParamLocal::Float(local), ValueShape::Float) => {
                self.define_existing_local(name, LocalId::Float(*local));
            }
            (ParamLocal::String(local), ValueShape::String) => {
                self.define_existing_local(name, LocalId::String(*local));
            }
            (ParamLocal::BitArray(local), ValueShape::BitArray) => {
                self.define_existing_local(name, LocalId::BitArray(*local));
            }
            (ParamLocal::UtfCodepoint(local), ValueShape::UtfCodepoint) => {
                self.define_existing_local(name, LocalId::UtfCodepoint(*local));
            }
            (ParamLocal::Custom(local), ValueShape::Custom(shape)) if local.shape() == &shape => {
                self.next_custom_local = self.next_custom_local.max(local.id().0 + 1);
                self.bindings
                    .insert(name, LocalBinding::Custom(local.clone()));
            }
            (ParamLocal::Bool(local), ValueShape::Bool) => {
                self.define_existing_local(name, LocalId::Bool(*local));
            }
            (ParamLocal::Nil(local), ValueShape::Nil) => {
                self.define_existing_local(name, LocalId::Nil(*local));
            }
            (ParamLocal::Tuple { local, type_ }, ValueShape::Tuple(shape))
                if shape
                    .iter()
                    .map(ValueShape::value_type)
                    .eq(type_.iter().cloned()) =>
            {
                self.next_tuple_local = self.next_tuple_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Tuple {
                        local: *local,
                        shape,
                    },
                );
            }
            (ParamLocal::List(local), ValueShape::List(item_shape))
                if item_shape.value_type() == local.item_type() =>
            {
                self.bump_list_local(local);
                self.bindings.insert(
                    name,
                    LocalBinding::List {
                        local: local.clone(),
                        item_shape: *item_shape,
                    },
                );
            }
            (ParamLocal::IntFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_int_function_local = self.next_int_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Int {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::FloatFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_float_function_local = self.next_float_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Float {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::StringFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_string_function_local = self.next_string_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::String {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::BitArrayFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_bit_array_function_local =
                    self.next_bit_array_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::BitArray {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::UtfCodepointFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_utf_codepoint_function_local =
                    self.next_utf_codepoint_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::UtfCodepoint {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::CustomFunction(local), ValueShape::Function(shape))
                if shape.type_() == local.type_().to_function_type() =>
            {
                self.next_custom_function_local =
                    self.next_custom_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Custom(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::BoolFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_bool_function_local = self.next_bool_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Bool {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::NilFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_nil_function_local = self.next_nil_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Nil {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::TupleFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_tuple_function_local = self.next_tuple_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Tuple {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::ListFunction(local), ValueShape::Function(shape))
                if shape.type_() == *local.type_() =>
            {
                self.next_list_function_local =
                    self.next_list_function_local.max(local.index() + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::List(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::FunctionFunction(local), ValueShape::Function(shape))
                if shape.type_() == local.type_().to_function_type() =>
            {
                self.next_function_function_local =
                    self.next_function_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Function(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::GenericFunction(local), ValueShape::Function(shape))
                if local.type_().shape() == *shape =>
            {
                self.next_generic_function_local =
                    self.next_generic_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Generic(local.clone()),
                        shape: *shape,
                    },
                );
            }
            _ => return Err(invalid_local_shape()),
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn define_int_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> IntFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_int_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_int_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Int { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_int_function_local(&mut self) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_string_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> StringFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_string_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_string_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::String { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_string_function_local(&mut self) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_bit_array_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BitArrayFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_bit_array_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_bit_array_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::BitArray { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_bit_array_function_local(&mut self) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        local
    }

    pub(super) fn define_utf_codepoint_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> UtfCodepointFunctionLocalId {
        let local = UtfCodepointFunctionLocalId(self.next_utf_codepoint_function_local);
        self.next_utf_codepoint_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::UtfCodepoint { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_utf_codepoint_function_local(
        &mut self,
    ) -> UtfCodepointFunctionLocalId {
        let local = UtfCodepointFunctionLocalId(self.next_utf_codepoint_function_local);
        self.next_utf_codepoint_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_custom_function_local(
        &mut self,
        name: EcoString,
        type_: CustomFunctionType,
    ) -> CustomFunctionLocal {
        let shape = FunctionShape::new(
            type_.argument_shapes().to_vec(),
            ValueShape::Custom(type_.return_().clone()),
        );
        self.define_custom_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_custom_function_local_shape(
        &mut self,
        name: EcoString,
        type_: CustomFunctionType,
        shape: FunctionShape,
    ) -> CustomFunctionLocal {
        let local = CustomFunctionLocal::new(
            CustomFunctionLocalId(self.next_custom_function_local),
            type_,
        );
        self.next_custom_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Custom(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_custom_function_local(
        &mut self,
        type_: CustomFunctionType,
    ) -> CustomFunctionLocal {
        let local = CustomFunctionLocal::new(
            CustomFunctionLocalId(self.next_custom_function_local),
            type_,
        );
        self.next_custom_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_float_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FloatFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_float_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_float_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Float { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_float_function_local(&mut self) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_bool_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BoolFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_bool_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_bool_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Bool { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_bool_function_local(&mut self) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_nil_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> NilFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_nil_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_nil_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Nil { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_nil_function_local(&mut self) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_tuple_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> TupleFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_tuple_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_tuple_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Tuple { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_tuple_function_local(&mut self) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_list_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionLocal {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_list_function_local_shape(name, type_, item_type, shape)
    }

    pub(super) fn define_list_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        item_type: ValueType,
        shape: FunctionShape,
    ) -> ListFunctionLocal {
        let local =
            ListFunctionLocal::from_item_type(self.next_list_function_local, type_, item_type);
        self.next_list_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::List(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_list_function_local(
        &mut self,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionLocal {
        let local =
            ListFunctionLocal::from_item_type(self.next_list_function_local, type_, item_type);
        self.next_list_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_function_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionFunctionType,
    ) -> FunctionFunctionLocal {
        let shape = FunctionShape::from_function_type(type_.to_function_type());
        self.define_function_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_function_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionFunctionType,
        shape: FunctionShape,
    ) -> FunctionFunctionLocal {
        let local = FunctionFunctionLocal::new(
            FunctionFunctionLocalId(self.next_function_function_local),
            type_,
        );
        self.next_function_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Function(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_function_function_local(
        &mut self,
        type_: FunctionFunctionType,
    ) -> FunctionFunctionLocal {
        let local = FunctionFunctionLocal::new(
            FunctionFunctionLocalId(self.next_function_function_local),
            type_,
        );
        self.next_function_function_local += 1;
        local
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Int(local)));
        local
    }

    pub(super) fn define_generic_local(
        &mut self,
        name: EcoString,
        parameter: crate::plan::TypeParameterId,
    ) -> crate::plan::GenericLocal {
        let local = crate::plan::GenericLocal::new(
            crate::plan::GenericLocalId(self.next_generic_local),
            parameter,
        );
        self.next_generic_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Generic(local)));
        local
    }

    pub(super) fn define_internal_generic_local(
        &mut self,
        parameter: crate::plan::TypeParameterId,
    ) -> crate::plan::GenericLocal {
        let local = crate::plan::GenericLocal::new(
            crate::plan::GenericLocalId(self.next_generic_local),
            parameter,
        );
        self.next_generic_local += 1;
        local
    }

    pub(super) fn define_generic_function_local_shape(
        &mut self,
        name: EcoString,
        type_: crate::plan::GenericFunctionType,
        shape: FunctionShape,
    ) -> crate::plan::GenericFunctionLocal {
        let local = crate::plan::GenericFunctionLocal::new(
            crate::plan::GenericFunctionLocalId(self.next_generic_function_local),
            type_,
        );
        self.next_generic_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Generic(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_generic_function_local(
        &mut self,
        type_: crate::plan::GenericFunctionType,
    ) -> crate::plan::GenericFunctionLocal {
        let local = crate::plan::GenericFunctionLocal::new(
            crate::plan::GenericFunctionLocalId(self.next_generic_function_local),
            type_,
        );
        self.next_generic_function_local += 1;
        local
    }

    pub(super) fn define_internal_int_local(&mut self) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::String(local)));
        local
    }

    pub(super) fn define_internal_string_local(&mut self) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        local
    }

    pub(super) fn define_bit_array_local(&mut self, name: EcoString) -> BitArrayLocalId {
        let local = BitArrayLocalId(self.next_bit_array_local);
        self.next_bit_array_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::BitArray(local)));
        local
    }

    pub(super) fn define_internal_bit_array_local(&mut self) -> BitArrayLocalId {
        let local = BitArrayLocalId(self.next_bit_array_local);
        self.next_bit_array_local += 1;
        local
    }

    pub(super) fn define_utf_codepoint_local(&mut self, name: EcoString) -> UtfCodepointLocalId {
        let local = UtfCodepointLocalId(self.next_utf_codepoint_local);
        self.next_utf_codepoint_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::UtfCodepoint(local)));
        local
    }

    pub(super) fn define_internal_utf_codepoint_local(&mut self) -> UtfCodepointLocalId {
        let local = UtfCodepointLocalId(self.next_utf_codepoint_local);
        self.next_utf_codepoint_local += 1;
        local
    }

    pub(super) fn define_custom_local_shape(
        &mut self,
        name: EcoString,
        shape: CustomValueShape,
    ) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Custom(crate::plan::CustomLocal::from_shape(local, shape)),
        );
        local
    }

    pub(super) fn define_internal_custom_local(&mut self) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
        local
    }

    pub(super) fn define_float_local(&mut self, name: EcoString) -> FloatLocalId {
        let local = FloatLocalId(self.next_float_local);
        self.next_float_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Float(local)));
        local
    }

    pub(super) fn define_internal_float_local(&mut self) -> FloatLocalId {
        let local = FloatLocalId(self.next_float_local);
        self.next_float_local += 1;
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Bool(local)));
        local
    }

    pub(super) fn define_internal_bool_local(&mut self) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Nil(local)));
        local
    }

    pub(super) fn define_internal_nil_local(&mut self) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_tuple_local(
        &mut self,
        name: EcoString,
        type_: Vec<ValueType>,
    ) -> TupleLocalId {
        let shape = type_
            .into_iter()
            .map(ValueShape::from_value_type)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.define_tuple_local_shape(name, shape)
    }

    pub(super) fn define_tuple_local_shape(
        &mut self,
        name: EcoString,
        shape: Box<[ValueShape]>,
    ) -> TupleLocalId {
        let local = TupleLocalId(self.next_tuple_local);
        self.next_tuple_local += 1;
        self.bindings
            .insert(name, LocalBinding::Tuple { local, shape });
        local
    }

    pub(super) fn define_internal_tuple_local(&mut self) -> TupleLocalId {
        let local = TupleLocalId(self.next_tuple_local);
        self.next_tuple_local += 1;
        local
    }

    pub(super) fn define_list_local(
        &mut self,
        name: EcoString,
        element_type: ValueType,
    ) -> ListLocal {
        self.define_list_local_shape(name, ValueShape::from_value_type(element_type))
    }

    pub(super) fn define_list_local_shape(
        &mut self,
        name: EcoString,
        item_shape: ValueShape,
    ) -> ListLocal {
        let local = self.next_list_local(item_shape.value_type());
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        local
    }

    pub(super) fn define_list_value(
        &mut self,
        name: EcoString,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        let item_shape = value.item_shape().clone();
        let (local, value) = self.next_list_local_expr(value);
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        (local, value)
    }

    pub(super) fn define_internal_list_value(
        &mut self,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        self.next_list_local_expr(value)
    }

    fn define_list_capture_value(
        &mut self,
        name: EcoString,
        source: ListLocal,
        item_shape: ValueShape,
    ) -> PlannedCapture {
        let local = self.next_list_local(item_shape.value_type());
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        Self::planned_capture(
            ParamLocal::list(local),
            ParamLocal::list(source),
            ValueShape::List(Box::new(item_shape)),
        )
    }

    fn next_list_local(&mut self, element_type: ValueType) -> ListLocal {
        match element_type {
            ValueType::Parameter(parameter) => {
                let local = crate::plan::GenericListLocalId(self.next_generic_list_local);
                self.next_generic_list_local += 1;
                ListLocal::generic(local, parameter)
            }
            ValueType::Int => {
                let local = IntListLocalId(self.next_int_list_local);
                self.next_int_list_local += 1;
                ListLocal::int(local)
            }
            ValueType::String => {
                let local = StringListLocalId(self.next_string_list_local);
                self.next_string_list_local += 1;
                ListLocal::string(local)
            }
            ValueType::BitArray => {
                let local = BitArrayListLocalId(self.next_bit_array_list_local);
                self.next_bit_array_list_local += 1;
                ListLocal::bit_array(local)
            }
            ValueType::UtfCodepoint => {
                let local = UtfCodepointListLocalId(self.next_utf_codepoint_list_local);
                self.next_utf_codepoint_list_local += 1;
                ListLocal::utf_codepoint(local)
            }
            ValueType::Custom(item_type) => {
                let local = CustomListLocalId(self.next_custom_list_local);
                self.next_custom_list_local += 1;
                ListLocal::custom(local, item_type)
            }
            ValueType::Float => {
                let local = FloatListLocalId(self.next_float_list_local);
                self.next_float_list_local += 1;
                ListLocal::float(local)
            }
            ValueType::Bool => {
                let local = BoolListLocalId(self.next_bool_list_local);
                self.next_bool_list_local += 1;
                ListLocal::bool(local)
            }
            ValueType::Nil => {
                let local = NilListLocalId(self.next_nil_list_local);
                self.next_nil_list_local += 1;
                ListLocal::nil(local)
            }
            ValueType::Tuple(item_type) => {
                let local = TupleListLocalId(self.next_tuple_list_local);
                self.next_tuple_list_local += 1;
                ListLocal::tuple(local, item_type)
            }
            ValueType::List(item_type) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                ListLocal::list(local, *item_type)
            }
            ValueType::Function(item_type) => {
                let local = FunctionListLocalId(self.next_function_list_local);
                self.next_function_list_local += 1;
                ListLocal::function(local, *item_type)
            }
        }
    }

    fn next_list_local_expr(&mut self, value: ListExpr) -> (ListLocal, ListLocalExpr) {
        match value {
            ListExpr::Generic(value) => {
                let local = crate::plan::GenericListLocalId(self.next_generic_list_local);
                self.next_generic_list_local += 1;
                let parameter = value.item().parameter();
                (
                    ListLocal::generic(local, parameter),
                    ListLocalExpr::Generic {
                        local,
                        parameter,
                        value,
                    },
                )
            }
            ListExpr::ParameterList(value) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                let parameter = value.item().parameter();
                (
                    ListLocal::list(local, ValueType::Parameter(parameter)),
                    ListLocalExpr::ParameterList {
                        local,
                        parameter,
                        value,
                    },
                )
            }
            ListExpr::Int(value) => {
                let local = IntListLocalId(self.next_int_list_local);
                self.next_int_list_local += 1;
                (ListLocal::int(local), ListLocalExpr::Int { local, value })
            }
            ListExpr::String(value) => {
                let local = StringListLocalId(self.next_string_list_local);
                self.next_string_list_local += 1;
                (
                    ListLocal::string(local),
                    ListLocalExpr::String { local, value },
                )
            }
            ListExpr::BitArray(value) => {
                let local = BitArrayListLocalId(self.next_bit_array_list_local);
                self.next_bit_array_list_local += 1;
                (
                    ListLocal::bit_array(local),
                    ListLocalExpr::BitArray { local, value },
                )
            }
            ListExpr::UtfCodepoint(value) => {
                let local = UtfCodepointListLocalId(self.next_utf_codepoint_list_local);
                self.next_utf_codepoint_list_local += 1;
                (
                    ListLocal::utf_codepoint(local),
                    ListLocalExpr::UtfCodepoint { local, value },
                )
            }
            ListExpr::Custom(value) => {
                let local = CustomListLocalId(self.next_custom_list_local);
                self.next_custom_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::custom(local, item_type.clone()),
                    ListLocalExpr::Custom {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::Float(value) => {
                let local = FloatListLocalId(self.next_float_list_local);
                self.next_float_list_local += 1;
                (
                    ListLocal::float(local),
                    ListLocalExpr::Float { local, value },
                )
            }
            ListExpr::Bool(value) => {
                let local = BoolListLocalId(self.next_bool_list_local);
                self.next_bool_list_local += 1;
                (ListLocal::bool(local), ListLocalExpr::Bool { local, value })
            }
            ListExpr::Nil(value) => {
                let local = NilListLocalId(self.next_nil_list_local);
                self.next_nil_list_local += 1;
                (ListLocal::nil(local), ListLocalExpr::Nil { local, value })
            }
            ListExpr::Tuple(value) => {
                let local = TupleListLocalId(self.next_tuple_list_local);
                self.next_tuple_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::tuple(local, item_type.clone()),
                    ListLocalExpr::Tuple {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::List(value) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::list(local, item_type.as_ref().clone()),
                    ListLocalExpr::List {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::Function(value) => {
                let local = FunctionListLocalId(self.next_function_list_local);
                self.next_function_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::function(local, item_type.clone()),
                    ListLocalExpr::Function {
                        local,
                        item_type,
                        value,
                    },
                )
            }
        }
    }

    fn bump_list_local(&mut self, local: &ListLocal) {
        match local {
            ListLocal::Generic { local, .. } => {
                self.next_generic_list_local = self.next_generic_list_local.max(local.0 + 1);
            }
            ListLocal::Int(local) => {
                self.next_int_list_local = self.next_int_list_local.max(local.0 + 1);
            }
            ListLocal::String(local) => {
                self.next_string_list_local = self.next_string_list_local.max(local.0 + 1);
            }
            ListLocal::BitArray(local) => {
                self.next_bit_array_list_local = self.next_bit_array_list_local.max(local.0 + 1);
            }
            ListLocal::UtfCodepoint(local) => {
                self.next_utf_codepoint_list_local =
                    self.next_utf_codepoint_list_local.max(local.0 + 1);
            }
            ListLocal::Custom { local, .. } => {
                self.next_custom_list_local = self.next_custom_list_local.max(local.0 + 1);
            }
            ListLocal::Float(local) => {
                self.next_float_list_local = self.next_float_list_local.max(local.0 + 1);
            }
            ListLocal::Bool(local) => {
                self.next_bool_list_local = self.next_bool_list_local.max(local.0 + 1);
            }
            ListLocal::Nil(local) => {
                self.next_nil_list_local = self.next_nil_list_local.max(local.0 + 1);
            }
            ListLocal::Tuple { local, .. } => {
                self.next_tuple_list_local = self.next_tuple_list_local.max(local.0 + 1);
            }
            ListLocal::List { local, .. } => {
                self.next_list_list_local = self.next_list_list_local.max(local.0 + 1);
            }
            ListLocal::Function { local, .. } => {
                self.next_function_list_local = self.next_function_list_local.max(local.0 + 1);
            }
        }
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        match self.bindings.get(name)? {
            LocalBinding::Primitive(local) => Some((*local, local.value_type())),
            LocalBinding::Custom(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_custom_local(&self, name: &EcoString) -> Option<crate::plan::CustomLocal> {
        match self.bindings.get(name)? {
            LocalBinding::Custom(local) => Some(local.clone()),
            LocalBinding::Primitive(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_tuple_local(
        &self,
        name: &EcoString,
    ) -> Option<(TupleLocalId, Box<[ValueShape]>)> {
        match self.bindings.get(name)? {
            LocalBinding::Tuple { local, shape } => Some((*local, shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_list_local(&self, name: &EcoString) -> Option<(ListLocal, ValueShape)> {
        match self.bindings.get(name)? {
            LocalBinding::List { local, item_shape } => Some((local.clone(), item_shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_module_function(
        &self,
        module: &EcoString,
        name: &EcoString,
    ) -> Option<FunctionInfo> {
        match self.registry {
            RegistryAccess::Program { registry } => registry.function(module, name),
            #[cfg(test)]
            RegistryAccess::Local { functions, .. } if module == self.module_name => {
                functions.get(name).cloned()
            }
            #[cfg(test)]
            RegistryAccess::Local { .. } => None,
        }
    }

    pub(super) fn module_is_linked(&self, module: &EcoString) -> bool {
        match self.registry {
            RegistryAccess::Program { registry } => registry.module_id(module).is_some(),
            #[cfg(test)]
            RegistryAccess::Local { .. } => module == self.module_name,
        }
    }

    pub(super) fn module_constant_expr(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Option<crate::plan::Expr> {
        match self.registry {
            RegistryAccess::Program { registry } => registry.constant_expr(module, name, shape),
            #[cfg(test)]
            RegistryAccess::Local { .. } => None,
        }
    }

    pub(super) fn module_constant_instantiation(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Option<crate::plan::ConstantInstantiation> {
        match self.registry {
            RegistryAccess::Program { registry } => {
                registry.constant_instantiation(module, name, shape)
            }
            #[cfg(test)]
            RegistryAccess::Local { .. } => None,
        }
    }

    fn custom_type_definition(
        &self,
        name: &crate::plan::CustomTypeName,
    ) -> Option<&CustomTypeDefinition> {
        match self.registry {
            RegistryAccess::Program { registry } => registry.custom_type(name),
            #[cfg(test)]
            RegistryAccess::Local { custom_types, .. } => custom_types
                .iter()
                .find(|definition| definition.name() == name),
        }
    }

    pub(super) fn custom_constructor(
        &self,
        constructor: &ValueConstructor,
    ) -> Result<CustomConstructor, PlanError> {
        let ValueConstructorVariant::Record {
            name,
            module,
            variant_index,
            ..
        } = &constructor.variant
        else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::error::InvalidExpressionShapeKind::RecordConstructor,
                },
            });
        };

        self.custom_constructor_from_type(
            constructor.type_.as_ref(),
            name.clone(),
            module,
            usize::from(*variant_index),
        )
        .map(ResolvedCustomConstructor::into_constructor)
    }

    pub(super) fn module_custom_constructor(
        &self,
        type_: &Type,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
    ) -> Result<CustomConstructor, PlanError> {
        self.custom_constructor_from_type(type_, name, module, variant_index)
            .map(ResolvedCustomConstructor::into_constructor)
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
        let mut type_parameters = self.type_parameters.clone();
        let type_ = match ValueShape::from_gleam_in(return_type, &mut type_parameters) {
            ValueShape::Custom(shape) => shape.type_().clone(),
            _ => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name,
                        reason: crate::planner::error::InvalidCustomTypeReason::ConstructorType,
                    },
                });
            }
        };
        let field_types = match signature {
            Some((field_types, _)) => field_types,
            None => Vec::new(),
        };
        let field_types = field_types
            .into_iter()
            .map(|field| {
                ValueShape::from_gleam_in(field.as_ref(), &mut type_parameters).value_type()
            })
            .collect();

        self.custom_constructor_from_parts(type_, name, module, variant_index, field_types)
    }

    pub(super) fn custom_construction_shape(
        &self,
        construction: &crate::plan::CustomConstruction,
    ) -> Result<CustomValueShape, PlanError> {
        let constructor = construction.constructor();
        let fields = construction.fields();
        let type_ = constructor.type_();
        let templates = if type_.type_name().package().is_empty()
            && type_.type_name().module() == PRELUDE_MODULE_NAME
            && type_.type_name().name() == "Result"
        {
            match constructor.index() {
                0 => vec![CustomTypeTemplate::Parameter(
                    crate::plan::CustomTypeParameterId(0),
                )],
                1 => vec![CustomTypeTemplate::Parameter(
                    crate::plan::CustomTypeParameterId(1),
                )],
                _ => {
                    return Err(invalid_custom_type(
                        type_,
                        InvalidCustomTypeReason::ConstructorIndex,
                    ));
                }
            }
        } else {
            let definition = self
                .custom_type_definition(type_.type_name())
                .ok_or_else(|| {
                    invalid_custom_type(type_, InvalidCustomTypeReason::UnknownDefinition)
                })?;
            definition
                .constructor(constructor.index())
                .ok_or_else(|| {
                    invalid_custom_type(type_, InvalidCustomTypeReason::ConstructorIndex)
                })?
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect()
        };
        let mut arguments = vec![None; type_.arguments().len()];
        for (template, field) in templates.iter().zip(fields) {
            if instantiate_custom_type_template(template, type_)? != field.value_type() {
                return Err(invalid_custom_type(
                    type_,
                    InvalidCustomTypeReason::FieldType,
                ));
            }
            collect_parameter_shapes(template, field.value_shape(), &mut arguments, type_)?;
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

    pub(super) fn custom_expr_from_construction(
        &self,
        construction: crate::plan::CustomConstruction,
    ) -> Result<crate::plan::CustomExpr, PlanError> {
        let shape = self.custom_construction_shape(&construction)?;
        Ok(crate::plan::CustomExpr::from_construction(
            shape,
            construction,
        ))
    }

    pub(super) fn custom_pattern_constructor(
        &self,
        type_: &Type,
        constructor: &PatternConstructor,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        let mut type_parameters = self.type_parameters.clone();
        let type_ = match ValueShape::from_gleam_in(type_, &mut type_parameters) {
            ValueShape::Custom(shape) => shape.type_().clone(),
            _ => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: constructor.name.clone(),
                        reason: InvalidCustomTypeReason::ConstructorType,
                    },
                });
            }
        };
        self.custom_constructor_from_parts(
            type_,
            constructor.name.clone(),
            &constructor.module,
            usize::from(constructor.constructor_index),
            field_types,
        )
    }

    pub(super) fn custom_field_access(
        &self,
        source: CustomExpr,
        index: usize,
        label: Option<EcoString>,
        expected: &ValueType,
    ) -> Result<(CustomFieldAccess, ValueShape), PlanError> {
        let custom_type = source.type_();
        let custom_shape = source.shape();
        let Some(type_definition) = self.custom_type_definition(custom_type.type_name()) else {
            return Err(invalid_custom_type(
                custom_type,
                InvalidCustomTypeReason::UnknownDefinition,
            ));
        };
        if type_definition.parameters().len() != custom_type.arguments().len() {
            return Err(invalid_custom_type(
                custom_type,
                InvalidCustomTypeReason::TypeArgumentCount,
            ));
        }

        let constructors = match custom_shape.constructor() {
            crate::plan::CustomConstructorRefinement::Exact(index) => {
                vec![type_definition.constructor(index).ok_or_else(|| {
                    invalid_custom_type(custom_type, InvalidCustomTypeReason::ConstructorIndex)
                })?]
            }
            crate::plan::CustomConstructorRefinement::Any => {
                type_definition.constructors().iter().collect()
            }
        };
        let mut result_shape: Option<ValueShape> = None;
        for constructor in constructors {
            let field = constructor.fields().get(index).ok_or_else(|| {
                invalid_custom_type(custom_type, InvalidCustomTypeReason::FieldIndex)
            })?;
            if field.label() != label.as_ref() {
                return Err(invalid_custom_type(
                    custom_type,
                    InvalidCustomTypeReason::FieldLabel,
                ));
            }
            let field_shape = instantiate_custom_shape_template(field.type_(), custom_shape)?;
            let actual = field_shape.value_type();
            if &actual != expected {
                return Err(invalid_custom_type(
                    custom_type,
                    InvalidCustomTypeReason::FieldType,
                ));
            }
            result_shape = Some(match result_shape {
                Some(previous) => previous.merge(&field_shape).ok_or_else(|| {
                    invalid_custom_type(custom_type, InvalidCustomTypeReason::FieldType)
                })?,
                None => field_shape,
            });
            constructor.fields().iter().try_for_each(|field| {
                instantiate_custom_type_template(field.type_(), custom_type).map(|_| ())
            })?;
        }

        let Some(result_shape) = result_shape else {
            return Err(invalid_custom_type(
                custom_type,
                InvalidCustomTypeReason::FieldIndex,
            ));
        };
        Ok((CustomFieldAccess::new(source, index, label), result_shape))
    }

    fn custom_constructor_from_parts(
        &self,
        type_: CustomType,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        if module != type_.type_name().module() {
            return Err(invalid_custom_type(
                &type_,
                InvalidCustomTypeReason::ConstructorModule,
            ));
        }
        let (fields, constructor_count) = if type_.type_name().package().is_empty()
            && module == PRELUDE_MODULE_NAME
            && type_.type_name().module() == PRELUDE_MODULE_NAME
            && type_.type_name().name() == "Result"
        {
            let [ok, error] = type_.arguments() else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::TypeArgumentCount,
                ));
            };
            match variant_index {
                0 if name == "Ok" => (vec![(None, ok.clone())], 2),
                1 if name == "Error" => (vec![(None, error.clone())], 2),
                0 | 1 => {
                    return Err(invalid_custom_type(
                        &type_,
                        InvalidCustomTypeReason::ConstructorName,
                    ));
                }
                _ => {
                    return Err(invalid_custom_type(
                        &type_,
                        InvalidCustomTypeReason::ConstructorIndex,
                    ));
                }
            }
        } else {
            let Some(type_definition) = self.custom_type_definition(type_.type_name()) else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::UnknownDefinition,
                ));
            };
            if type_definition.parameters().len() != type_.arguments().len() {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::TypeArgumentCount,
                ));
            }
            let Some(constructor_definition) = type_definition.constructor(variant_index) else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorIndex,
                ));
            };
            if constructor_definition.name() != &name {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorName,
                ));
            };
            if constructor_definition.fields().len() != field_types.len() {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorArity,
                ));
            }
            let mut fields = Vec::with_capacity(constructor_definition.fields().len());
            for field in constructor_definition.fields() {
                fields.push((
                    field.label().cloned(),
                    instantiate_custom_type_template(field.type_(), &type_)?,
                ));
            }
            (fields, type_definition.constructors().len())
        };
        if fields.len() != field_types.len()
            || fields.iter().map(|(_, type_)| type_).ne(field_types.iter())
        {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: type_.type_name().name().clone(),
                    reason: InvalidCustomTypeReason::FieldType,
                },
            });
        }
        let fields = fields
            .into_iter()
            .map(|(label, type_)| CustomConstructorField::new(label, type_))
            .collect();

        Ok(ResolvedCustomConstructor {
            constructor: CustomConstructor::new(type_, name, variant_index, fields),
            constructor_count,
        })
    }

    pub(super) fn lookup_function_local(
        &self,
        name: &EcoString,
    ) -> Option<(FunctionLocalBinding, FunctionShape)> {
        match self.bindings.get(name)? {
            LocalBinding::Function { binding, shape } => Some((binding.clone(), shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. } => None,
        }
    }

    pub(super) fn anonymous_function_error_name(&self) -> EcoString {
        self.anonymous_functions.next_name()
    }

    pub(super) fn reserve_anonymous_function_name(&mut self) -> EcoString {
        self.anonymous_functions.reserve_name()
    }

    pub(super) fn allocate_anonymous_function_shape(
        &mut self,
        name: EcoString,
        return_shape: ValueShape,
        params: Vec<FunctionParam>,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> (EcoString, FunctionInfo) {
        self.anonymous_functions
            .allocate(name, return_shape, params, type_parameters)
    }

    pub(super) fn push_anonymous_function(&mut self, function: FunctionTemplate) {
        self.anonymous_functions.push(function);
    }

    pub(super) fn anonymous_function_context(
        &mut self,
        function_name: EcoString,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> PlanContext<'_> {
        PlanContext {
            module_name: self.module_name,
            current_function: function_name,
            registry: self.registry,
            anonymous_functions: self.anonymous_functions,
            bindings: HashMap::new(),
            next_generic_local: 0,
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_generic_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
            next_generic_function_local: 0,
            type_parameters,
        }
    }

    pub(super) fn capture_bindings(
        &self,
        names: &[EcoString],
    ) -> Result<Vec<CaptureBinding>, PlanError> {
        names
            .iter()
            .map(|name| {
                let Some(binding) = self.bindings.get(name).cloned() else {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                    });
                };

                Ok(CaptureBinding {
                    name: name.clone(),
                    binding,
                })
            })
            .collect()
    }

    pub(super) fn define_captures(&mut self, captures: Vec<CaptureBinding>) -> PlannedCaptures {
        let captures = captures
            .into_iter()
            .map(|capture| self.define_capture(capture))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        PlannedCaptures { captures }
    }

    pub(super) fn with_local_scope<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let bindings = self.bindings.clone();
        let result = f(self);
        self.bindings = bindings;
        result
    }

    fn define_capture(&mut self, capture: CaptureBinding) -> PlannedCapture {
        let name = capture.name;
        match capture.binding {
            LocalBinding::Primitive(LocalId::Generic(local)) => {
                let target = self.define_generic_local(name, local.parameter());
                Self::planned_capture(
                    ParamLocal::generic(target),
                    ParamLocal::generic(local),
                    ValueShape::Parameter(local.parameter()),
                )
            }
            LocalBinding::Primitive(LocalId::Int(local)) => {
                let target = self.define_int_local(name);
                Self::planned_capture(
                    ParamLocal::int(target),
                    ParamLocal::int(local),
                    ValueShape::Int,
                )
            }
            LocalBinding::Primitive(LocalId::Float(local)) => {
                let target = self.define_float_local(name);
                Self::planned_capture(
                    ParamLocal::float(target),
                    ParamLocal::float(local),
                    ValueShape::Float,
                )
            }
            LocalBinding::Primitive(LocalId::String(local)) => {
                let target = self.define_string_local(name);
                Self::planned_capture(
                    ParamLocal::string(target),
                    ParamLocal::string(local),
                    ValueShape::String,
                )
            }
            LocalBinding::Primitive(LocalId::BitArray(local)) => {
                let target = self.define_bit_array_local(name);
                Self::planned_capture(
                    ParamLocal::bit_array(target),
                    ParamLocal::bit_array(local),
                    ValueShape::BitArray,
                )
            }
            LocalBinding::Primitive(LocalId::UtfCodepoint(local)) => {
                let target = self.define_utf_codepoint_local(name);
                Self::planned_capture(
                    ParamLocal::utf_codepoint(target),
                    ParamLocal::utf_codepoint(local),
                    ValueShape::UtfCodepoint,
                )
            }
            LocalBinding::Custom(local) => {
                let shape = local.shape().clone();
                let target = self.define_custom_local_shape(name, shape.clone());
                Self::planned_capture(
                    ParamLocal::custom_shape(target, shape.clone()),
                    ParamLocal::Custom(local),
                    ValueShape::Custom(shape),
                )
            }
            LocalBinding::Primitive(LocalId::Bool(local)) => {
                let target = self.define_bool_local(name);
                Self::planned_capture(
                    ParamLocal::bool(target),
                    ParamLocal::bool(local),
                    ValueShape::Bool,
                )
            }
            LocalBinding::Primitive(LocalId::Nil(local)) => {
                let target = self.define_nil_local(name);
                Self::planned_capture(
                    ParamLocal::nil(target),
                    ParamLocal::nil(local),
                    ValueShape::Nil,
                )
            }
            LocalBinding::Tuple { local, shape } => {
                let type_: Vec<ValueType> = shape.iter().map(ValueShape::value_type).collect();
                let target = self.define_tuple_local_shape(name, shape.clone());
                Self::planned_capture(
                    ParamLocal::tuple(target, type_.clone()),
                    ParamLocal::tuple(local, type_),
                    ValueShape::Tuple(shape),
                )
            }
            LocalBinding::List { local, item_shape } => {
                self.define_list_capture_value(name, local, item_shape)
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Generic(local),
                shape,
            } => {
                let target = self.define_generic_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::generic_function(target),
                    ParamLocal::generic_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Int { local, type_ },
                shape,
            } => {
                let target =
                    self.define_int_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::int_function(target, type_.clone()),
                    ParamLocal::int_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Float { local, type_ },
                shape,
            } => {
                let target =
                    self.define_float_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::float_function(target, type_.clone()),
                    ParamLocal::float_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::String { local, type_ },
                shape,
            } => {
                let target =
                    self.define_string_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::string_function(target, type_.clone()),
                    ParamLocal::string_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::BitArray { local, type_ },
                shape,
            } => {
                let target =
                    self.define_bit_array_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::bit_array_function(target, type_.clone()),
                    ParamLocal::bit_array_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::UtfCodepoint { local, type_ },
                shape,
            } => {
                let target = self.define_utf_codepoint_function_local_shape(
                    name,
                    type_.clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::utf_codepoint_function(target, type_.clone()),
                    ParamLocal::utf_codepoint_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Custom(local),
                shape,
            } => {
                let target = self.define_custom_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::custom_function(target),
                    ParamLocal::custom_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Bool { local, type_ },
                shape,
            } => {
                let target =
                    self.define_bool_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::bool_function(target, type_.clone()),
                    ParamLocal::bool_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Nil { local, type_ },
                shape,
            } => {
                let target =
                    self.define_nil_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::nil_function(target, type_.clone()),
                    ParamLocal::nil_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Tuple { local, type_ },
                shape,
            } => {
                let target =
                    self.define_tuple_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::tuple_function(target, type_.clone()),
                    ParamLocal::tuple_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::List(local),
                shape,
            } => {
                let target = self.define_list_function_local_shape(
                    name,
                    local.type_().clone(),
                    local.item_type(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::list_function(target),
                    ParamLocal::list_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Function(local),
                shape,
            } => {
                let target = self.define_function_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::function_function(target),
                    ParamLocal::function_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
        }
    }

    fn planned_capture(local: ParamLocal, source: ParamLocal, shape: ValueShape) -> PlannedCapture {
        PlannedCapture {
            slot: ParamSlot::new(local, shape),
            source: CaptureArg::new(source),
        }
    }
}

pub(in crate::planner) struct AnonymousFunctions {
    module: crate::plan::ModuleId,
    next_function_index: usize,
    next_anonymous_index: usize,
    functions: Vec<FunctionTemplate>,
}

impl AnonymousFunctions {
    pub(in crate::planner) fn new(next_function_index: usize) -> Self {
        Self::in_module(crate::plan::ModuleId::root(), next_function_index)
    }

    pub(in crate::planner) fn in_module(
        module: crate::plan::ModuleId,
        next_function_index: usize,
    ) -> Self {
        Self {
            module,
            next_function_index,
            next_anonymous_index: 0,
            functions: Vec::new(),
        }
    }

    pub(in crate::planner) fn into_functions(mut self) -> Vec<FunctionTemplate> {
        self.functions.sort_by_key(|function| function.id().index());
        self.functions
    }

    fn next_name(&self) -> EcoString {
        format!("<anonymous:{}>", self.next_anonymous_index).into()
    }

    fn reserve_name(&mut self) -> EcoString {
        let name = self.next_name();
        self.next_anonymous_index += 1;
        name
    }

    fn allocate(
        &mut self,
        name: EcoString,
        return_shape: ValueShape,
        params: Vec<FunctionParam>,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> (EcoString, FunctionInfo) {
        let id = crate::plan::FunctionTemplateId::in_module(self.module, self.next_function_index);
        let shape = FunctionShape::new(
            params.iter().map(|param| param.shape().clone()).collect(),
            return_shape.clone(),
        );
        let info = FunctionInfo {
            signature: FunctionTemplateSignature::new(id, type_parameters.scheme(), shape),
            type_parameters,
            return_shape,
            params,
        };
        self.next_function_index += 1;
        (name, info)
    }

    fn push(&mut self, function: FunctionTemplate) {
        self.functions.push(function);
    }
}

impl Default for AnonymousFunctions {
    fn default() -> Self {
        Self::new(0)
    }
}

impl FunctionInfo {
    pub(super) fn arity(&self) -> usize {
        self.params.len()
    }

    pub(super) fn return_shape(&self) -> ValueShape {
        self.return_shape.clone()
    }

    pub(super) fn instantiate(
        &self,
        actual: &FunctionShape,
    ) -> Result<
        crate::plan::FunctionInstantiation,
        super::type_parameter::FunctionInstantiationMismatch,
    > {
        super::type_parameter::instantiate(&self.signature, actual)
    }

    pub(super) fn reference(
        &self,
        instantiation: crate::plan::FunctionInstantiation,
    ) -> FunctionReference {
        FunctionReference::new(instantiation)
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::FunctionLocalBinding;
    use super::{
        AnonymousFunctions, FunctionInfo, PlanContext, PlannedCapture, ResolvedCustomConstructor,
        collect_parameter_shapes, instantiate_custom_shape_template,
        instantiate_custom_type_template,
    };
    use crate::plan::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CaptureArg, CustomConstruction, CustomConstructor,
        CustomConstructorDefinition, CustomConstructorField, CustomConstructorRefinement,
        CustomFieldDefinition, CustomFunctionLocal, CustomFunctionLocalId, CustomFunctionType,
        CustomType, CustomTypeDefinition, CustomTypeName, CustomTypeParameterId,
        CustomTypePublicity, CustomTypeTemplate, CustomValueShape, Expr, FloatFunctionLocalId,
        FloatListLocalId, FloatLocalId, FunctionExpr, FunctionFunctionLocal,
        FunctionFunctionLocalId, FunctionFunctionType, FunctionListLocalId, FunctionReference,
        FunctionShape, FunctionType, GenericFunctionLocal, GenericFunctionLocalId,
        GenericFunctionType, GenericListLocalId, IntExpr, IntFunctionLocalId, IntListLocalId,
        IntLocalId, ListExpr, ListListLocalId, ListLocal, ListLocalExpr, LocalId,
        NilFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal, ParamSlot, StringExpr,
        StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
        TupleListLocalId, TupleLocalId, TypeParameterId, UtfCodepointListLocalId, ValueShape,
        ValueType,
    };
    use crate::planner::{InvalidCustomTypeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::Publicity;
    use gleam_core::type_::{
        self, Deprecation, PatternConstructor, ValueConstructor, ValueConstructorVariant,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn constant_lookup_without_a_registry_returns_none() {
        let module = EcoString::from("main");
        let mut anonymous = AnonymousFunctions::default();
        let (function_name, function) = anonymous.allocate(
            "present".into(),
            ValueShape::Int,
            Vec::new(),
            Default::default(),
        );
        let function_id = function.signature.id();
        let functions = HashMap::from([(function_name, function)]);
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context.module_constant_expr(&module, &EcoString::from("missing"), &ValueShape::Int,),
            None,
        );
        assert_eq!(
            context
                .lookup_module_function(&module, &EcoString::from("present"))
                .map(function_template_id),
            Some(function_id),
        );
        assert_eq!(
            context
                .lookup_module_function(&EcoString::from("other"), &EcoString::from("present"))
                .map(function_template_id),
            None,
        );
    }

    fn function_template_id(function: FunctionInfo) -> crate::plan::FunctionTemplateId {
        function.signature.id()
    }

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
        let expected = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Generic".into(),
                reason: InvalidCustomTypeReason::ParameterType,
            },
        };

        assert_eq!(
            instantiate_custom_shape_template(
                &CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                &owner_shape,
            ),
            Err(expected.clone()),
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
        ] {
            assert_eq!(
                instantiate_custom_shape_template(&template, &owner_shape),
                Err(expected.clone()),
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
            Err(expected.clone()),
        );
        let mut arguments = vec![Some(ValueShape::Int)];
        assert_eq!(
            collect_parameter_shapes(
                &CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                &ValueShape::String,
                &mut arguments,
                &owner,
            ),
            Err(expected.clone()),
        );

        let tuple = CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
            CustomTypeParameterId(0),
        )]);
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&tuple, &ValueShape::Int, &mut arguments, &owner),
            Err(expected.clone()),
        );
        assert_eq!(
            collect_parameter_shapes(
                &tuple,
                &ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
                &mut arguments,
                &owner,
            ),
            Err(expected.clone()),
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
        ] {
            let mut arguments = vec![None];
            assert_eq!(
                collect_parameter_shapes(&template, &shape, &mut arguments, &owner),
                Err(expected.clone()),
            );
        }

        let list = CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
            CustomTypeParameterId(0),
        )));
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&list, &ValueShape::Int, &mut arguments, &owner),
            Err(expected.clone()),
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

        let function = CustomTypeTemplate::Function {
            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
            return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(0))),
        };
        let mut arguments = vec![None];
        assert_eq!(
            collect_parameter_shapes(&function, &ValueShape::Int, &mut arguments, &owner),
            Err(expected.clone()),
        );
        assert_eq!(
            collect_parameter_shapes(
                &function,
                &ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int,))),
                &mut arguments,
                &owner,
            ),
            Err(expected.clone()),
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
            Err(expected.clone()),
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
            Err(expected.clone()),
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
            Vec::new(),
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
                InvalidCustomTypeReason::ConstructorIndex,
            )),
        );
        assert_eq!(
            context
                .custom_expr_from_construction(invalid_result)
                .map(|_| ()),
            Err(invalid_custom_constructor_error(
                &result,
                InvalidCustomTypeReason::ConstructorIndex,
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
                InvalidCustomTypeReason::UnknownDefinition,
            )),
        );

        let boxed = CustomType::new(boxed_name, Vec::new());
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
                InvalidCustomTypeReason::ConstructorIndex,
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
            Err(invalid_custom_field_error(&boxed)),
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
                InvalidCustomTypeReason::ParameterType,
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
                    CustomConstructorField::new(None, ValueType::Function(Box::new(function_type))),
                ],
            ),
            vec![field(0, 0), field(1, 1)],
        )
        .expect("test construction has exact descriptor arity");
        assert_eq!(
            context.custom_construction_shape(&conflicting_fields),
            Err(invalid_custom_constructor_error(
                &repeated,
                InvalidCustomTypeReason::ParameterType,
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
            Err(invalid_custom_field_error(&generic_int)),
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
                InvalidCustomTypeReason::ConstructorName,
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
                InvalidCustomTypeReason::ConstructorModule,
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
                InvalidCustomTypeReason::ConstructorArity,
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
                InvalidCustomTypeReason::ConstructorIndex,
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
                    name: "Missing".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
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
                InvalidCustomTypeReason::TypeArgumentCount,
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
            Err(invalid_custom_field_error(&result_type)),
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
                InvalidCustomTypeReason::ConstructorName,
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
                InvalidCustomTypeReason::ConstructorIndex,
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
                InvalidCustomTypeReason::TypeArgumentCount,
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
                InvalidCustomTypeReason::UnknownDefinition,
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
                    name: "Broken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );

        let missing_parameter = CustomType::new(generic_name, Vec::new());
        let parameter_error = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Generic".into(),
                reason: InvalidCustomTypeReason::ParameterType,
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
                    name: "Result".into(),
                    reason: InvalidCustomTypeReason::FieldType,
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
                    name: "Ok".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
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
                    name: "Invalid".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_eq!(
            context.custom_pattern_constructor(type_::int().as_ref(), &pattern, Vec::new()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Invalid".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
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
                name: type_.type_name().name().clone(),
                reason,
            },
        }
    }

    fn invalid_custom_field_error(type_: &CustomType) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: type_.type_name().name().clone(),
                reason: InvalidCustomTypeReason::FieldType,
            },
        }
    }

    #[test]
    fn local_scope_restores_names_after_error_without_reusing_ids() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(context.define_int_local("x".into()), IntLocalId(0));
        let result = context.with_local_scope(|context| {
            assert_eq!(context.define_int_local("x".into()), IntLocalId(1));
            Err::<(), _>(())
        });

        assert_eq!(result, Err(()));
        assert_eq!(
            context.lookup_local(&"x".into()),
            Some((LocalId::Int(IntLocalId(0)), ValueType::Int))
        );
        assert_eq!(context.define_int_local("y".into()), IntLocalId(2));
    }

    #[test]
    fn define_function_local_records_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = int_function_type();

        let local = context.define_int_function_local("f".into(), type_.clone());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Int {
                    local,
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_),
            ))
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn generic_function_params_preserve_parameter_shape_and_local_sequence() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let shape = FunctionShape::new(
            vec![ValueShape::Parameter(parameter)],
            ValueShape::Parameter(parameter),
        );
        let type_ = GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter);
        let local = GenericFunctionLocal::new(GenericFunctionLocalId(0), type_.clone());

        assert_eq!(
            context.define_param_local_shape(
                "function".into(),
                ValueShape::Function(Box::new(shape.clone())),
            ),
            ParamLocal::generic_function(local.clone()),
        );
        assert_eq!(
            context.lookup_function_local(&"function".into()),
            Some((FunctionLocalBinding::Generic(local), shape.clone())),
        );

        let existing = GenericFunctionLocal::new(GenericFunctionLocalId(4), type_.clone());
        context
            .define_existing_param(
                "existing".into(),
                &ParamLocal::generic_function(existing.clone()),
                ValueShape::Function(Box::new(shape.clone())),
            )
            .unwrap();
        assert_eq!(
            context.lookup_function_local(&"existing".into()),
            Some((FunctionLocalBinding::Generic(existing), shape.clone())),
        );
        assert_eq!(
            context.define_generic_function_local_shape("next".into(), type_, shape),
            GenericFunctionLocal::new(
                GenericFunctionLocalId(5),
                GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter,),
            ),
        );
    }

    #[test]
    fn param_local_shapes_preserve_remaining_primitive_and_function_families() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context.define_param_local_shape("float".into(), ValueShape::Float),
            ParamLocal::float(FloatLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("string".into(), ValueShape::String),
            ParamLocal::string(StringLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("bool".into(), ValueShape::Bool),
            ParamLocal::bool(BoolLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("nil".into(), ValueShape::Nil),
            ParamLocal::nil(NilLocalId(0)),
        );

        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        assert_eq!(
            context.define_param_local_shape(
                "float_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    float_type.clone(),
                ))),
            ),
            ParamLocal::float_function(FloatFunctionLocalId(0), float_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "string_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    string_type.clone(),
                ))),
            ),
            ParamLocal::string_function(StringFunctionLocalId(0), string_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "bool_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    bool_type.clone(),
                ))),
            ),
            ParamLocal::bool_function(BoolFunctionLocalId(0), bool_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "nil_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    nil_type.clone(),
                ))),
            ),
            ParamLocal::nil_function(NilFunctionLocalId(0), nil_type),
        );
    }

    #[test]
    fn define_existing_param_records_tuple_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]));

        context
            .define_existing_param(
                "f".into(),
                &ParamLocal::tuple_function(TupleFunctionLocalId(2), type_.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(type_.clone()))),
            )
            .unwrap();

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Tuple {
                    local: TupleFunctionLocalId(2),
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_.clone()),
            )),
        );
        assert_eq!(context.define_tuple_function_local("g".into(), type_).0, 3);
    }

    #[test]
    fn define_existing_param_records_tuple_and_function_family_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bit_array_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type());
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let function_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));

        context
            .define_existing_param(
                "tuple".into(),
                &ParamLocal::tuple(TupleLocalId(2), tuple_type.clone()),
                ValueShape::from_value_type(ValueType::Tuple(tuple_type.clone())),
            )
            .unwrap();
        context
            .define_existing_param(
                "string_fn".into(),
                &ParamLocal::string_function(StringFunctionLocalId(3), string_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(string_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "bit_array_fn".into(),
                &ParamLocal::bit_array_function(BitArrayFunctionLocalId(4), bit_array_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(bit_array_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "custom_fn".into(),
                &ParamLocal::custom_function(CustomFunctionLocal::new(
                    CustomFunctionLocalId(8),
                    custom_function_type.clone(),
                )),
                ValueShape::from_value_type(ValueType::Function(Box::new(
                    custom_function_type.to_function_type(),
                ))),
            )
            .unwrap();
        context
            .define_existing_param(
                "float_fn".into(),
                &ParamLocal::float_function(FloatFunctionLocalId(4), float_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(float_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "bool_fn".into(),
                &ParamLocal::bool_function(BoolFunctionLocalId(5), bool_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(bool_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "nil_fn".into(),
                &ParamLocal::nil_function(NilFunctionLocalId(6), nil_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(nil_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "function_fn".into(),
                &ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(7),
                    function_type.clone(),
                )),
                ValueShape::from_value_type(ValueType::Function(Box::new(
                    function_type.to_function_type(),
                ))),
            )
            .unwrap();

        assert_eq!(
            context.lookup_tuple_local(&"tuple".into()),
            Some((TupleLocalId(2), vec![ValueShape::Int].into_boxed_slice(),)),
        );
        assert_eq!(
            context.lookup_function_local(&"string_fn".into()),
            Some((
                FunctionLocalBinding::String {
                    local: StringFunctionLocalId(3),
                    type_: string_type.clone(),
                },
                FunctionShape::from_function_type(string_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"bit_array_fn".into()),
            Some((
                FunctionLocalBinding::BitArray {
                    local: BitArrayFunctionLocalId(4),
                    type_: bit_array_type.clone(),
                },
                FunctionShape::from_function_type(bit_array_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"custom_fn".into()),
            Some((
                FunctionLocalBinding::Custom(CustomFunctionLocal::new(
                    CustomFunctionLocalId(8),
                    custom_function_type.clone(),
                )),
                FunctionShape::from_function_type(custom_function_type.to_function_type()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"float_fn".into()),
            Some((
                FunctionLocalBinding::Float {
                    local: FloatFunctionLocalId(4),
                    type_: float_type.clone(),
                },
                FunctionShape::from_function_type(float_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"bool_fn".into()),
            Some((
                FunctionLocalBinding::Bool {
                    local: BoolFunctionLocalId(5),
                    type_: bool_type.clone(),
                },
                FunctionShape::from_function_type(bool_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"nil_fn".into()),
            Some((
                FunctionLocalBinding::Nil {
                    local: NilFunctionLocalId(6),
                    type_: nil_type.clone(),
                },
                FunctionShape::from_function_type(nil_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"function_fn".into()),
            Some((
                FunctionLocalBinding::Function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(7),
                    function_type.clone(),
                )),
                FunctionShape::from_function_type(function_type.to_function_type()),
            )),
        );

        assert_eq!(
            context
                .define_tuple_local("next_tuple".into(), tuple_type)
                .0,
            3
        );
        assert_eq!(
            context
                .define_string_function_local("next_string_fn".into(), string_type)
                .0,
            4,
        );
        assert_eq!(
            context
                .define_bit_array_function_local("next_bit_array_fn".into(), bit_array_type)
                .0,
            5,
        );
        assert_eq!(
            context
                .define_custom_function_local("next_custom_fn".into(), custom_function_type)
                .id(),
            CustomFunctionLocalId(9),
        );
        assert_eq!(
            context.define_internal_custom_function_local(CustomFunctionType::new(
                Vec::new(),
                custom_type(),
            )),
            CustomFunctionLocal::new(
                CustomFunctionLocalId(10),
                CustomFunctionType::new(Vec::new(), custom_type()),
            ),
        );
        assert_eq!(
            context
                .define_float_function_local("next_float_fn".into(), float_type)
                .0,
            5,
        );
        assert_eq!(
            context
                .define_bool_function_local("next_bool_fn".into(), bool_type)
                .0,
            6,
        );
        assert_eq!(
            context
                .define_nil_function_local("next_nil_fn".into(), nil_type)
                .0,
            7,
        );
        assert_eq!(
            context
                .define_function_function_local("next_function_fn".into(), function_type)
                .id(),
            FunctionFunctionLocalId(8),
        );
    }

    #[test]
    fn define_internal_tuple_local_reserves_id_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];

        assert_eq!(context.define_internal_tuple_local(), TupleLocalId(0));
        assert_eq!(context.lookup_tuple_local(&"<tuple:0>".into()), None);
        assert_eq!(context.lookup_tuple_local(&"<case:tuple:0>".into()), None);
        assert_eq!(
            context.define_tuple_local("tuple".into(), tuple_type.clone()),
            TupleLocalId(1),
        );
        assert_eq!(
            context.lookup_tuple_local(&"tuple".into()),
            Some((TupleLocalId(1), vec![ValueShape::Int].into_boxed_slice())),
        );
    }

    #[test]
    fn define_internal_list_local_reserves_id_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context
                .define_internal_list_value(ListExpr::value(Vec::new(), ValueType::Int))
                .0,
            ListLocal::int(IntListLocalId(0))
        );
        assert_eq!(context.lookup_list_local(&"<list:int:0>".into()), None);
        assert_eq!(context.lookup_list_local(&"<case:list:int:0>".into()), None);
        assert_eq!(
            context.define_list_local("values".into(), ValueType::Int),
            ListLocal::int(IntListLocalId(1)),
        );
        assert_eq!(
            context.lookup_list_local(&"values".into()),
            Some((ListLocal::int(IntListLocalId(1)), ValueShape::Int)),
        );
    }

    #[test]
    fn define_list_capture_value_preserves_every_item_family() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let nested_item_type = Box::new(ValueType::String);
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Bool);

        assert_eq!(
            context.define_list_capture_value(
                "ints".into(),
                ListLocal::int(IntListLocalId(9)),
                ValueShape::Int,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::int(IntListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::int(IntListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "strings".into(),
                ListLocal::string(StringListLocalId(9)),
                ValueShape::String,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::string(
                    StringListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::string(StringListLocalId(9),))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "bit_arrays".into(),
                ListLocal::bit_array(BitArrayListLocalId(9)),
                ValueShape::BitArray,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::bit_array(
                    BitArrayListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::bit_array(
                    BitArrayListLocalId(9),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "utf_codepoints".into(),
                ListLocal::utf_codepoint(UtfCodepointListLocalId(9)),
                ValueShape::UtfCodepoint,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::utf_codepoint(
                    UtfCodepointListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::utf_codepoint(
                    UtfCodepointListLocalId(9),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "floats".into(),
                ListLocal::float(FloatListLocalId(9)),
                ValueShape::Float,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::float(FloatListLocalId(
                    0
                ),))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::float(FloatListLocalId(9,)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "bools".into(),
                ListLocal::bool(BoolListLocalId(9)),
                ValueShape::Bool,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::bool(BoolListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::bool(BoolListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "nils".into(),
                ListLocal::nil(NilListLocalId(9)),
                ValueShape::Nil,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::nil(NilListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::nil(NilListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "tuples".into(),
                ListLocal::tuple(TupleListLocalId(9), tuple_type.clone()),
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::tuple(
                    TupleListLocalId(0),
                    tuple_type.clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::tuple(
                    TupleListLocalId(9),
                    tuple_type.clone(),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "lists".into(),
                ListLocal::list(ListListLocalId(9), nested_item_type.as_ref().clone()),
                ValueShape::List(Box::new(ValueShape::String)),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::list(
                    ListListLocalId(0),
                    nested_item_type.as_ref().clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::list(
                    ListListLocalId(9),
                    nested_item_type.as_ref().clone(),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "functions".into(),
                ListLocal::function(FunctionListLocalId(9), function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    function_type.clone(),
                ))),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(0),
                    function_type.clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(9),
                    function_type.clone(),
                ))),
            },
        );

        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some((
                ListLocal::function(FunctionListLocalId(0), function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(function_type,))),
            )),
        );
    }

    #[test]
    fn define_list_locals_preserve_item_family_boundaries() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int, ValueType::String];
        let nested_function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let parameter = TypeParameterId(0);

        assert_eq!(
            context.define_list_local("generic".into(), ValueType::Parameter(parameter)),
            ListLocal::generic(GenericListLocalId(0), parameter),
        );

        assert_eq!(
            context.define_list_local("strings".into(), ValueType::String),
            ListLocal::string(StringListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("bit_arrays".into(), ValueType::BitArray),
            ListLocal::bit_array(BitArrayListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("floats".into(), ValueType::Float),
            ListLocal::float(FloatListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("bools".into(), ValueType::Bool),
            ListLocal::bool(BoolListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("nils".into(), ValueType::Nil),
            ListLocal::nil(NilListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("tuples".into(), ValueType::Tuple(tuple_type.clone())),
            ListLocal::tuple(TupleListLocalId(0), tuple_type.clone()),
        );
        assert_eq!(
            context.define_list_local("lists".into(), ValueType::List(Box::new(ValueType::Float)),),
            ListLocal::list(ListListLocalId(0), ValueType::Float),
        );
        assert_eq!(
            context.define_list_local(
                "functions".into(),
                ValueType::Function(Box::new(nested_function_type.clone())),
            ),
            ListLocal::function(FunctionListLocalId(0), nested_function_type.clone()),
        );

        assert_eq!(
            context.lookup_list_local(&"tuples".into()),
            Some((
                ListLocal::tuple(TupleListLocalId(0), tuple_type),
                ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
            )),
        );
        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some((
                ListLocal::function(FunctionListLocalId(0), nested_function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    nested_function_type,
                ))),
            )),
        );
        assert_eq!(
            context.lookup_list_local(&"generic".into()),
            Some((
                ListLocal::generic(GenericListLocalId(0), parameter),
                ValueShape::Parameter(parameter),
            )),
        );
    }

    #[test]
    fn define_generic_list_value_preserves_parameter_and_typed_local_expression() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let value = ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty generic list has a sealed item parameter");
        let typed_value = value
            .clone()
            .into_generic()
            .expect("a parameter item list has generic list storage");

        assert_eq!(
            context.define_list_value("values".into(), value),
            (
                ListLocal::generic(GenericListLocalId(0), parameter),
                ListLocalExpr::Generic {
                    local: GenericListLocalId(0),
                    parameter,
                    value: typed_value,
                },
            ),
        );
        assert_eq!(
            context.lookup_list_local(&"values".into()),
            Some((
                ListLocal::generic(GenericListLocalId(0), parameter),
                ValueShape::Parameter(parameter),
            )),
        );
    }

    #[test]
    fn define_existing_list_params_bump_their_own_item_family() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let nested_function_type = FunctionType::new(Vec::new(), ValueType::Int);

        context
            .define_existing_param(
                "strings".into(),
                &ParamLocal::list(ListLocal::string(StringListLocalId(2))),
                ValueShape::List(Box::new(ValueShape::String)),
            )
            .unwrap();
        context
            .define_existing_param(
                "bit_arrays".into(),
                &ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(2))),
                ValueShape::List(Box::new(ValueShape::BitArray)),
            )
            .unwrap();
        context
            .define_existing_param(
                "floats".into(),
                &ParamLocal::list(ListLocal::float(FloatListLocalId(3))),
                ValueShape::List(Box::new(ValueShape::Float)),
            )
            .unwrap();
        context
            .define_existing_param(
                "bools".into(),
                &ParamLocal::list(ListLocal::bool(BoolListLocalId(4))),
                ValueShape::List(Box::new(ValueShape::Bool)),
            )
            .unwrap();
        context
            .define_existing_param(
                "nils".into(),
                &ParamLocal::list(ListLocal::nil(NilListLocalId(5))),
                ValueShape::List(Box::new(ValueShape::Nil)),
            )
            .unwrap();
        context
            .define_existing_param(
                "tuples".into(),
                &ParamLocal::list(ListLocal::tuple(TupleListLocalId(6), tuple_type.clone())),
                ValueShape::List(Box::new(ValueShape::Tuple(
                    vec![ValueShape::Int].into_boxed_slice(),
                ))),
            )
            .unwrap();
        context
            .define_existing_param(
                "lists".into(),
                &ParamLocal::list(ListLocal::list(ListListLocalId(7), ValueType::Int)),
                ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::Int)))),
            )
            .unwrap();
        context
            .define_existing_param(
                "functions".into(),
                &ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(8),
                    nested_function_type.clone(),
                )),
                ValueShape::List(Box::new(ValueShape::Function(Box::new(
                    FunctionShape::from_function_type(nested_function_type.clone()),
                )))),
            )
            .unwrap();

        assert_eq!(
            context.define_list_local("next_string".into(), ValueType::String),
            ListLocal::string(StringListLocalId(3)),
        );
        assert_eq!(
            context.define_list_local("next_bit_array".into(), ValueType::BitArray),
            ListLocal::bit_array(BitArrayListLocalId(3)),
        );
        assert_eq!(
            context.define_list_local("next_float".into(), ValueType::Float),
            ListLocal::float(FloatListLocalId(4)),
        );
        assert_eq!(
            context.define_list_local("next_bool".into(), ValueType::Bool),
            ListLocal::bool(BoolListLocalId(5)),
        );
        assert_eq!(
            context.define_list_local("next_nil".into(), ValueType::Nil),
            ListLocal::nil(NilListLocalId(6)),
        );
        assert_eq!(
            context.define_list_local("next_tuple".into(), ValueType::Tuple(tuple_type)),
            ListLocal::tuple(TupleListLocalId(7), vec![ValueType::Int]),
        );
        assert_eq!(
            context.define_list_local(
                "next_list".into(),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            ListLocal::list(ListListLocalId(8), ValueType::Int),
        );
        assert_eq!(
            context.define_list_local(
                "next_function".into(),
                ValueType::Function(Box::new(nested_function_type.clone())),
            ),
            ListLocal::function(FunctionListLocalId(9), nested_function_type),
        );
    }

    #[test]
    fn define_internal_primitive_locals_reserve_ids_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(context.define_internal_int_local(), IntLocalId(0));
        assert_eq!(context.define_internal_string_local(), StringLocalId(0));
        assert_eq!(
            context.define_internal_bit_array_local(),
            BitArrayLocalId(0),
        );
        assert_eq!(context.define_internal_float_local(), FloatLocalId(0));
        assert_eq!(context.define_internal_bool_local(), BoolLocalId(0));
        assert_eq!(context.define_internal_nil_local(), NilLocalId(0));
        assert_eq!(context.lookup_local(&"<case:int:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:string:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:bit_array:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:float:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:bool:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:nil:0>".into()), None);

        assert_eq!(context.define_int_local("int".into()), IntLocalId(1));
        assert_eq!(
            context.define_string_local("string".into()),
            StringLocalId(1),
        );
        assert_eq!(
            context.define_bit_array_local("bit_array".into()),
            BitArrayLocalId(1),
        );
        assert_eq!(context.define_float_local("float".into()), FloatLocalId(1));
        assert_eq!(context.define_bool_local("bool".into()), BoolLocalId(1));
        assert_eq!(context.define_nil_local("nil".into()), NilLocalId(1));
    }

    #[test]
    fn define_internal_function_locals_reserve_ids_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            context.define_internal_int_function_local(),
            IntFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_string_function_local(),
            StringFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_bit_array_function_local(),
            BitArrayFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_float_function_local(),
            FloatFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_bool_function_local(),
            BoolFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_nil_function_local(),
            NilFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_tuple_function_local(),
            TupleFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_list_function_local(
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int
            ),
            crate::plan::ListFunctionLocal::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                ),
                crate::plan::ValueType::Int,
            ),
        );
        assert_eq!(
            context.define_internal_function_function_local(FunctionFunctionType::new(
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            )),
            FunctionFunctionLocal::new(
                FunctionFunctionLocalId(0),
                FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                ),
            ),
        );
        assert_eq!(
            context.lookup_function_local(&"<case:int_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:string_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:bit_array_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:float_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:bool_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:nil_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:tuple_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:list_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:function_function:0>".into()),
            None,
        );

        assert_eq!(
            context.define_int_function_local("f".into(), type_),
            IntFunctionLocalId(1),
        );
    }

    #[test]
    fn function_local_shadows_primitive_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = int_function_type();

        context.define_int_local("f".into());
        context.define_int_function_local("f".into(), type_.clone());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Int {
                    local: IntFunctionLocalId(0),
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_),
            ))
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn primitive_binding_shadows_function_local() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        context.define_int_function_local("f".into(), int_function_type());
        let local = context.define_int_local("f".into());

        assert_eq!(context.lookup_function_local(&"f".into()), None);
        assert_eq!(
            context.lookup_local(&"f".into()),
            Some((LocalId::Int(local), ValueType::Int))
        );
    }

    #[test]
    fn define_captures_records_float_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Float], ValueType::Float);

        context.define_float_function_local("f".into(), type_.clone());
        let captures = context.capture_bindings(&[EcoString::from("f")]).unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![ParamSlot::from_local(ParamLocal::float_function(
                FloatFunctionLocalId(1),
                type_.clone(),
            ))],
        );
        assert_eq!(
            sources,
            vec![CaptureArg::new(ParamLocal::float_function(
                FloatFunctionLocalId(0),
                type_,
            ))],
        );
    }

    #[test]
    fn define_captures_preserves_bit_array_and_nil_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        context.define_bit_array_local("bits".into());
        context.define_nil_local("nil".into());
        let captures = context
            .capture_bindings(&["bits".into(), "nil".into()])
            .expect("defined locals should be capturable");
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::bit_array(BitArrayLocalId(1))),
                ParamSlot::from_local(ParamLocal::nil(NilLocalId(1))),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::bit_array(BitArrayLocalId(0))),
                CaptureArg::new(ParamLocal::nil(NilLocalId(0))),
            ],
        );
    }

    #[test]
    fn define_captures_records_tuple_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]));

        context.define_tuple_function_local("f".into(), type_.clone());
        let captures = context.capture_bindings(&[EcoString::from("f")]).unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![ParamSlot::from_local(ParamLocal::tuple_function(
                TupleFunctionLocalId(1),
                type_.clone(),
            ))],
        );
        assert_eq!(
            sources,
            vec![CaptureArg::new(ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                type_,
            ))],
        );
    }

    #[test]
    fn define_captures_records_list_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let element_type = ValueType::Int;
        let function_type = FunctionType::new(
            vec![ValueType::List(Box::new(element_type.clone()))],
            ValueType::List(Box::new(element_type.clone())),
        );

        context.define_list_local("values".into(), element_type.clone());
        context.define_list_function_local("f".into(), function_type.clone(), element_type);
        let captures = context
            .capture_bindings(&[EcoString::from("values"), EcoString::from("f")])
            .unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::list(ListLocal::int(IntListLocalId(1)))),
                ParamSlot::from_local(ParamLocal::list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        1,
                        function_type.clone(),
                        ValueType::Int,
                    ),
                )),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::list(ListLocal::int(IntListLocalId(0)))),
                CaptureArg::new(ParamLocal::list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        function_type,
                        ValueType::Int,
                    ),
                )),
            ],
        );
    }

    #[test]
    fn define_captures_records_remaining_function_families() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bit_array_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let function_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));

        context.define_string_function_local("string_fn".into(), string_type.clone());
        context.define_bit_array_function_local("bit_array_fn".into(), bit_array_type.clone());
        context.define_bool_function_local("bool_fn".into(), bool_type.clone());
        context.define_nil_function_local("nil_fn".into(), nil_type.clone());
        context.define_function_function_local("function_fn".into(), function_type.clone());
        let captures = context
            .capture_bindings(&[
                "string_fn".into(),
                "bit_array_fn".into(),
                "bool_fn".into(),
                "nil_fn".into(),
                "function_fn".into(),
            ])
            .unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::string_function(
                    StringFunctionLocalId(1),
                    string_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::bit_array_function(
                    BitArrayFunctionLocalId(1),
                    bit_array_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::bool_function(
                    BoolFunctionLocalId(1),
                    bool_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::nil_function(
                    NilFunctionLocalId(1),
                    nil_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(1),
                    function_type.clone()
                ),)),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::string_function(
                    StringFunctionLocalId(0),
                    string_type,
                )),
                CaptureArg::new(ParamLocal::bit_array_function(
                    BitArrayFunctionLocalId(0),
                    bit_array_type,
                )),
                CaptureArg::new(ParamLocal::bool_function(BoolFunctionLocalId(0), bool_type,)),
                CaptureArg::new(ParamLocal::nil_function(NilFunctionLocalId(0), nil_type)),
                CaptureArg::new(ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    function_type,
                ))),
            ],
        );
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Int)
    }
}
