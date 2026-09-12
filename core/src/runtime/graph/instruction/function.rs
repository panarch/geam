use super::super::RuntimeGraphState;
use super::super::environment::BlockEnvironment;
use super::value::{
    InstructionValue, InstructionValueWithoutConstant, custom_projection, list_element,
    tuple_projection,
};
use crate::plan::ValueType;
use crate::plan::execution::function::{ProfiledFunctionFunctionId, RuntimeListFunctionId};
use crate::plan::execution::graph::{
    ExternalFunctionCallTarget, ExternalFunctionInstructionKind, ExternalFunctionInstructionView,
    ExternalFunctionTarget, FunctionCapture, FunctionInstruction, FunctionInstructionKind,
    FunctionLocal, FunctionTarget, ParamLocal,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::InvariantError;
use crate::runtime::evaluated::{
    EvaluatedCapture, EvaluatedCustomFunction, EvaluatedFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedListCapture, EvaluatedValue, FunctionReferenceId,
};
use std::convert::Infallible;

#[derive(Clone, Copy)]
enum FunctionIdentity {
    Reference,
    Instance,
}

pub(in crate::runtime) type CoreFunctionInstructionValue =
    InstructionValue<EvaluatedFunctionValue, ProfiledFunctionFunctionId<Infallible>, FunctionLocal>;

pub(in crate::runtime) type ExternalFunctionInstructionValue =
    InstructionValueWithoutConstant<EvaluatedFunctionValue, ExternalFunctionCallTarget>;

pub(in crate::runtime) trait FunctionParameterPlan:
    RuntimeExecutionPlan
{
    fn function_target_params(&self, target: &FunctionTarget) -> Vec<ParamLocal>;

    fn external_function_target_params(&self, target: &ExternalFunctionTarget) -> Vec<ParamLocal>;
}

pub(in crate::runtime) fn evaluate_action<Plan, State>(
    plan: &Plan,
    state: &State,
    environment: &BlockEnvironment,
    instruction: &FunctionInstruction,
    expected: &ValueType,
) -> Result<CoreFunctionInstructionValue, State::Error>
where
    Plan: FunctionParameterPlan,
    State: RuntimeGraphState,
{
    use FunctionInstructionKind as I;
    use InstructionValue as V;

    let value: Result<_, State::Error> = match instruction.kind() {
        I::Constant(id) => Ok(V::Constant(*id)),
        I::Reference(target) => Ok(V::Ready(target_value(
            plan,
            target,
            Vec::new(),
            instruction.type_().clone(),
            FunctionIdentity::Reference,
        ))),
        I::Closure { target, captures } => Ok(V::Ready(target_value(
            plan,
            target,
            capture_values(environment, captures),
            instruction.type_().clone(),
            FunctionIdentity::Instance,
        ))),
        I::Constructor(constructor) => Ok(V::Ready(
            EvaluatedCustomFunction::constructor(*constructor, instruction.type_().clone()).into(),
        )),
        I::Call {
            function,
            args,
            site,
        } => Ok(V::Call {
            function: function.clone(),
            origin: crate::runtime::error::HostCallOrigin::source(site.clone()),
            inputs: environment.retain(args),
        }),
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.core_function_function(function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(function.captures());
            Ok(V::Call {
                function: function.runtime_id(),
                origin: crate::runtime::error::HostCallOrigin::source(site.clone()),
                inputs,
            })
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan.value_metadata(),
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Function(value) => Some(value.clone()),
                _ => None,
            },
        )
        .map(V::Ready),
        I::CustomField { source, index } => custom_projection(
            plan,
            environment,
            source,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::Function(value) => Some(value.clone()),
                _ => None,
            },
        )
        .map(V::Ready),
        I::ListIndex { list, index } => list_element(
            expected,
            *index,
            &state
                .lists()
                .function_values(&environment.function_list(*list)),
        )
        .map(V::Ready),
    };
    match value? {
        V::Ready(value) => {
            validate_return_family(value, instruction.family(), instruction.type_().clone())
                .map(V::Ready)
        }
        value => Ok(value),
    }
}

pub(in crate::runtime) fn evaluate_external_action<Plan>(
    plan: &Plan,
    environment: &BlockEnvironment,
    instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
) -> ExternalFunctionInstructionValue
where
    Plan: FunctionParameterPlan,
{
    use ExternalFunctionInstructionKind as I;
    use InstructionValueWithoutConstant as V;

    let instruction = instruction.instruction();
    match instruction.kind() {
        I::Reference(target) => V::Ready(external_target_value(
            plan,
            target,
            Vec::new(),
            instruction.type_().clone(),
            FunctionIdentity::Reference,
        )),
        I::Closure { target, captures } => V::Ready(external_target_value(
            plan,
            target,
            capture_values(environment, captures),
            instruction.type_().clone(),
            FunctionIdentity::Instance,
        )),
        I::Call {
            function,
            args,
            site,
        } => V::Call {
            function: function.clone(),
            origin: crate::runtime::error::HostCallOrigin::source(site.clone()),
            inputs: environment.retain(args),
        },
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.external_function_function(function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(function.captures());
            V::Call {
                function: function.runtime_id(),
                origin: crate::runtime::error::HostCallOrigin::source(site.clone()),
                inputs,
            }
        }
    }
}

