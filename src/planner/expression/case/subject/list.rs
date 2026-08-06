use super::super::super::{
    conversion::expect_expression, plan_expr_with_expected_source_stop_shape,
};
use super::super::super::{list_index_expr, tuple_index_expr};
use super::{CaseClause, OrderedCaseCandidateInput, OrderedCasePattern};
use crate::plan::{
    BoolExpr, CustomBindingPattern, CustomExpr, Expr, FloatExpr, IntExpr, ListExpr, ListLocal,
    Step, StringExpr, TupleExpr, ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use crate::planner::pattern::plan_custom_subject_pattern;
use ecow::EcoString;
use gleam_core::ast::{AssignName, Pattern, SrcSpan, TailPattern, TypedExpr};
use gleam_core::strings::convert_string_escape_chars;
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    element_type: ValueType,
    subject_shape: ValueShape,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject_value_type = ValueType::List(Box::new(element_type.clone()));
    let subject = plan_expr_with_expected_source_stop_shape(subject, subject_shape, context)?;
    let return_shape = context.value_shape(type_.as_ref());

    let subject: ListExpr = expect_expression(subject)?;
    let (subject_step, subject) = bind_list_case_subject(subject, context);
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
                    let pattern = plan_list_case_pattern_with_context(
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
pub(super) struct ListCasePattern {
    match_condition: Option<BoolExpr>,
    branch_bindings: Vec<(EcoString, Expr)>,
    total_branch_steps: Vec<Step>,
    is_total: bool,
}

pub(super) type ListCasePatternParts = (Option<BoolExpr>, Vec<(EcoString, Expr)>, Vec<Step>, bool);

impl ListCasePattern {
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
        let prefix_binding = left_side_assignment
            .map(|(name, _)| (name, Expr::string(StringExpr::value(prefix.clone()))));
        let suffix_binding = match right_side_assignment {
            AssignName::Variable(name) => {
                Some((name, Expr::string(StringExpr::drop_prefix(value, prefix))))
            }
            AssignName::Discard(_) => None,
        };
        pattern.branch_bindings.extend(prefix_binding);
        pattern.branch_bindings.extend(suffix_binding);

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

    pub(super) fn into_parts(self) -> ListCasePatternParts {
        (
            self.match_condition,
            self.branch_bindings,
            self.total_branch_steps,
            self.is_total,
        )
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

#[cfg(test)]
pub(super) fn plan_list_case_pattern(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
) -> Result<ListCasePattern, PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
    plan_list_case_pattern_with_context(pattern, value, subject_type, &mut context)
}

pub(super) fn plan_list_case_pattern_with_context(
    pattern: Pattern<Arc<Type>>,
    value: Expr,
    subject_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<ListCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable {
            ref name,
            ref type_,
            ..
        } if matches_type(type_.as_ref(), &subject_type, context) => {
            Ok(ListCasePattern::any().with_binding(name.clone(), value))
        }
        ref pattern @ Pattern::Variable { .. } => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), &subject_type, context) => {
            Ok(ListCasePattern::any())
        }
        ref pattern @ Pattern::Discard { .. } => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_list_case_pattern_with_context(
                *pattern,
                value.clone(),
                subject_type,
                context,
            )?;
            Ok(pattern.with_binding(name, value))
        }
        Pattern::List {
            location,
            elements,
            tail,
            type_,
        } => plan_list_structural_case_pattern(
            location,
            elements,
            tail.map(|tail| *tail),
            type_,
            value,
            subject_type,
            context,
        ),
        Pattern::Tuple { location, elements } => {
            plan_tuple_case_pattern(location, elements, value, subject_type, context)
        }
        Pattern::Int { int_value, .. } if subject_type == ValueType::Int => Ok(
            ListCasePattern::literal(value, Expr::int(IntExpr::value(int_value))),
        ),
        Pattern::Float { float_value, .. } if subject_type == ValueType::Float => Ok(
            ListCasePattern::literal(value, Expr::float(FloatExpr::value(float_value.value()))),
        ),
        Pattern::String { value: literal, .. } if subject_type == ValueType::String => {
            Ok(ListCasePattern::literal(
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
                ("True", ValueType::Bool) => Ok(ListCasePattern::literal(
                    value,
                    Expr::bool(BoolExpr::value(true)),
                )),
                ("False", ValueType::Bool) => Ok(ListCasePattern::literal(
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
                Ok(ListCasePattern::any())
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
            Ok(ListCasePattern {
                match_condition: Some(BoolExpr::custom_matches(value, pattern.pattern)),
                branch_bindings: Vec::new(),
                total_branch_steps,
                is_total: pattern.is_total,
            })
        }
        Pattern::StringPrefix {
            left_side_string,
            left_side_assignment,
            right_side_assignment,
            ..
        } if subject_type == ValueType::String => ListCasePattern::string_prefix(
            value,
            convert_string_escape_chars(&left_side_string),
            left_side_assignment,
            right_side_assignment,
        ),
        Pattern::BitArray { segments, .. } if subject_type == ValueType::BitArray => {
            let value = expect_expression(value)?;
            super::bit_array::plan_structural_pattern(segments, value, context)
                .map(ListCasePattern::from_bit_array_pattern)
        }
        ref pattern @ (Pattern::BitArraySize(_)
        | Pattern::BitArray { .. }
        | Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Constructor { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            pattern,
            &ValueShape::from_value_type(subject_type),
            context,
        )),
    }
}

fn plan_list_structural_case_pattern(
    location: SrcSpan,
    elements: Vec<Pattern<Arc<Type>>>,
    tail: Option<TailPattern<Arc<Type>>>,
    type_: Arc<Type>,
    value: Expr,
    subject_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<ListCasePattern, PlanError> {
    let validated = crate::planner::pattern::validate_list_pattern(
        &Pattern::List {
            location,
            elements: elements.clone(),
            tail: tail.clone().map(Box::new),
            type_,
        },
        &ValueShape::from_value_type(subject_type),
        context,
    )?;
    let element_type = validated.item_shape.value_type();
    let list: ListExpr = expect_expression(value)?;

    let has_tail = tail.is_some();
    let element_count = elements.len();
    let mut patterns = Vec::with_capacity(elements.len());
    for (index, pattern) in elements.into_iter().enumerate() {
        let value = list_index_expr(list.clone(), index, element_type.clone())?;
        patterns.push(plan_list_case_pattern_with_context(
            pattern,
            value,
            element_type.clone(),
            context,
        )?);
    }

    let mut pattern = combine_list_case_patterns(patterns);
    pattern.match_condition = combine_conditions(
        list_length_condition(list.clone(), element_count, has_tail),
        pattern.match_condition,
    );
    pattern.is_total = has_tail && element_count == 0 && pattern.is_total;
    if let Some(tail) = validated.tail
        && let Some(binding) = plan_list_tail_binding(tail, list, element_count)
    {
        pattern.branch_bindings.push(binding);
    }

    Ok(pattern)
}

fn plan_tuple_case_pattern(
    location: SrcSpan,
    elements: Vec<Pattern<Arc<Type>>>,
    value: Expr,
    subject_type: ValueType,
    context: &mut PlanContext<'_>,
) -> Result<ListCasePattern, PlanError> {
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
        patterns.push(plan_list_case_pattern_with_context(
            pattern, value, type_, context,
        )?);
    }

    Ok(combine_list_case_patterns(patterns))
}

fn plan_list_tail_binding(
    tail: crate::planner::pattern::ValidatedListTail,
    list: ListExpr,
    element_count: usize,
) -> Option<(EcoString, Expr)> {
    match tail {
        crate::planner::pattern::ValidatedListTail::Named(name) => Some((
            name.clone(),
            Expr::list(ListExpr::drop_first(list, element_count)),
        )),
        crate::planner::pattern::ValidatedListTail::Discard => None,
    }
}

fn combine_list_case_patterns(patterns: Vec<ListCasePattern>) -> ListCasePattern {
    let mut combined = ListCasePattern::any();
    for pattern in patterns {
        combined.match_condition =
            combine_conditions(combined.match_condition, pattern.match_condition);
        combined.branch_bindings.extend(pattern.branch_bindings);
        combined
            .total_branch_steps
            .extend(pattern.total_branch_steps);
        combined.is_total &= pattern.is_total;
    }

    combined
}

fn combine_conditions(left: Option<BoolExpr>, right: Option<BoolExpr>) -> Option<BoolExpr> {
    match (left, right) {
        (Some(left), Some(right)) => Some(BoolExpr::and(left, right)),
        (Some(condition), None) | (None, Some(condition)) => Some(condition),
        (None, None) => None,
    }
}

fn list_length_condition(list: ListExpr, length: usize, has_tail: bool) -> Option<BoolExpr> {
    match (has_tail, length) {
        (true, 0) => None,
        (true, _) => Some(BoolExpr::list_length_at_least(list, length)),
        (false, _) => Some(BoolExpr::list_length_equals(list, length)),
    }
}

fn matches_type(type_: &Type, subject_type: &ValueType, context: &mut PlanContext<'_>) -> bool {
    context.value_type(type_) == *subject_type
}

fn bind_list_case_subject(subject: ListExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let item_shape = subject.item_shape().clone();
    let (local, value) = context.define_internal_list_value(subject);
    let name = internal_list_case_subject_name(&local);
    (
        Step::let_list_expr(name.clone(), value),
        Expr::list(ListExpr::local_get(local, name).with_item_shape(item_shape)),
    )
}

fn internal_list_case_subject_name(local: &ListLocal) -> EcoString {
    format!("<case:list:{}:{}>", local.family_name(), local.index()).into()
}

fn total_custom_binding_steps(
    value: CustomExpr,
    binding: CustomBindingPattern,
    context: &mut PlanContext<'_>,
) -> Vec<Step> {
    let local = context.define_internal_custom_local();
    let name = format!("<case:list:custom:{}>", local.0).into();
    vec![
        Step::let_custom(local, name, value),
        Step::bind_custom_fields(local, binding),
    ]
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        AssertPattern, BitArrayExpr, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArraySegment, BoolExpr,
        CustomConstructor, CustomConstructorField, CustomExpr, CustomLocalId, CustomPattern,
        CustomType, CustomTypeName, Endianness, Expr, IntExpr, IntListLocalId, IntLocalId,
        ListExpr, ListLocal, Signedness, Step, StringExpr, TupleExpr, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bool_, bool_return_block, bool_return_expr, equal, function, int, int_return_block,
        int_return_expr, let_list_step, list, local_int, local_list, module,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::ast::Pattern;
    use gleam_core::type_::error::VariableOrigin;
    use std::collections::HashMap;

    #[test]
    fn list_case_subject_local_preserves_custom_item_refinement() {
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
        let subject = ListExpr::try_value(vec![Expr::custom(value)], ValueType::Custom(type_))
            .expect("test custom list should be valid");

        let (_, local) = super::bind_list_case_subject(subject, &mut context);

        assert_eq!(
            local.value_shape(),
            &ValueShape::List(Box::new(custom_shape)),
        );
    }

    #[test]
    fn plan_list_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    value as alias -> value == alias
  }
}
"#,
        ))
        .expect("source should plan");
        let value = local_list(1, "value", ValueType::Int);
        let alias = local_list(2, "alias", ValueType::Int);
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    bool_return_block(
                        [
                            let_list_step(
                                1,
                                "value",
                                local_list(0, "<case:list:int:0>", ValueType::Int),
                            ),
                            let_list_step(
                                2,
                                "alias",
                                local_list(0, "<case:list:int:0>", ValueType::Int),
                            ),
                        ],
                        bool_return_expr(equal(value, alias)),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_subject_guard_wraps_condition_and_branch_with_bindings() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    value if value == [] -> False
    value as alias if alias == [1, 2] -> value == alias
    _ -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let second_value = local_list(2, "value", ValueType::Int);
        let second_alias = local_list(3, "alias", ValueType::Int);
        let first_binding = let_list_step(
            1,
            "value",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let second_value_binding = let_list_step(
            2,
            "value",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let second_alias_binding = let_list_step(
            3,
            "alias",
            local_list(0, "<case:list:int:0>", ValueType::Int),
        );
        let first_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![first_binding.clone()],
                BoolExpr::equal(
                    Expr::from(local_list(1, "value", ValueType::Int)),
                    Expr::from(list(Vec::<Expr>::new(), ValueType::Int)),
                ),
            ),
        );
        let second_condition = BoolExpr::and(
            BoolExpr::value(true),
            BoolExpr::block(
                vec![second_value_binding.clone(), second_alias_binding.clone()],
                BoolExpr::equal(
                    Expr::from(local_list(3, "alias", ValueType::Int)),
                    Expr::from(list([int(1), int(2)], ValueType::Int)),
                ),
            ),
        );
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    crate::plan::BoolReturn::bool_case(
                        first_condition,
                        bool_return_block([first_binding], bool_return_expr(bool_(false))),
                        crate::plan::BoolReturn::bool_case(
                            second_condition,
                            bool_return_block(
                                [second_value_binding, second_alias_binding],
                                bool_return_expr(equal(second_value, second_alias)),
                            ),
                            bool_return_expr(bool_(false)),
                        ),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_subject_structural_pattern_binds_element_after_length_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2] {
    [first, ..] -> first
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "<case:list:int:0>".into(),
        );
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2)], ValueType::Int),
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::list_length_at_least(subject.clone(), 1),
                        int_return_block(
                            [Step::let_int(
                                IntLocalId(0),
                                "first".into(),
                                crate::plan::IntExpr::list_index(subject, 0),
                            )],
                            int_return_expr(local_int(0, "first")),
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
    fn plan_list_subject_tail_binding_uses_drop_first_after_length_match() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [1, 2, 3] {
    [first, ..rest] -> rest == [2, 3]
    _ -> False
  }
}
"#,
        ))
        .expect("source should plan");
        let subject = ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "<case:list:int:0>".into(),
        );
        let rest = local_list(1, "rest", ValueType::Int);
        let expected = module(
            "main",
            function(
                "main",
                bool_return_block(
                    [let_list_step(
                        0,
                        "<case:list:int:0>",
                        list([int(1), int(2), int(3)], ValueType::Int),
                    )],
                    crate::plan::BoolReturn::bool_case(
                        BoolExpr::list_length_at_least(subject.clone(), 1),
                        bool_return_block(
                            [
                                Step::let_int(
                                    IntLocalId(0),
                                    "first".into(),
                                    crate::plan::IntExpr::list_index(subject.clone(), 0),
                                ),
                                Step::let_list_expr(
                                    "rest".into(),
                                    crate::plan::ListLocalExpr::Int {
                                        local: IntListLocalId(1),
                                        value: ListExpr::drop_first(subject.clone(), 1)
                                            .into_int()
                                            .expect("expected int list"),
                                    },
                                ),
                            ],
                            bool_return_expr(equal(rest, list([int(2), int(3)], ValueType::Int))),
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
    fn reject_profile_list_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case { <<1:native>> [1] } {
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
    fn reject_profile_list_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case [1] {
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
    fn reject_margin_list_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
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
  case [1] {
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
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut subject_expression_family_mismatch = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
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
            type_: gleam_core::type_::list(gleam_core::type_::int()),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(subject_expression_family_mismatch),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::List,
                InvalidExpressionType::Int,
            )),
        );

        let mut invalid_pattern_during_case_lowering = crate::planner::support::compile(
            r#"
pub fn main() {
  case [1] {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut invalid_pattern_during_case_lowering.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(invalid_pattern_during_case_lowering),
            Err(super::super::pattern_type_mismatch(
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn reject_margin_list_case_pattern_mismatched_and_invalid_shapes() {
        let list_type = ValueType::List(Box::new(ValueType::Int));
        let subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
                subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Invalid {
                    location: dummy_span(),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                subject,
                list_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
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
            super::plan_list_case_pattern(
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
            super::plan_list_case_pattern(
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
    fn reject_margin_list_structural_element_projection_type_mismatch() {
        let int_list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::string(),
                    }],
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::string()),
                },
                int_list_subject,
                ValueType::List(Box::new(ValueType::String)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch {
                    expected: ValueType::String,
                    actual: ValueType::Int,
                },
            }),
        );
    }

    #[test]
    fn list_case_pattern_binds_string_prefix_parts() {
        let value = Expr::string(StringExpr::value("Hello, Geam".into()));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: Some(("prefix".into(), dummy_span())),
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Variable("name".into()),
                },
                value.clone(),
                ValueType::String,
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::string_starts_with(
                    StringExpr::value("Hello, Geam".into()),
                    "Hello, ".into(),
                )),
                branch_bindings: vec![
                    (
                        "prefix".into(),
                        Expr::string(StringExpr::value("Hello, ".into()))
                    ),
                    (
                        "name".into(),
                        Expr::string(StringExpr::drop_prefix(
                            value.into_string().expect("value should be string"),
                            "Hello, ".into(),
                        )),
                    ),
                ],
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Discard("_rest".into()),
                },
                Expr::string(StringExpr::value("Hello, Geam".into())),
                ValueType::String,
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::string_starts_with(
                    StringExpr::value("Hello, Geam".into()),
                    "Hello, ".into(),
                )),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn list_case_pattern_literal_builds_match_condition() {
        assert_eq!(
            super::ListCasePattern::literal(int(1).into(), int(1).into()),
            super::ListCasePattern {
                match_condition: Some(BoolExpr::equal(int(1).into(), int(1).into())),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            },
        );
    }

    #[test]
    fn list_case_pattern_supports_literal_leaf_families() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: gleam_core::parse::LiteralFloatValue::ONE,
                },
                crate::planner::dsl::float(1.0).into(),
                ValueType::Float,
            ),
            Ok(super::ListCasePattern::literal(
                crate::planner::dsl::float(1.0).into(),
                crate::planner::dsl::float(1.0).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::String {
                    location: dummy_span(),
                    value: "one".into(),
                },
                crate::planner::dsl::string("one").into(),
                ValueType::String,
            ),
            Ok(super::ListCasePattern::literal(
                crate::planner::dsl::string("one").into(),
                crate::planner::dsl::string("one").into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
            Ok(super::ListCasePattern::literal(
                bool_(true).into(),
                bool_(true).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
            Ok(super::ListCasePattern::literal(
                bool_(false).into(),
                bool_(false).into(),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
                crate::planner::dsl::nil().into(),
                ValueType::Nil,
            ),
            Ok(super::ListCasePattern::any()),
        );
    }

    #[test]
    fn list_structural_case_pattern_builds_fixed_length_condition() {
        let list_subject = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());

        assert_eq!(
            super::plan_list_case_pattern(
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
                Expr::list(list_subject.clone()),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::list_length_equals(list_subject, 1)),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn reject_margin_list_structural_pattern_shapes() {
        let list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        let list_type = ValueType::List(Box::new(ValueType::Int));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::generic_var(0)),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::List(Box::new(ValueType::Parameter(
                    crate::plan::TypeParameterId(0),
                ))),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::string()),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(super::super::pattern_type_mismatch(
                list_type.clone(),
                ValueType::List(Box::new(ValueType::String)),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::int(),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::KindMismatch {
                        expected: ValueType::Int,
                        actual: crate::planner::PatternKind::List,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                list_type.clone(),
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::List,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: None,
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(super::super::pattern_kind_mismatch(
                ValueType::Int,
                crate::planner::PatternKind::List,
            )),
        );
    }

    #[test]
    fn list_case_pattern_binds_tail_shapes() {
        let list_subject = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Variable {
                            location: dummy_span(),
                            name: "rest".into(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                            origin: VariableOrigin::generated(),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::list(list_subject.clone()),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: None,
                branch_bindings: vec![(
                    "rest".into(),
                    Expr::list(ListExpr::drop_first(list_subject.clone(), 0)),
                )],
                total_branch_steps: Vec::new(),
                is_total: true,
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                Expr::list(list_subject),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Ok(super::ListCasePattern {
                match_condition: None,
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: true,
            }),
        );
    }

    #[test]
    fn reject_margin_list_structural_tail_shapes() {
        let list_subject = Expr::list(ListExpr::local_get(
            ListLocal::int(IntListLocalId(0)),
            "values".into(),
        ));
        let list_type = ValueType::List(Box::new(ValueType::Int));

        for tail_pattern in [
            gleam_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "rest".into(),
                type_: gleam_core::type_::list(gleam_core::type_::string()),
                origin: VariableOrigin::generated(),
            },
            gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_core::type_::list(gleam_core::type_::string()),
            },
        ] {
            assert_eq!(
                super::plan_list_case_pattern(
                    gleam_core::ast::Pattern::List {
                        location: dummy_span(),
                        elements: Vec::new(),
                        tail: Some(Box::new(gleam_core::ast::TailPattern {
                            location: dummy_span(),
                            pattern: tail_pattern,
                        })),
                        type_: gleam_core::type_::list(gleam_core::type_::int()),
                    },
                    list_subject.clone(),
                    list_type.clone(),
                ),
                Err(super::super::pattern_type_mismatch(
                    list_type.clone(),
                    ValueType::List(Box::new(ValueType::String)),
                )),
            );
        }

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Invalid {
                            location: dummy_span(),
                            type_: gleam_core::type_::list(gleam_core::type_::int()),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                list_subject.clone(),
                list_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::ListTailKind {
                        actual: crate::planner::PatternKind::Invalid,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: Some(Box::new(gleam_core::ast::TailPattern {
                        location: dummy_span(),
                        pattern: gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: num_bigint::BigInt::from(1),
                        },
                    })),
                    type_: gleam_core::type_::list(gleam_core::type_::int()),
                },
                list_subject,
                list_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::ListTailKind {
                        actual: crate::planner::PatternKind::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn list_tuple_case_pattern_combines_literal_match_conditions() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: num_bigint::BigInt::from(1),
                        },
                        gleam_core::ast::Pattern::Int {
                            location: dummy_span(),
                            value: "2".into(),
                            int_value: num_bigint::BigInt::from(2),
                        },
                    ],
                },
                crate::planner::dsl::tuple([int(1), int(2)]).into(),
                ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
            ),
            Ok(super::ListCasePattern {
                match_condition: Some(BoolExpr::and(
                    BoolExpr::equal(
                        crate::planner::dsl::tuple([int(1), int(2)])
                            .index_int(0)
                            .into(),
                        int(1).into(),
                    ),
                    BoolExpr::equal(
                        crate::planner::dsl::tuple([int(1), int(2)])
                            .index_int(1)
                            .into(),
                        int(2).into(),
                    ),
                )),
                branch_bindings: Vec::new(),
                total_branch_steps: Vec::new(),
                is_total: false,
            }),
        );
    }

    #[test]
    fn reject_margin_list_nested_tuple_pattern_shapes() {
        let tuple_type = ValueType::Tuple(vec![ValueType::Int]);
        let tuple_value = Expr::tuple(crate::plan::TupleExpr::value(
            vec![Expr::int(crate::plan::IntExpr::value(1.into()))],
            vec![ValueType::Int],
        ));

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: Vec::new(),
                },
                tuple_value.clone(),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Tuple(Vec::new()),
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::int(),
                        },
                        gleam_core::ast::Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: gleam_core::type_::int(),
                        },
                    ],
                },
                tuple_value.clone(),
                tuple_type.clone(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TupleArity {
                        expected: 1,
                        actual: 2,
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: gleam_core::type_::int(),
                    }],
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                tuple_type.clone(),
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Tuple {
                        location: dummy_span(),
                        elements: Vec::new(),
                    }],
                },
                tuple_value,
                tuple_type,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Tuple(Vec::new()),
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
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![gleam_core::ast::Pattern::Discard {
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
    }

    #[test]
    fn list_case_pattern_helpers_preserve_condition_shapes() {
        let left = BoolExpr::local_get(crate::plan::BoolLocalId(0), "left".into());
        let right = BoolExpr::local_get(crate::plan::BoolLocalId(1), "right".into());
        assert_eq!(
            super::combine_conditions(Some(left.clone()), Some(right.clone())),
            Some(BoolExpr::and(left.clone(), right.clone())),
        );
        assert_eq!(
            super::combine_conditions(Some(left.clone()), None),
            Some(left),
        );
        assert_eq!(
            super::combine_conditions(None, Some(right.clone())),
            Some(right),
        );
        assert_eq!(super::combine_conditions(None, None), None);

        let list = ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "values".into());
        assert_eq!(super::list_length_condition(list.clone(), 0, true), None);
        assert_eq!(
            super::list_length_condition(list.clone(), 1, true),
            Some(BoolExpr::list_length_at_least(list.clone(), 1)),
        );
        assert_eq!(
            super::list_length_condition(list.clone(), 2, false),
            Some(BoolExpr::list_length_equals(list, 2)),
        );
    }

    #[test]
    fn reject_margin_list_element_special_patterns_with_wrong_subject_shape() {
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_string: "Hello, ".into(),
                    left_side_assignment: Some(("prefix".into(), dummy_span())),
                    right_location: dummy_span(),
                    right_side_assignment: gleam_core::ast::AssignName::Variable("name".into()),
                },
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::String,
            ),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::String,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Bool,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
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
                Expr::int(crate::plan::IntExpr::value(1.into())),
                ValueType::Int,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::Int,
                ValueType::Nil,
            )),
        );
        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::BitArray {
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
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::BitArray {
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
    }

    #[test]
    fn plan_list_nested_bit_array_pattern_checks_length_before_projection() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
pub fn main() {
  case [<<1>>] {
    [<<1>>] -> 1
    _ -> 0
  }
}
"#,
        ))
        .expect("source should plan");
        let subject_name = "<case:list:bit array:0>";
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
        let local: ListExpr = local_list(0, subject_name, ValueType::BitArray).into();
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_list_step(
                        0,
                        subject_name,
                        list([Expr::bit_array(subject)], ValueType::BitArray),
                    )],
                    crate::plan::IntReturn::bool_case(
                        BoolExpr::and(
                            BoolExpr::list_length_equals(local.clone(), 1),
                            BoolExpr::bit_array_matches(
                                BitArrayExpr::list_index(
                                    local
                                        .into_bit_array()
                                        .expect("local must be List(BitArray)"),
                                    0,
                                ),
                                pattern,
                            ),
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
    fn refutable_custom_list_element_keeps_match_without_total_binding_steps() {
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
        let pattern = gleam_core::ast::Pattern::Constructor {
            location: dummy_span(),
            name_location: dummy_span(),
            name: "Ok".into(),
            arguments: vec![gleam_core::ast::CallArg {
                label: None,
                location: dummy_span(),
                value: gleam_core::ast::Pattern::Int {
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
            super::plan_list_case_pattern_with_context(
                pattern,
                Expr::custom(value.clone()),
                ValueType::Custom(type_),
                &mut context,
            ),
            Ok(super::ListCasePattern {
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
    fn custom_pattern_rejects_a_non_custom_projected_list_element() {
        let custom_type =
            gleam_core::type_::result(gleam_core::type_::int(), gleam_core::type_::string());
        let subject_type = ValueType::from_gleam(custom_type.as_ref())
            .expect("custom return type should map to a plan type");

        assert_eq!(
            super::plan_list_case_pattern(
                gleam_core::ast::Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: dummy_span(),
                        value: gleam_core::ast::Pattern::Discard {
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
            super::plan_list_case_pattern(
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
            super::plan_list_case_pattern(
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
