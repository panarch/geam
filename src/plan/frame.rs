use super::expression::{
    BoolExpr, BoolFunctionExpr, CallArg, Expr, FunctionCallArg, FunctionExpr, FunctionFunctionExpr,
    IntExpr, IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use super::function::{Param, ParamLocal, ReturnExpr};
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId,
    NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
};
use super::step::Step;
use super::{
    BoolExprKind, BoolFunctionExprKind, CallArgKind, ExprKind, FunctionCallArgKind,
    FunctionExprKind, FunctionFunctionExprKind, IntExprKind, IntFunctionExprKind, NilExprKind,
    NilFunctionExprKind, ReturnExprKind, StepKind, StringExprKind, StringFunctionExprKind,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FrameLayout {
    ints: usize,
    strings: usize,
    bools: usize,
    nils: usize,
    int_functions: usize,
    string_functions: usize,
    bool_functions: usize,
    nil_functions: usize,
    function_functions: usize,
}

impl FrameLayout {
    pub(crate) fn from_function_parts(
        params: &[Param],
        steps: &[Step],
        return_: &ReturnExpr,
    ) -> Self {
        let mut layout = Self::default();

        for param in params {
            layout.include_local(param.local());
        }
        layout.include_steps(steps);
        layout.include_return_expr(return_);

        layout
    }

    pub(crate) fn include_local(&mut self, local: &ParamLocal) {
        match local {
            ParamLocal::Int(local) => self.include_int(*local),
            ParamLocal::String(local) => self.include_string(*local),
            ParamLocal::Bool(local) => self.include_bool(*local),
            ParamLocal::Nil(local) => self.include_nil(*local),
            ParamLocal::IntFunction { local, .. } => self.include_int_function(*local),
            ParamLocal::StringFunction { local, .. } => self.include_string_function(*local),
            ParamLocal::BoolFunction { local, .. } => self.include_bool_function(*local),
            ParamLocal::NilFunction { local, .. } => self.include_nil_function(*local),
            ParamLocal::FunctionFunction { local, .. } => self.include_function_function(*local),
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

    pub(crate) fn include_int_function(&mut self, local: IntFunctionLocalId) {
        self.int_functions = self.int_functions.max(local.0 + 1);
    }

    pub(crate) fn include_string_function(&mut self, local: StringFunctionLocalId) {
        self.string_functions = self.string_functions.max(local.0 + 1);
    }

    pub(crate) fn include_bool_function(&mut self, local: BoolFunctionLocalId) {
        self.bool_functions = self.bool_functions.max(local.0 + 1);
    }

    pub(crate) fn include_nil_function(&mut self, local: NilFunctionLocalId) {
        self.nil_functions = self.nil_functions.max(local.0 + 1);
    }

    pub(crate) fn include_function_function(&mut self, local: FunctionFunctionLocalId) {
        self.function_functions = self.function_functions.max(local.0 + 1);
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

    #[cfg(test)]
    pub(crate) fn nils(self) -> usize {
        self.nils
    }

    pub(crate) fn int_functions(self) -> usize {
        self.int_functions
    }

    pub(crate) fn string_functions(self) -> usize {
        self.string_functions
    }

    pub(crate) fn bool_functions(self) -> usize {
        self.bool_functions
    }

    pub(crate) fn nil_functions(self) -> usize {
        self.nil_functions
    }

    pub(crate) fn function_functions(self) -> usize {
        self.function_functions
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
            StepKind::LetIntFunction { local, value, .. } => {
                self.include_int_function_expr(value);
                self.include_int_function(*local);
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
            StepKind::LetFunctionFunction { local, value, .. } => {
                self.include_function_function_expr(value);
                self.include_function_function(*local);
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

    fn include_return_expr(&mut self, expression: &ReturnExpr) {
        match expression.kind() {
            ReturnExprKind::Int(expression) => self.include_int_expr(expression),
            ReturnExprKind::String(expression) => self.include_string_expr(expression),
            ReturnExprKind::Bool(expression) => self.include_bool_expr(expression),
            ReturnExprKind::Nil(expression) => self.include_nil_expr(expression),
            ReturnExprKind::Function(expression) => self.include_function_expr(expression),
        }
    }

    fn include_call_args(&mut self, args: &[CallArg]) {
        for arg in args {
            match arg.kind() {
                CallArgKind::Int { value, .. } => self.include_int_expr(value),
                CallArgKind::String { value, .. } => self.include_string_expr(value),
                CallArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CallArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CallArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CallArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CallArgKind::BoolFunction { value, .. } => self.include_bool_function_expr(value),
                CallArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
                CallArgKind::FunctionFunction { value, .. } => {
                    self.include_function_function_expr(value);
                }
            }
        }
    }

    fn include_function_call_args(&mut self, args: &[FunctionCallArg]) {
        for arg in args {
            match arg.kind() {
                FunctionCallArgKind::Int(value) => self.include_int_expr(value),
                FunctionCallArgKind::String(value) => self.include_string_expr(value),
                FunctionCallArgKind::Bool(value) => self.include_bool_expr(value),
                FunctionCallArgKind::Nil(value) => self.include_nil_expr(value),
                FunctionCallArgKind::IntFunction(value) => self.include_int_function_expr(value),
                FunctionCallArgKind::StringFunction(value) => {
                    self.include_string_function_expr(value);
                }
                FunctionCallArgKind::BoolFunction(value) => self.include_bool_function_expr(value),
                FunctionCallArgKind::NilFunction(value) => self.include_nil_function_expr(value),
                FunctionCallArgKind::FunctionFunction(value) => {
                    self.include_function_function_expr(value);
                }
            }
        }
    }

    fn include_int_expr(&mut self, expression: &IntExpr) {
        match expression.kind() {
            IntExprKind::Value(_) => {}
            IntExprKind::LocalGet { local, .. } => self.include_int(*local),
            IntExprKind::Call { args, .. } => self.include_call_args(args),
            IntExprKind::FunctionCall { function, args } => {
                self.include_int_function_expr(function);
                self.include_function_call_args(args);
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
                self.include_string_function_expr(function);
                self.include_function_call_args(args);
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
                self.include_bool_function_expr(function);
                self.include_function_call_args(args);
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
                self.include_nil_function_expr(function);
                self.include_function_call_args(args);
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
            FunctionExprKind::Int(expression) => self.include_int_function_expr(expression),
            FunctionExprKind::String(expression) => self.include_string_function_expr(expression),
            FunctionExprKind::Bool(expression) => self.include_bool_function_expr(expression),
            FunctionExprKind::Nil(expression) => self.include_nil_function_expr(expression),
            FunctionExprKind::Function(expression) => {
                self.include_function_function_expr(expression);
            }
        }
    }

    fn include_int_function_expr(&mut self, expression: &IntFunctionExpr) {
        match expression.kind() {
            IntFunctionExprKind::Value(_) => {}
            IntFunctionExprKind::LocalGet { local, .. } => self.include_int_function(*local),
            IntFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            IntFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_function_call_args(args);
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
            IntFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_function_expr(return_);
            }
        }
    }

    fn include_string_function_expr(&mut self, expression: &StringFunctionExpr) {
        match expression.kind() {
            StringFunctionExprKind::Value(_) => {}
            StringFunctionExprKind::LocalGet { local, .. } => {
                self.include_string_function(*local);
            }
            StringFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            StringFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_function_call_args(args);
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
            StringFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_function_expr(return_);
            }
        }
    }

    fn include_bool_function_expr(&mut self, expression: &BoolFunctionExpr) {
        match expression.kind() {
            BoolFunctionExprKind::Value(_) => {}
            BoolFunctionExprKind::LocalGet { local, .. } => self.include_bool_function(*local),
            BoolFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            BoolFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_function_call_args(args);
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
            BoolFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_function_expr(return_);
            }
        }
    }

    fn include_nil_function_expr(&mut self, expression: &NilFunctionExpr) {
        match expression.kind() {
            NilFunctionExprKind::Value(_) => {}
            NilFunctionExprKind::LocalGet { local, .. } => self.include_nil_function(*local),
            NilFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            NilFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_function_call_args(args);
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
            NilFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_function_expr(return_);
            }
        }
    }

    fn include_function_function_expr(&mut self, expression: &FunctionFunctionExpr) {
        match expression.kind() {
            FunctionFunctionExprKind::Value(_) => {}
            FunctionFunctionExprKind::LocalGet { local, .. } => {
                self.include_function_function(*local);
            }
            FunctionFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FunctionFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_function_call_args(args);
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
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolFunctionValue, BoolLocalId, Expr,
        FunctionExpr, IntExpr, IntFunctionExpr, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, NilExpr, NilFunctionExpr, NilFunctionLocalId,
        NilFunctionValue, NilLocalId, ParamLocal, ReturnExpr, Step, StringExpr, StringFunctionExpr,
        StringFunctionLocalId, StringFunctionValue, StringLocalId,
    };

    #[test]
    fn frame_layout_includes_local_ids() {
        let mut layout = FrameLayout::default();

        layout.include_local(&ParamLocal::int(IntLocalId(1)));
        layout.include_local(&ParamLocal::string(StringLocalId(2)));
        layout.include_local(&ParamLocal::bool(BoolLocalId(3)));
        layout.include_local(&ParamLocal::nil(NilLocalId(4)));
        layout.include_int_function(IntFunctionLocalId(5));
        layout.include_nil_function(NilFunctionLocalId(6));

        assert_eq!(layout.ints(), 2);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.bools(), 4);
        assert_eq!(layout.nils(), 5);
        assert_eq!(layout.int_functions(), 6);
        assert_eq!(layout.nil_functions(), 7);
    }

    #[test]
    fn frame_layout_includes_function_expression_nested_locals() {
        let nested_block = IntFunctionExpr::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(4),
                "value".into(),
            )))],
            int_function_expr(),
        );
        let nested_case = IntFunctionExpr::int_case(
            IntExpr::local_get(IntLocalId(3), "subject".into()),
            vec![(1.into(), nested_block)],
            int_function_expr(),
        );
        let function_case = IntFunctionExpr::bool_case(
            BoolExpr::local_get(BoolLocalId(2), "flag".into()),
            int_function_expr(),
            nested_case,
        );
        let steps = vec![Step::evaluate(Expr::function(FunctionExpr::int(
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::function(FunctionExpr::int(
                    function_case,
                )))],
                int_function_expr(),
            ),
        )))];
        let return_ = ReturnExpr::int(IntExpr::value(0.into()));

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
                int_function_expr().type_().clone(),
            ),
        )];
        let return_ = ReturnExpr::int(IntExpr::function_call(
            IntFunctionExpr::local_get(
                IntFunctionLocalId(3),
                "h".into(),
                int_function_expr().type_().clone(),
            ),
            Vec::new(),
        ));

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
                    string_function_expr().type_().clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(2),
                    "bool".into(),
                    bool_function_expr().type_().clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::local_get(
                    NilFunctionLocalId(3),
                    "nil".into(),
                    nil_function_expr().type_().clone(),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.string_functions(), 2);
        assert_eq!(layout.bool_functions(), 3);
        assert_eq!(layout.nil_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_step_and_function_expression_families() {
        let steps = vec![
            Step::let_string(
                StringLocalId(1),
                "text".into(),
                StringExpr::block(
                    Vec::new(),
                    StringExpr::call(crate::plan::StringFunctionId(0), Vec::new()),
                ),
            ),
            Step::let_bool(
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
            Step::let_nil(
                NilLocalId(1),
                "none".into(),
                NilExpr::block(
                    Vec::new(),
                    NilExpr::call(crate::plan::NilFunctionId(0), Vec::new()),
                ),
            ),
            Step::let_string_function(
                StringFunctionLocalId(2),
                "string_fn".into(),
                StringFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    StringFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), string_function_expr())],
                        string_function_expr(),
                    ),
                    StringFunctionExpr::block(Vec::new(), string_function_expr()),
                ),
            ),
            Step::let_bool_function(
                BoolFunctionLocalId(2),
                "bool_fn".into(),
                BoolFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    BoolFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), bool_function_expr())],
                        bool_function_expr(),
                    ),
                    BoolFunctionExpr::block(Vec::new(), bool_function_expr()),
                ),
            ),
            Step::let_nil_function(
                NilFunctionLocalId(2),
                "nil_fn".into(),
                NilFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    NilFunctionExpr::int_case(
                        IntExpr::value(1.into()),
                        vec![(1.into(), nil_function_expr())],
                        nil_function_expr(),
                    ),
                    NilFunctionExpr::block(Vec::new(), nil_function_expr()),
                ),
            ),
            Step::evaluate(Expr::string(StringExpr::function_call(
                StringFunctionExpr::local_get(
                    StringFunctionLocalId(3),
                    "string_fn".into(),
                    string_function_expr().type_().clone(),
                ),
                Vec::new(),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::function_call(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(3),
                    "bool_fn".into(),
                    bool_function_expr().type_().clone(),
                ),
                Vec::new(),
            ))),
            Step::evaluate(Expr::nil(NilExpr::function_call(
                NilFunctionExpr::local_get(
                    NilFunctionLocalId(3),
                    "nil_fn".into(),
                    nil_function_expr().type_().clone(),
                ),
                Vec::new(),
            ))),
            Step::evaluate(Expr::int(IntExpr::negate(IntExpr::value(1.into())))),
        ];
        let return_ = ReturnExpr::int(IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.strings(), 2);
        assert_eq!(layout.bools(), 2);
        assert_eq!(layout.nils(), 2);
        assert_eq!(layout.string_functions(), 4);
        assert_eq!(layout.bool_functions(), 4);
        assert_eq!(layout.nil_functions(), 4);
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            crate::plan::StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            crate::plan::BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            crate::plan::NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }
}
