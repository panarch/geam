use crate::plan::execution::explain::{ExplainContext, FunctionLabel};
use crate::plan::execution::function::FunctionEntry;
use crate::plan::execution::function::{
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
use crate::plan::execution::graph::{BlockGraph, BlockGraphExitId};
use crate::plan::execution::graph::{ExplainLocal, write_graph};

pub(crate) struct FunctionBody<Return, TailCall> {
    block_graph: BlockGraph,
    exits: Box<[FunctionExit<Return, TailCall>]>,
}

pub(crate) enum FunctionExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: Box<[crate::plan::execution::graph::ParamLocal]>,
    },
}

impl<Return, TailCall> FunctionBody<Return, TailCall> {
    pub(in crate::plan::execution) fn from_parts(
        block_graph: BlockGraph,
        exits: Vec<FunctionExit<Return, TailCall>>,
    ) -> Self {
        Self {
            block_graph,
            exits: exits.into_boxed_slice(),
        }
    }

    pub(crate) fn block_graph(&self) -> &BlockGraph {
        &self.block_graph
    }

    pub(crate) fn exit(&self, id: BlockGraphExitId) -> &FunctionExit<Return, TailCall> {
        &self.exits[id.index()]
    }
}

pub(in crate::plan::execution::function) trait ExplainFunctionBody {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    );
}

impl<Return, TailCall> ExplainFunctionBody for FunctionBody<Return, TailCall>
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
            self.block_graph(),
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
    exit: &FunctionExit<Return, TailCall>,
    family: &'static str,
) where
    Return: ExplainLocal,
    TailCall: TailFunctionIndex,
{
    match exit {
        FunctionExit::Return(value) => {
            context.push_str("return ");
            context.write(value);
        }
        FunctionExit::TailCall { function, args } => {
            context.push_str("tail ");
            FunctionLabel::new(family, function.tail_function_index()).write(context.output());
            context.push_str(" args=");
            context.write_list(args, |context, argument| context.write(argument));
        }
    }
}

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
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::Terminator;

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
        use crate::plan::execution::function::{
            BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionFunctionId,
            BitArrayListFunctionId, BoolFunctionFunctionId, BoolFunctionId,
            BoolListFunctionFunctionId, BoolListFunctionId, CustomListFunctionFunctionId,
            CustomListFunctionId, FloatFunctionFunctionId, FloatFunctionId,
            FloatListFunctionFunctionId, FloatListFunctionId, FunctionListFunctionFunctionId,
            FunctionListFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId,
            IntListFunctionFunctionId, IntListFunctionId, ListFunctionFunctionId,
            ListListFunctionFunctionId, ListListFunctionId, NeverFunctionFunctionId,
            NeverFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionFunctionId,
            NilListFunctionId, ParameterListFunctionFunctionId, ParameterListFunctionId,
            ParameterListListFunctionFunctionId, ParameterListListFunctionId,
            StringFunctionFunctionId, StringFunctionId, StringListFunctionFunctionId,
            StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
            TupleListFunctionFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
            UtfCodepointFunctionId, UtfCodepointListFunctionFunctionId, UtfCodepointListFunctionId,
        };
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, FloatListTypeId,
            FunctionListTypeId, FunctionShape, FunctionType, GenericFunctionType, IntListTypeId,
            ListListTypeId, ListTypeId, NilListTypeId, ParameterListListTypeId,
            ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
            ValueShapeId, ValueType,
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
            let body = plan.int_function(IntFunctionId(1)).body();
            let mut context = explain::ExplainContext::new(plan, output);
            for block in body.block_graph().blocks() {
                if let Terminator::Exit(exit) = block.terminator() {
                    if !context.output().is_empty() {
                        context.push_str(" | ");
                    }
                    write_function_exit(&mut context, body.exit(*exit), "int");
                }
            }
        });
    }
}
