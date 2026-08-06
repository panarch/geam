use super::super::super::plan_int_expr;
use super::super::coverage::{CaseBranchRequirement, require_branch};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, IntExpr, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use num_bigint::BigInt;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_int_expr(subject, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    if clauses
        .iter()
        .any(|clause| clause.guard.is_some() || clause.has_alternative_patterns())
    {
        let (subject_step, subject) = super::bind_int_case_subject(subject, context);
        let case = plan_guarded_int_case(return_shape, subject, clauses, context)?;
        return Ok(super::case_subject_block(subject_step, case));
    }
    let needs_subject_binding = clauses.iter().any(clause_has_int_bound_name);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_int_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        let pattern = plan_int_case_pattern(clause.pattern, context)?;
        let bindings = super::branch_bindings(pattern.bound_names(), Expr::int(subject.clone()));
        let branch = super::plan_case_branch(&return_shape, clause.then, bindings, context)?;

        match pattern {
            IntCasePattern::Literal { value, .. } => {
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

    let fallback = require_branch(fallback, CaseBranchRequirement::Fallback)?;

    super::super::result::int_case_expr(subject, literal_clauses, fallback).map(|case| {
        match subject_step {
            Some(step) => super::case_subject_block(step, case),
            None => case,
        }
    })
}

fn plan_guarded_int_case(
    return_shape: ValueShape,
    subject: IntExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_int_case_pattern(pattern, context)?;
            let bindings =
                super::branch_bindings(pattern.bound_names(), Expr::int(subject.clone()));
            let is_total = matches!(pattern, IntCasePattern::Any { .. }) && clause.guard.is_none();
            let match_condition = match pattern {
                IntCasePattern::Literal { value, .. } => {
                    BoolExpr::equal(Expr::int(subject.clone()), Expr::int(IntExpr::value(value)))
                }
                IntCasePattern::Any { .. } => BoolExpr::value(true),
            };
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition,
                    is_total,
                    reachable,
                    exhaustive_remainder,
                },
                context,
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntCasePattern {
    Literal {
        value: BigInt,
        bound_names: Vec<EcoString>,
    },
    Any {
        bound_names: Vec<EcoString>,
    },
}

impl IntCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        match self {
            IntCasePattern::Literal { bound_names, .. } | IntCasePattern::Any { bound_names } => {
                bound_names
            }
        }
    }

    fn add_bound_name(&mut self, name: EcoString) {
        match self {
            IntCasePattern::Literal { bound_names, .. } | IntCasePattern::Any { bound_names } => {
                bound_names.push(name);
            }
        }
    }
}

