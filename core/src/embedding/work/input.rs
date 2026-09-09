use super::{Future, FutureType, ScopeBrand, ScopedOutput, SharedList};
use crate::embedding::input::{InputConstructions, ScopedFreshInput, ScopedInputValue};
use crate::host::HostExternalSchema;
use crate::plan::execution::LibraryListConstructions;
use crate::runtime::{EmbeddingInputStorage, EmbeddingInputValue, EvaluatedExternalValue};
use std::sync::Arc;

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    ScopedInputValue<Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    type ScopedRuntime = EvaluatedExternalValue;

    fn owners_match(_: &Future<'scope, Value::Value<'scope>, Schema>, _: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: Future<'scope, Value::Value<'scope>, Schema>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
    ) -> Self::ScopedRuntime {
        input.value
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    ScopedInputValue<&Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    type ScopedRuntime = EvaluatedExternalValue;

    fn owners_match(_: &&Future<'scope, Value::Value<'scope>, Schema>, _: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: &Future<'scope, Value::Value<'scope>, Schema>,
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
    ) -> Self::ScopedRuntime {
        input.value.clone()
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    ScopedFreshInput<Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::ScopedRuntime as EmbeddingInputValue>::ListType {
        lists.externals[index]
    }
}

impl<'scope, Value: ScopedOutput<Schema>, Schema: HostExternalSchema>
    ScopedFreshInput<&Future<'scope, Value::Value<'scope>, Schema>, ScopeBrand<'scope>>
    for FutureType<Value, Schema>
{
    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::ScopedRuntime as EmbeddingInputValue>::ListType {
        lists.externals[index]
    }
}

impl<'scope, Value: super::value::SourceType>
    ScopedInputValue<&SharedList<Value::Value<'scope>>, ScopeBrand<'scope>>
    for crate::embedding::List<Value>
{
    type ScopedRuntime = crate::runtime::EmbeddingListInput;

    fn owners_match(input: &&SharedList<Value::Value<'scope>>, owner: &Arc<()>) -> bool {
        input.owners_match(owner)
    }

    fn into_runtime(
        input: &SharedList<Value::Value<'scope>>,
        constructions: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
    ) -> Self::ScopedRuntime {
        constructions.skip::<Self>();
        input.input()
    }
}
