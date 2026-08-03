use super::{InvalidCaseShapeReason, PlanError, invalid_case_shape};
use gleam_core::ast::TypedClause;
use gleam_core::exhaustiveness::{CompiledCase, Decision, FallbackCheck};

pub(super) struct CaseCoverage {
    reachable: Vec<bool>,
    exhaustive_remainder: Option<usize>,
}

impl CaseCoverage {
    pub(super) fn new(
        compiled: &CompiledCase,
        subject_count: usize,
        clauses: &[TypedClause],
    ) -> Result<Self, PlanError> {
        if compiled.subject_variables.len() != subject_count {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::CompiledCaseSubjectCountMismatch,
            ));
        }

        let mut reachable = vec![false; clauses.len()];
        visit(&compiled.tree, clauses, &mut reachable)?;
        let exhaustive_remainder = reachable.iter().rposition(|reachable| *reachable);

        Ok(Self {
            reachable,
            exhaustive_remainder,
        })
    }

    pub(super) fn is_reachable(&self, clause: usize) -> bool {
        self.reachable[clause]
    }

    pub(super) fn is_exhaustive_remainder(&self, clause: usize) -> bool {
        self.exhaustive_remainder == Some(clause)
    }
}

fn visit(
    decision: &Decision,
    clauses: &[TypedClause],
    reachable: &mut [bool],
) -> Result<(), PlanError> {
    match decision {
        Decision::Run { body } => {
            mark_body(body.clause_index, clauses, reachable)?;
            Ok(())
        }
        Decision::Guard {
            guard,
            if_true,
            if_false,
        } => {
            let Some(clause) = clauses.get(*guard) else {
                return Err(invalid_case_shape(
                    InvalidCaseShapeReason::CompiledCaseGuardIndex,
                ));
            };
            if clause.guard.is_none() || if_true.clause_index != *guard {
                return Err(invalid_case_shape(
                    InvalidCaseShapeReason::CompiledCaseGuard,
                ));
            }
            reachable[*guard] = true;
            visit(if_false, clauses, reachable)
        }
        Decision::Switch {
            choices,
            fallback,
            fallback_check,
            ..
        } => {
            match fallback_check.as_ref() {
                FallbackCheck::InfiniteCatchAll
                | FallbackCheck::RuntimeCheck { .. }
                | FallbackCheck::CatchAll { .. } => {}
            }
            for (_, choice) in choices {
                visit(choice, clauses, reachable)?;
            }
            visit(fallback, clauses, reachable)
        }
        Decision::Fail => Err(invalid_case_shape(
            InvalidCaseShapeReason::CompiledCaseFailure,
        )),
    }
}

fn mark_body(
    clause: usize,
    clauses: &[TypedClause],
    reachable: &mut [bool],
) -> Result<(), PlanError> {
    if clauses.get(clause).is_none() {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::CompiledCaseClauseIndex,
        ));
    }
    reachable[clause] = true;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CaseCoverage;
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
    use gleam_core::ast::{Statement, TypedClause, TypedExpr};
    use gleam_core::exhaustiveness::{Body, CompiledCase, Decision};

    #[test]
    fn marks_reachable_clauses_and_the_last_source_remainder() {
        let (compiled, clauses, subject_count) = compile_case(
            r#"
pub type Choice { First(Int) Second(Int) }

fn choose(choice: Choice) {
  case choice {
    First(_) -> 1
    Second(_) -> 2
    _ -> 3
  }
}

pub fn main() { choose(First(1)) }
"#,
        );

        let coverage = CaseCoverage::new(&compiled, subject_count, &clauses)
            .expect("the analyzer proof should be valid");

        assert!(coverage.is_reachable(0));
        assert!(coverage.is_reachable(1));
        assert!(!coverage.is_reachable(2));
        assert!(!coverage.is_exhaustive_remainder(0));
        assert!(coverage.is_exhaustive_remainder(1));
        assert!(!coverage.is_exhaustive_remainder(2));
    }

    #[test]
    fn accepts_guarded_clauses_before_an_exhaustive_remainder() {
        let (compiled, clauses, subject_count) = compile_case(
            r#"
pub fn main() {
  case [1] {
    [value, ..] if value > 1 -> value
    [] -> 0
    [value, ..] -> value
  }
}
"#,
        );

        let coverage = CaseCoverage::new(&compiled, subject_count, &clauses)
            .expect("the guarded compiled proof should be valid");

        assert!(coverage.is_reachable(0));
        assert!(coverage.is_reachable(1));
        assert!(coverage.is_reachable(2));
        assert!(coverage.is_exhaustive_remainder(2));
    }

    #[test]
    fn rejects_compiled_case_subject_count_mismatch() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
