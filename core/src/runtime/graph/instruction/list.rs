use super::super::environment::BlockEnvironment;
use super::super::{GraphValue, RuntimeGraphState};
use super::value::{InstructionValue, custom_projection, ensure_list_index, tuple_projection};
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
use crate::runtime::InvariantError;
use crate::runtime::error::HostCallOrigin;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalListFunction, EvaluatedExternalValue,
    EvaluatedFunctionValue, EvaluatedListFunction, EvaluatedValue,
};
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    ExternalListAllocation, ExternalListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StoredListValueId, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) enum ListInstructionValue {
    Parameter(
        InstructionValue<ParameterListValueId, ParameterListFunctionId, ParameterListLocalId>,
    ),
    ParameterList(
        InstructionValue<
            ParameterListListValueId,
            ParameterListListFunctionId,
            ParameterListListLocalId,
        >,
    ),
    Int(InstructionValue<IntListValueId, IntListFunctionId, IntListLocalId>),
    String(InstructionValue<StringListValueId, StringListFunctionId, StringListLocalId>),
    BitArray(InstructionValue<BitArrayListValueId, BitArrayListFunctionId, BitArrayListLocalId>),
    UtfCodepoint(
        InstructionValue<
            UtfCodepointListValueId,
            UtfCodepointListFunctionId,
            UtfCodepointListLocalId,
        >,
    ),
    Custom(InstructionValue<CustomListValueId, CustomListFunctionId, CustomListLocalId>),
    Float(InstructionValue<FloatListValueId, FloatListFunctionId, FloatListLocalId>),
    Bool(InstructionValue<BoolListValueId, BoolListFunctionId, BoolListLocalId>),
    Nil(InstructionValue<NilListValueId, NilListFunctionId, NilListLocalId>),
    Tuple(InstructionValue<TupleListValueId, TupleListFunctionId, TupleListLocalId>),
    List(InstructionValue<ListListValueId, ListListFunctionId, ListListLocalId>),
    Function(InstructionValue<FunctionListValueId, FunctionListFunctionId, FunctionListLocalId>),
}

pub(in crate::runtime) type ExternalListInstructionValue =
    InstructionValue<ExternalListValueId, ExternalListFunctionId, ExternalListLocalId>;

pub(in crate::runtime) fn evaluate<Plan, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment,
    instruction: &ListInstruction,
    expected: &ValueType,
) -> Result<ListInstructionValue, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
{
    use ListInstructionValue as V;

    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            parameter(plan, state, environment, *type_id, instruction, expected).map(V::Parameter)
        }
        ListInstruction::ParameterList(type_id, instruction) => typed::<ParameterListFamily, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::ParameterList),
        ListInstruction::Int(type_id, instruction) => {
            typed::<IntFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Int)
        }
        ListInstruction::String(type_id, instruction) => {
            typed::<StringFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::String)
        }
        ListInstruction::BitArray(type_id, instruction) => {
            typed::<BitArrayFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::BitArray)
        }
        ListInstruction::UtfCodepoint(type_id, instruction) => typed::<UtfCodepointFamily, _, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(V::UtfCodepoint),
        ListInstruction::Custom(type_id, instruction) => {
            typed::<CustomFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Custom)
        }
        ListInstruction::Float(type_id, instruction) => {
            typed::<FloatFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Float)
        }
        ListInstruction::Bool(type_id, instruction) => {
            typed::<BoolFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Bool)
        }
        ListInstruction::Nil(type_id, instruction) => {
            typed::<NilFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Nil)
        }
        ListInstruction::Tuple(type_id, instruction) => {
            typed::<TupleFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Tuple)
        }
        ListInstruction::List(type_id, instruction) => {
            typed::<ListFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::List)
        }
        ListInstruction::Function(type_id, instruction) => {
            typed::<FunctionFamily, _, _>(plan, state, environment, *type_id, instruction, expected)
                .map(V::Function)
        }
    }
}

