use super::prepared::rust::{Emit, Rust};
use crate::plan::execution::function::{
    BoolFunctionId, CustomFunctionId, IntFunctionId, IntListFunctionId,
};
use crate::plan::execution::graph::BlockId;
use crate::plan::execution::storage::Table;
use crate::runtime::compiled::bit_array::BitArrayKernel;
use crate::runtime::compiled::int_list::IntListKernel;
use crate::runtime::compiled::numeric::NumericKernel;

/// Compiler-generated implementations, separate from the canonical graph.
///
/// An empty table explicitly selects interpreted execution. Admission checks
/// every supplied target and checkpoint before sealing the execution owner.
pub struct CompiledFunctions {
    pub ints: Table<CompiledFunction<IntFunctionId>>,
    pub bools: Table<CompiledFunction<BoolFunctionId>>,
    pub customs: Table<CompiledFunction<usize>>,
    pub int_lists: Table<CompiledFunction<IntListFunctionId>>,
}

pub struct CompiledFunction<Id> {
    pub function: Id,
    pub implementation: CompiledImplementation,
}

pub enum CompiledImplementation {
    Numeric(NumericImplementation),
    BitArray(BitArrayImplementation),
    IntList(IntListImplementation),
}

impl CompiledImplementation {
    pub(crate) fn entry(&self) -> usize {
        match self {
            Self::Numeric(value) => value.entry,
            Self::BitArray(value) => value.entry,
            Self::IntList(value) => value.entry,
        }
    }

    pub(crate) fn checkpoints(&self) -> &[CompiledCheckpoint] {
        match self {
            Self::Numeric(value) => &value.checkpoints,
            Self::BitArray(value) => &value.checkpoints,
            Self::IntList(value) => &value.checkpoints,
        }
    }
}

pub struct NumericImplementation {
    pub entry: usize,
    pub checkpoints: Table<CompiledCheckpoint>,
    pub run: NumericKernel,
}

pub struct BitArrayImplementation {
    pub entry: usize,
    pub checkpoints: Table<CompiledCheckpoint>,
    pub run: BitArrayKernel,
}

pub struct IntListImplementation {
    pub entry: usize,
    pub checkpoints: Table<CompiledCheckpoint>,
    pub run: IntListKernel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompiledCheckpoint {
    pub block: BlockId,
    pub instruction: usize,
    pub ints: usize,
    pub bools: usize,
    pub bit_arrays: usize,
    pub int_lists: usize,
}

impl CompiledFunctions {
    pub const fn interpreted() -> Self {
        Self {
            ints: Table::Static(&[]),
            bools: Table::Static(&[]),
            customs: Table::Static(&[]),
            int_lists: Table::Static(&[]),
        }
    }

    pub(in crate::plan::execution) fn borrowed(&'static self) -> Self {
        Self {
            ints: Table::Static(&self.ints),
            bools: Table::Static(&self.bools),
            customs: Table::Static(&self.customs),
            int_lists: Table::Static(&self.int_lists),
        }
    }

    pub(crate) fn int(&self, id: IntFunctionId) -> Option<&CompiledImplementation> {
        self.ints
            .binary_search_by_key(&id.0, |entry| entry.function.0)
            .ok()
            .map(|index| &self.ints[index].implementation)
    }

    pub(crate) fn bool(&self, id: BoolFunctionId) -> Option<&CompiledImplementation> {
        self.bools
            .binary_search_by_key(&id.0, |entry| entry.function.0)
            .ok()
            .map(|index| &self.bools[index].implementation)
    }

    pub(crate) fn custom(&self, id: CustomFunctionId) -> Option<&CompiledImplementation> {
        self.customs
            .binary_search_by_key(&id.index(), |entry| entry.function)
            .ok()
            .map(|index| &self.customs[index].implementation)
    }

    pub(crate) fn int_list(&self, id: IntListFunctionId) -> Option<&CompiledImplementation> {
        self.int_lists
            .binary_search_by_key(&(id.index, id.type_id.list_type.0), |entry| {
                (entry.function.index, entry.function.type_id.list_type.0)
            })
            .ok()
            .map(|index| &self.int_lists[index].implementation)
    }
}

impl Emit for CompiledCheckpoint {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::CompiledCheckpoint",
            &[
                ("block", &self.block),
                ("instruction", &self.instruction),
                ("ints", &self.ints),
                ("bools", &self.bools),
                ("bit_arrays", &self.bit_arrays),
                ("int_lists", &self.int_lists),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayImplementation, CompiledCheckpoint, CompiledFunction, CompiledFunctions,
        CompiledImplementation, IntListImplementation, NumericImplementation, Rust,
    };
    use crate::plan::execution::function::{
        BoolFunctionId, CustomFunctionId, IntFunctionId, IntListFunctionId,
    };
    use crate::plan::execution::graph::BlockId;
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, IntListTypeId, ListTypeId,
    };
    use crate::runtime::compiled::tests::{
        metadata_bit_array, metadata_int_list, metadata_numeric,
    };

