use crate::plan::execution::explain::Explain;
use crate::plan::execution::explain::{ExplainContext, FunctionLabel};
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionFunctionId,
    ExternalListFunctionId, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId,
    FunctionListFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionId, ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionId, ParameterListFunctionId,
    ParameterListListFunctionId, ProfiledListFunctionFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};
use crate::plan::execution::graph::{
    BlockGraphExitExplanation, BlockGraphExitId, LocalLabel, ParamSlot, ProfiledBlockGraph,
};

pub(crate) struct ProfiledFunctionBody<Return, TailCall, Graph: ExecutionGraphProfile> {
    block_graph: ProfiledBlockGraph<Graph>,
    exits: Box<[FunctionExit<Return, TailCall>]>,
}

pub(crate) type FunctionBody<Return, TailCall> =
    ProfiledFunctionBody<Return, TailCall, HostedExecutionGraph>;

pub(crate) enum FunctionExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: Box<[crate::plan::execution::graph::ParamLocal]>,
    },
}

pub(crate) trait FunctionBodyOwner {
    type Return;
    type TailCall;
    type Graph: ExecutionGraphProfile;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph>;
}

struct FunctionExitExplanation<'a, Return, TailCall, Graph: ExecutionGraphProfile> {
    body: &'a ProfiledFunctionBody<Return, TailCall, Graph>,
    family: &'static str,
}

impl<Return, TailCall, Graph: ExecutionGraphProfile> ProfiledFunctionBody<Return, TailCall, Graph> {
    pub(in crate::plan::execution) fn from_parts(
        block_graph: ProfiledBlockGraph<Graph>,
        exits: Vec<FunctionExit<Return, TailCall>>,
    ) -> Self {
        Self {
            block_graph,
            exits: exits.into_boxed_slice(),
        }
    }

    pub(crate) fn block_graph(&self) -> &ProfiledBlockGraph<Graph> {
        &self.block_graph
    }

    pub(crate) fn exit(&self, id: BlockGraphExitId) -> &FunctionExit<Return, TailCall> {
        &self.exits[id.index()]
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (
        ProfiledBlockGraph<Graph>,
        Box<[FunctionExit<Return, TailCall>]>,
    ) {
        (self.block_graph, self.exits)
    }

    pub(in crate::plan::execution) fn write_explanation(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) where
        Return: LocalLabel,
        TailCall: TailCallLabelIndex,
        Graph::ExternalFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionId: FunctionLabelSource,
        Graph::ExternalFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalInstruction: Explain,
        Graph::ExternalListInstruction: Explain,
        Graph::ExternalFunctionInstruction: Explain,
    {
        let exits = FunctionExitExplanation { body: self, family };
        self.block_graph()
            .write_explanation(context, entry_params, entry_captures, &exits);
    }
}

impl<Return, TailCall, Graph: ExecutionGraphProfile> FunctionBodyOwner
    for ProfiledFunctionBody<Return, TailCall, Graph>
{
    type Return = Return;
    type TailCall = TailCall;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        self
    }
}

impl<Return, TailCall, Graph: ExecutionGraphProfile> BlockGraphExitExplanation
    for FunctionExitExplanation<'_, Return, TailCall, Graph>
where
    Return: LocalLabel,
    TailCall: TailCallLabelIndex,
{
    fn write_exit(&self, context: &mut ExplainContext<'_, '_>, exit: BlockGraphExitId) {
        match self.body.exit(exit) {
            FunctionExit::Return(value) => {
                context.push_str("return ");
                context.write(value);
            }
            FunctionExit::TailCall { function, args } => {
                context.push_str("tail ");
                FunctionLabel::new(self.family, function.tail_call_label_index())
                    .write(context.output());
                context.push_str(" args=");
                context.write_list(args, |context, argument| context.write(argument));
            }
        }
    }
}

pub(in crate::plan::execution) trait TailCallLabelIndex {
    fn tail_call_label_index(&self) -> usize;
}

impl TailCallLabelIndex for usize {
    fn tail_call_label_index(&self) -> usize {
        *self
    }
}

impl TailCallLabelIndex for NeverFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for IntFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl<Function: TailCallLabelIndex> TailCallLabelIndex
    for crate::plan::FunctionCallTarget<Function>
{
    fn tail_call_label_index(&self) -> usize {
        self.function().tail_call_label_index()
    }
}

impl TailCallLabelIndex for FloatFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for StringFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for BitArrayFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for UtfCodepointFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for BoolFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for NilFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for TupleFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for IntFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for FloatFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for StringFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for BitArrayFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for UtfCodepointFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for BoolFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for NilFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for TupleFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl TailCallLabelIndex for ParameterListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for IntListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for StringListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for BitArrayListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for UtfCodepointListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for CustomListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for ExternalListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for FloatListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for BoolListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for NilListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for TupleListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for ParameterListListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for ListListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for FunctionListFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for GenericFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for NeverFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.index()
    }
}

