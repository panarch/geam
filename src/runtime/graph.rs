mod bit_array;
mod function;
mod instruction;
mod pattern;

#[cfg(test)]
pub(in crate::runtime) use function::{run_int, run_int_list};

use crate::plan::execution::{
    BlockId, Edge, ExecutionPlan, FunctionGraph, GraphNeverReturn, MatchEdge, MatchEdgeArgument,
    RuntimeFunctionId, SourceStopKind, Terminator,
};
use crate::runtime::environment::{BlockEnvironment, RetainedValues};
use crate::runtime::error::{ExecutionResult, PanicKind};
use crate::runtime::evaluated::{EvaluatedFunctionValue, EvaluatedValue};
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, Value};
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

pub(super) enum GraphExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: RetainedValues,
    },
}

pub(super) trait GraphValue {
    type Evaluated;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated;
}

pub(super) fn run_main(plan: &ExecutionPlan) -> ExecutionResult<Value> {
    let mut state = RuntimeState::new();
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        RuntimeFunctionId::Never(function) => {
            return function::run_never(plan, &mut state, function, inputs)
                .map(|never| match never {});
        }
        RuntimeFunctionId::Int(function) => {
            function::run_int(plan, &mut state, function, inputs).map(EvaluatedValue::Int)
        }
        RuntimeFunctionId::Float(function) => {
            function::run_float(plan, &mut state, function, inputs).map(EvaluatedValue::Float)
        }
        RuntimeFunctionId::String(function) => {
            function::run_string(plan, &mut state, function, inputs).map(EvaluatedValue::String)
        }
        RuntimeFunctionId::BitArray(function) => {
            function::run_bit_array(plan, &mut state, function, inputs)
                .map(EvaluatedValue::BitArray)
        }
        RuntimeFunctionId::UtfCodepoint(function) => {
            function::run_utf_codepoint(plan, &mut state, function, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        RuntimeFunctionId::Custom(function) => {
            function::run_custom(plan, &mut state, function, inputs).map(EvaluatedValue::Custom)
        }
        RuntimeFunctionId::Bool(function) => {
            function::run_bool(plan, &mut state, function, inputs).map(EvaluatedValue::Bool)
        }
        RuntimeFunctionId::Nil(function) => {
            function::run_nil(plan, &mut state, function, inputs).map(|()| EvaluatedValue::Nil)
        }
        RuntimeFunctionId::Tuple { id, .. } => {
            function::run_tuple(plan, &mut state, id, inputs).map(EvaluatedValue::Tuple)
        }
        RuntimeFunctionId::List(function) => {
            function::run_list(plan, &mut state, function, inputs).map(EvaluatedValue::List)
        }
        RuntimeFunctionId::Function { id, .. } => {
            function::run_function(plan, &mut state, id, inputs).map(EvaluatedValue::Function)
        }
    }?;
    state.drain_releases();
    Ok(crate::runtime::materialize::value(plan, &state, value))
}

pub(super) fn evaluate<Return, TailCall>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    graph: &FunctionGraph<Return, TailCall>,
    inputs: RetainedValues,
) -> ExecutionResult<GraphExit<Return::Evaluated, TailCall>>
where
    Return: GraphValue,
    TailCall: Clone,
{
    let mut block_id = graph.entry();
    let mut environment = BlockEnvironment::from_retained(inputs);

    loop {
        let block = graph.block(block_id);
        for instruction in block.instructions() {
            instruction::execute(plan, state, &mut environment, instruction)?;
        }

        match block.terminator() {
            Terminator::Jump(edge) => {
                (block_id, environment) = transition(state, environment, edge);
            }
            Terminator::BoolBranch {
                subject,
                true_,
                false_,
            } => {
                let edge = if environment.bool(*subject) {
                    true_
                } else {
                    false_
                };
                (block_id, environment) = transition(state, environment, edge);
            }
            Terminator::IntSwitch {
                subject,
                clauses,
                fallback,
            } => {
                let subject = environment.int(*subject);
                let selected = clauses
                    .iter()
                    .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
                let edge = match selected {
                    Some(edge) => edge,
                    None => fallback,
                };
                (block_id, environment) = transition(state, environment, edge);
            }
            Terminator::FloatSwitch {
                subject,
                clauses,
                fallback,
            } => {
                let subject = environment.float(*subject);
                let selected = clauses
                    .iter()
                    .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
                let edge = match selected {
                    Some(edge) => edge,
                    None => fallback,
                };
                (block_id, environment) = transition(state, environment, edge);
            }
            Terminator::StringSwitch {
                subject,
                clauses,
                fallback,
            } => {
                let subject = environment.string(*subject);
                let selected = clauses
                    .iter()
                    .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
                let edge = match selected {
                    Some(edge) => edge,
                    None => fallback,
                };
                (block_id, environment) = transition(state, environment, edge);
            }
            Terminator::Match {
                subject,
                pattern: matcher,
                success,
                failure,
            } => {
                let subject = environment.value(subject);
                let matched = pattern::match_pattern(plan, state, &environment, matcher, &subject)?;
                drop(subject);
                match matched {
                    Some(bindings) => {
                        (block_id, environment) =
                            transition_match(state, environment, success, bindings);
                    }
                    None => {
                        (block_id, environment) = transition(state, environment, failure);
                    }
                }
            }
            Terminator::Return(value) => {
                let value = value.read(&environment);
                drop(environment);
                state.drain_releases();
                return Ok(GraphExit::Return(value));
            }
            Terminator::TailCall { function, args } => {
                let args = environment.retain(args);
                let function = function.clone();
                drop(environment);
                state.drain_releases();
                return Ok(GraphExit::TailCall { function, args });
            }
            Terminator::SourceStop {
                kind,
                message,
                site,
            } => {
                let message = message.map(|message| environment.string(message));
                return Err(ExecutionError::source_panic(
                    plan.source_context(),
                    panic_kind(*kind),
                    message,
                    site.clone(),
                ));
            }
            Terminator::LetAssertPanic {
                subject,
                message,
                site,
                pattern_span,
            } => {
                let subject = environment.value(subject);
                let message = message.map(|message| environment.string(message));
                let subject = crate::runtime::materialize::value(plan, state, subject);
                return Err(ExecutionError::let_assert_panic(
                    plan.source_context(),
                    message,
                    site.clone(),
                    subject,
                    *pattern_span,
                ));
            }
            Terminator::NeverCall { function, args } => {
                let args = environment.retain(args);
                match function {
                    crate::plan::execution::NeverCallTarget::Direct(function) => {
                        let function = *function;
                        drop(environment);
                        state.drain_releases();
                        return function::run_never(plan, state, function, args)
                            .map(|never| match never {});
                    }
                    crate::plan::execution::NeverCallTarget::Value(function) => {
                        let function = environment.never_function(function);
                        drop(environment);
                        state.drain_releases();
                        return function::run_never_value(plan, state, function, args)
                            .map(|never| match never {});
                    }
                }
            }
        }
    }
}

