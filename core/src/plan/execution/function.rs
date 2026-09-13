use crate::plan::execution::prepared::rust::{Emit, Rust};
pub(in crate::plan::execution) mod body;
pub(in crate::plan::execution) mod entry;
pub(in crate::plan::execution) mod function_return;
pub(in crate::plan::execution) mod list_return;
pub(in crate::plan::execution) mod parameters;
pub(in crate::plan::execution) mod profile;
pub(in crate::plan::execution) mod runtime;
pub(in crate::plan::execution) mod table;
pub(in crate::plan::execution) mod value_return;

use crate::plan::execution::explain::FunctionLabel;

pub(in crate::plan::execution) trait FunctionLabelSource {
    fn function_label(&self) -> FunctionLabel;
}

pub(crate) use body::FunctionBodyOwner;
pub(in crate::plan::execution) use body::TailCallLabelIndex;
pub(crate) use body::{FunctionBody, FunctionExit, ProfiledFunctionBody};
pub(crate) use entry::FunctionEntry;
pub(in crate::plan::execution) use function_return::FunctionFunctionTables;
pub(crate) use function_return::{
    BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId, BitArrayListFunctionFunctionId,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolListFunctionFunctionId,
    CoreListFunctionFunctionBody, CustomFunctionFunctionBody, CustomFunctionFunctionId,
    CustomListFunctionFunctionId, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBoolFunctionFunctionBody, ExecutionCoreListFunctionFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionExternalFunctionFunctionBody,
    ExecutionExternalListFunctionFunctionBody, ExecutionFloatFunctionFunctionBody,
    ExecutionFunctionFunctionFunctionBody, ExecutionGenericFunctionFunctionBody,
    ExecutionIntFunctionFunctionBody, ExecutionNeverFunctionFunctionBody,
    ExecutionNilFunctionFunctionBody, ExecutionStringFunctionFunctionBody,
    ExecutionTupleFunctionFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    ExternalFunctionFunctionBody, ExternalFunctionFunctionId, ExternalListFunctionFunctionBody,
    ExternalListFunctionFunctionId, FloatFunctionFunctionBody, FloatFunctionFunctionId,
    FloatListFunctionFunctionId, FunctionFunctionFunctionBody, FunctionFunctionFunctionId,
    FunctionFunctionId, FunctionListFunctionFunctionId, GenericFunctionFunctionBody,
    GenericFunctionFunctionId, IntFunctionFunctionBody, IntFunctionFunctionId,
    IntListFunctionFunctionId, ListFunctionFunctionId, ListListFunctionFunctionId,
    NeverFunctionFunctionBody, NeverFunctionFunctionId, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilListFunctionFunctionId, ParameterListFunctionFunctionId,
    ParameterListListFunctionFunctionId, ProfiledCustomFunctionFunctionBody,
    ProfiledFunctionFunctionFunctionBody, ProfiledFunctionFunctionId,
    ProfiledListFunctionFunctionId, StringFunctionFunctionBody, StringFunctionFunctionId,
    StringListFunctionFunctionId, TupleFunctionFunctionBody, TupleFunctionFunctionId,
    TupleListFunctionFunctionId, TypedFunctionBody, UtfCodepointFunctionFunctionBody,
    UtfCodepointFunctionFunctionId, UtfCodepointListFunctionFunctionId,
};
pub(in crate::plan::execution) use list_return::ListFunctionTables;
pub(crate) use list_return::{
    BitArrayListFunctionBody, BitArrayListFunctionId, BoolListFunctionBody, BoolListFunctionId,
    CustomListFunctionBody, CustomListFunctionId, ExecutionBitArrayListFunctionBody,
    ExecutionBoolListFunctionBody, ExecutionCustomListFunctionBody,
    ExecutionExternalListFunctionBody, ExecutionFloatListFunctionBody,
    ExecutionFunctionListFunctionBody, ExecutionIntListFunctionBody, ExecutionListListFunctionBody,
    ExecutionNilListFunctionBody, ExecutionParameterListFunctionBody,
    ExecutionParameterListListFunctionBody, ExecutionStringListFunctionBody,
    ExecutionTupleListFunctionBody, ExecutionUtfCodepointListFunctionBody,
    ExternalListFunctionBody, ExternalListFunctionId, FloatListFunctionBody, FloatListFunctionId,
    FunctionListFunctionBody, FunctionListFunctionId, IntListFunctionBody, IntListFunctionId,
    LibraryListFunctionId, ListFunctionId, ListListFunctionBody, ListListFunctionId,
    NilListFunctionBody, NilListFunctionId, ParameterListFunctionBody, ParameterListFunctionId,
    ParameterListListFunctionBody, ParameterListListFunctionId, ProfiledListFunctionId,
    RuntimeListFunctionId, StringListFunctionBody, StringListFunctionId, TupleListFunctionBody,
    TupleListFunctionId, UtfCodepointListFunctionBody, UtfCodepointListFunctionId,
};
pub(crate) use parameters::{FunctionCatalog, FunctionParameterView, FunctionTableFamily};
pub(crate) use profile::{
    ExecutionFunction, ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef,
    ExecutionGraphProfile, ExecutionHostTarget, ExecutionNeverFunction, ExecutionNeverHostTarget,
    ExecutionProfile, HostedExecutionGraph,
};
pub(crate) use runtime::{
    CoreRuntimeFunctionId, FunctionReturnFamily, GenericCallableId, ProfiledCoreRuntimeFunctionId,
    ProfiledRuntimeFunctionId, RuntimeFunctionFunctionTarget, RuntimeFunctionId,
};
pub(super) use table::FunctionTables;
pub(in crate::plan::execution) use table::HostedFunctionTablesExplanation;
pub(in crate::plan::execution::function) use table::write_table;
pub(in crate::plan::execution) use value_return::ValueFunctionTables;
pub(crate) use value_return::{
    BitArrayFunctionBody, BitArrayFunctionId, BoolFunctionBody, BoolFunctionId, CustomFunctionBody,
    CustomFunctionId, ExecutionBitArrayFunctionBody, ExecutionBoolFunctionBody,
    ExecutionCustomFunctionBody, ExecutionExternalFunctionBody, ExecutionFloatFunctionBody,
    ExecutionIntFunctionBody, ExecutionNeverFunctionBody, ExecutionNilFunctionBody,
    ExecutionStringFunctionBody, ExecutionTupleFunctionBody, ExecutionUtfCodepointFunctionBody,
    ExternalFunctionBody, ExternalFunctionId, FloatFunctionBody, FloatFunctionId, IntFunctionBody,
    IntFunctionId, NeverFunctionBody, NeverFunctionId, NilFunctionBody, NilFunctionId,
    ProfiledCustomFunctionBody, StringFunctionBody, StringFunctionId, TupleFunctionBody,
    TupleFunctionId, UtfCodepointFunctionBody, UtfCodepointFunctionId, ValueFunctionEntry,
};

pub struct ExecutableFunction<Body> {
    pub entry: FunctionEntry,
    pub body: Body,
}

impl<Body> ExecutableFunction<Body> {
    pub(super) fn new(parameter_count: usize, body: Body) -> Self {
        Self {
            entry: FunctionEntry::new(parameter_count),
            body,
        }
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }

    pub(crate) fn entry(&self) -> &FunctionEntry {
        &self.entry
    }

    pub(in crate::plan::execution) fn from_parts(entry: FunctionEntry, body: Body) -> Self {
        Self { entry, body }
    }

    pub(in crate::plan::execution) fn into_parts(self) -> (FunctionEntry, Body) {
        (self.entry, self.body)
    }
}

impl<Body> Emit for ExecutableFunction<Body>
where
    Body: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self { entry, body } = self;
        output.structure(
            "function::ExecutableFunction",
            &[("entry", entry), ("body", body)],
        );
    }
}
