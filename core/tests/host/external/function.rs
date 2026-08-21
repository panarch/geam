use super::{
    Counter, CounterProvider, CounterSchema, ExternalProfile, ExternalRunState, GenericCallback,
    GenericCounterSchema, GenericIntCallback, GenericValue, HostCounter, IntArguments, NoArguments,
};
use geam_core::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable, HostFailure,
    HostFunctionType, HostModule, HostProviderModule, HostProviderSet, HostTypeParameter,
    HostedExecution, ModuleSource, PackageSource, PanicKind, Value, compile_typed_host_program,
    plan_host_program,
};
use num_bigint::BigInt;
use std::convert::Infallible;

#[test]
fn external_profile_preserves_every_callback_return_family() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn invoke<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, GenericValue>,
        function: HostCallable<'call, NoArguments, GenericValue>,
    ) -> Result<HostCallCompletion<'call, GenericValue>, HostCallError> {
        let value = call.invoke(function, ())?;
        Ok(call.return_value(value))
    }

    fn invoke_with_int<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, GenericValue>,
        function: HostCallable<'call, IntArguments, GenericValue>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, GenericValue>, HostCallError> {
        let value = call.invoke(function, (value, ()))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid")
        .with_scoped_function::<CounterProvider, (GenericCallback,), GenericValue, _>(
            "invoke", invoke,
        )
        .expect("generic callback should be valid")
        .with_scoped_function::<CounterProvider, (GenericIntCallback, BigInt), GenericValue, _>(
            "invoke_with_int",
            invoke_with_int,
        )
        .expect("generic callback with an argument should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "invoke")
fn invoke(function: fn() -> value) -> value

@external(erlang, "host", "invoke_with_int")
fn invoke_with_int(function: fn(Int) -> value, value: Int) -> value

fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Marker(2) }
fn external_value() { new_counter(3) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(4, False) }
fn list_value() { [5, 6] }
fn increment(value: Int) { value + 1 }
fn function_value() { increment }
fn external_list_value() { [new_counter(10)] }
fn external_function_value() { new_counter }
fn generic_value(value) { value }

