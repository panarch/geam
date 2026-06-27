use crate::plan::{
    BoolFunctionLocalId, BoolFunctionValue, BoolLocalId, FrameLayout, FunctionFunctionLocalId,
    FunctionFunctionValue, IntFunctionLocalId, IntFunctionValue, IntLocalId, NilFunctionLocalId,
    NilFunctionValue, NilLocalId, StringFunctionLocalId, StringFunctionValue, StringLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

pub(super) struct Frame {
    ints: Vec<BigInt>,
    strings: Vec<EcoString>,
    bools: Vec<bool>,
    int_functions: HashMap<IntFunctionLocalId, IntFunctionValue>,
    string_functions: HashMap<StringFunctionLocalId, StringFunctionValue>,
    bool_functions: HashMap<BoolFunctionLocalId, BoolFunctionValue>,
    nil_functions: HashMap<NilFunctionLocalId, NilFunctionValue>,
    function_functions: HashMap<FunctionFunctionLocalId, FunctionFunctionValue>,
}

impl Frame {
    pub(super) fn new(layout: FrameLayout) -> Self {
        Self {
            ints: vec![BigInt::from(0); layout.ints()],
            strings: vec![EcoString::default(); layout.strings()],
            bools: vec![false; layout.bools()],
            int_functions: HashMap::with_capacity(layout.int_functions()),
            string_functions: HashMap::with_capacity(layout.string_functions()),
            bool_functions: HashMap::with_capacity(layout.bool_functions()),
            nil_functions: HashMap::with_capacity(layout.nil_functions()),
            function_functions: HashMap::with_capacity(layout.function_functions()),
        }
    }

    pub(super) fn set_int(&mut self, local: IntLocalId, value: BigInt) {
        set_slot(&mut self.ints, local.0, value);
    }

    pub(super) fn get_int(&self, local: IntLocalId) -> BigInt {
        self.ints[local.0].clone()
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

    pub(super) fn set_int_function(&mut self, local: IntFunctionLocalId, value: IntFunctionValue) {
        self.int_functions.insert(local, value);
    }

    pub(super) fn get_int_function(&self, local: IntFunctionLocalId) -> IntFunctionValue {
        self.int_functions[&local].clone()
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
        BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue, BoolLocalId, FrameLayout,
        IntFunctionId, IntFunctionLocalId, IntFunctionValue, IntLocalId, NilFunctionId,
        NilFunctionLocalId, NilFunctionValue, NilLocalId, ParamLocal, StringFunctionId,
        StringFunctionLocalId, StringFunctionValue, StringLocalId,
    };
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let frame = frame_with_layout(1, 1, 1, 1);
        let mut frame = frame;
        let int_function = int_function_value();
        let string_function = string_function_value();
        let bool_function = bool_function_value();
        let nil_function = nil_function_value();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_string(StringLocalId(0), "geam".into());
        frame.set_bool(BoolLocalId(0), true);
        frame.set_nil(NilLocalId(0));
        frame.set_int_function(IntFunctionLocalId(0), int_function.clone());
        frame.set_string_function(StringFunctionLocalId(0), string_function.clone());
        frame.set_bool_function(BoolFunctionLocalId(0), bool_function.clone());
        frame.set_nil_function(NilFunctionLocalId(0), nil_function.clone());

        assert_eq!(frame.get_int(IntLocalId(0)), int(1));
        assert_eq!(frame.get_string(StringLocalId(0)), "geam");
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
        assert_eq!(frame.get_int_function(IntFunctionLocalId(0)), int_function);
        assert_eq!(
            frame.get_string_function(StringFunctionLocalId(0)),
            string_function,
        );
        assert_eq!(
            frame.get_bool_function(BoolFunctionLocalId(0)),
            bool_function,
        );
        assert_eq!(frame.get_nil_function(NilFunctionLocalId(0)), nil_function);
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = frame_with_layout(1, 0, 0, 0);

        frame.set_int(IntLocalId(0), int(1));
        frame.set_int(IntLocalId(0), int(2));
        frame.set_int_function(IntFunctionLocalId(0), int_function_value());
        frame.set_int_function(IntFunctionLocalId(0), other_int_function_value());

        assert_eq!(frame.get_int(IntLocalId(0)), int(2));
        assert_eq!(
            frame.get_int_function(IntFunctionLocalId(0)),
            other_int_function_value(),
        );
    }

    fn frame_with_layout(ints: usize, strings: usize, bools: usize, nils: usize) -> Frame {
        let mut layout = FrameLayout::default();
        if ints > 0 {
            layout.include_int(IntLocalId(ints - 1));
        }
        if strings > 0 {
            layout.include_string(StringLocalId(strings - 1));
        }
        if bools > 0 {
            layout.include_bool(BoolLocalId(bools - 1));
        }
        if nils > 0 {
            layout.include_nil(NilLocalId(nils - 1));
        }
        layout.include_int_function(IntFunctionLocalId(0));
        layout.include_string_function(StringFunctionLocalId(0));
        layout.include_bool_function(BoolFunctionLocalId(0));
        layout.include_nil_function(NilFunctionLocalId(0));
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
}
