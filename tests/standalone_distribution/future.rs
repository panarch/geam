use super::{copy_directory, geam_at, workspace_dependencies};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[test]
fn drives_explicit_work_with_tokio_and_preserves_standalone_lifecycle() {
    let fixture = fixture();
    let project = fixture.path().join("project");
    let added = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        added.status.success(),
        "{}",
        String::from_utf8_lossy(&added.stderr)
    );
    let prepared = geam_at(&project, ["prepare"]);
    assert!(
        prepared.status.success(),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    assert!(prepared.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&prepared.stderr).contains("initialized\n"));
    let manifest = fs::read(project.join("Cargo.toml")).expect("manifest");
    let lock = fs::read(project.join("Cargo.lock")).expect("lock");
    let runner = fs::read(project.join("build/geam/runner.rs")).expect("runner");

    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            "initialized\ntimer-pending\ntimer-complete\nstate:1\nstate-drop:1\n"
        );
    }

    for returned in [
        "[native.timer()]",
        "#(native.timer(), 1)",
        "Ok(native.timer())",
        "Future(native.timer())",
        "future.ready(native.timer())",
        "future.ready(Error(\"source data\"))",
    ] {
        fs::write(
            project.join("src/standalone_future.gleam"),
            format!(
                r#"import geam/future
import standalone_future/native
pub type Future(a) {{ Future(a) }}
pub fn main() {{ {returned} }}
"#
            ),
        )
        .expect("alternate source result");
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "{returned}: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            "initialized\nstate-drop:0\n"
        );
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("test server runtime");
    let listener = runtime
        .block_on(tokio::net::TcpListener::bind("127.0.0.1:0"))
        .expect("local listener");
    let address = listener.local_addr().expect("assigned port");
    let server = runtime.spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("native provider connects");
        let mut request = [0; 4];
        stream
            .read_exact(&mut request)
            .await
            .expect("native request");
        assert_eq!(&request, b"ping");
        stream.write_all(b"pong").await.expect("reply");
    });
    fs::write(
        project.join("src/standalone_future.gleam"),
        format!(
            r#"import geam/future
import standalone_future/native
pub type Response = future.Future(Int)
pub fn main() -> Response {{
  use response <- future.map(native.request("{address}"))
  echo response
  native.current()
}}
"#
        ),
    )
    .expect("native IO source");
    let run = geam_at(&project, ["run"]);
    if !run.status.success() {
        server.abort();
    }
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    runtime.block_on(server).expect("server completed");
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "initialized\nstate:0\nstate-drop:0\n"
    );
    assert!(
        String::from_utf8_lossy(&run.stderr).contains("src/standalone_future.gleam:6\n\"pong\"\n")
    );
    drop(runtime);

    fs::write(
        project.join("src/standalone_future.gleam"),
        "import standalone_future/native\npub fn main() { native.spawn_worker() }\n",
    )
    .expect("shutdown source");
    let run = geam_at(&project, ["run"]);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "initialized\nblocking-started\nentry-complete\nstate-drop:0\nblocking-finished\n"
    );

    fs::write(
        project.join("src/standalone_future.gleam"),
        "import standalone_future/native\npub fn main() { native.fail() }\n",
    )
    .expect("failure source");
    let run = geam_at(&project, ["run"]);
    assert!(!run.status.success());
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "initialized\nstate-drop:0\n"
    );
    let error = String::from_utf8_lossy(&run.stderr);
    assert!(error.contains("native failure"), "{error}");
    assert!(error.contains("standalone_future/native.fail"), "{error}");

    fs::write(
        project.join("src/standalone_future.gleam"),
        "import geam/future\nimport standalone_future/native\npub fn main() { future.all([native.pending(), native.fail()]) }\n",
    )
    .expect("pending sibling cancellation source");
    let run = geam_at(&project, ["run"]);
    assert!(!run.status.success());
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        "initialized\npending-started\npending-drop\nstate-drop:0\n"
    );
    assert!(String::from_utf8_lossy(&run.stderr).contains("native failure"));

    let prepared = geam_at(&project, ["prepare"]);
    assert!(
        prepared.status.success(),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    assert!(prepared.stdout.is_empty());
    assert_eq!(
        fs::read(project.join("Cargo.toml")).expect("manifest"),
        manifest
    );
    assert_eq!(fs::read(project.join("Cargo.lock")).expect("lock"), lock);
    assert_eq!(
        fs::read(project.join("build/geam/runner.rs")).expect("runner"),
        runner
    );
}

fn fixture() -> TempDir {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = root.join("cli/tests/fixtures/standalone_future");
    static PREPARED: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        &source.join("provider"),
        "cargo",
        &["fetch", "--locked", "--config", "net.offline=false"],
        "native fixture dependencies",
    );
    let fixture = tempdir().expect("temporary future fixture");
    copy_directory(&source, fixture.path());
    let project = fixture.path().join("project");
    let manifest = project.join("gleam.toml");
    let mut config = fs::read_to_string(&manifest)
        .expect("Gleam manifest")
        .parse::<toml::Table>()
        .expect("Gleam TOML");
    config["dependencies"]["geam"]["path"] = toml::Value::String(
        root.join("builtins/geam/gleam")
            .to_str()
            .expect("UTF-8 path")
            .to_owned(),
    );
    fs::write(&manifest, toml::to_string(&config).expect("manifest TOML"))
        .expect("local Future dependency");

    let manifest = fixture.path().join("provider/Cargo.toml");
    let mut config = fs::read_to_string(&manifest)
        .expect("provider manifest")
        .parse::<toml::Table>()
        .expect("provider TOML");
    config["patch"]["crates-io"]["geam"]["path"] =
        toml::Value::String(root.to_str().expect("workspace path").to_owned());
    fs::write(&manifest, toml::to_string(&config).expect("provider TOML")).expect("provider patch");
    fs::create_dir(project.join(".cargo")).expect("Cargo configuration");
    let root = toml::Value::String(root.to_str().expect("workspace path").to_owned());
    fs::write(
        project.join(".cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {root} }}\n\n[net]\noffline = true\n"),
    )
    .expect("checkout-only runner");
    fixture
}
