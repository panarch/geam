use super::Module;
use crate::plan::execution::{LibraryFunctionEntries, LibraryInputConstructions};
use crate::plan::{LibraryReturn, StandardVariant, ValueType};
use crate::runtime::{EmbeddingInput, EmbeddingOutput, RetainedValues};
use crate::{EchoSink, ExecutionError, HostProfile, HostedExecution};

pub(super) trait EmbeddingValue: Sized {
    const VARIANT_COUNT: usize;

    fn value_type() -> ValueType;

    fn collect_variants(variants: &mut Vec<StandardVariant>);

    fn push(self, inputs: &mut RetainedValues, constructions: &mut InputConstructions<'_>);

    fn into_nested(self, constructions: &mut InputConstructions<'_>) -> EmbeddingInput;

    fn take(output: &mut EmbeddingOutput) -> Self;
}

pub(super) trait Arguments {
    fn value_types() -> Vec<ValueType>;

    fn input_variants() -> Vec<StandardVariant>;

    fn into_inputs(self, constructions: &LibraryInputConstructions) -> RetainedValues;
}

pub(super) trait ReturnValue: EmbeddingValue {
    fn return_() -> LibraryReturn;

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
    ) -> Result<Self, ExecutionError>;
}

pub(super) struct InputConstructions<'a> {
    variants: &'a [[crate::plan::execution::type_::CustomConstructorId; 2]],
    next: usize,
}

impl InputConstructions<'_> {
    fn new(constructions: &LibraryInputConstructions) -> InputConstructions<'_> {
        InputConstructions {
            variants: constructions.variants(),
            next: 0,
        }
    }

    fn take_variant(&mut self) -> [crate::plan::execution::type_::CustomConstructorId; 2] {
        let variant = self.variants[self.next];
        self.next += 1;
        variant
    }

    fn skip(&mut self, count: usize) {
        self.next += count;
    }
}

macro_rules! scalar_value {
    ($type:ty, $value_type:ident, $input:ident, $take:ident) => {
        impl EmbeddingValue for $type {
            const VARIANT_COUNT: usize = 0;

            fn value_type() -> ValueType {
                ValueType::$value_type
            }

            fn collect_variants(_variants: &mut Vec<StandardVariant>) {}

            fn push(
                self,
                inputs: &mut RetainedValues,
                _constructions: &mut InputConstructions<'_>,
            ) {
                EmbeddingInput::$input(self).retain(inputs);
            }

            fn into_nested(self, _constructions: &mut InputConstructions<'_>) -> EmbeddingInput {
                EmbeddingInput::$input(self)
            }

            fn take(output: &mut EmbeddingOutput) -> Self {
                output.$take()
            }
        }
    };
}

scalar_value!(super::BigInt, Int, int, take_int);
scalar_value!(f64, Float, float, take_float);
scalar_value!(super::EcoString, String, string, take_string);
scalar_value!(super::BitArrayValue, BitArray, bit_array, take_bit_array);
scalar_value!(char, UtfCodepoint, utf_codepoint, take_utf_codepoint);
scalar_value!(bool, Bool, bool, take_bool);

impl EmbeddingValue for () {
    const VARIANT_COUNT: usize = 0;

    fn value_type() -> ValueType {
        ValueType::Nil
    }

    fn collect_variants(_variants: &mut Vec<StandardVariant>) {}

    fn push(self, inputs: &mut RetainedValues, _constructions: &mut InputConstructions<'_>) {
        EmbeddingInput::nil().retain(inputs);
    }

    fn into_nested(self, _constructions: &mut InputConstructions<'_>) -> EmbeddingInput {
        EmbeddingInput::nil()
    }

    fn take(output: &mut EmbeddingOutput) -> Self {
        output.take_nil();
    }
}

macro_rules! tuple_value {
    ($($type:ident => $value:ident),+) => {
        impl<$($type),+> EmbeddingValue for ($($type,)+)
        where
            $($type: EmbeddingValue,)+
        {
            const VARIANT_COUNT: usize = 0 $(+ $type::VARIANT_COUNT)+;

            fn value_type() -> ValueType {
                ValueType::Tuple(vec![$($type::value_type()),+])
            }

            fn collect_variants(variants: &mut Vec<StandardVariant>) {
                $($type::collect_variants(variants);)+
            }

            fn push(
                self,
                inputs: &mut RetainedValues,
                constructions: &mut InputConstructions<'_>,
            ) {
                let ($($value,)+) = self;
                EmbeddingInput::tuple([$($value.into_nested(constructions)),+])
                    .retain(inputs);
            }

            fn into_nested(
                self,
                constructions: &mut InputConstructions<'_>,
            ) -> EmbeddingInput {
                let ($($value,)+) = self;
                EmbeddingInput::tuple([$($value.into_nested(constructions)),+])
            }

            fn take(output: &mut EmbeddingOutput) -> Self {
                ($($type::take(output),)+)
            }
        }
    };
}

