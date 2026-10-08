pub(in crate::runtime::graph) mod arithmetic;
mod external;
mod function;
mod list;
mod value;

pub(in crate::runtime) use function::ExternalFunctionInstructionOutcome;
pub(in crate::runtime) use list::ExternalListInstructionOutcome;

use self::list::ListInstructionOutcome;
use self::value::{InstructionOutcome, InstructionOutcomeWithoutConstant};
use super::activation::{Activation, Frame, ReturnValue, Storage, enter_constant, enter_function};
use super::environment::{RetainedValues, StoragePool};
use super::{BlockEnvironment, RuntimeGraphState};
use crate::plan::execution::function::ProfiledFunctionFunctionId;
use crate::plan::execution::graph::{
    ExternalFunctionCallTarget, ExternalFunctionInstruction, ExternalFunctionInstructionView,
    ExternalListInstruction, ListInstruction, ParamLocal, ProfiledInstruction,
    ProfiledInstructionKind,
};
use crate::plan::execution::type_::ValueType;
use crate::runtime::captures::Captures;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::{CaptureStorage, ExecutableRuntimePlan};

pub(in crate::runtime) struct CallInputs<'call> {
    args: &'call [ParamLocal],
    captures: Option<&'call Captures>,
}

// Keep evaluator temporaries out of the graph/terminator loop. A run owns its
// frame across consecutive instructions; only calls, block ends and exhausted
// budgets return it to the graph driver.
#[inline(never)]
pub(super) fn advance<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    mut frame: Frame<'plan, Plan>,
    storage: &mut Storage<'plan, Plan>,
    remaining: &mut usize,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let block = frame.graph.block(frame.position.block);
    while let Some(instruction) = block.instructions().get(frame.position.instruction) {
        frame.position.instruction += 1;
        let instruction = match instruction {
            ProfiledInstruction::Value(instruction) => instruction,
            ProfiledInstruction::IntegerRegion(region) => {
                storage
                    .arithmetic
                    .execute(region, &mut frame.position.environment);
                if frame.position.instruction == block.instructions().len() || *remaining == 0 {
                    return Ok(Activation::Graph(frame));
                }
                *remaining -= 1;
                continue;
            }
        };
        let expected = plan.shape_value_type(instruction.output().shape());
        let environment = &frame.position.environment;
        macro_rules! store_value {
            ($value:expr) => {
                match $value {
                    InstructionOutcome::Error(error) => return Err(error),
                    InstructionOutcome::Ready(value) => value.push(&mut frame.position.environment),
                    InstructionOutcome::Constant(id) => {
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_constant(
                            plan,
                            id,
                            destination,
                            &mut storage.pool,
                            std::convert::identity,
                        ));
                    }
                    InstructionOutcome::Call {
                        function,
                        site,
                        inputs,
                    } => {
                        let origin = HostCallOrigin::source(site.clone());
                        let inputs = inputs.retain(environment, &mut storage.pool);
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_function(
                            plan,
                            function,
                            origin,
                            inputs,
                            destination,
                            Ok,
                        ));
                    }
                }
            };
        }
        macro_rules! store_list {
            ($value:expr) => {
                match $value {
                    ListInstructionOutcome::Error(error) => return Err(error),
                    ListInstructionOutcome::Ready(value) => {
                        value.push(&mut frame.position.environment)
                    }
                    ListInstructionOutcome::Projected(value) => {
                        frame.position.environment.push_stored_list(value)
                    }
                    ListInstructionOutcome::Constant(id) => {
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_constant(
                            plan,
                            id,
                            destination,
                            &mut storage.pool,
                            std::convert::identity,
                        ));
                    }
                    ListInstructionOutcome::Call {
                        function,
                        site,
                        inputs,
                    } => {
                        let origin = HostCallOrigin::source(site.clone());
                        let inputs = inputs.retain(environment, &mut storage.pool);
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_function(
                            plan,
                            function,
                            origin,
                            inputs,
                            destination,
                            Ok,
                        ));
                    }
                }
            };
        }
        macro_rules! store_without_constant {
            ($value:expr) => {
                match $value {
                    InstructionOutcomeWithoutConstant::Error(error) => return Err(error),
                    InstructionOutcomeWithoutConstant::Ready(value) => {
                        value.push(&mut frame.position.environment)
                    }
                    InstructionOutcomeWithoutConstant::Call {
                        function,
                        site,
                        inputs,
                    } => {
                        let origin = HostCallOrigin::source(site.clone());
                        let inputs = inputs.retain(environment, &mut storage.pool);
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_function(
                            plan,
                            function,
                            origin,
                            inputs,
                            destination,
                            Ok,
                        ));
                    }
                }
            };
        }
        macro_rules! evaluate_value {
            ($evaluate:ident, $instruction:expr) => {
                store_value!(value::$evaluate(
                    plan,
                    state,
                    environment,
                    $instruction,
                    expected
                ))
            };
        }
        match instruction.kind() {
            ProfiledInstructionKind::Int(instruction) => evaluate_value!(int, instruction),
            ProfiledInstructionKind::Float(instruction) => evaluate_value!(float, instruction),
            ProfiledInstructionKind::String(instruction) => evaluate_value!(string, instruction),
            ProfiledInstructionKind::BitArray(instruction) => {
                evaluate_value!(bit_array, instruction)
            }
            ProfiledInstructionKind::UtfCodepoint(instruction) => {
                store_without_constant!(value::utf_codepoint(
                    plan,
                    state,
                    environment,
                    instruction,
                    expected
                ));
            }
            ProfiledInstructionKind::Custom(instruction) => evaluate_value!(custom, instruction),
            ProfiledInstructionKind::Bool(instruction) => evaluate_value!(bool, instruction),
            ProfiledInstructionKind::Nil(instruction) => evaluate_value!(nil, instruction),
            ProfiledInstructionKind::Tuple(instruction) => evaluate_value!(tuple, instruction),
            ProfiledInstructionKind::External(instruction) => {
                store_without_constant!(external::evaluate_action(
                    plan,
                    state,
                    environment,
                    instruction,
                    expected
                ));
            }
            ProfiledInstructionKind::ExternalList(instruction) => {
                store_list!(plan.evaluate_external_list_instruction(
                    state,
                    environment,
                    instruction,
                    expected
                ));
            }
            ProfiledInstructionKind::ExternalFunction(instruction) => {
                match plan.evaluate_external_function_instruction(
                    state.captures(),
                    environment,
                    instruction,
                ) {
                    InstructionOutcomeWithoutConstant::Error(never) => match never {},
                    InstructionOutcomeWithoutConstant::Ready(value) => {
                        value.push(&mut frame.position.environment)
                    }
                    InstructionOutcomeWithoutConstant::Call {
                        function,
                        site,
                        inputs,
                    } => {
                        let origin = HostCallOrigin::source(site.clone());
                        let inputs = inputs.retain(environment, &mut storage.pool);
                        let metadata = instruction.instruction();
                        let family = metadata.family();
                        let type_ = metadata.type_().clone();
                        let destination = storage.returns.suspend(frame);
                        macro_rules! enter {
                            ($id:expr) => {
                                enter_function(
                                    plan,
                                    $id,
                                    origin,
                                    inputs,
                                    destination,
                                    move |value| {
                                        function::validate_return_family(
                                            value.into(),
                                            family,
                                            type_,
                                        )
                                    },
                                )
                            };
                        }
                        return Ok(match function {
                            ExternalFunctionCallTarget::Function(id) => enter!(id),
                            ExternalFunctionCallTarget::ListFunction { id, .. } => enter!(id),
                        });
                    }
                }
            }
            ProfiledInstructionKind::List(instruction) => {
                macro_rules! evaluate_list {
                    ($family:ident, $type_id:expr, $instruction:expr) => {
                        store_list!(list::typed::<list::$family, _, _>(
                            plan,
                            state,
                            environment,
                            *$type_id,
                            $instruction,
                            expected
                        ))
                    };
                }
                match instruction {
                    ListInstruction::Parameter(type_id, instruction) => {
                        store_value!(list::parameter(
                            plan,
                            state,
                            environment,
                            *type_id,
                            instruction,
                            expected
                        ));
                    }
                    ListInstruction::ParameterList(type_id, instruction) => {
                        evaluate_list!(ParameterListFamily, type_id, instruction)
                    }
                    ListInstruction::Int(type_id, instruction) => {
                        evaluate_list!(IntFamily, type_id, instruction)
                    }
                    ListInstruction::String(type_id, instruction) => {
                        evaluate_list!(StringFamily, type_id, instruction)
                    }
                    ListInstruction::BitArray(type_id, instruction) => {
                        evaluate_list!(BitArrayFamily, type_id, instruction)
                    }
                    ListInstruction::UtfCodepoint(type_id, instruction) => {
                        evaluate_list!(UtfCodepointFamily, type_id, instruction)
                    }
                    ListInstruction::Custom(type_id, instruction) => {
                        evaluate_list!(CustomFamily, type_id, instruction)
                    }
                    ListInstruction::Float(type_id, instruction) => {
                        evaluate_list!(FloatFamily, type_id, instruction)
                    }
                    ListInstruction::Bool(type_id, instruction) => {
                        evaluate_list!(BoolFamily, type_id, instruction)
                    }
                    ListInstruction::Nil(type_id, instruction) => {
                        evaluate_list!(NilFamily, type_id, instruction)
                    }
                    ListInstruction::Tuple(type_id, instruction) => {
                        evaluate_list!(TupleFamily, type_id, instruction)
                    }
                    ListInstruction::List(type_id, instruction) => {
                        evaluate_list!(ListFamily, type_id, instruction)
                    }
                    ListInstruction::Function(type_id, instruction) => {
                        evaluate_list!(FunctionFamily, type_id, instruction)
                    }
                }
            }
            ProfiledInstructionKind::Function(instruction) => {
                match function::evaluate_action(plan, state, environment, instruction, expected) {
                    InstructionOutcome::Error(error) => return Err(error),
                    InstructionOutcome::Ready(value) => value.push(&mut frame.position.environment),
                    InstructionOutcome::Constant(id) => {
                        let type_ = instruction.type_().clone();
                        let destination = storage.returns.suspend(frame);
                        return Ok(enter_constant(
                            plan,
                            id,
                            destination,
                            &mut storage.pool,
                            move |value| value.with_type(type_),
                        ));
                    }
                    InstructionOutcome::Call {
                        function,
                        site,
                        inputs,
                    } => {
                        let origin = HostCallOrigin::source(site.clone());
                        let inputs = inputs.retain(environment, &mut storage.pool);
                        let family = instruction.family();
                        let type_ = instruction.type_().clone();
                        let destination = storage.returns.suspend(frame);
                        macro_rules! enter {
                            ($id:expr) => {
                                enter_function(
                                    plan,
                                    $id,
                                    origin,
                                    inputs,
                                    destination,
                                    move |value| {
                                        function::validate_return_family(
                                            value.into(),
                                            family,
                                            type_,
                                        )
                                    },
                                )
                            };
                        }
                        use ProfiledFunctionFunctionId as F;
                        return Ok(match function {
                            F::Generic(id) => enter!(id),
                            F::Never(id) => enter!(id),
                            F::Int(id) => enter!(id),
                            F::Float(id) => enter!(id),
                            F::String(id) => enter!(id),
                            F::BitArray(id) => enter!(id),
                            F::UtfCodepoint(id) => enter!(id),
                            F::Custom(id) => enter!(id),
                            F::External(never) => match never {},
                            F::Bool(id) => enter!(id),
                            F::Nil(id) => enter!(id),
                            F::Tuple(id) => enter!(id),
                            F::List(id) => enter!(id),
                            F::Function(id) => enter!(id),
                        });
                    }
                }
            }
        }
        // The caller paid for the first instruction. Only the next instruction
        // in this run is charged here; the outer driver owns the terminator.
        if frame.position.instruction == block.instructions().len() || *remaining == 0 {
            break;
        }
        *remaining -= 1;
    }
    Ok(Activation::Graph(frame))
}

