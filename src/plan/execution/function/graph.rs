use crate::plan::execution::graph::{Block, BlockId, Graph, GraphExitId};

pub(crate) struct FunctionGraph<Return, TailCall> {
    graph: Graph,
    exits: Box<[FunctionGraphExit<Return, TailCall>]>,
}

pub(crate) enum FunctionGraphExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: Box<[crate::plan::execution::ParamLocal]>,
    },
}

impl<Return, TailCall> FunctionGraph<Return, TailCall> {
    pub(in crate::plan::execution) fn from_parts(
        graph: Graph,
        exits: Vec<FunctionGraphExit<Return, TailCall>>,
    ) -> Self {
        Self {
            graph,
            exits: exits.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.graph.entry()
    }

    #[cfg(test)]
    pub(crate) fn blocks(&self) -> &[Block] {
        self.graph.blocks()
    }

    pub(crate) fn block(&self, id: BlockId) -> &Block {
        self.graph.block(id)
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn exit(&self, id: GraphExitId) -> &FunctionGraphExit<Return, TailCall> {
        &self.exits[id.index()]
    }
}

use crate::plan::execution::explain::{ExplainContext, FunctionLabel};
use crate::plan::execution::function::FunctionEntry;
use crate::plan::execution::graph::{ExplainLocal, write_graph};

pub(in crate::plan::execution::function) trait ExplainFunctionBody {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    );
}

impl<Return, TailCall> ExplainFunctionBody for FunctionGraph<Return, TailCall>
where
    Return: ExplainLocal,
    TailCall: TailFunctionIndex,
{
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        write_graph(
            context,
            self.graph(),
            entry.params(self),
            entry.captures(self),
            &mut |context, exit| {
                write_function_exit(context, self.exit(exit), family);
            },
        );
    }
}

fn write_function_exit<Return, TailCall>(
    context: &mut ExplainContext<'_, '_>,
    exit: &FunctionGraphExit<Return, TailCall>,
    family: &'static str,
) where
    Return: ExplainLocal,
    TailCall: TailFunctionIndex,
{
    match exit {
        FunctionGraphExit::Return(value) => {
            context.push_str("return ");
            context.write(value);
        }
        FunctionGraphExit::TailCall { function, args } => {
            context.push_str("tail ");
            FunctionLabel::new(family, function.tail_function_index()).write(context.output());
            context.push_str(" args=");
            context.write_list(args, |context, argument| context.write(argument));
        }
    }
}

use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomListFunctionId, FloatFunctionFunctionId,
    FloatFunctionId, FloatListFunctionId, FunctionListFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListFunctionFunctionId,
    ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId,
    NilFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    StringFunctionFunctionId, StringFunctionId, StringListFunctionId, TupleFunctionFunctionId,
    TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};

pub(in crate::plan::execution::function) trait TailFunctionIndex {
    fn tail_function_index(&self) -> usize;
}

impl TailFunctionIndex for usize {
    fn tail_function_index(&self) -> usize {
        *self
    }
}

impl TailFunctionIndex for NeverFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for IntFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for FloatFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for StringFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for BitArrayFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for UtfCodepointFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for BoolFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for NilFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for TupleFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for IntFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for FloatFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for StringFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for BitArrayFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for UtfCodepointFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for BoolFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for NilFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for TupleFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.0
    }
}

impl TailFunctionIndex for ParameterListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for IntListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for StringListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for BitArrayListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for UtfCodepointListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for CustomListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for FloatListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for BoolListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for NilListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for TupleListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for ParameterListListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for ListListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for FunctionListFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for GenericFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for NeverFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        self.index()
    }
}

impl TailFunctionIndex for ListFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        match self {
            Self::Parameter { id, .. } => id.0,
            Self::ParameterList { id, .. } => id.0,
            Self::Int { id, .. } => id.0,
            Self::String { id, .. } => id.0,
            Self::BitArray { id, .. } => id.0,
            Self::UtfCodepoint { id, .. } => id.0,
            Self::Custom { id, .. } => id.0,
            Self::Float { id, .. } => id.0,
            Self::Bool { id, .. } => id.0,
            Self::Nil { id, .. } => id.0,
            Self::Tuple { id, .. } => id.0,
            Self::List { id, .. } => id.0,
            Self::Function { id, .. } => id.0,
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{TailFunctionIndex, write_function_exit};
    use crate::plan::execution::{IntFunctionId, Terminator, explain};

