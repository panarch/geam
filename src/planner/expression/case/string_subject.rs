use super::{
    case_return_type, invalid_case_shape, single_case_pattern, unsupported_case,
    validate_case_branch_type, validate_clause_shape,
};
use crate::plan::{Expr, ExprKind, StringCaseBranches, StringExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = super::super::plan_string_expr(subject, context)?;
    let return_type = case_return_type(type_.as_ref())?;
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        validate_clause_shape(&clause)?;
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_string_case_pattern(pattern)?;
        let branch = super::super::plan_expr_with_expected_source_stop_type(
            clause.then,
            return_type.clone(),
            context,
        )?;
        validate_case_branch_type(type_.as_ref(), &branch)?;

        match pattern {
            StringCasePattern::Literal(value) => {
                if fallback.is_none()
                    && literal_clauses
                        .iter()
                        .all(|(existing, _)| existing != &value)
                {
                    literal_clauses.push((value, branch));
                }
            }
            StringCasePattern::Any => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = fallback.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFallbackPattern,
    ))?;

    string_case_expr(subject, literal_clauses, fallback)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringCasePattern {
    Literal(EcoString),
    Any,
}

fn plan_string_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<StringCasePattern, PlanError> {
    match pattern {
        Pattern::String { value, .. } => Ok(StringCasePattern::Literal(value)),
        Pattern::Variable { type_, .. } if type_.is_string() => {
            Err(unsupported_case(UnsupportedCaseReason::VariablePattern))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_string() => Ok(StringCasePattern::Any),
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { pattern, .. } => match validate_string_case_assign_pattern(&pattern) {
            Ok(()) => Err(unsupported_case(UnsupportedCaseReason::AssignPattern)),
            Err(reason) => Err(invalid_case_shape(reason)),
        },
        Pattern::StringPrefix { .. } => {
            Err(unsupported_case(UnsupportedCaseReason::StringPrefixPattern))
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn validate_string_case_assign_pattern(
    pattern: &Pattern<Arc<Type>>,
) -> Result<(), InvalidCaseShapeReason> {
    match pattern {
        Pattern::String { .. } => Ok(()),
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } if type_.is_string() => {
            Ok(())
        }
        Pattern::Invalid { .. } => Err(InvalidCaseShapeReason::InvalidPattern),
        _ => Err(InvalidCaseShapeReason::PatternTypeMismatch),
    }
}

fn string_case_expr(
    subject: StringExpr,
    clauses: Vec<(EcoString, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    let branches = match fallback.into_kind() {
        ExprKind::Int(fallback) => StringCaseBranches::Int {
            clauses: int_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::String(fallback) => StringCaseBranches::String {
            clauses: string_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Float(fallback) => StringCaseBranches::Float {
            clauses: float_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Bool(fallback) => StringCaseBranches::Bool {
            clauses: bool_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Nil(fallback) => StringCaseBranches::Nil {
            clauses: nil_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Tuple(fallback) => StringCaseBranches::Tuple {
            clauses: tuple_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::List(fallback) => StringCaseBranches::List {
            clauses: list_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::Function(fallback) => function_case_branches(clauses, fallback)?,
    };

    Ok(Expr::string_case(subject, branches))
}

fn int_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::IntExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::StringExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::FloatExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::BoolExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::NilExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Nil(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn tuple_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::TupleExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Tuple(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn list_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::ListExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::List(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn function_case_branches(
    clauses: Vec<(EcoString, Expr)>,
    fallback: crate::plan::FunctionExpr,
) -> Result<StringCaseBranches, PlanError> {
    match fallback.into_kind() {
        crate::plan::FunctionExprKind::Int(fallback) => Ok(StringCaseBranches::IntFunction {
            clauses: int_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::String(fallback) => Ok(StringCaseBranches::StringFunction {
            clauses: string_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Float(fallback) => Ok(StringCaseBranches::FloatFunction {
            clauses: float_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Bool(fallback) => Ok(StringCaseBranches::BoolFunction {
            clauses: bool_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Nil(fallback) => Ok(StringCaseBranches::NilFunction {
            clauses: nil_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Tuple(fallback) => Ok(StringCaseBranches::TupleFunction {
            clauses: tuple_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::List(fallback) => Ok(StringCaseBranches::ListFunction {
            clauses: list_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::Function(fallback) => {
            Ok(StringCaseBranches::FunctionFunction {
                clauses: function_function_case_clauses(clauses)?,
                fallback,
            })
        }
    }
}

fn int_function_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::IntFunctionExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::StringFunctionExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::FloatFunctionExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::BoolFunctionExpr)>, PlanError> {
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
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::NilFunctionExpr)>, PlanError> {
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

fn tuple_function_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::TupleFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_tuple() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn list_function_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::ListFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(branch_return_type_mismatch());
        };
        let Some(clause) = clause.into_list() else {
            return Err(branch_return_type_mismatch());
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn function_function_case_clauses(
    clauses: Vec<(EcoString, Expr)>,
) -> Result<Vec<(EcoString, crate::plan::FunctionFunctionExpr)>, PlanError> {
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
        FunctionType, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId,
        ListFunctionId, LocalId, NilFunctionId, RuntimeFunctionId, StringCaseBranches,
        StringFunctionId, StringLocalId, TupleFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_string_case, float, function, function_ref, int,
        int_return_expr, int_return_string_case, list, list_return_expr, list_return_string_case,
        local_string, module, nil, nil_return_expr, nil_return_string_case, return_list, string,
        string_return_expr, string_return_string_case, tuple,
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
    fn plan_string_case_expressions() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "one" {
    "one" -> 10
    _ -> 0
  }
}

pub fn string_case(value: String) {
  case value {
    "a" -> "alpha"
    "b" -> "beta"
    _ -> "many"
  }
}

pub fn bool_case(value: String) {
  case value {
    "yes" -> True
    _ -> False
  }
}

pub fn nil_case(value: String) {
  case value {
    "nil" -> Nil
    _ -> Nil
  }
}

pub fn list_case(value: String) {
  case value {
    "one" -> [1]
    _ -> [0]
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_string_case(
                    string("one"),
                    [("one", int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "string_case",
                    string_return_string_case(
                        local_string(0, "value"),
                        [
                            ("a", string_return_expr(string("alpha"))),
                            ("b", string_return_expr(string("beta"))),
                        ],
                        string_return_expr(string("many")),
                    ),
                )
                .param_string(0, "value"),
                function(
                    "bool_case",
                    bool_return_string_case(
                        local_string(0, "value"),
                        [("yes", bool_return_expr(bool_(true)))],
                        bool_return_expr(bool_(false)),
                    ),
                )
                .param_string(0, "value"),
                function(
                    "nil_case",
                    nil_return_string_case(
                        local_string(0, "value"),
                        [("nil", nil_return_expr(nil()))],
                        nil_return_expr(nil()),
                    ),
                )
                .param_string(0, "value"),
                function(
                    "list_case",
                    return_list(
                        ValueType::Int,
                        list_return_string_case(
                            local_string(0, "value"),
                            [("one", list_return_expr(list([int(1)], ValueType::Int)))],
                            list_return_expr(list([int(0)], ValueType::Int)),
                        ),
                    ),
                )
                .param_string(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_wildcard_fallbacks() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "one" {
    "one" -> 10
    _ -> 0
  }
}

fn fallback_first(value: String) {
  case value {
    _ -> 0
    "one" -> 1
  }
}

fn fallback_then_fallback(value: String) {
  case value {
    _ -> 0
    _ -> 1
  }
}

fn duplicate_literal(value: String) {
  case value {
    "one" -> 1
    "one" -> 2
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
                int_return_string_case(
                    string("one"),
                    [("one", int_return_expr(int(10)))],
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "fallback_first",
                    int_return_string_case(local_string(0, "value"), [], int_return_expr(int(0))),
                )
                .param_string(0, "value"),
                function(
                    "fallback_then_fallback",
                    int_return_string_case(local_string(0, "value"), [], int_return_expr(int(0))),
                )
                .param_string(0, "value"),
                function(
                    "duplicate_literal",
                    int_return_string_case(
                        local_string(0, "value"),
                        [("one", int_return_expr(int(1)))],
                        int_return_expr(int(0)),
                    ),
                )
                .param_string(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_function_expr_shape() {
        let actual = super::string_case_expr(
            string("one").into(),
            vec![("one".into(), int_function_ref_expr(0))],
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
            IntFunctionExpr::string_case(
                string("one").into(),
                vec![("one".into(), branch)],
                fallback,
            ),
        )));

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_string_case_patterns() {
        let cases = [
            (
                r#"pub fn main() { case "one" { value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::VariablePattern,
            ),
            (
                r#"pub fn main() { case "one" { "one" as value -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case "one" { value as alias -> 1 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case "one" { _ as alias -> 1 } }"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"pub fn main() { case "prefix one" { "prefix " <> rest -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::StringPrefixPattern,
            ),
            (
                r#"pub fn main() { case "one" { "one" if True -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::Guard,
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
    fn reject_profile_string_case_unreachable_duplicate_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case "one" {
    "one" -> 1
    "one" -> echo 2
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_string_case_pattern_shapes() {
        let mut variable_type_mismatch = compile_string_case_module();
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

        let mut discard_type_mismatch = compile_string_case_module();
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

        let mut invalid_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::string(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_string_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut assign_invalid_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Invalid {
                location: dummy_span(),
                type_: type_::string(),
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

        let mut assign_type_mismatch = compile_string_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
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

        let mut empty_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut case_type_mismatch = compile_string_case_module();
        let (case_type, _, _) = super::super::expect_case_statement_mut(
            &mut case_type_mismatch.definitions.functions[0].body[0],
        );
        *case_type = type_::bool();
        assert_eq!(
            plan_module(case_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut missing_fallback_pattern = compile_string_case_module();
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

        let mut missing_function_fallback_pattern = crate::planner::support::compile(
            r#"
pub fn main() {
  let function = case "one" {
    "one" -> return_value
    _ -> return_value
  }
  function("value")
}

fn return_value(value: String) {
  value
}
"#,
        );
        let body = missing_function_fallback_pattern
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
        let (_, _, clauses) = super::super::expect_assignment_case_statement_mut(&mut body[0]);
        clauses.pop();
        assert_eq!(
            plan_module(missing_function_fallback_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_string_case_subject_type_mismatch() {
        let mut module = compile_string_case_module();
        let (_, subjects, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: type_::string(),
            value: "1".into(),
            int_value: BigInt::from(1),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_string_case_expr_type_mismatch() {
        let mut module = compile_string_case_module();
        let (type_, _, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        *type_ = type_::bit_array();
        assert_eq!(plan_module(module), Err(case_branch_return_type_mismatch()));

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), bool_(true).into())],
                int(0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                string("other").into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                float(1.0).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                bool_(false).into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                nil().into(),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                Expr::from(tuple([Expr::from(int(0))])),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                Expr::from(list([int(0)], ValueType::Int)),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), int(10).into())],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        let string_function: crate::plan::Expr = function_ref(
            RuntimeFunctionId::String(StringFunctionId(0)),
            [LocalId::String(StringLocalId(0))],
        )
        .into();

        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), string_function)],
                int_function_ref_expr(0),
            ),
            Err(case_branch_return_type_mismatch()),
        );
    }

    #[test]
    fn plan_string_case_function_branch_return_families_direct() {
        assert_eq!(
            super::string_case_expr(
                string("one").into(),
                vec![("one".into(), float(1.0).into())],
                float(0.0).into(),
            ),
            Ok(Expr::string_case(
                string("one").into(),
                StringCaseBranches::Float {
                    clauses: vec![("one".into(), FloatExpr::value(1.0))],
                    fallback: FloatExpr::value(0.0),
                },
            )),
        );

        assert_eq!(
            super::function_case_branches(
                vec![("one".into(), string_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(1)),
                    [LocalId::String(StringLocalId(0))],
                )),
            ),
            Ok(StringCaseBranches::StringFunction {
                clauses: vec![(
                    "one".into(),
                    string_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_string()
                        .expect("string function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(1)),
                    [LocalId::String(StringLocalId(0))],
                ))
                .into_string()
                .expect("string function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![("one".into(), float_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Float(FloatFunctionId(1)),
                    [LocalId::Float(crate::plan::FloatLocalId(0))],
                )),
            ),
            Ok(StringCaseBranches::FloatFunction {
                clauses: vec![(
                    "one".into(),
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
                vec![("one".into(), bool_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(1)),
                    [LocalId::Bool(crate::plan::BoolLocalId(0))],
                )),
            ),
            Ok(StringCaseBranches::BoolFunction {
                clauses: vec![(
                    "one".into(),
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
                vec![("one".into(), nil_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(1)),
                    [LocalId::Nil(crate::plan::NilLocalId(0))],
                )),
            ),
            Ok(StringCaseBranches::NilFunction {
                clauses: vec![(
                    "one".into(),
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
                vec![("one".into(), list_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List {
                        id: ListFunctionId(1),
                        return_type: Box::new(ValueType::Int),
                    },
                    [LocalId::Int(IntLocalId(0))],
                )),
            ),
            Ok(StringCaseBranches::ListFunction {
                clauses: vec![(
                    "one".into(),
                    list_function_ref_expr(0)
                        .into_function()
                        .expect("function expression")
                        .into_list()
                        .expect("list function expression"),
                )],
                fallback: FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List {
                        id: ListFunctionId(1),
                        return_type: Box::new(ValueType::Int),
                    },
                    [LocalId::Int(IntLocalId(0))],
                ))
                .into_list()
                .expect("list function expression"),
            }),
        );
        assert_eq!(
            super::function_case_branches(
                vec![("one".into(), function_function_ref_expr(0))],
                function_function_ref_expr(1)
                    .into_function()
                    .expect("function expression"),
            ),
            Ok(StringCaseBranches::FunctionFunction {
                clauses: vec![(
                    "one".into(),
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
    fn reject_margin_string_case_function_clause_family_mismatch_direct() {
        assert_eq!(
            super::string_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::string_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::bool_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::nil_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![("one".into(), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::function_function_case_clauses(vec![("one".into(), int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );

        assert_string_function_case_branch_mismatch(int_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(string_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(float_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(bool_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(nil_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(tuple_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(list_function_ref_expr(1));
        assert_string_function_case_branch_mismatch(function_function_ref_expr(1));
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
            [LocalId::String(StringLocalId(0))],
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

    fn list_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::List {
                id: ListFunctionId(id),
                return_type: Box::new(ValueType::Int),
            },
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn tuple_function_ref_expr(id: usize) -> crate::plan::Expr {
        function_ref(
            RuntimeFunctionId::Tuple {
                id: TupleFunctionId(id),
                return_type: vec![ValueType::Int],
            },
            [LocalId::Int(IntLocalId(0))],
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

    fn assert_string_function_case_branch_mismatch(fallback: crate::plan::Expr) {
        assert_eq!(
            super::function_case_branches(
                vec![("one".into(), Expr::from(int(1)))],
                fallback.into_function().expect("function expression"),
            ),
            Err(case_branch_return_type_mismatch()),
        );
    }

    fn case_branch_return_type_mismatch() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
            },
        }
    }

    fn compile_string_case_module() -> TypedModule {
        crate::planner::support::compile(
            r#"
pub fn main() {
  case "one" {
    "one" -> 10
    _ -> 0
  }
}
"#,
        )
    }
}