impl TailCallLabelIndex for ExternalListFunctionFunctionId {
    fn tail_call_label_index(&self) -> usize {
        self.0
    }
}

impl<Graph: crate::plan::execution::function::ExecutionGraphProfile> TailCallLabelIndex
    for ProfiledListFunctionFunctionId<Graph>
where
    Graph::ExternalListFunctionFunctionId: TailCallLabelIndex,
{
    fn tail_call_label_index(&self) -> usize {
        match self {
            Self::Parameter { id, .. } => id.0,
            Self::ParameterList { id, .. } => id.0,
            Self::Int { id, .. } => id.0,
            Self::String { id, .. } => id.0,
            Self::BitArray { id, .. } => id.0,
            Self::UtfCodepoint { id, .. } => id.0,
            Self::Custom { id, .. } => id.0,
            Self::External { id, .. } => id.tail_call_label_index(),
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
    use super::{FunctionBodyOwner, FunctionExitExplanation, TailCallLabelIndex};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{BlockGraphExitExplanation, Terminator};

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
    fn exposes_the_base_function_body_without_adaptation() {
        let source = "pub fn main() { 1 }";

        explain::with_execution_plan(source, |plan| {
            let body = plan.int_function(IntFunctionId(0)).body();
            assert!(std::ptr::eq(body, FunctionBodyOwner::function_body(body)));
        });
    }

    #[test]
    fn extracts_every_tail_call_label_index_explicitly() {
        use crate::plan::execution::function::{
            BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionFunctionId,
            BitArrayListFunctionId, BoolFunctionFunctionId, BoolFunctionId,
            BoolListFunctionFunctionId, BoolListFunctionId, CustomListFunctionFunctionId,
            CustomListFunctionId, ExternalListFunctionFunctionId, ExternalListFunctionId,
            FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionFunctionId,
            FloatListFunctionId, FunctionListFunctionFunctionId, FunctionListFunctionId,
            GenericFunctionFunctionId, IntFunctionFunctionId, IntListFunctionFunctionId,
            IntListFunctionId, ListFunctionFunctionId, ListListFunctionFunctionId,
            ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId,
            NilFunctionId, NilListFunctionFunctionId, NilListFunctionId,
            ParameterListFunctionFunctionId, ParameterListFunctionId,
            ParameterListListFunctionFunctionId, ParameterListListFunctionId,
            StringFunctionFunctionId, StringFunctionId, StringListFunctionFunctionId,
            StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
            TupleListFunctionFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
            UtfCodepointFunctionId, UtfCodepointListFunctionFunctionId, UtfCodepointListFunctionId,
        };
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
            ExternalTypeId, FloatListTypeId, FunctionListTypeId, FunctionShape, FunctionType,
            GenericFunctionType, IntListTypeId, ListListTypeId, ListTypeId, NilListTypeId,
            ParameterListListTypeId, ParameterListTypeId, StringListTypeId, TupleListTypeId,
            UtfCodepointListTypeId, ValueShapeId, ValueType,
        };

        assert_tail_call_label_index(&0usize, 0);
        assert_tail_call_label_index(&NeverFunctionId(1), 1);
        assert_tail_call_label_index(&IntFunctionId(2), 2);
        assert_tail_call_label_index(&FloatFunctionId(3), 3);
        assert_tail_call_label_index(&StringFunctionId(4), 4);
        assert_tail_call_label_index(&BitArrayFunctionId(5), 5);
        assert_tail_call_label_index(&UtfCodepointFunctionId(6), 6);
        assert_tail_call_label_index(&BoolFunctionId(7), 7);
        assert_tail_call_label_index(&NilFunctionId(8), 8);
        assert_tail_call_label_index(&TupleFunctionId(9), 9);

        let list_type = ListTypeId::new(0);
        let parameter_type = ParameterListTypeId::new(list_type, crate::plan::TypeParameterId(0));
        let custom_type = CustomTypeId::new(0);
        assert_tail_call_label_index(&ParameterListFunctionId::new(10, parameter_type), 10);
        assert_tail_call_label_index(
            &ParameterListListFunctionId::new(
                11,
                ParameterListListTypeId::new(list_type, parameter_type),
            ),
            11,
        );
        assert_tail_call_label_index(
            &IntListFunctionId::new(12, IntListTypeId::new(list_type)),
            12,
        );
        assert_tail_call_label_index(
            &StringListFunctionId::new(13, StringListTypeId::new(list_type)),
            13,
        );
        assert_tail_call_label_index(
            &BitArrayListFunctionId::new(14, BitArrayListTypeId::new(list_type)),
            14,
        );
        assert_tail_call_label_index(
            &UtfCodepointListFunctionId::new(15, UtfCodepointListTypeId::new(list_type)),
            15,
        );
        assert_tail_call_label_index(
            &CustomListFunctionId::new(16, CustomListTypeId::new(list_type, custom_type)),
            16,
        );
        let external_list_type = ExternalListTypeId::new(list_type, ExternalTypeId::new(0));
        assert_tail_call_label_index(&ExternalListFunctionId::new(17, external_list_type), 17);
        assert_tail_call_label_index(
            &FloatListFunctionId::new(18, FloatListTypeId::new(list_type)),
            18,
        );
        assert_tail_call_label_index(
            &BoolListFunctionId::new(19, BoolListTypeId::new(list_type)),
            19,
        );
        assert_tail_call_label_index(
            &NilListFunctionId::new(20, NilListTypeId::new(list_type)),
            20,
        );
        assert_tail_call_label_index(
            &TupleListFunctionId::new(21, TupleListTypeId::new(list_type, 0)),
            21,
        );
        assert_tail_call_label_index(
            &ListListFunctionId::new(22, ListListTypeId::new(list_type, list_type)),
            22,
        );
        assert_tail_call_label_index(
            &FunctionListFunctionId::new(23, FunctionListTypeId::new(list_type, 0)),
            23,
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_shape = FunctionShape::new(ValueShapeId::new(0), function_type.clone());
        let generic_type = GenericFunctionType::from_shapes(function_type.clone(), function_shape);
        assert_tail_call_label_index(
            &GenericFunctionFunctionId::new(24, generic_type.clone()),
            24,
        );
        assert_tail_call_label_index(&NeverFunctionFunctionId::new(25, generic_type), 25);
        assert_tail_call_label_index(&IntFunctionFunctionId(26), 26);
        assert_tail_call_label_index(&FloatFunctionFunctionId(27), 27);
        assert_tail_call_label_index(&StringFunctionFunctionId(28), 28);
        assert_tail_call_label_index(&BitArrayFunctionFunctionId(29), 29);
        assert_tail_call_label_index(&UtfCodepointFunctionFunctionId(30), 30);
        assert_tail_call_label_index(&BoolFunctionFunctionId(31), 31);
        assert_tail_call_label_index(&NilFunctionFunctionId(32), 32);
        assert_tail_call_label_index(&TupleFunctionFunctionId(33), 33);

        let list_function_functions = [
            ListFunctionFunctionId::Parameter {
                id: ParameterListFunctionFunctionId(34),
                type_: function_type.clone(),
                list_type: parameter_type,
            },
            ListFunctionFunctionId::ParameterList {
                id: ParameterListListFunctionFunctionId(35),
                type_: function_type.clone(),
                list_type: ParameterListListTypeId::new(list_type, parameter_type),
            },
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(36),
                type_: function_type.clone(),
                list_type: IntListTypeId::new(list_type),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(37),
                type_: function_type.clone(),
                list_type: StringListTypeId::new(list_type),
            },
            ListFunctionFunctionId::BitArray {
                id: BitArrayListFunctionFunctionId(38),
                type_: function_type.clone(),
                list_type: BitArrayListTypeId::new(list_type),
            },
            ListFunctionFunctionId::UtfCodepoint {
                id: UtfCodepointListFunctionFunctionId(39),
                type_: function_type.clone(),
                list_type: UtfCodepointListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Custom {
                id: CustomListFunctionFunctionId(40),
                type_: function_type.clone(),
                list_type: CustomListTypeId::new(list_type, custom_type),
            },
            ListFunctionFunctionId::External {
                id: ExternalListFunctionFunctionId(41),
                type_: function_type.clone(),
                list_type: external_list_type,
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(42),
                type_: function_type.clone(),
                list_type: FloatListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(43),
                type_: function_type.clone(),
                list_type: BoolListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(44),
                type_: function_type.clone(),
                list_type: NilListTypeId::new(list_type),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(45),
                type_: function_type.clone(),
                list_type: TupleListTypeId::new(list_type, 0),
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(46),
                type_: function_type.clone(),
                list_type: ListListTypeId::new(list_type, list_type),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(47),
                type_: function_type,
                list_type: FunctionListTypeId::new(list_type, 0),
            },
        ];

        for (expected, function) in (34..).zip(list_function_functions) {
            assert_tail_call_label_index(&function, expected);
        }
    }

    fn assert_tail_call_label_index(function: &impl TailCallLabelIndex, expected: usize) {
        assert_eq!(function.tail_call_label_index(), expected);
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
                    FunctionExitExplanation {
                        body,
                        family: "int",
                    }
                    .write_exit(&mut context, *exit);
                }
            }
        });
    }
}
