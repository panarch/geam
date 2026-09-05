use super::ResumableState;
use crate::plan::execution::AsyncHostedExecution;
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, FunctionBodyOwner, FunctionExit,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::HostCallOrigin;
use crate::runtime::graph::{GraphValue, ProfiledRetainedValues};
use crate::runtime::{TransferExecutionError, TransferValues};
use std::future::Future;
use std::pin::Pin;

pub(in crate::runtime) type ResumableFuture<'call, Return> =
    Pin<Box<dyn Future<Output = super::TransferExecutionResult<Return>> + Send + 'call>>;

enum EvaluatedFunctionExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: ProfiledRetainedValues<TransferValues>,
    },
}

fn invoke_immediate_never<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    origin: HostCallOrigin,
    target: crate::plan::execution::host::HostNeverFunctionId,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> super::TransferExecutionResult<std::convert::Infallible>
where
    Profile: crate::HostProfile,
{
    let function = plan.host_never_function(target);
    let result =
        crate::plan::execution::host::call_resumable_never(function, state.host(), &inputs);
    drop(inputs);
    result.map_err(|failure| {
        TransferExecutionError::host_failure(plan, origin, function.metadata(), failure)
    })
}

fn async_host_error(
    plan: &impl RuntimeExecutionPlan,
    origin: HostCallOrigin,
    function: &crate::plan::execution::host::HostedFunctionMetadata,
    error: crate::AsyncHostCallError,
) -> TransferExecutionError {
    match crate::plan::execution::host::async_host_failure(error) {
        Ok(failure) => TransferExecutionError::host_failure(plan, origin, function, failure),
        Err(error) => error,
    }
}

macro_rules! resumable_value_function {
    (
        $run:ident,
        $evaluate:ident,
        $id:ty,
        $return:ty,
        $function:ident,
        $host_function:ident,
        $convert:expr
    ) => {
        pub(in crate::runtime) fn $run<'call, Profile>(
            plan: &'call AsyncHostedExecution<Profile>,
            state: &'call mut ResumableState<'_, Profile>,
            mut function: $id,
            mut origin: HostCallOrigin,
            mut inputs: ProfiledRetainedValues<TransferValues>,
        ) -> ResumableFuture<'call, $return>
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            Box::pin(async move {
                loop {
                    let exit = $evaluate(plan, state, function, origin, inputs).await?;
                    match exit {
                        EvaluatedFunctionExit::Return(value) => return Ok(value),
                        EvaluatedFunctionExit::TailCall {
                            function: target,
                            args,
                        } => {
                            origin = HostCallOrigin::source(target.site().clone());
                            function = *target.function();
                            inputs = args;
                        }
                    }
                }
            })
        }

        fn $evaluate<'call, Profile>(
            plan: &'call AsyncHostedExecution<Profile>,
            state: &'call mut ResumableState<'_, Profile>,
            function: $id,
            origin: HostCallOrigin,
            inputs: ProfiledRetainedValues<TransferValues>,
        ) -> ResumableFuture<
            'call,
            EvaluatedFunctionExit<$return, crate::plan::FunctionCallTarget<$id>>,
        >
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            Box::pin(async move {
                match plan.$function(function).as_ref() {
                    ExecutionFunctionRef::Graph(function) => {
                        let body = function.body().function_body();
                        let completed = crate::runtime::graph::execute_resumable(
                            plan,
                            state,
                            body.block_graph(),
                            inputs,
                        )
                        .await?;
                        match body.exit(completed.exit()) {
                            FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                                completed.into_value(state, value),
                            )),
                            FunctionExit::TailCall { function, args } => {
                                let function = function.clone();
                                let args = completed.into_retained(state, args.as_ref());
                                Ok(EvaluatedFunctionExit::TailCall { function, args })
                            }
                        }
                    }
                    ExecutionFunctionRef::Host(target) => match target {
                        crate::plan::execution::host::ResumableHostedFunctionTarget::Value(
                            target,
                        ) => {
                            let function = plan.$host_function(target);
                            match function.implementation() {
                                crate::plan::execution::host::ResumableHostCallback::Immediate(
                                    callback,
                                ) => {
                                    let result = callback.call(state.host(), &inputs);
                                    drop(inputs);
                                    result
                                        .map($convert)
                                        .map(EvaluatedFunctionExit::Return)
                                        .map_err(|failure| {
                                            TransferExecutionError::host_failure(
                                                plan,
                                                origin,
                                                function.metadata(),
                                                failure,
                                            )
                                        })
                                }
                                crate::plan::execution::host::ResumableHostCallback::AsyncOwned(
                                    callback,
                                ) => {
                                    let future = callback(&inputs);
                                    drop(inputs);
                                    future
                                        .await
                                        .map($convert)
                                        .map(EvaluatedFunctionExit::Return)
                                        .map_err(|error| {
                                            async_host_error(
                                                plan,
                                                origin,
                                                function.metadata(),
                                                error,
                                            )
                                        })
                                }
                                crate::plan::execution::host::ResumableHostCallback::AsyncScoped(
                                    callback,
                                ) => {
                                    let port = crate::host::AsyncHostRequestPort::new();
                                    let scope = ();
                                    let context = crate::host::AsyncHostRequestContext::new(
                                        std::sync::Arc::clone(&port),
                                        HostCallOrigin::host(function.metadata()),
                                        &scope,
                                        state.lists.clone(),
                                    );
                                    let future = callback(context, &inputs);
                                    drop(inputs);
                                    crate::runtime::resumable::drive_async_host(
                                        future,
                                        port,
                                        plan,
                                        state,
                                    )
                                    .await
                                    .map($convert)
                                    .map(EvaluatedFunctionExit::Return)
                                    .map_err(|error| {
                                        async_host_error(
                                            plan,
                                            origin,
                                            function.metadata(),
                                            error,
                                        )
                                    })
                                }
                            }
                        }
                        crate::plan::execution::host::ResumableHostedFunctionTarget::Never(
                            target,
                        ) => invoke_immediate_never(plan, state, origin, *target, inputs)
                            .map(|never| match never {}),
                    },
                }
            })
        }
    };
}

