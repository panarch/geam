use super::super::super::plan_bool_expr;
use super::super::coverage::{CaseBranchRequirement, require_branch};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_bool_expr(subject, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    if clauses
        .iter()
        .any(|clause| clause.guard.is_some() || clause.has_alternative_patterns())
    {
        let (subject_step, subject) = super::bind_bool_case_subject(subject, context);
        let case = plan_guarded_bool_case(return_shape, subject, clauses, context)?;
        return Ok(super::case_subject_block(subject_step, case));
    }
    let needs_subject_binding = clauses.iter().any(clause_has_bool_bound_name);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_bool_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut true_branch = None;
    let mut false_branch = None;
    for clause in clauses {
        let pattern = plan_bool_case_pattern(clause.pattern, context)?;
        let bindings = super::branch_bindings(pattern.bound_names(), Expr::bool(subject.clone()));
        let branch = super::plan_case_branch(&return_shape, clause.then, bindings, context)?;

        match pattern {
            BoolCasePattern::Literal { value: true, .. } => {
                set_case_branch(&mut true_branch, branch)
            }
            BoolCasePattern::Literal { value: false, .. } => {
                set_case_branch(&mut false_branch, branch)
            }
            BoolCasePattern::Any { .. } => {
                set_case_branch(&mut true_branch, branch.clone());
                set_case_branch(&mut false_branch, branch);
            }
        }
    }

    let true_ = require_branch(true_branch, CaseBranchRequirement::True)?;
    let false_ = require_branch(false_branch, CaseBranchRequirement::False)?;

    super::super::result::bool_case_expr(subject, true_, false_).map(|case| match subject_step {
        Some(step) => super::case_subject_block(step, case),
        None => case,
    })
}

fn plan_guarded_bool_case(
    return_shape: ValueShape,
    subject: BoolExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut true_clauses = Vec::new();
    let mut false_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_bool_case_pattern(pattern, context)?;
            let bindings =
                super::branch_bindings(pattern.bound_names(), Expr::bool(subject.clone()));
            let is_total = clause.guard.is_none();
            let ordered_clause = super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
                    reachable,
                    exhaustive_remainder,
                },
                context,
            )?;

            match pattern {
                BoolCasePattern::Literal { value: true, .. } => true_clauses.push(ordered_clause),
                BoolCasePattern::Literal { value: false, .. } => false_clauses.push(ordered_clause),
                BoolCasePattern::Any { .. } => {
                    true_clauses.push(ordered_clause.clone());
                    false_clauses.push(ordered_clause);
                }
            }
        }
    }

    let true_ = super::ordered_case_expr_for(true_clauses, CaseBranchRequirement::True)?;
    let false_ = super::ordered_case_expr_for(false_clauses, CaseBranchRequirement::False)?;

    super::super::result::bool_case_expr(subject, true_, false_)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoolCasePattern {
    Literal {
        value: bool,
        bound_names: Vec<EcoString>,
    },
    Any {
        bound_names: Vec<EcoString>,
    },
}

impl BoolCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        match self {
            BoolCasePattern::Literal { bound_names, .. } | BoolCasePattern::Any { bound_names } => {
                bound_names
            }
        }
    }

    fn add_bound_name(&mut self, name: EcoString) {
        match self {
            BoolCasePattern::Literal { bound_names, .. } | BoolCasePattern::Any { bound_names } => {
                bound_names.push(name);
            }
        }
    }
}

fn plan_bool_case_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<BoolCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Constructor {
            ref name,
            ref arguments,
            ref spread,
            ref type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => match name.as_str() {
            "True" | "False" => {
                crate::planner::pattern::validate_pattern(pattern, &ValueShape::Bool, context)?;
                Ok(BoolCasePattern::Literal {
                    value: name == "True",
                    bound_names: Vec::new(),
                })
            }
            _ => Err(crate::planner::pattern::unexpected_pattern(
                pattern,
                &ValueShape::Bool,
                context,
            )),
        },
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::Bool, context)?;
            Ok(BoolCasePattern::Any {
                bound_names: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::Bool, context)?;
            Ok(BoolCasePattern::Any {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_bool_case_pattern(*pattern, context)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &ValueShape::Bool,
            context,
        )),
    }
}

