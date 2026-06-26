use crate::plan::{Expr, StringExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_core::ast::TypedExpr;

pub(super) fn concatenate(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::string(StringExpr::concatenate(
        super::super::plan_string_expr(left, context)?,
        super::super::plan_string_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::super::{module_returning_typed_expr, typed_int_expr, typed_string_expr};
    use crate::planner::dsl::{function, module, string};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
    use gleam_core::ast::{BinOp, TypedExpr};
    use gleam_core::type_;

    #[test]
    fn plan_string_concatenation() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  "hello, " <> "geam"
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", string("hello, ").concatenate(string("geam"))),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_string_operator_type_mismatch() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::string(),
                operator: BinOp::Concatenate,
                operator_start: 0,
                left: Box::new(typed_int_expr(1)),
                right: Box::new(typed_string_expr("bad")),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::string(),
                operator: BinOp::Concatenate,
                operator_start: 0,
                left: Box::new(typed_string_expr("bad")),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }
}
