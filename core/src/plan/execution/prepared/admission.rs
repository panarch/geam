mod block;
mod body;
mod call;
mod callables;
mod catalog;
mod compiled;
mod constant;
mod control;
mod edge;
mod entry;
mod error;
mod functions;
mod guard;
mod hosts;
mod incoming;
mod input;
mod instruction;
mod literal;
mod local;
mod operand;
mod pattern;
mod place;
mod source;
mod terminator;
mod transfer;
mod type_;

use super::{FORMAT_VERSION, ModuleArtifact, ProgramTables};
use crate::plan::execution::compiled::CompiledCallbackBodies;
use crate::plan::execution::function::ExecutionProfile;
use crate::plan::execution::graph::ExternalListInstructionView;

pub use error::PreparedError;

pub(crate) struct AdmittedHostedModule<Profile: crate::HostProfile> {
    pub(crate) program:
        AdmittedModule<'static, crate::plan::execution::host::HostedExecutionProfile>,
    hosts: crate::plan::execution::host::HostFunctionTables<Profile>,
    callables: &'static [crate::plan::execution::LibraryNativeConstruction],
}

pub(crate) struct AdmittedModule<'data, Profile: ExecutionProfile> {
    artifact: &'data ModuleArtifact<Profile>,
    views: AdmittedViews<'data>,
    compiled_callback_bodies: CompiledCallbackBodies<'data, Profile>,
}

struct AdmittedViews<'data> {
    exports: &'data [super::Export],
    types: type_::Types<'data>,
    inputs: Vec<&'data crate::plan::execution::LibraryInputConstructions>,
    callables: Vec<&'data [crate::plan::execution::LibraryCallable]>,
}

#[derive(Debug, PartialEq, Eq)]
enum SelectionError {
    Binding(crate::embedding::BindingError),
    Callable {
        name: ecow::EcoString,
        error: callables::CallableError,
    },
    Input {
        name: ecow::EcoString,
        error: input::InputError,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum Error<HostError> {
    Types(type_::TypeError),
    Sources(source::SourceError),
    Catalog(catalog::CatalogError),
    Hosts(HostError),
    Functions(functions::FunctionError<HostError>),
    Compiled(compiled::CompiledError),
    Constants(constant::ConstantBodyError),
    Entries(entry::EntryError),
    Main(entry::MainError),
}

#[derive(Debug, PartialEq, Eq)]
struct FormatError {
    expected: u32,
    found: u32,
}

pub(super) fn plain(
    artifact: &ModuleArtifact<std::convert::Infallible>,
) -> Result<AdmittedModule<'_, std::convert::Infallible>, PreparedError> {
    format(artifact.format)?;
    module(artifact, &functions::InfallibleHosts).map_err(PreparedError::from)
}

pub(super) fn hosted<Profile: crate::HostProfile>(
    artifact: &'static super::HostedModuleArtifact,
    providers: crate::HostProviderSet<Profile>,
) -> Result<AdmittedHostedModule<Profile>, PreparedError> {
    format(artifact.module.format)?;
    let hosts = hosts::NativeFunctions::new(
        &artifact.value_functions,
        &artifact.never_functions,
        providers,
    )
    .map_err(PreparedError::from)?;
    let (types, catalog, compiled_callback_bodies) =
        program(&artifact.module.program, &hosts).map_err(PreparedError::from)?;
    hosts
        .library_callables(&artifact.callables, &catalog, &types)
        .map_err(PreparedError::from)?;
    let program = module_entries::<_, hosts::NativeError>(
        &artifact.module,
        types,
        &catalog,
        compiled_callback_bodies,
    )
    .map_err(PreparedError::from)?;
    Ok(AdmittedHostedModule {
        program,
        hosts: hosts.into_tables(),
        callables: &artifact.callables,
    })
}

pub(super) fn hosted_entry<Profile: crate::HostProfile>(
    artifact: &'static super::HostedEntryArtifact,
    providers: crate::HostProviderSet<Profile>,
) -> Result<crate::HostedExecution<Profile>, PreparedError> {
    format(artifact.format)?;
    let hosts = hosts::NativeFunctions::new(
        &artifact.value_functions,
        &artifact.never_functions,
        providers,
    )
    .map_err(PreparedError::from)?;
    let (types, catalog, compiled_callback_bodies) =
        program(&artifact.program, &hosts).map_err(PreparedError::from)?;
    entry::main(&artifact.program.main, &catalog, &types)
        .map_err(|error| PreparedError::from(Error::<hosts::NativeError>::Main(error)))?;
    Ok(crate::HostedExecution::from_program(
        crate::plan::execution::HostedProgram {
            program: artifact.program.execution(compiled_callback_bodies),
            host_functions: hosts.into_tables(),
        },
    ))
}

fn format(found: u32) -> Result<(), FormatError> {
    if found != FORMAT_VERSION {
        return Err(FormatError {
            expected: FORMAT_VERSION,
            found,
        });
    }
    Ok(())
}

fn module<'data, Profile: ExecutionProfile, Host: functions::Hosts<Profile>>(
    artifact: &'data ModuleArtifact<Profile>,
    hosts: &Host,
) -> Result<AdmittedModule<'data, Profile>, Error<Host::Error>>
where
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::RuntimeFunctionFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::InvocableFunctionFunctionId: call::Target,
    <<Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: operand::Operand,
{
    let (types, catalog, compiled_callback_bodies) = program(&artifact.program, hosts)?;
    module_entries(artifact, types, &catalog, compiled_callback_bodies)
}

fn module_entries<'data, Profile: ExecutionProfile, HostError>(
    artifact: &'data ModuleArtifact<Profile>,
    types: type_::Types<'data>,
    catalog: &catalog::Catalog<'data>,
    compiled_callback_bodies: CompiledCallbackBodies<'data, Profile>,
) -> Result<AdmittedModule<'data, Profile>, Error<HostError>>
where
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::RuntimeFunctionFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::InvocableFunctionFunctionId: call::Target,
{
    let (inputs, callables) = entry::all(
        &artifact.program.main,
        &artifact.entries,
        &artifact.exports,
        catalog,
        &types,
    )
    .map_err(Error::Entries)?;
    Ok(AdmittedModule {
        artifact,
        compiled_callback_bodies,
        views: AdmittedViews {
            exports: &artifact.exports,
            types,
            inputs,
            callables,
        },
    })
}

fn program<'data, Profile: ExecutionProfile, Host: functions::Hosts<Profile>>(
    program: &'data ProgramTables<Profile>,
    hosts: &Host,
) -> Result<
    (
        type_::Types<'data>,
        catalog::Catalog<'data>,
        CompiledCallbackBodies<'data, Profile>,
    ),
    Error<Host::Error>,
>
where
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionId: call::Target,
    <Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionId: call::Target,
    <<Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: operand::Operand,
{
    let types = type_::Types::admit(
        &program.list_types,
        &program.custom_types,
        &program.external_types,
        &program.value_shapes,
    )
    .map_err(Error::Types)?;
    let sources = source::Sources::admit(program.root, &program.modules).map_err(Error::Sources)?;
    let catalog = catalog::Catalog::admit(&program.function_parameters, &program.functions, &types)
        .map_err(Error::Catalog)?;
    let context = instruction::Instructions {
        types: &types,
        sources: &sources,
        catalog: &catalog,
        constants: &program.constants,
    };
    hosts.tables(&context).map_err(Error::Hosts)?;
    functions::all(&program.functions, &context, hosts).map_err(Error::Functions)?;
    let compiled_callback_bodies =
        compiled::admit(&program.compiled, &program.functions, &program.custom_types)
            .map_err(Error::Compiled)?;
    hosts.callables(&context).map_err(Error::Hosts)?;
    constant::all(&program.constants, &context).map_err(Error::Constants)?;
    Ok((types, catalog, compiled_callback_bodies))
}

impl<Profile: crate::HostProfile> AdmittedHostedModule<Profile> {
    pub(crate) fn select_callable(
        &self,
        declaration: &crate::host::RegisteredCallableConstruction,
        signature: &crate::plan::LibraryNativeSignature,
        standard: &[crate::plan::StandardVariant],
    ) -> Result<usize, PreparedError> {
        let mut mismatch = None;
        for (slot, entry) in self
            .callables
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.declaration.matches(declaration))
        {
            match self.callable_mapping(entry, declaration, signature, standard) {
                Ok(()) => return Ok(slot),
                Err(error) => mismatch = Some(error),
            }
        }
        Err(mismatch.unwrap_or_else(|| {
            PreparedError::from(SelectionError::Binding(
                crate::embedding::BindingError::NativeCallable {
                    package: declaration.identity.package.clone(),
                    module: declaration.identity.module.clone(),
                    name: declaration.identity.name.clone(),
                },
            ))
        }))
    }

    fn callable_mapping(
        &self,
        entry: &crate::plan::execution::LibraryNativeConstruction,
        declaration: &crate::host::RegisteredCallableConstruction,
        signature: &crate::plan::LibraryNativeSignature,
        standard: &[crate::plan::StandardVariant],
    ) -> Result<(), PreparedError> {
        let name = declaration.identity.name.clone();
        callables::mapping(
            std::slice::from_ref(&entry.invocation),
            std::slice::from_ref(&signature.invocation),
            &self.program.views.types,
        )
        .map_err(|error| {
            PreparedError::from(SelectionError::Callable {
                name: name.clone(),
                error,
            })
        })?;
        input::mapping(
            &entry.captures,
            &signature.capture_variants,
            &signature.capture_lists,
            standard,
            &self.program.views.types,
        )
        .map_err(|error| {
            PreparedError::from(SelectionError::Input {
                name: name.clone(),
                error,
            })
        })?;
        for (expected, actual) in signature
            .captures
            .iter()
            .zip(entry.construction.captures.iter())
        {
            let actual = &self.program.views.types.shape_types()[actual.shape().index()];
            if !self.program.views.types.metadata_matches_value(
                &crate::plan::execution::type_::TypeMetadata::from_public(expected),
                actual,
            ) {
                return Err(PreparedError::from(SelectionError::Callable {
                    name,
                    error: callables::CallableError::Signature,
                }));
            }
        }
        if signature.captures.len() != entry.construction.captures.len() {
            return Err(PreparedError::from(SelectionError::Callable {
                name,
                error: callables::CallableError::Signature,
            }));
        }
        Ok(())
    }

    pub(crate) fn into_execution(
        self,
    ) -> (
        crate::HostedExecution<Profile>,
        crate::plan::execution::LibraryFunctionEntries,
        crate::plan::execution::storage::Table<crate::plan::execution::LibraryNativeConstruction>,
    ) {
        let artifact = self.program.artifact;
        let execution = crate::plan::execution::HostedProgram {
            program: artifact
                .program
                .execution(self.program.compiled_callback_bodies),
            host_functions: self.hosts,
        };
        (
            crate::HostedExecution::from_program(execution),
            artifact.entries.borrowed(),
            crate::plan::execution::storage::Table::Static(self.callables),
        )
    }
}

