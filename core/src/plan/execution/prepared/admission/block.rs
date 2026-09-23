use super::catalog::Function;
use super::type_::{TypeError, Types};
use crate::plan::execution::function::{ExecutionGraphProfile, FunctionEntry};
use crate::plan::execution::graph::{BlockId, BlockView, ProfiledBlockGraph};
use std::ops::Range;

pub(super) struct Blocks<'data, Graph: ExecutionGraphProfile> {
    raw: &'data ProfiledBlockGraph<Graph>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BlockError {
    Missing {
        index: usize,
    },
    ParameterRange {
        block: usize,
        range: Range<usize>,
        length: usize,
    },
    InstructionRange {
        block: usize,
        range: Range<usize>,
        length: usize,
    },
    ParameterOrder {
        block: usize,
        expected: usize,
        found: usize,
    },
    InstructionOrder {
        block: usize,
        expected: usize,
        found: usize,
    },
    UnclaimedParameters {
        claimed: usize,
        length: usize,
    },
    UnclaimedInstructions {
        claimed: usize,
        length: usize,
    },
    ParameterCount {
        expected: usize,
        found: usize,
    },
    CaptureCount {
        expected: usize,
        found: usize,
    },
    ParameterType {
        index: usize,
    },
    CaptureType {
        index: usize,
    },
    Type(TypeError),
}

impl<'data, Graph: ExecutionGraphProfile> Blocks<'data, Graph> {
    pub(super) fn admit(raw: &'data ProfiledBlockGraph<Graph>) -> Result<Self, BlockError> {
        if raw.entry.index() >= raw.blocks.len() {
            return Err(BlockError::Missing {
                index: raw.entry.index(),
            });
        }
        let mut params = 0;
        let mut instructions = 0;
        for (block, header) in raw.blocks.iter().enumerate() {
            raw.params
                .get(header.params.clone())
                .ok_or_else(|| BlockError::ParameterRange {
                    block,
                    range: header.params.clone(),
                    length: raw.params.len(),
                })?;
            raw.instructions
                .get(header.instructions.clone())
                .ok_or_else(|| BlockError::InstructionRange {
                    block,
                    range: header.instructions.clone(),
                    length: raw.instructions.len(),
                })?;
            if header.params.start != params {
                return Err(BlockError::ParameterOrder {
                    block,
                    expected: params,
                    found: header.params.start,
                });
            }
            if header.instructions.start != instructions {
                return Err(BlockError::InstructionOrder {
                    block,
                    expected: instructions,
                    found: header.instructions.start,
                });
            }
            params = header.params.end;
            instructions = header.instructions.end;
        }
        if params != raw.params.len() {
            return Err(BlockError::UnclaimedParameters {
                claimed: params,
                length: raw.params.len(),
            });
        }
        if instructions != raw.instructions.len() {
            return Err(BlockError::UnclaimedInstructions {
                claimed: instructions,
                length: raw.instructions.len(),
            });
        }
        Ok(Self { raw })
    }

