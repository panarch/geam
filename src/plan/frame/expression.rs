use super::FrameLayout;
use crate::plan::{
    BoolExpr, BoolExprKind, Expr, ExprKind, FloatExpr, FloatExprKind, IntExpr, IntExprKind,
    NilExpr, NilExprKind, StringExpr, StringExprKind, TupleExpr, TupleExprKind,
};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_expr(&mut self, expression: &Expr) {
        match expression.kind() {
            ExprKind::Int(expression) => self.include_int_expr(expression),
            ExprKind::String(expression) => self.include_string_expr(expression),
            ExprKind::Float(expression) => self.include_float_expr(expression),
            ExprKind::Bool(expression) => self.include_bool_expr(expression),
            ExprKind::Nil(expression) => self.include_nil_expr(expression),
            ExprKind::Tuple(expression) => self.include_tuple_expr(expression),
            ExprKind::Function(expression) => self.include_function_expr(expression),
        }
    }

    pub(in crate::plan::frame) fn include_int_expr(&mut self, expression: &IntExpr) {
        match expression.kind() {
            IntExprKind::Value(_) => {}
            IntExprKind::LocalGet { local, .. } => self.include_int(*local),
            IntExprKind::Call { args, .. } => self.include_call_args(args),
            IntExprKind::FunctionCall { function, args } => {
                self.include_int_function_expr(function);
                self.include_call_args(args);
            }
            IntExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            IntExprKind::Add { left, right }
            | IntExprKind::Sub { left, right }
            | IntExprKind::Mult { left, right }
            | IntExprKind::Div { left, right }
            | IntExprKind::Remainder { left, right } => {
                self.include_int_expr(left);
                self.include_int_expr(right);
            }
            IntExprKind::Negate(value) => self.include_int_expr(value),
            IntExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_expr(true_);
                self.include_int_expr(false_);
            }
            IntExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_expr(&mut self, expression: &StringExpr) {
        match expression.kind() {
            StringExprKind::Value(_) => {}
            StringExprKind::LocalGet { local, .. } => self.include_string(*local),
            StringExprKind::Call { args, .. } => self.include_call_args(args),
            StringExprKind::FunctionCall { function, args } => {
                self.include_string_function_expr(function);
                self.include_call_args(args);
            }
            StringExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            StringExprKind::Concatenate { left, right } => {
                self.include_string_expr(left);
                self.include_string_expr(right);
            }
            StringExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_expr(true_);
                self.include_string_expr(false_);
            }
            StringExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_expr(&mut self, expression: &BoolExpr) {
        match expression.kind() {
            BoolExprKind::Value(_) => {}
            BoolExprKind::LocalGet { local, .. } => self.include_bool(*local),
            BoolExprKind::Call { args, .. } => self.include_call_args(args),
            BoolExprKind::FunctionCall { function, args } => {
                self.include_bool_function_expr(function);
                self.include_call_args(args);
            }
            BoolExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            BoolExprKind::Not(value) => self.include_bool_expr(value),
            BoolExprKind::LtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::LtEqInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtEqInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::LtFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::LtEqFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::GtFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::GtEqFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::Equal { left, right } => self.include_binary_expr(left, right),
            BoolExprKind::NotEqual { left, right } => self.include_binary_expr(left, right),
            BoolExprKind::And { left, right } => self.include_bool_binary_expr(left, right),
            BoolExprKind::Or { left, right } => self.include_bool_binary_expr(left, right),
            BoolExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_expr(true_);
                self.include_bool_expr(false_);
            }
            BoolExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_expr(return_);
            }
        }
    }

    fn include_int_binary_expr(&mut self, left: &IntExpr, right: &IntExpr) {
        self.include_int_expr(left);
        self.include_int_expr(right);
    }

    fn include_float_binary_expr(&mut self, left: &FloatExpr, right: &FloatExpr) {
        self.include_float_expr(left);
        self.include_float_expr(right);
    }

    fn include_binary_expr(&mut self, left: &Expr, right: &Expr) {
        self.include_expr(left);
        self.include_expr(right);
    }

    fn include_bool_binary_expr(&mut self, left: &BoolExpr, right: &BoolExpr) {
        self.include_bool_expr(left);
        self.include_bool_expr(right);
    }

    pub(in crate::plan::frame) fn include_nil_expr(&mut self, expression: &NilExpr) {
        match expression.kind() {
            NilExprKind::Value => {}
            NilExprKind::LocalGet { local, .. } => self.include_nil(*local),
            NilExprKind::Call { args, .. } => self.include_call_args(args),
            NilExprKind::FunctionCall { function, args } => {
                self.include_nil_function_expr(function);
                self.include_call_args(args);
            }
            NilExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            NilExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_expr(true_);
                self.include_nil_expr(false_);
            }
            NilExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_float_expr(&mut self, expression: &FloatExpr) {
        match expression.kind() {
            FloatExprKind::Value(_) => {}
            FloatExprKind::LocalGet { local, .. } => self.include_float(*local),
            FloatExprKind::Call { args, .. } => self.include_call_args(args),
            FloatExprKind::FunctionCall { function, args } => {
                self.include_float_function_expr(function);
                self.include_call_args(args);
            }
            FloatExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            FloatExprKind::Add { left, right }
            | FloatExprKind::Sub { left, right }
            | FloatExprKind::Mult { left, right }
            | FloatExprKind::Div { left, right } => {
                self.include_float_expr(left);
                self.include_float_expr(right);
            }
            FloatExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_float_expr(true_);
                self.include_float_expr(false_);
            }
            FloatExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_float_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_tuple_expr(&mut self, expression: &TupleExpr) {
        match expression.kind() {
            TupleExprKind::Value(elements) => {
                for element in elements {
                    self.include_expr(element);
                }
            }
            TupleExprKind::LocalGet { local, .. } => self.include_tuple(*local),
            TupleExprKind::Call { args, .. } => self.include_call_args(args),
            TupleExprKind::FunctionCall { function, args } => {
                self.include_tuple_function_expr(function);
                self.include_call_args(args);
            }
            TupleExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            TupleExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_tuple_expr(true_);
                self.include_tuple_expr(false_);
            }
            TupleExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_tuple_expr(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolLocalId, CallArg, Expr, FloatExpr, FloatFunctionId, FloatFunctionLocalId,
        FloatLocalId, FunctionType, IntExpr, IntFunctionId, IntLocalId, NilExpr, NilLocalId,
        ReturnExpr, Step, StringExpr, StringLocalId, TupleExpr, TupleFunctionExpr, TupleFunctionId,
        TupleFunctionLocalId, TupleLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_bool_operator_families() {
        let steps = vec![Step::evaluate(Expr::bool(BoolExpr::and(
            BoolExpr::and(
                BoolExpr::and(
                    BoolExpr::lte_int(
                        IntExpr::local_get(IntLocalId(1), "lte_left".into()),
                        IntExpr::local_get(IntLocalId(2), "lte_right".into()),
                    ),
                    BoolExpr::gt_int(
                        IntExpr::local_get(IntLocalId(3), "gt_left".into()),
                        IntExpr::local_get(IntLocalId(4), "gt_right".into()),
                    ),
                ),
                BoolExpr::and(
                    BoolExpr::gte_int(
                        IntExpr::local_get(IntLocalId(5), "gte_left".into()),
                        IntExpr::local_get(IntLocalId(6), "gte_right".into()),
                    ),
                    BoolExpr::not_equal(
                        Expr::int(IntExpr::local_get(IntLocalId(7), "not_equal_left".into())),
                        Expr::int(IntExpr::local_get(IntLocalId(8), "not_equal_right".into())),
                    ),
                ),
            ),
            BoolExpr::and(
                BoolExpr::lte_float(
                    FloatExpr::local_get(FloatLocalId(0), "float_lte_left".into()),
                    FloatExpr::local_get(FloatLocalId(1), "float_lte_right".into()),
                ),
                BoolExpr::gt_float(
                    FloatExpr::local_get(FloatLocalId(2), "float_gt_left".into()),
                    FloatExpr::local_get(FloatLocalId(3), "float_gt_right".into()),
                ),
            ),
        )))];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 9);
        assert_eq!(layout.floats(), 4);
    }

    #[test]
    fn frame_layout_includes_primitive_case_and_block_families() {
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(6), "int_bool_case_subject".into()),
                IntExpr::local_get(IntLocalId(7), "int_bool_true".into()),
                IntExpr::local_get(IntLocalId(8), "int_bool_false".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::int_case(
                IntExpr::local_get(IntLocalId(9), "int_case_subject".into()),
                vec![
                    (
                        0.into(),
                        IntExpr::local_get(IntLocalId(10), "int_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        IntExpr::local_get(IntLocalId(11), "int_branch_one".into()),
                    ),
                ],
                IntExpr::local_get(IntLocalId(12), "int_fallback".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::string_case(
                StringExpr::local_get(StringLocalId(6), "int_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        IntExpr::local_get(IntLocalId(15), "int_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        IntExpr::local_get(IntLocalId(16), "int_string_branch_two".into()),
                    ),
                ],
                IntExpr::local_get(IntLocalId(17), "int_string_fallback".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::float_case(
                FloatExpr::local_get(FloatLocalId(18), "int_float_case_subject".into()),
                vec![(
                    1.0,
                    IntExpr::local_get(IntLocalId(18), "int_float_branch".into()),
                )],
                IntExpr::local_get(IntLocalId(19), "int_float_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(0), "string_case_subject".into()),
                StringExpr::local_get(StringLocalId(1), "string_true".into()),
                StringExpr::local_get(StringLocalId(2), "string_false".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::int_case(
                IntExpr::local_get(IntLocalId(13), "string_case_subject".into()),
                vec![
                    (
                        0.into(),
                        StringExpr::local_get(StringLocalId(3), "string_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        StringExpr::local_get(StringLocalId(4), "string_branch_one".into()),
                    ),
                ],
                StringExpr::local_get(StringLocalId(5), "string_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::string_case(
                StringExpr::local_get(StringLocalId(7), "string_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        StringExpr::local_get(StringLocalId(8), "string_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        StringExpr::local_get(StringLocalId(9), "string_string_branch_two".into()),
                    ),
                ],
                StringExpr::local_get(StringLocalId(10), "string_string_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::float_case(
                FloatExpr::local_get(FloatLocalId(19), "string_float_case_subject".into()),
                vec![(
                    1.0,
                    StringExpr::local_get(StringLocalId(13), "string_float_branch".into()),
                )],
                StringExpr::local_get(StringLocalId(14), "string_float_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::int_case(
                IntExpr::local_get(IntLocalId(3), "bool_case_subject".into()),
                vec![(
                    0.into(),
                    BoolExpr::local_get(BoolLocalId(4), "bool_branch".into()),
                )],
                BoolExpr::local_get(BoolLocalId(5), "bool_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::string_case(
                StringExpr::local_get(StringLocalId(11), "bool_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        BoolExpr::local_get(BoolLocalId(8), "bool_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        BoolExpr::local_get(BoolLocalId(9), "bool_string_branch_two".into()),
                    ),
                ],
                BoolExpr::local_get(BoolLocalId(10), "bool_string_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::float_case(
                FloatExpr::local_get(FloatLocalId(20), "bool_float_case_subject".into()),
                vec![(
                    1.0,
                    BoolExpr::local_get(BoolLocalId(11), "bool_float_branch".into()),
                )],
                BoolExpr::local_get(BoolLocalId(12), "bool_float_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(7), "nil_bool_case_subject".into()),
                NilExpr::local_get(NilLocalId(1), "nil_bool_true".into()),
                NilExpr::local_get(NilLocalId(2), "nil_bool_false".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::int_case(
                IntExpr::local_get(IntLocalId(14), "nil_case_subject".into()),
                vec![
                    (
                        0.into(),
                        NilExpr::local_get(NilLocalId(3), "nil_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        NilExpr::local_get(NilLocalId(4), "nil_branch_one".into()),
                    ),
                ],
                NilExpr::local_get(NilLocalId(5), "nil_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::string_case(
                StringExpr::local_get(StringLocalId(12), "nil_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        NilExpr::local_get(NilLocalId(6), "nil_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        NilExpr::local_get(NilLocalId(7), "nil_string_branch_two".into()),
                    ),
                ],
                NilExpr::local_get(NilLocalId(8), "nil_string_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::float_case(
                FloatExpr::local_get(FloatLocalId(21), "nil_float_case_subject".into()),
                vec![(
                    1.0,
                    NilExpr::local_get(NilLocalId(9), "nil_float_branch".into()),
                )],
                NilExpr::local_get(NilLocalId(10), "nil_float_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::add(
                FloatExpr::local_get(FloatLocalId(22), "float_add_left".into()),
                FloatExpr::sub(
                    FloatExpr::local_get(FloatLocalId(23), "float_sub_left".into()),
                    FloatExpr::mult(
                        FloatExpr::local_get(FloatLocalId(24), "float_mult_left".into()),
                        FloatExpr::div(
                            FloatExpr::local_get(FloatLocalId(25), "float_div_left".into()),
                            FloatExpr::local_get(FloatLocalId(26), "float_div_right".into()),
                        ),
                    ),
                ),
            ))),
            Step::evaluate(Expr::float(FloatExpr::call(
                FloatFunctionId(0),
                vec![CallArg::float(
                    FloatLocalId(0),
                    FloatExpr::local_get(FloatLocalId(27), "float_call_arg".into()),
                )],
            ))),
            Step::evaluate(Expr::float(FloatExpr::function_call(
                crate::plan::FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(0),
                    "float_function".into(),
                    super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                ),
                vec![CallArg::float(
                    FloatLocalId(1),
                    FloatExpr::local_get(FloatLocalId(28), "float_function_call_arg".into()),
                )],
            ))),
            Step::evaluate(Expr::float(FloatExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(13), "float_bool_case_subject".into()),
                FloatExpr::local_get(FloatLocalId(29), "float_bool_true".into()),
                FloatExpr::local_get(FloatLocalId(30), "float_bool_false".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::int_case(
                IntExpr::local_get(IntLocalId(20), "float_int_case_subject".into()),
                vec![(
                    1.into(),
                    FloatExpr::local_get(FloatLocalId(31), "float_int_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(32), "float_int_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::string_case(
                StringExpr::local_get(StringLocalId(15), "float_string_case_subject".into()),
                vec![(
                    "one".into(),
                    FloatExpr::local_get(FloatLocalId(33), "float_string_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(34), "float_string_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::float_case(
                FloatExpr::local_get(FloatLocalId(35), "float_case_subject".into()),
                vec![(
                    1.0,
                    FloatExpr::local_get(FloatLocalId(36), "float_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(37), "float_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(21),
                    "float_block_step".into(),
                )))],
                FloatExpr::local_get(FloatLocalId(38), "float_block_return".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::int(IntExpr::local_get(
                        IntLocalId(25),
                        "int_tuple_index_value".into(),
                    ))],
                    vec![ValueType::Int],
                ),
                0,
            ))),
            Step::evaluate(Expr::string(StringExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::string(StringExpr::local_get(
                        StringLocalId(18),
                        "string_tuple_index_value".into(),
                    ))],
                    vec![ValueType::String],
                ),
                0,
            ))),
            Step::evaluate(Expr::float(FloatExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::float(FloatExpr::local_get(
                        FloatLocalId(40),
                        "float_tuple_index_value".into(),
                    ))],
                    vec![ValueType::Float],
                ),
                0,
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::value(
                vec![
                    Expr::int(IntExpr::local_get(IntLocalId(22), "tuple_int".into())),
                    Expr::string(StringExpr::local_get(
                        StringLocalId(16),
                        "tuple_string".into(),
                    )),
                ],
                vec![ValueType::Int, ValueType::String],
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::local_get(
                TupleLocalId(0),
                "tuple_local".into(),
                vec![ValueType::Int],
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::call(
                TupleFunctionId(0),
                vec![CallArg::tuple(
                    TupleLocalId(1),
                    TupleExpr::local_get(TupleLocalId(2), "tuple_call_arg".into(), tuple_type()),
                )],
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::function_call(
                TupleFunctionExpr::local_get(
                    TupleFunctionLocalId(0),
                    "tuple_function".into(),
                    tuple_function_type(),
                ),
                vec![CallArg::tuple(
                    TupleLocalId(3),
                    TupleExpr::local_get(
                        TupleLocalId(4),
                        "tuple_function_call_arg".into(),
                        tuple_type(),
                    ),
                )],
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::tuple_index(
                TupleExpr::local_get(
                    TupleLocalId(5),
                    "tuple_index_subject".into(),
                    vec![ValueType::Tuple(tuple_type())],
                ),
                0,
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(14), "tuple_bool_case_subject".into()),
                TupleExpr::local_get(TupleLocalId(6), "tuple_bool_true".into(), tuple_type()),
                TupleExpr::local_get(TupleLocalId(7), "tuple_bool_false".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::int_case(
                IntExpr::local_get(IntLocalId(23), "tuple_int_case_subject".into()),
                vec![(
                    1.into(),
                    TupleExpr::local_get(TupleLocalId(8), "tuple_int_branch".into(), tuple_type()),
                )],
                TupleExpr::local_get(TupleLocalId(9), "tuple_int_fallback".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::string_case(
                StringExpr::local_get(StringLocalId(17), "tuple_string_case_subject".into()),
                vec![(
                    "one".into(),
                    TupleExpr::local_get(
                        TupleLocalId(10),
                        "tuple_string_branch".into(),
                        tuple_type(),
                    ),
                )],
                TupleExpr::local_get(
                    TupleLocalId(11),
                    "tuple_string_fallback".into(),
                    tuple_type(),
                ),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::float_case(
                FloatExpr::local_get(FloatLocalId(39), "tuple_float_case_subject".into()),
                vec![(
                    1.0,
                    TupleExpr::local_get(
                        TupleLocalId(12),
                        "tuple_float_branch".into(),
                        tuple_type(),
                    ),
                )],
                TupleExpr::local_get(
                    TupleLocalId(13),
                    "tuple_float_fallback".into(),
                    tuple_type(),
                ),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(24),
                    "tuple_block_step".into(),
                )))],
                TupleExpr::local_get(TupleLocalId(14), "tuple_block_return".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(6),
                    "nil_block_step".into(),
                )))],
                NilExpr::local_get(NilLocalId(0), "nil_return".into()),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 26);
        assert_eq!(layout.floats(), 41);
        assert_eq!(layout.strings(), 19);
        assert_eq!(layout.bools(), 15);
        assert_eq!(layout.nils(), 11);
        assert_eq!(layout.tuples(), 15);
        assert_eq!(layout.float_functions(), 1);
        assert_eq!(layout.tuple_functions(), 1);
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int]
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(tuple_type())],
            ValueType::Tuple(tuple_type()),
        )
    }
}
