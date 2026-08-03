use super::{CustomValueShape, ExternalTypeId, FunctionShape, ValueShapeId, ValueType};
use crate::plan::execution::explain::{Explain, ExplainContext};

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
pub(crate) struct ExternalFunctionType {
    type_: FunctionType,
    arguments: Box<[ValueShapeId]>,
    return_: ExternalTypeId,
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

impl Explain for FunctionType {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("fn(");
        for (index, argument) in self.argument_types().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            context.write(argument);
        }
        context.push_str(") -> ");
        context.write(self.return_());
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

impl ExternalFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: ExternalTypeId,
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

#[cfg(test)]
mod explain_tests {
    use super::FunctionType;
    use crate::plan::execution::explain;
    use crate::plan::execution::type_::ValueType;

    #[test]
    fn writes_function_argument_and_return_types() {
        let source = "pub fn main() { 1 }";
        let type_ = FunctionType::new(
            vec![ValueType::Bool, ValueType::Int, ValueType::Bool],
            ValueType::Int,
        );
        let expected = "fn(Bool, Int, Bool) -> Int";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&type_);
        });
    }
}
