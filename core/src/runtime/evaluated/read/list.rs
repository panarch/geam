use super::{EvaluatedFunctionRef, EvaluatedValueRef};
use crate::StringValue;
use crate::plan::execution::type_::ListTypeId;
use crate::runtime::RuntimeListStorage;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::state::list::{
    ListSequenceIter, ParameterListValueId, StoredListValueId, StoredListValueRef,
};
use num_bigint::BigInt;
#[cfg(test)]
use std::cell::Cell;
use std::ops::Range;

pub(in crate::runtime) struct EvaluatedListRef<'value> {
    handle: StoredListValueRef<'value>,
    #[cfg(test)]
    item_reads: Cell<usize>,
}

pub(in crate::runtime) struct EvaluatedListIter<'value> {
    inner: ListItems<'value>,
    #[cfg(test)]
    item_reads: &'value Cell<usize>,
}

enum ListItems<'value> {
    Nil(Range<usize>),
    ParameterList(ParameterListValueId, Range<usize>),
    Int(ListSequenceIter<'value, BigInt>),
    String(ListSequenceIter<'value, StringValue>),
    BitArray(ListSequenceIter<'value, EvaluatedBitArray>),
    UtfCodepoint(ListSequenceIter<'value, char>),
    Custom(ListSequenceIter<'value, EvaluatedCustomValue>),
    External(ListSequenceIter<'value, EvaluatedExternalValue>),
    Float(ListSequenceIter<'value, f64>),
    Bool(ListSequenceIter<'value, bool>),
    Tuple(ListSequenceIter<'value, Vec<EvaluatedValue>>),
    List(ListSequenceIter<'value, StoredListValueId>),
    Function(ListSequenceIter<'value, EvaluatedFunctionValue>),
}

impl<'value> EvaluatedListRef<'value> {
    pub(in crate::runtime) fn new(handle: StoredListValueRef<'value>) -> Self {
        Self {
            handle,
            #[cfg(test)]
            item_reads: Cell::new(0),
        }
    }

    pub(in crate::runtime::evaluated) fn list_type(&self) -> ListTypeId {
        match self.handle {
            StoredListValueRef::Nil(value) => value.type_id().list_type(),
            StoredListValueRef::ParameterList(value) => value.type_id().list_type(),
            StoredListValueRef::Int(value) => value.type_id().list_type(),
            StoredListValueRef::String(value) => value.type_id().list_type(),
            StoredListValueRef::BitArray(value) => value.type_id().list_type(),
            StoredListValueRef::UtfCodepoint(value) => value.type_id().list_type(),
            StoredListValueRef::Custom(value) => value.type_id().list_type(),
            StoredListValueRef::External(value) => value.type_id().list_type(),
            StoredListValueRef::Float(value) => value.type_id().list_type(),
            StoredListValueRef::Bool(value) => value.type_id().list_type(),
            StoredListValueRef::Tuple(value) => value.type_id().list_type(),
            StoredListValueRef::List(value) => value.type_id().list_type(),
            StoredListValueRef::Function(value) => value.type_id().list_type(),
        }
    }

    pub(in crate::runtime) fn len(&self) -> usize {
        match self.handle {
            StoredListValueRef::Nil(value) => value.len(),
            StoredListValueRef::ParameterList(value) => value.len(),
            StoredListValueRef::Int(value) => value.values().len(),
            StoredListValueRef::String(value) => value.values().len(),
            StoredListValueRef::BitArray(value) => value.values().len(),
            StoredListValueRef::UtfCodepoint(value) => value.values().len(),
            StoredListValueRef::Custom(value) => value.values().len(),
            StoredListValueRef::External(value) => value.values().len(),
            StoredListValueRef::Float(value) => value.values().len(),
            StoredListValueRef::Bool(value) => value.values().len(),
            StoredListValueRef::Tuple(value) => value.values().len(),
            StoredListValueRef::List(value) => value.values().len(),
            StoredListValueRef::Function(value) => value.values().len(),
        }
    }

    pub(in crate::runtime) fn iter_prefix(&self, limit: usize) -> EvaluatedListIter<'_> {
        let inner = match self.handle {
            StoredListValueRef::Nil(value) => ListItems::Nil(0..limit.min(value.len())),
            StoredListValueRef::ParameterList(value) => ListItems::ParameterList(
                ParameterListValueId::new(value.type_id().item_type()),
                0..limit.min(value.len()),
            ),
            StoredListValueRef::Int(value) => ListItems::Int(value.values().iter_prefix(limit)),
            StoredListValueRef::String(value) => {
                ListItems::String(value.values().iter_prefix(limit))
            }
            StoredListValueRef::BitArray(value) => {
                ListItems::BitArray(value.values().iter_prefix(limit))
            }
            StoredListValueRef::UtfCodepoint(value) => {
                ListItems::UtfCodepoint(value.values().iter_prefix(limit))
            }
            StoredListValueRef::Custom(value) => {
                ListItems::Custom(value.values().iter_prefix(limit))
            }
            StoredListValueRef::External(value) => {
                ListItems::External(value.values().iter_prefix(limit))
            }
            StoredListValueRef::Float(value) => ListItems::Float(value.values().iter_prefix(limit)),
            StoredListValueRef::Bool(value) => ListItems::Bool(value.values().iter_prefix(limit)),
            StoredListValueRef::Tuple(value) => ListItems::Tuple(value.values().iter_prefix(limit)),
            StoredListValueRef::List(value) => ListItems::List(value.values().iter_prefix(limit)),
            StoredListValueRef::Function(value) => {
                ListItems::Function(value.values().iter_prefix(limit))
            }
        };
        EvaluatedListIter {
            inner,
            #[cfg(test)]
            item_reads: &self.item_reads,
        }
    }

    pub(in crate::runtime) fn retain(&self) -> EvaluatedValue {
        match self.handle {
            StoredListValueRef::Nil(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::ParameterList(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Int(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::String(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::BitArray(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::UtfCodepoint(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Custom(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::External(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Float(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Bool(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Tuple(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::List(value) => EvaluatedValue::List(value.clone().into()),
            StoredListValueRef::Function(value) => EvaluatedValue::List(value.clone().into()),
        }
    }

    pub(in crate::runtime) fn tail(
        &self,
        lists: &RuntimeListStorage,
        count: usize,
    ) -> EvaluatedValue {
        EvaluatedValue::List(lists.drop_first(self.handle, count))
    }

    #[cfg(test)]
    pub(in crate::runtime) fn item_reads(&self) -> usize {
        self.item_reads.get()
    }
}

impl<'value> From<&'value StoredListValueId> for EvaluatedListRef<'value> {
    fn from(value: &'value StoredListValueId) -> Self {
        Self::new(value.into())
    }
}

impl<'value> Iterator for EvaluatedListIter<'value> {
    type Item = EvaluatedValueRef<'value>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match &mut self.inner {
            ListItems::Nil(values) => values.next().map(|_| EvaluatedValueRef::Nil),
            ListItems::ParameterList(value, values) => values
                .next()
                .map(|_| EvaluatedValueRef::ParameterList(*value)),
            ListItems::Int(values) => values.next().map(EvaluatedValueRef::Int),
            ListItems::String(values) => values.next().map(EvaluatedValueRef::String),
            ListItems::BitArray(values) => values.next().map(EvaluatedValueRef::BitArray),
            ListItems::UtfCodepoint(values) => {
                values.next().copied().map(EvaluatedValueRef::UtfCodepoint)
            }
            ListItems::Custom(values) => values.next().map(EvaluatedValueRef::Custom),
            ListItems::External(values) => values.next().map(EvaluatedValueRef::External),
            ListItems::Float(values) => values.next().copied().map(EvaluatedValueRef::Float),
            ListItems::Bool(values) => values.next().copied().map(EvaluatedValueRef::Bool),
            ListItems::Tuple(values) => values.next().map(|value| EvaluatedValueRef::Tuple(value)),
            ListItems::List(values) => values
                .next()
                .map(|value| EvaluatedValueRef::List(EvaluatedListRef::from(value))),
            ListItems::Function(values) => values
                .next()
                .map(|value| EvaluatedValueRef::Function(EvaluatedFunctionRef::from(value))),
        };
        #[cfg(test)]
        if value.is_some() {
            self.item_reads.set(self.item_reads.get() + 1);
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluatedListRef;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::execution_fixture::TestHost;
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostFutureStore,
        HostList, HostListType, HostProfile, HostProviderModule, HostProviderSet,
        HostTypeParameter, HostValue, HostWorkProfile,
    };
    use crate::runtime::borrowed::BorrowedValue;
    use crate::runtime::evaluated::EvaluatedValueRef;
    use crate::runtime::plan_src;
    use crate::runtime::state::list::{RuntimeListStorage, StoredListValueRef};
    use crate::work_fixture::WorkComponent;
    use crate::{ModuleSource, PackageSource, compile_typed_host_program};
    use num_bigint::BigInt;
    use std::ptr;

    #[test]
    fn integer_prefixes_borrow_original_items_across_sequence_boundaries() {
        let plan = plan_src("pub fn main() -> List(Int) { [] }");
        let storage = RuntimeListStorage::default();
        let handle = storage.int(
            plan.int_list_function_id(0).type_id(),
            (0..600).map(BigInt::from).collect(),
        );
        let borrowed = EvaluatedListRef::new(StoredListValueRef::Int(&handle));
        for limit in [0, 1, 31, 32, 33, 255, 256, 257, 600, 601] {
            let mut cursor = borrowed.iter_prefix(limit);
            for index in 0..limit.min(600) {
                assert!(matches!(
                    cursor.next(),
                    Some(EvaluatedValueRef::Int(value))
                        if ptr::eq(value, handle.values().get(index).unwrap())
                            && value == &BigInt::from(index)
                ));
            }
            assert!(cursor.next().is_none());
        }
    }

    #[test]
    fn borrowed_prefixes_and_owned_bindings_preserve_every_source_item_family() {
        let mut providers = WorkComponent::providers::<Profile>().expect("work provider");
        providers.push(
            HostProviderModule::new("application", "library")
                .expect("observer module")
                .with_scoped_function::<
                    WorkComponent,
                    (HostListType<HostTypeParameter<0>>, HostTypeParameter<0>),
                    (),
                    _,
                >("observe", observe)
                .expect("typed observer"),
        );
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "fixture/work.gleam",
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

@external(erlang, "host", "observe")
fn observe(values: List(a), expected: a) -> Nil

fn check(value: a) {
  let assert retained = value
  let values = [retained, retained, retained]
  let assert [first, ..tail] as original = values
  observe([first], value)
  observe(tail, value)
  observe(original, value)
  observe([], value)
}

pub fn run() {
  check(1208925819614629174706176)
  check(1.5)
  check("borrowed string")
  check(<<1:3>>)
  let assert <<codepoint:utf8_codepoint>> = <<"x":utf8>>
  check(codepoint)
  check(True)
  check(Nil)
  check(#(1, "tuple field"))
  check(Ok(1))
  let operation = work.ready(1)
  check(operation)
  check([1, 2])
  check([])
  let captured = "captured string"
  check(fn() { captured })
  check(fn() { 42 })
  check(fn() { 1.5 })
  check(fn() { <<1:3>> })
  check(fn() { codepoint })
  check(fn() { True })
  check(fn() { Nil })
  check(fn() { #(1, "tuple field") })
  check(fn() { Ok(1) })
  check(fn() { operation })
  check(fn() { [1] })
  check(fn() { [] })
  check(fn() { [[]] })
  check(fn() { ["list"] })
  check(fn() { [<<1:3>>] })
  check(fn() { [codepoint] })
  check(fn() { [Ok(1)] })
  check(fn() { [operation] })
  check(fn() { [1.5] })
  check(fn() { [True] })
  check(fn() { [Nil] })
  check(fn() { [#(1)] })
  check(fn() { [[1]] })
  check(fn() { [fn() { 42 }] })
  check(fn() { fn() { 42 } })
  check(fn() { fn() { operation } })
  check(fn() { panic as "not invoked" })
}
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("providers"),
        )
        .expect("typed source");
        let (bindings, run) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(), ()>::new("run"))
            .expect("entry");
        let mut module = bindings.seal().expect("module");
        let host = TestHost::default();
        let mut state = ();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope
                    .call(&run, ())
                    .await
                    .expect("borrowed prefix assertions");
            }),
        )
        .expect("execution");
        assert!(echo.is_empty());
    }

    fn observe<'call>(
        mut call: HostCall<'call, Profile, WorkComponent, ()>,
        values: HostList<'call, HostTypeParameter<0>>,
        expected: HostValue<'call, HostTypeParameter<0>>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert_eq!(call.state(), &());
        let len = call.list_len(values);
        let owner = call.retain_value::<HostListType<HostTypeParameter<0>>>(values);
        let expected = call.retain_value::<HostTypeParameter<0>>(expected);
        let handle = BorrowedValue::from_value(owner.value()).stored_list();
        let borrowed = EvaluatedListRef::from(handle);
        assert_eq!(borrowed.len(), len);
        assert_eq!(borrowed.item_reads(), 0);
        for limit in [0, 1, 2, len, len + 1, usize::MAX] {
            let before = borrowed.item_reads();
            let mut cursor = borrowed.iter_prefix(limit);
            for _ in 0..limit.min(len) {
                let item = cursor.next().expect("prefix item");
                assert_eq!(&item.retain(), expected.value());
            }
            assert!(cursor.next().is_none());
            assert!(cursor.next().is_none());
            assert_eq!(borrowed.item_reads(), before + limit.min(len));
        }
        let storage = RuntimeListStorage::default();
        let retained = borrowed.retain();
        let tail = borrowed.tail(&storage, 1.min(len));
        drop(owner);
        let retained = EvaluatedListRef::from(BorrowedValue::from_value(&retained).stored_list());
        assert_eq!(retained.len(), len);
        for item in retained.iter_prefix(usize::MAX) {
            assert_eq!(&item.retain(), expected.value());
        }
        let tail = EvaluatedListRef::from(BorrowedValue::from_value(&tail).stored_list());
        assert_eq!(tail.len(), len.saturating_sub(1));
        for item in tail.iter_prefix(usize::MAX) {
            assert_eq!(&item.retain(), expected.value());
        }
        Ok(call.return_value(()))
    }

    struct Profile;

    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }

    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }

    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
}
