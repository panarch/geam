use super::registry::ProviderCandidate;
use crate::error::CliError;
use hexpm::version::Version as GleamVersion;
use std::io::{BufRead, Write};

pub(crate) trait ProviderApproval {
    fn approve(
        &mut self,
        package: &str,
        gleam_version: &GleamVersion,
        replacing: Option<&str>,
        candidates: &[ProviderCandidate],
    ) -> Result<ProviderCandidate, CliError>;
}

pub(crate) struct TerminalApproval<'io> {
    context: ApprovalContext,
    terminal: bool,
    reader: &'io mut dyn BufRead,
    writer: &'io mut dyn Write,
}

enum ApprovalContext {
    Standalone,
    Embedding,
}

impl<'io> TerminalApproval<'io> {
    pub(crate) fn new(
        terminal: bool,
        reader: &'io mut dyn BufRead,
        writer: &'io mut dyn Write,
    ) -> Self {
        Self {
            context: ApprovalContext::Standalone,
            terminal,
            reader,
            writer,
        }
    }

    pub(crate) fn for_embedding(
        terminal: bool,
        reader: &'io mut dyn BufRead,
        writer: &'io mut dyn Write,
    ) -> Self {
        Self {
            context: ApprovalContext::Embedding,
            ..Self::new(terminal, reader, writer)
        }
    }
}

impl ProviderApproval for TerminalApproval<'_> {
    fn approve(
        &mut self,
        package: &str,
        gleam_version: &GleamVersion,
        replacing: Option<&str>,
        candidates: &[ProviderCandidate],
    ) -> Result<ProviderCandidate, CliError> {
        if !self.terminal {
            let candidate = &candidates[0];
            return Err(CliError::ProviderApprovalRequired {
                package: package.to_owned(),
                command: match self.context {
                    ApprovalContext::Standalone => format!(
                        "geam provider add {}@{}",
                        candidate.crate_name(),
                        candidate.version(),
                    ),
                    ApprovalContext::Embedding => "geam embedding sync".to_owned(),
                },
            });
        }

        let mut output =
            format!("Gleam package {package} {gleam_version} requires native provider code.\n");
        if let Some(provider) = replacing {
            output.push_str(&format!(
                "The selected provider {provider} is no longer compatible and must be replaced.\n"
            ));
        }
        output.push_str("Metadata compatibility is not an endorsement.\n");
        for (index, candidate) in candidates.iter().enumerate() {
            output.push_str(&format!(
                "  {}. {} {} (Gleam {})\n",
                index + 1,
                candidate.crate_name(),
                candidate.version(),
                candidate.gleam_range(),
            ));
        }
        let mut selection = (candidates.len() == 1).then_some(0);
        loop {
            if let Some(selection) = selection {
                output.push_str(&format!(
                    "Approve {} {}? [y/N] ",
                    candidates[selection].crate_name(),
                    candidates[selection].version(),
                ));
            } else {
                output.push_str(&format!(
                    "Select a provider [1-{}], or 0 to cancel: ",
                    candidates.len(),
                ));
            }
            self.write(&output)?;
            output.clear();
            let Some(response) = self.read()? else {
                return Err(CliError::ProviderApprovalCancelled {
                    package: package.to_owned(),
                });
            };
            if let Some(selected) = selection {
                match response.to_ascii_lowercase().as_str() {
                    "y" | "yes" => return Ok(candidates[selected].clone()),
                    "" | "n" | "no" => {
                        return Err(CliError::ProviderApprovalCancelled {
                            package: package.to_owned(),
                        });
                    }
                    _ => output.push_str("Enter yes or no.\n"),
                }
            } else {
                match response.parse::<usize>() {
                    Ok(0) => {
                        return Err(CliError::ProviderApprovalCancelled {
                            package: package.to_owned(),
                        });
                    }
                    Ok(selected) if selected <= candidates.len() => selection = Some(selected - 1),
                    _ => output.push_str("Enter one of the listed numbers.\n"),
                }
            }
        }
    }
}

