use crate::plan::{
    BoolFunctionLocalId, BoolFunctionValue, BoolListLocalId, BoolLocalId, FloatFunctionLocalId,
    FloatFunctionValue, FloatListLocalId, FloatLocalId, FrameLayout, FunctionFunctionLocalId,
    FunctionFunctionValue, FunctionListLocalId, FunctionValue, IntFunctionLocalId,
    IntFunctionValue, IntListLocalId, IntLocalId, ListFunctionLocal, ListFunctionValue,
    ListListLocalId, ListLocal, ListValue, ListValueKind, NilFunctionLocalId, NilFunctionValue,
    NilListLocalId, NilLocalId, StringFunctionLocalId, StringFunctionValue, StringListLocalId,
    StringLocalId, TupleFunctionLocalId, TupleFunctionValue, TupleListLocalId, TupleLocalId, Value,
    ValueType,
};
use crate::runtime::ExecutionError;
use crate::runtime::error::ExecutionResult;
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

pub(super) struct Frame {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bools: Vec<bool>,
    tuples: Vec<Vec<Value>>,
    int_lists: Vec<Vec<BigInt>>,
    string_lists: Vec<Vec<EcoString>>,
    float_lists: Vec<Vec<f64>>,
    bool_lists: Vec<Vec<bool>>,
    nil_lists: Vec<usize>,
    tuple_lists: Vec<Vec<Vec<Value>>>,
    tuple_list_types: Vec<Vec<ValueType>>,
    list_lists: Vec<Vec<ListValue>>,
    list_list_types: Vec<ValueType>,
    function_lists: Vec<Vec<FunctionValue>>,
    function_list_types: Vec<crate::plan::FunctionType>,
    int_functions: HashMap<IntFunctionLocalId, IntFunctionValue>,
    float_functions: HashMap<FloatFunctionLocalId, FloatFunctionValue>,
    string_functions: HashMap<StringFunctionLocalId, StringFunctionValue>,
    bool_functions: HashMap<BoolFunctionLocalId, BoolFunctionValue>,
    nil_functions: HashMap<NilFunctionLocalId, NilFunctionValue>,
    tuple_functions: HashMap<TupleFunctionLocalId, TupleFunctionValue>,
    list_functions: HashMap<ListFunctionLocal, ListFunctionValue>,
    function_functions: HashMap<FunctionFunctionLocalId, FunctionFunctionValue>,
}

impl Frame {
    pub(super) fn new(layout: FrameLayout) -> Self {
        Self {
            ints: vec![BigInt::from(0); layout.ints()],
            floats: vec![0.0; layout.floats()],
            strings: vec![EcoString::default(); layout.strings()],
            bools: vec![false; layout.bools()],
            tuples: vec![Vec::new(); layout.tuples()],
            int_lists: vec![Vec::new(); layout.int_lists()],
            string_lists: vec![Vec::new(); layout.string_lists()],
            float_lists: vec![Vec::new(); layout.float_lists()],
            bool_lists: vec![Vec::new(); layout.bool_lists()],
            nil_lists: vec![0; layout.nil_lists()],
            tuple_lists: vec![Vec::new(); layout.tuple_lists().len()],
            tuple_list_types: layout.tuple_lists().to_vec(),
            list_lists: vec![Vec::new(); layout.list_lists().len()],
            list_list_types: layout.list_lists().to_vec(),
            function_lists: vec![Vec::new(); layout.function_lists().len()],
            function_list_types: layout.function_lists().to_vec(),
            int_functions: HashMap::with_capacity(layout.int_functions()),
            float_functions: HashMap::with_capacity(layout.float_functions()),
            string_functions: HashMap::with_capacity(layout.string_functions()),
            bool_functions: HashMap::with_capacity(layout.bool_functions()),
            nil_functions: HashMap::with_capacity(layout.nil_functions()),
            tuple_functions: HashMap::with_capacity(layout.tuple_functions()),
            list_functions: HashMap::with_capacity(layout.list_functions().len()),
            function_functions: HashMap::with_capacity(layout.function_functions()),
        }
    }

