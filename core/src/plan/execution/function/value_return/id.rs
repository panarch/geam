use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::type_::{CustomValueShape, ExternalTypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeverFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtfCodepointFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomFunctionId {
    pub index: usize,
    pub return_shape: CustomValueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalFunctionId {
    pub index: usize,
    pub return_type: ExternalTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NilFunctionId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleFunctionId(pub usize);

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

    pub(crate) fn with_index(self, index: usize) -> Self {
        Self {
            index,
            return_shape: self.return_shape,
        }
    }
}

impl ExternalFunctionId {
    pub(in crate::plan::execution) fn new(index: usize, return_type: ExternalTypeId) -> Self {
        Self { index, return_type }
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }

    pub(in crate::plan::execution) fn return_type(self) -> ExternalTypeId {
        self.return_type
    }

    pub(crate) fn with_index(self, index: usize) -> Self {
        Self {
            index,
            return_type: self.return_type,
        }
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

impl FunctionLabelSource for ExternalFunctionId {
    fn function_label(&self) -> FunctionLabel {
        FunctionLabel::new("external", self.index())
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

impl Emit for NeverFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::NeverFunctionId", &[field_0]);
    }
}

impl Emit for IntFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::IntFunctionId", &[field_0]);
    }
}

impl Emit for FloatFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::FloatFunctionId", &[field_0]);
    }
}

impl Emit for StringFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::StringFunctionId", &[field_0]);
    }
}

impl Emit for BitArrayFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::BitArrayFunctionId", &[field_0]);
    }
}

impl Emit for UtfCodepointFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::UtfCodepointFunctionId", &[field_0]);
    }
}

impl Emit for CustomFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            index,
            return_shape,
        } = self;
        output.structure(
            "function::CustomFunctionId",
            &[("index", index), ("return_shape", return_shape)],
        );
    }
}

impl Emit for ExternalFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self { index, return_type } = self;
        output.structure(
            "function::ExternalFunctionId",
            &[("index", index), ("return_type", return_type)],
        );
    }
}

impl Emit for BoolFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::BoolFunctionId", &[field_0]);
    }
}

impl Emit for NilFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::NilFunctionId", &[field_0]);
    }
}

impl Emit for TupleFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("function::TupleFunctionId", &[field_0]);
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{
        BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
        IntFunctionId, NeverFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
        UtfCodepointFunctionId,
    };
    use crate::plan::execution::explain;
    use crate::plan::execution::function::FunctionLabelSource;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalTypeId,
    };

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
        assert_function(
            &ExternalFunctionId::new(7, ExternalTypeId::new(0)),
            "external#7",
        );
        assert_function(&BoolFunctionId(8), "bool#8");
        assert_function(&NilFunctionId(9), "nil#9");
        assert_function(&TupleFunctionId(10), "tuple#10");
    }

    fn assert_function(function: &impl FunctionLabelSource, expected: &str) {
        explain::assert_written(expected, |output| function.function_label().write(output));
    }
}