pub fn main() {
  let external_identity: fn(Counter) -> Counter = generic_value
  #(
    invoke(int_value),
    invoke(float_value),
    invoke(string_value),
    invoke(bit_array_value),
    invoke(utf_codepoint_value),
    invoke(custom_value),
    invoke_with_int(Marker, 7),
    invoke(external_value),
    invoke(bool_value),
    invoke(nil_value),
    invoke(tuple_value),
    invoke(list_value),
    invoke(function_value)(8),
    invoke(external_list_value),
    invoke(external_function_value)(11),
    external_identity(new_counter(12)),
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("callback family source should compile");
    let plan = plan_host_program(typed).expect("callback family source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("callback family execution should seal");

    assert_eq!(
        execution
            .run_main(&mut ExternalRunState::default(), &mut Vec::new())
            .expect("every callback family should execute")
            .inspect()
            .to_string(),
        r#"#(1, 1.5, "text", <<1>>, 'A', Marker(2), Marker(7), Counter(3), True, Nil, #(4, False), [5, 6], 9, [Counter(10)], Counter(11), Counter(12))"#,
    );
}

#[test]
fn external_profile_reports_diverging_external_function_returns() {
    type CounterCallable = HostFunctionType<IntArguments, HostCounter>;

    fn stop<'call>(
        _call: HostCall<'call, ExternalProfile, CounterProvider, CounterCallable>,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("external function stopped").into())
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_diverging_function::<CounterProvider, (), CounterCallable, _>("stop", stop)
        .expect("diverging external function provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop")
fn stop() -> fn(Int) -> Counter

pub fn main() {
  echo Nil as "before stop"
  stop()
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("diverging external function source should compile");
    let plan = plan_host_program(typed).expect("diverging external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("diverging external function execution should seal");
    let mut echoes = Vec::new();
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect_err("diverging external function should fail");
    let ExecutionError::Host(error) = error else {
        panic!("diverging external function should produce a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "stop");
    assert_eq!(error.failure().message(), "external function stopped");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("before stop"),
    );
}

#[test]
fn external_function_block_stops_before_returning_its_callable() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn stop() -> value {
  panic as "before external callable"
}

pub fn main() {
  let callable = {
    stop()
    fn(_value) { new_counter(1) }
  }
  callable
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external function block source should compile");
    let plan =
        plan_host_program(typed).expect("external function block source should plan completely");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external function block execution should seal");
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect_err("the source panic should stop before the callable is returned");
    let ExecutionError::Panic(error) = error else {
        panic!("the block should preserve its source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}

#[test]
fn external_profile_preserves_a_nested_failure_from_a_never_callback() {
    type NeverReturn = HostTypeParameter<0>;
    type NeverCallable = HostFunctionType<NoArguments, NeverReturn>;

    fn invoke_never<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        function: HostCallable<'call, NoArguments, NeverReturn>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let _ = call.invoke(function, ())?;
        Ok(call.return_value(0.into()))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<CounterProvider, (NeverCallable,), BigInt, _>(
            "invoke_never",
            invoke_never,
        )
        .expect("Never callback should register");
    let source = r#"
@external(erlang, "host", "invoke_never")
fn invoke_never(function: fn() -> value) -> Int

fn stop() -> value {
  panic as "nested Never callback"
}

pub fn main() {
  invoke_never(stop)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("Never callback source should compile");
    let plan = plan_host_program(typed).expect("Never callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("Never callback execution should seal");
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect_err("nested Never callback should preserve its panic");
    let ExecutionError::Panic(error) = error else {
        panic!("nested Never callback should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}

#[test]
fn preserves_external_function_values_across_expression_boundaries() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Maker {
  Maker(run: fn(Int) -> Counter)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

const constant_maker = new_counter
const generic_identity = identity

fn reference_maker() -> fn(Int) -> Counter {
  new_counter
}

fn constant_value() -> fn(Int) -> Counter {
  constant_maker
}

fn closure_value() -> fn(Int) -> Counter {
  fn(value) { new_counter(value + 1) }
}

fn direct_call_value() -> fn(Int) -> Counter {
  reference_maker()
}

fn function_call_value() -> fn(Int) -> Counter {
  let get = reference_maker
  get()
}

fn tuple_value() -> fn(Int) -> Counter {
  #(new_counter, 0).0
}

fn custom_value() -> fn(Int) -> Counter {
  Maker(new_counter).run
}

fn list_value() -> fn(Int) -> Counter {
  let assert [run] = [new_counter]
  run
}

fn panic_value() -> fn(Int) -> Counter {
  case True {
    True -> new_counter
    False -> panic
  }
}

fn bool_value(value: Bool) -> fn(Int) -> Counter {
  case value {
    True -> new_counter
    False -> closure_value()
  }
}

fn int_value(value: Int) -> fn(Int) -> Counter {
  case value {
    1 -> new_counter
    _ -> closure_value()
  }
}

fn float_value(value: Float) -> fn(Int) -> Counter {
  case value {
    1.5 -> new_counter
    _ -> closure_value()
  }
}

fn string_value(value: String) -> fn(Int) -> Counter {
  case value {
    "maker" -> new_counter
    _ -> closure_value()
  }
}

fn block_value() -> fn(Int) -> Counter {
  {
    let run = new_counter
    run
  }
}

fn tail_value() -> fn(Int) -> Counter {
  direct_call_value()
}

fn select_maker(_value: Int) -> fn(Int) -> Counter {
  new_counter
}

fn direct_call_argument_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let selected = select_maker(panic as "unreached direct function argument")
      selected
    }
    False -> new_counter
  }
}

fn function_call_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      { panic as "unreached function-valued source" }(0)
    }
    False -> new_counter
  }
}

fn function_call_argument_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let get = select_maker
      let selected = get(panic as "unreached function-valued argument")
      selected
    }
    False -> new_counter
  }
}

fn function_tuple_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      #(panic as "unreached function tuple source", new_counter).1
    }
    False -> new_counter
  }
}

fn function_custom_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      Maker(run: panic as "unreached function custom source").run
    }
    False -> new_counter
  }
}