impl<Profile: ExecutionProfile> AdmittedModule<'_, Profile> {
    pub(crate) fn select(
        &self,
        name: ecow::EcoString,
        expected: crate::plan::FunctionType,
        variants: &[crate::plan::LibraryVariant],
        lists: &[crate::plan::LibraryValueType],
        standard: &[crate::plan::StandardVariant],
        callables: &[crate::plan::LibraryCallableSignature],
    ) -> Result<usize, super::PreparedError> {
        self.views
            .select(name, expected, variants, lists, standard, callables)
    }
}

impl AdmittedViews<'_> {
    fn select(
        &self,
        name: ecow::EcoString,
        expected: crate::plan::FunctionType,
        variants: &[crate::plan::LibraryVariant],
        lists: &[crate::plan::LibraryValueType],
        standard: &[crate::plan::StandardVariant],
        callables: &[crate::plan::LibraryCallableSignature],
    ) -> Result<usize, super::PreparedError> {
        use crate::embedding::BindingError;

        let Some((index, export)) = self
            .exports
            .iter()
            .enumerate()
            .find(|(_, export)| export.name.as_str() == name.as_str())
        else {
            return Err(super::PreparedError::from(SelectionError::Binding(
                BindingError::MissingFunction { name },
            )));
        };
        let signature = &export.signature;
        if signature.arguments.len() != expected.argument_types().len()
            || signature
                .arguments
                .iter()
                .zip(expected.argument_types())
                .any(|(left, right)| !left.compare(right).is_eq())
            || !signature.return_.compare(expected.return_()).is_eq()
        {
            return Err(super::PreparedError::from(SelectionError::Binding(
                BindingError::SignatureMismatch {
                    name,
                    expected,
                    found: signature.materialize(),
                },
            )));
        }
        input::mapping(self.inputs[index], variants, lists, standard, &self.types).map_err(
            |error| {
                super::PreparedError::from(SelectionError::Input {
                    name: name.clone(),
                    error,
                })
            },
        )?;
        callables::mapping(self.callables[index], callables, &self.types).map_err(|error| {
            super::PreparedError::from(SelectionError::Callable { name, error })
        })?;
        Ok(export.slot)
    }
}