    #[test]
    fn writes_return_and_tail_call_exits() {
        let source = r#"
fn loop(value: Int) {
  case value {
    0 -> 0
    _ -> loop(value - 1)
  }
}

pub fn main() { loop(2) }
"#;
        let expected = "return %int#0 | tail int#1 args=[%int#2]";

        assert_explanation(source, expected);
    }

    #[test]
    fn extracts_every_tail_function_index_explicitly() {
        use crate::plan::execution::{
            BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionFunctionId,
            BitArrayListFunctionId, BitArrayListTypeId, BoolFunctionFunctionId, BoolFunctionId,
            BoolListFunctionFunctionId, BoolListFunctionId, BoolListTypeId,
            CustomListFunctionFunctionId, CustomListFunctionId, CustomListTypeId, CustomTypeId,
            FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionFunctionId,
            FloatListFunctionId, FloatListTypeId, FunctionListFunctionFunctionId,
            FunctionListFunctionId, FunctionListTypeId, FunctionShape, FunctionType,
            GenericFunctionFunctionId, GenericFunctionType, IntFunctionFunctionId,
            IntListFunctionFunctionId, IntListFunctionId, IntListTypeId, ListFunctionFunctionId,
            ListListFunctionFunctionId, ListListFunctionId, ListListTypeId, ListTypeId,
            NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId, NilFunctionId,
            NilListFunctionFunctionId, NilListFunctionId, NilListTypeId,
            ParameterListFunctionFunctionId, ParameterListFunctionId,
            ParameterListListFunctionFunctionId, ParameterListListFunctionId,
            ParameterListListTypeId, ParameterListTypeId, StringFunctionFunctionId,
            StringFunctionId, StringListFunctionFunctionId, StringListFunctionId, StringListTypeId,
            TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionFunctionId,
            TupleListFunctionId, TupleListTypeId, UtfCodepointFunctionFunctionId,
            UtfCodepointFunctionId, UtfCodepointListFunctionFunctionId, UtfCodepointListFunctionId,
            UtfCodepointListTypeId, ValueShapeId, ValueType,
        };

        assert_tail_index(&0usize, 0);
        assert_tail_index(&NeverFunctionId(1), 1);
        assert_tail_index(&IntFunctionId(2), 2);
        assert_tail_index(&FloatFunctionId(3), 3);
        assert_tail_index(&StringFunctionId(4), 4);
        assert_tail_index(&BitArrayFunctionId(5), 5);
        assert_tail_index(&UtfCodepointFunctionId(6), 6);
        assert_tail_index(&BoolFunctionId(7), 7);
        assert_tail_index(&NilFunctionId(8), 8);
        assert_tail_index(&TupleFunctionId(9), 9);

        let list_type = ListTypeId::new(0);
        let parameter_type = ParameterListTypeId::new(list_type, crate::plan::TypeParameterId(0));
        let custom_type = CustomTypeId::new(0);
        assert_tail_index(&ParameterListFunctionId::new(10, parameter_type), 10);
        assert_tail_index(
            &ParameterListListFunctionId::new(
                11,
                ParameterListListTypeId::new(list_type, parameter_type),
            ),
            11,
        );
        assert_tail_index(
            &IntListFunctionId::new(12, IntListTypeId::new(list_type)),
            12,
        );
        assert_tail_index(
            &StringListFunctionId::new(13, StringListTypeId::new(list_type)),
            13,
        );
        assert_tail_index(
            &BitArrayListFunctionId::new(14, BitArrayListTypeId::new(list_type)),
            14,
        );
        assert_tail_index(
            &UtfCodepointListFunctionId::new(15, UtfCodepointListTypeId::new(list_type)),
            15,
        );
        assert_tail_index(
            &CustomListFunctionId::new(16, CustomListTypeId::new(list_type, custom_type)),
            16,
        );
        assert_tail_index(
            &FloatListFunctionId::new(17, FloatListTypeId::new(list_type)),
            17,
        );
        assert_tail_index(
            &BoolListFunctionId::new(18, BoolListTypeId::new(list_type)),
            18,
        );
        assert_tail_index(
            &NilListFunctionId::new(19, NilListTypeId::new(list_type)),
            19,
        );
        assert_tail_index(
            &TupleListFunctionId::new(20, TupleListTypeId::new(list_type, 0)),
            20,
        );
        assert_tail_index(
            &ListListFunctionId::new(21, ListListTypeId::new(list_type, list_type)),
            21,
        );
        assert_tail_index(
            &FunctionListFunctionId::new(22, FunctionListTypeId::new(list_type, 0)),
            22,
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_shape = FunctionShape::new(ValueShapeId::new(0), function_type.clone());
        let generic_type = GenericFunctionType::from_shapes(function_type.clone(), function_shape);
        assert_tail_index(
            &GenericFunctionFunctionId::new(23, generic_type.clone()),
            23,
        );
        assert_tail_index(&NeverFunctionFunctionId::new(24, generic_type), 24);
        assert_tail_index(&IntFunctionFunctionId(25), 25);
        assert_tail_index(&FloatFunctionFunctionId(26), 26);
        assert_tail_index(&StringFunctionFunctionId(27), 27);
        assert_tail_index(&BitArrayFunctionFunctionId(28), 28);
        assert_tail_index(&UtfCodepointFunctionFunctionId(29), 29);
        assert_tail_index(&BoolFunctionFunctionId(30), 30);
        assert_tail_index(&NilFunctionFunctionId(31), 31);
        assert_tail_index(&TupleFunctionFunctionId(32), 32);

        let list_function_functions = [
            ListFunctionFunctionId::Parameter {
                id: ParameterListFunctionFunctionId(33),
                type_: function_type.clone(),
                list_type: parameter_type,
            },
            ListFunctionFunctionId::ParameterList {
                id: ParameterListListFunctionFunctionId(34),
                type_: function_type.clone(),
                list_type: ParameterListListTypeId::new(list_type, parameter_type),
            },
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(35),
                type_: function_type.clone(),
                list_type: IntListTypeId::new(list_type),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(36),
                type_: function_type.clone(),
                list_type: StringListTypeId::new(list_type),
            },
            ListFunctionFunctionId::BitArray {
                id: BitArrayListFunctionFunctionId(37),
                type_: function_type.clone(),
                list_type: BitArrayListTypeId::new(list_type),
            },
            ListFunctionFunctionId::UtfCodepoint {
                id: UtfCodepointListFunctionFunctionId(38),
                type_: function_type.clone(),
                list_type: UtfCodepointListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Custom {
                id: CustomListFunctionFunctionId(39),
                type_: function_type.clone(),
                list_type: CustomListTypeId::new(list_type, custom_type),
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(40),
                type_: function_type.clone(),
                list_type: FloatListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(41),
                type_: function_type.clone(),
                list_type: BoolListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(42),
                type_: function_type.clone(),
                list_type: NilListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(43),
                type_: function_type.clone(),
                list_type: TupleListTypeId::new(list_type, 0),
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(44),
                type_: function_type.clone(),
                list_type: ListListTypeId::new(list_type, list_type),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(45),
                type_: function_type,
                list_type: FunctionListTypeId::new(list_type, 0),
            },
        ];

        for (expected, function) in (33..).zip(list_function_functions) {
            assert_tail_index(&function, expected);
        }
    }

    fn assert_tail_index(function: &impl TailFunctionIndex, expected: usize) {
        assert_eq!(function.tail_function_index(), expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.int_function(IntFunctionId(1)).graph();
            let mut context = explain::ExplainContext::new(plan, output);
            for block in graph.blocks() {
                if let Terminator::Exit(exit) = block.terminator() {
                    if !context.output().is_empty() {
                        context.push_str(" | ");
                    }
                    write_function_exit(&mut context, graph.exit(*exit), "int");
                }
            }
        });
    }
}
