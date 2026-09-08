use super::{Future, FutureType, ScopeBrand, ScopedOutput, SharedList};
use crate::embedding::value::EmbeddingValue;
use crate::host::HostExternalSchema;
use crate::host::{HostFutureStore, HostProfile};
use crate::plan::execution::{LibraryFunctionEntries, LibraryInputConstructions};
use crate::runtime::work::driver::Driver;
use crate::runtime::{EmbeddingOutput, TransferInputs, TransferValues};
use std::sync::Arc;

pub(in crate::embedding) trait ScopedReturn<Schema: HostExternalSchema>:
    ScopedTake<Schema>
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions;

    fn call<'scope, Profile: HostProfile>(
        driver: &mut Driver<'_, Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: TransferInputs,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Result<Self::Value<'scope>, crate::AsyncExecutionError>;
}

pub(in crate::embedding) trait ScopedTake<Schema: HostExternalSchema>:
    ScopedOutput<Schema> + EmbeddingValue
{
    fn take<'scope>(
        output: &mut EmbeddingOutput<TransferValues>,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Self::Value<'scope>;
}

macro_rules! scalar {
    ($type:ty, $entries:ident, $run:ident, $take:ident) => {
        impl<Schema: HostExternalSchema> ScopedTake<Schema> for $type {
            fn take<'scope>(
                output: &mut EmbeddingOutput<TransferValues>,
                _: ScopeBrand<'scope>,
                _: &HostFutureStore,
                _: &Arc<()>,
            ) -> Self {
                output.$take()
            }
        }
        impl<Schema: HostExternalSchema> ScopedReturn<Schema> for $type {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.$entries[slot].inputs()
            }
            fn call<'scope, Profile: HostProfile>(
                driver: &mut Driver<'_, Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: TransferInputs,
                _: ScopeBrand<'scope>,
                _: &HostFutureStore,
                _: &Arc<()>,
            ) -> Result<Self, crate::AsyncExecutionError> {
                driver.$run(*entries.$entries[slot].function(), inputs)
            }
        }
    };
}

scalar!(crate::embedding::BigInt, ints, run_int, take_int);
scalar!(f64, floats, run_float, take_float);
scalar!(
    crate::embedding::EcoString,
    strings,
    run_string,
    take_string
);
scalar!(
    crate::BitArrayValue,
    bit_arrays,
    run_bit_array,
    take_bit_array
);
scalar!(char, utf_codepoints, run_utf_codepoint, take_utf_codepoint);
scalar!(bool, bools, run_bool, take_bool);
scalar!((), nils, run_nil, take_nil);

macro_rules! compound_return {
    ($container:ty, $entries:ident, $run:ident, $($type:ident),+) => {
        impl<Schema: HostExternalSchema, $($type: ScopedTake<Schema>),+> ScopedReturn<Schema> for $container {
            fn input_constructions(entries: &LibraryFunctionEntries, slot: usize) -> &LibraryInputConstructions { entries.$entries[slot].inputs() }
            fn call<'scope, Profile: HostProfile>(driver: &mut Driver<'_, Profile>, entries: &LibraryFunctionEntries, slot: usize, inputs: TransferInputs, brand: ScopeBrand<'scope>, store: &HostFutureStore, owner: &Arc<()>) -> Result<Self::Value<'scope>, crate::AsyncExecutionError> {
                driver.$run(*entries.$entries[slot].function(), inputs).map(|mut output| <Self as ScopedTake<Schema>>::take(&mut output, brand, store, owner))
            }
        }
    };
}

macro_rules! tuple {
    ($($type:ident),+) => {
        impl<Schema: HostExternalSchema, $($type: ScopedTake<Schema>),+> ScopedTake<Schema> for ($($type,)+) {
            fn take<'scope>(output: &mut EmbeddingOutput<TransferValues>, brand: ScopeBrand<'scope>, store: &HostFutureStore, owner: &Arc<()>) -> Self::Value<'scope> {
                ($($type::take(output, brand, store, owner),)+)
            }
        }
        compound_return!(($($type,)+), tuples, run_tuple, $($type),+);
    };
}

tuple!(A);
tuple!(A, B);
tuple!(A, B, C);
tuple!(A, B, C, D);
tuple!(A, B, C, D, E);
tuple!(A, B, C, D, E, F);
tuple!(A, B, C, D, E, F, G);

impl<Schema: HostExternalSchema, Success: ScopedTake<Schema>, Failure: ScopedTake<Schema>>
    ScopedTake<Schema> for Result<Success, Failure>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput<TransferValues>,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Self::Value<'scope> {
        if output.take_variant() == 0 {
            Ok(Success::take(output, brand, store, owner))
        } else {
            Err(Failure::take(output, brand, store, owner))
        }
    }
}

compound_return!(Result<Success, Failure>, customs, run_custom, Success, Failure);

impl<Schema: HostExternalSchema, Value: ScopedTake<Schema>> ScopedTake<Schema> for Option<Value> {
    fn take<'scope>(
        output: &mut EmbeddingOutput<TransferValues>,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Self::Value<'scope> {
        if output.take_variant() == 0 {
            Some(Value::take(output, brand, store, owner))
        } else {
            None
        }
    }
}

compound_return!(Option<Value>, customs, run_custom, Value);

impl<Schema: HostExternalSchema, Value: ScopedTake<Schema>> ScopedTake<Schema>
    for crate::embedding::List<Value>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput<TransferValues>,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Self::Value<'scope> {
        SharedList::new(output.take_list(), Self::context(brand, store, owner))
    }
}

compound_return!(crate::embedding::List<Value>, lists, run_list, Value);

impl<Value: ScopedTake<Schema>, Schema: HostExternalSchema> ScopedTake<Schema>
    for FutureType<Value, Schema>
{
    fn take<'scope>(
        output: &mut EmbeddingOutput<TransferValues>,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Self::Value<'scope> {
        Future::new(output.take_external(), Self::context(brand, store, owner))
    }
}

impl<Value: ScopedTake<Schema>, Schema: HostExternalSchema> ScopedReturn<Schema>
    for FutureType<Value, Schema>
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions {
        entries.externals[slot].inputs()
    }
    fn call<'scope, Profile: HostProfile>(
        driver: &mut Driver<'_, Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: TransferInputs,
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> Result<Self::Value<'scope>, crate::AsyncExecutionError> {
        driver
            .run_external(*entries.externals[slot].function(), inputs)
            .map(|value| Future::new(value, Self::context(brand, store, owner)))
    }
}
