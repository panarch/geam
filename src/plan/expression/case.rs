use super::{BoolExpr, FunctionExpr, IntExpr, NilExpr, StringExpr};
use num_bigint::BigInt;

pub(crate) enum BoolCaseBranches {
    Int {
        true_: IntExpr,
        false_: IntExpr,
    },
    String {
        true_: StringExpr,
        false_: StringExpr,
    },
    Bool {
        true_: BoolExpr,
        false_: BoolExpr,
    },
    Nil {
        true_: NilExpr,
        false_: NilExpr,
    },
    Function {
        true_: FunctionExpr,
        false_: FunctionExpr,
    },
}

pub(crate) enum IntCaseBranches {
    Int {
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: StringExpr,
    },
    Bool {
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: NilExpr,
    },
    Function {
        clauses: Vec<(BigInt, FunctionExpr)>,
        fallback: FunctionExpr,
    },
}
