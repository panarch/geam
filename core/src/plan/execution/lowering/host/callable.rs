use super::super::LoweringContext;
use super::super::function;
use super::super::local::{self, ParameterPrefix};
use super::super::specialization::{
    Representability, SpecializationKey, SpecializedValueShape, ValueInhabitation,
};
use crate::host::RegisteredHostConstructions;
use crate::plan::HostFunctionTemplate;
use crate::plan::execution::host::{HostCallableConstruction, HostSpecializationError};

pub(super) fn seal(
    template: &HostFunctionTemplate,
    constructions: &RegisteredHostConstructions,
    key: &SpecializationKey,
    context: &mut LoweringContext,
) -> Result<Vec<HostCallableConstruction>, HostSpecializationError> {
    template
        .callable_constructions()
        .iter()
        .zip(constructions.callables())
        .map(|(instantiation, construction)| {
            seal_construction(
                template,
                construction,
                instantiation,
                key.substitution(),
                context,
            )
        })
        .collect()
}

pub(super) fn seal_construction(
    template: &HostFunctionTemplate,
    construction: &crate::host::RegisteredCallableConstruction,
    instantiation: &crate::plan::FunctionInstantiation,
    substitution: &super::super::specialization::SpecializedTypeSubstitution,
    context: &mut LoweringContext,
) -> Result<HostCallableConstruction, HostSpecializationError> {
    let (target_key, shape) = SpecializationKey::from_instantiation(instantiation, substitution);
    let Representability::Inhabited(parameters) = context.entry_templates[&target_key.template()]
        .stored_parameters(target_key.substitution(), &context.representations)
    else {
        return Err(HostSpecializationError::uninhabited_callback_arguments(
            template.package().clone(),
            template.module().into(),
            template.name().into(),
            crate::plan::FunctionShape::new(
                template
                    .parameters()
                    .iter()
                    .map(crate::host::HostTypeDescriptor::value_shape)
                    .collect(),
                template.return_type().value_shape(),
            )
            .substitute(&substitution.to_module_substitution())
            .type_(),
            shape.to_module_shape().type_(),
        ));
    };
    let mut capture_shapes = Vec::new();
    for capture in &construction.captures {
        let capture = SpecializedValueShape::instantiate(&capture.value_shape(), substitution);
        let ValueInhabitation::Inhabited(stored) = context.representations.inhabitation(&capture)
        else {
            return Err(HostSpecializationError::uninhabited_callable_capture(
                template,
                shape.to_module_shape().type_(),
                capture.to_module_shape().value_type(),
            ));
        };
        capture_shapes.push(stored);
    }
    let return_ = context.representations.inhabitation(shape.return_());
    let family = match &return_ {
        ValueInhabitation::Inhabited(stored) => {
            function::stored_function_table_family(stored, &context.representations)
        }
        ValueInhabitation::Uninhabited(_) => function::FunctionTableFamily::Never,
    };
    let reserved =
        context.reserve_provisional_specialization(target_key, family, parameters.clone());
    let target = match return_ {
        ValueInhabitation::Inhabited(stored) => function::function_id(
            &stored,
            reserved.index,
            &mut context.types,
            &context.representations,
        ),
        ValueInhabitation::Uninhabited(_) => {
            crate::plan::execution::function::RuntimeFunctionId::Core(
                crate::plan::execution::function::CoreRuntimeFunctionId::Never(
                    crate::plan::execution::function::NeverFunctionId(reserved.index),
                ),
            )
        }
    };
    let mut prefix = ParameterPrefix::default();
    let parameters = local::parameter_slots(&parameters, &mut prefix, context).into();
    let captures = local::parameter_slots(&capture_shapes, &mut prefix, context).into();
    Ok(HostCallableConstruction {
        target,
        type_: context.lower_concrete_function_type(&shape),
        parameters,
        captures,
    })
}

#[cfg(test)]
mod tests {
    use crate::embedding::{FunctionDeclaration, HostPreparation, HostedModuleBuilder};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallableSchema, HostCaptures,
        HostConstructions, HostCreatedFunction, HostDeclarations, HostDiverges, HostFailure,
        HostFunctionDeclaration, HostFunctionType, HostProvider, HostProviderModule,
        HostProviderModuleDeclaration, HostProviderSet, HostReturns, HostType, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, HostTypeParameter, ModuleSource, PackageSource,
        StatelessHostProfile,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;
    use std::marker::PhantomData;

