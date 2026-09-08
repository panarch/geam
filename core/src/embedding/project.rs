use crate::frontend::{
    HostedTypedProgram, ProjectError, TransferHostedTypedProgram, TypedProgram,
    compile_typed_host_project, compile_typed_project, compile_typed_transfer_host_project,
};
use crate::host::{HostProfile, HostProviderSet, HostRegistrationError, TransferHostProviderSet};
use camino::Utf8PathBuf;
use ecow::EcoString;

mod error;
pub use error::HostedProjectError;

/// One resolved Gleam project selection for plain Rust embedding.
pub struct Project {
    root: Utf8PathBuf,
    module: EcoString,
}

/// A resolved source selection with explicitly transferable provider storage.
pub struct TransferHostedProject<Profile: HostProfile> {
    root: Utf8PathBuf,
    module: EcoString,
    register_providers: fn() -> Result<TransferHostProviderSet<Profile>, HostRegistrationError>,
}

/// One resolved Gleam project selection and static provider registration for
/// hosted embedding.
pub struct HostedProject<Profile: HostProfile> {
    root: Utf8PathBuf,
    module: EcoString,
    register_providers: fn() -> Result<HostProviderSet<Profile>, HostRegistrationError>,
}

impl Project {
    /// Selects a root module from an already resolved Gleam project.
    pub fn new(root: impl Into<Utf8PathBuf>, module: impl Into<EcoString>) -> Self {
        Self {
            root: root.into(),
            module: module.into(),
        }
    }

    /// Compiles the selected project and consumes its loading inputs.
    pub fn compile(self) -> Result<TypedProgram, ProjectError> {
        compile_typed_project(self.root, self.module)
    }
}

impl<Profile: HostProfile> TransferHostedProject<Profile> {
    /// Selects source and defers static registration until compilation.
    pub fn new(
        root: impl Into<Utf8PathBuf>,
        module: impl Into<EcoString>,
        register_providers: fn() -> Result<TransferHostProviderSet<Profile>, HostRegistrationError>,
    ) -> Self {
        Self {
            root: root.into(),
            module: module.into(),
            register_providers,
        }
    }

    /// Registers the selected providers and compiles ordinary source types.
    ///
    /// Compilation is synchronous and read-only; it does not poll host Futures
    /// or choose an executor.
    pub fn compile(self) -> Result<TransferHostedTypedProgram<Profile>, HostedProjectError> {
        let providers = (self.register_providers)()?;
        compile_typed_transfer_host_project(self.root, self.module, providers)
            .map_err(HostedProjectError::from)
    }
}

impl<Profile: HostProfile> HostedProject<Profile> {
    /// Selects a root module and defers its static provider registration until
    /// compilation.
    pub fn new(
        root: impl Into<Utf8PathBuf>,
        module: impl Into<EcoString>,
        register_providers: fn() -> Result<HostProviderSet<Profile>, HostRegistrationError>,
    ) -> Self {
        Self {
            root: root.into(),
            module: module.into(),
            register_providers,
        }
    }