resumable_value_function!(
    run_int,
    evaluate_int_entry,
    crate::plan::execution::function::IntFunctionId,
    num_bigint::BigInt,
    int_function,
    host_int_function,
    std::convert::identity
);

pub(in crate::runtime) fn run_custom<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    mut function: crate::plan::execution::function::CustomFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, crate::runtime::EvaluatedCustomValue<TransferValues>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        loop {
            let exit = evaluate_custom_entry(plan, state, function, origin, inputs).await?;
            match exit {
                EvaluatedFunctionExit::Return(value) => return Ok(value),
                EvaluatedFunctionExit::TailCall {
                    function: target,
                    args,
                } => {
                    origin = HostCallOrigin::source(target.site().clone());
                    function = function.with_index(*target.function());
                    inputs = args;
                }
            }
        }
    })
}

pub(in crate::runtime) fn run_external<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    mut function: crate::plan::execution::function::ExternalFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, crate::runtime::EvaluatedExternalValue<TransferValues>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        loop {
            let exit = evaluate_external_entry(plan, state, function, origin, inputs).await?;
            match exit {
                EvaluatedFunctionExit::Return(value) => return Ok(value),
                EvaluatedFunctionExit::TailCall {
                    function: target,
                    args,
                } => {
                    origin = HostCallOrigin::source(target.site().clone());
                    function = function.with_index(*target.function());
                    inputs = args;
                }
            }
        }
    })
}

fn evaluate_external_entry<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::plan::execution::function::ExternalFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<
    'call,
    EvaluatedFunctionExit<
        crate::runtime::EvaluatedExternalValue<TransferValues>,
        crate::plan::FunctionCallTarget<usize>,
    >,
