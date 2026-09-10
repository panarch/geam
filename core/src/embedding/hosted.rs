use super::binding::{BindingBuilder, BindingParts, Bindings};
use super::{Arguments, BindingError, EmbeddingValue, Function, FunctionDeclaration};
use crate::PlanError;
use crate::frontend::HostedTypedProgram;
use crate::host::HostProfile;
use crate::plan::HostedLibraryModulePlan;
use crate::plan::execution::{HostSpecializationError, HostedExecution, LibraryFunctionEntries};
use std::sync::Arc;

/// Plans a hosted Gleam project before selecting its first embedded function.
///
/// Provider schemas and implementations are fixed by the supplied
/// [`HostedTypedProgram`]. The first successful [`Self::function`] call creates
/// a non-empty [`HostedModuleBindings`] owner.
pub struct HostedModuleBuilder<Profile: HostProfile> {
    inner: BindingBuilder<HostedLibraryModulePlan<Profile>>,
}

/// Collects one or more typed function bindings before hosted sealing.
pub struct HostedModuleBindings<Profile: HostProfile> {
    inner: Bindings<HostedLibraryModulePlan<Profile>>,
}

/// One sealed hosted execution shared by all selected function handles.
///
/// The module owns immutable execution data and provider external stores.
/// Mutable provider state remains caller-owned and is supplied to every call.
pub struct HostedModule<Profile: HostProfile> {
    pub(in crate::embedding) execution: HostedExecution<Profile>,
    pub(in crate::embedding) entries: LibraryFunctionEntries,
    pub(in crate::embedding) owner: Arc<()>,
}

impl<Profile: HostProfile> HostedModuleBuilder<Profile> {
    /// Plans every source and provider body without requiring a `main` function.
    pub fn new(program: HostedTypedProgram<Profile>) -> Result<Self, PlanError> {
        let public_functions = program.root_public_functions().cloned().collect();
        let plan = crate::planner::plan_host_library_program(program)?;
        Ok(Self {
            inner: BindingBuilder::new(plan, public_functions),
        })
    }

    /// Selects the first function and creates a non-empty binding owner.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<
        (
            HostedModuleBindings<Profile>,
            Function<ArgumentsType, Return>,
        ),
        BindingError,
    >
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner
            .function(declaration, Return::library_type())
            .map(|(inner, function)| (HostedModuleBindings { inner }, function))
    }
}

impl<Profile: HostProfile> HostedModuleBindings<Profile> {
    /// Validates and selects another named function for the shared execution.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, BindingError>
    where
        ArgumentsType: Arguments,
        Return: EmbeddingValue,
    {
        self.inner.function(declaration, Return::library_type())
    }

