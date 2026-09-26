use geam_core::embedding::{
    FunctionDeclaration, HostPreparation, NativeType, PreparedHostedModule,
};
use geam_core::{
    HostCallableSchema, HostCreatedFunction, HostCustomConstructorListEnd, HostCustomSchema,
    HostCustomType, HostDeclarations, HostFunctionDeclaration, HostFunctionType,
    HostProviderModuleDeclaration, HostReturns, HostType, HostTypeList, HostTypeListEnd,
    HostTypeParameter, ModuleSource, PackageSource,
};
use num_bigint::BigInt;
use std::marker::PhantomData;

pub(crate) type End = HostTypeListEnd;
pub(crate) type One<Type> = HostTypeList<Type, End>;
pub(crate) type T = HostTypeParameter<0>;

pub(crate) struct Constant<Type>(PhantomData<Type>);
impl<Type: HostType> HostCallableSchema for Constant<Type> {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/private";
    const NAME: &'static str = "constant";
    type Arguments = End;
    type Return = Type;
    type Captures = One<Type>;
    type Constructions = End;
    type Completion = HostReturns;
}

pub(crate) type U = HostTypeParameter<1>;
pub(crate) type Wrapped<Input, Output> = HostFunctionType<One<Input>, Output>;
pub(crate) struct Wrap<Input, Output>(PhantomData<(Input, Output)>);
impl<Input: HostType, Output: HostType> HostCallableSchema for Wrap<Input, Output> {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/private";
    const NAME: &'static str = "wrap";
    type Arguments = One<Input>;
    type Return = Output;
    type Captures = One<Wrapped<Input, Output>>;
    type Constructions = End;
    type Completion = HostReturns;
}
type WrapDeclaration =
    HostFunctionDeclaration<(Wrapped<T, U>,), Wrapped<T, U>, One<HostCreatedFunction<Wrap<T, U>>>>;
pub(crate) const WRAP: WrapDeclaration = HostFunctionDeclaration::new("wrap");

pub(crate) struct NeverSchema;
impl HostCustomSchema for NeverSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "library";
    const NAME: &'static str = "Never";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorListEnd;
}
pub(crate) type Never = HostCustomType<NeverSchema>;
pub(crate) type NeverView = <Never as NativeType>::Shape;

pub(crate) struct Stop<Type>(PhantomData<Type>);
impl<Type: HostType> HostCallableSchema for Stop<Type> {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/private";
    const NAME: &'static str = "stop";
    type Arguments = End;
    type Return = Type;
    type Captures = End;
    type Constructions = End;
    type Completion = HostReturns;
}
pub(crate) const MAKE_STOP: HostFunctionDeclaration<
    (),
    ConstantFunction<T>,
    One<HostCreatedFunction<Stop<T>>>,
> = HostFunctionDeclaration::new("make_stop");
pub(crate) const PRODUCE: HostFunctionDeclaration<(), T> = HostFunctionDeclaration::new("produce");

pub(crate) struct Add;
impl HostCallableSchema for Add {
    const PACKAGE: &'static str = "support";
    const MODULE: &'static str = "support/private";
    const NAME: &'static str = "add";
    type Arguments = One<BigInt>;
    type Return = BigInt;
    type Captures = One<BigInt>;
    type Constructions = End;
    type Completion = HostReturns;
}

pub(crate) type ConstantFunction<Type> = HostFunctionType<End, Type>;
pub(crate) type AddFunction = HostFunctionType<One<BigInt>, BigInt>;
pub(crate) const MAKE_CONSTANT: HostFunctionDeclaration<
    (T,),
    ConstantFunction<T>,
    One<HostCreatedFunction<Constant<T>>>,
> = HostFunctionDeclaration::new("make_constant");
pub(crate) const MAKE_ADDER: HostFunctionDeclaration<
    (BigInt,),
    AddFunction,
    One<HostCreatedFunction<Add>>,
> = HostFunctionDeclaration::new("make_adder");

