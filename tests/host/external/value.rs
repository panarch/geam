use super::{
    Counter, CounterProvider, CounterSchema, ExternalProfile, ExternalRunState, GenericValue,
    HostCounter,
};
use ecow::EcoString;
use geam::{
    BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostExternal, HostModule,
    HostProviderModule, HostProviderSet, HostValue, HostedExecution, ModuleSource, PackageSource,
    Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn hashes_symbolic_and_external_function_values_through_the_host_call() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    fn source_hash<'call>(
        call: HostCall<'call, ExternalProfile, CounterProvider, BigInt>,
        value: HostValue<'call, GenericValue>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let hash = call.source_hash::<GenericValue>(value);
        Ok(call.return_value(BigInt::from(hash)))
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
        .with_scoped_function::<CounterProvider, (GenericValue,), BigInt, _>(
            "source_hash",
            source_hash,
        )
        .expect("source hash should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "source_hash")
fn source_hash(value: value) -> Int

pub type Marker {
  Marker(Int)
}

fn identity(value) {
  value
}

fn never() -> value {
  panic as "source hash must not invoke its argument"
}

fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Marker(2) }
fn external(value: Int) {
  new_counter(value)
}
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(4, False) }
fn list_value() { [5, 6] }
fn function_value() { int_value }
fn parameter_list() -> List(value) { [] }

pub fn main() {
  let parameters = parameter_list()
  let generic = identity
  let no_return = never
  let constructor = Marker
  let closure = fn(value) { value + 1 }
  #(
    source_hash(parameters) == source_hash(parameters),
    source_hash(generic) == source_hash(generic),
    source_hash(no_return) == source_hash(no_return),
    source_hash(int_value) == source_hash(int_value),
    source_hash(float_value) == source_hash(float_value),
    source_hash(string_value) == source_hash(string_value),
    source_hash(bit_array_value) == source_hash(bit_array_value),
    source_hash(utf_codepoint_value) == source_hash(utf_codepoint_value),
    source_hash(custom_value) == source_hash(custom_value),
    source_hash(constructor) == source_hash(constructor),
    source_hash(external) == source_hash(external),
    source_hash(bool_value) == source_hash(bool_value),
    source_hash(nil_value) == source_hash(nil_value),
    source_hash(tuple_value) == source_hash(tuple_value),
    source_hash(list_value) == source_hash(list_value),
    source_hash(function_value) == source_hash(function_value),
    source_hash(closure) == source_hash(closure),
  )
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
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("source hash source should compile");
    let plan = plan_host_program(typed).expect("source hash source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("source hash execution should seal");

    assert_eq!(
        execution.run_main(&mut ExternalRunState::default(), &mut Vec::new()),
        Ok(Value::Tuple(vec![Value::Bool(true); 17])),
    );
}

#[test]
fn creates_compares_and_returns_opaque_external_values() {
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

    let provider = HostProviderModule::<ExternalProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_external_type::<CounterProvider, CounterSchema>()
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
        .expect("reader provider should be valid");
    let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "counter_value")
fn counter_value(counter: Counter) -> Int

