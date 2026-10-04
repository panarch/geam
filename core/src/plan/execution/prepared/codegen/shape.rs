use super::bit_array::BitArrayMatch;
use super::custom::{CustomField, CustomMatch};
use super::custom_loop::{CustomLoopInstruction, LoopCallOutput};
use super::int_list::{IntListInstruction, IntListMatch, IntListTest};
use crate::plan::execution::compiled::{CompiledCheckpoint, CompiledLoopCall};
use crate::plan::execution::function::{ExecutionGraphProfile, FunctionExit, ProfiledFunctionBody};
use crate::plan::execution::graph::{
    ArithmeticRegion, BlockGraphExitId, BlockGraphView, BlockId, BoolInstruction, BoolLocalId,
    BoolTest, CustomListLocalId, Edge, IntInstruction, IntLocalId, IntegerLiteral, IntegerOperand,
    ListInstruction, ListLocal, MatchEdge, ParamLocal, ParamSlot, ProfiledInstruction,
    ProfiledInstructionKind, StringInstruction, Terminator, TypedListInstruction,
};
use crate::plan::execution::type_::CustomTypeTable;
use std::collections::{BTreeMap, BTreeSet};

/// Borrowed inspection used only while emitting or admitting prepared data.
/// None of the structuring work survives in the executable program.
pub(in crate::plan::execution::prepared) struct CompiledShape<'graph, Graph: ExecutionGraphProfile>
{
    pub graph: BlockGraphView<'graph, Graph>,
    pub(super) blocks: BTreeMap<usize, CompiledBlock<'graph>>,
    pub checkpoints: Vec<CompiledCheckpoint>,
    pub(super) starts: BTreeMap<usize, usize>,
    pub(super) joins: Vec<Option<BlockId>>,
    pub(super) repeats: bool,
    pub kind: KernelKind,
    pub(super) order: Vec<BlockId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::plan::execution::prepared) enum KernelKind {
    Numeric,
    IntList,
    BitArray,
    CustomLoop,
    Callback,
}

// These are preparation-local views. They borrow the canonical graph and
// narrow only the variants that this emitter implements; they are never
// stored in an execution program or used as a second runtime evaluator.
pub(super) struct CompiledBlock<'graph> {
    pub instructions: Vec<CompiledInstruction<'graph>>,
    pub terminator: CompiledTerminator<'graph>,
}

