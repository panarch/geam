mod callable;
mod contract;
mod link;

use super::catalog::Function;
use super::functions::Hosts;
use super::instruction::Instructions;
use super::local::Output;
use super::type_::TypeError;
use crate::host::HostProfile;
use crate::plan::execution::function::{ExecutionFunctionBody, HostedExecutionGraph};
use crate::plan::execution::host::{
    HostFunctionId, HostNeverFunctionId, HostedExecutionProfile, HostedFunctionTarget,
};

pub(super) use link::NativeFunctions;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum NativeError {
    Registration {
        package: String,
        module: String,
        function: String,
        reason: RegistrationError,
    },
    Contract {
        value: bool,
        index: usize,
        reason: ContractError,
    },
    MissingValue(usize),
    MissingNever(usize),
    Call(ContractError),
    ExternalType {
        package: String,
        module: String,
        name: String,
        expected: usize,
        actual: Option<usize>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum RegistrationError {
    SharedCustomType {
        custom_type: Box<crate::plan::CustomTypeName>,
    },
    Missing,
    Declaration,
    ReturnKind,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ContractError {
    Type(TypeError),
    Source(super::source::SourceError),
    Signature,
    TypeArguments,
    Parameters,
    Captures,
    Callable,
    Construction,
    Native,
    Callback,
}

impl<Profile: HostProfile> NativeFunctions<'_, Profile> {
    pub(super) fn library_callables(
        &self,
        entries: &[crate::plan::execution::LibraryNativeConstruction],
        catalog: &super::catalog::Catalog<'_>,
        types: &super::type_::Types<'_>,
    ) -> Result<(), NativeError> {
        callable::library(self, entries, catalog, types)
    }
}

impl<Profile: HostProfile> Hosts<HostedExecutionProfile> for NativeFunctions<'_, Profile> {
    type Error = NativeError;

    fn callables(
        &self,
        context: &Instructions<'_, '_, HostedExecutionGraph>,
    ) -> Result<(), NativeError> {
        callable::admit(self, context)
    }

    fn tables(
        &self,
        context: &Instructions<'_, '_, HostedExecutionGraph>,
    ) -> Result<(), NativeError> {
        admit_external_types(context.types.external_types(), &self.external_types)?;
        for (index, (metadata, _, registration)) in self.values.iter().enumerate() {
            contract::metadata(
                metadata,
                &self.registrations[*registration],
                context.types,
                true,
            )
            .and_then(|()| {
                context
                    .sources
                    .span(metadata.module(), metadata.site.span())
                    .map_err(ContractError::Source)
            })
            .map_err(|reason| NativeError::Contract {
                value: true,
                index,
                reason,
            })?;
        }
        for (index, (metadata, _, registration)) in self.nevers.iter().enumerate() {
            contract::metadata(
                metadata,
                &self.registrations[*registration],
                context.types,
                false,
            )
            .and_then(|()| {
                context
                    .sources
                    .span(metadata.module(), metadata.site.span())
                    .map_err(ContractError::Source)
            })
            .map_err(|reason| NativeError::Contract {
                value: false,
                index,
                reason,
            })?;
        }
        Ok(())
    }

    fn function<Body: ExecutionFunctionBody>(
        &self,
        target: &HostedFunctionTarget<Body>,
        declaration: &Function<'_>,
        context: &Instructions<'_, '_, HostedExecutionGraph>,
    ) -> Result<(), NativeError>
    where
        Body::Return: Output,
    {
        match target {
            HostedFunctionTarget::Value(HostFunctionId { index, return_, .. }) => {
                let (metadata, _, registration) = self
                    .values
                    .get(*index)
                    .ok_or(NativeError::MissingValue(*index))?;
                contract::call(
                    metadata,
                    &self.registrations[*registration],
                    declaration,
                    context.types,
                )
                .map_err(NativeError::Call)?;
                if !context.types.inhabited(declaration.return_shape) {
                    return Err(NativeError::Call(ContractError::Signature));
                }
                return_
                    .admit(declaration.return_, context.types)
                    .map_err(|error| NativeError::Call(ContractError::Type(error)))?;
                if let Some(entry) = metadata.callable_entry {
                    self.callable_bindings
                        .borrow_mut()
                        .insert(entry, (true, *index));
                }
                Ok(())
            }
            HostedFunctionTarget::Never(target) => self.never(target, declaration, context),
        }
    }

    fn never(
        &self,
        target: &HostNeverFunctionId,
        declaration: &Function<'_>,
        context: &Instructions<'_, '_, HostedExecutionGraph>,
    ) -> Result<(), NativeError> {
        let (metadata, _, registration) = self
            .nevers
            .get(target.0)
            .ok_or(NativeError::MissingNever(target.0))?;
        contract::call(
            metadata,
            &self.registrations[*registration],
            declaration,
            context.types,
        )
        .map_err(NativeError::Call)?;
        if let Some(entry) = metadata.callable_entry {
            self.callable_bindings
                .borrow_mut()
                .insert(entry, (false, target.0));
        }
        Ok(())
    }
}

fn admit_external_types(
    expected: &[crate::plan::execution::type_::NominalTypeMetadata],
    registered: &[crate::host::HostExternalTypeSchema],
) -> Result<(), NativeError> {
    for type_ in expected {
        let actual = registered
            .iter()
            .find(|schema| {
                schema.package().as_str() == type_.package.as_str()
                    && schema.module().as_str() == type_.module.as_str()
                    && schema.name().as_str() == type_.name.as_str()
            })
            .map(|schema| schema.parameter_count());
        if actual != Some(type_.arguments.len()) {
            return Err(NativeError::ExternalType {
                package: type_.package.to_string(),
                module: type_.module.to_string(),
                name: type_.name.to_string(),
                expected: type_.arguments.len(),
                actual,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Hosts;
    use super::{
        ContractError, HostedExecutionProfile, Instructions, NativeError, NativeFunctions,
        RegistrationError, admit_external_types,
    };
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeParameter, HostValue, StatelessHostProfile,
    };
    use crate::plan::execution::host::HostedFunctionMetadata;
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog, functions, source::Sources, type_::Types,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::TypeMetadata;
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;
    use std::sync::Arc;

    #[test]
    fn external_types_require_exact_package_module_name_and_generic_arity() {
        use crate::host::HostExternalTypeSchema;
        use crate::plan::execution::type_::NominalTypeMetadata;
        let expected = [NominalTypeMetadata {
            package: "app".into(),
            module: "resource".into(),
            name: "Handle".into(),
            arguments: vec![TypeMetadata::Int].into(),
        }];
        assert_eq!(admit_external_types(&[], &[]), Ok(()));
        assert_eq!(
            admit_external_types(&expected, &[]),
            Err(NativeError::ExternalType {
                package: "app".into(),
                module: "resource".into(),
                name: "Handle".into(),
                expected: 1,
                actual: None,
            })
        );
        for (schema, actual) in [
            (
                HostExternalTypeSchema::new("elsewhere", "resource", "Handle", 1),
                None,
            ),
            (
                HostExternalTypeSchema::new("app", "other", "Handle", 1),
                None,
            ),
            (
                HostExternalTypeSchema::new("app", "resource", "Other", 1),
                None,
            ),
            (
                HostExternalTypeSchema::new("app", "resource", "Handle", 0),
                Some(0),
            ),
            (
                HostExternalTypeSchema::new("app", "resource", "Handle", 2),
                Some(2),
            ),
        ] {
            assert_eq!(
                admit_external_types(&expected, &[schema]),
                Err(NativeError::ExternalType {
                    package: "app".into(),
                    module: "resource".into(),
                    name: "Handle".into(),
                    expected: 1,
                    actual,
                })
            );
        }
        assert_eq!(
            admit_external_types(
                &expected,
                &[
                    HostExternalTypeSchema::new("other", "other", "Unused", 0),
                    HostExternalTypeSchema::new("app", "resource", "Handle", 1),
                ]
            ),
            Ok(())
        );
    }

    #[test]
    fn tables_require_external_types_even_without_native_function_targets() {
        let (program, values, nevers) = super::super::tests::lowered_native(
            r#"
pub type Key
@external(erlang, "native", "key")
fn key() -> Key
pub fn main() { fn(value: Key) { value } }
"#,
        );
        assert!(values.is_empty());
        assert!(nevers.is_empty());
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
        let linked = NativeFunctions::<StatelessHostProfile>::new(
            &values,
            &nevers,
            HostProviderSet::from_providers([]).unwrap(),
        )
        .unwrap();
        assert_eq!(
            linked.tables(&context),
            Err(NativeError::ExternalType {
                package: "app".into(),
                module: "main".into(),
                name: "Key".into(),
                expected: 0,
                actual: None,
            })
        );
    }

    #[test]
    fn external_values_keep_native_semantics_through_every_hosted_return_family() {
        use crate::host::{
            HostExternal, HostExternalBinding, HostExternalEquality, HostExternalHashing,
            HostExternalInspection, HostExternalSchema, HostExternalStorage, HostExternalStore,
            HostExternalType, HostProfile,
        };

        struct Profile;
        impl HostProfile for Profile {
            type RunState = usize;
            type ExternalStores = HostExternalStore<u8>;
            type ExecutionState = ();
        }
        struct Native;
        impl HostProvider<Profile> for Native {
            type State = usize;
            fn project(state: &mut usize) -> &mut usize {
                state
            }
        }
        struct Key;
        impl HostExternalSchema for Key {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "main";
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
        type Item = HostTypeParameter<0>;
        fn make(
            mut call: HostCall<'_, Profile, Native, KeyType>,
        ) -> Result<HostCallCompletion<'_, KeyType>, HostCallError> {
            *call.state() += 1;
            let value = call.create_external_with_binding::<Native>(42);
            Ok(call.return_value(value))
        }
        fn identity<'call>(
            mut call: HostCall<'call, Profile, Native, Item>,
            value: HostValue<'call, Item>,
        ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
            *call.state() += 1;
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
            let equal = call.equal::<KeyType>(left, right);
            Ok(call.return_value(equal))
        }
        let hosts = || {
            HostProviderSet::<Profile>::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_external_type::<Native, Key>()
                .unwrap()
                .with_scoped_function::<Native, (), KeyType, _>("make", make)
                .unwrap()
                .with_scoped_function::<Native, (Item,), Item, _>("identity", identity)
                .unwrap()
                .with_scoped_function::<Native, (KeyType, KeyType), bool, _>("same", same)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type Key
pub type Wrapped { Wrapped(Key) }
@external(erlang, "native", "make")
fn make() -> Key
@external(erlang, "native", "identity")
fn identity(value: a) -> a
@external(erlang, "native", "same")
fn same(left: Key, right: Key) -> Bool
fn tuple_key(value: #(Key)) { value.0 }
fn wrapped_key(value: Wrapped) { let Wrapped(key) = value key }
fn first_key(values: List(Key)) { let assert [first, ..] = values first }
pub fn main() {
  let key = identity(make())
  let keys = identity([key])
  let get = identity(fn() { key })
  let get_keys = identity(fn() { keys })
  let get_get = identity(fn() { get })
  let native_get = make
  let extracted = wrapped_key(Wrapped(tuple_key(#(get()))))
  let assert [first] = keys
  let assert [returned] = get_keys()
  echo key
  #(extracted == native_get(), same(first_key(keys), returned), first == get_get()())
}
"#;
        let (program, values, nevers) = lowered(source, hosts());
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
            sources: &sources,
            catalog: &catalog,
            constants: &common.constants,
        };
        let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
        assert_eq!(linked.tables(&context), Ok(()));
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );

        let missing_type =
            HostProviderSet::<Profile>::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Native, (), KeyType, _>("make", make)
                .unwrap()
                .with_scoped_function::<Native, (Item,), Item, _>("identity", identity)
                .unwrap()
                .with_scoped_function::<Native, (KeyType, KeyType), bool, _>("same", same)
                .unwrap()])
            .unwrap();
        let missing_type = NativeFunctions::new(&values, &nevers, missing_type).unwrap();
        assert_eq!(
            missing_type.tables(&context),
            Err(NativeError::ExternalType {
                package: "app".into(),
                module: "main".into(),
                name: "Key".into(),
                expected: 0,
                actual: None,
            })
        );

        use super::super::tests::owned_mut;
        use crate::plan::execution::function::{
            ExecutionFunctionBody, FunctionTableFamily, FunctionTables, ValueFunctionEntry,
        };
        use crate::plan::execution::host::HostedFunctionTarget;

        fn invalidate<Body: ExecutionFunctionBody>(
            entry: &mut ValueFunctionEntry<Body, HostedFunctionTarget<Body>>,
        ) -> bool {
            if let ValueFunctionEntry::Host(HostedFunctionTarget::Value(target)) = entry {
                target.index = 99;
                true
            } else {
                false
            }
        }
        type Corrupt = fn(&mut FunctionTables<HostedExecutionProfile>) -> Vec<usize>;
        let corruptions: [(
            FunctionTableFamily,
            Corrupt,
            functions::FunctionErrorKind<NativeError>,
        ); 5] = [
            (
                FunctionTableFamily::External,
                |tables| {
                    owned_mut(&mut tables.value_returns.external_functions)
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(index, entry)| invalidate(entry).then_some(index))
                        .collect()
                },
                functions::FunctionErrorKind::Host(NativeError::MissingValue(99)),
            ),
            (
                FunctionTableFamily::ExternalList,
                |tables| {
                    owned_mut(&mut tables.list_returns.external_list_functions)
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(index, (_, entry))| invalidate(entry).then_some(index))
                        .collect()
                },
                functions::FunctionErrorKind::Host(NativeError::MissingValue(99)),
            ),
            (
                FunctionTableFamily::ExternalFunction,
                |tables| {
                    owned_mut(&mut tables.function_returns.external_function_functions)
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(index, entry)| invalidate(entry).then_some(index))
                        .collect()
                },
                functions::FunctionErrorKind::Host(NativeError::MissingValue(99)),
            ),
            (
                FunctionTableFamily::ExternalListFunction,
                |tables| {
                    owned_mut(&mut tables.function_returns.external_list_function_functions)
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(index, entry)| invalidate(entry).then_some(index))
                        .collect()
                },
                functions::FunctionErrorKind::Host(NativeError::MissingValue(99)),
            ),
            (
                FunctionTableFamily::ExternalList,
                |tables| {
                    owned_mut(&mut tables.list_returns.external_list_functions)[0]
                        .0
                        .index = 99;
                    vec![0]
                },
                functions::FunctionErrorKind::ListIdentity,
            ),
        ];
        for (family, corrupt, kind) in corruptions {
            let (mut changed, values, nevers) = lowered(source, hosts());
            let invalid = corrupt(owned_mut(&mut changed.functions));
            assert!(!invalid.is_empty());
            let common = &changed.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &changed.functions, &types).unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            assert_eq!(linked.tables(&context), Ok(()));
            assert_eq!(
                functions::all(&changed.functions, &context, &linked),
                Err(functions::FunctionError {
                    family,
                    index: invalid[0],
                    kind,
                })
            );
        }

        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut state = 0;
        let mut echo = Vec::new();
        let host = crate::execution_fixture::TestHost::default();
        let value = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap();
        assert_eq!(
            value,
            crate::Value::Tuple(vec![
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true)
            ])
        );
        assert_eq!(state, 7);
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["src/main.gleam:23\nKey(42)"]
        );
    }

    #[test]
    fn never_native_targets_are_checked_and_preserve_their_source_failure() {
        use std::convert::Infallible;
        let hosts = || {
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "app", "main",
            )
            .unwrap()
            .with_fallible_function("stop", || -> Result<Infallible, crate::HostFailure> {
                Err(crate::HostFailure::new("native stopped"))
            })
            .unwrap()])
            .unwrap()
        };
        for source in [
            "@external(erlang, \"native\", \"stop\") fn stop() -> a pub fn main() { stop() }",
            "@external(erlang, \"native\", \"stop\") fn stop() -> a pub fn main() -> Int { stop() + 1 }",
        ] {
            let (program, values, nevers) = lowered(source, hosts());
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
                sources: &sources,
                catalog: &catalog,
                constants: &common.constants,
            };
            assert_eq!(nevers.len(), 1);
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            assert_eq!(linked.tables(&context), Ok(()));
            assert_eq!(
                functions::all(&program.functions, &context, &linked),
                Ok(())
            );
            use super::super::call::Target;
            let declaration = common.main.resolve(&catalog, &types).unwrap();
            assert_eq!(
                linked.never(
                    &crate::plan::execution::host::HostNeverFunctionId(999),
                    &declaration,
                    &context
                ),
                Err(NativeError::MissingNever(999)),
            );
            let mut changed = common.main.resolve(&catalog, &types).unwrap();
            changed.return_type = &crate::plan::execution::type_::ValueType::Bool;
            assert_eq!(
                linked.never(
                    &crate::plan::execution::host::HostNeverFunctionId(0),
                    &changed,
                    &context
                ),
                Err(NativeError::Call(ContractError::Signature)),
            );
            drop(linked);
            let mut invalid = nevers;
            invalid[0].signature.arguments = Table::Static(&[TypeMetadata::Int]);
            let invalid_link = NativeFunctions::new(&values, &invalid, hosts()).unwrap();
            assert_eq!(
                invalid_link.tables(&context),
                Err(NativeError::Contract {
                    value: false,
                    index: 0,
                    reason: ContractError::Signature,
                })
            );
            drop(invalid_link);
            invalid[0].signature.arguments = Table::Static(&[]);
            let span = crate::plan::SourceSpan::new(source.len() + 1, source.len() + 2);
            invalid[0].site = crate::plan::HostCallSite::from_static("main", "stop", span);
            let invalid_link = NativeFunctions::new(&values, &invalid, hosts()).unwrap();
            assert_eq!(
                invalid_link.tables(&context),
                Err(NativeError::Contract {
                    value: false,
                    index: 0,
                    reason: ContractError::Source(super::super::source::SourceError::SpanBounds {
                        module: "main".into(),
                        span,
                    }),
                })
            );
            let typed = crate::compile_typed_host_program(
                "app",
                "main",
                [PackageSource::new(
                    "app",
                    Vec::<&str>::new(),
                    [ModuleSource::new("main", "src/main.gleam", source)],
                )],
                hosts(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let error = crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
                .unwrap_err();
            assert!(matches!(error, crate::ExecutionError::Host(error)
                if error.package() == "app" && error.module() == "main"
                    && error.function() == "stop" && error.failure().message() == "native stopped"));
        }
    }

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    fn scalar_hosts() -> HostProviderSet {
        HostProviderSet::from_providers([HostProviderModule::new("app", "main")
            .unwrap()
            .with_function("double", |value: BigInt| value * 2)
            .unwrap()])
        .unwrap()
    }

    pub(super) fn lowered<Profile: crate::HostProfile>(
        source: &str,
        hosts: HostProviderSet<Profile>,
    ) -> (
        crate::plan::execution::ExecutionProgram<HostedExecutionProfile>,
        Vec<HostedFunctionMetadata>,
        Vec<HostedFunctionMetadata>,
    ) {
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts,
        )
        .unwrap();
        let (program, functions) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(typed).unwrap(),
        )
        .unwrap();
        let (values, nevers) = functions.into_metadata();
        let values = values
            .into_vec()
            .into_iter()
            .map(|metadata| Arc::try_unwrap(metadata).ok().unwrap())
            .collect();
        let nevers = nevers
            .into_vec()
            .into_iter()
            .map(|metadata| Arc::try_unwrap(metadata).ok().unwrap())
            .collect();
        (program, values, nevers)
    }

    #[test]
    fn value_targets_require_an_inhabited_result_and_the_declared_return_slot() {
        use crate::plan::execution::function::{FunctionTableFamily, ValueFunctionEntry};
        use crate::plan::execution::graph::IntLocalId;
        use crate::plan::execution::host::HostedFunctionTarget;
        use crate::plan::execution::prepared::admission::{tests::owned_mut, type_::TypeError};
        use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId, ValueType};

        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        type Item = HostTypeParameter<0>;
        fn unavailable(
            mut call: HostCall<'_, StatelessHostProfile, Provider, Item>,
        ) -> Result<HostCallCompletion<'_, Item>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Err(crate::HostFailure::new("no value").into())
        }
        let providers = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Provider, (), Item, _>("make", unavailable)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "make") fn make() -> a
