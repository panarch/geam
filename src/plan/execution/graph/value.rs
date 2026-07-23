mod function;
mod list;
mod local;
mod param;

pub(crate) use function::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BoolFunctionLocalId,
    BoolListFunctionLocalId, CustomFunctionLocal, CustomFunctionLocalId, CustomListFunctionLocalId,
    FloatFunctionLocalId, FloatListFunctionLocalId, FunctionFunctionLocal, FunctionFunctionLocalId,
    FunctionListFunctionLocalId, FunctionLocal, GenericFunctionLocal, GenericFunctionLocalId,
    IntFunctionLocalId, IntListFunctionLocalId, ListFunctionLocal, ListListFunctionLocalId,
    NeverFunctionLocal, NeverFunctionLocalId, NilFunctionLocalId, NilListFunctionLocalId,
    ParameterListFunctionLocalId, ParameterListListFunctionLocalId, StringFunctionLocalId,
    StringListFunctionLocalId, TupleFunctionLocalId, TupleListFunctionLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId,
};
pub(crate) use list::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, FloatListLocalId, FunctionListLocalId,
    IntListLocalId, ListListLocalId, ListLocal, NilListLocalId, ParameterListListLocalId,
    ParameterListLocalId, StoredListLocal, StringListLocalId, TupleListLocalId,
    UtfCodepointListLocalId,
};
pub(crate) use local::{
    BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, FloatLocalId, IntLocalId, NilLocalId,
    StringLocalId, TupleLocalId, UtfCodepointLocalId,
};
pub(crate) use param::{ParamLocal, ParamSlot};

use crate::plan::execution::explain::{Explain, ExplainContext};
use std::convert::Infallible;

impl<Value> Explain for Value
where
    Value: LocalLabel,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.write_local_label(context.output());
    }
}

fn write_list<Value>(
    output: &mut String,
    values: &[Value],
    mut write_value: impl FnMut(&mut String, &Value),
) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_value(output, value);
    }
    output.push(']');
}

pub(in crate::plan::execution) trait LocalLabel {
    fn write_local_label(&self, output: &mut String);
}

pub(in crate::plan::execution) fn write_local_labels(output: &mut String, locals: &[ParamLocal]) {
    write_list(output, locals, |output, local| {
        local.write_local_label(output)
    });
}

impl LocalLabel for IntLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "int", self.0);
    }
}

impl LocalLabel for FloatLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "float", self.0);
    }
}

impl LocalLabel for StringLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "string", self.0);
    }
}

impl LocalLabel for BitArrayLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "bit_array", self.0);
    }
}

impl LocalLabel for UtfCodepointLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "utf_codepoint", self.0);
    }
}

impl LocalLabel for BoolLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "bool", self.0);
    }
}

impl LocalLabel for NilLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "nil", self.0);
    }
}

impl LocalLabel for TupleLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "tuple", self.0);
    }
}

impl LocalLabel for ParameterListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.parameter", self.0);
    }
}

impl LocalLabel for ParameterListListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.parameter_list", self.0);
    }
}

impl LocalLabel for IntListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.int", self.0);
    }
}

impl LocalLabel for StringListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.string", self.0);
    }
}

impl LocalLabel for BitArrayListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.bit_array", self.0);
    }
}

impl LocalLabel for UtfCodepointListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.utf_codepoint", self.0);
    }
}

impl LocalLabel for CustomListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.custom", self.0);
    }
}

impl LocalLabel for FloatListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.float", self.0);
    }
}

impl LocalLabel for BoolListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.bool", self.0);
    }
}

impl LocalLabel for NilListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.nil", self.0);
    }
}

impl LocalLabel for TupleListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.tuple", self.0);
    }
}

impl LocalLabel for ListListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.list", self.0);
    }
}

impl LocalLabel for FunctionListLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "list.function", self.0);
    }
}

impl LocalLabel for IntFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.int", self.0);
    }
}

impl LocalLabel for FloatFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.float", self.0);
    }
}

impl LocalLabel for StringFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.string", self.0);
    }
}