>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        match plan.external_function(function).as_ref() {
            ExecutionFunctionRef::Graph(function) => {
                let body = function.body().function_body();
                let completed = crate::runtime::graph::execute_resumable(
                    plan,
                    state,
                    body.block_graph(),
                    inputs,
                )
                .await?;
                match body.exit(completed.exit()) {
                    FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                        completed.into_value(state, value),
                    )),
                    FunctionExit::TailCall { function, args } => {
                        let function = function.clone();
                        let args = completed.into_retained(state, args.as_ref());
                        Ok(EvaluatedFunctionExit::TailCall { function, args })
                    }
                }
            }
            ExecutionFunctionRef::Host(target) => match target {
                crate::plan::execution::host::ResumableHostedFunctionTarget::Value(target) => {
                    let function = plan.host_external_function(target);
                    let port = crate::host::AsyncHostRequestPort::new();
                    let scope = ();
                    let context = crate::host::AsyncHostRequestContext::new(
                        std::sync::Arc::clone(&port),
                        HostCallOrigin::host(function.metadata()),
                        &scope,
                        state.lists.clone(),
                    );
                    let future = function.implementation()(context, &inputs);
                    drop(inputs);
                    crate::runtime::resumable::drive_async_host(future, port, plan, state)
                        .await
                        .map(|lease| {
                            EvaluatedFunctionExit::Return(
                                crate::runtime::EvaluatedExternalValue::new(
                                    target.return_().type_id(),
                                    lease,
                                ),
                            )
                        })
                        .map_err(|error| async_host_error(plan, origin, function.metadata(), error))
                }
                crate::plan::execution::host::ResumableHostedFunctionTarget::Never(target) => {
                    invoke_immediate_never(plan, state, origin, *target, inputs)
                        .map(|never| match never {})
                }
            },
        }
    })
}

fn evaluate_custom_entry<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::plan::execution::function::CustomFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<
    'call,
    EvaluatedFunctionExit<
        crate::runtime::EvaluatedCustomValue<TransferValues>,
        crate::plan::FunctionCallTarget<usize>,
    >,
>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        match plan.custom_function(function).as_ref() {
            ExecutionFunctionRef::Graph(function) => {
                let body = function.body().function_body();
                let completed = crate::runtime::graph::execute_resumable(
                    plan,
                    state,
                    body.block_graph(),
                    inputs,
                )
                .await?;
                match body.exit(completed.exit()) {
                    FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                        completed.into_value(state, value),
                    )),
                    FunctionExit::TailCall { function, args } => {
                        let function = function.clone();
                        let args = completed.into_retained(state, args.as_ref());
                        Ok(EvaluatedFunctionExit::TailCall { function, args })
                    }
                }
            }
            ExecutionFunctionRef::Host(target) => match target {
                crate::plan::execution::host::ResumableHostedFunctionTarget::Value(target) => {
                    match *target {}
                }
                crate::plan::execution::host::ResumableHostedFunctionTarget::Never(target) => {
                    invoke_immediate_never(plan, state, origin, *target, inputs)
                        .map(|never| match never {})
                }
            },
        }
    })
}

pub(in crate::runtime) fn run_tuple<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    mut function: crate::plan::execution::function::TupleFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, Vec<crate::runtime::EvaluatedValue<TransferValues>>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        loop {
            let exit = evaluate_tuple_entry(plan, state, function, origin, inputs).await?;
            match exit {
                EvaluatedFunctionExit::Return(value) => return Ok(value),
                EvaluatedFunctionExit::TailCall {
                    function: target,
                    args,
                } => {
                    origin = HostCallOrigin::source(target.site().clone());
                    function = *target.function();
                    inputs = args;
                }
            }
        }
    })
}

fn evaluate_tuple_entry<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::plan::execution::function::TupleFunctionId,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<
    'call,
    EvaluatedFunctionExit<
        Vec<crate::runtime::EvaluatedValue<TransferValues>>,
        crate::plan::FunctionCallTarget<crate::plan::execution::function::TupleFunctionId>,
    >,
