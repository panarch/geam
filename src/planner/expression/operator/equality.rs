use crate::plan::{BoolExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_core::ast::TypedExpr;

pub(super) fn equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::equal(
        super::super::plan_expr(left, context)?,
        super::super::plan_expr(right, context)?,
    )))
}

pub(super) fn not_equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::bool(BoolExpr::not_equal(
        super::super::plan_expr(left, context)?,
        super::super::plan_expr(right, context)?,
    )))
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{bool_, equal, function, int, module, not_equal};
    use crate::planner::plan_module;
    use crate::planner::support::compile;

    #[test]
    fn plan_equality_operators() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1 == 1
}

pub fn not_equal() {
  True != False
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", equal(int(1), int(1))),
            [function("not_equal", not_equal(bool_(true), bool_(false)))],
        );

        assert_eq!(actual, expected);
    }
}
