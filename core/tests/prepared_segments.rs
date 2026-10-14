#![cfg(feature = "tokio")]

use geam_core::__prepared_support as data;
use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder, ModuleBuilder};
use geam_core::execution::TokioHost;
use geam_core::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};

static PROGRAM: data::HostedModuleArtifact =
    include!("fixtures/prepared/generated/float_segments.rs");

#[test]
fn segmented_handoff_emission_keeps_operations_and_omits_unused_budget() {
    let source = format!(
        r#"
fn inspect(value: Float) -> Float {{
  let callback = fn(value) {{ value }}
  {}
  case value >. 0.0 {{
    True -> {{ echo value callback(value) }}
    False -> callback(value +. 1.0)
  }}
}}
pub fn main() {{ inspect(1.0) }}
"#,
        "let value = value +. 1.0\n".repeat(33)
    );
    let typed = geam_core::compile_typed_module("example", "src/example.gleam", &source).unwrap();
    let (plain, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), f64>::new("main"))
        .unwrap();
    let typed = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new("example", "src/example.gleam", &source)],
        )],
        HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
    )
    .unwrap();
    let (hosted, _) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), f64>::new("main"))
        .unwrap();
    for emitted in [
        plain.prepare().emit_rust(),
        hosted.prepare().unwrap().emit_rust(),
    ] {
        let signatures = emitted
            .lines()
            .map(str::trim_start)
            .filter(|line| line.starts_with("fn ") && line.contains("_segment_"))
            .collect::<Vec<_>>();
        assert_eq!(signatures.len(), 8);
        assert_eq!(
            signatures.first().copied(),
            Some(
                "fn calls_float_1_segment_0(mut active: Float1Segment0State, ops: &mut CallOps<'_>, budget: &mut usize) -> ControlFlow<FunctionStep, Float1State> {"
            ),
        );
        assert_eq!(
            signatures.get(3).copied(),
            Some(
                "fn calls_float_1_segment_69(active: Float1Segment69State, _ops: &mut CallOps<'_>, _budget: &mut usize) -> ControlFlow<FunctionStep, Float1State> {"
            ),
        );
    }
}

#[test]
fn prepared_hosted_float_segments_preserve_both_conditional_destinations_on_a_small_stack() {
    assert_eq!(PROGRAM.module.program.compiled.function_calls.len(), 3);
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(|| {
            let typed = geam_core::compile_typed_host_program(
                "example",
                "example",
                [PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "example",
                        "src/float_segments.gleam",
                        include_str!("fixtures/prepared/float_segments.gleam"),
                    )],
                )],
                HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
            )
            .unwrap();
            let (bindings, dynamic_function) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), f64>::new("main"))
                .unwrap();
            let mut bindings_prepared = PROGRAM
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let prepared_function = bindings_prepared
                .function(FunctionDeclaration::<(), f64>::new("main"))
                .unwrap();
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let host = TokioHost::new(runtime.handle().clone());
            for (mut module, function) in [
                (bindings.seal().unwrap(), dynamic_function),
                (bindings_prepared.seal(), prepared_function),
            ] {
                let mut echo = Vec::new();
                let result = runtime
                    .block_on(
                        module.with_execution(&host, &mut (), &mut echo, async |scope| {
                            scope.call(&function, ()).await.unwrap()
                        }),
                    )
                    .unwrap()
                    .try_into_value()
                    .unwrap();
                assert_eq!(result, 69.0);
                assert!(echo.is_empty());
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
