use super::super::body::FunctionBody;
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

pub(crate) type IntFunctionBody = FunctionBody<IntLocalId, IntFunctionId>;
pub(crate) type NeverFunctionBody = FunctionBody<Infallible, NeverFunctionId>;
pub(crate) type FloatFunctionBody = FunctionBody<FloatLocalId, FloatFunctionId>;
pub(crate) type StringFunctionBody = FunctionBody<StringLocalId, StringFunctionId>;
pub(crate) type BitArrayFunctionBody = FunctionBody<BitArrayLocalId, BitArrayFunctionId>;
pub(crate) type UtfCodepointFunctionBody =
    FunctionBody<UtfCodepointLocalId, UtfCodepointFunctionId>;
pub(crate) type BoolFunctionBody = FunctionBody<BoolLocalId, BoolFunctionId>;
pub(crate) type NilFunctionBody = FunctionBody<NilLocalId, NilFunctionId>;
pub(crate) type TupleFunctionBody = FunctionBody<TupleLocalId, TupleFunctionId>;

pub(crate) struct CustomFunctionBody {
    signature_shape: CustomValueShape,
    _body_shape: CustomValueShape,
    body: FunctionBody<CustomLocal, usize>,
}

impl CustomFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: CustomValueShape,
        body_shape: CustomValueShape,
        body: FunctionBody<CustomLocal, usize>,
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

    pub(crate) fn function_body(&self) -> &FunctionBody<CustomLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(index, self.signature_shape)
    }
}

impl ExplainFunctionBody for CustomFunctionBody {
    fn write_function_body(
        &self,
        context: &mut ExplainContext<'_, '_>,
        family: &'static str,
        entry: &FunctionEntry,
    ) {
        self.function_body()
            .write_function_body(context, family, entry);
    }
}