    /// Seals every selected entry and its reachable provider specializations.
    pub fn seal(self) -> Result<HostedModule<Profile>, HostSpecializationError> {
        let BindingParts {
            plan,
            first,
            remaining,
            owner,
        } = self.inner.into_parts();
        let (execution, entries) = HostedExecution::try_from_library_plan(plan, first, remaining)?;
        Ok(HostedModule {
            execution,
            entries,
            owner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HostedModuleBindings, HostedModuleBuilder};
    use crate::embedding::{
        Arguments, BindingError, CallError, Function, FunctionDeclaration, ReturnValue,
    };
    use crate::planner::UnsupportedBitArraySegmentReason;
    use crate::{
        BitArrayValue, ExecutionError, FunctionType, HostCall, HostCallCompletion, HostCallError,
        HostCallable, HostFailure, HostFunctionType, HostModule, HostProfile, HostProvider,
        HostProviderModule, HostProviderSet, HostTypeListEnd, HostTypeParameter, ModuleSource,
        PackageSource, PanicKind, PanicSite, PlanError, SourceSpan, StatelessHostProfile, Value,
        ValueType, compile_typed_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::convert::Infallible;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct StatefulProfile;

    #[derive(Default)]
    struct RunState {
        calls: usize,
    }

    struct Counter;

    impl HostProfile for StatefulProfile {
        type RunState = RunState;
        type ExternalStores = ();
    }

    impl HostProvider<StatefulProfile> for Counter {
        type State = RunState;

        fn project(state: &mut RunState) -> &mut Self::State {
            state
        }
    }

    fn next<'call>(
        mut call: HostCall<'call, StatefulProfile, Counter, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        call.state().calls += 1;
        let value = BigInt::from(call.state().calls);
        Ok(call.return_value(value))
    }

    fn stop<'call>(
        mut call: HostCall<'call, StatefulProfile, Counter, BigInt>,
    ) -> Result<Infallible, HostCallError> {
        call.state().calls += 1;
        Err(HostFailure::new("stopped").into())
    }

    type IntCallback = HostFunctionType<HostTypeListEnd, BigInt>;

    fn around<'call>(
        call: HostCall<'call, StatefulProfile, Counter, BigInt>,
        constructions: crate::HostConstructions<'call, HostTypeListEnd>,
        callback: HostCallable<'call, HostTypeListEnd, BigInt>,
    ) -> Result<crate::HostCallContinuation<'call, BigInt>, HostCallError> {
        let callback = call.owned_callable(callback, &constructions);
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let value = callback
                    .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                    .await?;
                Ok(crate::HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn stateful_hosts() -> HostProviderSet<StatefulProfile> {
        let counter = HostModule::<StatefulProfile>::new_for_profile(
            "host_support",
            "host/counter",
        )
        .expect("counter module should be valid")
        .with_scoped_function::<Counter, (), BigInt, _>("next", next)
        .expect("next should register")
        .with_scoped_diverging_function::<Counter, (), BigInt, _>("stop", stop)
        .expect("stop should register")
        .with_resumable_function::<Counter, (IntCallback,), BigInt, geam_core::HostTypeListEnd, _>(
            "around", around,
        )
        .expect("around should register");
        HostProviderSet::new([counter]).expect("counter module should be unique")
    }

    fn stateful_builder(source: &str) -> HostedModuleBuilder<StatefulProfile> {
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            stateful_hosts(),
        )
        .expect("stateful source should compile");
        HostedModuleBuilder::new(program).expect("stateful source should plan")
    }

