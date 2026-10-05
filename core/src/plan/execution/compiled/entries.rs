use super::{CallTarget, CompiledFunctions, CompiledImplementation, NativeLoopTarget};
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId, ExecutionProfile, FloatFunctionId,
    FunctionTables, IntFunctionFunctionId, IntFunctionId, NilFunctionId, StringFunctionId,
    UtfCodepointFunctionId,
};

/// Execution views derived once from admitted static implementations. Each
/// slot borrows its original implementation; no graph or contract is copied.
#[derive(Default)]
pub(in crate::plan::execution) struct CompiledEntries {
    ints: Vec<Option<&'static CompiledImplementation>>,
    floats: Vec<Option<&'static CompiledImplementation>>,
    strings: Vec<Option<&'static CompiledImplementation>>,
    bit_arrays: Vec<Option<&'static CompiledImplementation>>,
    utf_codepoints: Vec<Option<&'static CompiledImplementation>>,
    nils: Vec<Option<&'static CompiledImplementation>>,
    bools: Vec<Option<&'static CompiledImplementation>>,
    int_functions: Vec<Option<&'static CompiledImplementation>>,
    bool_functions: Vec<Option<&'static CompiledImplementation>>,
}

impl CompiledEntries {
    pub(in crate::plan::execution) fn new<Profile: ExecutionProfile>(
        compiled: &'static CompiledFunctions,
        functions: &FunctionTables<Profile>,
    ) -> Self {
        if compiled.ints.is_empty()
            && compiled.bools.is_empty()
            && compiled.function_calls.is_empty()
            && compiled.native_loops.is_empty()
        {
            return Self::default();
        }
        Self {
            ints: (0..functions.value_returns.int_functions.len())
                .map(|index| compiled.int(IntFunctionId(index)))
                .collect(),
            bools: (0..functions.value_returns.bool_functions.len())
                .map(|index| compiled.bool(BoolFunctionId(index)))
                .collect(),
            floats: if compiled.native_loops.is_empty() {
                Vec::new()
            } else {
                (0..functions.value_returns.float_functions.len())
                    .map(|index| {
                        compiled.native_loop(NativeLoopTarget::Float(FloatFunctionId(index)))
                    })
                    .collect()
            },
            strings: if compiled.native_loops.is_empty() {
                Vec::new()
            } else {
                (0..functions.value_returns.string_functions.len())
                    .map(|index| {
                        compiled.native_loop(NativeLoopTarget::String(StringFunctionId(index)))
                    })
                    .collect()
            },
            bit_arrays: if compiled.native_loops.is_empty() {
                Vec::new()
            } else {
                (0..functions.value_returns.bit_array_functions.len())
                    .map(|index| {
                        compiled.native_loop(NativeLoopTarget::BitArray(BitArrayFunctionId(index)))
                    })
                    .collect()
            },
            utf_codepoints: if compiled.native_loops.is_empty() {
                Vec::new()
            } else {
                (0..functions.value_returns.utf_codepoint_functions.len())
                    .map(|index| {
                        compiled.native_loop(NativeLoopTarget::UtfCodepoint(
                            UtfCodepointFunctionId(index),
                        ))
                    })
                    .collect()
            },
            nils: if compiled.native_loops.is_empty() {
                Vec::new()
            } else {
                (0..functions.value_returns.nil_functions.len())
                    .map(|index| compiled.native_loop(NativeLoopTarget::Nil(NilFunctionId(index))))
                    .collect()
            },
            int_functions: (0..functions.function_returns.int_function_functions.len())
                .map(|index| {
                    compiled.call_root(CallTarget::IntFunction(IntFunctionFunctionId(index)))
                })
                .collect(),
            bool_functions: (0..functions.function_returns.bool_function_functions.len())
                .map(|index| {
                    compiled.call_root(CallTarget::BoolFunction(BoolFunctionFunctionId(index)))
                })
                .collect(),
        }
    }

