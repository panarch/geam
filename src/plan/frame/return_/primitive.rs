use crate::plan::{FrameLayout, ReturnBodyKind};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_int_return(&mut self, body: &crate::plan::IntReturn) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_int_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_return(true_);
                self.include_int_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_return(branch);
                }
                self.include_int_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_return(
        &mut self,
        body: &crate::plan::StringReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_string_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_return(true_);
                self.include_string_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_return(branch);
                }
                self.include_string_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_return(&mut self, body: &crate::plan::BoolReturn) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_bool_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_return(true_);
                self.include_bool_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_return(branch);
                }
                self.include_bool_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_return(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_nil_return(&mut self, body: &crate::plan::NilReturn) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_nil_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_return(true_);
                self.include_nil_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_return(branch);
                }
                self.include_nil_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_return(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolLocalId, CallArg, Expr, FrameLayout, IntExpr, IntFunctionId, IntLocalId,
        NilExpr, NilLocalId, ReturnBody, ReturnExpr, Step, StringExpr, StringLocalId,
    };

    #[test]
    fn frame_layout_includes_primitive_return_body_locals() {
        let return_ = ReturnExpr::int_body(
            IntFunctionId(0),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(1),
                    "step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(2), "flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(3), "subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::tail_call(
                                IntFunctionId(1),
                                vec![CallArg::int(
                                    IntLocalId(4),
                                    IntExpr::local_get(IntLocalId(5), "arg".into()),
                                )],
                            ),
                        )],
                        ReturnBody::expr(IntExpr::local_get(IntLocalId(6), "fallback".into())),
                    ),
                    ReturnBody::expr(IntExpr::local_get(IntLocalId(7), "false".into())),
                ),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 8);
        assert_eq!(layout.bools(), 3);
    }

    #[test]
    fn frame_layout_includes_primitive_return_body_families() {
        let string_return = ReturnExpr::string_body(
            crate::plan::StringFunctionId(0),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(20),
                    "string_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(20), "string_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(21), "string_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::expr(StringExpr::local_get(
                                StringLocalId(20),
                                "string_hit".into(),
                            )),
                        )],
                        ReturnBody::expr(StringExpr::local_get(
                            StringLocalId(21),
                            "string_fallback".into(),
                        )),
                    ),
                    ReturnBody::expr(StringExpr::local_get(
                        StringLocalId(22),
                        "string_false".into(),
                    )),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &string_return);
        assert_eq!(layout.ints(), 22);
        assert_eq!(layout.bools(), 21);
        assert_eq!(layout.strings(), 23);

        let bool_return = ReturnExpr::bool_body(
            crate::plan::BoolFunctionId(0),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(22),
                    "bool_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(21), "bool_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(23), "bool_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::expr(BoolExpr::local_get(
                                BoolLocalId(22),
                                "bool_hit".into(),
                            )),
                        )],
                        ReturnBody::expr(BoolExpr::local_get(
                            BoolLocalId(23),
                            "bool_fallback".into(),
                        )),
                    ),
                    ReturnBody::expr(BoolExpr::local_get(BoolLocalId(24), "bool_false".into())),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &bool_return);
        assert_eq!(layout.ints(), 24);
        assert_eq!(layout.bools(), 25);

        let nil_return = ReturnExpr::nil_body(
            crate::plan::NilFunctionId(0),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(24),
                    "nil_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(25), "nil_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(25), "nil_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::expr(NilExpr::local_get(NilLocalId(20), "nil_hit".into())),
                        )],
                        ReturnBody::expr(NilExpr::local_get(NilLocalId(21), "nil_fallback".into())),
                    ),
                    ReturnBody::expr(NilExpr::local_get(NilLocalId(22), "nil_false".into())),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &nil_return);
        assert_eq!(layout.ints(), 26);
        assert_eq!(layout.bools(), 26);
        assert_eq!(layout.nils(), 23);
    }
}
