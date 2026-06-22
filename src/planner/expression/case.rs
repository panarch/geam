use super::{plan_bool_expr, plan_expr};
use crate::plan::{BoolExpr, Expr, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
};
use gleam_core::ast::{Pattern, TypedClause, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_case(
    type_: Arc<Type>,
    subjects: Vec<TypedExpr>,
    clauses: Vec<TypedClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = single_case_subject(subjects)?;
    if !subject.type_().is_bool() {
        return Err(unsupported_case(UnsupportedCaseReason::NonBoolSubject));
    }
    let subject = plan_bool_expr(subject, context)?;

    if clauses.is_empty() {
        return Err(invalid_case_shape(InvalidCaseShapeReason::EmptyClauses));
    }

    let mut true_branch = None;
    let mut false_branch = None;
    for clause in clauses {
        if !clause.alternative_patterns.is_empty() {
            return Err(unsupported_case(UnsupportedCaseReason::AlternativePatterns));
        }
        if clause.guard.is_some() {
            return Err(unsupported_case(UnsupportedCaseReason::Guard));
        }

        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_bool_case_pattern(pattern)?;
        let branch = plan_expr(clause.then, context)?;

        match pattern {
            BoolCasePattern::True => set_case_branch(&mut true_branch, branch)?,
            BoolCasePattern::False => set_case_branch(&mut false_branch, branch)?,
        }
    }

    let true_ = true_branch.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingTruePattern,
    ))?;
    let false_ = false_branch.ok_or(invalid_case_shape(
        InvalidCaseShapeReason::MissingFalsePattern,
    ))?;

    bool_case_expr(type_, subject, true_, false_)
}

fn single_case_subject(subjects: Vec<TypedExpr>) -> Result<TypedExpr, PlanError> {
    let mut subjects = subjects.into_iter();
    let subject = subjects
        .next()
        .ok_or(invalid_case_shape(InvalidCaseShapeReason::EmptySubjects))?;
    if subjects.next().is_some() {
        return Err(unsupported_case(UnsupportedCaseReason::MultipleSubjects));
    }

    Ok(subject)
}

fn single_case_pattern(patterns: Vec<Pattern<Arc<Type>>>) -> Result<Pattern<Arc<Type>>, PlanError> {
    let mut patterns = patterns.into_iter();
    let pattern = patterns.next().ok_or(invalid_case_shape(
        InvalidCaseShapeReason::PatternSubjectCountMismatch,
    ))?;
    if patterns.next().is_some() {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternSubjectCountMismatch,
        ));
    }

    Ok(pattern)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoolCasePattern {
    True,
    False,
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
        Pattern::Variable { .. } => Err(unsupported_case(UnsupportedCaseReason::VariablePattern)),
        Pattern::Discard { .. } => Err(unsupported_case(UnsupportedCaseReason::DiscardPattern)),
        Pattern::Assign { .. } => Err(unsupported_case(UnsupportedCaseReason::AssignPattern)),
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

fn set_case_branch(branch: &mut Option<Expr>, value: Expr) -> Result<(), PlanError> {
    if branch.is_some() {
        return Err(unsupported_case(
            UnsupportedCaseReason::DuplicateBoolPattern,
        ));
    }

    *branch = Some(value);
    Ok(())
}

fn bool_case_expr(
    case_type: Arc<Type>,
    subject: BoolExpr,
    true_: Expr,
    false_: Expr,
) -> Result<Expr, PlanError> {
    let branch_type = true_.value_type();
    if ValueType::from_gleam(case_type.as_ref()) != Some(branch_type) {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::BranchReturnTypeMismatch,
        ));
    }

    Expr::bool_case(subject, true_, false_)
        .map_err(|_| invalid_case_shape(InvalidCaseShapeReason::BranchReturnTypeMismatch))
}

fn unsupported_case(reason: UnsupportedCaseReason) -> PlanError {
    PlanError::UnsupportedCase { reason }
}

