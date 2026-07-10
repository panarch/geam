mod conversion;
mod function;
mod primitive;

use crate::plan::{
    BoolFunctionReturn, BoolReturn, FloatFunctionReturn, FloatReturn, FunctionFunctionReturn,
    FunctionType, IntFunctionReturn, IntReturn, ListFunctionReturn, ListReturn, NilFunctionReturn,
    NilReturn, ReturnExpr, StringFunctionReturn, StringReturn, TupleFunctionReturn, TupleReturn,
    ValueType,
};
use crate::planner::context::FunctionRuntimeIds;

pub(crate) use function::*;
pub(crate) use primitive::*;

#[derive(Debug, PartialEq)]
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
    List(ListReturn),
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
    ListFunction {
        item_type: ValueType,
        type_: FunctionType,
        body: ListFunctionReturn,
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
            Self::List(ListReturn::Int(body)) => {
                ReturnExpr::int_list_body(runtime_ids.next_int_list_id(), body)
            }
            Self::List(ListReturn::String(body)) => {
                ReturnExpr::string_list_body(runtime_ids.next_string_list_id(), body)
            }
            Self::List(ListReturn::Float(body)) => {
                ReturnExpr::float_list_body(runtime_ids.next_float_list_id(), body)
            }
            Self::List(ListReturn::Bool(body)) => {
                ReturnExpr::bool_list_body(runtime_ids.next_bool_list_id(), body)
            }
            Self::List(ListReturn::Nil(body)) => {
                ReturnExpr::nil_list_body(runtime_ids.next_nil_list_id(), body)
            }
            Self::List(ListReturn::Tuple { item_type, body }) => {
                ReturnExpr::tuple_list_body(runtime_ids.next_tuple_list_id(), item_type, body)
            }
            Self::List(ListReturn::List { item_type, body }) => {
                ReturnExpr::list_list_body(runtime_ids.next_list_list_id(), item_type, body)
            }
            Self::List(ListReturn::Function { item_type, body }) => {
                ReturnExpr::function_list_body(runtime_ids.next_function_list_id(), item_type, body)
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
            Self::ListFunction {
                item_type,
                type_,
                body,
            } => ReturnExpr::list_function_body(
                runtime_ids.next_list_function_id(type_, item_type),
                body,
            ),
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
        IntFunctionId, ListFunctionFunctionId, NilFunctionFunctionId, NilFunctionId, ParamLocal,
        ReturnBody, ReturnExpr, StringFunctionFunctionId, StringFunctionId,
        TupleFunctionFunctionId, TupleFunctionId, ValueType,
    };
    use crate::planner::context::FunctionRuntimeIds;
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float, float_function_ref, function_function_ref, int,
        int_function_ref, list, list_function_ref, nil, nil_function_ref, string,
        string_function_ref, tuple, tuple_function_ref,
    };

    #[test]
    fn function_return_build_allocates_runtime_ids_by_return_family() {
        let mut runtime_ids = FunctionRuntimeIds::default();

        assert_eq!(
            FunctionReturn::from(int(1)).build(&mut runtime_ids),
            ReturnExpr::int_body(IntFunctionId(0), ReturnBody::expr(int(1).into())),
        );
        assert_eq!(
            FunctionReturn::from(string("value")).build(&mut runtime_ids),
            ReturnExpr::string_body(
                StringFunctionId(0),
                ReturnBody::expr(string("value").into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(float(1.5)).build(&mut runtime_ids),
            ReturnExpr::float_body(FloatFunctionId(0), ReturnBody::expr(float(1.5).into())),
        );
        assert_eq!(
            FunctionReturn::from(bool_(true)).build(&mut runtime_ids),
            ReturnExpr::bool_body(BoolFunctionId(0), ReturnBody::expr(bool_(true).into())),
        );
        assert_eq!(
            FunctionReturn::from(nil()).build(&mut runtime_ids),
            ReturnExpr::nil_body(NilFunctionId(0), ReturnBody::expr(nil().into())),
        );
        assert_eq!(
            FunctionReturn::from(tuple([Expr::from(int(1))])).build(&mut runtime_ids),
            ReturnExpr::tuple_body(
                TupleFunctionId(0),
                vec![ValueType::Int],
                ReturnBody::expr(tuple([Expr::from(int(1))]).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list([int(1)], ValueType::Int)).build(&mut runtime_ids),
            ReturnExpr::int_list_body(
                crate::plan::IntListFunctionId(0),
                crate::plan::IntListReturn::expr(
                    crate::plan::ListExpr::from(list([int(1)], ValueType::Int))
                        .into_int()
                        .expect("expression should be List(Int)"),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::String))
                .build(&mut runtime_ids),
            ReturnExpr::string_list_body(
                crate::plan::StringListFunctionId(0),
                crate::plan::StringListReturn::expr(
                    crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::String))
                        .into_string()
                        .expect("expression should be List(String)"),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Float))
                .build(&mut runtime_ids),
            ReturnExpr::float_list_body(
                crate::plan::FloatListFunctionId(0),
                crate::plan::FloatListReturn::expr(
                    crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Float))
                        .into_float()
                        .expect("expression should be List(Float)"),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Bool)).build(&mut runtime_ids),
            ReturnExpr::bool_list_body(
                crate::plan::BoolListFunctionId(0),
                crate::plan::BoolListReturn::expr(
                    crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Bool))
                        .into_bool()
                        .expect("expression should be List(Bool)"),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Nil)).build(&mut runtime_ids),
            ReturnExpr::nil_list_body(
                crate::plan::NilListFunctionId(0),
                crate::plan::NilListReturn::expr(
                    crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Nil))
                        .into_nil()
                        .expect("expression should be List(Nil)"),
                ),
            ),
        );

        let tuple_item = vec![ValueType::Int];
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::Tuple(tuple_item.clone())
            ))
            .build(&mut runtime_ids),
            ReturnExpr::tuple_list_body(
                crate::plan::TupleListFunctionId(0),
                tuple_item.clone(),
                crate::plan::TupleListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::Tuple(tuple_item)
                    ))
                    .into_tuple()
                    .expect("expression should be List(Tuple)"),
                ),
            ),
        );

        let list_item = Box::new(ValueType::Int);
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::List(list_item.clone())))
                .build(&mut runtime_ids),
            ReturnExpr::list_list_body(
                crate::plan::ListListFunctionId(0),
                list_item.clone(),
                crate::plan::ListListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::List(list_item)
                    ))
                    .into_list()
                    .expect("expression should be List(List)"),
                ),
            ),
        );

        let function_item = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::Function(Box::new(function_item.clone())),
            ))
            .build(&mut runtime_ids),
            ReturnExpr::function_list_body(
                crate::plan::FunctionListFunctionId(0),
                function_item.clone(),
                crate::plan::FunctionListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::Function(Box::new(function_item)),
                    ))
                    .into_function()
                    .expect("expression should be List(Function)"),
                ),
            ),
        );

        assert_eq!(
            FunctionReturn::from(int_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids),
            ReturnExpr::int_function_body(
                IntFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::Int),
                ReturnBody::expr(int_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(string_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids),
            ReturnExpr::string_function_body(
                StringFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::String),
                ReturnBody::expr(string_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(float_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids),
            ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::Float),
                ReturnBody::expr(float_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(bool_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids),
            ReturnExpr::bool_function_body(
                BoolFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::Bool),
                ReturnBody::expr(bool_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(nil_function_ref(0, Vec::<ParamLocal>::new()))
                .build(&mut runtime_ids),
            ReturnExpr::nil_function_body(
                NilFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::Nil),
                ReturnBody::expr(nil_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            ))
            .build(&mut runtime_ids),
            ReturnExpr::tuple_function_body(
                TupleFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                ReturnBody::expr(
                    tuple_function_ref(0, Vec::<ParamLocal>::new(), [ValueType::Int]).into(),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int
            ))
            .build(&mut runtime_ids),
            ReturnExpr::list_function_body(
                ListFunctionFunctionId::from_item_type(
                    0,
                    FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                    ValueType::Int,
                ),
                ReturnBody::expr(
                    list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ))
            .build(&mut runtime_ids),
            ReturnExpr::function_function_body(
                FunctionFunctionFunctionId(0),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
                ReturnBody::expr(
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )
                    .into(),
                ),
            ),
        );
    }
}
