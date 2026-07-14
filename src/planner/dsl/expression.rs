mod arg;
mod block;
mod call;
mod case;
mod conversion;
mod function_value;
mod local;
mod operator;
mod step;
mod value;

pub(crate) use arg::*;
pub(crate) use block::*;
pub(crate) use call::*;
pub(crate) use case::*;
pub(crate) use function_value::*;
pub(crate) use local::*;
pub(crate) use operator::*;
pub(crate) use step::*;
pub(crate) use value::*;

use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr,
    FunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr,
    NilExpr, NilFunctionExpr, ParamLocal, StringExpr, StringFunctionExpr, TupleExpr,
    TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr, ValueType,
};

pub(crate) struct Int(IntExpr);

pub(crate) struct String(StringExpr);

pub(crate) struct BitArray(BitArrayExpr);

pub(crate) struct UtfCodepoint(UtfCodepointExpr);

pub(crate) struct Float(FloatExpr);

pub(crate) struct Bool(BoolExpr);

pub(crate) struct Nil(NilExpr);

pub(crate) struct Tuple(TupleExpr);

pub(crate) struct List(ListExpr);

pub(crate) struct Function(FunctionExpr);

pub(crate) struct IntFunction(IntFunctionExpr);

pub(crate) struct StringFunction(StringFunctionExpr);

pub(crate) struct BitArrayFunction(BitArrayFunctionExpr);

pub(crate) struct UtfCodepointFunction(UtfCodepointFunctionExpr);

pub(crate) struct FloatFunction(FloatFunctionExpr);

pub(crate) struct BoolFunction(BoolFunctionExpr);

pub(crate) struct NilFunction(NilFunctionExpr);

pub(crate) struct TupleFunction(TupleFunctionExpr);

pub(crate) struct ListFunction(ListFunctionExpr);

pub(crate) struct FunctionFunction(FunctionFunctionExpr);

pub(crate) trait IntoValueType {
    fn into_value_type(self) -> ValueType;
}

pub(crate) trait IntoParamLocal {
    fn into_param_local(self) -> ParamLocal;
}
