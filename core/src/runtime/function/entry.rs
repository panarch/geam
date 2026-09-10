use crate::plan::execution::function::{
    self as function, ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef,
    ExecutionHostTarget, ExecutionNeverHostTarget, FunctionBodyOwner,
};
use crate::runtime::error::HostCallOrigin;
use crate::runtime::graph::GraphValue;
use crate::runtime::graph::RetainedValues;
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};
use std::convert::Infallible;

pub(in crate::runtime) trait EntryTarget<Plan: ExecutableRuntimePlan>:
    Send + Sized
{
    type Body: ExecutionFunctionBody<
            Graph = RuntimeGraph<Plan>,
            Return: GraphValue + Sync + 'static,
            TailCall: Sync,
        > + Sync;

    type HostTarget: Clone + Send + Sync + 'static;

    fn entry<'plan>(
        &self,
        plan: &'plan Plan,
    ) -> ExecutionFunctionRef<'plan, Self::Body, Self::HostTarget>;

    fn prepare_host<'plan>(
        plan: &'plan Plan,
        origin: HostCallOrigin,
        target: &Self::HostTarget,
        inputs: RetainedValues,
    ) -> Plan::HostInvocation<
        'plan,
        <<Self::Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated,
    >;

    fn next(&self, target: &<Self::Body as FunctionBodyOwner>::TailCall) -> (Self, HostCallOrigin);
}

macro_rules! entry_target {
    ($id:ty, $body:ident, $entry:ident, |$current:ident| $argument:expr, |$previous:pat_param, $target:ident| $next:expr) => {
        impl<Plan: ExecutableRuntimePlan> EntryTarget<Plan> for $id {
            type Body = function::$body<Plan::Profile>;
            type HostTarget = ExecutionHostTarget<Plan::Profile, Self::Body>;

            fn entry<'plan>(
                &self,
                plan: &'plan Plan,
            ) -> ExecutionFunctionRef<'plan, Self::Body, Self::HostTarget> {
                let $current = self;
                plan.$entry($argument).as_ref()
            }

            fn prepare_host<'plan>(
                plan: &'plan Plan,
                origin: HostCallOrigin,
                target: &Self::HostTarget,
                inputs: RetainedValues,
            ) -> Plan::HostInvocation<
                'plan,
                <<Self::Body as FunctionBodyOwner>::Return as GraphValue>::Evaluated,
            > {
                plan.prepare_host::<Self::Body>(origin, target, inputs)
            }

            fn next(
                &self,
                target: &<Self::Body as FunctionBodyOwner>::TailCall,
            ) -> (Self, HostCallOrigin) {
                let $previous = self;
                let $target = target;
                ($next, HostCallOrigin::source(target.site().clone()))
            }
        }
    };
}

impl<Plan: ExecutableRuntimePlan> EntryTarget<Plan> for function::NeverFunctionId {
    type Body = function::ExecutionNeverFunctionBody<Plan::Profile>;
    type HostTarget = ExecutionNeverHostTarget<Plan::Profile>;

    fn entry<'plan>(
        &self,
        plan: &'plan Plan,
    ) -> ExecutionFunctionRef<'plan, Self::Body, Self::HostTarget> {
        plan.never_function(*self).as_ref()
    }

    fn prepare_host<'plan>(
        plan: &'plan Plan,
        origin: HostCallOrigin,
        target: &Self::HostTarget,
        inputs: RetainedValues,
    ) -> Plan::HostInvocation<'plan, Infallible> {
        plan.prepare_host_never(origin, target, inputs)
    }

    fn next(&self, target: &<Self::Body as FunctionBodyOwner>::TailCall) -> (Self, HostCallOrigin) {
        (
            *target.function(),
            HostCallOrigin::source(target.site().clone()),
        )
    }
}

