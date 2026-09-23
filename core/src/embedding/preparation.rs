use super::binding::{BindingBuilder, BindingParts, Bindings};
use super::{Arguments, BindingError, EmbeddingValue, FunctionDeclaration, PreparedHostedModule};
use crate::host::HostFunctionBinding;
use crate::plan::ProfiledHostedLibraryModulePlan;
use crate::{DeclaredTypedProgram, HostSpecializationError, PlanError};

type DeclarationPlan = ProfiledHostedLibraryModulePlan<HostFunctionBinding<(), ()>>;

/// Plans a program from native declarations without constructing any Rust bodies.
///
/// Select at least one public function before preparing its immutable graph.
/// This owner has no execution methods and cannot produce an executable module.
pub struct HostPreparation {
    inner: BindingBuilder<DeclarationPlan>,
}

/// A nonempty selection of declared entries ready for immutable preparation.
pub struct HostPreparationBindings {
    pub(super) inner: Bindings<DeclarationPlan>,
}

impl HostPreparation {
    pub fn new(program: DeclaredTypedProgram) -> Result<Self, PlanError> {
        let public_functions = program.root_public_functions().cloned().collect();
        let plan = crate::planner::plan_declared_library_program(program)?;
        Ok(Self {
            inner: BindingBuilder::new(plan, public_functions),
        })
    }

    /// Selects the first public entry using the same typed binding rules as execution.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<HostPreparationBindings, BindingError>
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner
            .function(declaration, Return::library_type())
            .map(|(inner, _)| HostPreparationBindings { inner })
    }
}

impl HostPreparationBindings {
    /// Selects another entry before preparation; this does not create a callable handle.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<(), BindingError>
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner
            .function(declaration, Return::library_type())
            .map(|_| ())
    }

    /// Specializes the declarations and emits no native implementation or runtime state.
    pub fn prepare(self) -> Result<PreparedHostedModule, HostSpecializationError> {
        let BindingParts {
            plan,
            first,
            remaining,
            exports,
            owner: _,
        } = self.inner.into_parts();
        PreparedHostedModule::new(plan, first, remaining, exports)
    }
}

#[cfg(test)]
mod tests {
    use super::HostPreparation;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostDeclarations,
        HostDiverges, HostFailure, HostFunctionDeclaration, HostProvider, HostProviderModule,
        HostProviderModuleDeclaration, HostProviderSet, HostTypeListEnd, HostTypeParameter,
        HostValue, ModuleSource, PackageSource, StatelessHostProfile,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    struct Provider;

    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();

        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn declarations_and_real_bodies_prepare_identical_generic_and_diverging_entries() {
        const IDENTITY: HostFunctionDeclaration<(HostTypeParameter<0>,), HostTypeParameter<0>> =
            HostFunctionDeclaration::new("identity");
        const STOP: HostFunctionDeclaration<(), BigInt, HostTypeListEnd, HostDiverges> =
            HostFunctionDeclaration::new("stop");

        fn identity<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, HostTypeParameter<0>>,
            _: HostConstructions<'call, HostTypeListEnd>,
            value: HostValue<'call, HostTypeParameter<0>>,
        ) -> Result<HostCallCompletion<'call, HostTypeParameter<0>>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(value))
        }

        fn stop(
            _: HostCall<'_, StatelessHostProfile, Provider, BigInt>,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("application stopped").into())
        }

        let source = r#"
@external(erlang, "application", "identity")
fn identity(value: a) -> a
@external(erlang, "application", "stop")
fn stop() -> Int
pub fn run() { identity(31) + 11 }
pub fn check() { identity(True) }
pub fn stopped() { stop() }
"#;
        let packages = || {
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )]
        };
        let declarations = HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
            "application",
            "library",
        )
        .unwrap()
        .with_function(IDENTITY)
        .unwrap()
        .with_function(STOP)
        .unwrap()])
        .unwrap();
        let program = crate::compile_declared_host_program(
            "application",
            "library",
            packages(),
            declarations,
        )
        .unwrap();
        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "library");
        let mut preparation = HostPreparation::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        preparation
            .function(FunctionDeclaration::<(), bool>::new("check"))
            .unwrap();
        preparation
            .function(FunctionDeclaration::<(), BigInt>::new("stopped"))
            .unwrap();
        let declarations_only = preparation.prepare().unwrap().emit_rust();

        let dynamic = || {
            let providers = HostProviderSet::from_providers([HostProviderModule::new(
                "application",
                "library",
            )
            .unwrap()
            .with_declared_function::<Provider, _, _, _, _>(IDENTITY, identity)
            .unwrap()
            .with_declared_diverging_function::<Provider, _, _, _>(STOP, stop)
            .unwrap()])
            .unwrap();
            let program =
                crate::compile_typed_host_program("application", "library", packages(), providers)
                    .unwrap();
            let (mut bindings, run) = HostedModuleBuilder::new(program)
                .unwrap()
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .unwrap();
            let check = bindings
                .function(FunctionDeclaration::<(), bool>::new("check"))
                .unwrap();
            let stopped = bindings
                .function(FunctionDeclaration::<(), BigInt>::new("stopped"))
                .unwrap();
            (bindings, run, check, stopped)
        };
        assert_eq!(
            declarations_only,
            dynamic().0.prepare().unwrap().emit_rust()
        );

        let (bindings, run, check, stopped) = dynamic();
        let mut module = bindings.seal().unwrap();
        let execution_host = crate::execution_fixture::TestHost::default();
        execution_host
            .block_on(
                module.with_execution(&execution_host, &mut (), &mut drop, async |scope| {
                    assert_eq!(scope.call(&run, ()).await.unwrap(), BigInt::from(42));
                    assert!(scope.call(&check, ()).await.unwrap());
                    assert_eq!(
                        scope.call(&stopped, ()).await.unwrap_err().to_string(),
                        "host function application::library.stop failed: application stopped",
                    );
                }),
            )
            .unwrap();
    }
}
