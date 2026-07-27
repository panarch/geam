use ecow::EcoString;
use geam::{
    BitArrayValue, ExecutionError, HostCall, HostFailure, HostLocation, HostModule, HostProfile,
    HostProvider, HostProviderModule, HostProviderSet, HostedExecution, InvariantError,
    ModuleSource, PackageSource, Value, ValueType, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

struct StatefulProfile;

struct RunState {
    total: BigInt,
}

struct Counter;

impl HostProfile for StatefulProfile {
    type RunState = RunState;
}

impl HostProvider<StatefulProfile> for Counter {
    type State = BigInt;

    fn project(state: &mut RunState) -> &mut Self::State {
        &mut state.total
    }
}

#[test]
fn source_less_modules_use_the_same_stateful_profile_registration() {
    let module = HostModule::<StatefulProfile>::new_for_profile("host_support", "host/state")
        .expect("host module should be valid")
        .with_function("int", std::convert::identity::<BigInt>)
        .expect("host function should be valid")
        .with_function("float", std::convert::identity::<f64>)
        .expect("host function should be valid")
        .with_function("string", std::convert::identity::<EcoString>)
        .expect("host function should be valid")
        .with_function("bit_array", std::convert::identity::<BitArrayValue>)
        .expect("host function should be valid")
        .with_function("utf_codepoint", std::convert::identity::<char>)
        .expect("host function should be valid")
        .with_function("bool", std::convert::identity::<bool>)
        .expect("host function should be valid")
        .with_function("nil", std::convert::identity::<()>)
        .expect("host function should be valid");
    let source = r#"
import host/state

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  #(
    state.int(1),
    state.float(2.5),
    state.string("three"),
    state.bit_array(<<4>>),
    state.utf_codepoint(codepoint),
    state.bool(True),
    state.nil(Nil),
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([module]).expect("host module should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");

    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        [("host_support", "host/state"), ("application", "main")],
    );
    assert!(
        plan.modules()[0]
            .functions()
            .iter()
            .all(|function| function.host_template().is_some()),
    );
    let execution = HostedExecution::from_module_plan(plan);
    let mut state = RunState {
        total: BigInt::from(0),
    };
    assert_eq!(
        execution.run_main(&mut state, &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Int(1.into()),
            Value::Float(2.5),
            Value::String("three".into()),
            Value::BitArray(BitArrayValue::from_bytes(vec![4])),
            Value::UtfCodepoint('A'),
            Value::Bool(true),
            Value::Nil,
        ])),
    );
}

#[test]
fn executes_source_backed_provider_calls_with_caller_owned_state() {
    let provider = HostProviderModule::<StatefulProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Counter, _, _, _>(
            "accumulate",
            |call: &mut HostCall<'_, StatefulProfile, Counter>, left: BigInt, right: BigInt| {
                let total = call.state();
                *total += left + right;
                Ok(total.clone())
            },
        )
        .expect("provider function should be valid");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<StatefulProfile>>::new(), [provider])
            .expect("provider modules should be unique");
    let source = r#"
@external(erlang, "host", "accumulate")
fn accumulate(left: Int, right: Int) -> Int

fn apply(function: fn(Int, Int) -> Int, left: Int, right: Int) {
  function(left, right)
}

fn tail(left: Int, right: Int) {
  accumulate(left, right)
}

pub fn main() {
  let function = accumulate
  #(
    accumulate(1, 2),
    apply(function, 4, 5),
    tail(6, 7),
    function == accumulate,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("provider should link");
    let functions = plan.modules()[0].functions();
    assert_eq!(
        functions
            .iter()
            .map(|function| {
                if let Some(function) = function.gleam_body() {
                    function.name().as_str()
                } else {
                    function
                        .host_template()
                        .expect("function should have one body owner")
                        .name()
                        .as_str()
                }
            })
            .collect::<Vec<_>>(),
        ["main", "accumulate", "apply", "tail"],
    );
    assert!(functions[0].gleam_body().is_some());
    assert!(functions[1].host_template().is_some());
    let execution = HostedExecution::from_module_plan(plan);
    let mut state = RunState { total: 0.into() };
    let mut echoes = Vec::new();

    assert_eq!(
        execution.run_main(&mut state, &mut echoes),
        Ok(Value::Tuple(vec![
            Value::Int(3.into()),
            Value::Int(12.into()),
            Value::Int(25.into()),
            Value::Bool(true),
        ])),
    );
    assert_eq!(state.total, BigInt::from(25));
    assert!(echoes.is_empty());

    let mut independent_state = RunState { total: 100.into() };
    assert_eq!(
        execution.run_main(&mut independent_state, &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Int(103.into()),
            Value::Int(112.into()),
            Value::Int(125.into()),
            Value::Bool(true),
        ])),
    );
    assert_eq!(independent_state.total, BigInt::from(125));
}

#[test]
fn executes_external_gleam_fallback_without_a_provider() {
    let source = r#"
@external(erlang, "host", "increment")
fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  increment(41)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new(Vec::<HostModule>::new()).expect("empty host set should be valid"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("fallback body should plan");
    assert!(plan.modules()[0].functions()[0].gleam_body().is_some());
    let execution = HostedExecution::from_module_plan(plan);

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

#[test]
fn reports_fallible_provider_failure_at_the_tail_call_site() {
    let provider = HostProviderModule::<geam::StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_fallible_function("fail", |_: BigInt| -> Result<BigInt, HostFailure> {
            Err(HostFailure::new("service unavailable"))
        })
        .expect("provider function should be valid");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
        .expect("provider modules should be unique");
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
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("provider should link");
    let execution = HostedExecution::from_module_plan(plan);
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("fallible provider should fail");
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
fn preserves_nested_execution_failure_without_rewrapping_it_as_host_failure() {
    let provider = HostProviderModule::<StatefulProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Counter, _, _, _>(
            "nested",
            |_call: &mut HostCall<'_, StatefulProfile, Counter>| {
                Result::<bool, geam::HostCallError>::Err(
                    ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                        item_type: ValueType::Bool,
                        index: 2,
                        length: 1,
                    })
                    .into(),
                )
            },
        )
        .expect("provider function should be valid");
    let hosts =
        HostProviderSet::with_providers(Vec::<HostModule<StatefulProfile>>::new(), [provider])
            .expect("provider modules should be unique");
    let source = r#"
@external(erlang, "host", "nested")
fn nested() -> Bool

pub fn main() {
  nested()
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("provider should link");
    let execution = HostedExecution::from_module_plan(plan);
    let mut state = RunState { total: 0.into() };

    assert_eq!(
        execution.run_main(&mut state, &mut Vec::new()),
        Err(ExecutionError::Invariant(
            InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Bool,
                index: 2,
                length: 1,
            },
        )),
    );
}
