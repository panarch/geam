use crate::error::CliError;
use std::io::{self, Read, Write};
use std::process::{Command, Output, Stdio};
use std::thread;

pub(super) fn run(
    command: &mut Command,
    writer: &mut (dyn Write + Send),
    stdout: Stdio,
) -> Result<Output, CliError> {
    run_with_pipe(command, writer, stdout, io::pipe())
}

fn run_with_pipe(
    command: &mut Command,
    writer: &mut (dyn Write + Send),
    stdout: Stdio,
    pipe: io::Result<(io::PipeReader, io::PipeWriter)>,
) -> Result<Output, CliError> {
    let display = super::display_command(command);
    let output_error = |error| CliError::ProcessOutputIo {
        command: display.clone(),
        error,
    };
    let (mut reader, pipe_writer) = pipe.map_err(&output_error)?;
    command
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(pipe_writer);
    let mut stderr = Vec::new();
    let mut forwarding = Ok(());
    let output = thread::scope(|scope| {
        let captured = &mut stderr;
        let result = &mut forwarding;
        let relay = thread::Builder::new().spawn_scoped(scope, move || {
            *result = forward(&mut reader, writer, captured);
        });
        let output = relay.map_err(&output_error).and_then(|_| {
            command.output().map_err(|error| CliError::ProcessIo {
                command: display.clone(),
                error,
            })
        });
        // Command retains its pipe writer after the child exits; close it so
        // the relay can reach EOF before the scope joins its thread.
        command.stderr(Stdio::null());
        output
    })?;
    let output = super::check_status(display.clone(), Output { stderr, ..output })?;
    forwarding.map_err(output_error)?;
    Ok(output)
}

