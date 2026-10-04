use super::{NativeCall, NativeTargetIndex};
use crate::host::{
    HostCall, HostCallContinuation, HostCallError, HostCallable, HostConstructions,
    HostExecutionContext, HostExecutionError, HostFunctionType, HostFunctionValue,
    HostFunctionValueType, HostOwnedCompletion, HostProfile, HostProvider, HostType, HostTypeAt,
    HostTypeList, HostTypeListEnd, HostTypeSequence,
};
use crate::runtime::NativeValue;
use crate::runtime::execution::ExecutionContext;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// A retained source callback paired with its sealed native input conversion.
///
/// Different input types can share native storage without losing their typed
/// invocation target. Calling it uses the receiver's logical execution in the
/// original domain, not the execution that originally retained the callback.
pub struct NativeCallable<Profile: HostProfile> {
    source: NativeValue,
    invoke: Arc<Invoke<Profile>>,
}

/// A retained source function with the registered native invocation boundary.
/// Symbolic inputs preserve the function while offering no callable projection.
pub struct NativeFunctionValue<Profile: HostProfile> {
    kind: NativeFunctionValueKind<Profile>,
}

enum NativeFunctionValueKind<Profile: HostProfile> {
    Invocable(NativeCallable<Profile>),
    Symbolic {
        source: NativeValue,
        invoke: Arc<Invoke<Profile>>,
    },
}

impl<Profile: HostProfile> Clone for NativeFunctionValue<Profile> {
    fn clone(&self) -> Self {
        Self {
            kind: match &self.kind {
                NativeFunctionValueKind::Invocable(callable) => {
                    NativeFunctionValueKind::Invocable(callable.clone())
                }
                NativeFunctionValueKind::Symbolic { source, invoke } => {
                    NativeFunctionValueKind::Symbolic {
                        source: source.clone(),
                        invoke: Arc::clone(invoke),
                    }
                }
            },
        }
    }
}

impl<Profile: HostProfile> NativeFunctionValue<Profile> {
    pub fn native_value(&self) -> &NativeValue {
        match &self.kind {
            NativeFunctionValueKind::Invocable(callable) => callable.native_value(),
            NativeFunctionValueKind::Symbolic { source, .. } => source,
        }
    }

    pub fn callable(&self) -> Option<&NativeCallable<Profile>> {
        match &self.kind {
            NativeFunctionValueKind::Invocable(callable) => Some(callable),
            NativeFunctionValueKind::Symbolic { .. } => None,
        }
    }

    /// Applies the existing native input boundary in the receiver's execution.
    /// An unrepresentable symbolic input fails after endpoint admission, without
    /// entering source code or fabricating a typed argument.
    pub async fn invoke<Provider, Constructions>(
        &self,
        context: &HostExecutionContext<'_, Profile, Provider, Constructions>,
        input: NativeValue,
    ) -> Completion
    where
        Provider: HostProvider<Profile>,
        Constructions: HostTypeSequence,
    {
        match &self.kind {
            NativeFunctionValueKind::Invocable(callable) => callable.invoke(context, input).await,
            NativeFunctionValueKind::Symbolic { invoke, .. } => {
                let invocation = invoke(context.execution().clone(), input);
                let _scope = context;
                invocation.await
            }
        }
    }
}

type Completion = Result<NativeValue, HostExecutionError>;
type Invocation = Pin<Box<dyn Future<Output = Completion> + Send>>;
type Invoke<Profile> = dyn Fn(ExecutionContext<Profile>, NativeValue) -> Invocation + Send + Sync;

impl<Profile: HostProfile> Clone for NativeCallable<Profile> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            invoke: Arc::clone(&self.invoke),
        }
    }
}

impl<Profile: HostProfile> NativeCallable<Profile> {
    /// The original source function, for native equality, hashing and inspection.
    pub fn native_value(&self) -> &NativeValue {
        &self.source
    }

    pub fn invoke<'request, Provider, Constructions>(
        &'request self,
        context: &'request HostExecutionContext<'_, Profile, Provider, Constructions>,
        input: NativeValue,
    ) -> impl Future<Output = Completion> + Send + 'request
    where
        Provider: HostProvider<Profile>,
        Constructions: HostTypeSequence,
    {
        let invocation = (self.invoke)(context.execution().clone(), input);
        async move {
            let _scope = context;
            invocation.await
        }
    }
}