impl AdmittedModule<'static, std::convert::Infallible> {
    pub(crate) fn into_execution(
        self,
    ) -> (
        crate::ExecutionPlan,
        crate::plan::execution::LibraryFunctionEntries<std::convert::Infallible>,
    ) {
        (
            crate::ExecutionPlan {
                program: self
                    .artifact
                    .program
                    .execution(self.compiled_callback_bodies),
            },
            self.artifact.entries.borrowed(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, functions, module, plain};
    use crate::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostExternalBinding, HostExternalEquality,
        HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
        HostExternalStore, HostExternalType, HostProfile, HostProvider, HostProviderModule,
        HostProviderSet,
    };
    use crate::plan::execution::graph::{Match, MatchPattern, ProfiledInstruction, Terminator};
    use crate::plan::execution::host::{HostedExecutionProfile, HostedFunctionMetadata};
    use crate::plan::execution::prepared::{
        FORMAT_VERSION, ModuleArtifact, PreparedModule, ProgramTables,
    };
    use crate::plan::execution::storage::Storage;
    use crate::runtime::compiled::tests::metadata_numeric;
    use std::convert::Infallible;
    use std::sync::Arc;

    pub(super) struct NativeProfile;
    impl HostProfile for NativeProfile {
        type RunState = ();
        type ExternalStores = HostExternalStore<u8>;
        type ExecutionState = ();
    }

    struct Native;
    impl HostProvider<NativeProfile> for Native {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
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
    impl HostExternalBinding<NativeProfile, Key> for Native {
        type Storage = Self;
    }
    impl HostExternalStorage<NativeProfile, Key> for Native {
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

    pub(super) fn native_hosts() -> HostProviderSet<NativeProfile> {
        fn key(
            mut call: HostCall<'_, NativeProfile, Native, HostExternalType<Key>>,
        ) -> Result<HostCallCompletion<'_, HostExternalType<Key>>, HostCallError> {
            let _ = call.state();
            let value = call.create_external_with_binding::<Native>(42);
            let same = call.create_external_with_binding::<Native>(42);
            let different = call.create_external_with_binding::<Native>(43);
            assert_eq!(
                call.source_hash::<HostExternalType<Key>>(value),
                call.source_hash::<HostExternalType<Key>>(same)
            );
            assert!(call.equal::<HostExternalType<Key>>(value, same));
            assert!(!call.equal::<HostExternalType<Key>>(value, different));
            Ok(call.return_value(value))
        }
        HostProviderSet::from_providers([HostProviderModule::new("app", "main")
            .unwrap()
            .with_external_type::<Native, Key>()
            .unwrap()
            .with_scoped_function::<Native, (), HostExternalType<Key>, _>("key", key)
            .unwrap()])
        .unwrap()
    }

    pub(super) fn lowered_native(
        source: &str,
    ) -> (
        crate::plan::execution::ExecutionProgram<HostedExecutionProfile>,
        Vec<HostedFunctionMetadata>,
        Vec<HostedFunctionMetadata>,
    ) {
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            native_hosts(),
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
            .map(std::sync::Arc::try_unwrap)
            .collect::<Result<Vec<_>, _>>()
            .ok()
            .unwrap();
        let nevers = nevers
            .into_vec()
            .into_iter()
            .map(std::sync::Arc::try_unwrap)
            .collect::<Result<Vec<_>, _>>()
            .ok()
            .unwrap();
        (program, values, nevers)
    }

    #[test]
    fn native_function_table_fixture_executes_its_declared_key_contract() {
        let source = "pub type Key\n@external(erlang, \"native\", \"key\") fn key() -> Key\npub fn main() { echo key() Nil }";
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            native_hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
            crate::Value::Nil
        );
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["src/main.gleam:3\nKey(42)"]
        );
    }

    #[test]
    fn nested_list_guards_exclude_only_independently_contradicted_paths() {
        use super::body::BodyError;
        use super::functions::{FunctionError, FunctionErrorKind};
        use super::guard::GuardError;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::graph::{
            BoolTest, Jump, ListListLocalId, Terminator, TestBranch,
        };

        let source = r#"
pub type Tree { Bytes(Int) Text(Int) Many(List(Tree)) }
fn to_list(stack: List(List(Tree)), acc: List(Int)) -> List(Int) {
  case stack {
    [] -> acc
    [[], ..remaining] -> to_list(remaining, acc)
    [[Bytes(value), ..rest], ..remaining] -> to_list([rest, ..remaining], [value, ..acc])
    [[Text(value), ..rest], ..remaining] -> to_list([rest, ..remaining], [value, ..acc])
    [[Many(trees), ..rest], ..remaining] -> to_list([trees, rest, ..remaining], acc)
  }
}
pub fn main() {
  to_list([[Bytes(1), Many([]), Text(2), Many([Bytes(3)])]], []) == [3, 2, 1]
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, function) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), bool>::new("main"))
            .unwrap();
        assert!(
            bindings
                .seal()
                .call(&function, (), &mut Vec::new())
                .unwrap()
        );

        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), bool>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);

        // Without excluding an empty outer stack, the later nonempty checks
        // can fail. Their failure edges must still prevent an unguarded read.
        let (_, function) =
            &mut owned_mut(&mut artifact.program.functions.list_returns.int_list_functions)[0];
        let blocks = owned_mut(&mut function.body.block_graph.blocks);
        let branches = blocks
            .iter()
            .filter_map(|header| match &header.terminator {
                Terminator::TestBranch(TestBranch {
                    test: BoolTest::ListLengthEquals { length, .. },
                    false_,
                    ..
                }) => Some((*length, false_)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(branches[0].0, 0);
        blocks[0].terminator = Terminator::Jump(Jump::new(branches[0].1.clone()));
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(FunctionError {
                family: FunctionTableFamily::IntList,
                index: 0,
                kind: FunctionErrorKind::Body(BodyError::Guard {
                    block: 18,
                    index: 0,
                    error: GuardError::Unproved {
                        local: ListListLocalId(0).into(),
                        requirement: "at least 1 list elements".into(),
                    },
                }),
            }))
        );
    }

    #[test]
    fn admits_exhaustive_nested_constructor_failures() {
        let source = r#"
pub type Option(a) { Some(a) None }
fn inspect(value: Result(Option(Int), String)) -> String {
  case value {
    Ok(Some(_)) -> "present"
    Ok(None) -> "missing"
    Error(reason) -> reason
  }
}
pub fn main() { inspect(Ok(Some(42))) <> inspect(Ok(None)) <> inspect(Error("failed")) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, function) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), crate::StringValue>::new("main"))
            .unwrap();
        assert_eq!(
            bindings
                .seal()
                .call(&function, (), &mut Vec::new())
                .unwrap()
                .as_str(),
            "presentmissingfailed"
        );
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), crate::StringValue>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);

        // Repeating Some instead of excluding None leaves a possible Ok value.
        // Its Option payload must never be admitted as Error's String field.
        use crate::plan::execution::graph::Terminator;
        let function =
            &mut owned_mut(&mut artifact.program.functions.value_returns.string_functions)[1];
        let mut matches = owned_mut(&mut function.body.block_graph.blocks)
            .iter_mut()
            .filter_map(|header| match &mut header.terminator {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            });
        let repeated = matches.next().unwrap().pattern.clone();
        matches.next().unwrap().pattern = repeated;
        assert!(matches.next().is_none());
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(super::functions::FunctionError {
                family: crate::plan::execution::function::FunctionTableFamily::String,
                index: 1,
                kind: super::functions::FunctionErrorKind::Body(
                    super::body::BodyError::Instruction {
                        block: 4,
                        index: 0,
                        error: super::instruction::InstructionError::OutputType,
                    }
                ),
            }))
        );
    }

    #[test]
    fn nested_exclusions_preserve_hypotheses_through_match_bindings() {
        let source = r#"
pub type Option(a) { Some(a) None }
pub type Envelope { Value(Result(Option(Int), String)) Empty }
fn inspect(envelope: Envelope) -> String {
  case envelope {
    Value(value) -> case value {
      Ok(Some(_)) -> "present"
      Ok(None) -> "missing"
      Error(reason) -> reason
    }
    Empty -> "empty"
  }
}
pub fn main() { inspect(Value(Error("failed"))) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, function) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), crate::StringValue>::new("main"))
            .unwrap();
        assert_eq!(
            bindings
                .seal()
                .call(&function, (), &mut Vec::new())
                .unwrap()
                .as_str(),
            "failed"
        );
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), crate::StringValue>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);

        use crate::plan::execution::graph::Terminator;
        let function =
            &mut owned_mut(&mut artifact.program.functions.value_returns.string_functions)[1];
        let mut matches = owned_mut(&mut function.body.block_graph.blocks)
            .iter_mut()
            .filter_map(|header| match &mut header.terminator {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            });
        // Keep the outer Value binding and replace only the nested None case.
        matches.next().unwrap();
        let repeated = matches.next().unwrap().pattern.clone();
        matches.next().unwrap().pattern = repeated;
        assert!(matches.next().is_none());
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(super::functions::FunctionError {
                family: crate::plan::execution::function::FunctionTableFamily::String,
                index: 1,
                kind: super::functions::FunctionErrorKind::Body(
                    super::body::BodyError::Instruction {
                        block: 5,
                        index: 0,
                        error: super::instruction::InstructionError::OutputType,
                    }
                ),
            }))
        );
    }

    #[test]
    fn exhaustive_field_reads_require_the_unconstructed_variant_metadata() {
        let source = r#"
pub type Option(a) { Some(a) None }
pub type Builder { Builder(name: Option(String)) }
fn start(builder: Builder) -> String {
  case builder.name {
    None -> "unnamed"
    Some(name) -> name
  }
}
pub fn main() { start(Builder(None)) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), crate::StringValue>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);

        let option = owned_mut(&mut artifact.program.custom_types.types)
            .iter_mut()
            .find(|type_| type_.type_.name.as_str() == "Option")
            .unwrap();
        assert_eq!(
            option
                .constructors
                .iter()
                .map(|constructor| constructor.name.as_str())
                .collect::<Vec<_>>(),
            ["Some", "None"],
        );
        // A sparse table is valid only if it still describes every retained use.
        // Removing the never-constructed Some must not authorize its field read.
        option.constructors = option.constructors[1..].to_vec().into();
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(super::functions::FunctionError {
                family: crate::plan::execution::function::FunctionTableFamily::String,
                index: 1,
                kind: super::functions::FunctionErrorKind::Body(
                    super::body::BodyError::Instruction {
                        block: 2,
                        index: 0,
                        error: super::instruction::InstructionError::CustomField { index: 0 },
                    }
                ),
            }))
        );
    }

    #[test]
    fn nested_list_projections_require_the_original_list_storage_identity() {
        use super::body::BodyError;
        use super::functions::{FunctionError, FunctionErrorKind};
        use super::instruction::InstructionError;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::graph::{
            IntListLocalId, IntLocalId, ListInstruction, ListLocal, ParamLocal,
            ProfiledInstruction, ProfiledInstructionKind, TupleInstruction, TypedListInstruction,
        };
        use crate::plan::execution::type_::{
            IntListTypeId, ListStorageTypeId, ListTypeId, ValueShapeId, ValueType,
        };

        for (name, source, tuple) in [
            (
                "guard",
                r#"
pub fn main() {
  let expected = [42]
  let nested = [[42]]
  case nested {
    [first, ..] if first == expected -> True
    _ -> False
  }
}
"#,
                false,
            ),
            (
                "tuple",
                r#"
pub fn main() {
  let expected = [42]
  let nested = #([42])
  let first = nested.0
  first == expected
}
"#,
                true,
            ),
            (
                "binding",
                r#"
pub fn main() {
  let expected = [42]
  let nested = [[42]]
  let assert [first, ..] = nested
  first == expected
}
"#,
                false,
            ),
        ] {
            for (alias, linked) in [(false, false), (true, false), (true, true)] {
                let typed =
                    crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
                let (bindings, _) = ModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(), bool>::new("main"))
                    .unwrap();
                let mut artifact = artifact(bindings.prepare());
                if alias {
                    let graph = &mut owned_mut(
                        &mut artifact.program.functions.value_returns.bool_functions,
                    )[0]
                    .body
                    .block_graph;
                    let instructions = owned_mut(&mut graph.instructions);
                    let original = IntListTypeId {
                        list_type: ListTypeId(0),
                    };
                    assert_eq!(
                        instructions
                            .iter()
                            .enumerate()
                            .filter_map(|(index, instruction)| {
                                match &instruction.value().unwrap().kind {
                                    ProfiledInstructionKind::List(ListInstruction::Int(
                                        type_id,
                                        TypedListInstruction::Value(items),
                                    )) => Some((index, *type_id, items.to_vec())),
                                    _ => None,
                                }
                            })
                            .collect::<Vec<_>>(),
                        vec![
                            (1, original, vec![IntLocalId(0)]),
                            (3, original, vec![IntLocalId(1)])
                        ]
                    );
                    let original_child = ParamLocal::List(ListLocal::Int {
                        local: IntListLocalId(1),
                        type_id: original,
                    });
                    assert_eq!(
                        instructions
                            .iter()
                            .enumerate()
                            .filter_map(|(index, instruction)| {
                                match &instruction.value().unwrap().kind {
                                    ProfiledInstructionKind::Tuple(TupleInstruction::Value(
                                        items,
                                    )) => Some((index, items.to_vec())),
                                    _ => None,
                                }
                            })
                            .collect::<Vec<_>>(),
                        if tuple {
                            vec![(4, vec![original_child.clone()])]
                        } else {
                            Vec::new()
                        }
                    );
                    let mut child = instructions[3].value().unwrap().clone();
                    assert_eq!(child.output.local, original_child);
                    let alias_type = IntListTypeId {
                        list_type: ListTypeId(artifact.program.list_types.types.len()),
                    };
                    let mut list_types = artifact.program.list_types.types.to_vec();
                    list_types.push(ListStorageTypeId::Int(alias_type));
                    artifact.program.list_types.types = list_types.into();
                    let shapes = &mut artifact.program.value_shapes;
                    let alias_shape = ValueShapeId(shapes.shapes.len());
                    let mut descriptors = shapes.shapes.to_vec();
                    descriptors.push(descriptors[child.output.shape.index()].clone());
                    let mut shape_types = shapes.shape_types.to_vec();
                    shape_types.push(ValueType::List(alias_type.list_type));
                    shapes.shapes = descriptors.into();
                    shapes.shape_types = shape_types.into();

                    if linked {
                        // The child is locally well-typed. Only its enclosing
                        // container still requires the original storage identity.
                        child.kind = ProfiledInstructionKind::List(ListInstruction::Int(
                            alias_type,
                            TypedListInstruction::Value(vec![IntLocalId(1)].into()),
                        ));
                        child.output.shape = alias_shape;
                        child.output.local = ParamLocal::List(ListLocal::Int {
                            local: IntListLocalId(1),
                            type_id: alias_type,
                        });
                        let child_local = child.output.local.clone();
                        if tuple {
                            let mut tuple_instruction = instructions[4].value().unwrap().clone();
                            tuple_instruction.kind = ProfiledInstructionKind::Tuple(
                                TupleInstruction::Value(vec![child_local].into()),
                            );
                            instructions[4] = ProfiledInstruction::Value(tuple_instruction);
                        }
                    }
                    instructions[3] = ProfiledInstruction::Value(child);
                }
                let artifact = Box::leak(Box::new(artifact));
                let error = module(artifact, &functions::InfallibleHosts).err();
                if linked {
                    assert_eq!(
                        error,
                        Some(Error::Functions(FunctionError {
                            family: FunctionTableFamily::Bool,
                            index: 0,
                            kind: FunctionErrorKind::Body(BodyError::Instruction {
                                block: 0,
                                index: 4,
                                error: InstructionError::Flow,
                            }),
                        })),
                        "{name}"
                    );
                } else {
                    assert_eq!(error, None, "{name}");
                    let (execution, _) = artifact.admit().unwrap().into_execution();
                    assert_eq!(
                        crate::run_main(&execution, &mut Vec::new()).unwrap(),
                        crate::Value::Bool(true),
                        "{name}"
                    );
                }
            }
        }
    }

    #[test]
    fn nested_constructor_remainders_admit_the_final_payload() {
        use super::body::BodyError;
        use super::functions::{FunctionError, FunctionErrorKind};
        use super::instruction::InstructionError;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::graph::MatchPattern;
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

        let source = r#"
pub type Control { Ping }
pub type InternalMessage {
  ReceiveMessage(Int)
  Closed
  Passive
  SocketError(Int)
  Ready
  Close
}
pub type Message { Internal(InternalMessage) User(Control) }
fn choose(message: Message) -> Int {
  case message {
    Internal(Closed) | Internal(Close) -> 0
    Internal(Ready) -> 1
    User(_) -> 2
    Internal(ReceiveMessage(_)) -> 3
    Internal(Passive) -> 4
    Internal(SocketError(reason)) -> reason
  }
}
pub fn main() {
  let assert 0 = choose(Internal(Closed))
  let assert 0 = choose(Internal(Close))
  let assert 1 = choose(Internal(Ready))
  let assert 2 = choose(User(Ping))
  let assert 3 = choose(Internal(ReceiveMessage(13)))
  let assert 4 = choose(Internal(Passive))
  let assert 9 = choose(Internal(SocketError(9)))
  Nil
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), ()>::new("main"))
            .unwrap();
        let mut data = artifact(bindings.prepare());
        assert_eq!(module(&data, &functions::InfallibleHosts).err(), None);

        let graph = &mut owned_mut(&mut data.program.functions.value_returns.int_functions)[0]
            .body
            .block_graph;
        let original = owned_mut(&mut graph.blocks)[6].terminator.clone();
        let matcher = remainder_fixture_matcher(&mut owned_mut(&mut graph.blocks)[6].terminator);
        matcher.pattern = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(1),
                index: 0,
            },
            fields: vec![MatchPattern::Discard].into(),
        };
        assert_eq!(
            module(&data, &functions::InfallibleHosts).err(),
            Some(Error::Functions(FunctionError {
                family: FunctionTableFamily::Int,
                index: 0,
                kind: FunctionErrorKind::Body(BodyError::Instruction {
                    block: 12,
                    index: 0,
                    error: InstructionError::OutputType,
                }),
            }))
        );
        let graph = &mut owned_mut(&mut data.program.functions.value_returns.int_functions)[0]
            .body
            .block_graph;
        owned_mut(&mut graph.blocks)[6].terminator = original;
        let index = graph.blocks[12].instructions.start;
        let field_index =
            remainder_fixture_field_index(&mut owned_mut(&mut graph.instructions)[index]);
        assert_eq!(*field_index, 0);
        *field_index = 1;
        assert_eq!(
            module(&data, &functions::InfallibleHosts).err(),
            Some(Error::Functions(FunctionError {
                family: FunctionTableFamily::Int,
                index: 0,
                kind: FunctionErrorKind::Body(BodyError::Instruction {
                    block: 12,
                    index: 0,
                    error: InstructionError::CustomField { index: 1 },
                }),
            }))
        );
        let graph = &mut owned_mut(&mut data.program.functions.value_returns.int_functions)[0]
            .body
            .block_graph;
        *remainder_fixture_field_index(&mut owned_mut(&mut graph.instructions)[index]) = 0;

        let (execution, _) = plain(Box::leak(Box::new(data))).unwrap().into_execution();
        for _ in 0..2 {
            let mut echo = Vec::new();
            assert_eq!(
                crate::run_main(&execution, &mut echo).unwrap(),
                crate::Value::Nil
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn symbolic_constructor_remainders_admit_fixed_payloads() {
        let source = r#"
pub type InternalMessage {
  Close
  Ready
  ReceiveMessage(Int)
  Closed
  Passive
  SocketError(Int)
}

pub type Message(user) {
  Internal(InternalMessage)
  User(user)
}

fn choose(message: Message(user)) -> Int {
  case message {
    Internal(Closed) | Internal(Close) -> 0
    Internal(Ready) -> 1
    User(_) -> 2
    Internal(ReceiveMessage(_)) -> 3
    Internal(Passive) -> 4
    Internal(SocketError(reason)) -> reason
  }
}

pub fn main() {
  let assert 0 = choose(Internal(Closed))
  let assert 0 = choose(Internal(Close))
  let assert 1 = choose(Internal(Ready))
  let assert 2 = choose(User(Nil))
  let assert 3 = choose(Internal(ReceiveMessage(13)))
  let assert 4 = choose(Internal(Passive))
  let assert 9 = choose(Internal(SocketError(9)))
  Nil
}
"#;
        for (source, stored_symbolic_variant) in [
            (source.to_string(), false),
            (source.to_string(), true),
            (
                source.replace("message: Message(user)", "message: Message(Nil)"),
                false,
            ),
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let (bindings, _) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), ()>::new("main"))
                .unwrap();
            let mut data = artifact(bindings.prepare());
            if stored_symbolic_variant {
                use crate::plan::TypeParameterId;
                use crate::plan::execution::type_::custom::FieldRefinement;
                use crate::plan::execution::type_::{
                    CustomConstructorDescriptor, CustomConstructorId, CustomFieldDescriptor,
                    CustomTypeId, TypeMetadata, ValueShapeDescriptor,
                };

                // Other retained signatures may require the symbolic User layout
                // even though this specialization cannot construct a User payload.
                let type_index = data
                    .program
                    .custom_types
                    .types
                    .iter()
                    .position(|type_| {
                        type_.type_.name.as_str() == "Message"
                            && type_
                                .type_
                                .arguments
                                .iter()
                                .any(|argument| matches!(argument, TypeMetadata::Parameter(_)))
                    })
                    .unwrap();
                let parameter_shape = data
                    .program
                    .value_shapes
                    .custom_shapes
                    .iter()
                    .find(|shape| shape.type_id.index() == type_index)
                    .unwrap()
                    .arguments[0];
                assert_eq!(
                    data.program.value_shapes.shapes[parameter_shape.index()],
                    ValueShapeDescriptor::Parameter(TypeParameterId(0))
                );
                let field_type =
                    data.program.value_shapes.shape_types[parameter_shape.index()].clone();
                let type_ = &mut owned_mut(&mut data.program.custom_types.types)[type_index];
                assert_eq!(type_.constructors.len(), 1);
                let mut constructors = type_.constructors.to_vec();
                constructors.push(CustomConstructorDescriptor {
                    id: CustomConstructorId {
                        type_id: CustomTypeId(type_index),
                        index: 1,
                    },
                    name: "User".into(),
                    native_tag: "user".into(),
                    fields: vec![CustomFieldDescriptor {
                        label: None,
                        type_: field_type,
                        shape: parameter_shape,
                        refinement: FieldRefinement::Argument(0),
                    }]
                    .into(),
                });
                type_.constructors = constructors.into();
            }
            assert_eq!(module(&data, &functions::InfallibleHosts).err(), None);
            let (execution, _) = plain(Box::leak(Box::new(data))).unwrap().into_execution();
            for _ in 0..2 {
                let mut echo = Vec::new();
                assert_eq!(
                    crate::run_main(&execution, &mut echo).unwrap(),
                    crate::Value::Nil
                );
                assert!(echo.is_empty());
            }
        }
    }

    fn remainder_fixture_matcher(terminator: &mut Terminator) -> &mut Match {
        match terminator {
            Terminator::Match(matcher) => matcher,
            _ => panic!("expected a constructor remainder match"),
        }
    }

    fn remainder_fixture_field_index(
        instruction: &mut ProfiledInstruction<Infallible>,
    ) -> &mut usize {
        use crate::plan::execution::graph::block::instruction::ProfiledValueInstruction;
        use crate::plan::execution::graph::{
            CustomInstruction, ProfiledInstruction, ProfiledInstructionKind,
        };
        match instruction {
            ProfiledInstruction::Value(ProfiledValueInstruction {
                kind: ProfiledInstructionKind::Custom(CustomInstruction::CustomField { index, .. }),
                ..
            }) => index,
            _ => panic!("expected a constructor remainder field"),
        }
    }

    #[test]
    #[should_panic(expected = "expected a constructor remainder match")]
    fn remainder_fixture_matcher_rejects_non_matches() {
        use crate::plan::execution::graph::{BlockGraphExitId, Terminator};
        remainder_fixture_matcher(&mut Terminator::Exit(BlockGraphExitId(0)));
    }

    #[test]
    #[should_panic(expected = "expected a constructor remainder field")]
    fn remainder_fixture_field_rejects_non_projections() {
        use crate::plan::execution::graph::{
            NilInstruction, NilLocalId, ParamLocal, ParamSlot, ProfiledInstruction,
            ProfiledInstructionKind,
        };
        use crate::plan::execution::type_::ValueShapeId;
        remainder_fixture_field_index(&mut ProfiledInstruction::new(
            ParamSlot::new(ParamLocal::Nil(NilLocalId(0)), ValueShapeId(0)),
            ProfiledInstructionKind::Nil(NilInstruction::Value),
        ));
    }

    #[test]
    fn arithmetic_regions_preserve_scalar_projections_and_list_guards_at_admission() {
        use crate::plan::execution::graph::ProfiledInstruction;

        let source = r#"
type Boxed { Boxed(Int) }
fn calculate(value: Int, pair: #(Int, Bool), boxed: Boxed, items: List(Int)) {
  let scaled = value * 2 + 1
  let first = pair.0
  let negative = -first
  let Boxed(field) = boxed
  let assert [head, ..] = items
  scaled + field + head - negative
}
pub fn main() { calculate(7, #(2, True), Boxed(5), [20]) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap();
        let artifact = Box::leak(Box::new(artifact(bindings.prepare())));
        let graph = &artifact.program.functions.value_returns.int_functions[1]
            .body
            .block_graph;
        assert_eq!(
            graph
                .instructions
                .iter()
                .filter_map(|instruction| match instruction {
                    ProfiledInstruction::IntegerRegion(region) => Some(region.nodes.len()),
                    ProfiledInstruction::Value(_) => None,
                })
                .collect::<Vec<_>>(),
            [2, 3]
        );
        let admitted = plain(artifact).unwrap();
        assert!(std::ptr::eq(admitted.artifact, artifact));
        let (execution, _) = admitted.into_execution();
        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
    }

    #[test]
    fn invalid_numeric_links_are_rejected_before_a_plain_binding_owner_is_created() {
        use crate::plan::execution::compiled::{
            CompiledFunction, CompiledImplementation, NumericImplementation,
        };
        use crate::plan::execution::function::IntFunctionId;
        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 42 }")
                .unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        artifact.program.compiled.ints = vec![CompiledFunction {
            function: IntFunctionId(999),
            implementation: CompiledImplementation::Numeric(NumericImplementation {
                entry: 0,
                checkpoints: vec![].into(),
                run: metadata_numeric,
            }),
        }]
        .into();

        assert_eq!(
            plain(Box::leak(Box::new(artifact)))
                .err()
                .unwrap()
                .to_string(),
            "invalid prepared program: Compiled(CompiledError { family: Int, function: 999, reason: MissingFunction }); regenerate the prepared program"
        );
    }

    fn artifact(prepared: PreparedModule) -> ModuleArtifact<Infallible> {
        let common = Arc::try_unwrap(prepared.program.common).ok().unwrap();
        let functions = owned(prepared.program.functions);
        let constants = owned(common.constants);
        let value_shapes = owned(common.value_shapes);
        ModuleArtifact {
            format: FORMAT_VERSION,
            program: ProgramTables {
                root: common.root,
                modules: common.modules,
                main: common.main,
                functions: *functions,
                compiled: prepared.program.compiled,
                constants: *constants,
                function_parameters: Arc::try_unwrap(common.function_parameters).ok().unwrap(),
                list_types: Arc::try_unwrap(common.list_types).ok().unwrap(),
                custom_types: Arc::try_unwrap(common.custom_types).ok().unwrap(),
                external_types: Arc::try_unwrap(common.external_types).ok().unwrap(),
                value_shapes: *value_shapes,
            },
            entries: prepared.entries,
            exports: prepared.exports,
        }
    }

    #[test]
    fn native_selection_checks_executable_types_and_input_graphs_beyond_declaration_keys() {
        use crate::plan::execution::host::registration::RegistrationType;
        use crate::plan::execution::prepared::HostedModuleArtifact;
        use crate::plan::{
            FunctionType, LibraryCallableSignature, LibraryEntry, LibraryNativeSignature,
            LibraryValueType, ValueType,
        };
        use crate::{
            HostCallableSchema, HostCaptures, HostConstructions, HostReturns, HostTypeList,
            HostTypeListEnd, HostTypeSequence,
        };
        type End = HostTypeListEnd;
        type One<T> = HostTypeList<T, End>;
        struct Add<Args, Captures>(std::marker::PhantomData<(Args, Captures)>);
        impl<Args: HostTypeSequence, Captures: HostTypeSequence> HostCallableSchema
            for Add<Args, Captures>
        {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "private/bodies";
            const NAME: &'static str = "add";
            type Arguments = Args;
            type Return = BigInt;
            type Captures = Captures;
            type Constructions = End;
            type Completion = HostReturns;
        }
        type Original = Add<One<BigInt>, One<bool>>;
        struct Provider;
        impl HostProvider<NativeProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn add<'call>(
            mut call: HostCall<'call, NativeProfile, Provider, BigInt>,
            captures: HostCaptures<'call, One<bool>>,
            _: HostConstructions<'call, End>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            assert_eq!(call.captures(captures), (true, ()));
            Ok(call.return_value(value + 2))
        }
        let hosts = || {
            HostProviderSet::new([])
                .unwrap()
                .with_callable::<Provider, Original, (BigInt,), _>(add)
                .unwrap()
        };
        #[derive(Clone, Copy)]
        enum Change {
            None,
            Missing,
            Argument,
            CaptureType,
            CaptureCount,
            InvocationInput,
            CaptureInput,
            ConstructionCount,
            LibraryTarget,
        }
        for (change, expected) in [
            (Change::None, None),
            (
                Change::ConstructionCount,
                Some(
                    "invalid prepared program: Hosts(Contract { value: true, index: 0, reason: Callable }); regenerate the prepared program",
                ),
            ),
            (
                Change::LibraryTarget,
                Some(
                    "prepared provider registration mismatch: Call(Callable); regenerate with the matching providers",
                ),
            ),
            (
                Change::Missing,
                Some(
                    "native callable app:private/bodies.add is missing or has an incompatible declaration",
                ),
            ),
            (
                Change::Argument,
                Some(
                    "function add has incompatible prepared callable contracts: Signature; regenerate with the matching declarations",
                ),
            ),
            (
                Change::CaptureType,
                Some(
                    "function add has incompatible prepared callable contracts: Signature; regenerate with the matching declarations",
                ),
            ),
            (
                Change::CaptureCount,
                Some(
                    "function add has incompatible prepared callable contracts: Signature; regenerate with the matching declarations",
                ),
            ),
            (
                Change::InvocationInput,
                Some(
                    "function add has incompatible prepared callable contracts: Input(ListCount { family: Int, expected: 0, actual: 1 }); regenerate with the matching declarations",
                ),
            ),
            (
                Change::CaptureInput,
                Some(
                    "function add has incompatible prepared Rust inputs: ListCount { family: Int, expected: 0, actual: 1 }; regenerate with the matching declarations",
                ),
            ),
        ] {
            let typed = crate::compile_typed_host_program(
                "app",
                "main",
                [crate::PackageSource::new(
                    "app",
                    Vec::<&str>::new(),
                    [crate::ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { [42] }",
                    )],
                )],
                hosts(),
            )
            .unwrap();
            let mut plan = crate::planner::plan_host_library_program(typed).unwrap();
            plan.callable(
                crate::host::RegisteredCallableConstruction::of::<Original>(),
                LibraryNativeSignature {
                    invocation: LibraryCallableSignature {
                        type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                        input_variants: vec![],
                        input_lists: vec![],
                        callables: vec![],
                    },
                    captures: vec![ValueType::Bool],
                    capture_variants: vec![],
                    capture_lists: vec![],
                },
            )
            .unwrap();
            let main = plan
                .functions()
                .iter()
                .find(|function| function.name() == "main")
                .unwrap()
                .signature()
                .id();
            let (program, hosts_metadata, entries, mut callables) =
                crate::plan::execution::lowering::lower_hosted_library(
                    plan,
                    LibraryEntry::new(
                        main,
                        LibraryValueType::List(Box::new(LibraryValueType::Int)),
                        vec![],
                        vec![],
                    ),
                    vec![],
                )
                .unwrap();
            let common = Arc::try_unwrap(program.common).ok().unwrap();
            let list = crate::plan::execution::type_::IntListTypeId::new(
                crate::plan::execution::type_::ListTypeId(0),
            );
            assert_eq!(
                common.list_types.types.as_ref(),
                &[crate::plan::execution::type_::ListStorageTypeId::Int(list)]
            );
            let callable = &mut owned_mut(&mut callables)[0];
            match change {
                Change::None | Change::ConstructionCount | Change::LibraryTarget => {}
                Change::Missing => {}
                Change::Argument => {
                    callable.declaration.arguments = vec![RegistrationType::Bool].into()
                }
                Change::CaptureType => {
                    callable.declaration.captures = vec![RegistrationType::Int].into()
                }
                Change::CaptureCount => callable.declaration.captures = Vec::new().into(),
                Change::InvocationInput => {
                    callable.invocation.inputs.lists.ints = vec![list].into()
                }
                Change::CaptureInput => callable.captures.lists.ints = vec![list].into(),
            }
            // The callable declaration is a selection key. Every executable link
            // remains the real lowered body, and selection must compare those
            // links against the fresh Rust view before returning a handle.
            let (values, nevers) = hosts_metadata.into_metadata();
            let artifact = Box::leak(Box::new(HostedModuleArtifact {
                module: ModuleArtifact {
                    format: FORMAT_VERSION,
                    program: ProgramTables {
                        root: common.root,
                        modules: common.modules,
                        main: common.main,
                        functions: *owned(program.functions),
                        compiled: program.compiled,
                        constants: *owned(common.constants),
                        function_parameters: Arc::try_unwrap(common.function_parameters)
                            .ok()
                            .unwrap(),
                        list_types: Arc::try_unwrap(common.list_types).ok().unwrap(),
                        custom_types: Arc::try_unwrap(common.custom_types).ok().unwrap(),
                        external_types: Arc::try_unwrap(common.external_types).ok().unwrap(),
                        value_shapes: *owned(common.value_shapes),
                    },
                    entries,
                    exports: vec![super::super::Export::new(
                        "main".into(),
                        FunctionType::new(vec![], ValueType::List(Box::new(ValueType::Int))),
                        0,
                    )]
                    .into(),
                },
                value_functions: values
                    .into_vec()
                    .into_iter()
                    .map(Arc::try_unwrap)
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
                    .unwrap()
                    .into(),
                never_functions: nevers
                    .into_vec()
                    .into_iter()
                    .map(Arc::try_unwrap)
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
                    .unwrap()
                    .into(),
                callables,
            }));
            if matches!(change, Change::ConstructionCount) {
                // An artifact cannot grant a private body an extra construction
                // permission that its fresh declaration does not contain.
                owned_mut(&mut artifact.value_functions)[0]
                    .constructions
                    .callables = vec![artifact.callables[0].construction.clone()].into();
            }
            if matches!(change, Change::LibraryTarget) {
                // The real source main is not a registered private body.
                owned_mut(&mut artifact.callables)[0].construction.target =
                    artifact.module.program.main.clone();
            }
            if matches!(change, Change::ConstructionCount | Change::LibraryTarget) {
                assert_eq!(
                    artifact
                        .load(hosts())
                        .err()
                        .map(|error| error.to_string())
                        .as_deref(),
                    expected
                );
                continue;
            }
            let mut bindings = artifact.load(hosts()).unwrap();
            let error = match change {
                Change::Argument => bindings.callable::<Add<One<bool>, One<bool>>>().err(),
                Change::CaptureType => bindings.callable::<Add<One<BigInt>, One<BigInt>>>().err(),
                Change::CaptureCount => bindings.callable::<Add<One<BigInt>, End>>().err(),
                Change::Missing => bindings.callable::<Add<One<bool>, End>>().err(),
                Change::InvocationInput | Change::CaptureInput => {
                    bindings.callable::<Original>().err()
                }
                Change::None | Change::ConstructionCount | Change::LibraryTarget => {
                    let factory = bindings.callable::<Original>().unwrap();
                    let mut module = bindings.seal();
                    let host = crate::execution_fixture::TestHost::default();
                    host.block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut drop,
                        async |scope| {
                            let callback = scope.construct(&factory, (true, ())).unwrap();
                            assert_eq!(
                                scope.invoke(&callback, (40.into(),)).await.unwrap(),
                                BigInt::from(42)
                            );
                        },
                    ))
                    .unwrap()
                    .try_into_value()
                    .unwrap();
                    None
                }
            };
            assert_eq!(error.as_ref().map(ToString::to_string).as_deref(), expected);
        }
    }

    #[test]
    fn typed_nil_fallthrough_preserves_pattern_validation_and_projection_evidence() {
        use super::body::BodyError;
        use super::functions::{FunctionError, FunctionErrorKind};
        use super::instruction::InstructionError;
        use super::pattern::PatternError;
        use super::terminator::TerminatorError;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

        let source = r#"
pub type Reason { Closed Timeout }
pub fn choose(value: Result(Nil, Reason)) -> Int {
  case value {
    Ok(Nil) -> 0
    Error(reason) -> case reason { Closed -> 1 Timeout -> 2 }
  }
}
pub fn main() {
  let assert 0 = choose(Ok(Nil))
  let assert 1 = choose(Error(Closed))
  let assert 2 = choose(Error(Timeout))
  Nil
}
"#;
        for (type_id, fields, discard, expected) in [
            (0, vec![MatchPattern::Nil], false, None),
            (
                0,
                vec![MatchPattern::Nil],
                true,
                Some(BodyError::Instruction {
                    block: 2,
                    index: 0,
                    error: InstructionError::OutputType,
                }),
            ),
            (
                0,
                vec![MatchPattern::Bool(true)],
                false,
                Some(BodyError::Terminator {
                    block: 0,
                    error: TerminatorError::Pattern(PatternError::SubjectType),
                }),
            ),
            (
                1,
                vec![MatchPattern::Nil],
                false,
                Some(BodyError::Terminator {
                    block: 0,
                    error: TerminatorError::Pattern(PatternError::SubjectType),
                }),
            ),
            (
                0,
                vec![MatchPattern::Nil, MatchPattern::Nil],
                false,
                Some(BodyError::Terminator {
                    block: 0,
                    error: TerminatorError::Pattern(PatternError::FieldCount {
                        expected: 1,
                        found: 2,
                    }),
                }),
            ),
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let (bindings, _) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), ()>::new("main"))
                .unwrap();
            let mut artifact = artifact(bindings.prepare());
            let function =
                &mut owned_mut(&mut artifact.program.functions.value_returns.int_functions)[0];
            let pattern = fixture_match_pattern(
                &mut owned_mut(&mut function.body.block_graph.blocks)[0].terminator,
            );
            *pattern = if discard {
                MatchPattern::Discard
            } else {
                MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(type_id),
                        index: 0,
                    },
                    fields: fields.into(),
                }
            };
            assert_eq!(
                module(&artifact, &functions::InfallibleHosts).err(),
                expected.map(|error| Error::Functions(FunctionError {
                    family: FunctionTableFamily::Int,
                    index: 0,
                    kind: FunctionErrorKind::Body(error),
                }))
            );
        }

        let source = r#"