fn target_value<Plan>(
    plan: &Plan,
    target: &FunctionTarget,
    captures: Vec<EvaluatedCapture>,
    type_: crate::plan::execution::type_::FunctionType,
    identity: FunctionIdentity,
) -> EvaluatedFunctionValue
where
    Plan: FunctionParameterPlan,
{
    let params = plan.function_target_params(target);
    match target {
        FunctionTarget::Generic(function) => {
            evaluated_function(function.clone(), params, captures, type_, identity).into()
        }
        FunctionTarget::Never(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::Int(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::Float(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::String(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::BitArray(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::UtfCodepoint(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::Custom(function) => EvaluatedCustomFunction::Function(evaluated_function(
            *function, params, captures, type_, identity,
        ))
        .into(),
        FunctionTarget::Bool(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::Nil(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::Tuple(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        FunctionTarget::List(function) => evaluated_function(
            RuntimeListFunctionId::Core(function.clone()),
            params,
            captures,
            type_,
            identity,
        )
        .into(),
        FunctionTarget::Function(function) => EvaluatedFunctionFunction::Core(evaluated_function(
            function.clone(),
            params,
            captures,
            type_,
            identity,
        ))
        .into(),
    }
}

fn external_target_value<Plan>(
    plan: &Plan,
    target: &ExternalFunctionTarget,
    captures: Vec<EvaluatedCapture>,
    type_: crate::plan::execution::type_::FunctionType,
    identity: FunctionIdentity,
) -> EvaluatedFunctionValue
where
    Plan: FunctionParameterPlan,
{
    let params = plan.external_function_target_params(target);
    match target {
        ExternalFunctionTarget::Value(function) => {
            evaluated_function(*function, params, captures, type_, identity).into()
        }
        ExternalFunctionTarget::List(function) => evaluated_function(
            RuntimeListFunctionId::External(*function),
            params,
            captures,
            type_,
            identity,
        )
        .into(),
        ExternalFunctionTarget::Function(function) => {
            EvaluatedFunctionFunction::External(evaluated_function(
                ExternalFunctionCallTarget::Function(function.clone()),
                params,
                captures,
                type_,
                identity,
            ))
            .into()
        }
        ExternalFunctionTarget::ListFunction {
            id,
            type_: function_type,
            list_type,
        } => EvaluatedFunctionFunction::External(evaluated_function(
            ExternalFunctionCallTarget::ListFunction {
                id: *id,
                type_: function_type.clone(),
                list_type: *list_type,
            },
            params,
            captures,
            type_,
            identity,
        ))
        .into(),
    }
}

fn evaluated_function<Id>(
    function: Id,
    params: Vec<ParamLocal>,
    captures: Vec<EvaluatedCapture>,
    type_: crate::plan::execution::type_::FunctionType,
    identity: FunctionIdentity,
) -> EvaluatedFunction<Id>
where
    Id: Clone + FunctionReferenceId,
{
    match identity {
        FunctionIdentity::Reference => {
            EvaluatedFunction::reference(function, params, captures, type_)
        }
        FunctionIdentity::Instance => EvaluatedFunction::closure(function, params, captures, type_),
    }
}

pub(in crate::runtime) fn validate_return_family<Error>(
    value: EvaluatedFunctionValue,
    expected: crate::plan::execution::function::FunctionReturnFamily,
    type_: crate::plan::execution::type_::FunctionType,
) -> Result<EvaluatedFunctionValue, Error>
where
    Error: From<InvariantError>,
{
    let actual = value.kind().family();
    if actual == expected {
        Ok(value.with_type(type_))
    } else {
        Err(InvariantError::FunctionReturnFamilyMismatch { expected, actual }.into())
    }
}

impl<Plan: RuntimeExecutionPlan> FunctionParameterPlan for Plan {
    fn function_target_params(&self, target: &FunctionTarget) -> Vec<ParamLocal> {
        self.function_parameters().function(target).to_vec()
    }

    fn external_function_target_params(&self, target: &ExternalFunctionTarget) -> Vec<ParamLocal> {
        self.function_parameters()
            .external_function(target)
            .to_vec()
    }
}

fn capture_values(
    environment: &BlockEnvironment,
    captures: &[FunctionCapture],
) -> Vec<EvaluatedCapture> {
    captures
        .iter()
        .map(|capture| match capture {
            FunctionCapture::Int { target, source } => {
                EvaluatedCapture::int(*target, environment.int(*source))
            }
            FunctionCapture::Float { target, source } => {
                EvaluatedCapture::float(*target, environment.float(*source))
            }
            FunctionCapture::String { target, source } => {
                EvaluatedCapture::string(*target, environment.string(*source))
            }
            FunctionCapture::BitArray { target, source } => {
                EvaluatedCapture::bit_array(*target, environment.bit_array(*source))
            }
            FunctionCapture::UtfCodepoint { target, source } => {
                EvaluatedCapture::utf_codepoint(*target, environment.utf_codepoint(*source))
            }
            FunctionCapture::Custom { target, source } => {
                EvaluatedCapture::custom(*target, environment.custom(*source))
            }
            FunctionCapture::External { target, source } => {
                EvaluatedCapture::external(*target, environment.external(*source))
            }
            FunctionCapture::Bool { target, source } => {
                EvaluatedCapture::bool(*target, environment.bool(*source))
            }
            FunctionCapture::Nil { target, source } => {
                environment.nil(*source);
                EvaluatedCapture::nil(*target)
            }
            FunctionCapture::Tuple { target, source } => {
                EvaluatedCapture::tuple(*target, environment.tuple(*source))
            }
            FunctionCapture::ParameterList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Parameter {
                    local: *target,
                    value: environment.parameter_list(*source),
                })
            }
            FunctionCapture::ParameterListList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::ParameterList {
                    local: *target,
                    value: environment.parameter_list_list(*source),
                })
            }
            FunctionCapture::IntList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Int {
                    local: *target,
                    value: environment.int_list(*source),
                })
            }
            FunctionCapture::StringList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::String {
                    local: *target,
                    value: environment.string_list(*source),
                })
            }
            FunctionCapture::BitArrayList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::BitArray {
                    local: *target,
                    value: environment.bit_array_list(*source),
                })
            }
            FunctionCapture::UtfCodepointList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::UtfCodepoint {
                    local: *target,
                    value: environment.utf_codepoint_list(*source),
                })
            }
            FunctionCapture::CustomList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Custom {
                    local: *target,
                    value: environment.custom_list(*source),
                })
            }
            FunctionCapture::ExternalList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::External {
                    local: *target,
                    value: environment.external_list(*source),
                })
            }
            FunctionCapture::FloatList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Float {
                    local: *target,
                    value: environment.float_list(*source),
                })
            }
            FunctionCapture::BoolList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Bool {
                    local: *target,
                    value: environment.bool_list(*source),
                })
            }
            FunctionCapture::NilList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Nil {
                    local: *target,
                    value: environment.nil_list(*source),
                })
            }
            FunctionCapture::TupleList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                    local: *target,
                    value: environment.tuple_list(*source),
                })
            }
            FunctionCapture::ListList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::List {
                    local: *target,
                    value: environment.list_list(*source),
                })
            }
            FunctionCapture::FunctionList { target, source } => {
                EvaluatedCapture::list(EvaluatedListCapture::Function {
                    local: *target,
                    value: environment.function_list(*source),
                })
            }
            FunctionCapture::IntFunction { target, source } => {
                EvaluatedCapture::int_function(*target, environment.int_function(*source))
            }
            FunctionCapture::FloatFunction { target, source } => {
                EvaluatedCapture::float_function(*target, environment.float_function(*source))
            }
            FunctionCapture::StringFunction { target, source } => {
                EvaluatedCapture::string_function(*target, environment.string_function(*source))
            }
            FunctionCapture::BitArrayFunction { target, source } => {
                EvaluatedCapture::bit_array_function(
                    *target,
                    environment.bit_array_function(*source),
                )
            }
            FunctionCapture::UtfCodepointFunction { target, source } => {
                EvaluatedCapture::utf_codepoint_function(
                    *target,
                    environment.utf_codepoint_function(*source),
                )
            }
            FunctionCapture::GenericFunction { target, source } => {
                EvaluatedCapture::generic_function(
                    target.clone(),
                    environment.generic_function(source),
                )
            }
            FunctionCapture::NeverFunction { target, source } => {
                EvaluatedCapture::never_function(target.clone(), environment.never_function(source))
            }
            FunctionCapture::CustomFunction { target, source } => {
                EvaluatedCapture::custom_function(
                    target.clone(),
                    environment.custom_function(source),
                )
            }
            FunctionCapture::ExternalFunction { target, source } => {
                EvaluatedCapture::external_function(
                    target.clone(),
                    environment.external_function(source),
                )
            }
            FunctionCapture::BoolFunction { target, source } => {
                EvaluatedCapture::bool_function(*target, environment.bool_function(*source))
            }
            FunctionCapture::NilFunction { target, source } => {
                EvaluatedCapture::nil_function(*target, environment.nil_function(*source))
            }
            FunctionCapture::TupleFunction { target, source } => {
                EvaluatedCapture::tuple_function(*target, environment.tuple_function(*source))
            }
            FunctionCapture::ListFunction { target, source } => {
                EvaluatedCapture::list_function(target.clone(), environment.list_function(source))
            }
            FunctionCapture::FunctionFunction { target, source } => {
                EvaluatedCapture::function_function(
                    target.clone(),
                    environment.function_function(source),
                )
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::super::environment::{BlockEnvironment, RetainedValues};
    use super::{CoreFunctionInstructionValue, InstructionValue, evaluate_action};
    use crate::plan::ValueType;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, FunctionReturnFamily, RuntimeFunctionId, TupleFunctionId,
    };
    use crate::plan::execution::graph::{
        FunctionInstructionKind, FunctionTarget, ListInstruction, ProfiledInstructionKind,
    };
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedValue};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::StoredListValueId;
    use crate::runtime::{BitArrayValue, ExecutionError, InvariantError, ListValue, Value};
    use std::convert::Infallible;

    type InstructionKind = ProfiledInstructionKind<Infallible>;

    fn reference_function(value: CoreFunctionInstructionValue) -> EvaluatedFunctionValue {
        match value {
            InstructionValue::Ready(value) => value,
            _ => panic!("the source reference is immediate"),
        }
    }

    #[test]
    #[should_panic(expected = "the source reference is immediate")]
    fn reference_fixture_rejects_a_function_producing_call() {
        let plan = crate::runtime::plan_src(
            r#"
fn make() { fn(value: Int) { value } }
pub fn main() { #(42, fn(value: Int) { value }, make()) }
"#,
        );
        let graph = plan
            .tuple_function(tuple_main_id(&plan))
            .body()
            .block_graph();
        let function = graph
            .blocks()
            .iter()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction.kind() {
                InstructionKind::Function(function)
                    if matches!(function.kind(), FunctionInstructionKind::Call { .. }) =>
                {
                    Some(function)
                }
                _ => None,
            })
            .expect("source calls its function constructor");
        let expected = ValueType::Function(Box::new(plan.function_type(function.type_())));
        let mut echo = Vec::new();
        let state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());
        reference_function(
            evaluate_action(&plan, &state, &environment, function, &expected).unwrap(),
        );
    }

    #[test]
    fn function_projections_stop_corruption_at_their_owning_invariant() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(callback: fn(Int) -> Int) }

