use super::evaluated::{
    EvaluatedBoolFunction, EvaluatedCapture, EvaluatedCaptureKind, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedValue,
};
use super::state::{ListValueId, RuntimeState};
use super::{
    BoolFunctionValue, CaptureListValue, CaptureValue, FloatFunctionValue, FunctionFunctionValue,
    FunctionValue, FunctionValueKind, IntFunctionValue, ListFunctionValue, ListValue,
    NilFunctionValue, StringFunctionValue, TupleFunctionValue, Value,
};
use crate::plan::execution::ExecutionPlan;

pub(super) fn value(plan: &ExecutionPlan, state: &RuntimeState, value: EvaluatedValue) -> Value {
    match value {
        EvaluatedValue::Int(value) => Value::Int(value),
        EvaluatedValue::Float(value) => Value::Float(value),
        EvaluatedValue::String(value) => Value::String(value),
        EvaluatedValue::Bool(value) => Value::Bool(value),
        EvaluatedValue::Nil => Value::Nil,
        EvaluatedValue::Tuple(values) => Value::Tuple(
            values
                .into_iter()
                .map(|value| self::value(plan, state, value))
                .collect(),
        ),
        EvaluatedValue::List(value) => Value::List(list(plan, state, &value)),
        EvaluatedValue::Function(value) => Value::Function(function(plan, state, value)),
    }
}

fn list(plan: &ExecutionPlan, state: &RuntimeState, value: &ListValueId) -> ListValue {
    match value {
        ListValueId::Int(value) => ListValue::int(state.int_values(value).to_vec()),
        ListValueId::String(value) => ListValue::string(state.string_values(value).to_vec()),
        ListValueId::Float(value) => ListValue::float(state.float_values(value).to_vec()),
        ListValueId::Bool(value) => ListValue::bool(state.bool_values(value).to_vec()),
        ListValueId::Nil(value) => ListValue::nil(state.nil_len(value)),
        ListValueId::Tuple(value) => ListValue::from_evaluated_tuple(
            plan.tuple_list_item_type(value.type_id()),
            state
                .tuple_values(value)
                .iter()
                .cloned()
                .map(|values| {
                    values
                        .into_iter()
                        .map(|value| self::value(plan, state, value))
                        .collect()
                })
                .collect(),
        ),
        ListValueId::List(value) => ListValue::from_evaluated_list(
            plan.nested_list_item_type(value.type_id()),
            state
                .list_values(value)
                .iter()
                .cloned()
                .map(|core| {
                    list(
                        plan,
                        state,
                        &ListValueId::from_core(plan, value.type_id().item_type(), core),
                    )
                })
                .collect(),
        ),
        ListValueId::Function(value) => ListValue::from_evaluated_function(
            plan.function_list_item_type(value.type_id()),
            state
                .function_values(value)
                .iter()
                .cloned()
                .map(|value| function(plan, state, value))
                .collect(),
        ),
    }
}