fn invalid_case_shape(reason: InvalidCaseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CaseShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{
        bool_, call_bool, case_bool, case_int, case_nil, case_string, function, int, local_bool,
        module, nil, string,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedCaseReason,
    };
    use gleam_core::ast::{
        Pattern, Statement, TypedClause, TypedExpr, TypedModule, TypedStatement,
    };
    use gleam_core::type_;
    use num_bigint::BigInt;

    #[test]
    fn plan_bool_case_expressions() {
        let actual = plan_module(compile(
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
            function("main", case_int(bool_(true), int(1), int(0))),
            [
                function(
                    "string_case",
                    case_string(local_bool(0, "value"), string("yes"), string("no")),
                )
                .param_bool(0, "value"),
                function(
                    "bool_case",
                    case_bool(bool_(false).negate_bool(), bool_(false), bool_(true)),
                ),
                function("nil_case", case_nil(int(1).lt_int(int(2)), nil(), nil())),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_function_call_subject() {
        let actual = plan_module(compile(
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
            function("main", case_int(call_bool(0, []), int(1), int(0))),
            [function("flag", bool_(true))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_case_expressions() {
        let cases = [
            (
                r#"
pub fn main() {
  case True, False {
    True, False -> 1
    _, _ -> 0
  }
}
"#,
                UnsupportedCaseReason::MultipleSubjects,
            ),
            (
                r#"pub fn main() { case 1 { 1 -> 2 _ -> 3 } }"#,
                UnsupportedCaseReason::NonBoolSubject,
            ),
            (
                r#"pub fn main() { case True { value -> 1 } }"#,
                UnsupportedCaseReason::VariablePattern,
            ),
            (
                r#"pub fn main() { case True { _ -> 1 } }"#,
                UnsupportedCaseReason::DiscardPattern,
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
    True if True -> 1
    False -> 0
    _ -> 2
  }
}
"#,
                UnsupportedCaseReason::Guard,
            ),
            (
                r#"pub fn main() { case True { True | False -> 1 } }"#,
                UnsupportedCaseReason::AlternativePatterns,
            ),
            (
                r#"
pub fn main() {
  case True {
    True -> 1
    True -> 2
    False -> 0
  }
}
"#,
                UnsupportedCaseReason::DuplicateBoolPattern,
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
    fn reject_margin_case_shapes() {
        let mut empty_subjects = compile_bool_case_module();
        let (_, subjects, _) =
            expect_case_statement_mut(&mut empty_subjects.definitions.functions[0].body[0]);
        subjects.clear();
        assert_eq!(
            plan_module(empty_subjects),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::EmptySubjects,
                },
            }),
        );

        let mut empty_clauses = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut empty_clauses.definitions.functions[0].body[0]);
        clauses.clear();
        assert_eq!(
            plan_module(empty_clauses),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::EmptyClauses,
                },
            }),
        );

        let mut empty_pattern = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut empty_pattern.definitions.functions[0].body[0]);
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut extra_pattern = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut extra_pattern.definitions.functions[0].body[0]);
        let pattern = clauses[0].pattern[0].clone();
        clauses[0].pattern.push(pattern);
        assert_eq!(
            plan_module(extra_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut invalid_pattern = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut invalid_pattern.definitions.functions[0].body[0]);
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

        let mut pattern_type_mismatch = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut pattern_type_mismatch.definitions.functions[0].body[0]);
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

        let mut bool_constructor_name_mismatch = compile_bool_case_module();
        let (_, _, clauses) = expect_case_statement_mut(
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

        let mut missing_true_pattern = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut missing_true_pattern.definitions.functions[0].body[0]);
        clauses.remove(0);
        assert_eq!(
            plan_module(missing_true_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingTruePattern,
                },
            }),
        );

        let mut missing_false_pattern = compile_bool_case_module();
        let (_, _, clauses) =
            expect_case_statement_mut(&mut missing_false_pattern.definitions.functions[0].body[0]);
        clauses.pop();
        assert_eq!(
            plan_module(missing_false_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFalsePattern,
                },
            }),
        );

        let mut case_type_mismatch = compile_bool_case_module();
        let (case_type, _, _) =
            expect_case_statement_mut(&mut case_type_mismatch.definitions.functions[0].body[0]);
        *case_type = type_::bool();
        assert_eq!(
            plan_module(case_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut branch_type_mismatch = compile_bool_case_module();
        let (case_type, _, clauses) =
            expect_case_statement_mut(&mut branch_type_mismatch.definitions.functions[0].body[0]);
        *case_type = type_::string();
        clauses[0].then = TypedExpr::String {
            location: dummy_span(),
            type_: type_::string(),
            value: "bad".into(),
        };
        assert_eq!(
            plan_module(branch_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
    }

    fn compile_bool_case_module() -> TypedModule {
        compile(
            r#"
pub fn main() {
  case True {
    True -> 1
    False -> 0
  }
}
"#,
        )
    }

    fn expect_case_statement_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut Vec<TypedExpr>,
        &mut Vec<TypedClause>,
    ) {
        let Statement::Expression(TypedExpr::Case {
            type_,
            subjects,
            clauses,
            ..
        }) = statement
        else {
            panic!("expected case expression statement");
        };
        (type_, subjects, clauses)
    }

    #[test]
    #[should_panic(expected = "expected case expression statement")]
    fn expect_case_statement_mut_panics_on_int() {
        let mut module = compile_minimal_module();

        expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
    }
}
