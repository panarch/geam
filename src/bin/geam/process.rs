use crate::error::CliError;
use std::ffi::OsStr;
use std::process::{Command, Output};

pub(super) fn run_checked(command: &mut Command) -> Result<Output, CliError> {
    let display = display_command(command);
    let output = command.output().map_err(|error| CliError::ProcessIo {
        command: display.clone(),
        error,
    })?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(CliError::ProcessFailure {
            command: display,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
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

#[cfg(test)]
mod tests {
    use super::run_checked;
    use crate::error::CliError;
    use std::process::Command;

    #[test]
    fn returns_successful_process_output() {
        let output = run_checked(Command::new("rustc").arg("--version"))
            .expect("rustc version should succeed");

        assert!(String::from_utf8_lossy(&output.stdout).starts_with("rustc "));
    }

    #[test]
    fn preserves_failed_process_status_and_stderr() {
        let error = run_checked(Command::new("rustc").arg("--definitely-not-a-rustc-option"))
            .expect_err("invalid rustc option should fail");

        assert!(matches!(
            error,
            CliError::ProcessFailure { command, status: Some(_), stderr }
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
            CliError::ProcessIo { command, .. }
                if command == "geam-command-that-does-not-exist"
        ));
    }
}