impl CallInputs<'_> {
    fn retain(self, environment: &BlockEnvironment, pool: &mut StoragePool) -> RetainedValues {
        let mut inputs = pool.acquire();
        inputs.append_locals(environment, self.args);
        if let Some(captures) = self.captures {
            inputs.append_captures(captures);
        }
        inputs
    }
}

pub(super) fn evaluate_external_list<'call, Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    environment: &'call BlockEnvironment,
    instruction: &'call ExternalListInstruction,
    expected: &ValueType,
) -> ExternalListInstructionOutcome<'call, crate::ExecutionError> {
    list::evaluate_external(plan, state, environment, instruction, expected)
}

pub(super) fn evaluate_external_function<'call>(
    captures: &CaptureStorage,
    environment: &'call BlockEnvironment,
    instruction: &'call ExternalFunctionInstruction,
) -> ExternalFunctionInstructionOutcome<'call> {
    function::evaluate_external_action(captures, environment, instruction)
}

#[cfg(test)]
mod tests {
    use super::value::{ensure_list_index, list_element};
    use super::{CallInputs, StoragePool};
    use crate::ExecutionPlan;
    use crate::execution_fixture::TestHost;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use crate::plan::execution::ExecutionProgram;
    use crate::plan::execution::function::{
        BoolFunctionId, ExecutionBoolFunctionBody, ExecutionFunction, ExecutionFunctionBody,
        ExecutionHostTarget, ExecutionIntFunctionBody, ExecutionNeverHostTarget, IntFunctionId,
        TupleFunctionId,
    };
    use crate::plan::execution::graph::{IntLocalId, ParamLocal, StringLocalId};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ValueType as ExecutionValueType;
    use crate::plan::{
        CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, TypeParameterId,
        ValueType,
    };
    use crate::runtime::compiled::native_calls::NativeCallsMachine;
    use crate::runtime::error::ExecutionResult;
    use crate::runtime::execution::invocation::Waiting;
    use crate::runtime::execution::{Domain, ServiceContext};
    use crate::runtime::graph::tests::{CanonicalProgress, canonical_progress};
    use crate::runtime::graph::{
        BlockEnvironment, GraphExecution, GraphStorage, GraphValue, RuntimeGraphState,
    };
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::ListSequence;
    use crate::runtime::{
        CaptureStorage, EvaluatedCapture, EvaluatedValue, ExecutableRuntimePlan, ExecutionError,
        HostCallOrigin, InvariantError, RetainedValues,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{HostProviderSet, HostedExecution, ModuleSource, PackageSource};
    use std::convert::Infallible;
    use std::num::NonZeroUsize;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn call_inputs_preserve_argument_order_duplicates_and_captures_without_consuming_the_caller() {
        let mut caller = RetainedValues::empty();
        caller.push_int(10.into());
        caller.push_int(20.into());
        caller.push_string("argument".into());
        let caller = BlockEnvironment::from_retained(caller);
        let args = [
            ParamLocal::Int(IntLocalId(1)),
            ParamLocal::String(StringLocalId(0)),
            ParamLocal::Int(IntLocalId(0)),
            ParamLocal::Int(IntLocalId(1)),
        ];
        let storage = CaptureStorage::default();
        let captures = storage.capture(vec![
            EvaluatedCapture::int(IntLocalId(3), 30.into()),
            EvaluatedCapture::string(StringLocalId(1), "capture".into()),
        ]);
        let inputs = CallInputs {
            args: &args,
            captures: Some(&captures),
        }
        .retain(&caller, &mut StoragePool::default());
        assert!(inputs.belongs_to(Some(storage.domain())));
        assert!(!inputs.belongs_to(None));
        assert_eq!(caller.int(IntLocalId(0)), num_bigint::BigInt::from(10));
        assert_eq!(caller.int(IntLocalId(1)), num_bigint::BigInt::from(20));
        assert_eq!(caller.string(StringLocalId(0)), "argument");
        drop(caller);
        drop(captures);
        let inputs = BlockEnvironment::from_retained(inputs);
        assert_eq!(inputs.int(IntLocalId(0)), num_bigint::BigInt::from(20));
        assert_eq!(inputs.int(IntLocalId(1)), num_bigint::BigInt::from(10));
        assert_eq!(inputs.int(IntLocalId(2)), num_bigint::BigInt::from(20));
        assert_eq!(inputs.int(IntLocalId(3)), num_bigint::BigInt::from(30));
        assert_eq!(inputs.string(StringLocalId(0)), "argument");
        assert_eq!(inputs.string(StringLocalId(1)), "capture");
    }

    #[test]
    fn empty_call_inputs_preserve_the_presence_of_a_callable_domain() {
        let caller = BlockEnvironment::from_retained(RetainedValues::empty());
        let storage = CaptureStorage::default();
        let captures = storage.capture(Vec::new());
        let captured = CallInputs {
            args: &[],
            captures: Some(&captures),
        }
        .retain(&caller, &mut StoragePool::default());
        let direct = CallInputs {
            args: &[],
            captures: None,
        }
        .retain(&caller, &mut StoragePool::default());
        assert!(captured.belongs_to(Some(storage.domain())));
        assert!(!captured.belongs_to(None));
        assert!(!captured.belongs_to(Some(CaptureStorage::default().domain())));
        assert!(direct.belongs_to(None));
        assert!(direct.belongs_to(Some(storage.domain())));
    }

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    fn hosted_program(source: &str) -> HostedExecution<Profile> {
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("main", "main.gleam", source)],
                ),
            ],
            HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                .unwrap(),
        )
        .expect(source);
        HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap()).unwrap()
    }

    #[test]
    fn resumed_instructions_preserve_each_tuple_projection_and_reject_corrupted_values() {
        let mut state = ();
        assert!(ptr::eq(Profile::component_state(&mut state), &state));
        let work_type = ValueType::External(ExternalType::new(
            ExternalTypeName::new("work_fixture".into(), "fixture/work".into(), "Work".into()),
            vec![ValueType::Int],
        ));
        for (source_type, sample, expected_type) in [
            ("Int", "42", ValueType::Int),
            ("Float", "1.5", ValueType::Float),
            ("String", "\"text\"", ValueType::String),
            ("BitArray", "<<42>>", ValueType::BitArray),
            (
                "UtfCodepoint",
                "{ let assert <<c:utf8_codepoint>> = <<65>> c }",
                ValueType::UtfCodepoint,
            ),
            (
                "Item",
                "Item(42)",
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new("application".into(), "main".into(), "Item".into()),
                    vec![],
                )),
            ),
            ("work.Work(Int)", "work.ready(42)", work_type.clone()),
            ("Bool", "True", ValueType::Bool),
            ("Nil", "Nil", ValueType::Nil),
            ("#(Int)", "#(42)", ValueType::Tuple(vec![ValueType::Int])),
            (
                "List(Int)",
                "[42]",
                ValueType::List(Box::new(ValueType::Int)),
            ),
            (
                "List(a)",
                "[]",
                ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
            ),
            (
                "List(work.Work(Int))",
                "[work.ready(42)]",
                ValueType::List(Box::new(work_type.clone())),
            ),
            (
                "fn() -> work.Work(Int)",
                "fn() { work.ready(42) }",
                ValueType::Function(Box::new(FunctionType::new(vec![], work_type))),
            ),
            (
                "fn() -> Int",
                "fn() { 42 }",
                ValueType::Function(Box::new(FunctionType::new(vec![], ValueType::Int))),
            ),
        ] {
            let source = format!(
                r#"
import fixture/work
pub type Item {{ Item(Int) }}
fn project(value: #({source_type})) {{ #(value.0, 42) }}
pub fn main() -> #(fn(#({source_type})) -> #({source_type}, Int), {source_type}) {{ #(project, {sample}) }}
"#
            );
            let mut execution = hosted_program(&source);
            let (plan, stores, captures) = execution.parts_mut();
            let host = TestHost::default();
            let mut state = ();
            let mut echo = Vec::new();
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut state,
                stores,
                &mut echo,
                captures.clone(),
                Domain::<Profile>::DEFAULT_BUDGET,
            );
            let context = domain.context();
            host.block_on(domain.drive(async {
                let values = context
                    .call(
                        TupleFunctionId(0),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(values.len(), 2);
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(EvaluatedValue::Tuple(vec![values[1].clone()]));
                let result = context
                    .call(TupleFunctionId(1), HostCallOrigin::Entry, inputs)
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    result,
                    vec![values[1].clone(), EvaluatedValue::Int(42.into())],
                    "{source}"
                );

                // Only the retained tuple field is malformed; the source-derived
                // entry, instruction and destination retain their original types.
                let (wrong, actual) = if expected_type == ValueType::Int {
                    (EvaluatedValue::Bool(true), ValueType::Bool)
                } else {
                    (EvaluatedValue::Int(0.into()), ValueType::Int)
                };
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(EvaluatedValue::Tuple(vec![wrong]));
                assert_eq!(
                    context
                        .call(TupleFunctionId(1), HostCallOrigin::Entry, inputs)
                        .await
                        .unwrap(),
                    Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch {
                            expected: expected_type,
                            actual,
                        }
                    )),
                    "{source}"
                );
            }))
            .unwrap();
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn hosted_arithmetic_regions_continue_into_effects_with_remaining_budget() {
        let source = r#"
fn calculate(value: Int) {
  let scaled = value * 2 + 1
  let pair = #(scaled, 3)
  let observed = echo pair
  observed.0 + observed.1
}
pub fn main() { calculate(19) }
"#;
        let mut execution = hosted_program(source);
        let host = TestHost::default();
        let mut echo = Vec::new();
        let result = host.block_on(execution.run_main(&host, &mut (), &mut echo));
        assert_eq!(
            result.unwrap().try_into_value().unwrap(),
            crate::Value::Int(42.into())
        );
        assert_eq!(echo.len(), 1);
        assert_eq!(
            echo[0].value(),
            &crate::Value::Tuple(vec![
                crate::Value::Int(39.into()),
                crate::Value::Int(3.into())
            ])
        );
    }

    #[test]
    fn instruction_calls_resume_each_value_family_before_continuing() {
        for source in [
            "fn produce(_) { 42 } fn consume(value) { value }",
            "fn produce(_) { 1.5 } fn consume(value) { let assert 1.5 = value 42 }",
            "fn produce(_) { \"text\" } fn consume(value) { let assert \"text\" = value 42 }",
            "fn produce(_) { <<42>> } fn consume(value) { let assert <<42>> = value 42 }",
            "fn produce(_) { let assert <<c:utf8_codepoint>> = <<65>> c } fn consume(value) { let assert <<65>> = <<value:utf8_codepoint>> 42 }",
            "type Item { Item(Int) } fn produce(_) { Item(42) } fn consume(item) { let Item(value) = item value }",
            "fn produce(_) { future.ready(42) } fn consume(value) { let assert True = value == value 42 }",
            "fn produce(_) { True } fn consume(value) { let assert True = value 42 }",
            "fn produce(_) { Nil } fn consume(value) { let Nil = value 42 }",
            "fn produce(_) { #(40, 2) } fn consume(value) { let #(a, b) = value a + b }",
            "fn produce(_) -> List(a) { [] } fn consume(value) { let assert [] = value 42 }",
            "fn produce(_) -> List(List(a)) { [[]] } fn consume(value) { let assert [[]] = value 42 }",
            "fn produce(_) { [40, 2] } fn consume(value) { let assert [a, b] = value a + b }",
            "fn produce(_) { [1.5] } fn consume(value) { let assert [1.5] = value 42 }",
            "fn produce(_) { [\"text\"] } fn consume(value) { let assert [\"text\"] = value 42 }",
            "fn produce(_) { [<<42>>] } fn consume(value) { let assert [<<42>>] = value 42 }",
            "fn produce(_) { let assert <<c:utf8_codepoint>> = <<65>> [c] } fn consume(value) { let assert [c] = value let assert <<65>> = <<c:utf8_codepoint>> 42 }",
            "type Item { Item(Int) } fn produce(_) { [Item(42)] } fn consume(value) { let assert [Item(number)] = value number }",
            "fn produce(_) { [future.ready(42)] } fn consume(value) { let assert [work] = value let assert True = work == work 42 }",
            "fn produce(_) { [True] } fn consume(value) { let assert [True] = value 42 }",
            "fn produce(_) { [Nil] } fn consume(value) { let assert [Nil] = value 42 }",
            "fn produce(_) { [#(40, 2)] } fn consume(value) { let assert [#(a, b)] = value a + b }",
            "fn produce(_) { [[40, 2]] } fn consume(value) { let assert [[a, b]] = value a + b }",
            "fn produce(_) { [fn() { 42 }] } fn consume(value) { let assert [callback] = value callback() }",
            "fn produce(_) { fn(value) { value } } fn consume(callback) { callback(42) }",
            "fn produce(_) { fn(value) { value } } fn consume(_) { 42 }",
            "fn produce(_) { fn() { panic } } fn consume(_) { 42 }",
            "fn produce(_) { let captured = 40 fn() { captured + 2 } } fn consume(callback) { callback() }",
            "fn produce(_) { fn() { 1.5 } } fn consume(callback) { let assert 1.5 = callback() 42 }",
            "fn produce(_) { fn() { \"text\" } } fn consume(callback) { let assert \"text\" = callback() 42 }",
            "fn produce(_) { fn() { <<42>> } } fn consume(callback) { let assert <<42>> = callback() 42 }",
            "fn produce(_) { fn() { let assert <<c:utf8_codepoint>> = <<65>> c } } fn consume(callback) { let c = callback() let assert <<65>> = <<c:utf8_codepoint>> 42 }",
            "type Item { Item(Nil, Int) } fn produce(_) { fn() { Item(Nil, 42) } } fn consume(callback) { let Item(Nil, value) = callback() value }",
            "fn produce(_) { fn() { True } } fn consume(callback) { let assert True = callback() 42 }",
            "fn produce(_) { fn() { Nil } } fn consume(callback) { let Nil = callback() 42 }",
            "fn produce(_) { fn() { #(40, 2) } } fn consume(callback) { let #(a, b) = callback() a + b }",
            "fn produce(_) { fn() { [40, 2] } } fn consume(callback) { let assert [a, b] = callback() a + b }",
            "fn produce(_) { fn() { fn() { 42 } } } fn consume(callback) { let inner = callback() inner() }",
            "fn produce(_) { fn() { future.ready(1) } } fn consume(callback) { let work = callback() let assert True = work == work 42 }",
            "fn produce(_) { fn() { [future.ready(1)] } } fn consume(callback) { let assert [work] = callback() let assert True = work == work 42 }",
        ] {
            let source = format!(
                r#"
import fixture/work as future
{source}
pub fn main() {{
  let before = 1
  let returned = produce(Nil)
  let answer = consume(returned)
  answer + before - 1
}}
"#
            );
            let mut execution = hosted_program(&source);
            let host = TestHost::default();
            let mut echo = Vec::new();
            let result = host.block_on(execution.run_main(&host, &mut (), &mut echo));
            assert_eq!(
                result.unwrap().try_into_value().unwrap(),
                crate::Value::Int(42.into()),
                "{source}"
            );
            assert!(echo.is_empty(), "{source}");
        }
    }

    #[test]
    fn instruction_constants_resume_typed_and_symbolic_lists() {
        let source = r#"
import fixture/work
type Item { Item(Int) }
fn answer() { 42 }
const integer = 42
const floating = 1.5
const text = "text"
const bits = <<42>>
const item = Item(42)
const boolean = True
const nil = Nil
const pair = #(40, 2)
const callback = answer
const integers = [40, 2]
const floats = [1.5]
const strings = ["text"]
const arrays = [<<42>>]
const codepoints: List(UtfCodepoint) = []
const works: List(work.Work(Int)) = []
const items = [Item(42)]
const booleans = [True]
const nils = [Nil]
const tuples = [#(40, 2)]
const nested = [[40, 2]]
const callbacks = [answer]
const empty = []
fn generic_empty() -> List(a) { empty }
fn nested_empty() -> List(List(a)) { empty }
pub fn main() {
  let assert 42 = integer
  let assert 1.5 = floating
  let assert "text" = text
  let assert <<42>> = bits
  let assert Item(42) = item
  let assert True = boolean
  let Nil = nil
  let assert #(40, 2) = pair
  let assert 42 = callback()
  let assert [40, 2] = integers
  let assert [1.5] = floats
  let assert ["text"] = strings
  let assert [<<42>>] = arrays
  let assert [] = codepoints
  let assert [] = works
  let assert [Item(42)] = items
  let assert [True] = booleans
  let assert [Nil] = nils
  let assert [#(40, 2)] = tuples
  let assert [[40, 2]] = nested
  let assert [call] = callbacks
  let assert 42 = call()
  let assert [] = generic_empty()
  let assert [] = nested_empty()
  42
}
"#;
        let mut execution = hosted_program(source);
        let host = TestHost::default();
        let mut echo = Vec::new();
        let result = host.block_on(execution.run_main(&host, &mut (), &mut echo));
        assert_eq!(
            result.unwrap().try_into_value().unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn resumed_function_instructions_reject_a_corrupted_factory_return() {
        use crate::plan::execution::function::FunctionReturnFamily;

        let source = r#"
fn apply_int(factory: fn(fn() -> Float) -> fn() -> Int, value: fn() -> Float) {
  let result = factory(value)
  result()
}
fn apply_float(factory: fn(fn() -> Float) -> fn() -> Float, value: fn() -> Float) {
  let result = factory(value)
  let assert 1.5 = result()
  42
}
fn integer(_value: fn() -> Float) { fn() { 42 } }
fn floating(value: fn() -> Float) { leaf(value) }
fn leaf(value: fn() -> Float) { value }
pub fn main() { #(apply_int, apply_float, integer, floating, fn() { 1.5 }) }
"#;
        let mut execution = hosted_program(source);
        let (plan, stores, captures) = execution.parts_mut();
        let host = TestHost::default();
        let mut echo = Vec::new();
        let mut state = ();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 5);
            for (entry, factory, expected) in [
                (IntFunctionId(0), 2, Ok(42.into())),
                (IntFunctionId(1), 3, Ok(42.into())),
                (
                    IntFunctionId(0),
                    3,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
            ] {
                // Only the runtime factory argument is replaced. Its source-derived
                // entry and typed continuation still require an Int callable.
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(values[factory].clone());
                inputs.push_evaluated(values[4].clone());
                let result = context
                    .call(entry, HostCallOrigin::Entry, inputs)
                    .await
                    .unwrap();
                assert_eq!(result, expected);
            }
        }))
        .unwrap();
        assert!(echo.is_empty());
    }

    struct CountingPlan {
        plan: ExecutionPlan,
        conversions: AtomicUsize,
    }

    impl CountingPlan {
        fn new(source: &str) -> Self {
            Self {
                plan: crate::runtime::plan_src(source),
                conversions: AtomicUsize::new(0),
            }
        }

        fn evaluate_graph(&self) {
            let body = self.int_function(IntFunctionId(0)).body();
            let mut graph =
                GraphExecution::new(body.block_graph().as_view(), RetainedValues::empty(), None);
            let mut storage = GraphStorage::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            loop {
                match canonical_progress(
                    graph
                        .advance(self, &mut state, &mut storage, &mut 32)
                        .unwrap(),
                ) {
                    CanonicalProgress::Continue(next) => graph = next,
                    CanonicalProgress::Complete(_) => return,
                }
            }
        }

        fn conversions(&self) -> usize {
            self.conversions.load(Ordering::Relaxed)
        }
    }

    impl RuntimeExecutionPlan for CountingPlan {
        type Profile = Infallible;
        type RunState = ();

        fn program(&self) -> &ExecutionProgram<Infallible> {
            self.plan.program()
        }

        fn int_function(
            &self,
            id: IntFunctionId,
        ) -> &ExecutionFunction<Infallible, ExecutionIntFunctionBody<Infallible>> {
            self.plan.int_function(id)
        }

        fn bool_function(
            &self,
            id: BoolFunctionId,
        ) -> &ExecutionFunction<Infallible, ExecutionBoolFunctionBody<Infallible>> {
            self.plan.bool_function(id)
        }

        fn value_type(&self, type_: &ExecutionValueType) -> crate::plan::ValueType {
            self.conversions.fetch_add(1, Ordering::Relaxed);
            self.plan.value_type(type_)
        }
    }

    impl ExecutableRuntimePlan for CountingPlan {
        type RuntimeHost<'run> = ();
        type HostInvocation<'plan, Output: Send + 'plan> = Infallible;
        type NativeLoopBinding = Infallible;

        fn bind_native_loop(
            &self,
            contract: &crate::plan::execution::compiled::NativeLoopContract,
        ) -> Option<Infallible> {
            self.plan.bind_native_loop(contract)
        }

        fn prepare_native_loop(
            &self,
            state: crate::runtime::compiled::native_loop::NativeLoopState<Infallible>,
            _allowance: usize,
        ) -> Infallible {
            match state.binding {}
        }

        fn prepare_native_calls(
            &self,
            binding: Infallible,
            _machine: NativeCallsMachine,
            _allowance: usize,
        ) -> Infallible {
            match binding {}
        }

        fn reject_foreign_callable<'plan, Output: Send + 'plan>(
            &self,
            inputs: &RetainedValues,
            domain: Option<crate::runtime::captures::ExecutionDomain>,
        ) -> Option<Infallible> {
            self.plan.reject_foreign_callable::<Output>(inputs, domain)
        }

        fn prepare_host<Body>(
            &self,
            _origin: HostCallOrigin,
            target: &ExecutionHostTarget<Self::Profile, Body>,
            _inputs: RetainedValues,
        ) -> Infallible
        where
            Body: ExecutionFunctionBody,
            Body::Return: GraphValue,
        {
            match *target {}
        }

        fn prepare_host_never(
            &self,
            _origin: HostCallOrigin,
            target: &ExecutionNeverHostTarget<Self::Profile>,
            _inputs: RetainedValues,
        ) -> Infallible {
            match *target {}
        }

        fn map_host<'plan, Input: Send + 'plan, Output: Send + 'plan>(
            invocation: Infallible,
            _map: impl FnOnce(Input) -> ExecutionResult<Output> + Send + 'plan,
        ) -> Infallible {
            match invocation {}
        }

        fn submit_host<'plan, Output: Send + 'plan>(
            invocation: Infallible,
            _context: &ServiceContext<Self>,
            _budget: NonZeroUsize,
        ) -> Waiting<'plan, Output> {
            match invocation {}
        }

        fn evaluate_external_list_instruction<'call>(
            &self,
            _state: &mut impl RuntimeGraphState<Error = ExecutionError>,
            _environment: &'call BlockEnvironment,
            instruction: &'call Infallible,
            _expected: &ExecutionValueType,
        ) -> super::ExternalListInstructionOutcome<'call, ExecutionError> {
            match *instruction {}
        }

        fn evaluate_external_function_instruction<'call>(
            &self,
            _captures: &CaptureStorage,
            _environment: &'call BlockEnvironment,
            instruction: &'call Infallible,
        ) -> super::ExternalFunctionInstructionOutcome<'call> {
            match *instruction {}
        }
    }

    #[test]
    fn a_graph_only_native_loop_binding_does_not_materialize_host_types() {
        use crate::plan::execution::compiled::NativeLoopContract;
        let plan = CountingPlan::new(
            r#"
fn keep(value: Int) { value }
fn cycle(counter: Int, producer: fn() -> Int) {
  let returned = keep(producer())
  case counter { 1 -> returned _ -> cycle(counter - 1, producer) }
}
pub fn main() { cycle(3, fn() { 7 }) }
"#,
        );
        let contract =
            NativeLoopContract::inspect(plan.int_function(IntFunctionId(2)).body().block_graph())
                .unwrap();
        assert!(plan.bind_native_loop(&contract).is_none());
        plan.evaluate_graph();
        assert_eq!(plan.conversions(), 0);
    }

    #[test]
    fn ordinary_dispatch_and_successful_list_access_do_not_materialize_expected_types() {
        let plan = CountingPlan::new(
            r#"
fn positive(value: Int) { value > 0 }
pub fn main() {
  let captured = [1, 2]
  let transform = fn(value) {
    case captured { [first, ..] -> value + first _ -> value }
  }
  let nested = [[3, 4], [5]]
  let assert [first, ..] = nested
  let assert [value, ..] = first
  case positive(value) { True -> transform(value) + 1 False -> 0 }
}
"#,
        );
        plan.evaluate_graph();
        assert_eq!(plan.conversions(), 0);
    }

    #[test]
    fn projections_still_materialize_the_types_needed_for_exact_comparisons() {
        let plan = CountingPlan::new(
            r#"
type Box(value) { Box(value: value) }
pub fn main() {
  let boxed = Box([#(2)])
  let values = boxed.value
  let assert [tuple, ..] = values
  tuple.0 + 1
}
"#,
        );
        plan.evaluate_graph();
        assert_eq!(plan.conversions(), 2);
    }

    #[test]
    fn list_access_materializes_item_types_only_for_bounds_errors() {
        let plan = CountingPlan::new("pub fn main() { 0 }");
        let type_ = ExecutionValueType::Tuple(vec![ExecutionValueType::Int].into());
        let value = vec![EvaluatedValue::Int(7.into())];
        let values = ListSequence::from(vec![value.clone()]);
        assert_eq!(
            list_element::<_, ExecutionError, _>(&plan, &type_, 0, &values),
            Ok(value)
        );
        assert_eq!(
            ensure_list_index::<ExecutionError>(&plan, &ExecutionValueType::Nil, 0, 1),
            Ok(())
        );
        assert_eq!(plan.conversions(), 0);
        assert_eq!(
            list_element::<_, ExecutionError, _>(&plan, &type_, 1, &values),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: crate::plan::ValueType::Tuple(vec![crate::plan::ValueType::Int]),
                    index: 1,
                    length: 1,
                }
            )),
        );
        assert_eq!(plan.conversions(), 1);
        assert_eq!(
            ensure_list_index::<ExecutionError>(&plan, &ExecutionValueType::Nil, 1, 1),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: crate::plan::ValueType::Nil,
                    index: 1,
                    length: 1,
                }
            )),
        );
        assert_eq!(plan.conversions(), 2);
    }
}
