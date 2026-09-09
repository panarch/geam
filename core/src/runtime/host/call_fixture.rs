use crate::host::{
    HostCall, HostCallArguments, HostCallCompletion, HostCallError, HostCustomArgumentSlot,
    HostCustomToken, HostExternalArgumentSlot, HostExternalToken, HostFunctionArgumentSlot,
    HostFunctionToken, HostListArgumentSlot, HostListToken, HostProfile, HostProvider,
    HostScopedValue, HostTupleArgumentSlot, HostTupleToken, HostTypeParameter, HostValue,
    HostValueArgumentSlot, HostValueFamily, HostValueToken, StatelessHostProfile,
};
use crate::host::{HostCallRuntime, HostTokenRuntime};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::host::scoped::{ScopedValues, StoredRuntimeList, StoredRuntimeValue};

pub(crate) struct TestHostProfile;
pub(crate) struct StatelessTestProvider;
pub(crate) type TestTypeParameter = HostTypeParameter<0>;

#[derive(Default)]
pub(crate) struct TestRunState {
    pub(crate) counter: usize,
    pub(crate) unrelated: bool,
}

pub(crate) struct TestHostCallRuntime<'state> {
    state: &'state mut TestRunState,
    arguments: Box<dyn HostCallArguments>,
    completed: Option<HostScopedValue>,
    external_leases: Vec<crate::host::ExternalPayloadLease>,
    list_builds: usize,
    execution: crate::HostedExecution<TestHostProfile>,
    work: crate::runtime::work::execution::ExecutionWork<TestHostProfile>,
    scoped: ScopedValues,
    lists: crate::runtime::RuntimeListStorage,
}

impl HostProfile for TestHostProfile {
    type RunState = TestRunState;
    type ExternalStores = ();
}

impl HostProvider<StatelessHostProfile> for StatelessTestProvider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

pub(crate) fn stateless_identity<'call>(
    call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, TestTypeParameter>,
    value: HostValue<'call, TestTypeParameter>,
) -> Result<HostCallCompletion<'call, TestTypeParameter>, HostCallError> {
    Ok(call.return_value(value))
}

impl<'state> TestHostCallRuntime<'state> {
    pub(crate) fn new(
        state: &'state mut TestRunState,
        arguments: impl HostCallArguments + 'static,
    ) -> Self {
        let host = crate::HostModule::<TestHostProfile>::new_for_profile("native", "native")
            .expect("fixture module")
            .with_function("value", || num_bigint::BigInt::from(0))
            .expect("fixture function");
        let program = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                ["native"],
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    "import native\npub fn main() { native.value() }",
                )],
            )],
            crate::HostProviderSet::new([host]).expect("fixture hosts"),
        )
        .expect("fixture source");
        let plan = crate::plan_host_program(program).expect("fixture plan");
        let execution =
            crate::HostedExecution::try_from_module_plan(plan).expect("fixture execution");
        let mut scoped = ScopedValues::default();
        scoped.push(crate::runtime::evaluated::EvaluatedValue::Function(
            crate::runtime::evaluated::EvaluatedIntFunction::reference(
                crate::plan::execution::function::IntFunctionId(0),
                Vec::new(),
                Vec::new(),
                crate::plan::execution::type_::FunctionType::new(
                    Vec::new(),
                    crate::plan::execution::type_::ValueType::Int,
                ),
            )
            .into(),
        ));
        Self {
            state,
            arguments: Box::new(arguments),
            completed: None,
            external_leases: Vec::new(),
            list_builds: 0,
            execution,
            work: crate::runtime::work::execution::ExecutionWork::new(),
            scoped,
            lists: Default::default(),
        }
    }

    pub(crate) fn completed(&self) -> Option<&HostScopedValue> {
        self.completed.as_ref()
    }

    pub(crate) fn list_builds(&self) -> usize {
        self.list_builds
    }
}

