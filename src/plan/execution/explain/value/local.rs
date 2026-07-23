use super::super::super::graph::FunctionLocal;
use super::super::super::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
    FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, ListLocal, NeverFunctionLocal, NilFunctionLocalId,
    NilListLocalId, NilLocalId, ParamLocal, ParameterListListLocalId, ParameterListLocalId,
    StoredListLocal, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use super::write_list;
use std::convert::Infallible;

pub(in crate::plan::execution::explain) trait ExplainLocal {
    fn write_local(&self, output: &mut String);
}

pub(in crate::plan::execution::explain) fn write_locals(
    output: &mut String,
    locals: &[ParamLocal],
) {
    write_list(output, locals, |output, local| local.write_local(output));
}

impl ExplainLocal for IntLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "int", self.0);
    }
}

impl ExplainLocal for FloatLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "float", self.0);
    }
}

impl ExplainLocal for StringLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "string", self.0);
    }
}

impl ExplainLocal for BitArrayLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "bit_array", self.0);
    }
}

impl ExplainLocal for UtfCodepointLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "utf_codepoint", self.0);
    }
}

impl ExplainLocal for BoolLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "bool", self.0);
    }
}

impl ExplainLocal for NilLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "nil", self.0);
    }
}

impl ExplainLocal for TupleLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "tuple", self.0);
    }
}

impl ExplainLocal for ParameterListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.parameter", self.0);
    }
}

impl ExplainLocal for ParameterListListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.parameter_list", self.0);
    }
}

impl ExplainLocal for IntListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.int", self.0);
    }
}

impl ExplainLocal for StringListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.string", self.0);
    }
}

impl ExplainLocal for BitArrayListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.bit_array", self.0);
    }
}

impl ExplainLocal for UtfCodepointListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.utf_codepoint", self.0);
    }
}

impl ExplainLocal for CustomListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.custom", self.0);
    }
}

impl ExplainLocal for FloatListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.float", self.0);
    }
}

impl ExplainLocal for BoolListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.bool", self.0);
    }
}

impl ExplainLocal for NilListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.nil", self.0);
    }
}

impl ExplainLocal for TupleListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.tuple", self.0);
    }
}

impl ExplainLocal for ListListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.list", self.0);
    }
}

impl ExplainLocal for FunctionListLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "list.function", self.0);
    }
}

impl ExplainLocal for IntFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.int", self.0);
    }
}

impl ExplainLocal for FloatFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.float", self.0);
    }
}

impl ExplainLocal for StringFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.string", self.0);
    }
}

impl ExplainLocal for BitArrayFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.bit_array", self.0);
    }
}

impl ExplainLocal for UtfCodepointFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.utf_codepoint", self.0);
    }
}

impl ExplainLocal for BoolFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.bool", self.0);
    }
}

impl ExplainLocal for NilFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.nil", self.0);
    }
}

impl ExplainLocal for TupleFunctionLocalId {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.tuple", self.0);
    }
}

impl ExplainLocal for CustomLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "custom", self.id().0);
    }
}

impl ExplainLocal for GenericFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.generic", self.id().0);
    }
}

impl ExplainLocal for NeverFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.never", self.id().0);
    }
}

impl ExplainLocal for CustomFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.custom", self.id().0);
    }
}

impl ExplainLocal for FunctionFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.function", self.id().0);
    }
}

impl ExplainLocal for ListLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Parameter { local, .. } => local.write_local(output),
            Self::ParameterList { local, .. } => local.write_local(output),
            Self::Int { local, .. } => local.write_local(output),
            Self::String { local, .. } => local.write_local(output),
            Self::BitArray { local, .. } => local.write_local(output),
            Self::UtfCodepoint { local, .. } => local.write_local(output),
            Self::Custom { local, .. } => local.write_local(output),
            Self::Float { local, .. } => local.write_local(output),
            Self::Bool { local, .. } => local.write_local(output),
            Self::Nil { local, .. } => local.write_local(output),
            Self::Tuple { local, .. } => local.write_local(output),
            Self::List { local, .. } => local.write_local(output),
            Self::Function { local, .. } => local.write_local(output),
        }
    }
}

