use super::{checked, command};
use geam::embedding::{BigInt, FunctionDeclaration, List, ModuleBuilder};
use std::fs;
use std::path::Path;

#[test]
fn generated_list_kernels_compile_strictly_and_resume_every_prefix_with_one_step_budgets() {
    let inversions = "  let flag = !flag\n".repeat(500);
    let source = format!(
        r#"
pub fn sum(values: List(Int), total: Int) -> Int {{
  case values {{
    [] -> total
    [head, ..tail] -> {{
      let total = case head {{
        0 -> total
        _ -> total + head
      }}
      sum(tail, total)
    }}
  }}
}}

pub fn choose(values: List(Int), flag: Bool) -> Int {{
{inversions}  case flag {{
    True -> {{
      let assert [head, ..] = values
      head
    }}
    False -> {{
      let assert [head, ..] = values
      0 - head
    }}
  }}
}}

pub fn same(left: List(Int), right: List(Int)) -> Bool {{ left == right }}

pub fn zero_tail(values: List(Int)) -> Bool {{
  let assert [..tail] as original = values
  tail == original
}}
"#
    );
    let typed = geam::compile_typed_module("example", "src/example.gleam", &source).unwrap();
    let (mut bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
            "sum",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<BigInt>, bool), BigInt>::new(
            "choose",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<BigInt>, List<BigInt>), bool>::new("same"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<BigInt>,), bool>::new(
            "zero_tail",
        ))
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
name = 'int-list-checkpoint-consumer'
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
use data::compiled::int_list::{IntListOps, IntListValues};
use data::compiled::{
    CompiledFunction, CompiledImplementation, CompiledProgress, IntListImplementation,
};
use geam::__prepared_support as data;
use geam::embedding::{BigInt, CallError, FunctionDeclaration, List};
use geam::{ExecutionError, PanicKind};
use std::convert::Infallible;
use std::sync::Mutex;

const ARTIFACT: data::ModuleArtifact<Infallible> = include!("program.rs");
static ORIGINAL: data::ModuleArtifact<Infallible> = ARTIFACT;
static VISITED: Mutex<Vec<Vec<bool>>> = Mutex::new(Vec::new());

// This observes the actual emitted function; it does not implement its graph.
// Limiting a batch to one canonical step forces every cold resume boundary.
fn checked_kernel(
    index: usize,
    point: usize,
    values: &mut IntListValues,
    lists: &IntListOps<'_>,
    budget: &mut usize,
) -> CompiledProgress {
    let CompiledImplementation::IntList(kernel) =
        &ORIGINAL.program.compiled.ints[index].implementation
    else {
        panic!("list target");
    };
    let mut visited = VISITED.lock().unwrap();
    if visited.is_empty() {
        *visited = ORIGINAL
            .program
            .compiled
            .ints
            .iter()
            .map(|target| {
                let CompiledImplementation::IntList(kernel) = &target.implementation else {
                    panic!("list target");
                };
                vec![false; kernel.checkpoints.len()]
            })
            .collect();
    }
    visited[index][point] = true;
    drop(visited);
    let prefix = kernel.checkpoints[point];
    assert_eq!(
        (
            values.ints.len(),
            values.bools.len(),
            values.int_lists.len()
        ),
        (prefix.ints, prefix.bools, prefix.int_lists)
    );
    let allowance = (*budget).min(1);
    let mut remaining = allowance;
    let progress = (kernel.run)(point, values, lists, &mut remaining);
    *budget -= allowance - remaining;
    if let CompiledProgress::Yield(next) | CompiledProgress::Interpreted(next) = progress {
        let prefix = kernel.checkpoints[next];
        assert_eq!(
            (
                values.ints.len(),
                values.bools.len(),
                values.int_lists.len()
            ),
            (prefix.ints, prefix.bools, prefix.int_lists)
        );
        if matches!(progress, CompiledProgress::Yield(_)) {
            assert_eq!(remaining, 0);
        }
    }
    progress
}

fn sum_kernel(
    point: usize,
    values: &mut IntListValues,
    lists: &IntListOps<'_>,
    budget: &mut usize,
) -> CompiledProgress {
    checked_kernel(0, point, values, lists, budget)
}

fn choose_kernel(
    point: usize,
    values: &mut IntListValues,
    lists: &IntListOps<'_>,
    budget: &mut usize,
) -> CompiledProgress {
    checked_kernel(1, point, values, lists, budget)
}

