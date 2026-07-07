use super::{
    case_return_type, invalid_case_shape, single_case_pattern, unsupported_case,
    validate_clause_shape,
};
use crate::plan::{BoolExpr, Expr, ExprKind, IntCaseBranches, IntExpr, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError, UnsupportedCaseReason};
use ecow::EcoString;
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
    let return_type = case_return_type(type_.as_ref())?;
    for clause in &clauses {
        validate_clause_shape(clause)?;
    }
    if clauses.iter().any(|clause| clause.guard.is_some()) {
        let (subject_step, subject) = super::bind_int_case_subject(subject, context);
        let case = plan_guarded_int_case(type_.as_ref(), return_type, subject, clauses, context)?;
        return Ok(super::case_subject_block(subject_step, case));
    }
    let needs_subject_binding = clauses.iter().any(clause_has_int_variable_pattern);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_int_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_int_case_pattern(pattern)?;
        let binding = pattern
            .bound_name()
            .cloned()
            .map(|name| (name, Expr::int(subject.clone())));
        let branch =
            super::plan_case_branch(type_.as_ref(), &return_type, clause.then, binding, context)?;

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
            IntCasePattern::Any { .. } => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = fallback.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFallbackPattern,
    ))?;

    int_case_expr(subject, literal_clauses, fallback).map(|case| match subject_step {
        Some(step) => super::case_subject_block(step, case),
        None => case,
    })
}

