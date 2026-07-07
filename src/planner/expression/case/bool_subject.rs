use super::{
    case_return_type, invalid_case_shape, single_case_pattern, unsupported_case,
    validate_clause_shape,
};
use crate::plan::{BoolCaseBranches, BoolExpr, Expr, ExprKind};
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
    let subject = super::super::plan_bool_expr(subject, context)?;
    let return_type = case_return_type(type_.as_ref())?;
    for clause in &clauses {
        validate_clause_shape(clause)?;
    }
    let needs_subject_binding = clauses.iter().any(clause_has_bool_variable_pattern);
    let (subject_step, subject) = if needs_subject_binding {
        let (step, subject) = super::bind_bool_case_subject(subject, context);
        (Some(step), subject)
    } else {
        (None, subject)
    };
    let mut true_branch = None;
    let mut false_branch = None;
    for clause in clauses {
        let pattern = single_case_pattern(clause.pattern)?;
        let pattern = plan_bool_case_pattern(pattern)?;
        let binding = pattern
            .bound_name()
            .cloned()
            .map(|name| (name, Expr::bool(subject.clone())));
        let branch =
            super::plan_case_branch(type_.as_ref(), &return_type, clause.then, binding, context)?;

        match pattern {
            BoolCasePattern::True => set_case_branch(&mut true_branch, branch),
            BoolCasePattern::False => set_case_branch(&mut false_branch, branch),
            BoolCasePattern::Any { .. } => {
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

    bool_case_expr(subject, true_, false_).map(|case| match subject_step {
        Some(step) => super::case_subject_block(step, case),
        None => case,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoolCasePattern {
    True,
    False,
    Any { bound_name: Option<EcoString> },
}

impl BoolCasePattern {
    fn bound_name(&self) -> Option<&EcoString> {
        match self {
            BoolCasePattern::Any { bound_name } => bound_name.as_ref(),
            BoolCasePattern::True | BoolCasePattern::False => None,
        }
    }
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
        Pattern::Variable { name, type_, .. } if type_.is_bool() => Ok(BoolCasePattern::Any {
            bound_name: Some(name),
        }),
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if type_.is_bool() => {
            Ok(BoolCasePattern::Any { bound_name: None })
        }
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

fn clause_has_bool_variable_pattern(clause: &TypedClause) -> bool {
    clause.pattern.iter().any(|pattern| {
        matches!(
            pattern,
            Pattern::Variable { type_, .. } if type_.is_bool()
        )
    })
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
    let branches = match (true_.into_kind(), false_.into_kind()) {
        (ExprKind::Int(true_), ExprKind::Int(false_)) => BoolCaseBranches::Int { true_, false_ },
        (ExprKind::String(true_), ExprKind::String(false_)) => {
            BoolCaseBranches::String { true_, false_ }
        }
        (ExprKind::Float(true_), ExprKind::Float(false_)) => {
            BoolCaseBranches::Float { true_, false_ }
        }
        (ExprKind::Bool(true_), ExprKind::Bool(false_)) => BoolCaseBranches::Bool { true_, false_ },
        (ExprKind::Nil(true_), ExprKind::Nil(false_)) => BoolCaseBranches::Nil { true_, false_ },
        (ExprKind::Tuple(true_), ExprKind::Tuple(false_)) => {
            BoolCaseBranches::Tuple { true_, false_ }
        }
        (ExprKind::List(true_), ExprKind::List(false_)) => BoolCaseBranches::List { true_, false_ },
        (ExprKind::Function(true_), ExprKind::Function(false_)) => {
            bool_function_case_branches(true_, false_)?
        }
        _ => {
            return Err(invalid_case_shape(
                InvalidCaseShapeReason::BranchReturnTypeMismatch,
            ));
        }
    };

    Ok(Expr::bool_case(subject, branches))
}

fn bool_function_case_branches(
    true_: crate::plan::FunctionExpr,
    false_: crate::plan::FunctionExpr,
) -> Result<BoolCaseBranches, PlanError> {
    match (true_.into_kind(), false_.into_kind()) {
        (crate::plan::FunctionExprKind::Int(true_), crate::plan::FunctionExprKind::Int(false_)) => {
            Ok(BoolCaseBranches::IntFunction { true_, false_ })
        }
        (
            crate::plan::FunctionExprKind::String(true_),
            crate::plan::FunctionExprKind::String(false_),
        ) => Ok(BoolCaseBranches::StringFunction { true_, false_ }),
        (
            crate::plan::FunctionExprKind::Float(true_),
            crate::plan::FunctionExprKind::Float(false_),
        ) => Ok(BoolCaseBranches::FloatFunction { true_, false_ }),
        (
            crate::plan::FunctionExprKind::Bool(true_),
            crate::plan::FunctionExprKind::Bool(false_),
        ) => Ok(BoolCaseBranches::BoolFunction { true_, false_ }),
        (crate::plan::FunctionExprKind::Nil(true_), crate::plan::FunctionExprKind::Nil(false_)) => {
            Ok(BoolCaseBranches::NilFunction { true_, false_ })
        }
        (
            crate::plan::FunctionExprKind::Tuple(true_),
            crate::plan::FunctionExprKind::Tuple(false_),
        ) => Ok(BoolCaseBranches::TupleFunction { true_, false_ }),
        (
            crate::plan::FunctionExprKind::List(true_),
            crate::plan::FunctionExprKind::List(false_),
        ) => Ok(BoolCaseBranches::ListFunction { true_, false_ }),
        (
            crate::plan::FunctionExprKind::Function(true_),
            crate::plan::FunctionExprKind::Function(false_),
        ) => Ok(BoolCaseBranches::FunctionFunction { true_, false_ }),
        _ => Err(invalid_case_shape(
            InvalidCaseShapeReason::BranchReturnTypeMismatch,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolFunctionId, Expr, FloatExpr, FloatFunctionId, FunctionExpr,
        FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId, IntLocalId,
        ListFunctionId, LocalId, NilFunctionId, RuntimeFunctionId, StringFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, bool_return_block, bool_return_bool_case, bool_return_expr, call_bool, function,
        function_ref, int, int_return_bool_case, int_return_expr, let_bool_step, list,
        list_return_bool_case, list_return_expr, local_bool, module, nil, nil_return_bool_case,
        nil_return_expr, return_list, string, string_return_bool_case, string_return_expr,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedCaseReason, UnsupportedExpressionKind,
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
                    return_list(
                        ValueType::Int,
                        list_return_bool_case(
                            local_bool(0, "value"),
                            list_return_expr(list([int(1)], ValueType::Int)),
                            list_return_expr(list([int(0)], ValueType::Int)),
                        ),
                    ),
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
            function(
                "main",
                int_return_bool_case(
                    call_bool(0, []),
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
    fn reject_margin_bool_case_function_branch_type_mismatch_direct() {
        assert_eq!(
            (super::bool_function_case_branches(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
            ))
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
        assert_eq!(
            (super::bool_function_case_branches(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
            ))
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn plan_bool_case_function_branch_return_families_direct() {
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::float(FloatExpr::value(1.0)),
                Expr::float(FloatExpr::value(0.0)),
            ),
            Ok(Expr::bool_case(
                BoolExpr::value(true),
                super::BoolCaseBranches::Float {
                    true_: FloatExpr::value(1.0),
                    false_: FloatExpr::value(0.0),
                },
            )),
        );

        let string_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                [LocalId::String(crate::plan::StringLocalId(0))],
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::String(StringFunctionId(1)),
                [LocalId::String(crate::plan::StringLocalId(0))],
            )),
        )
        .expect("string function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), string_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::String],
                ValueType::String,
            ))),
        );

        let float_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Float(FloatFunctionId(0)),
                [LocalId::Float(crate::plan::FloatLocalId(0))],
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Float(FloatFunctionId(1)),
                [LocalId::Float(crate::plan::FloatLocalId(0))],
            )),
        )
        .expect("float function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), float_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Float,
            ))),
        );

        let bool_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                [LocalId::Bool(crate::plan::BoolLocalId(0))],
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(1)),
                [LocalId::Bool(crate::plan::BoolLocalId(0))],
            )),
        )
        .expect("bool function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), bool_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Bool],
                ValueType::Bool,
            ))),
        );

        let nil_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                [LocalId::Nil(crate::plan::NilLocalId(0))],
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(1)),
                [LocalId::Nil(crate::plan::NilLocalId(0))],
            )),
        )
        .expect("nil function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), nil_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Nil],
                ValueType::Nil,
            ))),
        );

        let list_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::List {
                    id: ListFunctionId(0),
                    return_type: Box::new(ValueType::Int),
                },
                [LocalId::Int(crate::plan::IntLocalId(0))],
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::List {
                    id: ListFunctionId(1),
                    return_type: Box::new(ValueType::Int),
                },
                [LocalId::Int(crate::plan::IntLocalId(0))],
            )),
        )
        .expect("list function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), list_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::List(Box::new(ValueType::Int)),
            ))),
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_branches = super::bool_function_case_branches(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: returned_function_type.clone(),
                },
                Vec::<LocalId>::new(),
            )),
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    return_type: returned_function_type.clone(),
                },
                Vec::<LocalId>::new(),
            )),
        )
        .expect("function-returning-function branches");
        assert_eq!(
            Expr::bool_case(BoolExpr::value(true), function_branches).value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type)),
            ))),
        );
    }
    #[test]
    fn reject_profile_bool_case_patterns() {
        let cases = [
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
            (
                r#"
pub fn main() {
  case True {
    _ as alias -> 1
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
    fn reject_profile_bool_case_subject_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case echo True {
    True -> 1
    False -> 0
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
    fn reject_margin_bool_case_subject_type_mismatch() {
        let mut module = super::super::compile_bool_case_module();
        let (_, subjects, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
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
        let mut module = super::super::compile_bool_case_module();
        let (type_, _, _) =
            super::super::expect_case_statement_mut(&mut module.definitions.functions[0].body[0]);
        *type_ = type_::bit_array();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
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
    fn reject_margin_bool_case_function_expr_type_mismatch_direct() {
        assert_eq!(
            super::bool_case_expr(
                BoolExpr::value(true),
                Expr::from(function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
                Expr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                )),
            ),
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
