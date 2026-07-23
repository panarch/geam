use super::{ConstantId, ConstantProgram};
use crate::plan::execution::graph::FunctionLocal;
use crate::plan::execution::{
    BitArrayListLocalId, BitArrayLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId,
    CustomLocal, FloatListLocalId, FloatLocalId, FunctionListLocalId, IntListLocalId, IntLocalId,
    ListListLocalId, NilListLocalId, NilLocalId, ParameterListListLocalId, ParameterListLocalId,
    StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
};

#[derive(Default)]
pub(crate) struct ConstantTable {
    ints: Vec<ConstantProgram<IntLocalId>>,
    strings: Vec<ConstantProgram<StringLocalId>>,
    bit_arrays: Vec<ConstantProgram<BitArrayLocalId>>,
    customs: Vec<ConstantProgram<CustomLocal>>,
    floats: Vec<ConstantProgram<FloatLocalId>>,
    bools: Vec<ConstantProgram<BoolLocalId>>,
    nils: Vec<ConstantProgram<NilLocalId>>,
    tuples: Vec<ConstantProgram<TupleLocalId>>,
    parameter_lists: Vec<ConstantProgram<ParameterListLocalId>>,
    parameter_list_lists: Vec<ConstantProgram<ParameterListListLocalId>>,
    int_lists: Vec<ConstantProgram<IntListLocalId>>,
    string_lists: Vec<ConstantProgram<StringListLocalId>>,
    bit_array_lists: Vec<ConstantProgram<BitArrayListLocalId>>,
    utf_codepoint_lists: Vec<ConstantProgram<UtfCodepointListLocalId>>,
    custom_lists: Vec<ConstantProgram<CustomListLocalId>>,
    float_lists: Vec<ConstantProgram<FloatListLocalId>>,
    bool_lists: Vec<ConstantProgram<BoolListLocalId>>,
    nil_lists: Vec<ConstantProgram<NilListLocalId>>,
    tuple_lists: Vec<ConstantProgram<TupleListLocalId>>,
    list_lists: Vec<ConstantProgram<ListListLocalId>>,
    function_lists: Vec<ConstantProgram<FunctionListLocalId>>,
    functions: Vec<ConstantProgram<FunctionLocal>>,
}

pub(crate) trait ConstantValue: Sized {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>];
    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>>;
}

impl ConstantValue for IntLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.ints
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.ints
    }
}

impl ConstantValue for StringLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.strings
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.strings
    }
}

impl ConstantValue for BitArrayLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.bit_arrays
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.bit_arrays
    }
}

impl ConstantValue for CustomLocal {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.customs
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.customs
    }
}

impl ConstantValue for FloatLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.floats
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.floats
    }
}

impl ConstantValue for BoolLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.bools
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.bools
    }
}

impl ConstantValue for NilLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.nils
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.nils
    }
}

impl ConstantValue for TupleLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.tuples
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.tuples
    }
}

impl ConstantValue for ParameterListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.parameter_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.parameter_lists
    }
}

impl ConstantValue for ParameterListListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.parameter_list_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.parameter_list_lists
    }
}

impl ConstantValue for IntListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.int_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.int_lists
    }
}

impl ConstantValue for StringListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.string_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.string_lists
    }
}

impl ConstantValue for BitArrayListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.bit_array_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.bit_array_lists
    }
}

impl ConstantValue for UtfCodepointListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.utf_codepoint_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.utf_codepoint_lists
    }
}

impl ConstantValue for CustomListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.custom_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.custom_lists
    }
}

impl ConstantValue for FloatListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.float_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.float_lists
    }
}

impl ConstantValue for BoolListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.bool_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.bool_lists
    }
}

impl ConstantValue for NilListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.nil_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.nil_lists
    }
}

impl ConstantValue for TupleListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.tuple_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.tuple_lists
    }
}

impl ConstantValue for ListListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.list_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.list_lists
    }
}

impl ConstantValue for FunctionListLocalId {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.function_lists
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.function_lists
    }
}

