use crate::Result;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// A destination whose completed records must reach durable storage.
pub(super) trait DurableWrite: Write {
    fn synchronize(&mut self) -> io::Result<()>;
}

impl DurableWrite for File {
    fn synchronize(&mut self) -> io::Result<()> {
        self.sync_all()
    }
}

pub(super) fn write_json_new(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut file = File::create_new(path)?;
    write_json(&mut file, value)
}

fn write_json(writer: &mut dyn DurableWrite, value: &impl Serialize) -> Result<()> {
    serde_json::to_writer_pretty(&mut *writer, value)?;
    writeln!(writer)?;
    writer.synchronize()?;
    Ok(())
}

/// Publish a completion marker only after its complete bytes are durable.
pub(super) fn seal_json_new(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut pending = tempfile::NamedTempFile::new_in(path.with_file_name(""))?;
    let written = write_json(pending.as_file_mut(), value);
    publish(path, pending, written)
}

fn publish(path: &Path, pending: tempfile::NamedTempFile, written: Result<()>) -> Result<()> {
    written?;
    pending.persist_noclobber(path)?;
    Ok(())
}

pub(super) fn write_batch<T: Serialize>(
    writer: &mut dyn DurableWrite,
    records: impl IntoIterator<Item = T>,
) -> Result<()> {
    for record in records {
        serde_json::to_writer(&mut *writer, &record)?;
        writeln!(writer)?;
    }
    writer.synchronize()?;
    Ok(())
}

pub(super) fn write_text_new(path: &Path, text: &str) -> Result<()> {
    let mut file = File::create_new(path)?;
    write_text(&mut file, text)
}

fn write_text(writer: &mut dyn DurableWrite, text: &str) -> Result<()> {
    writer.write_all(text.as_bytes())?;
    writer.synchronize()?;
    Ok(())
}

pub(super) fn finish<T>(
    outcome: Result<T>,
    output: &Path,
    diagnostics: &mut dyn Write,
) -> Result<T> {
    if let Err(error) = &outcome
        && let Err(receipt) = fs::write(output.join("failure.txt"), error.to_string())
    {
        let _ = writeln!(diagnostics, "Could not save failure.txt: {receipt}");
    }
    outcome
}

#[cfg(test)]
mod tests {
    use super::{
        DurableWrite, finish, write_batch, write_json, write_json_new, write_text, write_text_new,
    };
    use std::fs;
    use std::io::{self, Write};

    struct Destination {
        bytes: Vec<u8>,
        remaining: usize,
        fail_sync: bool,
        synchronized: bool,
    }

    impl Write for Destination {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.remaining == 0 {
                return Err(io::Error::other("destination write failed"));
            }
            let count = bytes.len().min(self.remaining);
            self.bytes.extend_from_slice(&bytes[..count]);
            self.remaining -= count;
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl DurableWrite for Destination {
        fn synchronize(&mut self) -> io::Result<()> {
            if self.fail_sync {
                return Err(io::Error::other("destination sync failed"));
            }
            self.synchronized = true;
            Ok(())
        }
    }

    #[test]
    fn framing_preserves_partial_bytes_and_requires_durable_completion() {
        for (limit, fail_sync, expected, failure) in [
            (0, false, b"".as_slice(), Some("destination write failed")),
            (
                4,
                false,
                b"null".as_slice(),
                Some("destination write failed"),
            ),
            (
                usize::MAX,
                true,
                b"null\n".as_slice(),
                Some("destination sync failed"),
            ),
            (usize::MAX, false, b"null\n".as_slice(), None),
        ] {
            let mut destination = Destination {
                bytes: Vec::new(),
                remaining: limit,
                fail_sync,
                synchronized: false,
            };
            let result = write_json(&mut destination, &());
            assert_eq!(
                result.err().map(|error| error.to_string()).as_deref(),
                failure
            );
            assert_eq!(destination.bytes, expected);
            assert_eq!(destination.synchronized, failure.is_none());

            destination.bytes.clear();
            destination.remaining = limit;
            destination.synchronized = false;
            let result = write_batch(&mut destination, [()]);
            assert_eq!(
                result.err().map(|error| error.to_string()).as_deref(),
                failure
            );
            assert_eq!(destination.bytes, expected);
            assert_eq!(destination.synchronized, failure.is_none());
            destination.flush().unwrap();
        }
        let mut destination = Destination {
            bytes: Vec::new(),
            remaining: usize::MAX,
            fail_sync: false,
            synchronized: false,
        };
        write_batch(&mut destination, [1, 2]).unwrap();
        assert_eq!(destination.bytes, b"1\n2\n");
        assert!(destination.synchronized);
    }