impl TerminalApproval<'_> {
    fn read(&mut self) -> Result<Option<String>, CliError> {
        let mut response = String::new();
        let read =
            self.reader
                .read_line(&mut response)
                .map_err(|error| CliError::ProviderApprovalIo {
                    operation: "read",
                    error,
                })?;
        Ok((read != 0).then(|| response.trim().to_owned()))
    }

    fn write(&mut self, output: &str) -> Result<(), CliError> {
        self.writer
            .write_all(output.as_bytes())
            .and_then(|()| self.writer.flush())
            .map_err(|error| CliError::ProviderApprovalIo {
                operation: "write",
                error,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderApproval, TerminalApproval};
    use crate::error::CliError;
    use crate::provider::metadata::ProviderMetadata;
    use crate::provider::registry::ProviderCandidate;
    use semver::Version;
    use std::io::{self, Cursor, Read};

    #[test]
    fn approves_one_candidate_and_describes_replacements() {
        let mut input = Cursor::new(b"maybe\nyes\n");
        let mut output = Vec::new();
        let candidates = [candidate(
            "geam-images",
            "1.2.3",
            "images",
            ">= 1.0.0 and < 2.0.0",
        )];

        {
            let mut approval = TerminalApproval::new(true, &mut input, &mut output);
            assert_eq!(
                approval
                    .approve(
                        "images",
                        &hexpm::version::Version::new(1, 5, 0),
                        Some("geam-images-old"),
                        &candidates,
                    )
                    .expect("candidate should be approved")
                    .crate_name(),
                "geam-images",
            );
        }
        let output = String::from_utf8(output).expect("output should be UTF-8");
        assert!(output.contains("images 1.5.0 requires native provider code"));
        assert!(output.contains("geam-images-old is no longer compatible"));
        assert!(output.contains("Metadata compatibility is not an endorsement"));
        assert!(output.contains("1. geam-images 1.2.3 (Gleam >= 1.0.0 and < 2.0.0)"));
        assert!(output.contains("Enter yes or no."));
    }

    #[test]
    fn selects_numbered_candidates_and_cancels_without_approval() {
        let candidates = [
            candidate("geam-images", "1.0.0", "images", ">= 1.0.0"),
            candidate("geam-images-alt", "2.0.0", "images", ">= 1.0.0"),
        ];
        let mut input = Cursor::new(b"bad\n3\n2\ny\n");
        let mut output = Vec::new();
        {
            let mut selected = TerminalApproval::new(true, &mut input, &mut output);
            assert_eq!(
                selected
                    .approve(
                        "images",
                        &hexpm::version::Version::new(1, 0, 0),
                        None,
                        &candidates,
                    )
                    .expect("second candidate should be approved")
                    .crate_name(),
                "geam-images-alt",
            );
        }
        let output = String::from_utf8(output).expect("output should be UTF-8");
        assert_eq!(
            output.matches("Enter one of the listed numbers.").count(),
            2
        );

        for input in [b"".as_slice(), b"0\n", b"1\nno\n", b"1\n"] {
            let mut reader = Cursor::new(input);
            let mut output = Vec::<u8>::new();
            let mut cancelled = TerminalApproval::new(true, &mut reader, &mut output);
            assert!(matches!(
                cancelled.approve(
                    "images",
                    &hexpm::version::Version::new(1, 0, 0),
                    None,
                    &candidates,
                ),
                Err(CliError::ProviderApprovalCancelled { ref package }) if package == "images"
            ));
        }
    }

    #[test]
    fn refuses_nonterminal_selection_with_an_exact_explicit_command() {
        let candidates = [candidate("geam-images-alt", "2.3.4", "images", ">= 1.0.0")];
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::<u8>::new();
        let mut approval = TerminalApproval::new(false, &mut input, &mut output);

        assert!(matches!(
            approval.approve(
                "images",
                &hexpm::version::Version::new(1, 0, 0),
                None,
                &candidates,
            ),
            Err(CliError::ProviderApprovalRequired { ref package, ref command })
                if package == "images" && command == "geam provider add geam-images-alt@2.3.4"
        ));
    }

    #[test]
    fn embedding_uses_the_same_prompt_with_its_own_noninteractive_recovery_command() {
        let candidates = [candidate("geam-images", "1.2.3", "images", ">= 1.0.0")];
        let version = hexpm::version::Version::new(1, 0, 0);
        let mut input = Cursor::new(b"y\n");
        let mut output = Vec::new();
        let error = TerminalApproval::for_embedding(false, &mut input, &mut output)
            .approve("images", &version, None, &candidates)
            .expect_err("noninteractive embedding must not approve native code");
        assert_eq!(
            error.to_string(),
            "Gleam package images requires native provider approval; run Geam interactively or select it explicitly with `geam embedding sync`"
        );
        assert_eq!(input.position(), 0);
        assert!(output.is_empty());

        let selected = TerminalApproval::for_embedding(true, &mut input, &mut output)
            .approve("images", &version, None, &candidates)
            .expect("interactive approval");
        assert_eq!(selected, candidates[0]);
        assert_eq!(
            String::from_utf8(output).expect("UTF-8 prompt"),
            concat!(
                "Gleam package images 1.0.0 requires native provider code.\n",
                "Metadata compatibility is not an endorsement.\n",
                "  1. geam-images 1.2.3 (Gleam >= 1.0.0)\n",
                "Approve geam-images 1.2.3? [y/N] ",
            )
        );
    }

    #[test]
    fn preserves_prompt_read_and_write_failures() {
        let candidates = [candidate("geam-images", "1.0.0", "images", ">= 1.0.0")];
        let mut input = std::io::BufReader::new(FailingReader);
        let mut output = Vec::<u8>::new();
        let mut read_failure = TerminalApproval::new(true, &mut input, &mut output);
        assert!(matches!(
            read_failure.approve(
                "images",
                &hexpm::version::Version::new(1, 0, 0),
                None,
                &candidates,
            ),
            Err(CliError::ProviderApprovalIo {
                operation: "read",
                error,
            }) if error.kind() == io::ErrorKind::Other
                && error.to_string() == "fixture read failure"
        ));

        let mut input = Cursor::new(b"yes\n");
        let mut output = FailingWriter;
        let mut write_failure = TerminalApproval::new(true, &mut input, &mut output);
        assert!(matches!(
            write_failure.approve(
                "images",
                &hexpm::version::Version::new(1, 0, 0),
                None,
                &candidates,
            ),
            Err(CliError::ProviderApprovalIo {
                operation: "write",
                error,
            }) if error.kind() == io::ErrorKind::Other
                && error.to_string() == "fixture write failure"
        ));
    }

    fn candidate(crate_name: &str, version: &str, package: &str, range: &str) -> ProviderCandidate {
        let source = format!(
            "name = \"{crate_name}\"\nversion = \"{version}\"\n\n[metadata.geam.provider]\nschema = 1\ngleam-package = \"{package}\"\ngleam-version = \"{range}\"\n"
        );
        let package = source
            .parse::<toml::Table>()
            .expect("candidate metadata should parse");
        ProviderCandidate::new(
            version.parse::<Version>().expect("version should parse"),
            ProviderMetadata::from_manifest(crate_name, &package)
                .expect("provider metadata should parse"),
        )
    }

    struct FailingReader;

    impl Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("fixture read failure"))
        }
    }

    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("fixture write failure"))
        }
    }
}