fn int_function(value: Int) -> Int { value }
fn float_function(value: Float) -> Float { value }

pub fn main() {
  let tuple_function = #(int_function).0
  let field_function = Boxed(int_function).callback
  let list_function = case [int_function] {
    [function] -> function
    _ -> int_function
  }
  #(float_function, tuple_function, field_function, list_function)
}
"#,
        );
        let graph = plan
            .tuple_function(tuple_main_id(&plan))
            .body()
            .block_graph();
        let function_list_type = graph
            .blocks()
            .iter()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction.kind() {
                InstructionKind::List(ListInstruction::Function(type_id, _)) => Some(*type_id),
                _ => None,
            })
            .expect("tuple main should allocate its function list");
        let float_reference = graph
            .blocks()
            .iter()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction.kind() {
                InstructionKind::Function(function)
                    if matches!(
                        function.kind(),
                        FunctionInstructionKind::Reference(FunctionTarget::Float(_))
                    ) =>
                {
                    Some(function)
                }
                _ => None,
            })
            .expect("tuple main should contain its Float function reference");
        let mut echo = Vec::new();
        let state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());
        let expected = ValueType::Function(Box::new(plan.function_type(float_reference.type_())));
        let float_function =
            evaluate_action(&plan, &state, &environment, float_reference, &expected)
                .expect("a typed function reference should evaluate");
        let float_function = reference_function(float_function);

        assert_eq!(float_function.kind().family(), FunctionReturnFamily::Float,);

        let mut tuple_projection_checked = false;
        let mut custom_projection_checked = false;
        let mut list_projection_checked = false;
        for block in graph.blocks() {
            for instruction in block.instructions() {
                let InstructionKind::Function(function) = instruction.kind() else {
                    continue;
                };
                let expected = ValueType::Function(Box::new(plan.function_type(function.type_())));
                match function.kind() {
                    FunctionInstructionKind::TupleIndex { .. } => {
                        let mut malformed = RetainedValues::empty();
                        malformed.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(
                            1.into(),
                        )]));
                        let environment = BlockEnvironment::from_retained(malformed);
                        let mut echo = Vec::new();
                        let state = RuntimeState::new(&mut echo);
                        assert_eq!(
                            evaluate_action(&plan, &state, &environment, function, &expected,)
                                .map(|_| ()),
                            Err(ExecutionError::Invariant(
                                InvariantError::TupleIndexFamilyMismatch {
                                    expected: expected.clone(),
                                    actual: ValueType::Int,
                                },
                            )),
                        );

                        let mut wrong_family = RetainedValues::empty();
                        wrong_family.push_evaluated(EvaluatedValue::Tuple(vec![
                            EvaluatedValue::Function(
                                float_function.clone().with_type(function.type_().clone()),
                            ),
                        ]));
                        let environment = BlockEnvironment::from_retained(wrong_family);
                        assert_eq!(
                            evaluate_action(&plan, &state, &environment, function, &expected,)
                                .map(|_| ()),
                            Err(ExecutionError::Invariant(
                                InvariantError::FunctionReturnFamilyMismatch {
                                    expected: FunctionReturnFamily::Int,
                                    actual: FunctionReturnFamily::Float,
                                },
                            )),
                        );
                        tuple_projection_checked = true;
                    }
                    FunctionInstructionKind::CustomField { .. } => {
                        let constructor = plan.custom_constructor_id(0, 0);
                        let descriptor = plan.custom_constructor(constructor);
                        let value = EvaluatedCustomValue::from_fields(
                            constructor,
                            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
                        );
                        let mut malformed = RetainedValues::empty();
                        malformed.push_evaluated(EvaluatedValue::Custom(value));
                        let environment = BlockEnvironment::from_retained(malformed);
                        let mut echo = Vec::new();
                        let state = RuntimeState::new(&mut echo);

                        assert_eq!(
                            evaluate_action(&plan, &state, &environment, function, &expected,)
                                .map(|_| ()),
                            Err(ExecutionError::Invariant(
                                InvariantError::CustomFieldFamilyMismatch {
                                    custom_type: plan.custom_value_type(constructor.type_id()),
                                    constructor: descriptor.name().clone(),
                                    field_index: 0,
                                    expected,
                                    actual: ValueType::Int,
                                },
                            )),
                        );
                        custom_projection_checked = true;
                    }
                    FunctionInstructionKind::ListIndex { index, .. } => {
                        let mut echo = Vec::new();
                        let mut state = RuntimeState::new(&mut echo);
                        let empty = state.lists_mut().function(function_list_type, Vec::new());
                        let mut values = RetainedValues::empty();
                        values.push_evaluated(EvaluatedValue::List(StoredListValueId::Function(
                            empty,
                        )));
                        let environment = BlockEnvironment::from_retained(values);

                        assert_eq!(
                            evaluate_action(&plan, &state, &environment, function, &expected,)
                                .map(|_| ()),
                            Err(ExecutionError::Invariant(
                                InvariantError::ListIndexOutOfBounds {
                                    item_type: expected,
                                    index: *index,
                                    length: 0,
                                },
                            )),
                        );
                        list_projection_checked = true;
                    }
                    _ => {}
                }
            }
        }

        assert!(tuple_projection_checked);
        assert!(custom_projection_checked);
        assert!(list_projection_checked);
    }

    fn tuple_main_id(plan: &crate::ExecutionPlan) -> TupleFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Tuple { id, .. }) => id,
            _ => panic!("expected main in the Tuple function table"),
        }
    }

    #[test]
    #[should_panic(expected = "expected main in the Tuple function table")]
    fn tuple_main_guard_rejects_other_function_tables() {
        tuple_main_id(&crate::runtime::plan_src("pub fn main() { 1 }"));
    }

    #[test]
    fn source_int_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }
fn identity(value: Int) -> Int { value }
fn make_adder(offset: Int) -> fn(Int) -> Int {
  fn(value) { value + offset }
}

pub fn main() {
  let local = add_one
  let maker = make_adder
  #(
    add_one(0),
    { let captured = 1 fn(value) { value + captured } }(1),
    local(2),
    make_adder(1)(3),
    maker(1)(4),
    #(add_one).0(5),
    case [add_one] { [function] -> function(6) _ -> 0 },
    case True { True -> add_one False -> identity }(7),
    case False { True -> identity False -> add_one }(8),
    case 1 { 1 -> add_one _ -> identity }(9),
    case 0 { 1 -> identity _ -> add_one }(10),
    case "hit" { "hit" -> add_one _ -> identity }(11),
    case "miss" { "hit" -> identity _ -> add_one }(12),
    case 1.0 { 1.0 -> add_one _ -> identity }(13),
    case 0.0 { 1.0 -> identity _ -> add_one }(14),
    { let _ = 0 add_one }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple((1_i64..=16).map(|value| Value::Int(value.into())).collect(),),
        );
    }

    #[test]
    fn source_float_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn add_half(value: Float) -> Float { value +. 0.5 }
fn identity(value: Float) -> Float { value }
fn make_adder(offset: Float) -> fn(Float) -> Float {
  fn(value) { value +. offset }
}

