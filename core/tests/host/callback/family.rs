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

type Owned<Type> =
    geam_core::provider::Value<Type, geam_core::provider::ProviderValueContext<Type>>;

fn invoke_float<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, f64>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, f64>,
) -> Result<geam_core::HostCallContinuation<'call, f64>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_string<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, EcoString>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, EcoString>,
) -> Result<geam_core::HostCallContinuation<'call, EcoString>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_bit_array<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BitArrayValue>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, BitArrayValue>,
) -> Result<geam_core::HostCallContinuation<'call, BitArrayValue>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_utf_codepoint<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, char>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, char>,
) -> Result<geam_core::HostCallContinuation<'call, char>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_bool<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, bool>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, bool>,
) -> Result<geam_core::HostCallContinuation<'call, bool>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_nil<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, ()>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, ()>,
) -> Result<geam_core::HostCallContinuation<'call, ()>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            function
                .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(()))
            }))
        })
    }))
}

fn invoke_tuple<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, Pair>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, Pair>,
) -> Result<geam_core::HostCallContinuation<'call, Pair>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(
                    &context,
                    |_, _| (),
                    |call, _, value| Ok(Owned::<Pair>::from_host(&call, value)),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |mut call, _| {
                let value = value.into_host(&mut call);
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_list<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, IntList>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, IntList>,
) -> Result<geam_core::HostCallContinuation<'call, IntList>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(
                    &context,
                    |_, _| (),
                    |call, _, value| Ok(Owned::<IntList>::from_host(&call, value)),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |mut call, _| {
                let value = value.into_host(&mut call);
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_custom<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, Marker>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, Marker>,
) -> Result<geam_core::HostCallContinuation<'call, Marker>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(
                    &context,
                    |_, _| (),
                    |call, _, value| Ok(Owned::<Marker>::from_host(&call, value)),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |mut call, _| {
                let value = value.into_host(&mut call);
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_constructor<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, Marker>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, IntArguments, Marker>,
) -> Result<geam_core::HostCallContinuation<'call, Marker>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(
                    &context,
                    |_, _| (BigInt::from(11), ()),
                    |call, _, value| Ok(Owned::<Marker>::from_host(&call, value)),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |mut call, _| {
                let value = value.into_host(&mut call);
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_function<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, IntCallable>,
) -> Result<geam_core::HostCallContinuation<'call, BigInt>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let returned = function
                .invoke(
                    &context,
                    |_, _| (),
                    |call, constructions, value| Ok(call.owned_callable(value, &constructions)),
                )
                .await?;
            let value = returned
                .invoke(
                    &context,
                    |_, _| (BigInt::from(41), ()),
                    |_, _, value| Ok(value),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn invoke_with_function_argument<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, FunctionArgumentArguments, BigInt>,
    argument: HostCallable<'call, IntArguments, BigInt>,
) -> Result<geam_core::HostCallContinuation<'call, BigInt>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    let argument = Owned::<IntCallable>::from_host(&call, argument);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let value = function
                .invoke(
                    &context,
                    move |mut call, _| (argument.into_host(&mut call), ()),
                    |_, _, value| Ok(value),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn wrap_callable<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, IntCallableList>,
    function: HostCallable<'call, IntArguments, BigInt>,
) -> Result<HostCallCompletion<'call, IntCallableList>, HostCallError> {
    Ok(call.return_list([function]))
}

fn invoke_first<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    functions: HostList<'call, IntCallable>,
    value: BigInt,
) -> Result<geam_core::HostCallContinuation<'call, BigInt>, HostCallError> {
    let function = call
        .list_item(functions, 0)
        .ok_or_else(|| geam_core::HostFailure::new("callback list should contain one function"))?;
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let returned = function
                .invoke(&context, move |_, _| (value, ()), |_, _, value| Ok(value))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(returned))
            }))
        })
    }))
}

