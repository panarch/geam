mod function;
mod primitive;

use super::FrameLayout;
use crate::plan::{ReturnExpr, ReturnExprKind};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_return_expr(&mut self, expression: &ReturnExpr) {
        match expression.kind() {
            ReturnExprKind::Int { body, .. } => self.include_int_return(body),
            ReturnExprKind::Float { body, .. } => self.include_float_return(body),
            ReturnExprKind::String { body, .. } => self.include_string_return(body),
            ReturnExprKind::Bool { body, .. } => self.include_bool_return(body),
            ReturnExprKind::Nil { body, .. } => self.include_nil_return(body),
            ReturnExprKind::IntFunction { body, .. } => {
                self.include_int_function_return(body);
            }
            ReturnExprKind::FloatFunction { body, .. } => {
                self.include_float_function_return(body);
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
}
