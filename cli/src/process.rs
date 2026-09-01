mod streaming;

use crate::error::CliError;
use crate::progress::Progress;
use std::ffi::OsStr;
use std::process::{Command, Output, Stdio};

pub(super) fn run_checked(command: &mut Command) -> Result<Output, CliError> {
    let display = display_command(command);
    let output = command.output().map_err(|error| CliError::ProcessIo {
        command: display.clone(),
        error,
    })?;
    check_status(display, output)
}

pub(crate) fn run_checked_with_progress(
    command: &mut Command,
    progress: &mut Progress<'_>,
    stdout: Stdio,
) -> Result<Output, CliError> {
    match progress {
        Progress::Hidden => run_checked(command),
        Progress::Visible(writer) => streaming::run(command, *writer, stdout),
    }
}

pub(super) fn run_inherited(command: &mut Command) -> Result<(), CliError> {
    let display = display_command(command);
    let status = command.status().map_err(|error| CliError::ProcessIo {
        command: display.clone(),
        error,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(CliError::InheritedProcessFailure {
            command: display,
            status: status.code(),
        })
    }
}

fn display_command(command: &Command) -> String {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(OsStr::to_string_lossy)
        .collect::<Vec<_>>()
        .join(" ")
}

fn check_status(command: String, output: Output) -> Result<Output, CliError> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(CliError::ProcessFailure {
            command,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{run_checked, run_checked_with_progress, run_inherited};
    use crate::error::CliError;
    use crate::progress::Progress;
    use std::process::{Command, Stdio};

    #[test]
    fn returns_successful_process_output() {
        let output = run_checked(Command::new("rustc").arg("--version"))
            .expect("rustc version should succeed");

        assert!(String::from_utf8_lossy(&output.stdout).starts_with("rustc "));
    }

    #[cfg(unix)]
    #[test]
    fn selects_capture_or_live_output_without_exposing_machine_stdout() {
        let native = "printf '{\"packages\":[]}'; printf 'native\\routput' >&2";
        let captured = run_checked_with_progress(
            Command::new("sh").args(["-c", native]),
            &mut Progress::Hidden,
            Stdio::inherit(),
        )
        .expect("hidden progress should retain both streams");
        assert_eq!(captured.stdout, b"{\"packages\":[]}");
        assert_eq!(captured.stderr, b"native\routput");

        let mut progress = Vec::new();
        let streamed = run_checked_with_progress(
            Command::new("sh").args(["-c", native]),
            &mut Progress::Visible(&mut progress),
            Stdio::piped(),
        )
        .expect("visible progress should forward stderr but keep JSON private");
        assert_eq!(streamed.stdout, captured.stdout);
        assert_eq!(streamed.stderr, captured.stderr);
        assert_eq!(progress, b"native\routput");
    }

    #[test]
    fn preserves_failed_process_status_and_stderr() {
        let error = run_checked(Command::new("rustc").arg("--definitely-not-a-rustc-option"))
            .expect_err("invalid rustc option should fail");

        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(1), stderr }
                if command == "rustc --definitely-not-a-rustc-option"
                    && stderr.contains("Unrecognized option")
        ));
    }

    #[test]
    fn preserves_process_start_failures() {
        let mut command = Command::new("geam-command-that-does-not-exist");
        let error = run_checked(&mut command).expect_err("missing process should fail");

        assert!(matches!(
            error,
            CliError::ProcessIo { command, error }
                if command == "geam-command-that-does-not-exist"
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn runs_with_inherited_streams_and_preserves_status_failures() {
        run_inherited(
            Command::new("rustc")
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null()),
        )
        .expect("inherited process should succeed");

        let error = run_inherited(
            Command::new("rustc")
                .arg("--definitely-not-a-rustc-option")
                .stdout(Stdio::null())
                .stderr(Stdio::null()),
        )
        .expect_err("inherited process status should be preserved");
        assert!(matches!(
            error,
            CliError::InheritedProcessFailure { command, status: Some(1) }
                if command == "rustc --definitely-not-a-rustc-option"
        ));

        let error = run_inherited(&mut Command::new("geam-command-that-does-not-exist"))
            .expect_err("missing inherited process should fail to start");
        assert!(matches!(
            error,
            CliError::ProcessIo { command, error }
                if command == "geam-command-that-does-not-exist"
                    && error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[cfg(unix)]
    #[test]
    fn leaves_application_input_and_output_destinations_untouched() {
        let directory = tempfile::tempdir().expect("stream directory");
        let input = directory.path().join("stdin");
        let stdout = directory.path().join("stdout");
        let stderr = directory.path().join("stderr");
        std::fs::write(&input, b"application input\n").expect("stdin bytes");
        run_inherited(
            Command::new("sh")
                .args(["-c", "cat; printf 'application stderr' >&2"])
                .stdin(std::fs::File::open(input).expect("stdin source"))
                .stdout(std::fs::File::create(&stdout).expect("stdout destination"))
                .stderr(std::fs::File::create(&stderr).expect("stderr destination")),
        )
        .expect("application streams should remain directly connected");
        assert_eq!(
            std::fs::read(stdout).expect("stdout bytes"),
            b"application input\n"
        );
        assert_eq!(
            std::fs::read(stderr).expect("stderr bytes"),
            b"application stderr"
        );
    }
}