pub(in crate::runtime) fn evaluate_external<Plan, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment,
    instruction: &ExternalListInstruction,
    expected: &ValueType,
) -> Result<ExternalListInstructionValue, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
{
    typed::<ExternalFamily, _, _>(
        plan,
        state,
        environment,
        instruction.type_id(),
        instruction.instruction(),
        expected,
    )
}

fn parameter<Plan, State>(
    plan: &Plan,
    state: &State,
    environment: &BlockEnvironment,
    type_id: ParameterListTypeId,
    instruction: &ParameterListInstruction,
    expected: &ValueType,
) -> Result<
    InstructionValue<ParameterListValueId, ParameterListFunctionId, ParameterListLocalId>,
    State::Error,
>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
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

trait RuntimeTypedList {
    type TypeId: Copy;
    type ElementLocal;
    type Element: Clone;
    type Local: Copy
        + crate::plan::execution::constant::ConstantValue
        + GraphValue<Evaluated = Self::Handle>;
    type Function: Clone;
    type FunctionLocal;
    type FunctionValue: Clone;
    type Handle: Clone;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element;
    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle;
    fn function(environment: &BlockEnvironment, local: &Self::FunctionLocal)
    -> Self::FunctionValue;
    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture];
    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError>;
    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element>;
    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle;
    fn projected(value: &StoredListValueId) -> Option<Self::Handle>;
    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle;
}

type TypedListInstructionValue<Family> = InstructionValue<
    <Family as RuntimeTypedList>::Handle,
    <Family as RuntimeTypedList>::Function,
    <Family as RuntimeTypedList>::Local,
>;

