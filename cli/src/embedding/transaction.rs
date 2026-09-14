use super::output;
use super::package::EmbeddingProject;
use crate::error::CliError;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

pub(super) struct Inputs {
    files: [File; 3],
}

struct File {
    path: Utf8PathBuf,
    original: Option<Vec<u8>>,
    written: Option<Vec<u8>>,
}

#[derive(Clone, Copy)]
pub(super) enum Input {
    Manifest,
    CargoLock,
    GleamLock,
}

impl Inputs {
    pub(super) fn capture(project: &EmbeddingProject) -> Result<Self, CliError> {
        Ok(Self {
            files: [
                File::capture(project.manifest().to_owned())?,
                File::capture(project.cargo_lock_path())?,
                File::capture(project.project_root().join("manifest.toml"))?,
            ],
        })
    }

    pub(super) fn update<ResultValue>(
        &mut self,
        input: Input,
        operation: impl FnOnce() -> Result<ResultValue, CliError>,
    ) -> Result<ResultValue, CliError> {
        let result = operation();
        let file = &mut self.files[input as usize];
        match (result, read(&file.path)) {
            (result, Ok(written)) => {
                file.written = written;
                result
            }
            (Ok(_), Err(error)) => Err(error),
            (Err(failure), Err(rollback)) => Err(CliError::EmbeddingRestore {
                failure: Box::new(failure),
                rollback: Box::new(rollback),
            }),
        }
    }

    pub(super) fn restore(self, failure: CliError) -> CliError {
        match self.restore_files() {
            Ok(()) => failure,
            Err(rollback) => CliError::EmbeddingRestore {
                failure: Box::new(failure),
                rollback: Box::new(rollback),
            },
        }
    }

    fn restore_files(self) -> Result<(), CliError> {
        for file in self.files.into_iter().rev() {
            if file.written == file.original {
                continue;
            }
            if read(&file.path)? != file.written {
                return Err(CliError::EmbeddingFileConflict {
                    path: file.path,
                    reason: "the file changed after preparation; automatic recovery would overwrite another edit".to_owned(),
                });
            }
            match file.original {
                Some(bytes) => {
                    output::sync(&file.path.with_file_name(""), &file.path, &bytes)?;
                }
                None => fs::remove_file(&file.path).map_err(|error| CliError::FileWrite {
                    path: file.path,
                    error,
                })?,
            }
        }
        Ok(())
    }
}

impl File {
    fn capture(path: Utf8PathBuf) -> Result<Self, CliError> {
        let original = read(&path)?;
        Ok(Self {
            path,
            written: original.clone(),
            original,
        })
    }
}

