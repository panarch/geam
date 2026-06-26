use crate::plan::Expr;
use crate::planner::context::PlanContext;
use crate::planner::error::{PlanError, UnsupportedExpressionKind};
use gleam_core::ast::{FunctionLiteralKind, TypedArg, TypedStatement};
use gleam_core::type_::Type;
use std::sync::Arc;
use vec1::Vec1;

pub(super) fn plan_anonymous(
    _type_: Arc<Type>,
    _kind: FunctionLiteralKind,
    _arguments: Vec<TypedArg>,
    _body: Vec1<TypedStatement>,
    _context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Err(PlanError::UnsupportedExpression {
        kind: UnsupportedExpressionKind::AnonymousFunction,
    })
}

#[cfg(test)]
mod tests {
    use crate::planner::error::{PlanError, UnsupportedExpressionKind};
    use crate::planner::plan_module;
    use crate::planner::support::compile;

    #[test]
    fn reject_profile_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn(value) { value }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::AnonymousFunction,
            }),
        );
    }

    #[test]
    fn reject_profile_capturing_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::AnonymousFunction,
            }),
        );
    }
}