fn typed<Family, Plan, State>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment,
    type_id: Family::TypeId,
    instruction: &TypedListInstruction<
        Family::ElementLocal,
        Family::Local,
        Family::Function,
        Family::FunctionLocal,
    >,
    expected: &ValueType,
) -> Result<TypedListInstructionValue<Family>, State::Error>
where
    Family: RuntimeTypedList,
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
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

        impl RuntimeTypedList for $family {
            type TypeId = $type_id;
            type ElementLocal = $element_local;
            type Element = $element;
            type Local = $local;
            type Function = $function;
            type FunctionLocal = ListFunctionLocal;
            type FunctionValue = EvaluatedListFunction;
            type Handle = $handle;

            fn element(
                environment: &BlockEnvironment,
                local: &Self::ElementLocal,
            ) -> Self::Element {
                environment.$element_method(*local)
            }

            fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
                environment.$local_method(local)
            }

            fn function(
                environment: &BlockEnvironment,
                local: &Self::FunctionLocal,
            ) -> Self::FunctionValue {
                environment.list_function(local)
            }

            fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
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

            fn values<State: RuntimeGraphState>(
                state: &State,
                value: &Self::Handle,
            ) -> Vec<Self::Element> {
                state.lists().$values_method(value).to_vec()
            }

            fn allocate<State: RuntimeGraphState>(
                state: &mut State,
                type_id: Self::TypeId,
                values: Vec<Self::Element>,
            ) -> Self::Handle {
                state.lists_mut().$allocate_method(type_id, values)
            }

            fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
                <$handle>::from_stored(value)
            }

            fn from_core(
                type_id: Self::TypeId,
                core: crate::runtime::state::list::ListHandleCore,
            ) -> Self::Handle {
                <$handle>::new(type_id, core)
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

impl RuntimeTypedList for TupleFamily {
    type TypeId = TupleListTypeId;
    type ElementLocal = crate::plan::execution::graph::TupleLocalId;
    type Element = Vec<EvaluatedValue>;
    type Local = TupleListLocalId;
    type Function = TupleListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = TupleListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.tuple(*local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.tuple_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Tuple(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().tuple_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().tuple(type_id, values)
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        TupleListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        TupleListValueId::new(type_id, core)
    }
}

struct CustomFamily;

impl RuntimeTypedList for CustomFamily {
    type TypeId = CustomListTypeId;
    type ElementLocal = crate::plan::execution::graph::CustomLocal;
    type Element = EvaluatedCustomValue;
    type Local = CustomListLocalId;
    type Function = CustomListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = CustomListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.custom(*local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.custom_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Custom(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().custom_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .custom(CustomListAllocation::new(type_id, values))
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        CustomListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        CustomListValueId::new(type_id, core)
    }
}

struct ExternalFamily;

impl RuntimeTypedList for ExternalFamily {
    type TypeId = ExternalListTypeId;
    type ElementLocal = crate::plan::execution::graph::ExternalLocal;
    type Element = EvaluatedExternalValue;
    type Local = ExternalListLocalId;
    type Function = ExternalListFunctionId;
    type FunctionLocal = ExternalListFunctionLocalId;
    type FunctionValue = EvaluatedExternalListFunction;
    type Handle = ExternalListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.external(*local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.external_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.external_list_function(*local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        Ok(function.runtime_id())
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().external_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .external(ExternalListAllocation::new(type_id, values))
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ExternalListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        ExternalListValueId::new(type_id, core)
    }
}

struct NilFamily;

impl RuntimeTypedList for NilFamily {
    type TypeId = NilListTypeId;
    type ElementLocal = crate::plan::execution::graph::NilLocalId;
    type Element = ();
    type Local = NilListLocalId;
    type Function = NilListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = NilListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.nil(*local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.nil_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Nil(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        vec![(); state.lists().nil_len(value)]
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().nil(type_id, values.len())
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        NilListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        NilListValueId::new(type_id, core)
    }
}

struct ParameterListFamily;

impl RuntimeTypedList for ParameterListFamily {
    type TypeId = ParameterListListTypeId;
    type ElementLocal = ParameterListLocalId;
    type Element = ParameterListValueId;
    type Local = ParameterListListLocalId;
    type Function = ParameterListListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = ParameterListListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.parameter_list(*local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.parameter_list_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::ParameterList(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        vec![
            ParameterListValueId::new(value.type_id().item_type());
            state.lists().parameter_list_list_len(value)
        ]
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().parameter_list_list(type_id, values.len())
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ParameterListListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        ParameterListListValueId::new(type_id, core)
    }
}

struct ListFamily;

impl RuntimeTypedList for ListFamily {
    type TypeId = ListListTypeId;
    type ElementLocal = StoredListLocal;
    type Element = StoredListValueId;
    type Local = ListListLocalId;
    type Function = ListListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = ListListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.stored_list(local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.list_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::List(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().list_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().list(type_id, values)
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ListListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        ListListValueId::new(type_id, core)
    }
}

struct FunctionFamily;

impl RuntimeTypedList for FunctionFamily {
    type TypeId = FunctionListTypeId;
    type ElementLocal = crate::plan::execution::graph::FunctionLocal;
    type Element = EvaluatedFunctionValue;
    type Local = FunctionListLocalId;
    type Function = FunctionListFunctionId;
    type FunctionLocal = ListFunctionLocal;
    type FunctionValue = EvaluatedListFunction;
    type Handle = FunctionListValueId;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element {
        environment.function_value(local)
    }

    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle {
        environment.function_list(local)
    }

    fn function(
        environment: &BlockEnvironment,
        local: &Self::FunctionLocal,
    ) -> Self::FunctionValue {
        environment.list_function(local)
    }

    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture] {
        function.captures()
    }

    fn function_id(function: &Self::FunctionValue) -> Result<Self::Function, InvariantError> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Function(function)) => Ok(function),
            _ => Err(list_function_mismatch()),
        }
    }

    fn values<State: RuntimeGraphState>(state: &State, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().function_values(value).to_vec()
    }

    fn allocate<State: RuntimeGraphState>(
        state: &mut State,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().function(type_id, values)
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        FunctionListValueId::from_stored(value)
    }

    fn from_core(
        type_id: Self::TypeId,
        core: crate::runtime::state::list::ListHandleCore,
    ) -> Self::Handle {
        FunctionListValueId::new(type_id, core)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::environment::{BlockEnvironment, RetainedValues};
    use super::{
        BitArrayFamily, BoolFamily, CustomFamily, ExternalFamily, FloatFamily, FunctionFamily,
        IntFamily, ListFamily, NilFamily, ParameterListFamily, RuntimeTypedList, StringFamily,
        TupleFamily, UtfCodepointFamily, evaluate, list_function_mismatch, parameter, typed,
    };
    use crate::frontend::compile_typed_host_program;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet};
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

    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::ListValueId;
    use crate::runtime::{
        EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedListFunction, EvaluatedValue,
        ExecutionError, InvariantError,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{ModuleSource, PackageSource};

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

    fn evaluated_int_list_function(plan: &crate::ExecutionPlan) -> EvaluatedListFunction {
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

    fn evaluated_nil_list_function(plan: &crate::ExecutionPlan) -> EvaluatedListFunction {
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

    fn assert_list_function_mismatch<Family>(function: EvaluatedListFunction)
    where
        Family: RuntimeTypedList<FunctionValue = EvaluatedListFunction>,
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

    fn assert_every_list_function_mismatch(plan: &crate::ExecutionPlan) {
        let wrong_int = evaluated_int_list_function(plan);

        assert_list_function_mismatch::<IntFamily>(evaluated_nil_list_function(plan));
        assert_list_function_mismatch::<StringFamily>(wrong_int.clone());
        assert_list_function_mismatch::<BitArrayFamily>(wrong_int.clone());
        assert_list_function_mismatch::<UtfCodepointFamily>(wrong_int.clone());
        assert_list_function_mismatch::<CustomFamily>(wrong_int.clone());
        assert_list_function_mismatch::<FloatFamily>(wrong_int.clone());
        assert_list_function_mismatch::<BoolFamily>(wrong_int.clone());
        assert_list_function_mismatch::<NilFamily>(wrong_int.clone());
        assert_list_function_mismatch::<TupleFamily>(wrong_int.clone());
        assert_list_function_mismatch::<ParameterListFamily>(wrong_int.clone());
        assert_list_function_mismatch::<ListFamily>(wrong_int.clone());
        assert_list_function_mismatch::<FunctionFamily>(wrong_int);
    }

    #[test]
    fn list_function_value_dispatch_rejects_every_wrong_item_family() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);

        assert_every_list_function_mismatch(&plan);
        assert_every_list_function_mismatch(&plan);
    }

    #[test]
    fn parameter_list_function_call_rejects_a_wrong_list_family() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);
        let int_function = evaluated_int_list_function(&plan);
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
        let environment = BlockEnvironment::from_retained(retained);
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
            evaluate(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                &environment,
                &instruction,
                &expected,
            )
            .map(|_| ()),
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
        let environment = BlockEnvironment::from_retained(retained);
        let instruction = ListInstruction::Parameter(
            type_id,
            ParameterListInstruction::TupleIndex {
                tuple: TupleLocalId(0),
                index: 0,
            },
        );
        let expected = ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0))));

        assert_eq!(
            evaluate(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                &environment,
                &instruction,
                &expected,
            )
            .map(|_| ()),
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

    struct ProjectionProfile;
    impl HostProfile for ProjectionProfile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl crate::host::HostWorkProfile for ProjectionProfile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for ProjectionProfile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    #[test]
    fn transfer_external_list_projections_reject_corrupted_field_families() {
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        crate::work_fixture::WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
pub type Counter = future.Work(Int)

pub type CounterListBox {
  CounterListBox(values: List(Counter))
}

pub fn boxed() -> CounterListBox {
  CounterListBox([])
}
"#,
                    )],
                ),
            ],
            HostProviderSet::<ProjectionProfile>::from_providers(
                WorkComponent::providers().expect("Future providers"),
            )
            .expect("providers"),
        )
        .expect("external List projection source");
        let plan = crate::planner::plan_host_library_program(program)
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
        let (execution, entries) =
            crate::plan::execution::HostedProgram::from_library_plan(plan, entry, Vec::new())
                .expect("transferable list entry qualification");
        let function = *entries.customs[0].function();
        let custom_local = direct_custom_return_local(execution.custom_function(function).as_ref());
        let mut host = ();
        assert!(std::ptr::eq(
            <WorkComponent as crate::HostProvider<ProjectionProfile>>::project(&mut host),
            &host,
        ));
        let mut echo = drop;
        let mut stores = HostFutureStore::default();
        assert!(std::ptr::eq(
            ProjectionProfile::component_stores(&stores),
            &stores,
        ));
        let executor = crate::execution_fixture::TestHost::default();
        let execution = std::sync::Arc::new(execution);
        let domain = crate::runtime::execution::Domain::new(
            std::sync::Arc::clone(&execution),
            &executor,
            &mut host,
            &mut stores,
            &mut echo,
            std::num::NonZeroUsize::MIN,
        );
        let context = domain.context();
        let custom = executor
            .block_on(domain.drive(context.call(
                function,
                crate::runtime::error::HostCallOrigin::Entry,
                RetainedValues::empty(),
            )))
            .expect("host cleanup")
            .expect("active entry")
            .expect("boxed external List should evaluate");
        let work = crate::runtime::work::execution::ExecutionWork::new();
        let mut units = crate::runtime::execution::Units::new(());
        let mut runtime = RuntimeState::with_host_and_lists(
            &mut echo,
            crate::runtime::state::RuntimeHost::<ProjectionProfile>::new(
                &mut host,
                &stores,
                &work,
                &mut units,
                work.execution(),
                crate::execution::ExecutionClock::new(&executor),
            ),
            Default::default(),
        );
        {
            let state = &mut runtime;
            let constructor = custom.constructor();
            assert_eq!(custom.fields().len(), 1);
            let mut fields = RetainedValues::empty();
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

            let mut tuple_values = RetainedValues::empty();
            tuple_values.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]));
            let tuple_environment = BlockEnvironment::from_retained(tuple_values);
            let tuple_instruction = Instruction::TupleIndex {
                tuple: TupleLocalId(0),
                index: 0,
            };
            assert_eq!(
                execution_error(
                    typed::<ExternalFamily, _, _>(
                        execution.as_ref(),
                        state,
                        &tuple_environment,
                        list_type,
                        &tuple_instruction,
                        &expected,
                    ),
                    "corrupted external List tuple projection should fail"
                )
                .into_materialized(),
                ExecutionError::Invariant(InvariantError::TupleIndexFamilyMismatch {
                    expected: expected.clone(),
                    actual: ValueType::Int,
                }),
            );

            let malformed = EvaluatedCustomValue::from_fields(
                constructor,
                vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
            );
            let mut custom_values = RetainedValues::empty();
            custom_values.push_evaluated(EvaluatedValue::Custom(malformed));
            let custom_environment = BlockEnvironment::from_retained(custom_values);
            let custom_instruction = Instruction::CustomField {
                source: custom_local,
                index: 0,
            };
            assert_eq!(
                execution_error(
                    typed::<ExternalFamily, _, _>(
                        execution.as_ref(),
                        state,
                        &custom_environment,
                        list_type,
                        &custom_instruction,
                        &expected,
                    ),
                    "corrupted external List custom projection should fail"
                )
                .into_materialized(),
                ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                    custom_type,
                    constructor: "CounterListBox".into(),
                    field_index: 0,
                    expected,
                    actual: ValueType::Int,
                }),
            );
        }
    }

    struct ProjectionContext<'a> {
        plan: &'a crate::ExecutionPlan,
        int_type: IntListTypeId,
        string_type: StringListTypeId,
        custom_local: CustomLocal,
        constructor: crate::plan::execution::type_::CustomConstructorId,
        custom_type: CustomType,
    }

    fn assert_projection_mismatches<Family: RuntimeTypedList<FunctionLocal = ListFunctionLocal>>(
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

    fn assert_tuple_projection_error<Family: RuntimeTypedList<FunctionLocal = ListFunctionLocal>>(
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

    fn assert_custom_projection_error<Family: RuntimeTypedList<FunctionLocal = ListFunctionLocal>>(
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

    fn assert_projection_error<Family: RuntimeTypedList<FunctionLocal = ListFunctionLocal>>(
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
                typed::<Family, _, _>(plan, state, environment, type_id, instruction, expected,),
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

    fn assert_nested_list_missing<Family: RuntimeTypedList<FunctionLocal = ListFunctionLocal>>(
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
                typed::<Family, _, _>(
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
