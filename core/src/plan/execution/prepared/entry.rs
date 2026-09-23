use super::rust::{Emit, Rust};
use super::{FORMAT_VERSION, PreparedError, ProgramEmission, ProgramTables, admission};
use crate::host::{HostProviderSet, HostWorkProfile};
use crate::plan::HostedModulePlan;
use crate::plan::execution::host::{HostedExecutionProfile, HostedFunctionMetadata};
use crate::plan::execution::storage::Table;
use crate::plan::execution::{ExecutionProgram, HostSpecializationError, HostedEntry, lowering};
use std::sync::Arc;

/// A prepared standalone main with native declarations and no live runtime state.
pub struct PreparedHostedEntry {
    program: ExecutionProgram<HostedExecutionProfile>,
    value_functions: Table<Arc<HostedFunctionMetadata>>,
    never_functions: Table<Arc<HostedFunctionMetadata>>,
}

/// Compiler-visible standalone data admitted before a runtime entry is created.
pub struct HostedEntryArtifact {
    pub format: u32,
    pub program: ProgramTables<HostedExecutionProfile>,
    pub value_functions: Table<HostedFunctionMetadata>,
    pub never_functions: Table<HostedFunctionMetadata>,
}

impl PreparedHostedEntry {
    /// Prepares main without initializing provider state or executing Gleam.
    pub fn try_from_module_plan<Profile: crate::HostProfile>(
        plan: HostedModulePlan<Profile>,
    ) -> Result<Self, HostSpecializationError> {
        let (program, hosts) = lowering::lower_hosted(plan)?;
        let (value_functions, never_functions) = hosts.into_metadata();
        Ok(Self {
            program,
            value_functions,
            never_functions,
        })
    }

    /// Emits static Rust data using `data` as the generated support API alias.
    pub fn emit_rust(&self) -> String {
        Rust::expression(self)
    }
}

impl HostedEntryArtifact {
    /// Admits prepared main and links its native functions to a fresh execution.
    ///
    /// The host still supplies runtime state, an executor and output to `run`.
    pub fn load<Profile: HostWorkProfile>(
        &'static self,
        providers: HostProviderSet<Profile>,
    ) -> Result<HostedEntry<Profile>, PreparedError> {
        admission::hosted_entry(self, providers).map(HostedEntry::from_execution)
    }
}