>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        match plan.tuple_function(function).as_ref() {
            ExecutionFunctionRef::Graph(function) => {
                let body = function.body().function_body();
                let completed = crate::runtime::graph::execute_resumable(
                    plan,
                    state,
                    body.block_graph(),
                    inputs,
                )
                .await?;
                match body.exit(completed.exit()) {
                    FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                        completed.into_value(state, value),
                    )),
                    FunctionExit::TailCall { function, args } => {
                        let function = function.clone();
                        let args = completed.into_retained(state, args.as_ref());
                        Ok(EvaluatedFunctionExit::TailCall { function, args })
                    }
                }
            }
            ExecutionFunctionRef::Host(target) => match target {
                crate::plan::execution::host::ResumableHostedFunctionTarget::Value(target) => {
                    match *target {}
                }
                crate::plan::execution::host::ResumableHostedFunctionTarget::Never(target) => {
                    invoke_immediate_never(plan, state, origin, *target, inputs)
                        .map(|never| match never {})
                }
            },
        }
    })
}
resumable_value_function!(
    run_float,
    evaluate_float_entry,
    crate::plan::execution::function::FloatFunctionId,
    f64,
    float_function,
    host_float_function,
    std::convert::identity
);
resumable_value_function!(
    run_string,
    evaluate_string_entry,
    crate::plan::execution::function::StringFunctionId,
    ecow::EcoString,
    string_function,
    host_string_function,
    std::convert::identity
);
resumable_value_function!(
    run_bit_array,
    evaluate_bit_array_entry,
    crate::plan::execution::function::BitArrayFunctionId,
    crate::runtime::EvaluatedBitArray,
    bit_array_function,
    host_bit_array_function,
    crate::runtime::EvaluatedBitArray::from_value
);
resumable_value_function!(
    run_utf_codepoint,
    evaluate_utf_codepoint_entry,
    crate::plan::execution::function::UtfCodepointFunctionId,
    char,
    utf_codepoint_function,
    host_utf_codepoint_function,
    std::convert::identity
);
resumable_value_function!(
    run_bool,
    evaluate_bool_entry,
    crate::plan::execution::function::BoolFunctionId,
    bool,
    bool_function,
    host_bool_function,
    std::convert::identity
);
resumable_value_function!(
    run_nil,
    evaluate_nil_entry,
    crate::plan::execution::function::NilFunctionId,
    (),
    nil_function,
    host_nil_function,
    std::convert::identity
);

pub(in crate::runtime) fn run_never<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    mut function: crate::plan::execution::function::NeverFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, std::convert::Infallible>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        loop {
            match plan.never_function(function).as_ref() {
                ExecutionFunctionRef::Graph(entry) => {
                    let body = entry.body().function_body();
                    let completed = crate::runtime::graph::execute_resumable(
                        plan,
                        state,
                        body.block_graph(),
                        inputs,
                    )
                    .await?;
                    match body.exit(completed.exit()) {
                        FunctionExit::Return(value) => match *value {},
                        FunctionExit::TailCall {
                            function: target,
                            args,
                        } => {
                            origin = HostCallOrigin::source(target.site().clone());
                            function = *target.function();
                            inputs = completed.into_retained(state, args.as_ref());
                        }
                    }
                }
                ExecutionFunctionRef::Host(target) => {
                    return invoke_immediate_never(plan, state, origin, *target, inputs);
                }
            }
        }
    })
}

pub(in crate::runtime) fn run_never_value<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::runtime::EvaluatedNeverFunction<TransferValues>,
    origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, std::convert::Infallible>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), origin, inputs)
}

macro_rules! resumable_list_function {
    ($run:ident, $evaluate:ident, $id:ty, $return:ty, $function:ident) => {
        pub(in crate::runtime) fn $run<'call, Profile>(
            plan: &'call AsyncHostedExecution<Profile>,
            state: &'call mut ResumableState<'_, Profile>,
            mut function: $id,
            mut origin: HostCallOrigin,
            mut inputs: ProfiledRetainedValues<TransferValues>,
        ) -> ResumableFuture<'call, $return>
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            Box::pin(async move {
                loop {
                    let exit = $evaluate(plan, state, function, origin, inputs).await?;
                    match exit {
                        EvaluatedFunctionExit::Return(value) => return Ok(value),
                        EvaluatedFunctionExit::TailCall {
                            function: target,
                            args,
                        } => {
                            origin = HostCallOrigin::source(target.site().clone());
                            function = *target.function();
                            inputs = args;
                        }
                    }
                }
            })
        }

        fn $evaluate<'call, Profile>(
            plan: &'call AsyncHostedExecution<Profile>,
            state: &'call mut ResumableState<'_, Profile>,
            function: $id,
            origin: HostCallOrigin,
            inputs: ProfiledRetainedValues<TransferValues>,
        ) -> ResumableFuture<
            'call,
            EvaluatedFunctionExit<$return, crate::plan::FunctionCallTarget<$id>>,
        >
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            Box::pin(async move {
                match plan.$function(function).as_ref() {
                    ExecutionFunctionRef::Graph(function) => {
                        let body = function.body().function_body();
                        let completed = crate::runtime::graph::execute_resumable(
                            plan,
                            state,
                            body.block_graph(),
                            inputs,
                        )
                        .await?;
                        match body.exit(completed.exit()) {
                            FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                                completed.into_value(state, value),
                            )),
                            FunctionExit::TailCall { function, args } => {
                                let function = function.clone();
                                let args = completed.into_retained(state, args.as_ref());
                                Ok(EvaluatedFunctionExit::TailCall { function, args })
                            }
                        }
                    }
                    ExecutionFunctionRef::Host(target) => match target {
                        crate::plan::execution::host::ResumableHostedFunctionTarget::Value(
                            target,
                        ) => match *target {},
                        crate::plan::execution::host::ResumableHostedFunctionTarget::Never(
                            target,
                        ) => invoke_immediate_never(plan, state, origin, *target, inputs)
                            .map(|never| match never {}),
                    },
                }
            })
        }
    };
}

