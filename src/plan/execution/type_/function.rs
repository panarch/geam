use super::{CustomValueShape, FunctionShape, ValueShapeId, ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionType {
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionType {
    type_: FunctionType,
    arguments: Box<[ValueShapeId]>,
    return_: CustomValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericFunctionType {
    type_: FunctionType,
    shape: FunctionShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionFunctionType {
    type_: FunctionType,
    arguments: Box<[ValueShapeId]>,
    return_: FunctionShape,
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub(crate) fn return_(&self) -> &ValueType {
        &self.return_
    }

    pub(crate) fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }
}

impl CustomFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: CustomValueShape,
    ) -> Self {
        Self {
            type_,
            arguments: arguments.into_boxed_slice(),
            return_,
        }
    }
}

impl GenericFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        shape: FunctionShape,
    ) -> Self {
        Self { type_, shape }
    }
}

impl FunctionFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: FunctionShape,
    ) -> Self {
        Self {
            type_,
            arguments: arguments.into_boxed_slice(),
            return_,
        }
    }

    #[cfg(test)]
    pub(crate) fn argument_shapes(&self) -> &[ValueShapeId] {
        &self.arguments
    }

    #[cfg(test)]
    pub(crate) fn return_shape(&self) -> &FunctionShape {
        &self.return_
    }
}
