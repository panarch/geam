use super::FrameLayout;
use crate::plan::{
    BoolFunctionExpr, BoolFunctionExprKind, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
    FunctionFunctionExprKind, IntFunctionExpr, IntFunctionExprKind, NilFunctionExpr,
    NilFunctionExprKind, StringFunctionExpr, StringFunctionExprKind,
};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_function_expr(&mut self, expression: &FunctionExpr) {
        match expression.kind() {
            FunctionExprKind::Int(expression) => self.include_int_function_expr(expression),
            FunctionExprKind::String(expression) => self.include_string_function_expr(expression),
            FunctionExprKind::Bool(expression) => self.include_bool_function_expr(expression),
            FunctionExprKind::Nil(expression) => self.include_nil_function_expr(expression),
            FunctionExprKind::Function(expression) => {
                self.include_function_function_expr(expression);
            }
        }
    }

    pub(in crate::plan::frame) fn include_int_function_expr(
        &mut self,
        expression: &IntFunctionExpr,
    ) {
        match expression.kind() {
            IntFunctionExprKind::Value(_) => {}
            IntFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            IntFunctionExprKind::LocalGet { local, .. } => self.include_int_function(*local),
            IntFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            IntFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            IntFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_function_expr(true_);
                self.include_int_function_expr(false_);
            }
            IntFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_expr(branch);
                }
                self.include_int_function_expr(fallback);
            }
            IntFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_expr(branch);
                }
                self.include_int_function_expr(fallback);
            }
            IntFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_function_expr(
        &mut self,
        expression: &StringFunctionExpr,
    ) {
        match expression.kind() {
            StringFunctionExprKind::Value(_) => {}
            StringFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            StringFunctionExprKind::LocalGet { local, .. } => {
                self.include_string_function(*local);
            }
            StringFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            StringFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            StringFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_function_expr(true_);
                self.include_string_function_expr(false_);
            }
            StringFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_expr(branch);
                }
                self.include_string_function_expr(fallback);
            }
            StringFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_expr(branch);
                }
                self.include_string_function_expr(fallback);
            }
            StringFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_function_expr(
        &mut self,
        expression: &BoolFunctionExpr,
    ) {
        match expression.kind() {
            BoolFunctionExprKind::Value(_) => {}
            BoolFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            BoolFunctionExprKind::LocalGet { local, .. } => self.include_bool_function(*local),
            BoolFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            BoolFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            BoolFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_function_expr(true_);
                self.include_bool_function_expr(false_);
            }
            BoolFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_expr(branch);
                }
                self.include_bool_function_expr(fallback);
            }
            BoolFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_expr(branch);
                }
                self.include_bool_function_expr(fallback);
            }
            BoolFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_nil_function_expr(
        &mut self,
        expression: &NilFunctionExpr,
    ) {
        match expression.kind() {
            NilFunctionExprKind::Value(_) => {}
            NilFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            NilFunctionExprKind::LocalGet { local, .. } => self.include_nil_function(*local),
            NilFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            NilFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            NilFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_function_expr(true_);
                self.include_nil_function_expr(false_);
            }
            NilFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_expr(branch);
                }
                self.include_nil_function_expr(fallback);
            }
            NilFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_expr(branch);
                }
                self.include_nil_function_expr(fallback);
            }
            NilFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_function_function_expr(
        &mut self,
        expression: &FunctionFunctionExpr,
    ) {
        match expression.kind() {
            FunctionFunctionExprKind::Value(_) => {}
            FunctionFunctionExprKind::Closure { captures, .. } => {
                self.include_capture_args(captures);
            }
            FunctionFunctionExprKind::LocalGet { local, .. } => {
                self.include_function_function(*local);
            }
            FunctionFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FunctionFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            FunctionFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_function_function_expr(true_);
                self.include_function_function_expr(false_);
            }
            FunctionFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_expr(branch);
                }
                self.include_function_function_expr(fallback);
            }
            FunctionFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_expr(branch);
                }
                self.include_function_function_expr(fallback);
            }
            FunctionFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_function_function_expr(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, CallArg, CaptureArg, Expr,
        FunctionExpr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntLocalId, NilFunctionExpr, NilFunctionId, NilFunctionLocalId,
        ReturnExpr, Step, StringFunctionExpr, StringFunctionId, StringFunctionLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_function_expression_nested_locals() {
        let nested_block = IntFunctionExpr::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(4),
                "value".into(),
            )))],
            super::super::test_helpers::int_function_expr(),
        );
        let nested_case = IntFunctionExpr::int_case(
            IntExpr::local_get(IntLocalId(3), "subject".into()),
            vec![(1.into(), nested_block)],
            super::super::test_helpers::int_function_expr(),
        );
        let function_case = IntFunctionExpr::bool_case(
            BoolExpr::local_get(crate::plan::BoolLocalId(2), "flag".into()),
            super::super::test_helpers::int_function_expr(),
            nested_case,
        );
        let steps = vec![Step::evaluate(Expr::function(FunctionExpr::int(
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::function(FunctionExpr::int(
                    function_case,
                )))],
                super::super::test_helpers::int_function_expr(),
            ),
        )))];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 5);
        assert_eq!(layout.bools(), 3);
    }

    #[test]
    fn frame_layout_includes_function_local_storage() {
        let steps = vec![Step::let_int_function(
            IntFunctionLocalId(1),
            "f".into(),
            IntFunctionExpr::local_get(
                IntFunctionLocalId(2),
                "g".into(),
                super::super::test_helpers::int_function_expr()
                    .type_()
                    .clone(),
            ),
        )];
        let return_ = ReturnExpr::int(
            IntFunctionId(0),
            IntExpr::function_call(
                IntFunctionExpr::local_get(
                    IntFunctionLocalId(3),
                    "h".into(),
                    super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                ),
                Vec::new(),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.int_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_function_expression_return_families() {
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::local_get(
                    StringFunctionLocalId(1),
                    "string".into(),
                    super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(2),
                    "bool".into(),
                    super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::local_get(
                    NilFunctionLocalId(3),
                    "nil".into(),
                    super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.string_functions(), 2);
        assert_eq!(layout.bool_functions(), 3);
        assert_eq!(layout.nil_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_function_expression_call_and_closure_families() {
        let string_type = super::super::test_helpers::string_function_expr()
            .type_()
            .clone();
        let bool_type = super::super::test_helpers::bool_function_expr()
            .type_()
            .clone();
        let nil_type = super::super::test_helpers::nil_function_expr()
            .type_()
            .clone();
        let function_type = super::super::test_helpers::function_returning_int_function_type();

        let string_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(string_type.clone())),
        );
        let bool_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(bool_type.clone())),
        );
        let nil_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(nil_type.clone())),
        );
        let function_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(function_type.clone())),
        );

        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::closure(
                    StringFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(30), "string_closure_capture".into()),
                    )],
                    string_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(20),
                        "string_callee".into(),
                        string_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(31), "string_call_arg".into()),
                    )],
                    string_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::closure(
                    BoolFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(32), "bool_closure_capture".into()),
                    )],
                    bool_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(21),
                        "bool_callee".into(),
                        bool_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(33), "bool_call_arg".into()),
                    )],
                    bool_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(NilFunctionExpr::closure(
                NilFunctionId(1),
                Vec::new(),
                vec![CaptureArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(34), "nil_closure_capture".into()),
                )],
                nil_type.clone(),
            )))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(22),
                        "nil_callee".into(),
                        nil_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(35), "nil_call_arg".into()),
                    )],
                    nil_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(36), "function_closure_capture".into()),
                    )],
                    function_type.clone(),
                    super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(23),
                        "function_callee".into(),
                        function_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(37), "function_call_arg".into()),
                    )],
                    function_type,
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 38);
        assert_eq!(layout.function_functions(), 24);
    }
}
