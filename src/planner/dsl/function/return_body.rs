mod conversion;
mod function;
mod primitive;

use crate::plan::{
    BoolFunctionReturn, BoolReturn, FloatFunctionReturn, FloatReturn, FunctionFunctionReturn,
    FunctionType, IntFunctionReturn, IntReturn, NilFunctionReturn, NilReturn, ReturnExpr,
    StringFunctionReturn, StringReturn, TupleFunctionReturn, TupleReturn, ValueType,
};
use crate::planner::context::FunctionRuntimeIds;

pub(crate) use function::*;
pub(crate) use primitive::*;

pub(crate) enum FunctionReturn {
    Int(IntReturn),
    String(StringReturn),
    Float(FloatReturn),
    Bool(BoolReturn),
    Nil(NilReturn),
    Tuple {
        type_: Vec<ValueType>,
        body: TupleReturn,
    },
    IntFunction {
        type_: FunctionType,
        body: IntFunctionReturn,
    },
    StringFunction {
        type_: FunctionType,
        body: StringFunctionReturn,
    },
    FloatFunction {
        type_: FunctionType,
        body: FloatFunctionReturn,
    },
    BoolFunction {
        type_: FunctionType,
        body: BoolFunctionReturn,
    },
    NilFunction {
        type_: FunctionType,
        body: NilFunctionReturn,
    },
    TupleFunction {
        type_: FunctionType,
        body: TupleFunctionReturn,
    },
    FunctionFunction {
        type_: FunctionType,
        body: FunctionFunctionReturn,
    },
}

impl FunctionReturn {
    pub(super) fn build(self, runtime_ids: &mut FunctionRuntimeIds) -> ReturnExpr {
        match self {
            Self::Int(body) => ReturnExpr::int_body(runtime_ids.next_int_id(), body),
            Self::String(body) => ReturnExpr::string_body(runtime_ids.next_string_id(), body),
            Self::Float(body) => ReturnExpr::float_body(runtime_ids.next_float_id(), body),
            Self::Bool(body) => ReturnExpr::bool_body(runtime_ids.next_bool_id(), body),
            Self::Nil(body) => ReturnExpr::nil_body(runtime_ids.next_nil_id(), body),
            Self::Tuple { type_, body } => {
                ReturnExpr::tuple_body(runtime_ids.next_tuple_id(), type_, body)
            }
            Self::IntFunction { type_, body } => {
                ReturnExpr::int_function_body(runtime_ids.next_int_function_id(), type_, body)
            }
            Self::StringFunction { type_, body } => {
                ReturnExpr::string_function_body(runtime_ids.next_string_function_id(), type_, body)
            }
            Self::FloatFunction { type_, body } => {
                ReturnExpr::float_function_body(runtime_ids.next_float_function_id(), type_, body)
            }
            Self::BoolFunction { type_, body } => {
                ReturnExpr::bool_function_body(runtime_ids.next_bool_function_id(), type_, body)
            }
            Self::NilFunction { type_, body } => {
                ReturnExpr::nil_function_body(runtime_ids.next_nil_function_id(), type_, body)
            }
            Self::TupleFunction { type_, body } => {
                ReturnExpr::tuple_function_body(runtime_ids.next_tuple_function_id(), type_, body)
            }
            Self::FunctionFunction { type_, body } => ReturnExpr::function_function_body(
                runtime_ids.next_function_function_id(),
                type_,
                body,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturn;
    use crate::plan::{
        BoolFunctionFunctionId, BoolFunctionId, Expr, FloatFunctionFunctionId, FloatFunctionId,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionType, IntFunctionFunctionId,
        IntFunctionId, NilFunctionFunctionId, NilFunctionId, ParamLocal, ReturnExprKind,
        StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId, TupleFunctionId,
        ValueType,
    };
    use crate::planner::context::FunctionRuntimeIds;
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, function_function_ref, int,
        int_function_ref, nil, nil_function_ref, string, string_function_ref, tuple,
        tuple_function_ref,
    };

    #[test]
    fn function_return_build_allocates_runtime_ids_by_return_family() {
        let mut runtime_ids = FunctionRuntimeIds::default();

        assert!(matches!(
            FunctionReturn::from(int(1)).build(&mut runtime_ids).kind(),
            ReturnExprKind::Int {
                runtime_id: IntFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(string("value"))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::String {
                runtime_id: StringFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(float(1.5))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::Float {
                runtime_id: FloatFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(bool_(true))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::Bool {
                runtime_id: BoolFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(nil()).build(&mut runtime_ids).kind(),
            ReturnExprKind::Nil {
                runtime_id: NilFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(tuple([Expr::from(int(1))]))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::Tuple {
                runtime_id: TupleFunctionId(0),
                ..
            },
        ));

        assert!(matches!(
            FunctionReturn::from(int_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::IntFunction {
                runtime_id: IntFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(string_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::StringFunction {
                runtime_id: StringFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(float_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::FloatFunction {
                runtime_id: FloatFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(bool_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::BoolFunction {
                runtime_id: BoolFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(nil_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids)
                .kind(),
            ReturnExprKind::NilFunction {
                runtime_id: NilFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            ))
            .build(&mut runtime_ids)
            .kind(),
            ReturnExprKind::TupleFunction {
                runtime_id: TupleFunctionFunctionId(0),
                ..
            },
        ));
        assert!(matches!(
            FunctionReturn::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ))
            .build(&mut runtime_ids)
            .kind(),
            ReturnExprKind::FunctionFunction {
                runtime_id: FunctionFunctionFunctionId(0),
                ..
            },
        ));
    }
}