pub(super) enum CompiledInstruction<'graph> {
    Integer(IntLocalId, NumericInteger<'graph>),
    Boolean(BoolLocalId, CompiledBoolean),
    Region {
        region: &'graph ArithmeticRegion,
        outputs: Vec<IntLocalId>,
    },
    IntList(IntListInstruction<'graph>),
    CustomField(CustomField),
    CustomLoop(CustomLoopInstruction<'graph>),
}

pub(super) enum NumericInteger<'graph> {
    Value(&'graph IntegerLiteral),
    Binary(NumericOperation, IntegerOperand, IntegerOperand),
    Negate(IntLocalId),
}

#[derive(Clone, Copy)]
pub(super) enum NumericOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

pub(super) enum CompiledBoolean {
    Value(bool),
    Test(CompiledTest),
}

pub(super) enum CompiledTest {
    Not(BoolLocalId),
    Compare(NumericComparison, IntegerOperand, IntegerOperand),
    IntList(IntListTest),
    CustomListLength {
        list: CustomListLocalId,
        length: usize,
        at_least: bool,
    },
}

pub(super) enum NumericComparison {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub(super) enum CompiledTerminator<'graph> {
    Jump(&'graph Edge),
    Boolean {
        subject: BoolLocalId,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Test {
        test: CompiledTest,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Switch {
        subject: IntLocalId,
        clauses: &'graph [(IntegerLiteral, Edge)],
        fallback: &'graph Edge,
    },
    Exit(BlockGraphExitId),
    BitArray(BitArrayMatch<'graph>),
    Match(IntListMatch<'graph>),
    Custom(CustomMatch<'graph>),
    Interpreted,
}

#[derive(Clone, Copy)]
pub(super) enum CompiledEdge<'graph> {
    Ordinary(&'graph Edge),
    Match(&'graph MatchEdge),
}

impl CompiledEdge<'_> {
    pub(super) fn target(self) -> BlockId {
        match self {
            Self::Ordinary(edge) => edge.target(),
            Self::Match(edge) => edge.target(),
        }
    }
}

impl<'graph, Graph: ExecutionGraphProfile> CompiledShape<'graph, Graph> {
    pub(in crate::plan::execution::prepared) fn inspect<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
    ) -> Option<Self> {
        Self::inspect_supported(body, KernelKind::Numeric, None)
    }

    pub(in crate::plan::execution::prepared) fn inspect_bits<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
    ) -> Option<Self> {
        Self::inspect_supported(body, KernelKind::BitArray, None)
    }

    pub(in crate::plan::execution::prepared) fn start(&self, block: BlockId) -> usize {
        self.starts[&block.index()]
    }

    pub(in crate::plan::execution::prepared) fn loop_calls(&self) -> Vec<CompiledLoopCall> {
        let mut calls = self
            .order
            .iter()
            .flat_map(|&block| {
                self.block(block)
                    .instructions
                    .iter()
                    .enumerate()
                    .filter_map(move |(index, instruction)| {
                        let CompiledInstruction::CustomLoop(CustomLoopInstruction::Call {
                            function,
                            args,
                            output,
                        }) = instruction
                        else {
                            return None;
                        };
                        Some(CompiledLoopCall {
                            point: self.start(block) + index,
                            function: function.clone(),
                            args: args.to_vec().into(),
                            output: output.local(),
                        })
                    })
            })
            .collect::<Vec<_>>();
        calls.sort_by_key(|call| call.point);
        calls
    }

    pub(in crate::plan::execution::prepared) fn inspect_custom_loop<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        types: &CustomTypeTable,
    ) -> Option<Self> {
        let shape = Self::inspect_supported(body, KernelKind::CustomLoop, Some(types))?;
        if !shape.repeats
            || !shape.checkpoints.iter().any(|point| point.custom_lists > 0)
            || !shape.blocks.values().any(|block| {
                block.instructions.iter().any(|instruction| {
                    matches!(
                        instruction,
                        CompiledInstruction::CustomLoop(CustomLoopInstruction::Call { .. })
                    )
                })
            })
        {
            return None;
        }
        Some(shape)
    }

    pub(in crate::plan::execution::prepared) fn inspect_callback<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        types: &CustomTypeTable,
    ) -> Option<Self> {
        let shape = Self::inspect_supported(body, KernelKind::Callback, Some(types))?;
        if shape.repeats {
            return None;
        }
        // Canonical error suffixes can remain interpreted. Additional calls or
        // successful unsupported work would create a second call-chain model.
        for (index, block) in shape.graph.blocks().enumerate() {
            if !matches!(
                block.terminator(),
                Terminator::SourceStop(_) | Terminator::LetAssertPanic(_)
            ) && shape
                .blocks
                .get(&index)
                .is_none_or(|block| matches!(block.terminator, CompiledTerminator::Interpreted))
            {
                return None;
            }
            for instruction in block.instructions() {
                let supported = CompiledInstruction::inspect(instruction)
                    .filter(|value| !matches!(value, CompiledInstruction::IntList(_)))
                    .or_else(|| {
                        CustomField::inspect(instruction).map(CompiledInstruction::CustomField)
                    });
                if supported.is_none()
                    && !matches!(instruction, ProfiledInstruction::Value(value) if matches!(value.kind(), ProfiledInstructionKind::String(StringInstruction::Value(_))))
                {
                    return None;
                }
            }
        }
        Some(shape)
    }

    pub(super) fn block(&self, block: BlockId) -> &CompiledBlock<'graph> {
        &self.blocks[&block.index()]
    }

    fn inspect_supported<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        kind: KernelKind,
        custom_types: Option<&CustomTypeTable>,
    ) -> Option<Self> {
        let bit_arrays = kind == KernelKind::BitArray;
        let custom = matches!(kind, KernelKind::CustomLoop | KernelKind::Callback);
        let partial = bit_arrays || custom;
        let graph = body.block_graph().as_view();
        let count = graph.blocks().len();
        let mut checkpoints = Vec::new();
        let mut blocks = BTreeMap::new();
        let mut starts = BTreeMap::new();
        let mut successors = Vec::with_capacity(count);
        let mut repeats = false;
        let can_repeat = if partial {
            repeating_blocks(graph)
        } else {
            vec![false; count]
        };
        for (index, block) in graph.blocks().enumerate() {
            let mut point = CompiledCheckpoint {
                block: BlockId(index),
                instruction: 0,
                ints: 0,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            };
            if block
                .params()
                .iter()
                .try_for_each(|slot| add_slot(&mut point, slot, kind))
                .is_none()
            {
                if !partial {
                    return None;
                }
                successors.push(Vec::new());
                continue;
            }
            starts.insert(index, checkpoints.len());
            checkpoints.push(point);
            let mut instructions = Vec::with_capacity(block.instructions().len());
            for instruction in block.instructions() {
                let inspected = CompiledInstruction::inspect(instruction)
                    .filter(|instruction| {
                        !(bit_arrays || custom)
                            || !matches!(instruction, CompiledInstruction::IntList(_))
                    })
                    .or_else(|| {
                        if kind == KernelKind::CustomLoop
                            && let Some(value) =
                                CustomLoopInstruction::inspect(instruction, block.params())
                        {
                            return Some(CompiledInstruction::CustomLoop(value));
                        }
                        if custom {
                            CustomField::inspect(instruction).map(CompiledInstruction::CustomField)
                        } else {
                            None
                        }
                    });
                let Some(instruction) = inspected else {
                    if !partial || can_repeat[index] {
                        return None;
                    }
                    break;
                };
                match &instruction {
                    CompiledInstruction::Integer(..) => point.ints += 1,
                    CompiledInstruction::Boolean(..) => point.bools += 1,
                    CompiledInstruction::Region { outputs, .. } => point.ints += outputs.len(),
                    CompiledInstruction::IntList(IntListInstruction::Index { .. }) => {
                        point.ints += 1
                    }
                    CompiledInstruction::IntList(
                        IntListInstruction::Value { .. }
                        | IntListInstruction::Spread { .. }
                        | IntListInstruction::Tail { .. },
                    ) => point.int_lists += 1,
                    CompiledInstruction::CustomField(CustomField::Integer { .. }) => {
                        point.ints += 1
                    }
                    CompiledInstruction::CustomField(CustomField::Boolean { .. }) => {
                        point.bools += 1
                    }
                    CompiledInstruction::CustomLoop(instruction) => match instruction {
                        CustomLoopInstruction::Index { .. } => point.customs += 1,
                        CustomLoopInstruction::Tail { .. } => point.custom_lists += 1,
                        CustomLoopInstruction::Call {
                            output: LoopCallOutput::Int(_),
                            ..
                        } => point.ints += 1,
                        CustomLoopInstruction::Call { .. } => point.bools += 1,
                    },
                }
                instructions.push(instruction);
                point.instruction += 1;
                checkpoints.push(point);
            }
            let terminator = if instructions.len() < block.instructions().len() {
                CompiledTerminator::Interpreted
            } else {
                let inspected = CompiledTerminator::inspect(block.terminator()).or_else(|| {
                    if bit_arrays && let Terminator::Match(matcher) = block.terminator() {
                        return BitArrayMatch::inspect(matcher).map(CompiledTerminator::BitArray);
                    }
                    if let Some(types) = custom_types && let Terminator::Match(matcher) = block.terminator() {
                        return CustomMatch::inspect(matcher, types).map(CompiledTerminator::Custom);
                    }
                    None
                }).filter(|terminator| {
                    !bit_arrays || !matches!(terminator, CompiledTerminator::Exit(exit) if !matches!(body.exit(*exit), FunctionExit::Return(_)))
                });
                match inspected {
                    Some(terminator) => terminator,
                    None if partial && !can_repeat[index] => CompiledTerminator::Interpreted,
                    None => return None,
                }
            };
            let edges = terminator.targets();
            repeats |= edges.contains(&graph.entry());
            successors.push(edges.iter().map(|edge| edge.index()).collect::<Vec<_>>());
            blocks.insert(
                index,
                CompiledBlock {
                    instructions,
                    terminator,
                },
            );
        }
        // Every retained edge forwards supported locals or supported match
        // bindings. The typed graph therefore gives its destination the same
        // supported families. A custom suffix stops at its first instruction,
        // before it can produce a custom value or forward it along an edge.
        starts.get(&graph.entry().index())?;
        let mut order = Vec::with_capacity(count);
        let mut visited = vec![0; count];
        visit(
            graph.entry().index(),
            graph.entry().index(),
            &successors,
            &mut visited,
            &mut order,
        )?;
        // Scalar generation covers the complete frozen graph. Pattern generation
        // stops before unsupported terminal suffixes; each retained checkpoint
        // still records an exact canonical prefix.
        if !partial && order.len() != count {
            return None;
        }
        if bit_arrays
            && !order.iter().any(|&index| {
                matches!(
                    blocks.get(&index).map(|block| &block.terminator),
                    Some(CompiledTerminator::BitArray(_))
                )
            })
        {
            return None;
        }
        // Entry back-edges and returns lead to one virtual exit. In reverse
        // topological order, every successor already has an immediate
        // postdominator. Keep that tree rather than all transitive sets:
        // a long sequence of joins then needs O(blocks) preparation storage.
        let exit = count;
        let mut parents = vec![exit; count + 1];
        let mut ranks = vec![0; count + 1];
        for (rank, &block) in order.iter().enumerate() {
            ranks[block] = rank + 1;
        }
        let mut joins = vec![None; count];
        for &block in &order {
            let mut common = None;
            for &next in &successors[block] {
                let path = if next == graph.entry().index() {
                    exit
                } else {
                    next
                };
                common = Some(match common {
                    None => path,
                    Some(previous) => common_postdominator(previous, path, &parents, &ranks),
                });
            }
            let common = common.unwrap_or(exit);
            parents[block] = common;
            if common != exit {
                joins[block] = Some(BlockId(common));
            } else {
                // A return or entry back-edge finishes its Rust branch. Other
                // paths may still share a suffix: it need not postdominate the
                // terminating path. Validate single emission below as usual.
                joins[block] = shared_suffix(block, graph.entry().index(), &successors, &ranks);
            }
        }
        let kind = if matches!(kind, KernelKind::CustomLoop | KernelKind::Callback) {
            kind
        } else if bit_arrays {
            KernelKind::BitArray
        } else if checkpoints.iter().any(|point| point.int_lists > 0) {
            KernelKind::IntList
        } else {
            KernelKind::Numeric
        };
        if count < 2 && kind == KernelKind::Numeric {
            return None;
        }
        let shape = Self {
            graph,
            blocks,
            checkpoints,
            starts,
            joins,
            repeats,
            kind,
            order: order.iter().rev().copied().map(BlockId).collect(),
        };
        if !bit_arrays {
            let mut emitted = BTreeSet::new();
            shape.structured_path(graph.entry(), None, &mut emitted)?;
        }
        Some(shape)
    }

    fn structured_path(
        &self,
        block: BlockId,
        stop: Option<BlockId>,
        emitted: &mut BTreeSet<usize>,
    ) -> Option<()> {
        if Some(block) == stop {
            return Some(());
        }
        if !emitted.insert(block.index()) {
            return None;
        }
        let outgoing = self.block(block).terminator.targets();
        let join = self.joins[block.index()];
        for edge in outgoing {
            if edge != self.graph.entry() {
                self.structured_path(edge, join.or(stop), emitted)?;
            }
        }
        if let Some(join) = join {
            self.structured_path(join, stop, emitted)?;
        }
        Some(())
    }
}

fn repeating_blocks<Graph: ExecutionGraphProfile>(graph: BlockGraphView<'_, Graph>) -> Vec<bool> {
    let mut predecessors = vec![Vec::new(); graph.blocks().len()];
    for (index, block) in graph.blocks().enumerate() {
        let targets = match block.terminator() {
            Terminator::Jump(jump) => vec![jump.edge.target()],
            Terminator::BoolBranch(branch) => vec![branch.true_.target(), branch.false_.target()],
            Terminator::TestBranch(branch) => vec![branch.true_.target(), branch.false_.target()],
            Terminator::IntSwitch(switch) => switch
                .clauses
                .iter()
                .map(|(_, edge)| edge.target())
                .chain(std::iter::once(switch.fallback.target()))
                .collect(),
            Terminator::FloatSwitch(switch) => switch
                .clauses
                .iter()
                .map(|(_, edge)| edge.target())
                .chain(std::iter::once(switch.fallback.target()))
                .collect(),
            Terminator::StringSwitch(switch) => switch
                .clauses
                .iter()
                .map(|(_, edge)| edge.target())
                .chain(std::iter::once(switch.fallback.target()))
                .collect(),
            Terminator::Match(matcher) => vec![matcher.success.target(), matcher.failure.target()],
            Terminator::Echo(echo) => vec![echo.next.target()],
            Terminator::Exit(_)
            | Terminator::SourceStop(_)
            | Terminator::LetAssertPanic(_)
            | Terminator::NeverCall(_) => Vec::new(),
        };
        for target in targets {
            predecessors[target.index()].push(index);
        }
    }
    let mut repeating = vec![false; predecessors.len()];
    let mut pending = vec![graph.entry().index()];
    while let Some(block) = pending.pop() {
        if !repeating[block] {
            repeating[block] = true;
            pending.extend_from_slice(&predecessors[block]);
        }
    }
    repeating
}

fn shared_suffix(
    block: usize,
    entry: usize,
    successors: &[Vec<usize>],
    ranks: &[usize],
) -> Option<BlockId> {
    if successors[block].len() < 2 {
        return None;
    }
    let mut visits = vec![0; successors.len()];
    let mut paths = vec![0; successors.len()];
    let mut selected = None;
    for (index, &start) in successors[block].iter().enumerate() {
        let mut pending = vec![start];
        while let Some(next) = pending.pop() {
            if next == entry || visits[next] == index + 1 {
                continue;
            }
            visits[next] = index + 1;
            paths[next] += 1;
            if paths[next] > 1 && selected.is_none_or(|previous| ranks[next] > ranks[previous]) {
                selected = Some(next);
            }
            pending.extend(&successors[next]);
        }
    }
    selected.map(BlockId)
}

fn common_postdominator(
    mut left: usize,
    mut right: usize,
    parents: &[usize],
    ranks: &[usize],
) -> usize {
    while left != right {
        if ranks[left] > ranks[right] {
            left = parents[left];
        } else {
            right = parents[right];
        }
    }
    left
}

fn add_slot(point: &mut CompiledCheckpoint, slot: &ParamSlot, kind: KernelKind) -> Option<()> {
    match slot.local() {
        ParamLocal::Int(_) => point.ints += 1,
        ParamLocal::Bool(_) => point.bools += 1,
        ParamLocal::List(ListLocal::Int { .. }) if kind == KernelKind::Numeric => {
            point.int_lists += 1
        }
        ParamLocal::BitArray(_) if kind == KernelKind::BitArray => point.bit_arrays += 1,
        ParamLocal::Custom(_) if matches!(kind, KernelKind::CustomLoop | KernelKind::Callback) => {
            point.customs += 1
        }
        ParamLocal::List(ListLocal::Custom { .. }) if kind == KernelKind::CustomLoop => {
            point.custom_lists += 1
        }
        ParamLocal::IntFunction { .. } if kind == KernelKind::CustomLoop => {
            point.int_functions += 1
        }
        ParamLocal::BoolFunction { .. } if kind == KernelKind::CustomLoop => {
            point.bool_functions += 1
        }
        _ => return None,
    }
    Some(())
}

impl<'graph> CompiledInstruction<'graph> {
    fn inspect<Graph: ExecutionGraphProfile>(
        instruction: &'graph ProfiledInstruction<Graph>,
    ) -> Option<Self> {
        match instruction {
            ProfiledInstruction::IntegerRegion(region) if region.native => {
                let outputs = region
                    .outputs
                    .iter()
                    .filter_map(|output| {
                        output
                            .slot
                            .local()
                            .storage_slot()
                            .map(|slot| IntLocalId(slot.index))
                    })
                    .collect();
                // Frozen graph lowering and artifact graph admission already
                // establish that every arithmetic output is an Int slot.
                Some(Self::Region { region, outputs })
            }
            ProfiledInstruction::Value(value) => match (value.kind(), value.output().local()) {
                (
                    ProfiledInstructionKind::Int(IntInstruction::ListIndex { list, index }),
                    ParamLocal::Int(output),
                ) => Some(Self::IntList(IntListInstruction::Index {
                    output: *output,
                    list: *list,
                    index: *index,
                })),
                (ProfiledInstructionKind::Int(instruction), ParamLocal::Int(output)) => Some(
                    Self::Integer(*output, NumericInteger::inspect(instruction)?),
                ),
                (
                    ProfiledInstructionKind::List(ListInstruction::Int(
                        type_id,
                        TypedListInstruction::Value(elements),
                    )),
                    ParamLocal::List(ListLocal::Int { local, .. }),
                ) => Some(Self::IntList(IntListInstruction::Value {
                    output: *local,
                    type_id: *type_id,
                    elements,
                })),
                (
                    ProfiledInstructionKind::List(ListInstruction::Int(
                        type_id,
                        TypedListInstruction::Spread { elements, tail },
                    )),
                    ParamLocal::List(ListLocal::Int { local, .. }),
                ) => Some(Self::IntList(IntListInstruction::Spread {
                    output: *local,
                    type_id: *type_id,
                    elements,
                    tail: *tail,
                })),
                (
                    ProfiledInstructionKind::List(ListInstruction::Int(
                        type_id,
                        TypedListInstruction::DropFirst { list, count },
                    )),
                    ParamLocal::List(ListLocal::Int { local, .. }),
                ) => Some(Self::IntList(IntListInstruction::Tail {
                    output: *local,
                    type_id: *type_id,
                    list: *list,
                    count: *count,
                })),
                (ProfiledInstructionKind::Bool(instruction), ParamLocal::Bool(output)) => {
                    let expression = match instruction {
                        BoolInstruction::Value(value) => CompiledBoolean::Value(*value),
                        BoolInstruction::Test(test) => {
                            CompiledBoolean::Test(CompiledTest::inspect(test)?)
                        }
                        _ => return None,
                    };
                    Some(Self::Boolean(*output, expression))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl<'graph> NumericInteger<'graph> {
    fn inspect(instruction: &'graph IntInstruction) -> Option<Self> {
        let (operation, left, right) = match instruction {
            IntInstruction::Value(literal) if i64::try_from(literal.materialize()).is_ok() => {
                return Some(Self::Value(literal));
            }
            IntInstruction::Add { left, right } => (NumericOperation::Add, left, right),
            IntInstruction::Sub { left, right } => (NumericOperation::Subtract, left, right),
            IntInstruction::Mult { left, right } => (NumericOperation::Multiply, left, right),
            IntInstruction::Div { left, right } => (NumericOperation::Divide, left, right),
            IntInstruction::Remainder { left, right } => (NumericOperation::Remainder, left, right),
            IntInstruction::Negate(value) => return Some(Self::Negate(*value)),
            _ => return None,
        };
        Some(Self::Binary(operation, *left, *right))
    }
}

impl CompiledTest {
    fn inspect(test: &BoolTest) -> Option<Self> {
        let (comparison, left, right) = match test {
            BoolTest::Not(value) => return Some(Self::Not(*value)),
            BoolTest::ListLengthEquals {
                value: ListLocal::Custom { local, .. },
                length,
            } => {
                return Some(Self::CustomListLength {
                    list: *local,
                    length: *length,
                    at_least: false,
                });
            }
            BoolTest::ListLengthAtLeast {
                value: ListLocal::Custom { local, .. },
                length,
            } => {
                return Some(Self::CustomListLength {
                    list: *local,
                    length: *length,
                    at_least: true,
                });
            }
            BoolTest::EqualInt { left, right } => (NumericComparison::Equal, left, right),
            BoolTest::NotEqualInt { left, right } => (NumericComparison::NotEqual, left, right),
            BoolTest::LtInt { left, right } => (NumericComparison::Less, left, right),
            BoolTest::LtEqInt { left, right } => (NumericComparison::LessEqual, left, right),
            BoolTest::GtInt { left, right } => (NumericComparison::Greater, left, right),
            BoolTest::GtEqInt { left, right } => (NumericComparison::GreaterEqual, left, right),
            _ => return IntListTest::inspect(test).map(Self::IntList),
        };
        Some(Self::Compare(comparison, *left, *right))
    }
}

impl<'graph> CompiledTerminator<'graph> {
    fn inspect(terminator: &'graph Terminator) -> Option<Self> {
        Some(match terminator {
            Terminator::Jump(jump) => Self::Jump(&jump.edge),
            Terminator::BoolBranch(branch) => Self::Boolean {
                subject: branch.subject,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::TestBranch(branch) => Self::Test {
                test: CompiledTest::inspect(&branch.test)?,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::IntSwitch(switch)
                if switch
                    .clauses
                    .iter()
                    .all(|(literal, _)| i64::try_from(literal.materialize()).is_ok()) =>
            {
                Self::Switch {
                    subject: switch.subject,
                    clauses: &switch.clauses,
                    fallback: &switch.fallback,
                }
            }
            Terminator::Exit(exit) => Self::Exit(*exit),
            Terminator::Match(matcher) => Self::Match(IntListMatch::inspect(matcher)?),
            Terminator::SourceStop(stop) if stop.message().is_none() => Self::Interpreted,
            Terminator::LetAssertPanic(panic) if panic.message().is_none() => Self::Interpreted,
            _ => return None,
        })
    }

    fn targets(&self) -> Vec<BlockId> {
        match self {
            Self::BitArray(matcher) => vec![matcher.success.target(), matcher.failure.target()],
            _ => self.edges().iter().map(|edge| edge.target()).collect(),
        }
    }

    fn edges(&self) -> Vec<CompiledEdge<'graph>> {
        match self {
            Self::Jump(edge) => vec![CompiledEdge::Ordinary(edge)],
            Self::Boolean { true_, false_, .. } | Self::Test { true_, false_, .. } => {
                vec![
                    CompiledEdge::Ordinary(true_),
                    CompiledEdge::Ordinary(false_),
                ]
            }
            Self::Switch {
                clauses, fallback, ..
            } => clauses
                .iter()
                .map(|(_, edge)| CompiledEdge::Ordinary(edge))
                .chain(std::iter::once(CompiledEdge::Ordinary(fallback)))
                .collect(),
            Self::Match(view) => vec![
                CompiledEdge::Match(view.matcher.success()),
                CompiledEdge::Ordinary(view.matcher.failure()),
            ],
            Self::Custom(view) => vec![
                CompiledEdge::Match(view.matcher.success()),
                CompiledEdge::Ordinary(view.matcher.failure()),
            ],
            Self::BitArray(_) | Self::Exit(_) | Self::Interpreted => vec![],
        }
    }
}

fn visit(
    block: usize,
    entry: usize,
    successors: &[Vec<usize>],
    visited: &mut [u8],
    order: &mut Vec<usize>,
) -> Option<()> {
    match visited[block] {
        1 => return None,
        2 => return Some(()),
        _ => {}
    }
    visited[block] = 1;
    for &next in &successors[block] {
        if next != entry {
            visit(next, entry, successors, visited, order)?;
        }
    }
    visited[block] = 2;
    order.push(block);
    Some(())
}

#[cfg(test)]
mod tests {
    use super::{CompiledCheckpoint, CompiledShape, shared_suffix, visit};
    use crate::plan::execution::function::{ExecutionIntFunctionBody, FunctionExit, IntFunctionId};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, Edge, IntLocalId, IntSwitch, IntegerLiteral, Jump, ParamLocal,
        ProfiledBlock, ProfiledBlockGraph, StringLocalId, Terminator,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use std::collections::{BTreeMap, BTreeSet};
    use std::convert::Infallible;

    fn source_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
    }

    #[test]
    fn construction_without_list_inputs_keeps_each_completed_list_output_in_its_checkpoint() {
        use super::KernelKind;
        for (source, expected) in [
            (
                "pub fn main() -> List(Int) { [] }",
                vec![(0, 0, 0), (1, 0, 1)],
            ),
            (
                "pub fn main() -> List(Int) { [7, -9] }",
                vec![(0, 0, 0), (1, 1, 0), (2, 2, 0), (3, 2, 1)],
            ),
        ] {
            let plan = source_plan(source);
            let body = plan.int_list_function(plan.int_list_function_id(0)).body();
            let shape = CompiledShape::inspect(body).unwrap();
            assert_eq!(shape.kind, KernelKind::IntList);
            assert_eq!(shape.starts, BTreeMap::from([(0, 0)]));
            assert!(!shape.repeats);
            assert_eq!(
                shape.checkpoints,
                expected
                    .into_iter()
                    .map(|(instruction, ints, int_lists)| CompiledCheckpoint {
                        block: BlockId(0),
                        instruction,
                        ints,
                        bools: 0,
                        bit_arrays: 0,
                        int_lists,
                        customs: 0,
                        custom_lists: 0,
                        int_functions: 0,
                        bool_functions: 0,
                    })
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn list_creation_keeps_bit_kernel_repetition_closed_and_terminal_suffixes_interpreted() {
        for (source, compiled) in [
            (
                r#"
fn walk(input: BitArray, total: Int) {
  case input {
    <<value:8, rest:bits>> -> {
      let assert [head, ..] = [value, value]
      walk(rest, total + head)
    }
    _ -> total
  }
}
pub fn main() { walk(<<1, 2>>, 0) }
"#,
                false,
            ),
            (
                r#"
fn walk(input: BitArray, total: Int) {
  case input {
    <<value:8, rest:bits>> -> walk(rest, total + value)
    _ -> {
      let assert [head, ..] = [total, 7]
      head
    }
  }
}
pub fn main() { walk(<<1, 2>>, 0) }
"#,
                true,
            ),
        ] {
            let plan = source_plan(source);
            let shape = CompiledShape::inspect_bits(plan.int_function(IntFunctionId(1)).body());
            assert_eq!(shape.is_some(), compiled);
            if let Some(shape) = shape {
                assert_eq!(shape.kind, super::KernelKind::BitArray);
                assert!(shape.repeats);
                assert!(shape.blocks.values().any(|block| {
                    matches!(block.terminator, super::CompiledTerminator::Interpreted)
                }));
                assert!(shape.checkpoints.iter().all(|point| point.int_lists == 0));
            }
            assert_eq!(
                crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
                crate::Value::Int(3.into())
            );
        }
    }

    #[test]
    fn typed_tail_exits_are_supported_without_generating_the_callee_call() {
        use super::KernelKind;
        let plan = source_plan(
            "fn prepend(first: Int, second: Int, tail: List(Int)) { [first, second, ..tail] } pub fn main() { prepend(7, -9, []) }",
        );
        let entry = plan.int_list_function(plan.int_list_function_id(0)).body();
        let callee = plan.int_list_function(plan.int_list_function_id(1)).body();
        let shape = CompiledShape::inspect(entry).unwrap();
        assert_eq!(shape.kind, KernelKind::IntList);
        assert_eq!(entry.exits.len(), 1);
        assert_eq!(
            Rust::expression(&entry.exits[0]),
            r#"
data::function::FunctionExit::TailCall {
    function: data::source::FunctionCallTarget {
        function: data::function::IntListFunctionId {
            index: 1,
            type_id: data::type_::IntListTypeId {
                list_type: data::type_::ListTypeId(0),
            },
        },
        site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(97, 115)),
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
            local: data::graph::IntListLocalId(0),
            type_id: data::type_::IntListTypeId {
                list_type: data::type_::ListTypeId(0),
            },
        }),
    ]),
    transfer: data::graph::Transfer {
        families: data::Storage::Static(&[]),
    },
}
"#.trim_matches('\n')
        );
        let shape = CompiledShape::inspect(callee).unwrap();
        assert_eq!(shape.kind, KernelKind::IntList);
        assert_eq!(
            shape.checkpoints,
            [
                CompiledCheckpoint {
                    block: BlockId(0),
                    instruction: 0,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 1,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(0),
                    instruction: 1,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 2,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
            ]
        );
        assert_eq!(
            Rust::expression(&callee.exits[0]),
            "data::function::FunctionExit::Return(data::graph::IntListLocalId(1))"
        );
    }

    #[test]
    fn numeric_int_and_bool_branches_keep_the_same_canonical_tail_exit_rule() {
        use super::KernelKind;
        use crate::plan::execution::function::BoolFunctionId;
        let plan = source_plan(
            "fn other(value: Int) { value + 2 } fn choose(value: Int, flag: Bool) { case flag { True -> other(value) False -> value } } pub fn main() { choose(7, True) }",
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let shape = CompiledShape::inspect(body).unwrap();
        assert_eq!(shape.kind, KernelKind::Numeric);
        assert_eq!(shape.starts.len(), 3);
        assert!(
            body.exits
                .iter()
                .any(|exit| matches!(exit, FunctionExit::TailCall { .. }))
        );
        let plan = source_plan(
            "fn other(value: Bool) { !value } fn choose(value: Bool, flag: Bool) { case flag { True -> other(value) False -> value } } pub fn main() { choose(True, False) }",
        );
        let body = plan.bool_function(BoolFunctionId(1)).body();
        let shape = CompiledShape::inspect(body).unwrap();
        assert_eq!(shape.kind, KernelKind::Numeric);
        assert_eq!(shape.starts.len(), 3);
        assert!(
            body.exits
                .iter()
                .any(|exit| matches!(exit, FunctionExit::TailCall { .. }))
        );
    }

    #[test]
    fn fixed_bit_fields_repeat_with_shared_guard_failure_and_terminal_panic_prefixes() {
        let plan = source_plan(
            r#"
fn walk(input: BitArray, total: Int) {
  case input {
    <<value:8, rest:bits>> if value > 0 -> walk(rest, total + value)
    <<0:8, rest:bits>> -> walk(rest, total)
    <<>> -> total
    _ -> panic as "incomplete"
  }
}
pub fn main() { walk(<<1, 0, 2>>, 0) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        assert!(CompiledShape::inspect(body).is_none());
        let shape = CompiledShape::inspect_bits(body).unwrap();
        assert!(shape.repeats);
        assert_eq!(
            shape.checkpoints[shape.start(shape.graph.entry())].bit_arrays,
            1
        );
        assert_eq!(shape.order.first(), Some(&shape.graph.entry()));
        let matches = shape
            .blocks
            .values()
            .filter(|block| matches!(block.terminator, super::CompiledTerminator::BitArray(_)))
            .count();
        assert_eq!(matches, 3);
        for block in shape.blocks.values() {
            if let super::CompiledTerminator::BitArray(matcher) = &block.terminator {
                assert_eq!(
                    block.terminator.targets(),
                    [matcher.success.target(), matcher.failure.target()]
                );
                assert!(block.terminator.edges().is_empty());
            }
        }
        assert!(
            shape
                .blocks
                .values()
                .any(|block| matches!(block.terminator, super::CompiledTerminator::Interpreted))
        );
        for point in &shape.checkpoints {
            assert!(point.instruction <= shape.block(point.block).instructions.len());
        }
    }

    #[test]
    fn boolean_short_circuit_edges_participate_in_the_entry_repeat() {
        let plan = source_plan(
            r#"
fn walk(input: BitArray, total: Int, flag: Bool) {
  case flag && total < 10 {
    True -> case input {
      <<value:8, rest:bits>> -> walk(rest, total + value, flag)
      _ -> total
    }
    False -> total
  }
}
pub fn main() { walk(<<1, 2>>, 0, True) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        assert!(
            graph
                .blocks()
                .any(|block| matches!(block.terminator(), Terminator::BoolBranch(_)))
        );
        let shape = CompiledShape::inspect_bits(body).unwrap();
        assert!(shape.repeats);
        assert_eq!(
            crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(3.into())
        );
    }

    #[test]
    fn terminal_calls_keep_their_original_exit_and_scalar_bodies_need_no_bit_kernel() {
        let plan = source_plan(
            r#"
fn other(total: Int) { total + 1 }
fn walk(input: BitArray, total: Int) {
  case input {
    <<value:8, rest:bits>> -> walk(rest, total + value)
    _ -> other(total)
  }
}
pub fn main() { walk(<<1>>, 0) }
"#,
        );
        let shapes = (0..3)
            .map(|index| {
                CompiledShape::inspect_bits(plan.int_function(IntFunctionId(index)).body())
            })
            .collect::<Vec<_>>();
        assert_eq!(shapes.iter().filter(|shape| shape.is_some()).count(), 1);
        let shape = shapes.into_iter().flatten().next().unwrap();
        assert!(
            shape
                .blocks
                .values()
                .any(|block| matches!(block.terminator, super::CompiledTerminator::Interpreted))
        );
        let scalar = source_plan(
            "fn choose(value: Int) { case value { 0 -> 1 _ -> value } } pub fn main() { choose(2) }",
        );
        assert!(
            CompiledShape::inspect_bits(scalar.int_function(IntFunctionId(1)).body()).is_none()
        );
    }

    #[test]
    fn nonnumeric_switches_and_big_integer_switches_cannot_enter_a_generated_repeat() {
        for source in [
            "fn walk(input: BitArray, total: Int, flag: Float) { case flag { 0.0 -> total _ -> case input { <<value:8, rest:bits>> -> walk(rest, total + value, flag) _ -> total } } } pub fn main() { walk(<<1>>, 0, 1.0) }",
            "fn walk(input: BitArray, total: Int, flag: String) { case flag { \"\" -> total _ -> case input { <<value:8, rest:bits>> -> walk(rest, total + value, flag) _ -> total } } } pub fn main() { walk(<<1>>, 0, \"x\") }",
            "fn walk(input: BitArray, total: Int) { case total { 9223372036854775808 -> total _ -> case input { <<value:8, rest:bits>> -> walk(rest, total + value) _ -> total } } } pub fn main() { walk(<<1>>, 0) }",
        ] {
            let plan = source_plan(source);
            assert!(
                CompiledShape::inspect_bits(plan.int_function(IntFunctionId(1)).body()).is_none(),
                "{source}"
            );
        }
    }

    #[test]
    fn unsupported_bit_fields_and_interpreted_work_inside_repetition_keep_the_original_executor() {
        for source in [
            "fn walk(input: BitArray, total: Int, width: Int) { case input { <<value:size(width), rest:bits>> -> walk(rest, total + value, width) <<>> -> total _ -> -1 } } pub fn main() { walk(<<1>>, 0, 8) }",
            "fn walk(input: BitArray, total: Int) { case input { <<value:65, rest:bits>> -> walk(rest, total + value) <<>> -> total _ -> -1 } } pub fn main() { walk(<<1>>, 0) }",
            "fn walk(input: BitArray, total: Int) { case input { <<_:64-float, rest:bits>> -> walk(rest, total + 1) <<>> -> total _ -> -1 } } pub fn main() { walk(<<1>>, 0) }",
            "fn walk(input: BitArray, total: Int) { case input { <<_:utf8, rest:bits>> -> walk(rest, total + 1) <<>> -> total _ -> -1 } } pub fn main() { walk(<<1>>, 0) }",
            "fn walk(input: BitArray, total: Int) { case input { <<_:8, rest:bits>> -> walk(rest, other(total)) <<>> -> total _ -> -1 } } fn other(total: Int) { total + 1 } pub fn main() { walk(<<1>>, 0) }",
            "fn walk(input: BitArray, total: Int) { case input { <<value:8, rest:bits>> -> { echo value walk(rest, total + value) } <<>> -> total _ -> -1 } } pub fn main() { walk(<<1>>, 0) }",
            "fn walk(left: BitArray, right: BitArray, total: Int) { case left, right { <<a:8, l:bits>>, <<b:8, r:bits>> -> walk(l, r, total + a + b) _, _ -> total } } pub fn main() { walk(<<1>>, <<2>>, 0) }",
        ] {
            let plan = source_plan(source);
            assert!(
                CompiledShape::inspect_bits(plan.int_function(IntFunctionId(1)).body()).is_none(),
                "{source}"
            );
        }
    }

    #[test]
    fn shared_suffix_joins_only_distinct_paths_before_a_return_or_entry_back_edge() {
        // Entry branches: one returns, two share block 4, which loops to entry.
        // Block 4 cannot postdominate the returning path, but is emitted once.
        let successors = vec![vec![1, 2, 3], vec![], vec![4], vec![4], vec![0]];
        let ranks = [5, 1, 3, 4, 2];
        assert_eq!(shared_suffix(0, 0, &successors, &ranks), Some(BlockId(4)));
        assert_eq!(shared_suffix(2, 0, &successors, &ranks), None);
        assert_eq!(shared_suffix(1, 0, &successors, &ranks), None);
        // Two paths inside only one outgoing branch do not establish a join
        // with the other branch, which returns at block 1.
        let successors = vec![vec![1, 2], vec![], vec![3, 4], vec![5], vec![5], vec![]];
        let ranks = [6, 1, 5, 3, 4, 2];
        assert_eq!(shared_suffix(0, 0, &successors, &ranks), None);
        assert_eq!(shared_suffix(2, 0, &successors, &ranks), Some(BlockId(5)));
    }

    #[test]
    fn entry_repetition_has_complete_checkpoints_for_region_outputs_without_internal_nodes() {
        let plan = source_plan(
            r#"
fn walk(n: Int, total: Int) {
  case n {
    0 -> total
    _ -> walk(n - 1, total + 1)
  }
}

pub fn main() { walk(3, 0) }
"#,
        );
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        assert!(shape.repeats);
        assert_eq!(shape.starts, BTreeMap::from([(0, 0), (1, 1), (2, 2)]));
        assert_eq!(shape.joins, [None, None, None]);
        assert_eq!(
            shape.checkpoints,
            [
                CompiledCheckpoint {
                    block: BlockId(0),
                    instruction: 0,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(1),
                    instruction: 0,
                    ints: 1,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(2),
                    instruction: 0,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(2),
                    instruction: 1,
                    ints: 4,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
            ]
        );
    }

    #[test]
    fn a_join_is_emitted_once_after_nested_scalar_conditions() {
        let plan = source_plan(
            r#"
fn choose(value: Int, flag: Bool) {
  let selected = case flag {
    True -> value + 1
    False -> value - 1
  }
  case selected > 0 {
    True -> selected
    False -> 0
  }
}

pub fn main() { choose(7, True) }
"#,
        );
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        assert!(!shape.repeats);
        let join = shape.joins[0].unwrap();
        assert_ne!(join, shape.graph.entry());
        let point = shape.checkpoints[shape.start(join)];
        assert_eq!((point.instruction, point.ints, point.bools), (0, 1, 0));
        assert_eq!(
            shape.checkpoints.len(),
            shape
                .graph
                .blocks()
                .map(|block| block.instructions().len() + 1)
                .sum::<usize>()
        );
    }

    #[test]
    fn valid_bodies_outside_the_numeric_scope_keep_interpreted_execution() {
        for source in [
            r#"
fn choose(value: Int) { value + 1 }

pub fn main() { choose(7) }
"#,
            r#"
const offset = 2

fn choose(value: Int, flag: Bool) {
  case flag {
    True -> value + offset
    False -> value
  }
}

pub fn main() { choose(7, True) }
"#,
            r#"
fn other(value: Int) { value + 2 }

fn choose(value: Int, flag: Bool) {
  let result = other(value)
  case flag {
    True -> result
    False -> value
  }
}

pub fn main() { choose(7, True) }
"#,
            r#"
fn choose(value: Int, text: String) {
  case text {
    "" -> value
    _ -> value + 1
  }
}

pub fn main() { choose(7, "x") }
"#,
            r#"
fn choose(value: Int, flag: Bool) {
  case flag {
    True -> 9223372036854775808
    False -> value
  }
}

pub fn main() { choose(7, True) }
"#,
            r#"
fn choose(value: Int, flag: Bool) {
  let result = value * value * value
  case flag {
    True -> result
    False -> 0
  }
}

pub fn main() { choose(7, True) }
"#,
            r#"
fn choose(left: Bool, right: Bool) {
  case left == right {
    True -> 1
    False -> 2
  }
}

pub fn main() { choose(True, False) }
"#,
            r#"
fn invert(value: Bool) { !value }

fn choose(value: Bool) {
  let flag = invert(value)
  case flag {
    True -> 1
    False -> 2
  }
}

pub fn main() { choose(True) }
"#,
            r#"
fn choose(left: Bool, right: Bool) {
  let same = left == right
  case left {
    True -> case same {
      True -> 1
      False -> 2
    }
    False -> 3
  }
}

pub fn main() { choose(True, False) }
"#,
        ] {
            let plan = source_plan(source);
            assert!(
                CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).is_none(),
                "{source}"
            );
        }
    }

    fn ordinary_edge(terminator: &Terminator) -> &Edge {
        match super::CompiledTerminator::inspect(terminator)
            .unwrap()
            .edges()[0]
        {
            super::CompiledEdge::Ordinary(edge) => edge,
            super::CompiledEdge::Match(_) => panic!("ordinary numeric edge"),
        }
    }

    #[test]
    #[should_panic(expected = "ordinary numeric edge")]
    fn numeric_edge_fixture_rejects_a_pattern_binding_edge() {
        let plan = source_plan(
            "fn head(values: List(Int)) { let assert [first, ..] = values first } pub fn main() { head([1]) }",
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let terminator = body
            .block_graph()
            .blocks()
            .map(|block| block.terminator())
            .find(|terminator| matches!(terminator, Terminator::Match(_)))
            .unwrap();
        ordinary_edge(terminator);
    }

    #[test]
    fn incomplete_graphs_and_internal_cycle_margins_cannot_publish_numeric_checkpoints() {
        let plan = source_plan(
            r#"
fn walk(n: Int, total: Int) {
  case n {
    0 -> total
    _ -> walk(n - 1, total + 1)
  }
}

pub fn main() { walk(3, 0) }
"#,
        );
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        for change in [0, 1, 2] {
            let mut blocks = graph
                .blocks()
                .enumerate()
                .map(|(index, block)| {
                    let mut params = block.params().to_vec();
                    let mut terminator = block.terminator().clone();
                    if index == graph.entry().index() {
                        if change == 0 {
                            let edge = ordinary_edge(&terminator);
                            terminator = Terminator::Jump(Jump::new(edge.clone()));
                        } else if change == 1 {
                            params[0].local = ParamLocal::String(StringLocalId(0));
                        }
                    }
                    ProfiledBlock::new(params, block.instructions().to_vec(), terminator)
                })
                .collect::<Vec<_>>();
            // The third block normally jumps to entry; point it to itself for
            // the non-entry-cycle margin, keeping its real arguments and slots.
            if change == 2 {
                let (params, instructions, terminator) = blocks.pop().unwrap().into_parts();
                let edge = ordinary_edge(&terminator);
                let mut edge = edge.clone();
                edge.target = BlockId(2);
                blocks.push(ProfiledBlock::new(
                    params.into(),
                    instructions.into(),
                    Terminator::Jump(Jump::new(edge)),
                ));
            }
            let body: ExecutionIntFunctionBody<Infallible> =
                ExecutionIntFunctionBody::<Infallible>::from_parts(
                    ProfiledBlockGraph::from_parts(graph.entry(), blocks),
                    vec![FunctionExit::Return(IntLocalId(0))].into(),
                );
            assert!(CompiledShape::inspect(&body).is_none(), "margin {change}");
        }
    }

    #[test]
    fn overlapping_branch_bodies_keep_interpreted_execution_instead_of_duplicating_code() {
        let plan = source_plan(
            r#"
fn choose(value: Int) {
  case value {
    0 -> value + 1
    _ -> value - 1
  }
}

pub fn main() { choose(7) }
"#,
        );
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let params = graph.block(graph.entry()).params();
        let edge = ordinary_edge(graph.block(graph.entry()).terminator());
        let transfer = edge.transfer.clone();
        let edge = |target| {
            Edge::new(
                BlockId(target),
                vec![ParamLocal::Int(IntLocalId(0))],
                transfer.clone(),
            )
        };
        let branch = |left, right| {
            Terminator::IntSwitch(IntSwitch {
                subject: IntLocalId(0),
                clauses: vec![(
                    IntegerLiteral::from(num_bigint::BigInt::from(0)),
                    edge(left),
                )]
                .into(),
                fallback: edge(right),
            })
        };
        // Both children share two bodies without a common join. This valid DAG
        // is outside this emitter's structured scope; it must remain interpreted.
        let blocks = [
            branch(1, 2),
            branch(3, 4),
            branch(3, 4),
            Terminator::Exit(BlockGraphExitId(0)),
            Terminator::Exit(BlockGraphExitId(0)),
        ]
        .into_iter()
        .map(|terminator| ProfiledBlock::new(params.to_vec(), vec![], terminator))
        .collect();
        let body: ExecutionIntFunctionBody<Infallible> =
            ExecutionIntFunctionBody::<Infallible>::from_parts(
                ProfiledBlockGraph::from_parts(BlockId(0), blocks),
                vec![FunctionExit::Return(IntLocalId(0))].into(),
            );
        assert!(CompiledShape::inspect(&body).is_none());

        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let mut already_emitted = BTreeSet::from([shape.graph.entry().index()]);
        assert_eq!(
            shape.structured_path(shape.graph.entry(), None, &mut already_emitted),
            None
        );
        assert_eq!(
            shape.structured_path(
                shape.graph.entry(),
                Some(shape.graph.entry()),
                &mut already_emitted
            ),
            Some(())
        );

        let joined = source_plan(
            r#"
fn choose(value: Int, flag: Bool) {
  let selected = case flag {
    True -> value + 1
    False -> value - 1
  }
  case selected > 0 {
    True -> selected
    False -> 0
  }
}

pub fn main() { choose(7, True) }
"#,
        );
        let shape = CompiledShape::inspect(joined.int_function(IntFunctionId(1)).body()).unwrap();
        let join = shape.joins[shape.graph.entry().index()].unwrap();
        let mut already_emitted = BTreeSet::from([join.index()]);
        assert_eq!(
            shape.structured_path(shape.graph.entry(), None, &mut already_emitted),
            None
        );
    }

    #[test]
    fn internal_cycles_are_rejected_while_revisits_and_entry_edges_are_accepted() {
        let mut order = vec![];
        assert_eq!(
            visit(
                0,
                0,
                &[vec![1, 2], vec![3], vec![3], vec![0]],
                &mut [0; 4],
                &mut order
            ),
            Some(())
        );
        assert_eq!(order, [3, 1, 2, 0]);
        assert_eq!(
            visit(0, 0, &[vec![1], vec![2], vec![1]], &mut [0; 3], &mut vec![]),
            None
        );
    }

    #[test]
    fn completed_boolean_values_extend_the_actual_checkpoint_prefix() {
        let plan = source_plan(
            r#"
fn choose(flag: Bool) {
  let flag = !flag
  let flag = !flag
  let flag = !flag
  case flag {
    True -> 7
    False -> -7
  }
}

pub fn main() { choose(True) }
"#,
        );
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        // Two completed Not values precede the final Not condition. The
        // selected literal blocks have no Boolean arguments or outputs.
        assert_eq!(
            shape
                .checkpoints
                .iter()
                .map(|point| point.bools)
                .collect::<Vec<_>>(),
            [1, 2, 3, 0, 0, 0, 0]
        );
    }

    #[test]
    fn a_float_computation_inside_an_int_function_remains_interpreted() {
        let plan = source_plan(
            r#"
fn choose(flag: Bool) {
  let value = 1.5
  case flag {
    True -> case value >. 0.0 {
      True -> 1
      False -> 2
    }
    False -> 3
  }
}

pub fn main() { choose(True) }
"#,
        );
        assert!(CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).is_none());
    }

    #[test]
    fn generic_boolean_equality_remains_outside_the_numeric_test_contract() {
        let plan = source_plan(
            r#"
fn choose(left: Bool, right: Bool) {
  case left == right {
    True -> 7
    False -> -7
  }
}

pub fn main() { choose(True, False) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        let terminator = graph.block(graph.entry()).terminator();
        assert!(super::CompiledTerminator::inspect(terminator).is_none());
        assert!(CompiledShape::inspect(body).is_none());
    }

    #[test]
    fn integer_tests_preserve_the_exact_comparison_and_canonical_operands() {
        use super::{CompiledTest, NumericComparison};
        use crate::plan::execution::graph::{BoolLocalId, BoolTest, IntegerOperand};
        use crate::plan::execution::prepared::rust::Rust;
        use std::mem::discriminant;

        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Immediate(-7);
        let inspected_comparison = |test: &BoolTest| match CompiledTest::inspect(test) {
            Some(CompiledTest::Compare(comparison, left, right)) => Some((
                discriminant(&comparison),
                Rust::expression(&left),
                Rust::expression(&right),
            )),
            _ => None,
        };
        let inspected_not = |test: &BoolTest| match CompiledTest::inspect(test) {
            Some(CompiledTest::Not(value)) => Some(value.0),
            _ => None,
        };
        for (test, expected) in [
            (BoolTest::EqualInt { left, right }, NumericComparison::Equal),
            (
                BoolTest::NotEqualInt { left, right },
                NumericComparison::NotEqual,
            ),
            (BoolTest::LtInt { left, right }, NumericComparison::Less),
            (
                BoolTest::LtEqInt { left, right },
                NumericComparison::LessEqual,
            ),
            (BoolTest::GtInt { left, right }, NumericComparison::Greater),
            (
                BoolTest::GtEqInt { left, right },
                NumericComparison::GreaterEqual,
            ),
        ] {
            assert_eq!(inspected_not(&test), None);
            assert_eq!(
                inspected_comparison(&test),
                Some((
                    discriminant(&expected),
                    Rust::expression(&left),
                    Rust::expression(&right),
                ))
            );
        }
        assert_eq!(inspected_not(&BoolTest::Not(BoolLocalId(5))), Some(5));
        assert_eq!(inspected_comparison(&BoolTest::Not(BoolLocalId(5))), None);
        assert_eq!(
            inspected_comparison(&BoolTest::Equal {
                left: ParamLocal::Bool(BoolLocalId(0)),
                right: ParamLocal::Bool(BoolLocalId(1)),
            }),
            None
        );
    }

    #[test]
    fn an_effectful_terminator_keeps_its_function_interpreted() {
        let plan = source_plan(
            r#"
fn choose(value: Int) {
  echo value
  value
}

pub fn main() { choose(7) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        let terminator = graph.block(graph.entry()).terminator();
        assert!(super::CompiledTerminator::inspect(terminator).is_none());
        assert!(CompiledShape::inspect(body).is_none());
    }

    #[test]
    fn guarded_list_projections_keep_index_and_tail_in_compiled_checkpoints() {
        use super::super::int_list::IntListInstruction;
        use super::{CompiledInstruction, KernelKind};
        let plan = source_plan(
            r#"
fn select(values: List(Int), flag: Bool) {
  case values {
    [first, ..rest] if flag -> case rest { [] -> first _ -> 0 }
    _ -> 0
  }
}
pub fn main() { select([7], True) }
"#,
        );
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        assert_eq!(shape.kind, KernelKind::IntList);
        assert_eq!(
            shape
                .blocks
                .values()
                .flat_map(|block| &block.instructions)
                .filter(|instruction| matches!(
                    instruction,
                    CompiledInstruction::IntList(IntListInstruction::Index { .. })
                ))
                .count(),
            1
        );
        assert_eq!(
            shape
                .blocks
                .values()
                .flat_map(|block| &block.instructions)
                .filter(|instruction| matches!(
                    instruction,
                    CompiledInstruction::IntList(IntListInstruction::Tail { .. })
                ))
                .count(),
            1
        );
        assert!(shape.checkpoints.iter().any(|point| point.int_lists == 2));
    }

    #[test]
    fn boolean_calls_and_other_value_families_are_rejected_by_the_instruction_owner() {
        use super::CompiledInstruction;
        use crate::plan::execution::prepared::rust::Rust;
        let plan = source_plan(
            r#"
fn identity(value: Bool) { value }
fn select(flag: Bool) { let returned = identity(flag) case returned { True -> 1 False -> 0 } }
pub fn main() { select(True) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        let instructions = graph.block(graph.entry()).instructions();
        assert_eq!(instructions.len(), 1);
        assert_eq!(
            Rust::expression(instructions[0].value().unwrap().kind()),
            r#"
data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
    function: data::function::BoolFunctionId(0),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
    ]),
    site: data::source::HostCallSite::from_static("example", "select", data::source::SourceSpan::new(75, 89)),
})
"#
            .trim_matches('\n')
        );
        assert!(CompiledInstruction::inspect(&instructions[0]).is_none());
        let plan = source_plan(
            r#"fn select(flag: Bool) { let text = "kept" case flag { True -> 1 False -> 0 } } pub fn main() { select(True) }"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        let instructions = graph.block(graph.entry()).instructions();
        assert_eq!(instructions.len(), 1);
        assert_eq!(
            Rust::expression(instructions[0].value().unwrap().kind()),
            r#"
data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("kept")))
"#
            .trim_matches('\n')
        );
        assert!(CompiledInstruction::inspect(&instructions[0]).is_none());
    }

    #[test]
    fn non_integer_list_matches_stay_outside_the_compiled_terminator_protocol() {
        let plan = source_plan(
            r#"
fn select(values: List(String)) {
  let assert [first, ..] = values
  case first { "kept" -> 1 _ -> 0 }
}
pub fn main() { select(["kept"]) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph().as_view();
        assert!(
            super::CompiledTerminator::inspect(graph.block(graph.entry()).terminator()).is_none()
        );
        assert!(CompiledShape::inspect(body).is_none());
    }

    #[test]
    fn failures_resume_canonical_terminators_and_explicit_messages_stay_interpreted() {
        use super::CompiledTerminator;
        use std::mem::discriminant;
        for (source, has_message) in [
            (
                r#"fn select(values: List(Int)) { let assert [first, ..] = values as "empty" first } pub fn main() { select([7]) }"#,
                true,
            ),
            (
                r#"fn select(flag: Bool) { case flag { True -> 1 False -> panic as "stopped" } } pub fn main() { select(True) }"#,
                true,
            ),
            (
                r#"fn select(values: List(Int)) { let assert [first, ..] = values first } pub fn main() { select([7]) }"#,
                false,
            ),
            (
                r#"fn select(flag: Bool) { case flag { True -> 1 False -> panic } } pub fn main() { select(True) }"#,
                false,
            ),
        ] {
            let plan = source_plan(source);
            let body = plan.int_function(IntFunctionId(1)).body();
            let terminator = body
                .block_graph()
                .blocks()
                .map(|block| block.terminator())
                .find(|terminator| {
                    matches!(
                        terminator,
                        Terminator::LetAssertPanic(_) | Terminator::SourceStop(_)
                    )
                })
                .unwrap();
            assert_eq!(
                CompiledTerminator::inspect(terminator)
                    .as_ref()
                    .map(discriminant),
                (!has_message).then(|| discriminant(&CompiledTerminator::Interpreted)),
            );
            assert_eq!(CompiledShape::inspect(body).is_none(), has_message);
        }
    }

    #[test]
    fn custom_scalar_match_keeps_exact_success_failure_and_completed_prefixes() {
        let plan = source_plan(
            r#"
type Entry { Credit(Int) Debit(Int) Ignored }
fn adjust(total: Int, entry: Entry) {
  case entry {
    Credit(amount) -> total + amount
    Debit(amount) -> total - amount
    Ignored -> total
  }
}
pub fn main() { adjust(7, Credit(3)) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        assert!(CompiledShape::inspect(body).is_none());
        assert!(CompiledShape::inspect_bits(body).is_none());
        let shape =
            CompiledShape::inspect_callback(body, &plan.program.common.custom_types).unwrap();
        assert_eq!(shape.kind, super::KernelKind::Callback);
        assert!(!shape.repeats);
        assert_eq!(
            shape.checkpoints,
            vec![
                CompiledCheckpoint {
                    block: BlockId(0),
                    instruction: 0,
                    ints: 1,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 1,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(1),
                    instruction: 0,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(1),
                    instruction: 1,
                    ints: 3,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(2),
                    instruction: 0,
                    ints: 1,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 1,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(3),
                    instruction: 0,
                    ints: 2,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(3),
                    instruction: 1,
                    ints: 3,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
                CompiledCheckpoint {
                    block: BlockId(4),
                    instruction: 0,
                    ints: 1,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: 0,
                    bool_functions: 0,
                },
            ]
        );
        assert_eq!(
            crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(10.into())
        );
    }

    #[test]
    fn unsupported_custom_fields_and_repeated_calls_keep_canonical_execution() {
        for (source, expected) in [
            (
                r#"
type Item { Item(String) }
fn read(item: Item) {
  case item { Item("text") -> 1 _ -> 0 }
}
pub fn main() { read(Item("text")) }
"#,
                1,
            ),
            (
                r#"
type Inner { Inner(Int) }
type Item { Item(Inner) }
fn read(item: Item) {
  case item { Item(Inner(value)) -> value }
}
pub fn main() { read(Item(Inner(3))) }
"#,
                3,
            ),
            (
                r#"
type Item { Item(Int) }
fn read(item: Item) {
  case item { Item(9223372036854775808) -> 1 _ -> 0 }
}
pub fn main() { read(Item(3)) }
"#,
                0,
            ),
            (
                r#"
type Item { Item(Int) }
fn other(value: Int) { value + 1 }
fn read(item: Item, count: Int) {
  case count > 0 {
    True -> {
      let Item(value) = item
      read(item, other(count) - value)
    }
    False -> 0
  }
}
pub fn main() { read(Item(3), 1) }
"#,
                0,
            ),
        ] {
            let plan = source_plan(source);
            assert!(
                CompiledShape::inspect_callback(
                    plan.int_function(IntFunctionId(1)).body(),
                    &plan.program.common.custom_types,
                )
                .is_none(),
                "{source}"
            );
            assert_eq!(
                crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
                crate::Value::Int(expected.into()),
                "{source}",
            );
        }
    }
    #[test]
    fn callback_shape_rejects_repetition_and_successful_unsupported_work() {
        for (source, expected) in [
            (
                r#"
fn walk(count: Int, total: Int) {
  case count > 0 { True -> walk(count - 1, total + count) False -> total }
}
pub fn main() { walk(3, 0) }
"#,
                6,
            ),
            (
                r#"
type Item { Item(Int) }
fn read(item: Item) { echo item let Item(value) = item value }
pub fn main() { read(Item(7)) }
"#,
                7,
            ),
        ] {
            let plan = source_plan(source);
            assert_eq!(
                crate::run_main(&plan, &mut Vec::new()).unwrap(),
                crate::Value::Int(expected.into())
            );
            assert!(
                CompiledShape::inspect_callback(
                    plan.int_function(IntFunctionId(1)).body(),
                    &plan.program.common.custom_types
                )
                .is_none()
            );
        }
    }

    #[test]
    fn custom_list_exact_length_remains_a_typed_condition_in_a_connected_repeat() {
        use super::{CompiledTerminator, CompiledTest};
        let source = r#"
type Item { Item(Int) }
fn fold(items: List(Item), total: Int, step: fn(Int, Item) -> Int) {
  case items {
    [] -> total
    [first, second] -> step(step(total, first), second)
    [head, ..tail] -> fold(tail, step(total, head), step)
  }
}
fn add(total: Int, item: Item) { let Item(value) = item total + value }
pub fn main() { fold([Item(2), Item(3), Item(4)], 0, add) }
"#;
        let plan = source_plan(source);
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(9.into())
        );
        let shapes = plan
            .program
            .functions
            .value_returns
            .int_functions
            .iter()
            .filter_map(|function| {
                CompiledShape::inspect_custom_loop(
                    function.body(),
                    &plan.program.common.custom_types,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(shapes.len(), 1);
        let shape = &shapes[0];
        assert!(shape.blocks.values().any(|block| matches!(
            &block.terminator,
            CompiledTerminator::Test {
                test: CompiledTest::CustomListLength {
                    length: 2,
                    at_least: false,
                    ..
                },
                ..
            }
        )));
    }
    #[test]
    fn callback_error_suffixes_allow_literals_but_keep_message_computation_canonical() {
        let source = r#"
type Item { Item(Int) }
fn stop(item: Item) -> Int {
  let Item(value) = item
  case value > 0 {
    True -> panic as { "callback " <> "stop" }
    False -> value
  }
}
pub fn main() {
  let assert 0 = stop(Item(0))
  stop(Item(7))
}
"#;
        let plan = source_plan(source);
        assert!(
            CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types
            )
            .is_none()
        );
        let error = crate::run_main(&plan, &mut Vec::new()).unwrap_err();
        assert!(
            matches!(error, crate::ExecutionError::Panic(panic) if panic.message() == &crate::PanicMessage::Explicit("callback stop".into()) && panic.site().function() == "stop")
        );
    }

    #[test]
    fn callback_error_suffixes_with_observation_keep_the_canonical_order() {
        let source = r#"
type Item { Item(Int) }
fn stop(item: Item) -> Int {
  let Item(value) = item
  case value > 0 {
    True -> {
      echo value
      panic as "callback stop"
    }
    False -> value
  }
}
pub fn main() {
  let assert 0 = stop(Item(0))
  stop(Item(7))
}
"#;
        let plan = source_plan(source);
        assert!(
            CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types
            )
            .is_none()
        );
        let mut echo = Vec::new();
        let error = crate::run_main(&plan, &mut echo).unwrap_err();
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value(), &crate::Value::Int(7.into()));
        assert!(
            matches!(error, crate::ExecutionError::Panic(panic) if panic.message() == &crate::PanicMessage::Explicit("callback stop".into()) && panic.site().function() == "stop")
        );
    }

    #[test]
    fn a_callback_with_an_unsupported_float_prefix_keeps_its_source_error() {
        let source = r#"
type Item { Item(Int) }
fn stop(item: Item) -> Int {
  let Item(value) = item
  case value > 0 {
    True -> {
      let ignored = 1.0
      panic as "unsupported prefix"
    }
    False -> value
  }
}
pub fn main() {
  let assert 0 = stop(Item(0))
  stop(Item(7))
}
"#;
        let plan = source_plan(source);
        assert!(
            CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types
            )
            .is_none()
        );
        let error = crate::run_main(&plan, &mut Vec::new()).unwrap_err();
        assert!(
            matches!(error, crate::ExecutionError::Panic(panic) if panic.message() == &crate::PanicMessage::Explicit("unsupported prefix".into()) && panic.site().function() == "stop")
        );
    }

    #[test]
    fn custom_list_boolean_nodes_preserve_the_exact_length_relation() {
        use crate::plan::execution::graph::{BoolTest, CustomListLocalId, ListLocal};
        use crate::plan::execution::type_::{CustomListTypeId, CustomTypeId, ListTypeId};
        let list = ListLocal::Custom {
            type_id: CustomListTypeId {
                list_type: ListTypeId(0),
                item_type: CustomTypeId(0),
            },
            local: CustomListLocalId(3),
        };
        for (node, expected) in [
            (
                BoolTest::ListLengthEquals {
                    value: list.clone(),
                    length: 2,
                },
                "b2_l3.len() == 2",
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: list,
                    length: 2,
                },
                "b2_l3.len() >= 2",
            ),
        ] {
            let view = super::CompiledTest::inspect(&node).unwrap();
            assert_eq!(
                super::super::test_expression(crate::plan::execution::graph::BlockId(2), &view),
                expected
            );
        }
    }

    #[test]
    fn scalar_field_reads_advance_the_integer_callback_prefix_once() {
        let source = r#"
type Item { Item(value: Int) }
fn read(item: Item) { item.value + 1 }
pub fn main() { read(Item(7)) }
"#;
        let plan = source_plan(source);
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(8.into())
        );
        let shape = CompiledShape::inspect_callback(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        let entry = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            customs: 1,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        assert_eq!(
            shape.checkpoints,
            vec![
                entry,
                CompiledCheckpoint {
                    instruction: 1,
                    ints: 1,
                    ..entry
                },
                CompiledCheckpoint {
                    instruction: 2,
                    ints: 2,
                    ..entry
                },
            ]
        );
    }

    #[test]
    fn boolean_field_reads_advance_the_integer_callback_prefix_once() {
        let source = r#"
type Item { Item(enabled: Bool) }
fn read(item: Item) { case item.enabled { True -> 7 False -> -9 } }
pub fn main() {
  let assert -9 = read(Item(False))
  read(Item(True))
}
"#;
        let plan = source_plan(source);
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(7.into())
        );
        let shape = CompiledShape::inspect_callback(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        let entry = CompiledCheckpoint {
            block: BlockId(0),
            instruction: 0,
            ints: 0,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            customs: 1,
            custom_lists: 0,
            int_functions: 0,
            bool_functions: 0,
        };
        assert_eq!(
            &shape.checkpoints[..2],
            &[
                entry,
                CompiledCheckpoint {
                    instruction: 1,
                    bools: 1,
                    ..entry
                },
            ]
        );
    }

    #[test]
    fn a_literal_callback_error_suffix_keeps_the_supported_success_branch() {
        let source = r#"
type Item { Item(Int) }
fn stop(item: Item) -> Int {
  let Item(value) = item
  case value > 0 {
    True -> panic as "callback stop"
    False -> value
  }
}
pub fn main() {
  let assert 0 = stop(Item(0))
  stop(Item(7))
}
"#;
        let plan = source_plan(source);
        let shape = CompiledShape::inspect_callback(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        assert_eq!(shape.kind, super::KernelKind::Callback);
        assert!(!shape.repeats);
        let error = crate::run_main(&plan, &mut Vec::new()).unwrap_err();
        assert!(
            matches!(error, crate::ExecutionError::Panic(panic) if panic.message() == &crate::PanicMessage::Explicit("callback stop".into()) && panic.site().function() == "stop")
        );
    }

    #[test]
    fn integer_accumulators_can_repeat_through_a_boolean_callback() {
        use super::super::CompiledLoopFunction;
        use super::super::custom_loop::LoopCallOutput;
        use crate::plan::execution::graph::BoolFunctionLocalId;
        use crate::plan::execution::type_::{CustomTypeId, FunctionType, ValueType};
        let source = r#"
type Item { Item(Int) }
fn fold(items: List(Item), total: Int, check: fn(Int, Item) -> Bool) {
  case items {
    [] -> total
    [head, ..tail] -> {
      let increment = case check(total, head) { True -> 1 False -> 0 }
      fold(tail, total + increment, check)
    }
  }
}
fn positive(total: Int, item: Item) {
  let Item(value) = item
  total + value > 0
}
pub fn main() { fold([Item(2), Item(-1), Item(4)], 0, positive) }
"#;
        let plan = source_plan(source);
        let shape = CompiledShape::inspect_custom_loop(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        assert_eq!(shape.loop_calls().len(), 1);
        assert_eq!(
            shape.loop_calls()[0].function,
            CompiledLoopFunction::Bool {
                local: BoolFunctionLocalId(0),
                type_: FunctionType::new(
                    vec![ValueType::Int, ValueType::Custom(CustomTypeId(0))],
                    ValueType::Bool,
                ),
            }
        );
        assert!(
            shape
                .blocks
                .values()
                .any(
                    |block| block.instructions.iter().any(|instruction| matches!(
                        instruction,
                        super::CompiledInstruction::CustomLoop(
                            super::CustomLoopInstruction::Call {
                                output: LoopCallOutput::Bool(_),
                                ..
                            }
                        )
                    ))
                )
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(2.into())
        );
    }
}
