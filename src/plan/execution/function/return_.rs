use super::graph::FunctionGraph;
use crate::plan::execution::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId,
    BitArrayListFunctionId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionFunctionId,
    BoolFunctionId, BoolFunctionLocalId, BoolListFunctionId, BoolListLocalId, BoolLocalId,
    CustomFunctionFunctionId, CustomFunctionId, CustomFunctionLocal, CustomFunctionType,
    CustomListFunctionId, CustomListLocalId, CustomLocal, CustomValueShape,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatListFunctionId,
    FloatListLocalId, FloatLocalId, FunctionFunctionFunctionId, FunctionFunctionLocal,
    FunctionFunctionType, FunctionListFunctionId, FunctionListLocalId, FunctionShape,
    GenericFunctionFunctionId, GenericFunctionLocal, IntFunctionFunctionId, IntFunctionId,
    IntFunctionLocalId, IntListFunctionId, IntListLocalId, IntLocalId, ListFunctionFunctionId,
    ListFunctionLocal, ListListFunctionId, ListListLocalId, NeverFunctionFunctionId,
    NeverFunctionId, NeverFunctionLocal, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    NilListFunctionId, NilListLocalId, NilLocalId, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListLocalId, ParameterListLocalId,
    StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId, StringListFunctionId,
    StringListLocalId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleListFunctionId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointFunctionLocalId,
    UtfCodepointListFunctionId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
use std::convert::Infallible;

pub(crate) type IntReturn = FunctionGraph<IntLocalId, IntFunctionId>;
pub(crate) type NeverReturn = FunctionGraph<Infallible, NeverFunctionId>;
pub(crate) type FloatReturn = FunctionGraph<FloatLocalId, FloatFunctionId>;
pub(crate) type StringReturn = FunctionGraph<StringLocalId, StringFunctionId>;
pub(crate) type BitArrayReturn = FunctionGraph<BitArrayLocalId, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn = FunctionGraph<UtfCodepointLocalId, UtfCodepointFunctionId>;
pub(crate) struct CustomReturn {
    signature_shape: CustomValueShape,
    _body_shape: CustomValueShape,
    body: FunctionGraph<CustomLocal, usize>,
}
pub(crate) type BoolReturn = FunctionGraph<BoolLocalId, BoolFunctionId>;
pub(crate) type NilReturn = FunctionGraph<NilLocalId, NilFunctionId>;
pub(crate) type TupleReturn = FunctionGraph<TupleLocalId, TupleFunctionId>;
pub(crate) type ParameterListReturn = FunctionGraph<ParameterListLocalId, ParameterListFunctionId>;
pub(crate) type IntListReturn = FunctionGraph<IntListLocalId, IntListFunctionId>;
pub(crate) type FloatListReturn = FunctionGraph<FloatListLocalId, FloatListFunctionId>;
pub(crate) type StringListReturn = FunctionGraph<StringListLocalId, StringListFunctionId>;
pub(crate) type BitArrayListReturn = FunctionGraph<BitArrayListLocalId, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListReturn =
    FunctionGraph<UtfCodepointListLocalId, UtfCodepointListFunctionId>;
pub(crate) type CustomListReturn = FunctionGraph<CustomListLocalId, CustomListFunctionId>;
pub(crate) type BoolListReturn = FunctionGraph<BoolListLocalId, BoolListFunctionId>;
pub(crate) type NilListReturn = FunctionGraph<NilListLocalId, NilListFunctionId>;
pub(crate) type TupleListReturn = FunctionGraph<TupleListLocalId, TupleListFunctionId>;
pub(crate) type ParameterListListReturn =
    FunctionGraph<ParameterListListLocalId, ParameterListListFunctionId>;
pub(crate) type ListListReturn = FunctionGraph<ListListLocalId, ListListFunctionId>;
pub(crate) type FunctionListReturn = FunctionGraph<FunctionListLocalId, FunctionListFunctionId>;
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
pub(crate) struct CustomFunctionReturn {
    _shape: FunctionShape,
    type_: CustomFunctionType,
    body: FunctionGraph<CustomFunctionLocal, usize>,
}
pub(crate) type BoolFunctionReturn =
    TypedFunctionReturn<FunctionGraph<BoolFunctionLocalId, BoolFunctionFunctionId>>;
pub(crate) type NilFunctionReturn =
    TypedFunctionReturn<FunctionGraph<NilFunctionLocalId, NilFunctionFunctionId>>;
pub(crate) type TupleFunctionReturn =
    TypedFunctionReturn<FunctionGraph<TupleFunctionLocalId, TupleFunctionFunctionId>>;
pub(crate) type ListFunctionReturn =
    TypedFunctionReturn<FunctionGraph<ListFunctionLocal, ListFunctionFunctionId>>;
pub(crate) struct FunctionFunctionReturn {
    _shape: FunctionShape,
    type_: FunctionFunctionType,
    body: FunctionGraph<FunctionFunctionLocal, usize>,
}

pub(crate) struct TypedFunctionReturn<Body> {
    _shape: FunctionShape,
    body: Body,
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