    type End = HostTypeListEnd;
    type One<T> = HostTypeList<T, End>;
    type T = HostTypeParameter<0>;
    type Thunk<Return> = HostFunctionType<End, Return>;

    struct Abort<Return>(PhantomData<Return>);
    impl<Return: HostType> HostCallableSchema for Abort<Return> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "abort";
        type Arguments = End;
        type Return = Return;
        type Captures = One<BigInt>;
        type Constructions = End;
        type Completion = HostDiverges;
    }

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    const MAKE_ABORT: HostFunctionDeclaration<
        (BigInt,),
        Thunk<T>,
        One<HostCreatedFunction<Abort<T>>>,
    > = HostFunctionDeclaration::new("make_abort");

    fn make_abort<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk<T>>,
        constructions: HostConstructions<'call, One<HostCreatedFunction<Abort<T>>>>,
        code: BigInt,
    ) -> Result<HostCallCompletion<'call, Thunk<T>>, HostCallError> {
        let function = call.construct_function(constructions.at::<HostTypeIndex0>(), (code, ()));
        Ok(call.return_value(function))
    }

    fn abort<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, T>,
        captures: HostCaptures<'call, One<BigInt>>,
        _: HostConstructions<'call, End>,
    ) -> Result<Infallible, HostCallError> {
        assert_eq!(call.state(), &mut ());
        let (code, ()) = call.captures(captures);
        Err(HostFailure::new(format!("stopped {code}")).into())
    }

    #[test]
    fn native_divergence_preserves_uninhabited_and_phantom_return_specializations() {
        let source = r#"
pub type Never
pub type Phantom(a) { Phantom }
@external(erlang, "native", "make_abort")
fn make_abort(code: Int) -> fn() -> a
pub fn stop_int() -> Int {
  let stop: fn() -> Int = make_abort(11)
  stop()
}
pub fn stop_never() -> Int {
  let stop: fn() -> Never = make_abort(22)
  let _ = stop()
  0
}
pub fn stop_phantom() -> Int {
  let stop: fn() -> Phantom(Never) = make_abort(33)
  let _ = stop()
  0
}
"#;
        let packages = || {
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )]
        };
        let declarations = HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
            "application",
            "library",
        )
        .unwrap()
        .with_function(MAKE_ABORT)
        .unwrap()])
        .unwrap()
        .with_callable::<Abort<T>>()
        .unwrap();
        let declared = crate::compile_declared_host_program(
            "application",
            "library",
            packages(),
            declarations,
        )
        .unwrap();
        let mut preparation = HostPreparation::new(declared)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("stop_int"))
            .unwrap();
        preparation
            .function(FunctionDeclaration::<(), BigInt>::new("stop_never"))
            .unwrap();
        preparation
            .function(FunctionDeclaration::<(), BigInt>::new("stop_phantom"))
            .unwrap();
        let declared = preparation.prepare().unwrap().emit_rust();

        let bind = || {
            let providers = HostProviderSet::from_providers([HostProviderModule::new(
                "application",
                "library",
            )
            .unwrap()
            .with_declared_function::<Provider, _, _, _, _>(MAKE_ABORT, make_abort)
            .unwrap()])
            .unwrap()
            .with_callable::<Provider, Abort<T>, (), _>(abort)
            .unwrap();
            let program =
                crate::compile_typed_host_program("application", "library", packages(), providers)
                    .unwrap();
            let (mut bindings, int) = HostedModuleBuilder::new(program)
                .unwrap()
                .function(FunctionDeclaration::<(), BigInt>::new("stop_int"))
                .unwrap();
            let never = bindings
                .function(FunctionDeclaration::<(), BigInt>::new("stop_never"))
                .unwrap();
            let phantom = bindings
                .function(FunctionDeclaration::<(), BigInt>::new("stop_phantom"))
                .unwrap();
            (bindings, [int, never, phantom])
        };
        assert_eq!(declared, bind().0.prepare().unwrap().emit_rust());
        let (bindings, functions) = bind();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                for (function, code) in functions.iter().zip([11, 22, 33]) {
                    assert_eq!(
                        scope.call(function, ()).await.unwrap_err().to_string(),
                        format!("host function application::library.abort failed: stopped {code}")
                    );
                }
            }),
        )
        .unwrap();
    }

    struct Constant<Return>(PhantomData<Return>);
    impl<Return: HostType> HostCallableSchema for Constant<Return> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "constant";
        type Arguments = End;
        type Return = Return;
        type Captures = One<Return>;
        type Constructions = End;
        type Completion = HostReturns;
    }

    #[test]
    fn one_generic_native_factory_preserves_recursive_source_return_families() {
        const MAKE: HostFunctionDeclaration<(T,), Thunk<T>, One<HostCreatedFunction<Constant<T>>>> =
            HostFunctionDeclaration::new("make");
        fn make<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk<T>>,
            constructions: HostConstructions<'call, One<HostCreatedFunction<Constant<T>>>>,
            value: crate::HostValue<'call, T>,
        ) -> Result<HostCallCompletion<'call, Thunk<T>>, HostCallError> {
            let callback =
                call.construct_function(constructions.at::<HostTypeIndex0>(), (value, ()));
            Ok(call.return_value(callback))
        }
        fn constant<'call>(
            call: HostCall<'call, StatelessHostProfile, Provider, T>,
            captures: HostCaptures<'call, One<T>>,
            _: HostConstructions<'call, End>,
        ) -> Result<HostCallCompletion<'call, T>, HostCallError> {
            let (value, ()) = call.captures(captures);
            Ok(call.return_value(value))
        }
        let source = r#"
