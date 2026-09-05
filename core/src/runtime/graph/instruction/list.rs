use super::super::environment::BlockEnvironment;
use super::super::{GraphValue, RuntimeGraphState};
use super::value::{
    InstructionValue, constant, custom_projection, ensure_list_index, tuple_projection,
};
use crate::plan::ValueType;
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
    FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, ListFunctionId,
    ListListFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    RuntimeListFunctionId, StringListFunctionId, TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, ExternalListFunctionLocalId,
    ExternalListInstruction, ExternalListInstructionView, ExternalListLocalId, FloatListLocalId,
    FunctionListLocalId, IntListLocalId, ListFunctionLocal, ListInstruction, ListListLocalId,
    NilListLocalId, ParameterListInstruction, ParameterListListLocalId, ParameterListLocalId,
    StoredListLocal, StringListLocalId, TupleListLocalId, TypedListInstruction,
    UtfCodepointListLocalId,
};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalListFunction, EvaluatedExternalValue,
    EvaluatedFunctionValue, EvaluatedListFunction, EvaluatedValue,
};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    ExternalListAllocation, ExternalListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StoredListValueId, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use crate::runtime::{InvariantError, RuntimeListStorage, RuntimeValueProfile};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) enum ListInstructionValue<Profile: RuntimeValueProfile> {
    Parameter(
        InstructionValue<
            Profile,
            ParameterListValueId<Profile>,
            ParameterListFunctionId,
            ParameterListLocalId,
        >,
    ),
    ParameterList(
        InstructionValue<
            Profile,
            ParameterListListValueId<Profile>,
            ParameterListListFunctionId,
            ParameterListListLocalId,
        >,
    ),
    Int(InstructionValue<Profile, IntListValueId<Profile>, IntListFunctionId, IntListLocalId>),
    String(
        InstructionValue<
            Profile,
            StringListValueId<Profile>,
            StringListFunctionId,
            StringListLocalId,
        >,
    ),
    BitArray(
        InstructionValue<
            Profile,
            BitArrayListValueId<Profile>,
            BitArrayListFunctionId,
            BitArrayListLocalId,
        >,
    ),
    UtfCodepoint(
        InstructionValue<
            Profile,
            UtfCodepointListValueId<Profile>,
            UtfCodepointListFunctionId,
            UtfCodepointListLocalId,
        >,
    ),
    Custom(
        InstructionValue<
            Profile,
            CustomListValueId<Profile>,
            CustomListFunctionId,
            CustomListLocalId,
        >,
    ),
    Float(
        InstructionValue<Profile, FloatListValueId<Profile>, FloatListFunctionId, FloatListLocalId>,
    ),
    Bool(InstructionValue<Profile, BoolListValueId<Profile>, BoolListFunctionId, BoolListLocalId>),
    Nil(InstructionValue<Profile, NilListValueId<Profile>, NilListFunctionId, NilListLocalId>),
    Tuple(
        InstructionValue<Profile, TupleListValueId<Profile>, TupleListFunctionId, TupleListLocalId>,
    ),
    List(InstructionValue<Profile, ListListValueId<Profile>, ListListFunctionId, ListListLocalId>),
    Function(
        InstructionValue<
            Profile,
            FunctionListValueId<Profile>,
            FunctionListFunctionId,
            FunctionListLocalId,
        >,
    ),
}

pub(in crate::runtime) type ExternalListInstructionValue<Profile> = InstructionValue<
    Profile,
    ExternalListValueId<Profile>,
    ExternalListFunctionId,
    ExternalListLocalId,
>;

pub(in crate::runtime) fn evaluate<Plan, Profile, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment<Profile>,
    instruction: &ListInstruction,
    expected: &ValueType,
) -> Result<ListInstructionValue<Profile>, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    Profile: RuntimeValueProfile,
    State: RuntimeGraphState<Profile>,
{
    use ListInstructionValue as V;

    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            parameter(plan, state, environment, *type_id, instruction, expected).map(V::Parameter)
        }
        ListInstruction::ParameterList(type_id, instruction) => {
            typed::<ParameterListFamily, _, _, _>(
                plan,
                state,
                environment,
                *type_id,
                instruction,
                expected,
            )
            .map(V::ParameterList)
        }
        ListInstruction::Int(type_id, instruction) => {
            typed::<IntFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Int)
        }
        ListInstruction::String(type_id, instruction) => typed::<StringFamily, _, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::String),
        ListInstruction::BitArray(type_id, instruction) => typed::<BitArrayFamily, _, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::BitArray),
        ListInstruction::UtfCodepoint(type_id, instruction) => {
            typed::<UtfCodepointFamily, _, _, _>(
                plan,
                state,
                environment,
                *type_id,
                instruction,
                expected,
            )
            .map(V::UtfCodepoint)
        }
        ListInstruction::Custom(type_id, instruction) => typed::<CustomFamily, _, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::Custom),
        ListInstruction::Float(type_id, instruction) => {
            typed::<FloatFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Float)
        }
        ListInstruction::Bool(type_id, instruction) => {
            typed::<BoolFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Bool)
        }
        ListInstruction::Nil(type_id, instruction) => {
            typed::<NilFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Nil)
        }
        ListInstruction::Tuple(type_id, instruction) => {
            typed::<TupleFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Tuple)
        }
        ListInstruction::List(type_id, instruction) => {
            typed::<ListFamily, _, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::List)
        }
        ListInstruction::Function(type_id, instruction) => typed::<FunctionFamily, _, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::Function),
    }
}

pub(in crate::runtime) fn evaluate_external<Plan, Profile, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment<Profile>,
    instruction: &ExternalListInstruction,
    expected: &ValueType,
) -> Result<ExternalListInstructionValue<Profile>, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    Profile: RuntimeValueProfile,
    State: RuntimeGraphState<Profile>,
{
    typed::<ExternalFamily, _, _, _>(
        plan,
        state,
        environment,
        instruction.type_id(),
        instruction.instruction(),
        expected,
    )
}

pub(super) fn execute<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &ListInstruction,
    expected: &ValueType,
) -> ExecutionResult<()> {
    evaluate(plan, state, environment, instruction, expected)
        .and_then(|value| resolve(plan, state, environment, value))
}

