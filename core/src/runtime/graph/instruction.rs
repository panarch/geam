mod external;
mod function;
mod list;
mod value;

use self::value::{InstructionValue, InstructionValueWithoutConstant};
use super::RuntimeGraphState;
use super::activation::{Activation, Frame, ReturnValue, Returns, enter_constant, enter_function};
use crate::plan::execution::constant::ConstantValue;
use crate::plan::execution::function::FunctionBodyOwner;
use crate::plan::execution::graph::{ProfiledInstruction, ProfiledInstructionKind};
use crate::runtime::error::ExecutionResult;
use crate::runtime::function::EntryTarget;
use crate::runtime::graph::GraphValue;
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};

pub(super) fn advance<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    instruction: &ProfiledInstruction<RuntimeGraph<Plan>>,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let expected = plan.shape_value_type(instruction.output().shape());
    let environment = &frame.position.environment;
    macro_rules! advance_value {
        ($evaluate:ident, $instruction:expr) => {{
            let value = value::$evaluate(plan, state, environment, $instruction, expected)?;
            Ok(advance_value(plan, frame, returns, value))
        }};
    }

    match instruction.kind() {
        ProfiledInstructionKind::Int(instruction) => advance_value!(int, instruction),
        ProfiledInstructionKind::Float(instruction) => advance_value!(float, instruction),
        ProfiledInstructionKind::String(instruction) => advance_value!(string, instruction),
        ProfiledInstructionKind::BitArray(instruction) => advance_value!(bit_array, instruction),
        ProfiledInstructionKind::UtfCodepoint(instruction) => {
            let value = value::utf_codepoint(plan, state, environment, instruction, expected)?;
            Ok(advance_without_constant(plan, frame, returns, value))
        }
        ProfiledInstructionKind::Custom(instruction) => advance_value!(custom, instruction),
        ProfiledInstructionKind::External(instruction) => {
            let value = external::evaluate_action(plan, state, environment, instruction, expected)?;
            Ok(advance_without_constant(plan, frame, returns, value))
        }
        ProfiledInstructionKind::ExternalList(instruction) => {
            plan.advance_external_list_instruction(state, frame, returns, instruction, expected)
        }
        ProfiledInstructionKind::ExternalFunction(instruction) => Ok(plan
            .advance_external_function_instruction(state.captures(), frame, returns, instruction)),
        ProfiledInstructionKind::Bool(instruction) => advance_value!(bool, instruction),
        ProfiledInstructionKind::Nil(instruction) => advance_value!(nil, instruction),
        ProfiledInstructionKind::Tuple(instruction) => advance_value!(tuple, instruction),
        ProfiledInstructionKind::List(instruction) => {
            let value = list::evaluate(plan, state, environment, instruction, expected)?;
            Ok(advance_list(plan, frame, returns, value))
        }
        ProfiledInstructionKind::Function(instruction) => {
            let value = function::evaluate_action(plan, state, environment, instruction, expected)?;
            Ok(advance_function(
                plan,
                frame,
                returns,
                value,
                instruction.family(),
                instruction.type_().clone(),
            ))
        }
    }
}

fn advance_value<'plan, Plan, Value, Id, Local>(
    plan: &'plan Plan,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    value: InstructionValue<Value, Id, Local>,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Value: ReturnValue,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    <Id::Body as FunctionBodyOwner>::Return: GraphValue<Evaluated = Value>,
    Local: ConstantValue + GraphValue<Evaluated = Value> + Sync + 'plan,
{
    match value {
        InstructionValue::Ready(value) => frame.store(value),
        InstructionValue::Constant(id) => {
            let destination = returns.suspend(frame);
            enter_constant(plan, id, destination, std::convert::identity)
        }
        InstructionValue::Call {
            function,
            origin,
            inputs,
        } => {
            let destination = returns.suspend(frame);
            enter_function(plan, function, origin, inputs, destination, Ok)
        }
    }
}