impl<'call, Profile, Provider, Return, Targets>
    NativeCall<'call, Profile, Provider, Return, Targets>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    /// Retains a one-argument callback with the conversion registered at `Index`.
    #[allow(private_bounds)]
    pub fn owned_callable<Index, CallbackReturn>(
        &self,
        callback: HostCallable<
            'call,
            HostTypeList<<Targets as HostTypeAt<Index>>::Type, HostTypeListEnd>,
            CallbackReturn,
        >,
    ) -> NativeCallable<Profile>
    where
        Targets: HostTypeAt<Index>,
        Index: NativeTargetIndex,
        CallbackReturn: HostType,
    {
        let source = self.source::<HostFunctionType<
            HostTypeList<<Targets as HostTypeAt<Index>>::Type, HostTypeListEnd>,
            CallbackReturn,
        >>(callback);
        let callable = self.call.runtime.callable(callback.token);
        let execution = self.call.runtime.execution().with_unit(None);
        let codec = self.call.runtime.codec_scope();
        let origin = crate::runtime::HostCallOrigin::host(codec.function());
        let rules = Arc::clone(&self.rules);
        NativeCallable {
            source,
            invoke: Arc::new(move |caller, input| {
                let execution = execution.for_invocation(&caller);
                let codec = codec.clone();
                let origin = origin.clone();
                let callable = callable.clone();
                let rules = Arc::clone(&rules);
                Box::pin(async move {
                    let execution = execution?;
                    let inputs = execution
                        .with_codec(codec, origin.clone(), move |runtime| {
                            let mut native = NativeCall::<Profile, Provider, Return, Targets>::new(
                                HostCall::new(runtime),
                                rules,
                            );
                            let value = native.convert::<Index>(&input)?;
                            let value = crate::host::type_::into_scoped::<
                                <Targets as HostTypeAt<Index>>::Type,
                            >(value);
                            Some(native.call.runtime.callback_inputs(Box::new([value])))
                        })
                        .await?
                        .ok_or_else(|| {
                            crate::HostFailure::new(
                                "native value does not match the registered callback input",
                            )
                        })?;
                    let value = execution
                        .invoke(callable, origin, inputs)
                        .await?
                        .map_err(HostCallError::nested)?;
                    Ok(NativeValue::from_stored(value))
                })
            }),
        }
    }

    /// Retains a general function value, using the existing native target when invocable.
    #[allow(private_bounds)]
    pub fn owned_function_value<Index, CallbackReturn>(
        &self,
        function: HostFunctionValue<
            'call,
            HostTypeList<<Targets as HostTypeAt<Index>>::Type, HostTypeListEnd>,
            CallbackReturn,
        >,
    ) -> NativeFunctionValue<Profile>
    where
        Targets: HostTypeAt<Index>,
        Index: NativeTargetIndex,
        CallbackReturn: HostType,
    {
        let kind = match function.callable() {
            Some(callable) => NativeFunctionValueKind::Invocable(
                self.owned_callable::<Index, CallbackReturn>(callable),
            ),
            None => {
                let codec = self.call.runtime.codec_scope();
                let origin = crate::runtime::HostCallOrigin::host(codec.function());
                let execution = self.call.runtime.execution().with_unit(None);
                NativeFunctionValueKind::Symbolic {
                    source: self.source::<HostFunctionValueType<
                        HostTypeList<<Targets as HostTypeAt<Index>>::Type, HostTypeListEnd>,
                        CallbackReturn,
                    >>(function),
                    invoke: Arc::new(move |caller, input| {
                        let execution = execution.for_invocation(&caller);
                        let codec = codec.clone();
                        let origin = origin.clone();
                        Box::pin(async move {
                            let _input = input;
                            let execution = execution?;
                            execution.with_codec(codec, origin, |_| ()).await?;
                            Err(crate::HostFailure::new(
                                "native value does not match the registered callback input",
                            )
                            .into())
                        })
                    }),
                }
            }
        };
        NativeFunctionValue { kind }
    }

    /// Resumes this native operation and converts its native result through the
    /// registered return target. Both codecs retain the original registration.
    #[allow(private_bounds)]
    pub fn resume<Index>(
        self,
        start: impl for<'run> FnOnce(
            HostExecutionContext<'run, Profile, Provider, Targets>,
        ) -> Pin<Box<dyn Future<Output = Completion> + Send + 'run>>
        + Send
        + 'static,
    ) -> HostCallContinuation<'call, Return>
    where
        Targets: HostTypeAt<Index, Type = Return>,
        Index: NativeTargetIndex,
    {
        let Self {
            call,
            rules,
            targets: _,
        } = self;
        call.resume(HostConstructions::<Targets>::new(), move |context| {
            Box::pin(async move {
                let value = start(context).await?;
                Ok(HostOwnedCompletion::new(move |call, _| {
                    let mut native =
                        NativeCall::<Profile, Provider, Return, Targets>::new(call, rules);
                    let returned = native.convert::<Index>(&value).ok_or_else(|| {
                        crate::HostFailure::new(
                            "native value does not match the registered return type",
                        )
                    })?;
                    Ok(native.finish(returned))
                }))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NativeCallable;
    use crate::embedding::{
        BigInt, CustomType, FunctionDeclaration, HostedModuleBuilder, NamedTypeSchema,
    };
    use crate::execution::ExecutionUnitId;
    use crate::execution_fixture::TestHost;
    use crate::host::native::{NativeCall, NativeRules};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostConstructions, HostExternal, HostExternalBinding, HostExternalEquality,
        HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
        HostExternalStore, HostExternalType, HostFunctionType, HostOwnedCallable,
        HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
    };
    use crate::runtime::NativeValue;
    use crate::{ModuleSource, PackageSource, ValueType};
    use futures_channel::oneshot;
    use futures_util::future;
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    struct Profile;
    struct Provider;
    struct HandlerSchema;
    struct Storage;
    struct Handlers;

    struct State {
        units: Vec<ExecutionUnitId>,
        gate: Option<oneshot::Receiver<()>>,
        ready: Option<oneshot::Sender<()>>,
        record: Option<oneshot::Sender<Recorded>>,
        completion: Option<super::Completion>,
    }

    type Integers = HostTypeList<BigInt, HostTypeListEnd>;

    // Observes the actual sealed callback and endpoint for closed-protocol tests.
    struct Recorded {
        native: NativeCallable<Profile>,
        typed: HostOwnedCallable<Profile, Provider, Integers, BigInt, HostTypeListEnd>,
        execution: crate::runtime::execution::ExecutionContext<Profile>,
        codec: crate::host::HostCodecScope,
        origin: crate::runtime::HostCallOrigin,
        input: NativeValue,
    }

    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = HostExternalStore<NativeCallable<Profile>>;
        type ExecutionState = ();
    }

    impl HostProvider<Profile> for Provider {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }

    impl HostExternalSchema for HandlerSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Handler";
        const PARAMETER_COUNT: usize = 1;
    }

    impl HostExternalBinding<Profile, HandlerSchema> for Provider {
        type Storage = Storage;
    }

    impl HostExternalStorage<Profile, HandlerSchema> for Storage {
        type Payload = NativeCallable<Profile>;
        fn store(
            stores: &<Profile as HostProfile>::ExternalStores,
        ) -> &HostExternalStore<Self::Payload> {
            stores
        }
        fn source_equal(
            context: &HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left.native_value()
                .source_equal(context, right.native_value())
        }
        fn source_hash(context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            value.native_value().source_hash(context)
        }
        fn inspect(context: &HostExternalInspection<'_>, value: &Self::Payload) -> ecow::EcoString {
            value.native_value().inspect(context)
        }
    }

    impl NamedTypeSchema for Handlers {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Handlers";
    }

    type First = HostTypeParameter<0>;
    type Second = HostTypeParameter<1>;
    type Handler<Output> = HostExternalType<HandlerSchema, HostTypeList<Output, HostTypeListEnd>>;
    type Target = HostTypeList<First, HostTypeListEnd>;
    type Input = HostTypeList<Second, HostTypeListEnd>;
    type Callback = HostFunctionType<Input, First>;

    fn retain<'call>(
        mut call: NativeCall<'call, Profile, Provider, Handler<First>, Input>,
        callback: HostCallable<'call, Input, First>,
    ) -> Result<HostCallCompletion<'call, Handler<First>>, HostCallError> {
        let callback = call.owned_callable::<HostTypeIndex0, First>(callback);
        let equal = call.call().create_external(callback.clone());
        let value = call.call().create_external(callback);
        assert!(call.call().equal::<Handler<First>>(value, equal));
        assert_eq!(
            call.call().source_hash::<Handler<First>>(value),
            call.call().source_hash::<Handler<First>>(equal)
        );
        Ok(call.finish(value))
    }

    fn apply<'call>(
        mut call: NativeCall<'call, Profile, Provider, First, Target>,
        handler: HostExternal<'call, Handler<First>>,
        input: HostValue<'call, Second>,
    ) -> Result<HostCallContinuation<'call, First>, HostCallError> {
        let input = call.source::<Second>(input);
        let handler = call.call().external_payload(handler).clone();
        Ok(call.resume::<HostTypeIndex0>(move |context| {
            Box::pin(async move { handler.invoke(&context, input).await })
        }))
    }

    fn mark<'call>(
        mut call: HostCall<'call, Profile, Provider, ()>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let unit = call.execution_unit().unwrap().id();
        call.state().units.push(unit);
        Ok(call.return_value(()))
    }

    fn hold<'call>(
        mut call: HostCall<'call, Profile, Provider, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        let gate = call.state().gate.take().unwrap();
        let ready = call.state().ready.take().unwrap();
        Ok(call.resume(constructions, move |_| {
            Box::pin(async move {
                ready.send(()).unwrap();
                gate.await.unwrap();
                Ok(HostOwnedCompletion::new(
                    |call, _| Ok(call.return_value(())),
                ))
            })
        }))
    }

    fn native_output<'call>(
        mut call: NativeCall<'call, Profile, Provider, First, Target>,
        value: HostValue<'call, First>,
    ) -> Result<HostCallContinuation<'call, First>, HostCallError> {
        let value = call.source::<First>(value);
        let completion = call.call().state().completion.take().unwrap_or(Ok(value));
        Ok(call.resume::<HostTypeIndex0>(move |_| Box::pin(async move { completion })))
    }

    fn record<'call>(
        mut call: NativeCall<'call, Profile, Provider, (), Integers>,
        callback: HostCallable<'call, Integers, BigInt>,
        input: BigInt,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let native = call.owned_callable::<HostTypeIndex0, BigInt>(callback);
        let typed = call
            .call()
            .owned_callable(callback, &HostConstructions::<HostTypeListEnd>::new());
        let execution = call.call().runtime.execution().with_unit(None);
        let codec = call.call().runtime.codec_scope();
        let origin = crate::runtime::HostCallOrigin::host(codec.function());
        let input = call.source::<BigInt>(input);
        let sender = call.call().state().record.take().unwrap();
        assert!(
            sender
                .send(Recorded {
                    native,
                    typed,
                    execution,
                    codec,
                    origin,
                    input
                })
                .is_ok()
        );
        Ok(call.finish(()))
    }

    fn program() -> HostedModuleBuilder<Profile> {
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_external_type::<Provider, HandlerSchema>()
            .unwrap()
            .with_native_function::<Provider, (Callback,), Handler<First>, Input, _>(
                "retain",
                NativeRules::default(),
                retain,
            )
            .unwrap()
            .with_resumable_native_function::<Provider, (Handler<First>, Second), First, Target, _>(
                "apply",
                NativeRules::default(),
                apply,
            )
            .unwrap()
            .with_scoped_function::<Provider, (), (), _>("mark", mark)
            .unwrap()
            .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>("hold", hold)
            .unwrap()
            .with_resumable_native_function::<Provider, (First,), First, Target, _>(
                "native_output",
                NativeRules::default(),
                native_output,
            )
            .unwrap()
            .with_native_function::<Provider, (HostFunctionType<Integers, BigInt>, BigInt), (), Integers, _>("record", NativeRules::default(), record)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
pub type Handler(output)
pub opaque type Handlers { Handlers(Handler(Int), Handler(Int)) }
@external(erlang, "native", "retain")
fn retain(callback: fn(input) -> output) -> Handler(output)
@external(erlang, "native", "apply")
fn apply(handler: Handler(output), input: input) -> output
@external(erlang, "native", "mark")
fn mark() -> Nil
@external(erlang, "native", "hold")
fn hold() -> Nil
@external(erlang, "native", "native_output")
fn native_output(value: value) -> value
pub fn output() { native_output(42) }
@external(erlang, "native", "record")
fn record(callback: fn(Int) -> Int, input: Int) -> Nil
pub fn capture(input: Int) {
  record(fn(value) {
    echo value
    case value { -1 -> panic as "recorded callback failed" _ -> value + 1 }
  }, input)
}
pub fn create() {
  mark()
  let offset = 1
  let first = retain(fn(value: #(Int, Int)) { mark() hold() value.0 + value.1 + offset })
  let second = retain(fn(value: Bool) { mark() case value { True -> 7 False -> 8 } })
  let assert True = first == first
  echo first
  Handlers(first, second)
}
pub fn run(handlers: Handlers) {
  let Handlers(first, second) = handlers
  #(apply(first, #(40, 2)), apply(second, True), apply(second, False))
}
pub fn invalid(handlers: Handlers) {
  let Handlers(first, _) = handlers
  apply(first, "wrong shape")
}
pub fn failing() {
  apply(retain(fn(_: Int) { panic as "callback failure" 0 }), 1)
}
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        HostedModuleBuilder::new(typed).unwrap()
    }

    #[test]
    fn general_native_function_values_preserve_sources_and_reject_symbolic_input_after_admission() {
        use super::NativeFunctionValue;
        use crate::host::{
            HostExecutionContext, HostFunctionValue, HostFunctionValueType, HostOwnedFunctionValue,
        };
        type Argument = HostTypeParameter<0>;
        type Arguments = HostTypeList<Argument, HostTypeListEnd>;
        struct GeneralProfile;
        struct GeneralProvider;
        struct GeneralState {
            record: Option<oneshot::Sender<GeneralRecord>>,
        }
        struct GeneralRecord {
            native: NativeFunctionValue<GeneralProfile>,
            typed: HostOwnedFunctionValue<
                GeneralProfile,
                GeneralProvider,
                Arguments,
                crate::StringValue,
                HostTypeListEnd,
            >,
            execution: crate::runtime::execution::ExecutionContext<GeneralProfile>,
            codec: crate::host::HostCodecScope,
            origin: crate::runtime::HostCallOrigin,
            input: NativeValue,
        }
        impl HostProfile for GeneralProfile {
            type RunState = GeneralState;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl HostProvider<GeneralProfile> for GeneralProvider {
            type State = GeneralState;
            fn project(state: &mut GeneralState) -> &mut GeneralState {
                state
            }
        }
        fn capture<'call>(
            mut call: NativeCall<'call, GeneralProfile, GeneralProvider, (), Arguments>,
            function: HostFunctionValue<'call, Arguments, crate::StringValue>,
        ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
            let native = call.owned_function_value::<HostTypeIndex0, crate::StringValue>(function);
            let typed = call
                .call()
                .owned_function_value(function, &HostConstructions::<HostTypeListEnd>::new());
            assert_eq!(native.callable().is_some(), typed.callable().is_some());
            let copy = native.clone();
            assert!(
                call.call()
                    .native_equal(native.native_value(), copy.native_value())
            );
            let execution = call.call().runtime.execution().with_unit(None);
            let codec = call.call().runtime.codec_scope();
            let origin = crate::runtime::HostCallOrigin::host(codec.function());
            let input = call.source::<BigInt>(41.into());
            assert!(
                call.call()
                    .state()
                    .record
                    .take()
                    .unwrap()
                    .send(GeneralRecord {
                        native,
                        typed,
                        execution,
                        codec,
                        origin,
                        input
                    })
                    .is_ok()
            );
            Ok(call.finish(()))
        }
        for (function, invocable) in [
            ("fn(_) { let label = \"symbolic\" label }", false),
            ("fn(value: Int) { echo value \"called\" }", true),
        ] {
            let providers = HostProviderSet::from_providers([HostProviderModule::new("app", "main").unwrap()
                .with_native_function::<GeneralProvider, (HostFunctionValueType<Arguments, crate::StringValue>,), (), Arguments, _>("capture", NativeRules::default(), capture).unwrap()]).unwrap();
            let source = format!(
                "@external(erlang, \"native\", \"capture\") fn capture(function: fn(item) -> String) -> Nil\npub fn main() {{ capture({function}) }}"
            );
            let typed = crate::compile_typed_host_program(
                "app",
                "main",
                [PackageSource::new(
                    "app",
                    Vec::<&str>::new(),
                    [ModuleSource::new("main", "main.gleam", source)],
                )],
                providers,
            )
            .unwrap();
            let (bindings, main) = HostedModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), ()>::new("main"))
                .unwrap();
            let mut module = bindings.seal().unwrap();
            let mut previous = None;
            for serviced in 0..=2 {
                let host = TestHost::default();
                let (sender, mut receiver) = oneshot::channel();
                let (release, finished) = oneshot::channel();
                let mut state = GeneralState {
                    record: Some(sender),
                };
                let mut echo = Vec::new();
                let mut driver =
                    Box::pin(
                        module.with_execution(&host, &mut state, &mut echo, async |scope| {
                            scope.call(&main, ()).await.unwrap();
                            finished.await.unwrap();
                        }),
                    );
                assert!(host.poll(driver.as_mut()).is_pending());
                let record = receiver.try_recv().unwrap().unwrap();
                assert_eq!(record.typed.clone().callable().is_some(), invocable);
                if let Some((execution, codec, origin)) = previous {
                    let mut foreign_scope = crate::host::execution::NativeScope;
                    let foreign = HostExecutionContext::<
                        GeneralProfile,
                        GeneralProvider,
                        HostTypeListEnd,
                    >::new(
                        &mut foreign_scope, execution, codec, origin, 0
                    );
                    assert_eq!(
                        host.block_on(record.native.invoke(&foreign, record.input.clone()))
                            .err()
                            .unwrap()
                            .to_string(),
                        "the native operation was cancelled"
                    );
                }
                previous = Some((
                    record.execution.clone(),
                    record.codec.clone(),
                    record.origin.clone(),
                ));
                let mut native_scope = crate::host::execution::NativeScope;
                let context =
                    HostExecutionContext::<GeneralProfile, GeneralProvider, HostTypeListEnd>::new(
                        &mut native_scope,
                        record.execution.clone(),
                        record.codec.clone(),
                        record.origin.clone(),
                        0,
                    );
                let mut invocation = Box::pin(record.native.invoke(&context, record.input.clone()));
                let mut cx = Context::from_waker(Waker::noop());
                let mut result = invocation.as_mut().poll(&mut cx);
                assert!(result.is_pending());
                for _ in 0..serviced {
                    if result.is_ready() {
                        break;
                    }
                    assert!(host.poll(driver.as_mut()).is_pending());
                    result = invocation.as_mut().poll(&mut cx);
                }
                if serviced == 0 || (invocable && serviced == 1) {
                    assert!(result.is_pending());
                    drop(driver);
                    assert_eq!(
                        invocation
                            .as_mut()
                            .poll(&mut cx)
                            .map(|value| value.err().unwrap().to_string()),
                        Poll::Ready("the native operation was cancelled".to_owned())
                    );
                    assert!(echo.is_empty());
                } else {
                    if invocable {
                        assert_eq!(
                            result.map(|value| value.unwrap().as_string().unwrap().to_string()),
                            Poll::Ready("called".to_owned())
                        );
                    } else {
                        assert_eq!(
                            result.map(|value| value.err().unwrap().to_string()),
                            Poll::Ready(
                                "native value does not match the registered callback input"
                                    .to_owned()
                            )
                        );
                    }
                    release.send(()).unwrap();
                    host.block_on(driver).unwrap();
                    assert_eq!(echo.len(), usize::from(invocable));
                    let closed = host.block_on(record.native.invoke(&context, record.input));
                    assert_eq!(
                        closed.err().unwrap().to_string(),
                        "the native operation was cancelled"
                    );
                }
            }
        }
    }

    #[test]
    fn native_callback_endpoints_close_at_input_and_invocation_waits() {
        let (builder, capture) = program()
            .function(FunctionDeclaration::<(BigInt,), ()>::new("capture"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        for (serviced, input, invalid) in [
            (0, 41, false),
            (1, 41, false),
            (2, 41, false),
            (2, -1, false),
            (1, 41, true),
        ] {
            let host = TestHost::default();
            let (sender, mut receiver) = oneshot::channel();
            let (release, finished) = oneshot::channel();
            let mut state = State {
                units: Vec::new(),
                gate: None,
                ready: None,
                record: Some(sender),
                completion: None,
            };
            let mut echo = Vec::new();
            let mut driver =
                Box::pin(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&capture, (input.into(),)).await.unwrap();
                        finished.await.unwrap();
                    }),
                );
            assert!(host.poll(driver.as_mut()).is_pending());
            let recorded = receiver.try_recv().unwrap().unwrap();
            let mut scope = crate::host::execution::NativeScope;
            let context =
                crate::host::HostExecutionContext::<Profile, Provider, HostTypeListEnd>::new(
                    &mut scope,
                    recorded.execution.clone(),
                    recorded.codec.clone(),
                    recorded.origin.clone(),
                    0,
                );
            let value = if invalid {
                NativeValue::symbol("not_an_integer")
            } else {
                recorded.input.clone()
            };
            let mut invocation = Box::pin(recorded.native.invoke(&context, value));
            let mut cx = Context::from_waker(Waker::noop());
            let mut result = invocation.as_mut().poll(&mut cx);
            assert!(result.is_pending());
            for _ in 0..serviced {
                assert!(host.poll(driver.as_mut()).is_pending());
                result = invocation.as_mut().poll(&mut cx);
            }
            if invalid {
                assert_eq!(
                    result.map(|result| result.err().unwrap().to_string()),
                    Poll::Ready(
                        "native value does not match the registered callback input".to_owned()
                    )
                );
                drop(driver);
                assert!(echo.is_empty());
            } else if serviced < 2 {
                assert!(result.is_pending());
                drop(driver);
                assert_eq!(
                    invocation
                        .as_mut()
                        .poll(&mut cx)
                        .map(|result| result.err().unwrap().to_string()),
                    Poll::Ready("the native operation was cancelled".to_owned())
                );
                assert!(echo.is_empty());
            } else {
                if input == -1 {
                    assert_eq!(
                        result.map(|result| result.err().unwrap().to_string()),
                        Poll::Ready("panic: recorded callback failed".to_owned())
                    );
                } else {
                    assert_eq!(
                        result.map(|result| result.ok().unwrap().as_int()),
                        Poll::Ready(Some(42.into()))
                    );
                }
                release.send(()).unwrap();
                host.block_on(driver).unwrap();
                assert_eq!(echo.len(), 1);
                assert_eq!(echo[0].value(), &crate::Value::Int(input.into()));
            }
        }
    }

    fn typed_inputs<'call>(
        _: HostCall<'call, Profile, Provider, ()>,
        _: HostConstructions<'call, HostTypeListEnd>,
    ) -> (BigInt, ()) {
        (41.into(), ())
    }

    fn typed_output<'call>(
        _: HostCall<'call, Profile, Provider, ()>,
        _: HostConstructions<'call, HostTypeListEnd>,
        value: BigInt,
    ) -> Result<BigInt, HostCallError> {
        Ok(value)
    }

    #[test]
    fn retained_native_and_typed_callbacks_reject_a_different_execution_domain() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let (builder, capture) = program()
            .function(FunctionDeclaration::<(BigInt,), ()>::new("capture"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = TestHost::default();
        let mut records = Vec::new();
        let conversions = Arc::new(AtomicUsize::new(0));
        let inputs = {
            let conversions = Arc::clone(&conversions);
            move |_: HostCall<'_, Profile, Provider, ()>,
                  _: HostConstructions<'_, HostTypeListEnd>| {
                conversions.fetch_add(1, Ordering::SeqCst);
                Ok((BigInt::from(41), ()))
            }
        };
        for _ in 0..2 {
            let (sender, mut receiver) = oneshot::channel();
            let (release, finished) = oneshot::channel();
            let mut state = State {
                units: Vec::new(),
                gate: None,
                ready: None,
                record: Some(sender),
                completion: None,
            };
            let mut echo = Vec::new();
            let mut driver =
                Box::pin(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        scope.call(&capture, (41.into(),)).await.unwrap();
                        finished.await.unwrap();
                    }),
                );
            assert!(host.poll(driver.as_mut()).is_pending());
            let recorded = receiver.try_recv().unwrap().unwrap();
            let mut scope = crate::host::execution::NativeScope;
            let context =
                crate::host::HostExecutionContext::<Profile, Provider, HostTypeListEnd>::new(
                    &mut scope,
                    recorded.execution.clone(),
                    recorded.codec.clone(),
                    recorded.origin.clone(),
                    0,
                );
            let mut typed = Box::pin(recorded.typed.invoke(&context, typed_inputs, typed_output));
            let mut cx = Context::from_waker(Waker::noop());
            let mut result = typed.as_mut().poll(&mut cx);
            assert!(result.is_pending());
            for _ in 0..3 {
                assert!(host.poll(driver.as_mut()).is_pending());
                result = typed.as_mut().poll(&mut cx);
            }
            assert_eq!(result.map(|result| result.unwrap()), Poll::Ready(42.into()));
            drop(typed);
            let mut fallible = Box::pin(recorded.typed.try_invoke(
                &context,
                inputs.clone(),
                typed_output,
            ));
            let mut result = fallible.as_mut().poll(&mut cx);
            assert!(result.is_pending());
            for _ in 0..3 {
                assert!(host.poll(driver.as_mut()).is_pending());
                result = fallible.as_mut().poll(&mut cx);
            }
            assert_eq!(result.map(|result| result.unwrap()), Poll::Ready(42.into()));
            drop(fallible);
            release.send(()).unwrap();
            host.block_on(driver).unwrap();
            records.push(recorded);
            assert_eq!(echo.len(), 2);
            assert_eq!(echo[0].value(), &crate::Value::Int(41.into()));
            assert_eq!(echo[1].value(), &crate::Value::Int(41.into()));
        }
        let [first, second]: [_; 2] = records.try_into().ok().unwrap();
        let mut scope = crate::host::execution::NativeScope;
        let context = crate::host::HostExecutionContext::<Profile, Provider, HostTypeListEnd>::new(
            &mut scope,
            second.execution,
            second.codec,
            second.origin,
            0,
        );
        let native = host.block_on(first.native.invoke(&context, first.input));
        assert_eq!(
            native.err().unwrap().to_string(),
            "the native operation was cancelled"
        );
        let typed = host.block_on(first.typed.invoke(&context, typed_inputs, typed_output));
        assert_eq!(
            typed.err().unwrap().to_string(),
            "the native operation was cancelled"
        );
        assert_eq!(conversions.load(Ordering::SeqCst), 2);
        let typed = host.block_on(first.typed.try_invoke(&context, inputs, typed_output));
        assert_eq!(
            typed.err().unwrap().to_string(),
            "the native operation was cancelled"
        );
        assert_eq!(conversions.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn heterogeneous_native_callbacks_retain_their_decoders_and_run_in_the_receivers_unit() {
        let (mut bindings, create) = program()
            .function(FunctionDeclaration::<(), CustomType<Handlers>>::new(
                "create",
            ))
            .unwrap();
        let run = bindings
            .function(FunctionDeclaration::<
                (CustomType<Handlers>,),
                (BigInt, BigInt, BigInt),
            >::new("run"))
            .unwrap();
        let invalid = bindings
            .function(FunctionDeclaration::<(CustomType<Handlers>,), BigInt>::new(
                "invalid",
            ))
            .unwrap();
        let failing = bindings
            .function(FunctionDeclaration::<(), BigInt>::new("failing"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = TestHost::default();
        let (release, gate) = oneshot::channel();
        let (ready, waiting) = oneshot::channel();
        let mut state = State {
            units: Vec::new(),
            gate: Some(gate),
            ready: Some(ready),
            record: None,
            completion: None,
        };
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                let handlers = scope.call(&create, ()).await.unwrap();
                let (value, ()) = future::join(scope.call(&run, (handlers.clone(),)), async move {
                    waiting.await.unwrap();
                    release.send(()).unwrap();
                })
                .await;
                assert_eq!(value, Ok((43.into(), 7.into(), 8.into())));
                let error = scope.call(&invalid, (handlers,)).await.unwrap_err();
                assert_eq!(
                    error.to_string(),
                    "host function application::library.apply failed: native value does not match the registered callback input"
                );
                let error = scope.call(&failing, ()).await.unwrap_err();
                assert_eq!(error.to_string(), "panic: callback failure");
            }),
        )
        .unwrap();
        assert_eq!(state.units.len(), 4);
        assert_ne!(state.units[0], state.units[1]);
        assert!(state.units[1..].iter().all(|unit| *unit == state.units[1]));
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["//fn(a) { ... }"]
        );
        assert_eq!(
            <Handlers as NamedTypeSchema>::arguments(),
            Vec::<ValueType>::new()
        );
    }

    #[test]
    fn native_completion_checks_its_return_type_and_preserves_operation_failure() {
        let (builder, output) = program()
            .function(FunctionDeclaration::<(), BigInt>::new("output"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        for (completion, expected) in [
            (None, Ok("42")),
            (
                Some(Ok(NativeValue::symbol("not_an_integer"))),
                Err(
                    "host function application::library.native_output failed: native value does not match the registered return type",
                ),
            ),
            (
                Some(Err(
                    crate::HostFailure::new("native operation failed").into()
                )),
                Err(
                    "host function application::library.native_output failed: native operation failed",
                ),
            ),
        ] {
            let host = TestHost::default();
            let mut state = State {
                units: Vec::new(),
                gate: None,
                ready: None,
                record: None,
                completion,
            };
            let result = host
                .block_on(module.with_execution(
                    &host,
                    &mut state,
                    &mut Vec::new(),
                    async |scope| scope.call(&output, ()).await,
                ))
                .unwrap()
                .map(|value| value.to_string())
                .map_err(|error| error.to_string());
            assert_eq!(result.as_deref().map_err(String::as_str), expected);
        }
    }
}
