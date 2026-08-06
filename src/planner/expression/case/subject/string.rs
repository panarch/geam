use super::super::super::plan_string_expr;
use super::super::coverage::{CaseBranchRequirement, require_branch};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, StringExpr, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{AssignName, Pattern, TypedExpr};
use gleam_core::strings::convert_string_escape_chars;
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_string_expr(subject, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    if clauses.iter().any(|clause| {
        clause.guard.is_some()
            || clause.has_alternative_patterns()
            || clause_has_string_prefix_pattern(clause)
    }) {
        let (subject_step, subject) = super::bind_string_case_subject(subject, context);
        let case = plan_ordered_string_case(return_shape, subject, clauses, context)?;
        return Ok(super::case_subject_block(subject_step, case));
    }
    let needs_subject_binding = clauses.iter().any(clause_has_string_bound_name);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_string_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut literal_clauses = Vec::new();
    let mut fallback = None;
    for clause in clauses {
        let pattern = plan_literal_string_case_pattern(clause.pattern, context)?;
        let bindings = pattern.branch_bindings(&subject);
        let branch = super::plan_case_branch(&return_shape, clause.then, bindings, context)?;

        match pattern {
            LiteralStringCasePattern::Literal { value, .. } => {
                if fallback.is_none()
                    && literal_clauses
                        .iter()
                        .all(|(existing, _)| existing != &value)
                {
                    literal_clauses.push((value, branch));
                }
            }
            LiteralStringCasePattern::Any { .. } => {
                if fallback.is_none() {
                    fallback = Some(branch);
                }
            }
        }
    }

    let fallback = require_branch(fallback, CaseBranchRequirement::Fallback)?;

    super::super::result::string_case_expr(subject, literal_clauses, fallback).map(|case| {
        match subject_step {
            Some(step) => super::case_subject_block(step, case),
            None => case,
        }
    })
}