impl LocalLabel for BitArrayFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.bit_array", self.0);
    }
}

impl LocalLabel for UtfCodepointFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.utf_codepoint", self.0);
    }
}

impl LocalLabel for BoolFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.bool", self.0);
    }
}

impl LocalLabel for NilFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.nil", self.0);
    }
}

impl LocalLabel for TupleFunctionLocalId {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.tuple", self.0);
    }
}

impl LocalLabel for CustomLocal {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "custom", self.id().0);
    }
}

impl LocalLabel for GenericFunctionLocal {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.generic", self.id().0);
    }
}

impl LocalLabel for NeverFunctionLocal {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.never", self.id().0);
    }
}

impl LocalLabel for CustomFunctionLocal {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.custom", self.id().0);
    }
}

impl LocalLabel for FunctionFunctionLocal {
    fn write_local_label(&self, output: &mut String) {
        write_indexed(output, "function.function", self.id().0);
    }
}

impl LocalLabel for ListLocal {
    fn write_local_label(&self, output: &mut String) {
        match self {
            Self::Parameter { local, .. } => local.write_local_label(output),
            Self::ParameterList { local, .. } => local.write_local_label(output),
            Self::Int { local, .. } => local.write_local_label(output),
            Self::String { local, .. } => local.write_local_label(output),
            Self::BitArray { local, .. } => local.write_local_label(output),
            Self::UtfCodepoint { local, .. } => local.write_local_label(output),
            Self::Custom { local, .. } => local.write_local_label(output),
            Self::Float { local, .. } => local.write_local_label(output),
            Self::Bool { local, .. } => local.write_local_label(output),
            Self::Nil { local, .. } => local.write_local_label(output),
            Self::Tuple { local, .. } => local.write_local_label(output),
            Self::List { local, .. } => local.write_local_label(output),
            Self::Function { local, .. } => local.write_local_label(output),
        }
    }
}

impl LocalLabel for StoredListLocal {
    fn write_local_label(&self, output: &mut String) {
        match self {
            Self::ParameterList(local) => local.write_local_label(output),
            Self::Int(local) => local.write_local_label(output),
            Self::String(local) => local.write_local_label(output),
            Self::BitArray(local) => local.write_local_label(output),
            Self::UtfCodepoint(local) => local.write_local_label(output),
            Self::Custom(local) => local.write_local_label(output),
            Self::Float(local) => local.write_local_label(output),
            Self::Bool(local) => local.write_local_label(output),
            Self::Nil(local) => local.write_local_label(output),
            Self::Tuple(local) => local.write_local_label(output),
            Self::List(local) => local.write_local_label(output),
            Self::Function(local) => local.write_local_label(output),
        }
    }
}

