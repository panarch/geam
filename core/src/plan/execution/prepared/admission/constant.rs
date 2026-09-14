mod body;

pub(super) use body::{ConstantBodyError, all};

use super::type_::{TypeError, Types};
use crate::plan::execution::constant::{ConstantId, ConstantValue, ProfiledConstantTable};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::ParamSlot;
use crate::plan::execution::type_::ValueShapeId;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ConstantError {
    Missing { index: usize },
    Type(TypeError),
    ReturnType,
}

pub(super) fn reference<Return: ConstantValue, Graph: ExecutionGraphProfile>(
    table: &ProfiledConstantTable<Graph>,
    id: ConstantId<Return>,
    output: &ParamSlot,
    types: &Types<'_>,
) -> Result<(), ConstantError> {
    let program = Return::programs(table)
        .get(id.index())
        .ok_or(ConstantError::Missing { index: id.index() })?;
    result_type(program.shape, output, types)
}

fn result_type(
    shape: ValueShapeId,
    output: &ParamSlot,
    types: &Types<'_>,
) -> Result<(), ConstantError> {
    types.slot(output).map_err(ConstantError::Type)?;
    if !types
        .can_flow(shape, output.shape)
        .map_err(ConstantError::Type)?
    {
        return Err(ConstantError::ReturnType);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ConstantError, ConstantId, ParamSlot, TypeError, Types, reference};
    use crate::plan::execution::graph::{
        IntInstruction, IntLocalId, ParamLocal, ProfiledInstructionKind,
    };
    use crate::plan::execution::type_::ValueShapeId;

    #[test]
    fn checks_constant_references_against_the_preserved_contract_without_execution() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "const saved = 42 pub fn main() { #(True, saved) }",
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
        let function = &plan.program.functions.value_returns.tuple_functions[0];
        let block = function
            .body()
            .block_graph()
            .block(function.body().block_graph().entry);
        let instruction = block
            .instructions()
            .iter()
            .find(|instruction| {
                matches!(
                    instruction.kind(),
                    ProfiledInstructionKind::Int(IntInstruction::Constant(_))
                )
            })
            .unwrap();
        reference(
            &common.constants,
            ConstantId::<IntLocalId>::new(0),
            instruction.output(),
            &types,
        )
        .unwrap();
        assert_eq!(
            reference(
                &common.constants,
                ConstantId::<IntLocalId>::new(1),
                instruction.output(),
                &types
            ),
            Err(ConstantError::Missing { index: 1 })
        );
        let boolean = block
            .instructions()
            .iter()
            .find(|instruction| matches!(instruction.output().local, ParamLocal::Bool(_)))
            .unwrap();
        assert_eq!(
            reference(
                &common.constants,
                ConstantId::<IntLocalId>::new(0),
                boolean.output(),
                &types
            ),
            Err(ConstantError::ReturnType)
        );
        let invalid = ParamSlot::new(ParamLocal::Int(IntLocalId(0)), ValueShapeId(usize::MAX));
        assert_eq!(
            reference(
                &common.constants,
                ConstantId::<IntLocalId>::new(0),
                &invalid,
                &types
            ),
            Err(ConstantError::Type(TypeError::MissingShape {
                index: usize::MAX
            }))
        );
        assert_eq!(
            super::result_type(ValueShapeId(999), instruction.output(), &types),
            Err(ConstantError::Type(TypeError::MissingShape { index: 999 }))
        );
    }
}