resumable_list_function!(
    run_parameter_list,
    evaluate_parameter_list_entry,
    crate::plan::execution::function::ParameterListFunctionId,
    crate::runtime::state::list::ParameterListValueId<TransferValues>,
    parameter_list_function
);
resumable_list_function!(
    run_parameter_list_list,
    evaluate_parameter_list_list_entry,
    crate::plan::execution::function::ParameterListListFunctionId,
    crate::runtime::state::list::ParameterListListValueId<TransferValues>,
    parameter_list_list_function
);
resumable_list_function!(
    run_int_list,
    evaluate_int_list_entry,
    crate::plan::execution::function::IntListFunctionId,
    crate::runtime::state::list::IntListValueId<TransferValues>,
    int_list_function
);
resumable_list_function!(
    run_string_list,
    evaluate_string_list_entry,
    crate::plan::execution::function::StringListFunctionId,
    crate::runtime::state::list::StringListValueId<TransferValues>,
    string_list_function
);
resumable_list_function!(
    run_bit_array_list,
    evaluate_bit_array_list_entry,
    crate::plan::execution::function::BitArrayListFunctionId,
    crate::runtime::state::list::BitArrayListValueId<TransferValues>,
    bit_array_list_function
);
resumable_list_function!(
    run_utf_codepoint_list,
    evaluate_utf_codepoint_list_entry,
    crate::plan::execution::function::UtfCodepointListFunctionId,
    crate::runtime::state::list::UtfCodepointListValueId<TransferValues>,
    utf_codepoint_list_function
);
resumable_list_function!(
    run_custom_list,
    evaluate_custom_list_entry,
    crate::plan::execution::function::CustomListFunctionId,
    crate::runtime::state::list::CustomListValueId<TransferValues>,
    custom_list_function
);
resumable_list_function!(
    run_external_list,
    evaluate_external_list_entry,
    crate::plan::execution::function::ExternalListFunctionId,
    crate::runtime::state::list::ExternalListValueId<TransferValues>,
    external_list_function
);
resumable_list_function!(
    run_float_list,
    evaluate_float_list_entry,
    crate::plan::execution::function::FloatListFunctionId,
    crate::runtime::state::list::FloatListValueId<TransferValues>,
    float_list_function
);
resumable_list_function!(
    run_bool_list,
    evaluate_bool_list_entry,
    crate::plan::execution::function::BoolListFunctionId,
    crate::runtime::state::list::BoolListValueId<TransferValues>,
    bool_list_function
);
resumable_list_function!(
    run_nil_list,
    evaluate_nil_list_entry,
    crate::plan::execution::function::NilListFunctionId,
    crate::runtime::state::list::NilListValueId<TransferValues>,
    nil_list_function
);
resumable_list_function!(
    run_tuple_list,
    evaluate_tuple_list_entry,
    crate::plan::execution::function::TupleListFunctionId,
    crate::runtime::state::list::TupleListValueId<TransferValues>,
    tuple_list_function
);
resumable_list_function!(
    run_list_list,
    evaluate_list_list_entry,
    crate::plan::execution::function::ListListFunctionId,
    crate::runtime::state::list::ListListValueId<TransferValues>,
    list_list_function
);
resumable_list_function!(
    run_function_list,
    evaluate_function_list_entry,
    crate::plan::execution::function::FunctionListFunctionId,
    crate::runtime::state::list::FunctionListValueId<TransferValues>,
    function_list_function
);

type EvaluatedGraphFunctionExit<Body> = EvaluatedFunctionExit<
    <<Body as FunctionBodyOwner>::Return as GraphValue<TransferValues>>::Evaluated,
    <Body as FunctionBodyOwner>::TailCall,
>;