pub type Never
pub type Boxed(a) { Boxed(a) }
pub type Phantom(a) { Phantom }
@external(erlang, "native", "make")
fn make(value: a) -> fn() -> a
fn checked(value: a, label: String) {
  let captured = make(value)
  let alias = captured
  let fresh = make(value)
  case captured == alias && captured != fresh && captured() == value
    && alias() == value && fresh() == value {
    True -> Nil
    False -> panic as label
  }
}
pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"x":utf8>>
  let never_argument: fn(Never) -> Int = fn(_) { 42 }
  let never_return: fn() -> Never = fn() { panic }
  let empty: List(Never) = []
  let phantom: Phantom(Never) = Phantom
  checked(42, "Int")
  checked(3.5, "Float")
  checked("retained", "String")
  checked(<<1, 2, 3>>, "BitArray")
  checked(codepoint, "UtfCodepoint")
  checked(True, "Bool")
  checked(Nil, "Nil")
  checked(#(42, "tuple"), "Tuple")
  checked(Boxed(42), "Custom")
  checked(phantom, "phantom Never")
  checked(Ok(Boxed(42)), "generic Result")
  checked(Error("reason"), "generic Result error")
  checked(empty, "List(Never)")
  checked([], "List(parameter)")
  checked([[]], "List(List(parameter))")
  checked([42], "List(Int)")
  checked([3.5], "List(Float)")
  checked(["retained"], "List(String)")
  checked([<<1, 2, 3>>], "List(BitArray)")
  checked([codepoint], "List(UtfCodepoint)")
  checked([True], "List(Bool)")
  checked([Nil], "List(Nil)")
  checked([#(42, "tuple")], "List(Tuple)")
  checked([Boxed(42)], "List(Custom)")
  checked([[42]], "List(List(Int))")
  checked([fn() { 42 }], "List(Function)")
  checked(never_argument, "opaque function with Never input")
  checked(never_return, "function returning Never")
  checked(fn() { 42 }, "function returning Int")
  checked(fn() { 3.5 }, "function returning Float")
  checked(fn() { "retained" }, "function returning String")
  checked(fn() { <<1, 2, 3>> }, "function returning BitArray")
  checked(fn() { codepoint }, "function returning UtfCodepoint")
  checked(fn() { True }, "function returning Bool")
  checked(fn() { Nil }, "function returning Nil")
  checked(fn() { #(42, "tuple") }, "function returning Tuple")
  checked(fn() { Boxed(42) }, "function returning Custom")
  checked(fn() { [] }, "function returning List(parameter)")
  checked(fn() { [[]] }, "function returning List(List(parameter))")
  checked(fn() { [42] }, "function returning List(Int)")
  checked(fn() { [3.5] }, "function returning List(Float)")
  checked(fn() { ["retained"] }, "function returning List(String)")
  checked(fn() { [<<1, 2, 3>>] }, "function returning List(BitArray)")
  checked(fn() { [codepoint] }, "function returning List(UtfCodepoint)")
  checked(fn() { [True] }, "function returning List(Bool)")
  checked(fn() { [Nil] }, "function returning List(Nil)")
  checked(fn() { [#(42, "tuple")] }, "function returning List(Tuple)")
  checked(fn() { [Boxed(42)] }, "function returning List(Custom)")
  checked(fn() { [[42]] }, "function returning List(List(Int))")
  checked(fn() { [fn() { 42 }] }, "function returning List(Function)")
  checked(fn() { fn() { 42 } }, "function returning Function")
  True
}
"#;
        let providers =
            HostProviderSet::from_providers([HostProviderModule::new("application", "library")
                .unwrap()
                .with_declared_function::<Provider, _, _, _, _>(MAKE, make)
                .unwrap()
                .with_callable::<Provider, Constant<T>, (), _>(constant)
                .unwrap()])
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )],
            providers,
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echoes = Vec::new();
        let result = crate::execution_fixture::run(&mut execution, &mut (), &mut echoes).unwrap();
        assert_eq!(result.inspect().to_string(), "True");
        assert!(echoes.is_empty());
    }

    #[test]
    fn native_factories_retain_external_and_work_values_through_mixed_function_families() {
        use crate::embedding::List;
        use crate::host::{HostExternal, HostExternalStore, HostFutureStore};
        use crate::work_fixture::{WorkComponent, WorkType};
        use std::cell::Cell;
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };

        struct RichProfile;
        struct State {
            calls: Cell<usize>,
            effects: Arc<AtomicUsize>,
            work: (),
        }
        #[derive(Default)]
        struct Stores {
            tokens: HostExternalStore<Cell<i32>>,
            work: HostFutureStore,
        }
        impl crate::HostProfile for RichProfile {
            type RunState = State;
            type ExternalStores = Stores;
            type ExecutionState = ();
        }
        impl crate::host::HostWorkProfile for RichProfile {
            type Work = WorkComponent;
        }
        impl crate::HostComponentProfile<WorkComponent> for RichProfile {
            fn component_stores(stores: &Stores) -> &HostFutureStore {
                &stores.work
            }
            fn component_state(state: &mut Self::RunState) -> &mut () {
                &mut state.work
            }
        }
        impl HostProvider<RichProfile> for Provider {
            type State = State;
            fn project(state: &mut State) -> &mut State {
                state
            }
        }
        struct Token;
        impl crate::HostExternalSchema for Token {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Token";
            const PARAMETER_COUNT: usize = 0;
        }
        impl crate::HostExternalBinding<RichProfile, Token> for Provider {
            type Storage = Self;
        }
        impl crate::HostExternalStorage<RichProfile, Token> for Provider {
            type Payload = Cell<i32>;
            fn store(stores: &Stores) -> &HostExternalStore<Self::Payload> {
                &stores.tokens
            }
            fn source_equal(
                _: &crate::HostExternalEquality<'_>,
                left: &Cell<i32>,
                right: &Cell<i32>,
            ) -> bool {
                left.get() == right.get()
            }
            fn source_hash(_: &crate::HostExternalHashing<'_>, value: &Cell<i32>) -> u64 {
                value.get() as u64
            }
            fn inspect(_: &crate::HostExternalInspection<'_>, _: &Cell<i32>) -> ecow::EcoString {
                "Token(7)".into()
            }
        }
        type Held = crate::HostExternalType<Token>;
        const MAKE: HostFunctionDeclaration<(T,), Thunk<T>, One<HostCreatedFunction<Constant<T>>>> =
            HostFunctionDeclaration::new("make");
        fn make<'call>(
            mut call: HostCall<'call, RichProfile, Provider, Thunk<T>>,
            constructions: HostConstructions<'call, One<HostCreatedFunction<Constant<T>>>>,
            value: crate::HostValue<'call, T>,
        ) -> Result<HostCallCompletion<'call, Thunk<T>>, HostCallError> {
            let callback =
                call.construct_function(constructions.at::<HostTypeIndex0>(), (value, ()));
            Ok(call.return_value(callback))
        }
        fn constant<'call>(
            call: HostCall<'call, RichProfile, Provider, T>,
            captures: HostCaptures<'call, One<T>>,
            _: HostConstructions<'call, End>,
        ) -> Result<HostCallCompletion<'call, T>, HostCallError> {
            let (value, ()) = call.captures(captures);
            Ok(call.return_value(value))
        }
        fn token<'call>(
            mut call: HostCall<'call, RichProfile, Provider, Held>,
        ) -> Result<HostCallCompletion<'call, Held>, HostCallError> {
            let token = call.create_external(Cell::new(7));
            Ok(call.return_value(token))
        }
        fn read<'call>(
            call: HostCall<'call, RichProfile, Provider, BigInt>,
            token: HostExternal<'call, Held>,
            original: HostExternal<'call, Held>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(
                call.source_hash::<Held>(token),
                call.source_hash::<Held>(original)
            );
            assert_eq!(call.inspect::<Held>(token), "Token(7)");
            let value = call.external_payload(token).get();
            Ok(call.return_value(value.into()))
        }
        fn touch<'call>(
            mut call: HostCall<'call, RichProfile, Provider, ()>,
        ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
            let state = call.state();
            assert!(std::ptr::eq(
                <RichProfile as crate::HostComponentProfile<WorkComponent>>::component_state(state),
                &state.work
            ));
            state.calls.set(state.calls.get() + 1);
            state.effects.fetch_add(1, Ordering::SeqCst);
            Ok(call.return_value(()))
        }
        let module = HostProviderModule::new("application", "library")
            .unwrap()
            .with_external_type::<Provider, Token>()
            .unwrap()
            .with_scoped_function::<Provider, (), Held, _>("token", token)
            .unwrap()
            .with_scoped_function::<Provider, (Held, Held), BigInt, _>("read", read)
            .unwrap()
            .with_scoped_function::<Provider, (), (), _>("touch", touch)
            .unwrap()
            .with_declared_function::<Provider, _, _, _, _>(MAKE, make)
            .unwrap()
            .with_callable::<Provider, Constant<T>, (), _>(constant)
            .unwrap();
        let providers = HostProviderSet::from_providers(
            WorkComponent::providers::<RichProfile>()
                .unwrap()
                .into_iter()
                .chain([module]),
        )
        .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