    pub(in crate::plan::execution) fn int(
        &self,
        id: IntFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.ints.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn bool(
        &self,
        id: BoolFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.bools.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn float(
        &self,
        id: FloatFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.floats.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn string(
        &self,
        id: StringFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.strings.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn bit_array(
        &self,
        id: BitArrayFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.bit_arrays.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn utf_codepoint(
        &self,
        id: UtfCodepointFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.utf_codepoints.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn nil(
        &self,
        id: NilFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.nils.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn int_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.int_functions.get(id.0).copied().flatten()
    }

    pub(in crate::plan::execution) fn bool_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> Option<&CompiledImplementation> {
        self.bool_functions.get(id.0).copied().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::CompiledEntries;
    use crate::plan::execution::compiled::{
        CompiledCallbacks, CompiledFunction, CompiledFunctions, CompiledImplementation,
        NumericImplementation,
    };
    use crate::plan::execution::function::{
        BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
    };
    use crate::plan::execution::storage::Table;
    use crate::runtime::compiled::tests::metadata_numeric;

    #[test]
    fn legacy_entries_borrow_original_implementations_and_leave_unsupported_slots_empty() {
        static COMPILED: CompiledFunctions = CompiledFunctions {
            ints: Table::Static(&[CompiledFunction {
                function: IntFunctionId(0),
                implementation: CompiledImplementation::Numeric(NumericImplementation {
                    entry: 0,
                    checkpoints: Table::Static(&[]),
                    run: metadata_numeric,
                }),
            }]),
            bools: Table::Static(&[CompiledFunction {
                function: BoolFunctionId(0),
                implementation: CompiledImplementation::Numeric(NumericImplementation {
                    entry: 1,
                    checkpoints: Table::Static(&[]),
                    run: metadata_numeric,
                }),
            }]),
            customs: Table::Static(&[]),
            int_lists: Table::Static(&[]),
            native_loops: Table::Static(&[]),
            function_calls: Table::Static(&[]),
            callbacks: CompiledCallbacks::interpreted(),
        };
        let source = r#"
fn number(value: Int) { value }
fn other_number(value: Int) { value + 1 }
fn predicate(value: Bool) { value }
fn other_predicate(value: Bool) { !value }
pub fn main() { #(number(42), other_number(4), predicate(True), other_predicate(False)) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let entries = CompiledEntries::new(&COMPILED, &plan.program.functions);
        assert_eq!(entries.ints.len(), 2);
        assert_eq!(entries.bools.len(), 2);
        assert!(std::ptr::eq(
            entries.int(IntFunctionId(0)).unwrap(),
            &COMPILED.ints[0].implementation
        ));
        assert!(std::ptr::eq(
            entries.bool(BoolFunctionId(0)).unwrap(),
            &COMPILED.bools[0].implementation
        ));
        assert!(entries.int(IntFunctionId(1)).is_none());
        assert!(entries.bool(BoolFunctionId(1)).is_none());
        assert!(entries.int(IntFunctionId(2)).is_none());
        assert!(entries.bool(BoolFunctionId(2)).is_none());
        assert!(entries.int_function(IntFunctionFunctionId(0)).is_none());
        assert!(entries.bool_function(BoolFunctionFunctionId(0)).is_none());

        static INTERPRETED: CompiledFunctions = CompiledFunctions::interpreted();
        let interpreted = CompiledEntries::new(&INTERPRETED, &plan.program.functions);
        assert!(interpreted.ints.is_empty());
        assert!(interpreted.bools.is_empty());
        assert!(interpreted.int_functions.is_empty());
        assert!(interpreted.bool_functions.is_empty());
        assert!(interpreted.int(IntFunctionId(0)).is_none());
        assert!(interpreted.bool(BoolFunctionId(0)).is_none());
        assert!(interpreted.int_function(IntFunctionFunctionId(0)).is_none());
        assert!(
            interpreted
                .bool_function(BoolFunctionFunctionId(0))
                .is_none()
        );
    }
}
