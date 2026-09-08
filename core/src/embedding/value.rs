use super::Module;
use super::input::{ListFamily, add_list_counts};
use crate::plan::execution::{LibraryFunctionEntries, LibraryInputConstructions};
use crate::plan::{LibraryValueType, StandardVariant, ValueType};
use crate::runtime::{
    EmbeddingCustomInput, EmbeddingInputValue, EmbeddingOutput, EmbeddingTupleInput, RetainedValues,
};
use crate::runtime::{LocalValues, RuntimeValueProfile};
use crate::{EchoSink, ExecutionError, HostProfile, HostedExecution};
use std::sync::Arc;

pub(super) trait EmbeddingValue: Sized {
    const VARIANT_COUNT: usize;
    const LIST_COUNTS: [usize; 11];
    const LIST_FAMILY: ListFamily;

    fn library_type() -> LibraryValueType;

    fn value_type() -> ValueType {
        Self::library_type().value_type()
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>);

    fn collect_input_variants(variants: &mut Vec<StandardVariant>) {
        Self::collect_variants(variants);
    }

    fn collect_lists(lists: &mut Vec<LibraryValueType>);

    fn standard_variants() -> Vec<StandardVariant> {
        let mut variants = Vec::with_capacity(Self::VARIANT_COUNT);
        Self::collect_variants(&mut variants);
        variants
    }
}

pub(super) trait EmbeddingInputRuntime: EmbeddingValue {
    type Runtime: EmbeddingInputValue;
}

pub(super) trait OutputValue<Profile: RuntimeValueProfile>: EmbeddingValue {
    fn plain_library_type() -> LibraryValueType<std::convert::Infallible>;

    fn take(output: &mut EmbeddingOutput<Profile>, owner: &Arc<()>) -> Self;
}

pub(super) trait Arguments {
    fn value_types() -> Vec<ValueType>;

    fn standard_variants() -> Vec<StandardVariant>;

    fn input_variants() -> Vec<StandardVariant>;

    fn input_lists() -> Vec<LibraryValueType>;
}

pub(super) trait ReturnValue: OutputValue<LocalValues> {
    fn input_constructions<Graph: crate::plan::execution::function::ExecutionGraphProfile>(
        entries: &LibraryFunctionEntries<Graph>,
        slot: usize,
    ) -> &LibraryInputConstructions;

    fn call(
        module: &Module,
        slot: usize,
        inputs: RetainedValues,
        echo: &mut dyn EchoSink,
    ) -> Result<Self, ExecutionError>;

    fn call_hosted<Profile: HostProfile>(
        execution: &HostedExecution<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedValues,
        state: &mut Profile::RunState,
        echo: &mut dyn EchoSink,
        owner: &Arc<()>,
    ) -> Result<Self, ExecutionError>;
}

macro_rules! scalar_value {
    ($type:ty, $value_type:ident, $take:ident) => {
        impl EmbeddingValue for $type {
            const VARIANT_COUNT: usize = 0;
            const LIST_COUNTS: [usize; 11] = [0; 11];
            const LIST_FAMILY: ListFamily = ListFamily::$value_type;

            fn library_type() -> LibraryValueType {
                LibraryValueType::$value_type
            }

            fn collect_variants(_variants: &mut Vec<StandardVariant>) {}

            fn collect_lists(_lists: &mut Vec<LibraryValueType>) {}
        }

        impl EmbeddingInputRuntime for $type {
            type Runtime = Self;
        }

        impl<Profile: RuntimeValueProfile> OutputValue<Profile> for $type {
            fn plain_library_type() -> LibraryValueType<std::convert::Infallible> {
                LibraryValueType::$value_type
            }

            fn take(output: &mut EmbeddingOutput<Profile>, _owner: &Arc<()>) -> Self {
                output.$take()
            }
        }
    };
}

scalar_value!(super::BigInt, Int, take_int);
scalar_value!(f64, Float, take_float);
scalar_value!(super::EcoString, String, take_string);
scalar_value!(super::BitArrayValue, BitArray, take_bit_array);
scalar_value!(char, UtfCodepoint, take_utf_codepoint);
scalar_value!(bool, Bool, take_bool);
scalar_value!((), Nil, take_nil);

