use super::{checked, command};
use geam::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
use std::fs;
use std::path::Path;

#[test]
fn generated_checkpoints_resume_long_blocks_without_growing_the_rust_stack() {
    // A long straight block exposes recursive helper chaining after a yield.
    // Generate the artifact in the consumer directory instead of committing
    // thousands of mechanically emitted lines for this regression.
    let inversions = "  let flag = !flag\n".repeat(500);
    let source = format!(
        r#"
pub fn choose(flag: Bool) -> Int {{
{inversions}  case flag {{
    True -> 7
    False -> -7
  }}
}}
"#,
    );
    let typed = geam::compile_typed_module("example", "src/example.gleam", &source).unwrap();
    let (bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(bool,), BigInt>::new("choose"))
        .unwrap();
    let artifact = bindings.prepare().emit_rust();
    let directory = tempfile::tempdir().unwrap();
    let application = fs::canonicalize(directory.path()).unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repository.join("target/prepared-acceptance");
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    fs::write(
        application.join("Cargo.toml"),
        format!(
            r#"
[package]
name = 'numeric-checkpoint-consumer'
version = '0.1.0'
edition = '2024'

[dependencies]
geam = {{ version = '={}', default-features = false, features = ['embedding'] }}

[workspace]
"#,
            env!("CARGO_PKG_VERSION"),
        ),
    )
    .unwrap();
    let repository_path = repository.to_str().unwrap();
    let target_path = target.to_str().unwrap();
    fs::write(
        application.join(".cargo/config.toml"),
        toml::to_string(&toml::toml! {
            [patch.crates-io.geam]
            path = repository_path
            [net]
            offline = true
            [build]
            target-dir = target_path
        })
        .unwrap(),
    )
    .unwrap();
    fs::write(application.join("src/source.gleam"), source).unwrap();
    fs::write(application.join("src/program.rs"), format!("{artifact}\n")).unwrap();
    fs::write(
        application.join("src/main.rs"),
        r#"
use data::compiled::{CompiledImplementation, CompiledProgress};
use data::compiled::numeric::NumericValues;
use geam::__prepared_support as data;
use geam::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
use std::convert::Infallible;

static PROGRAM: data::ModuleArtifact<Infallible> = include!("program.rs");

fn main() {
    let typed =
        geam::compile_typed_module("example", "src/example.gleam", include_str!("source.gleam"))
            .unwrap();
    let (bindings, function) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(bool,), BigInt>::new("choose"))
        .unwrap();
    let dynamic = bindings.seal();
    let mut bindings = PROGRAM.load().unwrap();
    let prepared_function = bindings
        .function(FunctionDeclaration::<(bool,), BigInt>::new("choose"))
        .unwrap();
    let prepared = bindings.seal();
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            assert_eq!(
                dynamic.call(&function, (true,), &mut Vec::new()).unwrap(),
                BigInt::from(7)
            );
            assert_eq!(
                prepared
                    .call(&prepared_function, (true,), &mut Vec::new())
                    .unwrap(),
                BigInt::from(7)
            );
            let implementation = &PROGRAM.program.compiled.ints[0].implementation;
            let CompiledImplementation::Numeric(implementation) = implementation else {
                panic!("Boolean fixture must select the scalar kernel");
            };
            assert_eq!(implementation.checkpoints.len(), 504);
            for allowance in [1, 1024] {
                for (flag, expected) in [(true, 7), (false, -7)] {
                    let mut values = NumericValues {
                        ints: vec![],
                        bools: vec![flag],
                    };
                    let mut budget = 1;
                    let first = (implementation.run)(implementation.entry, &mut values, &mut budget);
                    assert_eq!(first, CompiledProgress::Yield(1));
                    assert_eq!(budget, 0);
                    assert_eq!(values.bools, [flag, !flag]);
                    let mut point = 1;
                    let mut steps = 1;
                    loop {
                        let mut budget = allowance;
                        let progress = (implementation.run)(point, &mut values, &mut budget);
                        steps += allowance - budget;
                        assert!(steps <= 502);
                        match progress {
                            CompiledProgress::Yield(next) => {
                                assert_eq!(budget, 0);
                                let checkpoint = implementation.checkpoints[next];
                                assert_eq!(values.ints.len(), checkpoint.ints);
                                assert_eq!(values.bools.len(), checkpoint.bools);
                                assert_eq!(checkpoint.bit_arrays, 0);
                                point = next;
                            }
                            CompiledProgress::Complete(_) => {
                                assert_eq!(values.ints, [expected]);
                                assert!(values.bools.is_empty());
                                // The last Not is folded into the condition:
                                // 499 instructions, branch, literal, return.
                                assert_eq!(steps, 502);
                                break;
                            }
                            CompiledProgress::Interpreted(_) => {
                                panic!("Boolean fixture stays generated")
                            }
                        }
                    }
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
    println!("long checkpoint execution completed");
}
"#,
    )
    .unwrap();
    checked(command("cargo", &application).args(["generate-lockfile", "--offline"]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, b"long checkpoint execution completed\n");
    assert!(output.stderr.is_empty());
}
