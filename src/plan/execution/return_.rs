use super::{
    BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
    BitArrayListExpr, BitArrayListFunctionId, BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId,
    BoolFunctionId, BoolListExpr, BoolListFunctionId, CallArg, CustomExpr, CustomFunctionExprKind,
    CustomFunctionFunctionId, CustomFunctionId, CustomFunctionType, CustomListExpr,
    CustomListFunctionId, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatListExpr, FloatListFunctionId, FunctionFunctionExprKind, FunctionFunctionFunctionId,
    FunctionFunctionType, FunctionListExpr, FunctionListFunctionId, IntExpr, IntFunctionExpr,
    IntFunctionFunctionId, IntFunctionId, IntListExpr, IntListFunctionId, ListFunctionExpr,
    ListFunctionFunctionId, ListListExpr, ListListFunctionId, NilExpr, NilFunctionExpr,
    NilFunctionFunctionId, NilFunctionId, NilListExpr, NilListFunctionId, Step, StringExpr,
    StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringListExpr,
    StringListFunctionId, TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
    TupleListExpr, TupleListFunctionId, UtfCodepointExpr, UtfCodepointFunctionExpr,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListExpr,
    UtfCodepointListFunctionId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) type IntReturn = ReturnBody<IntExpr, IntFunctionId>;
pub(crate) type FloatReturn = ReturnBody<FloatExpr, FloatFunctionId>;
pub(crate) type StringReturn = ReturnBody<StringExpr, StringFunctionId>;
pub(crate) type BitArrayReturn = ReturnBody<BitArrayExpr, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn = ReturnBody<UtfCodepointExpr, UtfCodepointFunctionId>;
pub(crate) type CustomReturn = ReturnBody<CustomExpr, CustomFunctionId>;
pub(crate) type BoolReturn = ReturnBody<BoolExpr, BoolFunctionId>;
pub(crate) type NilReturn = ReturnBody<NilExpr, NilFunctionId>;
pub(crate) type TupleReturn = ReturnBody<TupleExpr, TupleFunctionId>;
pub(crate) type IntListReturn = ReturnBody<IntListExpr, IntListFunctionId>;
pub(crate) type FloatListReturn = ReturnBody<FloatListExpr, FloatListFunctionId>;
pub(crate) type StringListReturn = ReturnBody<StringListExpr, StringListFunctionId>;
pub(crate) type BitArrayListReturn = ReturnBody<BitArrayListExpr, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListReturn =
    ReturnBody<UtfCodepointListExpr, UtfCodepointListFunctionId>;
pub(crate) type CustomListReturn = ReturnBody<CustomListExpr, CustomListFunctionId>;
pub(crate) type BoolListReturn = ReturnBody<BoolListExpr, BoolListFunctionId>;
pub(crate) type NilListReturn = ReturnBody<NilListExpr, NilListFunctionId>;
pub(crate) type TupleListReturn = ReturnBody<TupleListExpr, TupleListFunctionId>;
pub(crate) type ListListReturn = ReturnBody<ListListExpr, ListListFunctionId>;
pub(crate) type FunctionListReturn = ReturnBody<FunctionListExpr, FunctionListFunctionId>;
pub(crate) type IntFunctionReturn = ReturnBody<IntFunctionExpr, IntFunctionFunctionId>;
pub(crate) type FloatFunctionReturn = ReturnBody<FloatFunctionExpr, FloatFunctionFunctionId>;
pub(crate) type StringFunctionReturn = ReturnBody<StringFunctionExpr, StringFunctionFunctionId>;
pub(crate) type BitArrayFunctionReturn =
    ReturnBody<BitArrayFunctionExpr, BitArrayFunctionFunctionId>;
pub(crate) type UtfCodepointFunctionReturn =
    ReturnBody<UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId>;
pub(crate) struct CustomFunctionReturn {
    type_: CustomFunctionType,
    body: ReturnBody<CustomFunctionExprKind, usize>,
}
pub(crate) type BoolFunctionReturn = ReturnBody<BoolFunctionExpr, BoolFunctionFunctionId>;
pub(crate) type NilFunctionReturn = ReturnBody<NilFunctionExpr, NilFunctionFunctionId>;
pub(crate) type TupleFunctionReturn = ReturnBody<TupleFunctionExpr, TupleFunctionFunctionId>;
pub(crate) type ListFunctionReturn = ReturnBody<ListFunctionExpr, ListFunctionFunctionId>;
pub(crate) struct FunctionFunctionReturn {
    type_: FunctionFunctionType,
    body: ReturnBody<FunctionFunctionExprKind, usize>,
}

pub(crate) struct ReturnBody<Expression, Function> {
    kind: ReturnBodyKind<Expression, Function>,
}

pub(crate) enum ReturnBodyKind<Expression, Function> {
    Expr(Expression),
    TailCall {
        function: Function,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: BoolExpr,
        true_: Box<ReturnBody<Expression, Function>>,
        false_: Box<ReturnBody<Expression, Function>>,
    },
    IntCase {
        subject: IntExpr,
        clauses: Vec<(BigInt, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    FloatCase {
        subject: FloatExpr,
        clauses: Vec<(f64, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    StringCase {
        subject: StringExpr,
        clauses: Vec<(EcoString, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ReturnBody<Expression, Function>>,
    },
}

impl<Expression, Function> ReturnBody<Expression, Function> {
    pub(super) fn from_kind(kind: ReturnBodyKind<Expression, Function>) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &ReturnBodyKind<Expression, Function> {
        &self.kind
    }
}

impl CustomFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        type_: CustomFunctionType,
        body: ReturnBody<CustomFunctionExprKind, usize>,
    ) -> Self {
        Self { type_, body }
    }

    pub(crate) fn type_(&self) -> &CustomFunctionType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &ReturnBody<CustomFunctionExprKind, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionFunctionId {
        CustomFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl FunctionFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        type_: FunctionFunctionType,
        body: ReturnBody<FunctionFunctionExprKind, usize>,
    ) -> Self {
        Self { type_, body }
    }

    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &ReturnBody<FunctionFunctionExprKind, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(index, self.type_.clone())
    }
}
