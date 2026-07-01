use super::FrameLayout;
use crate::plan::{
    BoolExpr, BoolExprKind, Expr, ExprKind, IntExpr, IntExprKind, NilExpr, NilExprKind, StringExpr,
    StringExprKind,
};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_expr(&mut self, expression: &Expr) {
        match expression.kind() {
            ExprKind::Int(expression) => self.include_int_expr(expression),
            ExprKind::String(expression) => self.include_string_expr(expression),
            ExprKind::Bool(expression) => self.include_bool_expr(expression),
            ExprKind::Nil(expression) => self.include_nil_expr(expression),
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
            BoolExprKind::Not(value) => self.include_bool_expr(value),
            BoolExprKind::LtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::LtEqInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtEqInt { left, right } => self.include_int_binary_expr(left, right),
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
            NilExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_expr(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolLocalId, Expr, IntExpr, IntFunctionId, IntLocalId, NilExpr, NilLocalId,
        ReturnExpr, Step, StringExpr, StringLocalId,
    };

    #[test]
    fn frame_layout_includes_bool_operator_families() {
        let steps = vec![Step::evaluate(Expr::bool(BoolExpr::and(
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
        )))];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 9);
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
            Step::evaluate(Expr::bool(BoolExpr::int_case(
                IntExpr::local_get(IntLocalId(3), "bool_case_subject".into()),
                vec![(
                    0.into(),
                    BoolExpr::local_get(BoolLocalId(4), "bool_branch".into()),
                )],
                BoolExpr::local_get(BoolLocalId(5), "bool_fallback".into()),
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

        assert_eq!(layout.ints(), 15);
        assert_eq!(layout.strings(), 6);
        assert_eq!(layout.bools(), 8);
        assert_eq!(layout.nils(), 6);
    }
}
