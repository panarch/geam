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
pub struct ExecutionScope<'scope, 'module, Profile: HostProfile> {
    context: EntryContext<Profile>,
    entries: &'module crate::plan::execution::LibraryFunctionEntries,
    owner: &'module Arc<()>,
    brand: ScopeBrand<'scope>,
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
        let (plan, stores) = self.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            host,
            state,
            stores,
            echo,
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &self.entries,
            owner: &self.owner,
            brand: ScopeBrand::new(),
        };
        domain.drive(run(scope)).await
    }
}

impl<'scope, Profile: HostProfile> ExecutionScope<'scope, '_, Profile> {
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
            let context = Return::context(self.brand, retention, self.owner);
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
    use crate::embedding::{BigInt, EcoString, FunctionDeclaration, HostedModuleBuilder, List};
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
    fn drives_typed_plain_data_on_the_callers_runtime_without_a_work_component() {
        type Scalars = (BigInt, f64, EcoString, crate::BitArrayValue, char, bool, ());
        type Data = (
            Scalars,
            Result<BigInt, EcoString>,
            List<(BigInt, EcoString)>,
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
                (List<(BigInt, EcoString)>,),
                List<(BigInt, EcoString)>,
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
            Some((BigInt::from(5), EcoString::from("five")))
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
    use crate::embedding::HostedModuleBuilder;
    use crate::embedding::{CallError, FunctionDeclaration, List};
    use crate::frontend::{HostedTypedProgram, compile_typed_host_program};
    use crate::host::{HostProfile, HostProviderSet};
    use crate::{EchoOutput, EchoSink, ModuleSource, PackageSource, PlanError};
    use ecow::EcoString;

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
    fn closing_the_domain_cancels_a_call_waiting_for_output_retention() {
        use super::{Domain, ExecutionScope, ScopeBrand};
        use std::future::Future;
        use std::sync::Arc;
        use std::task::{Context, Poll, Waker};

        type Strings = List<EcoString>;
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
        let (plan, stores) = module.execution.parts_mut();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let scope = ExecutionScope {
            context: domain.context(),
            entries: &module.entries,
            owner: &module.owner,
            brand: ScopeBrand::new(),
        };
        let mut call = std::pin::pin!(scope.call(&run, (vec![EcoString::from("retained")],)));
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
    fn foreign_functions_and_retained_values_fail_before_source_execution() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
pub fn keep(values: List(String)) {
  echo "entered"
  values
}
"#;
        type Strings = List<EcoString>;
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
                            .call(&foreign, (vec![EcoString::from("unused")],))
                            .await
                            .err(),
                        Some(CallError::ForeignFunction)
                    );
                    scope
                        .call(&keep, (vec![EcoString::from("first"), "second".into()],))
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
            Some(EcoString::from("second"))
        );
        assert_eq!(left_echo.0.len(), 2);
    }

    #[test]
    fn plain_scalar_tuple_result_and_list_returns_remain_direct() {
        let execution_host = crate::execution_fixture::TestHost::default();

        use crate::runtime::BitArrayValue;
        type Scalars = (BigInt, f64, EcoString, BitArrayValue, char, bool, ());
        type Data = (
            Scalars,
            Result<BigInt, EcoString>,
            List<(BigInt, EcoString)>,
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
            EcoString::from("three"),
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
                    for choice in [Ok(BigInt::from(4)), Err(EcoString::from("failed"))] {
                        let (actual, result, rows) = scope
                            .call(
                                &identity,
                                (
                                    scalars.clone(),
                                    choice.clone(),
                                    vec![(BigInt::from(5), EcoString::from("five"))],
                                ),
                            )
                            .await
                            .expect("direct call");
                        assert_eq!(actual, scalars);
                        assert_eq!(result, choice);
                        assert_eq!(
                            rows.read_item(0, |(number, text)| (number.clone(), text.clone())),
                            Some((BigInt::from(5), EcoString::from("five")))
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
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
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
        type Results = List<Result<BigInt, EcoString>>;
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
                            (vec![Ok(BigInt::from(42)), Err(EcoString::from("failed"))],),
                        )
                        .await
                        .expect("result items");
                    assert_eq!(
                        choices.read_item(0, |value| value.cloned().map_err(Clone::clone)),
                        Some(Ok(BigInt::from(42)))
                    );
                    assert_eq!(
                        choices.read_item(1, |value| value.cloned().map_err(Clone::clone)),
                        Some(Err(EcoString::from("failed")))
                    );
                },
            ))
            .expect("no Future allocation or suspension for direct entries");
        assert!(echo.0.is_empty());
    }
}
