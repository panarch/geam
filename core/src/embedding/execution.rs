use super::input::{InputShape, ScopedArgumentsInput};
use super::work::return_::ScopedReturn;
use super::work::{ScopeBrand, SharedValue};
use super::{CallError, Completed, Function, Future, HostedModule};
use crate::execution::{DriverError, ExecutionHost};
use crate::host::{HostExternalSchema, HostProfile};
use crate::runtime::execution::{Domain, EntryContext};
use crate::runtime::{ObservationError, SharedExecutionError};
use std::sync::Arc;

/// Typed function calls within one host-driven execution lifetime.
pub struct ExecutionScope<'scope, 'module: 'scope, Profile: HostProfile> {
    pub(super) context: EntryContext<Profile>,
    entries: &'module crate::plan::execution::LibraryFunctionEntries,
    pub(super) owner: &'module Arc<()>,
    pub(super) brand: ScopeBrand<'scope>,
    pub(super) native_callables: &'module [crate::plan::execution::LibraryNativeConstruction],
    pub(super) captures: crate::runtime::CaptureStorage,
}

impl<Profile: HostProfile> HostedModule<Profile> {
    /// Runs the Rust body while servicing its Gleam entries, then awaits cleanup.
    ///
    /// The host supplies the executor and clock. Provider state and Echo remain
    /// borrowed here; workers carry owned arguments and the shared sealed code.
    pub async fn with_execution<Output>(
        &mut self,
        host: &dyn ExecutionHost,
        state: &mut Profile::RunState,
        echo: &mut (dyn crate::EchoSink + Send),
        run: impl for<'scope> AsyncFnOnce(ExecutionScope<'scope, '_, Profile>) -> Output,
    ) -> Result<Output, DriverError> {
        let (plan, stores, captures) = self.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            host,
            state,
            stores,
            echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &self.entries,
            owner: &self.owner,
            brand: ScopeBrand::new(),
            native_callables: &self.native_callables,
            captures: domain.context().captures().clone(),
        };
        domain.drive(run(scope)).await
    }
}

impl<'scope, 'module: 'scope, Profile: HostProfile> ExecutionScope<'scope, 'module, Profile> {
    /// Executes one selected function. Dropping the returned Rust Future cancels its entry.
    ///
    /// A source Future result is returned as a value; call does not observe it.
    #[allow(private_bounds)]
    pub fn call<'call, Args, Return, Input, Shape>(
        &'call self,
        function: &Function<Args, Return, Shape>,
        arguments: Input,
    ) -> impl std::future::Future<Output = Result<Return::Value<'scope>, CallError>> + Send + 'call
    where
        Args: ScopedArgumentsInput<Input, ScopeBrand<'scope>>,
        Return: ScopedReturn<Profile>,
        Shape: InputShape<Input>,
    {
        let inputs = if !Arc::ptr_eq(&function.owner, self.owner) {
            Err(CallError::ForeignFunction)
        } else if !Args::owners_match(&arguments, self.owner) {
            Err(CallError::ForeignValue)
        } else {
            Ok(Args::into_inputs(
                arguments,
                Return::input_constructions(self.entries, function.slot),
            ))
        };
        let slot = function.slot;
        async move {
            let inputs = inputs?;
            let retention = self
                .context
                .retain_outputs(|stores| Return::retain(stores))
                .await
                .map_err(|_| CallError::Cancelled)?;
            let context = Return::context(
                self.brand,
                retention,
                self.owner,
                &mut super::callable::OutputCallables::new(Return::output_callables(
                    self.entries,
                    slot,
                )),
            );
            Return::call(&self.context, self.entries, slot, inputs, context).await
        }
    }

    /// Observes one source Future while the surrounding driver services its requests.
    #[allow(private_bounds)]
    pub fn observe<Value: SharedValue, Schema: HostExternalSchema>(
        &self,
        work: &Future<'scope, Value, Schema>,
    ) -> impl std::future::Future<Output = Result<Completed<Value>, ObservationError>>
    + Send
    + use<Profile, Value, Schema> {
        let source = work.work();
        let context = work.output_context();
        async move {
            let completed = source
                .observe()
                .await
                .map_err(|_| ObservationError::Cancelled)?;
            match completed.read(Clone::clone) {
                Ok(value) => Ok(Completed::new(value, context)),
                Err(error) => Err(ObservationError::Execution(SharedExecutionError(error))),
            }
        }
    }
}

