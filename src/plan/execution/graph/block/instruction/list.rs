use super::super::super::{FunctionLocal, StoredListLocal};
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListListFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomLocal, FloatListLocalId,
    FloatLocalId, FunctionListLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, NilListLocalId, ParamLocal, ParameterListListLocalId, ParameterListLocalId,
    StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, FloatListTypeId, FunctionListTypeId,
    IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId,
    StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

pub(crate) enum ParameterListInstruction {
    Empty,
    Constant(ConstantId<ParameterListLocalId>),
    Call {
        function: ParameterListFunctionId,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ListFunctionLocal,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: ParameterListListLocalId,
        index: usize,
    },
}

pub(crate) enum TypedListInstruction<Element, Local, Function> {
    Value(Box<[Element]>),
    Constant(ConstantId<Local>),
    Spread {
        elements: Box<[Element]>,
        tail: Local,
    },
    Call {
        function: Function,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ListFunctionLocal,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: ListListLocalId,
        index: usize,
    },
    DropFirst {
        list: Local,
        count: usize,
    },
}

pub(crate) enum ListInstruction {
    Parameter(ParameterListTypeId, ParameterListInstruction),
    ParameterList(
        ParameterListListTypeId,
        TypedListInstruction<
            ParameterListLocalId,
            ParameterListListLocalId,
            ParameterListListFunctionId,
        >,
    ),
    Int(
        IntListTypeId,
        TypedListInstruction<IntLocalId, IntListLocalId, IntListFunctionId>,
    ),
    String(
        StringListTypeId,
        TypedListInstruction<StringLocalId, StringListLocalId, StringListFunctionId>,
    ),
    BitArray(
        BitArrayListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::BitArrayLocalId,
            BitArrayListLocalId,
            BitArrayListFunctionId,
        >,
    ),
    UtfCodepoint(
        UtfCodepointListTypeId,
        TypedListInstruction<
            UtfCodepointLocalId,
            UtfCodepointListLocalId,
            UtfCodepointListFunctionId,
        >,
    ),
    Custom(
        CustomListTypeId,
        TypedListInstruction<CustomLocal, CustomListLocalId, CustomListFunctionId>,
    ),
    Float(
        FloatListTypeId,
        TypedListInstruction<FloatLocalId, FloatListLocalId, FloatListFunctionId>,
    ),
    Bool(
        BoolListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::BoolLocalId,
            BoolListLocalId,
            BoolListFunctionId,
        >,
    ),
    Nil(
        NilListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::NilLocalId,
            NilListLocalId,
            NilListFunctionId,
        >,
    ),
    Tuple(
        TupleListTypeId,
        TypedListInstruction<TupleLocalId, TupleListLocalId, TupleListFunctionId>,
    ),
    List(
        ListListTypeId,
        TypedListInstruction<StoredListLocal, ListListLocalId, ListListFunctionId>,
    ),
    Function(
        FunctionListTypeId,
        TypedListInstruction<FunctionLocal, FunctionListLocalId, FunctionListFunctionId>,
    ),
}

impl Explain for ListInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            Self::Parameter(type_id, instruction) => {
                output.push_str("list.parameter[type#");
                output.push_str(&type_id.list_type().index().to_string());
                output.push_str("] ");
                write_parameter(output, instruction);
            }
            Self::ParameterList(type_id, instruction) => write_typed(
                output,
                "parameter_list",
                type_id.list_type().index(),
                instruction,
            ),
            Self::Int(type_id, instruction) => {
                write_typed(output, "int", type_id.list_type().index(), instruction);
            }
            Self::String(type_id, instruction) => {
                write_typed(output, "string", type_id.list_type().index(), instruction);
            }
            Self::BitArray(type_id, instruction) => {
                write_typed(
                    output,
                    "bit_array",
                    type_id.list_type().index(),
                    instruction,
                );
            }
            Self::UtfCodepoint(type_id, instruction) => write_typed(
                output,
                "utf_codepoint",
                type_id.list_type().index(),
                instruction,
            ),
            Self::Custom(type_id, instruction) => {
                write_typed(output, "custom", type_id.list_type().index(), instruction);
            }
            Self::Float(type_id, instruction) => {
                write_typed(output, "float", type_id.list_type().index(), instruction);
            }
            Self::Bool(type_id, instruction) => {
                write_typed(output, "bool", type_id.list_type().index(), instruction);
            }
            Self::Nil(type_id, instruction) => {
                write_typed(output, "nil", type_id.list_type().index(), instruction);
            }
            Self::Tuple(type_id, instruction) => {
                write_typed(output, "tuple", type_id.list_type().index(), instruction);
            }
            Self::List(type_id, instruction) => {
                write_typed(output, "list", type_id.list_type().index(), instruction);
            }
            Self::Function(type_id, instruction) => {
                write_typed(output, "function", type_id.list_type().index(), instruction);
            }
        }
    }
}

