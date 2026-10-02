use crate::plan::execution::compiled_numeric::NumericCheckpoint;
use crate::plan::execution::function::{ExecutionGraphProfile, FunctionExit, ProfiledFunctionBody};
use crate::plan::execution::graph::{
    ArithmeticRegion, BlockGraphExitId, BlockGraphView, BlockId, BoolInstruction, BoolLocalId,
    BoolTest, Edge, IntInstruction, IntLocalId, IntegerLiteral, IntegerOperand, ParamLocal,
    ParamSlot, ProfiledInstruction, ProfiledInstructionKind, Terminator,
};
use std::collections::BTreeSet;

/// Borrowed inspection used only while emitting or admitting prepared data.
/// None of the structuring work survives in the executable program.
pub(in crate::plan::execution::prepared) struct NumericShape<'graph, Graph: ExecutionGraphProfile> {
    pub graph: BlockGraphView<'graph, Graph>,
    pub blocks: Vec<NumericBlock<'graph>>,
    pub checkpoints: Vec<NumericCheckpoint>,
    pub starts: Vec<usize>,
    pub joins: Vec<Option<BlockId>>,
    pub repeats: bool,
}

// These are preparation-local views. They borrow the canonical graph and
// narrow only the variants that this emitter implements; they are never
// stored in an execution program or used as a second runtime evaluator.
pub(in crate::plan::execution::prepared) struct NumericBlock<'graph> {
    pub instructions: Vec<NumericInstruction<'graph>>,
    pub terminator: NumericTerminator<'graph>,
}

pub(in crate::plan::execution::prepared) enum NumericInstruction<'graph> {
    Integer(IntLocalId, NumericInteger<'graph>),
    Boolean(BoolLocalId, NumericBoolean),
    Region {
        region: &'graph ArithmeticRegion,
        outputs: Vec<IntLocalId>,
    },
}

pub(in crate::plan::execution::prepared) enum NumericInteger<'graph> {
    Value(&'graph IntegerLiteral),
    Binary(NumericOperation, IntegerOperand, IntegerOperand),
    Negate(IntLocalId),
}

#[derive(Clone, Copy)]
pub(in crate::plan::execution::prepared) enum NumericOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

pub(in crate::plan::execution::prepared) enum NumericBoolean {
    Value(bool),
    Test(NumericTest),
}

pub(in crate::plan::execution::prepared) enum NumericTest {
    Not(BoolLocalId),
    Compare(NumericComparison, IntegerOperand, IntegerOperand),
}