    pub(super) fn set_int(&mut self, local: IntLocalId, value: BigInt) {
        set_slot(&mut self.ints, local.0, value);
    }

    pub(super) fn get_int(&self, local: IntLocalId) -> BigInt {
        self.ints[local.0].clone()
    }

    pub(super) fn set_float(&mut self, local: FloatLocalId, value: f64) {
        set_slot(&mut self.floats, local.0, value);
    }

    pub(super) fn get_float(&self, local: FloatLocalId) -> f64 {
        self.floats[local.0]
    }

    pub(super) fn set_string(&mut self, local: StringLocalId, value: EcoString) {
        set_slot(&mut self.strings, local.0, value);
    }

    pub(super) fn get_string(&self, local: StringLocalId) -> EcoString {
        self.strings[local.0].clone()
    }

    pub(super) fn set_bool(&mut self, local: BoolLocalId, value: bool) {
        set_slot(&mut self.bools, local.0, value);
    }

    pub(super) fn get_bool(&self, local: BoolLocalId) -> bool {
        self.bools[local.0]
    }

    pub(super) fn set_nil(&mut self, _local: NilLocalId) {}

    pub(super) fn get_nil(&self, _local: NilLocalId) {}

    pub(super) fn set_tuple(&mut self, local: TupleLocalId, value: Vec<Value>) {
        set_slot(&mut self.tuples, local.0, value);
    }

    pub(super) fn get_tuple(&self, local: TupleLocalId) -> Vec<Value> {
        self.tuples[local.0].clone()
    }

    pub(super) fn set_int_list(&mut self, local: IntListLocalId, value: Vec<BigInt>) {
        set_slot(&mut self.int_lists, local.0, value);
    }

    pub(super) fn get_int_list(&self, local: IntListLocalId) -> Vec<BigInt> {
        self.int_lists[local.0].clone()
    }

    pub(super) fn set_string_list(&mut self, local: StringListLocalId, value: Vec<EcoString>) {
        set_slot(&mut self.string_lists, local.0, value);
    }

    pub(super) fn get_string_list(&self, local: StringListLocalId) -> Vec<EcoString> {
        self.string_lists[local.0].clone()
    }

    pub(super) fn set_float_list(&mut self, local: FloatListLocalId, value: Vec<f64>) {
        set_slot(&mut self.float_lists, local.0, value);
    }

    pub(super) fn get_float_list(&self, local: FloatListLocalId) -> Vec<f64> {
        self.float_lists[local.0].clone()
    }

    pub(super) fn set_bool_list(&mut self, local: BoolListLocalId, value: Vec<bool>) {
        set_slot(&mut self.bool_lists, local.0, value);
    }

    pub(super) fn get_bool_list(&self, local: BoolListLocalId) -> Vec<bool> {
        self.bool_lists[local.0].clone()
    }

    pub(super) fn set_nil_list(&mut self, local: NilListLocalId, len: usize) {
        set_slot(&mut self.nil_lists, local.0, len);
    }

    pub(super) fn get_nil_list(&self, local: NilListLocalId) -> usize {
        self.nil_lists[local.0]
    }

    pub(super) fn set_tuple_list(&mut self, local: TupleListLocalId, value: Vec<Vec<Value>>) {
        set_slot(&mut self.tuple_lists, local.0, value);
    }

    pub(super) fn get_tuple_list(&self, local: TupleListLocalId) -> Vec<Vec<Value>> {
        self.tuple_lists[local.0].clone()
    }

    pub(super) fn set_list_list(&mut self, local: ListListLocalId, value: Vec<ListValue>) {
        set_slot(&mut self.list_lists, local.0, value);
    }

    pub(super) fn get_list_list(&self, local: ListListLocalId) -> Vec<ListValue> {
        self.list_lists[local.0].clone()
    }

    pub(super) fn set_function_list(
        &mut self,
        local: FunctionListLocalId,
        value: Vec<FunctionValue>,
    ) {
        set_slot(&mut self.function_lists, local.0, value);
    }

    pub(super) fn get_function_list(&self, local: FunctionListLocalId) -> Vec<FunctionValue> {
        self.function_lists[local.0].clone()
    }

