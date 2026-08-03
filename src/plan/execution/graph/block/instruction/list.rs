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
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomLocal, ExternalListLocalId,
    ExternalLocal, FloatListLocalId, FloatLocalId, FunctionListLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, NilListLocalId, ParamLocal, ParameterListListLocalId,
    ParameterListLocalId, StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypedListInstruction<Element, Local, Function, FunctionLocal = ListFunctionLocal> {
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
        function: FunctionLocal,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExternalListInstruction {
    type_id: ExternalListTypeId,
    instruction: TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        crate::plan::execution::function::ExternalListFunctionId,
        crate::plan::execution::graph::ExternalListFunctionLocalId,
    >,
}

pub(crate) trait ExternalListInstructionView {
    type Function;
    type FunctionLocal;

    fn type_id(&self) -> ExternalListTypeId;

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    >;
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

impl ExternalListInstruction {
    pub(in crate::plan::execution) fn new(
        type_id: ExternalListTypeId,
        instruction: TypedListInstruction<
            ExternalLocal,
            ExternalListLocalId,
            crate::plan::execution::function::ExternalListFunctionId,
            crate::plan::execution::graph::ExternalListFunctionLocalId,
        >,
    ) -> Self {
        Self {
            type_id,
            instruction,
        }
    }
}

impl ExternalListInstructionView for ExternalListInstruction {
    type Function = crate::plan::execution::function::ExternalListFunctionId;
    type FunctionLocal = crate::plan::execution::graph::ExternalListFunctionLocalId;

    fn type_id(&self) -> ExternalListTypeId {
        self.type_id
    }

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    > {
        &self.instruction
    }
}

impl ExternalListInstructionView for std::convert::Infallible {
    type Function = std::convert::Infallible;
    type FunctionLocal = std::convert::Infallible;

    fn type_id(&self) -> ExternalListTypeId {
        match *self {}
    }

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    > {
        match *self {}
    }
}

impl Explain for ExternalListInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_typed(
            context.output(),
            "external",
            self.type_id.list_type().index(),
            &self.instruction,
        );
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

fn write_typed<Element, Local, Function, FunctionLocal>(
    output: &mut String,
    family: &'static str,
    type_id: usize,
    instruction: &TypedListInstruction<Element, Local, Function, FunctionLocal>,
) where
    Element: LocalLabel,
    Local: LocalLabel,
    Function: FunctionLabelSource,
    FunctionLocal: LocalLabel,
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
mod external_list_view_tests {
    use super::{ExternalListInstruction, ExternalListInstructionView, TypedListInstruction};
    use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId};
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn exposes_external_list_type_and_instruction() {
        let external_type = ExternalTypeId::new(3);
        let list_type = ExternalListTypeId::new(ListTypeId::new(7), external_type);
        let instruction = ExternalListInstruction::new(
            list_type,
            TypedListInstruction::Value(
                vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into_boxed_slice(),
            ),
        );

        assert_eq!(instruction.type_id(), list_type);
        assert_eq!(
            instruction.instruction(),
            &TypedListInstruction::Value(
                vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into_boxed_slice(),
            ),
        );
    }

    #[test]
    fn plain_external_list_instruction_view_is_uninhabited() {
        fn assert_view<View>()
        where
            View: ExternalListInstructionView<
                    Function = std::convert::Infallible,
                    FunctionLocal = std::convert::Infallible,
                >,
        {
        }

        assert_view::<std::convert::Infallible>();
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{ExternalListInstruction, TypedListInstruction};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId};
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

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

    #[test]
    fn writes_external_list_instruction_grammar() {
        let source = "pub fn main() { 1 }";
        let expected = "list.external[type#7] value elements=[%external#2]";

        explain::assert_rendered(source, expected, |plan, output| {
            let external_type = ExternalTypeId::new(3);
            let instruction = ExternalListInstruction::new(
                ExternalListTypeId::new(ListTypeId::new(7), external_type),
                TypedListInstruction::Value(
                    vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into_boxed_slice(),
                ),
            );
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&instruction);
        });
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