fn advance_without_constant<'plan, Plan, Value, Id>(
    plan: &'plan Plan,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    value: InstructionValueWithoutConstant<Value, Id>,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Value: ReturnValue,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    <Id::Body as FunctionBodyOwner>::Return: GraphValue<Evaluated = Value>,
{
    match value {
        InstructionValueWithoutConstant::Ready(value) => frame.store(value),
        InstructionValueWithoutConstant::Call {
            function,
            origin,
            inputs,
        } => {
            let destination = returns.suspend(frame);
            enter_function(plan, function, origin, inputs, destination, Ok)
        }
    }
}

fn advance_list<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    value: list::ListInstructionValue,
) -> Activation<'plan, Plan> {
    use list::ListInstructionValue as V;
    match value {
        V::Parameter(value) => advance_value(plan, frame, returns, value),
        V::ParameterList(value) => advance_value(plan, frame, returns, value),
        V::Int(value) => advance_value(plan, frame, returns, value),
        V::String(value) => advance_value(plan, frame, returns, value),
        V::BitArray(value) => advance_value(plan, frame, returns, value),
        V::UtfCodepoint(value) => advance_value(plan, frame, returns, value),
        V::Custom(value) => advance_value(plan, frame, returns, value),
        V::Float(value) => advance_value(plan, frame, returns, value),
        V::Bool(value) => advance_value(plan, frame, returns, value),
        V::Nil(value) => advance_value(plan, frame, returns, value),
        V::Tuple(value) => advance_value(plan, frame, returns, value),
        V::List(value) => advance_value(plan, frame, returns, value),
        V::Function(value) => advance_value(plan, frame, returns, value),
    }
}

pub(super) fn advance_external_list<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    instruction: &crate::plan::execution::graph::ExternalListInstruction,
    expected: &crate::plan::execution::type_::ValueType,
) -> ExecutionResult<Activation<'plan, Plan>> {
    let value = list::evaluate_external(
        plan,
        state,
        &frame.position.environment,
        instruction,
        expected,
    )?;
    Ok(advance_value(plan, frame, returns, value))
}

fn advance_function<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    value: function::CoreFunctionInstructionValue,
    family: crate::plan::execution::function::FunctionReturnFamily,
    type_: crate::plan::execution::type_::FunctionType,
) -> Activation<'plan, Plan> {
    use crate::plan::execution::function::ProfiledFunctionFunctionId as F;
    match value {
        InstructionValue::Ready(value) => frame.store(value),
        InstructionValue::Constant(id) => {
            let destination = returns.suspend(frame);
            enter_constant(plan, id, destination, move |value| value.with_type(type_))
        }
        InstructionValue::Call {
            function,
            origin,
            inputs,
        } => {
            let destination = returns.suspend(frame);
            macro_rules! enter {
                ($function:expr) => {
                    enter_function(plan, $function, origin, inputs, destination, move |value| {
                        function::validate_return_family(value.into(), family, type_)
                    })
                };
            }
            match function {
                F::Generic(function) => enter!(function),
                F::Never(function) => enter!(function),
                F::Int(function) => enter!(function),
                F::Float(function) => enter!(function),
                F::String(function) => enter!(function),
                F::BitArray(function) => enter!(function),
                F::UtfCodepoint(function) => enter!(function),
                F::Custom(function) => enter!(function),
                F::External(never) => match never {},
                F::Bool(function) => enter!(function),
                F::Nil(function) => enter!(function),
                F::Tuple(function) => enter!(function),
                F::List(function) => enter!(function),
                F::Function(function) => enter!(function),
            }
        }
    }
}