    pub(super) fn get_list(&self, local: &ListLocal) -> ExecutionResult<ListValue> {
        match local {
            ListLocal::Int(local) => Ok(ListValue::int(self.get_int_list(*local))),
            ListLocal::String(local) => Ok(ListValue::string(self.get_string_list(*local))),
            ListLocal::Float(local) => Ok(ListValue::float(self.get_float_list(*local))),
            ListLocal::Bool(local) => Ok(ListValue::bool(self.get_bool_list(*local))),
            ListLocal::Nil(local) => Ok(ListValue::nil(self.get_nil_list(*local))),
            ListLocal::Tuple { local, item_type } => {
                let actual = self.tuple_list_types[local.0].clone();
                if &actual != item_type {
                    return Err(ExecutionError::list_item_type_mismatch(
                        ValueType::Tuple(item_type.clone()),
                        ValueType::Tuple(actual),
                    ));
                }
                Ok(ListValue::tuple(
                    item_type.clone(),
                    self.get_tuple_list(*local),
                ))
            }
            ListLocal::List { local, item_type } => {
                let actual = self.list_list_types[local.0].clone();
                if actual != item_type.as_ref().clone() {
                    return Err(ExecutionError::list_item_type_mismatch(
                        ValueType::List(item_type.clone()),
                        ValueType::List(Box::new(actual)),
                    ));
                }
                Ok(ListValue::list(
                    item_type.as_ref().clone(),
                    self.get_list_list(*local),
                ))
            }
            ListLocal::Function { local, item_type } => {
                let actual = self.function_list_types[local.0].clone();
                if &actual != item_type {
                    return Err(ExecutionError::list_item_type_mismatch(
                        ValueType::Function(Box::new(item_type.clone())),
                        ValueType::Function(Box::new(actual)),
                    ));
                }
                Ok(ListValue::function(
                    item_type.clone(),
                    self.get_function_list(*local),
                ))
            }
        }
    }

    pub(super) fn set_list(&mut self, local: &ListLocal, value: ListValue) -> ExecutionResult<()> {
        let expected = local.item_type();
        let actual = value.item_type();
        if let Some(actual) = value.item_value_mismatch() {
            return Err(ExecutionError::list_item_type_mismatch(expected, actual));
        }

        match (local, value.into_kind()) {
            (ListLocal::Int(local), ListValueKind::Int(value)) => self.set_int_list(*local, value),
            (ListLocal::String(local), ListValueKind::String(value)) => {
                self.set_string_list(*local, value)
            }
            (ListLocal::Float(local), ListValueKind::Float(value)) => {
                self.set_float_list(*local, value)
            }
            (ListLocal::Bool(local), ListValueKind::Bool(value)) => {
                self.set_bool_list(*local, value)
            }
            (ListLocal::Nil(local), ListValueKind::Nil(value)) => self.set_nil_list(*local, value),
            (ListLocal::Tuple { local, .. }, ListValueKind::Tuple { values, .. }) => {
                self.set_tuple_list(*local, values)
            }
            (ListLocal::List { local, .. }, ListValueKind::List { values, .. }) => {
                self.set_list_list(*local, values)
            }
            (ListLocal::Function { local, .. }, ListValueKind::Function { values, .. }) => {
                self.set_function_list(*local, values)
            }
            _ => return Err(ExecutionError::list_item_type_mismatch(expected, actual)),
        }

        Ok(())
    }

    pub(super) fn set_int_function(&mut self, local: IntFunctionLocalId, value: IntFunctionValue) {
        self.int_functions.insert(local, value);
    }

    pub(super) fn get_int_function(&self, local: IntFunctionLocalId) -> IntFunctionValue {
        self.int_functions[&local].clone()
    }

    pub(super) fn set_float_function(
        &mut self,
        local: FloatFunctionLocalId,
        value: FloatFunctionValue,
    ) {
        self.float_functions.insert(local, value);
    }

    pub(super) fn get_float_function(&self, local: FloatFunctionLocalId) -> FloatFunctionValue {
        self.float_functions[&local].clone()
    }

