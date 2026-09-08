use super::return_::ScopedReturn;
use super::{Completed, ExecutionGuard, Future, ScopeBrand, ScopedOutput, SharedValue};
use crate::embedding::binding::{BindingBuilder, BindingParts, Bindings};
use crate::embedding::input::{AsyncArgumentsInput, InputShape};
use crate::embedding::{Arguments, AsyncCallError, BindingError, Function, FunctionDeclaration};
use crate::host::{HostFutureStore, HostProfile, HostWorkProfile, HostWorkSchema};
use crate::plan::TransferHostedLibraryModulePlan;
use crate::plan::execution::{
    HostSpecializationError, LibraryFunctionEntries, TransferHostedExecution,
};
pub use crate::runtime::SharedExecutionError;
use crate::runtime::work::driver::Driver;
use std::sync::Arc;

/// Plans ordinary and work-valued functions for one Rust host.
pub struct WorkModuleBuilder<Profile: HostProfile> {
    inner: BindingBuilder<TransferHostedLibraryModulePlan<Profile>>,
}

/// Selects statically typed entries before sealing their shared execution.
pub struct WorkModuleBindings<Profile: HostProfile> {
    inner: Bindings<TransferHostedLibraryModulePlan<Profile>>,
}

/// One loaded execution and its persistent provider stores, shared across scopes.
pub struct WorkModule<Profile: HostProfile> {
    execution: TransferHostedExecution<Profile>,
    stores: Profile::ExternalStores,
    entries: LibraryFunctionEntries,
    owner: Arc<()>,
}

/// An attachment to the application's original state, capabilities, and Echo.
pub struct ExecutionScope<'scope, 'host, Profile: HostProfile> {
    driver: Driver<'host, Profile>,
    entries: &'host LibraryFunctionEntries,
    owner: &'host Arc<()>,
    store: HostFutureStore,
    brand: ScopeBrand<'scope>,
}

/// Work termination is separate from a source `Result` value.
#[derive(Clone)]
pub enum ObservationError {
    /// The operation was cancelled before completing.
    Cancelled,
    /// A source panic or native failure at its original site.
    Execution(SharedExecutionError),
}

impl<Profile: HostProfile> WorkModuleBuilder<Profile> {
    /// Plans all supplied source and registered host bodies.
    ///
    /// Transferable execution requires Send state, but not Sync:
    ///
    /// ```compile_fail
    /// use geam_core::{HostProfile, frontend::TransferHostedTypedProgram};
    /// use geam_core::embedding::WorkModuleBuilder;
    /// struct Local;
    /// impl HostProfile for Local {
    ///     type RunState = std::rc::Rc<()>;
    ///     type ExternalStores = ();
    /// }
    /// fn build(program: TransferHostedTypedProgram<Local>) {
    ///     let _ = WorkModuleBuilder::new(program);
    /// }
    /// ```
    pub fn new(
        program: crate::frontend::TransferHostedTypedProgram<Profile>,
    ) -> Result<Self, crate::PlanError>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
    {
        let public = program.root_public_functions().cloned().collect();
        crate::planner::plan_transfer_host_library_program(program).map(|plan| Self {
            inner: BindingBuilder::new(plan, public),
        })
    }

    /// Validates the first function's source signature.
    #[allow(private_bounds)]
    pub fn function<Args, Return>(
        self,
        declaration: FunctionDeclaration<Args, Return>,
    ) -> Result<(WorkModuleBindings<Profile>, Function<Args, Return>), BindingError>
    where
        Profile: HostWorkProfile,
        Args: Arguments + ScopedOutput<HostWorkSchema<Profile>>,
        Return: ScopedReturn<HostWorkSchema<Profile>>,
    {
        self.inner
            .function(declaration, Return::library_type())
            .map(|(inner, function)| (WorkModuleBindings { inner }, function))
    }
}

impl<Profile: HostProfile> WorkModuleBindings<Profile> {
    /// Validates another entry without changing earlier call signatures.
    #[allow(private_bounds)]
    pub fn function<Args, Return>(
        &mut self,
        declaration: FunctionDeclaration<Args, Return>,
    ) -> Result<Function<Args, Return>, BindingError>
    where
        Profile: HostWorkProfile,
        Args: Arguments + ScopedOutput<HostWorkSchema<Profile>>,
        Return: ScopedReturn<HostWorkSchema<Profile>>,
    {
        self.inner.function(declaration, Return::library_type())
    }

    /// Seals all selected entries into the same direct execution.
    pub fn seal(self) -> Result<WorkModule<Profile>, HostSpecializationError> {
        let BindingParts {
            plan,
            first,
            remaining,
            owner,
        } = self.inner.into_parts();
        let (execution, entries) =
            TransferHostedExecution::from_library_plan(plan, first, remaining)?;
        Ok(WorkModule {
            execution,
            entries,
            owner,
            stores: Profile::ExternalStores::default(),
        })
    }
}

