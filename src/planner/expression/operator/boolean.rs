use crate::plan::{BoolExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_core::ast::TypedExpr;

pub(super) fn and(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::and(
        super::super::plan_bool_expr(left, context)?,
        super::super::plan_bool_expr(right, context)?,
    )))
}

pub(super) fn or(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::or(
        super::super::plan_bool_expr(left, context)?,
        super::super::plan_bool_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        module_returning_typed_expr, typed_int_expr, typed_prelude_constructor,
    };
    use crate::planner::dsl::{bool_, function, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use gleam_core::ast::{BinOp, TypedExpr};
    use gleam_core::type_;

    #[test]
    fn plan_bool_operators() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  True && False
}

pub fn or_op() {
  False || True
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", bool_(true).and_bool(bool_(false))),
            [function("or_op", bool_(false).or_bool(bool_(true)))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_bool_operator_operands() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  False && { True }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Block,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  True || { False }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Block,
            },
        );
    }

    #[test]
    fn reject_margin_bool_operator_type_mismatch() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::bool(),
                operator: BinOp::And,
                operator_start: 0,
                left: Box::new(typed_int_expr(1)),
                right: Box::new(typed_prelude_constructor("True", type_::bool())),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::bool(),
                operator: BinOp::Or,
                operator_start: 0,
                left: Box::new(typed_prelude_constructor("True", type_::bool())),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }
}
