use super::{NativeError, RegistrationError};
use crate::host::{
    HostFunctionImplementation, HostFunctionSchema, HostNeverFunction, HostProfile,
    HostProviderSet, HostValueFunction, RegisteredHostConstructions,
};
use crate::plan::execution::host::{HostFunctionTables, HostedFunction, HostedFunctionMetadata};
use std::cell::RefCell;
use std::collections::HashMap;

pub(in crate::plan::execution::prepared::admission) struct NativeFunctions<
    'data,
    Profile: HostProfile,
> {
    // Populated by the ordinary function-table admission walk, before callable
    // construction links are checked. This scratch state never enters execution.
    pub(super) callable_bindings:
        RefCell<HashMap<crate::plan::execution::host::HostCallableEntry, (bool, usize)>>,
    pub(super) registrations: Vec<Registration>,
    pub(super) external_types: Vec<crate::host::HostExternalTypeSchema>,
    pub(super) values: Vec<(
        &'data HostedFunctionMetadata,
        HostValueFunction<Profile>,
        usize,
    )>,
    pub(super) nevers: Vec<(
        &'data HostedFunctionMetadata,
        HostNeverFunction<Profile>,
        usize,
    )>,
}

pub(super) struct Registration {
    pub(super) schema: HostFunctionSchema,
    pub(super) constructions: RegisteredHostConstructions,
}

impl<'data, Profile: HostProfile> NativeFunctions<'data, Profile> {
    pub(in crate::plan::execution::prepared::admission) fn new(
        value_functions: &'data [HostedFunctionMetadata],
        never_functions: &'data [HostedFunctionMetadata],
        hosts: HostProviderSet<Profile>,
    ) -> Result<Self, NativeError> {
        let (modules, providers, callables, implementations) = hosts.into_registered();
        let mut external_types = Vec::new();
        let mut shared_custom_types = HashMap::new();
        let sources = modules
            .into_iter()
            .map(|module| {
                let (package, module, functions) = module.into_parts();
                (package, module, functions, Vec::new(), Vec::new())
            })
            .chain(providers.into_iter().map(|module| module.into_parts()))
            .chain(callables.into_iter().map(|callable| {
                (
                    callable.identity.package,
                    callable.identity.module,
                    vec![callable.function],
                    Vec::new(),
                    Vec::new(),
                )
            }));
        let mut registrations = Vec::new();
        let mut index = HashMap::new();
        for (package, module, functions, schemas, shared) in sources {
            external_types.extend(schemas);
            for schema in shared {
                shared_custom_types.insert(
                    (
                        schema.package().clone(),
                        schema.module().clone(),
                        schema.name().clone(),
                    ),
                    schema,
                );
            }
            for function in functions {
                let (schema, constructions, implementation) = function.into_parts();
                let key = (
                    package.to_string(),
                    module.to_string(),
                    schema.name().to_string(),
                    schema.is_callable(),
                );
                let slot = registrations.len();
                index.insert(key, (slot, implementation));
                registrations.push(Registration {
                    schema,
                    constructions,
                });
            }
        }
        let find = |metadata: &HostedFunctionMetadata| {
            let key = (
                metadata.package().to_string(),
                metadata.module().to_string(),
                metadata.name().to_string(),
                metadata.registration.callable,
            );
            let failure = |reason| NativeError::Registration {
                package: key.0.clone(),
                module: key.1.clone(),
                function: key.2.clone(),
                reason,
            };
            let (slot, implementation) = index
                .get(&key)
                .ok_or_else(|| failure(RegistrationError::Missing))?;
            let registered = &registrations[*slot];
            if !metadata
                .registration
                .matches(&registered.schema, &registered.constructions)
            {
                return Err(failure(RegistrationError::Declaration));
            }
            for required in registered
                .schema
                .custom_schemas()
                .iter()
                .chain(registered.constructions.custom_schemas())
            {
                if required.requires_shared_access()
                    && shared_custom_types.get(&(
                        required.package().clone(),
                        required.module().clone(),
                        required.name().clone(),
                    )) != Some(required)
                {
                    return Err(failure(RegistrationError::SharedCustomType {
                        custom_type: Box::new(crate::plan::CustomTypeName::new(
                            required.package().clone(),
                            required.module().clone(),
                            required.name().clone(),
                        )),
                    }));
                }
            }
            Ok((*slot, implementations.implementation(*implementation)))
        };
        let mut values = Vec::with_capacity(value_functions.len());
        for metadata in value_functions {
            let (slot, implementation) = find(metadata)?;
            let HostFunctionImplementation::Value(implementation) = implementation.as_ref() else {
                return Err(return_kind(metadata));
            };
            values.push((metadata, implementation.clone(), slot));
        }
        let mut nevers = Vec::with_capacity(never_functions.len());
        for metadata in never_functions {
            let (slot, implementation) = find(metadata)?;
            let HostFunctionImplementation::Never(implementation) = implementation.as_ref() else {
                return Err(return_kind(metadata));
            };
            nevers.push((metadata, implementation.clone(), slot));
        }
        Ok(Self {
            callable_bindings: RefCell::new(HashMap::new()),
            registrations,
            external_types,
            values,
            nevers,
        })
    }
}

