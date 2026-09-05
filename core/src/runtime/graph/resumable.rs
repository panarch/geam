use super::CompletedGraph;
use super::environment::{BlockEnvironment, ProfiledRetainedValues};
use super::instruction::{
    CoreFunctionInstructionValue, ExternalFunctionInstructionValue, ExternalListInstructionValue,
    InstructionValue, InstructionValueWithoutConstant, ListInstructionValue, evaluate_bit_array,
    evaluate_bool, evaluate_custom, evaluate_external, evaluate_external_function,
    evaluate_external_list, evaluate_float, evaluate_function, evaluate_int, evaluate_list,
    evaluate_nil, evaluate_string, evaluate_tuple, evaluate_utf_codepoint, validate_return_family,
};
use super::terminator::{GraphAction, NeverCall, terminator_action};
use crate::plan::execution::AsyncHostedExecution;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
    FunctionReturnFamily, IntFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, CustomLocal, ExternalFunctionInstructionView, FloatLocalId,
    IntLocalId, NilLocalId, ProfiledBlockGraph, ProfiledInstructionKind, StringLocalId,
    TupleLocalId,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::plan::execution::type_::FunctionType;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedValue,
};
use crate::runtime::resumable::{
    ResumableState, TransferExecutionResult, run_bit_array, run_bit_array_list, run_bool,
    run_bool_list, run_core_function, run_custom, run_custom_list, run_external,
    run_external_function_function, run_external_list, run_float, run_float_list,
    run_function_list, run_int, run_int_list, run_list_list, run_never, run_never_value, run_nil,
    run_nil_list, run_parameter_list, run_parameter_list_list, run_string, run_string_list,
    run_tuple, run_tuple_list, run_utf_codepoint, run_utf_codepoint_list,
};
use crate::runtime::{RuntimeListStorage, TransferValues};
use ecow::EcoString;
use num_bigint::BigInt;

type IntInstructionValue = InstructionValue<TransferValues, BigInt, IntFunctionId, IntLocalId>;
type FloatInstructionValue = InstructionValue<TransferValues, f64, FloatFunctionId, FloatLocalId>;
type StringInstructionValue =
    InstructionValue<TransferValues, EcoString, StringFunctionId, StringLocalId>;
type BitArrayInstructionValue =
    InstructionValue<TransferValues, EvaluatedBitArray, BitArrayFunctionId, BitArrayLocalId>;
type UtfCodepointInstructionValue =
    InstructionValueWithoutConstant<TransferValues, char, UtfCodepointFunctionId>;
type CustomInstructionValue = InstructionValue<
    TransferValues,
    EvaluatedCustomValue<TransferValues>,
    CustomFunctionId,
    CustomLocal,
>;
type ExternalInstructionValue = InstructionValueWithoutConstant<
    TransferValues,
    EvaluatedExternalValue<TransferValues>,
    ExternalFunctionId,
>;
type BoolInstructionValue = InstructionValue<TransferValues, bool, BoolFunctionId, BoolLocalId>;
type NilInstructionValue = InstructionValue<TransferValues, (), NilFunctionId, NilLocalId>;
type TupleInstructionValue = InstructionValue<
    TransferValues,
    Vec<EvaluatedValue<TransferValues>>,
    TupleFunctionId,
    TupleLocalId,
>;

enum PreparedInstruction {
    Int(IntInstructionValue),
    Float(FloatInstructionValue),
    String(StringInstructionValue),
    BitArray(BitArrayInstructionValue),
    UtfCodepoint(UtfCodepointInstructionValue),
    Custom(CustomInstructionValue),
    External(ExternalInstructionValue),
    List(ListInstructionValue<TransferValues>),
    ExternalList(ExternalListInstructionValue<TransferValues>),
    ExternalFunction {
        value: ExternalFunctionInstructionValue<TransferValues>,
        family: FunctionReturnFamily,
        type_: FunctionType,
    },
    Bool(BoolInstructionValue),
    Nil(NilInstructionValue),
    Tuple(TupleInstructionValue),
    Function {
        value: CoreFunctionInstructionValue<TransferValues>,
        family: FunctionReturnFamily,
        type_: FunctionType,
    },
}