tuple_value!(A => a);
tuple_value!(A => a, B => b);
tuple_value!(A => a, B => b, C => c);
tuple_value!(A => a, B => b, C => c, D => d);
tuple_value!(A => a, B => b, C => c, D => d, E => e);
tuple_value!(A => a, B => b, C => c, D => d, E => e, F => f);
tuple_value!(A => a, B => b, C => c, D => d, E => e, F => f, G => g);

impl<Success, Failure> EmbeddingValue for Result<Success, Failure>
where
    Success: EmbeddingValue,
    Failure: EmbeddingValue,
{
    const VARIANT_COUNT: usize = 1 + Success::VARIANT_COUNT + Failure::VARIANT_COUNT;

    fn value_type() -> ValueType {
        ValueType::Custom(
            StandardVariant::Result.custom_type(vec![Success::value_type(), Failure::value_type()]),
        )
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Result);
        Success::collect_variants(variants);
        Failure::collect_variants(variants);
    }

    fn push(self, inputs: &mut RetainedValues, constructions: &mut InputConstructions<'_>) {
        result_value(self, constructions).retain(inputs);
    }

    fn into_nested(self, constructions: &mut InputConstructions<'_>) -> EmbeddingInput {
        result_value(self, constructions)
    }

    fn take(output: &mut EmbeddingOutput) -> Self {
        if output.take_variant() == 0 {
            Ok(Success::take(output))
        } else {
            Err(Failure::take(output))
        }
    }
}

impl<Value> EmbeddingValue for Option<Value>
where
    Value: EmbeddingValue,
{
    const VARIANT_COUNT: usize = 1 + Value::VARIANT_COUNT;

    fn value_type() -> ValueType {
        ValueType::Custom(StandardVariant::Option.custom_type(vec![Value::value_type()]))
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        variants.push(StandardVariant::Option);
        Value::collect_variants(variants);
    }

    fn push(self, inputs: &mut RetainedValues, constructions: &mut InputConstructions<'_>) {
        option_value(self, constructions).retain(inputs);
    }

    fn into_nested(self, constructions: &mut InputConstructions<'_>) -> EmbeddingInput {
        option_value(self, constructions)
    }

    fn take(output: &mut EmbeddingOutput) -> Self {
        if output.take_variant() == 0 {
            Some(Value::take(output))
        } else {
            None
        }
    }
}

fn result_value<Success, Failure>(
    value: Result<Success, Failure>,
    constructions: &mut InputConstructions<'_>,
) -> EmbeddingInput
where
    Success: EmbeddingValue,
    Failure: EmbeddingValue,
{
    let constructors = constructions.take_variant();
    match value {
        Ok(value) => {
            let field = value.into_nested(constructions);
            constructions.skip(Failure::VARIANT_COUNT);
            EmbeddingInput::custom(constructors[0], [field])
        }
        Err(value) => {
            constructions.skip(Success::VARIANT_COUNT);
            let field = value.into_nested(constructions);
            EmbeddingInput::custom(constructors[1], [field])
        }
    }
}

