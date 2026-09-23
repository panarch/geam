use super::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use super::state::list::{
    ListSequence, ListSequenceIter, ListValueId, ParameterListValueId, RuntimeListStorage,
    StoredListValueId,
};
use crate::StringValue;
use num_bigint::BigInt;

pub(in crate::runtime) struct RetainedList<Handle> {
    value: Handle,
    read: ListRead,
    #[cfg(test)]
    item_reads: std::cell::Cell<usize>,
}

impl<Handle> RetainedList<Handle>
where
    Handle: Clone + Into<ListValueId>,
{
    pub(in crate::runtime) fn new(value: Handle) -> Self {
        Self {
            read: ListRead::new(&value.clone().into()),
            value,
            #[cfg(test)]
            item_reads: std::cell::Cell::new(0),
        }
    }

    pub(in crate::runtime) fn len(&self) -> usize {
        self.read.len()
    }

    pub(in crate::runtime) fn item(&self, index: usize) -> Option<EvaluatedValue> {
        #[cfg(test)]
        self.item_reads.set(self.item_reads.get() + 1);
        self.read.item(index)
    }

    pub(in crate::runtime) fn iter(&self) -> RetainedListIter<'_> {
        RetainedListIter {
            inner: self.read.iter(),
            #[cfg(test)]
            reads: &self.item_reads,
        }
    }

    pub(in crate::runtime) fn handle(&self) -> &Handle {
        &self.value
    }

    #[cfg(test)]
    pub(in crate::runtime) fn item_reads(&self) -> usize {
        self.item_reads.get()
    }
}

pub(in crate::runtime) struct RetainedListIter<'a> {
    inner: ListReadIter<'a>,
    #[cfg(test)]
    reads: &'a std::cell::Cell<usize>,
}

impl Iterator for RetainedListIter<'_> {
    type Item = EvaluatedValue;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next();
        #[cfg(test)]
        if value.is_some() {
            self.reads.set(self.reads.get() + 1);
        }
        value
    }
}

enum ListRead {
    Empty,
    Nil(usize),
    ParameterList(ParameterListValueId, usize),
    Int(ListSequence<BigInt>),
    String(ListSequence<StringValue>),
    BitArray(ListSequence<EvaluatedBitArray>),
    UtfCodepoint(ListSequence<char>),
    Custom(ListSequence<EvaluatedCustomValue>),
    External(ListSequence<EvaluatedExternalValue>),
    Float(ListSequence<f64>),
    Bool(ListSequence<bool>),
    Tuple(ListSequence<Vec<EvaluatedValue>>),
    List(ListSequence<StoredListValueId>),
    Function(ListSequence<EvaluatedFunctionValue>),
}

