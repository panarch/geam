use crate::error::CliError;
use std::fmt::Arguments;
use std::io::Write;

pub(crate) enum Progress<'a> {
    Hidden,
    Visible(&'a mut (dyn Write + Send)),
}

impl Progress<'_> {
    pub(crate) fn report(&mut self, message: Arguments<'_>) -> Result<(), CliError> {
        match self {
            Self::Hidden => Ok(()),
            Self::Visible(writer) => writeln!(writer, "geam: {message}")
                .and_then(|()| writer.flush())
                .map_err(CliError::PreparationProgressIo),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Progress;
    use crate::error::CliError;
    use std::io::{self, Write};

    #[derive(Default)]
    struct RecordedOutput {
        bytes: Vec<u8>,
        flushes: Vec<usize>,
    }

    impl Write for RecordedOutput {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes.push(self.bytes.len());
            Ok(())
        }
    }

    #[test]
    fn appends_and_flushes_each_geam_phase_without_terminal_controls() {
        let mut output = RecordedOutput::default();
        let mut progress = Progress::Visible(&mut output);
        progress
            .report(format_args!("Preparing app"))
            .expect("first phase");
        progress
            .report(format_args!("Prepared app"))
            .expect("completed phase");

        assert_eq!(output.bytes, b"geam: Preparing app\ngeam: Prepared app\n");
        assert_eq!(output.flushes, [20, 39]);
        Progress::Hidden
            .report(format_args!("hidden"))
            .expect("hidden output");
    }

    struct FailedOutput {
        fail_write: bool,
    }

    impl Write for FailedOutput {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed output"))
            } else {
                Ok(bytes.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed output"))
        }
    }

    #[test]
    fn preserves_phase_write_and_flush_failures() {
        for fail_write in [true, false] {
            let mut output = FailedOutput { fail_write };
            let error = Progress::Visible(&mut output)
                .report(format_args!("Preparing app"))
                .expect_err("closed output must fail");
            assert_eq!(error.to_string(), "failed to write preparation progress");
            assert!(matches!(error, CliError::PreparationProgressIo(error)
                if error.kind() == io::ErrorKind::BrokenPipe));
        }
    }
}
