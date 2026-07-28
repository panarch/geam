use crate::host::{HostCallError, HostProfile};
use crate::plan::execution::HostedExecution;
use crate::plan::execution::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayListFunctionBody, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolListFunctionBody, CustomFunctionBody, CustomFunctionFunctionBody,
    CustomListFunctionBody, ExecutionFunctionBody, FloatFunctionBody, FloatFunctionFunctionBody,
    FloatListFunctionBody, FunctionFunctionFunctionBody, FunctionListFunctionBody,
    GenericFunctionFunctionBody, IntFunctionBody, IntFunctionFunctionBody, IntListFunctionBody,
    ListFunctionFunctionBody, ListListFunctionBody, NeverFunctionBody, NeverFunctionFunctionBody,
    NilFunctionBody, NilFunctionFunctionBody, NilListFunctionBody, ParameterListFunctionBody,
    ParameterListListFunctionBody, StringFunctionBody, StringFunctionFunctionBody,
    StringListFunctionBody, TupleFunctionBody, TupleFunctionFunctionBody, TupleListFunctionBody,
    UtfCodepointFunctionBody, UtfCodepointFunctionFunctionBody, UtfCodepointListFunctionBody,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::host::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNeverFunctionId, HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId,
    HostedFunctionMetadata, HostedFunctionTarget,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::{GraphValue, RetainedValues};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{EvaluatedBitArray, ExecutionError};
use std::convert::Infallible;

pub(in crate::runtime) trait HostFunctionRuntime<Body: ExecutionFunctionBody>:
    RuntimeExecutionPlan
where
    Body::Return: GraphValue,
{
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &crate::plan::execution::function::ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<Body::Return as GraphValue>::Evaluated>;

    fn host_parameters(
        &self,
        target: &crate::plan::execution::function::ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal];
}

pub(in crate::runtime) trait CompleteHostRuntime:
    RuntimeExecutionPlan
    + HostFunctionRuntime<NeverFunctionBody>
    + HostFunctionRuntime<IntFunctionBody>
    + HostFunctionRuntime<FloatFunctionBody>
    + HostFunctionRuntime<StringFunctionBody>
    + HostFunctionRuntime<BitArrayFunctionBody>
    + HostFunctionRuntime<UtfCodepointFunctionBody>
    + HostFunctionRuntime<CustomFunctionBody>
    + HostFunctionRuntime<BoolFunctionBody>
    + HostFunctionRuntime<NilFunctionBody>
    + HostFunctionRuntime<TupleFunctionBody>
    + HostFunctionRuntime<ParameterListFunctionBody>
    + HostFunctionRuntime<IntListFunctionBody>
    + HostFunctionRuntime<FloatListFunctionBody>
    + HostFunctionRuntime<StringListFunctionBody>
    + HostFunctionRuntime<BitArrayListFunctionBody>
    + HostFunctionRuntime<UtfCodepointListFunctionBody>
    + HostFunctionRuntime<CustomListFunctionBody>
    + HostFunctionRuntime<BoolListFunctionBody>
    + HostFunctionRuntime<NilListFunctionBody>
    + HostFunctionRuntime<TupleListFunctionBody>
    + HostFunctionRuntime<ParameterListListFunctionBody>
    + HostFunctionRuntime<ListListFunctionBody>
    + HostFunctionRuntime<FunctionListFunctionBody>
    + HostFunctionRuntime<IntFunctionFunctionBody>
    + HostFunctionRuntime<FloatFunctionFunctionBody>
    + HostFunctionRuntime<StringFunctionFunctionBody>
    + HostFunctionRuntime<BitArrayFunctionFunctionBody>
    + HostFunctionRuntime<UtfCodepointFunctionFunctionBody>
    + HostFunctionRuntime<GenericFunctionFunctionBody>
    + HostFunctionRuntime<NeverFunctionFunctionBody>
    + HostFunctionRuntime<CustomFunctionFunctionBody>
    + HostFunctionRuntime<BoolFunctionFunctionBody>
    + HostFunctionRuntime<NilFunctionFunctionBody>
    + HostFunctionRuntime<TupleFunctionFunctionBody>
    + HostFunctionRuntime<ListFunctionFunctionBody>
    + HostFunctionRuntime<FunctionFunctionFunctionBody>
{
}