pub(in crate::runtime) async fn execute<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    graph: &ProfiledBlockGraph<crate::plan::execution::function::HostedExecutionGraph>,
    inputs: ProfiledRetainedValues<TransferValues>,
) -> TransferExecutionResult<CompletedGraph<TransferValues>>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let mut block_id = graph.entry();
    let mut environment = BlockEnvironment::from_retained(inputs);

    loop {
        let block = graph.block(block_id);
        for instruction in block.instructions() {
            execute_instruction(plan, state, &mut environment, instruction).await?;
        }

        let action = match terminator_action(plan, state, &environment, block.terminator()) {
            Ok(action) => action,
            Err(error) => return Err(error),
        };
        match action {
            GraphAction::Continue { block, inputs } => {
                drop(environment);
                state.lists_mut().drain_releases();
                block_id = block;
                environment = BlockEnvironment::from_retained(inputs);
            }
            GraphAction::Exit(exit) => return Ok(CompletedGraph::new(exit, environment)),
            GraphAction::NeverCall {
                function,
                inputs,
                site,
            } => {
                drop(environment);
                state.lists_mut().drain_releases();
                let origin = crate::runtime::error::HostCallOrigin::source(site);
                return match function {
                    NeverCall::Direct(function) => run_never(plan, state, function, origin, inputs)
                        .await
                        .map(|never| match never {}),
                    NeverCall::Value(function) => {
                        run_never_value(plan, state, function, origin, inputs)
                            .await
                            .map(|never| match never {})
                    }
                };
            }
        }
    }
}

async fn execute_instruction<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    environment: &mut BlockEnvironment<TransferValues>,
    instruction: &crate::plan::execution::graph::ProfiledInstruction<
        crate::plan::execution::function::HostedExecutionGraph,
    >,
) -> TransferExecutionResult<()>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let prepared = prepare_instruction(plan, state, environment, instruction)?;
    execute_prepared_instruction(plan, state, environment, prepared).await
}

fn prepare_instruction<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    environment: &BlockEnvironment<TransferValues>,
    instruction: &crate::plan::execution::graph::ProfiledInstruction<
        crate::plan::execution::function::HostedExecutionGraph,
    >,
) -> TransferExecutionResult<PreparedInstruction>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    let expected = plan.value_type(&plan.shape_value_type(instruction.output().shape()));

    match instruction.kind() {
        ProfiledInstructionKind::Int(instruction) => {
            evaluate_int(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Int)
        }
        ProfiledInstructionKind::Float(instruction) => {
            evaluate_float(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Float)
        }
        ProfiledInstructionKind::String(instruction) => {
            evaluate_string(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::String)
        }
        ProfiledInstructionKind::BitArray(instruction) => {
            evaluate_bit_array(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::BitArray)
        }
        ProfiledInstructionKind::UtfCodepoint(instruction) => {
            evaluate_utf_codepoint(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::UtfCodepoint)
        }
        ProfiledInstructionKind::Custom(instruction) => {
            evaluate_custom(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Custom)
        }
        ProfiledInstructionKind::External(instruction) => {
            evaluate_external(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::External)
        }
        ProfiledInstructionKind::ExternalList(instruction) => {
            evaluate_external_list(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::ExternalList)
        }
        ProfiledInstructionKind::ExternalFunction(instruction) => {
            let instruction = instruction.instruction();
            Ok({
                let value = evaluate_external_function(plan, environment, instruction);
                PreparedInstruction::ExternalFunction {
                    value,
                    family: instruction.family(),
                    type_: instruction.type_().clone(),
                }
            })
        }
        ProfiledInstructionKind::Bool(instruction) => {
            evaluate_bool(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Bool)
        }
        ProfiledInstructionKind::Nil(instruction) => {
            evaluate_nil(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Nil)
        }
        ProfiledInstructionKind::Tuple(instruction) => {
            evaluate_tuple(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::Tuple)
        }
        ProfiledInstructionKind::List(instruction) => {
            evaluate_list(plan, state, environment, instruction, &expected)
                .map(PreparedInstruction::List)
        }
        ProfiledInstructionKind::Function(instruction) => {
            evaluate_function(plan, state, environment, instruction, &expected).map(|value| {
                PreparedInstruction::Function {
                    value,
                    family: instruction.family(),
                    type_: instruction.type_().clone(),
                }
            })
        }
    }
}