macro_rules! tuple_value {
    ($($type:ident),+) => {
        impl<$($type),+> EmbeddingValue for ($($type,)+)
        where
            $($type: EmbeddingValue,)+
        {

            const VARIANT_COUNT: usize = 0 $(+ $type::VARIANT_COUNT)+;
            const LIST_COUNTS: [usize; 11] = {
                let counts = [0; 11];
                $(let counts = add_list_counts(counts, $type::LIST_COUNTS);)+
                counts
            };
            const LIST_FAMILY: ListFamily = ListFamily::Tuple;

            fn library_type() -> LibraryValueType {
                LibraryValueType::Tuple(vec![$($type::value_type()),+])
            }

            fn collect_variants(variants: &mut Vec<StandardVariant>) {
                $($type::collect_variants(variants);)+
            }

            fn collect_input_variants(variants: &mut Vec<StandardVariant>) {
                $($type::collect_input_variants(variants);)+
            }

            fn collect_lists(lists: &mut Vec<LibraryValueType>) {
                $($type::collect_lists(lists);)+
            }

        }

        impl<$($type: EmbeddingValue),+> EmbeddingInputRuntime for ($($type,)+) {
            type Runtime = EmbeddingTupleInput;
        }

        impl<Profile, $($type),+> OutputValue<Profile> for ($($type,)+)
        where
            Profile: RuntimeValueProfile,
            $($type: OutputValue<Profile>,)+
        {
            fn plain_library_type() -> LibraryValueType<std::convert::Infallible> {
                LibraryValueType::Tuple(vec![$($type::value_type()),+])
            }

            fn take(output: &mut EmbeddingOutput<Profile>, owner: &Arc<()>) -> Self {
                ($($type::take(output, owner),)+)
            }
        }
    };
}

tuple_value!(A);
tuple_value!(A, B);
tuple_value!(A, B, C);
tuple_value!(A, B, C, D);
tuple_value!(A, B, C, D, E);
tuple_value!(A, B, C, D, E, F);
tuple_value!(A, B, C, D, E, F, G);

impl<Success: EmbeddingValue, Failure: EmbeddingValue> EmbeddingValue for Result<Success, Failure> {
    const VARIANT_COUNT: usize = 1 + Success::VARIANT_COUNT + Failure::VARIANT_COUNT;
    const LIST_COUNTS: [usize; 11] = add_list_counts(Success::LIST_COUNTS, Failure::LIST_COUNTS);
    const LIST_FAMILY: ListFamily = ListFamily::Custom;

    fn library_type() -> LibraryValueType {
        LibraryValueType::Custom(
            StandardVariant::Result.custom_type(vec![Success::value_type(), Failure::value_type()]),
        )
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Result);
        Success::collect_variants(variants);
        Failure::collect_variants(variants);
    }

    fn collect_input_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Result);
        Success::collect_input_variants(variants);
        Failure::collect_input_variants(variants);
    }

    fn collect_lists(lists: &mut Vec<LibraryValueType>) {
        Success::collect_lists(lists);
        Failure::collect_lists(lists);
    }
}

impl<Success: EmbeddingValue, Failure: EmbeddingValue> EmbeddingInputRuntime
    for Result<Success, Failure>
{
    type Runtime = EmbeddingCustomInput;
}

impl<Profile, Success, Failure> OutputValue<Profile> for Result<Success, Failure>
where
    Profile: RuntimeValueProfile,
    Success: OutputValue<Profile>,
    Failure: OutputValue<Profile>,
{
    fn plain_library_type() -> LibraryValueType<std::convert::Infallible> {
        LibraryValueType::Custom(
            StandardVariant::Result.custom_type(vec![Success::value_type(), Failure::value_type()]),
        )
    }

    fn take(output: &mut EmbeddingOutput<Profile>, owner: &Arc<()>) -> Self {
        if output.take_variant() == 0 {
            Ok(Success::take(output, owner))
        } else {
            Err(Failure::take(output, owner))
        }
    }
}

impl<Value: EmbeddingValue> EmbeddingValue for Option<Value> {
    const VARIANT_COUNT: usize = 1 + Value::VARIANT_COUNT;
    const LIST_COUNTS: [usize; 11] = Value::LIST_COUNTS;
    const LIST_FAMILY: ListFamily = ListFamily::Custom;

    fn library_type() -> LibraryValueType {
        LibraryValueType::Custom(StandardVariant::Option.custom_type(vec![Value::value_type()]))
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Option);
        Value::collect_variants(variants);
    }

    fn collect_input_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Option);
        Value::collect_input_variants(variants);
    }

    fn collect_lists(lists: &mut Vec<LibraryValueType>) {
        Value::collect_lists(lists);
    }
}

