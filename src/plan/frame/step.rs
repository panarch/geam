use super::FrameLayout;
use crate::plan::{AssertPattern, ListAssertPattern, ListAssertTail, Step, StepKind};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_steps(&mut self, steps: &[Step]) {
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
            StepKind::LetList { local, value, .. } => {
                self.include_list_expr(value);
                self.include_list(*local);
            }
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
                self.include_list_function(*local);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                self.include_function_function_expr(value);
                self.include_function_function(*local);
            }
            StepKind::AssertList {
                local,
                pattern,
                message,
            } => {
                self.include_list(*local);
                self.include_assert_pattern(pattern);
                if let Some(message) = message {
                    self.include_string_expr(message);
                }
            }
            StepKind::Evaluate(value) => self.include_expr(value),
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

    fn include_assert_pattern(&mut self, pattern: &AssertPattern) {
        match pattern {
            AssertPattern::Bind(binding) => self.include_local(binding.local()),
            AssertPattern::Discard => {}
            AssertPattern::Tuple(elements) => {
                for element in elements {
                    self.include_assert_pattern(element);
                }
            }
            AssertPattern::List(pattern) => self.include_list_assert_pattern(pattern),
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
        Expr, IntExpr, IntFunctionId, IntLocalId, ListAssertPattern, ListAssertTail, ListLocalId,
        NilExpr, NilFunctionExpr, NilFunctionLocalId, NilLocalId, ParamLocal, ReturnExpr,
        StringExpr, StringFunctionExpr, StringFunctionLocalId, StringLocalId, ValueType,
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
        let list_element_type = ValueType::Tuple(vec![ValueType::Int, ValueType::String]);
        let steps = [crate::plan::Step::assert_list(
            ListLocalId(0),
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
                ])],
                Some(ListAssertTail::bind(ListLocalId(1), "rest".into())),
            )),
            Some(StringExpr::local_get(StringLocalId(1), "message".into())),
        )];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 1);
        assert_eq!(layout.strings(), 2);
        assert_eq!(layout.lists(), 2);
    }
}
