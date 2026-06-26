use super::{
    invalid_case_shape, single_case_pattern, unsupported_case, validate_case_branch_type,
    validate_clause_shape,
};
use crate::plan::{BoolExpr, Expr};
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
    let subject = super::super::plan_bool_expr(subject, context)?;
    let mut true_branch = None;
    let mut false_branch = None;
    for clause in clauses {
        validate_clause_shape(&clause)?;
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_bool_case_pattern(pattern)?;
        let branch = super::super::plan_expr(clause.then, context)?;
        validate_case_branch_type(type_.as_ref(), &branch)?;

        match pattern {
            BoolCasePattern::True => set_case_branch(&mut true_branch, branch),
            BoolCasePattern::False => set_case_branch(&mut false_branch, branch),
            BoolCasePattern::Any => {
                set_case_branch(&mut true_branch, branch.clone());
                set_case_branch(&mut false_branch, branch);
            }
        }
    }

    let true_ = true_branch.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingTruePattern,
    ))?;
    let false_ = false_branch.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFalsePattern,
    ))?;

    bool_case_expr(subject, true_, false_)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoolCasePattern {
    True,
    False,
    Any,
}

fn plan_bool_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<BoolCasePattern, PlanError> {
    match pattern {
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => match name.as_str() {
            "True" => Ok(BoolCasePattern::True),
            "False" => Ok(BoolCasePattern::False),
            _ => Err(invalid_case_shape(
                InvalidCaseShapeReason::PatternTypeMismatch,
            )),
        },
        Pattern::Variable { type_, .. } if type_.is_bool() => {
            Err(unsupported_case(UnsupportedCaseReason::VariablePattern))
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_bool() => Ok(BoolCasePattern::Any),
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { pattern, .. } => match validate_bool_case_assign_pattern(&pattern) {
            Ok(()) => Err(unsupported_case(UnsupportedCaseReason::AssignPattern)),
            Err(reason) => Err(invalid_case_shape(reason)),
        },
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Int { .. }
        | Pattern::Float { .. }
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

fn validate_bool_case_assign_pattern(
    pattern: &Pattern<Arc<Type>>,
) -> Result<(), InvalidCaseShapeReason> {
    match pattern {
        Pattern::Constructor {
            name,
            arguments,
            spread,
            type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => {
            if matches!(name.as_str(), "True" | "False") {
                Ok(())
            } else {
                Err(InvalidCaseShapeReason::PatternTypeMismatch)
            }
        }
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } if type_.is_bool() => {
            Ok(())
        }
        Pattern::Invalid { .. } => Err(InvalidCaseShapeReason::InvalidPattern),
        _ => Err(InvalidCaseShapeReason::PatternTypeMismatch),
    }
}

fn set_case_branch(branch: &mut Option<Expr>, value: Expr) {
    if branch.is_none() {
        *branch = Some(value);
    }
}

fn bool_case_expr(subject: BoolExpr, true_: Expr, false_: Expr) -> Result<Expr, PlanError> {
    Expr::bool_case(subject, true_, false_)
        .map_err(|_| invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch))
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{
        bool_, bool_case_bool, bool_case_int, bool_case_nil, bool_case_string, call_bool, function,
        int, local_bool, module, nil, string,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
    };
    use gleam_core::ast::Pattern;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

    #[test]
    fn plan_bool_case_expressions() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    True -> 1
    False -> 0
  }
}

pub fn string_case(value: Bool) {
  case value {
    True -> "yes"
    False -> "no"
  }
}

pub fn bool_case() {
  case !False {
    True -> False
    False -> True
  }
}

pub fn nil_case() {
  case 1 < 2 {
    True -> Nil
    False -> Nil
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", bool_case_int(bool_(true), int(1), int(0))),
            [
                function(
                    "string_case",
                    bool_case_string(local_bool(0, "value"), string("yes"), string("no")),
                )
                .param_bool(0, "value"),
                function(
                    "bool_case",
                    bool_case_bool(bool_(false).negate_bool(), bool_(false), bool_(true)),
                ),
                function(
                    "nil_case",
                    bool_case_nil(int(1).lt_int(int(2)), nil(), nil()),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_function_call_subject() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
fn flag() {
  True
}

pub fn main() {
  case flag() {
    True -> 1
    False -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", bool_case_int(call_bool(0, []), int(1), int(0))),
            [function("flag", bool_(true))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_wildcard_fallbacks() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    True -> 1
    _ -> 0
  }
}

fn false_fallback(value: Bool) {
  case value {
    False -> 0
    _ -> 1
  }
}

fn only_fallback(value: Bool) {
  case value {
    _ -> 1
  }
}

fn fallback_first(value: Bool) {
  case value {
    _ -> 0
    True -> 1
  }
}

fn redundant_fallback(value: Bool) {
  case value {
    True -> 1
    False -> 0
    _ -> 2
  }
}

fn duplicate_true(value: Bool) {
  case value {
    True -> 1
    True -> 2
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", bool_case_int(bool_(true), int(1), int(0))),
            [
                function(
                    "false_fallback",
                    bool_case_int(local_bool(0, "value"), int(1), int(0)),
                )
                .param_bool(0, "value"),
                function(
                    "only_fallback",
                    bool_case_int(local_bool(0, "value"), int(1), int(1)),
                )
                .param_bool(0, "value"),
                function(
                    "fallback_first",
                    bool_case_int(local_bool(0, "value"), int(0), int(0)),
                )
                .param_bool(0, "value"),
                function(
                    "redundant_fallback",
                    bool_case_int(local_bool(0, "value"), int(1), int(0)),
                )
                .param_bool(0, "value"),
                function(
                    "duplicate_true",
                    bool_case_int(local_bool(0, "value"), int(1), int(0)),
                )
                .param_bool(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_bool_case_patterns() {
        let cases = [
            (
                r#"pub fn main() { case True { value -> 1 } }"#,
                UnsupportedCaseReason::VariablePattern,
            ),
            (
                r#"
pub fn main() {
  case True {
    True as value -> 1
    False -> 0
  }
}
"#,
                UnsupportedCaseReason::AssignPattern,
            ),
            (
                r#"
pub fn main() {
  case True {
    value as alias -> 1
  }
}
"#,
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
    fn reject_margin_bool_case_pattern_shapes() {
        let mut invalid_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::bool(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );

        let mut pattern_type_mismatch = super::super::compile_bool_case_module();
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

        let mut variable_type_mismatch = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut variable_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: type_::int(),
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

        let mut discard_type_mismatch = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut discard_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(discard_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut assign_type_mismatch = super::super::compile_bool_case_module();
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

        let mut assign_constructor_name_mismatch = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_constructor_name_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Other".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: type_::bool(),
            }),
        };
        assert_eq!(
            plan_module(assign_constructor_name_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut assign_invalid_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut assign_invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "value".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Invalid {
                location: dummy_span(),
                type_: type_::bool(),
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

        let mut bool_constructor_name_mismatch = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut bool_constructor_name_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Other".into(),
            arguments: Vec::new(),
            module: None,
            constructor: Default::default(),
            spread: None,
            type_: type_::bool(),
        };
        assert_eq!(
            plan_module(bool_constructor_name_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut missing_true_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut missing_true_pattern.definitions.functions[0].body[0],
        );
        clauses.remove(0);
        assert_eq!(
            plan_module(missing_true_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingTruePattern,
                },
            }),
        );

        let mut missing_false_pattern = super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::expect_case_statement_mut(
            &mut missing_false_pattern.definitions.functions[0].body[0],
        );
        clauses.pop();
        assert_eq!(
            plan_module(missing_false_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFalsePattern,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bool_case_expr_type_mismatch() {
        assert_eq!(
            super::bool_case_expr(bool_(true).into(), int(1).into(), bool_(false).into()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bool_case_function_branch_type_mismatch() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  let function = case True {
    True -> add_one
    False -> add_one
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
}