impl<Value: EmbeddingValue> EmbeddingInputRuntime for Option<Value> {
    type Runtime = EmbeddingCustomInput;
}

impl<Profile, Value> OutputValue<Profile> for Option<Value>
where
    Profile: RuntimeValueProfile,
    Value: OutputValue<Profile>,
{
    fn plain_library_type() -> LibraryValueType<std::convert::Infallible> {
        LibraryValueType::Custom(StandardVariant::Option.custom_type(vec![Value::value_type()]))
    }

    fn take(output: &mut EmbeddingOutput<Profile>, owner: &Arc<()>) -> Self {
        if output.take_variant() == 0 {
            Some(Value::take(output, owner))
        } else {
            None
        }
    }
}

impl Arguments for () {
    fn value_types() -> Vec<ValueType> {
        Vec::new()
    }

    fn input_variants() -> Vec<StandardVariant> {
        Vec::new()
    }

    fn standard_variants() -> Vec<StandardVariant> {
        Vec::new()
    }

    fn input_lists() -> Vec<LibraryValueType> {
        Vec::new()
    }
}

macro_rules! arguments {
    ($($type:ident),+) => {
        impl<$($type),+> Arguments for ($($type,)+)
        where
            $($type: EmbeddingValue,)+
        {
            fn value_types() -> Vec<ValueType> {
                vec![$($type::value_type()),+]
            }

            fn input_variants() -> Vec<StandardVariant> {
                let mut variants = Vec::with_capacity(0 $(+ $type::VARIANT_COUNT)+);
                $($type::collect_input_variants(&mut variants);)+
                variants
            }

            fn standard_variants() -> Vec<StandardVariant> {
                let mut variants = Vec::new();
                $($type::collect_variants(&mut variants);)+
                variants
            }

            fn input_lists() -> Vec<LibraryValueType> {
                let mut lists = Vec::new();
                $($type::collect_lists(&mut lists);)+
                lists
            }
        }
    };
}

arguments!(A);
arguments!(A, B);
arguments!(A, B, C);
arguments!(A, B, C, D);
arguments!(A, B, C, D, E);
arguments!(A, B, C, D, E, F);
arguments!(A, B, C, D, E, F, G);

macro_rules! scalar_return {
    ($type:ty, $entries:ident, $run:ident, $run_hosted:ident) => {
        impl ReturnValue for $type {
            fn input_constructions<
                Graph: crate::plan::execution::function::ExecutionGraphProfile,
            >(
                entries: &LibraryFunctionEntries<Graph>,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.$entries[slot].inputs()
            }

            fn call(
                module: &Module,
                slot: usize,
                inputs: RetainedValues,
                echo: &mut dyn EchoSink,
            ) -> Result<Self, ExecutionError> {
                let entry = &module.entries.$entries[slot];
                crate::runtime::$run(&module.execution, *entry.function(), inputs, echo)
            }

            fn call_hosted<Profile: HostProfile>(
                execution: &HostedExecution<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedValues,
                state: &mut Profile::RunState,
                echo: &mut dyn EchoSink,
                _owner: &Arc<()>,
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.$entries[slot];
                crate::runtime::$run_hosted(execution, *entry.function(), inputs, state, echo)
            }
        }
    };
}

scalar_return!(
    super::BigInt,
    ints,
    run_embedded_int,
    run_hosted_embedded_int
);
scalar_return!(f64, floats, run_embedded_float, run_hosted_embedded_float);
scalar_return!(
    super::EcoString,
    strings,
    run_embedded_string,
    run_hosted_embedded_string
);
scalar_return!(
    super::BitArrayValue,
    bit_arrays,
    run_embedded_bit_array,
    run_hosted_embedded_bit_array
);
scalar_return!(
    char,
    utf_codepoints,
    run_embedded_utf_codepoint,
    run_hosted_embedded_utf_codepoint
);
scalar_return!(bool, bools, run_embedded_bool, run_hosted_embedded_bool);
scalar_return!((), nils, run_embedded_nil, run_hosted_embedded_nil);

