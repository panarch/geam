use super::super::LoweringContext;
use super::super::specialization::{
    FunctionRepresentation, RepresentationContext, SpecializationKey, SpecializedCustomConstructor,
    SpecializedCustomConstructorField, SpecializedCustomValueShape, SpecializedFunctionShape,
    SpecializedTypeSubstitution, SpecializedValueShape,
};
use crate::host::{HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
use crate::plan::execution::host::HostSpecializationError;
use crate::plan::{
    CustomConstructorRefinement, CustomTypeName, FunctionType, HostFunctionTemplate,
};
use ecow::EcoString;
use std::collections::{HashMap, HashSet};

pub(super) fn seal_callbacks(
    template: &HostFunctionTemplate,
    key: &SpecializationKey,
    shape: &SpecializedFunctionShape,
    representations: &RepresentationContext,
    include_return: bool,
) -> Result<(), HostSpecializationError> {
    if let Some(callback) = first_uninhabited_callback(
        template,
        key.substitution(),
        representations,
        include_return,
    ) {
        return Err(HostSpecializationError::uninhabited_callback_arguments(
            template.package().clone(),
            template.site().module().clone(),
            template.site().function().clone(),
            shape.to_module_shape().type_(),
            callback,
        ));
    }
    Ok(())
}

pub(super) fn seal_custom_return_constructors(
    template: &HostFunctionTemplate,
    key: &SpecializationKey,
    context: &mut LoweringContext,
) {
    let HostTypeDescriptor::Custom { schema, arguments } = template.return_type() else {
        return;
    };
    let schemas = template
        .custom_schemas()
        .iter()
        .map(|schema| (identity(schema), schema))
        .collect::<HashMap<_, _>>();
    let type_ = SpecializedCustomValueShape::new(
        CustomTypeName::new(
            schema.package().clone(),
            schema.module().clone(),
            schema.name().clone(),
        ),
        arguments
            .iter()
            .map(|argument| {
                SpecializedValueShape::instantiate(&argument.value_shape(), key.substitution())
            })
            .collect(),
        CustomConstructorRefinement::Any,
    );

    for (index, constructor) in schema.constructors().iter().enumerate() {
        let fields = constructor
            .fields()
            .iter()
            .map(|field| {
                let descriptor = schema_descriptor(&schemas, field.type_(), arguments);
                SpecializedCustomConstructorField::new(
                    field.label().cloned(),
                    SpecializedValueShape::instantiate(
                        &descriptor.value_shape(),
                        key.substitution(),
                    ),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        context
            .types
            .custom_constructor(SpecializedCustomConstructor::new(
                type_.clone(),
                constructor.name().clone(),
                index,
                fields,
            ));
    }
}

fn first_uninhabited_callback(
    template: &HostFunctionTemplate,
    substitution: &SpecializedTypeSubstitution,
    representations: &RepresentationContext,
    include_return: bool,
) -> Option<FunctionType> {
    let schemas = template
        .custom_schemas()
        .iter()
        .map(|schema| (identity(schema), schema))
        .collect();
    let mut search = CallbackSearch {
        substitution,
        representations,
        schemas,
        visiting: HashSet::new(),
    };

    for parameter in template.parameters() {
        if let Some(callback) = search.find(parameter) {
            return Some(callback);
        }
    }
    if include_return {
        return search.find(template.return_type());
    }
    None
}

struct CallbackSearch<'a> {
    substitution: &'a SpecializedTypeSubstitution,
    representations: &'a RepresentationContext,
    schemas: HashMap<CustomIdentity, &'a HostCustomTypeSchema>,
    visiting: HashSet<SpecializedCustomValueShape>,
}

type CustomIdentity = (EcoString, EcoString, EcoString);

impl CallbackSearch<'_> {
    fn find(&mut self, descriptor: &HostTypeDescriptor) -> Option<FunctionType> {
        match descriptor {
            HostTypeDescriptor::Parameter(_)
            | HostTypeDescriptor::Int
            | HostTypeDescriptor::Float
            | HostTypeDescriptor::String
            | HostTypeDescriptor::BitArray
            | HostTypeDescriptor::UtfCodepoint
            | HostTypeDescriptor::Bool
            | HostTypeDescriptor::Nil
            | HostTypeDescriptor::External { .. } => None,
            HostTypeDescriptor::List(item) => {
                let item_shape =
                    SpecializedValueShape::instantiate(&item.value_shape(), self.substitution);
                self.representations
                    .is_inhabited(&item_shape)
                    .then(|| self.find(item))
                    .flatten()
            }
            HostTypeDescriptor::Tuple(elements) => {
                for element in elements {
                    if let Some(callback) = self.find(element) {
                        return Some(callback);
                    }
                }
                None
            }
            HostTypeDescriptor::Function { arguments, return_ } => {
                let function = SpecializedFunctionShape::new(
                    arguments
                        .iter()
                        .map(|argument| {
                            SpecializedValueShape::instantiate(
                                &argument.value_shape(),
                                self.substitution,
                            )
                        })
                        .collect(),
                    SpecializedValueShape::instantiate(&return_.value_shape(), self.substitution),
                );
                match function.representation(self.representations) {
                    FunctionRepresentation::Symbolic => Some(function.to_module_shape().type_()),
                    FunctionRepresentation::Never(_) => None,
                    FunctionRepresentation::Executable(_) => self.find(return_),
                }
            }
            HostTypeDescriptor::Custom { schema, arguments } => self.find_custom(schema, arguments),
        }
    }

    fn find_custom(
        &mut self,
        schema: &HostCustomTypeSchema,
        arguments: &[HostTypeDescriptor],
    ) -> Option<FunctionType> {
        let concrete = SpecializedCustomValueShape::new(
            CustomTypeName::new(
                schema.package().clone(),
                schema.module().clone(),
                schema.name().clone(),
            ),
            arguments
                .iter()
                .map(|argument| {
                    SpecializedValueShape::instantiate(&argument.value_shape(), self.substitution)
                })
                .collect(),
            CustomConstructorRefinement::Any,
        );
        if !self.visiting.insert(concrete.clone()) {
            return None;
        }

        for constructor in schema.constructors() {
            let fields = constructor
                .fields()
                .iter()
                .map(|field| schema_descriptor(&self.schemas, field.type_(), arguments))
                .collect::<Vec<_>>();
            if fields.iter().all(|field| {
                let shape =
                    SpecializedValueShape::instantiate(&field.value_shape(), self.substitution);
                self.representations.is_inhabited(&shape)
            }) {
                for field in &fields {
                    if let Some(callback) = self.find(field) {
                        self.visiting.remove(&concrete);
                        return Some(callback);
                    }
                }
            }
        }

        self.visiting.remove(&concrete);
        None
    }
}

fn schema_descriptor(
    schemas: &HashMap<CustomIdentity, &HostCustomTypeSchema>,
    type_: &HostSchemaType,
    arguments: &[HostTypeDescriptor],
) -> HostTypeDescriptor {
    match type_ {
        HostSchemaType::Parameter(index) => arguments[*index].clone(),
        HostSchemaType::Int => HostTypeDescriptor::Int,
        HostSchemaType::Float => HostTypeDescriptor::Float,
        HostSchemaType::String => HostTypeDescriptor::String,
        HostSchemaType::BitArray => HostTypeDescriptor::BitArray,
        HostSchemaType::UtfCodepoint => HostTypeDescriptor::UtfCodepoint,
        HostSchemaType::Bool => HostTypeDescriptor::Bool,
        HostSchemaType::Nil => HostTypeDescriptor::Nil,
        HostSchemaType::List(item) => {
            HostTypeDescriptor::List(Box::new(schema_descriptor(schemas, item, arguments)))
        }
        HostSchemaType::Tuple(elements) => HostTypeDescriptor::Tuple(
            elements
                .iter()
                .map(|element| schema_descriptor(schemas, element, arguments))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        HostSchemaType::Function {
            arguments: function_arguments,
            return_,
        } => HostTypeDescriptor::Function {
            arguments: function_arguments
                .iter()
                .map(|argument| schema_descriptor(schemas, argument, arguments))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            return_: Box::new(schema_descriptor(schemas, return_, arguments)),
        },
        HostSchemaType::Custom {
            package,
            module,
            name,
            arguments: custom_arguments,
        } => {
            let identity = (package.clone(), module.clone(), name.clone());
            HostTypeDescriptor::Custom {
                schema: schemas[&identity].clone(),
                arguments: custom_arguments
                    .iter()
                    .map(|argument| schema_descriptor(schemas, argument, arguments))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            }
        }
        HostSchemaType::External {
            schema,
            arguments: external_arguments,
        } => HostTypeDescriptor::External {
            schema: schema.clone(),
            arguments: external_arguments
                .iter()
                .map(|argument| schema_descriptor(schemas, argument, arguments))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        },
    }
}

fn identity(schema: &HostCustomTypeSchema) -> CustomIdentity {
    (
        schema.package().clone(),
        schema.module().clone(),
        schema.name().clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::{CallbackSearch, schema_descriptor};
    use crate::host::test::StatelessTestProvider;
    use crate::host::{
        HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
        HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
        HostCustomIndex0, HostCustomSchema, HostCustomType, HostExternalTypeSchema, HostSchemaType,
        HostTypeDescriptor, StatelessHostProfile,
    };
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializedTypeSubstitution,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostModule, HostProviderModule,
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };
    use num_bigint::BigInt;
    use std::collections::{HashMap, HashSet};

    struct OutputSchema;

    struct OutputDefinition;

    struct ValueField;

    impl HostCustomField for ValueField {
        const LABEL: Option<&'static str> = None;

        type Type = BigInt;
    }

    impl HostCustomConstructorDefinition for OutputDefinition {
        const NAME: &'static str = "Output";

        type Fields = HostCustomFieldList<ValueField, HostCustomFieldListEnd>;
    }

    impl HostCustomSchema for OutputSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Output";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<OutputDefinition, HostCustomConstructorListEnd>;
    }

    type Output = HostCustomType<OutputSchema>;
    type OutputConstructor = HostCustomConstructorAt<Output, HostCustomIndex0, OutputDefinition>;

    #[test]
    fn resolves_external_schema_arguments_inside_custom_fields() {
        let schema = HostExternalTypeSchema::new("domain", "domain/resource", "Resource", 2);
        let substitution = SpecializedTypeSubstitution::empty();
        let representations = RepresentationContext::new(Vec::new());
        let search = CallbackSearch {
            substitution: &substitution,
            representations: &representations,
            schemas: HashMap::new(),
            visiting: HashSet::new(),
        };
        let source = HostSchemaType::External {
            schema: schema.clone(),
            arguments: vec![
                HostSchemaType::Parameter(0),
                HostSchemaType::List(Box::new(HostSchemaType::Int)),
            ]
            .into_boxed_slice(),
        };

        assert_eq!(
            schema_descriptor(&search.schemas, &source, &[HostTypeDescriptor::String]),
            HostTypeDescriptor::External {
                schema,
                arguments: vec![
                    HostTypeDescriptor::String,
                    HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int)),
                ]
                .into_boxed_slice(),
            },
        );
    }

    #[test]
    fn seals_host_custom_return_constructors_without_source_constructor_use() {
        fn output(
            call: HostCall<'_, StatelessHostProfile, StatelessTestProvider, Output>,
        ) -> Result<HostCallCompletion<'_, Output>, HostCallError> {
            Ok(call.return_custom::<OutputConstructor>((BigInt::from(7), ())))
        }

        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<StatelessTestProvider, (), Output, _>("output", output)
            .expect("custom return provider should be valid");
        let source = r#"
pub type Output {
  Output(Int)
}

@external(erlang, "host", "output")
fn output() -> Output

pub fn main() {
  output()
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("source should compile");
        let plan = plan_host_program(typed).expect("source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("host execution should seal");

        let value = execution
            .run_main(&mut (), &mut Vec::new())
            .expect("custom host return should run");

        assert_eq!(value.inspect().to_string(), "Output(7)");
    }
}
