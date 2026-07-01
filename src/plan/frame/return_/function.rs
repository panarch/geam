use crate::plan::{FrameLayout, ReturnBodyKind};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_int_function_return(
        &mut self,
        body: &crate::plan::IntFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_int_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_function_return(true_);
                self.include_int_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_return(branch);
                }
                self.include_int_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_function_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_function_return(
        &mut self,
        body: &crate::plan::StringFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_string_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_function_return(true_);
                self.include_string_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_return(branch);
                }
                self.include_string_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_function_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_function_return(
        &mut self,
        body: &crate::plan::BoolFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_bool_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_function_return(true_);
                self.include_bool_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_return(branch);
                }
                self.include_bool_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_function_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_nil_function_return(
        &mut self,
        body: &crate::plan::NilFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_nil_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_function_return(true_);
                self.include_nil_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_return(branch);
                }
                self.include_nil_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_function_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_function_function_return(
        &mut self,
        body: &crate::plan::FunctionFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_function_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_function_function_return(true_);
                self.include_function_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_return(branch);
                }
                self.include_function_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_function_function_return(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionLocalId, BoolLocalId, Expr,
        FrameLayout, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionLocalId,
        IntExpr, IntFunctionFunctionId, IntFunctionLocalId, IntLocalId, NilFunctionExpr,
        NilFunctionFunctionId, NilFunctionLocalId, ReturnBody, ReturnExpr, Step,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionLocalId,
    };

    #[test]
    fn frame_layout_includes_function_return_body_families() {
        let function_return = ReturnExpr::int_function_body(
            IntFunctionFunctionId(0),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(8),
                    "function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(4), "function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(9), "function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(IntFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                            IntFunctionLocalId(6),
                            "function_fallback".into(),
                            super::super::super::test_helpers::int_function_expr()
                                .type_()
                                .clone(),
                        )),
                    ),
                    ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                        IntFunctionLocalId(7),
                        "function_false".into(),
                        super::super::super::test_helpers::int_function_expr()
                            .type_()
                            .clone(),
                    )),
                ),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &function_return);

        assert_eq!(layout.ints(), 10);
        assert_eq!(layout.bools(), 5);
        assert_eq!(layout.int_functions(), 8);

        let string_function_return = ReturnExpr::string_function_body(
            StringFunctionFunctionId(0),
            super::super::super::test_helpers::string_function_expr()
                .type_()
                .clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(10),
                    "string_function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(5), "string_function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(14), "string_function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(StringFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(StringFunctionExpr::local_get(
                            StringFunctionLocalId(4),
                            "string_function_true".into(),
                            super::super::super::test_helpers::string_function_expr()
                                .type_()
                                .clone(),
                        )),
                    ),
                    ReturnBody::expr(StringFunctionExpr::local_get(
                        StringFunctionLocalId(5),
                        "string_function_false".into(),
                        super::super::super::test_helpers::string_function_expr()
                            .type_()
                            .clone(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &string_function_return);
        assert_eq!(layout.ints(), 15);
        assert_eq!(layout.bools(), 6);
        assert_eq!(layout.string_functions(), 6);

        let bool_function_return = ReturnExpr::bool_function_body(
            BoolFunctionFunctionId(0),
            super::super::super::test_helpers::bool_function_expr()
                .type_()
                .clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(11),
                    "bool_function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(6), "bool_function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(15), "bool_function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(BoolFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(BoolFunctionExpr::local_get(
                            BoolFunctionLocalId(4),
                            "bool_function_true".into(),
                            super::super::super::test_helpers::bool_function_expr()
                                .type_()
                                .clone(),
                        )),
                    ),
                    ReturnBody::expr(BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(5),
                        "bool_function_false".into(),
                        super::super::super::test_helpers::bool_function_expr()
                            .type_()
                            .clone(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &bool_function_return);
        assert_eq!(layout.ints(), 16);
        assert_eq!(layout.bools(), 7);
        assert_eq!(layout.bool_functions(), 6);

        let nil_function_return = ReturnExpr::nil_function_body(
            NilFunctionFunctionId(0),
            super::super::super::test_helpers::nil_function_expr()
                .type_()
                .clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(12),
                    "nil_function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(7), "nil_function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(16), "nil_function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(NilFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(NilFunctionExpr::local_get(
                            NilFunctionLocalId(4),
                            "nil_function_true".into(),
                            super::super::super::test_helpers::nil_function_expr()
                                .type_()
                                .clone(),
                        )),
                    ),
                    ReturnBody::expr(NilFunctionExpr::local_get(
                        NilFunctionLocalId(5),
                        "nil_function_false".into(),
                        super::super::super::test_helpers::nil_function_expr()
                            .type_()
                            .clone(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &nil_function_return);
        assert_eq!(layout.ints(), 17);
        assert_eq!(layout.bools(), 8);
        assert_eq!(layout.nil_functions(), 6);

        let function_function_type = super::super::super::test_helpers::int_function_expr()
            .type_()
            .clone();
        let function_function_return = ReturnExpr::function_function_body(
            FunctionFunctionFunctionId(0),
            function_function_type.clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(13),
                    "function_function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(8), "function_function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(26), "function_function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(FunctionFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(FunctionFunctionExpr::local_get(
                            FunctionFunctionLocalId(4),
                            "function_function_fallback".into(),
                            function_function_type.clone(),
                        )),
                    ),
                    ReturnBody::expr(FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(5),
                        "function_function_false".into(),
                        function_function_type,
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &function_function_return);
        assert_eq!(layout.ints(), 27);
        assert_eq!(layout.bools(), 9);
        assert_eq!(layout.function_functions(), 6);
    }
}