impl ConstantValue for FunctionLocal {
    fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
        &table.functions
    }

    fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
        &mut table.functions
    }
}

impl ConstantTable {
    pub(in crate::plan::execution) fn push<Value: ConstantValue>(
        &mut self,
        program: ConstantProgram<Value>,
    ) -> ConstantId<Value> {
        let programs = Value::programs_mut(self);
        let id = ConstantId::new(programs.len());
        programs.push(program);
        id
    }

    pub(crate) fn get<Value: ConstantValue>(
        &self,
        id: ConstantId<Value>,
    ) -> &ConstantProgram<Value> {
        &Value::programs(self)[id.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantId;
    use crate::plan::execution::graph::{IntInstruction, SourceStop, SourceStopKind};
    use crate::plan::execution::{
        BlockId, ExecutionPlan, Instruction, InstructionKind, IntLocalId, ParamLocal, Terminator,
    };
    use num_bigint::BigInt;
    #[test]
    fn constant_entry_is_a_reusable_zero_argument_typed_graph_program() {
        let plan = execution_plan("const one = 1 pub fn main() { one + one }");
        let program = plan.constant(ConstantId::<IntLocalId>::new(0));
        let graph = program.graph();

        assert_eq!(graph.entry(), BlockId::new(0));
        assert_eq!(graph.blocks().len(), 1);
        let block = graph.block(BlockId::new(0));
        assert!(block.params().is_empty());
        assert_eq!(block.instructions().len(), 1);
        let instruction = &block.instructions()[0];
        assert_eq!(
            instruction.output().local(),
            &ParamLocal::Int(IntLocalId(0))
        );
        assert_eq!(int_literal(instruction), &1.into());
        assert_eq!(returned_int(program, block.terminator()), IntLocalId(0));

        let main = plan.int_function(crate::plan::execution::IntFunctionId(0));
        let block = main.graph().block(BlockId::new(0));
        assert_eq!(block.instructions().len(), 3);
        for (index, instruction) in block.instructions()[..2].iter().enumerate() {
            let output = IntLocalId(index);
            assert_eq!(instruction.output().local(), &ParamLocal::Int(output));
            assert_eq!(int_constant(instruction), ConstantId::new(0));
        }
    }

    #[test]
    #[should_panic(expected = "constant fixture should contain an Int literal")]
    fn int_literal_guard_rejects_other_instructions() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let graph = plan
            .int_function(crate::plan::execution::IntFunctionId(0))
            .graph();
        int_literal(&graph.block(graph.entry()).instructions()[0]);
    }

    #[test]
    #[should_panic(expected = "constant fixture should return an Int local")]
    fn returned_int_guard_rejects_other_terminators() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let program = plan.constant(ConstantId::<IntLocalId>::new(0));
        returned_int(
            program,
            &Terminator::SourceStop(SourceStop::new(
                SourceStopKind::Panic,
                None,
                crate::plan::PanicSite::unknown(),
            )),
        );
    }

    #[test]
    #[should_panic(expected = "constant fixture should reference an Int constant")]
    fn int_constant_guard_rejects_other_instructions() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let graph = plan.constant(ConstantId::<IntLocalId>::new(0)).graph();
        int_constant(&graph.block(graph.entry()).instructions()[0]);
    }

    fn int_literal(instruction: &Instruction) -> &BigInt {
        match instruction.kind() {
            InstructionKind::Int(IntInstruction::Value(value)) => value,
            _ => panic!("constant fixture should contain an Int literal"),
        }
    }

    fn returned_int(
        program: &super::ConstantProgram<IntLocalId>,
        terminator: &Terminator,
    ) -> IntLocalId {
        match terminator {
            Terminator::Exit(exit) => *program.return_(*exit),
            _ => panic!("constant fixture should return an Int local"),
        }
    }

    fn int_constant(instruction: &Instruction) -> ConstantId<IntLocalId> {
        match instruction.kind() {
            InstructionKind::Int(IntInstruction::Constant(id)) => *id,
            _ => panic!("constant fixture should reference an Int constant"),
        }
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
