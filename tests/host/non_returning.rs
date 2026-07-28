use geam::{
    ExecutionError, HostFailure, HostModule, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
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
        let execution = HostedExecution::from_module_plan(plan);
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
    let execution = HostedExecution::from_module_plan(plan);

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Bool(true)),
    );
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
        let execution = HostedExecution::from_module_plan(plan);
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
        let execution = HostedExecution::from_module_plan(plan);
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
