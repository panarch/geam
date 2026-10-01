use super::super::draft::instruction::{DraftIntInstruction, DraftIntegerOperand};
use super::super::draft::{
    DraftGraphBuilder, DraftGraphValue, DraftInstruction, DraftInt, DraftValueKey,
};
use crate::plan::execution::graph::{ArithmeticNode, ArithmeticOperand, MAX_ARITHMETIC_NODES};
use std::collections::HashMap;

pub(in crate::plan::execution::lowering) struct DraftArithmeticRegion {
    pub inputs: Vec<DraftInt>,
    pub nodes: Vec<ArithmeticNode>,
    pub outputs: Vec<(usize, DraftInt)>,
}

pub(super) fn form_regions<Return: DraftGraphValue, TailCall>(
    graph: &mut DraftGraphBuilder<Return, TailCall>,
) {
    let mut uses = HashMap::<DraftValueKey, usize>::new();
    let mut scratch = Vec::new();
    for block in graph.graph.blocks.values() {
        for instruction in &block.instructions {
            instruction.uses(&mut scratch);
        }
        block.terminator.uses(&mut scratch);
        for key in scratch.drain(..) {
            *uses.entry(key).or_default() += 1;
        }
    }
    for value in &graph.returns {
        *uses.entry(value.key()).or_default() += 1;
    }
    for block in graph.graph.blocks.values_mut() {
        let mut run = Vec::new();
        for instruction in std::mem::take(&mut block.instructions) {
            match instruction {
                DraftInstruction::Int { output, kind } => {
                    if let Some(operation) = ArithmeticOperation::from_instruction(&kind) {
                        run.push(ArithmeticInstruction { output, operation });
                        if run.len() == MAX_ARITHMETIC_NODES {
                            flush(&mut run, &mut block.instructions, &uses);
                        }
                    } else {
                        flush(&mut run, &mut block.instructions, &uses);
                        block
                            .instructions
                            .push(DraftInstruction::Int { output, kind });
                    }
                }
                instruction => {
                    flush(&mut run, &mut block.instructions, &uses);
                    block.instructions.push(instruction);
                }
            }
        }
        flush(&mut run, &mut block.instructions, &uses);
    }
}

struct ArithmeticInstruction {
    output: DraftInt,
    operation: ArithmeticOperation,
}

enum ArithmeticOperation {
    Add(DraftIntegerOperand, DraftIntegerOperand),
    Subtract(DraftIntegerOperand, DraftIntegerOperand),
    Multiply(DraftIntegerOperand, DraftIntegerOperand),
    Divide(DraftIntegerOperand, DraftIntegerOperand),
    Remainder(DraftIntegerOperand, DraftIntegerOperand),
    Negate(DraftInt),
}

impl ArithmeticOperation {
    fn from_instruction(instruction: &DraftIntInstruction) -> Option<Self> {
        Some(match instruction {
            DraftIntInstruction::Add { left, right } => Self::Add(left.clone(), right.clone()),
            DraftIntInstruction::Sub { left, right } => Self::Subtract(left.clone(), right.clone()),
            DraftIntInstruction::Mult { left, right } => {
                Self::Multiply(left.clone(), right.clone())
            }
            DraftIntInstruction::Div { left, right } => Self::Divide(left.clone(), right.clone()),
            DraftIntInstruction::Remainder { left, right } => {
                Self::Remainder(left.clone(), right.clone())
            }
            DraftIntInstruction::Negate(value) => Self::Negate(value.clone()),
            _ => return None,
        })
    }

    fn into_instruction(self) -> DraftIntInstruction {
        match self {
            Self::Add(left, right) => DraftIntInstruction::Add { left, right },
            Self::Subtract(left, right) => DraftIntInstruction::Sub { left, right },
            Self::Multiply(left, right) => DraftIntInstruction::Mult { left, right },
            Self::Divide(left, right) => DraftIntInstruction::Div { left, right },
            Self::Remainder(left, right) => DraftIntInstruction::Remainder { left, right },
            Self::Negate(value) => DraftIntInstruction::Negate(value),
        }
    }
}

fn flush(
    run: &mut Vec<ArithmeticInstruction>,
    instructions: &mut Vec<DraftInstruction>,
    uses: &HashMap<DraftValueKey, usize>,
) {
    if run.len() < 2 {
        instructions.extend(run.drain(..).map(|instruction| DraftInstruction::Int {
            output: instruction.output,
            kind: instruction.operation.into_instruction(),
        }));
    } else {
        instructions.push(DraftInstruction::IntegerRegion(region(run, uses)));
        run.clear();
    }
}