fn plan_ordered_string_case(
    return_shape: ValueShape,
    subject: StringExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_string_case_pattern_with_context(pattern, context)?;
            let bindings = pattern.branch_bindings(&subject);
            let is_total = pattern.is_total() && clause.guard.is_none();
            let match_condition = pattern.match_condition(&subject);
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
enum StringCasePattern {
    Literal {
        value: EcoString,
        subject_bindings: Vec<EcoString>,
    },
    Prefix {
        prefix: EcoString,
        prefix_bindings: Vec<StringPrefixBinding>,
        subject_bindings: Vec<EcoString>,
    },
    Any {
        subject_bindings: Vec<EcoString>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringPrefixBinding {
    PrefixLiteral(EcoString),
    Suffix(EcoString),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LiteralStringCasePattern {
    Literal {
        value: EcoString,
        subject_bindings: Vec<EcoString>,
    },
    Any {
        subject_bindings: Vec<EcoString>,
    },
}

impl LiteralStringCasePattern {
    fn branch_bindings(&self, subject: &StringExpr) -> Vec<(EcoString, Expr)> {
        match self {
            LiteralStringCasePattern::Literal {
                subject_bindings, ..
            }
            | LiteralStringCasePattern::Any { subject_bindings } => subject_bindings
                .iter()
                .map(|name| (name.clone(), Expr::string(subject.clone())))
                .collect(),
        }
    }

    fn add_subject_binding(&mut self, name: EcoString) {
        match self {
            LiteralStringCasePattern::Literal {
                subject_bindings, ..
            }
            | LiteralStringCasePattern::Any { subject_bindings } => {
                subject_bindings.push(name);
            }
        }
    }
}

impl StringCasePattern {
    fn branch_bindings(&self, subject: &StringExpr) -> Vec<(EcoString, Expr)> {
        match self {
            StringCasePattern::Literal {
                subject_bindings, ..
            }
            | StringCasePattern::Any { subject_bindings } => subject_bindings
                .iter()
                .map(|name| (name.clone(), Expr::string(subject.clone())))
                .collect(),
            StringCasePattern::Prefix {
                prefix,
                prefix_bindings,
                subject_bindings,
            } => {
                let mut bindings: Vec<_> = prefix_bindings
                    .iter()
                    .map(|binding| match binding {
                        StringPrefixBinding::PrefixLiteral(name) => (
                            name.clone(),
                            Expr::string(StringExpr::value(prefix.clone())),
                        ),
                        StringPrefixBinding::Suffix(name) => (
                            name.clone(),
                            Expr::string(StringExpr::drop_prefix(subject.clone(), prefix.clone())),
                        ),
                    })
                    .collect();
                bindings.extend(
                    subject_bindings
                        .iter()
                        .map(|name| (name.clone(), Expr::string(subject.clone()))),
                );
                bindings
            }
        }
    }

    fn match_condition(&self, subject: &StringExpr) -> BoolExpr {
        match self {
            StringCasePattern::Literal { value, .. } => BoolExpr::equal(
                Expr::string(subject.clone()),
                Expr::string(StringExpr::value(value.clone())),
            ),
            StringCasePattern::Prefix { prefix, .. } => {
                BoolExpr::string_starts_with(subject.clone(), prefix.clone())
            }
            StringCasePattern::Any { .. } => BoolExpr::value(true),
        }
    }

    fn is_total(&self) -> bool {
        matches!(self, StringCasePattern::Any { .. })
    }

    fn add_subject_binding(&mut self, name: EcoString) {
        match self {
            StringCasePattern::Literal {
                subject_bindings, ..
            }
            | StringCasePattern::Prefix {
                subject_bindings, ..
            }
            | StringCasePattern::Any { subject_bindings } => {
                subject_bindings.push(name);
            }
        }
    }
}

fn plan_literal_string_case_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<LiteralStringCasePattern, PlanError> {
    match pattern {
        Pattern::String { value, .. } => Ok(LiteralStringCasePattern::Literal {
            value: convert_string_escape_chars(&value),
            subject_bindings: Vec::new(),
        }),
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::String, context)?;
            Ok(LiteralStringCasePattern::Any {
                subject_bindings: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::String, context)?;
            Ok(LiteralStringCasePattern::Any {
                subject_bindings: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_literal_string_case_pattern(*pattern, context)?;
            pattern.add_subject_binding(name);
            Ok(pattern)
        }
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &ValueShape::String,
            context,
        )),
    }
}

fn prefix_bindings(
    left_side_assignment: Option<(EcoString, gleam_core::ast::SrcSpan)>,
    right_side_assignment: AssignName,
) -> Vec<StringPrefixBinding> {
    let mut bindings = Vec::new();
    if let Some((name, _)) = left_side_assignment {
        bindings.push(StringPrefixBinding::PrefixLiteral(name));
    }
    if let AssignName::Variable(name) = right_side_assignment {
        bindings.push(StringPrefixBinding::Suffix(name));
    }

    bindings
}

fn plan_string_case_pattern_with_context(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<StringCasePattern, PlanError> {
    match pattern {
        Pattern::String { value, .. } => Ok(StringCasePattern::Literal {
            value: convert_string_escape_chars(&value),
            subject_bindings: Vec::new(),
        }),
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::String, context)?;
            Ok(StringCasePattern::Any {
                subject_bindings: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(pattern, &ValueShape::String, context)?;
            Ok(StringCasePattern::Any {
                subject_bindings: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_string_case_pattern_with_context(*pattern, context)?;
            pattern.add_subject_binding(name);
            Ok(pattern)
        }
        Pattern::StringPrefix {
            left_side_string,
            left_side_assignment,
            right_side_assignment,
            ..
        } => Ok(StringCasePattern::Prefix {
            prefix: convert_string_escape_chars(&left_side_string),
            prefix_bindings: prefix_bindings(left_side_assignment, right_side_assignment),
            subject_bindings: Vec::new(),
        }),
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &ValueShape::String,
            context,
        )),
    }
}

#[cfg(test)]
fn plan_string_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<StringCasePattern, PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module_name, &functions, &mut anonymous);
    plan_string_case_pattern_with_context(pattern, &context)
}

fn clause_has_string_bound_name(clause: &CaseClause) -> bool {
    string_pattern_has_bound_name(&clause.pattern)
}

fn clause_has_string_prefix_pattern(clause: &CaseClause) -> bool {
    std::iter::once(&clause.pattern)
        .chain(&clause.alternative_patterns)
        .any(string_pattern_has_prefix)
}

fn string_pattern_has_bound_name(pattern: &Pattern<Arc<Type>>) -> bool {
    match pattern {
        Pattern::Variable { type_, .. } if type_.is_string() => true,
        Pattern::Assign { .. } => true,
        _ => false,
    }
}

fn string_pattern_has_prefix(pattern: &Pattern<Arc<Type>>) -> bool {
    match pattern {
        Pattern::StringPrefix { .. } => true,
        Pattern::Assign { pattern, .. } => string_pattern_has_prefix(pattern),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{BoolExpr, Step, StringExpr, StringLocalId, StringReturn, ValueType};
    use crate::planner::dsl::{
        bool_, bool_return_expr, bool_return_string_case, function, int, int_return_expr,
        int_return_string_case, let_string_step, list, list_return_expr, list_return_string_case,
        local_string, module, nil, nil_return_expr, nil_return_string_case, return_list, string,
        string_return_block, string_return_expr, string_return_string_case,
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
                    return_list(list_return_string_case(
                        local_string(0, "value"),
                        [("one", list_return_expr(list([int(1)], ValueType::Int)))],
                        list_return_expr(list([int(0)], ValueType::Int)),
                    )),
                )
                .param_string(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_variable_pattern_binds_subject_once_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "geam" {
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
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("geam"))],
                    string_return_string_case(
                        local_string(0, "<case:string:0>"),
                        [],
                        string_return_block(
                            [let_string_step(
                                1,
                                "other",
                                local_string(0, "<case:string:0>"),
                            )],
                            string_return_expr(local_string(1, "other")),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_variable_alias_binds_inner_then_alias_in_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "geam" {
    other as alias -> other <> alias
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("geam"))],
                    string_return_string_case(
                        local_string(0, "<case:string:0>"),
                        [],
                        string_return_block(
                            [
                                let_string_step(1, "other", local_string(0, "<case:string:0>")),
                                let_string_step(2, "alias", local_string(0, "<case:string:0>")),
                            ],
                            string_return_expr(
                                local_string(1, "other").concatenate(local_string(2, "alias")),
                            ),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_literal_alias_binds_subject_once_for_alias_value() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "geam" {
    "geam" as alias -> alias
    _ -> ""
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("geam"))],
                    string_return_string_case(
                        local_string(0, "<case:string:0>"),
                        [(
                            "geam",
                            string_return_block(
                                [let_string_step(
                                    1,
                                    "alias",
                                    local_string(0, "<case:string:0>"),
                                )],
                                string_return_expr(local_string(1, "alias")),
                            ),
                        )],
                        string_return_expr(string("")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_guard_binds_subject_once_and_falls_through() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "geam" {
    other if other == "geam" -> other
    _ -> ""
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_string_step(1, "other", local_string(0, "<case:string:0>"));
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_other.clone()],
                BoolExpr::equal(local_string(1, "other").into(), string("geam").into()),
            ),
        );
        let guarded_branch =
            string_return_block([bind_other], string_return_expr(local_string(1, "other")));
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("geam"))],
                    StringReturn::bool_case(
                        condition,
                        guarded_branch,
                        string_return_expr(string("")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_guarded_alias_binds_guard_and_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "geam" {
    other as alias if alias == "geam" -> other <> alias
    _ -> ""
  }
}
"#,
        ))
        .expect("source should plan");
        let bind_other = let_string_step(1, "other", local_string(0, "<case:string:0>"));
        let bind_alias = let_string_step(2, "alias", local_string(0, "<case:string:0>"));
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_other.clone(), bind_alias.clone()],
                BoolExpr::equal(local_string(2, "alias").into(), string("geam").into()),
            ),
        );
        let guarded_branch = string_return_block(
            [bind_other, bind_alias],
            string_return_expr(local_string(1, "other").concatenate(local_string(2, "alias"))),
        );
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("geam"))],
                    StringReturn::bool_case(
                        condition,
                        guarded_branch,
                        string_return_expr(string("")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_prefix_binds_suffix_after_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "Hello, Geam" {
    "Hello, " <> name -> name
    _ -> "Unknown"
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = local_string(0, "<case:string:0>");
        let suffix = StringExpr::drop_prefix(subject.into(), "Hello, ".into());
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("Hello, Geam"))],
                    StringReturn::bool_case(
                        BoolExpr::string_starts_with(
                            local_string(0, "<case:string:0>").into(),
                            "Hello, ".into(),
                        ),
                        string_return_block(
                            [Step::let_string(StringLocalId(1), "name".into(), suffix)],
                            string_return_expr(local_string(1, "name")),
                        ),
                        string_return_expr(string("Unknown")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_string_case_prefix_whole_alias_binds_suffix_then_subject_alias() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "Hello, Geam" {
    "Hello, " <> name as whole -> name <> whole
    _ -> "Unknown"
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = local_string(0, "<case:string:0>");
        let bind_name = Step::let_string(
            StringLocalId(1),
            "name".into(),
            StringExpr::drop_prefix(subject.into(), "Hello, ".into()),
        );
        let bind_whole = let_string_step(2, "whole", local_string(0, "<case:string:0>"));
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("Hello, Geam"))],
                    StringReturn::bool_case(
                        BoolExpr::string_starts_with(
                            local_string(0, "<case:string:0>").into(),
                            "Hello, ".into(),
                        ),
                        string_return_block(
                            [bind_name, bind_whole],
                            string_return_expr(
                                local_string(1, "name").concatenate(local_string(2, "whole")),
                            ),
                        ),
                        string_return_expr(string("Unknown")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn string_case_pattern_assigns_literal_subject_alias() {
        assert_eq!(
            super::plan_string_case_pattern(Pattern::Assign {
                name: "alias".into(),
                location: dummy_span(),
                pattern: Box::new(Pattern::String {
                    location: dummy_span(),
                    value: "geam".into(),
                }),
            }),
            Ok(super::StringCasePattern::Literal {
                value: "geam".into(),
                subject_bindings: vec!["alias".into()],
            }),
        );
    }

    #[test]
    fn reject_margin_string_case_pattern_assign_invalid_inner_pattern() {
        assert_eq!(
            super::plan_string_case_pattern(Pattern::Assign {
                name: "alias".into(),
                location: dummy_span(),
                pattern: Box::new(Pattern::Invalid {
                    location: dummy_span(),
                    type_: type_::string(),
                }),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    #[test]
    fn plan_string_case_prefix_guard_wraps_suffix_binding_after_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case "Hello, Geam" {
    "Hello, " as prefix <> name if name == "Geam" -> prefix <> name
    _ -> "Unknown"
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = local_string(0, "<case:string:0>");
        let bind_prefix = let_string_step(1, "prefix", string("Hello, "));
        let bind_name = Step::let_string(
            StringLocalId(2),
            "name".into(),
            StringExpr::drop_prefix(subject.into(), "Hello, ".into()),
        );
        let condition = BoolExpr::and(
            BoolExpr::string_starts_with(
                local_string(0, "<case:string:0>").into(),
                "Hello, ".into(),
            ),
            BoolExpr::block(
                vec![bind_prefix.clone(), bind_name.clone()],
                BoolExpr::equal(local_string(2, "name").into(), string("Geam").into()),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_string_step(0, "<case:string:0>", string("Hello, Geam"))],
                    StringReturn::bool_case(
                        condition,
                        string_return_block(
                            [bind_prefix, bind_name],
                            string_return_expr(
                                local_string(1, "prefix").concatenate(local_string(2, "name")),
                            ),
                        ),
                        string_return_expr(string("Unknown")),
                    ),
                ),
            ),
            [],
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
    fn reject_profile_string_case_unreachable_duplicate_clause_body() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case "one" {
    "one" -> 1
    "one" -> { <<1:native>> 2 }
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
    fn reject_margin_string_case_pattern_shapes() {
        let mut variable_type_mismatch = compile_string_case_module();
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
                ValueType::String,
                ValueType::Bool,
            )),
        );

        let mut discard_type_mismatch = compile_string_case_module();
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
                ValueType::String,
                ValueType::Bool,
            )),
        );

        let mut invalid_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::string(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut pattern_type_mismatch = compile_string_case_module();
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
                ValueType::String,
                ValueType::Int,
            )),
        );

        let mut assign_invalid_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut assign_type_mismatch = compile_string_case_module();
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
                ValueType::String,
                ValueType::Int,
            )),
        );

        let mut empty_pattern = compile_string_case_module();
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

        let mut case_type_mismatch = compile_string_case_module();
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

        let mut invalid_compiled_clause = compile_string_case_module();
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

        let mut variable_type_mismatch = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut variable_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern[0] = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: type_::bool(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(variable_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::String,
                ValueType::Bool,
            )),
        );

        let mut discard_type_mismatch = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut discard_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern[0] = Pattern::Discard {
            name: "_".into(),
            location: dummy_span(),
            type_: type_::bool(),
        };
        assert_eq!(
            plan_module(discard_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::String,
                ValueType::Bool,
            )),
        );

        let mut invalid_pattern = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: type_::string(),
        };
        assert_eq!(
            plan_module(invalid_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_guarded_string_case_pattern_shapes() {
        let mut empty_pattern = compile_string_case_module();
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

        let mut pattern_type_mismatch = compile_string_case_module();
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        }));
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::String,
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn reject_margin_string_case_guard_must_be_bool() {
        let mut module = compile_string_case_module();
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
    fn reject_margin_string_case_subject_type_mismatch() {
        let mut module = compile_string_case_module();
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
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
    fn reject_margin_string_case_return_annotation_mismatch() {
        let mut module = compile_string_case_module();
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