async fn execute_prepared_instruction<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    environment: &mut BlockEnvironment<TransferValues>,
    instruction: PreparedInstruction,
) -> TransferExecutionResult<()>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    macro_rules! resolve_value {
        ($value:expr, $run:ident) => {{
            match $value {
                InstructionValue::Ready(value) => value,
                InstructionValue::Constant(id) => {
                    crate::runtime::constant::evaluate_resumable(plan, state, plan.constant(id))
                        .await?
                }
                InstructionValue::Call {
                    function,
                    origin,
                    inputs,
                } => $run(plan, state, function, origin, inputs).await?,
            }
        }};
    }

    macro_rules! resolve_value_without_constant {
        ($value:expr, $run:ident) => {{
            match $value {
                InstructionValueWithoutConstant::Ready(value) => value,
                InstructionValueWithoutConstant::Call {
                    function,
                    origin,
                    inputs,
                } => $run(plan, state, function, origin, inputs).await?,
            }
        }};
    }

    match instruction {
        PreparedInstruction::Int(value) => {
            environment.push_int(resolve_value!(value, run_int));
        }
        PreparedInstruction::Float(value) => {
            environment.push_float(resolve_value!(value, run_float));
        }
        PreparedInstruction::String(value) => {
            environment.push_string(resolve_value!(value, run_string));
        }
        PreparedInstruction::BitArray(value) => {
            environment.push_bit_array(resolve_value!(value, run_bit_array));
        }
        PreparedInstruction::UtfCodepoint(value) => {
            environment
                .push_utf_codepoint(resolve_value_without_constant!(value, run_utf_codepoint));
        }
        PreparedInstruction::Custom(value) => {
            environment.push_custom(resolve_value!(value, run_custom));
        }
        PreparedInstruction::External(value) => {
            environment.push_external(resolve_value_without_constant!(value, run_external));
        }
        PreparedInstruction::List(value) => {
            return execute_list_value(plan, state, environment, value).await;
        }
        PreparedInstruction::ExternalList(value) => {
            environment.push_external_list(resolve_value!(value, run_external_list));
        }
        PreparedInstruction::ExternalFunction {
            value,
            family,
            type_,
        } => {
            let value = resolve_value_without_constant!(value, run_external_function_function);
            environment.push_function_value(validate_return_family::<
                _,
                crate::runtime::TransferExecutionError,
            >(value, family, type_)?);
        }
        PreparedInstruction::Bool(value) => {
            environment.push_bool(resolve_value!(value, run_bool));
        }
        PreparedInstruction::Nil(value) => {
            let () = resolve_value!(value, run_nil);
            environment.push_nil();
        }
        PreparedInstruction::Tuple(value) => {
            environment.push_tuple(resolve_value!(value, run_tuple));
        }
        PreparedInstruction::Function {
            value,
            family,
            type_,
        } => {
            let value = resolve_value!(value, run_core_function);
            environment.push_function_value(validate_return_family::<
                _,
                crate::runtime::TransferExecutionError,
            >(value, family, type_)?);
        }
    }
    Ok(())
}

