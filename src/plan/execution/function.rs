mod body;
mod entry;
mod function_return;
mod list_return;
mod profile;
mod runtime;
mod table;
mod value_return;

use crate::plan::execution::explain::FunctionLabel;

pub(in crate::plan::execution) trait FunctionLabelSource {
    fn function_label(&self) -> FunctionLabel;
}

pub(crate) use body::FunctionBodyOwner;
pub(in crate::plan::execution) use body::TailCallLabelIndex;
pub(crate) use body::{FunctionBody, FunctionExit};
pub(crate) use entry::FunctionEntry;
pub(in crate::plan::execution) use function_return::FunctionFunctionTables;
pub(crate) use function_return::{
    BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId, BitArrayListFunctionFunctionId,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolListFunctionFunctionId,
    CustomFunctionFunctionBody, CustomFunctionFunctionId, CustomListFunctionFunctionId,
    FloatFunctionFunctionBody, FloatFunctionFunctionId, FloatListFunctionFunctionId,
    FunctionFunctionFunctionBody, FunctionFunctionFunctionId, FunctionFunctionId,
    FunctionListFunctionFunctionId, GenericFunctionFunctionBody, GenericFunctionFunctionId,
    IntFunctionFunctionBody, IntFunctionFunctionId, IntListFunctionFunctionId,
    ListFunctionFunctionBody, ListFunctionFunctionId, ListListFunctionFunctionId,
    NeverFunctionFunctionBody, NeverFunctionFunctionId, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilListFunctionFunctionId, ParameterListFunctionFunctionId,
    ParameterListListFunctionFunctionId, StringFunctionFunctionBody, StringFunctionFunctionId,
    StringListFunctionFunctionId, TupleFunctionFunctionBody, TupleFunctionFunctionId,
    TupleListFunctionFunctionId, TypedFunctionBody, UtfCodepointFunctionFunctionBody,
    UtfCodepointFunctionFunctionId, UtfCodepointListFunctionFunctionId,
};
pub(in crate::plan::execution) use list_return::ListFunctionTables;
pub(crate) use list_return::{
    BitArrayListFunctionBody, BitArrayListFunctionId, BoolListFunctionBody, BoolListFunctionId,
    CustomListFunctionBody, CustomListFunctionId, FloatListFunctionBody, FloatListFunctionId,
    FunctionListFunctionBody, FunctionListFunctionId, IntListFunctionBody, IntListFunctionId,
    ListFunctionId, ListListFunctionBody, ListListFunctionId, NilListFunctionBody,
    NilListFunctionId, ParameterListFunctionBody, ParameterListFunctionId,
    ParameterListListFunctionBody, ParameterListListFunctionId, StringListFunctionBody,
    StringListFunctionId, TupleListFunctionBody, TupleListFunctionId, UtfCodepointListFunctionBody,
    UtfCodepointListFunctionId,
};
pub(crate) use profile::{
    ExecutionFunction, ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionRef,
    ExecutionHostTarget, ExecutionProfile,
};
pub(crate) use runtime::{FunctionReturnFamily, GenericCallableId, RuntimeFunctionId};
pub(super) use table::FunctionTables;
pub(in crate::plan::execution) use table::HostedFunctionTablesExplanation;
pub(in crate::plan::execution::function) use table::write_table;
pub(in crate::plan::execution) use value_return::ValueFunctionTables;
pub(crate) use value_return::{
    BitArrayFunctionBody, BitArrayFunctionId, BoolFunctionBody, BoolFunctionId, CustomFunctionBody,
    CustomFunctionId, FloatFunctionBody, FloatFunctionId, IntFunctionBody, IntFunctionId,
    NeverFunctionBody, NeverFunctionId, NilFunctionBody, NilFunctionId, StringFunctionBody,
    StringFunctionId, TupleFunctionBody, TupleFunctionId, UtfCodepointFunctionBody,
    UtfCodepointFunctionId, ValueFunctionEntry,
};

pub(crate) struct ExecutableFunction<Body> {
    entry: FunctionEntry,
    body: Body,
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
}