impl HostCallRuntime<TestHostProfile> for TestHostCallRuntime<'_> {
    fn state(&mut self) -> &mut TestRunState {
        self.state
    }

    fn external_stores(&self) -> &() {
        &()
    }

    fn arguments(&self) -> &dyn HostCallArguments {
        self.arguments.as_ref()
    }

    fn scalar_context(&mut self) -> (&mut TestRunState, &dyn HostCallArguments) {
        (self.state, self.arguments.as_ref())
    }

    fn value(&self, _slot: HostValueArgumentSlot) -> HostValueToken {
        HostValueToken {
            family: HostValueFamily::Bool,
            index: 0,
        }
    }

    fn list(&self, _slot: HostListArgumentSlot) -> HostListToken {
        HostListToken::Stored(0)
    }

    fn tuple(&self, _slot: HostTupleArgumentSlot) -> HostTupleToken {
        HostTupleToken(0)
    }

    fn custom(&self, _slot: HostCustomArgumentSlot) -> HostCustomToken {
        HostCustomToken(0)
    }

    fn external(&self, _slot: HostExternalArgumentSlot) -> HostExternalToken {
        HostExternalToken(0)
    }

    fn function(&self, _slot: HostFunctionArgumentSlot) -> HostFunctionToken {
        HostFunctionToken(0)
    }

    fn list_len(&self, _value: HostListToken) -> usize {
        0
    }

    fn list_item(&mut self, _value: HostListToken, _index: usize) -> Option<HostValueToken> {
        None
    }

    fn tuple_len(&self, _value: HostTupleToken) -> usize {
        0
    }

    fn tuple_values(&mut self, _value: HostTupleToken) -> Box<[HostValueToken]> {
        Box::new([])
    }

    fn custom_constructor(&self, _value: HostCustomToken) -> usize {
        0
    }

    fn custom_fields(&mut self, _value: HostCustomToken) -> Box<[HostValueToken]> {
        Box::new([])
    }

    fn take_custom_fields(&mut self, _value: HostCustomToken) -> Box<[HostValueToken]> {
        Box::new([])
    }

    fn invoke(
        &mut self,
        _function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, HostCallError> {
        match arguments.into_vec().into_iter().next() {
            Some(value) => Ok(self.complete(value)),
            None => Ok(token(HostValueFamily::Nil)),
        }
    }

    fn equal(&self, _left: HostScopedValue, _right: HostScopedValue) -> bool {
        false
    }

    fn source_hash(&self, _value: HostScopedValue) -> u64 {
        17
    }

    fn inspect(&self, _value: HostScopedValue) -> ecow::EcoString {
        "inspected".into()
    }

    fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
        let token = match &value {
            HostScopedValue::Value(token) => *token,
            HostScopedValue::Int(_) => token(HostValueFamily::Int),
            HostScopedValue::Float(_) => token(HostValueFamily::Float),
            HostScopedValue::String(_) => token(HostValueFamily::String),
            HostScopedValue::BitArray(_) => token(HostValueFamily::BitArray),
            HostScopedValue::UtfCodepoint(_) => token(HostValueFamily::UtfCodepoint),
            HostScopedValue::Bool(_) => token(HostValueFamily::Bool),
            HostScopedValue::Nil => token(HostValueFamily::Nil),
            HostScopedValue::List(_) => token(HostValueFamily::List),
            HostScopedValue::Tuple(_) => token(HostValueFamily::Tuple),
            HostScopedValue::Custom(_) => token(HostValueFamily::Custom),
            HostScopedValue::External(_) => token(HostValueFamily::External),
            HostScopedValue::Function(_) => token(HostValueFamily::Function),
        };
        self.completed = Some(value);
        token
    }

    fn build_list(
        &mut self,
        _type_: &crate::host::HostTypeDescriptor,
        _values: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        self.list_builds += 1;
        token(HostValueFamily::List)
    }

    fn build_tuple(&mut self, _values: Box<[HostScopedValue]>) -> HostValueToken {
        token(HostValueFamily::Tuple)
    }

    fn build_custom(
        &mut self,
        _type_: &crate::host::HostTypeDescriptor,
        _constructor: usize,
        _fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        token(HostValueFamily::Custom)
    }

    fn build_external(
        &mut self,
        _type_: &crate::host::HostTypeDescriptor,
        value: crate::host::ExternalPayloadLease,
    ) -> HostExternalToken {
        let index = self.external_leases.len();
        self.external_leases.push(value);
        HostExternalToken(index)
    }

    fn external_lease(&self, value: HostExternalToken) -> crate::host::ExternalPayloadLease {
        self.external_leases[value.0].clone()
    }

    fn resolve_host_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType> {
        descriptor.resolve(&[])
    }

    fn retain_stored(&self, _value: HostScopedValue) -> crate::runtime::StoredRuntimeValue {
        crate::runtime::StoredRuntimeValue::test_int(0.into())
    }

    fn retain_list(&self, _value: HostListToken) -> crate::runtime::StoredRuntimeList {
        crate::runtime::StoredRuntimeList::test_ints(vec![1.into()])
    }

    fn restore_stored(&mut self, _value: &crate::runtime::StoredRuntimeValue) -> HostValueToken {
        token(HostValueFamily::Int)
    }

    fn work(&self) -> crate::runtime::work::execution::WorkContext<TestHostProfile> {
        self.work.context()
    }

    fn origin(&self) -> crate::runtime::HostCallOrigin {
        crate::runtime::HostCallOrigin::Entry
    }

    fn callable(&self, function: HostFunctionToken) -> crate::runtime::RetainedCallable {
        self.scoped.function(function)
    }

    fn codec_scope(&self) -> crate::host::HostCodecScope {
        use crate::plan::execution::function::{IntFunctionId, ValueFunctionEntry};
        use crate::plan::execution::host::HostedFunctionTarget;
        let execution = self.execution.execution();
        let ValueFunctionEntry::Host(HostedFunctionTarget::Value(id)) =
            execution.int_function(IntFunctionId(1))
        else {
            panic!("fixture native function must have a sealed host target");
        };
        crate::host::HostCodecScope::new(
            execution.host_value_function(id).metadata_handle().clone(),
        )
    }

    fn stored_equal(&self, left: &StoredRuntimeValue, right: &StoredRuntimeValue) -> bool {
        crate::runtime::evaluated::values_equal(&self.lists, left.value(), right.value())
    }

    fn stored_source_hash(&self, value: &StoredRuntimeValue) -> u64 {
        crate::runtime::evaluated::value_source_hash(&self.lists, value.value())
    }

    fn stored_inspect(&self, value: &StoredRuntimeValue) -> ecow::EcoString {
        crate::runtime::materialize::value(
            self.execution.execution().value_metadata(),
            &self.lists,
            value.value().clone(),
        )
        .inspect()
        .to_string()
        .into()
    }

    fn stored_list_len(&self, value: &StoredRuntimeValue) -> usize {
        self.lists
            .list_len(&crate::runtime::BorrowedValue::from_stored(value).list())
    }

    fn stored_list_item(
        &self,
        value: &StoredRuntimeValue,
        index: usize,
    ) -> Option<StoredRuntimeValue> {
        self.lists
            .evaluated_value_at(
                &crate::runtime::BorrowedValue::from_stored(value).list(),
                index,
            )
            .map(|value| {
                let type_ = value.value_type(self.execution.execution().value_metadata());
                StoredRuntimeValue::new(value, type_)
            })
    }

    fn restore_list(&mut self, value: &StoredRuntimeList) -> HostListToken {
        let token = self.scoped.push_list(value.handle());
        self.scoped.list_token(token)
    }

    fn callback_inputs(&self, values: Box<[HostScopedValue]>) -> crate::runtime::CallbackInputs {
        let mut inputs = crate::runtime::CallbackInputs::new();
        for value in values {
            inputs.push_value(self.scoped.value_from_scoped(value));
        }
        inputs
    }
}