    /// Registers static providers, then compiles the selected project and
    /// consumes its inputs.
    pub fn compile(self) -> Result<HostedTypedProgram<Profile>, HostedProjectError> {
        let providers = (self.register_providers)()?;
        compile_typed_host_project(self.root, self.module, providers)
            .map_err(HostedProjectError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostedProject, HostedProjectError, Project, TransferHostedProject};
    use crate::embedding::{
        FunctionDeclaration, HostedModuleBuilder, WorkModuleBuilder, with_execution_scope,
    };
    use crate::host::{
        AsyncHostComponentProfile, HostCallCompletion, HostFutureStore, HostProfile, HostProvider,
        TransferHostCall, TransferHostProviderModule, TransferHostProviderSet,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{
        EchoOutput, EchoSink, HostModule, HostProviderModule, HostProviderSet,
        HostRegistrationError, ProjectError, StatelessHostProfile,
    };
    use camino::{Utf8Path, Utf8PathBuf};
    use futures_util::FutureExt;
    use num_bigint::BigInt;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    struct TransferProfile;
    struct TransferProvider;
    impl HostProfile for TransferProfile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for TransferProfile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for TransferProfile {
        fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    impl HostProvider<TransferProfile> for TransferProvider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }
    fn transfer_providers()
    -> Result<TransferHostProviderSet<TransferProfile>, HostRegistrationError> {
        fn adjust<'call>(
            mut call: TransferHostCall<'call, TransferProfile, TransferProvider, BigInt>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, crate::AsyncHostCallError> {
            let () = *call.state();
            Ok(call.return_value(value + 1))
        }
        TransferHostProviderSet::new([TransferHostProviderModule::new_for_profile(
            "application",
            "inventory_rules",
        )
        .expect("fixture module")
        .with_scoped_function::<TransferProvider, (BigInt,), BigInt, _>("adjust", adjust)
        .expect("fixture function")])
    }

    #[derive(Default)]
    struct SendEcho {
        outputs: usize,
    }

    impl EchoSink for SendEcho {
        fn emit(&mut self, _output: EchoOutput) {
            self.outputs += 1;
        }
    }

    #[test]
    fn compiles_plain_project_import_closure() {
        let project = project();
        write_file(
            &project,
            "src/inventory_rules.gleam",
            "import inventory_support\npub fn label() { inventory_support.label() }",
        );
        write_file(
            &project,
            "src/inventory_support.gleam",
            "pub fn label() { \"inventory\" }",
        );

        let program = Project::new(project_root(&project), "inventory_rules")
            .compile()
            .expect("plain project descriptor should compile the selected closure");

        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "inventory_rules");
        assert_eq!(program.modules().len(), 2);
    }

    #[test]
    fn registers_host_providers_during_project_compilation() {
        let project = project();
        write_file(
            &project,
            "src/inventory_rules.gleam",
            r#"
import inventory_support

pub fn quantity() -> Int {
  inventory_support.quantity()
}
"#,
        );
        write_file(
            &project,
            "src/inventory_support.gleam",
            r#"
@external(erlang, "host", "quantity")
pub fn quantity() -> Int
"#,
        );
        let program = HostedProject::new(
            project_root(&project),
            "inventory_rules",
            inventory_providers,
        )
        .compile()
        .expect("hosted project descriptor should register its providers during compilation");
        let builder = HostedModuleBuilder::new(program).expect("hosted project should plan");
        let (bindings, quantity) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("quantity"))
            .expect("quantity should bind");
        let module = bindings.seal().expect("quantity should seal");

