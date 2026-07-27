mod arithmetic;
mod boolean;
mod equality;
mod ordering;
mod string;

use super::{plan_bool_expr, plan_int_expr};
use crate::plan::{BoolExpr, Expr, IntExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
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
        GleamBinOp::LtFloat => ordering::lt_float(left, right, context),
        GleamBinOp::LtEqFloat => ordering::lte_float(left, right, context),
        GleamBinOp::GtEqFloat => ordering::gte_float(left, right, context),
        GleamBinOp::GtFloat => ordering::gt_float(left, right, context),
        GleamBinOp::AddFloat => arithmetic::add_float(left, right, context),
        GleamBinOp::SubFloat => arithmetic::sub_float(left, right, context),
        GleamBinOp::MultFloat => arithmetic::mult_float(left, right, context),
        GleamBinOp::DivFloat => arithmetic::div_float(left, right, context),
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

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_int_expr, typed_prelude_constructor};
    use crate::planner::dsl::{
        function, host_call_site, int, int_arg, int_return_tail_call_at, local_bool, local_int,
        module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;

    #[test]
    fn plan_negation_expressions() {
        let source = r#"
pub fn negate(value: Int) {
  -value
}

pub fn invert(value: Bool) {
  !value
}

pub fn main() {
  negate(1)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call_at(
                    1,
                    [int_arg(int(1))],
                    host_call_site(source, "main", "negate(1)"),
                ),
            ),
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
            plan_module(module_returning_typed_expr(TypedExpr::NegateInt {
                location: dummy_span(),
                value: Box::new(typed_prelude_constructor("True", type_::bool())),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Bool,
                },
            }),
        );
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
}
