use super::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr,
    TupleFunctionExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolCaseBranches {
    Int {
        true_: IntExpr,
        false_: IntExpr,
    },
    String {
        true_: StringExpr,
        false_: StringExpr,
    },
    Float {
        true_: FloatExpr,
        false_: FloatExpr,
    },
    Bool {
        true_: BoolExpr,
        false_: BoolExpr,
    },
    Nil {
        true_: NilExpr,
        false_: NilExpr,
    },
    Tuple {
        true_: TupleExpr,
        false_: TupleExpr,
    },
    IntFunction {
        true_: IntFunctionExpr,
        false_: IntFunctionExpr,
    },
    StringFunction {
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    },
    FloatFunction {
        true_: FloatFunctionExpr,
        false_: FloatFunctionExpr,
    },
    BoolFunction {
        true_: BoolFunctionExpr,
        false_: BoolFunctionExpr,
    },
    NilFunction {
        true_: NilFunctionExpr,
        false_: NilFunctionExpr,
    },
    TupleFunction {
        true_: TupleFunctionExpr,
        false_: TupleFunctionExpr,
    },
    FunctionFunction {
        true_: FunctionFunctionExpr,
        false_: FunctionFunctionExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntCaseBranches {
    Int {
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: StringExpr,
    },
    Float {
        clauses: Vec<(BigInt, FloatExpr)>,
        fallback: FloatExpr,
    },
    Bool {
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: NilExpr,
    },
    Tuple {
        clauses: Vec<(BigInt, TupleExpr)>,
        fallback: TupleExpr,
    },
    IntFunction {
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    FloatFunction {
        clauses: Vec<(BigInt, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(BigInt, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    TupleFunction {
        clauses: Vec<(BigInt, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringCaseBranches {
    Int {
        clauses: Vec<(EcoString, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(EcoString, StringExpr)>,
        fallback: StringExpr,
    },
    Float {
        clauses: Vec<(EcoString, FloatExpr)>,
        fallback: FloatExpr,
    },
    Bool {
        clauses: Vec<(EcoString, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(EcoString, NilExpr)>,
        fallback: NilExpr,
    },
    Tuple {
        clauses: Vec<(EcoString, TupleExpr)>,
        fallback: TupleExpr,
    },
    IntFunction {
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    FloatFunction {
        clauses: Vec<(EcoString, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(EcoString, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(EcoString, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    TupleFunction {
        clauses: Vec<(EcoString, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FloatCaseBranches {
    Int {
        clauses: Vec<(f64, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(f64, StringExpr)>,
        fallback: StringExpr,
    },
    Float {
        clauses: Vec<(f64, FloatExpr)>,
        fallback: FloatExpr,
    },
    Bool {
        clauses: Vec<(f64, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(f64, NilExpr)>,
        fallback: NilExpr,
    },
    Tuple {
        clauses: Vec<(f64, TupleExpr)>,
        fallback: TupleExpr,
    },
    IntFunction {
        clauses: Vec<(f64, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    FloatFunction {
        clauses: Vec<(f64, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(f64, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(f64, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    TupleFunction {
        clauses: Vec<(f64, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(f64, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}
