use crate::frontend::{
    AsyncHostedTypedProgram, HostedTypedProgram, ProjectError, TypedProgram,
    compile_typed_async_host_project, compile_typed_host_project, compile_typed_project,
};
use crate::host::{AsyncHostProviderSet, HostProfile, HostProviderSet, HostRegistrationError};
use camino::Utf8PathBuf;
use ecow::EcoString;

mod error;
pub use error::HostedProjectError;

/// One resolved Gleam project selection for plain Rust embedding.
pub struct Project {
    root: Utf8PathBuf,
    module: EcoString,
}

/// One resolved Gleam project selection and embedding-owned resumable hosts.
pub struct AsyncHostedProject<Profile: HostProfile> {
    root: Utf8PathBuf,
    module: EcoString,
    hosts: AsyncHostProviderSet<Profile>,
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

    /// Adds embedding-owned async hosts and selects resumable compilation.
    ///
    /// This consumes the plain selection so immediate and resumable project
    /// owners cannot be mixed after host registration.
    pub fn with_async_hosts<Profile: HostProfile>(
        self,
        hosts: AsyncHostProviderSet<Profile>,
    ) -> AsyncHostedProject<Profile> {
        AsyncHostedProject {
            root: self.root,
            module: self.module,
            hosts,
        }
    }
}

impl<Profile: HostProfile> AsyncHostedProject<Profile> {
    /// Compiles the selected project with its resumable host registrations.
    ///
    /// Compilation is synchronous and read-only; it does not poll host Futures
    /// or choose an executor.
    pub fn compile(self) -> Result<AsyncHostedTypedProgram<Profile>, ProjectError> {
        compile_typed_async_host_project(self.root, self.module, self.hosts)
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
    use super::{HostedProject, HostedProjectError, Project};
    use crate::embedding::{AsyncHostedModuleBuilder, FunctionDeclaration, HostedModuleBuilder};
    use crate::{
        AsyncHostModule, AsyncHostProviderModule, AsyncHostProviderSet, EchoOutput, EchoSink,
        HostModule, HostProviderModule, HostProviderSet, HostRegistrationError, ProjectError,
        StatelessHostProfile,
    };
    use camino::{Utf8Path, Utf8PathBuf};
    use num_bigint::BigInt;
    use std::fs;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};
    use tempfile::{TempDir, tempdir};

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
    fn adds_async_hosts_to_a_plain_project_selection() {
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
        let provider = AsyncHostProviderModule::new("application", "inventory_rules")
            .expect("async provider module should be valid")
            .with_async_function("adjust", |value: BigInt| std::future::ready(value + 1))
            .expect("async provider function should be valid");
        let hosts = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [provider])
            .expect("async provider set should be valid");
        let program = Project::new(project_root(&project), "inventory_rules")
            .with_async_hosts(hosts)
            .compile()
            .expect("async hosted project selection should compile");
        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "inventory_rules");
        let (bindings, quantity) = AsyncHostedModuleBuilder::new(program)
            .expect("async hosted project should plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("quantity"))
            .expect("quantity should bind");
        let mut module = bindings.seal();
        let mut state = ();
        let mut echo = SendEcho::default();
        {
            let mut call =
                pin!(module.call_async(&quantity, (BigInt::from(41),), &mut state, &mut echo,));
            let mut context = Context::from_waker(Waker::noop());

            assert_eq!(call.as_mut().poll(&mut context), Poll::Ready(Ok(42.into())));
        }
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