pub fn main() {
  case True, False {
    True, _ -> 1
    False, _ -> 0
  }
}
"#,
        );
        compiled.subject_variables.pop();

        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(
                InvalidCaseShapeReason::CompiledCaseSubjectCountMismatch,
            )),
        );
    }

    #[test]
    fn rejects_compiled_case_clause_indices() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
pub fn main() {
  case 1 {
    1 -> 1
    _ -> 0
  }
}
"#,
        );
        compiled.tree = Decision::run(Body::new(clauses.len()));

        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(InvalidCaseShapeReason::CompiledCaseClauseIndex)),
        );
    }

    #[test]
    fn rejects_compiled_case_guard_body_mismatch() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
pub fn main() {
  case 1 {
    value if value > 0 -> value
    _ -> 0
  }
}
"#,
        );
        compiled.tree = Decision::guard(0, Body::new(1), Decision::run(Body::new(1)));
        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(InvalidCaseShapeReason::CompiledCaseGuard)),
        );
    }

    #[test]
    fn rejects_compiled_case_guard_indices() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
pub fn main() {
  case 1 {
    value if value > 0 -> value
    _ -> 0
  }
}
"#,
        );

        compiled.tree = Decision::guard(clauses.len(), Body::new(0), Decision::run(Body::new(1)));
        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(InvalidCaseShapeReason::CompiledCaseGuardIndex,)),
        );
    }

    #[test]
    fn rejects_reachable_compiled_case_failure() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
pub type Choice {
  First
  Second
  Third
}

fn choose(choice: Choice) {
  case choice {
    First -> 1
    Second -> 2
    Third -> 3
  }
}

pub fn main() { choose(First) }
"#,
        );
        compiled.tree = Decision::Fail;

        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(InvalidCaseShapeReason::CompiledCaseFailure)),
        );
    }

    #[test]
    fn rejects_a_failure_reachable_from_a_compiled_switch() {
        let (mut compiled, clauses, subject_count) = compile_case(
            r#"
fn choose(value: Int) {
  case value {
    1 -> 1
    2 -> 2
    _ -> 0
  }
}

pub fn main() { choose(1) }
"#,
        );
        switch_choices(&mut compiled.tree)
            .first_mut()
            .expect("the Int switch should have a choice")
            .1 = Decision::Fail;

        assert_eq!(
            CaseCoverage::new(&compiled, subject_count, &clauses).map(|_| ()),
            Err(case_error(InvalidCaseShapeReason::CompiledCaseFailure)),
        );
    }

    #[test]
    #[should_panic(expected = "expected case expression")]
    fn compile_case_requires_a_case_expression() {
        compile_case("pub fn main() { 1 }");
    }

    #[test]
    #[should_panic(expected = "expected compiled switch")]
    fn switch_choices_requires_a_switch() {
        switch_choices(&mut Decision::run(Body::new(0)));
    }

    fn compile_case(source: &str) -> (CompiledCase, Vec<TypedClause>, usize) {
        let mut module = crate::planner::support::compile(source);
        let Statement::Expression(TypedExpr::Case {
            subjects,
            clauses,
            compiled_case,
            ..
        }) = module.definitions.functions[0].body.remove(0)
        else {
            panic!("expected case expression");
        };
        (compiled_case, clauses, subjects.len())
    }

    fn switch_choices(
        decision: &mut Decision,
    ) -> &mut Vec<(gleam_core::exhaustiveness::RuntimeCheck, Decision)> {
        let Decision::Switch { choices, .. } = decision else {
            panic!("expected compiled switch");
        };
        choices
    }

    fn case_error(reason: InvalidCaseShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape { reason },
        }
    }
}