pub(in crate::plan::execution::prepared) enum NumericComparison {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub(in crate::plan::execution::prepared) enum NumericTerminator<'graph> {
    Jump(&'graph Edge),
    Boolean {
        subject: BoolLocalId,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Test {
        test: NumericTest,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Switch {
        subject: IntLocalId,
        clauses: &'graph [(IntegerLiteral, Edge)],
        fallback: &'graph Edge,
    },
    Exit(BlockGraphExitId),
}

impl<'graph, Graph: ExecutionGraphProfile> NumericShape<'graph, Graph> {
    pub(in crate::plan::execution::prepared) fn inspect<Return, Tail>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
    ) -> Option<Self> {
        let graph = body.block_graph().as_view();
        let count = graph.blocks().len();
        if count < 2
            || body
                .exits
                .iter()
                .any(|exit| !matches!(exit, FunctionExit::Return(_)))
        {
            return None;
        }
        let mut checkpoints = Vec::new();
        let mut blocks = Vec::with_capacity(count);
        let mut starts = Vec::with_capacity(count);
        let mut successors = Vec::with_capacity(count);
        let mut repeats = false;
        for (index, block) in graph.blocks().enumerate() {
            let mut point = NumericCheckpoint {
                block: BlockId(index),
                instruction: 0,
                ints: 0,
                bools: 0,
            };
            for slot in block.params() {
                add_slot(&mut point, slot)?;
            }
            starts.push(checkpoints.len());
            checkpoints.push(point);
            let mut instructions = Vec::with_capacity(block.instructions().len());
            for instruction in block.instructions() {
                let instruction = NumericInstruction::inspect(instruction)?;
                match &instruction {
                    NumericInstruction::Integer(..) => point.ints += 1,
                    NumericInstruction::Boolean(..) => point.bools += 1,
                    NumericInstruction::Region { outputs, .. } => point.ints += outputs.len(),
                }
                instructions.push(instruction);
                point.instruction += 1;
                checkpoints.push(point);
            }
            let terminator = NumericTerminator::inspect(block.terminator())?;
            let edges = terminator.edges();
            repeats |= edges.iter().any(|edge| edge.target() == graph.entry());
            successors.push(
                edges
                    .iter()
                    .map(|edge| edge.target().index())
                    .collect::<Vec<_>>(),
            );
            blocks.push(NumericBlock {
                instructions,
                terminator,
            });
        }
        let mut order = Vec::with_capacity(count);
        let mut visited = vec![0; count];
        visit(
            graph.entry().index(),
            graph.entry().index(),
            &successors,
            &mut visited,
            &mut order,
        )?;
        // A frozen graph contains only reachable blocks. Requiring all of them
        // also makes the checkpoint layout complete rather than a partial view.
        if order.len() != count {
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
            }
        }
        let shape = Self {
            graph,
            blocks,
            checkpoints,
            starts,
            joins,
            repeats,
        };
        let mut emitted = BTreeSet::new();
        shape.structured_path(graph.entry(), None, &mut emitted)?;
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
        let outgoing = self.blocks[block.index()].terminator.edges();
        let join = self.joins[block.index()];
        for edge in outgoing {
            if edge.target() != self.graph.entry() {
                self.structured_path(edge.target(), join.or(stop), emitted)?;
            }
        }
        if let Some(join) = join {
            self.structured_path(join, stop, emitted)?;
        }
        Some(())
    }
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

fn add_slot(point: &mut NumericCheckpoint, slot: &ParamSlot) -> Option<()> {
    match slot.local() {
        ParamLocal::Int(_) => point.ints += 1,
        ParamLocal::Bool(_) => point.bools += 1,
        _ => return None,
    }
    Some(())
}

impl<'graph> NumericInstruction<'graph> {
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
                (ProfiledInstructionKind::Int(instruction), ParamLocal::Int(output)) => Some(
                    Self::Integer(*output, NumericInteger::inspect(instruction)?),
                ),
                (ProfiledInstructionKind::Bool(instruction), ParamLocal::Bool(output)) => {
                    let expression = match instruction {
                        BoolInstruction::Value(value) => NumericBoolean::Value(*value),
                        BoolInstruction::Test(test) => {
                            NumericBoolean::Test(NumericTest::inspect(test)?)
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

impl NumericTest {
    fn inspect(test: &BoolTest) -> Option<Self> {
        let (comparison, left, right) = match test {
            BoolTest::Not(value) => return Some(Self::Not(*value)),
            BoolTest::EqualInt { left, right } => (NumericComparison::Equal, left, right),
            BoolTest::NotEqualInt { left, right } => (NumericComparison::NotEqual, left, right),
            BoolTest::LtInt { left, right } => (NumericComparison::Less, left, right),
            BoolTest::LtEqInt { left, right } => (NumericComparison::LessEqual, left, right),
            BoolTest::GtInt { left, right } => (NumericComparison::Greater, left, right),
            BoolTest::GtEqInt { left, right } => (NumericComparison::GreaterEqual, left, right),
            _ => return None,
        };
        Some(Self::Compare(comparison, *left, *right))
    }
}

impl<'graph> NumericTerminator<'graph> {
    fn inspect(terminator: &'graph Terminator) -> Option<Self> {
        Some(match terminator {
            Terminator::Jump(jump) => Self::Jump(&jump.edge),
            Terminator::BoolBranch(branch) => Self::Boolean {
                subject: branch.subject,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::TestBranch(branch) => Self::Test {
                test: NumericTest::inspect(&branch.test)?,
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
            _ => return None,
        })
    }

    fn edges(&self) -> Vec<&'graph Edge> {
        match self {
            Self::Jump(edge) => vec![edge],
            Self::Boolean { true_, false_, .. } | Self::Test { true_, false_, .. } => {
                vec![true_, false_]
            }
            Self::Switch {
                clauses, fallback, ..
            } => clauses
                .iter()
                .map(|(_, edge)| edge)
                .chain(std::iter::once(*fallback))
                .collect(),
            Self::Exit(_) => vec![],
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
    use super::{NumericCheckpoint, NumericShape, visit};
    use crate::plan::execution::function::{ExecutionIntFunctionBody, FunctionExit, IntFunctionId};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, Edge, IntLocalId, IntSwitch, IntegerLiteral, Jump, ParamLocal,
        ProfiledBlock, ProfiledBlockGraph, StringLocalId, Terminator,
    };
    use std::collections::BTreeSet;
    use std::convert::Infallible;

    fn source_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
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
        let shape = NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        assert!(shape.repeats);
        assert_eq!(shape.starts, [0, 1, 2]);
        assert_eq!(shape.joins, [None, None, None]);
        assert_eq!(
            shape.checkpoints,
            [
                NumericCheckpoint {
                    block: BlockId(0),
                    instruction: 0,
                    ints: 2,
                    bools: 0
                },
                NumericCheckpoint {
                    block: BlockId(1),
                    instruction: 0,
                    ints: 1,
                    bools: 0
                },
                NumericCheckpoint {
                    block: BlockId(2),
                    instruction: 0,
                    ints: 2,
                    bools: 0
                },
                NumericCheckpoint {
                    block: BlockId(2),
                    instruction: 1,
                    ints: 4,
                    bools: 0
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
        let shape = NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        assert!(!shape.repeats);
        let join = shape.joins[0].unwrap();
        assert_ne!(join, shape.graph.entry());
        let point = shape.checkpoints[shape.starts[join.index()]];
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
  case flag {
    True -> other(value)
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
                NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).is_none(),
                "{source}"
            );
        }
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
                            let edge = super::NumericTerminator::inspect(&terminator)
                                .unwrap()
                                .edges()[0]
                                .clone();
                            terminator = Terminator::Jump(Jump::new(edge));
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
                let mut edge = super::NumericTerminator::inspect(&terminator)
                    .unwrap()
                    .edges()[0]
                    .clone();
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
            assert!(NumericShape::inspect(&body).is_none(), "margin {change}");
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
        let transfer = super::NumericTerminator::inspect(graph.block(graph.entry()).terminator())
            .unwrap()
            .edges()[0]
            .transfer
            .clone();
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
        assert!(NumericShape::inspect(&body).is_none());

        let shape = NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
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
        let shape = NumericShape::inspect(joined.int_function(IntFunctionId(1)).body()).unwrap();
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
        let shape = NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
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
        assert!(NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).is_none());
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
        assert!(super::NumericTerminator::inspect(terminator).is_none());
        assert!(NumericShape::inspect(body).is_none());
    }

    #[test]
    fn integer_tests_preserve_the_exact_comparison_and_canonical_operands() {
        use super::{NumericComparison, NumericTest};
        use crate::plan::execution::graph::{BoolLocalId, BoolTest, IntegerOperand};
        use crate::plan::execution::prepared::rust::Rust;
        use std::mem::discriminant;

        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Immediate(-7);
        let inspected_comparison = |test: &BoolTest| match NumericTest::inspect(test) {
            Some(NumericTest::Compare(comparison, left, right)) => Some((
                discriminant(&comparison),
                Rust::expression(&left),
                Rust::expression(&right),
            )),
            _ => None,
        };
        let inspected_not = |test: &BoolTest| match NumericTest::inspect(test) {
            Some(NumericTest::Not(value)) => Some(value.0),
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
        assert!(super::NumericTerminator::inspect(terminator).is_none());
        assert!(NumericShape::inspect(body).is_none());
    }
}
