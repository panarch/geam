use super::manifest::{ProviderSelection, ProviderSource, read_provider_selections};
use crate::error::CliError;
use camino::Utf8Path;
use std::io::Write;

const GLEAM_HEADER: &str = "GLEAM PACKAGE";
const CRATE_HEADER: &str = "CARGO CRATE";

pub(super) fn write(project_root: &Utf8Path, writer: &mut dyn Write) -> Result<(), CliError> {
    let providers = read_provider_selections(project_root)?;
    let output = render(&providers);
    writer
        .write_all(output.as_bytes())
        .and_then(|()| writer.flush())
        .map_err(CliError::ProviderListIo)
}

fn render(providers: &[ProviderSelection]) -> String {
    if providers.is_empty() {
        return "No external providers are selected.\n".to_owned();
    }

    let gleam_width = providers
        .iter()
        .map(|provider| provider.gleam_package().len())
        .max()
        .unwrap_or(0)
        .max(GLEAM_HEADER.len());
    let crate_width = providers
        .iter()
        .map(|provider| provider.crate_name().len())
        .max()
        .unwrap_or(0)
        .max(CRATE_HEADER.len());
    let mut output =
        format!("{GLEAM_HEADER:<gleam_width$}  {CRATE_HEADER:<crate_width$}  SOURCE\n");
    for provider in providers {
        output.push_str(&format!(
            "{:<gleam_width$}  {:<crate_width$}  {}\n",
            provider.gleam_package(),
            provider.crate_name(),
            render_source(provider.source()),
        ));
    }
    output
}

fn render_source(source: &ProviderSource) -> String {
    match source {
        ProviderSource::Registry { version } => format!("crates.io {version}"),
        ProviderSource::Path { path } => format!("path {}", quoted(path.as_str())),
        ProviderSource::Git { url, rev } => match rev {
            Some(rev) => format!("git {} rev {}", quoted(url), quoted(rev)),
            None => format!("git {}", quoted(url)),
        },
    }
}

fn quoted(value: &str) -> String {
    format!("{value:?}")
}

#[cfg(test)]
mod tests {
    use super::write;
    use crate::error::CliError;
    use crate::provider::manifest::MANAGED_HEADER;
    use camino::Utf8PathBuf;
    use std::fs;
    use std::io::{self, Write};
    use tempfile::tempdir;

    #[test]
    fn lists_managed_sources_in_package_order_with_fixed_columns() {
        let project = tempdir().expect("temporary project should be created");
        let root = utf8_root(&project);
        let manifest = format!(
            r#"{MANAGED_HEADER}
[package.metadata.geam.runner]
schema = 1

[dependencies]
toml = "0.9"
geam_provider_search = {{ package = "geam-search", git = "https://example.com/search.git", rev = "abc123" }}
geam_provider_catalog = {{ package = "geam-catalog", path = '/workspace/catalog "draft"' }}
geam_provider_example_text_pattern = {{ package = "geam-example-text-pattern", version = "=0.1.0" }}
geam_provider_websocket = {{ package = "geam-websocket", git = "https://example.com/websocket.git" }}
"#,
        );
        fs::write(root.join("Cargo.toml"), &manifest).expect("managed manifest should be written");

        let mut output = Vec::new();
        write(&root, &mut output).expect("provider list should be written");

        assert_eq!(
            String::from_utf8(output).expect("provider list should be UTF-8"),
            concat!(
                "GLEAM PACKAGE         CARGO CRATE                SOURCE\n",
                "catalog               geam-catalog               path \"/workspace/catalog \\\"draft\\\"\"\n",
                "example_text_pattern  geam-example-text-pattern  crates.io 0.1.0\n",
                "search                geam-search                git \"https://example.com/search.git\" rev \"abc123\"\n",
                "websocket             geam-websocket             git \"https://example.com/websocket.git\"\n",
            ),
        );
        assert_eq!(
            fs::read_to_string(root.join("Cargo.toml"))
                .expect("managed manifest should remain readable"),
            manifest,
        );
    }

    #[test]
    fn reports_an_empty_selection_without_creating_a_manifest() {
        let project = tempdir().expect("temporary project should be created");
        let root = utf8_root(&project);
        let mut output = Vec::new();

        write(&root, &mut output).expect("empty provider list should be written");

        assert_eq!(output, b"No external providers are selected.\n");
        assert!(!root.join("Cargo.toml").exists());
    }

    #[test]
    fn preserves_managed_manifest_ownership_errors() {
        let project = tempdir().expect("temporary project should be created");
        let root = utf8_root(&project);
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"application\"\n",
        )
        .expect("user-owned manifest should be written");

        let error = write(&root, &mut Vec::new()).expect_err("user-owned manifest should fail");

        assert!(matches!(
            error,
            CliError::UserOwnedCargoManifest { path } if path == root.join("Cargo.toml")
        ));
    }

    #[test]
    fn preserves_write_and_flush_failures() {
        for (write_failure, expected_kind) in [
            (Some(io::ErrorKind::BrokenPipe), io::ErrorKind::BrokenPipe),
            (None, io::ErrorKind::WriteZero),
        ] {
            let project = tempdir().expect("temporary project should be created");
            let root = utf8_root(&project);
            let error = write(&root, &mut FailingWriter { write_failure })
                .expect_err("provider list output should fail");

            assert_eq!(error.to_string(), "failed to write provider list");
            assert!(matches!(
                error,
                CliError::ProviderListIo(error) if error.kind() == expected_kind
            ));
        }
    }

    struct FailingWriter {
        write_failure: Option<io::ErrorKind>,
    }

    impl Write for FailingWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            match self.write_failure {
                Some(kind) => Err(io::Error::from(kind)),
                None => Ok(buffer.len()),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::WriteZero))
        }
    }

    fn utf8_root(project: &tempfile::TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }
}