impl<Profile: HostWorkProfile> WorkModule<Profile> {
    /// Attaches one fresh execution lifetime to explicit caller-owned resources.
    pub fn attach<'scope, 'host>(
        &'host mut self,
        guard: ExecutionGuard<'scope>,
        state: &'host mut Profile::RunState,
        echo: &'host mut (dyn crate::EchoSink + Send),
    ) -> ExecutionScope<'scope, 'host, Profile>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
    {
        let store = crate::host::work_store::<Profile>(&self.stores).clone_handle();
        ExecutionScope {
            driver: Driver::new(&self.execution, state, &mut self.stores, echo),
            entries: &self.entries,
            owner: &self.owner,
            store,
            brand: guard.into_brand(),
        }
    }
}

impl<'scope, Profile: HostWorkProfile> ExecutionScope<'scope, '_, Profile> {
    /// Calls an entry immediately. A source Future return constructs work.
    #[allow(private_bounds)]
    pub fn call<Args, Return, Input, Shape>(
        &mut self,
        function: &Function<Args, Return, Shape>,
        arguments: Input,
    ) -> Result<Return::Value<'scope>, AsyncCallError>
    where
        Args:
            AsyncArgumentsInput<Input, ScopeBrand<'scope>> + ScopedOutput<HostWorkSchema<Profile>>,
        Return: ScopedReturn<HostWorkSchema<Profile>>,
        Shape: InputShape<Input>,
    {
        if !Arc::ptr_eq(&function.owner, self.owner) {
            return Err(AsyncCallError::ForeignFunction);
        }
        if !Args::owners_match(&arguments, self.owner) {
            return Err(AsyncCallError::ForeignValue);
        }
        let constructions = Return::input_constructions(self.entries, function.slot);
        let inputs = Args::into_inputs(arguments, constructions);
        Return::call(
            &mut self.driver,
            self.entries,
            function.slot,
            inputs,
            self.brand,
            &self.store,
            self.owner,
        )
        .map_err(AsyncCallError::Execution)
    }

    /// Drives one observation using the executor polling this Rust Future.
    #[allow(private_bounds)]
    pub async fn observe<Value: SharedValue>(
        &self,
        work: &Future<'scope, Value, HostWorkSchema<Profile>>,
    ) -> Result<Completed<Value>, ObservationError> {
        let operation = work.work();
        let completed = self
            .driver
            .observe(&operation)
            .await
            .map_err(|_| ObservationError::Cancelled)?;
        match completed.read(Clone::clone) {
            Ok(value) => Ok(Completed::new(value, work.output_context())),
            Err(error) => Err(ObservationError::Execution(SharedExecutionError(error))),
        }
    }
}

impl std::fmt::Debug for ObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Cancelled"),
            Self::Execution(error) => f.debug_tuple("Execution").field(error).finish(),
        }
    }
}

