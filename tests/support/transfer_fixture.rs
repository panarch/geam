use camino::Utf8Path;
use geam_builtin::FutureComponent;
use geam_core::embedding::{
    CallError, Function, FunctionDeclaration, HostedModule, HostedModuleBuilder,
};
use geam_core::frontend::HostedTypedProgram;
use geam_core::host::HostComponentProfile;
use geam_core::{
    EchoOutput, EchoSink, HostProfile, HostProviderSet, Value, ValueType,
    compile_typed_host_project,
};
use std::fs;
use std::path::Path;

use crate::execution_fixture;

#[path = "../../core/tests/support/fixture_observation.rs"]
mod fixture_observation;

pub(crate) use fixture_observation::ENTRY;

pub(crate) struct TransferFixture<Profile: HostProfile> {
    module: HostedModule<Profile>,
    entry: Function<(), ()>,
}

impl<Profile> TransferFixture<Profile>
where
    Profile: geam_core::host::HostWorkProfile<Work = FutureComponent>,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    pub(crate) fn new(program: HostedTypedProgram<Profile>, entry: &str) -> Self {
        let (bindings, entry) = HostedModuleBuilder::new(program)
            .expect("official source transfer plan")
            .function(FunctionDeclaration::<(), ()>::new(entry))
            .expect("typed observation entry");
        Self {
            module: bindings.seal().expect("official source transfer seal"),
            entry,
        }
    }

    pub(crate) fn run(
        &mut self,
        state: &mut Profile::RunState,
        echo: &mut (dyn EchoSink + Send),
    ) -> Result<(), CallError> {
        let host = execution_fixture::TestHost::default();
        host.block_on(
            self.module
                .with_execution(&host, state, echo, async |scope| {
                    scope.call(&self.entry, ()).await
                }),
        )
        .expect("controlled source fixture execution")
    }
}

#[derive(Default)]
pub(crate) struct ObservedEcho(Vec<(String, ValueType, String)>);

impl EchoSink for ObservedEcho {
    fn emit(&mut self, output: EchoOutput) {
        self.0.push((
            output.to_string(),
            output.value().value_type(),
            output.value().inspect().to_string(),
        ));
    }
}

impl ObservedEcho {
    pub(crate) fn assert_result(mut self, expected: &Value, expected_echo: &[EchoOutput]) {
        let (_, type_, inspection) = self.0.pop().expect("fixture result observation");
        assert_eq!(type_, expected.value_type());
        assert_eq!(inspection, expected.inspect().to_string());
        assert_eq!(
            self.0
                .into_iter()
                .map(|(rendered, _, _)| rendered)
                .collect::<Vec<_>>(),
            expected_echo
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
    }
}

pub(crate) fn observed_project<Profile: HostProfile>(
    root: &Utf8Path,
    root_module: &str,
    hosts: HostProviderSet<Profile>,
) -> HostedTypedProgram<Profile> {
    let directory = tempfile::tempdir().expect("temporary observation project");
    let observed = Utf8Path::from_path(directory.path()).expect("UTF-8 fixture path");
    for name in ["gleam.toml", "manifest.toml"] {
        fs::copy(root.join(name), observed.join(name)).expect("copy fixture manifest");
    }
    for name in ["src", "build/packages"] {
        copy_directory(
            root.join(name).as_std_path(),
            observed.join(name).as_std_path(),
        )
        .expect("copy fixture source");
    }
    let path = observed
        .join("src")
        .join(root_module)
        .with_extension("gleam");
    let original = fs::read_to_string(&path).expect("fixture entry source");
    fs::write(&path, fixture_observation::source(&original))
        .expect("add temporary observation entry");

    // The real loader owns all compiled source before the temporary copy is dropped.
    compile_typed_host_project(observed, root_module, hosts)
        .expect("resolved observation project should compile")
}

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

#[test]
fn observes_a_locked_project_without_mutating_its_sources() {
    use geam_core::host::HostFutureStore;

    struct Profile;

    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }

    impl geam_core::host::HostWorkProfile for Profile {
        type Work = FutureComponent;
    }
    impl HostComponentProfile<FutureComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }

        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    let directory = tempfile::tempdir().expect("original fixture project");
    let root = Utf8Path::from_path(directory.path()).expect("UTF-8 fixture root");
    let files = [
        (
            "gleam.toml",
            r#"name = "observation_test"
version = "1.0.0"

[dependencies]
observation_dependency = "1.0.0"
"#,
        ),
        (
            "manifest.toml",
            r#"packages = [
  { name = "observation_dependency", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "hex", outer_checksum = "0000000000000000000000000000000000000000000000000000000000000000" },
]

[requirements]
observation_dependency = { version = "1.0.0" }
"#,
        ),
        (
            "src/nested/entry.gleam",
            "import observation_dependency\n\npub fn main() { observation_dependency.answer() }\n",
        ),
        (
            "build/packages/observation_dependency/gleam.toml",
            "name = \"observation_dependency\"\nversion = \"1.0.0\"\n",
        ),
        (
            "build/packages/observation_dependency/src/observation_dependency.gleam",
            "pub fn answer() { 42 }\n",
        ),
    ];
    for (path, source) in files {
        let path = root.join(path);
        fs::create_dir_all(path.parent().expect("fixture parent")).expect("fixture directory");
        fs::write(path, source).expect("fixture source");
    }

    let program = observed_project(
        root,
        "nested/entry",
        HostProviderSet::<Profile>::from_providers([]).expect("provider-free fixture"),
    );
    let mut execution = TransferFixture::new(program, ENTRY);
    let mut echo = ObservedEcho::default();
    execution
        .run(&mut (), &mut echo)
        .expect("compiled observation outlives its temporary source copy");
    echo.assert_result(&Value::Int(42.into()), &[]);
    for (path, source) in files {
        assert_eq!(
            fs::read_to_string(root.join(path)).expect("original fixture"),
            source
        );
    }
}