#[test]
fn invokes_every_successful_callback_return_family() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, f64>,), f64, geam_core::HostTypeListEnd, _>(
            "invoke_float",
            invoke_float,
        )
        .expect("Float callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, EcoString>,), EcoString, geam_core::HostTypeListEnd, _>(
            "invoke_string",
            invoke_string,
        )
        .expect("String callback should register")
        .with_resumable_function::<
            Provider,
            (HostFunctionType<NoArguments, BitArrayValue>,),
            BitArrayValue,
            geam_core::HostTypeListEnd, _,
        >("invoke_bit_array", invoke_bit_array)
        .expect("BitArray callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, char>,), char, geam_core::HostTypeListEnd, _>(
            "invoke_utf_codepoint",
            invoke_utf_codepoint,
        )
        .expect("UtfCodepoint callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, bool>,), bool, geam_core::HostTypeListEnd, _>(
            "invoke_bool",
            invoke_bool,
        )
        .expect("Bool callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, ()>,), (), geam_core::HostTypeListEnd, _>(
            "invoke_nil",
            invoke_nil,
        )
        .expect("Nil callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, Pair>,), Pair, geam_core::HostTypeListEnd, _>(
            "invoke_tuple",
            invoke_tuple,
        )
        .expect("tuple callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, IntList>,), IntList, geam_core::HostTypeListEnd, _>(
            "invoke_list",
            invoke_list,
        )
        .expect("list callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<NoArguments, Marker>,), Marker, geam_core::HostTypeListEnd, _>(
            "invoke_custom",
            invoke_custom,
        )
        .expect("custom callback should register")
        .with_resumable_function::<Provider, (HostFunctionType<IntArguments, Marker>,), Marker, geam_core::HostTypeListEnd, _>(
            "invoke_constructor",
            invoke_constructor,
        )
        .expect("constructor callback should register")
        .with_resumable_function::<
            Provider,
            (HostFunctionType<NoArguments, IntCallable>,),
            BigInt,
            geam_core::HostTypeListEnd, _,
        >("invoke_function", invoke_function)
        .expect("function callback should register")
        .with_resumable_function::<
            Provider,
            (FunctionArgumentCallable, IntCallable),
            BigInt,
            geam_core::HostTypeListEnd, _,
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
    let mut execution =
        HostedExecution::try_from_module_plan(plan).expect("callback family execution should seal");

    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
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
        .with_resumable_function::<Provider, (IntCallableList, BigInt), BigInt, geam_core::HostTypeListEnd, _>(
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
    let mut execution =
        HostedExecution::try_from_module_plan(plan).expect("callback list execution should seal");

    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
            .expect("callback list should preserve invocation and identity")
            .inspect()
            .to_string(),
        "#(42, 42, True)",
    );
}

type NeverReturn = HostTypeParameter<0>;
type NeverCallable = HostFunctionType<NoArguments, NeverReturn>;

fn invoke_never<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    constructions: geam_core::HostConstructions<'call, HostTypeListEnd>,
    function: HostCallable<'call, NoArguments, NeverReturn>,
) -> Result<geam_core::HostCallContinuation<'call, BigInt>, HostCallError> {
    let function = call.owned_callable(function, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            function
                .invoke(&context, |_, _| (), |_, _, _| Ok(()))
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(0.into()))
            }))
        })
    }))
}

#[test]
fn preserves_a_nested_failure_from_a_never_callback() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_resumable_function::<Provider, (NeverCallable,), BigInt, geam_core::HostTypeListEnd, _>("invoke_never", invoke_never)
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
    let mut execution =
        HostedExecution::try_from_module_plan(plan).expect("Never callback execution should seal");
    let error = crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new())
        .expect_err("nested Never callback should preserve its panic");
    let ExecutionError::Panic(error) = error else {
        panic!("nested Never callback should remain a source panic");
    };

    assert_eq!(error.kind(), PanicKind::Panic);
    assert_eq!(error.site().function(), "stop");
}
