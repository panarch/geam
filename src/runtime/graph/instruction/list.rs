use super::super::environment::{BlockEnvironment, RetainedValues};
use super::value::{constant, custom_projection, ensure_list_index, tuple_projection};
use crate::plan::ValueType;
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
    FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, ListFunctionId,
    ListListFunctionId, NilListFunctionId, ParameterListListFunctionId, RuntimeListFunctionId,
    StringListFunctionId, TupleListFunctionId, UtfCodepointListFunctionId,
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
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListAllocation, CustomListValueId,
    ExternalListAllocation, ExternalListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListHandleCore, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StoredListValueId, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use crate::runtime::state::{RuntimeState, RuntimeStateFor};
use crate::runtime::{ExecutionError, InvariantError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn execute<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &ListInstruction,
    expected: &ValueType,
) -> ExecutionResult<()> {
    match instruction {
        ListInstruction::Parameter(type_id, instruction) => {
            parameter(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_parameter_list(value))
        }
        ListInstruction::ParameterList(type_id, instruction) => typed::<ParameterListFamily, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(|value| environment.push_parameter_list_list(value)),
        ListInstruction::Int(type_id, instruction) => {
            typed::<IntFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_int_list(value))
        }
        ListInstruction::String(type_id, instruction) => {
            typed::<StringFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_string_list(value))
        }
        ListInstruction::BitArray(type_id, instruction) => {
            typed::<BitArrayFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_bit_array_list(value))
        }
        ListInstruction::UtfCodepoint(type_id, instruction) => typed::<UtfCodepointFamily, _>(
            plan,
            state,
            environment,
            *type_id,
            instruction,
            expected,
        )
        .map(|value| environment.push_utf_codepoint_list(value)),
        ListInstruction::Custom(type_id, instruction) => {
            typed::<CustomFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_custom_list(value))
        }
        ListInstruction::Float(type_id, instruction) => {
            typed::<FloatFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_float_list(value))
        }
        ListInstruction::Bool(type_id, instruction) => {
            typed::<BoolFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_bool_list(value))
        }
        ListInstruction::Nil(type_id, instruction) => {
            typed::<NilFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_nil_list(value))
        }
        ListInstruction::Tuple(type_id, instruction) => {
            typed::<TupleFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_tuple_list(value))
        }
        ListInstruction::List(type_id, instruction) => {
            typed::<ListFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_list_list(value))
        }
        ListInstruction::Function(type_id, instruction) => {
            typed::<FunctionFamily, _>(plan, state, environment, *type_id, instruction, expected)
                .map(|value| environment.push_function_list(value))
        }
    }
}

pub(super) fn execute_external<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &mut BlockEnvironment,
    instruction: &ExternalListInstruction,
    expected: &ValueType,
) -> ExecutionResult<()> {
    typed::<ExternalFamily, _>(
        plan,
        state,
        environment,
        instruction.type_id(),
        instruction.instruction(),
        expected,
    )
    .map(|value| environment.push_external_list(value))
}

fn parameter<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &BlockEnvironment,
    type_id: ParameterListTypeId,
    instruction: &ParameterListInstruction,
    expected: &ValueType,
) -> ExecutionResult<ParameterListValueId> {
    use ParameterListInstruction as I;

    match instruction {
        I::Empty => Ok(ParameterListValueId::new(type_id)),
        I::Constant(id) => constant(plan, state, *id),
        I::Call {
            function,
            args,
            site,
        } => crate::runtime::function::run_parameter_list(
            plan,
            state,
            *function,
            HostCallOrigin::source(site.clone()),
            environment.retain(args),
        ),
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.list_function(function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(function.captures());
            match function.runtime_id() {
                RuntimeListFunctionId::Core(ListFunctionId::Parameter(function)) => {
                    crate::runtime::function::run_parameter_list(
                        plan,
                        state,
                        function,
                        HostCallOrigin::source(site.clone()),
                        inputs,
                    )
                }
                _ => Err(list_function_mismatch()),
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
        ),
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
        ),
        I::ListIndex { list, index } => {
            let length = state
                .lists()
                .parameter_list_list_len(&environment.parameter_list_list(*list));
            ensure_list_index(expected, *index, length).map(|()| ParameterListValueId::new(type_id))
        }
    }
}

trait RuntimeTypedList {
    type TypeId: Copy;
    type ElementLocal;
    type Element: Clone;
    type Local: Copy
        + crate::plan::execution::constant::ConstantValue
        + super::super::GraphValue<Evaluated = Self::Handle>;
    type Function: Clone;
    type FunctionLocal;
    type FunctionValue: Clone;
    type Handle: Clone;

