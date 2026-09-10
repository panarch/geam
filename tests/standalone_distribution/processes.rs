use super::{copy_directory, geam_at, workspace_dependencies};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};

#[test]
fn runs_process_services_and_ends_the_domain_at_the_selected_entry() {
    let fixture = fixture();
    let project = fixture.path();
    let source = project.join("src/geam_rust_embedding_processes.gleam");
    let original = fs::read_to_string(&source).expect("documented service source");
    let erlang = Command::new("gleam")
        .arg("run")
        .current_dir(project)
        .output()
        .expect("Erlang oracle");
    assert!(
        erlang.status.success(),
        "{}",
        String::from_utf8_lossy(&erlang.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&erlang.stdout), "20\n42\n");

    for command in ["prepare", "prepare"] {
        let output = geam_at(project, [command]);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
    }
    let manifest = fs::read(project.join("Cargo.toml")).expect("managed manifest");
    let lock = fs::read(project.join("Cargo.lock")).expect("managed lock");
    let runner = fs::read(project.join("build/geam/runner.rs")).expect("managed runner");
    for _ in 0..2 {
        let output = geam_at(project, ["run"]);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, erlang.stdout);
    }

    for (body, expected) in [
        (
            r#"import gleam/erlang/process
import gleam/io
pub fn main() {
  let ready = process.new_subject()
  let _ = process.spawn_unlinked(fn() {
    process.send(ready, Nil)
    process.sleep_forever()
  })
  process.receive_forever(ready)
  io.println("main-complete")
}
"#,
            "main-complete\n",
        ),
        (
            r#"import gleam/erlang/process
import gleam/io
pub fn main() {
  let ready = process.new_subject()
  let pid = process.spawn_unlinked(fn() {
    let stop = process.new_subject()
    process.send(ready, stop)
    process.receive_forever(stop)
    panic as "background failure"
  })
  let stop = process.receive_forever(ready)
  let monitor = process.monitor(pid)
  process.send(stop, Nil)
  let assert process.ProcessDown(_, _, process.Abnormal(_)) = process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  io.println("main-survived")
}
"#,
            "main-survived\n",
        ),
        (
            r#"import geam/future
import gleam/erlang/process
import gleam/io
pub fn main() {
  let creator = process.self()
  use _ <- future.map(future.ready(Nil))
  let assert False = creator == process.self()
  let reply = process.new_subject()
  let _ = process.spawn_unlinked(fn() { process.send(reply, "future-complete") })
  process.receive_forever(reply) |> io.println
}
"#,
            "future-complete\n",
        ),
    ] {
        fs::write(&source, body).expect("entry lifecycle source");
        let output = geam_at(project, ["run"]);
        assert!(
            output.status.success(),
            "{body}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
    }

    fs::write(&source, original).expect("restore documented service source");
    let prepared = geam_at(project, ["prepare"]);
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
    let source = root.join("examples/embedding/processes/gleam");
    static PREPARED: OnceLock<Result<(), String>> = OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        &source,
        "gleam",
        &["deps", "download"],
        "process example dependencies",
    );
    let fixture = tempdir().expect("temporary process fixture");
    let project = fixture.path();
    copy_directory(&source, project);
    copy_directory(
        &source.join("build/packages"),
        &project.join("build/packages"),
    );
    let manifest = project.join("gleam.toml");
    let mut config = fs::read_to_string(&manifest)
        .expect("Gleam manifest")
        .parse::<toml::Table>()
        .expect("Gleam TOML");
    config["dependencies"]
        .as_table_mut()
        .expect("dependency table")
        .insert(
            "geam".to_owned(),
            toml::Value::Table(toml::Table::from_iter([(
                "path".to_owned(),
                toml::Value::String(
                    root.join("builtins/geam/gleam")
                        .to_str()
                        .expect("UTF-8 path")
                        .to_owned(),
                ),
            )])),
        );
    fs::write(manifest, toml::to_string(&config).expect("manifest TOML"))
        .expect("Future dependency");
    fs::create_dir_all(project.join(".cargo")).expect("Cargo configuration");
    let path = toml::Value::String(root.to_str().expect("UTF-8 root").to_owned());
    fs::write(
        project.join(".cargo/config.toml"),
        format!("[patch.crates-io]\ngeam = {{ path = {path} }}\n\n[net]\noffline = true\n"),
    )
    .expect("checkout runner configuration");
    fixture
}