pub(super) fn execute_external<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &ExternalListInstruction,
    expected: &ValueType,
) -> ExecutionResult<()> {
    evaluate_external(plan, state, environment, instruction, expected)
        .and_then(|value| match value {
            InstructionValue::Ready(value) => Ok(value),
            InstructionValue::Constant(id) => constant(plan, state, id),
            InstructionValue::Call {
                function,
                origin,
                inputs,
            } => crate::runtime::function::run_external_list(plan, state, function, origin, inputs),
        })
        .map(|value| environment.push_external_list(value))
}

fn resolve<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    value: ListInstructionValue<crate::runtime::LocalValues>,
) -> ExecutionResult<()> {
    macro_rules! resolve_value {
        ($value:expr, $run:ident, $push:ident) => {{
            match $value {
                InstructionValue::Ready(value) => {
                    environment.$push(value);
                    Ok(())
                }
                InstructionValue::Constant(id) => constant(plan, state, id).map(|value| {
                    environment.$push(value);
                }),
                InstructionValue::Call {
                    function,
                    origin,
                    inputs,
                } => crate::runtime::function::$run(plan, state, function, origin, inputs).map(
                    |value| {
                        environment.$push(value);
                    },
                ),
            }
        }};
    }

    match value {
        ListInstructionValue::Parameter(value) => {
            resolve_value!(value, run_parameter_list, push_parameter_list)
        }
        ListInstructionValue::ParameterList(value) => {
            resolve_value!(value, run_parameter_list_list, push_parameter_list_list)
        }
        ListInstructionValue::Int(value) => resolve_value!(value, run_int_list, push_int_list),
        ListInstructionValue::String(value) => {
            resolve_value!(value, run_string_list, push_string_list)
        }
        ListInstructionValue::BitArray(value) => {
            resolve_value!(value, run_bit_array_list, push_bit_array_list)
        }
        ListInstructionValue::UtfCodepoint(value) => {
            resolve_value!(value, run_utf_codepoint_list, push_utf_codepoint_list)
        }
        ListInstructionValue::Custom(value) => {
            resolve_value!(value, run_custom_list, push_custom_list)
        }
        ListInstructionValue::Float(value) => {
            resolve_value!(value, run_float_list, push_float_list)
        }
        ListInstructionValue::Bool(value) => resolve_value!(value, run_bool_list, push_bool_list),
        ListInstructionValue::Nil(value) => resolve_value!(value, run_nil_list, push_nil_list),
        ListInstructionValue::Tuple(value) => {
            resolve_value!(value, run_tuple_list, push_tuple_list)
        }
        ListInstructionValue::List(value) => resolve_value!(value, run_list_list, push_list_list),
        ListInstructionValue::Function(value) => {
            resolve_value!(value, run_function_list, push_function_list)
        }
    }
}

fn parameter<Plan, Profile, State>(
    plan: &Plan,
    state: &State,
    environment: &BlockEnvironment<Profile>,
    type_id: ParameterListTypeId,
    instruction: &ParameterListInstruction,
    expected: &ValueType,
) -> Result<
    InstructionValue<
        Profile,
        ParameterListValueId<Profile>,
        ParameterListFunctionId,
        ParameterListLocalId,
    >,
    State::Error,
>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    Profile: RuntimeValueProfile,
    State: RuntimeGraphState<Profile>,
{
    use InstructionValue as V;
    use ParameterListInstruction as I;

    match instruction {
        I::Empty => Ok(V::Ready(ParameterListValueId::new(type_id))),
        I::Constant(id) => Ok(V::Constant(*id)),
        I::Call {
            function,
            args,
            site,
        } => Ok(V::Call {
            function: *function,
            origin: HostCallOrigin::source(site.clone()),
            inputs: environment.retain(args),
        }),
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.list_function(function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(function.captures());
            match function.runtime_id() {
                RuntimeListFunctionId::Core(ListFunctionId::Parameter(function)) => Ok(V::Call {
                    function,
                    origin: HostCallOrigin::source(site.clone()),
                    inputs,
                }),
                _ => Err(list_function_mismatch().into()),
            }
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan.value_metadata(),
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::ParameterList(value) => Some(*value),
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
                EvaluatedValue::ParameterList(value) => Some(*value),
                _ => None,
            },
        )
        .map(V::Ready),
        I::ListIndex { list, index } => {
            let length = state
                .lists()
                .parameter_list_list_len(&environment.parameter_list_list(*list));
            ensure_list_index(expected, *index, length)
                .map(|()| V::Ready(ParameterListValueId::new(type_id)))
        }
    }
}

trait RuntimeTypedList<Profile: RuntimeValueProfile> {
    type TypeId: Copy;
    type ElementLocal;
    type Element: Clone;
    type Local: Copy
        + crate::plan::execution::constant::ConstantValue
        + GraphValue<Profile, Evaluated = Self::Handle>;
    type Function: Clone;
    type FunctionLocal;
    type FunctionValue: Clone;
    type Handle: Clone;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element;
    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle;
    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue;
    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>];
    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError>;
    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element>;
    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle;
    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle>;
    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle;
}

type TypedListInstructionValue<Family, Profile> = InstructionValue<
    Profile,
    <Family as RuntimeTypedList<Profile>>::Handle,
    <Family as RuntimeTypedList<Profile>>::Function,
    <Family as RuntimeTypedList<Profile>>::Local,
>;