    fn element(environment: &BlockEnvironment, local: &Self::ElementLocal) -> Self::Element;
    fn local(environment: &BlockEnvironment, local: Self::Local) -> Self::Handle;
    fn function(environment: &BlockEnvironment, local: &Self::FunctionLocal)
    -> Self::FunctionValue;
    fn captures(function: &Self::FunctionValue) -> &[crate::runtime::EvaluatedCapture];
    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element>;
    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle;
    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle>;
    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle>;
    fn projected(value: &StoredListValueId) -> Option<Self::Handle>;
    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle;
}

fn typed<Family: RuntimeTypedList, Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &BlockEnvironment,
    type_id: Family::TypeId,
    instruction: &TypedListInstruction<
        Family::ElementLocal,
        Family::Local,
        Family::Function,
        Family::FunctionLocal,
    >,
    expected: &ValueType,
) -> ExecutionResult<Family::Handle> {
    use TypedListInstruction as I;

    match instruction {
        I::Value(elements) => Ok(Family::allocate(
            state,
            type_id,
            elements
                .iter()
                .map(|element| Family::element(environment, element))
                .collect(),
        )),
        I::Constant(id) => constant(plan, state, *id),
        I::Spread { elements, tail } => {
            let mut values = elements
                .iter()
                .map(|element| Family::element(environment, element))
                .collect::<Vec<_>>();
            values.extend(Family::values(state, &Family::local(environment, *tail)));
            Ok(Family::allocate(state, type_id, values))
        }
        I::Call {
            function,
            args,
            site,
        } => Family::run_direct(
            plan,
            state,
            function.clone(),
            HostCallOrigin::source(site.clone()),
            environment.retain(args),
        ),
        I::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = Family::function(environment, function);
            let mut inputs = environment.retain(args);
            inputs.append_captures(Family::captures(&function));
            Family::run_value(
                plan,
                state,
                function,
                HostCallOrigin::source(site.clone()),
                inputs,
            )
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
        ),
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
        ),
        I::ListIndex { list, index } => {
            let list = environment.list_list(*list);
            let values = state.lists().list_values(&list);
            match values.get(*index) {
                Some(value) => Ok(Family::from_core(type_id, value.clone().into_core())),
                None => Err(ExecutionError::Invariant(
                    InvariantError::ListIndexOutOfBounds {
                        item_type: expected.clone(),
                        index: *index,
                        length: values.len(),
                    },
                )),
            }
        }
        I::DropFirst { list, count } => {
            let values = Family::values(state, &Family::local(environment, *list));
            let values = values[(*count).min(values.len())..].to_vec();
            Ok(Family::allocate(state, type_id, values))
        }
    }
}

fn list_function_mismatch() -> ExecutionError {
    ExecutionError::Invariant(InvariantError::FunctionReturnFamilyMismatch {
        expected: crate::plan::execution::function::FunctionReturnFamily::List,
        actual: crate::plan::execution::function::FunctionReturnFamily::List,
    })
}

