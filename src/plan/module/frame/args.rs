use super::FrameLayout;
use crate::plan::{CallArg, CallArgKind, CaptureArg, CaptureArgKind};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_call_args(&mut self, args: &[CallArg]) {
        for arg in args {
            match arg.kind() {
                CallArgKind::Int { value, .. } => self.include_int_expr(value),
                CallArgKind::String { value, .. } => self.include_string_expr(value),
                CallArgKind::Float { value, .. } => self.include_float_expr(value),
                CallArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CallArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CallArgKind::Tuple { value, .. } => self.include_tuple_expr(value),
                CallArgKind::List(value) => self.include_list_local_expr(value),
                CallArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CallArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CallArgKind::FloatFunction { value, .. } => {
                    self.include_float_function_expr(value);
                }
                CallArgKind::BoolFunction { value, .. } => self.include_bool_function_expr(value),
                CallArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
                CallArgKind::TupleFunction { value, .. } => {
                    self.include_tuple_function_expr(value);
                }
                CallArgKind::ListFunction { value, .. } => {
                    self.include_list_function_expr(value);
                }
                CallArgKind::FunctionFunction { value, .. } => {
                    self.include_function_function_expr(value);
                }
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_capture_args(&mut self, args: &[CaptureArg]) {
        for arg in args {
            match arg.kind() {
                CaptureArgKind::Int { value, .. } => self.include_int_expr(value),
                CaptureArgKind::String { value, .. } => self.include_string_expr(value),
                CaptureArgKind::Float { value, .. } => self.include_float_expr(value),
                CaptureArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CaptureArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CaptureArgKind::Tuple { value, .. } => self.include_tuple_expr(value),
                CaptureArgKind::List(value) => self.include_list_local_expr(value),
                CaptureArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CaptureArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CaptureArgKind::FloatFunction { value, .. } => {
                    self.include_float_function_expr(value);
                }
                CaptureArgKind::BoolFunction { value, .. } => {
                    self.include_bool_function_expr(value)
                }
                CaptureArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
                CaptureArgKind::TupleFunction { value, .. } => {
                    self.include_tuple_function_expr(value);
                }
                CaptureArgKind::ListFunction { value, .. } => {
                    self.include_list_function_expr(value);
                }
                CaptureArgKind::FunctionFunction { value, .. } => {
                    self.include_function_function_expr(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, CallArg, CaptureArg, Expr, FloatExpr,
        FloatFunctionExpr, FloatFunctionLocalId, FloatLocalId, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionLocalId, IntExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntListLocalId, ListExpr, ListFunctionExpr, ListLocal, ReturnExpr,
        Step, StringExpr, StringFunctionExpr, StringFunctionLocalId, StringLocalId, TupleExpr,
        TupleFunctionExpr, TupleFunctionLocalId, TupleLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_function_arg_and_capture_families() {
        let returning_function_type =
            super::super::test_helpers::function_returning_int_function_type();
        let int_function_type = super::super::test_helpers::int_function_expr()
            .type_()
            .clone();
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::call(
                IntFunctionId(0),
                vec![
                    CallArg::string_function(
                        StringFunctionLocalId(1),
                        StringFunctionExpr::local_get(
                            StringFunctionLocalId(7),
                            "string_function_arg".into(),
                            super::super::test_helpers::string_function_expr()
                                .type_()
                                .clone(),
                        ),
                    ),
                    CallArg::float_function(
                        FloatFunctionLocalId(1),
                        FloatFunctionExpr::local_get(
                            FloatFunctionLocalId(19),
                            "float_function_arg".into(),
                            super::super::test_helpers::float_function_expr()
                                .type_()
                                .clone(),
                        ),
                    ),
                    CallArg::bool_function(
                        BoolFunctionLocalId(1),
                        BoolFunctionExpr::local_get(
                            BoolFunctionLocalId(8),
                            "bool_function_arg".into(),
                            super::super::test_helpers::bool_function_expr()
                                .type_()
                                .clone(),
                        ),
                    ),
                    CallArg::nil_function(
                        crate::plan::NilFunctionLocalId(1),
                        crate::plan::NilFunctionExpr::local_get(
                            crate::plan::NilFunctionLocalId(9),
                            "nil_function_arg".into(),
                            super::super::test_helpers::nil_function_expr()
                                .type_()
                                .clone(),
                        ),
                    ),
                    CallArg::tuple(
                        TupleLocalId(1),
                        TupleExpr::local_get(TupleLocalId(1), "tuple_arg".into(), tuple_type()),
                    ),
                    CallArg::tuple_function(
                        TupleFunctionLocalId(1),
                        TupleFunctionExpr::local_get(
                            TupleFunctionLocalId(1),
                            "tuple_function_arg".into(),
                            tuple_function_type(),
                        ),
                    ),
                    CallArg::list(crate::plan::ListLocalExpr::Int {
                        local: IntListLocalId(1),
                        value: ListExpr::local_get(
                            ListLocal::int(IntListLocalId(4)),
                            "list_arg".into(),
                        )
                        .into_int()
                        .expect("expected int list"),
                    }),
                    CallArg::list_function(
                        crate::plan::ListFunctionLocal::from_item_type(
                            1,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        ListFunctionExpr::local_get(
                            crate::plan::ListFunctionLocal::from_item_type(
                                4,
                                crate::plan::FunctionType::new(
                                    Vec::new(),
                                    crate::plan::ValueType::List(Box::new(
                                        crate::plan::ValueType::Int,
                                    )),
                                ),
                                crate::plan::ValueType::Int,
                            ),
                            "list_function_arg".into(),
                        ),
                    ),
                    CallArg::function_function(
                        FunctionFunctionLocalId(1),
                        FunctionFunctionExpr::local_get(
                            FunctionFunctionLocalId(10),
                            "function_function_arg".into(),
                            returning_function_type.clone(),
                        ),
                    ),
                ],
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    vec![
                        CaptureArg::string(
                            StringLocalId(3),
                            StringExpr::local_get(StringLocalId(15), "string_capture".into()),
                        ),
                        CaptureArg::float(
                            FloatLocalId(3),
                            FloatExpr::local_get(FloatLocalId(20), "float_capture".into()),
                        ),
                        CaptureArg::bool(
                            crate::plan::BoolLocalId(3),
                            BoolExpr::local_get(
                                crate::plan::BoolLocalId(16),
                                "bool_capture".into(),
                            ),
                        ),
                        CaptureArg::nil(
                            crate::plan::NilLocalId(3),
                            crate::plan::NilExpr::local_get(
                                crate::plan::NilLocalId(17),
                                "nil_capture".into(),
                            ),
                        ),
                        CaptureArg::tuple(
                            TupleLocalId(2),
                            TupleExpr::local_get(
                                TupleLocalId(2),
                                "tuple_capture".into(),
                                tuple_type(),
                            ),
                        ),
                        CaptureArg::int_function(
                            IntFunctionLocalId(2),
                            crate::plan::IntFunctionExpr::local_get(
                                IntFunctionLocalId(18),
                                "int_function_capture".into(),
                                int_function_type.clone(),
                            ),
                        ),
                        CaptureArg::string_function(
                            StringFunctionLocalId(2),
                            StringFunctionExpr::local_get(
                                StringFunctionLocalId(11),
                                "string_function_capture".into(),
                                super::super::test_helpers::string_function_expr()
                                    .type_()
                                    .clone(),
                            ),
                        ),
                        CaptureArg::float_function(
                            FloatFunctionLocalId(2),
                            FloatFunctionExpr::local_get(
                                FloatFunctionLocalId(21),
                                "float_function_capture".into(),
                                super::super::test_helpers::float_function_expr()
                                    .type_()
                                    .clone(),
                            ),
                        ),
                        CaptureArg::bool_function(
                            BoolFunctionLocalId(2),
                            BoolFunctionExpr::local_get(
                                BoolFunctionLocalId(12),
                                "bool_function_capture".into(),
                                super::super::test_helpers::bool_function_expr()
                                    .type_()
                                    .clone(),
                            ),
                        ),
                        CaptureArg::nil_function(
                            crate::plan::NilFunctionLocalId(2),
                            crate::plan::NilFunctionExpr::local_get(
                                crate::plan::NilFunctionLocalId(13),
                                "nil_function_capture".into(),
                                super::super::test_helpers::nil_function_expr()
                                    .type_()
                                    .clone(),
                            ),
                        ),
                        CaptureArg::tuple_function(
                            TupleFunctionLocalId(2),
                            TupleFunctionExpr::local_get(
                                TupleFunctionLocalId(2),
                                "tuple_function_capture".into(),
                                tuple_function_type(),
                            ),
                        ),
                        CaptureArg::list(crate::plan::ListLocalExpr::Int {
                            local: IntListLocalId(2),
                            value: ListExpr::local_get(
                                ListLocal::int(IntListLocalId(5)),
                                "list_capture".into(),
                            )
                            .into_int()
                            .expect("expected int list"),
                        }),
                        CaptureArg::list_function(
                            crate::plan::ListFunctionLocal::from_item_type(
                                2,
                                crate::plan::FunctionType::new(
                                    Vec::new(),
                                    crate::plan::ValueType::List(Box::new(
                                        crate::plan::ValueType::Int,
                                    )),
                                ),
                                crate::plan::ValueType::Int,
                            ),
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
                            ),
                        ),
                        CaptureArg::function_function(
                            FunctionFunctionLocalId(2),
                            FunctionFunctionExpr::local_get(
                                FunctionFunctionLocalId(14),
                                "function_function_capture".into(),
                                returning_function_type.clone(),
                            ),
                        ),
                    ],
                    returning_function_type.clone(),
                    int_function_type,
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(1), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.string_functions(), 12);
        assert_eq!(layout.bool_functions(), 13);
        assert_eq!(layout.nil_functions(), 14);
        assert_eq!(layout.function_functions(), 15);
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
