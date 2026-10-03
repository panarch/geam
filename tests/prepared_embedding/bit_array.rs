use super::{checked, command};
use geam::embedding::{BigInt, BitArrayValue, FunctionDeclaration, ModuleBuilder};
use std::fs;
use std::path::Path;

#[test]
fn generated_bit_array_kernels_compile_strictly_and_resume_source_visible_boundaries() {
    let inversions = "  let flag = !flag\n".repeat(500);
    let source = format!(
        r#"
pub fn sum(input: BitArray, total: Int) -> Int {{
  case input {{
    <<value:8, rest:bits>> -> sum(rest, total + value)
    <<>> -> total
    _ -> panic as "short input"
  }}
}}

pub fn choose(input: BitArray, flag: Bool) -> Int {{
{inversions}  let assert <<value:8>> = input
  case flag {{ True -> value False -> 0 - value }}
}}

pub fn wide(input: BitArray) -> Int {{
  let assert <<value:64>> = input
  value
}}

pub fn parse(input: BitArray, total: Int) -> Result(Int, Nil) {{
  case input {{
    <<value:8, rest:bits>> if value < 10 -> parse(rest, total + value)
    <<99, rest:bits>> -> parse(rest, total)
    <<>> -> Ok(total)
    _ -> Error(Nil)
  }}
}}
"#
    );
    let typed = geam::compile_typed_module("example", "src/example.gleam", &source).unwrap();
    let (mut bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BitArrayValue, BigInt), BigInt>::new(
            "sum",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BitArrayValue, bool), BigInt>::new(
            "choose",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new("wide"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (BitArrayValue, BigInt),
            Result<BigInt, ()>,
        >::new("parse"))
        .unwrap();
    let artifact = bindings.prepare().emit_rust();
    let directory = tempfile::tempdir().unwrap();
    let application = fs::canonicalize(directory.path()).unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(application.join("src")).unwrap();
    fs::create_dir_all(application.join(".cargo")).unwrap();
    fs::write(
        application.join("Cargo.toml"),
        format!(
            r#"
[package]
name = 'bit-array-checkpoint-consumer'
version = '0.1.0'
edition = '2024'

[dependencies]
geam = {{ version = '={}', default-features = false, features = ['embedding'] }}

[workspace]
"#,
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    let repository_path = repository.to_str().unwrap();
    let target = repository.join("target/prepared-acceptance");
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
    fs::write(application.join("src/program.rs"), format!("{artifact}\n")).unwrap();
    fs::write(
        application.join("src/main.rs"),
        r#"
#![deny(warnings)]
use data::compiled::bit_array::{BitArrayKernel, BitArrayValues};
use data::compiled::{
    BitArrayImplementation, CompiledFunction, CompiledImplementation, CompiledProgress,
};
use geam::__prepared_support as data;
use geam::embedding::{BigInt, BitArrayValue, CallError, FunctionDeclaration};
use geam::{ExecutionError, PanicKind};
use std::convert::Infallible;
use std::sync::Mutex;

const ARTIFACT: data::ModuleArtifact<Infallible> = include!("program.rs");
static ORIGINAL: data::ModuleArtifact<Infallible> = ARTIFACT;
static VISITED: Mutex<Vec<Vec<bool>>> = Mutex::new(Vec::new());

// Observe the actual emitted function. One canonical step per batch forces its
// cold resume boundaries; this observer does not implement the source graph.
fn checked_kernel(
    index: usize,
    point: usize,
    values: &mut BitArrayValues,
    budget: &mut usize,
) -> CompiledProgress {
    let implementation = if index < 3 {
        &ORIGINAL.program.compiled.ints[index].implementation
    } else {
        &ORIGINAL.program.compiled.customs[0].implementation
    };
    let CompiledImplementation::BitArray(kernel) = implementation else {
        panic!("BitArray source must select its generated kernel");
    };
    let mut visited = VISITED.lock().unwrap();
    if visited.is_empty() {
        *visited = ORIGINAL
            .program
            .compiled
            .ints
            .iter()
            .map(|target| &target.implementation)
            .chain(
                ORIGINAL
                    .program
                    .compiled
                    .customs
                    .iter()
                    .map(|target| &target.implementation),
            )
            .map(|implementation| {
                let CompiledImplementation::BitArray(kernel) = implementation else {
                    panic!("BitArray source must select its generated kernel");
                };
                vec![false; kernel.checkpoints.len()]
            })
            .collect();
    }
    visited[index][point] = true;
    let prefix = kernel.checkpoints[point];
    assert_eq!(
        (
            values.ints.len(),
            values.bools.len(),
            values.bit_arrays.len()
        ),
        (prefix.ints, prefix.bools, prefix.bit_arrays),
    );
    let allowance = (*budget).min(1);
    let mut remaining = allowance;
    let progress = (kernel.run)(point, values, &mut remaining);
    *budget -= allowance - remaining;
    if let CompiledProgress::Yield(next) | CompiledProgress::Interpreted(next) = progress {
        visited[index][next] = true;
        let prefix = kernel.checkpoints[next];
        assert_eq!(
            (
                values.ints.len(),
                values.bools.len(),
                values.bit_arrays.len()
            ),
            (prefix.ints, prefix.bools, prefix.bit_arrays),
        );
        if matches!(progress, CompiledProgress::Yield(_)) {
            assert_eq!(remaining, 0);
        }
    }
    progress
}

fn sum_kernel(point: usize, values: &mut BitArrayValues, budget: &mut usize) -> CompiledProgress {
    checked_kernel(0, point, values, budget)
}

fn choose_kernel(
    point: usize,
    values: &mut BitArrayValues,
    budget: &mut usize,
) -> CompiledProgress {
    checked_kernel(1, point, values, budget)
}

fn wide_kernel(point: usize, values: &mut BitArrayValues, budget: &mut usize) -> CompiledProgress {
    checked_kernel(2, point, values, budget)
}

fn parse_kernel(point: usize, values: &mut BitArrayValues, budget: &mut usize) -> CompiledProgress {
    checked_kernel(3, point, values, budget)
}

fn main() {
    assert_eq!(ORIGINAL.program.compiled.ints.len(), 3);
    assert_eq!(ORIGINAL.program.compiled.customs.len(), 1);
    let mut artifact = ARTIFACT;
    artifact.program.compiled.ints = ORIGINAL
        .program
        .compiled
        .ints
        .iter()
        .enumerate()
        .map(|(index, target)| {
            let CompiledImplementation::BitArray(kernel) = &target.implementation else {
                panic!("BitArray source must select its generated kernel");
            };
            CompiledFunction {
                function: target.function,
                implementation: CompiledImplementation::BitArray(BitArrayImplementation {
                    entry: kernel.entry,
                    checkpoints: kernel.checkpoints.clone(),
                    run: [sum_kernel as BitArrayKernel, choose_kernel, wide_kernel][index],
                }),
            }
        })
        .collect();
    let target = &ORIGINAL.program.compiled.customs[0];
    let CompiledImplementation::BitArray(kernel) = &target.implementation else {
        panic!("custom source must select its BitArray kernel");
    };
    artifact.program.compiled.customs = vec![CompiledFunction {
        function: target.function,
        implementation: CompiledImplementation::BitArray(BitArrayImplementation {
            entry: kernel.entry,
            checkpoints: kernel.checkpoints.clone(),
            run: parse_kernel,
        }),
    }]
    .into();
    let mut bindings = Box::leak(Box::new(artifact)).load().unwrap();
    let sum = bindings
        .function(FunctionDeclaration::<(BitArrayValue, BigInt), BigInt>::new(
            "sum",
        ))
        .unwrap();
    let choose = bindings
        .function(FunctionDeclaration::<(BitArrayValue, bool), BigInt>::new(
            "choose",
        ))
        .unwrap();
    let wide = bindings
        .function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new("wide"))
        .unwrap();
    let parse = bindings
        .function(FunctionDeclaration::<
            (BitArrayValue, BigInt),
            Result<BigInt, ()>,
        >::new("parse"))
        .unwrap();
    let module = bindings.seal();
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            let input = BitArrayValue::from_bytes(vec![1; 1200]);
            assert_eq!(
                module
                    .call(&sum, (input, 5.into()), &mut Vec::new())
                    .unwrap(),
                1205.into()
            );
            for (flag, expected) in [(true, 7), (false, -7)] {
                assert_eq!(
                    module
                        .call(
                            &choose,
                            (BitArrayValue::from_bytes(vec![7]), flag),
                            &mut Vec::new()
                        )
                        .unwrap(),
                    BigInt::from(expected),
                );
            }
            let short = BitArrayValue::try_from_parts(vec![0xff], 7).unwrap();
            for (error, kind) in [
                (
                    module
                        .call(&sum, (short.clone(), 0.into()), &mut Vec::new())
                        .unwrap_err(),
                    PanicKind::Panic,
                ),
                (
                    module
                        .call(&choose, (short.clone(), true), &mut Vec::new())
                        .unwrap_err(),
                    PanicKind::LetAssert,
                ),
                (
                    module.call(&wide, (short,), &mut Vec::new()).unwrap_err(),
                    PanicKind::LetAssert,
                ),
            ] {
                let CallError::Execution(ExecutionError::Panic(panic)) = error.into_materialized()
                else {
                    panic!("source failure must keep its diagnostic");
                };
                assert_eq!(panic.kind(), kind);
                assert_eq!(panic.site().module(), "example");
            }
            let big: BigInt = BigInt::from(1) << 180;
            assert_eq!(
                module
                    .call(
                        &sum,
                        (BitArrayValue::from_bytes(vec![1, 2]), big.clone()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                &big + 3,
            );
            assert_eq!(
                module
                    .call(
                        &sum,
                        (BitArrayValue::from_bytes(vec![1, 2]), i64::MAX.into()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                BigInt::from(i64::MAX) + 3,
            );
            for value in [7_u64, i64::MAX as u64 + 1, u64::MAX] {
                assert_eq!(
                    module
                        .call(
                            &wide,
                            (BitArrayValue::from_bytes(value.to_be_bytes().to_vec()),),
                            &mut Vec::new()
                        )
                        .unwrap(),
                    BigInt::from(value),
                );
            }
            for (bytes, expected) in [
                (vec![], Ok(BigInt::from(5))),
                (vec![1, 99, 2], Ok(BigInt::from(8))),
                (vec![12], Err(())),
            ] {
                assert_eq!(
                    module
                        .call(
                            &parse,
                            (BitArrayValue::from_bytes(bytes), 5.into()),
                            &mut Vec::new()
                        )
                        .unwrap(),
                    expected,
                );
            }
            assert_eq!(
                module
                    .call(
                        &parse,
                        (BitArrayValue::from_bytes(vec![1, 99, 2]), i64::MAX.into()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                Ok(BigInt::from(i64::MAX) + 3),
            );
            let visited = VISITED.lock().unwrap();
            assert_eq!(visited.len(), 4);
            let missed: Vec<_> = visited
                .iter()
                .enumerate()
                .flat_map(|(index, points)| {
                    points
                        .iter()
                        .enumerate()
                        .filter_map(move |(point, visited)| (!visited).then_some((index, point)))
                })
                .collect();
            assert!(
                missed.is_empty(),
                "unvisited source checkpoints: {missed:?}"
            );
        })
        .unwrap()
        .join()
        .unwrap();
    println!("bit-array checkpoint execution completed");
}
"#,
    )
    .unwrap();
    checked(command("cargo", &application).args(["generate-lockfile", "--offline"]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, b"bit-array checkpoint execution completed\n");
    assert!(output.stderr.is_empty());
}
