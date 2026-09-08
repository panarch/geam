use super::{Future, FutureType, ScopeBrand, ScopedOutput, SharedList};
use crate::embedding::input::{AsyncFreshInput, AsyncInputValue, InputConstructions};
use crate::host::HostExternalSchema;
use crate::plan::execution::LibraryListConstructions;
use crate::runtime::{
    EmbeddingInputStorage, EmbeddingInputValue, EvaluatedExternalValue, TransferValues,
};
use std::sync::Arc;

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    AsyncInputValue<Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    type TransferRuntime = EvaluatedExternalValue<TransferValues>;

    fn owners_match(_: &Future<'scope, Value::Value<'scope>, Schema>, _: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: Future<'scope, Value::Value<'scope>, Schema>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage<TransferValues>,
    ) -> Self::TransferRuntime {
        input.value
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    AsyncInputValue<&Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    type TransferRuntime = EvaluatedExternalValue<TransferValues>;

    fn owners_match(_: &&Future<'scope, Value::Value<'scope>, Schema>, _: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: &Future<'scope, Value::Value<'scope>, Schema>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage<TransferValues>,
    ) -> Self::TransferRuntime {
        input.value.clone()
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    AsyncFreshInput<Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::TransferRuntime as EmbeddingInputValue<TransferValues>>::ListType {
        lists.externals[index]
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    AsyncFreshInput<&Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::TransferRuntime as EmbeddingInputValue<TransferValues>>::ListType {
        lists.externals[index]
    }
}

impl<'scope, Value: super::value::SourceType>
    AsyncInputValue<&SharedList<Value::Value<'scope>>, ScopeBrand<'scope>>
    for crate::embedding::List<Value>
{
    type TransferRuntime = crate::runtime::EmbeddingListInput<TransferValues>;

    fn owners_match(input: &&SharedList<Value::Value<'scope>>, owner: &Arc<()>) -> bool {
        input.owners_match(owner)
    }

    fn into_runtime(
        input: &SharedList<Value::Value<'scope>>,
        constructions: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage<TransferValues>,
    ) -> Self::TransferRuntime {
        constructions.skip::<Self>();
        input.input()
    }
}