pub(super) fn advance_external_function<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    captures: &crate::runtime::CaptureStorage,
    frame: Frame<'plan, Plan>,
    returns: &mut Returns<'plan, Plan>,
    instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
) -> Activation<'plan, Plan> {
    use crate::plan::execution::graph::{
        ExternalFunctionCallTarget, ExternalFunctionInstructionView,
    };
    let metadata = instruction.instruction();
    let family = metadata.family();
    let type_ = metadata.type_().clone();
    let validate = move |value| function::validate_return_family(value, family, type_);
    match function::evaluate_external_action(
        plan,
        captures,
        &frame.position.environment,
        instruction,
    ) {
        InstructionValueWithoutConstant::Ready(value) => frame.store(value),
        InstructionValueWithoutConstant::Call {
            function,
            origin,
            inputs,
        } => {
            let destination = returns.suspend(frame);
            match function {
                ExternalFunctionCallTarget::Function(function) => {
                    enter_function(plan, function, origin, inputs, destination, move |value| {
                        validate(value.into())
                    })
                }
                ExternalFunctionCallTarget::ListFunction { id, .. } => {
                    enter_function(plan, id, origin, inputs, destination, move |value| {
                        validate(value.into())
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::value::{ensure_list_index, list_element};
    use crate::ExecutionPlan;
    use crate::execution_fixture::TestHost;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use crate::plan::execution::ExecutionProgram;
    use crate::plan::execution::function::{
        BoolFunctionId, ExecutionBoolFunctionBody, ExecutionFunction, ExecutionFunctionBody,
        ExecutionHostTarget, ExecutionIntFunctionBody, ExecutionNeverHostTarget, IntFunctionId,
        TupleFunctionId,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ValueType as ExecutionValueType;
    use crate::plan::{
        CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, ValueType,
    };
    use crate::runtime::error::ExecutionResult;
    use crate::runtime::execution::invocation::Waiting;
    use crate::runtime::execution::{Domain, ServiceContext};
    use crate::runtime::graph::{
        Activation, Frame, GraphExecution, GraphProgress, GraphValue, Returns, RuntimeGraphState,
    };
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{
        CaptureStorage, EvaluatedValue, ExecutableRuntimePlan, ExecutionError, HostCallOrigin,
        InvariantError, RetainedValues,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{HostProviderSet, HostedExecution, ModuleSource, PackageSource};
    use std::convert::Infallible;
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    #[test]
    fn resumed_instructions_preserve_each_tuple_projection_and_reject_corrupted_values() {
        let mut state = ();
        assert!(std::ptr::eq(Profile::component_state(&mut state), &state));
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
pub fn main() {{ #(project, {sample}) }}
"#
            );
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
                        [ModuleSource::new("main", "main.gleam", source.as_str())],
                    ),
                ],
                HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                    .unwrap(),
            )
            .expect("typed projection source");
            let mut execution =
                HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                    .unwrap();
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
                GraphExecution::new(body.block_graph().as_view(), RetainedValues::empty());
            let mut returns = Returns::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            loop {
                match graph
                    .advance(self, &mut state, &mut returns, &mut 32)
                    .unwrap()
                {
                    GraphProgress::Continue(next) => graph = next,
                    GraphProgress::Complete(_) => return,
                    GraphProgress::Host(never) => match never {},
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

        fn advance_external_list_instruction<'plan>(
            &'plan self,
            _state: &mut impl RuntimeGraphState<Error = ExecutionError>,
            _frame: Frame<'plan, Self>,
            _returns: &mut Returns<'plan, Self>,
            instruction: &Infallible,
            _expected: &ExecutionValueType,
        ) -> ExecutionResult<Activation<'plan, Self>> {
            match *instruction {}
        }

        fn advance_external_function_instruction<'plan>(
            &'plan self,
            _captures: &CaptureStorage,
            _frame: Frame<'plan, Self>,
            _returns: &mut Returns<'plan, Self>,
            instruction: &Infallible,
        ) -> Activation<'plan, Self> {
            match *instruction {}
        }
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
        let values = imbl::Vector::unit(value.clone());
        assert_eq!(
            list_element::<_, ExecutionError>(&plan, &type_, 0, &values),
            Ok(value)
        );
        assert_eq!(
            ensure_list_index::<ExecutionError>(&plan, &ExecutionValueType::Nil, 0, 1),
            Ok(())
        );
        assert_eq!(plan.conversions(), 0);
        assert_eq!(
            list_element::<_, ExecutionError>(&plan, &type_, 1, &values),
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
