use geam::{
    ExecutionError, HostCall, HostCallError, HostFailure, HostModule, HostProvider,
    HostProviderModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
    StatelessHostProfile, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::convert::Infallible;

#[test]
fn executes_direct_tail_non_tail_and_first_class_never_calls() {
    let cases = [
        (
            r#"
import host/control

pub fn main() {
  control.stop(1)
}
"#,
            "stopped at 1",
            5,
        ),
        (
            r#"
import host/control

pub fn main() -> Int {
  control.stop(2) + 1
}
"#,
            "stopped at 2",
            5,
        ),
        (
            r#"
import host/control

pub fn main() -> Int {
  let stop: fn(Int) -> Int = control.stop
  stop(3) + 1
}
"#,
            "stopped at 3",
            6,
        ),
    ];

    for (source, expected_failure, expected_line) in cases {
        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function("stop", |value: BigInt| -> Result<Infallible, HostFailure> {
                Err(HostFailure::new(format!("stopped at {value}")))
            })
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("stop should fail");
        let ExecutionError::Host(error) = error else {
            panic!("stop should produce a host error");
        };

        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/control");
        assert_eq!(error.function(), "stop");
        assert_eq!(error.failure().message(), expected_failure);
        assert_eq!(
            error.location().path().map(|path| path.as_str()),
            Some("main.gleam")
        );
        assert_eq!(error.location().line(), Some(expected_line));
    }
}

#[test]
fn executes_a_scoped_diverging_provider_in_the_default_stateless_profile() {
    struct Provider;

    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();

        fn project(state: &mut ()) -> &mut Self::State {
            state
        }
    }

    fn stop<'call>(
        _call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("scoped stop").into())
    }

    let control = HostModule::new("host_support", "host/control")
        .expect("host module should be valid")
        .with_scoped_diverging_function::<Provider, (), BigInt, _>("stop", stop)
        .expect("scoped diverging host should register");
    let source = r#"
import host/control

pub fn main() {
  control.stop()
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::new([control]).expect("host module should be unique"),
    )
    .expect("scoped diverging source should compile");
    let plan = plan_host_program(typed).expect("scoped diverging source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("scoped diverging execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("scoped diverging host should fail");
    let ExecutionError::Host(error) = error else {
        panic!("scoped diverging host should preserve its host failure");
    };

    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "scoped stop");
}

#[test]
fn compares_non_returning_function_references_without_invoking_them() {
    fn must_not_run() -> Infallible {
        panic!("function equality must not invoke the host callback")
    }

    let control = HostModule::new("host_support", "host/control")
        .expect("host module should be valid")
        .with_function("stop", must_not_run as fn() -> Infallible)
        .expect("host function should be valid");
    let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
    let source = r#"
import host/control

pub fn main() {
  let left: fn() -> Int = control.stop
  let right: fn() -> Int = control.stop
  left == right
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Bool(true)),
    );
}

