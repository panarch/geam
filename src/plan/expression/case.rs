use super::{
    BoolExpr, BoolFunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use ecow::EcoString;
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
    IntFunction {
        true_: IntFunctionExpr,
        false_: IntFunctionExpr,
    },
    StringFunction {
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    },
    BoolFunction {
        true_: BoolFunctionExpr,
        false_: BoolFunctionExpr,
    },
    NilFunction {
        true_: NilFunctionExpr,
        false_: NilFunctionExpr,
    },
    FunctionFunction {
        true_: FunctionFunctionExpr,
        false_: FunctionFunctionExpr,
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
    IntFunction {
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}

pub(crate) enum StringCaseBranches {
    Int {
        clauses: Vec<(EcoString, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(EcoString, StringExpr)>,
        fallback: StringExpr,
    },
    Bool {
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: NilExpr,
    },
    IntFunction {
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(EcoString, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(EcoString, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}
