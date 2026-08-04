use super::{
    Counter, CounterProvider, CounterSchema, ExternalProfile, ExternalRunState,
    GenericCounterSchema, HostCounter, HostGenericCounter,
};
use geam::{
    ExecutionError, HostCall, HostCallCompletion, HostCallError, HostExternal, HostFailure,
    HostModule, HostProviderModule, HostProviderSet, HostTupleType, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostValue, HostedExecution, ModuleSource, PackageSource, PanicKind, Value,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn external_function_block_rejects_an_unrepresentable_step_before_its_callable() {
    type Item = HostTypeParameter<0>;

    fn produce<'call>(
        _call: HostCall<'call, ExternalProfile, CounterProvider, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("produce should not run").into())
    }

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
        .with_scoped_function::<CounterProvider, (), Item, _>("produce", produce)
        .expect("generic producer should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("external constructor should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "produce")
fn produce() -> value

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

pub fn main() {
  let callable = {
    let _ = produce()
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
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("the unresolved producer should prevent executable sealing");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "produce");
    assert!(error.signature().argument_types().is_empty());
    assert!(matches!(
        error.signature().return_(),
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn specializes_generic_external_tail_calls_and_list_cases() {
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

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn identity(value) {
  value
}

fn forward(value) {
  identity(value)
}

fn list_identity(values: List(value)) -> List(value) {
  values
}

fn forward_list(values: List(value)) -> List(value) {
  list_identity(values)
}

fn thunk(value) -> fn() -> value {
  fn() { value }
}

fn forward_thunk(value) -> fn() -> value {
  thunk(value)
}

fn list_thunk(value) -> fn() -> List(value) {
  fn() { [value] }
}

fn forward_list_thunk(value) -> fn() -> List(value) {
  list_thunk(value)
}

fn choose_list(selected: Bool, value: Counter) -> List(Counter) {
  case selected {
    True -> [value]
    False -> []
  }
}

pub fn main() {
  let counter = new_counter(9)
  #(
    forward(counter),
    forward([counter]),
    forward_list([counter]),
    forward_thunk(counter)(),
    forward_list_thunk(counter)(),
    forward(fn() { [counter] })(),
    choose_list(True, counter),
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
    .expect("generic external source should compile");
    let plan = plan_host_program(typed).expect("generic external source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(9), [Counter(9)], [Counter(9)], Counter(9), [Counter(9)], [Counter(9)], [Counter(9)])",
    );
}

#[test]
fn specializes_symbolic_external_function_constants_and_capturing_closures() {
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

pub type Never

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

const keep_constant = keep

fn keep(_ignored: Never) -> Counter {
  new_counter(11)
}

fn capture(counter: Counter) -> fn(Never) -> Counter {
  fn(_ignored) { counter }
}

pub fn main() {
  let counter = new_counter(9)
  #(keep_constant, capture(counter))
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
    .expect("symbolic external callable source should compile");
    let plan = plan_host_program(typed).expect("symbolic external callable source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("symbolic external callable execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("symbolic external callable source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(//fn(a) { ... }, //fn(a) { ... })",
    );
}

#[test]
fn specializes_generic_external_types_by_their_source_arguments() {
    type ValueType = HostTypeParameter<0>;
    type SingletonElements = HostTypeList<ValueType, HostTypeListEnd>;
    type Singleton = HostTupleType<SingletonElements>;

    fn tag<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostGenericCounter>,
        _value: HostValue<'call, ValueType>,
        tag: BigInt,
    ) -> Result<HostCallCompletion<'call, HostGenericCounter>, HostCallError> {
        let counter = call.create_external(Counter { value: tag });
        Ok(call.return_value(counter))
    }

    fn read<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostGenericCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn singleton<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, Singleton>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, Singleton>, HostCallError> {
        Ok(call.return_tuple((value, ())))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, GenericCounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (ValueType, BigInt), HostGenericCounter, _>(
            "tag", tag,
        )
        .expect("generic constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostGenericCounter,), BigInt, _>("read", read)
        .expect("generic reader provider should be valid")
        .with_scoped_function::<CounterProvider, (ValueType,), Singleton, _>("singleton", singleton)
        .expect("generic tuple provider should be valid");
    let source = r#"
@external(erlang, "host", "GenericCounter")
pub type GenericCounter(value)

@external(erlang, "host", "tag")
fn tag(value: value, tag: Int) -> GenericCounter(value)

@external(erlang, "host", "read")
fn read(counter: GenericCounter(value)) -> Int

@external(erlang, "host", "singleton")
fn singleton(value: value) -> #(value)

pub fn main() {
  let int = tag(1, 10)
  let bool = tag(True, 20)
  #(int, bool, read(int), read(bool), singleton(int))
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
    .expect("generic external source should compile");
    let plan = plan_host_program(typed).expect("generic external source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic external execution should seal");
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic external source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(GenericCounter(10), GenericCounter(20), 10, 20, #(GenericCounter(10)))",
    );
}

#[test]
fn specializes_generic_function_expressions_to_external_returns() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type CallableBox(input, output) {
  CallableBox(function: fn(input) -> output)
}

fn external_function(_value: Int) -> Counter {
  panic as "unreached external function"
}

fn identity(value: value) -> value {
  value
}

const function_constant = identity

fn forward_function(function: fn(input) -> output) -> fn(input) -> output {
  function
}

fn provide_function(
  function: fn(input) -> output,
) -> fn() -> fn(input) -> output {
  fn() { function }
}

fn first_function(
  functions: List(fn(input) -> output),
) -> fn(input) -> output {
  case functions {
    [function] -> function
    _ -> panic as "expected one function"
  }
}

fn transform_function(
  function: fn(input) -> output,
) -> fn(input) -> output {
  let function = function_constant(function)
  let closure = fn(value) { function(value) }
  let direct = forward_function(closure)
  let provider = provide_function(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = CallableBox(function: from_tuple)
  let from_field = boxed.function
  let from_list = case [from_field] {
    [first] -> first
    _ -> function
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> function
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external function"
  }
}

fn select_function(
  function: fn(input) -> output,
  selector: Bool,
) -> fn(input) -> output {
  case selector {
    True -> function
    False -> function
  }
}

pub fn main() {
  let transformed = transform_function(external_function)
  let selected = select_function(external_function, True)
  let listed: fn(Int) -> Counter = first_function([external_function])
  #(
    transformed == transformed,
    selected == external_function,
    listed == external_function,
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
    .expect("generic function expression source should compile");
    let plan = plan_host_program(typed).expect("generic function expression source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("generic function expression execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("generic function expression source should execute");

    assert_eq!(
        returned,
        Value::Tuple(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
        ]),
    );
}

#[test]
fn evaluates_external_return_function_sources_before_uninhabited_arguments() {
    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
        .expect("external type should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Never

fn select(
  provider: fn() -> fn(trigger) -> fn(input) -> output,
  fail: fn() -> trigger,
) -> fn(input) -> output {
  provider()(fail())
}

fn external_function(_value: Int) -> Counter {
  panic as "external function should not run"
}

fn make(_trigger: Never) -> fn(Int) -> Counter {
  external_function
}

fn provide_factory() -> fn(Never) -> fn(Int) -> Counter {
  echo make as "function source"
}

fn fail_argument() -> Never {
  panic as "argument failed"
}

pub fn main() {
  select(provide_factory, fail_argument)
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
    .expect("diverging external function call source should compile");
    let plan =
        plan_host_program(typed).expect("diverging external function call source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("diverging external function call execution should seal");

    let mut echoes = Vec::new();
    let error = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect_err("the uninhabited argument should panic");
    let ExecutionError::Panic(error) = error else {
        panic!("the uninhabited argument should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "fail_argument");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("function source"),
    );
    assert_eq!(echoes[0].value().inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn preserves_external_constants_and_list_expression_owners() {
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

pub type ListHolder {
  ListHolder(selected: List(Counter))
}

pub type GenericListHolder(value) {
  GenericListHolder(selected: List(value))
}

pub type FunctionHolder(value) {
  FunctionHolder(run: value)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn identity(value: value) -> value {
  value
}

fn second(_ignored: first, value: second) -> second {
  value
}

const generic_empty = []
const generic_empty_alias = generic_empty
const exact_empty: List(Counter) = []
const exact_empty_alias: List(Counter) = exact_empty
const selected_constant: List(Counter) = generic_empty_alias
const selected_holder = ListHolder(selected: exact_empty_alias)
const generic_holder = GenericListHolder(selected: [])
const external_holder: GenericListHolder(Counter) = generic_holder
const generic_nested_empty = [generic_empty_alias]
const exact_nested_empty: List(List(Counter)) = generic_nested_empty
const direct_nested_empty: List(List(Counter)) = [[]]
const external_function: fn(Int) -> Counter = new_counter
const external_function_alias = external_function
const direct_function_holder = FunctionHolder(run: new_counter)
const alias_function_holder = FunctionHolder(run: external_function_alias)
const generic_function = identity
const generic_function_alias = generic_function
const generic_second = second
const generic_second_alias = generic_second
const external_second:
  fn(Nil, Counter) -> Counter =
  generic_second_alias
const specialized_identity:
  fn(fn(Int) -> Counter) -> fn(Int) -> Counter =
  generic_function_alias

fn provider(value: Int) -> List(Counter) {
  [new_counter(value)]
}

fn provider_function() -> fn(Int) -> List(Counter) {
  provider
}

fn from_tuple(value: Counter) -> List(Counter) {
  #([value], Nil).0
}

fn from_custom(value: Counter) -> List(Counter) {
  ListHolder(selected: [value]).selected
}

fn from_nested_list(value: Counter) -> List(Counter) {
  let assert [selected] = [[value]]
  selected
}

fn from_bool(value: Counter, selector: Bool) -> List(Counter) {
  case selector {
    True -> [value]
    False -> []
  }
}

fn from_int(value: Counter, selector: Int) -> List(Counter) {
  case selector {
    1 -> [value]
    _ -> []
  }
}

fn from_string(value: Counter, selector: String) -> List(Counter) {
  case selector {
    "selected" -> [value]
    _ -> []
  }
}

fn from_float(value: Counter, selector: Float) -> List(Counter) {
  case selector {
    1.0 -> [value]
    _ -> []
  }
}

fn from_block(value: Counter) -> List(Counter) {
  {
    let selected = value
    [selected]
  }
}

fn from_unreached_panic(value: Counter) -> List(Counter) {
  case True {
    True -> [value]
    False -> panic as "unselected external list panic"
  }
}

fn drop_first(values: List(Counter)) -> List(Counter) {
  case values {
    [_, ..tail] -> tail
    _ -> []
  }
}

fn list_first(values: List(Counter)) -> Counter {
  case values {
    [head, ..] -> head
    _ -> panic as "missing external list head"
  }
}

fn same(values: List(value)) -> Bool {
  values == values
}

fn forwarded_empty() -> List(value) {
  generic_empty_alias
}

fn forwarded_function() -> fn(value) -> value {
  generic_function_alias
}

pub fn main() {
  let first = new_counter(1)
  let second = external_function_alias(2)
  let specialized = specialized_identity(new_counter)
  let spread = [first, second, ..exact_empty_alias]
  let assert [head, ..tail] = spread
  let callable = provider_function()

  #(
    generic_empty_alias == selected_constant,
    forwarded_empty() == selected_constant,
    same(spread),
    head == first,
    list_first(spread) == first,
    tail == [second],
    drop_first(spread) == [second],
    provider(1) == [first],
    callable(2) == [second],
    from_tuple(first) == [first],
    from_custom(first) == [first],
    from_nested_list(first) == [first],
    from_bool(first, True) == [first],
    from_bool(first, False) == [],
    from_int(first, 1) == [first],
    from_int(first, 0) == [],
    from_string(first, "selected") == [first],
    from_string(first, "fallback") == [],
    from_float(first, 1.0) == [first],
    from_float(first, 0.0) == [],
    from_block(first) == [first],
    from_unreached_panic(first) == [first],
    specialized(1) == first,
    forwarded_function()(first) == first,
    selected_holder.selected == [],
    external_holder.selected == [],
    exact_nested_empty == [[]],
    direct_nested_empty == [[]],
    direct_function_holder.run(3) == new_counter(3),
    alias_function_holder.run(4) == new_counter(4),
    external_second(Nil, first) == first,
    provider == provider,
    provider_function == provider_function,
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
    .expect("external constant and list source should compile");
    let plan = plan_host_program(typed).expect("external constant and list source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external constant and list execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external constant and list source should execute");

    assert_eq!(returned, Value::Tuple(vec![Value::Bool(true); 33]));
}

#[test]
fn preserves_external_expression_divergence_in_specialized_graphs() {
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

pub type Wrapped {
  Wrapped(value: Counter)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn direct_call_argument(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let selected = new_counter(panic as "unreached direct argument")
      selected
    }
    False -> new_counter(1)
  }
}

fn function_call_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      { panic as "unreached function source" }(1)
    }
    False -> new_counter(2)
  }
}

fn function_call_argument(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let function = new_counter
      let selected = function(panic as "unreached function argument")
      selected
    }
    False -> new_counter(3)
  }
}

fn tuple_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      #(panic as "unreached tuple source", new_counter(4)).1
    }
    False -> new_counter(4)
  }
}

fn custom_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      Wrapped(value: panic as "unreached custom source").value
    }
    False -> new_counter(5)
  }
}

fn list_source(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      case [panic as "unreached list source"] {
        [selected] -> selected
        _ -> new_counter(6)
      }
    }
    False -> new_counter(6)
  }
}

fn block_step(diverge: Bool) -> Counter {
  case diverge {
    True -> {
      let selected = {
        let _: Nil = panic as "unreached block step"
        new_counter(7)
      }
      selected
    }
    False -> new_counter(7)
  }
}

fn choose_divergence(which: Int) -> Counter {
  case which {
    1 -> direct_call_argument(True)
    2 -> function_call_source(True)
    3 -> function_call_argument(True)
    4 -> tuple_source(True)
    5 -> custom_source(True)
    6 -> list_source(True)
    7 -> block_step(True)
    _ -> new_counter(0)
  }
}

fn route(
  value: Counter,
  fallback: Counter,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> Counter {
  let from_bool = case bool_selector {
    True -> value
    False -> fallback
  }
  let from_int = case int_selector {
    1 -> from_bool
    _ -> fallback
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> fallback
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> fallback
  }
  from_float
}

pub fn main() {
  #(
    route(
      choose_divergence(0),
      new_counter(9),
      True,
      1,
      "selected",
      1.0,
    ),
    direct_call_argument(False),
    function_call_source(False),
    function_call_argument(False),
    tuple_source(False),
    custom_source(False),
    list_source(False),
    block_step(False),
    direct_call_argument == direct_call_argument,
    function_call_source == function_call_source,
    function_call_argument == function_call_argument,
    tuple_source == tuple_source,
    custom_source == custom_source,
    list_source == list_source,
    block_step == block_step,
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
    .expect("external divergence source should compile");
    let plan = plan_host_program(typed).expect("external divergence source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("external divergence execution should seal");

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external divergence source should execute");

    assert_eq!(
        returned.inspect().to_string(),
        "#(Counter(0), Counter(1), Counter(2), Counter(3), Counter(4), Counter(5), Counter(6), Counter(7), True, True, True, True, True, True, True)",
    );
}
