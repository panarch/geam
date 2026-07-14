use crate::plan::{FrameLayout, ListItem, ReturnBody, ReturnBodyKind, TypedListExpr};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_int_return(
        &mut self,
        body: &crate::plan::IntReturn,
    ) {
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_return(branch);
                }
                self.include_int_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_float_return(
        &mut self,
        body: &crate::plan::FloatReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_float_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_float_return(true_);
                self.include_float_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_return(branch);
                }
                self.include_float_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_return(branch);
                }
                self.include_float_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_return(branch);
                }
                self.include_float_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_float_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_string_return(
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_return(branch);
                }
                self.include_string_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_bool_return(
        &mut self,
        body: &crate::plan::BoolReturn,
    ) {
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_return(branch);
                }
                self.include_bool_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_bit_array_return(
        &mut self,
        body: &crate::plan::BitArrayReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_bit_array_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bit_array_return(true_);
                self.include_bit_array_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_return(branch);
                }
                self.include_bit_array_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_return(branch);
                }
                self.include_bit_array_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bit_array_return(branch);
                }
                self.include_bit_array_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bit_array_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_utf_codepoint_return(
        &mut self,
        body: &crate::plan::UtfCodepointReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_utf_codepoint_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_utf_codepoint_return(true_);
                self.include_utf_codepoint_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_return(branch);
                }
                self.include_utf_codepoint_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_return(branch);
                }
                self.include_utf_codepoint_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_utf_codepoint_return(branch);
                }
                self.include_utf_codepoint_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_utf_codepoint_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_nil_return(
        &mut self,
        body: &crate::plan::NilReturn,
    ) {
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
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_return(branch);
                }
                self.include_nil_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
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

    pub(in crate::plan::module::frame) fn include_custom_return(
        &mut self,
        body: &crate::plan::CustomReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_custom_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_custom_return(true_);
                self.include_custom_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_return(branch);
                }
                self.include_custom_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_return(branch);
                }
                self.include_custom_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_custom_return(branch);
                }
                self.include_custom_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_custom_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_tuple_return(
        &mut self,
        body: &crate::plan::TupleReturn,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => self.include_tuple_expr(expression),
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_tuple_return(true_);
                self.include_tuple_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_return(branch);
                }
                self.include_tuple_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_return(branch);
                }
                self.include_tuple_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_return(branch);
                }
                self.include_tuple_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_tuple_return(return_);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_typed_list_return<Item: ListItem>(
        &mut self,
        body: &ReturnBody<TypedListExpr<Item>, Item::Function>,
    ) {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => {
                self.include_list_expr(&Item::expr_to_facade(expression.clone()))
            }
            ReturnBodyKind::TailCall { args, .. } => self.include_call_args(args),
            ReturnBodyKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_typed_list_return(true_);
                self.include_typed_list_return(false_);
            }
            ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_return(branch);
                }
                self.include_typed_list_return(fallback);
            }
            ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_return(branch);
                }
                self.include_typed_list_return(fallback);
            }
            ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_return(branch);
                }
                self.include_typed_list_return(fallback);
            }
            ReturnBodyKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_typed_list_return(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolLocalId, CallArg, CustomExpr, CustomFunctionId, CustomLocalId, CustomType,
        CustomTypeName, Expr, FloatExpr, FloatLocalId, FrameLayout, IntExpr, IntFunctionId,
        IntLocalId, NilExpr, NilLocalId, ReturnBody, ReturnExpr, Step, StringExpr, StringLocalId,
        TupleExpr, TupleFunctionId, TupleLocalId,
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

        let float_return = ReturnExpr::float_body(
            crate::plan::FloatFunctionId(0),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(26),
                    "float_step".into(),
                )))],
                ReturnBody::bool_case(
                    BoolExpr::local_get(BoolLocalId(26), "float_flag".into()),
                    ReturnBody::int_case(
                        IntExpr::local_get(IntLocalId(27), "float_subject".into()),
                        vec![(
                            1.into(),
                            ReturnBody::expr(FloatExpr::local_get(
                                FloatLocalId(20),
                                "float_hit".into(),
                            )),
                        )],
                        ReturnBody::expr(FloatExpr::local_get(
                            FloatLocalId(21),
                            "float_fallback".into(),
                        )),
                    ),
                    ReturnBody::string_case(
                        StringExpr::local_get(StringLocalId(23), "float_string_subject".into()),
                        vec![(
                            "one".into(),
                            ReturnBody::expr(FloatExpr::local_get(
                                FloatLocalId(22),
                                "float_string_hit".into(),
                            )),
                        )],
                        ReturnBody::expr(FloatExpr::local_get(
                            FloatLocalId(23),
                            "float_string_fallback".into(),
                        )),
                    ),
                ),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_return);
        assert_eq!(layout.ints(), 28);
        assert_eq!(layout.floats(), 24);
        assert_eq!(layout.bools(), 27);
        assert_eq!(layout.strings(), 24);

        let int_float_case_return = ReturnExpr::int_body(
            IntFunctionId(2),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(24), "int_float_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(IntExpr::local_get(IntLocalId(28), "int_float_hit".into())),
                )],
                ReturnBody::expr(IntExpr::local_get(
                    IntLocalId(29),
                    "int_float_fallback".into(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &int_float_case_return);
        assert_eq!(layout.ints(), 30);
        assert_eq!(layout.floats(), 25);

        let float_case_return = ReturnExpr::float_body(
            crate::plan::FloatFunctionId(1),
            ReturnBody::float_case(
                FloatExpr::local_get(FloatLocalId(24), "float_case_subject".into()),
                vec![(
                    1.0,
                    ReturnBody::expr(FloatExpr::local_get(
                        FloatLocalId(25),
                        "float_case_hit".into(),
                    )),
                )],
                ReturnBody::expr(FloatExpr::local_get(
                    FloatLocalId(26),
                    "float_case_fallback".into(),
                )),
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_case_return);
        assert_eq!(layout.floats(), 27);

        let float_tail_return = ReturnExpr::float_body(
            crate::plan::FloatFunctionId(2),
            ReturnBody::tail_call(
                crate::plan::FloatFunctionId(3),
                vec![CallArg::float(
                    FloatLocalId(27),
                    FloatExpr::local_get(FloatLocalId(28), "float_tail_arg".into()),
                )],
            ),
        );
        let layout = FrameLayout::from_function_parts(&[], &[], &float_tail_return);
        assert_eq!(layout.floats(), 29);

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

    #[test]
    fn frame_layout_includes_tuple_return_blocks() {
        let return_ = ReturnExpr::tuple_body(
            TupleFunctionId(0),
            vec![crate::plan::ValueType::Int],
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(2),
                    "tuple_step".into(),
                )))],
                ReturnBody::expr(TupleExpr::local_get(
                    TupleLocalId(3),
                    "tuple_return".into(),
                    vec![crate::plan::ValueType::Int],
                )),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 3);
        assert_eq!(layout.tuples(), 4);
    }

    #[test]
    fn frame_layout_includes_custom_return_blocks() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let return_ = ReturnExpr::custom_body(
            CustomFunctionId(0),
            type_.clone(),
            ReturnBody::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(2),
                    "step".into(),
                )))],
                ReturnBody::expr(CustomExpr::local_get(
                    CustomLocalId(3),
                    "return".into(),
                    type_,
                )),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 3);
        assert_eq!(layout.customs, 4);
    }
}
