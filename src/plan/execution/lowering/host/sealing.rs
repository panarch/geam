use super::super::LoweringContext;
use super::super::specialization::{
    FunctionRepresentation, RepresentationContext, SpecializationKey, SpecializedCustomConstructor,
    SpecializedCustomConstructorField, SpecializedCustomValueShape, SpecializedFunctionShape,
    SpecializedTypeSubstitution, SpecializedValueShape,
};
use crate::host::{HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
use crate::plan::execution::host::{HostConstructionTypes, HostSpecializationError};
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

pub(super) fn seal_host_types(
    template: &HostFunctionTemplate,
    key: &SpecializationKey,
    context: &mut LoweringContext,
) -> HostConstructionTypes {
    let schemas = template
        .custom_schemas()
        .iter()
        .map(|schema| (identity(schema), schema))
        .collect::<HashMap<_, _>>();
    let mut sealing = HostTypeSealing {
        substitution: key.substitution(),
        schemas,
        visiting: HashSet::new(),
        lists: HashMap::new(),
        customs: HashMap::new(),
        externals: HashMap::new(),
        context,
    };

    for descriptor in template.parameters() {
        sealing.seal(descriptor);
    }
    sealing.seal(template.return_type());
    sealing.finish()
}

struct HostTypeSealing<'a, 'context> {
    substitution: &'a SpecializedTypeSubstitution,
    schemas: HashMap<CustomIdentity, &'a HostCustomTypeSchema>,
    visiting: HashSet<SpecializedCustomValueShape>,
    lists: HashMap<crate::plan::ValueType, crate::plan::execution::type_::ListTypeId>,
    customs: HashMap<crate::plan::ValueType, crate::plan::execution::type_::CustomTypeId>,
    externals: HashMap<crate::plan::ValueType, crate::plan::execution::type_::ExternalTypeId>,
    context: &'context mut LoweringContext,
}

impl HostTypeSealing<'_, '_> {
    fn seal(&mut self, descriptor: &HostTypeDescriptor) {
        let shape =
            SpecializedValueShape::instantiate(&descriptor.value_shape(), self.substitution);
        let nominal = shape.to_module_shape().value_type();
        match self.context.lower_concrete_value_type(&shape) {
            crate::plan::execution::type_::ValueType::List(id) => {
                self.lists.entry(nominal).or_insert(id);
            }
            crate::plan::execution::type_::ValueType::Custom(id) => {
                self.customs.entry(nominal).or_insert(id);
            }
            crate::plan::execution::type_::ValueType::External(id) => {
                self.externals.entry(nominal).or_insert(id);
            }
            crate::plan::execution::type_::ValueType::Parameter(_)
            | crate::plan::execution::type_::ValueType::Int
            | crate::plan::execution::type_::ValueType::Float
            | crate::plan::execution::type_::ValueType::String
            | crate::plan::execution::type_::ValueType::BitArray
            | crate::plan::execution::type_::ValueType::UtfCodepoint
            | crate::plan::execution::type_::ValueType::Bool
            | crate::plan::execution::type_::ValueType::Nil
            | crate::plan::execution::type_::ValueType::Tuple(_)
            | crate::plan::execution::type_::ValueType::Function(_) => {}
        }

        match descriptor {
            HostTypeDescriptor::List(item) => self.seal(item),
            HostTypeDescriptor::Tuple(elements) => {
                for element in elements {
                    self.seal(element);
                }
            }
            HostTypeDescriptor::Function { arguments, return_ } => {
                for argument in arguments {
                    self.seal(argument);
                }
                self.seal(return_);
            }
            HostTypeDescriptor::Custom { schema, arguments } => {
                self.seal_custom(schema, arguments);
            }
            HostTypeDescriptor::External { arguments, .. } => {
                for argument in arguments {
                    self.seal(argument);
                }
            }
            HostTypeDescriptor::Parameter(_)
            | HostTypeDescriptor::OpaqueFunction { .. }
            | HostTypeDescriptor::Int
            | HostTypeDescriptor::Float
            | HostTypeDescriptor::String
            | HostTypeDescriptor::BitArray
            | HostTypeDescriptor::UtfCodepoint
            | HostTypeDescriptor::Bool
            | HostTypeDescriptor::Nil => {}
        }
    }

    fn seal_custom(&mut self, schema: &HostCustomTypeSchema, arguments: &[HostTypeDescriptor]) {
        for argument in arguments {
            self.seal(argument);
        }
        let type_ = SpecializedCustomValueShape::new(
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
        if self.visiting.insert(type_.clone()) {
            for (index, constructor) in schema.constructors().iter().enumerate() {
                let fields = constructor
                    .fields()
                    .iter()
                    .map(|field| {
                        (
                            field.label().cloned(),
                            schema_descriptor(&self.schemas, field.type_(), arguments),
                        )
                    })
                    .collect::<Vec<_>>();
                self.context
                    .types
                    .custom_constructor(SpecializedCustomConstructor::new(
                        type_.clone(),
                        constructor.name().clone(),
                        index,
                        fields
                            .iter()
                            .map(|(label, descriptor)| {
                                SpecializedCustomConstructorField::new(
                                    label.clone(),
                                    SpecializedValueShape::instantiate(
                                        &descriptor.value_shape(),
                                        self.substitution,
                                    ),
                                )
                            })
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    ));
                for (_, field) in &fields {
                    self.seal(field);
                }
            }

            self.visiting.remove(&type_);
        }
    }

    fn finish(self) -> HostConstructionTypes {
        HostConstructionTypes::new(self.lists, self.customs, self.externals)
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
            | HostTypeDescriptor::External { .. }
            | HostTypeDescriptor::OpaqueFunction { .. } => None,
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
    use super::{CallbackSearch, identity, schema_descriptor};
    use crate::host::test::StatelessTestProvider;
    use crate::host::{
        HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
        HostCustomConstructorListEnd, HostCustomConstructorSchema, HostCustomField,
        HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema, HostCustomIndex0,
        HostCustomSchema, HostCustomType, HostCustomTypeSchema, HostExternalTypeSchema,
        HostFunctionType, HostList, HostListType, HostSchemaType, HostTypeDescriptor, HostTypeList,
        HostTypeListEnd, HostTypeParameter, StatelessHostProfile,
    };
    use crate::plan::TypeParameterId;
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializedTypeSubstitution,
    };
    use crate::{
        FunctionType, HostCall, HostCallCompletion, HostCallError, HostModule, HostProviderModule,
        HostProviderSet, HostSpecializationErrorReason, HostedExecution, ModuleSource,
        PackageSource, Value, ValueType, compile_typed_host_program, plan_host_program,
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

    struct RecursiveOutputSchema;

    struct RecursiveOutputDefinition;

    struct RecursiveChildrenField;

    impl HostCustomField for RecursiveChildrenField {
        const LABEL: Option<&'static str> = None;

        type Type = HostListType<RecursiveOutput>;
    }

    impl HostCustomConstructorDefinition for RecursiveOutputDefinition {
        const NAME: &'static str = "RecursiveOutput";

        type Fields = HostCustomFieldList<RecursiveChildrenField, HostCustomFieldListEnd>;
    }

    impl HostCustomSchema for RecursiveOutputSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "RecursiveOutput";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<RecursiveOutputDefinition, HostCustomConstructorListEnd>;
    }

    type RecursiveOutput = HostCustomType<RecursiveOutputSchema>;
    type RecursiveOutputConstructor =
        HostCustomConstructorAt<RecursiveOutput, HostCustomIndex0, RecursiveOutputDefinition>;

    #[test]
    fn rejects_a_symbolic_callback_nested_in_a_host_list() {
        type GenericArgument = HostTypeParameter<0>;
        type CallbackArguments = HostTypeList<GenericArgument, HostTypeListEnd>;
        type Callback = HostFunctionType<CallbackArguments, BigInt>;
        type CallbackList = HostListType<Callback>;

        fn accept_list<'call>(
            call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, BigInt>,
            _values: HostList<'call, Callback>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            Ok(call.return_value(BigInt::from(1)))
        }

        let valid_provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<StatelessTestProvider, (CallbackList,), BigInt, _>(
                "accept_list",
                accept_list,
            )
            .expect("callback list should register");
        let valid_source = r#"
@external(erlang, "host", "accept_list")
fn accept_list(values: List(fn(value) -> Int)) -> Int

fn concrete(value: Int) {
  value
}

pub fn main() {
  accept_list([concrete])
}
"#;
        let valid_typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", valid_source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [valid_provider])
                .expect("provider module should be unique"),
        )
        .expect("concrete callback list source should compile");
        let valid_plan =
            plan_host_program(valid_typed).expect("concrete callback list source should plan");
        let valid_execution = HostedExecution::try_from_module_plan(valid_plan)
            .expect("concrete callback list execution should seal");

        assert_eq!(
            valid_execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Int(BigInt::from(1))),
        );

        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<StatelessTestProvider, (CallbackList,), BigInt, _>(
                "accept_list",
                accept_list,
            )
            .expect("callback list should register");
        let source = r#"
@external(erlang, "host", "accept_list")
fn accept_list(values: List(fn(value) -> Int)) -> Int

fn generic(_value) {
  1
}

pub fn main() {
  accept_list([generic])
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
        .expect("symbolic callback list source should compile");
        let plan = plan_host_program(typed).expect("symbolic callback list source should plan");
        let error = HostedExecution::try_from_module_plan(plan)
            .err()
            .expect("symbolic callback list should fail sealing");

        assert_eq!(error.function(), "accept_list");
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                callback: FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(0))],
                    ValueType::Int,
                ),
            },
        );
    }

    #[test]
    fn finds_symbolic_callbacks_nested_in_tuples_and_custom_fields() {
        let substitution = SpecializedTypeSubstitution::empty();
        let representations = RepresentationContext::new(Vec::new());
        let callback = HostTypeDescriptor::Function {
            arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            return_: Box::new(HostTypeDescriptor::Int),
        };
        let expected = FunctionType::new(
            vec![ValueType::Parameter(TypeParameterId(0))],
            ValueType::Int,
        );
        let mut tuple_search = CallbackSearch {
            substitution: &substitution,
            representations: &representations,
            schemas: HashMap::new(),
            visiting: HashSet::new(),
        };

        assert_eq!(
            tuple_search.find(&HostTypeDescriptor::Tuple(
                vec![HostTypeDescriptor::Bool, callback.clone()].into_boxed_slice(),
            )),
            Some(expected.clone()),
        );
        assert_eq!(
            tuple_search.find(&HostTypeDescriptor::Function {
                arguments: vec![HostTypeDescriptor::Int].into_boxed_slice(),
                return_: Box::new(HostTypeDescriptor::Parameter(0)),
            }),
            None,
        );

        let schema = HostCustomTypeSchema::new(
            "application",
            "main",
            "CallbackContainer",
            1,
            [HostCustomConstructorSchema::new(
                "CallbackContainer",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::Function {
                        arguments: vec![HostSchemaType::Parameter(0)].into_boxed_slice(),
                        return_: Box::new(HostSchemaType::Int),
                    },
                )],
            )],
        );
        let mut custom_search = CallbackSearch {
            substitution: &substitution,
            representations: &representations,
            schemas: HashMap::from([(identity(&schema), &schema)]),
            visiting: HashSet::new(),
        };

        assert_eq!(
            custom_search.find(&HostTypeDescriptor::Custom {
                schema: schema.clone(),
                arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            }),
            Some(expected),
        );
        let plain_schema = HostCustomTypeSchema::new(
            "application",
            "main",
            "PlainContainer",
            0,
            [HostCustomConstructorSchema::new(
                "PlainContainer",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::Bool,
                )],
            )],
        );
        custom_search
            .schemas
            .insert(identity(&plain_schema), &plain_schema);

        assert_eq!(
            custom_search.find(&HostTypeDescriptor::Custom {
                schema: plain_schema.clone(),
                arguments: Vec::new().into_boxed_slice(),
            }),
            None,
        );

        let uninhabited_schema = HostCustomTypeSchema::new(
            "application",
            "main",
            "UninhabitedContainer",
            1,
            [HostCustomConstructorSchema::new(
                "UninhabitedContainer",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::Parameter(0),
                )],
            )],
        );
        custom_search
            .schemas
            .insert(identity(&uninhabited_schema), &uninhabited_schema);

        assert_eq!(
            custom_search.find(&HostTypeDescriptor::Custom {
                schema: uninhabited_schema.clone(),
                arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
            }),
            None,
        );
    }

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

    #[test]
    fn seals_recursive_host_construction_types_once() {
        fn output(
            mut call: HostCall<'_, StatelessHostProfile, StatelessTestProvider, RecursiveOutput>,
        ) -> Result<HostCallCompletion<'_, RecursiveOutput>, HostCallError> {
            let children = call.create_list::<RecursiveOutput>([]);
            Ok(call.return_custom::<RecursiveOutputConstructor>((children, ())))
        }

        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_scoped_function::<StatelessTestProvider, (), RecursiveOutput, _>("output", output)
            .expect("recursive custom return provider should be valid");
        let source = r#"
pub type RecursiveOutput {
  RecursiveOutput(List(RecursiveOutput))
}

@external(erlang, "host", "output")
fn output() -> RecursiveOutput

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
            .expect("recursive custom host return should run");

        assert_eq!(value.inspect().to_string(), "RecursiveOutput([])");
    }
}
