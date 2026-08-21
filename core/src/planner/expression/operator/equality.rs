use crate::plan::{BoolExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use gleam_compiler_core::ast::TypedExpr;

pub(super) fn equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let left = super::super::plan_expr(left, context)?;
    let right = super::super::plan_expr(right, context)?;
    Ok(Expr::bool(BoolExpr::equal(left, right)))
}

pub(super) fn not_equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let left = super::super::plan_expr(left, context)?;
    let right = super::super::plan_expr(right, context)?;
    Ok(Expr::bool(BoolExpr::not_equal(left, right)))
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{bool_, equal, function, int, module, not_equal};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{PlanError, UnsupportedBitArraySegmentReason};

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

    #[test]
    fn equality_operand_expression_errors_propagate() {
        for (name, src) in [
            (
                "equal left",
                r#"
pub fn main() {
  {
    <<1:native>>
    1
  } == 1
}
"#,
            ),
            (
                "equal right",
                r#"
pub fn main() {
  1 == {
    <<1:native>>
    1
  }
}
"#,
            ),
            (
                "not equal left",
                r#"
pub fn main() {
  {
    <<1:native>>
    1
  } != 1
}
"#,
            ),
            (
                "not equal right",
                r#"
pub fn main() {
  1 != {
    <<1:native>>
    1
  }
}
"#,
            ),
        ] {
            assert_eq!(
                expect_plan_error(src),
                PlanError::UnsupportedBitArraySegment {
                    reason: UnsupportedBitArraySegmentReason::NativeEndianness,
                },
                "{name}",
            );
        }
    }
}
