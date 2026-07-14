use super::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, BoolListCaseBranches,
    FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListCaseBranches,
    ListFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr,
    TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
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
    BitArray {
        true_: BitArrayExpr,
        false_: BitArrayExpr,
    },
    UtfCodepoint {
        true_: UtfCodepointExpr,
        false_: UtfCodepointExpr,
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
    List(BoolListCaseBranches),
    IntFunction {
        true_: IntFunctionExpr,
        false_: IntFunctionExpr,
    },
    StringFunction {
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    },
    BitArrayFunction {
        true_: BitArrayFunctionExpr,
        false_: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        true_: UtfCodepointFunctionExpr,
        false_: UtfCodepointFunctionExpr,
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
    ListFunction {
        true_: ListFunctionExpr,
        false_: ListFunctionExpr,
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
    BitArray {
        clauses: Vec<(BigInt, BitArrayExpr)>,
        fallback: BitArrayExpr,
    },
    UtfCodepoint {
        clauses: Vec<(BigInt, UtfCodepointExpr)>,
        fallback: UtfCodepointExpr,
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
    List(ListCaseBranches<BigInt>),
    IntFunction {
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BitArrayFunction {
        clauses: Vec<(BigInt, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        clauses: Vec<(BigInt, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
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
    ListFunction {
        clauses: Vec<(BigInt, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
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
    BitArray {
        clauses: Vec<(EcoString, BitArrayExpr)>,
        fallback: BitArrayExpr,
    },
    UtfCodepoint {
        clauses: Vec<(EcoString, UtfCodepointExpr)>,
        fallback: UtfCodepointExpr,
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
    List(ListCaseBranches<EcoString>),
    IntFunction {
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BitArrayFunction {
        clauses: Vec<(EcoString, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        clauses: Vec<(EcoString, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
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
    ListFunction {
        clauses: Vec<(EcoString, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
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
    BitArray {
        clauses: Vec<(f64, BitArrayExpr)>,
        fallback: BitArrayExpr,
    },
    UtfCodepoint {
        clauses: Vec<(f64, UtfCodepointExpr)>,
        fallback: UtfCodepointExpr,
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
    List(ListCaseBranches<f64>),
    IntFunction {
        clauses: Vec<(f64, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BitArrayFunction {
        clauses: Vec<(f64, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        clauses: Vec<(f64, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
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
    ListFunction {
        clauses: Vec<(f64, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(f64, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}