fn function_list_source_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      case [panic as "unreached function list source"] {
        [selected] -> selected
        _ -> new_counter
      }
    }
    False -> new_counter
  }
}

fn function_block_step_diverges(diverge: Bool) -> fn(Int) -> Counter {
  case diverge {
    True -> {
      let selected = {
        let _: Nil = panic as "unreached function block step"
        new_counter
      }
      selected
    }
    False -> new_counter
  }
}

fn choose_divergent_maker(which: Int) -> fn(Int) -> Counter {
  case which {
    1 -> direct_call_argument_diverges(True)
    2 -> function_call_source_diverges(True)
    3 -> function_call_argument_diverges(True)
    4 -> function_tuple_source_diverges(True)
    5 -> function_custom_source_diverges(True)
    6 -> function_list_source_diverges(True)
    7 -> function_block_step_diverges(True)
    _ -> new_counter
  }
}

fn identity(value: value) -> value {
  value
}

fn wrap(value: value) -> List(value) {
  [value]
}

fn first(values: List(value)) -> value {
  let assert [head, ..] = values
  head
}

fn thunk(value: value) -> fn() -> value {
  fn() { value }
}

fn fail() {
  panic
}

pub fn main() {
  let direct = echo reference_maker() as "external function"
  let from_generic = identity(new_counter)
  let from_list = first(wrap(new_counter))
  let from_thunk = thunk(new_counter)()
  let safe_from_divergence = choose_divergent_maker(0)
  let external = identity(new_counter(20))
  let external_from_list = first(wrap(new_counter(21)))
  let external_from_thunk = thunk(new_counter(22))()
  let external_identity: fn(Counter) -> Counter = generic_identity
  let external_from_constant = external_identity(external)
  let guarded = case True {
    True -> external
    False -> fail()
  }
  #(
    direct(1),
    constant_value()(2),
    closure_value()(2),
    direct_call_value()(4),
    function_call_value()(5),
    tuple_value()(6),
    custom_value()(7),
    list_value()(8),
    panic_value()(9),
    bool_value(True)(10),
    int_value(1)(11),
    float_value(1.5)(12),
    string_value("maker")(13),
    block_value()(14),
    tail_value()(15),
    from_generic(16),
    from_list(17),
    from_thunk(18),
    safe_from_divergence(19),
    direct_call_argument_diverges(False)(23),
    function_call_source_diverges(False)(24),
    function_call_argument_diverges(False)(25),
    function_tuple_source_diverges(False)(26),
    function_custom_source_diverges(False)(27),
    function_list_source_diverges(False)(28),
    function_block_step_diverges(False)(29),
    external,
    external_from_list,
    external_from_thunk,
    external_from_constant,
    guarded,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("external function source should compile");
    let plan = plan_host_program(typed).expect("external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external function execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external function source should execute");

    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("external function"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(1), Counter(2), Counter(3), Counter(4), Counter(5), Counter(6), Counter(7), Counter(8), Counter(9), Counter(10), Counter(11), Counter(12), Counter(13), Counter(14), Counter(15), Counter(16), Counter(17), Counter(18), Counter(19), Counter(23), Counter(24), Counter(25), Counter(26), Counter(27), Counter(28), Counter(29), Counter(20), Counter(21), Counter(22), Counter(20), Counter(20))",
    );
}

#[test]
fn returns_core_function_values_from_an_external_profile() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() -> fn(Int) -> Int {
  echo Nil as "core function"
  increment
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("core function source should compile");
    let plan = plan_host_program(typed).expect("core function source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("core function execution should seal");
    let mut echoes = Vec::new();

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("core function source should execute");

    assert_eq!(returned.inspect().to_string(), "//fn(a) { ... }");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("core function"),
    );
}

