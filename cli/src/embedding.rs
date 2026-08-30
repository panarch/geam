mod boundary;
mod identifier;
mod output;
mod package;
mod profile;
mod render;

use crate::command::EmbeddingTarget;
use crate::error::CliError;
use crate::project::{ResolvedProject, read_existing_resolved_project};
use boundary::PlainBindings;
use camino::Utf8Path;
use package::EmbeddingPackage;
use profile::HostedBindings;
use std::collections::BTreeSet;

struct GeneratedBindings {
    package: EmbeddingPackage,
    source: String,
}

pub(super) fn check(current_directory: &Utf8Path, target: EmbeddingTarget) -> Result<(), CliError> {
    check_with_project_reader(current_directory, target, read_existing_resolved_project)
}

pub(super) fn sync(current_directory: &Utf8Path, target: EmbeddingTarget) -> Result<(), CliError> {
    sync_with_project_reader(current_directory, target, read_existing_resolved_project)
}

fn check_with_project_reader(
    current_directory: &Utf8Path,
    target: EmbeddingTarget,
    read_project: fn(&Utf8Path) -> Result<ResolvedProject, CliError>,
) -> Result<(), CliError> {
    let generated = generate_with_project_reader(current_directory, target, read_project)?;
    output::check(
        generated.package.manifest(),
        generated.package.output_path(),
        generated.source.as_bytes(),
    )
}

fn sync_with_project_reader(
    current_directory: &Utf8Path,
    target: EmbeddingTarget,
    read_project: fn(&Utf8Path) -> Result<ResolvedProject, CliError>,
) -> Result<(), CliError> {
    let generated = generate_with_project_reader(current_directory, target, read_project)?;
    output::sync(
        generated.package.output_directory(),
        generated.package.output_path(),
        generated.source.as_bytes(),
    )?;
    Ok(())
}

fn generate_with_project_reader(
    current_directory: &Utf8Path,
    target: EmbeddingTarget,
    read_project: fn(&Utf8Path) -> Result<ResolvedProject, CliError>,
) -> Result<GeneratedBindings, CliError> {
    let package = EmbeddingPackage::load(current_directory, target.manifest_path)?;
    package.require_geam_feature("embedding", "to generate Rust embedding bindings")?;
    let program = geam_core::compile_typed_project(
        package.project_root().to_path_buf(),
        package.root_module(),
    )?;
    let requirements = geam_core::required_host_functions(&program);
    let bindings = PlainBindings::from_program(package.geam_alias().clone(), &program)?;
    let source = match requirements.as_slice() {
        [] => render::plain(&bindings),
        [first, remaining @ ..] => {
            let remaining_packages = remaining
                .iter()
                .map(|requirement| requirement.package().to_string())
                .collect::<BTreeSet<_>>();
            let resolved_project = read_project(package.project_root())?;
            let hosted = HostedBindings::resolve(
                &package,
                bindings,
                first.package(),
                &remaining_packages,
                &resolved_project,
            )?;
            render::hosted(&hosted)
        }
    };
    Ok(GeneratedBindings { package, source })
}

#[cfg(test)]
mod tests {
    use super::{check, sync, sync_with_project_reader};
    use crate::command::EmbeddingTarget;
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use std::fs;
    use std::process::{Command, Output};
    use tempfile::{TempDir, tempdir};