import fixture/work
pub type Token
@external(erlang, "native", "make") fn make(value: a) -> fn() -> a
@external(erlang, "native", "token") fn token() -> Token
@external(erlang, "native", "read") fn read(value: Token, original: Token) -> Int
@external(erlang, "native", "touch") fn touch() -> Nil
fn checked(value: a) {
  let captured = make(value)
  let alias = captured
  let fresh = make(value)
  let assert True = captured == alias && captured != fresh
    && captured() == value && alias() == value && fresh() == value
}
pub fn main() {
  let held = token()
  let operation = work.map(work.ready(21), fn(value) { touch() value * 2 })
  checked(held)
  checked(operation)
  checked([held])
  checked([operation])
  checked(fn() { held })
  checked(fn() { operation })
  checked(fn() { [held] })
  checked(fn() { [operation] })
  checked([fn() { held }])
  checked([fn() { operation }])
  checked(fn() { fn() { held } })
  checked(fn() { fn() { operation } })
  checked(#(held, [operation], fn() { held }))
  let external_factory = make(fn() { held })
  let list_factory = make(fn() { [operation] })
  #(read(external_factory()(), held), list_factory()())
}
"#,
                    )],
                ),
            ],
            providers,
        )
        .unwrap();
        let (bindings, main) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), (BigInt, List<WorkType<BigInt>>)>::new("main"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let effects = Arc::new(AtomicUsize::new(0));
        let mut state = State {
            calls: Cell::new(0),
            effects: Arc::clone(&effects),
            work: (),
        };
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let (value, work) = scope.call(&main, ()).await.unwrap();
                assert_eq!(value, BigInt::from(7));
                assert_eq!(effects.load(Ordering::SeqCst), 0);
                let operation = work.read_item(0, |work| work).unwrap();
                drop(work);
                let first = scope.observe(&operation).await.unwrap();
                let second = scope.observe(&operation).await.unwrap();
                first.read(|value| {
                    assert_eq!(value, &BigInt::from(42));
                    second.read(|alias| assert!(std::ptr::eq(value, alias)));
                });
            }),
        )
        .unwrap();
        assert_eq!(state.calls.get(), 1);
        assert_eq!(effects.load(Ordering::SeqCst), 1);
        assert!(echo.is_empty());
    }

    #[test]
    fn native_and_named_entries_reject_uninhabited_nested_callable_views() {
        struct NeverSchema;
        impl crate::HostCustomSchema for NeverSchema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Never";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = crate::HostCustomConstructorListEnd;
        }
        type Never = crate::HostCustomType<NeverSchema>;
        type NeverView = <Never as crate::embedding::NativeType>::Shape;
        type Callback = HostFunctionType<One<Never>, BigInt>;
        type CallbackView = crate::embedding::CallableType<(NeverView,), BigInt>;
        type NestedView = crate::embedding::CallableType<(), CallbackView>;

        fn constant<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, T>,
            captures: HostCaptures<'call, One<T>>,
            _: HostConstructions<'call, End>,
        ) -> Result<HostCallCompletion<'call, T>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            let (value, ()) = call.captures(captures);
            Ok(call.return_value(value))
        }

        let packages = || {
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Never
pub fn run() { 42 }
pub fn named() -> fn() -> fn(Never) -> Int { fn() { fn(_) { 1 } } }
"#,
                )],
            )]
        };
        let declarations = || {
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<Constant<T>>()
                .unwrap()
        };
        let prepare = || {
            HostPreparation::new(
                crate::compile_declared_host_program(
                    "application",
                    "library",
                    packages(),
                    declarations(),
                )
                .unwrap(),
            )
            .unwrap()
        };
        let mut native = prepare()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        native.callable::<Constant<Callback>>().unwrap();
        let declared_error = native.prepare().err().unwrap();
        let named_error = prepare()
            .function(FunctionDeclaration::<(), NestedView>::new("named"))
            .unwrap()
            .prepare()
            .err()
            .unwrap();
        let providers = || {
            HostProviderSet::new([])
                .unwrap()
                .with_callable::<Provider, Constant<T>, (), _>(constant)
                .unwrap()
        };
        let positive =
            crate::compile_typed_host_program("application", "library", packages(), providers())
                .unwrap();
        let (mut bindings, run) = HostedModuleBuilder::new(positive)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let constant = bindings.callable::<Constant<BigInt>>().unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                assert_eq!(scope.call(&run, ()).await.unwrap(), BigInt::from(42));
                let function = scope.construct(&constant, (BigInt::from(7), ())).unwrap();
                assert_eq!(scope.invoke(&function, ()).await.unwrap(), BigInt::from(7));
            }),
        )
        .unwrap();
        assert!(echo.is_empty());
        let program =
            crate::compile_typed_host_program("application", "library", packages(), providers())
                .unwrap();
        let (mut native, _) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        native.callable::<Constant<Callback>>().unwrap();
        let dynamic_error = native.seal().err().unwrap();

        assert_eq!(declared_error, dynamic_error);
        assert_eq!(dynamic_error.function(), "constant");
        assert_eq!(named_error.function(), "named");
        let expected = crate::HostSpecializationErrorReason::UninhabitedCallbackArguments {
            callback: crate::FunctionType::new(
                vec![crate::ValueType::Custom(crate::plan::CustomType::new(
                    crate::plan::CustomTypeName::new(
                        "application".into(),
                        "library".into(),
                        "Never".into(),
                    ),
                    Vec::new(),
                ))],
                crate::ValueType::Int,
            ),
        };
        assert_eq!(dynamic_error.reason(), &expected);
        assert_eq!(named_error.reason(), &expected);
    }

    #[test]
    fn callable_permissions_reject_uninhabited_arguments_and_captures_before_execution() {
        struct NeverSchema;
        impl crate::HostCustomSchema for NeverSchema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Never";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = crate::HostCustomConstructorListEnd;
        }
        type Never = crate::HostCustomType<NeverSchema>;
        struct Argument;
        impl HostCallableSchema for Argument {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "argument";
            type Arguments = One<Never>;
            type Return = BigInt;
            type Captures = End;
            type Constructions = End;
            type Completion = HostReturns;
        }
        struct Capture;
        impl HostCallableSchema for Capture {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "capture";
            type Arguments = End;
            type Return = BigInt;
            type Captures = One<Never>;
            type Constructions = End;
            type Completion = HostReturns;
        }
        let typed = crate::compile_declared_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    "pub type Never\npub fn run() { 42 }",
                )],
            )],
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<Capture>()
                .unwrap(),
        )
        .unwrap();
        let mut bindings = HostPreparation::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        bindings.callable::<Capture>().unwrap();
        let error = bindings.prepare().err().unwrap();
        assert_eq!(error.function(), "capture");
        assert_eq!(
            error.reason(),
            &crate::HostSpecializationErrorReason::UninhabitedCallableCapture {
                capture: crate::ValueType::Custom(crate::plan::CustomType::new(
                    crate::plan::CustomTypeName::new(
                        "application".into(),
                        "library".into(),
                        "Never".into()
                    ),
                    vec![],
                )),
            }
        );
        let module = || HostProviderModuleDeclaration::new("application", "library").unwrap();
        let arguments = HostDeclarations::from_providers([module()
            .with_function(HostFunctionDeclaration::<
                (),
                BigInt,
                One<HostCreatedFunction<Argument>>,
            >::new("make"))
            .unwrap()])
        .unwrap()
        .with_callable::<Argument>()
        .unwrap();
        let captures = HostDeclarations::from_providers([module()
            .with_function(HostFunctionDeclaration::<
                (),
                BigInt,
                One<HostCreatedFunction<Capture>>,
            >::new("make"))
            .unwrap()])
        .unwrap()
        .with_callable::<Capture>()
        .unwrap();
        let never = crate::ValueType::Custom(crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new(
                "application".into(),
                "library".into(),
                "Never".into(),
            ),
            vec![],
        ));
        for (declarations, reason) in [
            (
                arguments,
                crate::HostSpecializationErrorReason::UninhabitedCallbackArguments {
                    callback: crate::FunctionType::new(vec![never.clone()], crate::ValueType::Int),
                },
            ),
            (
                captures,
                crate::HostSpecializationErrorReason::UninhabitedCallableCapture { capture: never },
            ),
        ] {
            let typed = crate::compile_declared_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
pub type Never
@external(erlang, "native", "make") fn make() -> Int
pub fn run() { make() }
"#,
                    )],
                )],
                declarations,
            )
            .unwrap();
            let error = HostPreparation::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .unwrap()
                .prepare()
                .err()
                .unwrap();
            assert_eq!(error.package(), "application");
            assert_eq!(error.module(), "library");
            assert_eq!(error.function(), "make");
            assert_eq!(
                error.signature(),
                &crate::FunctionType::new(vec![], crate::ValueType::Int)
            );
            assert_eq!(error.reason(), &reason);
        }
    }

    #[test]
    fn uninhabited_source_parameters_erase_their_native_constructions() {
        const MAKE: HostFunctionDeclaration<(T,), Thunk<T>, One<HostCreatedFunction<Constant<T>>>> =
            HostFunctionDeclaration::new("make");
        let declared = crate::compile_declared_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Never
@external(erlang, "native", "make")
fn make(value: a) -> fn() -> a
fn unused(value: Never) { make(value) }
pub fn run() { let _ = unused 42 }
"#,
                )],
            )],
            HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
                "application",
                "library",
            )
            .unwrap()
            .with_function(MAKE)
            .unwrap()])
            .unwrap()
            .with_callable::<Constant<T>>()
            .unwrap(),
        )
        .unwrap();
        let plan = crate::planner::plan_declared_library_program(declared).unwrap();
        let function = plan
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .unwrap()
            .gleam_body()
            .unwrap();
        let entry = crate::plan::LibraryEntry::new(
            function.id(),
            crate::plan::LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let (_, bodies, _, roots) =
            super::super::lower_hosted_library(plan, entry, Vec::new()).unwrap();
        assert!(bodies.value_functions().is_empty());
        assert!(bodies.never_functions().is_empty());
        assert!(roots.is_empty());
    }
}
