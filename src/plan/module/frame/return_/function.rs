use crate::plan::{FrameLayout, ReturnBodyKind};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_int_function_return(
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_return(branch);
                }
                self.include_int_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_float_function_return(
        &mut self,
        body: &crate::plan::FloatFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_float_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_float_function_return(true_);
                self.include_float_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_return(branch);
                }
                self.include_float_function_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_return(branch);
                }
                self.include_float_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_return(branch);
                }
                self.include_float_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_float_function_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_string_function_return(
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_return(branch);
                }
                self.include_string_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_bool_function_return(
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_return(branch);
                }
                self.include_bool_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_bit_array_function_return(
        &mut self,
        body: &crate::plan::BitArrayFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_bit_array_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bit_array_function_return(true_);
                self.include_bit_array_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_function_return(branch);
                }
                self.include_bit_array_function_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_function_return(branch);
                }
                self.include_bit_array_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_function_return(branch);
                }
                self.include_bit_array_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bit_array_function_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_utf_codepoint_function_return(
        &mut self,
        body: &crate::plan::UtfCodepointFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => {
                self.include_utf_codepoint_function_expr(expression)
            }
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_utf_codepoint_function_return(true_);
                self.include_utf_codepoint_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_function_return(branch);
                }
                self.include_utf_codepoint_function_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_function_return(branch);
                }
                self.include_utf_codepoint_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_function_return(branch);
                }
                self.include_utf_codepoint_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_utf_codepoint_function_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_nil_function_return(
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_return(branch);
                }
                self.include_nil_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_custom_function_return(
        &mut self,
        body: &crate::plan::CustomFunctionReturn,
    ) {
        self.include_custom_function_return_body(body.kind());
    }

    fn include_custom_function_return_body(
        &mut self,
        body: &ReturnBodyKind<crate::plan::CustomFunctionExprKind, usize>,
    ) {
        match body {
            ReturnBodyKind::Expr(expression) => self.include_custom_function_expr_kind(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_custom_function_return_body(true_.kind());
                self.include_custom_function_return_body(false_.kind());
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_function_return_body(branch.kind());
                }
                self.include_custom_function_return_body(fallback.kind());
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_function_return_body(branch.kind());
                }
                self.include_custom_function_return_body(fallback.kind());
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_function_return_body(branch.kind());
                }
                self.include_custom_function_return_body(fallback.kind());
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_custom_function_return_body(return_.kind());
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_tuple_function_return(
        &mut self,
        body: &crate::plan::TupleFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_tuple_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_tuple_function_return(true_);
                self.include_tuple_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_return(branch);
                }
                self.include_tuple_function_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_return(branch);
                }
                self.include_tuple_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_return(branch);
                }
                self.include_tuple_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_tuple_function_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_list_function_return(
        &mut self,
        body: &crate::plan::ListFunctionReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_list_function_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_list_function_return(true_);
                self.include_list_function_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_return(branch);
                }
                self.include_list_function_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_return(branch);
                }
                self.include_list_function_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_return(branch);
                }
                self.include_list_function_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_list_function_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_function_function_return(
        &mut self,
        body: &crate::plan::FunctionFunctionReturn,
    ) {
        self.include_function_function_return_body(body.kind());
    }

    fn include_function_function_return_body(
        &mut self,
        body: &ReturnBodyKind<crate::plan::FunctionFunctionExprKind, usize>,
    ) {
        match body {
            ReturnBodyKind::Expr(expression) => {
                self.include_function_function_expr_kind(expression)
            }
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_function_function_return_body(true_.kind());
                self.include_function_function_return_body(false_.kind());
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_return_body(branch.kind());
                }
                self.include_function_function_return_body(fallback.kind());
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_return_body(branch.kind());
                }
                self.include_function_function_return_body(fallback.kind());
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_return_body(branch.kind());
                }
                self.include_function_function_return_body(fallback.kind());
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_function_function_return_body(return_.kind());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionLocalId, BoolExpr,
        BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionLocalId, BoolLocalId, CallArg,
        CustomFunctionExpr, CustomFunctionFunctionId, CustomFunctionLocal, CustomFunctionLocalId,
        CustomFunctionReturn, CustomFunctionType, CustomType, CustomTypeName, Expr, FloatExpr,
        FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionLocalId, FloatLocalId,
        FrameLayout, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionLocal,
        FunctionFunctionLocalId, FunctionFunctionReturn, FunctionFunctionType, FunctionType,
        IntExpr, IntFunctionFunctionId, IntFunctionLocalId, IntLocalId, ListFunctionExpr,
        ListFunctionFunctionId, ListFunctionLocal, NilFunctionExpr, NilFunctionFunctionId,
        NilFunctionLocalId, ReturnBody, ReturnExpr, Step, StringExpr, StringFunctionExpr,
        StringFunctionFunctionId, StringFunctionLocalId, StringLocalId, ValueType,
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

        let float_function_return = ReturnExpr::float_function_body(
            FloatFunctionFunctionId(0),
            super::super::super::test_helpers::float_function_expr()
                .type_()
                .clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(27),
                    "float_function_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(9), "float_function_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(28), "float_function_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(FloatFunctionFunctionId(1), Vec::new()),
                        )],
                        ReturnBody::expr(FloatFunctionExpr::local_get(
                            FloatFunctionLocalId(4),
                            "float_function_true".into(),
                            super::super::super::test_helpers::float_function_expr()
                                .type_()
                                .clone(),
                        )),
                    ),
                    ReturnBody::expr(FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(5),
                        "float_function_false".into(),
                        super::super::super::test_helpers::float_function_expr()
                            .type_()
                            .clone(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_function_return);
        assert_eq!(layout.ints(), 29);
        assert_eq!(layout.bools(), 10);
        assert_eq!(layout.float_functions(), 6);

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

        let function_function_type = FunctionFunctionType::new(
            Vec::new(),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
        );
        let function_function_return = ReturnExpr::function_function_body(
            0,
            FunctionFunctionReturn::expr(FunctionFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(13),
                    "function_function_step".into(),
                )))],
                FunctionFunctionExpr::bool_case(
                    BoolExpr::local_get(BoolLocalId(8), "function_function_flag".into()),
                    FunctionFunctionExpr::int_case(
                        IntExpr::local_get(IntLocalId(26), "function_function_subject".into()),
                        vec![(
                            1.into(),
                            FunctionFunctionExpr::call(
                                FunctionFunctionFunctionId::new(1, function_function_type.clone()),
                                Vec::new(),
                            ),
                        )],
                        FunctionFunctionExpr::local_get(
                            FunctionFunctionLocal::new(
                                FunctionFunctionLocalId(4),
                                function_function_type.clone(),
                            ),
                            "function_function_fallback".into(),
                        ),
                    ),
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocal::new(
                            FunctionFunctionLocalId(5),
                            function_function_type,
                        ),
                        "function_function_false".into(),
                    ),
                ),
            )),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &function_function_return);
        assert_eq!(layout.ints(), 27);
        assert_eq!(layout.bools(), 9);
        assert_eq!(
            layout
                .function_functions()
                .iter()
                .map(FunctionFunctionLocal::id)
                .collect::<Vec<_>>(),
            vec![FunctionFunctionLocalId(4), FunctionFunctionLocalId(5)],
        );
    }

    #[test]
    fn frame_layout_includes_bit_array_function_tail_call_inside_block() {
        let return_ = ReturnExpr::bit_array_function_body(
            BitArrayFunctionFunctionId(0),
            FunctionType::new(Vec::new(), ValueType::BitArray),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(3),
                    "step".into(),
                )))],
                ReturnBody::tail_call(BitArrayFunctionFunctionId(1), Vec::new()),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 4);
    }

    #[test]
    fn frame_layout_includes_every_bit_array_function_return_case_dependency() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let local = |index: usize, name: &str| {
            BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(index),
                name.into(),
                type_.clone(),
            )
        };
        let return_ = ReturnExpr::bit_array_function_body(
            BitArrayFunctionFunctionId(0),
            type_.clone(),
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(0), "bool_subject".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(1), "int_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::string_case(
                            StringExpr::local_get(StringLocalId(2), "string_subject".into()),
                            vec![("one".into(), ReturnBody::expr(local(4, "string_branch")))],
                            ReturnBody::expr(local(5, "string_fallback")),
                        ),
                    )],
                    ReturnBody::float_case(
                        FloatExpr::local_get(FloatLocalId(3), "float_subject".into()),
                        vec![(1.0, ReturnBody::expr(local(6, "float_branch")))],
                        ReturnBody::expr(local(7, "float_fallback")),
                    ),
                ),
                ReturnBody::expr(local(8, "bool_fallback")),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.ints(), 2);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.floats(), 4);
        assert_eq!(layout.bit_array_functions, 9);
    }

    #[test]
    fn frame_layout_includes_function_return_body_string_case_families() {
        let int_function_return = ReturnExpr::int_function_body(
            IntFunctionFunctionId(0),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(0), "int_function_subject".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                        IntFunctionLocalId(1),
                        "int_function_branch".into(),
                        super::super::super::test_helpers::int_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                    IntFunctionLocalId(2),
                    "int_function_fallback".into(),
                    super::super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &int_function_return);
        assert_eq!(layout.strings(), 1);
        assert_eq!(layout.int_functions(), 3);

        let string_function_return = ReturnExpr::string_function_body(
            StringFunctionFunctionId(0),
            super::super::super::test_helpers::string_function_expr()
                .type_()
                .clone(),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(1), "string_function_subject".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(StringFunctionExpr::local_get(
                        StringFunctionLocalId(3),
                        "string_function_branch".into(),
                        super::super::super::test_helpers::string_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(StringFunctionExpr::local_get(
                    StringFunctionLocalId(4),
                    "string_function_fallback".into(),
                    super::super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &string_function_return);
        assert_eq!(layout.strings(), 2);
        assert_eq!(layout.string_functions(), 5);

        let float_function_return = ReturnExpr::float_function_body(
            FloatFunctionFunctionId(0),
            super::super::super::test_helpers::float_function_expr()
                .type_()
                .clone(),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(5), "float_function_subject".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(11),
                        "float_function_branch".into(),
                        super::super::super::test_helpers::float_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(12),
                    "float_function_fallback".into(),
                    super::super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_function_return);
        assert_eq!(layout.strings(), 6);
        assert_eq!(layout.float_functions(), 13);

        let bool_function_return = ReturnExpr::bool_function_body(
            BoolFunctionFunctionId(0),
            super::super::super::test_helpers::bool_function_expr()
                .type_()
                .clone(),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(2), "bool_function_subject".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(5),
                        "bool_function_branch".into(),
                        super::super::super::test_helpers::bool_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(6),
                    "bool_function_fallback".into(),
                    super::super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &bool_function_return);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.bool_functions(), 7);

        let nil_function_return = ReturnExpr::nil_function_body(
            NilFunctionFunctionId(0),
            super::super::super::test_helpers::nil_function_expr()
                .type_()
                .clone(),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(3), "nil_function_subject".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(NilFunctionExpr::local_get(
                        NilFunctionLocalId(7),
                        "nil_function_branch".into(),
                        super::super::super::test_helpers::nil_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(NilFunctionExpr::local_get(
                    NilFunctionLocalId(8),
                    "nil_function_fallback".into(),
                    super::super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &nil_function_return);
        assert_eq!(layout.strings(), 4);
        assert_eq!(layout.nil_functions(), 9);

        let function_function_type = FunctionFunctionType::new(
            Vec::new(),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
        );
        let function_function_return = ReturnExpr::function_function_body(
            0,
            FunctionFunctionReturn::expr(FunctionFunctionExpr::string_case(
                StringExpr::local_get(StringLocalId(4), "function_function_subject".into()),
                vec![(
                    "one".into(),
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocal::new(
                            FunctionFunctionLocalId(9),
                            function_function_type.clone(),
                        ),
                        "function_function_branch".into(),
                    ),
                )],
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocal::new(FunctionFunctionLocalId(10), function_function_type),
                    "function_function_fallback".into(),
                ),
            )),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &function_function_return);
        assert_eq!(layout.strings(), 5);
        assert_eq!(
            layout
                .function_functions()
                .iter()
                .map(FunctionFunctionLocal::id)
                .collect::<Vec<_>>(),
            vec![FunctionFunctionLocalId(9), FunctionFunctionLocalId(10)],
        );
    }

    #[test]
    fn frame_layout_includes_every_custom_function_return_case_dependency() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let type_ = CustomFunctionType::new(Vec::new(), custom_type);
        let return_ = ReturnExpr::custom_function_body(
            0,
            CustomFunctionReturn::expr(CustomFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(2),
                    "step".into(),
                )))],
                CustomFunctionExpr::bool_case(
                    BoolExpr::local_get(BoolLocalId(1), "flag".into()),
                    CustomFunctionExpr::int_case(
                        IntExpr::local_get(IntLocalId(3), "int_subject".into()),
                        vec![(
                            1.into(),
                            CustomFunctionExpr::local_get(
                                CustomFunctionLocal::new(CustomFunctionLocalId(2), type_.clone()),
                                "int_branch".into(),
                            ),
                        )],
                        CustomFunctionExpr::float_case(
                            FloatExpr::local_get(FloatLocalId(4), "float_subject".into()),
                            vec![(
                                1.0,
                                CustomFunctionExpr::local_get(
                                    CustomFunctionLocal::new(
                                        CustomFunctionLocalId(3),
                                        type_.clone(),
                                    ),
                                    "float_branch".into(),
                                ),
                            )],
                            CustomFunctionExpr::local_get(
                                CustomFunctionLocal::new(CustomFunctionLocalId(4), type_.clone()),
                                "float_fallback".into(),
                            ),
                        ),
                    ),
                    CustomFunctionExpr::string_case(
                        StringExpr::local_get(StringLocalId(5), "string_subject".into()),
                        vec![(
                            "tail".into(),
                            CustomFunctionExpr::call(
                                CustomFunctionFunctionId::new(1, type_.clone()),
                                Vec::new(),
                            ),
                        )],
                        CustomFunctionExpr::local_get(
                            CustomFunctionLocal::new(CustomFunctionLocalId(5), type_),
                            "string_fallback".into(),
                        ),
                    ),
                ),
            )),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 4);
        assert_eq!(layout.floats(), 5);
        assert_eq!(layout.strings(), 6);
        assert_eq!(layout.bools(), 2);
        assert_eq!(
            layout
                .custom_functions
                .iter()
                .map(CustomFunctionLocal::id)
                .collect::<Vec<_>>(),
            vec![
                CustomFunctionLocalId(2),
                CustomFunctionLocalId(3),
                CustomFunctionLocalId(4),
                CustomFunctionLocalId(5),
            ],
        );
    }

    #[test]
    fn frame_layout_includes_function_return_body_float_case_families() {
        let int_function_return = ReturnExpr::int_function_body(
            IntFunctionFunctionId(0),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(0), "int_function_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                        IntFunctionLocalId(1),
                        "int_function_branch".into(),
                        super::super::super::test_helpers::int_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(crate::plan::IntFunctionExpr::local_get(
                    IntFunctionLocalId(2),
                    "int_function_fallback".into(),
                    super::super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &int_function_return);
        assert_eq!(layout.floats(), 1);
        assert_eq!(layout.int_functions(), 3);

        let string_function_return = ReturnExpr::string_function_body(
            StringFunctionFunctionId(0),
            super::super::super::test_helpers::string_function_expr()
                .type_()
                .clone(),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(1), "string_function_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(StringFunctionExpr::local_get(
                        StringFunctionLocalId(3),
                        "string_function_branch".into(),
                        super::super::super::test_helpers::string_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(StringFunctionExpr::local_get(
                    StringFunctionLocalId(4),
                    "string_function_fallback".into(),
                    super::super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &string_function_return);
        assert_eq!(layout.floats(), 2);
        assert_eq!(layout.string_functions(), 5);

        let float_function_return = ReturnExpr::float_function_body(
            FloatFunctionFunctionId(0),
            super::super::super::test_helpers::float_function_expr()
                .type_()
                .clone(),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(2), "float_function_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(5),
                        "float_function_branch".into(),
                        super::super::super::test_helpers::float_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(6),
                    "float_function_fallback".into(),
                    super::super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_function_return);
        assert_eq!(layout.floats(), 3);
        assert_eq!(layout.float_functions(), 7);

        let bool_function_return = ReturnExpr::bool_function_body(
            BoolFunctionFunctionId(0),
            super::super::super::test_helpers::bool_function_expr()
                .type_()
                .clone(),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(3), "bool_function_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(7),
                        "bool_function_branch".into(),
                        super::super::super::test_helpers::bool_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(8),
                    "bool_function_fallback".into(),
                    super::super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &bool_function_return);
        assert_eq!(layout.floats(), 4);
        assert_eq!(layout.bool_functions(), 9);

        let nil_function_return = ReturnExpr::nil_function_body(
            NilFunctionFunctionId(0),
            super::super::super::test_helpers::nil_function_expr()
                .type_()
                .clone(),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(4), "nil_function_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(NilFunctionExpr::local_get(
                        NilFunctionLocalId(9),
                        "nil_function_branch".into(),
                        super::super::super::test_helpers::nil_function_expr()
                            .type_()
                            .clone(),
                    )),
                )],
                ReturnBody::expr(NilFunctionExpr::local_get(
                    NilFunctionLocalId(10),
                    "nil_function_fallback".into(),
                    super::super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &nil_function_return);
        assert_eq!(layout.floats(), 5);
        assert_eq!(layout.nil_functions(), 11);

        let function_function_type = FunctionFunctionType::new(
            Vec::new(),
            super::super::super::test_helpers::int_function_expr()
                .type_()
                .clone(),
        );
        let function_function_return = ReturnExpr::function_function_body(
            0,
            FunctionFunctionReturn::expr(FunctionFunctionExpr::float_case(
                FloatExpr::local_get(FloatLocalId(5), "function_function_subject".into()),
                vec![(
                    1.0,
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocal::new(
                            FunctionFunctionLocalId(11),
                            function_function_type.clone(),
                        ),
                        "function_function_branch".into(),
                    ),
                )],
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocal::new(FunctionFunctionLocalId(12), function_function_type),
                    "function_function_fallback".into(),
                ),
            )),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &function_function_return);
        assert_eq!(layout.floats(), 6);
        assert_eq!(
            layout
                .function_functions()
                .iter()
                .map(FunctionFunctionLocal::id)
                .collect::<Vec<_>>(),
            vec![FunctionFunctionLocalId(11), FunctionFunctionLocalId(12)],
        );
    }

    #[test]
    fn frame_layout_includes_list_function_return_body_families() {
        let type_ = list_function_type();
        let list_function_return = ReturnExpr::list_function_body(
            ListFunctionFunctionId::from_item_type(0, type_.clone(), ValueType::Int),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(0),
                    "block_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(0), "flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(1), "int_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(
                                ListFunctionFunctionId::from_item_type(
                                    1,
                                    type_.clone(),
                                    ValueType::Int,
                                ),
                                vec![CallArg::int(
                                    IntLocalId(0),
                                    IntExpr::local_get(IntLocalId(2), "tail_arg".into()),
                                )],
                            ),
                        )],
                        ReturnBody::expr(ListFunctionExpr::local_get(
                            ListFunctionLocal::from_item_type(
                                1,
                                type_.clone(),
                                crate::plan::ValueType::Int,
                            ),
                            "int_fallback".into(),
                        )),
                    ),
                    ReturnBody::expr(ListFunctionExpr::local_get(
                        ListFunctionLocal::from_item_type(
                            2,
                            type_.clone(),
                            crate::plan::ValueType::Int,
                        ),
                        "false_branch".into(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &list_function_return);
        assert_eq!(layout.ints(), 3);
        assert_eq!(layout.bools(), 1);
        assert_eq!(
            layout.list_functions(),
            &[
                ListFunctionLocal::from_item_type(1, type_.clone(), crate::plan::ValueType::Int),
                ListFunctionLocal::from_item_type(2, type_.clone(), crate::plan::ValueType::Int),
            ],
        );

        let string_case_return = ReturnExpr::list_function_body(
            ListFunctionFunctionId::from_item_type(0, type_.clone(), ValueType::Int),
            ReturnBody::string_case(
                StringExpr::local_get(StringLocalId(0), "string_subject".into()),
                vec![(
                    "hit".into(),
                    ReturnBody::expr(ListFunctionExpr::local_get(
                        ListFunctionLocal::from_item_type(
                            3,
                            type_.clone(),
                            crate::plan::ValueType::Int,
                        ),
                        "string_branch".into(),
                    )),
                )],
                ReturnBody::expr(ListFunctionExpr::local_get(
                    ListFunctionLocal::from_item_type(
                        4,
                        type_.clone(),
                        crate::plan::ValueType::Int,
                    ),
                    "string_fallback".into(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &string_case_return);
        assert_eq!(layout.strings(), 1);
        assert_eq!(
            layout.list_functions(),
            &[
                ListFunctionLocal::from_item_type(3, type_.clone(), crate::plan::ValueType::Int),
                ListFunctionLocal::from_item_type(4, type_.clone(), crate::plan::ValueType::Int),
            ],
        );

        let float_case_return = ReturnExpr::list_function_body(
            ListFunctionFunctionId::from_item_type(0, type_.clone(), ValueType::Int),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(0), "float_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(ListFunctionExpr::local_get(
                        ListFunctionLocal::from_item_type(
                            5,
                            type_.clone(),
                            crate::plan::ValueType::Int,
                        ),
                        "float_branch".into(),
                    )),
                )],
                ReturnBody::expr(ListFunctionExpr::local_get(
                    ListFunctionLocal::from_item_type(
                        6,
                        type_.clone(),
                        crate::plan::ValueType::Int,
                    ),
                    "float_fallback".into(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_case_return);
        assert_eq!(layout.floats(), 1);
        assert_eq!(
            layout.list_functions(),
            &[
                ListFunctionLocal::from_item_type(5, type_.clone(), crate::plan::ValueType::Int),
                ListFunctionLocal::from_item_type(6, type_, crate::plan::ValueType::Int),
            ],
        );
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }
}
