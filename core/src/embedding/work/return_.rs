use super::value::SharedValue;
use super::{Future, FutureType, ScopedOutput, SharedList};
use crate::embedding::CallError;
use crate::embedding::value::EmbeddingValue;
use crate::host::{HostProfile, HostWorkProfile, HostWorkSchema};
use crate::plan::execution::{LibraryFunctionEntries, LibraryInputConstructions};
use crate::runtime::execution::EntryContext;
use crate::runtime::{EmbeddingEntry, EmbeddingOutput, RetainedInputs};

pub(in crate::embedding) trait ScopedReturn<Profile: HostProfile>:
    ScopedTake<Profile>
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions;

    fn call<'scope>(
        execution: &EntryContext<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedInputs,
        context: <Self::Value<'scope> as SharedValue>::Context,
    ) -> impl std::future::Future<Output = Result<Self::Value<'scope>, CallError>> + Send;
}

pub(in crate::embedding) trait ScopedTake<Profile: HostProfile>:
    ScopedOutput<Profile> + EmbeddingValue
{
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope>;
}

macro_rules! scalar {
    ($type:ty, $entries:ident, $take:ident) => {
        impl<Profile: HostProfile> ScopedTake<Profile> for $type {
            fn take<'scope>(
                output: &mut EmbeddingOutput,
                _: &<Self::Value<'scope> as SharedValue>::Context,
            ) -> Self::Value<'scope> {
                output.$take()
            }
        }
        impl<Profile: HostProfile> ScopedReturn<Profile> for $type {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.$entries[slot].inputs()
            }
            async fn call<'scope>(
                execution: &EntryContext<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedInputs,
                _: (),
            ) -> Result<Self, CallError> {
                entries.$entries[slot]
                    .function()
                    .call(execution, inputs)
                    .await
            }
        }
    };
}

scalar!(crate::embedding::BigInt, ints, take_int);
scalar!(f64, floats, take_float);
scalar!(crate::embedding::EcoString, strings, take_string);
scalar!(crate::BitArrayValue, bit_arrays, take_bit_array);
scalar!(char, utf_codepoints, take_utf_codepoint);
scalar!(bool, bools, take_bool);
scalar!((), nils, take_nil);

macro_rules! compound_return {
    ($container:ty, $entries:ident, $($type:ident),+) => {
        impl<Profile: HostProfile, $($type: ScopedTake<Profile>),+> ScopedReturn<Profile> for $container {
            fn input_constructions(entries: &LibraryFunctionEntries, slot: usize) -> &LibraryInputConstructions { entries.$entries[slot].inputs() }
            async fn call<'scope>(
                execution: &EntryContext<Profile>, entries: &LibraryFunctionEntries,
                slot: usize, inputs: RetainedInputs,
                context: <Self::Value<'scope> as SharedValue>::Context,
            ) -> Result<Self::Value<'scope>, CallError> {
                let mut output = entries.$entries[slot].function().call(execution, inputs).await?;
                Ok(<Self as ScopedTake<Profile>>::take(&mut output, &context))
            }
        }
    };
}

macro_rules! tuple {
    ($($index:tt: $type:ident),+) => {
        impl<Profile: HostProfile, $($type: ScopedTake<Profile>),+> ScopedTake<Profile> for ($($type,)+) {
            fn take<'scope>(output: &mut EmbeddingOutput, context: &<Self::Value<'scope> as SharedValue>::Context) -> Self::Value<'scope> {
                ($($type::take(output, &context.$index),)+)
            }
        }
        compound_return!(($($type,)+), tuples, $($type),+);
    };
}

tuple!(0:A);
tuple!(0:A, 1:B);
tuple!(0:A, 1:B, 2:C);
tuple!(0:A, 1:B, 2:C, 3:D);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E, 5:F);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E, 5:F, 6:G);

impl<Profile: HostProfile, Success: ScopedTake<Profile>, Failure: ScopedTake<Profile>>
    ScopedTake<Profile> for Result<Success, Failure>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope> {
        if output.take_variant() == 0 {
            Ok(Success::take(output, &context.0))
        } else {
            Err(Failure::take(output, &context.1))
        }
    }
}
compound_return!(Result<Success, Failure>, customs, Success, Failure);

impl<Profile: HostProfile, Value: ScopedTake<Profile>> ScopedTake<Profile> for Option<Value> {
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope> {
        if output.take_variant() == 0 {
            Some(Value::take(output, context))
        } else {
            None
        }
    }
}
compound_return!(Option<Value>, customs, Value);

impl<Profile: HostProfile, Value: ScopedTake<Profile>> ScopedTake<Profile>
    for crate::embedding::List<Value>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope> {
        SharedList::new(output.take_list(), context.clone())
    }
}
compound_return!(crate::embedding::List<Value>, lists, Value);

impl<Profile: HostWorkProfile, Value: ScopedTake<Profile>> ScopedTake<Profile>
    for FutureType<Value, HostWorkSchema<Profile>>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput,
        context: &<Self::Value<'scope> as SharedValue>::Context,
    ) -> Self::Value<'scope> {
        Future::new(output.take_external(), context.clone())
    }
}

impl<Profile: HostWorkProfile, Value: ScopedTake<Profile>> ScopedReturn<Profile>
    for FutureType<Value, HostWorkSchema<Profile>>
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions {
        entries.externals[slot].inputs()
    }
    async fn call<'scope>(
        execution: &EntryContext<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedInputs,
        context: <Self::Value<'scope> as SharedValue>::Context,
    ) -> Result<Self::Value<'scope>, CallError> {
        let value = entries.externals[slot]
            .function()
            .call(execution, inputs)
            .await?;
        Ok(Future::new(value, context))
    }
}
