use super::super::super::plan_expr_with_expected_source_stop_type;
use super::{CaseClause, OrderedCaseCandidateInput, OrderedCasePattern};
use crate::plan::{BitArrayExpr, BitArrayLocalId, BoolExpr, Expr, ExprKind, Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
use crate::planner::pattern::plan_bit_array_pattern;
use ecow::EcoString;
use gleam_core::ast::{BitArrayOption, BitArraySegment, Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_type(subject, ValueType::BitArray, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    let actual = InvalidExpressionType::from_value_type(subject.value_type());
    let ExprKind::BitArray(subject) = subject.into_kind() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::BitArray,
                actual,
            },
        });
    };
    let (subject_step, subject) = bind_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            ordered_clauses.push(super::plan_ordered_case_candidate(
                OrderedCaseCandidateInput {
                    case_type: type_.as_ref(),
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    guard: clause.guard.clone(),
                    reachable,
                    exhaustive_remainder,
                },
                context,
                |context| {
                    let pattern = plan_pattern(pattern, subject.clone(), context)?;
                    Ok(OrderedCasePattern {
                        match_condition: pattern.match_condition,
                        branch_bindings: pattern.branch_bindings,
                        total_branch_steps: Vec::new(),
                        is_total: pattern.is_total,
                    })
                },
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

pub(super) struct BitArrayCasePattern {
    match_condition: BoolExpr,
    branch_bindings: Vec<(EcoString, Expr)>,
    is_total: bool,
}

impl BitArrayCasePattern {
    pub(super) fn into_parts(self) -> (BoolExpr, Vec<(EcoString, Expr)>, bool) {
        (self.match_condition, self.branch_bindings, self.is_total)
    }
}

fn plan_pattern(
    pattern: Pattern<Arc<Type>>,
    subject: BitArrayExpr,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::BitArray,
                context,
            )?;
            Ok(BitArrayCasePattern {
                match_condition: BoolExpr::value(true),
                branch_bindings: vec![(name.clone(), Expr::bit_array(subject))],
                is_total: true,
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::BitArray,
                context,
            )?;
            Ok(BitArrayCasePattern {
                match_condition: BoolExpr::value(true),
                branch_bindings: Vec::new(),
                is_total: true,
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_pattern(*pattern, subject.clone(), context)?;
            pattern
                .branch_bindings
                .push((name, Expr::bit_array(subject)));
            Ok(pattern)
        }
        Pattern::BitArray { segments, .. } => plan_structural_pattern(segments, subject, context),
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &crate::plan::ValueShape::BitArray,
            context,
        )),
    }
}

pub(super) fn plan_structural_pattern(
    segments: Vec<BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>>,
    subject: BitArrayExpr,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayCasePattern, PlanError> {
    if let Some(branch_bindings) = plan_total_bits_pattern(&segments, subject.clone(), context)? {
        return Ok(BitArrayCasePattern {
            match_condition: BoolExpr::value(true),
            branch_bindings,
            is_total: true,
        });
    }
    let (pattern, is_total) = plan_bit_array_pattern(segments, context)?;
    Ok(BitArrayCasePattern {
        match_condition: BoolExpr::bit_array_matches(subject, pattern),
        branch_bindings: Vec::new(),
        is_total,
    })
}

fn plan_total_bits_pattern(
    segments: &[BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>],
    subject: BitArrayExpr,
    context: &PlanContext<'_>,
) -> Result<Option<Vec<(EcoString, Expr)>>, PlanError> {
    let [segment] = segments else {
        return Ok(None);
    };
    if segment.size().is_some()
        || !matches!(segment.options.as_slice(), [BitArrayOption::Bits { .. }])
    {
        return Ok(None);
    }

    let mut names = Vec::new();
    collect_total_bits_bindings_with_context(segment.value.as_ref(), &mut names, context)?;
    let subject = Expr::bit_array(subject);
    Ok(Some(
        names
            .into_iter()
            .map(|name| (name, subject.clone()))
            .collect(),
    ))
}

fn collect_total_bits_bindings_with_context(
    pattern: &Pattern<Arc<Type>>,
    names: &mut Vec<EcoString>,
    context: &PlanContext<'_>,
) -> Result<(), PlanError> {
    match pattern {
        pattern @ Pattern::Variable { name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::BitArray,
                context,
            )?;
            names.push(name.clone());
            Ok(())
        }
        pattern @ Pattern::Discard { .. } => crate::planner::pattern::validate_pattern(
            pattern,
            &crate::plan::ValueShape::BitArray,
            context,
        ),
        Pattern::Assign { name, pattern, .. } => {
            collect_total_bits_bindings_with_context(pattern, names, context)?;
            names.push(name.clone());
            Ok(())
        }
        pattern => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &crate::plan::ValueShape::BitArray,
            context,
        )),
    }
}

