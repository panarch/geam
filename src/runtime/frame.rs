use crate::plan::{
    BoolLocalId, FrameLayout, FunctionLocalId, FunctionValue, IntLocalId, NilLocalId, StringLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) struct Frame {
    ints: Vec<BigInt>,
    strings: Vec<EcoString>,
    bools: Vec<bool>,
    functions: Vec<Option<FunctionValue>>,
}

impl Frame {
    pub(super) fn new(layout: FrameLayout) -> Self {
        Self {
            ints: vec![BigInt::from(0); layout.ints()],
            strings: vec![EcoString::default(); layout.strings()],
            bools: vec![false; layout.bools()],
            functions: vec![None; layout.functions()],
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

    pub(super) fn set_function(&mut self, local: FunctionLocalId, value: FunctionValue) {
        set_slot(&mut self.functions, local.0, Some(value));
    }

    pub(super) fn get_function(&self, local: FunctionLocalId) -> FunctionValue {
        match &self.functions[local.0] {
            Some(value) => value.clone(),
            None => FunctionValue::new(
                crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Nil),
                crate::plan::RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                Vec::new(),
            ),
        }
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
        BoolLocalId, FrameLayout, FunctionLocalId, IntLocalId, NilLocalId, StringLocalId, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let frame = frame_with_layout(1, 1, 1, 1, 0);
        let mut frame = frame;

        frame.set_int(IntLocalId(0), int(1));
        frame.set_string(StringLocalId(0), "geam".into());
        frame.set_bool(BoolLocalId(0), true);
        frame.set_nil(NilLocalId(0));

        assert_eq!(frame.get_int(IntLocalId(0)), int(1));
        assert_eq!(frame.get_string(StringLocalId(0)), "geam");
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = frame_with_layout(1, 0, 0, 0, 0);

        frame.set_int(IntLocalId(0), int(1));
        frame.set_int(IntLocalId(0), int(2));

        assert_eq!(frame.get_int(IntLocalId(0)), int(2));
    }

    #[test]
    fn frame_get_unset_function_local() {
        let frame = frame_with_layout(0, 0, 0, 0, 1);

        assert_eq!(
            frame.get_function(FunctionLocalId(0)).type_().return_(),
            &ValueType::Nil,
        );
    }

    fn frame_with_layout(
        ints: usize,
        strings: usize,
        bools: usize,
        nils: usize,
        functions: usize,
    ) -> Frame {
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
        if functions > 0 {
            layout.include_function(FunctionLocalId(functions - 1));
        }
        Frame::new(layout)
    }

    fn int(value: i64) -> BigInt {
        BigInt::from(value)
    }
}
