use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, FunctionFunctionId,
    IntFunctionId, ListFunctionId, NeverFunctionId, NilFunctionId, StringFunctionId,
    TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::{CustomConstructorId, FunctionType, ValueShapeId, ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenericCallableId {
    Function {
        template: usize,
        substitution: Box<[ValueShapeId]>,
    },
    Constructor(CustomConstructorId),
}

impl GenericCallableId {
    pub(in crate::plan::execution) fn function(
        template: usize,
        substitution: Vec<ValueShapeId>,
    ) -> Self {
        Self::Function {
            template,
            substitution: substitution.into_boxed_slice(),
        }
    }

    pub(in crate::plan::execution) fn constructor(constructor: CustomConstructorId) -> Self {
        Self::Constructor(constructor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Never(NeverFunctionId),
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple {
        id: TupleFunctionId,
        return_type: Vec<ValueType>,
    },
    List(ListFunctionId),
    Function {
        id: FunctionFunctionId,
        return_type: FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionReturnFamily {
    Generic,
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

impl std::fmt::Display for FunctionReturnFamily {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Generic => "Generic",
            Self::Never => "Never",
            Self::Int => "Int",
            Self::Float => "Float",
            Self::String => "String",
            Self::BitArray => "BitArray",
            Self::UtfCodepoint => "UtfCodepoint",
            Self::Custom => "Custom",
            Self::Bool => "Bool",
            Self::Nil => "Nil",
            Self::Tuple => "Tuple",
            Self::List => "List",
            Self::Function => "Function",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturnFamily;

    #[test]
    fn display_names_every_family() {
        assert_eq!(
            [
                FunctionReturnFamily::Generic,
                FunctionReturnFamily::Never,
                FunctionReturnFamily::Int,
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
                FunctionReturnFamily::BitArray,
                FunctionReturnFamily::UtfCodepoint,
                FunctionReturnFamily::Custom,
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::List,
                FunctionReturnFamily::Function,
            ]
            .map(|family| family.to_string()),
            [
                "Generic",
                "Never",
                "Int",
                "Float",
                "String",
                "BitArray",
                "UtfCodepoint",
                "Custom",
                "Bool",
                "Nil",
                "Tuple",
                "List",
                "Function",
            ],
        );
    }
}
