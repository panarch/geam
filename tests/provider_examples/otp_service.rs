use super::{copy_directory, geam_at};
use std::fs;
use std::path::Path;

#[test]
fn runs_profile_dependent_otp_callbacks_in_the_generated_standalone_host() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let directory = tempfile::tempdir().unwrap();
    let source = repository.join("tests/fixtures/otp_service");
    let project = directory.path().join("project");
    copy_directory(&source.join("project"), &project);
    copy_directory(&source.join("provider"), &directory.path().join("provider"));
    fs::create_dir_all(project.join(".cargo")).unwrap();
    let repository_path = repository.to_str().unwrap();
    let target = repository.join("target/prepared-acceptance");
    let target_path = target.to_str().unwrap();
    fs::write(
        project.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
            [build]
            target-dir = target_path
        })
        .unwrap(),
    )
    .unwrap();
    let add = geam_at(&project, ["provider", "add", "--path", "../provider"]);
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let prepared = geam_at(&project, ["prepare"]);
    assert!(
        prepared.status.success(),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    let manifest = fs::read(project.join("Cargo.toml")).unwrap();
    let lock = fs::read(project.join("Cargo.lock")).unwrap();
    let runner = fs::read(project.join("build/geam/runner.rs")).unwrap();
    for _ in 0..2 {
        let run = geam_at(&project, ["run"]);
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(run.stdout, b"original actor: state, suspend, resume\noriginal static child: retained callback in supervisor\noriginal factory: two typed callbacks, named and pid handles\n");
        assert_eq!(fs::read(project.join("Cargo.toml")).unwrap(), manifest);
        assert_eq!(fs::read(project.join("Cargo.lock")).unwrap(), lock);
        assert_eq!(
            fs::read(project.join("build/geam/runner.rs")).unwrap(),
            runner
        );
    }
}
