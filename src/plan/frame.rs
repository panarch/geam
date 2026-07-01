use super::expression::{
    BoolExpr, BoolFunctionExpr, CallArg, CaptureArg, Expr, FunctionExpr, FunctionFunctionExpr,
    IntExpr, IntFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
};
use super::function::{Param, ParamLocal, ReturnExpr};
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId,
    NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
};
use super::step::Step;
use super::{
    BoolExprKind, BoolFunctionExprKind, CallArgKind, CaptureArgKind, ExprKind, FunctionExprKind,
    FunctionFunctionExprKind, IntExprKind, IntFunctionExprKind, NilExprKind, NilFunctionExprKind,
    ReturnBodyKind, ReturnExprKind, StepKind, StringExprKind, StringFunctionExprKind,
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
            ReturnExprKind::Int { body, .. } => self.include_int_return(body),
            ReturnExprKind::String { body, .. } => self.include_string_return(body),
            ReturnExprKind::Bool { body, .. } => self.include_bool_return(body),
            ReturnExprKind::Nil { body, .. } => self.include_nil_return(body),
            ReturnExprKind::IntFunction { body, .. } => {
                self.include_int_function_return(body);
            }
            ReturnExprKind::StringFunction { body, .. } => {
                self.include_string_function_return(body);
            }
            ReturnExprKind::BoolFunction { body, .. } => {
                self.include_bool_function_return(body);
            }
            ReturnExprKind::NilFunction { body, .. } => {
                self.include_nil_function_return(body);
            }
            ReturnExprKind::FunctionFunction { body, .. } => {
                self.include_function_function_return(body);
            }
        }
    }

    fn include_int_return(&mut self, body: &crate::plan::IntReturn) {
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

    fn include_string_return(&mut self, body: &crate::plan::StringReturn) {
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

    fn include_bool_return(&mut self, body: &crate::plan::BoolReturn) {
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

    fn include_nil_return(&mut self, body: &crate::plan::NilReturn) {
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

    fn include_int_function_return(&mut self, body: &crate::plan::IntFunctionReturn) {
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

    fn include_string_function_return(&mut self, body: &crate::plan::StringFunctionReturn) {
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

    fn include_bool_function_return(&mut self, body: &crate::plan::BoolFunctionReturn) {
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

    fn include_nil_function_return(&mut self, body: &crate::plan::NilFunctionReturn) {
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

    fn include_function_function_return(&mut self, body: &crate::plan::FunctionFunctionReturn) {
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

    fn include_capture_args(&mut self, args: &[CaptureArg]) {
        for arg in args {
            match arg.kind() {
                CaptureArgKind::Int { value, .. } => self.include_int_expr(value),
                CaptureArgKind::String { value, .. } => self.include_string_expr(value),
                CaptureArgKind::Bool { value, .. } => self.include_bool_expr(value),
                CaptureArgKind::Nil { value, .. } => self.include_nil_expr(value),
                CaptureArgKind::IntFunction { value, .. } => self.include_int_function_expr(value),
                CaptureArgKind::StringFunction { value, .. } => {
                    self.include_string_function_expr(value);
                }
                CaptureArgKind::BoolFunction { value, .. } => {
                    self.include_bool_function_expr(value)
                }
                CaptureArgKind::NilFunction { value, .. } => self.include_nil_function_expr(value),
                CaptureArgKind::FunctionFunction { value, .. } => {
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

    fn include_string_expr(&mut self, expression: &StringExpr) {
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

    fn include_bool_expr(&mut self, expression: &BoolExpr) {
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

    fn include_nil_expr(&mut self, expression: &NilExpr) {
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
            IntFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            IntFunctionExprKind::LocalGet { local, .. } => self.include_int_function(*local),
            IntFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            IntFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
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
            StringFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            StringFunctionExprKind::LocalGet { local, .. } => {
                self.include_string_function(*local);
            }
            StringFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            StringFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
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
            BoolFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            BoolFunctionExprKind::LocalGet { local, .. } => self.include_bool_function(*local),
            BoolFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            BoolFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
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
            NilFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            NilFunctionExprKind::LocalGet { local, .. } => self.include_nil_function(*local),
            NilFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            NilFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
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
            FunctionFunctionExprKind::Closure { captures, .. } => {
                self.include_capture_args(captures);
            }
            FunctionFunctionExprKind::LocalGet { local, .. } => {
                self.include_function_function(*local);
            }
            FunctionFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FunctionFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
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