fn function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: EvaluatedFunctionValue,
) -> FunctionValue {
    let kind = match value.kind() {
        EvaluatedFunctionValueKind::Int(value) => {
            FunctionValueKind::Int(int_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Float(value) => {
            FunctionValueKind::Float(float_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::String(value) => {
            FunctionValueKind::String(string_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Bool(value) => {
            FunctionValueKind::Bool(bool_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Nil(value) => {
            FunctionValueKind::Nil(nil_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Tuple(value) => {
            FunctionValueKind::Tuple(tuple_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::List(value) => {
            FunctionValueKind::List(list_function(plan, state, value))
        }
        EvaluatedFunctionValueKind::Function(value) => {
            FunctionValueKind::Function(function_function(plan, state, value))
        }
    };
    FunctionValue::from_kind(kind)
}

fn int_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedIntFunction,
) -> IntFunctionValue {
    IntFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn float_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedFloatFunction,
) -> FloatFunctionValue {
    FloatFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn string_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedStringFunction,
) -> StringFunctionValue {
    StringFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn bool_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedBoolFunction,
) -> BoolFunctionValue {
    BoolFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn nil_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedNilFunction,
) -> NilFunctionValue {
    NilFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn tuple_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedTupleFunction,
) -> TupleFunctionValue {
    TupleFunctionValue::from_evaluated(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn list_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedListFunction,
) -> ListFunctionValue {
    ListFunctionValue::new_with_captures(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn function_function(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedFunctionFunction,
) -> FunctionFunctionValue {
    FunctionFunctionValue::from_evaluated(
        value.runtime_id(),
        value.params().to_vec(),
        captures(plan, state, value.captures()),
        plan.function_type(value.type_()),
    )
}

fn nested_list_values(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &super::state::ListListValueId,
) -> Vec<ListValue> {
    state
        .list_values(value)
        .iter()
        .cloned()
        .map(|core| {
            list(
                plan,
                state,
                &ListValueId::from_core(plan, value.type_id().item_type(), core),
            )
        })
        .collect()
}

fn captures(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    values: &[EvaluatedCapture],
) -> Vec<CaptureValue> {
    values
        .iter()
        .map(|value| capture(plan, state, value))
        .collect()
}

fn capture(plan: &ExecutionPlan, state: &RuntimeState, value: &EvaluatedCapture) -> CaptureValue {
    match value.kind() {
        EvaluatedCaptureKind::Int { local, value } => CaptureValue::int(*local, value.clone()),
        EvaluatedCaptureKind::Float { local, value } => CaptureValue::float(*local, *value),
        EvaluatedCaptureKind::String { local, value } => {
            CaptureValue::string(*local, value.clone())
        }
        EvaluatedCaptureKind::Bool { local, value } => CaptureValue::bool(*local, *value),
        EvaluatedCaptureKind::Nil { local } => CaptureValue::nil(*local),
        EvaluatedCaptureKind::Tuple { local, value } => CaptureValue::tuple(
            *local,
            value
                .iter()
                .cloned()
                .map(|value| self::value(plan, state, value))
                .collect(),
        ),
        EvaluatedCaptureKind::List(value) => CaptureValue::list(list_capture(plan, state, value)),
        EvaluatedCaptureKind::IntFunction { local, value } => {
            CaptureValue::int_function(*local, int_function(plan, state, value))
        }
        EvaluatedCaptureKind::FloatFunction { local, value } => {
            CaptureValue::float_function(*local, float_function(plan, state, value))
        }
        EvaluatedCaptureKind::StringFunction { local, value } => {
            CaptureValue::string_function(*local, string_function(plan, state, value))
        }
        EvaluatedCaptureKind::BoolFunction { local, value } => {
            CaptureValue::bool_function(*local, bool_function(plan, state, value))
        }
        EvaluatedCaptureKind::NilFunction { local, value } => {
            CaptureValue::nil_function(*local, nil_function(plan, state, value))
        }
        EvaluatedCaptureKind::TupleFunction { local, value } => {
            CaptureValue::tuple_function(*local, tuple_function(plan, state, value))
        }
        EvaluatedCaptureKind::ListFunction { local, value } => {
            CaptureValue::list_function(local.clone(), list_function(plan, state, value))
        }
        EvaluatedCaptureKind::FunctionFunction { local, value } => {
            CaptureValue::function_function(*local, function_function(plan, state, value))
        }
    }
}

fn list_capture(
    plan: &ExecutionPlan,
    state: &RuntimeState,
    value: &EvaluatedListCapture,
) -> CaptureListValue {
    match value {
        EvaluatedListCapture::Int { local, value } => CaptureListValue::Int {
            local: *local,
            value: state.int_values(value).to_vec(),
        },
        EvaluatedListCapture::String { local, value } => CaptureListValue::String {
            local: *local,
            value: state.string_values(value).to_vec(),
        },
        EvaluatedListCapture::Float { local, value } => CaptureListValue::Float {
            local: *local,
            value: state.float_values(value).to_vec(),
        },
        EvaluatedListCapture::Bool { local, value } => CaptureListValue::Bool {
            local: *local,
            value: state.bool_values(value).to_vec(),
        },
        EvaluatedListCapture::Nil { local, value } => CaptureListValue::Nil {
            local: *local,
            len: state.nil_len(value),
        },
        EvaluatedListCapture::Tuple { local, value } => CaptureListValue::Tuple {
            local: *local,
            item_type: plan.tuple_list_item_type(value.type_id()),
            value: state
                .tuple_values(value)
                .iter()
                .cloned()
                .map(|values| {
                    values
                        .into_iter()
                        .map(|value| self::value(plan, state, value))
                        .collect()
                })
                .collect(),
        },
        EvaluatedListCapture::List { local, value } => CaptureListValue::List {
            local: *local,
            item_type: Box::new(plan.nested_list_item_type(value.type_id())),
            value: nested_list_values(plan, state, value),
        },
        EvaluatedListCapture::Function { local, value } => CaptureListValue::Function {
            local: *local,
            item_type: plan.function_list_item_type(value.type_id()),
            value: state
                .function_values(value)
                .iter()
                .cloned()
                .map(|value| function(plan, state, value))
                .collect(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::value;
    use crate::plan::execution::{
        BoolFunctionId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId, FloatFunctionId,
        FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionListLocalId, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionId,
        ListFunctionLocal, ListListLocalId, NilFunctionId, NilFunctionLocalId, NilListLocalId,
        NilLocalId, StringFunctionId, StringFunctionLocalId, StringListLocalId, StringLocalId,
        TupleFunctionId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::evaluated::{
        EvaluatedBoolFunction, EvaluatedCapture, EvaluatedFloatFunction, EvaluatedFunctionFunction,
        EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction,
        EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedValue,
    };
    use crate::runtime::state::{ListValueId, RuntimeState};
    use crate::runtime::{CaptureListValue, CaptureValue, ListValue, Value};

    const EVERY_LIST_FAMILY_SOURCE: &str = r#"
fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }
pub fn main() { 0 }
"#;

    #[test]
    fn materializes_every_runtime_value_and_list_storage_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let int_function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );
        let int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into_core()],
        );
        let function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function)],
        );

        let actual = value(
            &plan,
            &state,
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int(1.into()),
                EvaluatedValue::Float(1.5),
                EvaluatedValue::String("one".into()),
                EvaluatedValue::Bool(true),
                EvaluatedValue::Nil,
                EvaluatedValue::Tuple(vec![EvaluatedValue::Int(1.into())]),
                EvaluatedValue::List(ListValueId::Int(int_list)),
                EvaluatedValue::List(ListValueId::String(string_list)),
                EvaluatedValue::List(ListValueId::Float(float_list)),
                EvaluatedValue::List(ListValueId::Bool(bool_list)),
                EvaluatedValue::List(ListValueId::Nil(nil_list)),
                EvaluatedValue::List(ListValueId::Tuple(tuple_list)),
                EvaluatedValue::List(ListValueId::List(nested_list)),
                EvaluatedValue::List(ListValueId::Function(function_list)),
            ]),
        );

        assert_eq!(
            actual,
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Float(1.5),
                Value::String("one".into()),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(1.into())]),
                Value::List(ListValue::int(vec![1.into()])),
                Value::List(ListValue::string(vec!["one".into()])),
                Value::List(ListValue::float(vec![1.5])),
                Value::List(ListValue::bool(vec![true])),
                Value::List(ListValue::nil(1)),
                Value::List(ListValue::from_evaluated_tuple(
                    vec![ValueType::Int],
                    vec![vec![Value::Int(1.into())]],
                )),
                Value::List(ListValue::from_evaluated_list(
                    ValueType::Int,
                    vec![ListValue::int(vec![1.into()])],
                )),
                Value::List(ListValue::from_evaluated_function(
                    FunctionType::new(Vec::new(), ValueType::Int),
                    vec![crate::runtime::FunctionValue::new(
                        crate::plan::execution::RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                        FunctionType::new(Vec::new(), ValueType::Int),
                    )],
                )),
            ]),
        );
    }

    #[test]
    fn materializes_every_function_and_capture_family() {
        let plan = crate::runtime::plan_src(EVERY_LIST_FAMILY_SOURCE);
        let mut state = RuntimeState::new();
        let execution_int_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let module_int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let int_function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            execution_int_type.clone(),
        );
        let float_function = EvaluatedFloatFunction::new(
            FloatFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Float,
            ),
        );
        let string_function = EvaluatedStringFunction::new(
            StringFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::String,
            ),
        );
        let bool_function = EvaluatedBoolFunction::new(
            BoolFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Bool,
            ),
        );
        let nil_function = EvaluatedNilFunction::new(
            NilFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Nil,
            ),
        );
        let tuple_function = EvaluatedTupleFunction::new(
            TupleFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Tuple(vec![
                    crate::plan::execution::ValueType::Int,
                ]),
            ),
        );
        let list_function_id = ListFunctionId::Int(plan.int_list_function_id(0));
        let list_function = EvaluatedListFunction::new(
            list_function_id.clone(),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::List(
                    plan.int_list_function_id(0).type_id().list_type(),
                ),
            ),
        );
        let function_function = EvaluatedFunctionFunction::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Function(Box::new(execution_int_type.clone())),
            ),
        );
        let int_list = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let string_list = state.string(
            plan.string_list_function_id(0).type_id(),
            vec!["one".into()],
        );
        let float_list = state.float(plan.float_list_function_id(0).type_id(), vec![1.5]);
        let bool_list = state.bool(plan.bool_list_function_id(0).type_id(), vec![true]);
        let nil_list = state.nil(plan.nil_list_function_id(0).type_id(), 1);
        let tuple_list = state.tuple(
            plan.tuple_list_function_id(0).type_id(),
            vec![vec![EvaluatedValue::Int(1.into())]],
        );
        let nested_child = state.int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let nested_list = state.list(
            plan.list_list_function_id(0).type_id(),
            vec![nested_child.into_core()],
        );
        let function_list = state.function(
            plan.function_list_function_id(0).type_id(),
            vec![EvaluatedFunctionValue::from(int_function.clone())],
        );
        let list_function_local = ListFunctionLocal::Int {
            local: IntListFunctionLocalId(0),
            type_: execution_int_type.clone(),
            list_type: plan.int_list_function_id(0).type_id(),
        };

        let captures = [
            EvaluatedCapture::int(IntLocalId(0), 1.into()),
            EvaluatedCapture::float(FloatLocalId(0), 1.5),
            EvaluatedCapture::string(StringLocalId(0), "one".into()),
            EvaluatedCapture::bool(BoolLocalId(0), true),
            EvaluatedCapture::nil(NilLocalId(0)),
            EvaluatedCapture::tuple(TupleLocalId(0), vec![EvaluatedValue::Int(1.into())]),
            EvaluatedCapture::list(EvaluatedListCapture::Int {
                local: IntListLocalId(0),
                value: int_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::String {
                local: StringListLocalId(0),
                value: string_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Float {
                local: FloatListLocalId(0),
                value: float_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Bool {
                local: BoolListLocalId(0),
                value: bool_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Nil {
                local: NilListLocalId(0),
                value: nil_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Tuple {
                local: TupleListLocalId(0),
                value: tuple_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::List {
                local: ListListLocalId(0),
                value: nested_list,
            }),
            EvaluatedCapture::list(EvaluatedListCapture::Function {
                local: FunctionListLocalId(0),
                value: function_list,
            }),
            EvaluatedCapture::int_function(IntFunctionLocalId(0), int_function.clone()),
            EvaluatedCapture::float_function(FloatFunctionLocalId(0), float_function.clone()),
            EvaluatedCapture::string_function(StringFunctionLocalId(0), string_function.clone()),
            EvaluatedCapture::bool_function(BoolFunctionLocalId(0), bool_function.clone()),
            EvaluatedCapture::nil_function(NilFunctionLocalId(0), nil_function.clone()),
            EvaluatedCapture::tuple_function(TupleFunctionLocalId(0), tuple_function.clone()),
            EvaluatedCapture::list_function(list_function_local.clone(), list_function.clone()),
            EvaluatedCapture::function_function(
                FunctionFunctionLocalId(0),
                function_function.clone(),
            ),
        ];
        let expected = [
            CaptureValue::int(IntLocalId(0), 1.into()),
            CaptureValue::float(FloatLocalId(0), 1.5),
            CaptureValue::string(StringLocalId(0), "one".into()),
            CaptureValue::bool(BoolLocalId(0), true),
            CaptureValue::nil(NilLocalId(0)),
            CaptureValue::tuple(TupleLocalId(0), vec![Value::Int(1.into())]),
            CaptureValue::list(CaptureListValue::Int {
                local: IntListLocalId(0),
                value: vec![1.into()],
            }),
            CaptureValue::list(CaptureListValue::String {
                local: StringListLocalId(0),
                value: vec!["one".into()],
            }),
            CaptureValue::list(CaptureListValue::Float {
                local: FloatListLocalId(0),
                value: vec![1.5],
            }),
            CaptureValue::list(CaptureListValue::Bool {
                local: BoolListLocalId(0),
                value: vec![true],
            }),
            CaptureValue::list(CaptureListValue::Nil {
                local: NilListLocalId(0),
                len: 1,
            }),
            CaptureValue::list(CaptureListValue::Tuple {
                local: TupleListLocalId(0),
                item_type: vec![ValueType::Int],
                value: vec![vec![Value::Int(1.into())]],
            }),
            CaptureValue::list(CaptureListValue::List {
                local: ListListLocalId(0),
                item_type: Box::new(ValueType::Int),
                value: vec![ListValue::int(vec![1.into()])],
            }),
            CaptureValue::list(CaptureListValue::Function {
                local: FunctionListLocalId(0),
                item_type: module_int_type.clone(),
                value: vec![crate::runtime::FunctionValue::new(
                    crate::plan::execution::RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::new(),
                    module_int_type.clone(),
                )],
            }),
            CaptureValue::int_function(
                IntFunctionLocalId(0),
                crate::runtime::IntFunctionValue::new(
                    IntFunctionId(0),
                    Vec::new(),
                    module_int_type.clone(),
                ),
            ),
            CaptureValue::float_function(
                FloatFunctionLocalId(0),
                crate::runtime::FloatFunctionValue::new_with_captures(
                    FloatFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Float),
                ),
            ),
            CaptureValue::string_function(
                StringFunctionLocalId(0),
                crate::runtime::StringFunctionValue::new_with_captures(
                    StringFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::String),
                ),
            ),
            CaptureValue::bool_function(
                BoolFunctionLocalId(0),
                crate::runtime::BoolFunctionValue::new_with_captures(
                    BoolFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Bool),
                ),
            ),
            CaptureValue::nil_function(
                NilFunctionLocalId(0),
                crate::runtime::NilFunctionValue::new_with_captures(
                    NilFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Nil),
                ),
            ),
            CaptureValue::tuple_function(
                TupleFunctionLocalId(0),
                crate::runtime::TupleFunctionValue::from_evaluated(
                    TupleFunctionId(0),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                ),
            ),
            CaptureValue::list_function(
                list_function_local,
                crate::runtime::ListFunctionValue::new_with_captures(
                    list_function_id,
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                ),
            ),
            CaptureValue::function_function(
                FunctionFunctionLocalId(0),
                crate::runtime::FunctionFunctionValue::from_evaluated(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Function(Box::new(module_int_type))),
                ),
            ),
        ];

        for (capture, expected) in captures.iter().zip(expected) {
            assert_eq!(super::capture(&plan, &state, capture), expected);
        }

        let functions = [
            EvaluatedFunctionValue::from(int_function),
            EvaluatedFunctionValue::from(float_function),
            EvaluatedFunctionValue::from(string_function),
            EvaluatedFunctionValue::from(bool_function),
            EvaluatedFunctionValue::from(nil_function),
            EvaluatedFunctionValue::from(tuple_function),
            EvaluatedFunctionValue::from(list_function),
            EvaluatedFunctionValue::from(function_function),
        ];
        for function in functions {
            let expected_type = ValueType::Function(Box::new(plan.function_type(function.type_())));
            let materialized = value(&plan, &state, EvaluatedValue::Function(function));
            assert_eq!(materialized.value_type(), expected_type);
        }
    }

    #[test]
    fn materializes_function_captures_through_the_function_owner() {
        let plan =
            crate::runtime::plan_src("fn identity(value: Int) { value } pub fn main() { 0 }");
        let state = RuntimeState::new();
        let function_type = crate::plan::execution::FunctionType::new(
            vec![crate::plan::execution::ValueType::Int],
            crate::plan::execution::ValueType::Int,
        );
        let function = EvaluatedIntFunction::new(
            IntFunctionId(0),
            vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
            vec![EvaluatedCapture::int(IntLocalId(1), 42.into())],
            function_type,
        );

        assert_eq!(
            value(
                &plan,
                &state,
                EvaluatedValue::Function(EvaluatedFunctionValue::from(function)),
            ),
            Value::Function(crate::runtime::FunctionValue::from(
                crate::runtime::IntFunctionValue::new_with_captures(
                    IntFunctionId(0),
                    vec![crate::plan::execution::ParamLocal::Int(IntLocalId(0))],
                    vec![CaptureValue::int(IntLocalId(1), 42.into())],
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            )),
        );
    }
}
