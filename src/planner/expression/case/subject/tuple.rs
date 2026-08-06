use super::super::super::tuple_index_expr;
use super::super::super::{
    conversion::expect_expression, plan_expr_with_expected_source_stop_shape,
};
use super::{CaseClause, OrderedCaseCandidateInput, OrderedCasePattern};
use crate::plan::{
    BoolExpr, CustomBindingPattern, CustomExpr, Expr, FloatExpr, IntExpr, Step, StringExpr,
    TupleExpr, TupleLocalId, ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use crate::planner::pattern::plan_custom_subject_pattern;
use ecow::EcoString;
use gleam_core::ast::{AssignName, Pattern, SrcSpan, TypedExpr};
use gleam_core::strings::convert_string_escape_chars;
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    subject_type: Vec<ValueType>,
    subject_shape: ValueShape,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject_value_type = ValueType::Tuple(subject_type.clone());
    let subject = plan_expr_with_expected_source_stop_shape(subject, subject_shape, context)?;
    let return_shape = context.value_shape(type_.as_ref());

    let subject: TupleExpr = expect_expression(subject)?;
    let (subject_step, subject) = bind_tuple_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            ordered_clauses.push(super::plan_ordered_case_candidate(
                OrderedCaseCandidateInput {
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    guard: clause.guard.clone(),
                    reachable,
                    exhaustive_remainder,
                },
                context,
                |context| {
                    let pattern = plan_tuple_case_pattern_with_context(
                        pattern,
                        subject.clone(),
                        subject_value_type.clone(),
                        context,
                    )?;
                    let is_total = pattern.is_total();
                    Ok(OrderedCasePattern {
                        match_condition: pattern.match_condition(),
                        branch_bindings: pattern.branch_bindings,
                        total_branch_steps: pattern.total_branch_steps,
                        is_total,
                    })
                },
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq)]
struct TupleCasePattern {
    match_condition: Option<BoolExpr>,
    branch_bindings: Vec<(EcoString, Expr)>,
    total_branch_steps: Vec<Step>,
    is_total: bool,
}

impl TupleCasePattern {
    fn any() -> Self {
        Self {
            match_condition: None,
            branch_bindings: Vec::new(),
            total_branch_steps: Vec::new(),
            is_total: true,
        }
    }

    fn literal(value: Expr, literal: Expr) -> Self {
        Self {
            match_condition: Some(BoolExpr::equal(value, literal)),
            branch_bindings: Vec::new(),
            total_branch_steps: Vec::new(),
            is_total: false,
        }
    }

    fn string_prefix(
        value: Expr,
        prefix: EcoString,
        left_side_assignment: Option<(EcoString, SrcSpan)>,
        right_side_assignment: AssignName,
    ) -> Result<Self, PlanError> {
        let value: StringExpr = expect_expression(value)?;
        let mut pattern = Self {
            match_condition: Some(BoolExpr::string_starts_with(value.clone(), prefix.clone())),
            branch_bindings: Vec::new(),
            total_branch_steps: Vec::new(),
            is_total: false,
        };
        if let Some((name, _)) = left_side_assignment {
            pattern
                .branch_bindings
                .push((name, Expr::string(StringExpr::value(prefix.clone()))));
        }
        if let AssignName::Variable(name) = right_side_assignment {
            pattern
                .branch_bindings
                .push((name, Expr::string(StringExpr::drop_prefix(value, prefix))));
        }

        Ok(pattern)
    }

    fn with_binding(mut self, name: EcoString, value: Expr) -> Self {
        self.branch_bindings.push((name, value));
        self
    }

    fn is_total(&self) -> bool {
        self.is_total
    }

    fn match_condition(&self) -> BoolExpr {
        self.match_condition
            .clone()
            .unwrap_or_else(|| BoolExpr::value(true))
    }
}

impl TupleCasePattern {
    fn from_list_pattern(pattern: super::list::ListCasePattern) -> Self {
        let (match_condition, branch_bindings, total_branch_steps, is_total) = pattern.into_parts();
        Self {
            match_condition,
            branch_bindings,
            total_branch_steps,
            is_total,
        }
    }

    fn from_bit_array_pattern(pattern: super::bit_array::BitArrayCasePattern) -> Self {
        let (match_condition, branch_bindings, is_total) = pattern.into_parts();
        Self {
            match_condition: Some(match_condition),
            branch_bindings,
            total_branch_steps: Vec::new(),
            is_total,
        }
    }
}

fn plan_tuple_case_pattern_with_context(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<TupleCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable {
            ref name,
            ref type_,
            ..
        } if matches_type(type_.as_ref(), &subject_type, context) => {
            Ok(TupleCasePattern::any().with_binding(name.clone(), value))
        }
        ref pattern @ Pattern::Variable { .. } => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), &subject_type, context) => {
            Ok(TupleCasePattern::any())
        }
        ref pattern @ Pattern::Discard { .. } => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_tuple_case_pattern_with_context(
                *pattern,
                value.clone(),
                subject_type,
                context,
            )?;
            Ok(pattern.with_binding(name, value))
        }
        Pattern::Tuple { location, elements } => {
            plan_tuple_structural_case_pattern(location, elements, value, subject_type, context)
        }
        Pattern::Int { int_value, .. } if subject_type == ValueType::Int => Ok(
            TupleCasePattern::literal(value, Expr::int(IntExpr::value(int_value))),
        ),
        Pattern::Float { float_value, .. } if subject_type == ValueType::Float => Ok(
            TupleCasePattern::literal(value, Expr::float(FloatExpr::value(float_value.value()))),
        ),
        Pattern::String { value: literal, .. } if subject_type == ValueType::String => {
            Ok(TupleCasePattern::literal(
                value,
                Expr::string(StringExpr::value(convert_string_escape_chars(&literal))),
            ))
        }
        ref pattern @ Pattern::Constructor {
            ref name,
            ref arguments,
            ref spread,
            ref type_,
            ..
        } if arguments.is_empty() && spread.is_none() && type_.is_bool() => {
            match (name.as_str(), &subject_type) {
                ("True", ValueType::Bool) => Ok(TupleCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(true)),
                )),
                ("False", ValueType::Bool) => Ok(TupleCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(false)),
                )),
                _ => Err(crate::planner::pattern::unexpected_pattern(
                    pattern,
                    &ValueShape::from_value_type(subject_type),
                    context,
                )),
            }
        }
        ref pattern @ Pattern::Constructor {
            ref name,
            ref arguments,
            ref spread,
            ref type_,
            ..
        } if name == "Nil" && arguments.is_empty() && spread.is_none() && type_.is_nil() => {
            if subject_type == ValueType::Nil {
                Ok(TupleCasePattern::any())
            } else {
                Err(crate::planner::pattern::unexpected_pattern(
                    pattern,
                    &ValueShape::from_value_type(subject_type),
                    context,
                ))
            }
        }
        ref pattern @ Pattern::Constructor {
            type_: ref pattern_type,
            ..
        } if matches!(&subject_type, ValueType::Custom(_))
            && matches_type(pattern_type.as_ref(), &subject_type, context) =>
        {
            let value: CustomExpr = expect_expression(value)?;
            let pattern =
                plan_custom_subject_pattern(pattern.clone(), value.shape().clone(), context)?;
            let total_branch_steps = pattern
                .custom_binding
                .clone()
                .map(|binding| {
                    binding
                        .clone()
                        .into_intrinsic_binding()
                        .unwrap_or_else(|| binding.into_exhaustive_remainder_binding())
                })
                .map(|binding| total_custom_binding_steps(value.clone(), binding, context))
                .unwrap_or_default();
            Ok(TupleCasePattern {
                match_condition: Some(BoolExpr::custom_matches(value, pattern.pattern)),
                branch_bindings: Vec::new(),
                total_branch_steps,
                is_total: pattern.is_total,
            })
        }
        Pattern::List { .. } if matches!(subject_type, ValueType::List(_)) => {
            let pattern = super::list::plan_list_case_pattern_with_context(
                pattern,
                value,
                subject_type,
                context,
            )?;
            Ok(TupleCasePattern::from_list_pattern(pattern))
        }
        Pattern::StringPrefix {
            left_side_string,
            left_side_assignment,
            right_side_assignment,
            ..
        } if subject_type == ValueType::String => TupleCasePattern::string_prefix(
            value,
            convert_string_escape_chars(&left_side_string),
            left_side_assignment,
            right_side_assignment,
        ),
        Pattern::BitArray { segments, .. } if subject_type == ValueType::BitArray => {
            let value = expect_expression(value)?;
            super::bit_array::plan_structural_pattern(segments, value, context)
                .map(TupleCasePattern::from_bit_array_pattern)
        }
        ref pattern @ (Pattern::BitArraySize(_)
        | Pattern::BitArray { .. }
        | Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Constructor { .. }
        | Pattern::List { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
    }
}