pub type Reason { Closed Timeout }
pub fn choose(value: Result(Bool, Reason)) -> Int {
  case value {
    Ok(True) -> 0
    Ok(False) -> 3
    Error(reason) -> case reason { Closed -> 1 Timeout -> 2 }
  }
}
pub fn main() { let assert 3 = choose(Ok(False)) Nil }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), ()>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);
        let function =
            &mut owned_mut(&mut artifact.program.functions.value_returns.int_functions)[0];
        *fixture_match_pattern(
            &mut owned_mut(&mut function.body.block_graph.blocks)[0].terminator,
        ) = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 0,
            },
            fields: vec![MatchPattern::Nil].into(),
        };
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(FunctionError {
                family: FunctionTableFamily::Int,
                index: 0,
                kind: FunctionErrorKind::Body(BodyError::Terminator {
                    block: 0,
                    error: TerminatorError::Pattern(PatternError::SubjectType),
                }),
            }))
        );
    }

    fn fixture_match_pattern(terminator: &mut Terminator) -> &mut MatchPattern {
        match terminator {
            Terminator::Match(matcher) => &mut matcher.pattern,
            _ => panic!("the mutated fixture must contain a match"),
        }
    }

    #[test]
    #[should_panic(expected = "the mutated fixture must contain a match")]
    fn match_fixture_guard_rejects_other_terminators() {
        use crate::plan::execution::graph::BlockGraphExitId;

        fixture_match_pattern(&mut Terminator::Exit(BlockGraphExitId(0)));
    }

    #[test]
    fn rejects_artifact_errors_at_their_admission_stage() {
        use super::block::BlockError;
        use super::body::BodyError;
        use super::catalog::CatalogError;
        use super::constant::ConstantBodyError;
        use super::entry::EntryError;
        use super::functions::{FunctionError, FunctionErrorKind};
        use super::source::SourceError;
        use super::type_::TypeError;
        use crate::plan::ModuleId;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::graph::BlockId;

        let source = "const saved = 2 pub fn main(value: Int) -> Int { value * saved }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("main"))
            .unwrap();
        let mut artifact = artifact(bindings.prepare());
        assert_eq!(module(&artifact, &functions::InfallibleHosts).err(), None);

        for (format, expected) in [
            (
                1,
                "prepared format 1 is incompatible with format 22; regenerate the prepared program",
            ),
            (
                8,
                "prepared format 8 is incompatible with format 22; regenerate the prepared program",
            ),
        ] {
            artifact.format = format;
            assert_eq!(plain(&artifact).err().unwrap().to_string(), expected);
        }
        artifact.format = FORMAT_VERSION;

        assert_eq!(artifact.program.value_shapes.shapes.len(), 1);
        let shape_types = std::mem::replace(
            &mut artifact.program.value_shapes.shape_types,
            Storage::Static(&[]),
        );
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Types(TypeError::ShapeTableLength {
                shapes: 1,
                types: 0
            }))
        );
        artifact.program.value_shapes.shape_types = shape_types;

        artifact.program.root = ModuleId::new(1);
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Sources(SourceError::Root {
                index: 1,
                modules: 1
            }))
        );
        artifact.program.root = ModuleId::new(0);

        artifact.program.function_parameters.families[0] = 0..2;
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Catalog(CatalogError::FamilyRange {
                family: 0,
                range: 0..2,
                length: 1
            }))
        );
        artifact.program.function_parameters.families[0] = 0..0;

        let entries = owned_mut(&mut artifact.program.functions.value_returns.int_functions);
        let entry = entries[0].body.block_graph.entry;
        entries[0].body.block_graph.entry = BlockId(99);
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Functions(FunctionError {
                family: FunctionTableFamily::Int,
                index: 0,
                kind: FunctionErrorKind::Body(BodyError::Block(BlockError::Missing { index: 99 })),
            }))
        );
        let entries = owned_mut(&mut artifact.program.functions.value_returns.int_functions);
        entries[0].body.block_graph.entry = entry;

        let constants = owned_mut(&mut artifact.program.constants.ints);
        let entry = constants[0].block_graph.entry;
        constants[0].block_graph.entry = BlockId(99);
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Constants(ConstantBodyError {
                family: "ints",
                index: 0,
                error: Box::new(BodyError::Block(BlockError::Missing { index: 99 })),
            }))
        );
        let constants = owned_mut(&mut artifact.program.constants.ints);
        constants[0].block_graph.entry = entry;

        let exports = std::mem::replace(&mut artifact.exports, Storage::Static(&[]));
        assert_eq!(
            module(&artifact, &functions::InfallibleHosts).err(),
            Some(Error::Entries(EntryError::Empty))
        );
        artifact.exports = exports;
        let admitted = plain(&artifact).unwrap();
        assert!(std::ptr::eq(admitted.artifact, &artifact));
        assert!(std::ptr::eq(
            admitted.views.inputs[0],
            &artifact.entries.ints[0].inputs
        ));
    }

    #[test]
    fn hosted_admission_checks_format_and_native_tables_before_selecting_entries() {
        use crate::plan::SourceSpan;
        use crate::plan::execution::compiled::{
            CompiledFunction, CompiledImplementation, NumericImplementation,
        };
        use crate::plan::execution::function::{IntFunctionId, NilFunctionId};
        use crate::plan::execution::prepared::HostedModuleArtifact;
        use crate::plan::execution::type_::{FunctionMetadata, TypeMetadata};
        use crate::plan::execution::{
            LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
            LibraryListConstructions,
        };

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Change {
            Format,
            Types,
            Sources,
            Catalog,
            Function,
            Numeric,
            Span,
            Exports,
            Constant,
            Registration,
            None,
        }
        let source = "const answer = 42\npub type Key\n@external(erlang, \"native\", \"key\") fn key() -> Key\npub fn main() { echo key() echo answer Nil }";
        for (change, expected) in [
            (
                Change::Format,
                Some(
                    "prepared format 1 is incompatible with format 22; regenerate the prepared program",
                ),
            ),
            (
                Change::Span,
                Some(
                    "invalid prepared program: Hosts(Contract { value: true, index: 0, reason: Source(SpanBounds { module: \"main\", span: SourceSpan { start: 9999, end: 10000 } }) }); regenerate the prepared program",
                ),
            ),
            (
                Change::Types,
                Some(
                    "invalid prepared program: Types(MissingList { index: 999 }); regenerate the prepared program",
                ),
            ),
            (
                Change::Sources,
                Some(
                    "invalid prepared program: Sources(Root { index: 999, modules: 1 }); regenerate the prepared program",
                ),
            ),
            (
                Change::Catalog,
                Some(
                    "invalid prepared program: Catalog(Type(MissingShape { index: 999 })); regenerate the prepared program",
                ),
            ),
            (
                Change::Function,
                Some(
                    "invalid prepared program: Functions(FunctionError { family: Nil, index: 0, kind: Host(MissingValue(99)) }); regenerate the prepared program",
                ),
            ),
            (
                Change::Exports,
                Some("invalid prepared program: Entries(Empty); regenerate the prepared program"),
            ),
            (
                Change::Numeric,
                Some(
                    "invalid prepared program: Compiled(CompiledError { family: Int, function: 99, reason: MissingFunction }); regenerate the prepared program",
                ),
            ),
            (
                Change::Constant,
                Some(
                    "invalid prepared program: Constants(ConstantBodyError { family: \"ints\", index: 0, error: Block(Missing { index: 99 }) }); regenerate the prepared program",
                ),
            ),
            (
                Change::Registration,
                Some(
                    "prepared provider registration mismatch: Registration { package: \"app\", module: \"main\", function: \"key\", reason: Missing }; regenerate with the matching providers",
                ),
            ),
            (Change::None, None),
        ] {
            let (program, mut values, nevers) = lowered_native(source);
            let common = Arc::try_unwrap(program.common).ok().unwrap();
            if change == Change::Span {
                values[0].site = crate::plan::HostCallSite::new(
                    "main".into(),
                    "key".into(),
                    SourceSpan::new(9999, 10000),
                );
            }
            let mut constants = *owned(common.constants);
            if change == Change::Constant {
                owned_mut(&mut constants.ints)[0].block_graph.entry =
                    crate::plan::execution::graph::BlockId(99);
            }
            let inputs = LibraryInputConstructions {
                variants: Storage::Static(&[]),
                lists: LibraryListConstructions {
                    functions: Vec::new().into(),
                    ints: Storage::Static(&[]),
                    floats: Storage::Static(&[]),
                    strings: Storage::Static(&[]),
                    bit_arrays: Storage::Static(&[]),
                    utf_codepoints: Storage::Static(&[]),
                    customs: Storage::Static(&[]),
                    externals: Storage::Static(&[]),
                    bools: Storage::Static(&[]),
                    nils: Storage::Static(&[]),
                    tuples: Storage::Static(&[]),
                    lists: Storage::Static(&[]),
                },
            };
            let artifact = Box::leak(Box::new(HostedModuleArtifact {
                callables: crate::plan::execution::storage::Table::Static(&[]),
                module: ModuleArtifact {
                    format: if change == Change::Format {
                        1
                    } else {
                        FORMAT_VERSION
                    },
                    program: ProgramTables {
                        root: common.root,
                        modules: common.modules,
                        main: common.main,
                        functions: *owned(program.functions),
                        compiled: program.compiled,
                        constants,
                        function_parameters: Arc::try_unwrap(common.function_parameters)
                            .ok()
                            .unwrap(),
                        list_types: Arc::try_unwrap(common.list_types).ok().unwrap(),
                        custom_types: Arc::try_unwrap(common.custom_types).ok().unwrap(),
                        external_types: Arc::try_unwrap(common.external_types).ok().unwrap(),
                        value_shapes: *owned(common.value_shapes),
                    },
                    entries: LibraryFunctionEntries {
                        functions: Vec::new().into(),
                        ints: Storage::Static(&[]),
                        floats: Storage::Static(&[]),
                        strings: Storage::Static(&[]),
                        bit_arrays: Storage::Static(&[]),
                        utf_codepoints: Storage::Static(&[]),
                        customs: Storage::Static(&[]),
                        externals: Storage::Static(&[]),
                        bools: Storage::Static(&[]),
                        nils: vec![LibraryFunctionEntry {
                            callables: Vec::new().into(),
                            function: NilFunctionId(0),
                            inputs,
                        }]
                        .into(),
                        tuples: Storage::Static(&[]),
                        lists: Storage::Static(&[]),
                    },
                    exports: if change == Change::Exports {
                        Storage::Static(&[])
                    } else {
                        vec![super::super::Export {
                            name: "main".into(),
                            slot: 0,
                            signature: FunctionMetadata {
                                arguments: Storage::Static(&[]),
                                return_: Box::new(TypeMetadata::Nil).into(),
                            },
                        }]
                        .into()
                    },
                },
                value_functions: values.into(),
                never_functions: nevers.into(),
            }));
            match change {
                Change::Types => {
                    owned_mut(&mut artifact.module.program.value_shapes.shape_types)[0] =
                        crate::plan::execution::type_::ValueType::List(
                            crate::plan::execution::type_::ListTypeId(999),
                        )
                }
                Change::Sources => artifact.module.program.root = crate::plan::ModuleId::new(999),
                Change::Catalog => {
                    owned_mut(&mut artifact.module.program.function_parameters.functions)[0]
                        .return_ = crate::plan::execution::type_::ValueShapeId(999)
                }
                Change::Function => {
                    owned_mut(
                        &mut artifact
                            .module
                            .program
                            .functions
                            .value_returns
                            .nil_functions,
                    )[0] = crate::plan::execution::function::ValueFunctionEntry::Host(
                        crate::plan::execution::host::HostedFunctionTarget::Value(
                            crate::plan::execution::host::HostFunctionId::new(
                                99,
                                crate::plan::execution::graph::NilLocalId(0),
                            ),
                        ),
                    )
                }
                Change::Numeric => {
                    artifact.module.program.compiled.ints = vec![CompiledFunction {
                        function: IntFunctionId(99),
                        implementation: CompiledImplementation::Numeric(NumericImplementation {
                            entry: 0,
                            checkpoints: vec![].into(),
                            run: metadata_numeric,
                        }),
                    }]
                    .into();
                }
                _ => {}
            }
            let providers = if change == Change::Registration {
                HostProviderSet::new([]).unwrap()
            } else {
                native_hosts()
            };
            assert_eq!(
                super::hosted(artifact, providers)
                    .err()
                    .map(|error| error.to_string())
                    .as_deref(),
                expected
            );
        }
    }

    #[test]
    fn standalone_admission_checks_format_program_native_linkage_and_main_before_loading() {
        use crate::plan::execution::function::{
            IntFunctionId, ProfiledCoreRuntimeFunctionId as Core,
            ProfiledRuntimeFunctionId as Runtime,
        };
        use crate::plan::execution::prepared::HostedEntryArtifact;

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Change {
            Format,
            Source,
            Registration,
            Main,
            None,
        }
        let source = "pub type Key\n@external(erlang, \"native\", \"key\") fn key() -> Key\npub fn main() { echo key() Nil }";
        for (change, expected) in [
            (
                Change::Format,
                Some(
                    "prepared format 1 is incompatible with format 22; regenerate the prepared program",
                ),
            ),
            (
                Change::Source,
                Some(
                    "invalid prepared program: Sources(Root { index: 999, modules: 1 }); regenerate the prepared program",
                ),
            ),
            (
                Change::Registration,
                Some(
                    "prepared provider registration mismatch: Registration { package: \"app\", module: \"main\", function: \"key\", reason: Missing }; regenerate with the matching providers",
                ),
            ),
            (
                Change::Main,
                Some(
                    "invalid prepared program: Main(Target(Catalog(MissingFunction { family: Int, index: 999 }))); regenerate the prepared program",
                ),
            ),
            (Change::None, None),
        ] {
            let (program, values, nevers) = lowered_native(source);
            let common = Arc::try_unwrap(program.common).ok().unwrap();
            let mut artifact = HostedEntryArtifact {
                format: FORMAT_VERSION,
                program: ProgramTables {
                    root: common.root,
                    modules: common.modules,
                    main: common.main,
                    functions: *owned(program.functions),
                    compiled: program.compiled,
                    constants: *owned(common.constants),
                    function_parameters: Arc::try_unwrap(common.function_parameters).ok().unwrap(),
                    list_types: Arc::try_unwrap(common.list_types).ok().unwrap(),
                    custom_types: Arc::try_unwrap(common.custom_types).ok().unwrap(),
                    external_types: Arc::try_unwrap(common.external_types).ok().unwrap(),
                    value_shapes: *owned(common.value_shapes),
                },
                value_functions: values.into(),
                never_functions: nevers.into(),
            };
            match change {
                Change::Format => artifact.format = 1,
                Change::Source => artifact.program.root = crate::plan::ModuleId::new(999),
                Change::Main => {
                    artifact.program.main = Runtime::Core(Core::Int(IntFunctionId(999)))
                }
                Change::Registration | Change::None => {}
            }
            let artifact = Box::leak(Box::new(artifact));
            let providers = if change == Change::Registration {
                HostProviderSet::new([]).unwrap()
            } else {
                native_hosts()
            };
            let result = super::hosted_entry(artifact, providers);
            assert_eq!(
                result.as_ref().err().map(ToString::to_string).as_deref(),
                expected
            );
            if let Ok(mut execution) = result {
                let second = super::hosted_entry(artifact, native_hosts()).unwrap();
                assert!(!Arc::ptr_eq(&execution.execution, &second.execution));
                assert!(!Arc::ptr_eq(
                    &execution.execution.program.common,
                    &second.execution.program.common
                ));
                assert!(std::ptr::eq(
                    execution.execution.program.functions.as_ref(),
                    &artifact.program.functions
                ));
                assert!(std::ptr::eq(
                    second.execution.program.functions.as_ref(),
                    &artifact.program.functions
                ));
                assert!(std::ptr::eq(
                    execution.execution.program.common.constants.as_ref(),
                    &artifact.program.constants
                ));
                let mut echo = Vec::new();
                assert_eq!(
                    crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
                    crate::Value::Nil
                );
                assert_eq!(
                    echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
                    ["src/main.gleam:3\nKey(42)"]
                );
            }
        }
    }

    #[test]
    fn fresh_opaque_and_callable_custom_views_must_match_the_prepared_capabilities() {
        use crate::plan::execution::prepared::HostedModuleArtifact;
        use crate::plan::{
            FunctionType, LibraryCallableSignature, LibraryEntry, LibraryValueType, ValueType,
        };
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() -> Result(fn(Int) -> Int, Int) { Ok(fn(value) { value + 1 }) }",
                )],
            )],
            HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let plan = crate::planner::plan_host_library_program(typed).unwrap();
        let main = plan
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap()
            .signature()
            .id();
        type Return = Result<crate::embedding::CallableType<(BigInt,), BigInt>, BigInt>;
        let returned = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                ValueType::Int,
            ],
        );
        let signature = FunctionType::new(vec![], ValueType::Custom(returned.clone()));
        let callbacks = vec![LibraryCallableSignature {
            type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            input_variants: vec![],
            input_lists: vec![],
            callables: vec![],
        }];
        let (program, hosts, entries, callables) =
            crate::plan::execution::lowering::lower_hosted_library(
                plan,
                LibraryEntry::new(main, LibraryValueType::Custom(returned), vec![], vec![])
                    .with_callables(callbacks.clone()),
                vec![],
            )
            .unwrap();
        let common = Arc::try_unwrap(program.common).ok().unwrap();
        let (values, nevers) = hosts.into_metadata();
        assert!(values.is_empty());
        assert!(nevers.is_empty());
        let artifact = Box::leak(Box::new(HostedModuleArtifact {
            module: ModuleArtifact {
                format: FORMAT_VERSION,
                program: ProgramTables {
                    root: common.root,
                    modules: common.modules,
                    main: common.main,
                    functions: *owned(program.functions),
                    compiled: program.compiled,
                    constants: *owned(common.constants),
                    function_parameters: Arc::try_unwrap(common.function_parameters).ok().unwrap(),
                    list_types: Arc::try_unwrap(common.list_types).ok().unwrap(),
                    custom_types: Arc::try_unwrap(common.custom_types).ok().unwrap(),
                    external_types: Arc::try_unwrap(common.external_types).ok().unwrap(),
                    value_shapes: *owned(common.value_shapes),
                },
                entries,
                exports: vec![super::super::Export::new(
                    "main".into(),
                    signature.clone(),
                    0,
                )]
                .into(),
            },
            value_functions: vec![].into(),
            never_functions: vec![].into(),
            callables,
        }));
        let admitted = super::hosted(
            artifact,
            HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        assert_eq!(
            admitted
                .program
                .select("main".into(), signature.clone(), &[], &[], &[], &callbacks)
                .unwrap(),
            0
        );
        assert_eq!(
            admitted
                .program
                .select("main".into(), signature, &[], &[], &[], &[])
                .unwrap_err()
                .to_string(),
            "function main has incompatible prepared callable contracts: Count { expected: 0, actual: 1 }; regenerate with the matching declarations"
        );
        let mut bindings = artifact
            .load(HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap())
            .unwrap();
        type Opaque = <crate::provider::ProviderResult<
            crate::HostFunctionType<crate::HostTypeList<BigInt, crate::HostTypeListEnd>, BigInt>,
            BigInt,
        > as crate::embedding::NativeType>::Shape;
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(), Opaque>::new("main"))
                .err()
                .unwrap()
                .to_string(),
            "function main has incompatible prepared callable contracts: Count { expected: 0, actual: 1 }; regenerate with the matching declarations"
        );
        let entry = bindings
            .function(FunctionDeclaration::<(), Return>::new("main"))
            .unwrap();
        let mut module = bindings.seal();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                let callback = scope.call(&entry, ()).await.unwrap().unwrap();
                assert_eq!(
                    scope.invoke(&callback, (BigInt::from(41),)).await.unwrap(),
                    BigInt::from(42)
                );
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
    }

    #[test]
    fn selects_only_matching_names_signatures_and_rust_input_mappings() {
        use crate::plan::{FunctionType, LibraryValueType, ValueType};

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub fn double(value: Int) -> Int { value * 2 }
pub fn same(value: Int) -> Int { value }
"#,
        )
        .unwrap();
        let (mut bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("same"))
            .unwrap();
        let artifact = artifact(bindings.prepare());
        let admitted = plain(&artifact).unwrap();
        let cases = [
            (
                "double",
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                Ok(0),
            ),
            (
                "same",
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                Ok(1),
            ),
            (
                "missing",
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                Err("function missing does not exist in the Gleam module"),
            ),
            (
                "double",
                FunctionType::new(Vec::new(), ValueType::Int),
                Err(
                    "function double has type FunctionType { arguments: [Int], return_: Int }, expected FunctionType { arguments: [], return_: Int }",
                ),
            ),
            (
                "double",
                FunctionType::new(vec![ValueType::Float], ValueType::Int),
                Err(
                    "function double has type FunctionType { arguments: [Int], return_: Int }, expected FunctionType { arguments: [Float], return_: Int }",
                ),
            ),
            (
                "double",
                FunctionType::new(vec![ValueType::Int], ValueType::Float),
                Err(
                    "function double has type FunctionType { arguments: [Int], return_: Int }, expected FunctionType { arguments: [Int], return_: Float }",
                ),
            ),
        ];
        for (name, signature, expected) in cases {
            assert_eq!(
                admitted
                    .select(name.into(), signature, &[], &[], &[], &[])
                    .map_err(|error| error.to_string()),
                expected.map_err(str::to_owned)
            );
        }
        assert_eq!(
            admitted
                .select(
                    "double".into(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    &[],
                    &[LibraryValueType::Int],
                    &[],
                    &[],
                )
                .unwrap_err()
                .to_string(),
            "function double has incompatible prepared Rust inputs: ListType { index: 0 }; regenerate with the matching declarations"
        );
        assert_eq!(
            admitted
                .select(
                    "double".into(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    &[],
                    &[],
                    &[],
                    &[],
                )
                .unwrap(),
            0
        );
    }

    #[test]
    #[should_panic(expected = "the preparation fixture must own its data")]
    fn owned_fixture_guard_rejects_static_storage() {
        assert_eq!(*owned(Storage::Owned(Box::new(42u32))), 42);
        owned(Storage::Static(&42u32));
    }

    #[test]
    #[should_panic(expected = "the mutated fixture must own its data")]
    fn mutable_fixture_guard_rejects_static_storage() {
        assert_eq!(*owned_mut(&mut Storage::Owned(Box::new(42u32))), 42);
        owned_mut(&mut Storage::Static(&42u32));
    }

    pub(super) fn owned<Data: ?Sized>(storage: Storage<Data>) -> Box<Data> {
        match storage {
            Storage::Owned(data) => data,
            Storage::Static(_) => panic!("the preparation fixture must own its data"),
        }
    }

    pub(super) fn owned_mut<Data: ?Sized>(storage: &mut Storage<Data>) -> &mut Data {
        match storage {
            Storage::Owned(data) => data,
            Storage::Static(_) => panic!("the mutated fixture must own its data"),
        }
    }

    pub(super) fn graph_body<Body: 'static, Host>(
        entry: &crate::plan::execution::function::ValueFunctionEntry<Body, Host>,
    ) -> &Body {
        match entry {
            crate::plan::execution::function::ValueFunctionEntry::Graph(function) => {
                function.body()
            }
            crate::plan::execution::function::ValueFunctionEntry::Host(_) => {
                panic!("the fixture entry must be a Gleam graph")
            }
        }
    }

    #[test]
    #[should_panic(expected = "the fixture entry must be a Gleam graph")]
    fn graph_fixture_guard_rejects_native_entries() {
        use crate::plan::execution::function::{
            ExecutableFunction, FunctionEntry, ValueFunctionEntry,
        };
        let graph: ValueFunctionEntry<(), ()> = ValueFunctionEntry::Graph(
            Box::new(ExecutableFunction {
                entry: FunctionEntry { parameter_count: 0 },
                body: (),
            })
            .into(),
        );
        assert_eq!(*graph_body(&graph), ());
        graph_body(&crate::plan::execution::function::ValueFunctionEntry::<
            (),
            (),
        >::Host(()));
    }
}
