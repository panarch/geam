use super::{
    BoolFunctionExpr, BoolListExpr, CallArg, CustomExpr, CustomFieldAccess, Expr, FloatExpr,
    IntExpr, ListExpr, PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::execution::{
    AssertPattern, BitArrayExpr, BitArrayPattern, BoolFunctionId, BoolLocalId, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub struct BoolExpr {
    kind: BoolExprKind,
}

pub(crate) enum BoolExprKind {
    Value(bool),
    LocalGet {
        local: BoolLocalId,
    },
    Call {
        function: BoolFunctionId,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<BoolFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<BoolListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    Not(Box<BoolExpr>),
    LtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    LtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    GtEqFloat {
        left: Box<FloatExpr>,
        right: Box<FloatExpr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    StringStartsWith {
        value: Box<StringExpr>,
        prefix: EcoString,
    },
    ListLengthEquals {
        value: Box<ListExpr>,
        length: usize,
    },
    ListLengthAtLeast {
        value: Box<ListExpr>,
        length: usize,
    },
    BitArrayMatches {
        value: Box<BitArrayExpr>,
        pattern: BitArrayPattern,
    },
    CustomMatches {
        value: Box<CustomExpr>,
        pattern: Box<AssertPattern>,
    },
    And {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    Or {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolExpr>,
        false_: Box<BoolExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolExpr>,
    },
}

impl BoolExpr {
    pub(in crate::plan::execution) fn from_kind(kind: BoolExprKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &BoolExprKind {
        &self.kind
    }
}