impl std::fmt::Display for ObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("the Future operation was cancelled"),
            Self::Execution(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl std::error::Error for ObservationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cancelled => None,
            Self::Execution(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ObservationError, WorkModuleBuilder};
    use crate::embedding::{AsyncCallError, FunctionDeclaration, List, with_execution_scope};
    use crate::frontend::{TransferHostedTypedProgram, compile_typed_transfer_host_program};
    use crate::host::{
        AsyncHostComponentProfile, HostFutureStore, HostProfile, TransferHostProviderSet,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{EchoOutput, EchoSink, ModuleSource, PackageSource, PlanError};
    use ecow::EcoString;
    use futures_util::FutureExt;
    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for Profile {
        fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    #[derive(Default)]
    struct Echo(Vec<String>);
    impl EchoSink for Echo {
        fn emit(&mut self, value: EchoOutput) {
            self.0.push(value.to_string());
        }
    }

    fn program(source: &str) -> TransferHostedTypedProgram<Profile> {
        compile_typed_transfer_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            TransferHostProviderSet::new([]).expect("empty providers"),
        )
        .expect("source typing")
    }

    #[test]
    fn preserves_library_planning_failures_in_unused_source() {
        let error = WorkModuleBuilder::new(program(
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
    fn foreign_functions_and_retained_values_fail_before_source_execution() {
        let source = r#"
pub fn keep(values: List(String)) {
  echo "entered"
  values
}
"#;
        type Strings = List<EcoString>;
        let (left, keep) = WorkModuleBuilder::new(program(source))
            .expect("left plan")
            .function(FunctionDeclaration::<(Strings,), Strings>::new("keep"))
            .expect("left binding");
        let (right, foreign) = WorkModuleBuilder::new(program(source))
            .expect("right plan")
            .function(FunctionDeclaration::<(Strings,), Strings>::new("keep"))
            .expect("right binding");
        let mut left = left.seal().expect("left seal");
        let mut right = right.seal().expect("right seal");
        let mut state = ();
        assert!(std::ptr::eq(
            <WorkComponent as crate::HostProvider<Profile>>::project(&mut state),
            &state,
        ));
        let mut left_echo = Echo::default();
        let values = with_execution_scope(async |guard| {
            let mut scope = left.attach(guard, &mut state, &mut left_echo);
            assert_eq!(
                scope
                    .call(&foreign, (vec![EcoString::from("unused")],))
                    .err(),
                Some(AsyncCallError::ForeignFunction)
            );
            scope
                .call(&keep, (vec![EcoString::from("first"), "second".into()],))
                .expect("fresh input")
        })
        .now_or_never()
        .expect("ordinary calls are immediate");
        assert_eq!(left_echo.0, ["src/library.gleam:3\n\"entered\""]);
        let mut right_echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = right.attach(guard, &mut state, &mut right_echo);
            assert_eq!(
                scope.call(&foreign, (&values,)).err(),
                Some(AsyncCallError::ForeignValue)
            );
        })
        .now_or_never()
        .expect("owner check is immediate");
        assert!(right_echo.0.is_empty());
        let retained = with_execution_scope(async |guard| {
            let mut scope = left.attach(guard, &mut state, &mut left_echo);
            assert_eq!(
                scope.call(&foreign, (&values,)).err(),
                Some(AsyncCallError::ForeignFunction)
            );
            scope.call(&keep, (&values,)).expect("same loaded owner")
        })
        .now_or_never()
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
        let (mut bindings, identity) = WorkModuleBuilder::new(program(source))
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
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
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
                    .expect("direct call");
                assert_eq!(actual, scalars);
                assert_eq!(result, choice);
                assert_eq!(
                    rows.read_item(0, |(number, text)| (number.clone(), text.clone())),
                    Some((BigInt::from(5), EcoString::from("five")))
                );
                assert_eq!(rows.read_item(1, |_| ()), None);
            }
            assert!(scope.call(&empty, ()).expect("empty list").is_empty());
        })
        .now_or_never()
        .expect("every ordinary entry is immediate");
        assert!(echo.0.is_empty());
    }

    #[test]
    fn each_plain_return_family_uses_its_direct_entry_and_preserves_nested_data() {
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
        let (mut bindings, integer) = WorkModuleBuilder::new(program(source))
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
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            assert_eq!(scope.call(&integer, (42.into(),)), Ok(42.into()));
            assert_eq!(scope.call(&float, (2.5,)), Ok(2.5));
            assert_eq!(scope.call(&string, ("direct".into(),)), Ok("direct".into()));
            let value = BitArrayValue::try_from_parts(vec![0b1010_0000], 3).expect("three bits");
            assert_eq!(scope.call(&bits, (value.clone(),)), Ok(value));
            assert_eq!(scope.call(&codepoint, ('x',)), Ok('x'));
            assert_eq!(scope.call(&boolean, (true,)), Ok(true));
            assert_eq!(scope.call(&nil, ((),)), Ok(()));
            let nested = scope
                .call(&lists, (vec![vec![(), ()], vec![]],))
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
                .expect("result items");
            assert_eq!(
                choices.read_item(0, |value| value.cloned().map_err(Clone::clone)),
                Some(Ok(BigInt::from(42)))
            );
            assert_eq!(
                choices.read_item(1, |value| value.cloned().map_err(Clone::clone)),
                Some(Err(EcoString::from("failed")))
            );
        })
        .now_or_never()
        .expect("no Future allocation or suspension for direct entries");
        assert!(echo.0.is_empty());
    }

    #[test]
    fn cancelled_observation_has_its_own_lifecycle_message_without_an_error_source() {
        use std::error::Error;
        let cancelled = ObservationError::Cancelled;
        assert_eq!(format!("{cancelled:?}"), "Cancelled");
        assert_eq!(cancelled.to_string(), "the Future operation was cancelled");
        assert!(cancelled.source().is_none());
    }

    #[test]
    fn failed_observation_keeps_the_shared_execution_diagnostic_as_its_source() {
        use crate::runtime::SharedExecutionError;
        use crate::runtime::shared::Shared;
        use crate::{AsyncExecutionError, InvariantError, ValueType};
        use std::error::Error;
        let error = ObservationError::Execution(SharedExecutionError(Shared::new(
            AsyncExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                item_type: ValueType::Int,
                index: 1,
                length: 0,
            }),
        )));
        assert_eq!(
            error.to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
        assert_eq!(
            format!("{error:?}"),
            "Execution(Invariant(ListIndexOutOfBounds { item_type: Int, index: 1, length: 0 }))"
        );
        assert_eq!(
            error
                .source()
                .expect("shared execution failure")
                .to_string(),
            "list index out of bounds for Int list (index 1, length 0)"
        );
    }
}
