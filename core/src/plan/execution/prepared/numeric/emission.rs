use super::NumericShape;
use super::shape::{
    NumericBoolean, NumericComparison, NumericInstruction, NumericInteger, NumericOperation,
    NumericTerminator, NumericTest,
};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionGraphProfile, ExecutionProfile,
    FunctionTables,
};
use crate::plan::execution::graph::{
    ArithmeticNode, ArithmeticOperand, BlockGraphExitId, BlockId, Edge, IntegerOperand, ParamLocal,
};
use crate::plan::execution::numeric::NumericCheckpoint;
use crate::plan::execution::prepared::rust::{Emit, Rust};

pub(in crate::plan::execution::prepared) struct NumericEmission<'program, Profile: ExecutionProfile>
{
    functions: &'program FunctionTables<Profile>,
}

struct FunctionEmission<'graph, Graph: ExecutionGraphProfile> {
    index: usize,
    name: String,
    shape: NumericShape<'graph, Graph>,
}

#[derive(Clone, Copy)]
enum ProgressOutput {
    Direct,
    Resume,
}

impl ProgressOutput {
    fn expression(self, progress: String) -> String {
        match self {
            Self::Direct => progress,
            Self::Resume => format!("NumericResume::Exit({progress})"),
        }
    }
}

impl<'program, Profile: ExecutionProfile> NumericEmission<'program, Profile> {
    pub(in crate::plan::execution::prepared) fn new(
        functions: &'program FunctionTables<Profile>,
    ) -> Self {
        Self { functions }
    }
}

impl<Profile: ExecutionProfile> Emit for NumericEmission<'_, Profile> {
    fn emit(&self, output: &mut Rust) {
        let ints = self
            .functions
            .value_returns
            .int_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                Some(FunctionEmission {
                    index,
                    name: format!("numeric_int_{index}"),
                    shape: NumericShape::inspect(function.body())?,
                })
            })
            .collect::<Vec<_>>();
        let bools = self
            .functions
            .value_returns
            .bool_functions
            .iter()
            .enumerate()
            .filter_map(|(index, function)| {
                let ExecutionFunctionRef::Graph(function) = function.as_ref() else {
                    return None;
                };
                Some(FunctionEmission {
                    index,
                    name: format!("numeric_bool_{index}"),
                    shape: NumericShape::inspect(function.body())?,
                })
            })
            .collect::<Vec<_>>();
        if ints.is_empty() && bools.is_empty() {
            output.call("numeric::NumericFunctions::interpreted", &[]);
            return;
        }
        let mut source = String::from(
            "{\nenum NumericResume {\nNext(usize),\nExit(data::numeric::NumericProgress),\n}\n",
        );
        for function in ints.iter().chain(&bools) {
            function.write_code(&mut source);
        }
        source.push_str("data::numeric::NumericFunctions {\nints: data::Storage::Static(&[\n");
        for function in &ints {
            function.write_target(&mut source, "Int");
        }
        source.push_str("]),\nbools: data::Storage::Static(&[\n");
        for function in &bools {
            function.write_target(&mut source, "Bool");
        }
        source.push_str("]),\n}\n}");
        output.raw(&source);
    }
}

