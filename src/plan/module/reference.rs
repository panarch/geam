use super::ParamSlot;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionReference {
    instantiation: super::FunctionInstantiation,
    params: Vec<ParamSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypedFunctionReference<Function> {
    instantiation: super::FunctionInstantiation,
    params: Vec<ParamSlot>,
    family: PhantomData<Function>,
}

pub(crate) type GenericFunctionReference = TypedFunctionReference<super::GenericExpr>;
pub(crate) type IntFunctionReference = TypedFunctionReference<super::IntExpr>;
pub(crate) type FloatFunctionReference = TypedFunctionReference<super::FloatExpr>;
pub(crate) type StringFunctionReference = TypedFunctionReference<super::StringExpr>;
pub(crate) type BitArrayFunctionReference = TypedFunctionReference<super::BitArrayExpr>;
pub(crate) type UtfCodepointFunctionReference = TypedFunctionReference<super::UtfCodepointExpr>;
pub(crate) type CustomFunctionReference = TypedFunctionReference<super::CustomExpr>;
pub(crate) type BoolFunctionReference = TypedFunctionReference<super::BoolExpr>;
pub(crate) type NilFunctionReference = TypedFunctionReference<super::NilExpr>;
pub(crate) type TupleFunctionReference = TypedFunctionReference<super::TupleExpr>;
pub(crate) type ListFunctionReference = TypedFunctionReference<super::ListExpr>;
pub(crate) type FunctionFunctionReference = TypedFunctionReference<super::FunctionExpr>;

impl FunctionReference {
    pub(crate) fn from_slots(
        instantiation: super::FunctionInstantiation,
        params: Vec<ParamSlot>,
    ) -> Self {
        Self {
            instantiation,
            params,
        }
    }

    #[cfg(test)]
    pub(crate) fn new(
        instantiation: super::FunctionInstantiation,
        params: Vec<super::ParamLocal>,
    ) -> Self {
        Self::from_slots(
            instantiation,
            params.into_iter().map(ParamSlot::from_local).collect(),
        )
    }

    pub(crate) fn into_parts(self) -> (super::FunctionInstantiation, Vec<ParamSlot>) {
        (self.instantiation, self.params)
    }

    pub(crate) fn substitute(&self, substitution: &super::TypeSubstitution) -> Self {
        Self {
            instantiation: self.instantiation.substitute(substitution),
            params: self.params.clone(),
        }
    }
}

impl<Function> TypedFunctionReference<Function> {
    pub(crate) fn from_slots(
        instantiation: super::FunctionInstantiation,
        params: Vec<ParamSlot>,
    ) -> Self {
        Self {
            instantiation,
            params,
            family: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn new(
        instantiation: super::FunctionInstantiation,
        params: Vec<super::ParamLocal>,
    ) -> Self {
        Self::from_slots(
            instantiation,
            params.into_iter().map(ParamSlot::from_local).collect(),
        )
    }

    pub(crate) fn instantiation(&self) -> &super::FunctionInstantiation {
        &self.instantiation
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }
}