enum ListReadIter<'a> {
    Empty,
    Nil(std::ops::Range<usize>),
    ParameterList(ParameterListValueId, std::ops::Range<usize>),
    Int(ListSequenceIter<'a, BigInt>),
    String(ListSequenceIter<'a, StringValue>),
    BitArray(ListSequenceIter<'a, EvaluatedBitArray>),
    UtfCodepoint(ListSequenceIter<'a, char>),
    Custom(ListSequenceIter<'a, EvaluatedCustomValue>),
    External(ListSequenceIter<'a, EvaluatedExternalValue>),
    Float(ListSequenceIter<'a, f64>),
    Bool(ListSequenceIter<'a, bool>),
    Tuple(ListSequenceIter<'a, Vec<EvaluatedValue>>),
    List(ListSequenceIter<'a, StoredListValueId>),
    Function(ListSequenceIter<'a, EvaluatedFunctionValue>),
}

impl ListRead {
    fn new(value: &ListValueId) -> Self {
        match value {
            ListValueId::Parameter(_) => Self::Empty,
            ListValueId::Nil(value) => {
                Self::Nil(RuntimeListStorage::from_handle(value.core()).nil_len(value))
            }
            ListValueId::ParameterList(value) => Self::ParameterList(
                ParameterListValueId::new(value.type_id().item_type()),
                RuntimeListStorage::from_handle(value.core()).parameter_list_list_len(value),
            ),
            ListValueId::Int(value) => {
                Self::Int(RuntimeListStorage::from_handle(value.core()).int_values(value))
            }
            ListValueId::String(value) => {
                Self::String(RuntimeListStorage::from_handle(value.core()).string_values(value))
            }
            ListValueId::BitArray(value) => Self::BitArray(
                RuntimeListStorage::from_handle(value.core()).bit_array_values(value),
            ),
            ListValueId::UtfCodepoint(value) => Self::UtfCodepoint(
                RuntimeListStorage::from_handle(value.core()).utf_codepoint_values(value),
            ),
            ListValueId::Custom(value) => {
                Self::Custom(RuntimeListStorage::from_handle(value.core()).custom_values(value))
            }
            ListValueId::External(value) => {
                Self::External(RuntimeListStorage::from_handle(value.core()).external_values(value))
            }
            ListValueId::Float(value) => {
                Self::Float(RuntimeListStorage::from_handle(value.core()).float_values(value))
            }
            ListValueId::Bool(value) => {
                Self::Bool(RuntimeListStorage::from_handle(value.core()).bool_values(value))
            }
            ListValueId::Tuple(value) => {
                Self::Tuple(RuntimeListStorage::from_handle(value.core()).tuple_values(value))
            }
            ListValueId::List(value) => {
                Self::List(RuntimeListStorage::from_handle(value.core()).list_values(value))
            }
            ListValueId::Function(value) => {
                Self::Function(RuntimeListStorage::from_handle(value.core()).function_values(value))
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Nil(len) | Self::ParameterList(_, len) => *len,
            Self::Int(values) => values.len(),
            Self::String(values) => values.len(),
            Self::BitArray(values) => values.len(),
            Self::UtfCodepoint(values) => values.len(),
            Self::Custom(values) => values.len(),
            Self::External(values) => values.len(),
            Self::Float(values) => values.len(),
            Self::Bool(values) => values.len(),
            Self::Tuple(values) => values.len(),
            Self::List(values) => values.len(),
            Self::Function(values) => values.len(),
        }
    }

    fn item(&self, index: usize) -> Option<EvaluatedValue> {
        match self {
            Self::Empty => None,
            Self::Nil(len) => (index < *len).then_some(EvaluatedValue::Nil),
            Self::ParameterList(value, len) => {
                (index < *len).then_some(EvaluatedValue::ParameterList(*value))
            }
            Self::Int(values) => values.get(index).cloned().map(EvaluatedValue::Int),
            Self::String(values) => values.get(index).cloned().map(EvaluatedValue::String),
            Self::BitArray(values) => values.get(index).cloned().map(EvaluatedValue::BitArray),
            Self::UtfCodepoint(values) => {
                values.get(index).copied().map(EvaluatedValue::UtfCodepoint)
            }
            Self::Custom(values) => values.get(index).cloned().map(EvaluatedValue::Custom),
            Self::External(values) => values.get(index).cloned().map(EvaluatedValue::External),
            Self::Float(values) => values.get(index).copied().map(EvaluatedValue::Float),
            Self::Bool(values) => values.get(index).copied().map(EvaluatedValue::Bool),
            Self::Tuple(values) => values.get(index).cloned().map(EvaluatedValue::Tuple),
            Self::List(values) => values.get(index).cloned().map(EvaluatedValue::from),
            Self::Function(values) => values.get(index).cloned().map(EvaluatedValue::Function),
        }
    }

    fn iter(&self) -> ListReadIter<'_> {
        match self {
            Self::Empty => ListReadIter::Empty,
            Self::Nil(len) => ListReadIter::Nil(0..*len),
            Self::ParameterList(value, len) => ListReadIter::ParameterList(*value, 0..*len),
            Self::Int(values) => ListReadIter::Int(values.iter()),
            Self::String(values) => ListReadIter::String(values.iter()),
            Self::BitArray(values) => ListReadIter::BitArray(values.iter()),
            Self::UtfCodepoint(values) => ListReadIter::UtfCodepoint(values.iter()),
            Self::Custom(values) => ListReadIter::Custom(values.iter()),
            Self::External(values) => ListReadIter::External(values.iter()),
            Self::Float(values) => ListReadIter::Float(values.iter()),
            Self::Bool(values) => ListReadIter::Bool(values.iter()),
            Self::Tuple(values) => ListReadIter::Tuple(values.iter()),
            Self::List(values) => ListReadIter::List(values.iter()),
            Self::Function(values) => ListReadIter::Function(values.iter()),
        }
    }
}

impl Iterator for ListReadIter<'_> {
    type Item = EvaluatedValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Nil(indices) => indices.next().map(|_| EvaluatedValue::Nil),
            Self::ParameterList(value, indices) => indices
                .next()
                .map(|_| EvaluatedValue::ParameterList(*value)),
            Self::Int(values) => values.next().cloned().map(EvaluatedValue::Int),
            Self::String(values) => values.next().cloned().map(EvaluatedValue::String),
            Self::BitArray(values) => values.next().cloned().map(EvaluatedValue::BitArray),
            Self::UtfCodepoint(values) => values.next().copied().map(EvaluatedValue::UtfCodepoint),
            Self::Custom(values) => values.next().cloned().map(EvaluatedValue::Custom),
            Self::External(values) => values.next().cloned().map(EvaluatedValue::External),
            Self::Float(values) => values.next().copied().map(EvaluatedValue::Float),
            Self::Bool(values) => values.next().copied().map(EvaluatedValue::Bool),
            Self::Tuple(values) => values.next().cloned().map(EvaluatedValue::Tuple),
            Self::List(values) => values.next().cloned().map(EvaluatedValue::from),
            Self::Function(values) => values.next().cloned().map(EvaluatedValue::Function),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RetainedList;
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostFutureStore,
        HostList, HostListType, HostProfile, HostProviderModule, HostProviderSet,
        HostTypeParameter, HostValue, HostWorkProfile,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{ModuleSource, PackageSource, compile_typed_host_program};

    #[test]
    fn retained_reads_and_cursors_preserve_every_source_item_family() {
        let mut providers = WorkComponent::providers::<Profile>().expect("work provider");
        providers.push(
            HostProviderModule::new("application", "library")
                .expect("observer module")
                .with_scoped_function::<
                    WorkComponent,
                    (HostListType<HostTypeParameter<0>>, HostTypeParameter<0>),
                    (),
                    _,
                >("check", check)
                .expect("typed item observer")
                .with_scoped_function::<
                    WorkComponent,
                    (HostListType<HostTypeParameter<0>>,),
                    (),
                    _,
                >("check_empty", check_empty)
                .expect("empty generic observer"),
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

@external(erlang, "host", "check")
fn check(values: List(a), expected: a) -> Nil
@external(erlang, "host", "check_empty")
fn check_empty(values: List(a)) -> Nil

pub fn run() {
  check([1], 1)
  check([], 1)
  check([1.5], 1.5)
  check([], 1.5)
  check(["hello"], "hello")
  check([], "hello")
  check([<<1:3>>], <<1:3>>)
  check([], <<1:3>>)
  let assert <<codepoint:utf8_codepoint>> = <<"x":utf8>>
  check([codepoint], codepoint)
  check([], codepoint)
  check([True], True)
  check([], True)
  check([Nil], Nil)
  check([], Nil)
  check([#(1, "hello")], #(1, "hello"))
  check([], #(1, "hello"))
  check([Ok(1)], Ok(1))
  check([], Ok(1))
  let operation = work.ready(1)
  check([operation], operation)
  check([], operation)
  let child = [1]
  check([child], child)
  check([], child)
  let callback = fn(value: Int) { value + 1 }
  check([callback], callback)
  check([], callback)
  check([[]], [])
  check([], [])
  check_empty([])
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
        let host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope
                    .call(&run, ())
                    .await
                    .expect("retained item assertions");
            }),
        )
        .expect("execution");
        assert!(echo.is_empty());
    }

    fn check<'call>(
        mut call: HostCall<'call, Profile, WorkComponent, ()>,
        values: HostList<'call, HostTypeParameter<0>>,
        expected: HostValue<'call, HostTypeParameter<0>>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        assert_eq!(call.state(), &());
        let handle = call.retain_list_value(values).handle();
        let retained = RetainedList::new(handle.clone());
        let expected = call.retain_value::<HostTypeParameter<0>>(expected);
        assert_eq!(retained.handle(), &handle);
        assert_eq!(retained.item_reads(), 0);
        let len = call.list_len(values);
        assert_eq!(retained.len(), len);
        let expected = if len == 0 {
            None
        } else {
            Some(expected.value().clone())
        };
        assert_eq!(retained.item(0), expected);
        assert_eq!(retained.item(1), None);
        let mut cursor = retained.iter();
        assert_eq!(retained.item_reads(), 2);
        assert_eq!(cursor.next(), expected);
        assert_eq!(cursor.next(), None);
        assert_eq!(cursor.next(), None);
        assert_eq!(retained.item_reads(), 2 + len);
        Ok(call.return_value(()))
    }

    fn check_empty<'call>(
        call: HostCall<'call, Profile, WorkComponent, ()>,
        values: HostList<'call, HostTypeParameter<0>>,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let retained = RetainedList::new(call.retain_list_value(values).handle());
        assert_eq!(retained.len(), 0);
        assert_eq!(retained.item(0), None);
        assert_eq!(retained.iter().next(), None);
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
