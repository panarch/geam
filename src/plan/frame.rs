use super::expression::{BoolExpr, CallArg, Expr, FunctionExpr, IntExpr, NilExpr, StringExpr};
use super::function::Param;
use super::id::{BoolLocalId, FunctionLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId};
use super::step::Step;
use super::{
    BoolExprKind, CallArgKind, ExprKind, FunctionExprKind, IntExprKind, NilExprKind, StepKind,
    StringExprKind,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FrameLayout {
    ints: usize,
    strings: usize,
    bools: usize,
    nils: usize,
    functions: usize,
}

impl FrameLayout {
    pub(crate) fn from_function_parts(params: &[Param], steps: &[Step], return_: &Expr) -> Self {
        let mut layout = Self::default();

        for param in params {
            layout.include_local(param.local());
        }
        layout.include_steps(steps);
        layout.include_expr(return_);

        layout
    }

    pub(crate) fn include_local(&mut self, local: LocalId) {
        match local {
            LocalId::Int(local) => self.include_int(local),
            LocalId::String(local) => self.include_string(local),
            LocalId::Bool(local) => self.include_bool(local),
            LocalId::Nil(local) => self.include_nil(local),
            LocalId::Function(local) => self.include_function(local),
        }
    }

    pub(crate) fn include_int(&mut self, local: IntLocalId) {
        self.ints = self.ints.max(local.0 + 1);
    }

    pub(crate) fn include_string(&mut self, local: StringLocalId) {
        self.strings = self.strings.max(local.0 + 1);
    }

    pub(crate) fn include_bool(&mut self, local: BoolLocalId) {
        self.bools = self.bools.max(local.0 + 1);
    }

    pub(crate) fn include_nil(&mut self, local: NilLocalId) {
        self.nils = self.nils.max(local.0 + 1);
    }

    pub(crate) fn include_function(&mut self, local: FunctionLocalId) {
        self.functions = self.functions.max(local.0 + 1);
    }

    pub(crate) fn ints(self) -> usize {
        self.ints
    }

    pub(crate) fn strings(self) -> usize {
        self.strings
    }

    pub(crate) fn bools(self) -> usize {
        self.bools
    }

    pub(crate) fn functions(self) -> usize {
        self.functions
    }

    #[cfg(test)]
    pub(crate) fn nils(self) -> usize {
        self.nils
    }

    fn include_steps(&mut self, steps: &[Step]) {
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
            StepKind::LetFunction { local, value, .. } => {
                self.include_function_expr(value);
                self.include_function(*local);
            }
            StepKind::Evaluate(value) => self.include_expr(value),
        }
    }

    fn include_expr(&mut self, expression: &Expr) {
        match expression.kind() {
            ExprKind::Int(expression) => self.include_int_expr(expression),
            ExprKind::String(expression) => self.include_string_expr(expression),
            ExprKind::Bool(expression) => self.include_bool_expr(expression),
            ExprKind::Nil(expression) => self.include_nil_expr(expression),
            ExprKind::Function(expression) => self.include_function_expr(expression),
        }
    }

    fn include_call_args(&mut self, args: &[CallArg]) {
        for arg in args {
            match arg.kind() {
                CallArgKind::Int { value, .. } => self.include_int_expr(value),
                CallArgKind::String { value, .. } => self.include_string_expr(value),
                CallArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CallArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CallArgKind::Function { value, .. } => self.include_function_expr(value),
            }
        }
    }

    fn include_int_expr(&mut self, expression: &IntExpr) {
        match expression.kind() {
            IntExprKind::Value(_) => {}
            IntExprKind::LocalGet { local, .. } => self.include_int(*local),
            IntExprKind::Call { args, .. } => self.include_call_args(args),
            IntExprKind::FunctionCall { function, args } => {
                self.include_function_expr(function);
                for arg in args {
                    self.include_expr(arg);
                }
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

    fn include_string_expr(&mut self, expression: &StringExpr) {
        match expression.kind() {
            StringExprKind::Value(_) => {}
            StringExprKind::LocalGet { local, .. } => self.include_string(*local),
            StringExprKind::Call { args, .. } => self.include_call_args(args),
            StringExprKind::FunctionCall { function, args } => {
                self.include_function_expr(function);
                for arg in args {
                    self.include_expr(arg);
                }
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

    fn include_bool_expr(&mut self, expression: &BoolExpr) {
        match expression.kind() {
            BoolExprKind::Value(_) => {}
            BoolExprKind::LocalGet { local, .. } => self.include_bool(*local),
            BoolExprKind::Call { args, .. } => self.include_call_args(args),
            BoolExprKind::FunctionCall { function, args } => {
                self.include_function_expr(function);
                for arg in args {
                    self.include_expr(arg);
                }
            }
            BoolExprKind::Not(value) => self.include_bool_expr(value),
            BoolExprKind::LtInt { left, right }
            | BoolExprKind::LtEqInt { left, right }
            | BoolExprKind::GtInt { left, right }
            | BoolExprKind::GtEqInt { left, right } => {
                self.include_int_expr(left);
                self.include_int_expr(right);
            }
            BoolExprKind::Equal { left, right } | BoolExprKind::NotEqual { left, right } => {
                self.include_expr(left);
                self.include_expr(right);
            }
            BoolExprKind::And { left, right } | BoolExprKind::Or { left, right } => {
                self.include_bool_expr(left);
                self.include_bool_expr(right);
            }
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

    fn include_nil_expr(&mut self, expression: &NilExpr) {
        match expression.kind() {
            NilExprKind::Value => {}
            NilExprKind::LocalGet { local, .. } => self.include_nil(*local),
            NilExprKind::Call { args, .. } => self.include_call_args(args),
            NilExprKind::FunctionCall { function, args } => {
                self.include_function_expr(function);
                for arg in args {
                    self.include_expr(arg);
                }
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

    fn include_function_expr(&mut self, expression: &FunctionExpr) {
        match expression.kind() {
            FunctionExprKind::Value(_) => {}
            FunctionExprKind::LocalGet { local, .. } => self.include_function(*local),
            FunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FunctionExprKind::FunctionCall { function, args } => {
                self.include_function_expr(function);
                for arg in args {
                    self.include_expr(arg);
                }
            }
            FunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_function_expr(true_);
                self.include_function_expr(false_);
            }
            FunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_expr(branch);
                }
                self.include_function_expr(fallback);
            }
            FunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_function_expr(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolLocalId, Expr, FunctionExpr, FunctionFunctionId, FunctionLocalId,
        FunctionType, FunctionValue, IntExpr, IntFunctionId, IntLocalId, LocalId, NilLocalId,
        RuntimeFunctionId, Step, StringLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_local_ids() {
        let mut layout = FrameLayout::default();

        layout.include_local(LocalId::Int(IntLocalId(1)));
        layout.include_local(LocalId::String(StringLocalId(2)));
        layout.include_local(LocalId::Bool(BoolLocalId(3)));
        layout.include_local(LocalId::Nil(NilLocalId(4)));
        layout.include_local(LocalId::Function(FunctionLocalId(5)));

        assert_eq!(layout.ints(), 2);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.bools(), 4);
        assert_eq!(layout.nils(), 5);
        assert_eq!(layout.functions(), 6);
    }

    #[test]
    fn frame_layout_includes_function_expression_locals() {
        let return_ = Expr::function(FunctionExpr::block(
            vec![Step::evaluate(Expr::function(
                FunctionExpr::bool_case(
                    BoolExpr::local_get(BoolLocalId(2), "flag".into()),
                    FunctionExpr::local_get(FunctionLocalId(1), "left".into(), function_type()),
                    FunctionExpr::int_case(
                        IntExpr::local_get(IntLocalId(3), "subject".into()),
                        vec![(
                            1.into(),
                            FunctionExpr::call(FunctionFunctionId(0), Vec::new(), function_type()),
                        )],
                        FunctionExpr::function_call(
                            FunctionExpr::value(function_value()),
                            vec![Expr::int(IntExpr::local_get(IntLocalId(4), "value".into()))],
                            function_type(),
                        ),
                    )
                    .expect("matching function branch types"),
                )
                .expect("matching function branch types"),
            ))],
            FunctionExpr::local_get(FunctionLocalId(5), "return".into(), function_type()),
        ));

        let layout = FrameLayout::from_function_parts(&[], &[], &return_);

        assert_eq!(layout.ints(), 5);
        assert_eq!(layout.bools(), 3);
        assert_eq!(layout.functions(), 6);
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            function_type(),
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }
}
