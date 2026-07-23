use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::type_::CustomValueShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NeverFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomFunctionId {
    index: usize,
    return_shape: CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionId(pub(crate) usize);

impl CustomFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, return_shape: CustomValueShape) -> Self {
        Self {
            index,
            return_shape,
        }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl FunctionLabelSource for NeverFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("never", self.0)
    }
}

impl FunctionLabelSource for IntFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("int", self.0)
    }
}

impl FunctionLabelSource for FloatFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("float", self.0)
    }
}

impl FunctionLabelSource for StringFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("string", self.0)
    }
}

impl FunctionLabelSource for BitArrayFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("bit_array", self.0)
    }
}

impl FunctionLabelSource for UtfCodepointFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("utf_codepoint", self.0)
    }
}

impl FunctionLabelSource for CustomFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("custom", self.index())
    }
}

impl FunctionLabelSource for BoolFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("bool", self.0)
    }
}

impl FunctionLabelSource for NilFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("nil", self.0)
    }
}

impl FunctionLabelSource for TupleFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("tuple", self.0)
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{
        BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, IntFunctionId,
        NeverFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::explain;
    use crate::plan::execution::function::FunctionLabelSource;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};

    #[test]
    fn writes_every_direct_function_id_family_explicitly() {
        let custom_type = CustomTypeId::new(0);
        let custom_shape = CustomValueShape::new(custom_type, CustomValueShapeId::new(0));

        assert_function(&NeverFunctionId(0), "never#0");
        assert_function(&IntFunctionId(1), "int#1");
        assert_function(&FloatFunctionId(2), "float#2");
        assert_function(&StringFunctionId(3), "string#3");
        assert_function(&BitArrayFunctionId(4), "bit_array#4");
        assert_function(&UtfCodepointFunctionId(5), "utf_codepoint#5");
        assert_function(&CustomFunctionId::new(6, custom_shape), "custom#6");
        assert_function(&BoolFunctionId(7), "bool#7");
        assert_function(&NilFunctionId(8), "nil#8");
        assert_function(&TupleFunctionId(9), "tuple#9");
    }

    fn assert_function(function: &impl FunctionLabelSource, expected: &str) {
        explain::assert_written(expected, |output| function.function_label().write(output));
    }
}
