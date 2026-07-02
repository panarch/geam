use super::{
    invalid_case_shape, single_case_pattern, unsupported_case, validate_case_branch_type,
    validate_clause_shape,
};
use crate::plan::{Expr, ExprKind, FloatCaseBranches, FloatExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = super::super::plan_float_expr(subject, context)?;
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        validate_clause_shape(&clause)?;
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_float_case_pattern(pattern)?;
        let branch = super::super::plan_expr(clause.then, context)?;
        validate_case_branch_type(type_.as_ref(), &branch)?;

        match pattern {
            FloatCasePattern::Literal(value) => {
                if fallback.is_none()
                    && literal_clauses
                        .iter()
                        .all(|(existing, _)| existing != &value)
                {
                    literal_clauses.push((value, branch));
                }
            }
            FloatCasePattern::Any => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = fallback.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFallbackPattern,
    ))?;

    float_case_expr(subject, literal_clauses, fallback)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FloatCasePattern {
    Literal(f64),
    Any,
}

fn plan_float_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<FloatCasePattern, PlanError> {
    match pattern {
        Pattern::Float { float_value, .. } => Ok(FloatCasePattern::Literal(float_value.value())),
        Pattern::Variable { type_, .. } if type_.is_float() => {
            Err(unsupported_case(UnsupportedCaseReason::VariablePattern))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_float() => Ok(FloatCasePattern::Any),
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { pattern, .. } => match validate_float_case_assign_pattern(&pattern) {
            Ok(()) => Err(unsupported_case(UnsupportedCaseReason::AssignPattern)),
            Err(reason) => Err(invalid_case_shape(reason)),
        },
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Int { .. }
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

fn validate_float_case_assign_pattern(
    pattern: &Pattern<Arc<Type>>,
) -> Result<(), InvalidCaseShapeReason> {
    match pattern {
        Pattern::Float { .. } => Ok(()),
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } if type_.is_float() => {
            Ok(())
        }
        Pattern::Invalid { .. } => Err(InvalidCaseShapeReason::InvalidPattern),
        _ => Err(InvalidCaseShapeReason::PatternTypeMismatch),
    }
}

fn float_case_expr(
    subject: FloatExpr,
    clauses: Vec<(f64, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    let branches = match fallback.into_kind() {
        ExprKind::Int(fallback) => FloatCaseBranches::Int {
            clauses: int_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::String(fallback) => FloatCaseBranches::String {
            clauses: string_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Float(fallback) => FloatCaseBranches::Float {
            clauses: float_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Bool(fallback) => FloatCaseBranches::Bool {
            clauses: bool_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Nil(fallback) => FloatCaseBranches::Nil {
            clauses: nil_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Function(fallback) => function_case_branches(clauses, fallback)?,
    };

    Ok(Expr::float_case(subject, branches))
}

fn int_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::IntExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::StringExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::FloatExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::BoolExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::NilExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
    fallback: crate::plan::FunctionExpr,
) -> Result<FloatCaseBranches, PlanError> {
    match fallback.into_kind() {
        crate::plan::FunctionExprKind::Int(fallback) => Ok(FloatCaseBranches::IntFunction {
            clauses: int_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::String(fallback) => Ok(FloatCaseBranches::StringFunction {
            clauses: string_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Float(fallback) => Ok(FloatCaseBranches::FloatFunction {
            clauses: float_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Bool(fallback) => Ok(FloatCaseBranches::BoolFunction {
            clauses: bool_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Nil(fallback) => Ok(FloatCaseBranches::NilFunction {
            clauses: nil_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Function(fallback) => {
            Ok(FloatCaseBranches::FunctionFunction {
                clauses: function_function_case_clauses(clauses)?,
                fallback,
            })
        }
    }
}

fn int_function_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::IntFunctionExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::StringFunctionExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::FloatFunctionExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::BoolFunctionExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::NilFunctionExpr)>, PlanError> {
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
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::FunctionFunctionExpr)>, PlanError> {
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
        BoolFunctionId, Expr, FloatCaseBranches, FloatFunctionFunctionId, FloatFunctionId,
        FunctionExpr, FunctionFunctionId, FunctionType, IntFunctionExpr, IntFunctionId, IntLocalId,
        LocalId, NilFunctionId, RuntimeFunctionId, StringFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_float_case, float, float_return_expr,
        float_return_float_case, function, function_ref, int, int_return_expr,
        int_return_float_case, local_float, module, nil, nil_return_expr, nil_return_float_case,
        string, string_return_expr, string_return_float_case,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCaseReason, UnsupportedExpressionKind,
    };
    use gleam_core::ast::{Pattern, TypedModule};
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, error::VariableOrigin};

    #[test]
    fn plan_float_case_expressions() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1.0 {
    1.0 -> 10
    _ -> 0
  }
}

pub fn string_case(value: Float) {
  case value {
    1.0 -> "one"
    _ -> "many"
  }
}

pub fn bool_case(value: Float) {
  case value {
    1.0 -> True
    _ -> False
  }
}

pub fn nil_case(value: Float) {
  case value {
    1.0 -> Nil
    _ -> Nil
  }
}

pub fn float_case(value: Float) {
  case value {
    1.0 -> 1.5
    _ -> 0.5
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_float_case(
                    float(1.0),
                    [(1.0, int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "string_case",
                    string_return_float_case(
                        local_float(0, "value"),
                        [(1.0, string_return_expr(string("one")))],
                        string_return_expr(string("many")),
                    ),
                )
                .param_float(0, "value"),
                function(
                    "bool_case",
                    bool_return_float_case(
                        local_float(0, "value"),
                        [(1.0, bool_return_expr(bool_(true)))],
                        bool_return_expr(bool_(false)),
                    ),
                )
                .param_float(0, "value"),
                function(
                    "nil_case",
                    nil_return_float_case(
                        local_float(0, "value"),
                        [(1.0, nil_return_expr(nil()))],
                        nil_return_expr(nil()),
                    ),
                )
                .param_float(0, "value"),
                function(
                    "float_case",
                    float_return_float_case(
                        local_float(0, "value"),
                        [(1.0, float_return_expr(float(1.5)))],
                        float_return_expr(float(0.5)),
                    ),
                )
                .param_float(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_case_wildcard_fallbacks() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1.0 {
    1.0 -> 10
    _ -> 0
  }
}

fn fallback_first(value: Float) {
  case value {
    _ -> 0
    1.0 -> 1
  }
}

fn fallback_then_fallback(value: Float) {
  case value {
    _ -> 0
    _ -> 1
  }
}

fn duplicate_literal(value: Float) {
  case value {
    1.0 -> 1
    1.0 -> 2
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
                int_return_float_case(
                    float(1.0),
                    [(1.0, int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "fallback_first",
                    int_return_float_case(local_float(0, "value"), [], int_return_expr(int(0))),
                )
                .param_float(0, "value"),
                function(
                    "fallback_then_fallback",
                    int_return_float_case(local_float(0, "value"), [], int_return_expr(int(0))),
                )
                .param_float(0, "value"),
                function(
                    "duplicate_literal",
                    int_return_float_case(
                        local_float(0, "value"),
                        [(1.0, int_return_expr(int(1)))],
                        int_return_expr(int(0)),
                    ),
                )
                .param_float(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_case_function_expr_shape() {
        let actual = super::float_case_expr(
            float(1.0).into(),
            vec![(1.0, int_function_ref_expr(0))],
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
        let expected = Ok(Expr::function(FunctionExpr::int(
            IntFunctionExpr::float_case(float(1.0).into(), vec![(1.0, branch)], fallback),
        )));

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_case_function_branch_return_families_direct() {
        assert_eq!(
            super::function_case_branches(
                vec![(1.0, string_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(1)),
                    [LocalId::String(crate::plan::StringLocalId(0))],
                )),
            ),
            Ok(FloatCaseBranches::StringFunction {
                clauses: vec![(
                    1.0,
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
                vec![(1.0, float_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Float(FloatFunctionId(1)),
                    [LocalId::Float(crate::plan::FloatLocalId(0))],
                )),
            ),
            Ok(FloatCaseBranches::FloatFunction {
                clauses: vec![(
                    1.0,
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
                vec![(1.0, bool_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(1)),
                    [LocalId::Bool(crate::plan::BoolLocalId(0))],
                )),
            ),
            Ok(FloatCaseBranches::BoolFunction {
                clauses: vec![(
                    1.0,
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
                vec![(1.0, nil_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(1)),
                    [LocalId::Nil(crate::plan::NilLocalId(0))],
                )),
            ),
            Ok(FloatCaseBranches::NilFunction {
                clauses: vec![(
                    1.0,
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
                vec![(1.0, function_function_ref_expr(0))],
                function_function_ref_expr(1)
                    .into_function()
                    .expect("function expression"),
            ),
            Ok(FloatCaseBranches::FunctionFunction {
                clauses: vec![(
                    1.0,
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
    fn reject_profile_float_case_patterns() {
        let cases = [
            (
                r#"pub fn main() { case 1.0 { value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::VariablePattern,
            ),
            (
                r#"pub fn main() { case 1.0 { 1.0 as value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case 1.0 { value as alias -> 1 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case 1.0 { _ as alias -> 1 } }"#,
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
    fn reject_profile_float_case_unreachable_duplicate_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1.0 {
    1.0 -> 1
    1.0 -> {
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
    fn reject_margin_float_case_pattern_shapes() {
        let mut variable_type_mismatch = compile_float_case_module();
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

        let mut discard_type_mismatch = compile_float_case_module();
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

        let mut invalid_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::float(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_float_case_module();
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

        let mut assign_invalid_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Invalid {
                location: dummy_span(),
                type_: type_::float(),
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

        let mut assign_type_mismatch = compile_float_case_module();
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

        let mut missing_fallback_pattern = compile_float_case_module();
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
    fn reject_margin_float_case_subject_type_mismatch() {
        let mut module = compile_float_case_module();
        let (_, subjects, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        subjects[0] = gleam_core::ast::TypedExpr::String {
            location: dummy_span(),
            type_: type_::float(),
            value: "not float".into(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_float_case_expr_type_mismatch() {
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, bool_(true).into())],
                int(0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, int(10).into())],
                string("other").into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, int(10).into())],
                float(0.0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, int(10).into())],
                bool_(false).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(float(1.0).into(), vec![(1.0, int(10).into())], nil().into(),),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, int(10).into())],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        let string_function: Expr = function_ref(
            RuntimeFunctionId::String(StringFunctionId(0)),
            [LocalId::String(crate::plan::StringLocalId(0))],
        )
        .into();
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, string_function)],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );
    }

    #[test]
    fn reject_margin_float_case_function_clause_family_mismatch_direct() {
        assert_eq!(
            super::string_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::string_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
    }

    fn int_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Int(IntFunctionId(id)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn string_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::String(StringFunctionId(id)),
            [LocalId::String(crate::plan::StringLocalId(0))],
        )
        .into()
    }

    fn float_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Float(FloatFunctionId(id)),
            [LocalId::Float(crate::plan::FloatLocalId(0))],
        )
        .into()
    }

    fn bool_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Bool(BoolFunctionId(id)),
            [LocalId::Bool(crate::plan::BoolLocalId(0))],
        )
        .into()
    }

    fn nil_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Nil(NilFunctionId(id)),
            [LocalId::Nil(crate::plan::NilLocalId(0))],
        )
        .into()
    }

    fn function_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Float(FloatFunctionFunctionId(id)),
                return_type: FunctionType::new(vec![ValueType::Float], ValueType::Float),
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

    fn compile_float_case_module() -> TypedModule {
        crate::planner::support::compile(
            r#"
pub fn main() {
  case 1.0 {
    1.0 -> 10
    _ -> 0
  }
}
"#,
        )
    }

    #[test]
    fn reject_margin_float_case_assign_literal_pattern_still_profile_boundary() {
        let mut module = compile_float_case_module();
        let (_, _, clauses) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Float {
                location: dummy_span(),
                value: "1.0".into(),
                float_value: LiteralFloatValue::ONE,
            }),
        };
        assert_eq!(
            plan_module(module),
            Err(PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::AssignPattern,
            }),
        );
    }
}
