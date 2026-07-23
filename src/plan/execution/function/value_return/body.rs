use super::super::graph::FunctionGraph;
use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, IntFunctionId,
    NeverFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::explain::ExplainContext;
use crate::plan::execution::function::{ExplainFunctionBody, FunctionEntry};
use crate::plan::execution::{
    BitArrayLocalId, BoolLocalId, CustomLocal, CustomValueShape, FloatLocalId, IntLocalId,
    NilLocalId, StringLocalId, TupleLocalId, UtfCodepointLocalId,
};
use std::convert::Infallible;

pub(crate) type IntReturn = FunctionGraph<IntLocalId, IntFunctionId>;
pub(crate) type NeverReturn = FunctionGraph<Infallible, NeverFunctionId>;
pub(crate) type FloatReturn = FunctionGraph<FloatLocalId, FloatFunctionId>;
pub(crate) type StringReturn = FunctionGraph<StringLocalId, StringFunctionId>;
pub(crate) type BitArrayReturn = FunctionGraph<BitArrayLocalId, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn = FunctionGraph<UtfCodepointLocalId, UtfCodepointFunctionId>;
pub(crate) type BoolReturn = FunctionGraph<BoolLocalId, BoolFunctionId>;
pub(crate) type NilReturn = FunctionGraph<NilLocalId, NilFunctionId>;
pub(crate) type TupleReturn = FunctionGraph<TupleLocalId, TupleFunctionId>;

pub(crate) struct CustomReturn {
    signature_shape: CustomValueShape,
    _body_shape: CustomValueShape,
    body: FunctionGraph<CustomLocal, usize>,
}

impl CustomReturn {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: CustomValueShape,
        body_shape: CustomValueShape,
        body: FunctionGraph<CustomLocal, usize>,
    ) -> Self {
        Self {
            signature_shape,
            _body_shape: body_shape,
            body,
        }
    }

    #[cfg(test)]
    pub(crate) fn body_shape(&self) -> &CustomValueShape {
        &self._body_shape
    }

    #[cfg(test)]
    pub(crate) fn signature_shape(&self) -> &CustomValueShape {
        &self.signature_shape
    }

    pub(crate) fn body(&self) -> &FunctionGraph<CustomLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(index, self.signature_shape)
    }
}

impl ExplainFunctionBody for CustomReturn {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        self.body().write_function_body(context, family, entry);
    }
}