    fn stateless_builder(source: &str) -> HostedModuleBuilder<StatelessHostProfile> {
        let hosts = HostProviderSet::new(Vec::<HostModule>::new())
            .expect("empty host module set should be valid");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            hosts,
        )
        .expect("stateless source should compile");
        HostedModuleBuilder::new(program).expect("stateless source should plan")
    }

    fn stateless_option_builder(source: &str) -> HostedModuleBuilder<StatelessHostProfile> {
        let hosts = HostProviderSet::new(Vec::<HostModule>::new())
            .expect("empty host module set should be valid");
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "gleam_stdlib/src/gleam/option.gleam",
                        "pub type Option(value) { Some(value) None }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new("library", "src/library.gleam", source)],
                ),
            ],
            hosts,
        )
        .expect("stateless Option source should compile");
        HostedModuleBuilder::new(program).expect("stateless Option source should plan")
    }

    #[test]
    fn preserves_library_planning_failures() {
        let hosts = HostProviderSet::new(Vec::<HostModule>::new())
            .expect("empty host module set should be valid");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn unsupported() { <<1:native>> }",
                )],
            )],
            hosts,
        )
        .expect("native-endian source should type-check");
        let error = HostedModuleBuilder::new(program)
            .err()
            .expect("native-endian source should fail planning");

        assert_eq!(
            error,
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    fn bind<Profile, ArgumentsType, Return>(
        bindings: &mut HostedModuleBindings<Profile>,
        name: &str,
    ) -> Function<ArgumentsType, Return>
    where
        Profile: HostProfile,
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        bindings
            .function(FunctionDeclaration::<ArgumentsType, Return>::new(name))
            .expect("function should bind")
    }

    #[test]
    fn calls_every_scalar_family_through_the_shared_typed_contract() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateless_builder(
            r#"
pub fn keep_int(value: Int) { value }
pub fn keep_float(value: Float) { value }
pub fn keep_string(value: String) { value }
pub fn keep_bits(value: BitArray) { value }
pub fn keep_codepoint(value: UtfCodepoint) { value }
pub fn keep_bool(value: Bool) { value }
pub fn keep_nil(value: Nil) { value }

pub fn mixed(
  _int: Int,
  _float: Float,
  _string: String,
  _bits: BitArray,
  _codepoint: UtfCodepoint,
  value: Bool,
  _nil: Nil,
) {
  value
}
"#,
        );
        let (mut bindings, int) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("keep_int"))
            .expect("first function should bind");
        let float = bind::<_, (f64,), f64>(&mut bindings, "keep_float");
        let string = bind::<_, (EcoString,), EcoString>(&mut bindings, "keep_string");
        let bits = bind::<_, (BitArrayValue,), BitArrayValue>(&mut bindings, "keep_bits");
        let codepoint = bind::<_, (char,), char>(&mut bindings, "keep_codepoint");
        let bool_ = bind::<_, (bool,), bool>(&mut bindings, "keep_bool");
        let nil = bind::<_, ((),), ()>(&mut bindings, "keep_nil");
        let mixed = bind::<_, (BigInt, f64, EcoString, BitArrayValue, char, bool, ()), bool>(
            &mut bindings,
            "mixed",
        );
        let mut module = bindings.seal().expect("hosted entries should seal");
        let mut state = ();
        let mut echo = Vec::new();
        let bits_value = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&int, (BigInt::from(3),)).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(3)),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&float, (1.25,)).await
                ))
                .expect("controlled execution"),
            Ok(1.25)
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&string, ("value".into(),)).await
                ))
                .expect("controlled execution"),
            Ok("value".into()),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&bits, (bits_value.clone(),)).await
                ))
                .expect("controlled execution"),
            Ok(bits_value.clone()),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&codepoint, ('한',)).await
                ))
                .expect("controlled execution"),
            Ok('한')
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&bool_, (true,)).await
                ))
                .expect("controlled execution"),
            Ok(true)
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&nil, ((),)).await
                ))
                .expect("controlled execution"),
            Ok(())
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| {
                        scope
                            .call(
                                &mixed,
                                (
                                    BigInt::from(1),
                                    2.0,
                                    "three".into(),
                                    bits_value,
                                    '四',
                                    false,
                                    (),
                                ),
                            )
                            .await
                    }
                ))
                .expect("controlled execution"),
            Ok(false),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn reuses_one_execution_with_caller_owned_mutable_state() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateful_builder(
            r#"
import host/counter

pub fn next() { counter.next() }
"#,
        );
        let (bindings, next) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("next"))
            .expect("next should bind");
        let mut module = bindings.seal().expect("next should seal");
        let mut first = RunState::default();
        let mut second = RunState::default();

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut first,
                    &mut Vec::new(),
                    async |scope| scope.call(&next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(1)),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut first,
                    &mut Vec::new(),
                    async |scope| scope.call(&next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(2)),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut second,
                    &mut Vec::new(),
                    async |scope| scope.call(&next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(1)),
        );
        assert_eq!(first.calls, 2);
        assert_eq!(second.calls, 1);
    }

    #[test]
    fn retains_lists_after_host_state_drop_and_rejects_foreign_inputs_before_mutation() {
        let execution_host = crate::execution_fixture::TestHost::default();

        use crate::embedding::List;

        let source = r#"
import host/counter

pub fn batch(values: List(Result(#(String, Int), String))) {
  echo counter.next()
  values
}

pub fn inspect(values: List(Result(#(String, Int), String))) {
  let count = counter.next()
  echo count
  case values { [] -> 0 [_, ..] -> count }
}
"#;
        type Row = Result<(EcoString, BigInt), EcoString>;
        let collect_rows = |list: &crate::embedding::SharedList<Row>| {
            (0..list.len())
                .map(|index| {
                    list.read_item(index, |row| {
                        row.map(|(label, value)| (label.clone(), value.clone()))
                            .map_err(Clone::clone)
                    })
                    .expect("row in bounds")
                })
                .collect::<Vec<_>>()
        };
        let rows: Vec<Row> = vec![Ok(("A".into(), 4.into())), Err("invalid".into())];
        let retained = {
            let (mut bindings, batch) = stateful_builder(source)
                .function(FunctionDeclaration::<(List<Row>,), List<Row>>::new("batch"))
                .expect("hosted List entry");
            let inspect = bind::<_, (List<Row>,), BigInt>(&mut bindings, "inspect");
            let mut module = bindings.seal().expect("hosted List seal");
            let mut state = RunState::default();
            let mut echo = Vec::new();
            let retained = execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut echo,
                    async |scope| scope.call(&batch, (rows.clone(),)).await,
                ))
                .expect("controlled execution")
                .expect("new rows");
            assert_eq!(state.calls, 1);
            assert_eq!(
                execution_host
                    .block_on(module.with_execution(
                        &execution_host,
                        &mut state,
                        &mut echo,
                        async |scope| scope.call(&inspect, (&retained,)).await
                    ))
                    .expect("controlled execution"),
                Ok(BigInt::from(2))
            );
            assert_eq!(state.calls, 2);
            assert_eq!(collect_rows(&retained), rows);

            let (bindings, other_inspect) = stateful_builder(source)
                .function(FunctionDeclaration::<(List<Row>,), BigInt>::new("inspect"))
                .expect("independent owner");
            let mut other = bindings.seal().expect("independent seal");
            let mut other_state = RunState::default();
            let mut other_echo = Vec::new();
            assert_eq!(
                execution_host
                    .block_on(other.with_execution(
                        &execution_host,
                        &mut other_state,
                        &mut other_echo,
                        async |scope| scope.call(&other_inspect, (&retained,)).await
                    ))
                    .expect("controlled execution"),
                Err(CallError::ForeignValue)
            );
            assert_eq!(other_state.calls, 0);
            assert!(other_echo.is_empty());
            assert_eq!(
                execution_host
                    .block_on(other.with_execution(
                        &execution_host,
                        &mut other_state,
                        &mut other_echo,
                        async |scope| scope.call(&other_inspect, (collect_rows(&retained),)).await
                    ))
                    .expect("controlled execution"),
                Ok(BigInt::from(1))
            );
            assert_eq!(other_state.calls, 1);
            assert_eq!(other_echo.len(), 1);
            assert_eq!(echo.len(), 2);
            retained
        };
        assert_eq!(collect_rows(&retained), rows);
    }

    #[test]
    fn moves_recursive_values_with_caller_owned_state_and_echo() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateful_builder(
            r#"
import host/counter

pub fn inspect(value: Result(#(Int, String), #(Bool, Nil))) {
  let count = counter.next()
  echo count as "hosted call"
  #(value, count)
}

pub fn keep_result(value: Result(#(Int, String), #(Bool, Nil))) {
  let _ = counter.next()
  value
}
"#,
        );
        type Input = Result<(BigInt, EcoString), (bool, ())>;
        let (mut bindings, inspect) = builder
            .function(FunctionDeclaration::<(Input,), (Input, BigInt)>::new(
                "inspect",
            ))
            .expect("recursive hosted entry should bind");
        let keep_result = bind::<_, (Input,), Input>(&mut bindings, "keep_result");
        let mut module = bindings.seal().expect("recursive hosted entry should seal");
        let mut first_state = RunState::default();
        let mut second_state = RunState::default();
        let mut first_echo = Vec::new();
        let mut second_echo = Vec::new();

        let success = Ok((BigInt::from(3), "three".into()));
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut first_state,
                    &mut first_echo,
                    async |scope| scope.call(&inspect, (success.clone(),)).await
                ))
                .expect("controlled execution"),
            Ok((success, BigInt::from(1))),
        );
        let failure = Err((true, ()));
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut first_state,
                    &mut first_echo,
                    async |scope| scope.call(&inspect, (failure.clone(),)).await
                ))
                .expect("controlled execution"),
            Ok((failure, BigInt::from(2))),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut second_state,
                    &mut second_echo,
                    async |scope| scope.call(&inspect, (Err((false, ())),)).await
                ))
                .expect("controlled execution"),
            Ok((Err((false, ())), BigInt::from(1))),
        );
        assert_eq!(first_state.calls, 2);
        assert_eq!(second_state.calls, 1);
        assert_eq!(first_echo.len(), 2);
        assert_eq!(second_echo.len(), 1);
        assert_eq!(first_echo[0].message(), Some(&"hosted call".into()));
        assert_eq!(first_echo[0].value(), &Value::Int(BigInt::from(1)));
        assert_eq!(first_echo[1].value(), &Value::Int(BigInt::from(2)));
        assert_eq!(second_echo[0].value(), &Value::Int(BigInt::from(1)));
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut first_state,
                    &mut Vec::new(),
                    async |scope| {
                        scope
                            .call(&keep_result, (Ok((BigInt::from(4), "four".into())),))
                            .await
                    }
                ))
                .expect("controlled execution"),
            Ok(Ok((BigInt::from(4), "four".into()))),
        );
        assert_eq!(first_state.calls, 3);
    }

    #[test]
    fn moves_exact_option_values_through_the_hosted_custom_family() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateless_option_builder(
            r#"
import gleam/option.{type Option as Maybe}

pub fn keep(value: Maybe(Result(Int, String))) { value }
"#,
        );
        type Value = Option<Result<BigInt, EcoString>>;
        let (bindings, keep) = builder
            .function(FunctionDeclaration::<(Value,), Value>::new("keep"))
            .expect("hosted Option entry should bind");
        let mut module = bindings.seal().expect("hosted Option entry should seal");

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&keep, (Some(Ok(BigInt::from(5))),)).await
                ))
                .expect("controlled execution"),
            Ok(Some(Ok(BigInt::from(5)))),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&keep, (None,)).await
                ))
                .expect("controlled execution"),
            Ok(None),
        );
    }

    #[test]
    fn calls_a_public_root_provider_without_a_gleam_wrapper() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "library")
            .expect("provider module should be valid")
            .with_function("increment", |value: BigInt| value + 1)
            .expect("increment should register");
        let hosts = HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "increment")
