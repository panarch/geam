use crate::plan::FunctionShape;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedFunctionExpr<Expression> {
    shape: FunctionShape,
    expression: Expression,
}

impl<Expression> TypedFunctionExpr<Expression> {
    pub(crate) fn new(shape: FunctionShape, expression: Expression) -> Self {
        Self { shape, expression }
    }

    pub(crate) fn shape(&self) -> &FunctionShape {
        &self.shape
    }

    pub(crate) fn expression(&self) -> &Expression {
        &self.expression
    }
}