macro_rules! vector_family {
    (
        $family:ident,
        $type_id:ty,
        $element_local:ty,
        $element:ty,
        $local:ty,
        $function:ty,
        $handle:ty,
        $function_variant:ident,
        $element_method:ident,
        $local_method:ident,
        $values_method:ident,
        $allocate_method:ident,
        $run_method:ident
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

            fn values<State>(
                state: &RuntimeState<'_, State>,
                value: &Self::Handle,
            ) -> Vec<Self::Element> {
                state.lists().$values_method(value).to_vec()
            }

            fn allocate<State>(
                state: &mut RuntimeState<'_, State>,
                type_id: Self::TypeId,
                values: Vec<Self::Element>,
            ) -> Self::Handle {
                state.lists_mut().$allocate_method(type_id, values)
            }

            fn run_direct<Plan: ExecutableRuntimePlan>(
                plan: &Plan,
                state: &mut RuntimeStateFor<'_, Plan>,
                function: Self::Function,
                origin: HostCallOrigin,
                inputs: RetainedValues,
            ) -> ExecutionResult<Self::Handle> {
                crate::runtime::function::$run_method(plan, state, function, origin, inputs)
            }

            fn run_value<Plan: ExecutableRuntimePlan>(
                plan: &Plan,
                state: &mut RuntimeStateFor<'_, Plan>,
                function: Self::FunctionValue,
                origin: HostCallOrigin,
                inputs: RetainedValues,
            ) -> ExecutionResult<Self::Handle> {
                match function.runtime_id() {
                    RuntimeListFunctionId::Core(ListFunctionId::$function_variant(function)) => {
                        crate::runtime::function::$run_method(plan, state, function, origin, inputs)
                    }
                    _ => Err(list_function_mismatch()),
                }
            }

            fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
                <$handle>::from_stored(value)
            }

            fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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
    int,
    run_int_list
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
    string,
    run_string_list
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
    bit_array,
    run_bit_array_list
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
    utf_codepoint,
    run_utf_codepoint_list
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
    float,
    run_float_list
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
    bool,
    run_bool_list
);
vector_family!(
    TupleFamily,
    TupleListTypeId,
    crate::plan::execution::graph::TupleLocalId,
    Vec<EvaluatedValue>,
    TupleListLocalId,
    TupleListFunctionId,
    TupleListValueId,
    Tuple,
    tuple,
    tuple_list,
    tuple_values,
    tuple,
    run_tuple_list
);

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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().custom_values(value).to_vec()
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .custom(CustomListAllocation::new(type_id, values))
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_custom_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Custom(function)) => {
                crate::runtime::function::run_custom_list(plan, state, function, origin, inputs)
            }
            _ => Err(list_function_mismatch()),
        }
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        CustomListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().external_values(value).to_vec()
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state
            .lists_mut()
            .external(ExternalListAllocation::new(type_id, values))
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_external_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_external_list(
            plan,
            state,
            function.runtime_id(),
            origin,
            inputs,
        )
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ExternalListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        vec![(); state.lists().nil_len(value)]
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().nil(type_id, values.len())
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_nil_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Nil(function)) => {
                crate::runtime::function::run_nil_list(plan, state, function, origin, inputs)
            }
            _ => Err(list_function_mismatch()),
        }
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        NilListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        vec![
            ParameterListValueId::new(value.type_id().item_type());
            state.lists().parameter_list_list_len(value)
        ]
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().parameter_list_list(type_id, values.len())
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_parameter_list_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::ParameterList(function)) => {
                crate::runtime::function::run_parameter_list_list(
                    plan, state, function, origin, inputs,
                )
            }
            _ => Err(list_function_mismatch()),
        }
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ParameterListListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().list_values(value).to_vec()
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().list(type_id, values)
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_list_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::List(function)) => {
                crate::runtime::function::run_list_list(plan, state, function, origin, inputs)
            }
            _ => Err(list_function_mismatch()),
        }
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        ListListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
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

    fn values<State>(state: &RuntimeState<'_, State>, value: &Self::Handle) -> Vec<Self::Element> {
        state.lists().function_values(value).to_vec()
    }

    fn allocate<State>(
        state: &mut RuntimeState<'_, State>,
        type_id: Self::TypeId,
        values: Vec<Self::Element>,
    ) -> Self::Handle {
        state.lists_mut().function(type_id, values)
    }

    fn run_direct<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::Function,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        crate::runtime::function::run_function_list(plan, state, function, origin, inputs)
    }

    fn run_value<Plan: ExecutableRuntimePlan>(
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        function: Self::FunctionValue,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<Self::Handle> {
        match function.runtime_id() {
            RuntimeListFunctionId::Core(ListFunctionId::Function(function)) => {
                crate::runtime::function::run_function_list(plan, state, function, origin, inputs)
            }
            _ => Err(list_function_mismatch()),
        }
    }

    fn projected(value: &StoredListValueId) -> Option<Self::Handle> {
        FunctionListValueId::from_stored(value)
    }

    fn from_core(type_id: Self::TypeId, core: ListHandleCore) -> Self::Handle {
        FunctionListValueId::new(type_id, core)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::environment::{BlockEnvironment, RetainedValues};
    use super::{
        BitArrayFamily, BoolFamily, CustomFamily, FloatFamily, FunctionFamily, IntFamily,
        ListFamily, NilFamily, ParameterListFamily, RuntimeTypedList, StringFamily, TupleFamily,
        UtfCodepointFamily, execute, list_function_mismatch, parameter, typed,
    };
    use crate::plan::execution::function::{
        CustomFunctionId, ListFunctionId, RuntimeListFunctionId,
    };
    use crate::plan::execution::graph::{
        CustomLocal, IntListFunctionLocalId, ListFunctionLocal, ListInstruction, ListListLocalId,
        ParameterListInstruction, ParameterListListLocalId, Terminator, TupleLocalId,
        TypedListInstruction,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::{IntListTypeId, ListListTypeId, StringListTypeId};
    use crate::plan::{CustomType, CustomTypeName, FunctionType, TypeParameterId, ValueType};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::ListValueId;
    use crate::runtime::{
        EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedListFunction, EvaluatedValue,
        ExecutionError, InvariantError,
    };

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

    fn assert_list_function_mismatch<Family>(
        plan: &crate::ExecutionPlan,
        function: EvaluatedListFunction,
    ) where
        Family: RuntimeTypedList<FunctionValue = EvaluatedListFunction>,
        Family::Handle: std::fmt::Debug + PartialEq,
    {
        assert_eq!(
            Family::run_value(
                plan,
                &mut RuntimeState::new(&mut Vec::new()),
                function,
                crate::runtime::error::HostCallOrigin::Entry,
                RetainedValues::empty()
            ),
            Err(list_function_mismatch()),
        );
    }

    #[test]
    fn list_function_value_dispatch_rejects_every_wrong_item_family() {
        let plan = crate::runtime::plan_src(LIST_FUNCTION_FAMILY_SOURCE);
        let wrong_int = evaluated_int_list_function(&plan);

        assert_list_function_mismatch::<IntFamily>(&plan, evaluated_nil_list_function(&plan));
        assert_list_function_mismatch::<CustomFamily>(&plan, wrong_int.clone());
        assert_list_function_mismatch::<NilFamily>(&plan, wrong_int.clone());
        assert_list_function_mismatch::<ParameterListFamily>(&plan, wrong_int.clone());
        assert_list_function_mismatch::<ListFamily>(&plan, wrong_int.clone());
        assert_list_function_mismatch::<FunctionFamily>(&plan, wrong_int);
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
            parameter(
                &plan,
                &mut RuntimeState::new(&mut Vec::new()),
                &environment,
                plan.parameter_list_function_id(0).type_id(),
                &instruction,
                &ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
            ),
            Err(list_function_mismatch()),
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
            custom_local: direct_custom_return_local(&plan, plan.custom_function_id(0)),
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
            parameter(
                context.plan,
                &mut state,
                &tuple_environment,
                context.plan.parameter_list_function_id(0).type_id(),
                &ParameterListInstruction::TupleIndex {
                    tuple: TupleLocalId(0),
                    index: 0,
                },
                &expected,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected: expected.clone(),
                    actual: actual.clone(),
                },
            )),
        );

        let custom = EvaluatedCustomValue::from_fields(
            context.constructor,
            vec![wrong_value].into_boxed_slice(),
        );
        let mut custom_values = RetainedValues::empty();
        custom_values.push_evaluated(EvaluatedValue::Custom(custom));
        let custom_environment = BlockEnvironment::from_retained(custom_values);
        assert_eq!(
            parameter(
                context.plan,
                &mut state,
                &custom_environment,
                context.plan.parameter_list_function_id(0).type_id(),
                &ParameterListInstruction::CustomField {
                    source: context.custom_local,
                    index: 0,
                },
                &expected,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: context.custom_type.clone(),
                    constructor: "Boxed".into(),
                    field_index: 0,
                    expected: expected.clone(),
                    actual,
                },
            )),
        );

        let empty = state
            .lists_mut()
            .parameter_list_list(context.plan.parameter_list_list_function_id(0).type_id(), 0);
        let mut list_values = RetainedValues::empty();
        list_values.push_evaluated(EvaluatedValue::from(ListValueId::ParameterList(empty)));
        let list_environment = BlockEnvironment::from_retained(list_values);
        assert_eq!(
            parameter(
                context.plan,
                &mut state,
                &list_environment,
                context.plan.parameter_list_function_id(0).type_id(),
                &ParameterListInstruction::ListIndex {
                    list: ParameterListListLocalId(0),
                    index: 0,
                },
                &expected,
            ),
            Err(ExecutionError::Invariant(
                InvariantError::ListIndexOutOfBounds {
                    item_type: expected,
                    index: 0,
                    length: 0,
                },
            )),
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
            typed::<Family, crate::ExecutionPlan>(
                plan,
                state,
                environment,
                type_id,
                instruction,
                expected,
            )
            .expect_err("malformed projected list should fail at its owning boundary"),
            expected_error,
        );
    }

    fn direct_custom_return_local(
        plan: &crate::ExecutionPlan,
        function: CustomFunctionId,
    ) -> CustomLocal {
        let return_ = plan.custom_function(function).body();
        let body = return_.function_body();
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
            typed::<Family, crate::ExecutionPlan>(
                plan,
                &mut state,
                &environment,
                child_type,
                &instruction,
                &expected,
            )
            .expect_err("missing nested list index should fail"),
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
        direct_custom_return_local(&plan, plan.custom_function_id(1));
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
        direct_custom_return_local(&plan, plan.custom_function_id(0));
    }
}