fn typed<Family, Plan, Profile, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment<Profile>,
    type_id: Family::TypeId,
    instruction: &TypedListInstruction<
        Family::ElementLocal,
        Family::Local,
        Family::Function,
        Family::FunctionLocal,
    >,
    expected: &ValueType,
) -> Result<TypedListInstructionValue<Family, Profile>, State::Error>
where
    Family: RuntimeTypedList<Profile>,
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    Profile: RuntimeValueProfile,
    State: RuntimeGraphState<Profile>,
{
    use InstructionValue as V;
    use TypedListInstruction as I;

    match instruction {
        I::Value(elements) => Ok(V::Ready(Family::allocate(
            state,
            type_id,
            elements
                .iter()
                .map(|element| Family::element(environment, element))
                .collect(),
        ))),
        I::Constant(id) => Ok(V::Constant(*id)),
        I::Spread { elements, tail } => {
            let mut values = elements
                .iter()
                .map(|element| Family::element(environment, element))
                .collect::<Vec<_>>();
            values.extend(Family::values(state, &Family::local(environment, *tail)));
            Ok(V::Ready(Family::allocate(state, type_id, values)))
        }
        I::Call {
            function,
            args,
            site,
        } => Ok(V::Call {
            function: function.clone(),
            origin: HostCallOrigin::source(site.clone()),
            inputs: environment.retain(args),
        }),
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = Family::function(environment, function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(Family::captures(&function));
            Family::function_id(&function)
                .map(|function| V::Call {
                    function,
                    origin: HostCallOrigin::source(site.clone()),
                    inputs,
                })
                .map_err(Into::into)
        }
        I::TupleIndex { tuple, index } => tuple_projection(
            plan.value_metadata(),
            environment,
            *tuple,
            *index,
            expected,
            |value| match value {
                EvaluatedValue::List(value) => Family::projected(value),
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
                EvaluatedValue::List(value) => Family::projected(value),
                _ => None,
            },
        )
        .map(V::Ready),
        I::ListIndex { list, index } => {
            let list = environment.list_list(*list);
            let values = state.lists().list_values(&list);
            match values.get(*index) {
                Some(value) => Ok(V::Ready(Family::from_core(
                    type_id,
                    value.clone().into_core(),
                ))),
                None => Err(InvariantError::ListIndexOutOfBounds {
                    item_type: expected.clone(),
                    index: *index,
                    length: values.len(),
                }
                .into()),
            }
        }
        I::DropFirst { list, count } => {
            let values = Family::values(state, &Family::local(environment, *list));
            let values = values[(*count).min(values.len())..].to_vec();
            Ok(V::Ready(Family::allocate(state, type_id, values)))
        }
    }
}

fn list_function_mismatch() -> InvariantError {
    InvariantError::FunctionReturnFamilyMismatch {
        expected: crate::plan::execution::function::FunctionReturnFamily::List,
        actual: crate::plan::execution::function::FunctionReturnFamily::List,
    }
}

macro_rules! vector_family {
    (
        $family:ident,
        $type_id:ty,
        $element_local:ty,
        $element:ty,
        $local:ty,
        $function:ty,
        $handle:ident,
        $function_variant:ident,
        $element_method:ident,
        $local_method:ident,
        $values_method:ident,
        $allocate_method:ident
    ) => {
        struct $family;

        impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for $family {
            type TypeId = $type_id;
            type ElementLocal = $element_local;
            type Element = $element;
            type Local = $local;
            type Function = $function;
            type FunctionLocal = ListFunctionLocal;
            type FunctionValue = EvaluatedListFunction<Profile>;
            type Handle = $handle<Profile>;

            fn element(
                environment: &BlockEnvironment<Profile>,
                local: &Self::ElementLocal,
            ) -> Self::Element {
                environment.$element_method(*local)
            }

            fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
                environment.$local_method(local)
            }

            fn function(
                environment: &BlockEnvironment<Profile>,
                local: &Self::FunctionLocal,
            ) -> Self::FunctionValue {
                environment.list_function(local)
            }

            fn captures(
                function: &Self::FunctionValue,
            ) -> &[crate::runtime::EvaluatedCapture<Profile>] {
                function.captures()
            }

            fn function_id(
                function: &Self::FunctionValue,
            ) -> Result<Self::Function, InvariantError> {
                match function.runtime_id() {
                    RuntimeListFunctionId::Core(ListFunctionId::$function_variant(function)) => {
                        Ok(function)
                    }
                    _ => Err(list_function_mismatch()),
                }
            }

            fn values<State: RuntimeGraphState<Profile>>(
                state: &State,
                value: &Self::Handle,
            ) -> Vec<Self::Element> {
                state.lists().$values_method(value).to_vec()
            }

            fn allocate<State: RuntimeGraphState<Profile>>(
                state: &mut State,
                type_id: Self::TypeId,
                values: Vec<Self::Element>,
            ) -> Self::Handle {
                state.lists_mut().$allocate_method(type_id, values)
            }

            fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
                <$handle<Profile>>::from_stored(value)
            }

            fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
                <$handle<Profile>>::new(type_id, core)
            }
        }
    };
}

vector_family!(
    IntFamily,
    IntListTypeId,
    crate::plan::execution::graph::IntLocalId,
    BigInt,
    IntListLocalId,
    IntListFunctionId,
    IntListValueId,
    Int,
    int,
    int_list,
    int_values,
    int
);
vector_family!(
    StringFamily,
    StringListTypeId,
    crate::plan::execution::graph::StringLocalId,
    EcoString,
    StringListLocalId,
    StringListFunctionId,
    StringListValueId,
    String,
    string,
    string_list,
    string_values,
    string
);
vector_family!(
    BitArrayFamily,
    BitArrayListTypeId,
    crate::plan::execution::graph::BitArrayLocalId,
    EvaluatedBitArray,
    BitArrayListLocalId,
    BitArrayListFunctionId,
    BitArrayListValueId,
    BitArray,
    bit_array,
    bit_array_list,
    bit_array_values,
    bit_array
);
vector_family!(
    UtfCodepointFamily,
    UtfCodepointListTypeId,
    crate::plan::execution::graph::UtfCodepointLocalId,
    char,
    UtfCodepointListLocalId,
    UtfCodepointListFunctionId,
    UtfCodepointListValueId,
    UtfCodepoint,
    utf_codepoint,
    utf_codepoint_list,
    utf_codepoint_values,
    utf_codepoint
);
vector_family!(
    FloatFamily,
    FloatListTypeId,
    crate::plan::execution::graph::FloatLocalId,
    f64,
    FloatListLocalId,
    FloatListFunctionId,
    FloatListValueId,
    Float,
    float,
    float_list,
    float_values,
    float
);
vector_family!(
    BoolFamily,
    BoolListTypeId,
    crate::plan::execution::graph::BoolLocalId,
    bool,
    BoolListLocalId,
    BoolListFunctionId,
    BoolListValueId,
    Bool,
    bool,
    bool_list,
    bool_values,
    bool
);

