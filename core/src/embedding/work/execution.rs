use super::return_::ScopedReturn;
use super::{Completed, ExecutionGuard, Future, ScopeBrand, ScopedOutput, SharedValue};
use crate::embedding::input::{InputShape, ScopedArgumentsInput};
use crate::embedding::{CallError, Function, HostedModule};
use crate::host::{HostFutureStore, HostProfile, HostWorkProfile, HostWorkSchema};
use crate::plan::execution::LibraryFunctionEntries;
use crate::runtime::work::driver::Driver;
pub use crate::runtime::{ObservationError, SharedExecutionError};
use std::sync::Arc;

/// An attachment to the application's original state, capabilities, and Echo.
pub struct ExecutionScope<'scope, 'host, Profile: HostProfile> {
    driver: Driver<'host, Profile>,
    entries: &'host LibraryFunctionEntries,
    owner: &'host Arc<()>,
    store: HostFutureStore,
    brand: ScopeBrand<'scope>,
}

impl<Profile: HostWorkProfile> HostedModule<Profile> {
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
        let (execution, stores) = self.execution.parts_mut();
        let store = crate::host::work_store::<Profile>(stores).clone_handle();
        ExecutionScope {
            driver: Driver::new(execution, state, stores, echo),
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
    ) -> Result<Return::Value<'scope>, CallError>
    where
        Args:
            ScopedArgumentsInput<Input, ScopeBrand<'scope>> + ScopedOutput<HostWorkSchema<Profile>>,
        Return: ScopedReturn<HostWorkSchema<Profile>>,
        Shape: InputShape<Input>,
    {
        if !Arc::ptr_eq(&function.owner, self.owner) {
            return Err(CallError::ForeignFunction);
        }
        if !Args::owners_match(&arguments, self.owner) {
            return Err(CallError::ForeignValue);
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
        .map_err(CallError::Execution)
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

#[cfg(test)]
mod tests {
    use crate::embedding::HostedModuleBuilder;
    use crate::embedding::{CallError, FunctionDeclaration, List, with_execution_scope};
    use crate::frontend::{HostedTypedProgram, compile_typed_host_program};
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet};
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
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
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
    fn foreign_functions_and_retained_values_fail_before_source_execution() {
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
                Some(CallError::ForeignFunction)
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
                Some(CallError::ForeignValue)
            );
        })
        .now_or_never()
        .expect("owner check is immediate");
        assert!(right_echo.0.is_empty());
        let retained = with_execution_scope(async |guard| {
            let mut scope = left.attach(guard, &mut state, &mut left_echo);
            assert_eq!(
                scope.call(&foreign, (&values,)).err(),
                Some(CallError::ForeignFunction)
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
}
