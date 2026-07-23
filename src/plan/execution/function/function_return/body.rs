use super::super::graph::FunctionGraph;
use super::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, ListFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    StringFunctionFunctionId, TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::plan::execution::{
    BitArrayFunctionLocalId, BoolFunctionLocalId, CustomFunctionLocal, CustomFunctionType,
    FloatFunctionLocalId, FunctionFunctionLocal, FunctionFunctionType, FunctionShape,
    GenericFunctionLocal, IntFunctionLocalId, ListFunctionLocal, NeverFunctionLocal,
    NilFunctionLocalId, StringFunctionLocalId, TupleFunctionLocalId, UtfCodepointFunctionLocalId,
};

pub(crate) type IntFunctionReturn =
    TypedFunctionReturn<FunctionGraph<IntFunctionLocalId, IntFunctionFunctionId>>;
pub(crate) type FloatFunctionReturn =
    TypedFunctionReturn<FunctionGraph<FloatFunctionLocalId, FloatFunctionFunctionId>>;
pub(crate) type StringFunctionReturn =
    TypedFunctionReturn<FunctionGraph<StringFunctionLocalId, StringFunctionFunctionId>>;
pub(crate) type BitArrayFunctionReturn =
    TypedFunctionReturn<FunctionGraph<BitArrayFunctionLocalId, BitArrayFunctionFunctionId>>;
pub(crate) type UtfCodepointFunctionReturn =
    TypedFunctionReturn<FunctionGraph<UtfCodepointFunctionLocalId, UtfCodepointFunctionFunctionId>>;
pub(crate) type GenericFunctionReturn =
    TypedFunctionReturn<FunctionGraph<GenericFunctionLocal, GenericFunctionFunctionId>>;
pub(crate) type NeverFunctionReturn =
    TypedFunctionReturn<FunctionGraph<NeverFunctionLocal, NeverFunctionFunctionId>>;
pub(crate) type BoolFunctionReturn =
    TypedFunctionReturn<FunctionGraph<BoolFunctionLocalId, BoolFunctionFunctionId>>;
pub(crate) type NilFunctionReturn =
    TypedFunctionReturn<FunctionGraph<NilFunctionLocalId, NilFunctionFunctionId>>;
pub(crate) type TupleFunctionReturn =
    TypedFunctionReturn<FunctionGraph<TupleFunctionLocalId, TupleFunctionFunctionId>>;
pub(crate) type ListFunctionReturn =
    TypedFunctionReturn<FunctionGraph<ListFunctionLocal, ListFunctionFunctionId>>;

pub(crate) struct CustomFunctionReturn {
    _shape: FunctionShape,
    type_: CustomFunctionType,
    body: FunctionGraph<CustomFunctionLocal, usize>,
}

pub(crate) struct FunctionFunctionReturn {
    _shape: FunctionShape,
    type_: FunctionFunctionType,
    body: FunctionGraph<FunctionFunctionLocal, usize>,
}

pub(crate) struct TypedFunctionReturn<Body> {
    _shape: FunctionShape,
    body: Body,
}

impl CustomFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: CustomFunctionType,
        body: FunctionGraph<CustomFunctionLocal, usize>,
    ) -> Self {
        Self {
            _shape: shape,
            type_,
            body,
        }
    }

    pub(crate) fn body(&self) -> &FunctionGraph<CustomFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionFunctionId {
        CustomFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl FunctionFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: FunctionFunctionType,
        body: FunctionGraph<FunctionFunctionLocal, usize>,
    ) -> Self {
        Self {
            _shape: shape,
            type_,
            body,
        }
    }

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &FunctionGraph<FunctionFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl<Body> TypedFunctionReturn<Body> {
    pub(in crate::plan::execution) fn new(shape: FunctionShape, body: Body) -> Self {
        Self {
            _shape: shape,
            body,
        }
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }
}

use crate::plan::execution::explain::ExplainContext;
use crate::plan::execution::function::{ExplainFunctionBody, FunctionEntry};

impl<Body> ExplainFunctionBody for TypedFunctionReturn<Body>
where
    Body: ExplainFunctionBody,
{
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        self.body().write_function_body(context, family, entry);
    }
}

impl ExplainFunctionBody for CustomFunctionReturn {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        self.body().write_function_body(context, family, entry);
    }
}

impl ExplainFunctionBody for FunctionFunctionReturn {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        self.body().write_function_body(context, family, entry);
    }
}
