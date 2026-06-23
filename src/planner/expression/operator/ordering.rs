use crate::plan::{BoolExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_core::ast::TypedExpr;

pub(super) fn lt(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::lt_int(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn lte(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::lte_int(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn gt(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::gt_int(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

pub(super) fn gte(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::gte_int(
        super::super::plan_int_expr(left, context)?,
        super::super::plan_int_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{function, int, module};
    use crate::planner::plan_module;
    use crate::planner::support::compile;

    #[test]
    fn plan_integer_ordering() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1 < 2
}

pub fn lte() {
  1 <= 2
}

pub fn gt() {
  2 > 1
}

pub fn gte() {
  2 >= 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1).lt_int(int(2))),
            [
                function("lte", int(1).lte_int(int(2))),
                function("gt", int(2).gt_int(int(1))),
                function("gte", int(2).gte_int(int(1))),
            ],
        );

        assert_eq!(actual, expected);
    }
}