    #[test]
    fn reports_preserve_write_and_sync_failures() {
        for (limit, fail_sync, expected, failure) in [
            (0, false, b"".as_slice(), Some("destination write failed")),
            (
                usize::MAX,
                true,
                b"report\n".as_slice(),
                Some("destination sync failed"),
            ),
            (usize::MAX, false, b"report\n".as_slice(), None),
        ] {
            let mut destination = Destination {
                bytes: Vec::new(),
                remaining: limit,
                fail_sync,
                synchronized: false,
            };
            let result = write_text(&mut destination, "report\n");
            assert_eq!(
                result.err().map(|error| error.to_string()).as_deref(),
                failure
            );
            assert_eq!(destination.bytes, expected);
            assert_eq!(destination.synchronized, failure.is_none());
        }
    }

    #[test]
    fn new_files_are_exact_and_never_replace_an_existing_path() {
        let root = tempfile::tempdir().unwrap();
        let json = root.path().join("record.json");
        write_json_new(&json, &vec![1, 2]).unwrap();
        assert_eq!(fs::read(&json).unwrap(), b"[\n  1,\n  2\n]\n");
        assert!(write_json_new(&json, &vec![3]).is_err());
        let invalid = std::collections::BTreeMap::from([((1, 2), 3)]);
        assert_eq!(
            write_json_new(&root.path().join("invalid.json"), &invalid)
                .unwrap_err()
                .to_string(),
            "key must be a string"
        );
        let report = root.path().join("REPORT.md");
        write_text_new(&report, "report\n").unwrap();
        assert_eq!(fs::read(&report).unwrap(), b"report\n");
        assert!(write_text_new(&report, "replacement").is_err());
    }

    #[test]
    fn failure_receipt_errors_do_not_replace_the_primary_failure() {
        let root = tempfile::tempdir().unwrap();
        let mut diagnostics = Vec::new();
        assert_eq!(finish(Ok(42), root.path(), &mut diagnostics).unwrap(), 42);
        assert!(!root.path().join("failure.txt").exists());
        assert!(diagnostics.is_empty());
        let failed = || Err::<(), _>(io::Error::other("original failure").into());
        assert_eq!(
            finish(failed(), root.path(), &mut diagnostics)
                .unwrap_err()
                .to_string(),
            "original failure"
        );
        assert_eq!(
            fs::read(root.path().join("failure.txt")).unwrap(),
            b"original failure"
        );
        assert!(diagnostics.is_empty());
        fs::remove_file(root.path().join("failure.txt")).unwrap();
        fs::create_dir(root.path().join("failure.txt")).unwrap();
        assert_eq!(
            finish(failed(), root.path(), &mut diagnostics)
                .unwrap_err()
                .to_string(),
            "original failure"
        );
        assert_eq!(
            String::from_utf8(diagnostics).unwrap(),
            "Could not save failure.txt: Is a directory (os error 21)\n"
        );
    }
    #[test]
    fn completion_markers_publish_only_after_durable_writes_without_replacement() {
        let root = tempfile::tempdir().unwrap();
        let marker = root.path().join("complete.json");
        let pending = tempfile::NamedTempFile::new_in(root.path()).unwrap();
        let temporary_path = pending.path().to_path_buf();
        fs::write(&temporary_path, b"null\n").unwrap();
        let error = super::publish(
            &marker,
            pending,
            Err(io::Error::other("sync failed").into()),
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "sync failed");
        assert!(!marker.exists());
        assert!(!temporary_path.exists());
        let invalid = std::collections::BTreeMap::from([((1, 2), 3)]);
        assert_eq!(
            super::seal_json_new(&marker, &invalid)
                .unwrap_err()
                .to_string(),
            "key must be a string"
        );
        assert!(!marker.exists());
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
        assert!(super::seal_json_new(&root.path().join("missing/complete.json"), &()).is_err());
        super::seal_json_new(&marker, &()).unwrap();
        assert_eq!(fs::read(&marker).unwrap(), b"null\n");
        assert!(super::seal_json_new(&marker, &42).is_err());
        assert_eq!(fs::read(&marker).unwrap(), b"null\n");
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    }
}
