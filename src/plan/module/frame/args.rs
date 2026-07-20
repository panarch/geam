use super::FrameLayout;
use crate::plan::{CallArg, CaptureArg};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_call_args(&mut self, args: &[CallArg]) {
        for arg in args {
            self.include_expr(arg.value());
        }
    }

    pub(in crate::plan::module::frame) fn include_capture_args(&mut self, args: &[CaptureArg]) {
        for arg in args {
            self.include_expr(arg.value());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, CallArg, CaptureArg, Expr, FloatExpr,
        FloatFunctionExpr, FloatFunctionLocalId, FloatLocalId, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionLocal, FunctionFunctionLocalId, FunctionShape, IntExpr, IntFunctionLocalId,
        IntListLocalId, ListExpr, ListFunctionExpr, ListLocal, ReturnBody, ReturnExpr, Step,
        StringExpr, StringFunctionExpr, StringFunctionLocalId, StringLocalId, TupleExpr,
        TupleFunctionExpr, TupleFunctionLocalId, TupleLocalId, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };

    #[test]
    fn frame_layout_includes_function_arg_and_capture_families() {
        let returning_function_type =
            super::super::test_helpers::function_returning_int_function_type();
        let int_function_type = super::super::test_helpers::int_function_expr()
            .type_()
            .clone();
        let call_args = vec![
            CallArg::new(crate::plan::Expr::function(
                StringFunctionExpr::local_get(
                    StringFunctionLocalId(7),
                    "string_function_arg".into(),
                    super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                )
                .into(),
            )),
            CallArg::new(crate::plan::Expr::function(
                FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(19),
                    "float_function_arg".into(),
                    super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                )
                .into(),
            )),
            CallArg::new(crate::plan::Expr::function(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(8),
                    "bool_function_arg".into(),
                    super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                )
                .into(),
            )),
            CallArg::new(crate::plan::Expr::function(
                crate::plan::NilFunctionExpr::local_get(
                    crate::plan::NilFunctionLocalId(9),
                    "nil_function_arg".into(),
                    super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                )
                .into(),
            )),
            CallArg::new(crate::plan::Expr::tuple(TupleExpr::local_get(
                TupleLocalId(1),
                "tuple_arg".into(),
                tuple_type(),
            ))),
            CallArg::new(crate::plan::Expr::function(
                TupleFunctionExpr::local_get(
                    TupleFunctionLocalId(1),
                    "tuple_function_arg".into(),
                    tuple_function_type(),
                )
                .into(),
            )),
            CallArg::new(crate::plan::Expr::list(ListExpr::local_get(
                ListLocal::int(IntListLocalId(4)),
                "list_arg".into(),
            ))),
            CallArg::new(crate::plan::Expr::function(
                ListFunctionExpr::local_get(
                    crate::plan::ListFunctionLocal::from_item_type(
                        4,
                        crate::plan::FunctionType::new(
                            Vec::new(),
                            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                        ),
                        crate::plan::ValueType::Int,
                    ),
                    "list_function_arg".into(),
                )
                .into(),
            )),
            CallArg::new(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocal::new(
                        FunctionFunctionLocalId(10),
                        returning_function_type.clone(),
                    ),
                    "function_function_arg".into(),
                ),
            ))),
        ];
        let call_shape = FunctionShape::new(
            call_args.iter().map(CallArg::parameter_shape).collect(),
            ValueShape::Int,
        );
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::call(
                monomorphic_function_instantiation(0, call_shape),
                call_args,
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::closure(
                    monomorphic_function_instantiation(
                        0,
                        FunctionShape::from_function_type(
                            returning_function_type.to_function_type(),
                        ),
                    ),
                    vec![
                        CaptureArg::new(crate::plan::Expr::string(StringExpr::local_get(
                            StringLocalId(15),
                            "string_capture".into(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::float(FloatExpr::local_get(
                            FloatLocalId(20),
                            "float_capture".into(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::bool(BoolExpr::local_get(
                            crate::plan::BoolLocalId(16),
                            "bool_capture".into(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::nil(crate::plan::NilExpr::local_get(
                            crate::plan::NilLocalId(17),
                            "nil_capture".into(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::tuple(TupleExpr::local_get(
                            TupleLocalId(2),
                            "tuple_capture".into(),
                            tuple_type(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::function(
                            crate::plan::IntFunctionExpr::local_get(
                                IntFunctionLocalId(18),
                                "int_function_capture".into(),
                                int_function_type.clone(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            StringFunctionExpr::local_get(
                                StringFunctionLocalId(11),
                                "string_function_capture".into(),
                                super::super::test_helpers::string_function_expr()
                                    .type_()
                                    .clone(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            FloatFunctionExpr::local_get(
                                FloatFunctionLocalId(21),
                                "float_function_capture".into(),
                                super::super::test_helpers::float_function_expr()
                                    .type_()
                                    .clone(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            BoolFunctionExpr::local_get(
                                BoolFunctionLocalId(12),
                                "bool_function_capture".into(),
                                super::super::test_helpers::bool_function_expr()
                                    .type_()
                                    .clone(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            crate::plan::NilFunctionExpr::local_get(
                                crate::plan::NilFunctionLocalId(13),
                                "nil_function_capture".into(),
                                super::super::test_helpers::nil_function_expr()
                                    .type_()
                                    .clone(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            TupleFunctionExpr::local_get(
                                TupleFunctionLocalId(2),
                                "tuple_function_capture".into(),
                                tuple_function_type(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::list(ListExpr::local_get(
                            ListLocal::int(IntListLocalId(5)),
                            "list_capture".into(),
                        ))),
                        CaptureArg::new(crate::plan::Expr::function(
                            ListFunctionExpr::local_get(
                                crate::plan::ListFunctionLocal::from_item_type(
                                    5,
                                    crate::plan::FunctionType::new(
                                        Vec::new(),
                                        crate::plan::ValueType::List(Box::new(
                                            crate::plan::ValueType::Int,
                                        )),
                                    ),
                                    crate::plan::ValueType::Int,
                                ),
                                "list_function_capture".into(),
                            )
                            .into(),
                        )),
                        CaptureArg::new(crate::plan::Expr::function(
                            FunctionFunctionExpr::local_get(
                                FunctionFunctionLocal::new(
                                    FunctionFunctionLocalId(14),
                                    returning_function_type.clone(),
                                ),
                                "function_function_capture".into(),
                            )
                            .into(),
                        )),
                    ],
                    returning_function_type.clone(),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int_body(ReturnBody::expr(IntExpr::value(0.into())));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.string_functions(), 12);
        assert_eq!(layout.bool_functions(), 13);
        assert_eq!(layout.nil_functions(), 14);
        assert_eq!(
            layout
                .function_functions()
                .iter()
                .map(FunctionFunctionLocal::id)
                .collect::<Vec<_>>(),
            vec![FunctionFunctionLocalId(10), FunctionFunctionLocalId(14)],
        );
        assert_eq!(layout.strings(), 16);
        assert_eq!(layout.bools(), 17);
        assert_eq!(layout.nils(), 18);
        assert_eq!(layout.int_functions(), 19);
        assert_eq!(layout.float_functions(), 22);
        assert_eq!(layout.floats(), 21);
        assert_eq!(layout.tuples(), 3);
        assert_eq!(layout.tuple_functions(), 3);
        assert_eq!(layout.int_lists(), 6);
        assert_eq!(
            layout.list_functions(),
            &[
                crate::plan::ListFunctionLocal::from_item_type(
                    4,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    ValueType::Int,
                ),
                crate::plan::ListFunctionLocal::from_item_type(
                    5,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    ValueType::Int,
                ),
            ],
        );
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int]
    }

    fn tuple_function_type() -> crate::plan::FunctionType {
        crate::plan::FunctionType::new(
            vec![ValueType::Tuple(tuple_type())],
            ValueType::Tuple(tuple_type()),
        )
    }
}