fn main() {
    assert_eq!(ORIGINAL.program.compiled.ints.len(), 2);
    assert_eq!(ORIGINAL.program.compiled.bools.len(), 2);
    let mut artifact = ARTIFACT;
    artifact.program.compiled.ints = ORIGINAL
        .program
        .compiled
        .ints
        .iter()
        .enumerate()
        .map(|(index, target)| {
            let CompiledImplementation::IntList(kernel) = &target.implementation else {
                panic!("list target");
            };
            let run = [
                sum_kernel as data::compiled::int_list::IntListKernel,
                choose_kernel,
            ][index];
            CompiledFunction {
                function: target.function,
                implementation: CompiledImplementation::IntList(IntListImplementation {
                    entry: kernel.entry,
                    checkpoints: kernel.checkpoints.clone(),
                    run,
                }),
            }
        })
        .collect();
    let mut bindings = Box::leak(Box::new(artifact)).load().unwrap();
    let sum = bindings
        .function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
            "sum",
        ))
        .unwrap();
    let choose = bindings
        .function(FunctionDeclaration::<(List<BigInt>, bool), BigInt>::new(
            "choose",
        ))
        .unwrap();
    let same = bindings
        .function(FunctionDeclaration::<(List<BigInt>, List<BigInt>), bool>::new("same"))
        .unwrap();
    let zero_tail = bindings
        .function(FunctionDeclaration::<(List<BigInt>,), bool>::new("zero_tail"))
        .unwrap();
    let module = bindings.seal();
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            let values: Vec<BigInt> = (0..1200).map(|index| BigInt::from(index % 3)).collect();
            assert_eq!(
                module
                    .call(&sum, (values, 5.into()), &mut Vec::new())
                    .unwrap(),
                BigInt::from(1205)
            );
            for flag in [false, true] {
                assert_eq!(
                    module
                        .call(&choose, (vec![7.into()], flag), &mut Vec::new())
                        .unwrap(),
                    BigInt::from(if flag { 7 } else { -7 })
                );
                let error = module
                    .call(&choose, (Vec::<BigInt>::new(), flag), &mut Vec::new())
                    .unwrap_err()
                    .into_materialized();
                let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                    panic!("let assert failure");
                };
                assert_eq!(panic.kind(), PanicKind::LetAssert);
            }
            // Every reachable checkpoint in these two branch/loop graphs has now
            // resumed with its exact prefix; a cold chain uses bounded Rust stack.
            let missed: Vec<_> = VISITED
                .lock()
                .unwrap()
                .iter()
                .enumerate()
                .flat_map(|(index, points)| {
                    let CompiledImplementation::IntList(kernel) =
                        &ORIGINAL.program.compiled.ints[index].implementation
                    else {
                        panic!("list target");
                    };
                    points
                        .iter()
                        .enumerate()
                        .filter_map(move |(point, visited)| {
                            (!visited).then_some((index, point, kernel.checkpoints[point]))
                        })
                })
                .collect();
            assert!(missed.is_empty(), "unvisited checkpoints: {missed:?}");
            let big: BigInt = BigInt::from(1) << 180;
            assert_eq!(
                module
                    .call(
                        &sum,
                        (vec![1.into(), big.clone(), 2.into()], 3.into()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                &big + 6
            );
            assert_eq!(
                module
                    .call(
                        &sum,
                        (vec![0.into(), 1.into()], big.clone()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                &big + 1
            );
            assert_eq!(
                module
                    .call(
                        &sum,
                        (vec![1.into(), 2.into()], i64::MAX.into()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                BigInt::from(i64::MAX) + 3
            );
            assert_eq!(
                module
                    .call(&choose, (vec![big.clone()], false), &mut Vec::new())
                    .unwrap(),
                -&big
            );
            assert!(
                module
                    .call(&same, (vec![big.clone()], vec![big]), &mut Vec::new())
                    .unwrap()
            );
            assert!(
                !module
                    .call(&same, (vec![1.into()], vec![]), &mut Vec::new())
                    .unwrap()
            );
            assert!(
                module
                    .call(&zero_tail, (vec![],), &mut Vec::new())
                    .unwrap()
            );
            assert!(
                module
                    .call(&zero_tail, (vec![BigInt::from(1) << 180],), &mut Vec::new())
                    .unwrap()
            );
        })
        .unwrap()
        .join()
        .unwrap();
    println!("list checkpoint execution completed");
}
"#,
    )
    .unwrap();
    checked(command("cargo", &application).args(["generate-lockfile", "--offline"]));
    let output = checked(command("cargo", &application).args(["run", "--quiet", "--locked"]));
    assert_eq!(output.stdout, b"list checkpoint execution completed\n");
    assert!(output.stderr.is_empty());
}