impl<Profile: HostProfile> NativeFunctions<'static, Profile> {
    pub(in crate::plan::execution::prepared::admission) fn into_tables(
        self,
    ) -> HostFunctionTables<Profile> {
        let values = self
            .values
            .into_iter()
            .map(|(metadata, implementation, _)| {
                HostedFunction::new(metadata.borrowed(), implementation)
            })
            .collect();
        let nevers = self
            .nevers
            .into_iter()
            .map(|(metadata, implementation, _)| {
                HostedFunction::new(metadata.borrowed(), implementation)
            })
            .collect();
        HostFunctionTables::new(values, nevers)
    }
}

fn return_kind(metadata: &HostedFunctionMetadata) -> NativeError {
    NativeError::Registration {
        package: metadata.package().to_string(),
        module: metadata.module().to_string(),
        function: metadata.name().to_string(),
        reason: RegistrationError::ReturnKind,
    }
}

#[cfg(test)]
mod tests {
    use super::{NativeError, NativeFunctions, RegistrationError};
    use crate::host::{HostModule, HostProviderModule, HostProviderSet};
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn prepared_consumers_require_the_selected_producers_exact_sharing_grant() {
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostCustom,
            HostCustomConstructorDefinition, HostCustomConstructorList,
            HostCustomConstructorListEnd, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
            HostProvider, StatelessHostProfile,
        };
        use std::sync::Arc;
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        struct Schema;
        struct Constructor;
        struct Replaced;
        impl HostCustomSchema for Schema {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "handles";
            const NAME: &'static str = "Handle";
            const PARAMETER_COUNT: usize = 0;
            const SHARED: bool = true;
            type Constructors =
                HostCustomConstructorList<Constructor, HostCustomConstructorListEnd>;
        }
        impl HostCustomSchema for Replaced {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "handles";
            const NAME: &'static str = "Handle";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = HostCustomConstructorListEnd;
        }
        impl HostCustomConstructorDefinition for Constructor {
            const NAME: &'static str = "Handle";
            type Fields = HostCustomFieldListEnd;
        }
        type Handle = HostCustomType<Schema>;
        fn retain<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Handle>,
            value: HostCustom<'call, Handle>,
        ) -> Result<HostCallCompletion<'call, Handle>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(value))
        }
        fn probe<'call>(
            call: HostCall<'call, StatelessHostProfile, Provider, bool>,
            _: crate::HostConstructions<'call, crate::HostTypeList<Handle, crate::HostTypeListEnd>>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            Ok(call.return_value(true))
        }
        let hosts = |grant| {
            let owner = HostProviderModule::new("app", "handles").unwrap();
            let owner = match grant {
                0 => owner,
                1 => owner.with_shared_custom_type::<Schema>().unwrap(),
                _ => owner.with_shared_custom_type::<Replaced>().unwrap(),
            };
            HostProviderSet::from_providers([
                owner,
                HostProviderModule::new("app", "main")
                    .unwrap()
                    .with_scoped_function::<Provider, (Handle,), Handle, _>("retain", retain)
                    .unwrap()
                    .with_scoped_function_and_constructions::<Provider, (), bool, crate::HostTypeList<Handle, crate::HostTypeListEnd>, _>("probe", probe)
                    .unwrap(),
            ])
            .unwrap()
        };
        let compile = || {
            crate::compile_typed_host_program(
                "app",
                "main",
                [crate::PackageSource::new(
                    "app",
                    Vec::<&str>::new(),
                    [
                        crate::ModuleSource::new(
                            "handles",
                            "handles.gleam",
                            "pub opaque type Handle { Handle }\npub fn make() { Handle }",
                        ),
                        crate::ModuleSource::new(
                            "main",
                            "main.gleam",
                            r#"
import handles
@external(erlang, "native", "retain")
fn retain(value: handles.Handle) -> handles.Handle
@external(erlang, "native", "probe")
fn probe() -> Bool
pub fn main() { retain(handles.make()) probe() Nil }
"#,
                        ),
                    ],
                )],
                hosts(1),
            )
            .unwrap()
        };
        let mut execution = crate::HostedExecution::try_from_module_plan(
            crate::plan_host_program(compile()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Nil
        );
        let (_, callbacks) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(compile()).unwrap(),
        )
        .unwrap();
        let (values, nevers) = callbacks.into_metadata();
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| Arc::try_unwrap(value).ok().unwrap())
            .collect::<Vec<_>>();
        assert!(nevers.is_empty());
        assert_eq!(values.len(), 2);
        NativeFunctions::new(&values, &[], hosts(1)).unwrap();
        for metadata in &values {
            for grant in [0, 2] {
                assert_eq!(
                    NativeFunctions::new(std::slice::from_ref(metadata), &[], hosts(grant)).err(),
                    Some(NativeError::Registration {
                        package: "app".into(),
                        module: "main".into(),
                        function: metadata.name().into(),
                        reason: RegistrationError::SharedCustomType {
                            custom_type: Box::new(crate::plan::CustomTypeName::new(
                                "app".into(),
                                "handles".into(),
                                "Handle".into()
                            )),
                        },
                    })
                );
            }
        }
        let mut altered = values;
        altered.sort_by_key(|metadata| metadata.name() != "retain");
        let mut registration = (*altered[0].registration).clone();
        let mut schemas = registration.custom_schemas.to_vec();
        schemas[0].shared = false;
        registration.custom_schemas = schemas.into();
        altered[0].registration = Box::new(registration).into();
        assert_eq!(
            NativeFunctions::new(&altered, &[], hosts(1)).err(),
            Some(NativeError::Registration {
                package: "app".into(),
                module: "main".into(),
                function: "retain".into(),
                reason: RegistrationError::Declaration,
            })
        );
    }

    #[test]
    fn source_less_functions_use_external_types_registered_with_the_source_declaration() {
        use crate::host::{
            HostCall, HostCallCompletion, HostCallError, HostExternal, HostExternalBinding,
            HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
            HostExternalStorage, HostExternalStore, HostExternalType, HostProfile, HostProvider,
        };
        use crate::plan::execution::prepared::admission::{
            catalog::Catalog, functions, functions::Hosts, instruction::Instructions,
            source::Sources, type_::Types,
        };
        use std::sync::Arc;

        struct Profile;
        impl HostProfile for Profile {
            type RunState = ();
            type ExternalStores = HostExternalStore<u8>;
            type ExecutionState = ();
        }
        struct Native;
        impl HostProvider<Profile> for Native {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        struct Key;
        impl HostExternalSchema for Key {
            const PACKAGE: &'static str = "support";
            const MODULE: &'static str = "native/keys";
            const NAME: &'static str = "Key";
            const PARAMETER_COUNT: usize = 0;
        }
        impl HostExternalBinding<Profile, Key> for Native {
            type Storage = Self;
        }
        impl HostExternalStorage<Profile, Key> for Native {
            type Payload = u8;
            fn store(stores: &HostExternalStore<u8>) -> &HostExternalStore<u8> {
                stores
            }
            fn source_equal(_: &HostExternalEquality<'_>, left: &u8, right: &u8) -> bool {
                left == right
            }
            fn source_hash(_: &HostExternalHashing<'_>, value: &u8) -> u64 {
                u64::from(*value)
            }
            fn inspect(_: &HostExternalInspection<'_>, value: &u8) -> ecow::EcoString {
                format!("Key({value})").into()
            }
        }
        type KeyType = HostExternalType<Key>;
        fn make(
            mut call: HostCall<'_, Profile, Native, KeyType>,
        ) -> Result<HostCallCompletion<'_, KeyType>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            let value = call.create_external_with_binding::<Native>(42);
            Ok(call.return_value(value))
        }
        fn same<'call>(
            call: HostCall<'call, Profile, Native, bool>,
            left: HostExternal<'call, KeyType>,
            right: HostExternal<'call, KeyType>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            assert_eq!(
                call.source_hash::<KeyType>(left),
                call.source_hash::<KeyType>(right)
            );
            let value = call.equal::<KeyType>(left, right);
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::with_providers(
                [
                    HostModule::<Profile>::new_for_profile("support", "native/compare")
                        .unwrap()
                        .with_scoped_function::<Native, (KeyType, KeyType), bool, _>("same", same)
                        .unwrap(),
                    HostModule::<Profile>::new_for_profile("app", "native/other")
                        .unwrap()
                        .with_scoped_function::<Native, (KeyType, KeyType), bool, _>("same", same)
                        .unwrap(),
                ],
                [HostProviderModule::<Profile>::new("support", "native/keys")
                    .unwrap()
                    .with_external_type::<Native, Key>()
                    .unwrap()
                    .with_scoped_function::<Native, (), KeyType, _>("make", make)
                    .unwrap()],
            )
            .unwrap()
        };
        let source = r#"import native/keys