    pub(super) fn block(&self, id: BlockId) -> Result<BlockView<'data, Graph>, BlockError> {
        self.find_block(id)
            .ok_or(BlockError::Missing { index: id.index() })
    }

    pub(super) fn find_block(&self, id: BlockId) -> Option<BlockView<'data, Graph>> {
        if id.index() >= self.raw.blocks.len() {
            return None;
        }
        Some(self.raw.block(id))
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = BlockView<'data, Graph>> {
        self.raw.blocks()
    }

    pub(super) fn entry(&self) -> crate::plan::execution::graph::BlockId {
        self.raw.entry
    }

    pub(super) fn entry_block(&self) -> BlockView<'data, Graph> {
        self.raw.block(self.raw.entry)
    }

    pub(super) fn entry_contract(
        &self,
        entry: &FunctionEntry,
        contract: &Function<'data>,
        types: &Types<'data>,
    ) -> Result<(), BlockError> {
        if entry.parameter_count != contract.parameters.len() {
            return Err(BlockError::ParameterCount {
                expected: contract.parameters.len(),
                found: entry.parameter_count,
            });
        }
        let params = self.entry_block().params();
        let (arguments, captures) =
            params
                .split_at_checked(entry.parameter_count)
                .ok_or(BlockError::ParameterCount {
                    expected: entry.parameter_count,
                    found: params.len(),
                })?;
        if captures.len() != contract.captures.len() {
            return Err(BlockError::CaptureCount {
                expected: contract.captures.len(),
                found: captures.len(),
            });
        }
        for (index, ((slot, expected), shape)) in arguments
            .iter()
            .zip(contract.parameters)
            .zip(contract.parameter_shapes)
            .enumerate()
        {
            types.slot(slot).map_err(BlockError::Type)?;
            if &slot.local != expected
                || !types.flow(slot.shape, *shape)
                || !types.flow(*shape, slot.shape)
            {
                return Err(BlockError::ParameterType { index });
            }
        }
        for (index, (slot, expected)) in captures.iter().zip(contract.captures).enumerate() {
            types.slot(slot).map_err(BlockError::Type)?;
            if slot.local != expected.local
                || !types.flow(slot.shape, expected.shape)
                || !types.flow(expected.shape, slot.shape)
            {
                return Err(BlockError::CaptureType { index });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockError, BlockId, Blocks, ProfiledBlockGraph, Range, Types};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockHeader, IntInstruction, IntLocalId, ParamLocal, ParamSlot,
        ProfiledInstruction, ProfiledInstructionKind, Terminator,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::ValueShapeId;
    use std::convert::Infallible;

    #[test]
    fn checks_entry_parameter_and_capture_contracts() {
        use super::super::catalog::Function;
        use super::super::type_::TypeError;
        use crate::plan::execution::function::FunctionEntry;
        use crate::plan::execution::graph::{BoolLocalId, ParamLocal};
        use crate::plan::execution::type_::{
            CustomTypeTable, ExternalTypeTable, ListTypeTable, ValueShapeDescriptor,
            ValueShapeTable, ValueType,
        };

        let lists = ListTypeTable::default();
        let customs = CustomTypeTable::new(Vec::new(), Vec::new());
        let externals = ExternalTypeTable::default();
        let shapes = ValueShapeTable {
            shapes: vec![ValueShapeDescriptor::Int, ValueShapeDescriptor::Bool].into(),
            shape_types: vec![ValueType::Int, ValueType::Bool].into(),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let parameter = ParamSlot::new(ParamLocal::Int(IntLocalId(0)), ValueShapeId(0));
        let capture = ParamSlot::new(ParamLocal::Int(IntLocalId(1)), ValueShapeId(0));
        let expected_parameters = [parameter.local.clone()];
        let expected_shapes = [ValueShapeId(0)];
        let expected_captures = [capture.clone()];
        let contract = Function {
            family: crate::plan::execution::function::FunctionTableFamily::Int,
            index: 0,
            parameters: &expected_parameters,
            parameter_shapes: &expected_shapes,
            return_: ValueShapeId(0),
            return_type: &ValueType::Int,
            return_shape: types.shape(ValueShapeId(0)).unwrap(),
            captures: &expected_captures,
        };
        let cases = [
            (1, vec![parameter.clone(), capture.clone()], Ok(())),
            (
                2,
                vec![parameter.clone(), capture.clone()],
                Err(BlockError::ParameterCount {
                    expected: 1,
                    found: 2,
                }),
            ),
            (
                1,
                Vec::new(),
                Err(BlockError::ParameterCount {
                    expected: 1,
                    found: 0,
                }),
            ),
            (
                1,
                vec![parameter.clone()],
                Err(BlockError::CaptureCount {
                    expected: 1,
                    found: 0,
                }),
            ),
            (
                1,
                vec![capture.clone(), capture.clone()],
                Err(BlockError::ParameterType { index: 0 }),
            ),
            (
                1,
                vec![parameter.clone(), parameter.clone()],
                Err(BlockError::CaptureType { index: 0 }),
            ),
            (
                1,
                vec![
                    ParamSlot::new(parameter.local.clone(), ValueShapeId(99)),
                    capture.clone(),
                ],
                Err(BlockError::Type(TypeError::MissingShape { index: 99 })),
            ),
            (
                1,
                vec![
                    parameter.clone(),
                    ParamSlot::new(capture.local.clone(), ValueShapeId(99)),
                ],
                Err(BlockError::Type(TypeError::MissingShape { index: 99 })),
            ),
            (
                1,
                vec![
                    ParamSlot::new(ParamLocal::Bool(BoolLocalId(0)), ValueShapeId(1)),
                    capture.clone(),
                ],
                Err(BlockError::ParameterType { index: 0 }),
            ),
            (
                1,
                vec![
                    parameter.clone(),
                    ParamSlot::new(ParamLocal::Bool(BoolLocalId(0)), ValueShapeId(1)),
                ],
                Err(BlockError::CaptureType { index: 0 }),
            ),
            (
                1,
                vec![
                    ParamSlot::new(parameter.local.clone(), ValueShapeId(1)),
                    capture.clone(),
                ],
                Err(BlockError::Type(TypeError::LocalTypeMismatch)),
            ),
            (
                1,
                vec![
                    parameter,
                    ParamSlot::new(capture.local.clone(), ValueShapeId(1)),
                ],
                Err(BlockError::Type(TypeError::LocalTypeMismatch)),
            ),
        ];
        for (parameter_count, params, expected) in cases {
            let raw = ProfiledBlockGraph::<Infallible> {
                entry: BlockId(0),
                blocks: vec![header(0..params.len(), 0..0)].into(),
                params: params.into(),
                instructions: Table::Static(&[]),
            };
            let blocks = Blocks::admit(&raw).unwrap();
            assert_eq!(blocks.entry(), BlockId(0));
            assert_eq!(
                blocks.entry_contract(&FunctionEntry { parameter_count }, &contract, &types),
                expected
            );
        }
    }

    fn header(params: Range<usize>, instructions: Range<usize>) -> BlockHeader {
        BlockHeader {
            params,
            instructions,
            terminator: Terminator::Exit(BlockGraphExitId(0)),
        }
    }

    fn graph() -> ProfiledBlockGraph<Infallible> {
        ProfiledBlockGraph {
            entry: BlockId(0),
            blocks: vec![header(0..0, 0..1)].into(),
            params: Table::Static(&[]),
            instructions: vec![ProfiledInstruction {
                output: ParamSlot {
                    local: ParamLocal::Int(IntLocalId(0)),
                    shape: ValueShapeId(0),
                },
                kind: ProfiledInstructionKind::Int(IntInstruction::Value(
                    num_bigint::BigInt::from(1).into(),
                )),
            }]
            .into(),
        }
    }

    #[test]
    fn checked_block_views_borrow_original_slices() {
        let raw = graph();
        let blocks = Blocks::admit(&raw).unwrap();
        let block = blocks.block(BlockId(0)).unwrap();
        assert!(std::ptr::eq(
            block.instructions(),
            raw.instructions.as_ref()
        ));
        assert_eq!(blocks.iter().len(), 1);
        assert_eq!(
            blocks.block(BlockId(1)).err(),
            Some(BlockError::Missing { index: 1 })
        );
    }

    #[test]
    fn rejects_invalid_ranges_order_and_unclaimed_records() {
        let mut raw = graph();
        raw.entry = BlockId(1);
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::Missing { index: 1 })
        );
        raw.entry = BlockId(0);
        raw.blocks = vec![header(0..1, 0..1)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::ParameterRange {
                block: 0,
                range: 0..1,
                length: 0
            })
        );
        raw.blocks = vec![header(0..0, 0..2)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::InstructionRange {
                block: 0,
                range: 0..2,
                length: 1
            })
        );
        raw.blocks = vec![header(0..0, 1..1)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::InstructionOrder {
                block: 0,
                expected: 0,
                found: 1
            })
        );
        raw.blocks = vec![header(0..0, 0..0)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::UnclaimedInstructions {
                claimed: 0,
                length: 1
            })
        );
        raw.params = vec![ParamSlot {
            local: ParamLocal::Int(IntLocalId(0)),
            shape: ValueShapeId(0),
        }]
        .into();
        raw.blocks = vec![header(1..1, 0..1)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::ParameterOrder {
                block: 0,
                expected: 0,
                found: 1
            })
        );
        raw.blocks = vec![header(0..0, 0..1)].into();
        assert_eq!(
            Blocks::admit(&raw).err(),
            Some(BlockError::UnclaimedParameters {
                claimed: 0,
                length: 1
            })
        );
    }
}
