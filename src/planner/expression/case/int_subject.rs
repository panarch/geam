use super::{
    invalid_case_shape, single_case_pattern, unsupported_case, validate_case_branch_type,
    validate_clause_shape,
};
use crate::plan::{Expr, ExprKind, IntCaseBranches, IntExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use num_bigint::BigInt;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = super::super::plan_int_expr(subject, context)?;
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        validate_clause_shape(&clause)?;
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_int_case_pattern(pattern)?;
        let branch = super::super::plan_expr(clause.then, context)?;
        validate_case_branch_type(type_.as_ref(), &branch)?;

        match pattern {
            IntCasePattern::Literal(value) => {
                if fallback.is_none()
                    && literal_clauses
                        .iter()
                        .all(|(existing, _)| existing != &value)
                {
                    literal_clauses.push((value, branch));
                }
            }
            IntCasePattern::Any => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = fallback.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFallbackPattern,
    ))?;

    int_case_expr(subject, literal_clauses, fallback)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntCasePattern {
    Literal(BigInt),
    Any,
}

fn plan_int_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<IntCasePattern, PlanError> {
    match pattern {
        Pattern::Int { int_value, .. } => Ok(IntCasePattern::Literal(int_value)),
        Pattern::Variable { type_, .. } if type_.is_int() => {
            Err(unsupported_case(UnsupportedCaseReason::VariablePattern))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_int() => Ok(IntCasePattern::Any),
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { pattern, .. } => match validate_int_case_assign_pattern(&pattern) {
            Ok(()) => Err(unsupported_case(UnsupportedCaseReason::AssignPattern)),
            Err(reason) => Err(invalid_case_shape(reason)),
        },
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn validate_int_case_assign_pattern(
    pattern: &Pattern<Arc<Type>>,
) -> Result<(), InvalidCaseShapeReason> {
    match pattern {
        Pattern::Int { .. } => Ok(()),
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } if type_.is_int() => {
            Ok(())
        }
        Pattern::Invalid { .. } => Err(InvalidCaseShapeReason::InvalidPattern),
        _ => Err(InvalidCaseShapeReason::PatternTypeMismatch),
    }
}

fn int_case_expr(
    subject: IntExpr,
    clauses: Vec<(BigInt, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    let branches = match fallback.into_kind() {
        ExprKind::Int(fallback) => IntCaseBranches::Int {
            clauses: int_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::String(fallback) => IntCaseBranches::String {
            clauses: string_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Float(fallback) => IntCaseBranches::Float {
            clauses: float_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Bool(fallback) => IntCaseBranches::Bool {
            clauses: bool_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Nil(fallback) => IntCaseBranches::Nil {
            clauses: nil_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Function(fallback) => function_case_branches(clauses, fallback)?,
    };

    Ok(Expr::int_case(subject, branches))
}

fn int_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::IntExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Int(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn string_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::StringExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::String(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn float_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::FloatExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Float(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn bool_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::BoolExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Bool(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn nil_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::NilExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Nil(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn function_case_branches(
    clauses: Vec<(BigInt, Expr)>,
    fallback: crate::plan::FunctionExpr,
) -> Result<IntCaseBranches, PlanError> {
    match fallback.into_kind() {
        crate::plan::FunctionExprKind::Int(fallback) => Ok(IntCaseBranches::IntFunction {
            clauses: int_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::String(fallback) => Ok(IntCaseBranches::StringFunction {
            clauses: string_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Float(fallback) => Ok(IntCaseBranches::FloatFunction {
            clauses: float_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Bool(fallback) => Ok(IntCaseBranches::BoolFunction {
            clauses: bool_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Nil(fallback) => Ok(IntCaseBranches::NilFunction {
            clauses: nil_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Function(fallback) => {
            Ok(IntCaseBranches::FunctionFunction {
                clauses: function_function_case_clauses(clauses)?,
                fallback,
            })
        }
    }
}

fn int_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::IntFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_int() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn string_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::StringFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_string() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn float_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::FloatFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_float() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn bool_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::BoolFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_bool() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn nil_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::NilFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_nil() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn function_function_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::FunctionFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_function() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn branch_return_type_mismatch() -> PlanError {
    invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch)
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolFunctionId, Expr, FloatExpr, FloatFunctionId, FunctionExpr, FunctionFunctionId,
        FunctionType, IntCaseBranches, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntLocalId, LocalId, NilFunctionId, RuntimeFunctionId, StringFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_int_case, float, function, function_ref, int,
        int_return_expr, int_return_int_case, local_int, module, nil, nil_return_expr,
        nil_return_int_case, string, string_return_expr, string_return_int_case,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCaseReason, UnsupportedExpressionKind,
    };
    use gleam_core::ast::{Pattern, TypedModule};
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

    #[test]
    fn plan_int_case_expressions() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1 {
    1 -> 10
    _ -> 0
  }
}

pub fn string_case(value: Int) {
  case value {
    0 -> "zero"
    1 -> "one"
    _ -> "many"
  }
}

pub fn bool_case(value: Int) {
  case value {
    1 -> True
    _ -> False
  }
}

pub fn nil_case(value: Int) {
  case value {
    1 -> Nil
    _ -> Nil
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_int_case(
                    int(1),
                    [(1, int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "string_case",
                    string_return_int_case(
                        local_int(0, "value"),
                        [
                            (0, string_return_expr(string("zero"))),
                            (1, string_return_expr(string("one"))),
                        ],
                        string_return_expr(string("many")),
                    ),
                )
                .param_int(0, "value"),
                function(
                    "bool_case",
                    bool_return_int_case(
                        local_int(0, "value"),
                        [(1, bool_return_expr(bool_(true)))],
                        bool_return_expr(bool_(false)),
                    ),
                )
                .param_int(0, "value"),
                function(
                    "nil_case",
                    nil_return_int_case(
                        local_int(0, "value"),
                        [(1, nil_return_expr(nil()))],
                        nil_return_expr(nil()),
                    ),
                )
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_int_case_wildcard_fallbacks() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1 {
    1 -> 10
    _ -> 0
  }
}

fn fallback_first(value: Int) {
  case value {
    _ -> 0
    1 -> 1
  }
}

fn fallback_then_fallback(value: Int) {
  case value {
    _ -> 0
    _ -> 1
  }
}

fn duplicate_literal(value: Int) {
  case value {
    1 -> 1
    1 -> 2
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_int_case(
                    int(1),
                    [(1, int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "fallback_first",
                    int_return_int_case(local_int(0, "value"), [], int_return_expr(int(0))),
                )
                .param_int(0, "value"),
                function(
                    "fallback_then_fallback",
                    int_return_int_case(local_int(0, "value"), [], int_return_expr(int(0))),
                )
                .param_int(0, "value"),
                function(
                    "duplicate_literal",
                    int_return_int_case(
                        local_int(0, "value"),
                        [(1, int_return_expr(int(1)))],
                        int_return_expr(int(0)),
                    ),
                )
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_int_case_function_expr_shape() {
        let actual = super::int_case_expr(
            int(1).into(),
            vec![(BigInt::from(1), int_function_ref_expr(0))],
            int_function_ref_expr(0),
        );
        let branch = FunctionExpr::from(function_ref(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            [LocalId::Int(IntLocalId(0))],
        ))
        .into_int()
        .expect("int function expression");
        let fallback = FunctionExpr::from(function_ref(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            [LocalId::Int(IntLocalId(0))],
        ))
        .into_int()
        .expect("int function expression");
        let expected = Ok(crate::plan::Expr::function(FunctionExpr::int(
            IntFunctionExpr::int_case(int(1).into(), vec![(BigInt::from(1), branch)], fallback),
        )));

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_int_case_function_clause_family_mismatch_direct() {
        assert_eq!(
            super::string_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::string_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![(
                BigInt::from(1),
                int_function_ref_expr(0),
            )]),
            Err(case_branch_return_type_mismatch()),
        );
    }

    #[test]
    fn plan_int_case_function_branch_return_families_direct() {
        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), float(1.0).into())],
                float(0.0).into(),
            ),
            Ok(Expr::int_case(
                int(1).into(),
                IntCaseBranches::Float {
                    clauses: vec![(BigInt::from(1), FloatExpr::value(1.0))],
                    fallback: FloatExpr::value(0.0),
                },
            )),
        );

        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), string_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(1)),
                    [LocalId::String(crate::plan::StringLocalId(0))],
                )),
            ),
            Ok(IntCaseBranches::StringFunction {
                clauses: vec![(
                    BigInt::from(1),
                    string_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_string()
                        .expect("string function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(1)),
                    [LocalId::String(crate::plan::StringLocalId(0))],
                ))
                .into_string()
                .expect("string function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), float_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Float(FloatFunctionId(1)),
                    [LocalId::Float(crate::plan::FloatLocalId(0))],
                )),
            ),
            Ok(IntCaseBranches::FloatFunction {
                clauses: vec![(
                    BigInt::from(1),
                    float_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_float()
                        .expect("float function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Float(FloatFunctionId(1)),
                    [LocalId::Float(crate::plan::FloatLocalId(0))],
                ))
                .into_float()
                .expect("float function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), bool_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(1)),
                    [LocalId::Bool(crate::plan::BoolLocalId(0))],
                )),
            ),
            Ok(IntCaseBranches::BoolFunction {
                clauses: vec![(
                    BigInt::from(1),
                    bool_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_bool()
                        .expect("bool function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(1)),
                    [LocalId::Bool(crate::plan::BoolLocalId(0))],
                ))
                .into_bool()
                .expect("bool function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), nil_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(1)),
                    [LocalId::Nil(crate::plan::NilLocalId(0))],
                )),
            ),
            Ok(IntCaseBranches::NilFunction {
                clauses: vec![(
                    BigInt::from(1),
                    nil_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_nil()
                        .expect("nil function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(1)),
                    [LocalId::Nil(crate::plan::NilLocalId(0))],
                ))
                .into_nil()
                .expect("nil function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), function_function_ref_expr(0))],
                function_function_ref_expr(1)
                    .into_function()
                    .expect("function expression"),
            ),
            Ok(IntCaseBranches::FunctionFunction {
                clauses: vec![(
                    BigInt::from(1),
                    function_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_function()
                        .expect("function-returning function expression"),
                )],
                fallback: function_function_ref_expr(1)
                    .into_function()
                    .expect("function expression")
                    .into_function()
                    .expect("function-returning function expression"),
            }),
        );
    }

    #[test]
    fn reject_profile_int_case_patterns() {
        let cases = [
            (
                r#"pub fn main() { case 1 { value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::VariablePattern,
            ),
            (
                r#"pub fn main() { case 1 { 1 as value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case 1 { value as alias -> 1 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case 1 { _ as alias -> 1 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
        ];

        for (src, reason) in cases {
            assert_eq!(
                expect_plan_error(src),
                PlanError::UnsupportedCase { reason },
            );
        }
    }

    #[test]
    fn reject_profile_int_case_unreachable_duplicate_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    1 -> 1
    1 -> {
      [1]
      2
    }
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::List,
            },
        );
    }

    #[test]
    fn reject_margin_int_case_pattern_shapes() {
        let mut variable_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut variable_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: type_::bool(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(variable_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut discard_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut discard_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[1].pattern[0] = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::bool(),
        };
        assert_eq!(
            plan_module(discard_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut invalid_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::String {
            location: dummy_span(),
            value: "bad".into(),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut assign_invalid_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            }),
        };
        assert_eq!(
            plan_module(assign_invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut assign_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::String {
                location: dummy_span(),
                value: "bad".into(),
            }),
        };
        assert_eq!(
            plan_module(assign_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut missing_fallback_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut missing_fallback_pattern.definitions.functions[0].body[0],
        );
        clauses.pop();
        assert_eq!(
            plan_module(missing_fallback_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_int_case_subject_type_mismatch() {
        let mut module = compile_int_case_module();
        let (_, subjects, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        subjects[0] = gleam_core::ast::TypedExpr::String {
            location: dummy_span(),
            type_: type_::int(),
            value: "not int".into(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_int_case_expr_type_mismatch() {
        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), bool_(true).into())],
                int(0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                string("other").into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                float(1.0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                bool_(false).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                nil().into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        let string_function: crate::plan::Expr = function_ref(
            RuntimeFunctionId::String(StringFunctionId(0)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into();

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), string_function)],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );
    }

    fn int_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Int(IntFunctionId(id)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn string_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::String(StringFunctionId(id)),
            [LocalId::String(crate::plan::StringLocalId(0))],
        )
        .into()
    }

    fn float_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Float(FloatFunctionId(id)),
            [LocalId::Float(crate::plan::FloatLocalId(0))],
        )
        .into()
    }

    fn bool_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Bool(BoolFunctionId(id)),
            [LocalId::Bool(crate::plan::BoolLocalId(0))],
        )
        .into()
    }

    fn nil_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Nil(NilFunctionId(id)),
            [LocalId::Nil(crate::plan::NilLocalId(0))],
        )
        .into()
    }

    fn function_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(id)),
                return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            },
            Vec::<LocalId>::new(),
        )
        .into()
    }

    fn case_branch_return_type_mismatch() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
            },
        }
    }

    #[test]
    fn reject_margin_int_case_function_branch_type_mismatch() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  let function = case 1 {
    1 -> add_one
    _ -> add_one
  }
  stringify
  1
}

fn add_one(value: Int) {
  value + 1
}

fn stringify(value: Int) {
  "value"
}
"#,
        );
        let body = module
            .definitions
            .functions
            .iter_mut()
            .find(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "main")
            })
            .map(|function| &mut function.body)
            .expect("expected main function");
        let replacement = super::super::expect_expression_statement(&body[1]).clone();
        let (_, _, clauses) = super::super::expect_assignment_case_statement_mut(&mut body[0]);
        clauses[1].then = replacement;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
    }

    fn compile_int_case_module() -> TypedModule {
        crate::planner::support::compile(
            r#"
pub fn main() {
  case 1 {
    1 -> 10
    _ -> 0
  }
}
"#,
        )
    }
}