fn plan_guarded_int_case(
    case_type: &Type,
    return_type: ValueType,
    subject: IntExpr,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut ordered_clauses = Vec::with_capacity(clauses.len());
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_int_case_pattern(pattern)?;
        let binding = pattern
            .bound_name()
            .cloned()
            .map(|name| (name, Expr::int(subject.clone())));
        let is_total = matches!(pattern, IntCasePattern::Any { .. }) && clause.guard.is_none();
        let match_condition = match pattern {
            IntCasePattern::Literal(value) => {
                BoolExpr::equal(Expr::int(subject.clone()), Expr::int(IntExpr::value(value)))
            }
            IntCasePattern::Any { .. } => BoolExpr::value(true),
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntCasePattern {
    Literal(BigInt),
    Any { bound_name: Option<EcoString> },
}

impl IntCasePattern {
    fn bound_name(&self) -> Option<&EcoString> {
        match self {
            IntCasePattern::Any { bound_name } => bound_name.as_ref(),
            IntCasePattern::Literal(_) => None,
        }
    }
}

fn plan_int_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<IntCasePattern, PlanError> {
    match pattern {
        Pattern::Int { int_value, .. } => Ok(IntCasePattern::Literal(int_value)),
        Pattern::Variable { name, type_, .. } if type_.is_int() => Ok(IntCasePattern::Any {
            bound_name: Some(name),
        }),
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_int() => {
            Ok(IntCasePattern::Any { bound_name: None })
        }
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

fn clause_has_int_variable_pattern(clause: &TypedClause) -> bool {
    clause.pattern.iter().any(|pattern| {
        matches!(
            pattern,
            Pattern::Variable { type_, .. } if type_.is_int()
        )
    })
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
        ExprKind::Tuple(fallback) => IntCaseBranches::Tuple {
            clauses: tuple_case_clauses(clauses)?,
            fallback,
        },
        ExprKind::List(fallback) => IntCaseBranches::List {
            clauses: list_case_clauses(clauses)?,
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
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
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        };
        typed_clauses.push((value, clause));
    }
    Ok(typed_clauses)
}

fn tuple_case_clauses(
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::TupleExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::ListExpr)>, PlanError> {
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
        crate::plan::FunctionExprKind::Tuple(fallback) => Ok(IntCaseBranches::TupleFunction {
            clauses: tuple_function_case_clauses(clauses)?,
            fallback,
        }),
        crate::plan::FunctionExprKind::List(fallback) => Ok(IntCaseBranches::ListFunction {
            clauses: list_function_case_clauses(clauses)?,
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::StringFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::FloatFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::BoolFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::NilFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::TupleFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::ListFunctionExpr)>, PlanError> {
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
    clauses: Vec<(BigInt, Expr)>,
) -> Result<Vec<(BigInt, crate::plan::FunctionFunctionExpr)>, PlanError> {
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
        BoolExpr, BoolFunctionId, Expr, FloatExpr, FloatFunctionId, FunctionExpr,
        FunctionFunctionId, FunctionType, IntCaseBranches, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntLocalId, IntReturn, ListFunctionId, LocalId, NilFunctionId,
        RuntimeFunctionId, StringFunctionId, TupleFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_int_case, float, function, function_ref, int,
        int_return_block, int_return_expr, int_return_int_case, let_int_step, list,
        list_return_expr, list_return_int_case, local_int, module, nil, nil_return_expr,
        nil_return_int_case, return_list, string, string_return_expr, string_return_int_case,
        tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCaseReason, UnsupportedExpressionKind,
    };
    use gleam_core::ast::{ClauseGuard, Constant, Pattern, TypedModule};
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

pub fn list_case(value: Int) {
  case value {
    1 -> [1]
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
                function(
                    "list_case",
                    return_list(
                        ValueType::Int,
                        list_return_int_case(
                            local_int(0, "value"),
                            [(1, list_return_expr(list([int(1)], ValueType::Int)))],
                            list_return_expr(list([int(0)], ValueType::Int)),
                        ),
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
    fn plan_int_case_variable_pattern_binds_subject_once_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 41 {
    other -> other + 1
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "<case:int:0>", int(41))],
                    int_return_int_case(
                        local_int(0, "<case:int:0>"),
                        [],
                        int_return_block(
                            [let_int_step(1, "other", local_int(0, "<case:int:0>"))],
                            int_return_expr(local_int(1, "other").add_int(int(1))),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_int_case_guard_binds_subject_once_and_falls_through() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 41 {
    other if other > 40 -> other + 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_int_step(1, "other", local_int(0, "<case:int:0>"));
        let condition = BoolExpr::block(
            vec![bind_other.clone()],
            BoolExpr::and(
                BoolExpr::value(true),
                BoolExpr::gt_int(local_int(1, "other").into(), int(40).into()),
            ),
        );
        let guarded_branch = int_return_block(
            [bind_other],
            int_return_expr(local_int(1, "other").add_int(int(1))),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "<case:int:0>", int(41))],
                    IntReturn::bool_case(condition, guarded_branch, int_return_expr(int(0))),
                ),
            ),
            [],
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
            super::tuple_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::tuple_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0),)]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![(BigInt::from(1), Expr::from(int(1)))]),
            Err(case_branch_return_type_mismatch()),
        );
        assert_eq!(
            super::list_function_case_clauses(vec![(BigInt::from(1), int_function_ref_expr(0))]),
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

        assert_int_function_case_branch_mismatch(int_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(string_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(float_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(bool_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(nil_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(tuple_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(list_function_ref_expr(1));
        assert_int_function_case_branch_mismatch(function_function_ref_expr(1));
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
                vec![(BigInt::from(1), list_function_ref_expr(0))],
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List {
                        id: ListFunctionId(1),
                        return_type: Box::new(ValueType::Int),
                    },
                    [LocalId::Int(IntLocalId(0))],
                )),
            ),
            Ok(IntCaseBranches::ListFunction {
                clauses: vec![(
                    BigInt::from(1),
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
                r#"pub fn main() { case 1 { 1 | 2 -> 1 _ -> 0 } }"#,
                UnsupportedCaseReason::AlternativePatterns,
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
    1 -> echo 2
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

        let mut empty_pattern = compile_int_case_module();
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

        let mut case_type_mismatch = compile_int_case_module();
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

        let mut missing_function_fallback_pattern = crate::planner::support::compile(
            r#"
pub fn main() {
  let function = case 1 {
    1 -> add_one
    _ -> add_one
  }
  function(1)
}

fn add_one(value: Int) {
  value + 1
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
    fn reject_margin_int_case_guard_must_be_bool() {
        let mut module = compile_int_case_module();
        let (_, _, clauses) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
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
    fn reject_margin_guarded_int_case_pattern_shapes() {
        let mut empty_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
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

        let mut pattern_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
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
        let mut module = compile_int_case_module();
        let (type_, _, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        *type_ = type_::bit_array();
        assert_eq!(plan_module(module), Err(case_branch_return_type_mismatch()));

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
                Expr::from(tuple([Expr::from(int(0))])),
            ),
            Err(case_branch_return_type_mismatch()),
        );

        assert_eq!(
            super::int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int(10).into())],
                Expr::from(list([int(0)], ValueType::Int)),
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

    fn assert_int_function_case_branch_mismatch(fallback: crate::plan::Expr) {
        assert_eq!(
            super::function_case_branches(
                vec![(BigInt::from(1), Expr::from(int(1)))],
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
