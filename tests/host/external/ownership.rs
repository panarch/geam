use super::{
    Counter, CounterProvider, CounterSchema, ExternalProfile, ExternalRunState, HostCounter,
    HostWrappedCounter, HostWrappedCounterConstructor,
};
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostCallable, HostCustom, HostExternal,
    HostFailure, HostFunctionType, HostList, HostListType, HostModule, HostProviderModule,
    HostProviderSet, HostTupleType, HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn preserves_external_values_through_lists_customs_captures_and_calls() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn counter_value<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn first_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        values: HostList<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let first = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("counter list should not be empty"))?;
        Ok(call.return_value(first))
    }

    fn duplicate_counter<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        Ok(call.return_list([counter, counter]))
    }

    type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
    type CounterCallable = HostFunctionType<IntArguments, HostCounter>;

    fn invoke_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        function: HostCallable<'call, IntArguments, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.invoke(function, (value, ()))?;
        Ok(call.return_value(counter))
    }

    fn forward_counter_function<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterCallable>,
        function: HostCallable<'call, IntArguments, HostCounter>,
    ) -> Result<HostCallCompletion<'call, CounterCallable>, HostCallError> {
        Ok(call.return_value(function))
    }

    fn counter_from_wrapped<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        wrapped: HostCustom<'call, HostWrappedCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let (counter, ()) = call
            .custom_fields::<HostWrappedCounterConstructor>(wrapped)
            .ok_or_else(|| HostFailure::new("wrapped counter should use Wrapped"))?;
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("constructor provider should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), BigInt, _>(
            "counter_value",
            counter_value,
        )
        .expect("reader provider should be valid")
        .with_scoped_function::<CounterProvider, (HostListType<HostCounter>,), HostCounter, _>(
            "first_counter",
            first_counter,
        )
        .expect("external list reader provider should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), HostListType<HostCounter>, _>(
            "duplicate_counter",
            duplicate_counter,
        )
        .expect("external list builder provider should be valid")
        .with_scoped_function::<CounterProvider, (CounterCallable, BigInt), HostCounter, _>(
            "invoke_counter",
            invoke_counter,
        )
        .expect("external callback provider should be valid")
        .with_scoped_function::<CounterProvider, (CounterCallable,), CounterCallable, _>(
            "forward_counter_function",
            forward_counter_function,
        )
        .expect("external callback forwarding provider should be valid")
        .with_scoped_function::<CounterProvider, (HostWrappedCounter,), BigInt, _>(
            "counter_from_wrapped",
            counter_from_wrapped,
        )
        .expect("external custom-field provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

pub type Wrapped {
  Wrapped(value: Counter)
}

pub type ValueBox(value) {
  ValueBox(value: value)
}

pub type ListBox(value) {
  ListBox(values: List(value))
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "counter_value")
fn counter_value(counter: Counter) -> Int

@external(erlang, "host", "first_counter")
fn first_counter(values: List(Counter)) -> Counter

@external(erlang, "host", "duplicate_counter")
fn duplicate_counter(counter: Counter) -> List(Counter)

@external(erlang, "host", "invoke_counter")
fn invoke_counter(function: fn(Int) -> Counter, value: Int) -> Counter

@external(erlang, "host", "forward_counter_function")
fn forward_counter_function(function: fn(Int) -> Counter) -> fn(Int) -> Counter

@external(erlang, "host", "counter_from_wrapped")
fn counter_from_wrapped(wrapped: Wrapped) -> Int

fn identity(value: value) -> value {
  value
}

fn transform_value(
  value: value,
  fallback: value,
  mapper: fn(value) -> value,
) -> value {
  let local = value
  let direct = identity(local)
  let called = mapper(direct)
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ValueBox(value: from_tuple)
  let from_field = boxed.value
  let from_list = case [from_field] {
    [first] -> first
    _ -> fallback
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> fallback
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> fallback
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> fallback
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external value"
  }
}

fn forward_list(values: List(value)) -> List(value) {
  values
}

fn provide_list(values: List(value)) -> fn() -> List(value) {
  fn() { values }
}

fn transform_list(values: List(value)) -> List(value) {
  let direct = forward_list(values)
  let provider = provide_list(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ListBox(values: from_tuple)
  let from_field = boxed.values
  let from_nested = case [from_field] {
    [first] -> first
    _ -> values
  }
  let from_int = case 1 {
    1 -> from_nested
    _ -> values
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> values
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> values
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic external list"
  }
}

fn prepend(value: value, values: List(value)) -> List(value) {
  [value, ..values]
}

fn drop_generic(values: List(value)) -> List(value) {
  case values {
    [_, ..tail] -> tail
    _ -> []
  }
}

fn tail_counter(value: Int) -> Counter {
  new_counter(value)
}

fn captured_reader(counter: Counter) -> fn() -> Int {
  fn() { counter_value(counter) }
}

fn bind_external(counter: Counter) -> Counter {
  case counter {
    selected -> selected
  }
}

fn discard_external(counter: Counter, fallback: Counter) -> Counter {
  case counter {
    _ -> fallback
  }
}

fn alias_external(counter: Counter) -> Counter {
  case counter {
    selected as alias -> {
      let _ = selected
      alias
    }
  }
}

fn preserve_external(counter: Counter) -> fn() -> Counter {
  fn() { counter }
}

fn preserve_external_list(values: List(Counter)) -> fn() -> List(Counter) {
  fn() { values }
}

fn preserve_external_function(function: fn(Int) -> Counter) -> fn() -> fn(Int) -> Counter {
  fn() { function }
}

pub fn main() {
  let make = new_counter
  let first = make(10)
  let second = tail_counter(20)
  let values = [first, second]
  let identity_values = identity(values)
  echo identity_values as "external list"
  let wrapped = Wrapped(second)
  let read = captured_reader(first)
  let transformed = transform_value(first, second, identity)
  let transformed_values = transform_list(identity_values)
  let prepended = prepend(transformed, transformed_values)
  let remaining = drop_generic(prepended)
  #(
    values,
    wrapped,
    read(),
    transformed,
    remaining,
    preserve_external(first),
    preserve_external_list(values),
    preserve_external_function(make),
    bind_external(first),
    discard_external(first, second),
    alias_external(first),
    first_counter(values),
    duplicate_counter(first),
    invoke_counter(new_counter, 30),
    forward_counter_function(make)(40),
    counter_from_wrapped(wrapped),
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
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");

    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external source should execute");
    drop(execution);
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("external list"),
    );
    assert_eq!(
        echoes[0].value().inspect().to_string(),
        "[Counter(10), Counter(20)]",
    );
    let Value::Tuple(values) = returned else {
        panic!("main should return a tuple");
    };
    let Value::List(counters) = &values[0] else {
        panic!("first tuple field should be a list");
    };
    assert_eq!(
        counters
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(20)"],
    );
    let Value::Custom(wrapped) = &values[1] else {
        panic!("second tuple field should be custom");
    };
    let Value::External(wrapped_counter) = wrapped.fields()[0].value() else {
        panic!("custom field should be external");
    };
    assert_eq!(wrapped_counter.inspection(), "Counter(20)");
    assert_eq!(values[2], Value::Int(BigInt::from(10)));
    let Value::External(transformed) = &values[3] else {
        panic!("fourth tuple field should be external");
    };
    assert_eq!(transformed.inspection(), "Counter(10)");
    let Value::List(remaining) = &values[4] else {
        panic!("fifth tuple field should be a list");
    };
    assert_eq!(
        remaining
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("remaining list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(20)"],
    );
    assert_eq!(values[5].inspect().to_string(), "//fn() { ... }");
    assert_eq!(values[6].inspect().to_string(), "//fn() { ... }");
    assert_eq!(values[7].inspect().to_string(), "//fn() { ... }");
    let Value::External(bound) = &values[8] else {
        panic!("ninth tuple field should be external");
    };
    assert_eq!(bound.inspection(), "Counter(10)");
    let Value::External(discarded) = &values[9] else {
        panic!("tenth tuple field should be external");
    };
    assert_eq!(discarded.inspection(), "Counter(20)");
    let Value::External(aliased) = &values[10] else {
        panic!("eleventh tuple field should be external");
    };
    assert_eq!(aliased.inspection(), "Counter(10)");
    let Value::External(from_list) = &values[11] else {
        panic!("twelfth tuple field should be external");
    };
    assert_eq!(from_list.inspection(), "Counter(10)");
    let Value::List(duplicated) = &values[12] else {
        panic!("thirteenth tuple field should be a list");
    };
    assert_eq!(
        duplicated
            .to_values()
            .iter()
            .map(|value| match value {
                Value::External(value) => value.inspection().as_str(),
                _ => panic!("duplicated list item should be external"),
            })
            .collect::<Vec<_>>(),
        ["Counter(10)", "Counter(10)"],
    );
    let Value::External(from_callback) = &values[13] else {
        panic!("fourteenth tuple field should be external");
    };
    assert_eq!(from_callback.inspection(), "Counter(30)");
    let Value::External(from_forwarded_callback) = &values[14] else {
        panic!("fifteenth tuple field should be external");
    };
    assert_eq!(from_forwarded_callback.inspection(), "Counter(40)");
    assert_eq!(values[15], Value::Int(BigInt::from(20)));
}

#[test]
fn passes_external_values_and_lists_through_scoped_callbacks() {
    type CounterArguments = HostTypeList<HostCounter, HostTypeListEnd>;
    type CounterReader = HostFunctionType<CounterArguments, BigInt>;
    type CounterListFunction = HostFunctionType<CounterArguments, HostListType<HostCounter>>;
    type CounterTuple = HostTupleType<CounterArguments>;

    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn read_counter<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.external_payload(counter).value.clone();
        Ok(call.return_value(value))
    }

    fn duplicate<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        Ok(call.return_list([counter, counter]))
    }

    fn wrap<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterTuple>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, CounterTuple>, HostCallError> {
        Ok(call.return_tuple((counter, ())))
    }

    fn invoke_reader<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        function: HostCallable<'call, CounterArguments, BigInt>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let value = call.invoke(function, (counter, ()))?;
        Ok(call.return_value(value))
    }

    fn invoke_list<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostListType<HostCounter>>,
        function: HostCallable<'call, CounterArguments, HostListType<HostCounter>>,
        counter: HostExternal<'call, HostCounter>,
    ) -> Result<HostCallCompletion<'call, HostListType<HostCounter>>, HostCallError> {
        let values = call.invoke(function, (counter, ()))?;
        let first = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("callback list should not be empty"))?;
        Ok(call.return_list([first]))
    }

    fn forward_list_function<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, CounterListFunction>,
        function: HostCallable<'call, CounterArguments, HostListType<HostCounter>>,
    ) -> Result<HostCallCompletion<'call, CounterListFunction>, HostCallError> {
        Ok(call.return_value(function))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
        .expect("external type should be valid")
        .with_scoped_function::<CounterProvider, (BigInt,), HostCounter, _>(
            "new_counter",
            new_counter,
        )
        .expect("counter constructor should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), BigInt, _>(
            "read_counter",
            read_counter,
        )
        .expect("counter reader should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), HostListType<HostCounter>, _>(
            "duplicate",
            duplicate,
        )
        .expect("counter list builder should be valid")
        .with_scoped_function::<CounterProvider, (HostCounter,), CounterTuple, _>("wrap", wrap)
        .expect("counter tuple builder should be valid")
        .with_scoped_function::<CounterProvider, (CounterReader, HostCounter), BigInt, _>(
            "invoke_reader",
            invoke_reader,
        )
        .expect("counter reader callback should be valid")
        .with_scoped_function::<
            CounterProvider,
            (CounterListFunction, HostCounter),
            HostListType<HostCounter>,
            _,
        >("invoke_list", invoke_list)
        .expect("counter list callback should be valid")
        .with_scoped_function::<
            CounterProvider,
            (CounterListFunction,),
            CounterListFunction,
            _,
        >("forward_list_function", forward_list_function)
        .expect("counter list callback forwarding should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "read_counter")
fn read_counter(counter: Counter) -> Int

@external(erlang, "host", "duplicate")
fn duplicate(counter: Counter) -> List(Counter)

@external(erlang, "host", "wrap")
fn wrap(counter: Counter) -> #(Counter)

@external(erlang, "host", "invoke_reader")
fn invoke_reader(function: fn(Counter) -> Int, counter: Counter) -> Int

@external(erlang, "host", "invoke_list")
fn invoke_list(
  function: fn(Counter) -> List(Counter),
  counter: Counter,
) -> List(Counter)

@external(erlang, "host", "forward_list_function")
fn forward_list_function(
  function: fn(Counter) -> List(Counter),
) -> fn(Counter) -> List(Counter)

fn tail_list(counter: Counter) -> List(Counter) {
  duplicate(counter)
}

pub fn main() {
  let counter = new_counter(7)
  let reader = read_counter
  let list_function = duplicate
  let forwarded_list_function = forward_list_function(list_function)
  let wrapped = wrap(counter)
  #(
    invoke_reader(reader, wrapped.0),
    invoke_list(list_function, counter),
    tail_list(counter),
    list_function(counter),
    forwarded_list_function(counter),
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
    .expect("external callback source should compile");
    let plan = plan_host_program(typed).expect("external callback source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external callback source should seal");

    assert_eq!(
        execution
            .run_main(&mut ExternalRunState::default(), &mut Vec::new())
            .expect("external callback source should execute")
            .inspect()
            .to_string(),
        "#(7, [Counter(7)], [Counter(7), Counter(7)], [Counter(7), Counter(7)], [Counter(7), Counter(7)])",
    );
}

#[test]
fn returns_captured_external_function_values_from_the_program_entry() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterSchema>()
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

fn provide() -> fn(Int) -> Counter {
  new_counter
}

fn retain_provider(
  provider: fn() -> fn(Int) -> Counter,
) -> fn() -> fn() -> fn(Int) -> Counter {
  fn() { provider }
}

pub fn main() -> fn(Int) -> Counter {
  let restore = retain_provider(provide)
  let provider = restore()
  let make = provider()
  let _ = make(41)
  make
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
    .expect("captured external function source should compile");
    let plan = plan_host_program(typed).expect("captured external function source should plan");
    let returned = {
        let execution = HostedExecution::try_from_module_plan(plan)
            .expect("captured external function execution should seal");
        let mut state = ExternalRunState::default();

        execution
            .run_main(&mut state, &mut Vec::new())
            .expect("captured external function source should execute")
    };
    assert_eq!(returned.inspect().to_string(), "//fn(a) { ... }");
}
