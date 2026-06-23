mod arithmetic;
mod boolean;
mod equality;
mod ordering;
mod string;

use super::{plan_bool_expr, plan_int_expr};
use crate::plan::{BoolExpr, Expr, IntExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::{PlanError, UnsupportedBinOpKind};
use gleam_core::ast::{BinOp as GleamBinOp, TypedExpr};

pub(super) fn plan_bin_op(
    operator: GleamBinOp,
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match operator {
        GleamBinOp::AddInt => arithmetic::add(left, right, context),
        GleamBinOp::SubInt => arithmetic::sub(left, right, context),
        GleamBinOp::MultInt => arithmetic::mult(left, right, context),
        GleamBinOp::DivInt => arithmetic::div(left, right, context),
        GleamBinOp::RemainderInt => arithmetic::remainder(left, right, context),
        GleamBinOp::LtInt => ordering::lt(left, right, context),
        GleamBinOp::LtEqInt => ordering::lte(left, right, context),
        GleamBinOp::GtInt => ordering::gt(left, right, context),
        GleamBinOp::GtEqInt => ordering::gte(left, right, context),
        GleamBinOp::Eq => equality::equal(left, right, context),
        GleamBinOp::NotEq => equality::not_equal(left, right, context),
        GleamBinOp::Concatenate => string::concatenate(left, right, context),
        GleamBinOp::And => boolean::and(left, right, context),
        GleamBinOp::Or => boolean::or(left, right, context),
        GleamBinOp::LtFloat => unsupported(UnsupportedBinOpKind::LtFloat),
        GleamBinOp::LtEqFloat => unsupported(UnsupportedBinOpKind::LtEqFloat),
        GleamBinOp::GtEqFloat => unsupported(UnsupportedBinOpKind::GtEqFloat),
        GleamBinOp::GtFloat => unsupported(UnsupportedBinOpKind::GtFloat),
        GleamBinOp::AddFloat => unsupported(UnsupportedBinOpKind::AddFloat),
        GleamBinOp::SubFloat => unsupported(UnsupportedBinOpKind::SubFloat),
        GleamBinOp::MultFloat => unsupported(UnsupportedBinOpKind::MultFloat),
        GleamBinOp::DivFloat => unsupported(UnsupportedBinOpKind::DivFloat),
    }
}

pub(super) fn plan_negate_int(
    value: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::int(IntExpr::negate(plan_int_expr(value, context)?)))
}

pub(super) fn plan_negate_bool(
    value: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::not(plan_bool_expr(value, context)?)))
}

fn unsupported(operator: UnsupportedBinOpKind) -> Result<Expr, PlanError> {
    Err(PlanError::UnsupportedBinOp { operator })
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_int_expr};
    use crate::planner::dsl::{call_int, function, int, int_arg, local_bool, local_int, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedBinOpKind,
    };
    use gleam_core::ast::TypedExpr;

    #[test]
    fn plan_negation_expressions() {
        let actual = plan_module(compile(
            r#"
pub fn negate(value: Int) {
  -value
}

pub fn invert(value: Bool) {
  !value
}

pub fn main() {
  negate(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", call_int(1, [int_arg(0, int(1))])),
            [
                function("negate", local_int(0, "value").negate_int()).param_int(0, "value"),
                function("invert", local_bool(0, "value").negate_bool()).param_bool(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_negation_type_mismatch() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::NegateBool {
                location: dummy_span(),
                value: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_binary_operators() {
        let cases = [
            (
                r#"pub fn main() { 1.0 <. 2.0 }"#,
                UnsupportedBinOpKind::LtFloat,
            ),
            (
                r#"pub fn main() { 1.0 <=. 2.0 }"#,
                UnsupportedBinOpKind::LtEqFloat,
            ),
            (
                r#"pub fn main() { 1.0 >=. 2.0 }"#,
                UnsupportedBinOpKind::GtEqFloat,
            ),
            (
                r#"pub fn main() { 1.0 >. 2.0 }"#,
                UnsupportedBinOpKind::GtFloat,
            ),
            (
                r#"pub fn main() { 1.0 +. 2.0 }"#,
                UnsupportedBinOpKind::AddFloat,
            ),
            (
                r#"pub fn main() { 1.0 -. 2.0 }"#,
                UnsupportedBinOpKind::SubFloat,
            ),
            (
                r#"pub fn main() { 1.0 *. 2.0 }"#,
                UnsupportedBinOpKind::MultFloat,
            ),
            (
                r#"pub fn main() { 1.0 /. 2.0 }"#,
                UnsupportedBinOpKind::DivFloat,
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(
                expect_plan_error(src),
                PlanError::UnsupportedBinOp { operator: expected },
            );
        }
    }
}