fn read(path: &Utf8Path) -> Result<Option<Vec<u8>>, CliError> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(CliError::FileRead {
            path: path.to_owned(),
            error,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{File, Input, Inputs};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;

    #[test]
    fn restores_only_bytes_written_by_preparation_after_a_later_failure() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).unwrap();
        let manifest = root.join("Cargo.toml");
        let cargo_lock = root.join("Cargo.lock");
        let gleam_lock = root.join("manifest.toml");
        fs::write(&manifest, "original manifest").unwrap();
        fs::write(&gleam_lock, "original lock").unwrap();
        let mut inputs = Inputs {
            files: [
                File::capture(manifest.clone()).unwrap(),
                File::capture(cargo_lock.clone()).unwrap(),
                File::capture(gleam_lock.clone()).unwrap(),
            ],
        };
        inputs
            .update(Input::Manifest, || {
                fs::write(&manifest, "new features").unwrap();
                Ok(())
            })
            .unwrap();
        inputs
            .update(Input::CargoLock, || {
                fs::write(&cargo_lock, "new cargo lock").unwrap();
                Ok(())
            })
            .unwrap();
        let failure = inputs
            .update(Input::GleamLock, || {
                fs::write(&gleam_lock, "new gleam lock").unwrap();
                Err::<(), _>(CliError::EmbeddingFileConflict {
                    path: root.join("provider"),
                    reason: "registration failed".into(),
                })
            })
            .unwrap_err();
        assert_eq!(
            inputs.restore(failure).to_string(),
            format!(
                "refusing to replace existing embedding file {}/provider: registration failed",
                root
            )
        );
        assert_eq!(fs::read_to_string(manifest).unwrap(), "original manifest");
        assert_eq!(fs::read_to_string(gleam_lock).unwrap(), "original lock");
        assert!(!cargo_lock.exists());
    }

    #[test]
    fn preserves_later_external_edits_and_reports_recovery_failure() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).unwrap();
        let path = root.join("manifest.toml");
        let mut inputs = Inputs {
            files: [
                File::capture(root.join("Cargo.toml")).unwrap(),
                File::capture(root.join("Cargo.lock")).unwrap(),
                File::capture(path.clone()).unwrap(),
            ],
        };
        inputs
            .update(Input::GleamLock, || {
                fs::write(&path, "prepared").unwrap();
                Ok(())
            })
            .unwrap();
        fs::write(&path, "user change").unwrap();
        let failure = CliError::InvalidEmbeddingBoundary {
            module: "main".into(),
            reason: "fixture failure".into(),
        };
        let failure = inputs.restore(failure);
        assert!(
            failure
                .to_string()
                .contains("automatic recovery would overwrite another edit")
        );
        assert!(
            matches!(failure, CliError::EmbeddingRestore { failure, .. } if failure.to_string().contains("fixture failure"))
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "user change");
    }

    #[test]
    fn leaves_unchanged_inputs_alone_and_propagates_unreadable_files() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).unwrap();
        let path = root.join("Cargo.toml");
        fs::write(&path, "unchanged").unwrap();
        let modified = fs::metadata(&path).unwrap().modified().unwrap();
        let inputs = Inputs {
            files: [
                File::capture(path.clone()).unwrap(),
                File::capture(root.join("missing")).unwrap(),
                File::capture(root.join("absent")).unwrap(),
            ],
        };
        inputs.restore_files().unwrap();
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), modified);
        assert!(matches!(
            File::capture(root.clone()).err().unwrap(),
            CliError::FileRead { path, .. } if path == root
        ));
    }

    #[cfg(unix)]
    #[test]
    fn reports_failed_reads_and_failed_recovery_without_hiding_the_original_failure() {
        use std::os::unix::fs::PermissionsExt;
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(directory.path().to_owned()).unwrap();
        let path = root.join("Cargo.lock");
        for original in [None, Some("original")] {
            if let Some(bytes) = original {
                fs::write(&path, bytes).unwrap();
            }
            let mut inputs = Inputs {
                files: [
                    File::capture(root.join("Cargo.toml")).unwrap(),
                    File::capture(path.clone()).unwrap(),
                    File::capture(root.join("manifest.toml")).unwrap(),
                ],
            };
            inputs
                .update(Input::CargoLock, || {
                    fs::write(&path, "written").unwrap();
                    Ok(())
                })
                .unwrap();
            fs::set_permissions(&root, fs::Permissions::from_mode(0o500)).unwrap();
            let error = inputs.restore(CliError::InvalidEmbeddingBoundary {
                module: "main".into(),
                reason: "original failure".into(),
            });
            fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
            assert!(error.to_string().contains("original failure"));
            assert!(
                matches!(error, CliError::EmbeddingRestore { failure, .. } if failure.to_string().contains("original failure"))
            );
            assert_eq!(fs::read_to_string(&path).unwrap(), "written");
            fs::remove_file(&path).unwrap();
        }
        let mut inputs = Inputs {
            files: [
                File::capture(root.join("Cargo.toml")).unwrap(),
                File::capture(path.clone()).unwrap(),
                File::capture(root.join("manifest.toml")).unwrap(),
            ],
        };
        for (update_fails, read_fails) in
            [(false, false), (true, false), (false, true), (true, true)]
        {
            let result = inputs.update(Input::CargoLock, || {
                if read_fails {
                    fs::create_dir(&path).unwrap();
                } else {
                    fs::write(&path, "updated").unwrap();
                }
                if update_fails {
                    Err(CliError::InvalidEmbeddingBoundary {
                        module: "main".into(),
                        reason: "original update failure".into(),
                    })
                } else {
                    Ok(())
                }
            });
            match (update_fails, read_fails) {
                (false, false) => result.unwrap(),
                (true, false) => assert_eq!(
                    result.unwrap_err().to_string(),
                    "invalid Rust embedding boundary module main: original update failure"
                ),
                (false, true) => assert!(
                    matches!(result.unwrap_err(), CliError::FileRead { path: actual, .. } if actual == path)
                ),
                (true, true) => {
                    let error = result.unwrap_err();
                    assert!(error.to_string().contains("original update failure"));
                    assert!(
                        matches!(error, CliError::EmbeddingRestore { rollback, .. } if matches!(rollback.as_ref(), CliError::FileRead { path: actual, .. } if actual == &path))
                    );
                }
            }
            if read_fails {
                fs::remove_dir(&path).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
        }
        inputs
            .update(Input::CargoLock, || {
                fs::write(&path, "prepared").unwrap();
                Ok(())
            })
            .unwrap();
        fs::remove_file(&path).unwrap();
        fs::create_dir(&path).unwrap();
        assert!(matches!(
            inputs.restore_files().unwrap_err(),
            CliError::FileRead { path: actual, .. } if actual == path
        ));
    }

    #[test]
    fn captures_all_three_input_files_before_any_preparation_write() {
        let directory = tempfile::tempdir().unwrap();
        let root = Utf8PathBuf::from_path_buf(fs::canonicalize(directory.path()).unwrap()).unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("gleam")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = 'capture'\nversion = '1.0.0'\n[workspace]\n",
        )
        .unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}\n").unwrap();
        let project = crate::embedding::package::EmbeddingProject::load(&root).unwrap();
        for file in ["Cargo.toml", "Cargo.lock", "gleam/manifest.toml"] {
            let path = root.join(file);
            let original = super::read(&path).unwrap();
            if original.is_some() {
                fs::remove_file(&path).unwrap();
            }
            fs::create_dir(&path).unwrap();
            let error = Inputs::capture(&project).err().unwrap();
            assert!(matches!(error, CliError::FileRead { path: actual, .. } if actual == path));
            fs::remove_dir(&path).unwrap();
            if let Some(original) = original {
                fs::write(&path, original).unwrap();
            }
        }
    }
}