    pub(super) fn set_string_function(
        &mut self,
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    ) {
        self.string_functions.insert(local, value);
    }

    pub(super) fn get_string_function(&self, local: StringFunctionLocalId) -> StringFunctionValue {
        self.string_functions[&local].clone()
    }

    pub(super) fn set_bool_function(
        &mut self,
        local: BoolFunctionLocalId,
        value: BoolFunctionValue,
    ) {
        self.bool_functions.insert(local, value);
    }

    pub(super) fn get_bool_function(&self, local: BoolFunctionLocalId) -> BoolFunctionValue {
        self.bool_functions[&local].clone()
    }

    pub(super) fn set_nil_function(&mut self, local: NilFunctionLocalId, value: NilFunctionValue) {
        self.nil_functions.insert(local, value);
    }

    pub(super) fn get_nil_function(&self, local: NilFunctionLocalId) -> NilFunctionValue {
        self.nil_functions[&local].clone()
    }

    pub(super) fn set_tuple_function(
        &mut self,
        local: TupleFunctionLocalId,
        value: TupleFunctionValue,
    ) {
        self.tuple_functions.insert(local, value);
    }

    pub(super) fn get_tuple_function(&self, local: TupleFunctionLocalId) -> TupleFunctionValue {
        self.tuple_functions[&local].clone()
    }

    pub(super) fn set_list_function(&mut self, local: ListFunctionLocal, value: ListFunctionValue) {
        self.list_functions.insert(local, value);
    }

    pub(super) fn get_list_function(&self, local: &ListFunctionLocal) -> ListFunctionValue {
        self.list_functions[local].clone()
    }

    pub(super) fn set_function_function(
        &mut self,
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    ) {
        self.function_functions.insert(local, value);
    }

    pub(super) fn get_function_function(
        &self,
        local: FunctionFunctionLocalId,
    ) -> FunctionFunctionValue {
        self.function_functions[&local].clone()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new(FrameLayout::default())
    }
}

