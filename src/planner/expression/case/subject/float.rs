use super::super::super::plan_float_expr;
use super::super::{invalid_case_shape, unsupported_case};
use super::{case_return_type, single_case_pattern, validate_clause_shape};
use crate::plan::{BoolExpr, Expr, ExprKind, FloatCaseBranches, FloatExpr, ValueType};
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
    let subject = plan_float_expr(subject, context)?;
    let return_type = case_return_type(type_.as_ref())?;
    for clause in &clauses {
        validate_clause_shape(clause)?;
    }
    if clauses.iter().any(|clause| clause.guard.is_some()) {
        let (subject_step, subject) = super::bind_float_case_subject(subject, context);
        let case = plan_guarded_float_case(type_.as_ref(), return_type, subject, clauses, context)?;
        return Ok(super::case_subject_block(subject_step, case));
    }
    let needs_subject_binding = clauses.iter().any(clause_has_float_variable_pattern);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_float_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_float_case_pattern(pattern)?;
        let binding = pattern
            .bound_name()
            .cloned()
            .map(|name| (name, Expr::float(subject.clone())));
        let branch =
            super::plan_case_branch(type_.as_ref(), &return_type, clause.then, binding, context)?;

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
            FloatCasePattern::Any { .. } => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = fallback.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFallbackPattern,
    ))?;

    float_case_expr(subject, literal_clauses, fallback).map(|case| match subject_step {
        Some(step) => super::case_subject_block(step, case),
        None => case,
    })
}

fn plan_guarded_float_case(
    case_type: &Type,
    return_type: ValueType,
    subject: FloatExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut ordered_clauses = Vec::with_capacity(clauses.len());
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_float_case_pattern(pattern)?;
        let binding = pattern
            .bound_name()
            .cloned()
            .map(|name| (name, Expr::float(subject.clone())));
        let is_total = matches!(pattern, FloatCasePattern::Any { .. }) && clause.guard.is_none();
        let match_condition = match pattern {
            FloatCasePattern::Literal(value) => BoolExpr::equal(
                Expr::float(subject.clone()),
                Expr::float(FloatExpr::value(value)),
            ),
            FloatCasePattern::Any { .. } => BoolExpr::value(true),
        };
        ordered_clauses.push(super::plan_ordered_case_clause(
            super::OrderedCaseClauseInput {
                case_type,
                return_type: &return_type,
                then: clause.then,
                variable_binding: binding,
                guard: clause.guard,
                match_condition,
                is_total,
            },
            context,
        )?);
    }

    super::ordered_case_expr(ordered_clauses)
}

#[derive(Debug, Clone, PartialEq)]
enum FloatCasePattern {
    Literal(f64),
    Any { bound_name: Option<EcoString> },
}

impl FloatCasePattern {
    fn bound_name(&self) -> Option<&EcoString> {
        match self {
            FloatCasePattern::Any { bound_name } => bound_name.as_ref(),
            FloatCasePattern::Literal(_) => None,
        }
    }
}