fn forward(
    reader: &mut dyn Read,
    writer: &mut dyn Write,
    captured: &mut Vec<u8>,
) -> io::Result<()> {
    let mut buffer = [0; 8192];
    let mut forwarding = Ok(());
    loop {
        let count = match reader.read(&mut buffer) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 {
            return forwarding;
        }
        let bytes = &buffer[..count];
        captured.extend_from_slice(bytes);
        if forwarding.is_ok() {
            forwarding = writer.write_all(bytes).and_then(|()| writer.flush());
        }
        // Continue draining after output failure so the child can finish and
        // its full stderr remains available for the structured process error.
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, run, run_with_pipe};
    use crate::error::CliError;
    use std::error::Error as _;
    use std::io::{self, Read, Write};
    use std::process::{Command, Stdio};
    use std::sync::mpsc;

    #[test]
    fn preserves_pipe_setup_failure_before_starting_a_child() {
        let mut native = Vec::new();
        let error = run_with_pipe(
            &mut Command::new("geam-command-that-does-not-exist"),
            &mut native,
            Stdio::piped(),
            Err(io::Error::other("pipe unavailable")),
        )
        .expect_err("pipe failure must precede process startup");
        assert_eq!(
            error.to_string(),
            "failed to forward output from `geam-command-that-does-not-exist`"
        );
        assert_eq!(
            error.source().expect("original IO error").to_string(),
            "pipe unavailable"
        );
        assert!(native.is_empty());
    }

    #[test]
    fn forwards_native_bytes_and_flushes_before_the_next_read() {
        let (sent, received) = mpsc::channel();
        let chunks: [&[u8]; 3] = [b"partial\r", b"\x1b[31m\xff", b"\nlast"];
        let mut output = AcknowledgedOutput {
            bytes: Vec::new(),
            sent,
        };
        let mut input = AcknowledgedInput {
            chunks: chunks.into_iter(),
            received,
            waiting: false,
        };
        let mut captured = Vec::new();
        forward(&mut input, &mut output, &mut captured).expect("native bytes should stream");
        assert_eq!(output.bytes, b"partial\r\x1b[31m\xff\nlast");
        assert_eq!(captured, output.bytes);
    }

    struct AcknowledgedOutput {
        bytes: Vec<u8>,
        sent: mpsc::Sender<()>,
    }

    impl Write for AcknowledgedOutput {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.sent
                .send(())
                .expect("reader should still be connected");
            Ok(())
        }
    }

    struct AcknowledgedInput<'a> {
        chunks: std::array::IntoIter<&'a [u8], 3>,
        received: mpsc::Receiver<()>,
        waiting: bool,
    }

    impl Read for AcknowledgedInput<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.waiting {
                self.received
                    .try_recv()
                    .expect("previous chunk must already be flushed");
            }
            match self.chunks.next() {
                Some(chunk) => {
                    buffer[..chunk.len()].copy_from_slice(chunk);
                    self.waiting = true;
                    Ok(chunk.len())
                }
                None => Ok(0),
            }
        }
    }

    struct FailedOutput {
        fail_write: bool,
    }

    impl Write for FailedOutput {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed writer"))
            } else {
                Ok(bytes.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed writer"))
        }
    }

    #[test]
    fn drains_and_captures_after_write_or_flush_failure() {
        let bytes = vec![b'x'; 32768];
        for fail_write in [true, false] {
            let mut input = bytes.as_slice();
            let mut captured = Vec::new();
            let error = forward(&mut input, &mut FailedOutput { fail_write }, &mut captured)
                .expect_err("closed writer must be reported after draining");
            assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
            assert!(input.is_empty());
            assert_eq!(captured, bytes);
        }
    }

    #[test]
    fn preserves_read_errors_and_retries_interrupted_reads() {
        struct InterruptedThenFailed(bool);
        impl Read for InterruptedThenFailed {
            fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
                let kind = if std::mem::take(&mut self.0) {
                    io::ErrorKind::Interrupted
                } else {
                    io::ErrorKind::BrokenPipe
                };
                Err(io::Error::new(kind, "read failed"))
            }
        }
        let error = forward(
            &mut InterruptedThenFailed(true),
            &mut Vec::new(),
            &mut Vec::new(),
        )
        .expect_err("non-interrupted read failure should propagate");
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    }

    #[cfg(unix)]
    #[test]
    fn captures_stdout_privately_while_streaming_large_stderr_without_reading_stdin() {
        let mut native = Vec::new();
        let output = run(
            Command::new("sh").args(["-c", "if read answer; then exit 9; fi; head -c 131072 /dev/zero; head -c 131072 /dev/zero >&2; printf '{\"ready\":true}'"]),
            &mut native,
            Stdio::piped(),
        ).expect("checked process should drain both pipes and see stdin EOF");
        let mut expected = vec![0; 131072];
        expected.extend_from_slice(b"{\"ready\":true}");
        assert_eq!(output.stdout, expected);
        assert_eq!(output.stderr, vec![0; 131072]);
        assert_eq!(native, output.stderr);
    }

    #[cfg(unix)]
    #[test]
    fn reaps_successful_children_before_reporting_relay_failure() {
        let marker = tempfile::tempdir().expect("marker directory");
        let path = marker.path().join("finished");
        let error = run(
            Command::new("sh")
                .args([
                    "-c",
                    "head -c 131072 /dev/zero >&2; printf finished > \"$1\"",
                    "sh",
                ])
                .arg(&path),
            &mut FailedOutput { fail_write: true },
            Stdio::inherit(),
        )
        .expect_err("output failure must not prevent the child from being drained and reaped");
        assert_eq!(std::fs::read(path).expect("child must finish"), b"finished");
        assert!(
            error
                .to_string()
                .starts_with("failed to forward output from `sh -c ")
        );
        assert!(matches!(error, CliError::ProcessOutputIo { error, .. }
            if error.kind() == io::ErrorKind::BrokenPipe));
    }

    #[cfg(unix)]
    #[test]
    fn keeps_stdout_on_its_destination_and_native_failure_context_after_relay_failure() {
        let directory = tempfile::tempdir().expect("output directory");
        let path = directory.path().join("stdout");
        let mut native = Vec::new();
        let output = run(
            Command::new("sh").args(["-c", "printf 'native stdout'; printf 'native stderr' >&2"]),
            &mut native,
            Stdio::from(std::fs::File::create(&path).expect("stdout destination")),
        )
        .expect("stdout should go directly to its configured destination");
        assert!(output.stdout.is_empty());
        assert_eq!(std::fs::read(path).expect("stdout bytes"), b"native stdout");
        assert_eq!(native, b"native stderr");
        assert_eq!(output.stderr, native);

        let error = run(
            Command::new("sh").args(["-c", "printf 'native failure' >&2; exit 7"]),
            &mut FailedOutput { fail_write: true },
            Stdio::piped(),
        )
        .expect_err("native failure must retain its command, status and captured stderr");
        assert!(
            matches!(error, CliError::ProcessFailure { command, status: Some(7), stderr }
            if command == "sh -c printf 'native failure' >&2; exit 7" && stderr == "native failure")
        );
    }

    #[test]
    fn preserves_start_and_status_failures_with_captured_stderr() {
        let mut native = Vec::new();
        let error = run(
            &mut Command::new("geam-command-that-does-not-exist"),
            &mut native,
            Stdio::piped(),
        )
        .expect_err("missing command should fail without hanging the relay");
        assert!(matches!(error, CliError::ProcessIo { error, .. }
            if error.kind() == io::ErrorKind::NotFound));

        let error = run(
            Command::new("rustc").arg("--definitely-not-a-rustc-option"),
            &mut native,
            Stdio::piped(),
        )
        .expect_err("native failure should retain stderr");
        assert!(
            matches!(error, CliError::ProcessFailure { command, status: Some(1), stderr }
            if command == "rustc --definitely-not-a-rustc-option"
                && stderr == String::from_utf8_lossy(&native).trim())
        );
    }
}
