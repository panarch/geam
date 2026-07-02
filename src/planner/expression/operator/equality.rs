use crate::plan::{BoolExpr, Expr, ValueType};
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
    reject_function_equality(&left, &right, UnsupportedBinOpKind::EqFunction)?;

    Ok(Expr::bool(BoolExpr::equal(left, right)))
}

pub(super) fn not_equal(
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let left = super::super::plan_expr(left, context)?;
    let right = super::super::plan_expr(right, context)?;
    reject_function_equality(&left, &right, UnsupportedBinOpKind::NotEqFunction)?;

    Ok(Expr::bool(BoolExpr::not_equal(left, right)))
}

fn reject_function_equality(
    left: &Expr,
    right: &Expr,
    operator: UnsupportedBinOpKind,
) -> Result<(), PlanError> {
    if contains_function_value(&left.value_type()) || contains_function_value(&right.value_type()) {
        return Err(PlanError::UnsupportedBinOp { operator });
    }

    Ok(())
}

fn contains_function_value(type_: &ValueType) -> bool {
    match type_ {
        ValueType::Function(_) => true,
        ValueType::Tuple(elements) => elements.iter().any(contains_function_value),
        ValueType::Int
        | ValueType::Float
        | ValueType::String
        | ValueType::Bool
        | ValueType::Nil => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{bool_, equal, function, int, module, not_equal};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{PlanError, UnsupportedBinOpKind};

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
}
