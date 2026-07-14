use super::FrameLayout;
use crate::plan::{
    AssertPattern, BitArrayAssertPattern, CustomBindingPattern, ListAssertPattern, ListAssertTail,
    Step, StepKind, TotalBindingPattern,
};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_steps(&mut self, steps: &[Step]) {
        for step in steps {
            self.include_step(step);
        }
    }

    fn include_step(&mut self, step: &Step) {
        match step.kind() {
            StepKind::LetInt { local, value, .. } => {
                self.include_int_expr(value);
                self.include_int(*local);
            }
            StepKind::LetFloat { local, value, .. } => {
                self.include_float_expr(value);
                self.include_float(*local);
            }
            StepKind::LetString { local, value, .. } => {
                self.include_string_expr(value);
                self.include_string(*local);
            }
            StepKind::LetBitArray { local, value, .. } => {
                self.include_bit_array_expr(value);
                self.include_bit_array(*local);
            }
            StepKind::LetUtfCodepoint { local, value, .. } => {
                self.include_utf_codepoint_expr(value);
                self.include_utf_codepoint(*local);
            }
            StepKind::LetCustom { local, value, .. } => {
                self.include_custom_expr(value);
                self.include_custom(*local);
            }
            StepKind::LetBool { local, value, .. } => {
                self.include_bool_expr(value);
                self.include_bool(*local);
            }
            StepKind::LetNil { local, value, .. } => {
                self.include_nil_expr(value);
                self.include_nil(*local);
            }
            StepKind::LetTuple { local, value, .. } => {
                self.include_tuple_expr(value);
                self.include_tuple(*local);
            }
            StepKind::LetList { value, .. } => self.include_list_local_expr(value),
            StepKind::LetIntFunction { local, value, .. } => {
                self.include_int_function_expr(value);
                self.include_int_function(*local);
            }
            StepKind::LetFloatFunction { local, value, .. } => {
                self.include_float_function_expr(value);
                self.include_float_function(*local);
            }
            StepKind::LetStringFunction { local, value, .. } => {
                self.include_string_function_expr(value);
                self.include_string_function(*local);
            }
            StepKind::LetBitArrayFunction { local, value, .. } => {
                self.include_bit_array_function_expr(value);
                self.include_bit_array_function(*local);
            }
            StepKind::LetUtfCodepointFunction { local, value, .. } => {
                self.include_utf_codepoint_function_expr(value);
                self.include_utf_codepoint_function(*local);
            }
            StepKind::LetCustomFunction { local, value, .. } => {
                self.include_custom_function_expr(value);
                self.include_custom_function(*local);
            }
            StepKind::LetBoolFunction { local, value, .. } => {
                self.include_bool_function_expr(value);
                self.include_bool_function(*local);
            }
            StepKind::LetNilFunction { local, value, .. } => {
                self.include_nil_function_expr(value);
                self.include_nil_function(*local);
            }
            StepKind::LetTupleFunction { local, value, .. } => {
                self.include_tuple_function_expr(value);
                self.include_tuple_function(*local);
            }
            StepKind::LetListFunction { local, value, .. } => {
                self.include_list_function_expr(value);
                self.include_list_function(local.clone());
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                self.include_function_function_expr(value);
                self.include_function_function(*local);
            }
            StepKind::AssertList {
                local,
                pattern,
                message,
                ..
            } => {
                self.include_list(local);
                self.include_assert_pattern(pattern);
                if let Some(message) = message {
                    self.include_string_expr(message);
                }
            }
            StepKind::AssertBitArray {
                local,
                pattern,
                message,
                ..
            } => {
                self.include_bit_array(*local);
                self.include_bit_array_assert_pattern(pattern);
                if let Some(message) = message {
                    self.include_string_expr(message);
                }
            }
            StepKind::AssertCustom {
                local,
                pattern,
                message,
                ..
            } => {
                self.include_custom(*local);
                self.include_assert_pattern(pattern);
                if let Some(message) = message {
                    self.include_string_expr(message);
                }
            }
            StepKind::BindCustomFields { local, pattern } => {
                self.include_custom(*local);
                self.include_custom_binding_pattern(pattern);
            }
            StepKind::AssertBool {
                condition, message, ..
            } => {
                self.include_bool_expr(condition);
                if let Some(message) = message {
                    self.include_string_expr(message);
                }
            }
            StepKind::Evaluate(value) => self.include_expr(value),
        }
    }

    fn include_custom_binding_pattern(&mut self, pattern: &CustomBindingPattern) {
        for field in pattern.fields() {
            self.include_total_binding_pattern(field);
        }
    }

    fn include_total_binding_pattern(&mut self, pattern: &TotalBindingPattern) {
        match pattern.kind() {
            crate::plan::module::TotalBindingPatternKind::Bind(binding) => {
                self.include_local(binding.local())
            }
            crate::plan::module::TotalBindingPatternKind::Discard => {}
            crate::plan::module::TotalBindingPatternKind::Tuple(elements) => {
                for element in elements {
                    self.include_total_binding_pattern(element);
                }
            }
            crate::plan::module::TotalBindingPatternKind::List(tail) => {
                if let ListAssertTail::Bind(binding) = tail {
                    self.include_list(binding.local());
                }
            }
            crate::plan::module::TotalBindingPatternKind::Custom(pattern) => {
                self.include_custom_binding_pattern(pattern)
            }
            crate::plan::module::TotalBindingPatternKind::Alias { pattern, binding } => {
                self.include_total_binding_pattern(pattern);
                self.include_local(binding.local());
            }
        }
    }

    fn include_list_assert_pattern(&mut self, pattern: &ListAssertPattern) {
        for element in pattern.elements() {
            self.include_assert_pattern(element);
        }
        if let Some(ListAssertTail::Bind(binding)) = pattern.tail() {
            self.include_list(binding.local());
        }
    }

    fn include_bit_array_assert_pattern(&mut self, pattern: &BitArrayAssertPattern) {
        match pattern {
            BitArrayAssertPattern::Pattern(pattern) => self.include_bit_array_pattern(pattern),
            BitArrayAssertPattern::Alias { pattern, local, .. } => {
                self.include_bit_array_assert_pattern(pattern);
                self.include_bit_array(*local);
            }
        }
    }

    pub(in crate::plan::module::frame) fn include_assert_pattern(
        &mut self,
        pattern: &AssertPattern,
    ) {
        match pattern {
            AssertPattern::Bind(binding) => self.include_local(binding.local()),
            AssertPattern::Discard
            | AssertPattern::Int(_)
            | AssertPattern::Float(_)
            | AssertPattern::String(_)
            | AssertPattern::Bool(_)
            | AssertPattern::Nil => {}
            AssertPattern::Tuple(elements) => {
                for element in elements {
                    self.include_assert_pattern(element);
                }
            }
            AssertPattern::List(pattern) => self.include_list_assert_pattern(pattern),
            AssertPattern::BitArray(pattern) => self.include_bit_array_pattern(pattern),
            AssertPattern::Custom(pattern) => {
                for field in pattern.fields() {
                    self.include_assert_pattern(field);
                }
            }
            AssertPattern::StringPrefix { left, right, .. } => {
                if let Some(binding) = left {
                    self.include_local(binding.local());
                }
                if let Some(binding) = right {
                    self.include_local(binding.local());
                }
            }
            AssertPattern::Alias { pattern, binding } => {
                self.include_assert_pattern(pattern);
                self.include_local(binding.local());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        AssertBinding, AssertPattern, BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolLocalId,
        Expr, IntExpr, IntFunctionId, IntLocalId, ListAssertPattern, ListAssertTail, ListLocal,
        NilExpr, NilFunctionExpr, NilFunctionLocalId, NilLocalId, PanicSite, ParamLocal,
        ReturnExpr, SourceSpan, StringExpr, StringFunctionExpr, StringFunctionLocalId,
        StringLocalId, TupleListLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_step_and_function_expression_families() {
        let steps = vec![
            crate::plan::Step::let_int(
                IntLocalId(6),
                "number".into(),
                IntExpr::bool_case(
                    BoolExpr::local_get(BoolLocalId(5), "use_true".into()),
                    IntExpr::int_case(
                        IntExpr::local_get(IntLocalId(4), "subject".into()),
                        vec![(1.into(), IntExpr::local_get(IntLocalId(3), "hit".into()))],
                        IntExpr::local_get(IntLocalId(2), "miss".into()),
                    ),
                    IntExpr::local_get(IntLocalId(1), "false_branch".into()),
                ),
            ),
            crate::plan::Step::let_string(
                StringLocalId(1),
                "text".into(),
                StringExpr::block(
                    Vec::new(),
                    StringExpr::call(crate::plan::StringFunctionId(0), Vec::new()),
                ),
            ),
            crate::plan::Step::let_bool(
                BoolLocalId(1),
                "flag".into(),
                BoolExpr::block(
                    Vec::new(),
                    BoolExpr::equal(
                        Expr::int(IntExpr::value(1.into())),
                        Expr::int(IntExpr::value(1.into())),
                    ),
                ),
            ),
            crate::plan::Step::let_nil(
                NilLocalId(1),
                "none".into(),
                NilExpr::block(
                    Vec::new(),
                    NilExpr::call(crate::plan::NilFunctionId(0), Vec::new()),
                ),
            ),
            crate::plan::Step::let_string_function(
                StringFunctionLocalId(2),
                "string_fn".into(),
                StringFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    StringFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), super::super::test_helpers::string_function_expr())],
                        super::super::test_helpers::string_function_expr(),
                    ),
                    StringFunctionExpr::block(
                        Vec::new(),
                        super::super::test_helpers::string_function_expr(),
                    ),
                ),
            ),
            crate::plan::Step::let_bool_function(
                BoolFunctionLocalId(2),
                "bool_fn".into(),
                BoolFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    BoolFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), super::super::test_helpers::bool_function_expr())],
                        super::super::test_helpers::bool_function_expr(),
                    ),
                    BoolFunctionExpr::block(
                        Vec::new(),
                        super::super::test_helpers::bool_function_expr(),
                    ),
                ),
            ),
            crate::plan::Step::let_nil_function(
                NilFunctionLocalId(2),
                "nil_fn".into(),
                NilFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    NilFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), super::super::test_helpers::nil_function_expr())],
                        super::super::test_helpers::nil_function_expr(),
                    ),
                    NilFunctionExpr::block(
                        Vec::new(),
                        super::super::test_helpers::nil_function_expr(),
                    ),
                ),
            ),
            crate::plan::Step::evaluate(Expr::string(StringExpr::function_call(
                StringFunctionExpr::local_get(
                    StringFunctionLocalId(3),
                    "string_fn".into(),
                    super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                ),
                Vec::new(),
            ))),
            crate::plan::Step::evaluate(Expr::bool(BoolExpr::function_call(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(3),
                    "bool_fn".into(),
                    super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                ),
                Vec::new(),
            ))),
            crate::plan::Step::evaluate(Expr::nil(NilExpr::function_call(
                NilFunctionExpr::local_get(
                    NilFunctionLocalId(3),
                    "nil_fn".into(),
                    super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                ),
                Vec::new(),
            ))),
            crate::plan::Step::evaluate(Expr::int(IntExpr::negate(IntExpr::value(1.into())))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 7);
        assert_eq!(layout.strings(), 2);
        assert_eq!(layout.bools(), 6);
        assert_eq!(layout.nils(), 2);
        assert_eq!(layout.string_functions(), 4);
        assert_eq!(layout.bool_functions(), 4);
        assert_eq!(layout.nil_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_assert_list_pattern_dependencies() {
        let tuple_type = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::String,
            ValueType::String,
        ];
        let list_element_type = ValueType::Tuple(tuple_type.clone());
        let subject_local = ListLocal::tuple(TupleListLocalId(0), tuple_type.clone());
        let tail_local = ListLocal::tuple(TupleListLocalId(1), tuple_type.clone());
        let steps = [crate::plan::Step::assert_list_at(
            subject_local,
            AssertPattern::list(ListAssertPattern::new(
                list_element_type.clone(),
                vec![AssertPattern::Tuple(vec![
                    AssertPattern::Bind(AssertBinding::new(
                        ParamLocal::int(IntLocalId(0)),
                        "number".into(),
                    )),
                    AssertPattern::alias(
                        AssertPattern::Discard,
                        AssertBinding::new(ParamLocal::string(StringLocalId(0)), "text".into()),
                    ),
                    AssertPattern::StringPrefix {
                        prefix: "pre".into(),
                        left: Some(AssertBinding::new(
                            ParamLocal::string(StringLocalId(1)),
                            "prefix".into(),
                        )),
                        right: Some(AssertBinding::new(
                            ParamLocal::string(StringLocalId(2)),
                            "suffix".into(),
                        )),
                    },
                    AssertPattern::StringPrefix {
                        prefix: "left".into(),
                        left: Some(AssertBinding::new(
                            ParamLocal::string(StringLocalId(3)),
                            "left_only".into(),
                        )),
                        right: None,
                    },
                ])],
                Some(ListAssertTail::bind(tail_local, "rest".into())),
            )),
            Some(StringExpr::local_get(StringLocalId(1), "message".into())),
            PanicSite::unknown(),
            SourceSpan::new(0, 0),
        )];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 1);
        assert_eq!(layout.strings(), 4);
        assert_eq!(layout.tuple_lists(), &[tuple_type.clone(), tuple_type,],);
    }

    #[test]
    fn frame_layout_includes_assert_bool_dependencies() {
        let steps = [crate::plan::Step::assert_bool_at(
            BoolExpr::local_get(BoolLocalId(0), "condition".into()),
            Some(StringExpr::local_get(StringLocalId(0), "message".into())),
            PanicSite::unknown(),
        )];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.strings(), 1);
    }
}