impl<Graph: ExecutionGraphProfile> FunctionEmission<'_, Graph> {
    fn write_code(&self, source: &mut String) {
        source.push_str(&format!("fn {}(point: usize, values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {{\n", self.name));
        source.push_str(&format!("const RESUME: [fn(&mut data::numeric::NumericValues, &mut usize) -> NumericResume; {}] = [\n", self.shape.checkpoints.len()));
        for (index, point) in self.shape.checkpoints.iter().enumerate() {
            if index == self.entry() {
                let inputs = checkpoint_inputs(*point);
                source.push_str(&format!(
                    "|values, budget| NumericResume::Exit({}_entry({inputs}, values, budget)),\n",
                    self.name
                ));
            } else {
                source.push_str(&format!("{},\n", self.resume_name(index)));
            }
        }
        source.push_str("];\nlet mut point = point;\nloop {\nmatch RESUME[point](values, budget) {\nNumericResume::Next(next) => point = next,\nNumericResume::Exit(progress) => return progress,\n}\n}\n}\n");
        let entry = self.shape.checkpoints[self.entry()];
        self.signature(source, &format!("{}_entry", self.name), entry);
        let pattern = self.locals(entry, self.shape.repeats);
        source.push_str(&format!("let {pattern} = inputs;\n"));
        if self.shape.repeats {
            source.push_str("'repeat: loop {\n");
        }
        self.path(source, entry.block, None);
        if self.shape.repeats {
            source.push_str("}\n");
        }
        source.push_str("}\n");
        // A resumed checkpoint returns its next point to the dispatcher before
        // another step runs. Its Rust stack depth does not grow with the suffix.
        for (index, point) in self.shape.checkpoints.iter().enumerate() {
            if index == self.entry() {
                continue;
            }
            source.push_str(&format!("fn {}(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {{\n", self.resume_name(index)));
            source.push_str(&format!(
                "let {} = {};\n",
                self.locals(*point, false),
                checkpoint_inputs(*point)
            ));
            self.tick(source, index, ProgressOutput::Resume);
            let block = &self.shape.blocks[point.block.index()];
            if let Some(instruction) = block.instructions.get(point.instruction) {
                self.instruction(source, *point, instruction);
                let next = self.shape.checkpoints[index + 1];
                self.big_exit(
                    source,
                    *point,
                    instruction,
                    index + 1,
                    ProgressOutput::Resume,
                );
                self.save(source, next);
                source.push_str(&format!("NumericResume::Next({})\n", index + 1));
            } else {
                self.resume_terminator(source, *point, &block.terminator);
            }
            source.push_str("}\n");
        }
    }

    fn write_target(&self, source: &mut String, family: &str) {
        source.push_str(&format!("data::numeric::NumericFunction {{ function: data::function::{family}FunctionId({}), implementation: data::numeric::NumericImplementation {{ entry: {}, checkpoints: data::Storage::Static(&{}), run: {} }} }},\n", self.index, self.entry(), Rust::expression(self.shape.checkpoints.as_slice()), self.name));
    }

    fn entry(&self) -> usize {
        self.shape.starts[self.shape.graph.entry().index()]
    }

    fn signature(&self, source: &mut String, name: &str, point: NumericCheckpoint) {
        let types = tuple(
            std::iter::repeat_n("i128".to_owned(), point.ints)
                .chain(std::iter::repeat_n("bool".to_owned(), point.bools)),
        );
        source.push_str(&format!("fn {name}(inputs: {types}, values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {{\n"));
    }

    fn locals(&self, point: NumericCheckpoint, mutable: bool) -> String {
        let prefix = if mutable { "mut " } else { "" };
        tuple(
            (0..point.ints)
                .map(|index| format!("{prefix}b{}_i{index}", point.block.0))
                .chain(
                    (0..point.bools).map(|index| format!("{prefix}b{}_v{index}", point.block.0)),
                ),
        )
    }

    fn resume_name(&self, point: usize) -> String {
        format!("{}_resume_{point}", self.name)
    }

    fn tick(&self, source: &mut String, index: usize, output: ProgressOutput) {
        source.push_str("if *budget == 0 {\n");
        self.save(source, self.shape.checkpoints[index]);
        let progress = output.expression(format!("data::numeric::NumericProgress::Yield({index})"));
        source.push_str(&format!("return {progress};\n}}\n*budget -= 1;\n"));
    }

    fn save(&self, source: &mut String, point: NumericCheckpoint) {
        let ints = (0..point.ints)
            .map(|index| format!("b{}_i{index}", point.block.0))
            .collect::<Vec<_>>()
            .join(", ");
        let bools = (0..point.bools)
            .map(|index| format!("b{}_v{index}", point.block.0))
            .collect::<Vec<_>>()
            .join(", ");
        source.push_str(&format!("values.ints.clear();\nvalues.ints.extend_from_slice(&[{ints}]);\nvalues.bools.clear();\nvalues.bools.extend_from_slice(&[{bools}]);\n"));
    }

    fn path(&self, source: &mut String, block: BlockId, stop: Option<BlockId>) {
        let body = &self.shape.blocks[block.index()];
        for (instruction_index, instruction) in body.instructions.iter().enumerate() {
            let index = self.shape.starts[block.0] + instruction_index;
            self.tick(source, index, ProgressOutput::Direct);
            self.instruction(source, self.shape.checkpoints[index], instruction);
            self.big_exit(
                source,
                self.shape.checkpoints[index],
                instruction,
                index + 1,
                ProgressOutput::Direct,
            );
        }
        let index = self.shape.starts[block.0] + body.instructions.len();
        let point = self.shape.checkpoints[index];
        self.tick(source, index, ProgressOutput::Direct);
        let join = self.shape.joins[block.0];
        if let Some(join) = join {
            let join_point = self.shape.checkpoints[self.shape.starts[join.0]];
            source.push_str(&format!("let {} = ", self.locals(join_point, false)));
        }
        self.branch(
            source,
            point,
            &body.terminator,
            self.shape.repeats,
            ProgressOutput::Direct,
            |source, edge| self.edge(source, block, edge, join.or(stop)),
        );
        if let Some(join) = join {
            source.push_str(";\n");
            if Some(join) == stop {
                source.push_str(
                    &self.locals(self.shape.checkpoints[self.shape.starts[join.0]], false),
                );
                source.push('\n');
            } else {
                self.path(source, join, stop);
            }
        }
    }

    fn edge(&self, source: &mut String, from: BlockId, edge: &Edge, stop: Option<BlockId>) {
        let inputs = edge_inputs(from, edge);
        let target = edge.target();
        if target == self.shape.graph.entry() {
            source.push_str(&format!(
                "{} = {inputs};\ncontinue 'repeat;\n",
                self.locals(self.shape.checkpoints[self.entry()], false)
            ));
        } else if Some(target) == stop {
            source.push_str(&inputs);
            source.push('\n');
        } else {
            let point = self.shape.checkpoints[self.shape.starts[target.0]];
            source.push_str(&format!("let {} = {inputs};\n", self.locals(point, false)));
            self.path(source, target, stop);
        }
    }

    fn branch(
        &self,
        source: &mut String,
        point: NumericCheckpoint,
        terminator: &NumericTerminator<'_>,
        returning: bool,
        output: ProgressOutput,
        emit_edge: impl Fn(&mut String, &Edge),
    ) {
        match terminator {
            NumericTerminator::Jump(edge) => {
                source.push_str("{\n");
                emit_edge(source, edge);
                source.push_str("}\n");
            }
            NumericTerminator::Boolean {
                subject,
                true_,
                false_,
            } => {
                source.push_str(&format!("if b{}_v{} {{\n", point.block.0, subject.0));
                emit_edge(source, true_);
                source.push_str("} else {\n");
                emit_edge(source, false_);
                source.push_str("}\n");
            }
            NumericTerminator::Test {
                test,
                true_,
                false_,
            } => {
                source.push_str(&format!("if {} {{\n", test_expression(point.block, test)));
                emit_edge(source, true_);
                source.push_str("} else {\n");
                emit_edge(source, false_);
                source.push_str("}\n");
            }
            NumericTerminator::Switch {
                subject,
                clauses,
                fallback,
            } => {
                for (index, (literal, edge)) in clauses.iter().enumerate() {
                    if index > 0 {
                        source.push_str("else ");
                    }
                    source.push_str(&format!(
                        "if b{}_i{} == {}_i128 {{\n",
                        point.block.0, subject.0, literal
                    ));
                    emit_edge(source, edge);
                    source.push_str("} ");
                }
                if !clauses.is_empty() {
                    source.push_str("else ");
                }
                source.push_str("{\n");
                emit_edge(source, fallback);
                source.push_str("}\n");
            }
            NumericTerminator::Exit(exit) => self.complete(source, point, *exit, returning, output),
        }
    }

    fn resume_terminator(
        &self,
        source: &mut String,
        point: NumericCheckpoint,
        terminator: &NumericTerminator<'_>,
    ) {
        self.branch(
            source,
            point,
            terminator,
            false,
            ProgressOutput::Resume,
            |source, edge| {
                let next = self.shape.starts[edge.target().0];
                let target = self.shape.checkpoints[next];
                let inputs = edge_inputs(point.block, edge);
                source.push_str(&format!("let {} = {inputs};\n", self.locals(target, false)));
                self.save(source, target);
                source.push_str(&format!("NumericResume::Next({next})\n"));
            },
        );
    }

    fn complete(
        &self,
        source: &mut String,
        point: NumericCheckpoint,
        exit: BlockGraphExitId,
        returning: bool,
        output: ProgressOutput,
    ) {
        self.save(source, point);
        let (prefix, suffix) = if returning {
            ("return ", ";")
        } else {
            ("", "")
        };
        let progress = output.expression(format!(
            "data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId({}))",
            exit.0
        ));
        source.push_str(&format!("{prefix}{progress}{suffix}\n"));
    }

    fn instruction(
        &self,
        source: &mut String,
        point: NumericCheckpoint,
        instruction: &NumericInstruction<'_>,
    ) {
        match instruction {
            NumericInstruction::Integer(output, expression) => {
                source.push_str(&format!(
                    "let b{}_i{} = {};\n",
                    point.block.0,
                    output.0,
                    int_expression(point.block, expression)
                ));
            }
            NumericInstruction::Boolean(output, expression) => {
                let expression = match expression {
                    NumericBoolean::Value(value) => value.to_string(),
                    NumericBoolean::Test(test) => test_expression(point.block, test),
                };
                source.push_str(&format!(
                    "let b{}_v{} = {expression};\n",
                    point.block.0, output.0
                ));
            }
            NumericInstruction::Region { region, outputs } => {
                for (index, node) in region.nodes.iter().enumerate() {
                    let operand = |operand| match operand {
                        ArithmeticOperand::Input(index) => {
                            format!("b{}_i{}", point.block.0, region.inputs[index].0)
                        }
                        ArithmeticOperand::Value(index) => {
                            format!("_r{}_n{index}", point.instruction)
                        }
                        ArithmeticOperand::Immediate(value) => format!("{value}_i128"),
                    };
                    let expression = match *node {
                        ArithmeticNode::Add(left, right) => {
                            format!("{} + {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Subtract(left, right) => {
                            format!("{} - {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Multiply(left, right) => {
                            format!("{} * {}", operand(left), operand(right))
                        }
                        ArithmeticNode::Divide(left, right) => {
                            division(operand(left), operand(right), "/")
                        }
                        ArithmeticNode::Remainder(left, right) => {
                            division(operand(left), operand(right), "%")
                        }
                        ArithmeticNode::Negate(value) => format!("-{}", operand(value)),
                    };
                    source.push_str(&format!(
                        "let _r{}_n{index} = {expression};\n",
                        point.instruction
                    ));
                }
                for (output, local) in region.outputs.iter().zip(outputs) {
                    source.push_str(&format!(
                        "let b{}_i{} = _r{}_n{};\n",
                        point.block.0, local.0, point.instruction, output.value
                    ));
                }
            }
        }
    }

    fn big_exit(
        &self,
        source: &mut String,
        point: NumericCheckpoint,
        instruction: &NumericInstruction<'_>,
        next: usize,
        output: ProgressOutput,
    ) {
        // A Small remainder (including zero and MIN % -1) stays Small;
        // admitted literals and Boolean outputs likewise need no Big exit.
        let outputs = match instruction {
            NumericInstruction::Integer(
                _,
                NumericInteger::Value(_) | NumericInteger::Binary(NumericOperation::Remainder, ..),
            )
            | NumericInstruction::Boolean(..) => return,
            NumericInstruction::Integer(output, _) => std::slice::from_ref(output),
            NumericInstruction::Region { outputs, .. } => outputs.as_slice(),
        };
        let checks = outputs
            .iter()
            .map(|local| {
                let name = format!("b{}_i{}", point.block.0, local.0);
                format!("{name} < i128::from(i64::MIN) || {name} > i128::from(i64::MAX)")
            })
            .collect::<Vec<_>>();
        if checks.is_empty() {
            return;
        }
        source.push_str(&format!("if {} {{\n", checks.join(" || ")));
        self.save(source, self.shape.checkpoints[next]);
        let progress = output.expression(format!(
            "data::numeric::NumericProgress::Interpreted({next})"
        ));
        source.push_str(&format!("return {progress};\n}}\n"));
    }
}

fn checkpoint_inputs(point: NumericCheckpoint) -> String {
    tuple(
        (0..point.ints)
            .map(|index| format!("values.ints[{index}]"))
            .chain((0..point.bools).map(|index| format!("values.bools[{index}]"))),
    )
}

fn tuple(values: impl IntoIterator<Item = String>) -> String {
    let values = values
        .into_iter()
        .map(|value| format!("{value},"))
        .collect::<Vec<_>>()
        .join(" ");
    format!("({values})")
}

fn edge_inputs(block: BlockId, edge: &Edge) -> String {
    let ints = edge.args().iter().filter_map(|local| match local {
        ParamLocal::Int(local) => Some(format!("b{}_i{}", block.0, local.0)),
        _ => None,
    });
    let bools = edge.args().iter().filter_map(|local| match local {
        ParamLocal::Bool(local) => Some(format!("b{}_v{}", block.0, local.0)),
        _ => None,
    });
    tuple(ints.chain(bools))
}

fn operand_expression(block: BlockId, operand: IntegerOperand) -> String {
    match operand {
        IntegerOperand::Local(local) => format!("b{}_i{}", block.0, local.0),
        IntegerOperand::Immediate(value) => format!("{value}_i128"),
    }
}

fn int_expression(block: BlockId, expression: &NumericInteger<'_>) -> String {
    match expression {
        NumericInteger::Value(value) => format!("{value}_i128"),
        NumericInteger::Negate(value) => format!("-b{}_i{}", block.0, value.0),
        NumericInteger::Binary(operation, left, right) => {
            let left = operand_expression(block, *left);
            let right = operand_expression(block, *right);
            match operation {
                NumericOperation::Add => format!("{left} + {right}"),
                NumericOperation::Subtract => format!("{left} - {right}"),
                NumericOperation::Multiply => format!("{left} * {right}"),
                NumericOperation::Divide => division(left, right, "/"),
                NumericOperation::Remainder => division(left, right, "%"),
            }
        }
    }
}

fn division(left: String, right: String, operator: &str) -> String {
    if right == "0_i128" {
        "0_i128".to_owned()
    } else {
        format!("if {right} == 0 {{ 0_i128 }} else {{ {left} {operator} {right} }}")
    }
}

fn test_expression(block: BlockId, test: &NumericTest) -> String {
    match test {
        NumericTest::Not(value) => format!("!b{}_v{}", block.0, value.0),
        NumericTest::Compare(comparison, left, right) => {
            let operator = match comparison {
                NumericComparison::Equal => "==",
                NumericComparison::NotEqual => "!=",
                NumericComparison::Less => "<",
                NumericComparison::LessEqual => "<=",
                NumericComparison::Greater => ">",
                NumericComparison::GreaterEqual => ">=",
            };
            format!(
                "{} {operator} {}",
                operand_expression(block, *left),
                operand_expression(block, *right)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FunctionEmission, NumericComparison, NumericEmission, NumericInteger, NumericOperation,
        NumericShape, NumericTerminator, NumericTest, ProgressOutput, Rust, int_expression,
        test_expression,
    };
    use crate::embedding::{BigInt, FunctionDeclaration, ModuleBuilder};
    use crate::plan::execution::function::{ExecutionIntFunctionBody, FunctionExit, IntFunctionId};
    use crate::plan::execution::graph::{
        ArithmeticNode, ArithmeticOperand, ArithmeticOutput, ArithmeticRegion, BlockGraphExitId,
        BlockId, BoolBranch, BoolLocalId, BoolTest, Edge, FamilyTransfer, IntLocalId, IntSwitch,
        IntegerLiteral, IntegerOperand, Jump, ParamLocal, ParamSlot, ProfiledBlock,
        ProfiledBlockGraph, StorageFamily, Terminator, TestBranch, Transfer,
    };
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::convert::Infallible;

    #[test]
    fn scalar_arithmetic_and_boolean_comparisons_have_exact_rust_expressions() {
        let block = BlockId(3);
        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Immediate(-7);
        for (operation, expected) in [
            (NumericOperation::Add, "b3_i2 + -7_i128"),
            (NumericOperation::Subtract, "b3_i2 - -7_i128"),
            (NumericOperation::Multiply, "b3_i2 * -7_i128"),
            (
                NumericOperation::Divide,
                "if -7_i128 == 0 { 0_i128 } else { b3_i2 / -7_i128 }",
            ),
            (
                NumericOperation::Remainder,
                "if -7_i128 == 0 { 0_i128 } else { b3_i2 % -7_i128 }",
            ),
        ] {
            assert_eq!(
                int_expression(block, &NumericInteger::Binary(operation, left, right)),
                expected
            );
        }
        for operation in [NumericOperation::Divide, NumericOperation::Remainder] {
            assert_eq!(
                int_expression(
                    block,
                    &NumericInteger::Binary(operation, left, IntegerOperand::Immediate(0))
                ),
                "0_i128"
            );
        }
        let literal = IntegerLiteral::from(BigInt::from(i64::MIN));
        assert_eq!(
            int_expression(block, &NumericInteger::Value(&literal)),
            "-9223372036854775808_i128"
        );
        assert_eq!(
            int_expression(block, &NumericInteger::Negate(IntLocalId(4))),
            "-b3_i4"
        );
        for (comparison, expected) in [
            (NumericComparison::Equal, "b3_i2 == -7_i128"),
            (NumericComparison::NotEqual, "b3_i2 != -7_i128"),
            (NumericComparison::Less, "b3_i2 < -7_i128"),
            (NumericComparison::LessEqual, "b3_i2 <= -7_i128"),
            (NumericComparison::Greater, "b3_i2 > -7_i128"),
            (NumericComparison::GreaterEqual, "b3_i2 >= -7_i128"),
        ] {
            assert_eq!(
                test_expression(block, &NumericTest::Compare(comparison, left, right)),
                expected
            );
        }
        assert_eq!(
            test_expression(block, &NumericTest::Not(BoolLocalId(5))),
            "!b3_v5"
        );
    }

    #[test]
    fn empty_switch_arms_and_unretained_region_outputs_emit_only_their_actual_work() {
        let source = "fn choose(value: Int) { case value { 0 -> value + 1 _ -> value - 1 } } pub fn main() { choose(7) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = NumericShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let point = shape.checkpoints[shape.starts[shape.graph.entry().index()]];
        let function = FunctionEmission {
            index: 1,
            name: "selected".into(),
            shape,
        };
        let edge = Edge::new(
            BlockId(1),
            vec![ParamLocal::Int(IntLocalId(0))],
            Transfer {
                families: vec![FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 1,
                    steps: vec![].into(),
                }]
                .into(),
            },
        );
        let terminator = NumericTerminator::Switch {
            subject: IntLocalId(0),
            clauses: &[],
            fallback: &edge,
        };
        let mut output = String::new();
        function.branch(
            &mut output,
            point,
            &terminator,
            false,
            ProgressOutput::Direct,
            |source, _| source.push_str("fallback\n"),
        );
        assert_eq!(output, "{\nfallback\n}\n");

        output.clear();
        function.resume_terminator(&mut output, point, &terminator);
        assert_eq!(
            output,
            "{\nlet (b1_i0,) = (b0_i0,);\nvalues.ints.clear();\nvalues.ints.extend_from_slice(&[b1_i0]);\nvalues.bools.clear();\nvalues.bools.extend_from_slice(&[]);\nNumericResume::Next(1)\n}\n"
        );

        // Artifact graph admission permits arithmetic whose outputs are not
        // retained. It still performs its nodes and has no Big prefix to save.
        let region = ArithmeticRegion {
            inputs: vec![IntLocalId(0)].into(),
            nodes: vec![
                ArithmeticNode::Add(ArithmeticOperand::Input(0), ArithmeticOperand::Immediate(1)),
                ArithmeticNode::Negate(ArithmeticOperand::Value(0)),
            ]
            .into(),
            outputs: vec![].into(),
            native: true,
        };
        let instruction = super::NumericInstruction::Region {
            region: &region,
            outputs: vec![],
        };
        output.clear();
        function.instruction(&mut output, point, &instruction);
        assert_eq!(
            output,
            "let _r0_n0 = b0_i0 + 1_i128;\nlet _r0_n1 = -_r0_n0;\n"
        );
        output.clear();
        function.big_exit(&mut output, point, &instruction, 0, ProgressOutput::Direct);
        assert_eq!(output, "");

        // Scalar and retained region outputs share the exact next checkpoint.
        // A Small remainder cannot promote; an Add or a retained region can.
        let next = function.shape.starts[1] + 1;
        let point = function.shape.checkpoints[function.shape.starts[1]];
        let expected = "if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {\nvalues.ints.clear();\nvalues.ints.extend_from_slice(&[b1_i0, b1_i1]);\nvalues.bools.clear();\nvalues.bools.extend_from_slice(&[]);\nreturn data::numeric::NumericProgress::Interpreted(2);\n}\n";
        output.clear();
        function.big_exit(
            &mut output,
            point,
            &function.shape.blocks[1].instructions[0],
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output, expected);
        output.clear();
        function.big_exit(
            &mut output,
            point,
            &super::NumericInstruction::Integer(
                IntLocalId(1),
                NumericInteger::Binary(
                    NumericOperation::Remainder,
                    IntegerOperand::Local(IntLocalId(0)),
                    IntegerOperand::Immediate(3),
                ),
            ),
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output, "");
        let region = ArithmeticRegion {
            outputs: vec![ArithmeticOutput {
                value: 1,
                slot: ParamSlot::new(
                    ParamLocal::Int(IntLocalId(1)),
                    function.shape.graph.block(BlockId(1)).params()[0].shape(),
                ),
            }]
            .into(),
            ..region
        };
        function.big_exit(
            &mut output,
            point,
            &super::NumericInstruction::Region {
                region: &region,
                outputs: vec![IntLocalId(1)],
            },
            next,
            ProgressOutput::Direct,
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn structured_paths_emit_exact_branch_expressions_and_simultaneous_join_arguments() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "fn choose(value: Int, flag: Bool) { case flag { True -> value + 1 False -> value - 1 } } pub fn main() { choose(7, True) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let params = graph.block(graph.entry()).params();
        let edge = Edge::new(
            BlockId(1),
            vec![
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Bool(BoolLocalId(0)),
            ],
            Transfer {
                families: [StorageFamily::Int, StorageFamily::Bool]
                    .into_iter()
                    .map(|family| FamilyTransfer {
                        family,
                        length: 1,
                        steps: vec![].into(),
                    })
                    .collect::<Vec<_>>()
                    .into(),
            },
        );
        let switch = |values: &[i64]| {
            Terminator::IntSwitch(IntSwitch {
                subject: IntLocalId(0),
                clauses: values
                    .iter()
                    .map(|&value| (IntegerLiteral::from(BigInt::from(value)), edge.clone()))
                    .collect::<Vec<_>>()
                    .into(),
                fallback: edge.clone(),
            })
        };
        let arguments = "(b0_i0, b0_v0,)\n";
        for (terminator, expected) in [
            (
                Terminator::Jump(Jump::new(edge.clone())),
                format!("{{\n{arguments}}}\n"),
            ),
            (
                Terminator::BoolBranch(BoolBranch {
                    subject: BoolLocalId(0),
                    true_: edge.clone(),
                    false_: edge.clone(),
                }),
                format!("if b0_v0 {{\n{arguments}}} else {{\n{arguments}}}\n"),
            ),
            (
                Terminator::TestBranch(TestBranch::new(
                    BoolTest::GtInt {
                        left: IntegerOperand::Local(IntLocalId(0)),
                        right: IntegerOperand::Immediate(0),
                    },
                    edge.clone(),
                    edge.clone(),
                )),
                format!("if b0_i0 > 0_i128 {{\n{arguments}}} else {{\n{arguments}}}\n"),
            ),
            (switch(&[]), format!("{{\n{arguments}}}\n")),
            (
                switch(&[0]),
                format!("if b0_i0 == 0_i128 {{\n{arguments}}} else {{\n{arguments}}}\n"),
            ),
            (
                switch(&[0, 1]),
                format!(
                    "if b0_i0 == 0_i128 {{\n{arguments}}} else if b0_i0 == 1_i128 {{\n{arguments}}} else {{\n{arguments}}}\n"
                ),
            ),
        ] {
            let body: ExecutionIntFunctionBody<Infallible> =
                ExecutionIntFunctionBody::<Infallible>::from_parts(
                    ProfiledBlockGraph::from_parts(
                        BlockId(0),
                        vec![
                            ProfiledBlock::new(params.to_vec(), vec![], terminator),
                            ProfiledBlock::new(
                                params.to_vec(),
                                vec![],
                                Terminator::Exit(BlockGraphExitId(0)),
                            ),
                        ],
                    ),
                    vec![FunctionExit::Return(IntLocalId(0))].into(),
                );
            let function = FunctionEmission {
                index: 1,
                name: "selected".into(),
                shape: NumericShape::inspect(&body).unwrap(),
            };
            let mut output = String::new();
            function.path(&mut output, BlockId(0), Some(BlockId(1)));
            assert_eq!(
                output,
                format!(
                    "if *budget == 0 {{\nvalues.ints.clear();\nvalues.ints.extend_from_slice(&[b0_i0]);\nvalues.bools.clear();\nvalues.bools.extend_from_slice(&[b0_v0]);\nreturn data::numeric::NumericProgress::Yield(0);\n}}\n*budget -= 1;\nlet (b1_i0, b1_v0,) = {expected};\n(b1_i0, b1_v0,)\n"
                )
            );
        }
    }

    // Three blocks: read the Bool entry argument, produce the selected literal,
    // and return. The five checkpoints include the completed literal outputs.
    // These exact protocol oracles stay local to their emitting owner.
    const INT_EXPECTED: &str = r#"{
enum NumericResume {
Next(usize),
Exit(data::numeric::NumericProgress),
}
fn numeric_int_0(point: usize, values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
const RESUME: [fn(&mut data::numeric::NumericValues, &mut usize) -> NumericResume; 5] = [
|values, budget| NumericResume::Exit(numeric_int_0_entry((values.bools[0],), values, budget)),
numeric_int_0_resume_1,
numeric_int_0_resume_2,
numeric_int_0_resume_3,
numeric_int_0_resume_4,
];
let mut point = point;
loop {
match RESUME[point](values, budget) {
NumericResume::Next(next) => point = next,
NumericResume::Exit(progress) => return progress,
}
}
}
fn numeric_int_0_entry(inputs: (bool,), values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
let (b0_v0,) = inputs;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b0_v0]);
return data::numeric::NumericProgress::Yield(0);
}
*budget -= 1;
if b0_v0 {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(1);
}
*budget -= 1;
let b1_i0 = 1_i128;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(2);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0))
} else {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(3);
}
*budget -= 1;
let b2_i0 = 2_i128;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(4);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1))
}
}
fn numeric_int_0_resume_1(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(1));
}
*budget -= 1;
let b1_i0 = 1_i128;
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Next(2)
}
fn numeric_int_0_resume_2(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b1_i0,) = (values.ints[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(2));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0)))
}
fn numeric_int_0_resume_3(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(3));
}
*budget -= 1;
let b2_i0 = 2_i128;
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Next(4)
}
fn numeric_int_0_resume_4(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b2_i0,) = (values.ints[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(4));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1)))
}
data::numeric::NumericFunctions {
ints: data::Storage::Static(&[
data::numeric::NumericFunction { function: data::function::IntFunctionId(0), implementation: data::numeric::NumericImplementation { entry: 0, checkpoints: data::Storage::Static(&[
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(0),
        instruction: 0,
        ints: 0,
        bools: 1,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 0,
        ints: 0,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 1,
        ints: 1,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 0,
        ints: 0,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 1,
        ints: 1,
        bools: 0,
    },
]), run: numeric_int_0 } },
]),
bools: data::Storage::Static(&[
]),
}
}"#;
    const BOOL_EXPECTED: &str = r#"{