        assert_eq!(
            module.call(&quantity, (), &mut (), &mut Vec::new()),
            Ok(BigInt::from(42)),
        );
    }

    #[test]
    fn registers_transfer_providers_at_compile_without_changing_source_return_types() {
        let project = project();
        write_file(
            &project,
            "src/inventory_rules.gleam",
            r#"
@external(erlang, "host", "adjust")
fn adjust(value: Int) -> Int {
  value
}

pub fn quantity(value: Int) -> Int {
  echo value as "input"
  adjust(value)
}
"#,
        );
        let program = TransferHostedProject::new(
            project_root(&project),
            "inventory_rules",
            transfer_providers,
        )
        .compile()
        .expect("transfer project compilation");
        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "inventory_rules");
        let (bindings, quantity) = WorkModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("quantity"))
            .expect("binding");
        let mut module = bindings.seal().expect("sealing");
        let mut state = ();
        assert!(std::ptr::eq(
            <WorkComponent as HostProvider<TransferProfile>>::project(&mut state),
            &state,
        ));
        let mut echo = SendEcho::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            assert_eq!(scope.call(&quantity, (41.into(),)), Ok(42.into()));
        })
        .now_or_never()
        .expect("ordinary result stays immediate");
        assert_eq!(echo.outputs, 1);
    }

    #[test]
    fn preserves_project_error_identity() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = project_root(&directory).join("missing");
        let error = Project::new(root.clone(), "inventory_rules")
            .compile()
            .expect_err("missing project should retain its config read failure");

        assert!(matches!(
            error,
            ProjectError::ConfigIo { path, .. } if path == root.join("gleam.toml")
        ));
    }

    #[test]
    fn preserves_host_registration_error_identity() {
        let project = project();
        let error =
            HostedProject::new(project_root(&project), "inventory_rules", invalid_providers)
                .compile()
                .err()
                .expect("invalid static provider registration should fail during compilation");

        assert!(matches!(
            error,
            HostedProjectError::HostRegistration(
                HostRegistrationError::InvalidModuleName { module }
            ) if module == "invalid module"
        ));
    }

    #[test]
    fn preserves_hosted_project_error_identity() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = project_root(&directory).join("missing");
        let error = HostedProject::new(root.clone(), "inventory_rules", inventory_providers)
            .compile()
            .err()
            .expect("missing hosted project should retain its config read failure");

        assert!(matches!(
            error,
            HostedProjectError::Project(ProjectError::ConfigIo { path, .. })
                if path == root.join("gleam.toml")
        ));
    }

    #[test]
    fn transfer_registration_is_deferred_and_preserves_error_identity() {
        fn invalid() -> Result<TransferHostProviderSet<TransferProfile>, HostRegistrationError> {
            Err(HostRegistrationError::InvalidModuleName {
                module: "invalid module".into(),
            })
        }
        let directory = tempdir().expect("directory");
        let root = project_root(&directory).join("missing");
        let selected = TransferHostedProject::new(root.clone(), "inventory_rules", invalid);
        let error = selected
            .compile()
            .err()
            .expect("registration precedes filesystem access");
        assert!(matches!(error, HostedProjectError::HostRegistration(
            HostRegistrationError::InvalidModuleName { module }
        ) if module == "invalid module"));
        let error = TransferHostedProject::new(root.clone(), "inventory_rules", transfer_providers)
            .compile()
            .err()
            .expect("read failure after successful registration");
        assert!(
            matches!(error, HostedProjectError::Project(ProjectError::ConfigIo { path, .. })
            if path == root.join("gleam.toml"))
        );
    }

    fn inventory_providers() -> Result<HostProviderSet<StatelessHostProfile>, HostRegistrationError>
    {
        let provider =
            HostProviderModule::<StatelessHostProfile>::new("application", "inventory_support")
                .expect("provider module should be valid")
                .with_function("quantity", || BigInt::from(42))
                .expect("provider function should be valid");
        let providers = HostProviderSet::with_providers(
            Vec::<HostModule<StatelessHostProfile>>::new(),
            [provider],
        )
        .expect("provider set should be valid");
        Ok(providers)
    }

    fn invalid_providers() -> Result<HostProviderSet<StatelessHostProfile>, HostRegistrationError> {
        Err(HostRegistrationError::InvalidModuleName {
            module: "invalid module".into(),
        })
    }

    fn project() -> TempDir {
        let project = tempdir().expect("temporary project should be created");
        write_file(
            &project,
            "gleam.toml",
            "name = \"application\"\nversion = \"1.0.0\"\n",
        );
        write_file(
            &project,
            "manifest.toml",
            "packages = []\n\n[requirements]\n",
        );
        project
    }

    fn project_root(project: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary project path should be UTF-8")
    }

    fn write_file(project: &TempDir, relative: &str, source: &str) {
        let root = project_root(project);
        let path = root.join(Utf8Path::new(relative));
        fs::create_dir_all(path.parent().expect("fixture path should have a parent"))
            .expect("fixture directory should be created");
        fs::write(path, source).expect("fixture source should be written");
    }
}