    #[test]
    fn synchronizes_formats_compiles_and_runs_plain_project_bindings() {
        let fixture = ApplicationFixture::new();
        fixture.write_plain_project();
        fixture.generate_lockfile();
        let lock_before =
            fs::read(fixture.root.join("Cargo.lock")).expect("fixture lockfile should be readable");

        sync(
            &fixture.root.join("src/nested"),
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("plain bindings should synchronize");
        check(
            &fixture.root.join("src/nested"),
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("exact plain bindings should pass checking from a nested directory");
        let generated_path = fixture.root.join("src/geam_bindings.rs");
        let generated = fs::read(&generated_path).expect("generated source should be readable");
        assert!(String::from_utf8_lossy(&generated).contains("use runtime::embedding::EcoString;"));
        assert!(
            String::from_utf8_lossy(&generated).contains("pub double: Function<(BigInt,), BigInt>")
        );
        assert_eq!(
            fs::read(fixture.root.join("Cargo.lock"))
                .expect("fixture lockfile should remain readable"),
            lock_before,
        );

        assert_success(
            Command::new("rustfmt").arg("--check").arg(&generated_path),
            "generated Rust formatting",
        );
        assert_success(
            fixture
                .cargo("run")
                .arg("--locked")
                .arg("--offline")
                .arg("--quiet"),
            "generated Rust application",
        );

        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: Some(fixture.root.join("Cargo.toml")),
            },
        )
        .expect("identical explicit synchronization should succeed");
        check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: Some(fixture.root.join("Cargo.toml")),
            },
        )
        .expect("exact explicit plain bindings should pass checking");
        assert_eq!(
            fs::read(&generated_path).expect("unchanged generated source should be readable"),
            generated,
        );

        let manifest_path = fixture.root.join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path)
            .expect("fixture manifest should be readable before metadata failure");
        fs::write(
            &manifest_path,
            manifest.replace("module = \"inventory_rules\"", "module = \"\""),
        )
        .expect("invalid embedding metadata should be written");
        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("invalid embedding metadata should fail synchronization");
        assert_eq!(
            fs::read(&generated_path).expect("metadata failure should preserve previous output"),
            generated,
        );
        fs::write(&manifest_path, manifest).expect("valid embedding metadata should be restored");

        fs::write(
            fixture.root.join("gleam/src/inventory_rules.gleam"),
            "pub fn invalid(",
        )
        .expect("invalid source fixture should be written");
        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("invalid source should fail synchronization");
        assert_eq!(
            fs::read(&generated_path).expect("previous output should remain readable"),
            generated,
        );

        fs::write(
            fixture.root.join("gleam/src/inventory_rules.gleam"),
            "pub fn unsupported(_value: List(Int)) -> Int { 1 }\n",
        )
        .expect("unsupported boundary fixture should be written");
        let error = sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("unsupported boundary should fail synchronization");
        assert!(
            error
                .to_string()
                .contains("invalid Rust embedding boundary module inventory_rules")
        );
        assert_eq!(
            fs::read(&generated_path).expect("boundary failure should preserve previous output"),
            generated,
        );

        fs::write(
            fixture.root.join("gleam/src/inventory_rules.gleam"),
            r#"
@external(erlang, "native", "normalize")
pub fn normalize(value: String) -> String
"#,
        )
        .expect("host-required source fixture should be written");
        let error = sync_with_project_reader(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
            |project_root| {
                Err(CliError::FileRead {
                    path: project_root.join("manifest.toml"),
                    error: std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "fixture resolution is unavailable",
                    ),
                })
            },
        )
        .expect_err("hosted synchronization should require an existing resolution");
        assert!(matches!(
            &error,
            CliError::FileRead { path, error }
                if path == &fixture.root.join("gleam/manifest.toml")
                    && error.kind() == std::io::ErrorKind::NotFound
        ));

        let error = sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("host-required project should require a direct provider");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingProvider { package, manifest, reason }
                if package == "embedding_application"
                    && manifest == fixture.root.join("Cargo.toml")
                    && reason.contains("no enabled direct provider dependency")
        ));
        assert_eq!(
            fs::read(&generated_path).expect("host failure should preserve previous output"),
            generated,
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::write(
                fixture.root.join("gleam/src/inventory_rules.gleam"),
                "pub fn changed() -> Int { 1 }\n",
            )
            .expect("changed source fixture should be written");
            let output_directory = fixture.root.join("src");
            fs::set_permissions(&output_directory, fs::Permissions::from_mode(0o500))
                .expect("output directory should become read-only");
            let result = sync(
                &fixture.root,
                EmbeddingTarget {
                    manifest_path: None,
                },
            );
            fs::set_permissions(&output_directory, fs::Permissions::from_mode(0o700))
                .expect("output directory permissions should be restored");

            let error = result.expect_err("read-only output directory should reject sync");
            assert!(matches!(
                error,
                CliError::FileWrite { path, error }
                    if path == generated_path
                        && error.kind() == std::io::ErrorKind::PermissionDenied
            ));
            assert_eq!(
                fs::read(&generated_path)
                    .expect("output failure should preserve previous generated source"),
                generated,
            );
        }
    }

    #[test]
    fn synchronizes_formats_lints_and_runs_source_backed_hosted_bindings() {
        let fixture = ApplicationFixture::new();
        fixture.write_hosted_project();
        fixture.generate_lockfile();

        sync(
            &fixture.root.join("src"),
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("hosted bindings should synchronize");
        check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("exact hosted bindings should pass checking");
        let generated_path = fixture.root.join("src/geam_bindings.rs");
        let generated = fs::read(&generated_path).expect("generated source should be readable");
        let source = String::from_utf8_lossy(&generated);
        assert!(source.contains("pub struct Profile;"));
        assert!(source.contains("patterns::Component"));
        assert!(source.contains("pub example_text_pattern: HostProviderConfiguration"));
        assert!(!source.contains("runtime::gleam_stdlib::Component"));
        assert!(!source.contains("runtime::gleam_json::Component"));
        assert!(!source.contains("runtime::gleam_time::Component"));
        assert!(!source.contains("unused_provider::Component"));

        assert_success(
            Command::new("rustfmt").arg("--check").arg(&generated_path),
            "hosted generated Rust formatting",
        );
        assert_success(
            fixture
                .cargo("clippy")
                .arg("--locked")
                .arg("--offline")
                .arg("--all-targets")
                .arg("--")
                .arg("-D")
                .arg("warnings"),
            "hosted generated Rust Clippy",
        );
        let output = success_output(
            fixture
                .cargo("run")
                .arg("--locked")
                .arg("--offline")
                .arg("--quiet"),
            "hosted generated Rust application",
        );
        assert_eq!(output.stdout, b"<Geam> + <Gleam> 2026\n");
        assert_eq!(output.stderr, b"");

        let manifest_path = fixture.root.join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path)
            .expect("hosted application manifest should be readable");
        let without_provider = manifest
            .lines()
            .filter(|line| !line.starts_with("patterns = "))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&manifest_path, format!("{without_provider}\n"))
            .expect("provider dependency should be removed");
        fixture.generate_lockfile();
        let checked_error = check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("missing direct provider should fail hosted checking");
        assert!(
            matches!(
                &checked_error,
                CliError::InvalidEmbeddingProvider { package, manifest: path, reason }
                    if package == "example_text_pattern"
                        && path == &manifest_path
                        && reason.contains("no enabled direct provider dependency")
            ),
            "unexpected provider check error: {checked_error:?}"
        );
        assert_eq!(
            fs::read(&generated_path)
                .expect("provider check failure should preserve previous output"),
            generated,
        );
        let error = sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("missing direct provider should fail hosted synchronization");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingProvider { package, manifest: path, reason }
                if package == "example_text_pattern"
                    && path == manifest_path
                    && reason.contains("no enabled direct provider dependency")
        ));
        assert_eq!(
            fs::read(&generated_path)
                .expect("provider graph failure should preserve previous output"),
            generated,
        );
    }

    #[test]
    fn synchronizes_and_runs_json_time_with_caller_owned_capabilities() {
        let fixture = ApplicationFixture::new();
        fixture.write_built_in_project();
        fixture.generate_lockfile();

        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("built-in hosted bindings should synchronize");
        check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("exact built-in bindings should pass checking");
        let generated_path = fixture.root.join("src/geam_bindings.rs");
        let generated = fs::read_to_string(&generated_path)
            .expect("built-in generated source should be readable");
        assert!(generated.contains("pub struct Profile<Io, Source>"));
        assert!(generated.contains("runtime::gleam_stdlib::Component<Io>"));
        assert!(generated.contains("runtime::gleam_json::Component"));
        assert!(generated.contains("runtime::gleam_time::Component<Source>"));
        assert!(generated.contains("            json: (),"));
        assert!(generated.contains("            time,"));
        assert!(!generated.contains("HostProviderComponentInitialization"));

        assert_success(
            Command::new("rustfmt").arg("--check").arg(&generated_path),
            "built-in generated Rust formatting",
        );
        assert_success(
            fixture
                .cargo("clippy")
                .arg("--locked")
                .arg("--offline")
                .arg("--all-targets")
                .arg("--")
                .arg("-D")
                .arg("warnings"),
            "built-in generated Rust Clippy",
        );
        assert_success(
            fixture
                .cargo("run")
                .arg("--locked")
                .arg("--offline")
                .arg("--quiet"),
            "built-in generated Rust application",
        );
    }

    #[test]
    fn checks_missing_stale_and_plain_boundary_drift_without_writing() {
        let fixture = ApplicationFixture::new();
        fixture.write_plain_project();
        fixture.generate_lockfile();
        let manifest_path = fixture.root.join("Cargo.toml");
        let generated_path = fixture.root.join("src/geam_bindings.rs");

        let missing = check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("missing generated bindings should fail checking");
        assert_eq!(
            missing.to_string(),
            format!(
                "Rust embedding bindings at {generated_path} are missing or stale for {manifest_path}; run `geam embedding sync --manifest-path {manifest_path}`"
            ),
        );
        assert!(!generated_path.exists());

        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("plain fixture should synchronize");
        let original = fs::read(&generated_path).expect("generated source should be readable");

        let support_path = fixture.root.join("gleam/src/support.gleam");
        let support = fs::read_to_string(&support_path).expect("support source should be readable");
        fs::write(
            &support_path,
            format!(
                "{}\nfn private_helper() -> Nil {{ Nil }}\n",
                support.replace("value * 2", "value + value"),
            ),
        )
        .expect("body-only and private changes should be written");
        check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("body-only and private changes should not alter bindings");
        assert_eq!(
            fs::read(&generated_path).expect("clean generated source should remain readable"),
            original,
        );

        let boundary_path = fixture.root.join("gleam/src/inventory_rules.gleam");
        let boundary =
            fs::read_to_string(&boundary_path).expect("boundary source should be readable");
        fs::write(
            &boundary_path,
            format!("{boundary}\npub fn added() -> Int {{ 1 }}\n"),
        )
        .expect("public addition should be written");
        assert!(matches!(
            check(
                &fixture.root,
                EmbeddingTarget {
                    manifest_path: None,
                },
            ),
            Err(CliError::EmbeddingBindingsOutOfDate { manifest, output })
                if manifest == manifest_path && output == generated_path
        ));
        assert_eq!(
            fs::read(&generated_path).expect("stale source should remain readable"),
            original,
        );

        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("public addition should synchronize");
        let with_addition =
            fs::read(&generated_path).expect("updated generated source should be readable");
        fs::write(&boundary_path, &boundary).expect("public addition should be removed");
        assert!(matches!(
            check(
                &fixture.root,
                EmbeddingTarget {
                    manifest_path: None,
                },
            ),
            Err(CliError::EmbeddingBindingsOutOfDate { manifest, output })
                if manifest == manifest_path && output == generated_path
        ));
        assert_eq!(
            fs::read(&generated_path).expect("removed-boundary output should remain readable"),
            with_addition,
        );

        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("restored boundary should synchronize");
        let restored = fs::read(&generated_path).expect("restored output should be readable");
        fs::write(
            &boundary_path,
            boundary.replace(
                "pub fn bindings() -> Int { 7 }",
                "pub fn bindings() -> Bool { True }",
            ),
        )
        .expect("signature change should be written");
        assert!(matches!(
            check(
                &fixture.root,
                EmbeddingTarget {
                    manifest_path: None,
                },
            ),
            Err(CliError::EmbeddingBindingsOutOfDate { manifest, output })
                if manifest == manifest_path && output == generated_path
        ));
        assert_eq!(
            fs::read(&generated_path).expect("signature-drift output should remain readable"),
            restored,
        );

        fs::write(&boundary_path, "pub fn invalid(")
            .expect("invalid boundary source should be written");
        check(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("invalid source should fail checking");
        assert_eq!(
            fs::read(&generated_path).expect("invalid-source output should remain readable"),
            restored,
        );
    }

    #[test]
    fn detects_host_requirement_drift_without_using_unused_dependencies() {
        let fixture = ApplicationFixture::new();
        fixture.write_hosted_project();
        fixture.generate_lockfile();
        sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect("hosted fixture should synchronize");
        let generated_path = fixture.root.join("src/geam_bindings.rs");
        let generated = fs::read(&generated_path).expect("hosted output should be readable");
        let manifest_path = fixture.root.join("Cargo.toml");

        fs::write(
            fixture.root.join("gleam/src/rust_embedding.gleam"),
            r#"pub fn format_words() -> String { "plain" }

pub fn contains_only_words(_text: String) -> Bool { True }
"#,
        )
        .expect("plain replacement boundary should be written");
        assert!(matches!(
            check(
                &fixture.root,
                EmbeddingTarget {
                    manifest_path: None,
                },
            ),
            Err(CliError::EmbeddingBindingsOutOfDate { manifest, output })
                if manifest == manifest_path && output == generated_path
        ));
        assert_eq!(
            fs::read(&generated_path).expect("host-requirement output should remain readable"),
            generated,
        );
    }

    #[test]
    fn rejects_missing_embedding_and_builtin_features_before_writing_bindings() {
        let plain = ApplicationFixture::new();
        plain.write_plain_project();
        let manifest_path = plain.root.join("Cargo.toml");
        let manifest =
            fs::read_to_string(&manifest_path).expect("plain fixture manifest should be readable");
        fs::write(
            &manifest_path,
            manifest.replace("features = [\"embedding\"]", "features = []"),
        )
        .expect("plain fixture should omit the embedding feature");
        plain.generate_lockfile();

        let error = sync(
            &plain.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("missing embedding feature should fail synchronization");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingDependency { package, manifest, reason }
                if package == "plain-embedding-application"
                    && manifest == manifest_path
                    && reason.contains("Geam feature `embedding`")
                    && reason.contains("direct Geam dependency")
        ));
        assert!(!plain.root.join("src/geam_bindings.rs").exists());

        let hosted = ApplicationFixture::new();
        hosted.write_built_in_project();
        let manifest_path = hosted.root.join("Cargo.toml");
        let manifest =
            fs::read_to_string(&manifest_path).expect("hosted fixture manifest should be readable");
        fs::write(&manifest_path, manifest.replace(", \"gleam-time\"", ""))
            .expect("hosted fixture should omit the required Time feature");
        hosted.generate_lockfile();

        let error = sync(
            &hosted.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("missing built-in feature should fail synchronization");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingDependency { package, manifest, reason }
                if package == "built-in-embedding-application"
                    && manifest == manifest_path
                    && reason.contains("Geam feature `gleam-time`")
                    && reason.contains("Gleam package `gleam_time`")
        ));
        assert!(!hosted.root.join("src/geam_bindings.rs").exists());
    }

    struct ApplicationFixture {
        _directory: TempDir,
        root: Utf8PathBuf,
        repository: Utf8PathBuf,
        target: Utf8PathBuf,
    }

    impl ApplicationFixture {
        fn new() -> Self {
            let directory = tempdir().expect("temporary directory should be created");
            let root = Utf8PathBuf::from_path_buf(
                fs::canonicalize(directory.path()).expect("temporary path should canonicalize"),
            )
            .expect("temporary path should be valid UTF-8");
            let repository = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .map(camino::Utf8Path::to_path_buf)
                .expect("CLI package should be inside the repository");
            let target = repository.join("target/embedding-sync-acceptance");
            Self {
                _directory: directory,
                root,
                repository,
                target,
            }
        }

        fn write_plain_project(&self) {
            fs::create_dir_all(self.root.join("src/nested"))
                .expect("Rust source directory should be created");
            fs::create_dir_all(self.root.join("gleam/src"))
                .expect("Gleam source directory should be created");
            fs::write(
                self.root.join("Cargo.toml"),
                format!(
                    r#"[package]
name = "plain-embedding-application"
version = "0.0.0"
edition = "2024"

[dependencies]
runtime = {{ package = "geam", path = {:?}, default-features = false, features = ["embedding"] }}

[package.metadata.geam.embedding]
project = "gleam"
module = "inventory_rules"

[workspace]
resolver = "3"
"#,
                    self.repository
                ),
            )
            .expect("Rust manifest should be written");
            fs::write(
                self.root.join("src/main.rs"),
                r#"mod geam_bindings;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program = runtime::compile_typed_project(
        concat!(env!("CARGO_MANIFEST_DIR"), "/gleam"),
        geam_bindings::ROOT_MODULE,
    )?;
    let builder = runtime::embedding::ModuleBuilder::from_program(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal();
    let mut echo = Vec::new();

    let first = module.call(&functions.normalize, ("AB-12".into(),), &mut echo)?;
    let second = module.call(&functions.normalize, ("C-4".into(),), &mut echo)?;
    let doubled = module.call(&functions.double, (21.into(),), &mut echo)?;
    let binding_name = module.call(&functions.bindings, (), &mut echo)?;
    let float = module.call(&functions.keep_float, (1.25,), &mut echo)?;
    let string = module.call(&functions.keep_string, ("value".into(),), &mut echo)?;
    let bits = runtime::embedding::BitArrayValue::try_from_parts(vec![0b1010_0000], 3)?;
    let returned_bits = module.call(&functions.keep_bits, (bits.clone(),), &mut echo)?;
    let codepoint = module.call(&functions.keep_codepoint, ('a',), &mut echo)?;
    let boolean = module.call(&functions.keep_bool, (true,), &mut echo)?;
    module.call(&functions.keep_nil, ((),), &mut echo)?;
    let mixed = module.call(
        &functions.mixed,
        (
            1.into(),
            2.5,
            "mixed".into(),
            bits.clone(),
            'b',
            true,
            (),
        ),
        &mut echo,
    )?;

    assert_eq!(first, "SKU:AB-12");
    assert_eq!(second, "SKU:C-4");
    assert_eq!(doubled, runtime::embedding::BigInt::from(42));
    assert_eq!(binding_name, runtime::embedding::BigInt::from(7));
    assert_eq!(float, 1.25);
    assert_eq!(string, "value");
    assert_eq!(returned_bits, bits);
    assert_eq!(codepoint, 'a');
    assert!(boolean);
    assert!(mixed);
    assert!(echo.is_empty());
    Ok(())
}
"#,
            )
            .expect("Rust application should be written");
            fs::write(
                self.root.join("gleam/gleam.toml"),
                "name = \"embedding_application\"\nversion = \"1.0.0\"\n",
            )
            .expect("Gleam package config should be written");
            fs::write(
                self.root.join("gleam/manifest.toml"),
                "packages = []\n\n[requirements]\n",
            )
            .expect("Gleam manifest should be written");
            fs::write(
                self.root.join("gleam/src/inventory_rules.gleam"),
                r#"import support

pub fn normalize(value: String) -> String {
  support.label(value)
}

pub fn double(value: Int) -> Int {
  support.double(value)
}

pub fn bindings() -> Int { 7 }

pub fn keep_float(value: Float) -> Float { value }
pub fn keep_string(value: String) -> String { value }
pub fn keep_bits(value: BitArray) -> BitArray { value }
pub fn keep_codepoint(value: UtfCodepoint) -> UtfCodepoint { value }
pub fn keep_bool(value: Bool) -> Bool { value }
pub fn keep_nil(value: Nil) -> Nil { value }

pub fn mixed(
  _int: Int,
  _float: Float,
  _string: String,
  _bits: BitArray,
  _codepoint: UtfCodepoint,
  value: Bool,
  _nil: Nil,
) -> Bool {
  value
}
"#,
            )
            .expect("Gleam boundary source should be written");
            fs::write(
                self.root.join("gleam/src/support.gleam"),
                r#"pub fn label(value: String) -> String {
  "SKU:" <> value
}

pub fn double(value: Int) -> Int {
  value * 2
}
"#,
            )
            .expect("imported Gleam source should be written");
        }

        fn write_hosted_project(&self) {
            fs::create_dir_all(self.root.join("src"))
                .expect("Rust source directory should be created");
            fs::create_dir_all(self.root.join("gleam/src"))
                .expect("Gleam source directory should be created");
            let provider = self.repository.join("examples/text_pattern/provider");
            let text_pattern = self
                .repository
                .join("examples/text_pattern/project/packages/example_text_pattern");
            fs::write(
                self.root.join("Cargo.toml"),
                format!(
                    r#"[package]
name = "hosted-embedding-application"
version = "0.0.0"
edition = "2024"

[dependencies]
runtime = {{ package = "geam", path = {:?}, default-features = false, features = ["embedding"] }}
patterns = {{ package = "geam-example-text-pattern", path = {provider:?} }}

[package.metadata.geam.embedding]
project = "gleam"
module = "rust_embedding"

[patch.crates-io]
geam = {{ path = {:?} }}

[workspace]
resolver = "3"
"#,
                    self.repository, self.repository,
                ),
            )
            .expect("hosted Rust manifest should be written");
            fs::write(
                self.root.join("src/main.rs"),
                r#"mod geam_bindings;

use runtime::embedding::HostedModuleBuilder;
use runtime::{HostProviderConfiguration, compile_typed_host_project};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program = compile_typed_host_project(
        concat!(env!("CARGO_MANIFEST_DIR"), "/gleam"),
        geam_bindings::ROOT_MODULE,
        geam_bindings::host_providers()?,
    )?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;
    let mut state = geam_bindings::RunState::initialize(
        geam_bindings::ProviderConfigurations {
            example_text_pattern: HostProviderConfiguration::empty(),
        },
    )?;
    let mut echo = Vec::new();

    let value = module.call(&functions.format_words, (), &mut state, &mut echo)?;
    let words = module.call(
        &functions.contains_only_words,
        ("Geam and Gleam".into(),),
        &mut state,
        &mut echo,
    )?;
    let numbers = module.call(
        &functions.contains_only_words,
        ("Geam 2026".into(),),
        &mut state,
        &mut echo,
    )?;

    assert_eq!(value, "<Geam> + <Gleam> 2026");
    assert!(words);
    assert!(!numbers);
    assert!(echo.is_empty());
    println!("{value}");
    Ok(())
}
"#,
            )
            .expect("hosted Rust application should be written");
            fs::write(
                self.root.join("gleam/gleam.toml"),
                format!(
                    r#"name = "embedding_application"
version = "1.0.0"

[dependencies]
example_text_pattern = {{ path = {text_pattern:?} }}
"#,
                ),
            )
            .expect("hosted Gleam config should be written");
            fs::write(
                self.root.join("gleam/manifest.toml"),
                format!(
                    r#"packages = [
  {{ name = "example_text_pattern", version = "0.1.0", build_tools = ["gleam"], requirements = [], source = "local", path = {text_pattern:?} }},
]

[requirements]
example_text_pattern = {{ path = {text_pattern:?} }}
"#,
                ),
            )
            .expect("hosted Gleam manifest should be written");
            fs::write(
                self.root.join("gleam/src/rust_embedding.gleam"),
                r#"import example_text_pattern as pattern

pub fn format_words() -> String {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  pattern.replace_all(words, "Geam + Gleam 2026", "<$0>")
}

pub fn contains_only_words(text: String) -> Bool {
  let assert Ok(words) = pattern.compile("^[A-Za-z ]+$")
  pattern.is_match(words, text)
}
"#,
            )
            .expect("hosted Gleam boundary should be written");
        }

        fn write_built_in_project(&self) {
            fs::create_dir_all(self.root.join("src"))
                .expect("Rust source directory should be created");
            fs::create_dir_all(self.root.join("gleam/src"))
                .expect("Gleam source directory should be created");
            for (package, module) in [("gleam_json", "json_native"), ("gleam_time", "time_native")]
            {
                let package_root = self.root.join("gleam/packages").join(package);
                fs::create_dir_all(package_root.join("src"))
                    .expect("built-in source package should be created");
                fs::write(
                    package_root.join("gleam.toml"),
                    format!("name = {package:?}\nversion = \"1.0.0\"\n"),
                )
                .expect("built-in package config should be written");
                fs::write(
                    package_root.join("src").join(format!("{module}.gleam")),
                    "@external(erlang, \"native\", \"touch\")\npub fn touch() -> Nil\n",
                )
                .expect("built-in package source should be written");
            }
            fs::write(
                self.root.join("Cargo.toml"),
                format!(
                    r#"[package]
name = "built-in-embedding-application"
version = "0.0.0"
edition = "2024"

[dependencies]
runtime = {{ package = "geam", path = {:?}, default-features = false, features = ["embedding", "gleam-json", "gleam-time"] }}

[package.metadata.geam.embedding]
project = "gleam"
module = "rust_embedding"

[workspace]
resolver = "3"
"#,
                    self.repository,
                ),
            )
            .expect("built-in Rust manifest should be written");
            fs::write(
                self.root.join("src/main.rs"),
                r#"mod geam_bindings;

use runtime::gleam_stdlib::{GleamStdlibRunState, IoOutput};
use runtime::gleam_time::TimeSource;
use runtime::HostFailure;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

struct FixedTime;

impl TimeSource for FixedTime {
    fn system_time(&mut self) -> Result<SystemTime, HostFailure> {
        Ok(UNIX_EPOCH)
    }

    fn local_offset_seconds(&mut self) -> Result<i32, HostFailure> {
        Ok(0)
    }
}

fn consume_functions(functions: geam_bindings::Functions) {
    let _ = functions.ready;
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(geam_bindings::ROOT_MODULE, "rust_embedding");
    let _consume = consume_functions;
    let _bind = geam_bindings::bind::<Vec<IoOutput>, FixedTime>;
    let _providers = geam_bindings::host_providers::<Vec<IoOutput>, FixedTime>()?;
    let mut state = geam_bindings::RunState::initialize(
        GleamStdlibRunState::from_seed([7; 32]),
        FixedTime,
        geam_bindings::ProviderConfigurations,
    );
    assert!(state.stdlib().io_outputs().is_empty());
    assert!(state.stdlib_mut().take_io_outputs().is_empty());
    Ok(())
}
"#,
            )
            .expect("built-in Rust application should be written");
            fs::write(
                self.root.join("gleam/gleam.toml"),
                r#"name = "embedding_application"
version = "1.0.0"

[dependencies]
gleam_json = { path = "packages/gleam_json" }
gleam_time = { path = "packages/gleam_time" }
"#,
            )
            .expect("built-in Gleam config should be written");
            fs::write(
                self.root.join("gleam/manifest.toml"),
                r#"packages = [
  { name = "gleam_json", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/gleam_json" },
  { name = "gleam_time", version = "1.0.0", build_tools = ["gleam"], requirements = [], source = "local", path = "packages/gleam_time" },
]

[requirements]
gleam_json = { path = "packages/gleam_json" }
gleam_time = { path = "packages/gleam_time" }
"#,
            )
            .expect("built-in Gleam manifest should be written");
            fs::write(
                self.root.join("gleam/src/rust_embedding.gleam"),
                r#"import json_native
import time_native

pub fn ready() -> Bool {
  json_native.touch()
  time_native.touch()
  True
}
"#,
            )
            .expect("built-in Gleam boundary should be written");
        }

        fn generate_lockfile(&self) {
            assert_success(
                self.cargo("generate-lockfile").arg("--offline"),
                "fixture lockfile generation",
            );
        }

        fn cargo(&self, command: &str) -> Command {
            let mut cargo = Command::new("cargo");
            cargo
                .arg(command)
                .arg("--manifest-path")
                .arg(self.root.join("Cargo.toml"))
                .env("CARGO_TARGET_DIR", &self.target)
                .current_dir(&self.root);
            for variable in [
                "CARGO_ENCODED_RUSTFLAGS",
                "LLVM_PROFILE_FILE",
                "RUSTDOCFLAGS",
                "RUSTFLAGS",
            ] {
                cargo.env_remove(variable);
            }
            cargo
        }
    }

    fn assert_success(command: &mut Command, operation: &str) {
        success_output(command, operation);
    }

    fn success_output(command: &mut Command, operation: &str) -> Output {
        let output = command.output().expect("fixture command should start");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "{operation} failed\nstdout:\n{stdout}\nstderr:\n{stderr}",
        );
        output
    }
}