fn region(
    run: &[ArithmeticInstruction],
    uses: &HashMap<DraftValueKey, usize>,
) -> DraftArithmeticRegion {
    let mut region = DraftArithmeticRegion {
        inputs: Vec::new(),
        nodes: Vec::new(),
        outputs: Vec::new(),
    };
    let mut values = HashMap::new();
    let mut inputs = HashMap::new();
    let mut internal_uses = HashMap::<DraftValueKey, usize>::new();
    let key = |operand: &DraftIntegerOperand| match operand {
        DraftIntegerOperand::Local(value) => Some(value.key),
        DraftIntegerOperand::Immediate(_) => None,
    };
    for instruction in run {
        let operands = match &instruction.operation {
            ArithmeticOperation::Add(left, right)
            | ArithmeticOperation::Subtract(left, right)
            | ArithmeticOperation::Multiply(left, right)
            | ArithmeticOperation::Divide(left, right)
            | ArithmeticOperation::Remainder(left, right) => [key(left), key(right)],
            ArithmeticOperation::Negate(value) => [Some(value.key), None],
        };
        for key in operands.into_iter().flatten() {
            *internal_uses.entry(key).or_default() += 1;
        }
    }
    for ArithmeticInstruction { output, operation } in run {
        let mut local = |local: &DraftInt| {
            if let Some(index) = values.get(&local.key) {
                ArithmeticOperand::Value(*index)
            } else {
                let index = *inputs.entry(local.key).or_insert_with(|| {
                    let index = region.inputs.len();
                    region.inputs.push(local.clone());
                    index
                });
                ArithmeticOperand::Input(index)
            }
        };
        let mut operand = |value: &DraftIntegerOperand| match value {
            DraftIntegerOperand::Immediate(value) => ArithmeticOperand::Immediate(*value),
            DraftIntegerOperand::Local(value) => local(value),
        };
        let node = match operation {
            ArithmeticOperation::Add(left, right) => {
                ArithmeticNode::Add(operand(left), operand(right))
            }
            ArithmeticOperation::Subtract(left, right) => {
                ArithmeticNode::Subtract(operand(left), operand(right))
            }
            ArithmeticOperation::Multiply(left, right) => {
                ArithmeticNode::Multiply(operand(left), operand(right))
            }
            ArithmeticOperation::Divide(left, right) => {
                ArithmeticNode::Divide(operand(left), operand(right))
            }
            ArithmeticOperation::Remainder(left, right) => {
                ArithmeticNode::Remainder(operand(left), operand(right))
            }
            ArithmeticOperation::Negate(value) => ArithmeticNode::Negate(local(value)),
        };
        let index = region.nodes.len();
        region.nodes.push(node);
        values.insert(output.key, index);
        if uses.get(&output.key).copied().unwrap_or(0)
            > internal_uses.get(&output.key).copied().unwrap_or(0)
        {
            region.outputs.push((index, output.clone()));
        }
    }
    region
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::{ExecutionGraphProfile, IntFunctionId};
    use crate::plan::execution::graph::{
        ArithmeticNode as N, ArithmeticOperand as O, ArithmeticRegion, IntLocalId, ParamLocal,
        ProfiledInstruction,
    };
    use crate::runtime::{Value, run_main};

    fn arithmetic_region<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
    ) -> Option<&ArithmeticRegion> {
        match instruction {
            ProfiledInstruction::IntegerRegion(region) => Some(region),
            ProfiledInstruction::Value(_) => None,
        }
    }

    fn plan_src(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap())
    }

    #[test]
    fn source_regions_remove_internal_slots_and_preserve_shared_external_outputs() {
        let plan = plan_src(
            r#"
fn calculate(n: Int, total: Int) {
  let next = n - 1
  let product = n * 3
  let value = total + product
  let value = value + product
  case next { 0 -> value _ -> value + next }
}
pub fn main() { calculate(4, 7) }
"#,
        );
        let function = plan.int_function(IntFunctionId(1));
        let graph = function.body().block_graph();
        let block = graph.block(graph.entry());
        assert_eq!(block.params().len(), 2);
        assert_eq!(block.instructions().len(), 1);
        let regions = block
            .instructions()
            .iter()
            .filter_map(arithmetic_region)
            .collect::<Vec<_>>();
        assert_eq!(regions.len(), 1);
        let region = regions[0];
        assert_eq!(region.inputs.as_ref(), &[IntLocalId(0), IntLocalId(1)]);
        assert_eq!(
            region.nodes.as_ref(),
            &[
                N::Subtract(O::Input(0), O::Immediate(1)),
                N::Multiply(O::Input(0), O::Immediate(3)),
                N::Add(O::Input(1), O::Value(1)),
                N::Add(O::Value(2), O::Value(1)),
            ]
        );
        assert!(region.native);
        assert_eq!(
            region
                .outputs
                .iter()
                .map(|value| (value.value, value.slot.local.clone()))
                .collect::<Vec<_>>(),
            vec![
                (0, ParamLocal::Int(IntLocalId(2))),
                (3, ParamLocal::Int(IntLocalId(3)))
            ]
        );
        // Four operations publish two values, not four global Int cells. The
        // shared product has one node and no enclosing storage slot.
        assert_eq!(
            run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(34.into())
        );
    }

    #[test]
    fn region_output_feeds_a_direct_branch_without_a_bool_slot() {
        for (arguments, expected) in [
            ("4, 7", "19"),
            ("4, 8", "19"),
            ("9223372036854775808, -27670116110564327424", "0"),
            ("9223372036854775807, 3", "27670116110564327423"),
        ] {
            let plan = plan_src(&format!(
                "fn choose(n: Int, total: Int) {{ let product = n * 3 let next = total + product case next < 20 {{ True -> next False -> next - 1 }} }} pub fn main() {{ choose({arguments}) }}"
            ));
            let graph = plan.int_function(IntFunctionId(1)).body().block_graph();
            let entry = graph.block(graph.entry());
            assert_eq!(entry.instructions().len(), 1);
            let region = arithmetic_region(&entry.instructions()[0]).unwrap();
            assert_eq!(region.inputs.as_ref(), &[IntLocalId(0), IntLocalId(1)]);
            assert_eq!(
                region.nodes.as_ref(),
                &[
                    N::Multiply(O::Input(0), O::Immediate(3)),
                    N::Add(O::Input(1), O::Value(0)),
                ]
            );
            assert!(region.native);
            assert_eq!(
                region
                    .outputs
                    .iter()
                    .map(|output| (output.value, output.slot.local.clone()))
                    .collect::<Vec<_>>(),
                [(1, ParamLocal::Int(IntLocalId(2)))]
            );
            let explanation = plan.explain().to_string();
            let branches = explanation
                .lines()
                .map(str::trim)
                .filter(|line| line.starts_with("branch_test "))
                .collect::<Vec<_>>();
            assert_eq!(
                branches,
                ["branch_test bool.lt_int %int#2 20 true=b1(%int#2) false=b2(%int#2)"]
            );
            assert_eq!(
                run_main(&plan, &mut Vec::new()).unwrap(),
                Value::Int(expected.parse().unwrap())
            );
        }
    }

    #[test]
    fn unproved_regions_still_form_and_execute_exact_large_intermediates() {
        let plan = plan_src(
            r#"
fn calculate(n: Int) {
  let squared = n * n
  let fourth = squared * squared
  let restored = fourth / squared
  restored / n
}
pub fn main() { calculate(9223372036854775807) }
"#,
        );
        let function = plan.int_function(IntFunctionId(1));
        let graph = function.body().block_graph();
        let regions = graph
            .block(graph.entry())
            .instructions()
            .iter()
            .filter_map(arithmetic_region)
            .collect::<Vec<_>>();
        assert_eq!(regions.len(), 1);
        let region = regions[0];
        assert!(!region.native);
        assert_eq!(region.nodes.len(), 4);
        assert_eq!(region.outputs.len(), 1);
        assert_eq!(
            run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(i64::MAX.into())
        );
    }

    #[test]
    fn isolated_arithmetic_remains_scalar_between_observable_effects() {
        let plan = plan_src(
            r#"
fn calculate(n: Int, other: Int) {
  let added = n + other
  echo added
  let subtracted = n - other
  echo subtracted
  let multiplied = n * other
  echo multiplied
  let divided = n / other
  echo divided
  let remainder = n % other
  echo remainder
  let negated = -n
  echo negated
  negated
}
pub fn main() { calculate(7, 3) }
"#,
        );
        let function = plan.int_function(IntFunctionId(1));
        assert_eq!(
            function
                .body()
                .block_graph()
                .blocks()
                .flat_map(|block| block.instructions().iter())
                .filter_map(arithmetic_region)
                .count(),
            0
        );
        let mut echo = Vec::new();
        assert_eq!(run_main(&plan, &mut echo).unwrap(), Value::Int((-7).into()));
        assert_eq!(
            echo.iter()
                .map(|output| output.to_string().lines().last().unwrap().to_owned())
                .collect::<Vec<_>>(),
            ["10", "4", "21", "2", "1", "-7"]
        );
    }

    #[test]
    fn source_regions_split_at_effects_calls_controls_and_the_node_limit() {
        let operations = (0..70)
            .map(|index| format!("let n = n + {}\n", index % 3 + 1))
            .collect::<String>();
        let plan = plan_src(&format!(
            "fn identity(n: Int) {{ n }}\nfn calculate(n: Int) {{ {operations} echo n let n = n + 1 let n = n * 2 let n = identity(n) n + 3 }}\npub fn main() {{ calculate(1) }}"
        ));
        let function = plan.int_function(IntFunctionId(1));
        let sizes = function
            .body()
            .block_graph()
            .blocks()
            .flat_map(|block| block.instructions().iter())
            .filter_map(arithmetic_region)
            .map(|region| region.nodes.len())
            .collect::<Vec<_>>();
        assert_eq!(sizes, vec![32, 32, 6, 2]);
        let mut echo = Vec::new();
        assert_eq!(run_main(&plan, &mut echo).unwrap(), Value::Int(285.into()));
        assert_eq!(echo.len(), 1);
    }
}
