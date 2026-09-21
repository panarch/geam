use super::{ContractError, Registration, Types};
use crate::host::{HostCustomTypeSchema, HostTypeDescriptor};
use crate::plan::ValueType;
use crate::plan::execution::type_::TypeMetadata;
use ecow::EcoString;
use std::collections::HashMap;

pub(super) fn admit(
    registration: &Registration,
    arguments: &[crate::plan::execution::host::HostTypeArgument],
    types: &Types<'_>,
    returns_value: bool,
) -> Result<(), ContractError> {
    let schema = &registration.schema;
    let inhabited_arguments = arguments
        .iter()
        .map(|argument| {
            types
                .shape(argument.shape)
                .map(|shape| types.inhabited(shape))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(ContractError::Type)?;
    let mut search = Callbacks {
        arguments: arguments.len(),
        inhabited_arguments: &inhabited_arguments,
        types,
        schemas: schema
            .custom_schemas()
            .iter()
            .map(|schema| {
                (
                    (
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    schema,
                )
            })
            .collect(),
        active: Vec::new(),
    };
    for parameter in schema.parameters().iter().chain(schema.captures()) {
        search.check(parameter)?;
    }
    if returns_value {
        search.check(schema.return_type())?;
    }
    Ok(())
}

struct Callbacks<'a, 'data> {
    arguments: usize,
    inhabited_arguments: &'a [bool],
    types: &'a Types<'data>,
    schemas: HashMap<(EcoString, EcoString, EcoString), &'a HostCustomTypeSchema>,
    active: Vec<HostTypeDescriptor>,
}

impl Callbacks<'_, '_> {
    fn inhabited(&self, descriptor: &HostTypeDescriptor) -> Result<bool, ContractError> {
        let template = descriptor
            .resolve(&|index| {
                (index < self.arguments)
                    .then_some(ValueType::Parameter(crate::plan::TypeParameterId(index)))
            })
            .ok_or(ContractError::TypeArguments)?;
        self.types
            .metadata_inhabited(
                &TypeMetadata::from_public(&template),
                self.inhabited_arguments,
            )
            .map_err(ContractError::Type)
    }

    fn check(&mut self, descriptor: &HostTypeDescriptor) -> Result<(), ContractError> {
        match descriptor {
            HostTypeDescriptor::Parameter(_)
            | HostTypeDescriptor::Int
            | HostTypeDescriptor::Float
            | HostTypeDescriptor::String
            | HostTypeDescriptor::BitArray
            | HostTypeDescriptor::UtfCodepoint
            | HostTypeDescriptor::Bool
            | HostTypeDescriptor::Nil
            | HostTypeDescriptor::OpaqueFunction { .. }
            | HostTypeDescriptor::External { .. } => {}
            HostTypeDescriptor::List(item) => {
                if self.inhabited(item)? {
                    self.check(item)?;
                }
            }
            HostTypeDescriptor::Tuple(items) => {
                for item in items {
                    self.check(item)?;
                }
            }
            HostTypeDescriptor::Function { arguments, return_ } => {
                for argument in arguments {
                    if !self.inhabited(argument)? {
                        return Err(ContractError::Callback);
                    }
                }
                if self.inhabited(return_)? {
                    self.check(return_)?;
                }
            }
            HostTypeDescriptor::Custom { schema, arguments } => {
                // Retain parameter and callable distinctions while closing recursive schemas.
                if self.active.contains(descriptor) {
                    return Ok(());
                }
                self.active.push(descriptor.clone());
                for constructor in schema.constructors() {
                    let fields = constructor
                        .fields()
                        .iter()
                        .map(|field| {
                            HostTypeDescriptor::from_schema(field.type_(), arguments, &self.schemas)
                        })
                        .collect::<Vec<_>>();
                    let mut inhabited = true;
                    for field in &fields {
                        if !self.inhabited(field)? {
                            inhabited = false;
                            break;
                        }
                    }
                    if inhabited {
                        for field in &fields {
                            self.check(field)?;
                        }
                    }
                }
                self.active.pop();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Callbacks, ContractError, HostCustomTypeSchema, HostTypeDescriptor, Types};
    use crate::host::{HostCustomConstructorSchema, HostCustomFieldSchema, HostSchemaType};

    #[test]
    fn registration_custom_schemas_validate_callbacks_before_native_execution() {
        use super::admit;
        use crate::host::{
            HostCall, HostCallCompletion, HostCallError, HostCustom,
            HostCustomConstructorDefinition, HostCustomConstructorList,
            HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
            HostCustomFieldListEnd, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
            HostFunctionType, HostProvider, HostProviderModule, HostProviderSet, HostTypeIndex0,
            HostTypeList, HostTypeListEnd, HostTypeParameter, StatelessHostProfile,
        };
        use crate::plan::execution::host::HostTypeArgument;
        use crate::plan::execution::prepared::admission::hosts::{NativeFunctions, tests::lowered};
        use crate::plan::execution::prepared::admission::type_::TypeError;
        use crate::plan::execution::type_::{TypeMetadata, ValueShapeDescriptor, ValueShapeId};
        use num_bigint::BigInt;

        struct CallbackBox;
        struct BoxConstructor;
        struct Callback;
        impl HostCustomSchema for CallbackBox {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "CallbackBox";
            const PARAMETER_COUNT: usize = 1;
            type Constructors =
                HostCustomConstructorList<BoxConstructor, HostCustomConstructorListEnd>;
        }
        impl HostCustomConstructorDefinition for BoxConstructor {
            const NAME: &'static str = "CallbackBox";
            type Fields = HostCustomFieldList<Callback, HostCustomFieldListEnd>;
        }
        impl HostCustomField for Callback {
            const LABEL: Option<&'static str> = None;
            type Type = HostFunctionType<
                HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
                BigInt,
            >;
        }
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        type Input =
            HostCustomType<CallbackBox, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;
        fn accept<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, bool>,
            _value: HostCustom<'call, Input>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(true))
        }
        let providers = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Provider, (Input,), bool, _>("accept", accept)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type CallbackBox(a) { CallbackBox(fn(a) -> Int) }
@external(erlang, "native", "accept")
fn accept(value: CallbackBox(a)) -> Bool
pub fn main() {
  echo fn(value) { value }
  accept(CallbackBox(fn(value: Int) { value + 1 }))
}
"#;
        let (program, values, nevers) = lowered(source, providers());
        let registration = NativeFunctions::new(&values, &nevers, providers())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        for returns_value in [false, true] {
            assert_eq!(
                admit(
                    &registration,
                    &values[0].type_arguments,
                    &types,
                    returns_value
                ),
                Ok(())
            );
        }
        assert_eq!(
            admit(
                &registration,
                &[HostTypeArgument {
                    type_: TypeMetadata::Int,
                    shape: ValueShapeId(999),
                }],
                &types,
                true
            ),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        let symbolic = common
            .value_shapes
            .shapes
            .iter()
            .position(|shape| matches!(shape, ValueShapeDescriptor::Parameter(_)))
            .unwrap();
        assert_eq!(
            admit(
                &registration,
                &[HostTypeArgument {
                    type_: TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
                    shape: ValueShapeId(symbolic),
                }],
                &types,
                true
            ),
            Err(ContractError::Callback)
        );
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            providers(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Bool(true)
        );

        fn unavailable<'call>(
            _call: HostCall<'call, StatelessHostProfile, Provider, Input>,
            _value: crate::host::HostValue<'call, HostTypeParameter<0>>,
        ) -> Result<HostCallCompletion<'call, Input>, HostCallError> {
            Err(crate::HostFailure::new("callback unavailable").into())
        }
        let returning = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Provider, (HostTypeParameter<0>,), Input, _>(
                    "make",
                    unavailable,
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type CallbackBox(a) { CallbackBox(fn(a) -> Int) }
@external(erlang, "native", "make") fn make(value: a) -> CallbackBox(a)
pub fn main() { echo fn(value) { value } make(42) }
"#;
        let (program, values, nevers) = lowered(source, returning());
        let registration = NativeFunctions::new(&values, &nevers, returning())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        assert_eq!(
            admit(&registration, &values[0].type_arguments, &types, true),
            Ok(())
        );
        let symbolic = common
            .value_shapes
            .shapes
            .iter()
            .position(|shape| matches!(shape, ValueShapeDescriptor::Parameter(_)))
            .unwrap();
        let arguments = [HostTypeArgument {
            type_: TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
            shape: ValueShapeId(symbolic),
        }];
        assert_eq!(admit(&registration, &arguments, &types, false), Ok(()));
        assert_eq!(
            admit(&registration, &arguments, &types, true),
            Err(ContractError::Callback)
        );
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            returning(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
                .unwrap_err()
                .to_string(),
            "host function app::main.make failed: callback unavailable"
        );
    }

    #[test]
    fn only_callable_capabilities_require_inhabited_arguments_in_nested_values() {
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Empty { Again(Empty) }
pub type Box(a) { Box(a) }
pub type Recursive(a) { End Next(List(Recursive(a))) Item(a) }
pub fn main() { 42 }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let box_schema = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Box",
            1,
            [HostCustomConstructorSchema::new(
                "Box",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::Parameter(0),
                )],
            )],
        );
        let recursive = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Recursive",
            1,
            [
                HostCustomConstructorSchema::new("End", []),
                HostCustomConstructorSchema::new(
                    "Next",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::list(HostSchemaType::custom(
                            "geam",
                            "example",
                            "Recursive",
                            [HostSchemaType::Parameter(0)],
                        )),
                    )],
                ),
                HostCustomConstructorSchema::new(
                    "Item",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Parameter(0),
                    )],
                ),
            ],
        );
        let schemas = [&box_schema, &recursive]
            .into_iter()
            .map(|schema| {
                (
                    (
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    schema,
                )
            })
            .collect();
        let mut search = Callbacks {
            types: &types,
            arguments: 1,
            inhabited_arguments: &[false],
            schemas,
            active: Vec::new(),
        };
        let symbolic = HostTypeDescriptor::Function {
            arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
            return_: Box::new(HostTypeDescriptor::Int),
        };
        let opaque = HostTypeDescriptor::OpaqueFunction {
            arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
            return_: Box::new(HostTypeDescriptor::Int),
        };
        let never = HostTypeDescriptor::Function {
            arguments: Box::new([]),
            return_: Box::new(HostTypeDescriptor::Parameter(0)),
        };
        for (descriptor, expected) in [
            (symbolic.clone(), Err(ContractError::Callback)),
            (opaque.clone(), Ok(())),
            (never.clone(), Ok(())),
            (
                HostTypeDescriptor::List(Box::new(symbolic.clone())),
                Err(ContractError::Callback),
            ),
            (
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Tuple(Box::new([
                    HostTypeDescriptor::Parameter(0),
                    symbolic.clone(),
                ])))),
                Ok(()),
            ),
            (
                HostTypeDescriptor::Tuple(Box::new([HostTypeDescriptor::Bool, symbolic.clone()])),
                Err(ContractError::Callback),
            ),
            (
                HostTypeDescriptor::Function {
                    arguments: Box::new([]),
                    return_: Box::new(symbolic.clone()),
                },
                Err(ContractError::Callback),
            ),
            (
                HostTypeDescriptor::Function {
                    arguments: Box::new([symbolic.clone()]),
                    return_: Box::new(HostTypeDescriptor::Int),
                },
                Ok(()),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: box_schema.clone(),
                    arguments: Box::new([symbolic.clone()]),
                },
                Err(ContractError::Callback),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: box_schema.clone(),
                    arguments: Box::new([opaque]),
                },
                Ok(()),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: recursive.clone(),
                    arguments: Box::new([symbolic]),
                },
                Err(ContractError::Callback),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: recursive.clone(),
                    arguments: Box::new([never]),
                },
                Ok(()),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: recursive.clone(),
                    arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
                },
                Ok(()),
            ),
        ] {
            search.active.clear();
            assert_eq!(search.check(&descriptor), expected, "{descriptor:?}");
        }
        search.arguments = 0;
        assert_eq!(
            search.inhabited(&HostTypeDescriptor::Parameter(0)),
            Err(ContractError::TypeArguments)
        );
        for descriptor in [
            HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Parameter(0))),
            HostTypeDescriptor::Function {
                arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
                return_: Box::new(HostTypeDescriptor::Int),
            },
            HostTypeDescriptor::Function {
                arguments: Box::new([]),
                return_: Box::new(HostTypeDescriptor::Parameter(0)),
            },
            HostTypeDescriptor::Custom {
                schema: box_schema.clone(),
                arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
            },
        ] {
            search.active.clear();
            assert_eq!(search.check(&descriptor), Err(ContractError::TypeArguments));
        }
    }

    #[test]
    fn recursive_visits_distinguish_parameter_refinements_from_nominal_arguments() {
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Empty { Again(Empty) }
pub type Choice { Live Dead(Empty) }
pub type Recursive(a) {
  Next(Recursive(Choice))
  Item(a, fn(Empty) -> Int)
}
pub fn main() { 42 }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let recursive = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Recursive",
            1,
            [
                HostCustomConstructorSchema::new(
                    "Next",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::custom(
                            "geam",
                            "example",
                            "Recursive",
                            [HostSchemaType::custom("geam", "example", "Choice", [])],
                        ),
                    )],
                ),
                HostCustomConstructorSchema::new(
                    "Item",
                    [
                        HostCustomFieldSchema::new(None::<&str>, HostSchemaType::Parameter(0)),
                        HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::Function {
                                arguments: Box::new([HostSchemaType::custom(
                                    "geam",
                                    "example",
                                    "Empty",
                                    [],
                                )]),
                                return_: Box::new(HostSchemaType::Int),
                            },
                        ),
                    ],
                ),
            ],
        );
        let choice = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Choice",
            0,
            [
                HostCustomConstructorSchema::new("Live", []),
                HostCustomConstructorSchema::new(
                    "Dead",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::custom("geam", "example", "Empty", []),
                    )],
                ),
            ],
        );
        let empty = HostCustomTypeSchema::new(
            "geam",
            "example",
            "Empty",
            0,
            [HostCustomConstructorSchema::new(
                "Again",
                [HostCustomFieldSchema::new(
                    None::<&str>,
                    HostSchemaType::custom("geam", "example", "Empty", []),
                )],
            )],
        );
        let mut search = Callbacks {
            arguments: 1,
            inhabited_arguments: &[false],
            types: &types,
            schemas: [&recursive, &choice, &empty]
                .into_iter()
                .map(|schema| {
                    (
                        (
                            schema.package().clone(),
                            schema.module().clone(),
                            schema.name().clone(),
                        ),
                        schema,
                    )
                })
                .collect(),
            active: Vec::new(),
        };
        assert_eq!(
            search.check(&HostTypeDescriptor::Custom {
                schema: recursive.clone(),
                arguments: Box::new([HostTypeDescriptor::Parameter(0)]),
            }),
            Err(ContractError::Callback)
        );
    }
}
