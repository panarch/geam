mod function;
mod primitive;

use super::FrameLayout;
use crate::plan::{ReturnExpr, ReturnExprKind};

impl FrameLayout {
    pub(in crate::plan::module::frame) fn include_return_expr(&mut self, expression: &ReturnExpr) {
        match expression.kind() {
            ReturnExprKind::Int { body, .. } => self.include_int_return(body),
            ReturnExprKind::Float { body, .. } => self.include_float_return(body),
            ReturnExprKind::String { body, .. } => self.include_string_return(body),
            ReturnExprKind::Bool { body, .. } => self.include_bool_return(body),
            ReturnExprKind::Nil { body, .. } => self.include_nil_return(body),
            ReturnExprKind::Tuple { body, .. } => self.include_tuple_return(body),
            ReturnExprKind::IntList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::StringList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::FloatList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::BoolList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::NilList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::TupleList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::ListList { body, .. } => self.include_typed_list_return(body),
            ReturnExprKind::FunctionList { body, .. } => self.include_typed_list_return(body),
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
            ReturnExprKind::TupleFunction { body, .. } => {
                self.include_tuple_function_return(body);
            }
            ReturnExprKind::ListFunction { body, .. } => {
                self.include_list_function_return(body);
            }
            ReturnExprKind::FunctionFunction { body, .. } => {
                self.include_function_function_return(body);
            }
        }
    }
}
