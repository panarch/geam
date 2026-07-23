use super::FunctionLabel;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionId, BoolListFunctionId,
    CustomFunctionId, CustomListFunctionId, FloatFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntFunctionId, IntListFunctionId, ListListFunctionId, NeverFunctionId,
    NilFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    StringFunctionId, StringListFunctionId, TupleFunctionId, TupleListFunctionId,
    UtfCodepointFunctionId, UtfCodepointListFunctionId,
};

pub(in super::super) trait ExplainFunctionId {
    fn label(&self) -> FunctionLabel;
}

impl ExplainFunctionId for NeverFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("never", self.0)
    }
}

impl ExplainFunctionId for IntFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("int", self.0)
    }
}

impl ExplainFunctionId for FloatFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("float", self.0)
    }
}

impl ExplainFunctionId for StringFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("string", self.0)
    }
}

impl ExplainFunctionId for BitArrayFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("bit_array", self.0)
    }
}

impl ExplainFunctionId for UtfCodepointFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("utf_codepoint", self.0)
    }
}

impl ExplainFunctionId for CustomFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("custom", self.index())
    }
}

impl ExplainFunctionId for BoolFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("bool", self.0)
    }
}

impl ExplainFunctionId for NilFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("nil", self.0)
    }
}

impl ExplainFunctionId for TupleFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("tuple", self.0)
    }
}

impl ExplainFunctionId for ParameterListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.parameter", self.index())
    }
}

impl ExplainFunctionId for ParameterListListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.parameter_list", self.index())
    }
}

impl ExplainFunctionId for IntListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.int", self.index())
    }
}

impl ExplainFunctionId for StringListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.string", self.index())
    }
}

impl ExplainFunctionId for BitArrayListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.bit_array", self.index())
    }
}

impl ExplainFunctionId for UtfCodepointListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.utf_codepoint", self.index())
    }
}

impl ExplainFunctionId for CustomListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.custom", self.index())
    }
}

impl ExplainFunctionId for FloatListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.float", self.index())
    }
}

impl ExplainFunctionId for BoolListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.bool", self.index())
    }
}

impl ExplainFunctionId for NilListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.nil", self.index())
    }
}

impl ExplainFunctionId for TupleListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.tuple", self.index())
    }
}

impl ExplainFunctionId for ListListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.list", self.index())
    }
}

impl ExplainFunctionId for FunctionListFunctionId {
    fn label(&self) -> FunctionLabel {
        FunctionLabel::new("list.function", self.index())
    }
}

#[cfg(test)]
mod tests {
    use super::ExplainFunctionId;
    use crate::plan::execution::{
        BitArrayFunctionId, BitArrayListFunctionId, BitArrayListTypeId, BoolFunctionId,
        BoolListFunctionId, BoolListTypeId, CustomFunctionId, CustomListFunctionId,
        CustomListTypeId, CustomTypeId, CustomValueShape, CustomValueShapeId, FloatFunctionId,
        FloatListFunctionId, FloatListTypeId, FunctionListFunctionId, FunctionListTypeId,
        IntFunctionId, IntListFunctionId, IntListTypeId, ListListFunctionId, ListListTypeId,
        ListTypeId, NeverFunctionId, NilFunctionId, NilListFunctionId, NilListTypeId,
        ParameterListFunctionId, ParameterListListFunctionId, ParameterListListTypeId,
        ParameterListTypeId, StringFunctionId, StringListFunctionId, StringListTypeId,
        TupleFunctionId, TupleListFunctionId, TupleListTypeId, UtfCodepointFunctionId,
        UtfCodepointListFunctionId, UtfCodepointListTypeId,
    };

    #[test]
    fn writes_every_direct_function_id_family_explicitly() {
        let list_type = ListTypeId::new(0);
        let parameter_type = ParameterListTypeId::new(list_type, crate::plan::TypeParameterId(0));
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
        assert_function(
            &ParameterListFunctionId::new(10, parameter_type),
            "list.parameter#10",
        );
        assert_function(
            &ParameterListListFunctionId::new(
                11,
                ParameterListListTypeId::new(list_type, parameter_type),
            ),
            "list.parameter_list#11",
        );
        assert_function(
            &IntListFunctionId::new(12, IntListTypeId::new(list_type)),
            "list.int#12",
        );
        assert_function(
            &StringListFunctionId::new(13, StringListTypeId::new(list_type)),
            "list.string#13",
        );
        assert_function(
            &BitArrayListFunctionId::new(14, BitArrayListTypeId::new(list_type)),
            "list.bit_array#14",
        );
        assert_function(
            &UtfCodepointListFunctionId::new(15, UtfCodepointListTypeId::new(list_type)),
            "list.utf_codepoint#15",
        );
        assert_function(
            &CustomListFunctionId::new(16, CustomListTypeId::new(list_type, custom_type)),
            "list.custom#16",
        );
        assert_function(
            &FloatListFunctionId::new(17, FloatListTypeId::new(list_type)),
            "list.float#17",
        );
        assert_function(
            &BoolListFunctionId::new(18, BoolListTypeId::new(list_type)),
            "list.bool#18",
        );
        assert_function(
            &NilListFunctionId::new(19, NilListTypeId::new(list_type)),
            "list.nil#19",
        );
        assert_function(
            &TupleListFunctionId::new(20, TupleListTypeId::new(list_type, 0)),
            "list.tuple#20",
        );
        assert_function(
            &ListListFunctionId::new(21, ListListTypeId::new(list_type, list_type)),
            "list.list#21",
        );
        assert_function(
            &FunctionListFunctionId::new(22, FunctionListTypeId::new(list_type, 0)),
            "list.function#22",
        );
    }

    fn assert_function(function: &impl ExplainFunctionId, expected: &str) {
        super::super::super::assert_written(expected, |output| {
            function.label().push_to(output);
        });
    }
}