pub fn increment(value: Int) -> Int
"#,
                )],
            )],
            hosts,
        )
        .expect("provider-backed source should compile");
        let builder = HostedModuleBuilder::new(program).expect("hosted source should plan");
        let (bindings, increment) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("increment"))
            .expect("public provider should bind");
        let mut module = bindings.seal().expect("public provider should seal");

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&increment, (BigInt::from(3),)).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(4)),
        );
    }

    #[test]
    fn invokes_a_successful_callback_with_caller_owned_state() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateful_builder(
            r#"
import host/counter

pub fn around_next() { counter.around(counter.next) }
"#,
        );
        let (bindings, around_next) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("around_next"))
            .expect("callback entry should bind");
        let mut module = bindings.seal().expect("callback entry should seal");
        let mut state = RunState::default();

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut state,
                    &mut Vec::new(),
                    async |scope| scope.call(&around_next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(1)),
        );
        assert_eq!(state.calls, 1);
    }

    struct Identity;

    impl HostProvider<StatelessHostProfile> for Identity {
        type State = ();

        fn project(state: &mut ()) -> &mut Self::State {
            state
        }
    }

    type GenericItem = HostTypeParameter<0>;

    static PRODUCE_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn produce<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, GenericItem>,
    ) -> Result<HostCallCompletion<'call, GenericItem>, HostCallError> {
        let _ = call.state();
        PRODUCE_CALLS.fetch_add(1, Ordering::SeqCst);
        Err(HostFailure::new("produce stopped").into())
    }

    fn generic_producer_builder(source: &str) -> HostedModuleBuilder<StatelessHostProfile> {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "library")
            .expect("provider module should be valid")
            .with_scoped_function::<Identity, (), GenericItem, _>("produce", produce)
            .expect("generic provider should register");
        let hosts = HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            hosts,
        )
        .expect("generic provider source should compile");
        HostedModuleBuilder::new(program).expect("hosted source should plan")
    }

    #[test]
    fn rejects_a_reachable_unrepresentable_provider_while_sealing() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let before = PRODUCE_CALLS.load(Ordering::SeqCst);
        let concrete = generic_producer_builder(
            r#"
@external(erlang, "native", "produce")
fn produce() -> value

pub fn concrete() -> Int {
  produce()
}
"#,
        );
        let (bindings, concrete) = concrete
            .function(FunctionDeclaration::<(), BigInt>::new("concrete"))
            .expect("concrete root should bind");
        let mut module = bindings.seal().expect("concrete provider should seal");
        let error = execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut (),
                &mut Vec::new(),
                async |scope| scope.call(&concrete, ()).await,
            ))
            .expect("controlled execution")
            .expect_err("concrete provider should return its failure");

        assert_eq!(
            error.to_string(),
            "host function application::library.produce failed: produce stopped"
        );
        assert_eq!(PRODUCE_CALLS.load(Ordering::SeqCst), before + 1);

        let unresolved = generic_producer_builder(
            r#"
@external(erlang, "native", "produce")
fn produce() -> value

pub fn selected() {
  let _ = produce()
  1
}
"#,
        );
        let (bindings, _) = unresolved
            .function(FunctionDeclaration::<(), BigInt>::new("selected"))
            .expect("concrete root should bind");
        let error = bindings
            .seal()
            .err()
            .expect("reachable unresolved provider should not seal");

        assert_eq!(error.package(), "application");
        assert_eq!(error.module(), "library");
        assert_eq!(error.function(), "produce");
        assert_eq!(PRODUCE_CALLS.load(Ordering::SeqCst), before + 1);
    }

    #[test]
    fn rejects_a_foreign_handle_before_invoking_its_provider() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