impl<Plan> CompleteHostRuntime for Plan where
    Plan: RuntimeExecutionPlan
        + HostFunctionRuntime<NeverFunctionBody>
        + HostFunctionRuntime<IntFunctionBody>
        + HostFunctionRuntime<FloatFunctionBody>
        + HostFunctionRuntime<StringFunctionBody>
        + HostFunctionRuntime<BitArrayFunctionBody>
        + HostFunctionRuntime<UtfCodepointFunctionBody>
        + HostFunctionRuntime<CustomFunctionBody>
        + HostFunctionRuntime<BoolFunctionBody>
        + HostFunctionRuntime<NilFunctionBody>
        + HostFunctionRuntime<TupleFunctionBody>
        + HostFunctionRuntime<ParameterListFunctionBody>
        + HostFunctionRuntime<IntListFunctionBody>
        + HostFunctionRuntime<FloatListFunctionBody>
        + HostFunctionRuntime<StringListFunctionBody>
        + HostFunctionRuntime<BitArrayListFunctionBody>
        + HostFunctionRuntime<UtfCodepointListFunctionBody>
        + HostFunctionRuntime<CustomListFunctionBody>
        + HostFunctionRuntime<BoolListFunctionBody>
        + HostFunctionRuntime<NilListFunctionBody>
        + HostFunctionRuntime<TupleListFunctionBody>
        + HostFunctionRuntime<ParameterListListFunctionBody>
        + HostFunctionRuntime<ListListFunctionBody>
        + HostFunctionRuntime<FunctionListFunctionBody>
        + HostFunctionRuntime<IntFunctionFunctionBody>
        + HostFunctionRuntime<FloatFunctionFunctionBody>
        + HostFunctionRuntime<StringFunctionFunctionBody>
        + HostFunctionRuntime<BitArrayFunctionFunctionBody>
        + HostFunctionRuntime<UtfCodepointFunctionFunctionBody>
        + HostFunctionRuntime<GenericFunctionFunctionBody>
        + HostFunctionRuntime<NeverFunctionFunctionBody>
        + HostFunctionRuntime<CustomFunctionFunctionBody>
        + HostFunctionRuntime<BoolFunctionFunctionBody>
        + HostFunctionRuntime<NilFunctionFunctionBody>
        + HostFunctionRuntime<TupleFunctionFunctionBody>
        + HostFunctionRuntime<ListFunctionFunctionBody>
        + HostFunctionRuntime<FunctionFunctionFunctionBody>
{
}

