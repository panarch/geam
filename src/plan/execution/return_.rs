use super::{
    BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolListExpr,
    BoolListFunctionId, CallArg, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId,
    FloatFunctionId, FloatListExpr, FloatListFunctionId, FunctionFunctionExpr,
    FunctionFunctionFunctionId, FunctionListExpr, FunctionListFunctionId, IntExpr, IntFunctionExpr,
    IntFunctionFunctionId, IntFunctionId, IntListExpr, IntListFunctionId, ListFunctionExpr,
    ListFunctionFunctionId, ListListExpr, ListListFunctionId, NilExpr, NilFunctionExpr,
    NilFunctionFunctionId, NilFunctionId, NilListExpr, NilListFunctionId, Step, StringExpr,
    StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringListExpr,
    StringListFunctionId, TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
    TupleListExpr, TupleListFunctionId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) type IntReturn = ReturnBody<IntExpr, IntFunctionId>;
pub(crate) type FloatReturn = ReturnBody<FloatExpr, FloatFunctionId>;
pub(crate) type StringReturn = ReturnBody<StringExpr, StringFunctionId>;
pub(crate) type BoolReturn = ReturnBody<BoolExpr, BoolFunctionId>;
pub(crate) type NilReturn = ReturnBody<NilExpr, NilFunctionId>;
pub(crate) type TupleReturn = ReturnBody<TupleExpr, TupleFunctionId>;
pub(crate) type IntListReturn = ReturnBody<IntListExpr, IntListFunctionId>;
pub(crate) type FloatListReturn = ReturnBody<FloatListExpr, FloatListFunctionId>;
pub(crate) type StringListReturn = ReturnBody<StringListExpr, StringListFunctionId>;
pub(crate) type BoolListReturn = ReturnBody<BoolListExpr, BoolListFunctionId>;
pub(crate) type NilListReturn = ReturnBody<NilListExpr, NilListFunctionId>;
pub(crate) type TupleListReturn = ReturnBody<TupleListExpr, TupleListFunctionId>;
pub(crate) type ListListReturn = ReturnBody<ListListExpr, ListListFunctionId>;
pub(crate) type FunctionListReturn = ReturnBody<FunctionListExpr, FunctionListFunctionId>;
pub(crate) type IntFunctionReturn = ReturnBody<IntFunctionExpr, IntFunctionFunctionId>;
pub(crate) type FloatFunctionReturn = ReturnBody<FloatFunctionExpr, FloatFunctionFunctionId>;
pub(crate) type StringFunctionReturn = ReturnBody<StringFunctionExpr, StringFunctionFunctionId>;
pub(crate) type BoolFunctionReturn = ReturnBody<BoolFunctionExpr, BoolFunctionFunctionId>;
pub(crate) type NilFunctionReturn = ReturnBody<NilFunctionExpr, NilFunctionFunctionId>;
pub(crate) type TupleFunctionReturn = ReturnBody<TupleFunctionExpr, TupleFunctionFunctionId>;
pub(crate) type ListFunctionReturn = ReturnBody<ListFunctionExpr, ListFunctionFunctionId>;
pub(crate) type FunctionFunctionReturn =
    ReturnBody<FunctionFunctionExpr, FunctionFunctionFunctionId>;

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