async fn execute_list_value<Profile>(
    plan: &AsyncHostedExecution<Profile>,
    state: &mut ResumableState<'_, Profile>,
    environment: &mut BlockEnvironment<TransferValues>,
    value: ListInstructionValue<TransferValues>,
) -> TransferExecutionResult<()>
where
    Profile: crate::HostProfile,
    Profile::RunState: Send,
    Profile::ExternalStores: Send,
{
    macro_rules! execute_value {
        ($value:expr, $run:ident, $push:ident) => {{
            let value = match $value {
                InstructionValue::Ready(value) => value,
                InstructionValue::Constant(id) => {
                    crate::runtime::constant::evaluate_resumable(plan, state, plan.constant(id))
                        .await?
                }
                InstructionValue::Call {
                    function,
                    origin,
                    inputs,
                } => $run(plan, state, function, origin, inputs).await?,
            };
            environment.$push(value);
            Ok(())
        }};
    }

    match value {
        ListInstructionValue::Parameter(value) => {
            execute_value!(value, run_parameter_list, push_parameter_list)
        }
        ListInstructionValue::ParameterList(value) => {
            execute_value!(value, run_parameter_list_list, push_parameter_list_list)
        }
        ListInstructionValue::Int(value) => execute_value!(value, run_int_list, push_int_list),
        ListInstructionValue::String(value) => {
            execute_value!(value, run_string_list, push_string_list)
        }
        ListInstructionValue::BitArray(value) => {
            execute_value!(value, run_bit_array_list, push_bit_array_list)
        }
        ListInstructionValue::UtfCodepoint(value) => {
            execute_value!(value, run_utf_codepoint_list, push_utf_codepoint_list)
        }
        ListInstructionValue::Custom(value) => {
            execute_value!(value, run_custom_list, push_custom_list)
        }
        ListInstructionValue::Float(value) => {
            execute_value!(value, run_float_list, push_float_list)
        }
        ListInstructionValue::Bool(value) => execute_value!(value, run_bool_list, push_bool_list),
        ListInstructionValue::Nil(value) => execute_value!(value, run_nil_list, push_nil_list),
        ListInstructionValue::Tuple(value) => {
            execute_value!(value, run_tuple_list, push_tuple_list)
        }
        ListInstructionValue::List(value) => execute_value!(value, run_list_list, push_list_list),
        ListInstructionValue::Function(value) => {
            execute_value!(value, run_function_list, push_function_list)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::host::{
        AsyncHostModule, AsyncHostProviderModule, AsyncHostProviderSet, HostExternalSchema,
    };
    use crate::plan::execution::AsyncHostedExecution;
    use crate::plan::execution::function::{
        ExternalListFunctionFunctionId, FloatFunctionFunctionId, FunctionReturnFamily,
        IntFunctionId, ProfiledFunctionFunctionId,
    };
    use crate::plan::execution::graph::ExternalFunctionCallTarget;
    use crate::plan::execution::type_::{
        ExternalListTypeId, ExternalTypeId, FunctionType, ListTypeId, ValueType,
    };
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::runtime::evaluated::{EvaluatedFunction, EvaluatedFunctionFunction};
    use crate::runtime::graph::ProfiledRetainedValues;
    use crate::runtime::resumable::{RecordedEcho, ResumableState, run_int};
    use crate::runtime::{EvaluatedFunctionValue, EvaluatedValue, InvariantError, TransferValues};
    use crate::{
        ExecutionError, ModuleSource, PackageSource, StatelessHostProfile,
        compile_typed_async_host_program,
    };
    use num_bigint::BigInt;
    use std::future::{Future, Ready, ready};
    use std::pin::{Pin, pin};
    use std::task::{Context, Poll, Waker};

    struct CounterSchema;

    impl HostExternalSchema for CounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    struct PendingOnce {
        value: Ready<BigInt>,
        pending_returned: bool,
    }

    impl Future for PendingOnce {
        type Output = BigInt;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if self.pending_returned {
                Pin::new(&mut self.value).poll(context)
            } else {
                self.pending_returned = true;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[test]
    fn resumable_function_calls_reject_corrupted_return_families() {
        let (execution, functions) = malformed_function_execution();

        assert_family_mismatch(
            &execution,
            functions[0],
            wrong_float_factory(),
            FunctionReturnFamily::Int,
            FunctionReturnFamily::Float,
        );
        assert_family_mismatch(
            &execution,
            functions[2],
            wrong_external_list_factory(),
            FunctionReturnFamily::External,
            FunctionReturnFamily::List,
        );
    }

    #[test]
    fn resumable_compound_projections_preserve_functions_and_lists_across_pending() {
        let (execution, functions) = malformed_function_execution();
        let mut host = ();
        let mut echo = RecordedEcho::default();
        let observed = std::sync::Arc::clone(&echo.0);
        let mut stores = ();
        let mut state =
            ResumableState::<StatelessHostProfile>::new(&mut host, &mut stores, &mut echo);
        let (result, pending) = poll_to_ready(run_int(
            &execution,
            &mut state,
            functions[4],
            crate::runtime::error::HostCallOrigin::Entry,
            ProfiledRetainedValues::empty(),
        ));
        drop(state);

        assert_eq!(
            result.expect("compound projections should resume"),
            BigInt::from(15),
        );
        assert_eq!(pending, 1);
        assert_eq!(observed.lock().expect("echo observation lock").len(), 1);
    }

    fn malformed_function_execution() -> (
        AsyncHostedExecution<StatelessHostProfile>,
        [IntFunctionId; 5],
    ) {
        let provider = AsyncHostProviderModule::new("application", "library")
            .expect("async provider module")
            .with_external_type_for_test::<CounterSchema>()
            .with_async_function("suspend", |value: BigInt| PendingOnce {
                value: ready(value),
                pending_returned: false,
            })
            .expect("async suspend function");
        let hosts = AsyncHostProviderSet::with_providers(Vec::<AsyncHostModule>::new(), [provider])
            .expect("async host set");
        let program = compile_typed_async_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "Counter")
pub type Counter

@external(erlang, "native", "suspend")
fn suspend(value: Int) -> Int

fn float_identity(value: Float) -> Float { value }
fn float_factory() -> fn(Float) -> Float { float_identity }
fn external_list_factory() -> fn() -> List(Counter) { fn() { [] } }
fn increment(value: Int) -> Int { value + 1 }

pub type FunctionBox { FunctionBox(callback: fn(Int) -> Int) }
pub type ListBox { ListBox(values: List(Int)) }

pub fn call_int_factory(factory: fn() -> fn(Int) -> Int) -> Int {
  echo 0 as "before"
  let _ = suspend(0)
  factory()(1)
}

pub fn expose_float_factory() -> Int {
  let factory = float_factory
  let _ = factory()(1.0)
  0
}

pub fn call_external_factory(factory: fn() -> fn() -> Counter) -> Int {
  echo 0 as "before"
  let _ = suspend(0)
  let _ = factory()
  0
}

pub fn expose_external_list_factory() -> Int {
  let factory = external_list_factory
  let _ = factory()()
  0
}

pub fn resume_compound_projections() -> Int {
  let tuple_function = #(increment).0
  let FunctionBox(field_function) = FunctionBox(increment)
  let tuple_list = #([4, 5]).0
  let ListBox(field_list) = ListBox([6, 7])
  echo 0 as "before"
  let _ = suspend(0)
  let assert [tuple_head, ..] = tuple_list
  let assert [field_head, ..] = field_list
  tuple_function(1) + field_function(2) + tuple_head + field_head
}

"#,
                )],
            )],
            hosts,
        )
        .expect("malformed function source");
        let plan = crate::planner::plan_async_host_library_program(program)
            .expect("malformed function plan");
        let entry = |name: &str| {
            let template = plan
                .functions()
                .iter()
                .find(|function| function.name() == name)
                .expect("malformed fixture public function")
                .signature()
                .id();
            LibraryEntry::new(template, LibraryValueType::Int, Vec::new(), Vec::new())
        };
        let first = entry("call_int_factory");
        let remaining = [
            entry("expose_float_factory"),
            entry("call_external_factory"),
            entry("expose_external_list_factory"),
            entry("resume_compound_projections"),
        ];
        let (execution, entries) =
            AsyncHostedExecution::from_library_plan(plan, first, remaining.into());
        let functions = std::array::from_fn(|index| *entries.ints[index].function());

        (execution, functions)
    }

    fn wrong_float_factory() -> EvaluatedFunctionValue<TransferValues> {
        let returned = FunctionType::new(vec![ValueType::Float], ValueType::Float);
        let type_ = FunctionType::new(Vec::new(), ValueType::Function(Box::new(returned)));
        let runtime_id = ProfiledFunctionFunctionId::<std::convert::Infallible>::Float(
            FloatFunctionFunctionId(0),
        );
        EvaluatedFunctionFunction::Core(EvaluatedFunction::reference(
            runtime_id,
            Vec::new(),
            Vec::new(),
            type_,
        ))
        .into()
    }

    fn wrong_external_list_factory() -> EvaluatedFunctionValue<TransferValues> {
        let external = ExternalTypeId::new(0);
        let list = ExternalListTypeId::for_test(ListTypeId::for_test(0), external);
        let returned = FunctionType::new(Vec::new(), ValueType::List(list.list_type()));
        let type_ = FunctionType::new(Vec::new(), ValueType::Function(Box::new(returned.clone())));
        let runtime_id = ExternalFunctionCallTarget::ListFunction {
            id: ExternalListFunctionFunctionId(0),
            type_: returned,
            list_type: list,
        };
        EvaluatedFunctionFunction::External(EvaluatedFunction::reference(
            runtime_id,
            Vec::new(),
            Vec::new(),
            type_,
        ))
        .into()
    }

    fn assert_family_mismatch(
        execution: &AsyncHostedExecution<StatelessHostProfile>,
        function: IntFunctionId,
        wrong_factory: EvaluatedFunctionValue<TransferValues>,
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) {
        let mut inputs = ProfiledRetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Function(wrong_factory));
        let mut host = ();
        let mut echo = RecordedEcho::default();
        let observed = std::sync::Arc::clone(&echo.0);
        let mut stores = ();
        let mut state =
            ResumableState::<StatelessHostProfile>::new(&mut host, &mut stores, &mut echo);
        let (result, pending) = poll_to_ready(run_int(
            execution,
            &mut state,
            function,
            crate::runtime::error::HostCallOrigin::Entry,
            inputs,
        ));
        drop(state);
        let error = result
            .expect_err("corrupted function family should fail")
            .into_local();

        assert_eq!(pending, 1);
        assert_eq!(observed.lock().expect("echo observation lock").len(), 1);
        assert_eq!(
            error,
            ExecutionError::Invariant(InvariantError::FunctionReturnFamilyMismatch {
                expected,
                actual,
            }),
        );
    }

    fn poll_to_ready<Output>(future: impl Future<Output = Output>) -> (Output, usize) {
        let mut future = pin!(future);
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut pending = 0;
        loop {
            match Pin::as_mut(&mut future).poll(&mut context) {
                Poll::Ready(output) => return (output, pending),
                Poll::Pending => pending += 1,
            }
        }
    }
}
