use geam::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable, HostFailure,
    HostFunctionType, HostLocation, HostModule, HostProvider, HostProviderModule, HostProviderSet,
    HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource, PackageSource, PanicKind,
    StatelessHostProfile, compile_typed_host_program, plan_host_program,
};
use miette::{GraphicalReportHandler, GraphicalTheme};
use num_bigint::BigInt;

#[test]
fn reports_source_less_host_module_failure_at_the_gleam_call_site() {
    let control = HostModule::new("host_support", "host/control")
        .expect("host module should be valid")
        .with_fallible_function("ready", || -> Result<bool, HostFailure> {
            Err(HostFailure::new("not ready"))
        })
        .expect("host function should be valid");
    let source = r#"
import host/control

pub fn main() {
  control.ready()
}
"#;
    let expected = r#"
geam::host_function

  x host function host_support::host/control.ready failed: not ready
   ,-[src/main.gleam:5:3]
 4 | pub fn main() {
 5 |   control.ready()
   :   ^^^^^^^|^^^^^^^
   :          `-- host function host_support::host/control.ready failed
 6 | }
   `----
"#;

    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([control]).expect("host module should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("fallible host function should fail");

    assert_eq!(render_execution_error(&error), expected.trim());
    let ExecutionError::Host(error) = error else {
        panic!("fallible host function should produce a host error");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/control");
    assert_eq!(error.function(), "ready");
    assert_eq!(error.failure().message(), "not ready");
    assert_eq!(error.signature().argument_types(), []);
    assert_eq!(error.signature().return_(), &geam::ValueType::Bool);
    let HostLocation::Resolved { site, path, line } = error.location() else {
        panic!("source-backed host call should resolve its call site");
    };
    assert_eq!(site.module(), "main");
    assert_eq!(site.function(), "main");
    assert_eq!(path.as_str(), "src/main.gleam");
    assert_eq!(*line, 5);
}

#[test]
fn reports_source_provider_failure_at_the_gleam_tail_call_site() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_fallible_function("fail", |_: BigInt| -> Result<BigInt, HostFailure> {
            Err(HostFailure::new("service unavailable"))
        })
        .expect("provider function should be valid");
    let source = r#"
@external(erlang, "host", "fail")
fn fail(value: Int) -> Int

fn tail(value: Int) {
  fail(value)
}

pub fn main() {
  tail(1)
}
"#;
    let expected = r#"
geam::host_function

  x host function application::main.fail failed: service unavailable
   ,-[src/main.gleam:6:3]
 5 | fn tail(value: Int) {
 6 |   fail(value)
   :   ^^^^^|^^^^^
   :        `-- host function application::main.fail failed
 7 | }
   `----
"#;

    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider modules should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("provider should link");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("fallible provider should fail");

    assert_eq!(render_execution_error(&error), expected.trim());
    let ExecutionError::Host(error) = error else {
        panic!("fallible provider should produce a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "fail");
    assert_eq!(error.failure().message(), "service unavailable");
    assert_eq!(error.signature().argument_types(), [geam::ValueType::Int]);
    assert_eq!(error.signature().return_(), &geam::ValueType::Int);
    let HostLocation::Resolved { site, path, line } = error.location() else {
        panic!("source-backed provider failure should resolve its call site");
    };
    assert_eq!(site.module(), "main");
    assert_eq!(site.function(), "tail");
    assert_eq!(path.as_str(), "src/main.gleam");
    assert_eq!(*line, 6);
}

#[test]
fn reports_nested_host_failure_with_the_immediate_host_caller() {
    let outer = HostModule::new("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<CallbackProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            invoke_callback,
        )
        .expect("outer callback should register");
    let inner = HostModule::new("host_support", "host/inner")
        .expect("inner host module should be valid")
        .with_fallible_function("fail", |_: BigInt| -> Result<BigInt, HostFailure> {
            Err(HostFailure::new("inner unavailable"))
        })
        .expect("inner failure should register");
    let source = r#"
import host/inner
import host/outer

pub fn main() {
  outer.apply(inner.fail, 1)
}
"#;
    let expected = r#"
geam::host_function

  x host function host_support::host/inner.fail failed: inner unavailable
"#;

    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer, inner]).expect("host modules should be unique"),
    )
    .expect("nested host failure source should compile");
    let plan = plan_host_program(typed).expect("nested host failure source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("nested host failure execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("inner host should fail");

    assert_eq!(render_execution_error(&error), expected.trim());
    let ExecutionError::Host(error) = error else {
        panic!("nested host failure should remain a host error");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/inner");
    assert_eq!(error.function(), "fail");
    assert_eq!(error.failure().message(), "inner unavailable");
    let HostLocation::Host { caller } = error.location() else {
        panic!("direct host re-entry should preserve its host caller");
    };
    assert_eq!(caller.package(), "host_support");
    assert_eq!(caller.module(), "host/outer");
    assert_eq!(caller.function(), "apply");
}

#[test]
fn preserves_nested_gleam_panic_without_host_rewrapping() {
    let outer = HostModule::new("host_support", "host/outer")
        .expect("outer host module should be valid")
        .with_scoped_function::<CallbackProvider, (IntCallable, BigInt), BigInt, _>(
            "apply",
            invoke_callback,
        )
        .expect("outer callback should register");
    let source = r#"
import host/outer

fn stop(_value: Int) -> Int {
  panic as "nested source"
}

pub fn main() {
  outer.apply(stop, 1)
}
"#;
    let expected = r#"
geam::panic

  x panic: nested source
   ,-[src/main.gleam:5:3]
 4 | fn stop(_value: Int) -> Int {
 5 |   panic as "nested source"
   :   ^^^^^^^^^^^^|^^^^^^^^^^^
   :               `-- panic in main.stop
 6 | }
   `----
"#;

    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([outer]).expect("host module should be unique"),
    )
    .expect("nested panic source should compile");
    let plan = plan_host_program(typed).expect("nested panic source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("nested panic execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("nested source should panic");

    assert_eq!(render_execution_error(&error), expected.trim());
    let ExecutionError::Panic(panic) = error else {
        panic!("nested source panic should not become a host error");
    };

    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(panic.site().module(), "main");
    assert_eq!(panic.site().function(), "stop");
}

type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
type IntCallable = HostFunctionType<IntArguments, BigInt>;

struct CallbackProvider;

impl HostProvider<StatelessHostProfile> for CallbackProvider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

fn invoke_callback<'call>(
    mut call: HostCall<'call, StatelessHostProfile, CallbackProvider, BigInt>,
    function: HostCallable<'call, IntArguments, BigInt>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let returned = call.invoke(function, (value, ()))?;
    Ok(call.return_value(returned))
}

fn render_execution_error(error: &ExecutionError) -> String {
    let handler = GraphicalReportHandler::new_themed(GraphicalTheme::none())
        .with_links(false)
        .with_urls(false)
        .without_cause_chain()
        .without_syntax_highlighting()
        .with_context_lines(1)
        .with_width(120)
        .with_wrap_lines(false)
        .with_break_words(false);
    let mut rendered = String::new();
    handler
        .render_report(&mut rendered, error)
        .expect("diagnostic should render");

    rendered.trim_end().to_string()
}