import native/compare
import native/other
pub fn main() {
  let key = echo keys.make()
  #(compare.same(key, keys.make()), other.same(key, keys.make()))
}
"#;
        let compile = || {
            crate::compile_typed_host_program(
                "app",
                "main",
                [
                    crate::PackageSource::new(
                        "app",
                        ["support"],
                        [crate::ModuleSource::new("main", "src/main.gleam", source)],
                    ),
                    crate::PackageSource::new(
                        "support",
                        Vec::<ecow::EcoString>::new(),
                        [crate::ModuleSource::new(
                            "native/keys",
                            "src/native/keys.gleam",
                            r#"
pub type Key
@external(erlang, "keys_ffi", "make")
pub fn make() -> Key
"#,
                        )],
                    ),
                ],
                hosts(),
            )
            .unwrap()
        };
        let (program, callbacks) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(compile()).unwrap(),
        )
        .unwrap();
        let (values, nevers) = callbacks.into_metadata();
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| Arc::try_unwrap(value).ok().unwrap())
            .collect::<Vec<_>>();
        assert!(nevers.is_empty());
        let linked = NativeFunctions::new(&values, &[], hosts()).unwrap();
        assert_eq!(
            linked
                .external_types
                .iter()
                .map(|schema| (
                    schema.package().as_str(),
                    schema.module().as_str(),
                    schema.name().as_str(),
                ))
                .collect::<Vec<_>>(),
            [("support", "native/keys", "Key")]
        );
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        assert_eq!(linked.tables(&context), Ok(()));
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );
        let mut execution = crate::HostedExecution::try_from_module_plan(
            crate::plan_host_program(compile()).unwrap(),
        )
        .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut echo),
            Ok(crate::Value::Tuple(vec![
                crate::Value::Bool(true),
                crate::Value::Bool(true)
            ]))
        );
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["src/main.gleam:5\nKey(42)"]
        );
    }

    #[test]
    fn fresh_registration_checks_names_declarations_and_return_table_placement() {
        let hosts = || {
            HostProviderSet::<crate::StatelessHostProfile>::from_providers([
                HostProviderModule::new("app", "main")
                    .unwrap()
                    .with_function("double", |value: BigInt| value * 2)
                    .unwrap()
                    .with_fallible_function("stop", || -> Result<Infallible, crate::HostFailure> {
                        Err(crate::HostFailure::new("stopped"))
                    })
                    .unwrap(),
            ])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "double") fn double(value: Int) -> Int
@external(erlang, "native", "stop") fn stop() -> a
pub fn main() { echo double(21) stop() }
"#;
        let (_, values, nevers) = super::super::tests::lowered(source, hosts());
        assert_eq!((values.len(), nevers.len()), (1, 1));
        let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
        assert_eq!((linked.values.len(), linked.nevers.len()), (1, 1));
        assert_eq!(linked.values[0].2, 0);
        assert_eq!(linked.nevers[0].2, 1);
        for (value_table, never_table, name) in [
            (&nevers[..], &[][..], "stop"),
            (&[][..], &values[..], "double"),
        ] {
            assert_eq!(
                NativeFunctions::new(value_table, never_table, hosts()).err(),
                Some(NativeError::Registration {
                    package: "app".into(),
                    module: "main".into(),
                    function: name.into(),
                    reason: RegistrationError::ReturnKind,
                })
            );
        }
        for (value_table, never_table, name) in [
            (&values[..], &[][..], "double"),
            (&[][..], &nevers[..], "stop"),
        ] {
            assert_eq!(
                NativeFunctions::new(
                    value_table,
                    never_table,
                    HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap()
                )
                .err(),
                Some(NativeError::Registration {
                    package: "app".into(),
                    module: "main".into(),
                    function: name.into(),
                    reason: RegistrationError::Missing,
                })
            );
        }
        let incompatible = HostProviderSet::new([HostModule::new("app", "main")
            .unwrap()
            .with_function("double", std::convert::identity::<crate::StringValue>)
            .unwrap()])
        .unwrap();
        assert_eq!(
            NativeFunctions::new(&values, &[], incompatible).err(),
            Some(NativeError::Registration {
                package: "app".into(),
                module: "main".into(),
                function: "double".into(),
                reason: RegistrationError::Declaration,
            })
        );
        let values = Box::leak(values.into_boxed_slice());
        let nevers = Box::leak(nevers.into_boxed_slice());
        let (loaded_values, loaded_nevers) = NativeFunctions::new(values, nevers, hosts())
            .unwrap()
            .into_tables()
            .into_metadata();
        assert_eq!(loaded_values[0].name(), "double");
        assert_eq!(loaded_nevers[0].name(), "stop");
        assert!(std::ptr::eq(
            loaded_values[0].registration.as_ref(),
            values[0].registration.as_ref()
        ));
        assert!(std::ptr::eq(
            loaded_nevers[0].registration.as_ref(),
            nevers[0].registration.as_ref()
        ));
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        let error = crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap_err();
        assert_eq!(
            error.to_string(),
            "host function app::main.stop failed: stopped"
        );
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["src/main.gleam:4\n42"]
        );
    }
}
