use crate::plan::{BoolExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::{PlanError, UnsupportedBinOpKind};
use gleam_core::ast::TypedExpr;

pub(super) fn equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let left = super::super::plan_expr(left, context)?;
    let right = super::super::plan_expr(right, context)?;
    reject_function_equality(&left, &right, UnsupportedBinOpKind::EqFunction, context)?;

    Ok(Expr::bool(BoolExpr::equal(left, right)))
}

pub(super) fn not_equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let left = super::super::plan_expr(left, context)?;
    let right = super::super::plan_expr(right, context)?;
    reject_function_equality(&left, &right, UnsupportedBinOpKind::NotEqFunction, context)?;

    Ok(Expr::bool(BoolExpr::not_equal(left, right)))
}

fn reject_function_equality(
    left: &Expr,
    right: &Expr,
    operator: UnsupportedBinOpKind,
    context: &PlanContext<'_>,
) -> Result<(), PlanError> {
    if context.contains_function_value(&left.value_type())?
        || context.contains_function_value(&right.value_type())?
    {
        return Err(PlanError::UnsupportedBinOp { operator });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::plan::{CustomExpr, CustomLocalId, CustomType, CustomTypeName, Expr, IntExpr};
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::dsl::{bool_, equal, function, int, module, not_equal};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidCustomTypeReason, InvalidTypedAstReason, PlanError, UnsupportedBinOpKind,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use std::collections::HashMap;

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
    fn reject_profile_equality_operand_expression_errors_propagate() {
        for (name, src) in [
            (
                "equal left",
                r#"
pub fn main() {
  {
    echo 1
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
    echo 1
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
    echo 1
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
    echo 1
    1
  }
}
"#,
            ),
        ] {
            assert_echo_reject(name, src);
        }
    }

    #[test]
    fn reject_profile_function_equality_operators() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  add_one == add_one
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::EqFunction,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  add_one != add_one
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::NotEqFunction,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
pub type Boxed(value) {
  Boxed(value)
}

pub type Wrapper(value) {
  Wrapper(Boxed(value))
}

fn value() {
  1
}

pub fn main() {
  Wrapper(Boxed(value)) == Wrapper(Boxed(value))
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::EqFunction,
            },
        );
    }

    #[test]
    fn equality_preserves_custom_type_definition_errors_from_either_operand() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let missing = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
            Vec::new(),
        );
        let custom = Expr::custom(CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), missing),
            "missing".into(),
        ));
        let int = Expr::int(IntExpr::value(1.into()));
        let expected = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Missing".into(),
                reason: InvalidCustomTypeReason::UnknownDefinition,
            },
        });

        assert_eq!(
            super::reject_function_equality(
                &custom,
                &int,
                UnsupportedBinOpKind::EqFunction,
                &context,
            ),
            expected.clone(),
        );
        assert_eq!(
            super::reject_function_equality(
                &int,
                &custom,
                UnsupportedBinOpKind::EqFunction,
                &context,
            ),
            expected,
        );
    }

    #[test]
    fn reject_profile_tuple_equality_containing_function_value() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  #(1, add_one) == #(1, add_one)
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::EqFunction,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  #(#(add_one)) != #(#(add_one))
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::NotEqFunction,
            },
        );
    }

    #[test]
    fn reject_profile_list_equality_containing_function_value() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  [add_one] == [add_one]
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::EqFunction,
            },
        );
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  [[add_one]] != [[add_one]]
}
"#,
            ),
            PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::NotEqFunction,
            },
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