impl Emit for PreparedHostedEntry {
    fn emit(&self, output: &mut Rust) {
        let Self {
            program,
            value_functions,
            never_functions,
        } = self;
        output.structure(
            "HostedEntryArtifact",
            &[
                ("format", &FORMAT_VERSION),
                ("program", &ProgramEmission::new(program)),
                ("value_functions", value_functions),
                ("never_functions", never_functions),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::PreparedHostedEntry;
    use crate::{HostProfile, HostProviderSet, ModuleSource, PackageSource};

    struct UninitializedStores {}

    impl Default for UninitializedStores {
        fn default() -> Self {
            panic!("preparation must not initialize external stores")
        }
    }

    struct PreparationProfile;

    impl HostProfile for PreparationProfile {
        type RunState = ();
        type ExternalStores = UninitializedStores;
        type ExecutionState = ();
    }

    #[test]
    fn preparation_keeps_source_without_running_main_or_initializing_stores() {
        let source = "pub fn main() { panic as \"not executed during preparation\" }";
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<String>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::<PreparationProfile>::new([]).unwrap(),
        )
        .unwrap();
        let prepared =
            PreparedHostedEntry::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(prepared.value_functions.len(), 0);
        assert_eq!(prepared.never_functions.len(), 0);
        assert_eq!(
            prepared.program.common.modules[0].source_context.as_ref(),
            Some(&crate::SourceContext::new("src/main.gleam", source)),
        );
        assert_eq!(
            prepared
                .program
                .functions
                .value_returns
                .never_functions
                .len(),
            1
        );
        assert_eq!(
            prepared.program.common.main,
            crate::plan::execution::function::ProfiledRuntimeFunctionId::Core(
                crate::plan::execution::function::ProfiledCoreRuntimeFunctionId::Never(
                    crate::plan::execution::function::NeverFunctionId(0)
                )
            )
        );
    }

    struct CallbackProvider;

    impl crate::HostProvider<crate::StatelessHostProfile> for CallbackProvider {
        type State = ();

        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    type Inputs = crate::HostTypeList<crate::HostTypeParameter<0>, crate::HostTypeListEnd>;

    fn accept<'call>(
        mut call: crate::HostCall<
            'call,
            crate::StatelessHostProfile,
            CallbackProvider,
            num_bigint::BigInt,
        >,
        _callback: crate::HostCallable<'call, Inputs, num_bigint::BigInt>,
    ) -> Result<crate::HostCallCompletion<'call, num_bigint::BigInt>, crate::HostCallError> {
        assert_eq!(call.state(), &mut ());
        Ok(call.return_value(42.into()))
    }

    #[test]
    fn preparation_preserves_specialization_failures_before_runtime_construction() {
        let plan = |source| {
            let providers = HostProviderSet::new([
                crate::HostModule::new("host_support", "host/function").unwrap()
                    .with_scoped_function::<CallbackProvider,
                        (crate::HostFunctionType<Inputs, num_bigint::BigInt>,), num_bigint::BigInt, _>(
                            "accept", accept,
                        ).unwrap(),
            ]).unwrap();
            crate::plan_host_program(
                crate::compile_typed_host_program(
                    "app",
                    "main",
                    [PackageSource::new(
                        "app",
                        ["host_support"],
                        [ModuleSource::new("main", "src/main.gleam", source)],
                    )],
                    providers,
                )
                .unwrap(),
            )
            .unwrap()
        };
        let symbolic = r#"
import host/function
fn generic(_value) { 1 }
pub fn main() { function.accept(generic) }
"#;
        let error = PreparedHostedEntry::try_from_module_plan(plan(symbolic))
            .err()
            .unwrap();
        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/function");
        assert_eq!(error.function(), "accept");
        assert_eq!(
            error.reason(),
            &crate::HostSpecializationErrorReason::UninhabitedCallbackArguments {
                callback: crate::FunctionType::new(
                    vec![crate::ValueType::Parameter(crate::plan::TypeParameterId(0))],
                    crate::ValueType::Int,
                ),
            }
        );
        let concrete = r#"
import host/function
pub fn main() { function.accept(fn(value: Int) { value + 1 }) }
"#;
        let prepared = PreparedHostedEntry::try_from_module_plan(plan(concrete)).unwrap();
        assert_eq!(prepared.value_functions.len(), 1);
        let mut execution = crate::HostedExecution::try_from_module_plan(plan(concrete)).unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn emits_a_complete_standalone_artifact_without_library_exports() {
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    "pub fn main() { Nil }",
                )],
            )],
            HostProviderSet::<PreparationProfile>::new([]).unwrap(),
        )
        .unwrap();
        let prepared =
            PreparedHostedEntry::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(prepared.emit_rust(), r#"
data::HostedEntryArtifact {
    format: 4,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("main"),
                source_context: Some(data::source::SourceContext::from_static("src/main.gleam", "pub fn main() { Nil }")),
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Nil(data::function::NilFunctionId(0))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[]),
                nil_functions: data::Storage::Static(&[
                    data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 0,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Value),
                                    },
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::NilLocalId(0)),
                            ]),
                        },
                    })),
                ]),
                tuple_functions: data::Storage::Static(&[]),
            },
            list_returns: data::function::ListFunctionTables {
                parameter_list_functions: data::Storage::Static(&[]),
                int_list_functions: data::Storage::Static(&[]),
                string_list_functions: data::Storage::Static(&[]),
                bit_array_list_functions: data::Storage::Static(&[]),
                utf_codepoint_list_functions: data::Storage::Static(&[]),
                custom_list_functions: data::Storage::Static(&[]),
                external_list_functions: data::Storage::Static(&[]),
                float_list_functions: data::Storage::Static(&[]),
                bool_list_functions: data::Storage::Static(&[]),
                nil_list_functions: data::Storage::Static(&[]),
                tuple_list_functions: data::Storage::Static(&[]),
                parameter_list_list_functions: data::Storage::Static(&[]),
                list_list_functions: data::Storage::Static(&[]),
                function_list_functions: data::Storage::Static(&[]),
            },
            function_returns: data::function::FunctionFunctionTables {
                int_function_functions: data::Storage::Static(&[]),
                float_function_functions: data::Storage::Static(&[]),
                string_function_functions: data::Storage::Static(&[]),
                bit_array_function_functions: data::Storage::Static(&[]),
                utf_codepoint_function_functions: data::Storage::Static(&[]),
                custom_function_functions: data::Storage::Static(&[]),
                external_function_functions: data::Storage::Static(&[]),
                bool_function_functions: data::Storage::Static(&[]),
                nil_function_functions: data::Storage::Static(&[]),
                tuple_function_functions: data::Storage::Static(&[]),
                generic_function_functions: data::Storage::Static(&[]),
                never_function_functions: data::Storage::Static(&[]),
                parameter_list_function_functions: data::Storage::Static(&[]),
                parameter_list_list_function_functions: data::Storage::Static(&[]),
                int_list_function_functions: data::Storage::Static(&[]),
                string_list_function_functions: data::Storage::Static(&[]),
                bit_array_list_function_functions: data::Storage::Static(&[]),
                utf_codepoint_list_function_functions: data::Storage::Static(&[]),
                custom_list_function_functions: data::Storage::Static(&[]),
                external_list_function_functions: data::Storage::Static(&[]),
                float_list_function_functions: data::Storage::Static(&[]),
                bool_list_function_functions: data::Storage::Static(&[]),
                nil_list_function_functions: data::Storage::Static(&[]),
                tuple_list_function_functions: data::Storage::Static(&[]),
                list_list_function_functions: data::Storage::Static(&[]),
                function_list_function_functions: data::Storage::Static(&[]),
                function_function_functions: data::Storage::Static(&[]),
            },
        },
        constants: data::constant::ProfiledConstantTable {
            ints: data::Storage::Static(&[]),
            strings: data::Storage::Static(&[]),
            bit_arrays: data::Storage::Static(&[]),
            customs: data::Storage::Static(&[]),
            floats: data::Storage::Static(&[]),
            bools: data::Storage::Static(&[]),
            nils: data::Storage::Static(&[]),
            tuples: data::Storage::Static(&[]),
            parameter_lists: data::Storage::Static(&[]),
            parameter_list_lists: data::Storage::Static(&[]),
            int_lists: data::Storage::Static(&[]),
            string_lists: data::Storage::Static(&[]),
            bit_array_lists: data::Storage::Static(&[]),
            utf_codepoint_lists: data::Storage::Static(&[]),
            custom_lists: data::Storage::Static(&[]),
            external_lists: data::Storage::Static(&[]),
            float_lists: data::Storage::Static(&[]),
            bool_lists: data::Storage::Static(&[]),
            nil_lists: data::Storage::Static(&[]),
            tuple_lists: data::Storage::Static(&[]),
            list_lists: data::Storage::Static(&[]),
            function_lists: data::Storage::Static(&[]),
            functions: data::Storage::Static(&[]),
        },
        function_parameters: data::function::FunctionCatalog {
            families: [
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..1,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..0,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
        },
        custom_types: data::type_::CustomTypeTable {
            types: data::Storage::Static(&[]),
            definitions: data::Storage::Static(&[]),
        },
        external_types: data::type_::ExternalTypeTable {
            types: data::Storage::Static(&[]),
        },
        value_shapes: data::type_::ValueShapeTable {
            shapes: data::Storage::Static(&[
                data::type_::ValueShapeDescriptor::Nil,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Nil,
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}"#.trim_start());
    }

    #[test]
    #[should_panic(expected = "preparation must not initialize external stores")]
    fn preparation_fixture_detects_runtime_store_initialization() {
        let _stores = UninitializedStores::default();
    }
}