pub(crate) fn token(family: HostValueFamily) -> HostValueToken {
    HostValueToken { family, index: 0 }
}
impl HostTokenRuntime for TestHostCallRuntime<'_> {
    fn int(&self, _value: HostValueToken) -> num_bigint::BigInt {
        0.into()
    }

    fn float(&self, _value: HostValueToken) -> f64 {
        0.0
    }

    fn string(&self, _value: HostValueToken) -> ecow::EcoString {
        "".into()
    }

    fn bit_array(&self, _value: HostValueToken) -> crate::BitArrayValue {
        crate::BitArrayValue::from_bytes(Vec::new())
    }

    fn utf_codepoint(&self, _value: HostValueToken) -> char {
        '\0'
    }

    fn bool(&self, _value: HostValueToken) -> bool {
        false
    }

    fn nil(&self, _value: HostValueToken) {}

    fn list_token(&self, _value: HostValueToken) -> HostListToken {
        HostListToken::Stored(0)
    }

    fn tuple_token(&self, _value: HostValueToken) -> HostTupleToken {
        HostTupleToken(0)
    }

    fn custom_token(&self, _value: HostValueToken) -> HostCustomToken {
        HostCustomToken(0)
    }

    fn external_token(&self, _value: HostValueToken) -> HostExternalToken {
        HostExternalToken(0)
    }

    fn function_token(&self, _value: HostValueToken) -> HostFunctionToken {
        HostFunctionToken(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostCallRuntime, HostFunctionToken, HostScopedValue};
    use crate::runtime::EvaluatedValue;
    use crate::runtime::graph::RetainedValues;
    use crate::runtime::host::scoped::{StoredRuntimeList, StoredRuntimeValue};
    use crate::runtime::state::list::StoredListValueId;
    use crate::{ModuleSource, PackageSource, Value, ValueType};
    use futures_util::FutureExt;

    #[test]
    fn fixture_callable_and_codec_retain_the_compiled_native_entry() {
        let mut state = TestRunState::default();
        let runtime = TestHostCallRuntime::new(&mut state, RetainedValues::empty());
        let mut echo = Vec::new();
        assert_eq!(
            runtime
                .execution
                .run_main(runtime.state, &mut echo)
                .expect("native entry"),
            Value::Int(0.into()),
        );
        assert!(echo.is_empty());
        let codec = runtime.codec_scope();
        assert_eq!(codec.function().package(), "native");
        assert_eq!(codec.function().module(), "native");
        assert_eq!(codec.function().name(), "value");
        assert_eq!(
            runtime
                .origin()
                .into_source_site(codec.function().site())
                .expect("entry site"),
            codec.function().site().clone(),
        );
        let first = runtime.callable(HostFunctionToken(0));
        let second = runtime.callable(HostFunctionToken(0));
        first.with_value(|first| second.with_value(|second| assert!(std::ptr::eq(first, second))));
        let work = runtime.work().ready(StoredRuntimeValue::new(
            EvaluatedValue::Int(42.into()),
            ValueType::Int,
        ));
        let completion = work
            .observe()
            .now_or_never()
            .expect("ready work")
            .expect("live scope");
        completion.read(|result| {
            result.as_ref().ok().expect("ready value").read(|value| {
                assert_eq!(value.value(), &EvaluatedValue::Int(42.into()));
            })
        });
    }

    #[test]
    fn fixture_retained_lists_and_callback_inputs_preserve_owned_values() {
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, RetainedValues::empty());
        let plan = crate::runtime::plan_src("pub fn main() { [7, 9] }");
        let list = runtime.lists.int(
            plan.int_list_function_id(0).type_id(),
            vec![7.into(), 9.into()],
        );
        let value = StoredRuntimeValue::new(
            EvaluatedValue::List(StoredListValueId::Int(list.clone())),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(runtime.stored_list_len(&value), 2);
        for (index, expected) in [7, 9].into_iter().enumerate() {
            let item = runtime
                .stored_list_item(&value, index)
                .expect("retained item");
            assert_eq!(item.value(), &EvaluatedValue::Int(expected.into()));
            assert_eq!(item.type_(), &ValueType::Int);
        }
        assert!(runtime.stored_list_item(&value, 2).is_none());
        let retained = StoredRuntimeList::new(list.into());
        let token = runtime.restore_list(&retained);
        assert_eq!(
            runtime
                .scoped
                .value_from_scoped(HostScopedValue::List(token)),
            *value.value(),
        );
        let inputs = runtime.callback_inputs(
            vec![
                HostScopedValue::Int(42.into()),
                HostScopedValue::String("text".into()),
            ]
            .into_boxed_slice(),
        );
        assert_eq!(
            inputs.into_arguments().as_ref(),
            &[
                EvaluatedValue::Int(42.into()),
                EvaluatedValue::String("text".into()),
            ]
        );
        assert!(
            runtime
                .callback_inputs(Box::new([]))
                .into_arguments()
                .is_empty()
        );
    }

    #[test]
    #[should_panic(expected = "fixture native function must have a sealed host target")]
    fn fixture_codec_rejects_a_source_function_in_the_native_slot() {
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, RetainedValues::empty());
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { other() } fn other() { 0 }",
                )],
            )],
            crate::HostProviderSet::<TestHostProfile>::new([]).expect("no providers"),
        )
        .expect("source");
        runtime.execution = crate::HostedExecution::try_from_module_plan(
            crate::plan_host_program(typed).expect("plan"),
        )
        .expect("execution");
        let _ = runtime.codec_scope();
    }
}