fn transition(
    state: &mut RuntimeState,
    environment: BlockEnvironment,
    edge: &Edge,
) -> (BlockId, BlockEnvironment) {
    let values = environment.retain(edge.args());
    drop(environment);
    state.drain_releases();
    (edge.target(), BlockEnvironment::from_retained(values))
}

fn transition_match(
    state: &mut RuntimeState,
    environment: BlockEnvironment,
    edge: &MatchEdge,
    bindings: pattern::MatchBindings,
) -> (BlockId, BlockEnvironment) {
    let mut values = RetainedValues::empty();
    for argument in edge.args() {
        match argument {
            MatchEdgeArgument::Binding(index) => {
                values.push_evaluated(bindings.value(*index));
            }
            MatchEdgeArgument::Value(local) => {
                values.push_evaluated(environment.value(local));
            }
        }
    }
    drop(bindings);
    drop(environment);
    state.drain_releases();
    (edge.target(), BlockEnvironment::from_retained(values))
}

fn panic_kind(kind: SourceStopKind) -> PanicKind {
    match kind {
        SourceStopKind::Panic => PanicKind::Panic,
        SourceStopKind::Todo => PanicKind::Todo,
        SourceStopKind::Assert => PanicKind::Assert,
        SourceStopKind::EmptyFunction => PanicKind::EmptyFunction,
        SourceStopKind::EmptyBlock => PanicKind::EmptyBlock,
        SourceStopKind::IncompleteUse => PanicKind::IncompleteUse,
    }
}