import host/counter

pub fn next() { counter.next() }
"#;
        let first = stateful_builder(source);
        let (first, first_next) = first
            .function(FunctionDeclaration::<(), BigInt>::new("next"))
            .expect("first next should bind");
        let mut first = first.seal().expect("first module should seal");
        let second = stateful_builder(source);
        let (second, second_next) = second
            .function(FunctionDeclaration::<(), BigInt>::new("next"))
            .expect("second next should bind");
        let mut second = second.seal().expect("second module should seal");
        let mut state = RunState::default();

        assert_eq!(
            execution_host
                .block_on(second.with_execution(
                    &execution_host,
                    &mut state,
                    &mut Vec::new(),
                    async |scope| scope.call(&first_next, ()).await
                ))
                .expect("controlled execution"),
            Err(CallError::ForeignFunction),
        );
        assert_eq!(state.calls, 0);
        assert_eq!(
            execution_host
                .block_on(first.with_execution(
                    &execution_host,
                    &mut state,
                    &mut Vec::new(),
                    async |scope| scope.call(&first_next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(1)),
        );
        assert_eq!(
            execution_host
                .block_on(second.with_execution(
                    &execution_host,
                    &mut state,
                    &mut Vec::new(),
                    async |scope| scope.call(&second_next, ()).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(2)),
        );
    }

    #[test]
    fn preserves_source_panic_identity() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
pub fn explode(_value: String) -> String { panic as "stopped" }
"#;
        let builder = stateless_builder(source);
        let (bindings, explode) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "explode",
            ))
            .expect("explode should bind");
        let mut module = bindings.seal().expect("explode should seal");
        let expression = "panic as \"stopped\"";
        let start = source
            .find(expression)
            .expect("fixture should contain panic expression");

        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&explode, ("value".into(),)).await
                ))
                .expect("controlled execution"),
            Err(CallError::Execution(ExecutionError::source_panic(
                Some(&crate::SourceContext::new("src/library.gleam", source)),
                PanicKind::Panic,
                Some("stopped".into()),
                PanicSite::new(
                    "library".into(),
                    "explode".into(),
                    SourceSpan::new(start, start + expression.len()),
                ),
            ))),
        );
    }

    #[test]
    fn preserves_provider_failure_identity_and_source_call_site() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
