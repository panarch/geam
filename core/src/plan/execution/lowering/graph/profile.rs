use super::super::specialization::Representability;
use crate::plan::execution::function::{
    HostedExecutionGraph, ProfiledCoreRuntimeFunctionId, ProfiledListFunctionId,
    ProfiledRuntimeFunctionId, RuntimeFunctionFunctionTarget,
};
use crate::plan::execution::graph::{
    ProfiledBlock, ProfiledBlockGraph, ProfiledInstruction, ProfiledInstructionKind,
};
use std::convert::Infallible;

pub(in crate::plan::execution::lowering) fn seal_plain_block_graph(
    graph: ProfiledBlockGraph<HostedExecutionGraph>,
) -> Representability<ProfiledBlockGraph<Infallible>> {
    let (entry, blocks) = graph.into_parts();
    Representability::collect(blocks.into_vec().into_iter().map(seal_plain_block))
        .map(|blocks| ProfiledBlockGraph::from_parts(entry, blocks))
}

pub(in crate::plan::execution::lowering) fn seal_plain_runtime_function_id(
    function: ProfiledRuntimeFunctionId<HostedExecutionGraph>,
) -> Representability<ProfiledRuntimeFunctionId<Infallible>> {
    match function {
        ProfiledRuntimeFunctionId::External(_) => Representability::Uninhabited,
        ProfiledRuntimeFunctionId::Core(function) => {
            seal_plain_core_runtime_function_id(function).map(ProfiledRuntimeFunctionId::Core)
        }
    }
}

fn seal_plain_core_runtime_function_id(
    function: ProfiledCoreRuntimeFunctionId<HostedExecutionGraph>,
) -> Representability<ProfiledCoreRuntimeFunctionId<Infallible>> {
    match function {
        ProfiledCoreRuntimeFunctionId::Never(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Never(function))
        }
        ProfiledCoreRuntimeFunctionId::Int(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Int(function))
        }
        ProfiledCoreRuntimeFunctionId::Float(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Float(function))
        }
        ProfiledCoreRuntimeFunctionId::String(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::String(function))
        }
        ProfiledCoreRuntimeFunctionId::BitArray(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::BitArray(function))
        }
        ProfiledCoreRuntimeFunctionId::UtfCodepoint(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::UtfCodepoint(function))
        }
        ProfiledCoreRuntimeFunctionId::Custom(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Custom(function))
        }
        ProfiledCoreRuntimeFunctionId::Bool(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Bool(function))
        }
        ProfiledCoreRuntimeFunctionId::Nil(function) => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Nil(function))
        }
        ProfiledCoreRuntimeFunctionId::Tuple { id, return_type } => {
            Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Tuple { id, return_type })
        }
        ProfiledCoreRuntimeFunctionId::List(function) => {
            seal_plain_list_function_id(function).map(ProfiledCoreRuntimeFunctionId::List)
        }
        ProfiledCoreRuntimeFunctionId::Function { id, return_type } => match id {
            RuntimeFunctionFunctionTarget::Core(id) => {
                Representability::Inhabited(ProfiledCoreRuntimeFunctionId::Function {
                    id,
                    return_type,
                })
            }
            RuntimeFunctionFunctionTarget::External(_) => Representability::Uninhabited,
        },
    }
}

fn seal_plain_block(
    block: ProfiledBlock<HostedExecutionGraph>,
) -> Representability<ProfiledBlock<Infallible>> {
    let (params, instructions, terminator) = block.into_parts();
    Representability::collect(
        instructions
            .into_vec()
            .into_iter()
            .map(seal_plain_instruction),
    )
    .map(|instructions| ProfiledBlock::new(params.into_vec(), instructions, terminator))
}

fn seal_plain_instruction(
    instruction: ProfiledInstruction<HostedExecutionGraph>,
) -> Representability<ProfiledInstruction<Infallible>> {
    let (output, kind) = instruction.into_parts();
    seal_plain_instruction_kind(kind).map(|kind| ProfiledInstruction::new(output, kind))
}

fn seal_plain_instruction_kind(
    instruction: ProfiledInstructionKind<HostedExecutionGraph>,
) -> Representability<ProfiledInstructionKind<Infallible>> {
    match instruction {
        ProfiledInstructionKind::Int(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Int(instruction))
        }
        ProfiledInstructionKind::Float(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Float(instruction))
        }
        ProfiledInstructionKind::String(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::String(instruction))
        }
        ProfiledInstructionKind::BitArray(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::BitArray(instruction))
        }
        ProfiledInstructionKind::UtfCodepoint(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::UtfCodepoint(instruction))
        }
        ProfiledInstructionKind::Custom(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Custom(instruction))
        }
        ProfiledInstructionKind::External(_)
        | ProfiledInstructionKind::ExternalList(_)
        | ProfiledInstructionKind::ExternalFunction(_) => Representability::Uninhabited,
        ProfiledInstructionKind::Bool(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Bool(instruction))
        }
        ProfiledInstructionKind::Nil(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Nil(instruction))
        }
        ProfiledInstructionKind::Tuple(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Tuple(instruction))
        }
        ProfiledInstructionKind::List(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::List(instruction))
        }
        ProfiledInstructionKind::Function(instruction) => {
            Representability::Inhabited(ProfiledInstructionKind::Function(instruction))
        }
    }
}