#[test]
fn preserves_symbolic_external_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox(input) {
  CallableBox(run: fn(input) -> Counter)
}

fn diverge(_value) -> Counter {
  panic as "unreached external function"
}

const diverging_constant = diverge

fn provider() {
  diverge
}

fn forwarded_provider() {
  provider()
}

fn divergence_provider(_value: Int) {
  diverge
}

fn divergence_provider_factory() {
  divergence_provider
}

fn failing_divergence_provider() {
  panic as "unreached symbolic function source"
}

fn unreachable_divergence(which: Int) -> fn(input) -> Counter {
  case which {
    1 -> {
      divergence_provider(panic as "unreached symbolic direct argument")
    }
    2 -> {
      let callable = divergence_provider_factory()
      callable(panic as "unreached symbolic function argument")
    }
    3 -> {
      let callable = failing_divergence_provider()
      callable(0)
    }
    4 -> {
      #(panic as "unreached symbolic tuple source", diverge).1
    }
    5 -> {
      CallableBox(run: panic as "unreached symbolic custom source").run
    }
    6 -> {
      case [panic as "unreached symbolic list source"] {
        [selected] -> selected
        _ -> diverge
      }
    }
    7 -> {
      let failed: Int = panic as "unreached symbolic block step"
      let _ = failed
      diverge
    }
    _ -> diverge
  }
}