fn set_slot<T>(slots: &mut [T], index: usize, value: T) {
    slots[index] = value;
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::plan::{
        BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue, BoolListLocalId, BoolLocalId,
        FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue, FloatListLocalId, FloatLocalId,
        FrameLayout, FunctionListLocalId, FunctionType, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntListLocalId, IntLocalId, ListListLocalId, ListLocal, ListValue,
        NilFunctionId, NilFunctionLocalId, NilFunctionValue, NilListLocalId, NilLocalId,
        ParamLocal, StringFunctionId, StringFunctionLocalId, StringFunctionValue,
        StringListLocalId, StringLocalId, TupleFunctionId, TupleFunctionLocalId,
        TupleFunctionValue, TupleListLocalId, TupleLocalId, Value, ValueType,
    };
    use crate::runtime::ExecutionError;
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let frame = frame_with_all_slots();
        let mut frame = frame;
        let int_function = int_function_value();
        let float_function = float_function_value();
        let string_function = string_function_value();
        let bool_function = bool_function_value();
        let nil_function = nil_function_value();
        let tuple_function = tuple_function_value();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_float(FloatLocalId(0), 1.5);
        frame.set_string(StringLocalId(0), "geam".into());
        frame.set_bool(BoolLocalId(0), true);
        frame.set_nil(NilLocalId(0));
        frame.set_tuple(TupleLocalId(0), vec![Value::Int(1.into())]);
        frame.set_int_function(IntFunctionLocalId(0), int_function.clone());
        frame.set_float_function(FloatFunctionLocalId(0), float_function.clone());
        frame.set_string_function(StringFunctionLocalId(0), string_function.clone());
        frame.set_bool_function(BoolFunctionLocalId(0), bool_function.clone());
        frame.set_nil_function(NilFunctionLocalId(0), nil_function.clone());
        frame.set_tuple_function(TupleFunctionLocalId(0), tuple_function.clone());

        assert_eq!(frame.get_int(IntLocalId(0)), int(1));
        assert_eq!(frame.get_float(FloatLocalId(0)), 1.5);
        assert_eq!(frame.get_string(StringLocalId(0)), "geam");
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_tuple(TupleLocalId(0)), vec![Value::Int(1.into())]);
        assert_eq!(frame.get_int_function(IntFunctionLocalId(0)), int_function);
        assert_eq!(
            frame.get_float_function(FloatFunctionLocalId(0)),
            float_function,
        );
        assert_eq!(
            frame.get_string_function(StringFunctionLocalId(0)),
            string_function,
        );
        assert_eq!(
            frame.get_bool_function(BoolFunctionLocalId(0)),
            bool_function,
        );
        assert_eq!(frame.get_nil_function(NilFunctionLocalId(0)), nil_function);
        assert_eq!(
            frame.get_tuple_function(TupleFunctionLocalId(0)),
            tuple_function,
        );
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = frame_with_overwrite_slots();
        let _ = Frame::default();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_int(IntLocalId(0), int(2));
        frame.set_float(FloatLocalId(0), 1.0);
        frame.set_float(FloatLocalId(0), 2.0);
        frame.set_tuple(TupleLocalId(0), vec![Value::Int(1.into())]);
        frame.set_tuple(TupleLocalId(0), vec![Value::Int(2.into())]);
        frame.set_int_function(IntFunctionLocalId(0), int_function_value());
        frame.set_int_function(IntFunctionLocalId(0), other_int_function_value());
        frame.set_float_function(FloatFunctionLocalId(0), float_function_value());
        frame.set_float_function(FloatFunctionLocalId(0), other_float_function_value());
        frame.set_tuple_function(TupleFunctionLocalId(0), tuple_function_value());
        frame.set_tuple_function(TupleFunctionLocalId(0), other_tuple_function_value());

        assert_eq!(frame.get_int(IntLocalId(0)), int(2));
        assert_eq!(frame.get_float(FloatLocalId(0)), 2.0);
        assert_eq!(frame.get_tuple(TupleLocalId(0)), vec![Value::Int(2.into())]);
        assert_eq!(
            frame.get_int_function(IntFunctionLocalId(0)),
            other_int_function_value(),
        );
        assert_eq!(
            frame.get_float_function(FloatFunctionLocalId(0)),
            other_float_function_value(),
        );
        assert_eq!(
            frame.get_tuple_function(TupleFunctionLocalId(0)),
            other_tuple_function_value(),
        );
    }

    #[test]
    fn frame_initializes_list_slots_from_layout_item_types() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let mut layout = FrameLayout::default();
        layout.include_list(ListLocal::int(IntListLocalId(0)));
        layout.include_list(ListLocal::string(StringListLocalId(0)));
        layout.include_list(ListLocal::float(FloatListLocalId(0)));
        layout.include_list(ListLocal::bool(BoolListLocalId(0)));
        layout.include_list(ListLocal::nil(NilListLocalId(0)));
        layout.include_list(ListLocal::tuple(
            TupleListLocalId(0),
            vec![ValueType::String],
        ));
        layout.include_list(ListLocal::list(ListListLocalId(0), ValueType::Float));
        layout.include_list(ListLocal::function(
            FunctionListLocalId(0),
            function_type.clone(),
        ));

        let frame = Frame::new(layout);

        assert_eq!(
            frame.get_list(&ListLocal::int(IntListLocalId(0))),
            Ok(ListValue::empty(ValueType::Int))
        );
        assert_eq!(
            frame.get_list(&ListLocal::string(StringListLocalId(0))),
            Ok(ListValue::empty(ValueType::String))
        );
        assert_eq!(
            frame.get_list(&ListLocal::float(FloatListLocalId(0))),
            Ok(ListValue::empty(ValueType::Float))
        );
        assert_eq!(
            frame.get_list(&ListLocal::bool(BoolListLocalId(0))),
            Ok(ListValue::empty(ValueType::Bool))
        );
        assert_eq!(
            frame.get_list(&ListLocal::nil(NilListLocalId(0))),
            Ok(ListValue::empty(ValueType::Nil))
        );
        assert_eq!(
            frame.get_list(&ListLocal::tuple(
                TupleListLocalId(0),
                vec![ValueType::String],
            )),
            Ok(ListValue::empty(ValueType::Tuple(vec![ValueType::String]))),
        );
        assert_eq!(
            frame.get_list(&ListLocal::list(ListListLocalId(0), ValueType::Float)),
            Ok(ListValue::empty(ValueType::List(Box::new(
                ValueType::Float
            )))),
        );
        assert_eq!(
            frame.get_list(&ListLocal::function(
                FunctionListLocalId(0),
                function_type.clone()
            )),
            Ok(ListValue::empty(ValueType::Function(Box::new(
                function_type
            )))),
        );
    }

    #[test]
    fn frame_set_list_preserves_typed_family_slots() {
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let int = ListLocal::int(IntListLocalId(0));
        let string = ListLocal::string(StringListLocalId(0));
        let float = ListLocal::float(FloatListLocalId(0));
        let bool_ = ListLocal::bool(BoolListLocalId(0));
        let nil = ListLocal::nil(NilListLocalId(0));
        let tuple = ListLocal::tuple(TupleListLocalId(0), vec![ValueType::String]);
        let list = ListLocal::list(ListListLocalId(0), ValueType::Float);
        let function = ListLocal::function(FunctionListLocalId(0), function_type.clone());
        let mut layout = FrameLayout::default();
        layout.include_list(&int);
        layout.include_list(&string);
        layout.include_list(&float);
        layout.include_list(&bool_);
        layout.include_list(&nil);
        layout.include_list(&tuple);
        layout.include_list(&list);
        layout.include_list(&function);
        let mut frame = Frame::new(layout);

        assert_eq!(frame.set_list(&int, ListValue::int(vec![1.into()])), Ok(()));
        assert_eq!(
            frame.set_list(&string, ListValue::string(vec!["one".into()])),
            Ok(()),
        );
        assert_eq!(frame.set_list(&float, ListValue::float(vec![1.5])), Ok(()));
        assert_eq!(frame.set_list(&bool_, ListValue::bool(vec![true])), Ok(()));
        assert_eq!(frame.set_list(&nil, ListValue::nil(1)), Ok(()));
        assert_eq!(
            frame.set_list(
                &tuple,
                ListValue::tuple(
                    vec![ValueType::String],
                    vec![vec![Value::String("one".into())]]
                ),
            ),
            Ok(()),
        );
        assert_eq!(
            frame.set_list(
                &list,
                ListValue::list(ValueType::Float, vec![ListValue::float(vec![1.5])]),
            ),
            Ok(()),
        );
        assert_eq!(
            frame.set_list(&function, ListValue::function(function_type, Vec::new())),
            Ok(()),
        );

        assert_eq!(frame.get_list(&int), Ok(ListValue::int(vec![1.into()])));
        assert_eq!(
            frame.get_list(&string),
            Ok(ListValue::string(vec!["one".into()]))
        );
        assert_eq!(frame.get_list(&float), Ok(ListValue::float(vec![1.5])));
        assert_eq!(frame.get_list(&bool_), Ok(ListValue::bool(vec![true])));
        assert_eq!(frame.get_list(&nil), Ok(ListValue::nil(1)));
        assert_eq!(
            frame.get_list(&tuple),
            Ok(ListValue::tuple(
                vec![ValueType::String],
                vec![vec![Value::String("one".into())]]
            )),
        );
        assert_eq!(
            frame.get_list(&list),
            Ok(ListValue::list(
                ValueType::Float,
                vec![ListValue::float(vec![1.5])]
            )),
        );
        assert_eq!(
            frame
                .get_list(&function)
                .expect("function list should be set")
                .len(),
            0
        );
    }

    #[test]
    fn frame_set_list_rejects_mismatched_item_type() {
        let mut layout = FrameLayout::default();
        let local = ListLocal::int(IntListLocalId(0));
        layout.include_list(local.clone());
        let mut frame = Frame::new(layout);

        assert_eq!(
            frame.set_list(&local, ListValue::string(vec!["wrong".into()])),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn frame_get_list_rejects_mismatched_item_metadata() {
        let tuple_slot = ListLocal::tuple(TupleListLocalId(0), vec![ValueType::String]);
        let wrong_tuple_metadata = ListLocal::tuple(TupleListLocalId(0), vec![ValueType::Int]);
        let list_slot = ListLocal::list(ListListLocalId(0), ValueType::String);
        let wrong_list_metadata = ListLocal::list(ListListLocalId(0), ValueType::Int);
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let wrong_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_slot = ListLocal::function(FunctionListLocalId(0), function_type.clone());
        let wrong_function_metadata =
            ListLocal::function(FunctionListLocalId(0), wrong_function_type.clone());
        let mut layout = FrameLayout::default();
        layout.include_list(tuple_slot.clone());
        layout.include_list(list_slot.clone());
        layout.include_list(function_slot.clone());
        let frame = Frame::new(layout);

        assert_eq!(
            frame.get_list(&wrong_tuple_metadata),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Tuple(vec![ValueType::Int]),
                ValueType::Tuple(vec![ValueType::String]),
            )),
        );
        assert_eq!(
            frame.get_list(&wrong_list_metadata),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::List(Box::new(ValueType::String)),
            )),
        );
        assert_eq!(
            frame.get_list(&wrong_function_metadata),
            Err(ExecutionError::list_item_type_mismatch(
                ValueType::Function(Box::new(wrong_function_type)),
                ValueType::Function(Box::new(function_type)),
            )),
        );
    }

    fn frame_with_all_slots() -> Frame {
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        layout.include_float(FloatLocalId(0));
        layout.include_string(StringLocalId(0));
        layout.include_bool(BoolLocalId(0));
        layout.include_nil(NilLocalId(0));
        layout.include_tuple(TupleLocalId(0));
        layout.include_int_function(IntFunctionLocalId(0));
        layout.include_float_function(FloatFunctionLocalId(0));
        layout.include_string_function(StringFunctionLocalId(0));
        layout.include_bool_function(BoolFunctionLocalId(0));
        layout.include_nil_function(NilFunctionLocalId(0));
        layout.include_tuple_function(TupleFunctionLocalId(0));
        Frame::new(layout)
    }

    fn frame_with_overwrite_slots() -> Frame {
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        layout.include_float(FloatLocalId(0));
        layout.include_tuple(TupleLocalId(0));
        layout.include_int_function(IntFunctionLocalId(0));
        layout.include_float_function(FloatFunctionLocalId(0));
        layout.include_tuple_function(TupleFunctionLocalId(0));
        Frame::new(layout)
    }

    fn int(value: i64) -> BigInt {
        BigInt::from(value)
    }

    fn int_function_value() -> IntFunctionValue {
        IntFunctionValue::new(IntFunctionId(0), vec![ParamLocal::int(IntLocalId(0))])
    }

    fn other_int_function_value() -> IntFunctionValue {
        IntFunctionValue::new(IntFunctionId(1), vec![ParamLocal::int(IntLocalId(0))])
    }

    fn float_function_value() -> FloatFunctionValue {
        FloatFunctionValue::new(FloatFunctionId(0), vec![ParamLocal::float(FloatLocalId(0))])
    }

    fn other_float_function_value() -> FloatFunctionValue {
        FloatFunctionValue::new(FloatFunctionId(1), vec![ParamLocal::float(FloatLocalId(0))])
    }

    fn string_function_value() -> StringFunctionValue {
        StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        )
    }

    fn bool_function_value() -> BoolFunctionValue {
        BoolFunctionValue::new(BoolFunctionId(0), vec![ParamLocal::bool(BoolLocalId(0))])
    }

    fn nil_function_value() -> NilFunctionValue {
        NilFunctionValue::new(NilFunctionId(0), vec![ParamLocal::int(IntLocalId(0))])
    }

    fn tuple_function_value() -> TupleFunctionValue {
        TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
            vec![ValueType::Int],
        )
    }

    fn other_tuple_function_value() -> TupleFunctionValue {
        TupleFunctionValue::new(
            TupleFunctionId(1),
            vec![ParamLocal::int(IntLocalId(0))],
            vec![ValueType::Int],
        )
    }
}
