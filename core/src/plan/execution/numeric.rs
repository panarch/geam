use super::prepared::rust::{Emit, Rust};
use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
use crate::plan::execution::graph::BlockId;
use crate::plan::execution::storage::Table;
use crate::runtime::numeric::NumericKernel;

/// Compiler-generated implementations, separate from the canonical graph.
///
/// An empty table explicitly selects interpreted execution. Admission checks
/// every supplied target and checkpoint before sealing the execution owner.
pub struct NumericFunctions {
    pub ints: Table<NumericFunction<IntFunctionId>>,
    pub bools: Table<NumericFunction<BoolFunctionId>>,
}

pub struct NumericFunction<Id> {
    pub function: Id,
    pub implementation: NumericImplementation,
}

pub struct NumericImplementation {
    pub entry: usize,
    pub checkpoints: Table<NumericCheckpoint>,
    pub run: NumericKernel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericCheckpoint {
    pub block: BlockId,
    pub instruction: usize,
    pub ints: usize,
    pub bools: usize,
}

impl NumericFunctions {
    pub const fn interpreted() -> Self {
        Self {
            ints: Table::Static(&[]),
            bools: Table::Static(&[]),
        }
    }

    pub(in crate::plan::execution) fn borrowed(&'static self) -> Self {
        Self {
            ints: Table::Static(&self.ints),
            bools: Table::Static(&self.bools),
        }
    }

    pub(crate) fn int(&self, id: IntFunctionId) -> Option<&NumericImplementation> {
        self.ints
            .binary_search_by_key(&id.0, |entry| entry.function.0)
            .ok()
            .map(|index| &self.ints[index].implementation)
    }

    pub(crate) fn bool(&self, id: BoolFunctionId) -> Option<&NumericImplementation> {
        self.bools
            .binary_search_by_key(&id.0, |entry| entry.function.0)
            .ok()
            .map(|index| &self.bools[index].implementation)
    }
}

impl Emit for NumericCheckpoint {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "numeric::NumericCheckpoint",
            &[
                ("block", &self.block),
                ("instruction", &self.instruction),
                ("ints", &self.ints),
                ("bools", &self.bools),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        NumericCheckpoint, NumericFunction, NumericFunctions, NumericImplementation, Rust,
    };
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
    use crate::plan::execution::graph::BlockId;
    use crate::plan::execution::storage::Table;
    use crate::runtime::numeric::{NumericProgress, NumericValues};

    static FUNCTIONS: NumericFunctions = NumericFunctions {
        ints: Table::Static(&[NumericFunction {
            function: IntFunctionId(2),
            implementation: NumericImplementation {
                entry: 0,
                checkpoints: Table::Static(&[]),
                run: |point, _, _| NumericProgress::Yield(point),
            },
        }]),
        bools: Table::Static(&[NumericFunction {
            function: BoolFunctionId(3),
            implementation: NumericImplementation {
                entry: 1,
                checkpoints: Table::Static(&[]),
                run: |point, _, _| NumericProgress::Yield(point),
            },
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
            borrowed.int(IntFunctionId(2)).unwrap(),
            &FUNCTIONS.ints[0].implementation
        ));
        assert!(std::ptr::eq(
            borrowed.bool(BoolFunctionId(3)).unwrap(),
            &FUNCTIONS.bools[0].implementation
        ));
        assert!(borrowed.int(IntFunctionId(3)).is_none());
        assert!(borrowed.bool(BoolFunctionId(2)).is_none());
        // The sidecar borrows the actual Rust implementation, as well as its IDs.
        let mut values = NumericValues::default();
        let mut budget = 7;
        assert_eq!(
            (borrowed.int(IntFunctionId(2)).unwrap().run)(3, &mut values, &mut budget),
            NumericProgress::Yield(3)
        );
        assert_eq!(
            (borrowed.bool(BoolFunctionId(3)).unwrap().run)(4, &mut values, &mut budget),
            NumericProgress::Yield(4)
        );
        assert_eq!(budget, 7);

        assert!(
            NumericFunctions::interpreted()
                .int(IntFunctionId(2))
                .is_none()
        );
        assert!(
            NumericFunctions::interpreted()
                .bool(BoolFunctionId(3))
                .is_none()
        );
    }

    #[test]
    fn emits_the_exact_block_position_and_actual_numeric_prefix() {
        assert_eq!(
            Rust::expression(&NumericCheckpoint {
                block: BlockId(2),
                instruction: 3,
                ints: 5,
                bools: 1,
            }),
            "data::numeric::NumericCheckpoint {\n    block: data::graph::BlockId(2),\n    instruction: 3,\n    ints: 5,\n    bools: 1,\n}"
        );
    }
}
