use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

#[test]
fn prepares_packages_and_runs_without_the_original_gleam_project() {
    let directory = tempfile::tempdir().unwrap();
    let root = fs::canonicalize(directory.path()).unwrap();
    let application = root.join("application");
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    fs::write(application.join("Cargo.toml"), format!(
        "[package]\nname = 'prepared-consumer'\nversion = '0.1.0'\nedition = '2024'\nlicense = 'Apache-2.0'\ndescription = 'Prepared embedding acceptance'\ninclude = ['src/**', 'Cargo.toml', 'Cargo.lock']\n[dependencies]\ngeam = {{ version = '={}', default-features = false, features = ['embedding'] }}\nmiette = '7'\n[workspace]\n",
        env!("CARGO_PKG_VERSION"),
    )).unwrap();
    fs::write(application.join("src/main.rs"), "fn main() {}\n").unwrap();
    let repository_path = repository.to_str().unwrap();
    let target_path = target.to_str().unwrap();
    let config = toml::toml! {
        [patch.crates-io.geam]
        path = repository_path
        [net]
        offline = true
        [build]
        target-dir = target_path
    };
    fs::write(
        application.join(".cargo/config.toml"),
        toml::to_string(&config).unwrap(),
    )
    .unwrap();
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "init"]));
    assert_eq!(
        fs::read_to_string(application.join("src/main.rs")).unwrap(),
        "fn main() {}\n"
    );
    assert!(
        fs::read_to_string(application.join("src/geam_bindings.rs"))
            .unwrap()
            .contains("pub fn project(")
    );

    fs::write(application.join("gleam/src/prepared_consumer.gleam"),
        "import support\npub fn double(value: Int) -> Int { support.double(value) }\npub fn fail() -> Int { support.fail() }\n"
    ).unwrap();
    fs::write(application.join("gleam/src/support.gleam"),
        "pub fn double(value: Int) -> Int { value * 2 }\n\npub fn fail() -> Int {\n  echo 7\n  panic as \"prepared failure\"\n}\n"
    ).unwrap();

    for mode in ["both", "prepared"] {
        let path = application.join("Cargo.toml");
        let mut manifest: toml::Value =
            toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        manifest["package"].as_table_mut().unwrap().insert(
            "metadata".into(),
            toml::toml! {
                [geam.embedding]
                generate = mode
            }
            .into(),
        );
        fs::write(&path, toml::to_string(&manifest).unwrap()).unwrap();
        let dynamic = if mode == "both" {
            "let program = geam_bindings::project().compile()?;\n    let (bindings, functions) = geam_bindings::bind(geam::embedding::ModuleBuilder::from_program(program)?)?;\n    assert_eq!(bindings.seal().call(&functions.double, (21.into(),), &mut Vec::new())?, 42.into());\n"
        } else {
            ""
        };
        fs::write(application.join("src/main.rs"), format!(r#"mod geam_bindings;
use miette::Diagnostic;

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    {dynamic}
    let (module, functions) = geam_bindings::load()?;
    let mut echo = Vec::new();
    if std::env::args_os().nth(1).is_some() {{
        let error = module.call(&functions.fail, (), &mut echo).unwrap_err();
        println!("{{error}}");
        let geam::embedding::CallError::Execution(geam::ExecutionError::Panic(panic)) = error else {{
            panic!("expected the Gleam source panic");
        }};
        let span = panic.site().span();
        let context = panic.source_code().unwrap().read_span(&(span.start()..span.end()).into(), 0, 0)?;
        assert_eq!(std::str::from_utf8(context.data())?.trim(), "panic as \"prepared failure\"");
        println!("{{}}:{{}}", context.name().unwrap(), context.line() + 1);
        assert_eq!(echo.len(), 1);
        println!("{{}}", echo[0]);
    }} else {{
        println!("{{}}", module.call(&functions.double, (21.into(),), &mut echo)?);
        assert!(echo.is_empty());
    }}
    Ok(())
}}
"#)).unwrap();
        let started = Instant::now();
        checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
        eprintln!("{mode} preparation: {:?}", started.elapsed());
        let files = managed_files(&application);
        checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "check"]));
        assert_eq!(managed_files(&application), files);
        let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
        assert_eq!(output.stdout, b"42\n");
        assert_eq!(output.stderr, b"");
    }

    let inputs = managed_files(&application);
    let program = application.join("src/geam_bindings/program.rs");
    eprintln!(
        "prepared Rust data: {} bytes",
        fs::metadata(&program).unwrap().len()
    );
    fs::write(
        &program,
        [fs::read(&program).unwrap(), b"\n// stale\n".to_vec()].concat(),
    )
    .unwrap();
    let stale = managed_files(&application);
    let output = command(env!("CARGO_BIN_EXE_geam"), &application)
        .args(["embedding", "check"])
        .output()
        .unwrap();
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing or stale"));
    assert_eq!(managed_files(&application), stale);
    checked(command(env!("CARGO_BIN_EXE_geam"), &application).args(["embedding", "sync"]));
    assert_eq!(managed_files(&application), inputs);

    let deploy = root.join("deployment");
    fs::create_dir(&deploy).unwrap();
    let executable = binary_path(&deploy, "application");
    fs::copy(
        binary_path(&target.join("debug"), "prepared-consumer"),
        &executable,
    )
    .unwrap();
    fs::remove_dir_all(application.join("gleam")).unwrap();
    for argument in [None, Some("fail")] {
        let mut run = command(&executable, &deploy);
        run.env("PATH", "");
        if let Some(argument) = argument {
            run.arg(argument);
        }
        let output = checked(&mut run);
        assert_eq!(output.stderr, b"");
        assert_eq!(
            output.stdout,
            if argument.is_some() {
                b"panic: prepared failure\nprepared_consumer/src/support.gleam:5\nprepared_consumer/src/support.gleam:4\n7\n".as_slice()
            } else {
                b"42\n"
            }
        );
    }

    let tools = root.join("unavailable-tools");
    fs::create_dir(&tools).unwrap();
    fs::write(tools.join("unavailable.rs"), "fn main() { std::fs::write(std::env::var_os(\"UNEXPECTED_GEAM_COMMAND\").unwrap(), b\"invoked\").unwrap(); std::process::exit(91); }\n").unwrap();
    checked(
        command("rustc", &tools)
            .arg("unavailable.rs")
            .arg("-o")
            .arg(binary_path(&tools, "geam")),
    );
    fs::copy(binary_path(&tools, "geam"), binary_path(&tools, "gleam")).unwrap();
    let path = std::env::join_paths(
        std::iter::once(tools).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let unexpected = root.join("unexpected-tool-call");
    let started = Instant::now();
    checked(
        command("cargo", &application)
            .args(["package", "--locked", "--allow-dirty"])
            .env("PATH", &path)
            .env("UNEXPECTED_GEAM_COMMAND", &unexpected),
    );
    eprintln!(
        "source-free Cargo package and verification: {:?}",
        started.elapsed()
    );
    let extracted = target.join("package/prepared-consumer-0.1.0");
    assert!(!extracted.join("gleam").exists());
    assert!(!extracted.join("build.rs").exists());
    assert_eq!(
        fs::read(extracted.join("src/geam_bindings.rs")).unwrap(),
        fs::read(application.join("src/geam_bindings.rs")).unwrap()
    );
    assert_eq!(
        fs::read(extracted.join("src/geam_bindings/program.rs")).unwrap(),
        fs::read(&program).unwrap()
    );
    let relocated = root.join("extracted consumer");
    copy_directory(&extracted, &relocated);
    fs::remove_dir_all(&extracted).unwrap();
    fs::remove_dir_all(&application).unwrap();
    let started = Instant::now();
    let output = checked(
        command("cargo", &relocated)
            .args(["run", "--quiet", "--locked", "--offline", "--target-dir"])
            .arg(&target)
            .arg("--config")
            .arg(format!(
                "patch.crates-io.geam.path={}",
                toml::Value::String(repository.to_str().unwrap().to_owned())
            ))
            .env("PATH", &path)
            .env("UNEXPECTED_GEAM_COMMAND", &unexpected),
    );
    eprintln!(
        "relocated source-free Cargo build and run: {:?}",
        started.elapsed()
    );
    assert_eq!(output.stdout, b"42\n");
    assert_eq!(output.stderr, b"");
    assert!(!unexpected.exists());
}

fn command(program: impl AsRef<std::ffi::OsStr>, directory: &Path) -> Command {
    let mut command = Command::new(program);
    command
        .current_dir(directory)
        .env("CARGO_INCREMENTAL", "0")
        .env("CARGO_NET_OFFLINE", "true")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_LLVM_COV")
        .env_remove("CARGO_LLVM_COV_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .env_remove("RUSTDOCFLAGS");
    command
}

fn checked(command: &mut Command) -> Output {
    let output = command.output().unwrap();
    assert!(output.status.success(), "{command:?}: {output:?}");
    output
}

fn binary_path(directory: &Path, name: &str) -> PathBuf {
    directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_directory(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}

fn managed_files(root: &Path) -> [Vec<u8>; 6] {
    [
        "Cargo.toml",
        "Cargo.lock",
        "gleam/gleam.toml",
        "gleam/manifest.toml",
        "src/geam_bindings.rs",
        "src/geam_bindings/program.rs",
    ]
    .map(|path| fs::read(root.join(path)).unwrap())
}
