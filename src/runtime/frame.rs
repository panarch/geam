use crate::plan::{
    BoolFunctionLocalId, BoolFunctionValue, BoolListLocalId, BoolLocalId, FloatFunctionLocalId,
    FloatFunctionValue, FloatListLocalId, FloatLocalId, FrameLayout, FunctionFunctionLocalId,
    FunctionFunctionValue, FunctionListLocalId, FunctionValue, IntFunctionLocalId,
    IntFunctionValue, IntListLocalId, IntLocalId, ListFunctionLocal, ListFunctionValue,
    ListListLocalId, ListValue, NilFunctionLocalId, NilFunctionValue, NilListLocalId, NilLocalId,
    StringFunctionLocalId, StringFunctionValue, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleFunctionValue, TupleListLocalId, TupleLocalId, Value,
};
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
    list_lists: Vec<Vec<ListValue>>,
    function_lists: Vec<Vec<FunctionValue>>,
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
            list_lists: vec![Vec::new(); layout.list_lists().len()],
            function_lists: vec![Vec::new(); layout.function_lists().len()],
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

        assert_eq!(frame.get_int_list(IntListLocalId(0)), Vec::<BigInt>::new());
        assert_eq!(
            frame.get_string_list(StringListLocalId(0)),
            Vec::<ecow::EcoString>::new(),
        );
        assert_eq!(frame.get_float_list(FloatListLocalId(0)), Vec::<f64>::new());
        assert_eq!(frame.get_bool_list(BoolListLocalId(0)), Vec::<bool>::new());
        assert_eq!(frame.get_nil_list(NilListLocalId(0)), 0);
        assert_eq!(
            frame.get_tuple_list(TupleListLocalId(0)),
            Vec::<Vec<Value>>::new(),
        );
        assert_eq!(
            frame.get_list_list(ListListLocalId(0)),
            Vec::<ListValue>::new()
        );
        assert_eq!(
            frame.get_function_list(FunctionListLocalId(0)),
            Vec::<crate::plan::FunctionValue>::new(),
        );
        assert_eq!(
            ValueType::Function(Box::new(function_type)),
            ListLocal::function(
                FunctionListLocalId(0),
                FunctionType::new(vec![ValueType::Int], ValueType::String)
            )
            .item_type(),
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

        frame.set_int_list(IntListLocalId(0), vec![1.into()]);
        frame.set_string_list(StringListLocalId(0), vec!["one".into()]);
        frame.set_float_list(FloatListLocalId(0), vec![1.5]);
        frame.set_bool_list(BoolListLocalId(0), vec![true]);
        frame.set_nil_list(NilListLocalId(0), 1);
        frame.set_tuple_list(TupleListLocalId(0), vec![vec![Value::String("one".into())]]);
        frame.set_list_list(ListListLocalId(0), vec![ListValue::float(vec![1.5])]);
        frame.set_function_list(FunctionListLocalId(0), Vec::new());

        assert_eq!(frame.get_int_list(IntListLocalId(0)), vec![1.into()]);
        assert_eq!(frame.get_string_list(StringListLocalId(0)), vec!["one"]);
        assert_eq!(frame.get_float_list(FloatListLocalId(0)), vec![1.5]);
        assert_eq!(frame.get_bool_list(BoolListLocalId(0)), vec![true]);
        assert_eq!(frame.get_nil_list(NilListLocalId(0)), 1);
        assert_eq!(
            frame.get_tuple_list(TupleListLocalId(0)),
            vec![vec![Value::String("one".into())]],
        );
        assert_eq!(
            frame.get_list_list(ListListLocalId(0)),
            vec![ListValue::float(vec![1.5])],
        );
        assert_eq!(
            frame.get_function_list(FunctionListLocalId(0)),
            Vec::<crate::plan::FunctionValue>::new(),
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