struct TupleFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for TupleFamily {
    type TypeId = TupleListTypeId;
    type ElementLocal = crate::plan::execution::graph::TupleLocalId;
    type Element = Vec<EvaluatedValue<Profile>>;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = TupleListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.tuple(*local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.tuple_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Tuple(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        state.lists().tuple_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().tuple(type_id, values)
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        TupleListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        TupleListValueId::new(type_id, core)
    }
}

struct CustomFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for CustomFamily {
    type TypeId = CustomListTypeId;
    type ElementLocal = crate::plan::execution::graph::CustomLocal;
    type Element = EvaluatedCustomValue<Profile>;
    type Local = CustomListLocalId;
    type Function = CustomListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = CustomListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.custom(*local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.custom_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Custom(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        state.lists().custom_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .custom(CustomListAllocation::new(type_id, values))
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        CustomListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        CustomListValueId::new(type_id, core)
    }
}

struct ExternalFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for ExternalFamily {
    type TypeId = ExternalListTypeId;
    type ElementLocal = crate::plan::execution::graph::ExternalLocal;
    type Element = EvaluatedExternalValue<Profile>;
    type Local = ExternalListLocalId;
    type Function = ExternalListFunctionId;
    type FunctionLocal = ExternalListFunctionLocalId;
    type FunctionValue = EvaluatedExternalListFunction<Profile>;
    type Handle = ExternalListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.external(*local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.external_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.external_list_function(*local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        Ok(function.runtime_id())
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        state.lists().external_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .external(ExternalListAllocation::new(type_id, values))
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        ExternalListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        ExternalListValueId::new(type_id, core)
    }
}

struct NilFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for NilFamily {
    type TypeId = NilListTypeId;
    type ElementLocal = crate::plan::execution::graph::NilLocalId;
    type Element = ();
    type Local = NilListLocalId;
    type Function = NilListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = NilListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.nil(*local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.nil_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Nil(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        vec![(); state.lists().nil_len(value)]
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().nil(type_id, values.len())
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        NilListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        NilListValueId::new(type_id, core)
    }
}

struct ParameterListFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for ParameterListFamily {
    type TypeId = ParameterListListTypeId;
    type ElementLocal = ParameterListLocalId;
    type Element = ParameterListValueId<Profile>;
    type Local = ParameterListListLocalId;
    type Function = ParameterListListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = ParameterListListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.parameter_list(*local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.parameter_list_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::ParameterList(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        vec![
            ParameterListValueId::new(value.type_id().item_type());
            state.lists().parameter_list_list_len(value)
        ]
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().parameter_list_list(type_id, values.len())
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        ParameterListListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        ParameterListListValueId::new(type_id, core)
    }
}

struct ListFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for ListFamily {
    type TypeId = ListListTypeId;
    type ElementLocal = StoredListLocal;
    type Element = StoredListValueId<Profile>;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = ListListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.stored_list(local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.list_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::List(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        state.lists().list_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().list(type_id, values)
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        ListListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        ListListValueId::new(type_id, core)
    }
}

struct FunctionFamily;

impl<Profile: RuntimeValueProfile> RuntimeTypedList<Profile> for FunctionFamily {
    type TypeId = FunctionListTypeId;
    type ElementLocal = crate::plan::execution::graph::FunctionLocal;
    type Element = EvaluatedFunctionValue<Profile>;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction<Profile>;
    type Handle = FunctionListValueId<Profile>;

    fn element(
        environment: &BlockEnvironment<Profile>,
        local: &Self::ElementLocal,
    ) -> Self::Element {
        environment.function_value(local)
    }

    fn local(environment: &BlockEnvironment<Profile>, local: Self::Local) -> Self::Handle {
        environment.function_list(local)
    }

    fn function(
        environment: &BlockEnvironment<Profile>,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture<Profile>] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Function(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState<Profile>>(
        state: &State,
        value: &Self::Handle,
    ) -> Vec<Self::Element> {
        state.lists().function_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState<Profile>>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().function(type_id, values)
    }

    fn projected(value: &StoredListValueId<Profile>) -> Option<Self::Handle> {
        FunctionListValueId::<Profile>::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: Profile::ListHandle) -> Self::Handle {
        FunctionListValueId::new(type_id, core)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::environment::{
        BlockEnvironment, ProfiledRetainedValues, RetainedValues,
    };
    use super::{
        BitArrayFamily, BoolFamily, CustomFamily, ExternalFamily, FloatFamily, FunctionFamily,
        IntFamily, ListFamily, NilFamily, ParameterListFamily, RuntimeTypedList, StringFamily,
        TupleFamily, UtfCodepointFamily, execute, list_function_mismatch, parameter, typed,
    };
    use crate::host::{
        AsyncHostModule, AsyncHostProviderModule, AsyncHostProviderSet, HostExternalSchema,
    };
    use crate::plan::execution::function::{
        ExecutionFunctionEntry, ExecutionFunctionRef, ExternalListFunctionId, FunctionBodyOwner,
        ListFunctionId, RuntimeListFunctionId,
    };
    use crate::plan::execution::graph::{
        CustomLocal, ExternalListFunctionLocalId, ExternalListLocalId, ExternalLocal,
        IntListFunctionLocalId, ListFunctionLocal, ListInstruction, ListListLocalId,
        ParameterListInstruction, ParameterListListLocalId, Terminator, TupleLocalId,
        TypedListInstruction,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::{IntListTypeId, ListListTypeId, StringListTypeId};
    use crate::plan::{
        CustomType, CustomTypeName, FunctionType, LibraryEntry, LibraryValueType, TypeParameterId,
        ValueType,
    };
    use crate::runtime::resumable::{RecordedEcho, ResumableFuture, ResumableState, run_custom};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::ListValueId;
    use crate::runtime::{
        EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedListFunction, EvaluatedValue,
        ExecutionError, InvariantError, RuntimeValueProfile, TransferExecutionError,
        TransferValues,
    };
    use crate::{ModuleSource, PackageSource, compile_typed_async_host_program};
    use std::task::{Context, Poll, Waker};

    const LIST_FUNCTION_FAMILY_SOURCE: &str = r#"
pub type Boxed { Boxed(Int) }

fn ints() -> List(Int) { [] }
fn nils() -> List(Nil) { [] }
fn customs() -> List(Boxed) { [] }
fn nested() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
fn parameters() -> List(value) { [] }
fn parameter_lists() -> List(List(value)) { [] }

pub fn main() {
  let _ = #(ints, nils, customs, nested, functions, parameters, parameter_lists)
  0
}
"#;

    fn execution_error<T, Error>(result: Result<T, Error>, message: &str) -> Error {
        result.err().expect(message)
    }

    fn evaluated_int_list_function<Profile: RuntimeValueProfile>(
        plan: &crate::ExecutionPlan,
    ) -> EvaluatedListFunction<Profile> {
        let function = plan.int_list_function_id(0);
        EvaluatedListFunction::reference(
            RuntimeListFunctionId::Core(ListFunctionId::Int(function)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::List(function.type_id().list_type()),
            ),
        )
    }

    fn evaluated_nil_list_function<Profile: RuntimeValueProfile>(
        plan: &crate::ExecutionPlan,
    ) -> EvaluatedListFunction<Profile> {
        let function = plan.nil_list_function_id(0);
        EvaluatedListFunction::reference(
            RuntimeListFunctionId::Core(ListFunctionId::Nil(function)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::List(function.type_id().list_type()),
            ),
        )
    }

    fn assert_list_function_mismatch<Profile, Family>(function: EvaluatedListFunction<Profile>)
    where
        Profile: RuntimeValueProfile,
        Family: RuntimeTypedList<Profile, FunctionValue = EvaluatedListFunction<Profile>>,
        Family::Handle: std::fmt::Debug + PartialEq,
    {
        assert_eq!(
            execution_error(
                Family::function_id(&function),
                "a list function must reject a different item family",
            ),
            list_function_mismatch(),
        );
    }

    fn assert_every_list_function_mismatch<Profile: RuntimeValueProfile>(
        plan: &crate::ExecutionPlan,
    ) {
        let wrong_int = evaluated_int_list_function::<Profile>(plan);

        assert_list_function_mismatch::<Profile, IntFamily>(
            evaluated_nil_list_function::<Profile>(plan),
        );
        assert_list_function_mismatch::<Profile, StringFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, BitArrayFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, UtfCodepointFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, CustomFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, FloatFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, BoolFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, NilFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, TupleFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, ParameterListFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, ListFamily>(wrong_int.clone());
        assert_list_function_mismatch::<Profile, FunctionFamily>(wrong_int);
    }

    #[test]
    fn list_function_value_dispatch_rejects_every_wrong_item_family() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);

        assert_every_list_function_mismatch::<crate::runtime::LocalValues>(&plan);
        assert_every_list_function_mismatch::<TransferValues>(&plan);
    }

    #[test]
    fn parameter_list_function_call_rejects_a_wrong_list_family() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);
        let int_function = evaluated_int_list_function::<crate::runtime::LocalValues>(&plan);
        let int_function_id = plan.int_list_function_id(0);
        let mut retained = RetainedValues::empty();
        retained.push_evaluated(EvaluatedValue::Function(EvaluatedFunctionValue::from(
            int_function,
        )));
        let environment = BlockEnvironment::from_retained(retained);
        let instruction = ParameterListInstruction::FunctionCall {
            function: ListFunctionLocal::Int {
                local: IntListFunctionLocalId(0),
                type_: crate::plan::execution::type_::FunctionType::new(
                    Vec::new(),
                    crate::plan::execution::type_::ValueType::List(
                        int_function_id.type_id().list_type(),
                    ),
                ),
                list_type: int_function_id.type_id(),
            },
            args: Box::new([]),
            site: crate::plan::HostCallSite::unknown(),
        };