import host/counter

pub fn fail() { counter.stop() }
"#;
        let builder = stateful_builder(source);
        let (bindings, fail) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("fail"))
            .expect("fail should bind");
        let mut module = bindings.seal().expect("fail should seal");
        let mut state = RunState::default();
        let error = execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut state,
                &mut Vec::new(),
                async |scope| scope.call(&fail, ()).await,
            ))
            .expect("controlled execution")
            .expect_err("provider failure should cross the embedding boundary");

        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.package().as_str(),
                    error.module().as_str(),
                    error.function().as_str(),
                    error.failure(),
                    error.location().site().map(|site| (
                        site.module().as_str(),
                        site.function().as_str(),
                    )),
                    error.location().path().map(|path| path.as_str()),
                    error.location().line(),
                ) == (
                    "host_support",
                    "host/counter",
                    "stop",
                    &HostFailure::new("stopped"),
                    Some(("library", "fail")),
                    Some("src/library.gleam"),
                    Some(4),
                )
        ));
        assert_eq!(state.calls, 1);
    }

    #[test]
    fn preserves_nested_host_call_origin_identity() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let source = r#"
import host/counter

pub fn nested() { counter.around(counter.stop) }
"#;
        let builder = stateful_builder(source);
        let (bindings, nested) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("nested"))
            .expect("nested should bind");
        let mut module = bindings.seal().expect("nested should seal");
        let error = execution_host
            .block_on(module.with_execution(
                &execution_host,
                &mut RunState::default(),
                &mut Vec::new(),
                async |scope| scope.call(&nested, ()).await,
            ))
            .expect("controlled execution")
            .expect_err("nested provider failure should cross the embedding boundary");

        assert!(matches!(
            &error,
            CallError::Execution(ExecutionError::Host(error))
                if (
                    error.function().as_str(),
                    error.failure(),
                    error.location().caller().map(|caller| (
                        caller.package().as_str(),
                        caller.module().as_str(),
                        caller.function().as_str(),
                    )),
                ) == (
                    "stop",
                    &HostFailure::new("stopped"),
                    Some(("host_support", "host/counter", "around")),
                )
        ));
    }

    static STORE_DEFAULTS: AtomicUsize = AtomicUsize::new(0);

    struct CountingStores;

    impl Default for CountingStores {
        fn default() -> Self {
            STORE_DEFAULTS.fetch_add(1, Ordering::SeqCst);
            Self
        }
    }

    struct CountingProfile;

    impl HostProfile for CountingProfile {
        type RunState = ();
        type ExternalStores = CountingStores;
    }

    #[test]
    fn creates_one_external_store_owner_for_all_selected_functions() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let before = STORE_DEFAULTS.load(Ordering::SeqCst);
        let hosts = HostProviderSet::new(Vec::<HostModule<CountingProfile>>::new())
            .expect("empty host set should be valid");
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn first(value: Int) { value }\npub fn second(value: Int) { value + 1 }",
                )],
            )],
            hosts,
        )
        .expect("counting source should compile");
        let builder = HostedModuleBuilder::new(program).expect("counting source should plan");
        let (mut bindings, first) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("first"))
            .expect("first should bind");
        let second = bind::<_, (BigInt,), BigInt>(&mut bindings, "second");
        let mut module = bindings.seal().expect("counting module should seal");

        assert_eq!(STORE_DEFAULTS.load(Ordering::SeqCst), before + 1);
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&first, (BigInt::from(1),)).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(1)),
        );
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&second, (BigInt::from(1),)).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(2)),
        );
        assert_eq!(STORE_DEFAULTS.load(Ordering::SeqCst), before + 1);
    }

    #[test]
    fn keeps_shared_binding_diagnostics_for_hosted_roots() {
        let execution_host = crate::execution_fixture::TestHost::default();

        let builder = stateless_builder(
            r#"
fn private(value: String) { value }
pub fn generic(value) { value }
pub fn number(value: Int) { value }
pub fn words(value: String) { value }
"#,
        );
        let (mut bindings, number) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
            .expect("number should bind");

        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "missing",
                ))
                .err(),
            Some(BindingError::MissingFunction {
                name: "missing".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "private",
                ))
                .err(),
            Some(BindingError::NonPublicFunction {
                name: "private".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "generic",
                ))
                .err(),
            Some(BindingError::GenericFunction {
                name: "generic".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("words"))
                .err(),
            Some(BindingError::SignatureMismatch {
                name: "words".into(),
                expected: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                found: FunctionType::new(vec![ValueType::String], ValueType::String),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
                .err(),
            Some(BindingError::DuplicateFunction {
                name: "number".into(),
            }),
        );
        let mut module = bindings.seal().expect("valid binding should still seal");
        assert_eq!(
            execution_host
                .block_on(module.with_execution(
                    &execution_host,
                    &mut (),
                    &mut Vec::new(),
                    async |scope| scope.call(&number, (BigInt::from(7),)).await
                ))
                .expect("controlled execution"),
            Ok(BigInt::from(7)),
        );
    }
}