impl ExplainLocal for StoredListLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::ParameterList(local) => local.write_local(output),
            Self::Int(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple(local) => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::Function(local) => local.write_local(output),
        }
    }
}

impl ExplainLocal for ListFunctionLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Parameter { local, .. } => {
                write_indexed(output, "function.list.parameter", local.0);
            }
            Self::ParameterList { local, .. } => {
                write_indexed(output, "function.list.parameter_list", local.0);
            }
            Self::Int { local, .. } => write_indexed(output, "function.list.int", local.0),
            Self::String { local, .. } => write_indexed(output, "function.list.string", local.0),
            Self::BitArray { local, .. } => {
                write_indexed(output, "function.list.bit_array", local.0);
            }
            Self::UtfCodepoint { local, .. } => {
                write_indexed(output, "function.list.utf_codepoint", local.0);
            }
            Self::Custom { local, .. } => write_indexed(output, "function.list.custom", local.0),
            Self::Float { local, .. } => write_indexed(output, "function.list.float", local.0),
            Self::Bool { local, .. } => write_indexed(output, "function.list.bool", local.0),
            Self::Nil { local, .. } => write_indexed(output, "function.list.nil", local.0),
            Self::Tuple { local, .. } => write_indexed(output, "function.list.tuple", local.0),
            Self::List { local, .. } => write_indexed(output, "function.list.list", local.0),
            Self::Function { local, .. } => {
                write_indexed(output, "function.list.function", local.0);
            }
        }
    }
}

impl ExplainLocal for FunctionLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Generic(local) => local.write_local(output),
            Self::Never(local) => local.write_local(output),
            Self::Int(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple(local) => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::Function(local) => local.write_local(output),
        }
    }
}

impl ExplainLocal for Infallible {
    fn write_local(&self, _output: &mut String) {
        match *self {}
    }
}

impl ExplainLocal for ParamLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Int(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple { local, .. } => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::IntFunction { local, .. } => local.write_local(output),
            Self::FloatFunction { local, .. } => local.write_local(output),
            Self::StringFunction { local, .. } => local.write_local(output),
            Self::BitArrayFunction { local, .. } => local.write_local(output),
            Self::UtfCodepointFunction { local, .. } => local.write_local(output),
            Self::GenericFunction(local) => local.write_local(output),
            Self::NeverFunction(local) => local.write_local(output),
            Self::CustomFunction(local) => local.write_local(output),
            Self::BoolFunction { local, .. } => local.write_local(output),
            Self::NilFunction { local, .. } => local.write_local(output),
            Self::TupleFunction { local, .. } => local.write_local(output),
            Self::ListFunction(local) => local.write_local(output),
            Self::FunctionFunction(local) => local.write_local(output),
        }
    }
}

fn write_indexed(output: &mut String, family: &str, index: usize) {
    output.push('%');
    output.push_str(family);
    output.push('#');
    output.push_str(&index.to_string());
}