        assert_eq!(
            execution_error(
                parameter(
                    &plan,
                    &RuntimeState::new(&mut Vec::new()),
                    &environment,
                    plan.parameter_list_function_id(0).type_id(),
                    &instruction,
                    &ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
                ),
                "a parameter list call must reject a different list family",
            ),
            ExecutionError::from(list_function_mismatch()),
        );
    }

    #[test]
    fn nested_parameter_list_instruction_preserves_symbolic_item_type() {
        assert_eq!(
            crate::runtime::run_src("pub fn main() -> List(List(value)) { [[]] }").value_type(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                TypeParameterId(0),
            ))))),
        );
    }

    #[test]
    fn nested_parameter_list_dispatch_propagates_projection_invariants() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);
        let type_id = plan.parameter_list_list_function_id(0).type_id();
        let mut retained = RetainedValues::empty();
        retained.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
        let mut environment = BlockEnvironment::from_retained(retained);
        let instruction = ListInstruction::ParameterList(
            type_id,
            TypedListInstruction::TupleIndex {
                tuple: TupleLocalId(0),
                index: 0,
            },
        );
        let expected = ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
            TypeParameterId(0),
        )))));

        assert_eq!(
            execute(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                &mut environment,
                &instruction,
                &expected,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected,
                    actual: ValueType::Int,
                },
            )),
        );
    }

    #[test]
    fn parameter_list_dispatch_propagates_projection_invariants() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);
        let type_id = plan.parameter_list_function_id(0).type_id();
        let mut retained = RetainedValues::empty();
        retained.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
        let mut environment = BlockEnvironment::from_retained(retained);
        let instruction = ListInstruction::Parameter(
            type_id,
            ParameterListInstruction::TupleIndex {
                tuple: TupleLocalId(0),
                index: 0,
            },
        );
        let expected = ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0))));

        assert_eq!(
            execute(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                &mut environment,
                &instruction,
                &expected,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected,
                    actual: ValueType::Int,
                },
            )),
        );
    }

    #[test]
    fn nested_list_projection_reports_the_exact_missing_index_for_every_storage_family() {
        let plan = crate::runtime::plan_src(
            "fn child() -> List(Int) { [] } fn parent() -> List(List(Int)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<IntFamily>(
            &plan,
            plan.int_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Int)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(String) { [] } fn parent() -> List(List(String)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<StringFamily>(
            &plan,
            plan.string_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::String)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(BitArray) { [] } fn parent() -> List(List(BitArray)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<BitArrayFamily>(
            &plan,
            plan.bit_array_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::BitArray)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(UtfCodepoint) { [] } fn parent() -> List(List(UtfCodepoint)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<UtfCodepointFamily>(
            &plan,
            plan.utf_codepoint_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
        );

        let plan = crate::runtime::plan_src(
            "pub type Boxed { Boxed(Int) } fn child() -> List(Boxed) { [] } fn parent() -> List(List(Boxed)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        let boxed = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        assert_nested_list_missing::<CustomFamily>(
            &plan,
            plan.custom_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Custom(boxed))),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(Float) { [] } fn parent() -> List(List(Float)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<FloatFamily>(
            &plan,
            plan.float_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Float)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(Bool) { [] } fn parent() -> List(List(Bool)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<BoolFamily>(
            &plan,
            plan.bool_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Bool)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(Nil) { [] } fn parent() -> List(List(Nil)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<NilFamily>(
            &plan,
            plan.nil_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Nil)),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(#(Int)) { [] } fn parent() -> List(List(#(Int))) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<TupleFamily>(
            &plan,
            plan.tuple_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(List(Int)) { [] } fn parent() -> List(List(List(Int))) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<ListFamily>(
            &plan,
            plan.list_list_function_id(0).type_id(),
            plan.list_list_function_id(1).type_id(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(fn(Int) -> Int) { [] } fn parent() -> List(List(fn(Int) -> Int)) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<FunctionFamily>(
            &plan,
            plan.function_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))))),
        );

        let plan = crate::runtime::plan_src(
            "fn child() -> List(List(value)) { [] } fn parent() -> List(List(List(value))) { [] } pub fn main() { let _ = child() let _ = parent() Nil }",
        );
        assert_nested_list_missing::<ParameterListFamily>(
            &plan,
            plan.parameter_list_list_function_id(0).type_id(),
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                TypeParameterId(0),
            ))))),
        );
    }

    #[test]
    fn tuple_and_custom_list_projections_reject_every_wrong_storage_family() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }

fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
fn customs() -> List(Boxed) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn(Int) -> Int) { [] }
fn parameter_values() -> List(value) { [] }
fn parameters() -> List(List(value)) { [] }

pub fn main() {
  let _ = ints()
  let _ = strings()
  let _ = bit_arrays()
  let _ = utf_codepoints()
  let _ = customs()
  let _ = floats()
  let _ = bools()
  let _ = nils()
  let _ = tuples()
  let _ = lists()
  let _ = functions()
  let _ = parameter_values()
  let _ = parameters()
  Boxed(0)
}
"#,
        );
        let boxed = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let context = ProjectionContext {
            int_type: plan.int_list_function_id(0).type_id(),
            string_type: plan.string_list_function_id(0).type_id(),
            custom_local: direct_custom_return_local::<_, ()>(ExecutionFunctionRef::Graph(
                plan.custom_function(plan.custom_function_id(0)),
            )),
            constructor: plan.custom_constructor_id(0, 0),
            custom_type: boxed.clone(),
            plan: &plan,
        };

        assert_projection_mismatches::<IntFamily>(
            &context,
            plan.int_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Int)),
            wrong_string_list,
        );
        assert_projection_mismatches::<StringFamily>(
            &context,
            plan.string_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::String)),
            wrong_int_list,
        );
        assert_projection_mismatches::<BitArrayFamily>(
            &context,
            plan.bit_array_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::BitArray)),
            wrong_int_list,
        );
        assert_projection_mismatches::<UtfCodepointFamily>(
            &context,
            plan.utf_codepoint_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            wrong_int_list,
        );
        assert_projection_mismatches::<CustomFamily>(
            &context,
            plan.custom_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Custom(boxed))),
            wrong_int_list,
        );
        assert_projection_mismatches::<FloatFamily>(
            &context,
            plan.float_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Float)),
            wrong_int_list,
        );
        assert_projection_mismatches::<BoolFamily>(
            &context,
            plan.bool_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Bool)),
            wrong_int_list,
        );
        assert_projection_mismatches::<NilFamily>(
            &context,
            plan.nil_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Nil)),
            wrong_int_list,
        );
        assert_projection_mismatches::<TupleFamily>(
            &context,
            plan.tuple_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            wrong_int_list,
        );
        assert_projection_mismatches::<ListFamily>(
            &context,
            plan.list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
            wrong_int_list,
        );
        assert_projection_mismatches::<FunctionFamily>(
            &context,
            plan.function_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))))),
            wrong_int_list,
        );
        assert_projection_mismatches::<ParameterListFamily>(
            &context,
            plan.parameter_list_list_function_id(0).type_id(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                TypeParameterId(0),
            ))))),
            wrong_int_list,
        );
        assert_parameter_list_projection_mismatches(&context);
    }

    struct ProjectionCounterSchema;

    impl HostExternalSchema for ProjectionCounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    #[test]
    fn resumable_external_list_projections_reject_corrupted_field_families() {
        let provider = AsyncHostProviderModule::new("application", "library")
            .expect("async provider module")
            .with_external_type_for_test::<ProjectionCounterSchema>();
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

pub type CounterListBox {
  CounterListBox(values: List(Counter))
}

pub fn boxed() -> CounterListBox {
  CounterListBox([])
}
"#,
                )],
            )],
            hosts,
        )
        .expect("external List projection source");
        let plan = crate::planner::plan_async_host_library_program(program)
            .expect("external List projection plan");
        let template = plan
            .functions()
            .iter()
            .find(|function| function.name() == "boxed")
            .expect("public boxed function")
            .signature()
            .id();
        let custom_type = CustomType::new(
            CustomTypeName::new(
                "application".into(),
                "library".into(),
                "CounterListBox".into(),
            ),
            Vec::new(),
        );
        let entry = LibraryEntry::new(
            template,
            LibraryValueType::Custom(custom_type.clone()),
            Vec::new(),
            Vec::new(),
        );
        let (execution, entries) = crate::plan::execution::AsyncHostedExecution::from_library_plan(
            plan,
            entry,
            Vec::new(),
        );
        let function = *entries.customs[0].function();
        let custom_local = direct_custom_return_local(execution.custom_function(function).as_ref());
        let mut host = ();
        let mut echo = RecordedEcho::default();
        let mut stores = ();
        let mut state =
            ResumableState::<crate::StatelessHostProfile>::new(&mut host, &mut stores, &mut echo);
        let custom = poll_ready_custom_fixture(run_custom(
            &execution,
            &mut state,
            function,
            crate::runtime::error::HostCallOrigin::Entry,
            ProfiledRetainedValues::empty(),
        ))
        .expect("boxed external List should evaluate");
        let constructor = custom.constructor();
        assert_eq!(custom.fields().len(), 1);
        let mut fields = ProfiledRetainedValues::empty();
        fields.push_evaluated(custom.fields()[0].clone());
        let fields = BlockEnvironment::from_retained(fields);
        let list_type = fields.external_list(ExternalListLocalId(0)).type_id();
        let expected = execution
            .value_metadata()
            .list_value_type(list_type.list_type());

        type Instruction = TypedListInstruction<
            ExternalLocal,
            ExternalListLocalId,
            ExternalListFunctionId,
            ExternalListFunctionLocalId,
        >;

        let mut tuple_values = ProfiledRetainedValues::empty();
        tuple_values.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
        let tuple_environment = BlockEnvironment::from_retained(tuple_values);
        let tuple_instruction = Instruction::TupleIndex {
            tuple: TupleLocalId(0),
            index: 0,
        };
        assert_eq!(
            execution_error(
                typed::<ExternalFamily, _, _, _>(
                    &execution,
                    &mut state,
                    &tuple_environment,
                    list_type,
                    &tuple_instruction,
                    &expected,
                ),
                "corrupted external List tuple projection should fail"
            )
            .into_execution(),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: expected.clone(),
                actual: ValueType::Int,
            }),
        );

        let malformed = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let mut custom_values = ProfiledRetainedValues::empty();
        custom_values.push_evaluated(EvaluatedValue::Custom(malformed));
        let custom_environment = BlockEnvironment::from_retained(custom_values);
        let custom_instruction = Instruction::CustomField {
            source: custom_local,
            index: 0,
        };
        assert_eq!(
            execution_error(
                typed::<ExternalFamily, _, _, _>(
                    &execution,
                    &mut state,
                    &custom_environment,
                    list_type,
                    &custom_instruction,
                    &expected,
                ),
                "corrupted external List custom projection should fail"
            )
            .into_execution(),
            ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                custom_type,
                constructor: "CounterListBox".into(),
                field_index: 0,
                expected,
                actual: ValueType::Int,
            }),
        );
    }

    fn poll_ready_custom_fixture(
        mut future: ResumableFuture<'_, EvaluatedCustomValue<TransferValues>>,
    ) -> Result<EvaluatedCustomValue<TransferValues>, TransferExecutionError> {
        let mut context = Context::from_waker(Waker::noop());
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("fixture without async calls should complete immediately"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture without async calls should complete immediately")]
    fn ready_fixture_guard_rejects_a_pending_future() {
        let _ = poll_ready_custom_fixture(Box::pin(std::future::pending()));
    }

    struct ProjectionContext<'a> {
        plan: &'a crate::ExecutionPlan,
        int_type: IntListTypeId,
        string_type: StringListTypeId,
        custom_local: CustomLocal,
        constructor: crate::plan::execution::type_::CustomConstructorId,
        custom_type: CustomType,
    }

    fn assert_projection_mismatches<
        Family: RuntimeTypedList<crate::runtime::LocalValues, FunctionLocal = ListFunctionLocal>,
    >(
        context: &ProjectionContext<'_>,
        type_id: Family::TypeId,
        expected: ValueType,
        wrong_list: fn(&mut RuntimeState, &ProjectionContext<'_>) -> ListValueId,
    ) where
        Family::Handle: std::fmt::Debug,
    {
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let wrong_list = wrong_list(&mut state, context);
        let wrong_value = EvaluatedValue::from(wrong_list);
        let actual = wrong_value.value_type(context.plan.value_metadata());

        assert_tuple_projection_error::<Family>(
            context,
            &mut state,
            type_id,
            &expected,
            wrong_value.clone(),
            actual.clone(),
        );
        assert_custom_projection_error::<Family>(
            context,
            &mut state,
            type_id,
            &expected,
            wrong_value,
            actual,
        );
        assert_tuple_projection_error::<Family>(
            context,
            &mut state,
            type_id,
            &expected,
            EvaluatedValue::Int(1.into()),
            ValueType::Int,
        );
        assert_custom_projection_error::<Family>(
            context,
            &mut state,
            type_id,
            &expected,
            EvaluatedValue::Int(1.into()),
            ValueType::Int,
        );
    }

    fn wrong_int_list(state: &mut RuntimeState, context: &ProjectionContext<'_>) -> ListValueId {
        state.lists_mut().int(context.int_type, Vec::new()).into()
    }

    fn wrong_string_list(state: &mut RuntimeState, context: &ProjectionContext<'_>) -> ListValueId {
        state
            .lists_mut()
            .string(context.string_type, Vec::new())
            .into()
    }

    fn assert_parameter_list_projection_mismatches(context: &ProjectionContext<'_>) {
        let expected = ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0))));
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let wrong: ListValueId = state.lists_mut().int(context.int_type, Vec::new()).into();
        let wrong_value = EvaluatedValue::from(wrong);
        let actual = wrong_value.value_type(context.plan.value_metadata());
        let mut tuple_values = RetainedValues::empty();
        tuple_values.push_evaluated(EvaluatedValue::Tuple(vec![wrong_value.clone()]));
        let tuple_environment = BlockEnvironment::from_retained(tuple_values);

        assert_eq!(
            execution_error(
                parameter(
                    context.plan,
                    &state,
                    &tuple_environment,
                    context.plan.parameter_list_function_id(0).type_id(),
                    &ParameterListInstruction::TupleIndex {
                        tuple: TupleLocalId(0),
                        index: 0,
                    },
                    &expected,
                ),
                "a tuple projection must preserve its list family",
            ),
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: expected.clone(),
                actual: actual.clone(),
            },),
        );

        let custom = EvaluatedCustomValue::from_fields(
            context.constructor,
            vec![wrong_value].into_boxed_slice(),
        );
        let mut custom_values = RetainedValues::empty();
        custom_values.push_evaluated(EvaluatedValue::Custom(custom));
        let custom_environment = BlockEnvironment::from_retained(custom_values);
        assert_eq!(
            execution_error(
                parameter(
                    context.plan,
                    &state,
                    &custom_environment,
                    context.plan.parameter_list_function_id(0).type_id(),
                    &ParameterListInstruction::CustomField {
                        source: context.custom_local,
                        index: 0,
                    },
                    &expected,
                ),
                "a custom projection must preserve its list family",
            ),
            ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                custom_type: context.custom_type.clone(),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: expected.clone(),
                actual,
            },),
        );

        let empty = state
            .lists_mut()
            .parameter_list_list(context.plan.parameter_list_list_function_id(0).type_id(), 0);
        let mut list_values = RetainedValues::empty();
        list_values.push_evaluated(EvaluatedValue::from(ListValueId::ParameterList(empty)));
        let list_environment = BlockEnvironment::from_retained(list_values);
        assert_eq!(
            execution_error(
                parameter(
                    context.plan,
                    &state,
                    &list_environment,
                    context.plan.parameter_list_function_id(0).type_id(),
                    &ParameterListInstruction::ListIndex {
                        list: ParameterListListLocalId(0),
                        index: 0,
                    },
                    &expected,
                ),
                "a list projection must remain in bounds",
            ),
            ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                item_type: expected,
                index: 0,
                length: 0,
            },),
        );
    }

    fn assert_tuple_projection_error<
        Family: RuntimeTypedList<crate::runtime::LocalValues, FunctionLocal = ListFunctionLocal>,
    >(
        context: &ProjectionContext<'_>,
        state: &mut RuntimeState,
        type_id: Family::TypeId,
        expected: &ValueType,
        value: EvaluatedValue,
        actual: ValueType,
    ) where
        Family::Handle: std::fmt::Debug,
    {
        let mut values = RetainedValues::empty();
        values.push_evaluated(EvaluatedValue::Tuple(vec![value]));
        let environment = BlockEnvironment::from_retained(values);
        let instruction = TypedListInstruction::<
            Family::ElementLocal,
            Family::Local,
            Family::Function,
        >::TupleIndex {
            tuple: TupleLocalId(0),
            index: 0,
        };

        assert_projection_error::<Family>(
            context.plan,
            state,
            &environment,
            type_id,
            &instruction,
            expected,
            ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                expected: expected.clone(),
                actual,
            }),
        );
    }

    fn assert_custom_projection_error<
        Family: RuntimeTypedList<crate::runtime::LocalValues, FunctionLocal = ListFunctionLocal>,
    >(
        context: &ProjectionContext<'_>,
        state: &mut RuntimeState,
        type_id: Family::TypeId,
        expected: &ValueType,
        value: EvaluatedValue,
        actual: ValueType,
    ) where
        Family::Handle: std::fmt::Debug,
    {
        let custom =
            EvaluatedCustomValue::from_fields(context.constructor, vec![value].into_boxed_slice());
        let mut values = RetainedValues::empty();
        values.push_evaluated(EvaluatedValue::Custom(custom));
        let environment = BlockEnvironment::from_retained(values);
        let instruction = TypedListInstruction::<
            Family::ElementLocal,
            Family::Local,
            Family::Function,
        >::CustomField {
            source: context.custom_local,
            index: 0,
        };

        assert_projection_error::<Family>(
            context.plan,
            state,
            &environment,
            type_id,
            &instruction,
            expected,
            ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                custom_type: context.custom_type.clone(),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: expected.clone(),
                actual,
            }),
        );
    }

    fn assert_projection_error<
        Family: RuntimeTypedList<crate::runtime::LocalValues, FunctionLocal = ListFunctionLocal>,
    >(
        plan: &crate::ExecutionPlan,
        state: &mut RuntimeState,
        environment: &BlockEnvironment,
        type_id: Family::TypeId,
        instruction: &TypedListInstruction<Family::ElementLocal, Family::Local, Family::Function>,
        expected: &ValueType,
        expected_error: ExecutionError,
    ) where
        Family::Handle: std::fmt::Debug,
    {
        assert_eq!(
            execution_error(
                typed::<Family, _, _, _>(plan, state, environment, type_id, instruction, expected,),
                "malformed projected list should fail at its owning boundary",
            ),
            expected_error,
        );
    }

    fn direct_custom_return_local<Body: FunctionBodyOwner<Return = CustomLocal>, Host>(
        function: ExecutionFunctionRef<'_, Body, Host>,
    ) -> CustomLocal {
        let ExecutionFunctionRef::Graph(function) = function else {
            panic!("custom fixture should lower to a graph function");
        };
        let body = function.body().function_body();
        let block_graph = body.block_graph();
        let Terminator::Exit(exit) = block_graph.block(block_graph.entry()).terminator() else {
            panic!("custom main should return its constructed value directly");
        };
        let crate::plan::execution::function::FunctionExit::Return(local) = body.exit(*exit) else {
            panic!("custom main should return its constructed value directly");
        };
        *local
    }

    fn assert_nested_list_missing<
        Family: RuntimeTypedList<crate::runtime::LocalValues, FunctionLocal = ListFunctionLocal>,
    >(
        plan: &crate::ExecutionPlan,
        child_type: Family::TypeId,
        parent_type: ListListTypeId,
        expected: ValueType,
    ) where
        Family::Handle: std::fmt::Debug,
    {
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let parent = state.lists_mut().list(parent_type, Vec::new());
        let mut values = RetainedValues::empty();
        values.push_evaluated(EvaluatedValue::List(parent.into()));
        let environment = BlockEnvironment::from_retained(values);
        let instruction = TypedListInstruction::<
            Family::ElementLocal,
            Family::Local,
            Family::Function,
        >::ListIndex {
            list: ListListLocalId(0),
            index: 2,
        };

        assert_eq!(
            execution_error(
                typed::<Family, _, _, _>(
                    plan,
                    &mut state,
                    &environment,
                    child_type,
                    &instruction,
                    &expected,
                ),
                "missing nested list index should fail",
            ),
            ExecutionError::Invariant(InvariantError::ListIndexOutOfBounds {
                item_type: expected,
                index: 2,
                length: 0,
            }),
        );
    }

    #[test]
    #[should_panic(expected = "custom main should return its constructed value directly")]
    fn direct_custom_return_local_guard_rejects_control_flow_entries() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }

fn choose(flag: Bool) {
  case flag {
    True -> Boxed(1)
    False -> Boxed(2)
  }
}

pub fn main() { choose(True) }
"#,
        );
        direct_custom_return_local::<_, ()>(ExecutionFunctionRef::Graph(
            plan.custom_function(plan.custom_function_id(1)),
        ));
    }

    #[test]
    #[should_panic(expected = "custom main should return its constructed value directly")]
    fn direct_custom_return_local_guard_rejects_tail_calls() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(Int) }

fn loop(value: Boxed) -> Boxed { loop(value) }

pub fn main() { loop(Boxed(1)) }
"#,
        );
        direct_custom_return_local::<_, ()>(ExecutionFunctionRef::Graph(
            plan.custom_function(plan.custom_function_id(0)),
        ));
    }

    #[test]
    #[should_panic(expected = "custom fixture should lower to a graph function")]
    fn direct_custom_return_local_guard_rejects_host_entries() {
        direct_custom_return_local::<
            crate::plan::execution::function::ExecutionCustomFunctionBody<std::convert::Infallible>,
            _,
        >(ExecutionFunctionRef::Host(&()));
    }
}