impl GraphValue for GraphNeverReturn {
    type Evaluated = Infallible;

    fn read(&self, _environment: &BlockEnvironment) -> Self::Evaluated {
        match *self {}
    }
}

impl GraphValue for crate::plan::execution::IntLocalId {
    type Evaluated = BigInt;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.int(*self)
    }
}

impl GraphValue for crate::plan::execution::FloatLocalId {
    type Evaluated = f64;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.float(*self)
    }
}

impl GraphValue for crate::plan::execution::StringLocalId {
    type Evaluated = EcoString;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.string(*self)
    }
}

impl GraphValue for crate::plan::execution::BitArrayLocalId {
    type Evaluated = crate::runtime::EvaluatedBitArray;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.bit_array(*self)
    }
}

impl GraphValue for crate::plan::execution::UtfCodepointLocalId {
    type Evaluated = char;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.utf_codepoint(*self)
    }
}

impl GraphValue for crate::plan::execution::CustomLocal {
    type Evaluated = crate::runtime::EvaluatedCustomValue;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.custom(*self)
    }
}

impl GraphValue for crate::plan::execution::BoolLocalId {
    type Evaluated = bool;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.bool(*self)
    }
}

impl GraphValue for crate::plan::execution::NilLocalId {
    type Evaluated = ();

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.nil(*self)
    }
}

impl GraphValue for crate::plan::execution::TupleLocalId {
    type Evaluated = Vec<EvaluatedValue>;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.tuple(*self)
    }
}

macro_rules! list_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;

            fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
                environment.$method(*self)
            }
        }
    };
}

list_graph_value!(
    crate::plan::execution::ParameterListLocalId,
    crate::runtime::state::ParameterListValueId,
    parameter_list
);
list_graph_value!(
    crate::plan::execution::IntListLocalId,
    crate::runtime::state::IntListValueId,
    int_list
);
list_graph_value!(
    crate::plan::execution::StringListLocalId,
    crate::runtime::state::StringListValueId,
    string_list
);
list_graph_value!(
    crate::plan::execution::BitArrayListLocalId,
    crate::runtime::state::BitArrayListValueId,
    bit_array_list
);
list_graph_value!(
    crate::plan::execution::UtfCodepointListLocalId,
    crate::runtime::state::UtfCodepointListValueId,
    utf_codepoint_list
);
list_graph_value!(
    crate::plan::execution::CustomListLocalId,
    crate::runtime::state::CustomListValueId,
    custom_list
);
list_graph_value!(
    crate::plan::execution::FloatListLocalId,
    crate::runtime::state::FloatListValueId,
    float_list
);
list_graph_value!(
    crate::plan::execution::BoolListLocalId,
    crate::runtime::state::BoolListValueId,
    bool_list
);
list_graph_value!(
    crate::plan::execution::NilListLocalId,
    crate::runtime::state::NilListValueId,
    nil_list
);
list_graph_value!(
    crate::plan::execution::TupleListLocalId,
    crate::runtime::state::TupleListValueId,
    tuple_list
);
list_graph_value!(
    crate::plan::execution::ParameterListListLocalId,
    crate::runtime::state::ParameterListListValueId,
    parameter_list_list
);
list_graph_value!(
    crate::plan::execution::ListListLocalId,
    crate::runtime::state::ListListValueId,
    list_list
);
list_graph_value!(
    crate::plan::execution::FunctionListLocalId,
    crate::runtime::state::FunctionListValueId,
    function_list
);

macro_rules! function_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;

            fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
                environment.$method(self.clone())
            }
        }
    };
}