pub fn main() { echo fn(value) { value } echo True make() + 1 }
"#;
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Change {
            None,
            Missing,
            ReturnSlot,
            Signature,
            Uninhabited,
        }
        for (change, expected) in [
            (Change::None, Ok(())),
            (Change::Missing, Err(NativeError::MissingValue(99))),
            (
                Change::ReturnSlot,
                Err(NativeError::Call(ContractError::Type(
                    TypeError::LocalTypeMismatch,
                ))),
            ),
            (
                Change::Signature,
                Err(NativeError::Call(ContractError::Signature)),
            ),
            (
                Change::Uninhabited,
                Err(NativeError::Call(ContractError::Signature)),
            ),
        ] {
            let (mut program, mut values, nevers) = lowered(source, providers());
            let targets = program
                .functions
                .value_returns
                .int_functions
                .iter()
                .enumerate()
                .filter_map(|(index, entry)| match entry {
                    ValueFunctionEntry::Host(HostedFunctionTarget::Value(target)) => {
                        Some((index, *target))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(targets.len(), 1);
            let (index, mut target) = targets[0];
            match change {
                Change::None => {}
                Change::Missing => target.index = 99,
                Change::ReturnSlot => target.return_ = IntLocalId(99),
                Change::Signature | Change::Uninhabited => {
                    let common = Arc::get_mut(&mut program.common).unwrap();
                    let shape = if change == Change::Uninhabited {
                        let symbolic_shapes = common
                            .value_shapes
                            .shapes
                            .iter()
                            .enumerate()
                            .filter_map(|(index, shape)| match shape {
                                ValueShapeDescriptor::Parameter(_) => Some(ValueShapeId(index)),
                                _ => None,
                            })
                            .collect::<Vec<_>>();
                        assert_eq!(symbolic_shapes.len(), 1);
                        let symbolic = symbolic_shapes[0];
                        values[0].type_arguments =
                            vec![crate::plan::execution::host::HostTypeArgument {
                                type_: TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
                                shape: symbolic,
                            }]
                            .into();
                        values[0].signature.return_ =
                            Node::Static(&TypeMetadata::Parameter(crate::plan::TypeParameterId(0)));
                        values[0].type_.return_ =
                            Node::Static(&ValueType::Parameter(crate::plan::TypeParameterId(0)));
                        symbolic
                    } else {
                        ValueShapeId(
                            common
                                .value_shapes
                                .shapes
                                .iter()
                                .position(|shape| shape == &ValueShapeDescriptor::Bool)
                                .unwrap(),
                        )
                    };
                    let catalog = Arc::get_mut(&mut common.function_parameters).unwrap();
                    let slot = catalog.families[FunctionTableFamily::Int as usize].start + index;
                    owned_mut(&mut catalog.functions)[slot].return_ = shape;
                }
            }
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
            let linked = NativeFunctions::new(&values, &nevers, providers()).unwrap();
            assert_eq!(linked.tables(&context), Ok(()));
            assert_eq!(
                linked.function(
                    &HostedFunctionTarget::Value(target),
                    &catalog.function(FunctionTableFamily::Int, index).unwrap(),
                    &context
                ),
                expected
            );
        }
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            providers(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
                .unwrap_err()
                .to_string(),
            "host function app::main.make failed: no value"
        );
    }

    #[test]
    fn checks_native_declarations_then_sealed_signatures_layouts_and_outputs() {
        let source = r#"
@external(erlang, "native", "double")
fn double(value: Int) -> Int
pub fn main() { double(21) }
"#;
        let (program, mut values, nevers) = lowered(source, scalar_hosts());
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let context = Instructions {
            types: &types,
            sources: &sources,
            catalog: &catalog,
            constants: &common.constants,
        };
        let linked = NativeFunctions::new(&values, &nevers, scalar_hosts()).unwrap();
        assert_eq!(linked.tables(&context), Ok(()));
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );
        drop(linked);

        values[0].type_arguments = vec![crate::plan::execution::host::HostTypeArgument {
            type_: TypeMetadata::Int,
            shape: crate::plan::execution::type_::ValueShapeId(0),
        }]
        .into();
        let linked = NativeFunctions::new(&values, &nevers, scalar_hosts()).unwrap();
        assert_eq!(
            linked.tables(&context),
            Err(NativeError::Contract {
                value: true,
                index: 0,
                reason: ContractError::TypeArguments
            })
        );
        drop(linked);
        values[0].type_arguments = Table::Static(&[]);
        values[0].signature.return_ = Node::Static(&TypeMetadata::Bool);
        let linked = NativeFunctions::new(&values, &nevers, scalar_hosts()).unwrap();
        assert_eq!(
            linked.tables(&context),
            Err(NativeError::Contract {
                value: true,
                index: 0,
                reason: ContractError::Signature
            })
        );
        drop(linked);
        values[0].signature.return_ = Node::Static(&TypeMetadata::Int);
        values[0].parameters.call = vec![crate::plan::execution::host::HostCallParameter::Bool(
            crate::plan::execution::graph::BoolLocalId(0),
        )]
        .into();
        let linked = NativeFunctions::new(&values, &nevers, scalar_hosts()).unwrap();
        assert_eq!(
            linked.tables(&context),
            Err(NativeError::Contract {
                value: true,
                index: 0,
                reason: ContractError::Parameters
            })
        );
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            scalar_hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
    }

    #[test]
    fn original_contract_mismatch_is_distinct_from_absent_registration() {
        let (_, values, nevers) = lowered(
            r#"
@external(erlang, "native", "double")
fn double(value: Int) -> Int
pub fn main() { double(21) }
"#,
            scalar_hosts(),
        );
        let wrong: HostProviderSet =
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_function("double", |value: bool| value)
                .unwrap()])
            .unwrap();
        assert_eq!(
            NativeFunctions::new(&values, &nevers, wrong).err(),
            Some(NativeError::Registration {
                package: "app".into(),
                module: "main".into(),
                function: "double".into(),
                reason: RegistrationError::Declaration,
            })
        );
        let absent = HostProviderSet::<StatelessHostProfile>::from_providers([]).unwrap();
        assert_eq!(
            NativeFunctions::new(&values, &nevers, absent).err(),
            Some(NativeError::Registration {
                package: "app".into(),
                module: "main".into(),
                function: "double".into(),
                reason: RegistrationError::Missing,
            })
        );
    }

    #[test]
    fn admits_each_generic_specialization_without_recomputing_the_execution_program() {
        type Item = HostTypeParameter<0>;
        fn identity<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Item>,
            value: HostValue<'call, Item>,
        ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
            assert_eq!(call.state(), &());
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Provider, (Item,), Item, _>("identity", identity)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type Box(a) { Box(a) }
@external(erlang, "native", "identity")
fn identity(value: a) -> a
fn unchanged(value) { value }
fn stop(_value: Int) -> a { panic }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<65>> value }
fn scalars() {
  #(identity(42), identity(1.5), identity("text"), identity(True), identity(Nil),
    identity(<<1>>), identity(codepoint()), identity(#(1, True)), identity(Box(7)))
}
fn lists() {
  #(identity([42]), identity([1.5]), identity(["text"]), identity([True]), identity([Nil]),
    identity([<<1>>]), identity([codepoint()]), identity([#(1, True)]), identity([Box(7)]),
    identity([]), identity([[]]), identity([[42]]), identity([unchanged]))
}
fn functions() {
  #(identity(fn() { 42 }), identity(fn() { 1.5 }), identity(fn() { "text" }),
    identity(fn() { True }), identity(fn() { Nil }), identity(fn() { <<1>> }),
    identity(fn() { codepoint() }), identity(fn() { #(1, True) }),
    identity(fn() { Box(7) }), identity(unchanged), identity(stop))
}
fn list_functions() {
  #(identity(fn() { [42] }), identity(fn() { [1.5] }), identity(fn() { ["text"] }),
    identity(fn() { [True] }), identity(fn() { [Nil] }), identity(fn() { [<<1>>] }),
    identity(fn() { [codepoint()] }), identity(fn() { [#(1, True)] }),
    identity(fn() { [Box(7)] }), identity(fn() { [] }), identity(fn() { [[]] }),
    identity(fn() { [[42]] }), identity(fn() { [unchanged] }))
}
pub fn main() {
  let _ = #(scalars(), lists(), functions(), list_functions(), identity(fn() { unchanged }))
  42
}
"#;
        let (program, values, nevers) = lowered(source, hosts());
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let context = Instructions {
            types: &types,
            sources: &sources,
            catalog: &catalog,
            constants: &common.constants,
        };
        let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
        assert_eq!(values.len(), 47);
        assert_eq!(linked.tables(&context), Ok(()));
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let host = crate::execution_fixture::TestHost::default();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut (), &mut Vec::new()))
                .unwrap(),
            crate::Value::Int(42.into()),
        );
    }
}