pub fn main() {
  let local = add_half
  let maker = make_adder
  #(
    add_half(0.0),
    { let captured = 1.0 fn(value) { value +. captured } }(1.0),
    local(2.0),
    make_adder(1.0)(3.0),
    maker(1.0)(4.0),
    #(add_half).0(5.0),
    case [add_half] { [function] -> function(6.0) _ -> 0.0 },
    case True { True -> add_half False -> identity }(7.0),
    case False { True -> identity False -> add_half }(8.0),
    case 1 { 1 -> add_half _ -> identity }(9.0),
    case 0 { 1 -> identity _ -> add_half }(10.0),
    case "hit" { "hit" -> add_half _ -> identity }(11.0),
    case "miss" { "hit" -> identity _ -> add_half }(12.0),
    case 1.0 { 1.0 -> add_half _ -> identity }(13.0),
    case 0.0 { 1.0 -> identity _ -> add_half }(14.0),
    { let _ = 0 add_half }(15.0),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                [
                    0.5, 2.0, 2.5, 4.0, 5.0, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5, 11.5, 12.5, 13.5, 14.5,
                    15.5,
                ]
                .into_iter()
                .map(Value::Float)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_string_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn append_bang(value: String) -> String { value <> "!" }
fn identity(value: String) -> String { value }
fn make_prefix(prefix: String) -> fn(String) -> String {
  fn(value) { prefix <> value }
}

pub fn main() {
  let local = append_bang
  let maker = make_prefix
  #(
    append_bang("0"),
    { let captured = "p" fn(value) { captured <> value } }("1"),
    local("2"),
    make_prefix("p")("3"),
    maker("p")("4"),
    #(append_bang).0("5"),
    case [append_bang] { [function] -> function("6") _ -> "missing" },
    case True { True -> append_bang False -> identity }("7"),
    case False { True -> identity False -> append_bang }("8"),
    case 1 { 1 -> append_bang _ -> identity }("9"),
    case 0 { 1 -> identity _ -> append_bang }("10"),
    case "hit" { "hit" -> append_bang _ -> identity }("11"),
    case "miss" { "hit" -> identity _ -> append_bang }("12"),
    case 1.0 { 1.0 -> append_bang _ -> identity }("13"),
    case 0.0 { 1.0 -> identity _ -> append_bang }("14"),
    { let _ = 0 append_bang }("15"),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                [
                    "0!", "p1", "2!", "p3", "p4", "5!", "6!", "7!", "8!", "9!", "10!", "11!",
                    "12!", "13!", "14!", "15!",
                ]
                .into_iter()
                .map(|value| Value::String(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_bool_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn invert(value: Bool) -> Bool { !value }
fn identity(value: Bool) -> Bool { value }
fn make_inverter() -> fn(Bool) -> Bool { invert }

pub fn main() {
  let local = invert
  let maker = make_inverter
  #(
    invert(False),
    { let captured = True fn(value) { value != captured } }(False),
    local(False),
    make_inverter()(False),
    maker()(True),
    #(invert).0(False),
    case [invert] { [function] -> function(False) _ -> False },
    case True { True -> invert False -> identity }(False),
    case False { True -> identity False -> invert }(True),
    case 1 { 1 -> invert _ -> identity }(False),
    case 0 { 1 -> identity _ -> invert }(True),
    case "hit" { "hit" -> invert _ -> identity }(False),
    case "miss" { "hit" -> identity _ -> invert }(True),
    case 1.0 { 1.0 -> invert _ -> identity }(False),
    case 0.0 { 1.0 -> identity _ -> invert }(True),
    { let _ = 0 invert }(False),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                vec![
                    true, true, true, true, false, true, true, true, false, true, false, true,
                    false, true, false, true,
                ]
                .into_iter()
                .map(Value::Bool)
                .collect(),
            ),
        );
    }

    #[test]
    fn source_nil_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
fn nil_value(_value: Int) -> Nil { Nil }
fn other_nil(_value: Int) -> Nil { Nil }
fn make_nil() -> fn(Int) -> Nil { nil_value }

pub fn main() {
  let local = nil_value
  let maker = make_nil
  #(
    nil_value(0),
    { let captured = 1 fn(_value) { let _ = captured Nil } }(1),
    local(2),
    make_nil()(3),
    maker()(4),
    #(nil_value).0(5),
    case [nil_value] { [function] -> function(6) _ -> Nil },
    case True { True -> nil_value False -> other_nil }(7),
    case False { True -> other_nil False -> nil_value }(8),
    case 1 { 1 -> nil_value _ -> other_nil }(9),
    case 0 { 1 -> other_nil _ -> nil_value }(10),
    case "hit" { "hit" -> nil_value _ -> other_nil }(11),
    case "miss" { "hit" -> other_nil _ -> nil_value }(12),
    case 1.0 { 1.0 -> nil_value _ -> other_nil }(13),
    case 0.0 { 1.0 -> other_nil _ -> nil_value }(14),
    { let _ = 0 nil_value }(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![Value::Nil; 16]),
        );
    }

    #[test]
    fn source_tuple_and_list_function_instructions_preserve_exact_values() {
        let source = r#"
fn pair(value: Int) { #(value) }
fn list(value: Int) { [value] }
fn make_pair(offset: Int) -> fn(Int) -> #(Int) { fn(value) { #(value + offset) } }
fn make_list(offset: Int) -> fn(Int) -> List(Int) { fn(value) { [value + offset] } }

pub fn main() {
  let pair_local = pair
  let list_local = list
  #(
    pair(0),
    { let captured = 1 fn(value) { #(value + captured) } }(0),
    pair_local(2),
    make_pair(1)(2),
    #(pair).0(5),
    case [pair] { [function] -> function(6) _ -> #(0) },
    case False { True -> fn(value) { #(value) } False -> pair }(7),
    list(0),
    { let captured = 1 fn(value) { [value + captured] } }(0),
    list_local(2),
    make_list(1)(2),
    #(list).0(5),
    case [list] { [function] -> function(6) _ -> [] },
    case False { True -> fn(value) { [value] } False -> list }(7),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![
                Value::Tuple(vec![Value::Int(0.into())]),
                Value::Tuple(vec![Value::Int(1.into())]),
                Value::Tuple(vec![Value::Int(2.into())]),
                Value::Tuple(vec![Value::Int(3.into())]),
                Value::Tuple(vec![Value::Int(5.into())]),
                Value::Tuple(vec![Value::Int(6.into())]),
                Value::Tuple(vec![Value::Int(7.into())]),
                Value::List(ListValue::int(vec![0.into()])),
                Value::List(ListValue::int(vec![1.into()])),
                Value::List(ListValue::int(vec![2.into()])),
                Value::List(ListValue::int(vec![3.into()])),
                Value::List(ListValue::int(vec![5.into()])),
                Value::List(ListValue::int(vec![6.into()])),
                Value::List(ListValue::int(vec![7.into()])),
            ]),
        );
    }

    #[test]
    fn source_custom_function_instruction_variants_evaluate_exact_values() {
        let source = r#"
pub type Boxed { Boxed(Int) }
fn boxed(value: Int) -> Boxed { Boxed(value) }
fn identity(value: Int) -> Boxed { Boxed(value) }
fn make_boxer(offset: Int) -> fn(Int) -> Boxed {
  fn(value) { Boxed(value + offset) }
}
fn unbox(value: Boxed) -> Int { case value { Boxed(inner) -> inner } }

pub fn main() {
  let constructor: fn(Int) -> Boxed = Boxed
  let local = boxed
  let maker = make_boxer
  #(
    unbox(constructor(0)),
    unbox({ let captured = 1 fn(value) { Boxed(value + captured) } }(0)),
    unbox(local(2)),
    unbox(make_boxer(1)(2)),
    unbox(maker(1)(3)),
    unbox(#(boxed).0(5)),
    case [boxed] { [function] -> unbox(function(6)) _ -> 0 },
    unbox(case True { True -> boxed False -> identity }(7)),
    unbox(case False { True -> identity False -> boxed }(8)),
    unbox(case 1 { 1 -> boxed _ -> identity }(9)),
    unbox(case 0 { 1 -> identity _ -> boxed }(10)),
    unbox(case "hit" { "hit" -> boxed _ -> identity }(11)),
    unbox(case "miss" { "hit" -> identity _ -> boxed }(12)),
    unbox(case 1.0 { 1.0 -> boxed _ -> identity }(13)),
    unbox(case 0.0 { 1.0 -> identity _ -> boxed }(14)),
    unbox({ let _ = 0 boxed }(15)),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple((0_i64..=15).map(|value| Value::Int(value.into())).collect(),),
        );
    }

    #[test]
    fn source_function_returning_function_instructions_preserve_values() {
        let source = r#"
fn add_one(value: Int) -> Int { value + 1 }
fn identity(value: Int) -> Int { value }
fn factory() -> fn(Int) -> Int { add_one }
fn other_factory() -> fn(Int) -> Int { identity }
fn return_factory() -> fn() -> fn(Int) -> Int { factory }
fn pass_factory(value: fn() -> fn(Int) -> Int) { value }

pub fn main() {
  let local = factory
  let pass = pass_factory
  #(
    factory()(0),
    { let captured = 1 fn() { fn(value) { value + captured } } }()(0),
    local()(2),
    return_factory()()(3),
    pass(factory)()(4),
    #(factory).0()(5),
    case [factory] { [value] -> value()(6) _ -> 0 },
    case True { True -> factory False -> other_factory }()(7),
    case False { True -> other_factory False -> factory }()(8),
    case 1 { 1 -> factory _ -> other_factory }()(9),
    case 0 { 1 -> other_factory _ -> factory }()(10),
    case "hit" { "hit" -> factory _ -> other_factory }()(11),
    case "miss" { "hit" -> other_factory _ -> factory }()(12),
    case 1.0 { 1.0 -> factory _ -> other_factory }()(13),
    case 0.0 { 1.0 -> other_factory _ -> factory }()(14),
    { let _ = 0 factory }()(15),
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(
                [1_i64, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                    .into_iter()
                    .map(|value| Value::Int(value.into()))
                    .collect(),
            ),
        );
    }

    #[test]
    fn source_bit_array_and_utf_codepoint_function_instructions_preserve_values() {
        let bit_array_bytes = [
            1, 2, 3, 4, 24, 5, 6, 23, 7, 99, 9, 99, 11, 99, 13, 99, 16, 99, 17, 99, 18, 99, 19, 99,
            15, 20, 21, 22,
        ];
        let codepoint_bytes = [
            1, 2, 3, 4, 24, 5, 6, 23, 25, 7, 99, 9, 99, 11, 99, 13, 99, 16, 99, 17, 99, 18, 99, 19,
            99, 15, 20, 21, 22,
        ];

        assert_eq!(
            crate::runtime::run_src(
                r#"fn identity(value: BitArray) -> BitArray {
  value
}

fn other(_: BitArray) -> BitArray {
  <<99>>
}

fn getter(_: Nil) -> fn(BitArray) -> BitArray {
  identity
}

fn selector() -> fn(Nil) -> fn(BitArray) -> BitArray {
  getter
}

fn apply(function: fn(BitArray) -> BitArray, value: BitArray) -> BitArray {
  function(value)
}

fn first(values: List(BitArray)) -> BitArray {
  let assert [value, ..] = values
  value
}

fn choose_bool(value: Bool) {
  case value {
    True -> identity
    False -> other
  }
}

fn choose_int(value: Int) {
  case value {
    1 -> identity
    _ -> other
  }
}

fn choose_string(value: String) {
  case value {
    "hit" -> identity
    _ -> other
  }
}

fn choose_float(value: Float) {
  case value {
    1.0 -> identity
    _ -> other
  }
}

pub fn main() {
  let local = identity
  let closure = fn(value) { value }
  let pair = #(identity)
  let assert [from_list] = [identity]
  let from_list_case = case [identity] {
    [function] -> function
    _ -> other
  }
  let from_direct_call = getter(Nil)
  let selected = selector()
  let from_function_call = selected(Nil)
  let from_function_subject = case identity {
    function -> function
  }
  let from_bool_case = case True {
    True -> identity
    False -> other
  }
  let from_bool_case_fallback = case False {
    True -> identity
    False -> other
  }
  let from_int_case = case 1 {
    1 -> identity
    _ -> other
  }
  let from_int_case_fallback = case 0 {
    1 -> identity
    _ -> other
  }
  let from_string_case = case "hit" {
    "hit" -> identity
    _ -> other
  }
  let from_string_case_fallback = case "miss" {
    "hit" -> identity
    _ -> other
  }
  let from_float_case = case 1.0 {
    1.0 -> identity
    _ -> other
  }
  let from_float_case_fallback = case 0.0 {
    1.0 -> identity
    _ -> other
  }
  let from_block = {
    let ignored = 1
    identity
  }
  let captured_function = identity
  let capturing_closure = fn(value) { captured_function(value) }

  #(
    local(<<1>>),
    closure(<<2>>),
    pair.0(<<3>>),
    from_list(<<4>>),
    from_list_case(<<24>>),
    from_direct_call(<<5>>),
    from_function_call(<<6>>),
    from_function_subject(<<23>>),
    choose_bool(True)(<<7>>),
    choose_bool(False)(<<8>>),
    choose_int(1)(<<9>>),
    choose_int(0)(<<10>>),
    choose_string("hit")(<<11>>),
    choose_string("miss")(<<12>>),
    choose_float(1.0)(<<13>>),
    choose_float(0.0)(<<14>>),
    from_bool_case(<<16>>),
    from_bool_case_fallback(<<16>>),
    from_int_case(<<17>>),
    from_int_case_fallback(<<17>>),
    from_string_case(<<18>>),
    from_string_case_fallback(<<18>>),
    from_float_case(<<19>>),
    from_float_case_fallback(<<19>>),
    from_block(<<15>>),
    apply(identity, <<20>>),
    first([<<21>>]),
    capturing_closure(<<22>>),
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[24], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[23], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[13], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[22], bit_len=8)])
"#
            ),
            Value::Tuple(
                bit_array_bytes
                    .into_iter()
                    .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                    .collect(),
            ),
        );
        assert_eq!(
            crate::runtime::run_src(
                r#"fn codepoint(value: Int) -> UtfCodepoint {
  case <<value>> {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

fn bits(value: UtfCodepoint) -> BitArray {
  <<value:utf8_codepoint>>
}

fn identity(value: UtfCodepoint) -> UtfCodepoint {
  value
}

fn other(_: UtfCodepoint) -> UtfCodepoint {
  codepoint(99)
}

fn getter(_: Nil) -> fn(UtfCodepoint) -> UtfCodepoint {
  identity
}

fn selector() -> fn(Nil) -> fn(UtfCodepoint) -> UtfCodepoint {
  getter
}

fn apply(function: fn(UtfCodepoint) -> UtfCodepoint, value: UtfCodepoint) {
  function(value)
}

fn first(values: List(UtfCodepoint)) -> UtfCodepoint {
  let assert [value, ..] = values
  value
}

fn choose_bool(value: Bool) {
  case value {
    True -> identity
    False -> other
  }
}

fn choose_int(value: Int) {
  case value {
    1 -> identity
    _ -> other
  }
}

fn choose_string(value: String) {
  case value {
    "hit" -> identity
    _ -> other
  }
}

fn choose_float(value: Float) {
  case value {
    1.0 -> identity
    _ -> other
  }
}

pub fn main() {
  let local = identity
  let closure = fn(value) { value }
  let pair = #(identity)
  let assert [from_list] = [identity]
  let from_list_case = case [identity] {
    [function] -> function
    _ -> other
  }
  let from_direct_call = getter(Nil)
  let selected = selector()
  let from_function_call = selected(Nil)
  let from_function_subject = case identity {
    function -> function
  }
  let from_guarded_function_subject = case identity {
    function if True -> function
    _ -> other
  }
  let from_bool_case = case True {
    True -> identity
    False -> other
  }
  let from_bool_case_fallback = case False {
    True -> identity
    False -> other
  }
  let from_int_case = case 1 {
    1 -> identity
    _ -> other
  }
  let from_int_case_fallback = case 0 {
    1 -> identity
    _ -> other
  }
  let from_string_case = case "hit" {
    "hit" -> identity
    _ -> other
  }
  let from_string_case_fallback = case "miss" {
    "hit" -> identity
    _ -> other
  }
  let from_float_case = case 1.0 {
    1.0 -> identity
    _ -> other
  }
  let from_float_case_fallback = case 0.0 {
    1.0 -> identity
    _ -> other
  }
  let from_block = {
    let ignored = 1
    identity
  }
  let captured_function = identity
  let capturing_closure = fn(value) { captured_function(value) }

  #(
    bits(local(codepoint(1))),
    bits(closure(codepoint(2))),
    bits(pair.0(codepoint(3))),
    bits(from_list(codepoint(4))),
    bits(from_list_case(codepoint(24))),
    bits(from_direct_call(codepoint(5))),
    bits(from_function_call(codepoint(6))),
    bits(from_function_subject(codepoint(23))),
    bits(from_guarded_function_subject(codepoint(25))),
    bits(choose_bool(True)(codepoint(7))),
    bits(choose_bool(False)(codepoint(8))),
    bits(choose_int(1)(codepoint(9))),
    bits(choose_int(0)(codepoint(10))),
    bits(choose_string("hit")(codepoint(11))),
    bits(choose_string("miss")(codepoint(12))),
    bits(choose_float(1.0)(codepoint(13))),
    bits(choose_float(0.0)(codepoint(14))),
    bits(from_bool_case(codepoint(16))),
    bits(from_bool_case_fallback(codepoint(16))),
    bits(from_int_case(codepoint(17))),
    bits(from_int_case_fallback(codepoint(17))),
    bits(from_string_case(codepoint(18))),
    bits(from_string_case_fallback(codepoint(18))),
    bits(from_float_case(codepoint(19))),
    bits(from_float_case_fallback(codepoint(19))),
    bits(from_block(codepoint(15))),
    bits(apply(identity, codepoint(20))),
    bits(first([codepoint(21)])),
    bits(capturing_closure(codepoint(22))),
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[24], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[23], bit_len=8), BitArray(bytes=[25], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[13], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[22], bit_len=8)])
"#
            ),
            Value::Tuple(
                codepoint_bytes
                    .into_iter()
                    .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                    .collect(),
            ),
        );
    }
}
