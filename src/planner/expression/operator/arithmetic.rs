use crate::plan::{Expr, FloatExpr, IntExpr};
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

pub(super) fn add_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::float(FloatExpr::add(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn sub_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::float(FloatExpr::sub(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn mult_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::float(FloatExpr::mult(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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

pub(super) fn div_float(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    Ok(Expr::float(FloatExpr::div(
        super::super::plan_float_expr(left, context)?,
        super::super::plan_float_expr(right, context)?,
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
    use crate::planner::dsl::{float, function, int, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
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
    fn plan_float_arithmetic() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1.0 +. 2.0
}

pub fn sub() {
  3.0 -. 2.0
}

pub fn mult() {
  3.0 *. 2.0
}

pub fn div() {
  11.0 /. 2.0
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", float(1.0).add_float(float(2.0))),
            [
                function("sub", float(3.0).sub_float(float(2.0))),
                function("mult", float(3.0).mult_float(float(2.0))),
                function("div", float(11.0).div_float(float(2.0))),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_arithmetic_operand_expression_errors_propagate() {
        for (name, src) in [
            (
                "add int left",
                r#"
pub fn main() {
  {
    echo 1
    1
  } + 1
}
"#,
            ),
            (
                "add int right",
                r#"
pub fn main() {
  1 + {
    echo 1
    1
  }
}
"#,
            ),
            (
                "sub int left",
                r#"
pub fn main() {
  {
    echo 1
    1
  } - 1
}
"#,
            ),
            (
                "sub int right",
                r#"
pub fn main() {
  1 - {
    echo 1
    1
  }
}
"#,
            ),
            (
                "mult int left",
                r#"
pub fn main() {
  {
    echo 1
    1
  } * 1
}
"#,
            ),
            (
                "mult int right",
                r#"
pub fn main() {
  1 * {
    echo 1
    1
  }
}
"#,
            ),
            (
                "div int left",
                r#"
pub fn main() {
  {
    echo 1
    1
  } / 1
}
"#,
            ),
            (
                "div int right",
                r#"
pub fn main() {
  1 / {
    echo 1
    1
  }
}
"#,
            ),
            (
                "remainder int left",
                r#"
pub fn main() {
  {
    echo 1
    1
  } % 1
}
"#,
            ),
            (
                "remainder int right",
                r#"
pub fn main() {
  1 % {
    echo 1
    1
  }
}
"#,
            ),
            (
                "add float left",
                r#"
pub fn main() {
  {
    echo 1
    1.0
  } +. 1.0
}
"#,
            ),
            (
                "add float right",
                r#"
pub fn main() {
  1.0 +. {
    echo 1
    1.0
  }
}
"#,
            ),
            (
                "sub float left",
                r#"
pub fn main() {
  {
    echo 1
    1.0
  } -. 1.0
}
"#,
            ),
            (
                "sub float right",
                r#"
pub fn main() {
  1.0 -. {
    echo 1
    1.0
  }
}
"#,
            ),
            (
                "mult float left",
                r#"
pub fn main() {
  {
    echo 1
    1.0
  } *. 1.0
}
"#,
            ),
            (
                "mult float right",
                r#"
pub fn main() {
  1.0 *. {
    echo 1
    1.0
  }
}
"#,
            ),
            (
                "div float left",
                r#"
pub fn main() {
  {
    echo 1
    1.0
  } /. 1.0
}
"#,
            ),
            (
                "div float right",
                r#"
pub fn main() {
  1.0 /. {
    echo 1
    1.0
  }
}
"#,
            ),
        ] {
            assert_echo_reject(name, src);
        }
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

    fn assert_echo_reject(name: &str, src: &str) {
        assert_eq!(
            expect_plan_error(src),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
            "{name}",
        );
    }
}
