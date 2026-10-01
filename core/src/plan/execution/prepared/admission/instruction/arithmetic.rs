use super::super::local::Locals;
use super::{InstructionError, Instructions, read};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    ArithmeticOperand, ArithmeticRegion, MAX_ARITHMETIC_NODES, native_proof,
};
use crate::plan::execution::type_::ValueType;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) enum ArithmeticError {
    Size,
    DuplicateInput,
    Operand { node: usize },
    Output { index: usize },
    Proof,
}

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn arithmetic(
        &self,
        region: &ArithmeticRegion,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let fail = InstructionError::Arithmetic;
        if !(2..=MAX_ARITHMETIC_NODES).contains(&region.nodes.len())
            || region.inputs.len() > MAX_ARITHMETIC_NODES * 2
            || region.outputs.len() > region.nodes.len()
        {
            return Err(fail(ArithmeticError::Size));
        }
        for (index, input) in region.inputs.iter().enumerate() {
            if region.inputs[..index].contains(input) {
                return Err(fail(ArithmeticError::DuplicateInput));
            }
            read(input, locals)?;
        }
        for (index, node) in region.nodes.iter().enumerate() {
            for operand in node.operands() {
                let valid = match operand {
                    ArithmeticOperand::Input(index) => index < region.inputs.len(),
                    ArithmeticOperand::Value(value) => value < index,
                    ArithmeticOperand::Immediate(_) => true,
                };
                if !valid {
                    return Err(fail(ArithmeticError::Operand { node: index }));
                }
            }
        }
        let mut previous = None;
        for (index, output) in region.outputs.iter().enumerate() {
            if output.value >= region.nodes.len()
                || previous.is_some_and(|value| value >= output.value)
            {
                return Err(fail(ArithmeticError::Output { index }));
            }
            self.output_type(&output.slot, &ValueType::Int)?;
            previous = Some(output.value);
        }
        if region.native != native_proof(region.inputs.len(), &region.nodes) {
            return Err(fail(ArithmeticError::Proof));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ArithmeticError, InstructionError, Instructions};
    use crate::plan::execution::graph::{
        ArithmeticNode as N, ArithmeticOperand as O, IntLocalId, MAX_ARITHMETIC_NODES,
        ProfiledInstruction,
    };
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog,
        local::{LocalError, Locals},
        source::Sources,
        type_::{TypeError, Types},
    };

    #[test]
    fn admission_rechecks_real_regions_and_rejects_forged_links_sizes_and_proofs() {
        let typed = crate::compile_typed_module(
            "main",
            "main.gleam",
            r#"
fn calculate(n: Int) { n * 2 + 1 }
pub fn main() { calculate(3) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let graph = plan.program.functions.value_returns.int_functions[1]
            .body()
            .block_graph();
        let block = graph.block(graph.entry());
        let mut locals = Locals::default();
        for slot in block.params() {
            locals.define(slot, &types).unwrap();
        }
        let main = plan.program.functions.value_returns.int_functions[0]
            .body()
            .block_graph();
        let regions = block
            .instructions()
            .iter()
            .chain(main.block(main.entry()).instructions().iter())
            .filter_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(region) => Some(region),
                ProfiledInstruction::Value(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(regions.len(), 1);
        let original = regions[0];
        assert_eq!(context.check(&block.instructions()[0], &locals), Ok(()));
        let fail = |region, expected| {
            assert_eq!(
                context.arithmetic(&region, &locals),
                Err(InstructionError::Arithmetic(expected))
            )
        };
        let mut region = original.clone();
        region.nodes = vec![N::Negate(O::Input(0))].into();
        fail(region, ArithmeticError::Size);
        let mut region = original.clone();
        region.nodes = vec![N::Negate(O::Input(0)); MAX_ARITHMETIC_NODES + 1].into();
        fail(region, ArithmeticError::Size);
        let mut region = original.clone();
        region.inputs = vec![IntLocalId(0); MAX_ARITHMETIC_NODES * 2 + 1].into();
        fail(region, ArithmeticError::Size);
        let mut region = original.clone();
        region.outputs = vec![original.outputs[0].clone(); 3].into();
        fail(region, ArithmeticError::Size);
        let mut region = original.clone();
        region.inputs = vec![IntLocalId(0), IntLocalId(0)].into();
        fail(region, ArithmeticError::DuplicateInput);
        for operand in [O::Input(1), O::Value(0)] {
            let mut region = original.clone();
            region.nodes = vec![N::Negate(operand), original.nodes[1]].into();
            fail(region, ArithmeticError::Operand { node: 0 });
        }
        let mut region = original.clone();
        let mut output = original.outputs[0].clone();
        output.value = 2;
        region.outputs = vec![output].into();
        fail(region, ArithmeticError::Output { index: 0 });
        let mut region = original.clone();
        region.outputs = vec![original.outputs[0].clone(); 2].into();
        fail(region, ArithmeticError::Output { index: 1 });
        let mut region = original.clone();
        region.native = !region.native;
        fail(region, ArithmeticError::Proof);
        let mut region = original.clone();
        region.nodes = vec![
            N::Multiply(O::Input(0), O::Input(0)),
            N::Multiply(O::Value(0), O::Value(0)),
        ]
        .into();
        region.native = true;
        fail(region.clone(), ArithmeticError::Proof);
        region.native = false;
        assert_eq!(context.arithmetic(&region, &locals), Ok(()));
        let mut region = original.clone();
        let mut output = original.outputs[0].clone();
        output.slot.shape = crate::plan::execution::type_::ValueShapeId(99);
        region.outputs = vec![output].into();
        assert_eq!(
            context.arithmetic(&region, &locals),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 99
            }))
        );
        let mut region = original.clone();
        region.inputs = vec![IntLocalId(99)].into();
        assert_eq!(
            context.arithmetic(&region, &locals),
            Err(InstructionError::Local(LocalError::Missing(
                IntLocalId(99).into()
            )))
        );
    }
}
