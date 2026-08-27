use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

pub fn prepare(
    prepared: &'static OnceLock<Result<(), String>>,
    current_dir: &Path,
    program: impl AsRef<OsStr>,
    arguments: &[&str],
    description: &str,
) {
    prepare_once(prepared, || {
        let mut command = Command::new(program);
        command.args(arguments);
        run(&mut command, current_dir, description)
    });
}

fn prepare_once(
    prepared: &'static OnceLock<Result<(), String>>,
    prepare: impl FnOnce() -> Result<(), String>,
) {
    if let Err(error) = prepared.get_or_init(prepare) {
        panic!("{error}");
    }
}

fn run(command: &mut Command, current_dir: &Path, description: &str) -> Result<(), String> {
    let output = command.current_dir(current_dir).output().map_err(|error| {
        format!(
            "failed to run {description} in {}: {error}",
            current_dir.display(),
        )
    })?;
    if output.status.success() {
        return Ok(());
    }

    Err(format!(
        "{description} failed in {} with status {}\nstdout:\n{}\nstderr:\n{}",
        current_dir.display(),
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    ))
}
