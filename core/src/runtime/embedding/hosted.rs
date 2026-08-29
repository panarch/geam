use super::super::error::HostCallOrigin;
use super::super::function;
use super::super::graph::RetainedValues;
use super::super::state::RuntimeState;
use super::super::{EchoSink, EvaluatedBitArray, ExecutionError};
use crate::host::HostProfile;
use crate::plan::execution::HostedExecution;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
    StringFunctionId, UtfCodepointFunctionId,
};

pub(crate) fn run_hosted_embedded_int<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: IntFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<num_bigint::BigInt, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_int(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_hosted_embedded_float<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: FloatFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<f64, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_float(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_hosted_embedded_string<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: StringFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<ecow::EcoString, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_string(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_hosted_embedded_bit_array<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: BitArrayFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<crate::BitArrayValue, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_bit_array(plan, &mut state, function, HostCallOrigin::Entry, inputs)
        .map(EvaluatedBitArray::into_value)
}

pub(crate) fn run_hosted_embedded_utf_codepoint<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: UtfCodepointFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<char, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_utf_codepoint(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_hosted_embedded_bool<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: BoolFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<bool, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_bool(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}

pub(crate) fn run_hosted_embedded_nil<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    function: NilFunctionId,
    inputs: RetainedValues,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<(), ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    function::run_nil(plan, &mut state, function, HostCallOrigin::Entry, inputs)
}
