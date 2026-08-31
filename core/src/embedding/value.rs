use super::Module;
use super::input::{ListFamily, add_list_counts};
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryInputConstructions, LibraryListConstructions,
};
use crate::plan::{LibraryValueType, StandardVariant, ValueType};
use crate::runtime::{
    EmbeddingCustomInput, EmbeddingInputValue, EmbeddingOutput, EmbeddingTupleInput, RetainedValues,
};
use crate::{EchoSink, ExecutionError, HostProfile, HostedExecution};
use std::sync::Arc;

pub(super) trait EmbeddingValue: Sized {
    type Runtime: EmbeddingInputValue;

    const VARIANT_COUNT: usize;
    const LIST_COUNTS: [usize; 10];
    const LIST_FAMILY: ListFamily;

    fn library_type() -> LibraryValueType;

    fn value_type() -> ValueType {
        Self::library_type().value_type()
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>);

    fn collect_lists(lists: &mut Vec<LibraryValueType>);

    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::Runtime as EmbeddingInputValue>::ListType;

    fn take(output: &mut EmbeddingOutput, owner: &Arc<()>) -> Self;
}

pub(super) trait Arguments {
    fn value_types() -> Vec<ValueType>;

    fn input_variants() -> Vec<StandardVariant>;

    fn input_lists() -> Vec<LibraryValueType>;
}

pub(super) trait ReturnValue: EmbeddingValue {
    fn standard_variants() -> Vec<StandardVariant> {
        let mut variants = Vec::with_capacity(Self::VARIANT_COUNT);
        Self::collect_variants(&mut variants);
        variants
    }

    fn input_constructions(
        entries: &LibraryFunctionEntries,
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
    ($type:ty, $value_type:ident, $lists:ident, $take:ident) => {
        impl EmbeddingValue for $type {
            type Runtime = Self;

            const VARIANT_COUNT: usize = 0;
            const LIST_COUNTS: [usize; 10] = [0; 10];
            const LIST_FAMILY: ListFamily = ListFamily::$value_type;

            fn library_type() -> LibraryValueType {
                LibraryValueType::$value_type
            }

            fn collect_variants(_variants: &mut Vec<StandardVariant>) {}

            fn collect_lists(_lists: &mut Vec<LibraryValueType>) {}

            fn list_id(
                lists: &LibraryListConstructions,
                index: usize,
            ) -> <Self::Runtime as EmbeddingInputValue>::ListType {
                lists.$lists[index]
            }

            fn take(output: &mut EmbeddingOutput, _owner: &Arc<()>) -> Self {
                output.$take()
            }
        }
    };
}

scalar_value!(super::BigInt, Int, ints, take_int);
scalar_value!(f64, Float, floats, take_float);
scalar_value!(super::EcoString, String, strings, take_string);
scalar_value!(super::BitArrayValue, BitArray, bit_arrays, take_bit_array);
scalar_value!(char, UtfCodepoint, utf_codepoints, take_utf_codepoint);
scalar_value!(bool, Bool, bools, take_bool);
scalar_value!((), Nil, nils, take_nil);

macro_rules! tuple_value {
    ($($type:ident),+) => {
        impl<$($type),+> EmbeddingValue for ($($type,)+)
        where
            $($type: EmbeddingValue,)+
        {
            type Runtime = EmbeddingTupleInput;

            const VARIANT_COUNT: usize = 0 $(+ $type::VARIANT_COUNT)+;
            const LIST_COUNTS: [usize; 10] = {
                let counts = [0; 10];
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

            fn collect_lists(lists: &mut Vec<LibraryValueType>) {
                $($type::collect_lists(lists);)+
            }

            fn list_id(
                lists: &LibraryListConstructions,
                index: usize,
            ) -> <Self::Runtime as EmbeddingInputValue>::ListType {
                lists.tuples[index]
            }

            fn take(output: &mut EmbeddingOutput, owner: &Arc<()>) -> Self {
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
    type Runtime = EmbeddingCustomInput;

    const VARIANT_COUNT: usize = 1 + Success::VARIANT_COUNT + Failure::VARIANT_COUNT;
    const LIST_COUNTS: [usize; 10] = add_list_counts(Success::LIST_COUNTS, Failure::LIST_COUNTS);
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

    fn collect_lists(lists: &mut Vec<LibraryValueType>) {
        Success::collect_lists(lists);
        Failure::collect_lists(lists);
    }

    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::Runtime as EmbeddingInputValue>::ListType {
        lists.customs[index]
    }

    fn take(output: &mut EmbeddingOutput, owner: &Arc<()>) -> Self {
        if output.take_variant() == 0 {
            Ok(Success::take(output, owner))
        } else {
            Err(Failure::take(output, owner))
        }
    }
}

impl<Value: EmbeddingValue> EmbeddingValue for Option<Value> {
    type Runtime = EmbeddingCustomInput;

    const VARIANT_COUNT: usize = 1 + Value::VARIANT_COUNT;
    const LIST_COUNTS: [usize; 10] = Value::LIST_COUNTS;
    const LIST_FAMILY: ListFamily = ListFamily::Custom;

    fn library_type() -> LibraryValueType {
        LibraryValueType::Custom(StandardVariant::Option.custom_type(vec![Value::value_type()]))
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Option);
        Value::collect_variants(variants);
    }

    fn collect_lists(lists: &mut Vec<LibraryValueType>) {
        Value::collect_lists(lists);
    }

    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::Runtime as EmbeddingInputValue>::ListType {
        lists.customs[index]
    }

    fn take(output: &mut EmbeddingOutput, owner: &Arc<()>) -> Self {
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
            fn input_constructions(
                entries: &LibraryFunctionEntries,
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
            $($type: EmbeddingValue,)+
        {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
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
                .map(|mut output| Self::take(&mut output, &module.owner))
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
                .map(|mut output| Self::take(&mut output, owner))
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
            Success: EmbeddingValue,
            Failure: EmbeddingValue,
        {
            fn input_constructions(
                entries: &LibraryFunctionEntries,
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
                .map(|mut output| Self::take(&mut output, &module.owner))
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
                .map(|mut output| Self::take(&mut output, owner))
            }
        }
    };
}

custom_return!(Result<Success, Failure>);

impl<Value> ReturnValue for Option<Value>
where
    Value: EmbeddingValue,
{
    fn input_constructions(
        entries: &LibraryFunctionEntries,
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
            .map(|mut output| <Self as EmbeddingValue>::take(&mut output, &module.owner))
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
        .map(|mut output| <Self as EmbeddingValue>::take(&mut output, owner))
    }
}