fn evaluate_graph_function_entry<'call, Profile, Body>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: &'call crate::plan::execution::function::ExecutionFunction<
        crate::plan::execution::host::AsyncHostedExecutionProfile,
        Body,
    >,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, EvaluatedGraphFunctionExit<Body>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
    Body: crate::plan::execution::function::ExecutionFunctionBody<
            Graph = crate::plan::execution::function::HostedExecutionGraph,
            AsyncHostTarget = std::convert::Infallible,
        > + Sync
        + 'call,
    Body::Return: GraphValue<TransferValues> + Sync,
    <Body::Return as GraphValue<TransferValues>>::Evaluated: Send,
    Body::TailCall: Clone + Send + Sync,
{
    Box::pin(async move {
        match function.as_ref() {
            ExecutionFunctionRef::Graph(function) => {
                let body = function.body().function_body();
                let completed = crate::runtime::graph::execute_resumable(
                    plan,
                    state,
                    body.block_graph(),
                    inputs,
                )
                .await?;
                match body.exit(completed.exit()) {
                    FunctionExit::Return(value) => Ok(EvaluatedFunctionExit::Return(
                        completed.into_value(state, value),
                    )),
                    FunctionExit::TailCall { function, args } => {
                        let function = function.clone();
                        let args = completed.into_retained(state, args.as_ref());
                        Ok(EvaluatedFunctionExit::TailCall { function, args })
                    }
                }
            }
            ExecutionFunctionRef::Host(target) => match target {
                crate::plan::execution::host::ResumableHostedFunctionTarget::Value(target) => {
                    match *target {}
                }
                crate::plan::execution::host::ResumableHostedFunctionTarget::Never(target) => {
                    invoke_immediate_never(plan, state, origin, *target, inputs)
                        .map(|never| match never {})
                }
            },
        }
    })
}

macro_rules! returning_function_entry {
    ($plan:expr, $method:ident, $function:expr, ref) => {
        $plan.$method(&$function)
    };
    ($plan:expr, $method:ident, $function:expr, copy) => {
        $plan.$method($function)
    };
}

macro_rules! returning_function_next {
    ($function:expr, $target:expr, clone) => {
        $target.function().clone()
    };
    ($function:expr, $target:expr, copy) => {
        *$target.function()
    };
    ($function:expr, $target:expr, with_index) => {
        $function.with_index(*$target.function())
    };
}

macro_rules! resumable_returning_function {
    (
        $run:ident,
        $id:ty,
        $return:ty,
        $function:ident,
        $lookup_mode:ident,
        $next_mode:ident
    ) => {
        pub(in crate::runtime) fn $run<'call, Profile>(
            plan: &'call AsyncHostedExecution<Profile>,
            state: &'call mut ResumableState<'_, Profile>,
            mut function: $id,
            mut origin: HostCallOrigin,
            mut inputs: ProfiledRetainedValues<TransferValues>,
        ) -> ResumableFuture<'call, $return>
        where
            Profile: crate::HostProfile,
            Profile::RunState: Send,
            Profile::ExternalStores: Send,
        {
            Box::pin(async move {
                loop {
                    let exit = evaluate_graph_function_entry(
                        plan,
                        state,
                        returning_function_entry!(plan, $function, function, $lookup_mode),
                        origin,
                        inputs,
                    )
                    .await?;
                    match exit {
                        EvaluatedFunctionExit::Return(value) => return Ok(value),
                        EvaluatedFunctionExit::TailCall {
                            function: target,
                            args,
                        } => {
                            origin = HostCallOrigin::source(target.site().clone());
                            function = returning_function_next!(function, target, $next_mode);
                            inputs = args;
                        }
                    }
                }
            })
        }
    };
}

