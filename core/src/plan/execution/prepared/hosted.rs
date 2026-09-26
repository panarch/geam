use super::rust::{Emit, Rust};
use super::{Export, ModuleArtifact, ModuleEmission};
use crate::plan::execution::host::{HostedExecutionProfile, HostedFunctionMetadata};
use crate::plan::execution::storage::Table;
use crate::plan::execution::{ExecutionProgram, LibraryFunctionEntries};
use std::sync::Arc;

/// A prepared hosted program with native declarations but no runtime state.
pub struct PreparedHostedModule {
    program: ExecutionProgram<HostedExecutionProfile>,
    entries: LibraryFunctionEntries,
    callables: Table<crate::plan::execution::LibraryNativeConstruction>,
    exports: Table<Export>,
    value_functions: Table<Arc<HostedFunctionMetadata>>,
    never_functions: Table<Arc<HostedFunctionMetadata>>,
}

pub struct HostedModuleArtifact {
    pub module: ModuleArtifact<HostedExecutionProfile>,
    pub value_functions: Table<HostedFunctionMetadata>,
    pub never_functions: Table<HostedFunctionMetadata>,
    pub callables: Table<crate::plan::execution::LibraryNativeConstruction>,
}

impl PreparedHostedModule {
    pub(crate) fn new<Value: Clone, Never: Clone + From<Value>>(
        plan: crate::plan::ProfiledHostedLibraryModulePlan<
            crate::host::HostFunctionBinding<Value, Never>,
        >,
        first: crate::plan::LibraryEntry,
        remaining: Vec<crate::plan::LibraryEntry>,
        exports: Vec<Export>,
    ) -> Result<Self, crate::HostSpecializationError> {
        let (program, host_functions, entries, callables) =
            crate::plan::execution::lowering::lower_hosted_library(plan, first, remaining)?;
        let (value_functions, never_functions) = host_functions.into_metadata();
        Ok(Self {
            program,
            entries,
            callables,
            exports: exports.into(),
            value_functions,
            never_functions,
        })
    }

    /// Emits static Rust data, excluding provider implementations and state.
    pub fn emit_rust(&self) -> String {
        Rust::expression(self)
    }
}

impl Emit for PreparedHostedModule {
    fn emit(&self, output: &mut Rust) {
        let Self {
            program,
            entries,
            exports,
            value_functions,
            never_functions,
            callables,
        } = self;
        output.structure(
            "HostedModuleArtifact",
            &[
                (
                    "module",
                    &ModuleEmission {
                        program,
                        entries,
                        exports,
                    },
                ),
                ("value_functions", value_functions),
                ("never_functions", never_functions),
                ("callables", callables),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostFunctionType, HostModule,
        HostProvider, HostProviderSet, HostSpecializationErrorReason, HostTypeList,
        HostTypeListEnd, HostTypeParameter, ModuleSource, PackageSource, StatelessHostProfile,
    };
    use num_bigint::BigInt;

    struct Provider;

    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();

        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    type Inputs = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;

    fn accept<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
        _function: HostCallable<'call, Inputs, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        assert_eq!(call.state(), &mut ());
        Ok(call.return_value(42.into()))
    }

    #[test]
    fn preparation_preserves_callback_specialization_failures_from_dynamic_sealing() {
        let bindings = |source| {
            let providers = HostProviderSet::new([HostModule::new(
                "host_support",
                "host/function",
            )
            .unwrap()
            .with_scoped_function::<Provider, (HostFunctionType<Inputs, BigInt>,), BigInt, _>(
                "accept", accept,
            )
            .unwrap()])
            .unwrap();
            let program = crate::compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    ["host_support"],
                    [ModuleSource::new("main", "src/main.gleam", source)],
                )],
                providers,
            )
            .unwrap();
            HostedModuleBuilder::new(program)
                .unwrap()
                .function(FunctionDeclaration::<(), BigInt>::new("main"))
                .unwrap()
        };
        let symbolic = r#"
import host/function
fn generic(_value) { 1 }
pub fn main() { function.accept(generic) }
"#;
        let prepared = bindings(symbolic).0.prepare().err().unwrap();
        let dynamic = bindings(symbolic).0.seal().err().unwrap();
        assert_eq!(prepared, dynamic);
        assert_eq!(prepared.package(), "host_support");
        assert_eq!(prepared.module(), "host/function");
        assert_eq!(prepared.function(), "accept");
        assert_eq!(
            prepared.reason(),
            &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                callback: crate::FunctionType::new(
                    vec![crate::ValueType::Parameter(crate::plan::TypeParameterId(0))],
                    crate::ValueType::Int,
                ),
            },
        );

        let concrete = r#"
import host/function
pub fn main() { function.accept(fn(value: Int) { value + 1 }) }
"#;
        let prepared = bindings(concrete).0.prepare().unwrap();
        assert_eq!(prepared.value_functions.len(), 1);
        assert_eq!(prepared.never_functions.len(), 0);
        let (bindings, function) = bindings(concrete);
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        assert_eq!(
            host.block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    scope.call(&function, ()).await
                })
            )
            .unwrap(),
            Ok(BigInt::from(42)),
        );
        assert!(echo.is_empty());
    }
}