fn write_parameter(output: &mut String, instruction: &ParameterListInstruction) {
    match instruction {
        ParameterListInstruction::Empty => output.push_str("empty"),
        ParameterListInstruction::Constant(id) => write_constant(output, "list.parameter", *id),
        ParameterListInstruction::Call { function, args, .. } => {
            write_call(output, "call", function, args);
        }
        ParameterListInstruction::FunctionCall { function, args, .. } => {
            write_function_call(output, "function_call", function, args);
        }
        ParameterListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        ParameterListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        ParameterListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

fn write_typed<Element, Local, Function>(
    output: &mut String,
    family: &'static str,
    type_id: usize,
    instruction: &TypedListInstruction<Element, Local, Function>,
) where
    Element: LocalLabel,
    Local: LocalLabel,
    Function: FunctionLabelSource,
{
    output.push_str("list.");
    output.push_str(family);
    output.push_str("[type#");
    output.push_str(&type_id.to_string());
    output.push_str("] ");
    match instruction {
        TypedListInstruction::Value(elements) => {
            output.push_str("value elements=");
            write_list_values(output, elements);
        }
        TypedListInstruction::Constant(id) => {
            write_constant(output, &format!("list.{family}"), *id);
        }
        TypedListInstruction::Spread { elements, tail } => {
            output.push_str("spread elements=");
            write_list_values(output, elements);
            output.push_str(" tail=");
            tail.write_local_label(output);
        }
        TypedListInstruction::Call { function, args, .. } => {
            write_call(output, "call", function, args);
        }
        TypedListInstruction::FunctionCall { function, args, .. } => {
            write_function_call(output, "function_call", function, args);
        }
        TypedListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        TypedListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        TypedListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
        TypedListInstruction::DropFirst { list, count } => {
            output.push_str("drop_first ");
            list.write_local_label(output);
            output.push_str(" count=");
            output.push_str(&count.to_string());
        }
    }
}

fn write_list_values<Value: LocalLabel>(output: &mut String, values: &[Value]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        value.write_local_label(output);
    }
    output.push(']');
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::TupleFunctionId;

    #[test]
    fn writes_list_instruction_grammar() {
        let source = r#"
pub fn main() {
  let tail = [3]
  let values = [1, 2, ..tail]
  let assert [_, ..rest] = values
  #([], values, rest)
}
"#;
        let expected = concat!(
            "    %int#0:shape#0(Int) = int.value 3\n",
            "    %list.int#0:shape#1(list_type#1) = list.int[type#1] value ",
            "elements=[%int#0]\n",
            "    %int#1:shape#0(Int) = int.value 1\n",
            "    %int#2:shape#0(Int) = int.value 2\n",
            "    %list.int#1:shape#1(list_type#1) = list.int[type#1] spread ",
            "elements=[%int#1, %int#2] tail=%list.int#0\n",
            "    %list.parameter#0:shape#3(list_type#0) = list.parameter[type#0] empty\n",
            "    %tuple#0:shape#4(#(list_type#0, list_type#1, list_type#1)) = ",
            "tuple.value elements=[%list.parameter#0, %list.int#1, %list.int#0]\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut context = explain::ExplainContext::new(plan, output);
            for block in graph.blocks() {
                for instruction in block.instructions() {
                    context.write(instruction);
                }
            }
        });
    }
}
