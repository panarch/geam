use super::graph::{FunctionGraph, NeverReturn as NeverValue};
use super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId,
    BitArrayListFunctionId, BitArrayListLocalId, BoolFunctionFunctionId, BoolFunctionId,
    BoolFunctionLocalId, BoolListFunctionId, BoolListLocalId, CustomFunctionFunctionId,
    CustomFunctionId, CustomFunctionLocal, CustomFunctionType, CustomListFunctionId,
    CustomListLocalId, CustomLocal, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId,
    FloatListFunctionId, FloatListLocalId, FunctionFunctionFunctionId, FunctionFunctionLocal,
    FunctionFunctionType, FunctionListFunctionId, FunctionListLocalId, GenericFunctionFunctionId,
    GenericFunctionLocal, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
    IntListFunctionId, IntListLocalId, ListFunctionFunctionId, ListFunctionLocal,
    ListListFunctionId, ListListLocalId, NeverFunctionFunctionId, NeverFunctionId,
    NeverFunctionLocal, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    NilListFunctionId, NilListLocalId, ParameterListFunctionId, ParameterListListFunctionId,
    ParameterListListLocalId, ParameterListLocalId, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringListFunctionId, StringListLocalId, TupleFunctionFunctionId,
    TupleFunctionId, TupleFunctionLocalId, TupleListFunctionId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointFunctionLocalId,
    UtfCodepointListFunctionId, UtfCodepointListLocalId,
};

pub(crate) type IntReturn = FunctionGraph<super::IntLocalId, IntFunctionId>;
pub(crate) type NeverReturn = FunctionGraph<NeverValue, NeverFunctionId>;
pub(crate) type FloatReturn = FunctionGraph<super::FloatLocalId, FloatFunctionId>;
pub(crate) type StringReturn = FunctionGraph<super::StringLocalId, StringFunctionId>;
pub(crate) type BitArrayReturn = FunctionGraph<super::BitArrayLocalId, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn =
    FunctionGraph<super::UtfCodepointLocalId, UtfCodepointFunctionId>;
pub(crate) struct CustomReturn {
    signature_shape: super::CustomValueShape,
    _body_shape: super::CustomValueShape,
    body: FunctionGraph<CustomLocal, usize>,
}
pub(crate) type BoolReturn = FunctionGraph<super::BoolLocalId, BoolFunctionId>;
pub(crate) type NilReturn = FunctionGraph<super::NilLocalId, NilFunctionId>;
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
    _shape: super::FunctionShape,
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
    _shape: super::FunctionShape,
    type_: FunctionFunctionType,
    body: FunctionGraph<FunctionFunctionLocal, usize>,
}

pub(crate) struct TypedFunctionReturn<Body> {
    _shape: super::FunctionShape,
    body: Body,
}

impl CustomReturn {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: super::CustomValueShape,
        body_shape: super::CustomValueShape,
        body: FunctionGraph<CustomLocal, usize>,
    ) -> Self {
        Self {
            signature_shape,
            _body_shape: body_shape,
            body,
        }
    }

    #[cfg(test)]
    pub(crate) fn body_shape(&self) -> &super::CustomValueShape {
        &self._body_shape
    }

    #[cfg(test)]
    pub(crate) fn signature_shape(&self) -> &super::CustomValueShape {
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
        shape: super::FunctionShape,
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
        shape: super::FunctionShape,
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
    pub(in crate::plan::execution) fn new(shape: super::FunctionShape, body: Body) -> Self {
        Self {
            _shape: shape,
            body,
        }
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }
}