impl<Body> HostFunctionRuntime<Body> for crate::ExecutionPlan
where
    Body: ExecutionFunctionBody,
    Body::Return: GraphValue,
{
    fn call_host(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &crate::plan::execution::function::ExecutionHostTarget<Self::Profile, Body>,
        _inputs: RetainedValues,
    ) -> ExecutionResult<<Body::Return as GraphValue>::Evaluated> {
        match *target {}
    }

    fn host_parameters(
        &self,
        target: &crate::plan::execution::function::ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal] {
        match *target {}
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<IntFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostIntFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<num_bigint::BigInt> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().int(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(&self, target: &HostedFunctionTarget<HostIntFunctionId>) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| tables.int(target).metadata())
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<FloatFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostFloatFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<f64> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().float(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(&self, target: &HostedFunctionTarget<HostFloatFunctionId>) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| {
            tables.float(target).metadata()
        })
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<StringFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostStringFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<ecow::EcoString> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().string(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(
        &self,
        target: &HostedFunctionTarget<HostStringFunctionId>,
    ) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| {
            tables.string(target).metadata()
        })
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<BitArrayFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostBitArrayFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<EvaluatedBitArray> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().bit_array(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function
                        .call(state.host_state(), &inputs)
                        .map(EvaluatedBitArray::from_value),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(
        &self,
        target: &HostedFunctionTarget<HostBitArrayFunctionId>,
    ) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| {
            tables.bit_array(target).metadata()
        })
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<UtfCodepointFunctionBody>
    for HostedExecution<Profile>
{
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostUtfCodepointFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<char> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().utf_codepoint(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(
        &self,
        target: &HostedFunctionTarget<HostUtfCodepointFunctionId>,
    ) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| {
            tables.utf_codepoint(target).metadata()
        })
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<BoolFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostBoolFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<bool> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().bool(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(&self, target: &HostedFunctionTarget<HostBoolFunctionId>) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| {
            tables.bool(target).metadata()
        })
    }
}

impl<Profile: HostProfile> HostFunctionRuntime<NilFunctionBody> for HostedExecution<Profile> {
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<HostNilFunctionId>,
        inputs: RetainedValues,
    ) -> ExecutionResult<()> {
        match target {
            HostedFunctionTarget::Value(target) => {
                let function = self.host_functions().nil(*target);
                host_result(
                    self,
                    origin,
                    function.metadata(),
                    function.call(state.host_state(), &inputs),
                )
            }
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(&self, target: &HostedFunctionTarget<HostNilFunctionId>) -> &[ParamLocal] {
        host_parameters(self, target, |tables, target| tables.nil(target).metadata())
    }
}

impl<Profile, Body> HostFunctionRuntime<Body> for HostedExecution<Profile>
where
    Profile: HostProfile,
    Body: ExecutionFunctionBody<HostValueTarget = Infallible>,
    Body::Return: GraphValue,
{
    fn call_host(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &HostedFunctionTarget<Infallible>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<Body::Return as GraphValue>::Evaluated> {
        match target {
            HostedFunctionTarget::Value(target) => match *target {},
            HostedFunctionTarget::Never(target) => {
                call_never(self, state, origin, *target, &inputs)
            }
        }
    }

    fn host_parameters(&self, target: &HostedFunctionTarget<Infallible>) -> &[ParamLocal] {
        match target {
            HostedFunctionTarget::Value(target) => match *target {},
            HostedFunctionTarget::Never(target) => {
                self.host_functions().never(*target).parameters()
            }
        }
    }
}

fn call_never<Profile: HostProfile, Return>(
    plan: &HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'_, HostedExecution<Profile>>,
    origin: HostCallOrigin,
    target: HostNeverFunctionId,
    inputs: &RetainedValues,
) -> ExecutionResult<Return> {
    let function = plan.host_functions().never(target);
    match function.call(state.host_state(), inputs) {
        Ok(never) => match never {},
        Err(error) => Err(host_error(plan, origin, function.metadata(), error)),
    }
}

fn host_result<Profile: HostProfile, Return>(
    plan: &HostedExecution<Profile>,
    origin: HostCallOrigin,
    function: &HostedFunctionMetadata,
    result: Result<Return, HostCallError>,
) -> ExecutionResult<Return> {
    result.map_err(|error| host_error(plan, origin, function, error))
}

fn host_error<Profile: HostProfile>(
    plan: &HostedExecution<Profile>,
    origin: HostCallOrigin,
    function: &HostedFunctionMetadata,
    error: HostCallError,
) -> ExecutionError {
    let site = origin.into_site(function.site());
    ExecutionError::from_host_call(
        function,
        site.clone(),
        plan.source_context_for(site.module()),
        error,
    )
}

fn host_parameters<'a, Profile: HostProfile, Target: Copy>(
    plan: &'a HostedExecution<Profile>,
    target: &HostedFunctionTarget<Target>,
    value: impl FnOnce(
        &'a crate::plan::execution::host::HostFunctionTables<Profile>,
        Target,
    ) -> &'a HostedFunctionMetadata,
) -> &'a [ParamLocal] {
    match target {
        HostedFunctionTarget::Value(target) => value(plan.host_functions(), *target).parameters(),
        HostedFunctionTarget::Never(target) => plan.host_functions().never(*target).parameters(),
    }
}