fn plan_int_case_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<IntCasePattern, PlanError> {
    match pattern {
        Pattern::Int { int_value, .. } => Ok(IntCasePattern::Literal {
            value: int_value,
            bound_names: Vec::new(),
        }),
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::Int, context)?;
            Ok(IntCasePattern::Any {
                bound_names: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::Int, context)?;
            Ok(IntCasePattern::Any {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_int_case_pattern(*pattern, context)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        pattern @ (Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &ValueShape::Int,
            context,
        )),
    }
}

fn clause_has_int_bound_name(clause: &CaseClause) -> bool {
    int_pattern_has_bound_name(&clause.pattern)
}

fn int_pattern_has_bound_name(pattern: &Pattern<Arc<Type>>) -> bool {
    match pattern {
        Pattern::Variable { type_, .. } if type_.is_int() => true,
        Pattern::Assign { .. } => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, FunctionType, IntReturn, ValueType};
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_int_case, function, int, int_return_block,
        int_return_expr, int_return_int_case, let_int_step, list, list_return_expr,
        list_return_int_case, local_int, module, nil, nil_return_expr, nil_return_int_case,
        return_list, string, string_return_expr, string_return_int_case,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::ast::{ClauseGuard, Constant, Pattern, TypedModule};
    use gleam_core::exhaustiveness::{Body, Decision};
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
                    return_list(list_return_int_case(
                        local_int(0, "value"),
                        [(1, list_return_expr(list([int(1)], ValueType::Int)))],
                        list_return_expr(list([int(0)], ValueType::Int)),
                    )),
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
    fn plan_int_case_variable_alias_binds_inner_then_alias_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 41 {
    other as alias -> other + alias
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
                            [
                                let_int_step(1, "other", local_int(0, "<case:int:0>")),
                                let_int_step(2, "alias", local_int(0, "<case:int:0>")),
                            ],
                            int_return_expr(local_int(1, "other").add_int(local_int(2, "alias"))),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_int_case_literal_alias_binds_subject_once_for_alias_value() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1 {
    1 as alias -> alias
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
                int_return_block(
                    [let_int_step(0, "<case:int:0>", int(1))],
                    int_return_int_case(
                        local_int(0, "<case:int:0>"),
                        [(
                            1,
                            int_return_block(
                                [let_int_step(1, "alias", local_int(0, "<case:int:0>"))],
                                int_return_expr(local_int(1, "alias")),
                            ),
                        )],
                        int_return_expr(int(0)),
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
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_other.clone()],
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
    fn plan_int_case_guarded_alias_binds_guard_and_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 2 {
    other as alias if alias == 2 -> other + alias
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_int_step(1, "other", local_int(0, "<case:int:0>"));
        let bind_alias = let_int_step(2, "alias", local_int(0, "<case:int:0>"));
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_other.clone(), bind_alias.clone()],
                BoolExpr::equal(local_int(2, "alias").into(), int(2).into()),
            ),
        );
        let guarded_branch = int_return_block(
            [bind_other, bind_alias],
            int_return_expr(local_int(1, "other").add_int(local_int(2, "alias"))),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "<case:int:0>", int(2))],
                    IntReturn::bool_case(condition, guarded_branch, int_return_expr(int(0))),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_int_case_alternative_patterns_expand_to_ordered_fallthrough() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case 1 {
    1 | 2 -> 10
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let first_condition = BoolExpr::equal(local_int(0, "<case:int:0>").into(), int(1).into());
        let second_condition = BoolExpr::equal(local_int(0, "<case:int:0>").into(), int(2).into());
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "<case:int:0>", int(1))],
                    IntReturn::bool_case(
                        first_condition,
                        int_return_expr(int(10)),
                        IntReturn::bool_case(
                            second_condition,
                            int_return_expr(int(10)),
                            int_return_expr(int(0)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_int_case_unreachable_duplicate_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case 1 {
    1 -> 1
    1 -> { <<1:native>> 2 }
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn reject_margin_int_case_pattern_shapes() {
        let mut variable_type_mismatch = compile_int_case_module();
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Bool,
            )),
        );

        let mut discard_type_mismatch = compile_int_case_module();
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Bool,
            )),
        );

        let mut invalid_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_int_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::String {
            location: dummy_span(),
            value: "bad".into(),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );

        let mut assign_invalid_pattern = compile_int_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut assign_type_mismatch = compile_int_case_module();
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );

        let mut empty_pattern = compile_int_case_module();
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

        let mut case_type_mismatch = compile_int_case_module();
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
            &mut case_type_mismatch.definitions.functions[0].body[0],
        );
        *case_type = type_::bool();
        assert_eq!(
            plan_module(case_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Bool,
                        actual: ValueType::Int,
                    },
                },
            }),
        );

        let mut invalid_compiled_clause = compile_int_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_compiled_clause.definitions.functions[0].body[0],
        );
        clauses.pop();
        assert_eq!(
            plan_module(invalid_compiled_clause),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::CompiledCaseClauseIndex,
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
        let (_, _, clauses, compiled_case) =
            super::super::super::expect_assignment_case_statement_mut(&mut body[0]);
        clauses.pop();
        compiled_case.tree = Decision::run(Body::new(0));
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
    fn reject_margin_guarded_int_case_pattern_shapes() {
        let mut empty_pattern = compile_int_case_module();
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

        let mut pattern_type_mismatch = compile_int_case_module();
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn reject_margin_int_case_subject_type_mismatch() {
        let mut module = compile_int_case_module();
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
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
        let replacement = super::super::super::expect_expression_statement(&body[1]).clone();
        let (_, _, clauses, _) =
            super::super::super::expect_assignment_case_statement_mut(&mut body[0]);
        clauses[1].then = replacement;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Function(Box::new(FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::Int,
                        ))),
                        actual: ValueType::Function(Box::new(FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::String,
                        ))),
                    },
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