pub fn main() {
  let first = new_counter(42)
  let same_value = new_counter(42)
  echo first as "created"
  #(first, counter_value(first), first == first, first == same_value)
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
    let planned_external = &plan.modules()[0].external_types()[0];
    assert_eq!(planned_external.name().package(), "application");
    assert_eq!(planned_external.name().module(), "main");
    assert_eq!(planned_external.name().name(), "Counter");
    assert!(planned_external.parameters().is_empty());
    let (external, echoes, read, identity_equal, source_equal) = {
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
        let mut state = ExternalRunState::default();
        let mut echoes = Vec::new();
        let returned = execution
            .run_main(&mut state, &mut echoes)
            .expect("external source should execute");
        let Value::Tuple(mut values) = returned else {
            panic!("main should return an external value tuple");
        };
        let Value::Bool(source_equal) = values.pop().expect("source equality") else {
            panic!("last tuple field should be Bool");
        };
        let Value::Bool(identity_equal) = values.pop().expect("identity equality") else {
            panic!("third tuple field should be Bool");
        };
        let Value::Int(read) = values.pop().expect("read value") else {
            panic!("second tuple field should be Int");
        };
        let Value::External(external) = values.pop().expect("external value") else {
            panic!("first tuple field should be external");
        };

        (external, echoes, read, identity_equal, source_equal)
    };

    assert_eq!(read, BigInt::from(42));
    assert!(identity_equal);
    assert!(source_equal);
    assert_eq!(external.inspection(), "Counter(42)");
    assert_eq!(external.type_().type_name().package(), "application");
    assert_eq!(external.type_().type_name().module(), "main");
    assert_eq!(external.type_().type_name().name(), "Counter");
    assert_eq!(echoes.len(), 1);
    assert_eq!(
        echoes[0].message().map(|message| message.as_str()),
        Some("created"),
    );
    let Value::External(echoed) = echoes[0].value() else {
        panic!("echo should preserve the external value");
    };
    assert_eq!(echoed.identity(), external.identity());
    assert_eq!(echoed.inspection(), "Counter(42)");

    let cloned = external.clone();
    assert_eq!(cloned.identity(), external.identity());
    assert_eq!(cloned.inspection(), "Counter(42)");
}

#[test]
fn explains_external_targets_lists_and_captures() {
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

fn preserve(
  counter: Counter,
  values: List(Counter),
  maker: fn(Int) -> Counter,
) {
  fn() { #(counter, values, maker) }
}

pub fn main() {
  let counter = new_counter(1)
  let values = [counter]
  let maker = new_counter
  preserve(counter, values, maker)
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
    let expected = r#"
module main
main function.tuple#0

function external#0
  host application::main.new_counter signature=fn(Int) -> external_type#0

function tuple#0
  entry b0 params=[] captures=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
  block b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
    %tuple#0:shape#4(#(external_type#0, list_type#0, fn(Int) -> external_type#0)) = tuple.value elements=[%external#0, %list.external#0, %function.external#0]
    return %tuple#0

function function.tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 1
    %external#0:shape#1(external_type#0) = external.call external#0 args=[%int#0]
    %list.external#0:shape#2(list_type#0) = list.external[type#0] value elements=[%external#0]
    %function.external#0:shape#3(fn(Int) -> external_type#0) = function[External] reference external#0
    tail function.tuple#1 args=[%external#0, %list.external#0, %function.external#0]

function function.tuple#1
  entry b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)] captures=[]
  block b0 params=[%external#0:shape#1(external_type#0), %list.external#0:shape#2(list_type#0), %function.external#0:shape#3(fn(Int) -> external_type#0)]
    %function.tuple#0:shape#5(fn() -> #(external_type#0, list_type#0, fn(Int) -> external_type#0)) = function[Tuple] closure target=tuple#0 captures=[%external#0<-%external#0, %list.external#0<-%list.external#0, %function.external#0<-%function.external#0]
    return %function.tuple#0
"#;

    assert_eq!(execution.explain().to_string().trim(), expected.trim());
}

#[test]
fn external_storage_profiles_preserve_existing_runtime_families() {
    fn new_counter<'call>(
        mut call: HostCall<'call, ExternalProfile, CounterProvider, HostCounter>,
        value: BigInt,
    ) -> Result<HostCallCompletion<'call, HostCounter>, HostCallError> {
        let counter = call.create_external(Counter { value });
        Ok(call.return_value(counter))
    }

    let scalars = HostModule::<ExternalProfile>::new_for_profile("host_support", "host/scalars")
        .expect("host module should be valid")
        .with_function("int", |value: BigInt| value)
        .expect("Int host function should be valid")
        .with_function("float", |value: f64| value)
        .expect("Float host function should be valid")
        .with_function("string", |value: EcoString| value)
        .expect("String host function should be valid")
        .with_function("bit_array", |value: BitArrayValue| value)
        .expect("BitArray host function should be valid")
        .with_function("utf_codepoint", |value: char| value)
        .expect("UtfCodepoint host function should be valid")
        .with_function("bool", |value: bool| value)
        .expect("Bool host function should be valid")
        .with_function("nil", |(): ()| ())
        .expect("Nil host function should be valid");
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
import host/scalars