macro_rules! tuple_return {
    ($($type:ident),+) => {
        impl<$($type),+> ReturnValue for ($($type,)+)
        where
            $($type: OutputValue<LocalValues>,)+
        {
            fn input_constructions<Graph: crate::plan::execution::function::ExecutionGraphProfile>(
                entries: &LibraryFunctionEntries<Graph>,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.tuples[slot].inputs()
            }

            fn call(
                module: &Module,
                slot: usize,
                inputs: RetainedValues,
                echo: &mut dyn EchoSink,
            ) -> Result<Self, ExecutionError> {
                let entry = &module.entries.tuples[slot];
                crate::runtime::run_embedded_tuple(
                    &module.execution,
                    *entry.function(),
                    inputs,
                    echo,
                )
                .map(|mut output| {
                    <Self as OutputValue<LocalValues>>::take(&mut output, &module.owner)
                })
            }

            fn call_hosted<Profile: HostProfile>(
                execution: &HostedExecution<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedValues,
                state: &mut Profile::RunState,
                echo: &mut dyn EchoSink,
                owner: &std::sync::Arc<()>,
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.tuples[slot];
                crate::runtime::run_hosted_embedded_tuple(
                    execution,
                    *entry.function(),
                    inputs,
                    state,
                    echo,
                )
                .map(|mut output| <Self as OutputValue<LocalValues>>::take(&mut output, owner))
            }
        }
    };
}

tuple_return!(A);
tuple_return!(A, B);
tuple_return!(A, B, C);
tuple_return!(A, B, C, D);
tuple_return!(A, B, C, D, E);
tuple_return!(A, B, C, D, E, F);
tuple_return!(A, B, C, D, E, F, G);

macro_rules! custom_return {
    ($container:ty) => {
        impl<Success, Failure> ReturnValue for $container
        where
            Success: OutputValue<LocalValues>,
            Failure: OutputValue<LocalValues>,
        {
            fn input_constructions<
                Graph: crate::plan::execution::function::ExecutionGraphProfile,
            >(
                entries: &LibraryFunctionEntries<Graph>,
                slot: usize,
            ) -> &LibraryInputConstructions {
                entries.customs[slot].inputs()
            }

            fn call(
                module: &Module,
                slot: usize,
                inputs: RetainedValues,
                echo: &mut dyn EchoSink,
            ) -> Result<Self, ExecutionError> {
                let entry = &module.entries.customs[slot];
                crate::runtime::run_embedded_custom(
                    &module.execution,
                    *entry.function(),
                    inputs,
                    echo,
                )
                .map(|mut output| {
                    <Self as OutputValue<LocalValues>>::take(&mut output, &module.owner)
                })
            }

            fn call_hosted<Profile: HostProfile>(
                execution: &HostedExecution<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedValues,
                state: &mut Profile::RunState,
                echo: &mut dyn EchoSink,
                owner: &std::sync::Arc<()>,
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.customs[slot];
                crate::runtime::run_hosted_embedded_custom(
                    execution,
                    *entry.function(),
                    inputs,
                    state,
                    echo,
                )
                .map(|mut output| <Self as OutputValue<LocalValues>>::take(&mut output, owner))
            }
        }
    };
}

custom_return!(Result<Success, Failure>);

impl<Value> ReturnValue for Option<Value>
where
    Value: OutputValue<LocalValues>,
{
    fn input_constructions<Graph: crate::plan::execution::function::ExecutionGraphProfile>(
        entries: &LibraryFunctionEntries<Graph>,
        slot: usize,
    ) -> &LibraryInputConstructions {
        entries.customs[slot].inputs()
    }

    fn call(
        module: &Module,
        slot: usize,
        inputs: RetainedValues,
        echo: &mut dyn EchoSink,
    ) -> Result<Self, ExecutionError> {
        let entry = &module.entries.customs[slot];
        crate::runtime::run_embedded_custom(&module.execution, *entry.function(), inputs, echo)
            .map(|mut output| <Self as OutputValue<LocalValues>>::take(&mut output, &module.owner))
    }

    fn call_hosted<Profile: HostProfile>(
        execution: &HostedExecution<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedValues,
        state: &mut Profile::RunState,
        echo: &mut dyn EchoSink,
        owner: &std::sync::Arc<()>,
    ) -> Result<Self, ExecutionError> {
        let entry = &entries.customs[slot];
        crate::runtime::run_hosted_embedded_custom(
            execution,
            *entry.function(),
            inputs,
            state,
            echo,
        )
        .map(|mut output| <Self as OutputValue<LocalValues>>::take(&mut output, owner))
    }
}
