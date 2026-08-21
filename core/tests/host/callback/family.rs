use ecow::EcoString;
use geam_core::{
    BitArrayValue, ExecutionError, HostCall, HostCallCompletion, HostCallError, HostCallable,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
    HostFunctionType, HostList, HostListType, HostProvider, HostProviderModule, HostProviderSet,
    HostTupleType, HostTypeList, HostTypeListEnd, HostTypeParameter, HostedExecution, ModuleSource,
    PackageSource, PanicKind, StatelessHostProfile, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

struct Provider;

impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

struct MarkerSchema;

struct MarkerField;

impl HostCustomField for MarkerField {
    const LABEL: Option<&'static str> = None;

    type Type = BigInt;
}

struct MarkerDefinition;

impl HostCustomConstructorDefinition for MarkerDefinition {
    const NAME: &'static str = "Marker";

    type Fields = HostCustomFieldList<MarkerField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for MarkerSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Marker";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<MarkerDefinition, HostCustomConstructorListEnd>;
}

type NoArguments = HostTypeListEnd;
type IntArguments = HostTypeList<BigInt, HostTypeListEnd>;
type PairElements = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
type Pair = HostTupleType<PairElements>;
type IntList = HostListType<BigInt>;
type Marker = HostCustomType<MarkerSchema>;
type IntCallable = HostFunctionType<IntArguments, BigInt>;
type IntCallableList = HostListType<IntCallable>;
type FunctionArgumentArguments = HostTypeList<IntCallable, HostTypeListEnd>;
type FunctionArgumentCallable = HostFunctionType<FunctionArgumentArguments, BigInt>;

fn invoke_float<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, f64>,
    function: HostCallable<'call, NoArguments, f64>,
) -> Result<HostCallCompletion<'call, f64>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_string<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, EcoString>,
    function: HostCallable<'call, NoArguments, EcoString>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_bit_array<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BitArrayValue>,
    function: HostCallable<'call, NoArguments, BitArrayValue>,
) -> Result<HostCallCompletion<'call, BitArrayValue>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_utf_codepoint<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, char>,
    function: HostCallable<'call, NoArguments, char>,
) -> Result<HostCallCompletion<'call, char>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_bool<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, bool>,
    function: HostCallable<'call, NoArguments, bool>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_nil<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, ()>,
    function: HostCallable<'call, NoArguments, ()>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    call.invoke(function, ())?;
    Ok(call.return_value(()))
}

fn invoke_tuple<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, Pair>,
    function: HostCallable<'call, NoArguments, Pair>,
) -> Result<HostCallCompletion<'call, Pair>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_list<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, IntList>,
    function: HostCallable<'call, NoArguments, IntList>,
) -> Result<HostCallCompletion<'call, IntList>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_custom<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, Marker>,
    function: HostCallable<'call, NoArguments, Marker>,
) -> Result<HostCallCompletion<'call, Marker>, HostCallError> {
    let value = call.invoke(function, ())?;
    Ok(call.return_value(value))
}

fn invoke_constructor<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, Marker>,
    function: HostCallable<'call, IntArguments, Marker>,
) -> Result<HostCallCompletion<'call, Marker>, HostCallError> {
    let value = call.invoke(function, (BigInt::from(11), ()))?;
    Ok(call.return_value(value))
}

fn invoke_function<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    function: HostCallable<'call, NoArguments, IntCallable>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let returned = call.invoke(function, ())?;
    let value = call.invoke(returned, (BigInt::from(41), ()))?;
    Ok(call.return_value(value))
}

fn invoke_with_function_argument<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    function: HostCallable<'call, FunctionArgumentArguments, BigInt>,
    argument: HostCallable<'call, IntArguments, BigInt>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let value = call.invoke(function, (argument, ()))?;
    Ok(call.return_value(value))
}

fn wrap_callable<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, IntCallableList>,
    function: HostCallable<'call, IntArguments, BigInt>,
) -> Result<HostCallCompletion<'call, IntCallableList>, HostCallError> {
    Ok(call.return_list([function]))
}

fn invoke_first<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    functions: HostList<'call, IntCallable>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let function = call
        .list_item(functions, 0)
        .ok_or_else(|| geam_core::HostFailure::new("callback list should contain one function"))?;
    let returned = call.invoke(function, (value, ()))?;
    Ok(call.return_value(returned))
}

#[test]
fn invokes_every_successful_callback_return_family() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, f64>,), f64, _>(
            "invoke_float",
            invoke_float,
        )
        .expect("Float callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, EcoString>,), EcoString, _>(
            "invoke_string",
            invoke_string,
        )
        .expect("String callback should register")
        .with_scoped_function::<
            Provider,
            (HostFunctionType<NoArguments, BitArrayValue>,),
            BitArrayValue,
            _,
        >("invoke_bit_array", invoke_bit_array)
        .expect("BitArray callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, char>,), char, _>(
            "invoke_utf_codepoint",
            invoke_utf_codepoint,
        )
        .expect("UtfCodepoint callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, bool>,), bool, _>(
            "invoke_bool",
            invoke_bool,
        )
        .expect("Bool callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, ()>,), (), _>(
            "invoke_nil",
            invoke_nil,
        )
        .expect("Nil callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, Pair>,), Pair, _>(
            "invoke_tuple",
            invoke_tuple,
        )
        .expect("tuple callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, IntList>,), IntList, _>(
            "invoke_list",
            invoke_list,
        )
        .expect("list callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<NoArguments, Marker>,), Marker, _>(
            "invoke_custom",
            invoke_custom,
        )
        .expect("custom callback should register")
        .with_scoped_function::<Provider, (HostFunctionType<IntArguments, Marker>,), Marker, _>(
            "invoke_constructor",
            invoke_constructor,
        )
        .expect("constructor callback should register")
        .with_scoped_function::<
            Provider,
            (HostFunctionType<NoArguments, IntCallable>,),
            BigInt,
            _,
        >("invoke_function", invoke_function)
        .expect("function callback should register")
        .with_scoped_function::<
            Provider,
            (FunctionArgumentCallable, IntCallable),
            BigInt,
            _,
        >(
            "invoke_with_function_argument",
            invoke_with_function_argument,
        )
        .expect("function-valued callback argument should register");
    let source = r#"
pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "invoke_float")
fn invoke_float(function: fn() -> Float) -> Float

@external(erlang, "host", "invoke_string")
fn invoke_string(function: fn() -> String) -> String

@external(erlang, "host", "invoke_bit_array")
fn invoke_bit_array(function: fn() -> BitArray) -> BitArray

@external(erlang, "host", "invoke_utf_codepoint")
fn invoke_utf_codepoint(function: fn() -> UtfCodepoint) -> UtfCodepoint

@external(erlang, "host", "invoke_bool")
fn invoke_bool(function: fn() -> Bool) -> Bool

@external(erlang, "host", "invoke_nil")
fn invoke_nil(function: fn() -> Nil) -> Nil

@external(erlang, "host", "invoke_tuple")
fn invoke_tuple(function: fn() -> #(Int, Bool)) -> #(Int, Bool)

@external(erlang, "host", "invoke_list")
fn invoke_list(function: fn() -> List(Int)) -> List(Int)

@external(erlang, "host", "invoke_custom")
fn invoke_custom(function: fn() -> Marker) -> Marker

@external(erlang, "host", "invoke_constructor")
fn invoke_constructor(function: fn(Int) -> Marker) -> Marker

@external(erlang, "host", "invoke_function")
fn invoke_function(function: fn() -> fn(Int) -> Int) -> Int

@external(erlang, "host", "invoke_with_function_argument")
fn invoke_with_function_argument(
  function: fn(fn(Int) -> Int) -> Int,
  argument: fn(Int) -> Int,
) -> Int

fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(7, False) }
fn list_value() { [8, 9] }
fn custom_value() { Marker(10) }
fn increment(value: Int) { value + 1 }
fn function_value() { increment }
fn apply(function: fn(Int) -> Int) { function(41) }

pub fn main() {
  #(
    invoke_float(float_value),
    invoke_string(string_value),
    invoke_bit_array(bit_array_value),
    invoke_utf_codepoint(utf_codepoint_value),
    invoke_bool(bool_value),
    invoke_nil(nil_value),
    invoke_tuple(tuple_value),
    invoke_list(list_value),
    invoke_custom(custom_value),
    invoke_constructor(Marker),
    invoke_function(function_value),
    invoke_with_function_argument(apply, increment),
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
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("callback family source should compile");
    let plan = plan_host_program(typed).expect("callback family source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("callback family execution should seal");

    assert_eq!(
        execution
            .run_main(&mut (), &mut Vec::new())
            .expect("every successful callback family should execute")
            .inspect()
            .to_string(),
        r#"#(1.5, "text", <<1>>, 'A', True, Nil, #(7, False), [8, 9], Marker(10), Marker(11), 42, 42)"#,
    );
}

#[test]
fn returns_and_invokes_callables_nested_in_lists() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (IntCallable,), IntCallableList, _>(
            "wrap_callable",
            wrap_callable,
        )
        .expect("callback list return should register")
        .with_scoped_function::<Provider, (IntCallableList, BigInt), BigInt, _>(
            "invoke_first",
            invoke_first,
        )
        .expect("callback list invocation should register");
    let source = r#"
@external(erlang, "host", "wrap_callable")
fn wrap_callable(function: fn(Int) -> Int) -> List(fn(Int) -> Int)

@external(erlang, "host", "invoke_first")
fn invoke_first(functions: List(fn(Int) -> Int), value: Int) -> Int

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  let functions = wrap_callable(increment)
  let assert [wrapped] = functions
  #(invoke_first(functions, 41), wrapped(41), wrapped == increment)
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
    .expect("callback list source should compile");
    let plan = plan_host_program(typed).expect("callback list source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("callback list execution should seal");

    assert_eq!(
        execution
            .run_main(&mut (), &mut Vec::new())
            .expect("callback list should preserve invocation and identity")
            .inspect()
            .to_string(),
        "#(42, 42, True)",
    );
}

type NeverReturn = HostTypeParameter<0>;
type NeverCallable = HostFunctionType<NoArguments, NeverReturn>;

fn invoke_never<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    function: HostCallable<'call, NoArguments, NeverReturn>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let _ = call.invoke(function, ())?;
    Ok(call.return_value(0.into()))
}

#[test]
fn preserves_a_nested_failure_from_a_never_callback() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (NeverCallable,), BigInt, _>("invoke_never", invoke_never)
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
        .run_main(&mut (), &mut Vec::new())
        .expect_err("nested Never callback should preserve its panic");
    let ExecutionError::Panic(error) = error else {
        panic!("nested Never callback should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}