pub(in crate::plan::execution::lowering) fn seal_plain_list_function_id(
    function: ProfiledListFunctionId<HostedExecutionGraph>,
) -> Representability<ProfiledListFunctionId<Infallible>> {
    match function {
        ProfiledListFunctionId::Core(function) => {
            Representability::Inhabited(ProfiledListFunctionId::Core(function))
        }
        ProfiledListFunctionId::External(_) => Representability::Uninhabited,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Representability, seal_plain_block_graph, seal_plain_instruction_kind,
        seal_plain_runtime_function_id,
    };
    use crate::plan::execution::function::{
        ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionFunctionId,
        ExternalListFunctionId, FunctionReturnFamily, HostedExecutionGraph,
        ProfiledCoreRuntimeFunctionId, ProfiledListFunctionId, ProfiledRuntimeFunctionId,
        RuntimeFunctionFunctionTarget,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, ExternalFunctionCallTarget, ExternalFunctionInstruction,
        ExternalFunctionInstructionKind, ExternalFunctionTarget, ExternalInstruction,
        ExternalListInstruction, ExternalLocal, ExternalLocalId, ParamLocal, ParamSlot,
        ProfiledBlock, ProfiledBlockGraph, ProfiledInstruction, ProfiledInstructionKind,
        Terminator, TupleLocalId, TypedListInstruction,
    };
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalListTypeId, ExternalTypeId, FunctionType, ListTypeId,
        ValueShapeId, ValueType,
    };
    use std::convert::Infallible;

    #[test]
    fn plain_graph_sealing_rejects_external_nodes() {
        let external_type = ExternalTypeId::new(0);
        let list_type = ExternalListTypeId::new(ListTypeId::new(1), external_type);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let graph = ProfiledBlockGraph::<HostedExecutionGraph>::from_parts(
            BlockId::new(0),
            vec![ProfiledBlock::new(
                Vec::new(),
                vec![ProfiledInstruction::new(
                    ParamSlot::new(
                        ParamLocal::External(ExternalLocal::new(ExternalLocalId(0), external_type)),
                        ValueShapeId::new(0),
                    ),
                    ProfiledInstructionKind::External(ExternalInstruction::TupleIndex {
                        tuple: TupleLocalId(0),
                        index: 1,
                    }),
                )],
                Terminator::Exit(BlockGraphExitId::new(0)),
            )],
        );

        assert_eq!(
            std::mem::discriminant(&seal_plain_block_graph(graph)),
            std::mem::discriminant(
                &Representability::<ProfiledBlockGraph<Infallible>>::Uninhabited
            ),
        );
        assert_eq!(
            std::mem::discriminant(&seal_plain_instruction_kind(
                ProfiledInstructionKind::External(ExternalInstruction::TupleIndex {
                    tuple: TupleLocalId(6),
                    index: 7,
                },)
            )),
            std::mem::discriminant(
                &Representability::<ProfiledInstructionKind<Infallible>>::Uninhabited
            ),
        );
        assert_eq!(
            std::mem::discriminant(&seal_plain_instruction_kind(
                ProfiledInstructionKind::ExternalList(ExternalListInstruction::new(
                    list_type,
                    TypedListInstruction::Value(Box::new([])),
                ),)
            )),
            std::mem::discriminant(
                &Representability::<ProfiledInstructionKind<Infallible>>::Uninhabited
            ),
        );
        assert_eq!(
            std::mem::discriminant(&seal_plain_instruction_kind(
                ProfiledInstructionKind::ExternalFunction(ExternalFunctionInstruction::new(
                    function_type,
                    FunctionReturnFamily::External,
                    ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(
                        ExternalFunctionId::new(2, external_type),
                    )),
                ),)
            )),
            std::mem::discriminant(
                &Representability::<ProfiledInstructionKind<Infallible>>::Uninhabited
            ),
        );
    }

    #[test]
    fn plain_runtime_function_sealing_rejects_external_targets() {
        let external_type = ExternalTypeId::new(0);
        let list_type = ExternalListTypeId::new(ListTypeId::new(1), external_type);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));

        assert_eq!(
            seal_plain_runtime_function_id(ProfiledRuntimeFunctionId::External(
                ExternalFunctionId::new(1, external_type),
            )),
            Representability::Uninhabited,
        );
        assert_eq!(
            seal_plain_runtime_function_id(ProfiledRuntimeFunctionId::Core(
                ProfiledCoreRuntimeFunctionId::List(ProfiledListFunctionId::External(
                    ExternalListFunctionId::new(2, list_type),
                )),
            )),
            Representability::Uninhabited,
        );
        assert_eq!(
            seal_plain_runtime_function_id(ProfiledRuntimeFunctionId::Core(
                ProfiledCoreRuntimeFunctionId::Function {
                    id: RuntimeFunctionFunctionTarget::External(
                        ExternalFunctionCallTarget::Function(ExternalFunctionFunctionId::new(
                            3,
                            ExternalFunctionType::from_shapes(
                                function_type.clone(),
                                Vec::new(),
                                external_type,
                            ),
                        ),)
                    ),
                    return_type: function_type.clone(),
                },
            )),
            Representability::Uninhabited,
        );
        assert_eq!(
            seal_plain_runtime_function_id(ProfiledRuntimeFunctionId::Core(
                ProfiledCoreRuntimeFunctionId::Function {
                    id: RuntimeFunctionFunctionTarget::External(
                        ExternalFunctionCallTarget::ListFunction {
                            id: ExternalListFunctionFunctionId(4),
                            type_: function_type.clone(),
                            list_type,
                        },
                    ),
                    return_type: function_type,
                },
            )),
            Representability::Uninhabited,
        );
    }
}
