use super::FrameLayout;
use crate::plan::{CallArg, CallArgKind, CaptureArg, CaptureArgKind};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_call_args(&mut self, args: &[CallArg]) {
        for arg in args {
            match arg.kind() {
                CallArgKind::Int { value, .. } => self.include_int_expr(value),
                CallArgKind::String { value, .. } => self.include_string_expr(value),
                CallArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CallArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CallArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CallArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CallArgKind::BoolFunction { value, .. } => self.include_bool_function_expr(value),
                CallArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
                CallArgKind::FunctionFunction { value, .. } => {
                    self.include_function_function_expr(value);
                }
            }
        }
    }

    pub(in crate::plan::frame) fn include_capture_args(&mut self, args: &[CaptureArg]) {
        for arg in args {
            match arg.kind() {
                CaptureArgKind::Int { value, .. } => self.include_int_expr(value),
                CaptureArgKind::String { value, .. } => self.include_string_expr(value),
                CaptureArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CaptureArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CaptureArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CaptureArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CaptureArgKind::BoolFunction { value, .. } => {
                    self.include_bool_function_expr(value)
                }
                CaptureArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
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
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, CallArg, CaptureArg, Expr, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId, IntExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, ReturnExpr, Step, StringExpr,
        StringFunctionExpr, StringFunctionLocalId, StringLocalId,
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
    }
}
