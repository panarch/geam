mod block;
mod body;
mod call;
mod catalog;
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
use crate::plan::execution::function::ExecutionProfile;
use crate::plan::execution::graph::ExternalListInstructionView;

pub use error::PreparedError;

pub(crate) struct AdmittedHostedModule<Profile: crate::HostProfile> {
    pub(crate) program:
        AdmittedModule<'static, crate::plan::execution::host::HostedExecutionProfile>,
    hosts: hosts::NativeFunctions<'static, Profile>,
}

pub(crate) struct AdmittedModule<'data, Profile: ExecutionProfile> {
    artifact: &'data ModuleArtifact<Profile>,
    types: type_::Types<'data>,
    inputs: Vec<&'data crate::plan::execution::LibraryInputConstructions>,
}

#[derive(Debug, PartialEq, Eq)]
enum SelectionError {
    Binding(crate::embedding::BindingError),
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
    let program = module(&artifact.module, &hosts).map_err(PreparedError::from)?;
    Ok(AdmittedHostedModule { program, hosts })
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
    let (types, catalog) = program(&artifact.program, &hosts).map_err(PreparedError::from)?;
    entry::main(&artifact.program.main, &catalog, &types)
        .map_err(|error| PreparedError::from(Error::<hosts::NativeError>::Main(error)))?;
    Ok(crate::HostedExecution::from_program(
        crate::plan::execution::HostedProgram {
            program: artifact.program.execution(),
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
    <<Profile::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: operand::Operand,
{
    let (types, catalog) = program(&artifact.program, hosts)?;
    let inputs = entry::all(
        &artifact.program.main,
        &artifact.entries,
        &artifact.exports,
        &catalog,
        &types,
    )
    .map_err(Error::Entries)?;
    Ok(AdmittedModule {
        artifact,
        types,
        inputs,
    })
}

fn program<'data, Profile: ExecutionProfile, Host: functions::Hosts<Profile>>(
    program: &'data ProgramTables<Profile>,
    hosts: &Host,
) -> Result<(type_::Types<'data>, catalog::Catalog<'data>), Error<Host::Error>>
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
    constant::all(&program.constants, &context).map_err(Error::Constants)?;
    Ok((types, catalog))
}

impl<Profile: crate::HostProfile> AdmittedHostedModule<Profile> {
    pub(crate) fn into_execution(
        self,
    ) -> (
        crate::HostedExecution<Profile>,
        crate::plan::execution::LibraryFunctionEntries,
    ) {
        let artifact = self.program.artifact;
        let execution = crate::plan::execution::HostedProgram {
            program: artifact.program.execution(),
            host_functions: self.hosts.into_tables(),
        };
        (
            crate::HostedExecution::from_program(execution),
            artifact.entries.borrowed(),
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
    ) -> Result<usize, super::PreparedError> {
        use crate::embedding::BindingError;

        let Some((index, export)) = self
            .artifact
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
        input::mapping(self.inputs[index], variants, lists, standard, &self.types)
            .map_err(|error| super::PreparedError::from(SelectionError::Input { name, error }))?;
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
                program: self.artifact.program.execution(),
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
    use crate::plan::execution::host::{HostedExecutionProfile, HostedFunctionMetadata};
    use crate::plan::execution::prepared::{
        FORMAT_VERSION, ModuleArtifact, PreparedModule, ProgramTables,
    };
    use crate::plan::execution::storage::Storage;
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

        artifact.format = 1;
        assert_eq!(
            plain(&artifact).err().unwrap().to_string(),
            "prepared format 1 is incompatible with format 2; regenerate the prepared program"
        );
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
            admitted.inputs[0],
            &artifact.entries.ints[0].inputs
        ));
    }

    #[test]
    fn hosted_admission_checks_format_and_native_tables_before_selecting_entries() {
        use crate::plan::SourceSpan;
        use crate::plan::execution::function::NilFunctionId;
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
                    "prepared format 1 is incompatible with format 2; regenerate the prepared program",
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
                        ints: Storage::Static(&[]),
                        floats: Storage::Static(&[]),
                        strings: Storage::Static(&[]),
                        bit_arrays: Storage::Static(&[]),
                        utf_codepoints: Storage::Static(&[]),
                        customs: Storage::Static(&[]),
                        externals: Storage::Static(&[]),
                        bools: Storage::Static(&[]),
                        nils: vec![LibraryFunctionEntry {
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
                    "prepared format 1 is incompatible with format 2; regenerate the prepared program",
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
                    .select(name.into(), signature, &[], &[], &[])
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
                    &[]
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
                    &[]
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

    fn owned<Data: ?Sized>(storage: Storage<Data>) -> Box<Data> {
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