enum NumericResume {
Next(usize),
Exit(data::numeric::NumericProgress),
}
fn numeric_bool_0(point: usize, values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
const RESUME: [fn(&mut data::numeric::NumericValues, &mut usize) -> NumericResume; 5] = [
|values, budget| NumericResume::Exit(numeric_bool_0_entry((values.bools[0],), values, budget)),
numeric_bool_0_resume_1,
numeric_bool_0_resume_2,
numeric_bool_0_resume_3,
numeric_bool_0_resume_4,
];
let mut point = point;
loop {
match RESUME[point](values, budget) {
NumericResume::Next(next) => point = next,
NumericResume::Exit(progress) => return progress,
}
}
}
fn numeric_bool_0_entry(inputs: (bool,), values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
let (b0_v0,) = inputs;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b0_v0]);
return data::numeric::NumericProgress::Yield(0);
}
*budget -= 1;
if b0_v0 {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(1);
}
*budget -= 1;
let b1_v0 = false;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b1_v0]);
return data::numeric::NumericProgress::Yield(2);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b1_v0]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0))
} else {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(3);
}
*budget -= 1;
let b2_v0 = true;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b2_v0]);
return data::numeric::NumericProgress::Yield(4);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b2_v0]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1))
}
}
fn numeric_bool_0_resume_1(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(1));
}
*budget -= 1;
let b1_v0 = false;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b1_v0]);
NumericResume::Next(2)
}
fn numeric_bool_0_resume_2(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b1_v0,) = (values.bools[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b1_v0]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(2));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b1_v0]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0)))
}
fn numeric_bool_0_resume_3(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let () = ();
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(3));
}
*budget -= 1;
let b2_v0 = true;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b2_v0]);
NumericResume::Next(4)
}
fn numeric_bool_0_resume_4(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b2_v0,) = (values.bools[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b2_v0]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(4));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[b2_v0]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1)))
}
data::numeric::NumericFunctions {
ints: data::Storage::Static(&[
]),
bools: data::Storage::Static(&[
data::numeric::NumericFunction { function: data::function::BoolFunctionId(0), implementation: data::numeric::NumericImplementation { entry: 0, checkpoints: data::Storage::Static(&[
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(0),
        instruction: 0,
        ints: 0,
        bools: 1,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 0,
        ints: 0,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 1,
        ints: 0,
        bools: 1,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 0,
        ints: 0,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 1,
        ints: 0,
        bools: 1,
    },
]), run: numeric_bool_0 } },
]),
}
}"#;

    #[test]
    fn plain_and_hosted_int_and_bool_families_emit_exact_execution_code_and_links() {
        macro_rules! check_family {
            ($source:expr, $hosted_source:expr, $return:ty, $family:expr, $expected:expr) => {{
                let typed =
                    crate::compile_typed_module("example", "src/example.gleam", $source).unwrap();
                let (bindings, _) = ModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(bool,), $return>::new("main"))
                    .unwrap();
                let prepared = bindings.prepare();
                assert_eq!(
                    Rust::expression(&NumericEmission::new(&prepared.program.functions)),
                    $expected
                );

                let typed = crate::compile_typed_host_program(
                    "example",
                    "example",
                    [PackageSource::new(
                        "example",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "example",
                            "src/example.gleam",
                            $hosted_source,
                        )],
                    )],
                    HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
                )
                .unwrap();
                let execution = crate::HostedExecution::try_from_module_plan(
                    crate::plan_host_program(typed).unwrap(),
                )
                .unwrap();
                // The standalone main occupies family slot zero. Its numeric
                // callee has exactly the same body, now linked as slot one.
                let expected = $expected
                    .replace(
                        &format!("numeric_{}_0", $family.0),
                        &format!("numeric_{}_1", $family.0),
                    )
                    .replace(
                        &format!("{}FunctionId(0)", $family.1),
                        &format!("{}FunctionId(1)", $family.1),
                    );
                assert_eq!(
                    Rust::expression(&NumericEmission::new(
                        &execution.execution.program.functions
                    )),
                    expected
                );
            }};
        }
        check_family!(
            "pub fn main(flag: Bool) { case flag { True -> 1 False -> 2 } }",
            "fn choose(flag: Bool) { case flag { True -> 1 False -> 2 } } pub fn main() { choose(True) }",
            BigInt,
            ("int", "Int"),
            INT_EXPECTED
        );
        check_family!(
            "pub fn main(flag: Bool) { case flag { True -> False False -> True } }",
            "fn choose(flag: Bool) { case flag { True -> False False -> True } } pub fn main() { choose(True) }",
            bool,
            ("bool", "Bool"),
            BOOL_EXPECTED
        );
    }

    #[test]
    fn unsupported_plain_and_hosted_programs_emit_an_explicit_interpreted_sidecar() {
        let source = "pub fn main() { 42 }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (bindings, _) = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap();
        let prepared = bindings.prepare();
        assert_eq!(
            Rust::expression(&NumericEmission::new(&prepared.program.functions)),
            "data::numeric::NumericFunctions::interpreted()"
        );
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new("example", "src/example.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            Rust::expression(&NumericEmission::new(
                &execution.execution.program.functions
            )),
            "data::numeric::NumericFunctions::interpreted()"
        );
    }
}