function_graph_value!(
    crate::plan::execution::IntFunctionLocalId,
    crate::runtime::EvaluatedIntFunction,
    int_function
);
function_graph_value!(
    crate::plan::execution::FloatFunctionLocalId,
    crate::runtime::EvaluatedFloatFunction,
    float_function
);
function_graph_value!(
    crate::plan::execution::StringFunctionLocalId,
    crate::runtime::EvaluatedStringFunction,
    string_function
);
function_graph_value!(
    crate::plan::execution::BitArrayFunctionLocalId,
    crate::runtime::EvaluatedBitArrayFunction,
    bit_array_function
);
function_graph_value!(
    crate::plan::execution::UtfCodepointFunctionLocalId,
    crate::runtime::EvaluatedUtfCodepointFunction,
    utf_codepoint_function
);
function_graph_value!(
    crate::plan::execution::BoolFunctionLocalId,
    crate::runtime::EvaluatedBoolFunction,
    bool_function
);
function_graph_value!(
    crate::plan::execution::NilFunctionLocalId,
    crate::runtime::EvaluatedNilFunction,
    nil_function
);
function_graph_value!(
    crate::plan::execution::TupleFunctionLocalId,
    crate::runtime::EvaluatedTupleFunction,
    tuple_function
);

impl GraphValue for crate::plan::execution::GenericFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedGenericFunction;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.generic_function(self)
    }
}

impl GraphValue for crate::plan::execution::NeverFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedNeverFunction;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.never_function(self)
    }
}

impl GraphValue for crate::plan::execution::CustomFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedCustomFunction;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.custom_function(self)
    }
}

impl GraphValue for crate::plan::execution::ListFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedListFunction;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.list_function(self)
    }
}

impl GraphValue for crate::plan::execution::FunctionFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedFunctionFunction;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        environment.function_function(self)
    }
}

impl GraphValue for crate::plan::execution::FunctionLocal {
    type Evaluated = EvaluatedFunctionValue;

    fn read(&self, environment: &BlockEnvironment) -> Self::Evaluated {
        match self {
            Self::Generic(local) => environment.generic_function(local).into(),
            Self::Never(local) => environment.never_function(local).into(),
            Self::Int(local) => environment.int_function(*local).into(),
            Self::Float(local) => environment.float_function(*local).into(),
            Self::String(local) => environment.string_function(*local).into(),
            Self::BitArray(local) => environment.bit_array_function(*local).into(),
            Self::UtfCodepoint(local) => environment.utf_codepoint_function(*local).into(),
            Self::Custom(local) => environment.custom_function(local).into(),
            Self::Bool(local) => environment.bool_function(*local).into(),
            Self::Nil(local) => environment.nil_function(*local).into(),
            Self::Tuple(local) => environment.tuple_function(*local).into(),
            Self::List(local) => environment.list_function(local).into(),
            Self::Function(local) => environment.function_function(local).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ValueType;
    use crate::plan::execution::IntFunctionId;
    use crate::runtime::environment::RetainedValues;
    use crate::runtime::error::InvariantError;
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{ExecutionError, Value};

    #[test]
    fn deeply_nested_intra_function_control_flow_runs_iteratively() {
        let mut body = "1".to_string();
        for _ in 0..512 {
            body = format!("case flag {{ True -> {body} False -> 0 }}");
        }
        let source =
            format!("fn deep(flag: Bool) -> Int {{ {body} }} pub fn main() {{ deep(True) }}");
        let plan = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let module = crate::compile_typed_module("main", "main.gleam", source.as_str())
                    .expect("deep source should compile");
                let module_plan = crate::plan_module(module).expect("deep source should plan");
                crate::ExecutionPlan::from_module_plan(module_plan)
            })
            .expect("deep-plan lowering thread should start")
            .join()
            .expect("deep-plan lowering should complete");

        assert_eq!(crate::run_main(&plan), Ok(Value::Int(1.into())));
    }

    #[test]
    fn match_terminator_propagates_custom_field_family_corruption() {
        let plan = execution_plan(
            r#"
pub type Boxed {
  Boxed(Int)
  Empty
}

fn read(value: Boxed) {
  let assert Boxed(inner) = value
  inner
}

pub fn main() {
  read(Boxed(1))
}
"#,
        );
        let constructor = plan.custom_constructor_id(0, 0);
        let descriptor = plan.custom_constructor(constructor);
        let malformed = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
        );
        let mut inputs = RetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Custom(malformed));

        assert_eq!(
            super::evaluate(
                &plan,
                &mut RuntimeState::new(),
                plan.int_function(IntFunctionId(1)).graph(),
                inputs,
            )
            .map(|_| ()),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(constructor.type_id()),
                    constructor: descriptor.name().clone(),
                    field_index: 0,
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let module = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(module).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
