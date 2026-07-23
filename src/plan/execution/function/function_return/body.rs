use super::super::body::FunctionBody;
use super::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, ListFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    StringFunctionFunctionId, TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::plan::execution::explain::ExplainContext;
use crate::plan::execution::function::{ExplainFunctionBody, FunctionEntry};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BoolFunctionLocalId, CustomFunctionLocal, FloatFunctionLocalId,
    FunctionFunctionLocal, GenericFunctionLocal, IntFunctionLocalId, ListFunctionLocal,
    NeverFunctionLocal, NilFunctionLocalId, StringFunctionLocalId, TupleFunctionLocalId,
    UtfCodepointFunctionLocalId,
};
use crate::plan::execution::type_::{CustomFunctionType, FunctionFunctionType, FunctionShape};

pub(crate) type IntFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<IntFunctionLocalId, IntFunctionFunctionId>>;
pub(crate) type FloatFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<FloatFunctionLocalId, FloatFunctionFunctionId>>;
pub(crate) type StringFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<StringFunctionLocalId, StringFunctionFunctionId>>;
pub(crate) type BitArrayFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<BitArrayFunctionLocalId, BitArrayFunctionFunctionId>>;
pub(crate) type UtfCodepointFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<UtfCodepointFunctionLocalId, UtfCodepointFunctionFunctionId>>;
pub(crate) type GenericFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<GenericFunctionLocal, GenericFunctionFunctionId>>;
pub(crate) type NeverFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<NeverFunctionLocal, NeverFunctionFunctionId>>;
pub(crate) type BoolFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<BoolFunctionLocalId, BoolFunctionFunctionId>>;
pub(crate) type NilFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<NilFunctionLocalId, NilFunctionFunctionId>>;
pub(crate) type TupleFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<TupleFunctionLocalId, TupleFunctionFunctionId>>;
pub(crate) type ListFunctionFunctionBody =
    TypedFunctionBody<FunctionBody<ListFunctionLocal, ListFunctionFunctionId>>;

pub(crate) struct CustomFunctionFunctionBody {
    _shape: FunctionShape,
    type_: CustomFunctionType,
    body: FunctionBody<CustomFunctionLocal, usize>,
}

pub(crate) struct FunctionFunctionFunctionBody {
    _shape: FunctionShape,
    type_: FunctionFunctionType,
    body: FunctionBody<FunctionFunctionLocal, usize>,
}

pub(crate) struct TypedFunctionBody<Body> {
    _shape: FunctionShape,
    body: Body,
}

impl CustomFunctionFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: CustomFunctionType,
        body: FunctionBody<CustomFunctionLocal, usize>,
    ) -> Self {
        Self {
            _shape: shape,
            type_,
            body,
        }
    }

    pub(crate) fn function_body(&self) -> &FunctionBody<CustomFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionFunctionId {
        CustomFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl FunctionFunctionFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: FunctionFunctionType,
        body: FunctionBody<FunctionFunctionLocal, usize>,
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

    pub(crate) fn function_body(&self) -> &FunctionBody<FunctionFunctionLocal, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl<Body> TypedFunctionBody<Body> {
    pub(in crate::plan::execution) fn new(shape: FunctionShape, body: Body) -> Self {
        Self {
            _shape: shape,
            body,
        }
    }

    pub(crate) fn function_body(&self) -> &Body {
        &self.body
    }
}

impl<Body> ExplainFunctionBody for TypedFunctionBody<Body>
where
    Body: ExplainFunctionBody,
{
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

impl ExplainFunctionBody for CustomFunctionFunctionBody {
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

impl ExplainFunctionBody for FunctionFunctionFunctionBody {
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