#[cfg(all(test, feature = "tokio"))]
mod tests {
    use crate::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder, List, StringValue};
    use crate::execution::TokioHost;
    use crate::host::{HostProfile, HostProviderSet};
    use crate::{ModuleSource, PackageSource};
    use std::cell::Cell;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = Cell<usize>;
        type ExternalStores = Cell<()>;
        type ExecutionState = ();
    }

    fn program(source: &str) -> HostedModuleBuilder<Profile> {
        HostedModuleBuilder::new(
            crate::compile_typed_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new("library", "library.gleam", source)],
                )],
                HostProviderSet::from_providers([]).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn native_captures_and_callable_inputs_reject_lists_from_another_loaded_owner() {
        use crate::{HostCallableSchema, HostReturns, HostTypeList, HostTypeListEnd};
        struct Profile;
        impl crate::HostProfile for Profile {
            type RunState = std::cell::Cell<usize>;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl crate::HostProvider<Profile> for Profile {
            type State = std::cell::Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        use crate::embedding::{
            CallError, CallableType, FunctionDeclaration, HostedModuleBuilder, List,
        };
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostCaptures, HostConstructions, HostList,
            HostListType,
        };
        type End = HostTypeListEnd;
        type One<T> = HostTypeList<T, End>;
        type Lists = HostListType<BigInt>;
        struct Lengths;
        impl HostCallableSchema for Lengths {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "callbacks";
            const NAME: &'static str = "lengths";
            type Arguments = One<Lists>;
            type Return = BigInt;
            type Captures = One<Lists>;
            type Constructions = End;
            type Completion = HostReturns;
        }
        fn lengths<'call>(
            mut call: HostCall<'call, Profile, Profile, BigInt>,
            captures: HostCaptures<'call, One<Lists>>,
            _: HostConstructions<'call, End>,
            values: HostList<'call, BigInt>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let (captured, ()) = call.captures(captures);
            let state = call.state();
            state.set(state.get() + 1);
            let count = call.list_len(captured) + call.list_len(values);
            Ok(call.return_value(count.into()))
        }
        let build = || {
            let typed = crate::compile_typed_host_program(
                "application",
                "library",
                [crate::PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [crate::ModuleSource::new(
                        "library",
                        "library.gleam",
                        r#"
pub fn values() { [1, 2, 3] }
pub fn first() { fn(values: List(Int)) { case values { [head, ..] -> head [] -> panic as "empty callback input" } } }
pub fn empty() -> List(Int) { [] }
"#,
                    )],
                )],
                crate::HostProviderSet::new([])
                    .unwrap()
                    .with_callable::<Profile, Lengths, (Lists,), _>(lengths)
                    .unwrap(),
            )
            .unwrap();
            let (mut bindings, values) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), List<BigInt>>::new("values"))
                .unwrap();
            let first = bindings
                .function(FunctionDeclaration::<
                    (),
                    CallableType<(List<BigInt>,), BigInt>,
                >::new("first"))
                .unwrap();
            let factory = bindings.callable::<Lengths>().unwrap();
            let empty = bindings
                .function(FunctionDeclaration::<(), List<BigInt>>::new("empty"))
                .unwrap();
            (bindings.seal().unwrap(), values, first, factory, empty)
        };
        let host = crate::execution_fixture::TestHost::default();
        let (mut foreign, values, _, foreign_factory, _) = build();
        let foreign_values = host
            .block_on(foreign.with_execution(
                &host,
                &mut std::cell::Cell::new(0),
                &mut drop,
                async |scope| scope.call(&values, ()).await.unwrap(),
            ))
            .unwrap();
        let (mut module, values, first, factory, empty) = build();
        let mut calls = std::cell::Cell::new(0);
        let own_values = host
            .block_on(
                module.with_execution(&host, &mut calls, &mut drop, async |scope| {
                    assert_eq!(
                        scope
                            .construct(&foreign_factory, (&foreign_values, ()))
                            .err(),
                        Some(CallError::ForeignFunction)
                    );
                    assert_eq!(
                        scope.construct(&factory, (&foreign_values, ())).err(),
                        Some(CallError::ForeignValue)
                    );
                    let first = scope.call(&first, ()).await.unwrap();
                    let empty = scope.call(&empty, ()).await.unwrap();
                    assert_eq!(
                        scope
                            .invoke(&first, (&empty,))
                            .await
                            .err()
                            .unwrap()
                            .to_string(),
                        "panic: empty callback input"
                    );
                    assert_eq!(
                        scope.invoke(&first, (&foreign_values,)).await,
                        Err(CallError::ForeignValue)
                    );
                    let own = scope.call(&values, ()).await.unwrap();
                    let callable = scope.construct(&factory, (&own, ())).unwrap();
                    assert_eq!(
                        scope.invoke(&callable, (&own,)).await.unwrap(),
                        BigInt::from(6)
                    );
                    assert_eq!(
                        scope.invoke(&first, (&own,)).await.unwrap(),
                        BigInt::from(1)
                    );
                    own
                }),
            )
            .unwrap();
        assert_eq!(calls.get(), 1);

        use super::{ExecutionScope, ScopeBrand};
        use crate::runtime::execution::Domain;
        use std::{
            future::Future,
            sync::Arc,
            task::{Context, Poll, Waker},
        };
        let mut echoes = Vec::new();
        let (plan, stores, captures) = module.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut calls,
            stores,
            &mut echoes,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &module.entries,
            owner: &module.owner,
            brand: ScopeBrand::new(),
            native_callables: &module.native_callables,
            captures: domain.context().captures().clone(),
        };
        let callable = scope.construct(&factory, (&own_values, ())).unwrap();
        let mut invocation = std::pin::pin!(scope.invoke(&callable, (&own_values,)));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(invocation.as_mut().poll(&mut cx).is_pending());
        drop(domain);
        assert_eq!(
            invocation.as_mut().poll(&mut cx),
            Poll::Ready(Err(CallError::Cancelled))
        );
        assert_eq!(calls.get(), 1);
        assert!(echoes.is_empty());
    }

    #[test]
    fn drives_typed_plain_data_on_the_callers_runtime_without_a_work_component() {
        type Scalars = (
            BigInt,
            f64,
            StringValue,
            crate::BitArrayValue,
            char,
            bool,
            (),
        );
        type Data = (
            Scalars,
            Result<BigInt, StringValue>,
            List<(BigInt, StringValue)>,
        );
        let (mut bindings, identity) = program(
            r#"
pub fn identity(
  scalars: #(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil),
  choice: Result(Int, String),
  rows: List(#(Int, String)),
) { #(scalars, choice, rows) }
pub fn count(rows: List(#(Int, String))) { rows }
"#,
        )
        .function(FunctionDeclaration::<Data, Data>::new("identity"))
        .unwrap();
        let rows = bindings
            .function(FunctionDeclaration::<
                (List<(BigInt, StringValue)>,),
                List<(BigInt, StringValue)>,
            >::new("count"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        let mut state = Cell::new(7);
        let mut echo = Vec::new();
        let bits = crate::BitArrayValue::try_from_parts(vec![0b1010_0000], 3).unwrap();
        let scalar = (42.into(), 2.5, "three".into(), bits, 'x', true, ());
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    let first = scope
                        .call(
                            &identity,
                            (
                                scalar.clone(),
                                Ok(BigInt::from(4)),
                                vec![(5.into(), "five".into())],
                            ),
                        )
                        .await
                        .unwrap();
                    assert_eq!(first.0, scalar);
                    assert_eq!(first.1, Ok(4.into()));
                    let retained = scope.call(&rows, (&first.2,)).await.unwrap();
                    assert_eq!(retained.len(), 1);
                    retained
                }),
            )
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result.read_item(0, |(number, text)| (number.clone(), text.clone())),
            Some((BigInt::from(5), StringValue::from("five")))
        );
        assert_eq!(state.get(), 7);
        assert!(echo.is_empty());
    }

    #[test]
    fn source_errors_and_foreign_handles_keep_their_original_boundary() {
        use crate::embedding::CallError;
        let source = "pub fn run() -> Int { panic as \"source stopped\" }";
        let (bindings, run) = program(source)
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let (_, foreign) = program(source)
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = TokioHost::new(runtime.handle().clone());
        let mut state = Cell::new(0);
        let mut echo = Vec::new();
        runtime
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    assert_eq!(
                        scope.call(&foreign, ()).await,
                        Err(CallError::ForeignFunction)
                    );
                    assert_eq!(
                        scope.call(&run, ()).await.unwrap_err().to_string(),
                        "panic: source stopped"
                    );
                }),
            )
            .unwrap();
        assert!(echo.is_empty());
    }
}