pub(crate) fn packages() -> [PackageSource; 2] {
    [
        PackageSource::new(
            "support",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "support",
                "support.gleam",
                r#"
@external(erlang, "ffi", "make_constant")
pub fn make_constant(value: a) -> fn() -> a
@external(erlang, "ffi", "make_adder")
pub fn make_adder(value: Int) -> fn(Int) -> Int
@external(erlang, "ffi", "wrap")
pub fn wrap(callback: fn(a) -> b) -> fn(a) -> b
@external(erlang, "ffi", "make_stop")
pub fn make_stop() -> fn() -> a
@external(erlang, "ffi", "produce")
pub fn produce() -> a
"#,
            )],
        ),
        PackageSource::new(
            "application",
            ["support"],
            [ModuleSource::new(
                "library",
                "library.gleam",
                r#"
import support
pub type Never
pub fn call_never(callback: fn() -> Never) { let _ = callback() 42 }
fn apply(callback, value) { callback(value) }
pub fn make_native(offset: Int) -> fn(Int) -> Int { support.make_adder(offset) }
pub fn keep(adjust: fn(Int) -> Int) -> fn(Int) -> Int { adjust }
pub fn calculate(value: Int, adjust: fn(Int) -> Int) -> Int { adjust(value) }
pub fn function_list(items: List(fn(Int) -> Int)) -> List(fn(Int) -> Int) { items }
pub fn container(adjust: fn(Int) -> Int) -> #(fn(Int) -> Int, Result(fn(Int) -> Int, Nil)) { #(adjust, Ok(adjust)) }
pub fn maker() -> fn(Int) -> fn(Int) -> Int { fn(offset) { support.make_adder(offset) } }
pub fn result_function() -> fn(Int) -> Result(Int, Nil) { fn(value) { Ok(value) } }
pub fn picker() -> fn(List(Result(Int, Nil))) -> Int {
    fn(items) { case items { [Ok(value)] -> value _ -> 0 } }
}
pub fn fail() {
    let stopped = support.make_stop()
    echo "before callable"
    let _ = stopped()
    echo "unreachable"
    42
}
pub fn producer() {
    echo "before producer"
    let _ = support.produce()
    echo "unreachable"
    42
}
pub fn run() {
    let add = support.make_adder(40)
    let constant = support.make_constant(2)
    apply(add, constant())
}
pub fn check() {
    let a = support.make_constant(True)
    let alias = a
    let b = support.make_constant(True)
    let list = support.make_constant([42])
    let is_answer = support.wrap(fn(value) { value == 42 })
    let increment = support.wrap(support.wrap(fn(value) { value + 1 }))
    let nested = support.wrap(fn(value) { support.make_constant(value) })
    a() && a == alias && a != b && list() == [42] && is_answer(increment(41)) && nested(42)() == 42
}
"#,
            )],
        ),
    ]
}

fn preparation() -> HostPreparation {
    let declarations = HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
        "support", "support",
    )
    .unwrap()
    .with_function(MAKE_CONSTANT)
    .unwrap()
    .with_function(MAKE_ADDER)
    .unwrap()
    .with_function(WRAP)
    .unwrap()
    .with_function(MAKE_STOP)
    .unwrap()
    .with_function(PRODUCE)
    .unwrap()
    .with_callable::<Stop<T>>()
    .unwrap()
    .with_callable::<Constant<T>>()
    .unwrap()
    .with_callable::<Add>()
    .unwrap()
    .with_callable::<Wrap<T, U>>()
    .unwrap()])
    .unwrap();
    let declared = geam_core::compile_declared_host_program(
        "application",
        "library",
        packages(),
        declarations,
    )
    .unwrap();
    HostPreparation::new(declared).unwrap()
}

pub(crate) fn prepare() -> PreparedHostedModule {
    let mut preparation = preparation()
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    preparation
        .function(FunctionDeclaration::<(), bool>::new("check"))
        .unwrap();
    preparation
        .function(FunctionDeclaration::<(), BigInt>::new("fail"))
        .unwrap();
    preparation
        .function(FunctionDeclaration::<(), BigInt>::new("producer"))
        .unwrap();
    preparation.prepare().unwrap()
}

pub(crate) fn prepare_scoped() -> PreparedHostedModule {
    use geam_core::embedding::{CallableType, List};
    type Adjust = CallableType<(BigInt,), BigInt>;
    let mut bindings = preparation()
        .function(FunctionDeclaration::<(BigInt,), Adjust>::new("make_native"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(Adjust,), Adjust>::new("keep"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, Adjust), BigInt>::new(
            "calculate",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<Adjust>,), List<Adjust>>::new(
            "function_list",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(Adjust,), (Adjust, Result<Adjust, ()>)>::new("container"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), CallableType<(BigInt,), Adjust>>::new("maker"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (),
            CallableType<(List<Result<BigInt, ()>>,), BigInt>,
        >::new("picker"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(CallableType<(), NeverView>,), BigInt>::new("call_never"))
        .unwrap();
    bindings.callable::<Stop<Never>>().unwrap();
    bindings.callable::<Add>().unwrap();
    bindings.callable::<Constant<bool>>().unwrap();
    bindings.callable::<Wrap<BigInt, BigInt>>().unwrap();
    bindings.prepare().unwrap()
}

pub(crate) fn prepare_native_views() -> PreparedHostedModule {
    use geam_core::embedding::CallableType;
    use geam_core::provider::ProviderResult;
    type Outcome = Result<BigInt, ()>;
    type Callback = CallableType<(BigInt,), Outcome>;
    let mut bindings = preparation()
        .function(FunctionDeclaration::<(), Callback>::new("result_function"))
        .unwrap();
    bindings
        .callable::<Constant<ProviderResult<BigInt, ()>>>()
        .unwrap();
    bindings
        .callable_as::<Constant<ProviderResult<BigInt, ()>>, (), Outcome, (Outcome, ())>()
        .unwrap();
    bindings.callable_as::<Wrap<BigInt, ProviderResult<BigInt, ()>>, (BigInt,), Outcome, (Callback, ())>().unwrap();
    bindings.prepare().unwrap()
}