impl LocalLabel for ListFunctionLocal {
    fn write_local_label(&self, output: &mut String) {
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

impl LocalLabel for FunctionLocal {
    fn write_local_label(&self, output: &mut String) {
        match self {
            Self::Generic(local) => local.write_local_label(output),
            Self::Never(local) => local.write_local_label(output),
            Self::Int(local) => local.write_local_label(output),
            Self::Float(local) => local.write_local_label(output),
            Self::String(local) => local.write_local_label(output),
            Self::BitArray(local) => local.write_local_label(output),
            Self::UtfCodepoint(local) => local.write_local_label(output),
            Self::Custom(local) => local.write_local_label(output),
            Self::Bool(local) => local.write_local_label(output),
            Self::Nil(local) => local.write_local_label(output),
            Self::Tuple(local) => local.write_local_label(output),
            Self::List(local) => local.write_local_label(output),
            Self::Function(local) => local.write_local_label(output),
        }
    }
}

impl LocalLabel for Infallible {
    fn write_local_label(&self, _output: &mut String) {
        match *self {}
    }
}

impl LocalLabel for ParamLocal {
    fn write_local_label(&self, output: &mut String) {
        match self {
            Self::Int(local) => local.write_local_label(output),
            Self::Float(local) => local.write_local_label(output),
            Self::String(local) => local.write_local_label(output),
            Self::BitArray(local) => local.write_local_label(output),
            Self::UtfCodepoint(local) => local.write_local_label(output),
            Self::Custom(local) => local.write_local_label(output),
            Self::Bool(local) => local.write_local_label(output),
            Self::Nil(local) => local.write_local_label(output),
            Self::Tuple { local, .. } => local.write_local_label(output),
            Self::List(local) => local.write_local_label(output),
            Self::IntFunction { local, .. } => local.write_local_label(output),
            Self::FloatFunction { local, .. } => local.write_local_label(output),
            Self::StringFunction { local, .. } => local.write_local_label(output),
            Self::BitArrayFunction { local, .. } => local.write_local_label(output),
            Self::UtfCodepointFunction { local, .. } => local.write_local_label(output),
            Self::GenericFunction(local) => local.write_local_label(output),
            Self::NeverFunction(local) => local.write_local_label(output),
            Self::CustomFunction(local) => local.write_local_label(output),
            Self::BoolFunction { local, .. } => local.write_local_label(output),
            Self::NilFunction { local, .. } => local.write_local_label(output),
            Self::TupleFunction { local, .. } => local.write_local_label(output),
            Self::ListFunction(local) => local.write_local_label(output),
            Self::FunctionFunction(local) => local.write_local_label(output),
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
mod explain_tests {
    use super::LocalLabel;
    use crate::plan::execution::explain;
    use crate::plan::execution::graph::{
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
        explain::assert_written("[%int#2]", |output| {
            super::write_local_labels(
                output,
                &[ParamLocal::Int(crate::plan::execution::graph::IntLocalId(
                    2,
                ))],
            );
        });
    }

    #[test]
    fn writes_a_local_through_the_explain_protocol() {
        let source = "pub fn main() { 1 }";
        let expected = "%int#3";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&IntLocalId(3));
        });
    }

    #[test]
    fn writes_rich_local_label_implementations_explicitly() {
        use crate::plan::execution::graph::{
            CustomFunctionLocal, CustomFunctionLocalId, CustomLocal, CustomLocalId,
            FunctionFunctionLocal, FunctionFunctionLocalId, GenericFunctionLocal,
            GenericFunctionLocalId, NeverFunctionLocal, NeverFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            CustomFunctionType, CustomTypeId, CustomValueShape, CustomValueShapeId,
            FunctionFunctionType, FunctionShape, FunctionType, GenericFunctionType, ValueShapeId,
            ValueType,
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
    }

    #[test]
    fn writes_every_list_function_local_family_explicitly() {
        use crate::plan::execution::graph::{
            BitArrayListFunctionLocalId, BoolListFunctionLocalId, CustomListFunctionLocalId,
            FloatListFunctionLocalId, FunctionListFunctionLocalId, IntListFunctionLocalId,
            ListFunctionLocal, ListListFunctionLocalId, NilListFunctionLocalId,
            ParameterListFunctionLocalId, ParameterListListFunctionLocalId,
            StringListFunctionLocalId, TupleListFunctionLocalId, UtfCodepointListFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
            FunctionListTypeId, FunctionType, IntListTypeId, ListListTypeId, ListTypeId,
            NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
            TupleListTypeId, UtfCodepointListTypeId, ValueType,
        };

        let custom_type = CustomTypeId::new(0);
        let value_function_type = FunctionType::new(Vec::new(), ValueType::Int);
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

    #[test]
    fn writes_every_list_local_family_explicitly() {
        use crate::plan::TypeParameterId;
        use crate::plan::execution::graph::{ListLocal, ParameterListListLocalId};
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
            FunctionListTypeId, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
            ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
            UtfCodepointListTypeId,
        };

        let list_type = ListTypeId::new(0);
        let parameter_type = ParameterListTypeId::new(list_type, TypeParameterId(0));
        let cases = [
            (
                ListLocal::Parameter {
                    local: ParameterListLocalId(0),
                    type_id: parameter_type,
                },
                "%list.parameter#0",
            ),
            (
                ListLocal::ParameterList {
                    local: ParameterListListLocalId(1),
                    type_id: ParameterListListTypeId::new(list_type, parameter_type),
                },
                "%list.parameter_list#1",
            ),
            (
                ListLocal::Int {
                    local: IntListLocalId(2),
                    type_id: IntListTypeId::new(list_type),
                },
                "%list.int#2",
            ),
            (
                ListLocal::String {
                    local: StringListLocalId(3),
                    type_id: StringListTypeId::new(list_type),
                },
                "%list.string#3",
            ),
            (
                ListLocal::BitArray {
                    local: BitArrayListLocalId(4),
                    type_id: BitArrayListTypeId::new(list_type),
                },
                "%list.bit_array#4",
            ),
            (
                ListLocal::UtfCodepoint {
                    local: UtfCodepointListLocalId(5),
                    type_id: UtfCodepointListTypeId::new(list_type),
                },
                "%list.utf_codepoint#5",
            ),
            (
                ListLocal::Custom {
                    local: CustomListLocalId(6),
                    type_id: CustomListTypeId::new(list_type, CustomTypeId::new(0)),
                },
                "%list.custom#6",
            ),
            (
                ListLocal::Float {
                    local: FloatListLocalId(7),
                    type_id: FloatListTypeId::new(list_type),
                },
                "%list.float#7",
            ),
            (
                ListLocal::Bool {
                    local: BoolListLocalId(8),
                    type_id: BoolListTypeId::new(list_type),
                },
                "%list.bool#8",
            ),
            (
                ListLocal::Nil {
                    local: NilListLocalId(9),
                    type_id: NilListTypeId::new(list_type),
                },
                "%list.nil#9",
            ),
            (
                ListLocal::Tuple {
                    local: TupleListLocalId(10),
                    type_id: TupleListTypeId::new(list_type, 0),
                },
                "%list.tuple#10",
            ),
            (
                ListLocal::List {
                    local: ListListLocalId(11),
                    type_id: ListListTypeId::new(list_type, list_type),
                },
                "%list.list#11",
            ),
            (
                ListLocal::Function {
                    local: FunctionListLocalId(12),
                    type_id: FunctionListTypeId::new(list_type, 0),
                },
                "%list.function#12",
            ),
        ];

        for (local, expected) in cases {
            assert_local(&local, expected);
        }
    }

    #[test]
    fn writes_every_stored_list_local_family_explicitly() {
        use crate::plan::execution::graph::{ParameterListListLocalId, StoredListLocal};

        let stored_cases = [
            (
                StoredListLocal::ParameterList(ParameterListListLocalId(13)),
                "%list.parameter_list#13",
            ),
            (StoredListLocal::Int(IntListLocalId(14)), "%list.int#14"),
            (
                StoredListLocal::String(StringListLocalId(15)),
                "%list.string#15",
            ),
            (
                StoredListLocal::BitArray(BitArrayListLocalId(16)),
                "%list.bit_array#16",
            ),
            (
                StoredListLocal::UtfCodepoint(UtfCodepointListLocalId(17)),
                "%list.utf_codepoint#17",
            ),
            (
                StoredListLocal::Custom(CustomListLocalId(18)),
                "%list.custom#18",
            ),
            (
                StoredListLocal::Float(FloatListLocalId(19)),
                "%list.float#19",
            ),
            (StoredListLocal::Bool(BoolListLocalId(20)), "%list.bool#20"),
            (StoredListLocal::Nil(NilListLocalId(21)), "%list.nil#21"),
            (
                StoredListLocal::Tuple(TupleListLocalId(22)),
                "%list.tuple#22",
            ),
            (StoredListLocal::List(ListListLocalId(23)), "%list.list#23"),
            (
                StoredListLocal::Function(FunctionListLocalId(24)),
                "%list.function#24",
            ),
        ];

        for (local, expected) in stored_cases {
            assert_local(&local, expected);
        }
    }

    #[test]
    fn writes_every_function_local_family_explicitly() {
        use crate::plan::execution::graph::{
            CustomFunctionLocal, CustomFunctionLocalId, FunctionFunctionLocal,
            FunctionFunctionLocalId, FunctionLocal, GenericFunctionLocal, GenericFunctionLocalId,
            ListFunctionLocal, NeverFunctionLocal, NeverFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            CustomFunctionType, CustomTypeId, CustomValueShape, CustomValueShapeId,
            FunctionFunctionType, FunctionShape, FunctionType, GenericFunctionType, IntListTypeId,
            ListTypeId, ValueShapeId, ValueType,
        };

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_shape = FunctionShape::new(ValueShapeId::new(0), function_type.clone());
        let generic_type =
            GenericFunctionType::from_shapes(function_type.clone(), function_shape.clone());
        let custom_type = CustomTypeId::new(0);
        let custom_shape = CustomValueShape::new(custom_type, CustomValueShapeId::new(0));
        let cases = [
            (
                FunctionLocal::Generic(GenericFunctionLocal::new(
                    GenericFunctionLocalId(0),
                    generic_type.clone(),
                )),
                "%function.generic#0",
            ),
            (
                FunctionLocal::Never(NeverFunctionLocal::new(
                    NeverFunctionLocalId(1),
                    generic_type,
                )),
                "%function.never#1",
            ),
            (FunctionLocal::Int(IntFunctionLocalId(2)), "%function.int#2"),
            (
                FunctionLocal::Float(FloatFunctionLocalId(3)),
                "%function.float#3",
            ),
            (
                FunctionLocal::String(StringFunctionLocalId(4)),
                "%function.string#4",
            ),
            (
                FunctionLocal::BitArray(BitArrayFunctionLocalId(5)),
                "%function.bit_array#5",
            ),
            (
                FunctionLocal::UtfCodepoint(UtfCodepointFunctionLocalId(6)),
                "%function.utf_codepoint#6",
            ),
            (
                FunctionLocal::Custom(CustomFunctionLocal::new(
                    CustomFunctionLocalId(7),
                    CustomFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::Custom(custom_type)),
                        Vec::new(),
                        custom_shape,
                    ),
                )),
                "%function.custom#7",
            ),
            (
                FunctionLocal::Bool(BoolFunctionLocalId(8)),
                "%function.bool#8",
            ),
            (FunctionLocal::Nil(NilFunctionLocalId(9)), "%function.nil#9"),
            (
                FunctionLocal::Tuple(TupleFunctionLocalId(10)),
                "%function.tuple#10",
            ),
            (
                FunctionLocal::List(ListFunctionLocal::Int {
                    local: crate::plan::execution::graph::IntListFunctionLocalId(11),
                    type_: function_type.clone(),
                    list_type: IntListTypeId::new(ListTypeId::new(0)),
                }),
                "%function.list.int#11",
            ),
            (
                FunctionLocal::Function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(12),
                    FunctionFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::Function(Box::new(function_type))),
                        Vec::new(),
                        function_shape,
                    ),
                )),
                "%function.function#12",
            ),
        ];

        for (local, expected) in cases {
            assert_local(&local, expected);
        }
    }

    #[test]
    fn writes_every_parameter_local_family_explicitly() {
        use crate::plan::execution::graph::{
            CustomFunctionLocal, CustomFunctionLocalId, CustomLocal, CustomLocalId,
            FunctionFunctionLocal, FunctionFunctionLocalId, GenericFunctionLocal,
            GenericFunctionLocalId, IntListFunctionLocalId, ListFunctionLocal, ListLocal,
            NeverFunctionLocal, NeverFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            CustomFunctionType, CustomTypeId, CustomValueShape, CustomValueShapeId,
            FunctionFunctionType, FunctionShape, FunctionType, GenericFunctionType, IntListTypeId,
            ListTypeId, ValueShapeId, ValueType,
        };

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_shape = FunctionShape::new(ValueShapeId::new(0), function_type.clone());
        let generic_type =
            GenericFunctionType::from_shapes(function_type.clone(), function_shape.clone());
        let custom_type = CustomTypeId::new(0);
        let custom_shape = CustomValueShape::new(custom_type, CustomValueShapeId::new(0));
        let list_type = IntListTypeId::new(ListTypeId::new(0));
        let cases = [
            (ParamLocal::Int(IntLocalId(0)), "%int#0"),
            (ParamLocal::Float(FloatLocalId(1)), "%float#1"),
            (ParamLocal::String(StringLocalId(2)), "%string#2"),
            (ParamLocal::BitArray(BitArrayLocalId(3)), "%bit_array#3"),
            (
                ParamLocal::UtfCodepoint(UtfCodepointLocalId(4)),
                "%utf_codepoint#4",
            ),
            (
                ParamLocal::Custom(CustomLocal::new(CustomLocalId(5), custom_shape)),
                "%custom#5",
            ),
            (ParamLocal::Bool(BoolLocalId(6)), "%bool#6"),
            (ParamLocal::Nil(NilLocalId(7)), "%nil#7"),
            (
                ParamLocal::Tuple {
                    local: TupleLocalId(8),
                    type_: vec![ValueType::Int],
                },
                "%tuple#8",
            ),
            (
                ParamLocal::List(ListLocal::Int {
                    local: IntListLocalId(9),
                    type_id: list_type,
                }),
                "%list.int#9",
            ),
            (
                ParamLocal::IntFunction {
                    local: IntFunctionLocalId(10),
                    type_: function_type.clone(),
                },
                "%function.int#10",
            ),
            (
                ParamLocal::FloatFunction {
                    local: FloatFunctionLocalId(11),
                    type_: function_type.clone(),
                },
                "%function.float#11",
            ),
            (
                ParamLocal::StringFunction {
                    local: StringFunctionLocalId(12),
                    type_: function_type.clone(),
                },
                "%function.string#12",
            ),
            (
                ParamLocal::BitArrayFunction {
                    local: BitArrayFunctionLocalId(13),
                    type_: function_type.clone(),
                },
                "%function.bit_array#13",
            ),
            (
                ParamLocal::UtfCodepointFunction {
                    local: UtfCodepointFunctionLocalId(14),
                    type_: function_type.clone(),
                },
                "%function.utf_codepoint#14",
            ),
            (
                ParamLocal::GenericFunction(GenericFunctionLocal::new(
                    GenericFunctionLocalId(15),
                    generic_type.clone(),
                )),
                "%function.generic#15",
            ),
            (
                ParamLocal::NeverFunction(NeverFunctionLocal::new(
                    NeverFunctionLocalId(16),
                    generic_type,
                )),
                "%function.never#16",
            ),
            (
                ParamLocal::CustomFunction(CustomFunctionLocal::new(
                    CustomFunctionLocalId(17),
                    CustomFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::Custom(custom_type)),
                        Vec::new(),
                        custom_shape,
                    ),
                )),
                "%function.custom#17",
            ),
            (
                ParamLocal::BoolFunction {
                    local: BoolFunctionLocalId(18),
                    type_: function_type.clone(),
                },
                "%function.bool#18",
            ),
            (
                ParamLocal::NilFunction {
                    local: NilFunctionLocalId(19),
                    type_: function_type.clone(),
                },
                "%function.nil#19",
            ),
            (
                ParamLocal::TupleFunction {
                    local: TupleFunctionLocalId(20),
                    type_: function_type.clone(),
                },
                "%function.tuple#20",
            ),
            (
                ParamLocal::ListFunction(ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(21),
                    type_: function_type.clone(),
                    list_type,
                }),
                "%function.list.int#21",
            ),
            (
                ParamLocal::FunctionFunction(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(22),
                    FunctionFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::Function(Box::new(function_type))),
                        Vec::new(),
                        function_shape,
                    ),
                )),
                "%function.function#22",
            ),
        ];

        for (local, expected) in cases {
            assert_local(&local, expected);
        }
    }

    fn assert_local(local: &impl LocalLabel, expected: &str) {
        explain::assert_written(expected, |output| {
            local.write_local_label(output);
        });
    }
}