fn plan_float_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<FloatCasePattern, PlanError> {
    match pattern {
        Pattern::Float { float_value, .. } => Ok(FloatCasePattern::Literal(float_value.value())),
        Pattern::Variable { name, type_, .. } if type_.is_float() => Ok(FloatCasePattern::Any {
            bound_name: Some(name),
        }),
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_float() => {
            Ok(FloatCasePattern::Any { bound_name: None })
        }
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

fn clause_has_float_variable_pattern(clause: &TypedClause) -> bool {
    clause.pattern.iter().any(|pattern| {
        matches!(
            pattern,
            Pattern::Variable { type_, .. } if type_.is_float()
        )
    })
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
        ExprKind::Tuple(fallback) => FloatCaseBranches::Tuple {
            clauses: tuple_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::List(fallback) => FloatCaseBranches::List {
            clauses: list_case_clauses(clauses)?,
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn tuple_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::TupleExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Tuple(clause) = clause.into_kind() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn list_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::ListExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::List(clause) = clause.into_kind() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
        crate::plan::FunctionExprKind::Tuple(fallback) => Ok(FloatCaseBranches::TupleFunction {
            clauses: tuple_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::List(fallback) => Ok(FloatCaseBranches::ListFunction {
            clauses: list_function_case_clauses(clauses)?,
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_int() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_string() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_float() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_bool() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_nil() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn tuple_function_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::TupleFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_tuple() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn list_function_case_clauses(
    clauses: Vec<(f64, Expr)>,
) -> Result<Vec<(f64, crate::plan::ListFunctionExpr)>, PlanError> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (value, clause) in clauses {
        let ExprKind::Function(clause) = clause.into_kind() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_list() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        let Some(clause) = clause.into_function() else {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolFunctionId, Expr, FloatCaseBranches, FloatFunctionFunctionId,
        FloatFunctionId, FloatReturn, FunctionExpr, FunctionFunctionId, FunctionType,
        IntFunctionExpr, IntFunctionId, IntLocalId, ListFunctionId, LocalId, NilFunctionId,
        RuntimeFunctionId, StringFunctionId, TupleFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_float_case, float, float_return_block,
        float_return_expr, float_return_float_case, function, function_ref, int, int_return_expr,
        int_return_float_case, let_float_step, list, list_return_expr, list_return_float_case,
        local_float, module, nil, nil_return_expr, nil_return_float_case, return_list, string,
        string_return_expr, string_return_float_case, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCaseReason, UnsupportedExpressionKind,
    };
    use gleam_core::ast::{ClauseGuard, Constant, Pattern, TypedModule};
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

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

pub fn list_case(value: Float) {
  case value {
    1.0 -> [1]
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
                function(
                    "list_case",
                    return_list(
                        ValueType::Int,
                        list_return_float_case(
                            local_float(0, "value"),
                            [(1.0, list_return_expr(list([int(1)], ValueType::Int)))],
                            list_return_expr(list([int(0)], ValueType::Int)),
                        ),
                    ),
                )
                .param_float(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_case_variable_pattern_binds_subject_once_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1.5 {
    other -> other
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                float_return_block(
                    [let_float_step(0, "<case:float:0>", float(1.5))],
                    float_return_float_case(
                        local_float(0, "<case:float:0>"),
                        [],
                        float_return_block(
                            [let_float_step(1, "other", local_float(0, "<case:float:0>"))],
                            float_return_expr(local_float(1, "other")),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_case_guard_binds_subject_once_and_falls_through() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1.5 {
    other if other >. 1.0 -> other
    _ -> 0.0
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_float_step(1, "other", local_float(0, "<case:float:0>"));
        let condition = BoolExpr::block(
            vec![bind_other.clone()],
            BoolExpr::and(
                BoolExpr::value(true),
                BoolExpr::gt_float(local_float(1, "other").into(), float(1.0).into()),
            ),
        );
        let guarded_branch =
            float_return_block([bind_other], float_return_expr(local_float(1, "other")));
        let expected = module(
            "main",
            function(
                "main",
                float_return_block(
                    [let_float_step(0, "<case:float:0>", float(1.5))],
                    FloatReturn::bool_case(
                        condition,
                        guarded_branch,
                        float_return_expr(float(0.0)),
                    ),
                ),
            ),
            [],
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
                vec![(1.0, list_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List {
                        id: ListFunctionId(1),
                        return_type: Box::new(ValueType::Int),
                    },
                    [LocalId::Int(IntLocalId(0))],
                )),
            ),
            Ok(FloatCaseBranches::ListFunction {
                clauses: vec![(
                    1.0,
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
    1.0 -> echo 2
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
    fn reject_margin_float_case_pattern_shapes() {
        let mut alternative_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut alternative_pattern.definitions.functions[0].body[0],
        );
        clauses[0].alternative_patterns.push(vec![Pattern::Float {
            location: dummy_span(),
            value: "2.0".into(),
            float_value: LiteralFloatValue::ONE,
        }]);
        assert_eq!(
            plan_module(alternative_pattern),
            Err(PlanError::UnsupportedCase {
                reason: UnsupportedCaseReason::AlternativePatterns,
            }),
        );

        let mut variable_type_mismatch = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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

        let mut empty_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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

        let mut case_type_mismatch = compile_float_case_module();
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
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

        let mut missing_fallback_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
  let function = case 1.0 {
    1.0 -> add_one
    _ -> add_one
  }
  function(1.0)
}

fn add_one(value: Float) {
  value +. 1.0
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
        let (_, _, clauses) =
            super::super::super::expect_assignment_case_statement_mut(&mut body[0]);
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
    fn reject_margin_guarded_float_case_pattern_shapes() {
        let mut empty_pattern = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern[0] = Pattern::String {
            location: dummy_span(),
            value: "one".into(),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_float_case_guard_must_be_bool() {
        let mut module = compile_float_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_float_case_subject_type_mismatch() {
        let mut module = compile_float_case_module();
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
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
        let mut module = compile_float_case_module();
        let (type_, _, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        *type_ = type_::bit_array();
        assert_eq!(plan_module(module), Err(case_branch_return_type_mismatch()));

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
                Expr::from(tuple([Expr::from(int(0))])),
            ),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::float_case_expr(
                float(1.0).into(),
                vec![(1.0, int(10).into())],
                Expr::from(list([int(0)], ValueType::Int)),
            ),
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
            super::tuple_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![(1.0, Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![(1.0, int_function_ref_expr(0))]),
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

        assert_float_function_case_branch_mismatch(int_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(string_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(float_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(bool_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(nil_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(tuple_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(list_function_ref_expr(1));
        assert_float_function_case_branch_mismatch(function_function_ref_expr(1));
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

    fn list_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::List {
                id: ListFunctionId(id),
                return_type: Box::new(ValueType::Int),
            },
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn tuple_function_ref_expr(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Tuple {
                id: TupleFunctionId(id),
                return_type: vec![ValueType::Int],
            },
            [LocalId::Int(IntLocalId(0))],
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

    fn assert_float_function_case_branch_mismatch(fallback: Expr) {
        assert_eq!(
            super::function_case_branches(
                vec![(1.0, Expr::from(int(1)))],
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
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
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