resumable_returning_function!(
    run_generic_function,
    crate::plan::execution::function::GenericFunctionFunctionId,
    crate::runtime::EvaluatedGenericFunction<TransferValues>,
    generic_function_function,
    ref,
    clone
);
resumable_returning_function!(
    run_never_function,
    crate::plan::execution::function::NeverFunctionFunctionId,
    crate::runtime::EvaluatedNeverFunction<TransferValues>,
    never_function_function,
    ref,
    clone
);
resumable_returning_function!(
    run_int_function,
    crate::plan::execution::function::IntFunctionFunctionId,
    crate::runtime::EvaluatedIntFunction<TransferValues>,
    int_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_float_function,
    crate::plan::execution::function::FloatFunctionFunctionId,
    crate::runtime::EvaluatedFloatFunction<TransferValues>,
    float_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_string_function,
    crate::plan::execution::function::StringFunctionFunctionId,
    crate::runtime::EvaluatedStringFunction<TransferValues>,
    string_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_bit_array_function,
    crate::plan::execution::function::BitArrayFunctionFunctionId,
    crate::runtime::EvaluatedBitArrayFunction<TransferValues>,
    bit_array_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_utf_codepoint_function,
    crate::plan::execution::function::UtfCodepointFunctionFunctionId,
    crate::runtime::EvaluatedUtfCodepointFunction<TransferValues>,
    utf_codepoint_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_custom_function,
    crate::plan::execution::function::CustomFunctionFunctionId,
    crate::runtime::EvaluatedCustomFunction<TransferValues>,
    custom_function_function,
    ref,
    with_index
);
resumable_returning_function!(
    run_bool_function,
    crate::plan::execution::function::BoolFunctionFunctionId,
    crate::runtime::EvaluatedBoolFunction<TransferValues>,
    bool_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_nil_function,
    crate::plan::execution::function::NilFunctionFunctionId,
    crate::runtime::EvaluatedNilFunction<TransferValues>,
    nil_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_tuple_function,
    crate::plan::execution::function::TupleFunctionFunctionId,
    crate::runtime::EvaluatedTupleFunction<TransferValues>,
    tuple_function_function,
    copy,
    copy
);
resumable_returning_function!(
    run_core_list_function,
    crate::plan::execution::function::ProfiledListFunctionFunctionId<std::convert::Infallible>,
    crate::runtime::EvaluatedListFunction<TransferValues>,
    core_list_function_function,
    ref,
    clone
);
resumable_returning_function!(
    run_function_function,
    crate::plan::execution::function::FunctionFunctionFunctionId,
    crate::runtime::EvaluatedFunctionFunction<TransferValues>,
    function_function_function,
    ref,
    with_index
);

pub(in crate::runtime) fn run_core_function<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::plan::execution::function::ProfiledFunctionFunctionId<
        std::convert::Infallible,
    >,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, crate::runtime::evaluated::EvaluatedFunctionValue<TransferValues>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    use crate::plan::execution::function::ProfiledFunctionFunctionId as F;

    Box::pin(async move {
        match function {
            F::Generic(function) => run_generic_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Never(function) => run_never_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Int(function) => run_int_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Float(function) => run_float_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::String(function) => run_string_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::BitArray(function) => run_bit_array_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::UtfCodepoint(function) => {
                run_utf_codepoint_function(plan, state, function, origin, inputs)
                    .await
                    .map(Into::into)
            }
            F::Custom(function) => run_custom_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::External(function) => match function {},
            F::Bool(function) => run_bool_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Nil(function) => run_nil_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Tuple(function) => run_tuple_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::List(function) => run_core_list_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
            F::Function(function) => run_function_function(plan, state, function, origin, inputs)
                .await
                .map(Into::into),
        }
    })
}

resumable_returning_function!(
    run_external_function,
    crate::plan::execution::function::ExternalFunctionFunctionId,
    crate::runtime::EvaluatedExternalFunction<TransferValues>,
    external_function_function,
    ref,
    with_index
);
resumable_returning_function!(
    run_external_list_function,
    crate::plan::execution::function::ExternalListFunctionFunctionId,
    crate::runtime::EvaluatedListFunction<TransferValues>,
    external_list_function_function,
    copy,
    copy
);

pub(in crate::runtime) fn run_external_function_function<'call, Profile>(
    plan: &'call AsyncHostedExecution<Profile>,
    state: &'call mut ResumableState<'_, Profile>,
    function: crate::plan::execution::graph::ExternalFunctionCallTarget,
    origin: HostCallOrigin,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> ResumableFuture<'call, crate::runtime::evaluated::EvaluatedFunctionValue<TransferValues>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    Box::pin(async move {
        match function {
            crate::plan::execution::graph::ExternalFunctionCallTarget::Function(function) => {
                run_external_function(plan, state, function, origin, inputs)
                    .await
                    .map(Into::into)
            }
            crate::plan::execution::graph::ExternalFunctionCallTarget::ListFunction {
                id, ..
            } => run_external_list_function(plan, state, id, origin, inputs)
                .await
                .map(Into::into),
        }
    })
}