    static FUNCTIONS: CompiledFunctions = CompiledFunctions {
        ints: Table::Static(&[CompiledFunction {
            function: IntFunctionId(2),
            implementation: CompiledImplementation::Numeric(NumericImplementation {
                entry: 0,
                checkpoints: Table::Static(&[]),
                run: metadata_numeric,
            }),
        }]),
        bools: Table::Static(&[CompiledFunction {
            function: BoolFunctionId(3),
            implementation: CompiledImplementation::IntList(IntListImplementation {
                entry: 1,
                checkpoints: Table::Static(&[]),
                run: metadata_int_list,
            }),
        }]),
        customs: Table::Static(&[CompiledFunction {
            function: 3,
            implementation: CompiledImplementation::BitArray(BitArrayImplementation {
                entry: 2,
                checkpoints: Table::Static(&[]),
                run: metadata_bit_array,
            }),
        }]),
        int_lists: Table::Static(&[CompiledFunction {
            function: IntListFunctionId {
                index: 5,
                type_id: IntListTypeId {
                    list_type: ListTypeId(2),
                },
            },
            implementation: CompiledImplementation::IntList(IntListImplementation {
                entry: 2,
                checkpoints: Table::Static(&[]),
                run: metadata_int_list,
            }),
        }]),
    };

    #[test]
    fn targets_keep_family_ids_and_borrow_the_complete_static_sidecar() {
        let borrowed = FUNCTIONS.borrowed();
        assert!(std::ptr::eq(
            borrowed.ints.as_ptr(),
            FUNCTIONS.ints.as_ptr()
        ));
        assert!(std::ptr::eq(
            borrowed.bools.as_ptr(),
            FUNCTIONS.bools.as_ptr()
        ));
        assert!(std::ptr::eq(
            borrowed.int_lists.as_ptr(),
            FUNCTIONS.int_lists.as_ptr()
        ));
        let list_id = FUNCTIONS.int_lists[0].function;
        assert!(std::ptr::eq(
            borrowed.int_list(list_id).unwrap(),
            &FUNCTIONS.int_lists[0].implementation
        ));
        assert!(
            borrowed
                .int_list(IntListFunctionId {
                    index: 4,
                    ..list_id
                })
                .is_none()
        );
        assert!(
            borrowed
                .int_list(IntListFunctionId {
                    type_id: IntListTypeId {
                        list_type: ListTypeId(3)
                    },
                    ..list_id
                })
                .is_none()
        );
        assert!(std::ptr::eq(
            borrowed.int(IntFunctionId(2)).unwrap(),
            &FUNCTIONS.ints[0].implementation
        ));
        assert!(std::ptr::eq(
            borrowed.bool(BoolFunctionId(3)).unwrap(),
            &FUNCTIONS.bools[0].implementation
        ));
        assert!(borrowed.int(IntFunctionId(3)).is_none());
        assert!(borrowed.bool(BoolFunctionId(2)).is_none());
        // Borrowing retains the kernel pointer and metadata without executing it.
        assert_eq!(borrowed.int(IntFunctionId(2)).unwrap().entry(), 0);
        assert_eq!(borrowed.bool(BoolFunctionId(3)).unwrap().entry(), 1);
        assert_eq!(borrowed.int(IntFunctionId(2)).unwrap().checkpoints(), []);
        assert_eq!(borrowed.bool(BoolFunctionId(3)).unwrap().checkpoints(), []);
        assert_eq!(borrowed.int_list(list_id).unwrap().entry(), 2);
        assert_eq!(borrowed.int_list(list_id).unwrap().checkpoints(), []);
        assert!(CompiledFunctions::interpreted().int_list(list_id).is_none());

        assert!(
            CompiledFunctions::interpreted()
                .int(IntFunctionId(2))
                .is_none()
        );
        assert!(
            CompiledFunctions::interpreted()
                .bool(BoolFunctionId(3))
                .is_none()
        );
    }

    #[test]
    fn custom_targets_use_the_index_without_changing_the_callers_return_shape() {
        let borrowed = FUNCTIONS.borrowed();
        assert!(std::ptr::eq(
            borrowed.customs.as_ptr(),
            FUNCTIONS.customs.as_ptr()
        ));
        let id = CustomFunctionId {
            index: 3,
            return_shape: CustomValueShape {
                type_id: CustomTypeId(1),
                shape_id: CustomValueShapeId(2),
            },
        };
        let target = borrowed.custom(id).unwrap();
        assert!(std::ptr::eq(target, &FUNCTIONS.customs[0].implementation));
        assert_eq!(id.return_shape.shape_id, CustomValueShapeId(2));
        assert_eq!(target.entry(), 2);
        assert_eq!(target.checkpoints(), []);
        assert!(borrowed.custom(id.with_index(2)).is_none());
        assert!(CompiledFunctions::interpreted().custom(id).is_none());
    }

    #[test]
    fn emits_the_exact_block_position_and_actual_typed_prefix() {
        assert_eq!(
            Rust::expression(&CompiledCheckpoint {
                block: BlockId(2),
                instruction: 3,
                ints: 5,
                bools: 1,
                bit_arrays: 3,
                int_lists: 2,
            }),
            r#"
data::compiled::CompiledCheckpoint {
    block: data::graph::BlockId(2),
    instruction: 3,
    ints: 5,
    bools: 1,
    bit_arrays: 3,
    int_lists: 2,
}
"#
            .trim_matches('\n')
        );
    }
}