@external(erlang, "host", "Counter")
pub type Counter

pub type Boxed {
  Boxed(Int)
}

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

fn custom_value() -> Boxed {
  Boxed(1)
}

fn tuple_value() -> #(Int) {
  #(1)
}

fn list_value() -> List(Int) {
  [1]
}

fn function_value() -> fn(Int) -> Int {
  scalars.int
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let int = scalars.int(1)
  let float = scalars.float(1.0)
  let string = scalars.string("one")
  let bit_array = scalars.bit_array(<<1>>)
  let utf_codepoint = scalars.utf_codepoint(codepoint)
  let custom = custom_value()
  let external = new_counter(8)
  let bool = scalars.bool(True)
  let nil = scalars.nil(Nil)
  let tuple = tuple_value()
  let list = list_value()
  let function = scalars.int

  echo #(
    int,
    float,
    string,
    bit_array,
    utf_codepoint,
    custom,
    external,
    bool,
    nil,
    tuple,
    list,
    function,
  ) as "values"

  let int_list = [int]
  let float_list = [float]
  let string_list = [string]
  let bit_array_list = [bit_array]
  let utf_codepoint_list = [utf_codepoint]
  let custom_list = [custom]
  let external_list = [external]
  let bool_list = [bool]
  let nil_list = [nil]
  let tuple_list = [tuple]
  let nested_list = [list]
  let function_list = [function]

  echo #(
    int_list,
    float_list,
    string_list,
    bit_array_list,
    utf_codepoint_list,
    custom_list,
    external_list,
    bool_list,
    nil_list,
    tuple_list,
    nested_list,
    function_list,
  ) as "lists"

  echo #(
    scalars.int,
    scalars.float,
    scalars.string,
    scalars.bit_array,
    scalars.utf_codepoint,
    custom_value,
    new_counter,
    scalars.bool,
    scalars.nil,
    tuple_value,
    list_value,
    function_value,
  ) as "functions"

  echo #(
    fn() { int },
    fn() { float },
    fn() { string },
    fn() { bit_array },
    fn() { utf_codepoint },
    fn() { custom },
    fn() { external },
    fn() { bool },
    fn() { nil },
    fn() { tuple },
    fn() { list },
    fn() { function },
  ) as "value captures"

  echo #(
    fn() { int_list },
    fn() { float_list },
    fn() { string_list },
    fn() { bit_array_list },
    fn() { utf_codepoint_list },
    fn() { custom_list },
    fn() { external_list },
    fn() { bool_list },
    fn() { nil_list },
    fn() { tuple_list },
    fn() { nested_list },
    fn() { function_list },
  ) as "list captures"

  echo #(
    fn() { scalars.int },
    fn() { scalars.float },
    fn() { scalars.string },
    fn() { scalars.bit_array },
    fn() { scalars.utf_codepoint },
    fn() { custom_value },
    fn() { Boxed },
    fn() { new_counter },
    fn() { scalars.bool },
    fn() { scalars.nil },
    fn() { tuple_value },
    fn() { list_value },
    fn() { function_value },
  ) as "function captures"

  external
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            ),
            PackageSource::new(
                "host_support",
                Vec::<&str>::new(),
                Vec::<ModuleSource>::new(),
            ),
        ],
        HostProviderSet::with_providers([scalars], [provider])
            .expect("host modules should be unique"),
    )
    .expect("external source should compile");
    let plan = plan_host_program(typed).expect("external source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("external execution should seal");
    let mut echoes = Vec::new();
    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut echoes)
        .expect("external source should execute");
    let Value::External(counter) = returned else {
        panic!("main should return an external value");
    };

    let expected_values = r#"#(1, 1.0, "one", <<1>>, 'A', Boxed(1), Counter(8), True, Nil, #(1), [1], //fn(a) { ... })"#;
    let expected_lists = r#"#([1], [1.0], ["one"], [<<1>>], ['A'], [Boxed(1)], [Counter(8)], [True], [Nil], [#(1)], [[1]], [//fn(a) { ... }])"#;
    let expected_functions = r#"#(//fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn() { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn(a) { ... }, //fn() { ... }, //fn() { ... }, //fn() { ... })"#;

    assert_eq!(echoes.len(), 6);
    assert_eq!(echoes[0].value().inspect().to_string(), expected_values);
    assert_eq!(echoes[1].value().inspect().to_string(), expected_lists);
    assert_eq!(echoes[2].value().inspect().to_string(), expected_functions,);
    let Value::Tuple(captures) = echoes[3].value() else {
        panic!("value captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 12);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    let Value::Tuple(captures) = echoes[4].value() else {
        panic!("list captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 12);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    let Value::Tuple(captures) = echoes[5].value() else {
        panic!("function captures echo should contain a tuple");
    };
    assert_eq!(captures.len(), 13);
    assert!(
        captures
            .iter()
            .all(|capture| capture.inspect().to_string() == "//fn() { ... }"),
    );
    assert_eq!(counter.inspection(), "Counter(8)");
}

#[test]
fn executes_external_values_across_expression_and_function_table_boundaries() {
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

const make_counter = new_counter

fn from_bool(first: Counter, second: Counter, value: Bool) -> Counter {
  case value {
    True -> first
    False -> second
  }
}

fn from_int(first: Counter, second: Counter, value: Int) -> Counter {
  case value {
    1 -> first
    _ -> second
  }
}

fn from_float(first: Counter, second: Counter, value: Float) -> Counter {
  case value {
    1.5 -> first
    _ -> second
  }
}

fn from_string(first: Counter, second: Counter, value: String) -> Counter {
  case value {
    "first" -> first
    _ -> second
  }
}

fn from_tuple(value: Counter) -> Counter {
  #(value, 1).0
}

fn from_custom(value: Counter) -> Counter {
  Wrapped(value).value
}

fn from_list(value: Counter) -> Counter {
  let assert [head] = [value]
  head
}

fn from_function(value: Counter) -> Counter {
  let read = fn() { value }
  read()
}

fn from_block(value: Counter) -> Counter {
  {
    let returned = value
    returned
  }
}

fn with_unreached_panic(value: Counter) -> Counter {
  case True {
    True -> value
    False -> panic
  }
}

fn external_value() -> Counter {
  new_counter(11)
}

fn external_list() -> List(Counter) {
  [external_value()]
}

fn external_function() -> fn() -> Counter {
  external_value
}

fn nested_external_function() -> fn() -> fn() -> Counter {
  external_function
}

fn identity(value: value) -> value {
  value
}

fn transform_value(
  value: value,
  fallback: value,
  transform: fn(value) -> value,
) -> value {
  case True {
    True -> transform(value)
    False -> fallback
  }
}

fn transform_list(values: List(value)) -> List(value) {
  values
}

fn prepend(value: value, values: List(value)) -> List(value) {
  [value, ..values]
}

fn drop_generic(values: List(value)) -> List(value) {
  let assert [_, ..remaining] = values
  remaining
}

pub fn main() -> Counter {
  let first = make_counter(11)
  let second = new_counter(12)
  let selected = case first {
    value -> value
  }
  let discarded = case selected {
    _ -> selected
  }
  let aliased = case discarded {
    value as alias if value == alias -> alias
    _ -> second
  }
  let assert [listed] = external_list()
  let read = external_function()
  let nested = nested_external_function()
  let transformed = transform_value(aliased, second, identity)
  let assert [transformed] = transform_list([transformed])
  let assert [transformed] = prepend(transformed, [])
  let assert [transformed] = drop_generic([second, transformed])
  let result = from_bool(
    from_int(
      from_float(
        from_string(
          from_tuple(
            from_custom(
              from_list(
                from_function(
                  from_block(with_unreached_panic(transformed)),
                ),
              ),
            ),
          ),
          second,
          "first",
        ),
        second,
        1.5,
      ),
      second,
      1,
    ),
    second,
    True,
  )
  let assert True = result == listed
  let assert True = result == read()
  let assert True = result == nested()()
  result
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

    let returned = execution
        .run_main(&mut ExternalRunState::default(), &mut Vec::new())
        .expect("external source should execute");
    let Value::External(counter) = returned else {
        panic!("main should return an external value");
    };

    assert_eq!(counter.inspection(), "Counter(11)");
}
