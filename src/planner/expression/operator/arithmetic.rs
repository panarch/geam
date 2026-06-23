use crate::plan::{Expr, IntExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_core::ast::TypedExpr;

pub(super) fn add(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::add(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn sub(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::sub(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn mult(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::mult(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn div(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::div(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn remainder(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::remainder(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        module_returning_typed_expr, typed_int_expr, typed_prelude_constructor, typed_string_expr,
    };
    use crate::planner::dsl::{function, int, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
    use gleam_core::ast::{BinOp, TypedExpr};
    use gleam_core::type_;

    #[test]
    fn plan_integer_arithmetic() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1 + 2
}

pub fn sub() {
  3 - 2
}

pub fn mult() {
  3 * 2
}

pub fn div() {
  11 / 3
}

pub fn remainder() {
  11 % 3
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1).add_int(int(2))),
            [
                function("sub", int(3).sub_int(int(2))),
                function("mult", int(3).mult_int(int(2))),
                function("div", int(11).div_int(int(3))),
                function("remainder", int(11).remainder_int(int(3))),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_integer_arithmetic_type_mismatch() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_string_expr("bad")),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_prelude_constructor("True", type_::bool())),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Bool,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_prelude_constructor("Nil", type_::nil())),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Nil,
                },
            }),
        );
    }
}