entry_target!(
    function::IntFunctionId,
    ExecutionIntFunctionBody,
    int_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::FloatFunctionId,
    ExecutionFloatFunctionBody,
    float_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::StringFunctionId,
    ExecutionStringFunctionBody,
    string_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::BitArrayFunctionId,
    ExecutionBitArrayFunctionBody,
    bit_array_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::UtfCodepointFunctionId,
    ExecutionUtfCodepointFunctionBody,
    utf_codepoint_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::CustomFunctionId,
    ExecutionCustomFunctionBody,
    custom_function,
    |id| *id,
    |id, target| id.with_index(*target.function())
);
entry_target!(
    function::ExternalFunctionId,
    ExecutionExternalFunctionBody,
    external_function,
    |id| *id,
    |id, target| id.with_index(*target.function())
);
entry_target!(
    function::BoolFunctionId,
    ExecutionBoolFunctionBody,
    bool_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::NilFunctionId,
    ExecutionNilFunctionBody,
    nil_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::TupleFunctionId,
    ExecutionTupleFunctionBody,
    tuple_function,
    |id| *id,
    |_, target| *target.function()
);

entry_target!(
    function::ParameterListFunctionId,
    ExecutionParameterListFunctionBody,
    parameter_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::ParameterListListFunctionId,
    ExecutionParameterListListFunctionBody,
    parameter_list_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::IntListFunctionId,
    ExecutionIntListFunctionBody,
    int_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::StringListFunctionId,
    ExecutionStringListFunctionBody,
    string_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::BitArrayListFunctionId,
    ExecutionBitArrayListFunctionBody,
    bit_array_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::UtfCodepointListFunctionId,
    ExecutionUtfCodepointListFunctionBody,
    utf_codepoint_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::CustomListFunctionId,
    ExecutionCustomListFunctionBody,
    custom_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::ExternalListFunctionId,
    ExecutionExternalListFunctionBody,
    external_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::FloatListFunctionId,
    ExecutionFloatListFunctionBody,
    float_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::BoolListFunctionId,
    ExecutionBoolListFunctionBody,
    bool_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::NilListFunctionId,
    ExecutionNilListFunctionBody,
    nil_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::TupleListFunctionId,
    ExecutionTupleListFunctionBody,
    tuple_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::ListListFunctionId,
    ExecutionListListFunctionBody,
    list_list_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::FunctionListFunctionId,
    ExecutionFunctionListFunctionBody,
    function_list_function,
    |id| *id,
    |_, target| *target.function()
);

entry_target!(
    function::GenericFunctionFunctionId,
    ExecutionGenericFunctionFunctionBody,
    generic_function_function,
    |id| id,
    |_, target| target.function().clone()
);
entry_target!(
    function::NeverFunctionFunctionId,
    ExecutionNeverFunctionFunctionBody,
    never_function_function,
    |id| id,
    |_, target| target.function().clone()
);
entry_target!(
    function::IntFunctionFunctionId,
    ExecutionIntFunctionFunctionBody,
    int_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::FloatFunctionFunctionId,
    ExecutionFloatFunctionFunctionBody,
    float_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::StringFunctionFunctionId,
    ExecutionStringFunctionFunctionBody,
    string_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::BitArrayFunctionFunctionId,
    ExecutionBitArrayFunctionFunctionBody,
    bit_array_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::UtfCodepointFunctionFunctionId,
    ExecutionUtfCodepointFunctionFunctionBody,
    utf_codepoint_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::CustomFunctionFunctionId,
    ExecutionCustomFunctionFunctionBody,
    custom_function_function,
    |id| id,
    |id, target| id.with_index(*target.function())
);
entry_target!(
    function::ExternalFunctionFunctionId,
    ExecutionExternalFunctionFunctionBody,
    external_function_function,
    |id| id,
    |id, target| id.with_index(*target.function())
);
entry_target!(
    function::BoolFunctionFunctionId,
    ExecutionBoolFunctionFunctionBody,
    bool_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::NilFunctionFunctionId,
    ExecutionNilFunctionFunctionBody,
    nil_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::TupleFunctionFunctionId,
    ExecutionTupleFunctionFunctionBody,
    tuple_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::ProfiledListFunctionFunctionId<Infallible>,
    ExecutionCoreListFunctionFunctionBody,
    core_list_function_function,
    |id| id,
    |_, target| target.function().clone()
);
entry_target!(
    function::ExternalListFunctionFunctionId,
    ExecutionExternalListFunctionFunctionBody,
    external_list_function_function,
    |id| *id,
    |_, target| *target.function()
);
entry_target!(
    function::FunctionFunctionFunctionId,
    ExecutionFunctionFunctionFunctionBody,
    function_function_function,
    |id| id,
    |id, target| id.with_index(*target.function())
);
