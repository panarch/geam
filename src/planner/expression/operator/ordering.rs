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

pub(super) fn lt_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::lt_float(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn lte_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::lte_float(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn gt_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::gt_float(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn gte_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::gte_float(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{float, function, int, module};
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

    #[test]
    fn plan_float_ordering() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1.0 <. 2.0
}

pub fn lte() {
  1.0 <=. 2.0
}

pub fn gt() {
  2.0 >. 1.0
}

pub fn gte() {
  2.0 >=. 1.0
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", float(1.0).lt_float(float(2.0))),
            [
                function("lte", float(1.0).lte_float(float(2.0))),
                function("gt", float(2.0).gt_float(float(1.0))),
                function("gte", float(2.0).gte_float(float(1.0))),
            ],
        );

        assert_eq!(actual, expected);
    }
}