fn option_value<Value>(
    value: Option<Value>,
    constructions: &mut InputConstructions<'_>,
) -> EmbeddingInput
where
    Value: EmbeddingValue,
{
    let constructors = constructions.take_variant();
    match value {
        Some(value) => EmbeddingInput::custom(constructors[0], [value.into_nested(constructions)]),
        None => {
            constructions.skip(Value::VARIANT_COUNT);
            EmbeddingInput::custom(constructors[1], [])
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

    fn into_inputs(self, _constructions: &LibraryInputConstructions) -> RetainedValues {
        RetainedValues::empty()
    }
}

macro_rules! arguments {
    ($($type:ident => $value:ident),+) => {
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

            fn into_inputs(self, constructions: &LibraryInputConstructions) -> RetainedValues {
                let ($($value,)+) = self;
                let mut constructions = InputConstructions::new(constructions);
                let mut inputs = RetainedValues::empty();
                $($value.push(&mut inputs, &mut constructions);)+
                inputs
            }
        }
    };
}

arguments!(A => a);
arguments!(A => a, B => b);
arguments!(A => a, B => b, C => c);
arguments!(A => a, B => b, C => c, D => d);
arguments!(A => a, B => b, C => c, D => d, E => e);
arguments!(A => a, B => b, C => c, D => d, E => e, F => f);
arguments!(A => a, B => b, C => c, D => d, E => e, F => f, G => g);

macro_rules! scalar_return {
    ($type:ty, $return_:ident, $entries:ident, $run:ident, $run_hosted:ident) => {
        impl ReturnValue for $type {
            fn return_() -> LibraryReturn {
                LibraryReturn::$return_
            }

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
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.$entries[slot];
                crate::runtime::$run_hosted(execution, *entry.function(), inputs, state, echo)
            }
        }
    };
}

scalar_return!(
    super::BigInt,
    Int,
    ints,
    run_embedded_int,
    run_hosted_embedded_int
);
scalar_return!(
    f64,
    Float,
    floats,
    run_embedded_float,
    run_hosted_embedded_float
);
scalar_return!(
    super::EcoString,
    String,
    strings,
    run_embedded_string,
    run_hosted_embedded_string
);
scalar_return!(
    super::BitArrayValue,
    BitArray,
    bit_arrays,
    run_embedded_bit_array,
    run_hosted_embedded_bit_array
);
scalar_return!(
    char,
    UtfCodepoint,
    utf_codepoints,
    run_embedded_utf_codepoint,
    run_hosted_embedded_utf_codepoint
);
scalar_return!(
    bool,
    Bool,
    bools,
    run_embedded_bool,
    run_hosted_embedded_bool
);
scalar_return!((), Nil, nils, run_embedded_nil, run_hosted_embedded_nil);

macro_rules! tuple_return {
    ($($type:ident),+) => {
        impl<$($type),+> ReturnValue for ($($type,)+)
        where
            $($type: EmbeddingValue,)+
        {
            fn return_() -> LibraryReturn {
                LibraryReturn::Tuple(vec![$($type::value_type()),+])
            }

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
                .map(|mut output| Self::take(&mut output))
            }

            fn call_hosted<Profile: HostProfile>(
                execution: &HostedExecution<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedValues,
                state: &mut Profile::RunState,
                echo: &mut dyn EchoSink,
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.tuples[slot];
                crate::runtime::run_hosted_embedded_tuple(
                    execution,
                    *entry.function(),
                    inputs,
                    state,
                    echo,
                )
                .map(|mut output| Self::take(&mut output))
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
            fn return_() -> LibraryReturn {
                LibraryReturn::Custom(
                    StandardVariant::Result
                        .custom_type(vec![Success::value_type(), Failure::value_type()]),
                )
            }

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
                .map(|mut output| Self::take(&mut output))
            }

            fn call_hosted<Profile: HostProfile>(
                execution: &HostedExecution<Profile>,
                entries: &LibraryFunctionEntries,
                slot: usize,
                inputs: RetainedValues,
                state: &mut Profile::RunState,
                echo: &mut dyn EchoSink,
            ) -> Result<Self, ExecutionError> {
                let entry = &entries.customs[slot];
                crate::runtime::run_hosted_embedded_custom(
                    execution,
                    *entry.function(),
                    inputs,
                    state,
                    echo,
                )
                .map(|mut output| Self::take(&mut output))
            }
        }
    };
}

custom_return!(Result<Success, Failure>);

impl<Value> ReturnValue for Option<Value>
where
    Value: EmbeddingValue,
{
    fn return_() -> LibraryReturn {
        LibraryReturn::Custom(StandardVariant::Option.custom_type(vec![Value::value_type()]))
    }

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
            .map(|mut output| <Self as EmbeddingValue>::take(&mut output))
    }

    fn call_hosted<Profile: HostProfile>(
        execution: &HostedExecution<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedValues,
        state: &mut Profile::RunState,
        echo: &mut dyn EchoSink,
    ) -> Result<Self, ExecutionError> {
        let entry = &entries.customs[slot];
        crate::runtime::run_hosted_embedded_custom(
            execution,
            *entry.function(),
            inputs,
            state,
            echo,
        )
        .map(|mut output| <Self as EmbeddingValue>::take(&mut output))
    }
}