fn plan_tuple_structural_case_pattern(
    location: SrcSpan,
    elements: Vec<Pattern<Arc<Type>>>,
    value: Expr,
    subject_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<TupleCasePattern, PlanError> {
    let validated = crate::planner::pattern::validate_tuple_pattern(
        &Pattern::Tuple {
            location,
            elements: elements.clone(),
        },
        &ValueShape::from_value_type(subject_type),
        context,
    )?;
    let element_types = validated
        .element_shapes
        .iter()
        .map(ValueShape::value_type)
        .collect::<Vec<_>>();
    let tuple: TupleExpr = expect_expression(value)?;

    let mut patterns = Vec::with_capacity(elements.len());
    for (index, (pattern, type_)) in elements.into_iter().zip(element_types).enumerate() {
        let value = tuple_index_expr(tuple.clone(), index, type_.clone())?;
        patterns.push(plan_tuple_case_pattern_with_context(
            pattern, value, type_, context,
        )?);
    }

    Ok(combine_tuple_case_patterns(patterns))
}

#[cfg(test)]
fn plan_tuple_case_pattern(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<TupleCasePattern, PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
    plan_tuple_case_pattern_with_context(pattern, value, subject_type, &mut context)
}

fn combine_tuple_case_patterns(patterns: Vec<TupleCasePattern>) -> TupleCasePattern {
    let mut combined = TupleCasePattern::any();
    for pattern in patterns {
        combined.match_condition = match (combined.match_condition, pattern.match_condition) {
            (Some(left), Some(right)) => Some(BoolExpr::and(left, right)),
            (Some(condition), None) | (None, Some(condition)) => Some(condition),
            (None, None) => None,
        };
        combined.branch_bindings.extend(pattern.branch_bindings);
        combined
            .total_branch_steps
            .extend(pattern.total_branch_steps);
        combined.is_total &= pattern.is_total;
    }

    combined
}

fn total_custom_binding_steps(
    value: CustomExpr,
    binding: CustomBindingPattern,
    context: &mut PlanContext<'_>,
) -> Vec<Step> {
    let local = context.define_internal_custom_local();
    let name = format!("<case:tuple:custom:{}>", local.0).into();
    vec![
        Step::let_custom(local, name, value),
        Step::bind_custom_fields(local, binding),
    ]
}

fn matches_type(type_: &Type, subject_type: &ValueType, context: &mut PlanContext<'_>) -> bool {
    context.value_shape(type_).value_type() == *subject_type
}

fn bind_tuple_case_subject(subject: TupleExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_tuple_local();
    let name = internal_tuple_case_subject_name(local);
    let type_ = subject.type_().to_vec();
    let shape = subject.shape().to_vec().into_boxed_slice();
    (
        Step::let_tuple(local, name.clone(), subject),
        Expr::tuple(TupleExpr::local_get(local, name, type_).with_shape(shape)),
    )
}

fn internal_tuple_case_subject_name(local: TupleLocalId) -> EcoString {
    format!("<case:tuple:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertPattern, BitArrayExpr, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArraySegment, BoolExpr,
        CustomBindingPattern, CustomConstructor, CustomConstructorField, CustomExpr, CustomLocalId,
        CustomPattern, CustomType, CustomTypeName, Endianness, Expr, IntExpr, IntLocalId, ListExpr,
        Signedness, Step, StringExpr, StringLocalId, TotalBindingPattern, TupleExpr, TupleLocalId,
        ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bool_, float, function, int, int_return_block, int_return_expr, let_int_step,
        let_tuple_step, list, local_int, local_string, local_tuple, module, nil, string,
        string_return_block, string_return_expr, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::ast::{AssignName, Pattern};
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::error::VariableOrigin;
    use std::collections::HashMap;

    #[test]
    fn tuple_case_subject_local_preserves_nested_custom_refinement() {
        let module_name = ecow::EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let value = CustomExpr::try_constructor(
            CustomConstructor::new(type_.clone(), "First".into(), 0, Vec::new()),
            Vec::new(),
        )
        .expect("test custom construction should be valid");
        let custom_shape = ValueShape::Custom(value.shape().clone());
        let subject = TupleExpr::value(vec![Expr::custom(value)], vec![ValueType::Custom(type_)]);

        let (_, local) = super::bind_tuple_case_subject(subject, &mut context);

        assert_eq!(
            local.value_shape(),
            &ValueShape::Tuple(vec![custom_shape].into_boxed_slice()),
        );
    }

    #[test]
    fn plan_tuple_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    value as alias -> value.0 + alias.1
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let value = local_tuple(1, "value", tuple_type.clone());
        let alias = local_tuple(2, "alias", tuple_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    int_return_block(
                        [
                            let_tuple_step(
                                1,
                                "value",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
                            ),
                            let_tuple_step(
                                2,
                                "alias",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
                            ),
                        ],
                        int_return_expr(value.index_int(0).add_int(alias.index_int(1))),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    value if value.0 > 10 -> 0
    value as alias if alias.1 == 2 -> value.0 + alias.1
    _ -> 999
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let second_value = local_tuple(2, "value", tuple_type.clone());
        let second_alias = local_tuple(3, "alias", tuple_type.clone());
        let first_binding = let_tuple_step(
            1,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let second_value_binding = let_tuple_step(
            2,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let second_alias_binding = let_tuple_step(
            3,
            "alias",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()),
        );
        let first_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![first_binding.clone()],
                BoolExpr::gt_int(
                    local_tuple(1, "value", tuple_type.clone())
                        .index_int(0)
                        .into(),
                    int(10).into(),
                ),
            ),
        );
        let second_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![second_value_binding.clone(), second_alias_binding.clone()],
                BoolExpr::equal(
                    Expr::from(local_tuple(3, "alias", tuple_type.clone()).index_int(1)),
                    Expr::from(int(2)),
                ),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(int(0))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_value_binding, second_alias_binding],
                                int_return_expr(
                                    second_value.index_int(0).add_int(second_alias.index_int(1)),
                                ),
                            ),
                            int_return_expr(int(999)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_alternatives_bind_each_pattern_scope_independently() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(11, 37) {
    #(value, 0) | #(11, value) -> value
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let first_binding = let_int_step(
            0,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
        );
        let second_binding = let_int_step(
            1,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let first_condition = BoolExpr::equal(
            Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1)),
            Expr::from(int(0)),
        );
        let second_condition = BoolExpr::equal(
            Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type).index_int(0)),
            Expr::from(int(11)),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple([int(11), int(37)]),
                    )],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(local_int(0, "value"))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_binding],
                                int_return_expr(local_int(1, "value")),
                            ),
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
    fn plan_tuple_subject_alternative_guard_wraps_each_pattern_binding() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(11, 37) {
    #(left, 0) | #(11, left) if left > 20 -> left
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let first_binding = let_int_step(
            0,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
        );
        let second_binding = let_int_step(
            1,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let first_condition = BoolExpr::and(
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1)),
                Expr::from(int(0)),
            ),
            BoolExpr::block(
                vec![first_binding.clone()],
                BoolExpr::gt_int(local_int(0, "left").into(), int(20).into()),
            ),
        );
        let second_condition = BoolExpr::and(
            BoolExpr::equal(
                Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
                Expr::from(int(11)),
            ),
            BoolExpr::block(
                vec![second_binding.clone()],
                BoolExpr::gt_int(local_int(1, "left").into(), int(20).into()),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple([int(11), int(37)]),
                    )],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(local_int(0, "left"))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_binding],
                                int_return_expr(local_int(1, "left")),
                            ),
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
    fn plan_tuple_subject_structural_pattern_binds_element_projections() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    #(left, right) -> left + right
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    int_return_block(
                        [
                            let_int_step(
                                0,
                                "left",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
                            ),
                            let_int_step(
                                1,
                                "right",
                                local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
                            ),
                        ],
                        int_return_expr(local_int(0, "left").add_int(local_int(1, "right"))),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_literal_pattern_preserves_ordered_fallthrough_shape() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(2, 5) {
    #(1, value) -> value
    #(2, value) -> value + 10
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let first_binding = let_int_step(
            0,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let second_binding = let_int_step(
            1,
            "value",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let first_condition = BoolExpr::equal(
            Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
            Expr::from(int(1)),
        );
        let second_condition = BoolExpr::equal(
            Expr::from(local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0)),
            Expr::from(int(2)),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(2), int(5)]))],
                    crate::plan::IntReturn::bool_case(
                        first_condition,
                        int_return_block([first_binding], int_return_expr(local_int(0, "value"))),
                        crate::plan::IntReturn::bool_case(
                            second_condition,
                            int_return_block(
                                [second_binding],
                                int_return_expr(local_int(1, "value").add_int(int(10))),
                            ),
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
    fn plan_tuple_subject_structural_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    #(left, right) if left > 0 -> right
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let left_binding = let_int_step(
            0,
            "left",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(0),
        );
        let right_binding = let_int_step(
            1,
            "right",
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_int(1),
        );
        let condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![left_binding.clone(), right_binding.clone()],
                BoolExpr::gt_int(local_int(0, "left").into(), int(0).into()),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(0, "<case:tuple:0>", tuple([int(1), int(2)]))],
                    crate::plan::IntReturn::bool_case(
                        condition,
                        int_return_block(
                            [left_binding, right_binding],
                            int_return_expr(local_int(1, "right")),
                        ),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn tuple_case_pattern_supports_literal_leaf_families() {
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                float(1.0).into(),
                ValueType::Float,
            ),
            Ok(super::TupleCasePattern::literal(
                float(1.0).into(),
                float(1.0).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::String {
                    location: dummy_span(),
                    value: "one".into(),
                },
                string("one").into(),
                ValueType::String,
            ),
            Ok(super::TupleCasePattern::literal(
                string("one").into(),
                string("one").into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "True".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: known_constructor("True", "gleam", 0),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                bool_(true).into(),
                ValueType::Bool,
            ),
            Ok(super::TupleCasePattern::literal(
                bool_(true).into(),
                bool_(true).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "False".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: known_constructor("False", "gleam", 1),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                bool_(false).into(),
                ValueType::Bool,
            ),
            Ok(super::TupleCasePattern::literal(
                bool_(false).into(),
                bool_(false).into(),
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Nil".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: known_constructor("Nil", "gleam", 0),
                    spread: None,
                    type_: gleam_core::type_::nil(),
                },
                nil().into(),
                ValueType::Nil,
            ),
            Ok(super::TupleCasePattern::any()),
        );
    }

    #[test]
    fn tuple_case_pattern_combines_literal_match_conditions() {
        let tuple_type = vec![ValueType::Int, ValueType::Int];
        let actual = super::plan_tuple_case_pattern(
            Pattern::Tuple {
                location: dummy_span(),
                elements: vec![
                    Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: num_bigint::BigInt::from(1),
                    },
                    Pattern::Int {
                        location: dummy_span(),
                        value: "2".into(),
                        int_value: num_bigint::BigInt::from(2),
                    },
                ],
            },
            tuple([int(1), int(2)]).into(),
            ValueType::Tuple(tuple_type.clone()),
        );

        assert_eq!(
            actual,
            Ok(super::TupleCasePattern {
                match_condition: Some(BoolExpr::and(
                    BoolExpr::equal(tuple([int(1), int(2)]).index_int(0).into(), int(1).into()),
                    BoolExpr::equal(tuple([int(1), int(2)]).index_int(1).into(), int(2).into()),
                )),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn tuple_case_pattern_binds_string_prefix_left_alias() {
        let actual = super::plan_tuple_case_pattern(
            Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: Some(("prefix".into(), dummy_span())),
                right_location: dummy_span(),
                left_side_string: "Hello, ".into(),
                right_side_assignment: AssignName::Discard("_rest".into()),
            },
            Expr::from(string("Hello, Geam")),
            ValueType::String,
        );

        assert_eq!(
            actual,
            Ok(super::TupleCasePattern {
                match_condition: Some(BoolExpr::string_starts_with(
                    string("Hello, Geam").into(),
                    "Hello, ".into(),
                )),
                branch_bindings: vec![("prefix".into(), Expr::from(string("Hello, ")))],
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn plan_tuple_subject_inner_string_prefix_pattern_uses_tuple_projection() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #("Hello, Geam", "!") {
    #("Hello, " <> name, suffix) -> name <> suffix
    _ -> "none"
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::String, ValueType::String];
        let first_element = local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_string(0);
        let second_element = local_tuple(0, "<case:tuple:0>", tuple_type.clone()).index_string(1);
        let bind_name = Step::let_string(
            StringLocalId(0),
            "name".into(),
            StringExpr::drop_prefix(first_element.into(), "Hello, ".into()),
        );
        let bind_suffix =
            Step::let_string(StringLocalId(1), "suffix".into(), second_element.into());
        let expected = module(
            "main",
            function(
                "main",
                string_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple([string("Hello, Geam"), string("!")]),
                    )],
                    crate::plan::StringReturn::bool_case(
                        BoolExpr::string_starts_with(
                            local_tuple(0, "<case:tuple:0>", tuple_type)
                                .index_string(0)
                                .into(),
                            "Hello, ".into(),
                        ),
                        string_return_block(
                            [bind_name, bind_suffix],
                            string_return_expr(
                                local_string(0, "name").concatenate(local_string(1, "suffix")),
                            ),
                        ),
                        string_return_expr(string("none")),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_subject_inner_list_pattern_uses_list_matcher() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #([1, 2], 3) {
    #([first, ..], value) -> first + value
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::List(Box::new(ValueType::Int)), ValueType::Int];
        let list_subject = ListExpr::tuple_index(
            local_tuple(0, "<case:tuple:0>", tuple_type.clone()).into(),
            0,
            ValueType::Int,
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_tuple_step(
                        0,
                        "<case:tuple:0>",
                        tuple(vec![
                            Expr::from(list([int(1), int(2)], ValueType::Int)),
                            Expr::from(int(3)),
                        ]),
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::list_length_at_least(list_subject.clone(), 1),
                        int_return_block(
                            [
                                Step::let_int(
                                    IntLocalId(0),
                                    "first".into(),
                                    crate::plan::IntExpr::list_index(list_subject, 0),
                                ),
                                let_int_step(
                                    1,
                                    "value",
                                    local_tuple(0, "<case:tuple:0>", tuple_type).index_int(1),
                                ),
                            ],
                            int_return_expr(local_int(0, "first").add_int(local_int(1, "value"))),
                        ),
                        int_return_expr(int(0)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_tuple_inner_list_pattern_errors_are_propagated() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  case #([1]) {
    #([value]) -> value
    _ -> 0
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        let elements = expect_single_tuple_list_elements(&mut clauses[0].pattern[0]);
        elements[0] = Pattern::List {
            location: dummy_span(),
            elements: Vec::new(),
            tail: None,
            type_: gleam_core::type_::int(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::KindMismatch {
                        expected: ValueType::Int,
                        actual: crate::planner::PatternKind::List,
                    },
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected a single tuple-inner list pattern")]
    fn tuple_inner_list_fixture_guard_rejects_int_pattern() {
        let mut pattern = Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        };
        let _ = expect_single_tuple_list_elements(&mut pattern);
    }

    fn expect_single_tuple_list_elements(
        pattern: &mut Pattern<std::sync::Arc<gleam_core::type_::Type>>,
    ) -> &mut Vec<Pattern<std::sync::Arc<gleam_core::type_::Type>>> {
        if let Pattern::Tuple { elements, .. } = pattern
            && let [Pattern::List { elements, .. }] = elements.as_mut_slice()
        {
            return elements;
        }
        panic!("expected a single tuple-inner list pattern");
    }

    #[test]
    fn reject_margin_tuple_structural_element_errors_are_propagated() {
        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Tuple {
                        location: dummy_span(),
                        elements: Vec::new(),
                    }],
                },
                tuple([int(1)]).into(),
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Tuple(Vec::new()),
            )),
        );
    }

    #[test]
    fn reject_margin_tuple_subject_invalid_pattern_during_case_lowering() {
        let mut module = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = Pattern::Invalid {
            location: dummy_span(),
            type_: gleam_core::type_::tuple(vec![
                gleam_core::type_::int(),
                gleam_core::type_::int(),
            ]),
        };
        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_tuple_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case { <<1:native>> #(1, 2) } {
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
    fn reject_profile_tuple_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case #(1, 2) {
    _ -> { <<1:native>> 0 }
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
    fn reject_margin_tuple_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
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
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::Parameter(crate::plan::TypeParameterId(0)),
                        actual: ValueType::Int,
                    },
                },
            }),
        );

        let mut empty_pattern = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[0].body[0],
        );
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch {
                        expected: 1,
                        actual: 0,
                    },
                },
            }),
        );

        let mut subject_expression_family_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case #(1, 2) {
    _ -> 1
  }
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut subject_expression_family_mismatch.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_core::type_::tuple(vec![
                gleam_core::type_::int(),
                gleam_core::type_::int(),
            ]),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(subject_expression_family_mismatch),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn reject_margin_tuple_case_pattern_mismatched_and_invalid_shapes() {
        let tuple_type = ValueType::Tuple(vec![ValueType::Int]);
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                tuple_type.clone(),
                ValueType::Int,
            )),
        );

        let conflicting_shape = Expr::tuple(
            crate::plan::TupleExpr::local_get(
                crate::plan::TupleLocalId(0),
                "pair".into(),
                vec![ValueType::Int],
            )
            .with_shape(vec![crate::plan::ValueShape::String].into_boxed_slice()),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                conflicting_shape,
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                },
                Expr::int(IntExpr::value(1.into())),
                ValueType::BitArray,
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::BitArray,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                },
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::BitArray,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(gleam_core::ast::Pattern::Variable {
                        location: dummy_span(),
                        name: "value".into(),
                        type_: gleam_core::type_::int(),
                        origin: VariableOrigin::generated(),
                    }),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                tuple_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                tuple_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                Expr::from(tuple([int(1)])),
                tuple_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                tuple_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(super::super::pattern_kind_mismatch(
                ValueType::Int,
                crate::planner::PatternKind::List,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "prefix".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::String,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "prefix".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                },
                Expr::from(int(1)),
                ValueType::String,
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::String,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Invalid {
                    location: dummy_span(),
                    type_: gleam_core::type_::tuple(vec![gleam_core::type_::int()]),
                },
                Expr::from(tuple([int(1)])),
                tuple_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "True".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::bool(),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Bool,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Nil".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: gleam_core::type_::nil(),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Nil,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: Vec::new(),
                },
                Expr::from(int(1)),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(Vec::new()),
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::from(tuple([int(1), int(2)])),
                ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TupleArity {
                        expected: 2,
                        actual: 1,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::from(int(1)),
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn nested_list_and_tuple_patterns_propagate_segment_validation() {
        let invalid = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::BitArraySegmentOptions {
                    reason: crate::planner::InvalidBitArraySegmentOptionsReason::MultipleKinds,
                },
            },
        });
        let bit_array = Expr::bit_array(BitArrayExpr::value(Vec::new()));

        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::List {
                    location: dummy_span(),
                    elements: vec![invalid_bit_array_pattern()],
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::bit_array()),
                },
                Expr::list(
                    ListExpr::try_value(vec![bit_array.clone()], ValueType::BitArray)
                        .expect("test bit-array list should be valid"),
                ),
                ValueType::List(Box::new(ValueType::BitArray)),
            ),
            invalid.clone(),
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![invalid_bit_array_pattern()],
                },
                Expr::tuple(TupleExpr::value(vec![bit_array], vec![ValueType::BitArray],)),
                ValueType::Tuple(vec![ValueType::BitArray]),
            ),
            invalid,
        );
    }

    #[test]
    fn plan_tuple_nested_bit_array_pattern_projects_before_matching() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case #(<<1>>) {
    #(<<1>>) -> 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let tuple_type = vec![ValueType::BitArray];
        let subject_name = "<case:tuple:0>";
        let subject = BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }]);
        let pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
            pattern: BitArrayPatternValue::Literal(1.into()),
            size: BitArrayPatternSize::new(BitArrayPatternSizeExpr::value(8.into()), 1),
            endianness: Endianness::Big,
            signedness: Signedness::Unsigned,
        }]);
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [Step::let_tuple(
                        TupleLocalId(0),
                        subject_name.into(),
                        TupleExpr::value(vec![Expr::bit_array(subject)], tuple_type.clone()),
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::bit_array_matches(
                            BitArrayExpr::tuple_index(
                                TupleExpr::local_get(
                                    TupleLocalId(0),
                                    subject_name.into(),
                                    tuple_type,
                                ),
                                0,
                            ),
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
    fn refutable_custom_tuple_element_keeps_match_without_total_binding_steps() {
        let module_name = ecow::EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let ast_type =
            gleam_core::type_::result(gleam_core::type_::int(), gleam_core::type_::string());
        let type_ = CustomType::new(
            CustomTypeName::new(
                "".into(),
                gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Ok".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let value = CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), type_.clone()),
            "value".into(),
        );
        let pattern = Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Ok".into(),
            arguments: vec![gleam_core::ast::CallArg {
                label: None,
                location: dummy_span(),
                value: Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                },
                implicit: None,
            }],
            module: None,
            constructor: gleam_core::analyse::Inferred::Known(
                gleam_core::type_::PatternConstructor {
                    name: "Ok".into(),
                    field_map: None,
                    documentation: None,
                    module: gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                    location: dummy_span(),
                    constructor_index: 0,
                },
            ),
            spread: None,
            type_: ast_type,
        };

        assert_eq!(
            super::plan_tuple_case_pattern_with_context(
                pattern,
                Expr::custom(value.clone()),
                ValueType::Custom(type_),
                &mut context,
            ),
            Ok(super::TupleCasePattern {
                match_condition: Some(BoolExpr::custom_matches(
                    value,
                    CustomPattern::new(constructor, vec![AssertPattern::Int(1.into())], None,),
                )),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn exact_custom_tuple_element_preserves_total_binding_steps() {
        let module_name = ecow::EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let ast_type =
            gleam_core::type_::result(gleam_core::type_::int(), gleam_core::type_::string());
        let type_ = CustomType::new(
            CustomTypeName::new(
                "".into(),
                gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Ok".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let value = CustomExpr::try_constructor(
            constructor.clone(),
            vec![Expr::int(IntExpr::value(1.into()))],
        )
        .expect("test custom construction should be valid");
        let pattern = Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Ok".into(),
            arguments: vec![gleam_core::ast::CallArg {
                label: None,
                location: dummy_span(),
                value: Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                implicit: None,
            }],
            module: None,
            constructor: gleam_core::analyse::Inferred::Known(
                gleam_core::type_::PatternConstructor {
                    name: "Ok".into(),
                    field_map: None,
                    documentation: None,
                    module: gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                    location: dummy_span(),
                    constructor_index: 0,
                },
            ),
            spread: None,
            type_: ast_type,
        };
        let binding = CustomBindingPattern::exact(
            value.shape().clone(),
            constructor.clone(),
            vec![TotalBindingPattern::discard(ValueType::Int)],
        );

        assert_eq!(
            super::plan_tuple_case_pattern_with_context(
                pattern,
                Expr::custom(value.clone()),
                ValueType::Custom(type_),
                &mut context,
            ),
            Ok(super::TupleCasePattern {
                match_condition: Some(BoolExpr::custom_matches(
                    value.clone(),
                    CustomPattern::new(
                        constructor,
                        vec![AssertPattern::Discard],
                        Some(vec![TotalBindingPattern::discard(ValueType::Int)]),
                    ),
                )),
                branch_bindings: Vec::new(),
                total_branch_steps: vec![
                    Step::let_custom(CustomLocalId(0), "<case:tuple:custom:0>".into(), value,),
                    Step::bind_custom_fields(CustomLocalId(0), binding),
                ],
                is_total: true,
            }),
        );
    }

    #[test]
    fn custom_pattern_rejects_a_non_custom_projected_tuple_element() {
        let custom_type =
            gleam_core::type_::result(gleam_core::type_::int(), gleam_core::type_::string());
        let subject_type = ValueType::from_gleam(custom_type.as_ref())
            .expect("custom return type should map to a plan type");

        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: dummy_span(),
                        value: Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::int(),
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: known_constructor("Ok", gleam_core::type_::PRELUDE_MODULE_NAME, 0,),
                    spread: None,
                    type_: custom_type.clone(),
                },
                Expr::int(IntExpr::value(1.into())),
                subject_type.clone(),
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Custom,
                InvalidExpressionType::Int,
            )),
        );

        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Boxed".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Default::default(),
                    spread: None,
                    type_: custom_type,
                },
                Expr::int(IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                subject_type,
            )),
        );

        let result_ast_type =
            gleam_core::type_::result(gleam_core::type_::int(), gleam_core::type_::string());
        let result_type = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new(
                "".into(),
                gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        assert_eq!(
            super::plan_tuple_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: dummy_span(),
                        value: gleam_core::ast::Pattern::BitArraySize(
                            gleam_core::ast::BitArraySize::Int {
                                location: dummy_span(),
                                value: "1".into(),
                                int_value: 1.into(),
                            },
                        ),
                        implicit: None,
                    }],
                    module: None,
                    constructor: gleam_core::analyse::Inferred::Known(
                        gleam_core::type_::PatternConstructor {
                            name: "Ok".into(),
                            field_map: None,
                            documentation: None,
                            module: gleam_core::type_::PRELUDE_MODULE_NAME.into(),
                            location: dummy_span(),
                            constructor_index: 0,
                        },
                    ),
                    spread: None,
                    type_: result_ast_type,
                },
                Expr::custom(crate::plan::CustomExpr::local_get(
                    crate::plan::CustomLocal::new(
                        crate::plan::CustomLocalId(0),
                        result_type.clone(),
                    ),
                    "value".into(),
                )),
                ValueType::Custom(result_type),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
                },
            }),
        );
    }

    fn known_constructor(
        name: &str,
        module: &str,
        index: u16,
    ) -> gleam_core::analyse::Inferred<gleam_core::type_::PatternConstructor> {
        gleam_core::analyse::Inferred::Known(gleam_core::type_::PatternConstructor {
            name: name.into(),
            field_map: None,
            documentation: None,
            module: module.into(),
            location: dummy_span(),
            constructor_index: index,
        })
    }

    fn invalid_bit_array_pattern() -> Pattern<std::sync::Arc<gleam_core::type_::Type>> {
        Pattern::BitArray {
            location: dummy_span(),
            segments: vec![gleam_core::ast::BitArraySegment {
                location: dummy_span(),
                value: Box::new(Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
                options: vec![
                    gleam_core::ast::BitArrayOption::Int {
                        location: dummy_span(),
                    },
                    gleam_core::ast::BitArrayOption::Float {
                        location: dummy_span(),
                    },
                ],
                type_: gleam_core::type_::int(),
            }],
        }
    }
}