#[test]
fn explains_non_returning_targets_in_every_scalar_function_table() {
    fn must_not_run() -> Infallible {
        panic!("function references must not invoke the host callback")
    }

    let control = HostModule::new("host_support", "host/control")
        .expect("host module should be valid")
        .with_function("stop", must_not_run as fn() -> Infallible)
        .expect("host function should be valid");
    let source = r#"
import host/control

pub fn main() {
  let int: fn() -> Int = control.stop
  let float: fn() -> Float = control.stop
  let string: fn() -> String = control.stop
  let bit_array: fn() -> BitArray = control.stop
  let utf_codepoint: fn() -> UtfCodepoint = control.stop
  let bool_: fn() -> Bool = control.stop
  let nil: fn() -> Nil = control.stop
  #(int, float, string, bit_array, utf_codepoint, bool_, nil)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::new([control]).expect("host module should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let expected_explanation = r#"
module main
main tuple#0

function int#0
  host host_support::host/control.stop signature=fn() -> Int

function float#0
  host host_support::host/control.stop signature=fn() -> Float

function string#0
  host host_support::host/control.stop signature=fn() -> String

function bit_array#0
  host host_support::host/control.stop signature=fn() -> BitArray

function utf_codepoint#0
  host host_support::host/control.stop signature=fn() -> UtfCodepoint

function bool#0
  host host_support::host/control.stop signature=fn() -> Bool

function nil#0
  host host_support::host/control.stop signature=fn() -> Nil

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.int#0:shape#1(fn() -> Int) = function[Int] reference int#0
    %function.float#0:shape#3(fn() -> Float) = function[Float] reference float#0
    %function.string#0:shape#5(fn() -> String) = function[String] reference string#0
    %function.bit_array#0:shape#7(fn() -> BitArray) = function[BitArray] reference bit_array#0
    %function.utf_codepoint#0:shape#9(fn() -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#0
    %function.bool#0:shape#11(fn() -> Bool) = function[Bool] reference bool#0
    %function.nil#0:shape#13(fn() -> Nil) = function[Nil] reference nil#0
    %tuple#0:shape#14(#(fn() -> Int, fn() -> Float, fn() -> String, fn() -> BitArray, fn() -> UtfCodepoint, fn() -> Bool, fn() -> Nil)) = tuple.value elements=[%function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.bool#0, %function.nil#0]
    return %tuple#0
"#
    .trim();

    assert_eq!(execution.explain().to_string().trim(), expected_explanation);
}

#[test]
fn executes_non_returning_hosts_through_every_scalar_and_compound_return_family() {
    let sources = [
        r#"
import host/control

fn stop() -> Float { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

fn stop() -> String { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

fn stop() -> BitArray { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

fn stop() -> UtfCodepoint { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

fn stop() -> Bool { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

fn stop() -> Nil { control.stop() }

pub fn main() { stop() }
"#,
        r#"
import host/control

pub fn main() {
  let stop: fn() -> #(Int) = control.stop
  stop()
}
"#,
    ];

    for source in sources {
        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function("stop", || -> Result<Infallible, HostFailure> {
                Err(HostFailure::new("stopped"))
            })
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("stop should fail");

        assert!(matches!(
            error,
            ExecutionError::Host(error) if error.failure().message() == "stopped"
        ));
    }
}

#[test]
fn locates_non_returning_hosts_specialized_to_function_returns() {
    let cases = [
        (
            r#"
import host/control

pub fn main() -> fn(Int) -> Int {
  control.stop()
}
"#,
            5,
        ),
        (
            r#"
import host/control

pub fn main() -> fn(Int) -> Int {
  let stop: fn() -> fn(Int) -> Int = control.stop
  stop()
}
"#,
            6,
        ),
    ];

    for (source, expected_line) in cases {
        let control = HostModule::new("host_support", "host/control")
            .expect("host module should be valid")
            .with_fallible_function("stop", || -> Result<Infallible, HostFailure> {
                Err(HostFailure::new("stopped"))
            })
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([control]).expect("host modules should be unique");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("stop should fail");
        let ExecutionError::Host(error) = error else {
            panic!("stop should produce a host error");
        };

        assert_eq!(error.function(), "stop");
        assert_eq!(error.failure().message(), "stopped");
        assert_eq!(
            error.location().path().map(|path| path.as_str()),
            Some("main.gleam"),
        );
        assert_eq!(error.location().line(), Some(expected_line));
    }
}

#[test]
fn seals_an_explicit_non_returning_provider_with_an_unresolved_source_return() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_fallible_function("stop", || -> Result<Infallible, HostFailure> {
            Err(HostFailure::new("stopped"))
        })
        .expect("non-returning provider should be valid");
    let source = r#"
@external(erlang, "host", "stop")
fn stop() -> value

pub fn main() {
  let stop_function = stop
  stop_function()
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("non-returning provider should seal without return storage");
    let expected_explanation = r#"
module main
main never#0

function never#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.never#0:shape#1(fn() -> param#0) = function[Never] reference never#1
    never_call %function.never#0 args=[]

function never#1
  host application::main.stop signature=fn() -> param#0
"#
    .trim();

    assert_eq!(execution.explain().to_string().trim(), expected_explanation);
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("stop should fail");
    let ExecutionError::Host(error) = error else {
        panic!("stop should produce a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "stopped");
}