#[cfg(test)]
fn plan_bool_case_pattern_for_test(
    pattern: Pattern<Arc<Type>>,
) -> Result<BoolCasePattern, PlanError> {
    let module = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module, &functions, &mut anonymous);
    plan_bool_case_pattern(pattern, &context)
}

fn clause_has_bool_bound_name(clause: &CaseClause) -> bool {
    bool_pattern_has_bound_name(&clause.pattern)
}

fn bool_pattern_has_bound_name(pattern: &Pattern<Arc<Type>>) -> bool {
    match pattern {
        Pattern::Variable { type_, .. } if type_.is_bool() => true,
        Pattern::Assign { .. } => true,
        _ => false,
    }
}

fn set_case_branch(branch: &mut Option<Expr>, value: Expr) {
    if branch.is_none() {
        *branch = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, FunctionType, IntReturn, ValueType};
    use crate::planner::dsl::{
        bool_, bool_return_block, bool_return_bool_case, bool_return_expr, call_bool_at, function,
        int, int_return_block, int_return_bool_case, int_return_expr, let_bool_step, list,
        list_return_bool_case, list_return_expr, local_bool, module, nil, nil_return_bool_case,
        nil_return_expr, return_list, string, string_return_bool_case, string_return_expr,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::ast::{BinOp, ClauseGuard, Constant, Pattern};
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

pub fn list_case(value: Bool) {
  case value {
    True -> [1]
    False -> [0]
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_bool_case(
                    bool_(true),
                    int_return_expr(int(1)),
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "string_case",
                    string_return_bool_case(
                        local_bool(0, "value"),
                        string_return_expr(string("yes")),
                        string_return_expr(string("no")),
                    ),
                )
                .param_bool(0, "value"),
                function(
                    "bool_case",
                    bool_return_bool_case(
                        bool_(false).negate_bool(),
                        bool_return_expr(bool_(false)),
                        bool_return_expr(bool_(true)),
                    ),
                ),
                function(
                    "nil_case",
                    nil_return_bool_case(
                        int(1).lt_int(int(2)),
                        nil_return_expr(nil()),
                        nil_return_expr(nil()),
                    ),
                ),
                function(
                    "list_case",
                    return_list(list_return_bool_case(
                        local_bool(0, "value"),
                        list_return_expr(list([int(1)], ValueType::Int)),
                        list_return_expr(list([int(0)], ValueType::Int)),
                    )),
                )
                .param_bool(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_variable_pattern_binds_subject_once_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    other -> other
  }
}
"#,
        ))
        .expect("source should plan");
        let branch = bool_return_block(
            [let_bool_step(1, "other", local_bool(0, "<case:bool:0>"))],
            bool_return_expr(local_bool(1, "other")),
        );
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_bool_step(0, "<case:bool:0>", bool_(true))],
                    bool_return_bool_case(local_bool(0, "<case:bool:0>"), branch.clone(), branch),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_variable_alias_binds_inner_then_alias_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    value as alias -> value && alias
  }
}
"#,
        ))
        .expect("source should plan");
        let branch = bool_return_block(
            [
                let_bool_step(1, "value", local_bool(0, "<case:bool:0>")),
                let_bool_step(2, "alias", local_bool(0, "<case:bool:0>")),
            ],
            bool_return_expr(local_bool(1, "value").and_bool(local_bool(2, "alias"))),
        );
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_bool_step(0, "<case:bool:0>", bool_(true))],
                    bool_return_bool_case(local_bool(0, "<case:bool:0>"), branch.clone(), branch),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_literal_alias_binds_subject_once_for_alias_value() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    True as alias -> alias
    False -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_bool_step(0, "<case:bool:0>", bool_(true))],
                    bool_return_bool_case(
                        local_bool(0, "<case:bool:0>"),
                        bool_return_block(
                            [let_bool_step(1, "alias", local_bool(0, "<case:bool:0>"))],
                            bool_return_expr(local_bool(1, "alias")),
                        ),
                        bool_return_expr(bool_(false)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_guard_binds_subject_once_and_falls_through() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    other if other -> 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_bool_step(1, "other", local_bool(0, "<case:bool:0>"));
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(vec![bind_other.clone()], local_bool(1, "other").into()),
        );
        let guarded_branch = int_return_block([bind_other], int_return_expr(int(1)));
        let guarded_case = IntReturn::bool_case(condition, guarded_branch, int_return_expr(int(0)));
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_bool_step(0, "<case:bool:0>", bool_(true))],
                    int_return_bool_case(
                        local_bool(0, "<case:bool:0>"),
                        guarded_case.clone(),
                        guarded_case,
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_guarded_alias_binds_guard_and_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case True {
    value as alias if value && alias -> alias
    _ -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_value = let_bool_step(1, "value", local_bool(0, "<case:bool:0>"));
        let bind_alias = let_bool_step(2, "alias", local_bool(0, "<case:bool:0>"));
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_value.clone(), bind_alias.clone()],
                local_bool(1, "value")
                    .and_bool(local_bool(2, "alias"))
                    .into(),
            ),
        );
        let guarded_branch = bool_return_block(
            [bind_value, bind_alias],
            bool_return_expr(local_bool(2, "alias")),
        );
        let guarded_case = crate::plan::BoolReturn::bool_case(
            condition,
            guarded_branch,
            bool_return_expr(bool_(false)),
        );
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_bool_step(0, "<case:bool:0>", bool_(true))],
                    bool_return_bool_case(
                        local_bool(0, "<case:bool:0>"),
                        guarded_case.clone(),
                        guarded_case,
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_function_call_subject() {
        let source = r#"
fn flag() {
  True
}

pub fn main() {
  case flag() {
    True -> 1
    False -> 0
  }
}
"#;
        let actual =
            plan_module(crate::planner::support::compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_bool_case(
                    call_bool_at(
                        1,
                        [],
                        crate::planner::dsl::host_call_site(source, "main", "flag()"),
                    ),
                    int_return_expr(int(1)),
                    int_return_expr(int(0)),
                ),
            ),
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
            function(
                "main",
                int_return_bool_case(
                    bool_(true),
                    int_return_expr(int(1)),
                    int_return_expr(int(0)),
                ),
            ),
            [
                function(
                    "false_fallback",
                    int_return_bool_case(
                        local_bool(0, "value"),
                        int_return_expr(int(1)),
                        int_return_expr(int(0)),
                    ),
                )
                .param_bool(0, "value"),
                function(
                    "only_fallback",
                    int_return_bool_case(
                        local_bool(0, "value"),
                        int_return_expr(int(1)),
                        int_return_expr(int(1)),
                    ),
                )
                .param_bool(0, "value"),
                function(
                    "fallback_first",
                    int_return_bool_case(
                        local_bool(0, "value"),
                        int_return_expr(int(0)),
                        int_return_expr(int(0)),
                    ),
                )
                .param_bool(0, "value"),
                function(
                    "redundant_fallback",
                    int_return_bool_case(
                        local_bool(0, "value"),
                        int_return_expr(int(1)),
                        int_return_expr(int(0)),
                    ),
                )
                .param_bool(0, "value"),
                function(
                    "duplicate_true",
                    int_return_bool_case(
                        local_bool(0, "value"),
                        int_return_expr(int(1)),
                        int_return_expr(int(0)),
                    ),
                )
                .param_bool(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_bool_case_subject_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case { <<1:native>> True } {
    True -> 1
    False -> 0
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
    fn reject_margin_bool_case_function_branch_missing_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case False {
    False -> add_one
  }
}
"#,
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingTruePattern,
                },
            },
        );
    }

    #[test]
    fn reject_margin_bool_case_pattern_shapes() {
        assert_eq!(
            super::plan_bool_case_pattern_for_test(Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "True".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: type_::bool(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );
        assert_eq!(
            super::plan_bool_case_pattern_for_test(Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Other".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Default::default(),
                spread: None,
                type_: type_::bool(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );

        let mut invalid_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::bool(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut pattern_type_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::Bool,
                ValueType::Int,
            )),
        );

        let mut variable_type_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Bool,
                ValueType::Int,
            )),
        );

        let mut discard_type_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut discard_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(discard_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::Bool,
                ValueType::Int,
            )),
        );

        let mut assign_type_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
            Err(super::super::pattern_type_mismatch(
                ValueType::Bool,
                ValueType::Int,
            )),
        );

        let mut assign_constructor_name_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );

        let mut assign_invalid_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut bool_constructor_name_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
                },
            }),
        );

        let mut missing_true_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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

        let mut missing_false_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
    fn reject_margin_guarded_bool_case_pattern_shapes() {
        let mut empty_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(bool_true_guard());
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut pattern_type_mismatch = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(bool_true_guard());
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::Bool,
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn reject_margin_guarded_bool_case_missing_patterns() {
        let mut missing_true_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut missing_true_pattern.definitions.functions[0].body[0],
        );
        clauses.remove(0);
        clauses[0].guard = Some(bool_true_guard());
        assert_eq!(
            plan_module(missing_true_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingTruePattern,
                },
            }),
        );

        let mut missing_false_pattern = super::super::super::compile_bool_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut missing_false_pattern.definitions.functions[0].body[0],
        );
        clauses.pop();
        clauses.push(gleam_core::ast::Clause {
            location: dummy_span(),
            pattern: vec![Pattern::Discard {
                name: "_".into(),
                location: dummy_span(),
                type_: type_::bool(),
            }],
            alternative_patterns: Vec::new(),
            guard: Some(bool_true_guard()),
            then: crate::planner::expression::typed_int_expr(0),
        });
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
    fn reject_margin_ordered_bool_case_branch_preserves_non_fallback_errors() {
        assert_eq!(
            super::super::ordered_case_expr_for(
                vec![
                    super::super::OrderedCaseClause {
                        condition: BoolExpr::value(true),
                        branch: int(1).into(),
                        is_total: false,
                        reachable: true,
                    },
                    super::super::OrderedCaseClause {
                        condition: BoolExpr::value(true),
                        branch: bool_(true).into(),
                        is_total: true,
                        reachable: true,
                    },
                ],
                super::super::super::coverage::CaseBranchRequirement::True,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected: ValueType::Bool,
                        actual: ValueType::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bool_case_subject_type_mismatch() {
        let mut module = super::super::super::compile_bool_case_module();
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::String {
            location: dummy_span(),
            type_: type_::bool(),
            value: "not bool".into(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bool_case_return_type_unsupported() {
        let mut module = super::super::super::compile_bool_case_module();
        let (type_, _, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        *type_ = super::super::mismatched_generic_case_return_type();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Parameter(crate::plan::TypeParameterId(0)),
                        actual: ValueType::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_bool_case_guard_must_be_bool() {
        let mut module = super::super::super::compile_bool_case_module();
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

    fn bool_true_guard() -> ClauseGuard<std::sync::Arc<gleam_core::type_::Type>> {
        ClauseGuard::BinaryOperator {
            location: dummy_span(),
            operator: BinOp::Eq,
            operator_start: 0,
            left: Box::new(ClauseGuard::Constant(Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            })),
            right: Box::new(ClauseGuard::Constant(Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            })),
        }
    }
}