#[cfg(test)]
fn collect_total_bits_bindings(
    pattern: &Pattern<Arc<Type>>,
    names: &mut Vec<EcoString>,
) -> Result<(), PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module_name, &functions, &mut anonymous);
    collect_total_bits_bindings_with_context(pattern, names, &context)
}

fn bind_subject(subject: BitArrayExpr, context: &mut PlanContext<'_>) -> (Step, BitArrayExpr) {
    let local = context.define_internal_bit_array_local();
    let name = internal_subject_name(local);
    (
        Step::let_bit_array(local, name.clone(), subject),
        BitArrayExpr::local_get(local, name),
    )
}

fn internal_subject_name(local: BitArrayLocalId) -> EcoString {
    format!("<case:bit_array:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BitArrayExpr, BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment,
        BitArrayPatternSize, BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayReturn,
        BitArraySegment, BoolExpr, Endianness, Expr, IntExpr, IntLocalId, PatternBinding,
        Signedness, Step, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        function, int, int_return_block, int_return_expr, local_int, module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionShapeKind, InvalidExpressionType,
        InvalidTypedAstReason, PlanError,
    };
    use gleam_core::ast::{
        BitArrayOption, BitArraySegment as PatternBitArraySegment, BitArraySize, ClauseGuard,
        Pattern, TypedExpr,
    };
    use gleam_core::type_;
    use std::collections::HashMap;

    #[test]
    fn plan_bit_array_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    value as alias -> 1
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    int_return_block(
                        [
                            Step::let_bit_array(
                                BitArrayLocalId(1),
                                "value".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
                            Step::let_bit_array(
                                BitArrayLocalId(2),
                                "alias".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
                        ],
                        int_return_expr(int(1)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn total_bit_array_binding_rejects_a_mismatched_variable_annotation() {
        let module = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let mut names = Vec::new();

        assert_eq!(
            super::collect_total_bits_bindings_with_context(
                &Pattern::Variable {
                    location: dummy_span(),
                    name: "bits".into(),
                    type_: type_::int(),
                    origin: gleam_core::type_::error::VariableOrigin::generated(),
                },
                &mut names,
                &context,
            ),
            Err(crate::planner::pattern::pattern_type_mismatch(
                ValueType::BitArray,
                ValueType::Int,
            )),
        );
        assert!(names.is_empty());
    }

    #[test]
    fn reject_margin_bit_array_subject_invalid_and_mismatched_patterns() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
            &mut unsupported_case_type.definitions.functions[0].body[0],
        );
        *case_type = super::super::mismatched_generic_case_return_type();
        assert_eq!(
            plan_module(unsupported_case_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut invalid = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: gleam_core::type_::bit_array(),
        };
        assert_eq!(
            plan_module(invalid),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut mismatch.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        };
        assert_eq!(
            plan_module(mismatch),
            Err(super::super::pattern_type_mismatch(
                ValueType::BitArray,
                ValueType::Int,
            )),
        );

        for mismatched_binding in [
            Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_: type_::int(),
                origin: gleam_core::type_::error::VariableOrigin::generated(),
            },
            Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_::int(),
            },
        ] {
            let mut mismatch = crate::planner::support::compile(
                r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
            );
            let (_, _, clauses) = super::super::super::expect_case_statement_mut(
                &mut mismatch.definitions.functions[0].body[0],
            );
            clauses[0].pattern[0] = mismatched_binding;
            assert_eq!(
                plan_module(mismatch),
                Err(super::super::pattern_type_mismatch(
                    ValueType::BitArray,
                    ValueType::Int,
                )),
            );
        }

        let mut invalid_alias = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    value as alias -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_alias.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Assign {
            name: "alias".into(),
            location: dummy_span(),
            pattern: Box::new(Pattern::Invalid {
                location: dummy_span(),
                type_: gleam_core::type_::bit_array(),
            }),
        };
        assert_eq!(
            plan_module(invalid_alias),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );

        let mut subject_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut subject_mismatch.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_core::type_::bit_array(),
            value: "1".into(),
            int_value: 1.into(),
        };
        assert_eq!(
            plan_module(subject_mismatch),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::BitArray,
                InvalidExpressionType::Int,
            )),
        );

        let mut invalid_size = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_size.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::BitArraySize(BitArraySize::Int {
            location: dummy_span(),
            value: "8".into(),
            int_value: 8.into(),
        });
        assert_eq!(
            plan_module(invalid_size),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
                },
            }),
        );

        let mut invalid_guard = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ if True -> 1
    _ -> 0
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_guard.definitions.functions[0].body[0],
        );
        clauses[0].guard = Some(ClauseGuard::Invalid {
            location: dummy_span(),
            type_: type_::bool(),
        });
        assert_eq!(
            plan_module(invalid_guard),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );

        let mut invalid_branch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_branch.definitions.functions[0].body[0],
        );
        clauses[0].then = TypedExpr::Invalid {
            location: dummy_span(),
            type_: type_::int(),
            extra_information: None,
        };
        assert_eq!(
            plan_module(invalid_branch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );

        let mut branch_type_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    _ -> 1
  }
}
"#,
        );
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
            &mut branch_type_mismatch.definitions.functions[0].body[0],
        );
        *case_type = type_::bool();
        assert_eq!(
            plan_module(branch_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_total_bit_array_pattern_binding() {
        for invalid_binding in [
            Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_::int(),
            },
            Pattern::Assign {
                name: "alias".into(),
                location: dummy_span(),
                pattern: Box::new(Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
            },
        ] {
            let segments = vec![PatternBitArraySegment {
                location: dummy_span(),
                value: Box::new(invalid_binding),
                options: vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_: type_::bit_array(),
            }];
            let mut invalid = crate::planner::support::compile(
                r#"
pub fn main() {
  case <<1>> {
    <<_:bits>> -> 1
  }
}
"#,
            );
            let (_, _, clauses) = super::super::super::expect_case_statement_mut(
                &mut invalid.definitions.functions[0].body[0],
            );
            clauses[0].pattern[0] = Pattern::BitArray {
                location: dummy_span(),
                segments,
            };

            assert_eq!(
                plan_module(invalid),
                Err(super::super::pattern_type_mismatch(
                    ValueType::BitArray,
                    ValueType::Int,
                )),
            );
        }
    }

    #[test]
    fn plan_structural_bit_array_pattern_evaluates_subject_once() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1, 2>> {
    <<1, 2>> -> 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![
            BitArraySegment::Int {
                value: IntExpr::value(1.into()),
                bit_size: 8,
                endianness: Endianness::Big,
            },
            BitArraySegment::Int {
                value: IntExpr::value(2.into()),
                bit_size: 8,
                endianness: Endianness::Big,
            },
        ]);
        let pattern = BitArrayPattern::new(vec![
            BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Literal(1.into()),
                size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
                endianness: Endianness::Big,
                signedness: Signedness::Unsigned,
            },
            BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Literal(2.into()),
                size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
                endianness: Endianness::Big,
                signedness: Signedness::Unsigned,
            },
        ]);
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::bit_array_matches(
                            BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            pattern,
                        ),
                        int_return_expr(int(1)),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_total_bits_pattern_binds_subject_without_runtime_match_condition() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    <<_ as inner:bits>> as whole -> inner
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let expected = module(
            "main",
            function(
                "main",
                BitArrayReturn::block(
                    vec![Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    BitArrayReturn::block(
                        vec![
                            Step::let_bit_array(
                                BitArrayLocalId(1),
                                "inner".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
                            Step::let_bit_array(
                                BitArrayLocalId(2),
                                "whole".into(),
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                            ),
                        ],
                        BitArrayReturn::expr(BitArrayExpr::local_get(
                            BitArrayLocalId(1),
                            "inner".into(),
                        )),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bit_array_pattern_binding_is_visible_to_guard_and_branch() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    <<value>> if value > 0 -> value
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
            pattern: BitArrayPatternValue::Bind(PatternBinding::new(IntLocalId(0), "value".into())),
            size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
            endianness: Endianness::Big,
            signedness: Signedness::Unsigned,
        }]);
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::and(
                            BoolExpr::bit_array_matches(
                                BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
                                pattern,
                            ),
                            BoolExpr::gt_int(local_int(0, "value").into(), int(0).into()),
                        ),
                        int_return_expr(local_int(0, "value")),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_guarded_bit_array_alias_binds_guard_and_branch_scope() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case <<1>> {
    value as alias if value == alias -> 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:bit_array:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let bind_value = Step::let_bit_array(
            BitArrayLocalId(1),
            "value".into(),
            BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
        );
        let bind_alias = Step::let_bit_array(
            BitArrayLocalId(2),
            "alias".into(),
            BitArrayExpr::local_get(BitArrayLocalId(0), subject_name.into()),
        );
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![bind_value.clone(), bind_alias.clone()],
                BoolExpr::equal(
                    Expr::bit_array(BitArrayExpr::local_get(BitArrayLocalId(1), "value".into())),
                    Expr::bit_array(BitArrayExpr::local_get(BitArrayLocalId(2), "alias".into())),
                ),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_bit_array(
                        BitArrayLocalId(0),
                        subject_name.into(),
                        subject,
                    )],
                    crate::plan::IntReturn::bool_case(
                        condition,
                        int_return_block([bind_value, bind_alias], int_return_expr(int(1))),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_total_bits_binding_collection_preserves_inner_to_outer_order() {
        let variable = Pattern::Variable {
            location: dummy_span(),
            name: "value".into(),
            type_: type_::bit_array(),
            origin: gleam_core::type_::error::VariableOrigin::generated(),
        };
        let nested_alias = Pattern::Assign {
            location: dummy_span(),
            name: "outer".into(),
            pattern: Box::new(Pattern::Assign {
                location: dummy_span(),
                name: "inner".into(),
                pattern: Box::new(variable),
            }),
        };
        let mut names = Vec::new();

        assert_eq!(
            super::collect_total_bits_bindings(&nested_alias, &mut names),
            Ok(()),
        );
        assert_eq!(names, ["value", "inner", "outer"]);

        let mut discard_names = Vec::new();
        assert_eq!(
            super::collect_total_bits_bindings(
                &Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::bit_array(),
                },
                &mut discard_names,
            ),
            Ok(()),
        );
        assert_eq!(discard_names, Vec::<ecow::EcoString>::new());
    }

    #[test]
    fn reject_profile_bit_array_subject_expression_error() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case { <<1:native>> <<1>> } {
    _ -> 1
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
    fn reject_profile_bit_array_subject_clause_error_during_ordered_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case <<1>> {
    _ -> { <<1:native>> 1 }
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }
}