fn exercise(
  function: fn(input) -> Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Counter {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> diverging_constant
    1 -> diverge
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise(function, False, 0, "", 0.0)
    5 -> {
      let selected = fn() { function }
      selected()
    }
    6 -> #(function).0
    7 -> CallableBox(run: function).run
    8 -> from_block
    9 -> provider()
    _ -> panic as "unselected symbolic external function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn same(function: fn(input) -> Counter) {
  function == function
}

fn panic_value() -> fn(input) -> Counter {
  panic as "unselected external function panic"
}

pub fn main() {
  #(
    same(echo diverge as "symbolic external function"),
    same(forwarded_provider()),
    same(unreachable_divergence(0)),
    same(exercise(diverge, True, 0, "selected", 1.0)),
    same(exercise(diverge, True, 1, "selected", 1.0)),
    same(exercise(diverge, True, 2, "selected", 1.0)),
    same(exercise(diverge, True, 3, "selected", 1.0)),
    same(exercise(diverge, True, 4, "selected", 1.0)),
    same(exercise(diverge, True, 5, "selected", 1.0)),
    same(exercise(diverge, True, 6, "selected", 1.0)),
    same(exercise(diverge, True, 7, "selected", 1.0)),
    same(exercise(diverge, True, 8, "selected", 1.0)),
    same(exercise(diverge, True, 9, "selected", 1.0)),
    panic_value == panic_value,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("symbolic external function source should compile");
    let plan = plan_host_program(typed).expect("symbolic external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("symbolic external function execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("symbolic external function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 14]));
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("symbolic external function"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn preserves_executable_external_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox {
  CallableBox(run: fn(Int) -> Counter)
}

fn diverge(_value: Int) -> Counter {
  panic as "unreached external function"
}

const diverging_constant = diverge

fn provider() {
  diverge
}

fn exercise(
  function: fn(Int) -> Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(Int) -> Counter {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> diverging_constant
    1 -> diverge
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise(function, False, 0, "", 0.0)
    5 -> {
      let selected = fn() { function }
      selected()
    }
    6 -> #(function).0
    7 -> CallableBox(run: function).run
    8 -> from_block
    9 -> provider()
    _ -> panic as "unselected executable external function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn same(function: fn(Int) -> Counter) {
  function == function
}

fn panic_value() -> fn(Int) -> Counter {
  panic as "unselected external function panic"
}

pub fn main() {
  #(
    same(exercise(diverge, True, 0, "selected", 1.0)),
    same(exercise(diverge, True, 1, "selected", 1.0)),
    same(exercise(diverge, True, 2, "selected", 1.0)),
    same(exercise(diverge, True, 3, "selected", 1.0)),
    same(exercise(diverge, True, 4, "selected", 1.0)),
    same(exercise(diverge, True, 5, "selected", 1.0)),
    same(exercise(diverge, True, 6, "selected", 1.0)),
    same(exercise(diverge, True, 7, "selected", 1.0)),
    same(exercise(diverge, True, 8, "selected", 1.0)),
    same(exercise(diverge, True, 9, "selected", 1.0)),
    panic_value == panic_value,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("executable external function source should compile");
    let plan = plan_host_program(typed).expect("executable external function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("executable external function execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("executable external function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 11]));
}

#[test]
fn preserves_generic_external_return_function_handoffs() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, GenericCounterSchema>()
        .expect("generic external type should be valid");
    let source = r#"
@external(erlang, "host", "GenericCounter")
pub type GenericCounter(value)

pub type CallableBox(value) {
  CallableBox(run: fn(Int) -> GenericCounter(value))
}

fn diverge(_value: Int) -> GenericCounter(value) {
  panic as "unreached generic external function"
}

const diverging_constant = diverge

fn from_constant() -> fn(Int) -> GenericCounter(value) {
  diverging_constant
}

fn from_reference() -> fn(Int) -> GenericCounter(value) {
  diverge
}

fn from_closure() -> fn(Int) -> GenericCounter(value) {
  fn(_value: Int) { panic as "unreached generic external closure" }
}

fn from_argument(
  function: fn(Int) -> GenericCounter(value),
) -> fn(Int) -> GenericCounter(value) {
  let local = function
  local
}

fn from_call() -> fn(Int) -> GenericCounter(value) {
  from_reference()
}

fn from_function_call(
  provider: fn() -> fn(Int) -> GenericCounter(value),
) -> fn(Int) -> GenericCounter(value) {
  let selected = provider()
  selected
}

fn from_tuple() -> fn(Int) -> GenericCounter(value) {
  #(diverge).0
}

fn from_custom() -> fn(Int) -> GenericCounter(value) {
  CallableBox(run: diverge).run
}

fn from_list(
  functions: List(fn(Int) -> GenericCounter(value)),
) -> fn(Int) -> GenericCounter(value) {
  let assert [function] = functions
  function
}

fn from_bool(selector: Bool) -> fn(Int) -> GenericCounter(value) {
  case selector {
    True -> diverge
    False -> diverging_constant
  }
}

fn from_int(selector: Int) -> fn(Int) -> GenericCounter(value) {
  case selector {
    0 -> diverge
    _ -> diverging_constant
  }
}

fn from_string(selector: String) -> fn(Int) -> GenericCounter(value) {
  case selector {
    "reference" -> diverge
    _ -> diverging_constant
  }
}

fn from_float(selector: Float) -> fn(Int) -> GenericCounter(value) {
  case selector {
    1.0 -> diverge
    _ -> diverging_constant
  }
}

fn from_block() -> fn(Int) -> GenericCounter(value) {
  {
    let _ = Nil
    diverge
  }
}

fn from_panic() -> fn(Int) -> GenericCounter(value) {
  panic as "unselected generic external function panic"
}

fn same(function: fn(Int) -> GenericCounter(value)) {
  function == function
}

fn same_int(function: fn(Int) -> GenericCounter(Int)) {
  function == function
}

pub fn main() {
  #(
    same(from_constant()),
    same(from_reference()),
    same(from_closure()),
    same(from_argument(diverge)),
    same(from_call()),
    same(from_function_call(from_reference)),
    same(from_tuple()),
    same(from_custom()),
    same(from_list([diverge])),
    same_int(from_list([diverge])),
    same(from_bool(True)),
    same(from_bool(False)),
    same(from_int(0)),
    same(from_string("reference")),
    same(from_float(1.0)),
    same(from_block()),
    from_panic == from_panic,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<ExternalProfile>>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("generic external return function source should compile");
    let plan =
        plan_host_program(typed).expect("generic external return function source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external return function execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external return function source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 17]));
}