#[cfg(test)]
mod tests {
    use super::ExplainLocal;
    use crate::plan::execution::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CustomListLocalId, FloatFunctionLocalId, FloatListLocalId,
        FloatLocalId, FunctionListLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId,
        ListListLocalId, NilFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal,
        ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId, StringListLocalId,
        StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };

    #[test]
    fn writes_every_indexed_local_family_explicitly() {
        assert_local(&IntLocalId(1), "%int#1");
        assert_local(&FloatLocalId(2), "%float#2");
        assert_local(&StringLocalId(3), "%string#3");
        assert_local(&BitArrayLocalId(4), "%bit_array#4");
        assert_local(&UtfCodepointLocalId(5), "%utf_codepoint#5");
        assert_local(&BoolLocalId(6), "%bool#6");
        assert_local(&NilLocalId(7), "%nil#7");
        assert_local(&TupleLocalId(8), "%tuple#8");
        assert_local(&ParameterListLocalId(9), "%list.parameter#9");
        assert_local(&ParameterListListLocalId(10), "%list.parameter_list#10");
        assert_local(&IntListLocalId(11), "%list.int#11");
        assert_local(&StringListLocalId(12), "%list.string#12");
        assert_local(&BitArrayListLocalId(13), "%list.bit_array#13");
        assert_local(&UtfCodepointListLocalId(14), "%list.utf_codepoint#14");
        assert_local(&CustomListLocalId(15), "%list.custom#15");
        assert_local(&FloatListLocalId(16), "%list.float#16");
        assert_local(&BoolListLocalId(17), "%list.bool#17");
        assert_local(&NilListLocalId(18), "%list.nil#18");
        assert_local(&TupleListLocalId(19), "%list.tuple#19");
        assert_local(&ListListLocalId(20), "%list.list#20");
        assert_local(&FunctionListLocalId(21), "%list.function#21");
        assert_local(&IntFunctionLocalId(22), "%function.int#22");
        assert_local(&FloatFunctionLocalId(23), "%function.float#23");
        assert_local(&StringFunctionLocalId(24), "%function.string#24");
        assert_local(&BitArrayFunctionLocalId(25), "%function.bit_array#25");
        assert_local(
            &UtfCodepointFunctionLocalId(26),
            "%function.utf_codepoint#26",
        );
        assert_local(&BoolFunctionLocalId(27), "%function.bool#27");
        assert_local(&NilFunctionLocalId(28), "%function.nil#28");
        assert_local(&TupleFunctionLocalId(29), "%function.tuple#29");
    }

    #[test]
    fn writes_local_argument_packs() {
        super::super::super::assert_written("[%int#2]", |output| {
            super::write_locals(
                output,
                &[ParamLocal::Int(crate::plan::execution::IntLocalId(2))],
            );
        });
    }

    #[test]
    fn writes_rich_and_list_function_local_families_explicitly() {
        use crate::plan::execution::{
            BitArrayListFunctionLocalId, BitArrayListTypeId, BoolListFunctionLocalId,
            BoolListTypeId, CustomFunctionLocal, CustomFunctionLocalId, CustomFunctionType,
            CustomListFunctionLocalId, CustomListTypeId, CustomLocal, CustomLocalId, CustomTypeId,
            CustomValueShape, CustomValueShapeId, FloatListFunctionLocalId, FloatListTypeId,
            FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionType,
            FunctionListFunctionLocalId, FunctionListTypeId, FunctionShape, FunctionType,
            GenericFunctionLocal, GenericFunctionLocalId, GenericFunctionType,
            IntListFunctionLocalId, IntListTypeId, ListFunctionLocal, ListListFunctionLocalId,
            ListListTypeId, ListTypeId, NeverFunctionLocal, NeverFunctionLocalId,
            NilListFunctionLocalId, NilListTypeId, ParameterListFunctionLocalId,
            ParameterListListFunctionLocalId, ParameterListListTypeId, ParameterListTypeId,
            StringListFunctionLocalId, StringListTypeId, TupleListFunctionLocalId, TupleListTypeId,
            UtfCodepointListFunctionLocalId, UtfCodepointListTypeId, ValueShapeId, ValueType,
        };

        let custom_type = CustomTypeId::new(0);
        let custom_shape = CustomValueShape::new(custom_type, CustomValueShapeId::new(0));
        let value_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_shape = FunctionShape::new(ValueShapeId::new(0), value_function_type.clone());

        assert_local(
            &CustomLocal::new(CustomLocalId(30), custom_shape),
            "%custom#30",
        );
        assert_local(
            &GenericFunctionLocal::new(
                GenericFunctionLocalId(31),
                GenericFunctionType::from_shapes(
                    value_function_type.clone(),
                    function_shape.clone(),
                ),
            ),
            "%function.generic#31",
        );
        assert_local(
            &NeverFunctionLocal::new(
                NeverFunctionLocalId(32),
                GenericFunctionType::from_shapes(
                    value_function_type.clone(),
                    function_shape.clone(),
                ),
            ),
            "%function.never#32",
        );
        assert_local(
            &CustomFunctionLocal::new(
                CustomFunctionLocalId(33),
                CustomFunctionType::from_shapes(
                    FunctionType::new(Vec::new(), ValueType::Custom(custom_type)),
                    Vec::new(),
                    custom_shape,
                ),
            ),
            "%function.custom#33",
        );
        assert_local(
            &FunctionFunctionLocal::new(
                FunctionFunctionLocalId(34),
                FunctionFunctionType::from_shapes(
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(value_function_type.clone())),
                    ),
                    Vec::new(),
                    function_shape,
                ),
            ),
            "%function.function#34",
        );

        let list_type = ListTypeId::new(0);
        let parameter_type = ParameterListTypeId::new(list_type, crate::plan::TypeParameterId(0));
        let list_function_locals = [
            (
                ListFunctionLocal::Parameter {
                    local: ParameterListFunctionLocalId(35),
                    type_: value_function_type.clone(),
                    list_type: parameter_type,
                },
                "%function.list.parameter#35",
            ),
            (
                ListFunctionLocal::ParameterList {
                    local: ParameterListListFunctionLocalId(36),
                    type_: value_function_type.clone(),
                    list_type: ParameterListListTypeId::new(list_type, parameter_type),
                },
                "%function.list.parameter_list#36",
            ),
            (
                ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(37),
                    type_: value_function_type.clone(),
                    list_type: IntListTypeId::new(list_type),
                },
                "%function.list.int#37",
            ),
            (
                ListFunctionLocal::String {
                    local: StringListFunctionLocalId(38),
                    type_: value_function_type.clone(),
                    list_type: StringListTypeId::new(list_type),
                },
                "%function.list.string#38",
            ),
            (
                ListFunctionLocal::BitArray {
                    local: BitArrayListFunctionLocalId(39),
                    type_: value_function_type.clone(),
                    list_type: BitArrayListTypeId::new(list_type),
                },
                "%function.list.bit_array#39",
            ),
            (
                ListFunctionLocal::UtfCodepoint {
                    local: UtfCodepointListFunctionLocalId(40),
                    type_: value_function_type.clone(),
                    list_type: UtfCodepointListTypeId::new(list_type),
                },
                "%function.list.utf_codepoint#40",
            ),
            (
                ListFunctionLocal::Custom {
                    local: CustomListFunctionLocalId(41),
                    type_: value_function_type.clone(),
                    list_type: CustomListTypeId::new(list_type, custom_type),
                },
                "%function.list.custom#41",
            ),
            (
                ListFunctionLocal::Float {
                    local: FloatListFunctionLocalId(42),
                    type_: value_function_type.clone(),
                    list_type: FloatListTypeId::new(list_type),
                },
                "%function.list.float#42",
            ),
            (
                ListFunctionLocal::Bool {
                    local: BoolListFunctionLocalId(43),
                    type_: value_function_type.clone(),
                    list_type: BoolListTypeId::new(list_type),
                },
                "%function.list.bool#43",
            ),
            (
                ListFunctionLocal::Nil {
                    local: NilListFunctionLocalId(44),
                    type_: value_function_type.clone(),
                    list_type: NilListTypeId::new(list_type),
                },
                "%function.list.nil#44",
            ),
            (
                ListFunctionLocal::Tuple {
                    local: TupleListFunctionLocalId(45),
                    type_: value_function_type.clone(),
                    list_type: TupleListTypeId::new(list_type, 0),
                },
                "%function.list.tuple#45",
            ),
            (
                ListFunctionLocal::List {
                    local: ListListFunctionLocalId(46),
                    type_: value_function_type.clone(),
                    list_type: ListListTypeId::new(list_type, list_type),
                },
                "%function.list.list#46",
            ),
            (
                ListFunctionLocal::Function {
                    local: FunctionListFunctionLocalId(47),
                    type_: value_function_type,
                    list_type: FunctionListTypeId::new(list_type, 0),
                },
                "%function.list.function#47",
            ),
        ];

        for (local, expected) in list_function_locals {
            assert_local(&local, expected);
        }
    }

    fn assert_local(local: &impl ExplainLocal, expected: &str) {
        super::super::super::assert_written(expected, |output| {
            local.write_local(output);
        });
    }
}
