use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use super::graph::{FunctionLocal, Graph, GraphExitId};
use super::{
    BitArrayListLocalId, BitArrayLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId,
    CustomLocal, FloatListLocalId, FloatLocalId, FunctionListLocalId, IntListLocalId, IntLocalId,
    ListListLocalId, NilListLocalId, NilLocalId, ParameterListListLocalId, ParameterListLocalId,
    StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
};

pub(crate) struct ConstantId<Value> {
    index: usize,
    value: PhantomData<fn() -> Value>,
}

impl<Value> ConstantId<Value> {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self {
            index,
            value: PhantomData,
        }
    }

    pub(in crate::plan::execution) fn index(self) -> usize {
        self.index
    }
}

impl<Value> Clone for ConstantId<Value> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Value> Copy for ConstantId<Value> {}

impl<Value> std::fmt::Debug for ConstantId<Value> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ConstantId")
            .field(&self.index)
            .finish()
    }
}

impl<Value> PartialEq for ConstantId<Value> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Value> Eq for ConstantId<Value> {}

impl<Value> Hash for ConstantId<Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

pub(crate) struct ConstantProgram<Value> {
    graph: Graph,
    returns: Box<[Value]>,
}

impl<Value> ConstantProgram<Value> {
    pub(in crate::plan::execution) fn from_parts(graph: Graph, returns: Vec<Value>) -> Self {
        Self {
            graph,
            returns: returns.into_boxed_slice(),
        }
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn return_(&self, id: GraphExitId) -> &Value {
        &self.returns[id.index()]
    }
}

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

macro_rules! constant_value {
    ($value:ty, $field:ident) => {
        impl ConstantValue for $value {
            fn programs(table: &ConstantTable) -> &[ConstantProgram<Self>] {
                &table.$field
            }

            fn programs_mut(table: &mut ConstantTable) -> &mut Vec<ConstantProgram<Self>> {
                &mut table.$field
            }
        }
    };
}

constant_value!(IntLocalId, ints);
constant_value!(StringLocalId, strings);
constant_value!(BitArrayLocalId, bit_arrays);
constant_value!(CustomLocal, customs);
constant_value!(FloatLocalId, floats);
constant_value!(BoolLocalId, bools);
constant_value!(NilLocalId, nils);
constant_value!(TupleLocalId, tuples);
constant_value!(ParameterListLocalId, parameter_lists);
constant_value!(ParameterListListLocalId, parameter_list_lists);
constant_value!(IntListLocalId, int_lists);
constant_value!(StringListLocalId, string_lists);
constant_value!(BitArrayListLocalId, bit_array_lists);
constant_value!(UtfCodepointListLocalId, utf_codepoint_lists);
constant_value!(CustomListLocalId, custom_lists);
constant_value!(FloatListLocalId, float_lists);
constant_value!(BoolListLocalId, bool_lists);
constant_value!(NilListLocalId, nil_lists);
constant_value!(TupleListLocalId, tuple_lists);
constant_value!(ListListLocalId, list_lists);
constant_value!(FunctionListLocalId, function_lists);
constant_value!(FunctionLocal, functions);

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
    use super::super::graph::IntInstruction;
    use super::ConstantId;
    use crate::plan::execution::{
        BlockId, ExecutionPlan, Instruction, InstructionKind, IntLocalId, ParamLocal,
        SourceStopKind, Terminator,
    };
    use num_bigint::BigInt;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct UncomparableValue;

    #[test]
    fn constant_id_traits_depend_only_on_the_typed_index() {
        fn assert_copy<T: Copy>() {}

        assert_copy::<ConstantId<UncomparableValue>>();
        let id = ConstantId::<UncomparableValue>::new(3);
        let copied = id;
        let cloned = <ConstantId<UncomparableValue> as Clone>::clone(&id);
        let different = ConstantId::<UncomparableValue>::new(4);

        assert_eq!(copied, id);
        assert_eq!(cloned, id);
        assert_ne!(id, different);
        assert_eq!(format!("{id:?}"), "ConstantId(3)");

        let mut id_hasher = DefaultHasher::new();
        id.hash(&mut id_hasher);
        let mut copied_hasher = DefaultHasher::new();
        copied.hash(&mut copied_hasher);
        let mut different_hasher = DefaultHasher::new();
        different.hash(&mut different_hasher);

        assert_eq!(id_hasher.finish(), copied_hasher.finish());
        assert_ne!(id_hasher.finish(), different_hasher.finish());
    }

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
            &Terminator::SourceStop {
                kind: SourceStopKind::Panic,
                message: None,
                site: crate::plan::PanicSite::unknown(),
            },
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