#[cfg(test)]
mod plain_outputs {
    use crate::StringValue;
    use crate::embedding::HostedModuleBuilder;
    use crate::embedding::{CallError, FunctionDeclaration, List};
    use crate::frontend::{HostedTypedProgram, compile_typed_host_program};
    use crate::host::{HostProfile, HostProviderSet};
    use crate::{EchoOutput, EchoSink, ModuleSource, PackageSource, PlanError};

    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = ();
        type ExecutionState = ();
    }
    #[derive(Default)]
    struct Echo(Vec<String>);
    impl EchoSink for Echo {
        fn emit(&mut self, value: EchoOutput) {
            self.0.push(value.to_string());
        }
    }

    fn program(source: &str) -> HostedTypedProgram<Profile> {
        compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::from_providers([]).expect("empty providers"),
        )
        .expect("source typing")
    }

    #[test]
    fn preserves_library_planning_failures_in_unused_source() {
        let error = HostedModuleBuilder::new(program(
            "pub fn run() { 42 } pub fn unsupported() { <<1:native>> }",
        ))
        .err()
        .expect("unsupported unused source still fails planning");
        assert_eq!(
            error,
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            }
        );
    }

    #[test]
    fn callable_factories_and_invocations_keep_the_original_source_failure() {
        use crate::embedding::CallableType;
        let (mut bindings, failed_factory) = HostedModuleBuilder::new(program(
            r#"
pub fn failed_factory() -> fn() -> Int { panic as "factory stopped" }
pub fn failed_body() { fn() -> Int { panic as "body stopped" } }
"#,
        ))
        .unwrap()
        .function(FunctionDeclaration::<(), CallableType<(), BigInt>>::new(
            "failed_factory",
        ))
        .unwrap();
        let failed_body = bindings
            .function(FunctionDeclaration::<(), CallableType<(), BigInt>>::new(
                "failed_body",
            ))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                assert_eq!(
                    scope
                        .call(&failed_factory, ())
                        .await
                        .err()
                        .unwrap()
                        .to_string(),
                    "panic: factory stopped"
                );
                let callback = scope.call(&failed_body, ()).await.unwrap();
                assert_eq!(
                    scope.invoke(&callback, ()).await.unwrap_err().to_string(),
                    "panic: body stopped"
                );
            }),
        )
        .unwrap();
    }

    #[test]
    fn closing_the_domain_cancels_native_invocation_waiting_for_output_retention() {
        use super::{Domain, ExecutionScope, ScopeBrand};
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostCallableSchema, HostCaptures,
            HostConstructions, HostProvider, HostReturns, HostTypeListEnd,
        };
        use std::{
            future::Future,
            sync::Arc,
            task::{Context, Poll, Waker},
        };
        struct Constant;
        impl HostCallableSchema for Constant {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "constant";
            type Arguments = HostTypeListEnd;
            type Return = BigInt;
            type Captures = HostTypeListEnd;
            type Constructions = HostTypeListEnd;
            type Completion = HostReturns;
        }
        impl HostProvider<Profile> for Constant {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn constant<'call>(
            mut call: HostCall<'call, Profile, Constant, BigInt>,
            _: HostCaptures<'call, HostTypeListEnd>,
            _: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(42.into()))
        }
        let typed = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    "pub fn run() { 42 }",
                )],
            )],
            HostProviderSet::new([])
                .unwrap()
                .with_callable::<Constant, Constant, (), _>(constant)
                .unwrap(),
        )
        .unwrap();
        let (mut bindings, _) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let factory = bindings.callable::<Constant>().unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        host.block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                let callback = scope.construct(&factory, ()).unwrap();
                assert_eq!(scope.invoke(&callback, ()).await.unwrap(), BigInt::from(42));
            }),
        )
        .unwrap();
        let mut state = ();
        let mut echo = Echo::default();
        let (plan, stores, captures) = module.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &module.entries,
            owner: &module.owner,
            brand: ScopeBrand::new(),
            native_callables: &module.native_callables,
            captures: domain.context().captures().clone(),
        };
        let callback = scope.construct(&factory, ()).unwrap();
        let mut invocation = std::pin::pin!(scope.invoke(&callback, ()));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(invocation.as_mut().poll(&mut cx).is_pending());
        drop(domain);
        assert_eq!(
            invocation.as_mut().poll(&mut cx),
            Poll::Ready(Err(CallError::Cancelled))
        );
        assert!(echo.0.is_empty());
    }

    #[test]
    fn closing_the_domain_cancels_a_call_waiting_for_output_retention() {
        use super::{Domain, ExecutionScope, ScopeBrand};
        use std::future::Future;
        use std::sync::Arc;
        use std::task::{Context, Poll, Waker};

        type Strings = List<StringValue>;
        let (bindings, run) = HostedModuleBuilder::new(program(
            "pub fn run(values: List(String)) { echo \"entered\" values }",
        ))
        .unwrap()
        .function(FunctionDeclaration::<(Strings,), Strings>::new("run"))
        .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut echo = Echo::default();
        let (plan, stores, captures) = module.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &module.entries,
            owner: &module.owner,
            brand: ScopeBrand::new(),
            native_callables: &module.native_callables,
            captures: domain.context().captures().clone(),
        };
        let mut call = std::pin::pin!(scope.call(&run, (vec![StringValue::from("retained")],)));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(call.as_mut().poll(&mut cx).is_pending());
        drop(domain);
        assert_eq!(
            call.as_mut().poll(&mut cx).map(Result::err),
            Poll::Ready(Some(CallError::Cancelled))
        );
        assert!(echo.0.is_empty());
    }

    #[test]
    fn closing_the_domain_cancels_each_named_callable_entry_before_source_effects() {
        use crate::embedding::{BitArrayValue, CallableType, CustomType, NamedTypeSchema};
        use crate::runtime::{RetainedInputs, execution::Domain};
        use std::{
            future::Future,
            sync::Arc,
            task::{Context, Poll, Waker},
        };

        struct Empty;
        impl NamedTypeSchema for Empty {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Empty";
        }
        let (mut bindings, _) = HostedModuleBuilder::new(program(
            r#"
pub type Empty { Again(Empty) }
pub fn ints() { echo "entered" fn() { 42 } }
pub fn floats() { echo "entered" fn() { 1.5 } }
pub fn strings() { echo "entered" fn() { "value" } }
pub fn bits() { echo "entered" fn() { <<42>> } }
pub fn codepoints() { echo "entered" fn(value: UtfCodepoint) { value } }
pub fn bools() { echo "entered" fn() { True } }
pub fn nils() { echo "entered" fn() { Nil } }
pub fn tuples() { echo "entered" fn() { #(42, "value") } }
pub fn choices() { echo "entered" fn() -> Result(Int, Bool) { Ok(42) } }
pub fn lists() { echo "entered" fn() { [42] } }
pub fn functions() { echo "entered" fn() { fn() { 42 } } }
pub fn empty() -> fn() -> Empty { echo "entered" fn() { panic as "unused" } }
"#,
        ))
        .unwrap()
        .function(FunctionDeclaration::<(), CallableType<(), BigInt>>::new(
            "ints",
        ))
        .unwrap();
        macro_rules! bind {
            ($name:literal, $args:ty, $return:ty) => {
                bindings
                    .function(FunctionDeclaration::<(), CallableType<$args, $return>>::new($name))
                    .unwrap();
            };
        }
        bind!("floats", (), f64);
        bind!("strings", (), StringValue);
        bind!("bits", (), BitArrayValue);
        bind!("codepoints", (char,), char);
        bind!("bools", (), bool);
        bind!("nils", (), ());
        bind!("tuples", (), (BigInt, StringValue));
        bind!("choices", (), Result<BigInt, bool>);
        bind!("lists", (), List<BigInt>);
        bind!("functions", (), CallableType<(), BigInt>);
        bind!("empty", (), CustomType<Empty>);
        let mut module = bindings.seal().unwrap();
        assert_eq!(module.entries.functions.len(), 12);
        let host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut echo = Echo::default();
        let (plan, stores, captures) = module.execution.parts_mut();
        for entry in module.entries.functions.iter() {
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut state,
                stores,
                &mut echo,
                captures.clone(),
                Domain::<Profile>::DEFAULT_BUDGET,
            );
            let context = domain.context();
            let mut call = std::pin::pin!(entry.function.call(&context, RetainedInputs::empty()));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(call.as_mut().poll(&mut cx).is_pending());
            drop(domain);
            assert_eq!(
                call.as_mut().poll(&mut cx).map(Result::err),
                Poll::Ready(Some(CallError::Cancelled))
            );
        }
        assert!(echo.0.is_empty());
    }

    #[test]
    fn foreign_functions_and_retained_values_fail_before_source_execution() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
pub fn keep(values: List(String)) {
  echo "entered"
  values
}
"#;
        type Strings = List<StringValue>;
        let (left, keep) = HostedModuleBuilder::new(program(source))
            .expect("left plan")
            .function(FunctionDeclaration::<(Strings,), Strings>::new("keep"))
            .expect("left binding");
        let (right, foreign) = HostedModuleBuilder::new(program(source))
            .expect("right plan")
            .function(FunctionDeclaration::<(Strings,), Strings>::new("keep"))
            .expect("right binding");
        let mut left = left.seal().expect("left seal");
        let mut right = right.seal().expect("right seal");
        let mut state = ();
        let mut left_echo = Echo::default();
        let values = execution_host
            .block_on(left.with_execution(
                &execution_host,
                &mut state,
                &mut left_echo,
                async |scope| {
                    assert_eq!(
                        scope
                            .call(&foreign, (vec![StringValue::from("unused")],))
                            .await
                            .err(),
                        Some(CallError::ForeignFunction)
                    );
                    scope
                        .call(&keep, (vec![StringValue::from("first"), "second".into()],))
                        .await
                        .expect("fresh input")
                },
            ))
            .expect("ordinary calls are immediate");
        assert_eq!(left_echo.0, ["src/library.gleam:3\n\"entered\""]);
        let mut right_echo = Echo::default();
        execution_host
            .block_on(right.with_execution(
                &execution_host,
                &mut state,
                &mut right_echo,
                async |scope| {
                    assert_eq!(
                        scope.call(&foreign, (&values,)).await.err(),
                        Some(CallError::ForeignValue)
                    );
                },
            ))
            .expect("owner check is immediate");
        assert!(right_echo.0.is_empty());
        let retained = execution_host
            .block_on(left.with_execution(
                &execution_host,
                &mut state,
                &mut left_echo,
                async |scope| {
                    assert_eq!(
                        scope.call(&foreign, (&values,)).await.err(),
                        Some(CallError::ForeignFunction)
                    );
                    scope
                        .call(&keep, (&values,))
                        .await
                        .expect("same loaded owner")
                },
            ))
            .expect("later scope is immediate");
        drop(values);
        drop(left);
        drop(right);
        assert_eq!(retained.len(), 2);
        assert_eq!(
            retained.read_item(1, Clone::clone),
            Some(StringValue::from("second"))
        );
        assert_eq!(left_echo.0.len(), 2);
    }

    #[test]
    fn plain_scalar_tuple_result_and_list_returns_remain_direct() {
        let execution_host = crate::execution_fixture::TestHost::default();

        use crate::runtime::BitArrayValue;
        type Scalars = (BigInt, f64, StringValue, BitArrayValue, char, bool, ());
        type Data = (
            Scalars,
            Result<BigInt, StringValue>,
            List<(BigInt, StringValue)>,
        );
        let source = r#"
pub fn identity(
  scalars: #(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil),
  choice: Result(Int, String),
  rows: List(#(Int, String)),
) { #(scalars, choice, rows) }
pub fn empty() -> List(Int) { [] }
"#;
        let (mut bindings, identity) = HostedModuleBuilder::new(program(source))
            .expect("plan")
            .function(FunctionDeclaration::<Data, Data>::new("identity"))
            .expect("recursive data binding");
        let empty = bindings
            .function(FunctionDeclaration::<(), List<BigInt>>::new("empty"))
            .expect("empty list specialization");
        let mut module = bindings.seal().expect("all selected families seal");
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3).expect("three bits");
        let scalars = (
            BigInt::from(1),
            2.5,
            StringValue::from("three"),
            bits,
            '四',
            true,
            (),
        );
        let mut state = ();
        let mut echo = Echo::default();
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    for choice in [Ok(BigInt::from(4)), Err(StringValue::from("failed"))] {
                        let (actual, result, rows) = scope
                            .call(
                                &identity,
                                (
                                    scalars.clone(),
                                    choice.clone(),
                                    vec![(BigInt::from(5), StringValue::from("five"))],
                                ),
                            )
                            .await
                            .expect("direct call");
                        assert_eq!(actual, scalars);
                        assert_eq!(result, choice);
                        assert_eq!(
                            rows.read_item(0, |(number, text)| (number.clone(), text.clone())),
                            Some((BigInt::from(5), StringValue::from("five")))
                        );
                        assert_eq!(rows.read_item(1, |_| ()), None);
                    }
                    assert!(scope.call(&empty, ()).await.expect("empty list").is_empty());
                },
            ))
            .expect("every ordinary entry is immediate");
        assert!(echo.0.is_empty());
    }

    #[test]
    fn each_plain_return_family_uses_its_direct_entry_and_preserves_nested_data() {
        let execution_host = crate::execution_fixture::TestHost::default();

        use crate::BitArrayValue;

        let source = r#"
pub fn integer(value: Int) { value }
pub fn float(value: Float) { value }
pub fn string(value: String) { value }
pub fn bits(value: BitArray) { value }
pub fn codepoint(value: UtfCodepoint) { value }
pub fn boolean(value: Bool) { value }
pub fn nil(value: Nil) { value }
pub fn lists(value: List(List(Nil))) { value }
pub fn results(value: List(Result(Int, String))) { value }
"#;
        let (mut bindings, integer) = HostedModuleBuilder::new(program(source))
            .expect("plan")
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("integer"))
            .expect("integer binding");
        let float = bindings
            .function(FunctionDeclaration::<(f64,), f64>::new("float"))
            .expect("float binding");
        let string = bindings
            .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                "string",
            ))
            .expect("string binding");
        let bits = bindings
            .function(FunctionDeclaration::<(BitArrayValue,), BitArrayValue>::new(
                "bits",
            ))
            .expect("bit array binding");
        let codepoint = bindings
            .function(FunctionDeclaration::<(char,), char>::new("codepoint"))
            .expect("codepoint binding");
        let boolean = bindings
            .function(FunctionDeclaration::<(bool,), bool>::new("boolean"))
            .expect("boolean binding");
        let nil = bindings
            .function(FunctionDeclaration::<((),), ()>::new("nil"))
            .expect("nil binding");
        type Nils = List<List<()>>;
        let lists = bindings
            .function(FunctionDeclaration::<(Nils,), Nils>::new("lists"))
            .expect("nested lists");
        type Results = List<Result<BigInt, StringValue>>;
        let results = bindings
            .function(FunctionDeclaration::<(Results,), Results>::new("results"))
            .expect("result list");
        let mut module = bindings.seal().expect("all direct entries");
        let mut state = ();
        let mut echo = Echo::default();
        execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut echo,
                async |scope| {
                    assert_eq!(scope.call(&integer, (42.into(),)).await, Ok(42.into()));
                    assert_eq!(scope.call(&float, (2.5,)).await, Ok(2.5));
                    assert_eq!(
                        scope.call(&string, ("direct".into(),)).await,
                        Ok("direct".into())
                    );
                    let value =
                        BitArrayValue::try_from_parts(vec![0b1010_0000], 3).expect("three bits");
                    assert_eq!(scope.call(&bits, (value.clone(),)).await, Ok(value));
                    assert_eq!(scope.call(&codepoint, ('x',)).await, Ok('x'));
                    assert_eq!(scope.call(&boolean, (true,)).await, Ok(true));
                    assert_eq!(scope.call(&nil, ((),)).await, Ok(()));
                    let nested = scope
                        .call(&lists, (vec![vec![(), ()], vec![]],))
                        .await
                        .expect("nested lists");
                    assert_eq!(
                        nested.read_item(0, |list| list.read_item(1, |nil| nil)),
                        Some(Some(()))
                    );
                    assert_eq!(nested.read_item(1, |list| list.len()), Some(0));
                    let choices = scope
                        .call(
                            &results,
                            (vec![Ok(BigInt::from(42)), Err(StringValue::from("failed"))],),
                        )
                        .await
                        .expect("result items");
                    assert_eq!(
                        choices.read_item(0, |value| value.cloned().map_err(Clone::clone)),
                        Some(Ok(BigInt::from(42)))
                    );
                    assert_eq!(
                        choices.read_item(1, |value| value.cloned().map_err(Clone::clone)),
                        Some(Err(StringValue::from("failed")))
                    );
                },
            ))
            .expect("no Future allocation or suspension for direct entries");
        assert!(echo.0.is_empty());
    }
}
