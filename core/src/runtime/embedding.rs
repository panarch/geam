mod hosted;
mod input;
mod list;
mod output;

pub(crate) use input::{
    EmbeddingCustomInput, EmbeddingInputStorage, EmbeddingInputValue, EmbeddingListInput,
    EmbeddingTupleInput,
};
pub(crate) use list::EmbeddingList;
pub(crate) use output::EmbeddingOutput;

pub(crate) use hosted::{
    run_hosted_embedded_bit_array, run_hosted_embedded_bool, run_hosted_embedded_custom,
    run_hosted_embedded_float, run_hosted_embedded_int, run_hosted_embedded_list,
    run_hosted_embedded_nil, run_hosted_embedded_string, run_hosted_embedded_tuple,
    run_hosted_embedded_utf_codepoint,
};

use super::error::HostCallOrigin;
use super::function;
use super::graph::RetainedValues;
use super::state::RuntimeState;
use super::{EchoSink, EvaluatedBitArray, ExecutionError};
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExecutionGraphProfile, FloatFunctionId,
    IntFunctionId, NilFunctionId, ProfiledListFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};

pub(crate) fn run_embedded_int(
    plan: &ExecutionPlan,
    function: IntFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<num_bigint::BigInt, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_int(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_float(
    plan: &ExecutionPlan,
    function: FloatFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<f64, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_float(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_string(
    plan: &ExecutionPlan,
    function: StringFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<ecow::EcoString, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_string(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_bit_array(
    plan: &ExecutionPlan,
    function: BitArrayFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<crate::BitArrayValue, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_bit_array(plan, &mut state, function, HostCallOrigin::Entry, inputs)
        .map(EvaluatedBitArray::into_value)
}

pub(crate) fn run_embedded_utf_codepoint(
    plan: &ExecutionPlan,
    function: UtfCodepointFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<char, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_utf_codepoint(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_custom(
    plan: &ExecutionPlan,
    function: CustomFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<EmbeddingOutput, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_custom(plan, &mut state, function, HostCallOrigin::Entry, inputs)
        .map(EmbeddingOutput::from_custom)
}

pub(crate) fn run_embedded_bool(
    plan: &ExecutionPlan,
    function: BoolFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<bool, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_bool(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_nil(
    plan: &ExecutionPlan,
    function: NilFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<(), ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_nil(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_embedded_tuple(
    plan: &ExecutionPlan,
    function: TupleFunctionId,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<EmbeddingOutput, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    function::run_tuple(plan, &mut state, function, HostCallOrigin::Entry, inputs)
        .map(EmbeddingOutput::from_tuple)
}

pub(crate) fn run_embedded_list(
    plan: &ExecutionPlan,
    function: &ProfiledListFunctionId<std::convert::Infallible>,
    inputs: RetainedValues,
    echo: &mut dyn EchoSink,
) -> Result<EmbeddingOutput, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    let function = std::convert::Infallible::list_function(function);
    function::run_list(plan, &mut state, function, HostCallOrigin::Entry, inputs)
        .map(|value| EmbeddingOutput::from_value(value.into()))
}
