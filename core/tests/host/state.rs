use ecow::EcoString;
use geam_core::{
    BitArrayValue, ExecutionError, HostCall, HostCallCompletion, HostCallError, HostFailure,
    HostModule, HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeParameter,
    HostedExecution, ModuleSource, PackageSource, Value, ValueType, compile_typed_host_program,
    plan_host_program,
};
use num_bigint::BigInt;

struct StatefulProfile;

struct RunState {
    total: BigInt,
    audit_enabled: bool,
}

struct Counter;

impl HostProfile for StatefulProfile {
    type RunState = RunState;
    type ExternalStores = ();
}

impl HostProvider<StatefulProfile> for Counter {
    type State = BigInt;

    fn project(state: &mut RunState) -> &mut Self::State {
        &mut state.total
    }
}

fn accumulate<'call>(
    mut call: HostCall<'call, StatefulProfile, Counter, BigInt>,
    left: BigInt,
    right: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let total = call.state();
    *total += left + right;
    let value = total.clone();
    Ok(call.return_value(value))
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
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let mut state = RunState {
        total: BigInt::from(0),
        audit_enabled: false,
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
fn stateful_profiles_preserve_fallible_bool_host_failures() {
    let module = HostModule::<StatefulProfile>::new_for_profile("host_support", "host/state")
        .expect("host module should be valid")
        .with_fallible_function("ready", || -> Result<bool, HostFailure> {
            Err(HostFailure::new("not ready"))
        })
        .expect("host function should be valid");
    let source = r#"
import host/state

pub fn main() {
  state.ready()
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
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let mut state = RunState {
        total: BigInt::from(9),
        audit_enabled: false,
    };
    let error = execution
        .run_main(&mut state, &mut Vec::new())
        .expect_err("fallible Bool host function should fail");
    let ExecutionError::Host(error) = error else {
        panic!("fallible Bool host function should produce a host error");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/state");
    assert_eq!(error.function(), "ready");
    assert_eq!(error.failure().message(), "not ready");
    assert_eq!(error.signature().argument_types(), []);
    assert_eq!(error.signature().return_(), &ValueType::Bool);
    assert_eq!(state.total, BigInt::from(9));
    assert!(!state.audit_enabled);
}

#[test]
fn stateful_profiles_reject_reachable_unresolved_value_returns_while_sealing() {
    type Item = HostTypeParameter<0>;

    fn produce<'call>(
        _call: HostCall<'call, StatefulProfile, Counter, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("produce should not run").into())
    }

    let provider = HostProviderModule::<StatefulProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Counter, (), Item, _>("produce", produce)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "produce")
fn produce() -> value

pub fn main() {
  produce()
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<EcoString>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<StatefulProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("reachable unresolved value return should not seal");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "produce");
    assert!(error.signature().argument_types().is_empty());
    assert!(matches!(
        error.signature().return_(),
        ValueType::Parameter(_)
    ));
}

#[test]
fn stateful_profiles_preserve_unresolved_empty_parameter_lists() {
    let source = r#"
fn tail(values: List(value), flag: Bool) {
  case #(values, flag) {
    #([..tail], True) -> tail
    _ -> []
  }
}

pub fn main() {
  tail([], True)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<EcoString>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new(Vec::<HostModule<StatefulProfile>>::new())
            .expect("empty host set should be valid"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let mut state = RunState {
        total: BigInt::from(9),
        audit_enabled: true,
    };
    let value = execution
        .run_main(&mut state, &mut Vec::new())
        .expect("generic list program should execute");
    let Value::List(value) = value else {
        panic!("main should return a list");
    };

    assert!(value.is_empty());
    assert!(matches!(value.item_type(), ValueType::Parameter(_)));
    assert_eq!(state.total, BigInt::from(9));
    assert!(state.audit_enabled);
}

#[test]
fn executes_source_backed_provider_calls_with_caller_owned_state() {
    let provider = HostProviderModule::<StatefulProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Counter, (BigInt, BigInt), BigInt, _>("accumulate", accumulate)
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
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let mut state = RunState {
        total: BigInt::from(0),
        audit_enabled: false,
    };
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
    assert!(!state.audit_enabled);
    assert!(echoes.is_empty());

    let mut independent_state = RunState {
        total: BigInt::from(100),
        audit_enabled: true,
    };
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
    assert!(independent_state.audit_enabled);
}
