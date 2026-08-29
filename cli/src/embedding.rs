mod boundary;
mod identifier;
mod output;
mod package;
mod render;

use crate::command::EmbeddingTarget;
use crate::error::CliError;
use boundary::PlainBindings;
use camino::Utf8Path;
use package::EmbeddingPackage;

pub(super) fn sync(current_directory: &Utf8Path, target: EmbeddingTarget) -> Result<(), CliError> {
    let package = EmbeddingPackage::load(current_directory, target.manifest_path)?;
    let program = geam_core::compile_typed_project(
        package.project_root().to_path_buf(),
        package.root_module(),
    )?;
    let requirements = geam_core::required_host_functions(&program);
    if !requirements.is_empty() {
        return Err(CliError::EmbeddingHostsRequired {
            module: package.root_module().to_owned(),
            requirements: requirements
                .iter()
                .map(|required| {
                    format!(
                        "{}/{}/{}",
                        required.package(),
                        required.module(),
                        required.function()
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        });
    }
    let bindings = PlainBindings::from_program(package.geam_alias().clone(), &program)?;
    let source = render::plain(&bindings);
    output::sync(
        package.output_directory(),
        package.output_path(),
        source.as_bytes(),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sync;
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
        let error = sync(
            &fixture.root,
            EmbeddingTarget {
                manifest_path: None,
            },
        )
        .expect_err("host-required project should fail plain synchronization");
        assert!(matches!(
            error,
            CliError::EmbeddingHostsRequired { module, requirements }
                if module == "inventory_rules"
                    && requirements == "embedding_application/inventory_rules/normalize"
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
name = "embedding-application"
version = "0.0.0"
edition = "2024"

[dependencies]
runtime = {{ package = "geam", path = {:?} }}

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
        let Output {
            status,
            stdout,
            stderr,
        } = command.output().expect("fixture command should start");
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        assert!(
            status.success(),
            "{operation} failed\nstdout:\n{stdout}\nstderr:\n{stderr}",
        );
    }
}
