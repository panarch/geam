mod assert;

use crate::plan::{
    AssertBinding, BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr,
    CustomBindingPattern, CustomConstructor, CustomExpr, CustomFunctionExpr, Expr, ExprKind,
    ExternalExpr, ExternalFunctionExpr, ExternalLocal, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListAssertTail, ListExpr, ListFunctionExpr,
    NilExpr, NilFunctionExpr, Step, StringExpr, StringFunctionExpr, TotalBindingPattern, TupleExpr,
    TupleFunctionExpr, TupleLocalId, TypedFunctionExprKind, UtfCodepointExpr,
    UtfCodepointFunctionExpr, ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedPatternKind,
};
use crate::planner::expression::{
    plan_expr, plan_expr_with_expected_source_stop_type, tuple_index_expr,
};
use ecow::EcoString;
use gleam_core::ast::{AssignmentKind, Pattern, TypedAssignment, TypedExpr, TypedPattern};

pub(super) fn plan_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match assignment.kind {
        AssignmentKind::Let => {
            let source_shape = context.value_shape_in_scope(assignment.value.type_().as_ref());
            let typed_pattern = assignment.pattern.clone();
            let pattern = plan_binding_pattern_in_context(assignment.pattern, context)?;
            let value = plan_ordinary_assignment_value(&pattern, assignment.value, context)?;
            crate::planner::pattern::validate_pattern(&typed_pattern, &source_shape, context)?;
            plan_assignment_steps(pattern, value, context)
        }
        AssignmentKind::Assert {
            location, message, ..
        } => assert::plan_assert_assignment_steps(
            location,
            assignment.pattern,
            assignment.value,
            message,
            context,
        ),
        AssignmentKind::Generated => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::GeneratedAssignment,
        }),
    }
}

pub(super) struct PlannedAssignment {
    pub(super) steps: Vec<Step>,
    pub(super) value: Expr,
}

pub(super) fn plan_final_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let (pattern, value) = match assignment.kind {
        AssignmentKind::Let => {
            let source_shape = context.value_shape_in_scope(assignment.value.type_().as_ref());
            let typed_pattern = assignment.pattern.clone();
            let pattern = plan_binding_pattern_in_context(assignment.pattern, context)?;
            let value = plan_ordinary_assignment_value(&pattern, assignment.value, context)?;
            crate::planner::pattern::validate_pattern(&typed_pattern, &source_shape, context)?;
            (pattern, value)
        }
        AssignmentKind::Assert {
            location, message, ..
        } => {
            return assert::plan_assert_assignment(
                location,
                assignment.pattern,
                assignment.value,
                message,
                context,
            );
        }
        AssignmentKind::Generated => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            });
        }
    };
    plan_bound_assignment(pattern, value, context)
}

fn plan_ordinary_assignment_value(
    pattern: &BindingPattern,
    value: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if matches!(pattern, BindingPattern::Discard) {
        plan_expr_with_expected_source_stop_type(value, ValueType::Nil, context)
    } else {
        plan_expr(value, context)
    }
}

fn plan_bound_assignment(
    pattern: BindingPattern,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    match pattern {
        BindingPattern::Named(name) => {
            let (step, value) = plan_variable_runtime_step_and_return(name, value, context);
            Ok(PlannedAssignment {
                steps: vec![step],
                value,
            })
        }
        BindingPattern::Discard => Ok(PlannedAssignment {
            steps: Vec::new(),
            value,
        }),
        BindingPattern::Tuple(elements) => plan_tuple_assignment(elements, value, context),
        BindingPattern::ListTail { tail, element_type } => {
            plan_list_tail_assignment(tail, element_type, value, context)
        }
        BindingPattern::Custom {
            source_shape: _,
            constructor_count,
            constructor,
            fields,
        } => plan_custom_assignment(constructor_count, constructor, fields, value, context),
        BindingPattern::Alias { pattern, name } => {
            plan_alias_assignment(*pattern, name, value, context)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BindingPattern {
    Named(EcoString),
    Discard,
    Tuple(Vec<BindingPattern>),
    ListTail {
        tail: ListTailBinding,
        element_type: ValueType,
    },
    Custom {
        source_shape: crate::plan::CustomValueShape,
        constructor_count: usize,
        constructor: CustomConstructor,
        fields: Vec<BindingPattern>,
    },
    Alias {
        pattern: Box<BindingPattern>,
        name: EcoString,
    },
}

fn plan_assignment_steps(
    pattern: BindingPattern,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match pattern {
        BindingPattern::Named(name) => Ok(vec![plan_variable_runtime_step(name, value, context)]),
        BindingPattern::Discard => Ok(vec![Step::evaluate(value)]),
        BindingPattern::Tuple(elements) => {
            Ok(plan_tuple_assignment(elements, value, context)?.steps)
        }
        BindingPattern::ListTail { tail, element_type } => {
            plan_list_tail_assignment_steps(tail, element_type, value, context)
        }
        BindingPattern::Custom {
            source_shape: _,
            constructor_count,
            constructor,
            fields,
        } => Ok(
            plan_custom_assignment(constructor_count, constructor, fields, value, context)?.steps,
        ),
        BindingPattern::Alias { pattern, name } => {
            Ok(plan_alias_assignment(*pattern, name, value, context)?.steps)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ListTailBinding {
    Named(EcoString),
    Discard,
}

fn plan_alias_assignment(
    pattern: BindingPattern,
    name: EcoString,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let mut planned = match pattern {
        BindingPattern::Named(name) => {
            let (step, value) = plan_variable_runtime_step_and_return(name, value, context);
            PlannedAssignment {
                steps: vec![step],
                value,
            }
        }
        BindingPattern::Discard => PlannedAssignment {
            steps: Vec::new(),
            value,
        },
        BindingPattern::Tuple(elements) => plan_tuple_assignment(elements, value, context)?,
        BindingPattern::ListTail { tail, element_type } => {
            plan_list_tail_assignment(tail, element_type, value, context)?
        }
        BindingPattern::Custom {
            source_shape: _,
            constructor_count,
            constructor,
            fields,
        } => plan_custom_assignment(constructor_count, constructor, fields, value, context)?,
        BindingPattern::Alias { pattern, name } => {
            plan_alias_assignment(*pattern, name, value, context)?
        }
    };
    let (step, value) = plan_variable_runtime_step_and_return(name, planned.value, context);
    planned.steps.push(step);
    Ok(PlannedAssignment {
        steps: planned.steps,
        value,
    })
}

fn plan_list_tail_assignment_steps(
    tail: ListTailBinding,
    element_type: ValueType,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    let planned = plan_list_tail_assignment(tail, element_type, value, context)?;
    if planned.steps.is_empty() {
        Ok(vec![Step::evaluate(planned.value)])
    } else {
        Ok(planned.steps)
    }
}

fn plan_list_tail_assignment(
    tail: ListTailBinding,
    element_type: ValueType,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let actual = value.value_type();
    let Some(value) = value.into_list() else {
        return Err(list_assignment_value_must_be_list(actual));
    };
    if value.element_type() != element_type {
        crate::planner::pattern::validate_pattern_value_type(
            ValueType::List(Box::new(element_type.clone())),
            ValueType::List(Box::new(value.element_type().clone())),
        )?;
    }

    match tail {
        ListTailBinding::Named(name) => {
            let item_shape = value.item_shape().clone();
            let (local, value) = context.define_list_value(name.clone(), value);
            Ok(PlannedAssignment {
                steps: vec![Step::let_list_expr(name.clone(), value)],
                value: Expr::list(ListExpr::local_get(local, name).with_item_shape(item_shape)),
            })
        }
        ListTailBinding::Discard => Ok(PlannedAssignment {
            steps: Vec::new(),
            value: Expr::list(value),
        }),
    }
}

fn plan_custom_assignment(
    constructor_count: usize,
    constructor: CustomConstructor,
    fields: Vec<BindingPattern>,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let actual = value.value_type();
    let Some(value) = value.into_custom() else {
        return Err(custom_assignment_value_must_be_custom(actual));
    };
    crate::planner::pattern::validate_pattern_value_type(
        ValueType::Custom(constructor.type_().clone()),
        ValueType::Custom(value.type_().clone()),
    )?;
    crate::planner::pattern::validate_constructor_arity(constructor.fields().len(), fields.len())?;

    let local = context.define_internal_custom_local();
    let typed_local = crate::plan::CustomLocal::from_shape(local, value.shape().clone());
    let name = internal_custom_name(local);
    let local_value = CustomExpr::local_get(typed_local, name.clone());
    let fields = fields
        .into_iter()
        .zip(constructor.fields())
        .map(|(pattern, field)| {
            plan_total_binding_pattern(
                pattern,
                ValueShape::from_value_type(field.type_().clone()),
                context,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let binding = total_custom_binding(
        value.shape().clone(),
        constructor_count,
        constructor,
        fields,
    )?;

    Ok(PlannedAssignment {
        steps: vec![
            Step::let_custom(local, name, value),
            Step::bind_custom_fields(local, binding),
        ],
        value: Expr::custom(local_value),
    })
}

fn plan_total_binding_pattern(
    pattern: BindingPattern,
    expected: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<TotalBindingPattern, PlanError> {
    match (pattern, expected) {
        (BindingPattern::Named(name), expected) => {
            let binding = AssertBinding::new(
                context.define_param_local_shape(name.clone(), expected.clone()),
                name,
                expected,
            );
            Ok(TotalBindingPattern::bind(binding))
        }
        (BindingPattern::Discard, expected) => {
            Ok(TotalBindingPattern::discard(expected.value_type()))
        }
        (BindingPattern::Tuple(patterns), ValueShape::Tuple(shapes)) => {
            crate::planner::pattern::validate_tuple_arity(shapes.len(), patterns.len())?;
            patterns
                .into_iter()
                .zip(shapes)
                .map(|(pattern, shape)| plan_total_binding_pattern(pattern, shape, context))
                .collect::<Result<Vec<_>, _>>()
                .map(TotalBindingPattern::tuple)
        }
        (BindingPattern::ListTail { tail, element_type }, expected @ ValueShape::List(_)) => {
            crate::planner::pattern::validate_pattern_value_type(
                ValueType::List(Box::new(element_type.clone())),
                expected.value_type(),
            )?;
            let tail = match tail {
                ListTailBinding::Named(name) => ListAssertTail::bind(
                    context.define_list_local(name.clone(), element_type.clone()),
                    name,
                ),
                ListTailBinding::Discard => ListAssertTail::Ignore,
            };
            Ok(TotalBindingPattern::list(element_type, tail))
        }
        (
            BindingPattern::Custom {
                source_shape,
                constructor_count,
                constructor,
                fields,
            },
            ValueShape::Custom(expected_shape),
        ) => {
            crate::planner::pattern::validate_pattern_value_type(
                ValueType::Custom(constructor.type_().clone()),
                ValueType::Custom(expected_shape.type_().clone()),
            )?;
            crate::planner::pattern::validate_constructor_arity(
                constructor.fields().len(),
                fields.len(),
            )?;
            let source_shape = refine_custom_binding_shape(expected_shape, source_shape)?;
            let fields = fields
                .into_iter()
                .zip(constructor.fields())
                .map(|(pattern, field)| {
                    plan_total_binding_pattern(
                        pattern,
                        ValueShape::from_value_type(field.type_().clone()),
                        context,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            total_custom_binding(source_shape, constructor_count, constructor, fields)
                .map(TotalBindingPattern::custom)
        }
        (BindingPattern::Alias { pattern, name }, expected) => {
            let pattern = plan_total_binding_pattern(*pattern, expected.clone(), context)?;
            let binding = AssertBinding::new(
                context.define_param_local_shape(name.clone(), expected.clone()),
                name,
                expected,
            );
            Ok(TotalBindingPattern::alias(pattern, binding))
        }
        (BindingPattern::Tuple(_), expected) => Err(binding_shape_mismatch(
            crate::planner::PatternKind::Tuple,
            &expected,
        )),
        (BindingPattern::ListTail { .. }, expected) => Err(binding_shape_mismatch(
            crate::planner::PatternKind::List,
            &expected,
        )),
        (BindingPattern::Custom { .. }, expected) => Err(binding_shape_mismatch(
            crate::planner::PatternKind::Constructor,
            &expected,
        )),
    }
}

fn total_custom_binding(
    source_shape: crate::plan::CustomValueShape,
    constructor_count: usize,
    constructor: CustomConstructor,
    fields: Vec<TotalBindingPattern>,
) -> Result<CustomBindingPattern, PlanError> {
    match source_shape.constructor() {
        crate::plan::CustomConstructorRefinement::Exact(index) if index == constructor.index() => {
            Ok(CustomBindingPattern::exact(
                source_shape,
                constructor,
                fields,
            ))
        }
        crate::plan::CustomConstructorRefinement::Any if constructor_count == 1 => Ok(
            CustomBindingPattern::only_constructor(source_shape, constructor, fields),
        ),
        crate::plan::CustomConstructorRefinement::Any
        | crate::plan::CustomConstructorRefinement::Exact(_) => {
            let actual = match source_shape.constructor() {
                crate::plan::CustomConstructorRefinement::Any => None,
                crate::plan::CustomConstructorRefinement::Exact(index) => Some(index),
            };
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason:
                        crate::planner::InvalidPatternShapeReason::BindingConstructorRefinement {
                            expected: constructor.index(),
                            actual,
                        },
                },
            })
        }
    }
}

fn refine_custom_binding_shape(
    expected: crate::plan::CustomValueShape,
    actual: crate::plan::CustomValueShape,
) -> Result<crate::plan::CustomValueShape, PlanError> {
    let type_ = ValueType::Custom(expected.type_().clone());
    match ValueShape::Custom(expected).refine(&ValueShape::Custom(actual)) {
        Some(ValueShape::Custom(shape)) => Ok(shape),
        Some(_) | None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::BindingShapeConflict { type_ },
            },
        }),
    }
}

fn binding_shape_mismatch(actual: crate::planner::PatternKind, expected: &ValueShape) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PatternShape {
            reason: crate::planner::InvalidPatternShapeReason::BindingShape {
                expected: expected.value_type(),
                actual,
            },
        },
    }
}

fn plan_tuple_assignment(
    elements: Vec<BindingPattern>,
    value: Expr,
    context: &mut PlanContext<'_>,
) -> Result<PlannedAssignment, PlanError> {
    let actual = value.value_type();
    let Some(value) = value.into_tuple() else {
        return Err(tuple_assignment_value_must_be_tuple(actual));
    };
    let shape = value.shape().to_vec().into_boxed_slice();
    let type_ = value.type_().to_vec();
    crate::planner::pattern::validate_tuple_arity(type_.len(), elements.len())?;

    let local = context.define_internal_tuple_local();
    let name = internal_tuple_name(local);
    let tuple_local = TupleExpr::local_get(local, name.clone(), type_.clone()).with_shape(shape);
    let mut steps = vec![Step::let_tuple(local, name, value)];

    for (index, (pattern, type_)) in elements.into_iter().zip(type_).enumerate() {
        let element = tuple_index_expr(tuple_local.clone(), index, type_)?;
        steps.extend(plan_assignment_steps(pattern, element, context)?);
    }

    Ok(PlannedAssignment {
        steps,
        value: Expr::tuple(tuple_local),
    })
}

fn internal_tuple_name(local: TupleLocalId) -> EcoString {
    format!("<tuple:{}>", local.0).into()
}

fn internal_custom_name(local: crate::plan::CustomLocalId) -> EcoString {
    format!("<custom:{}>", local.0).into()
}

fn list_assignment_value_must_be_list(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::List,
            actual: value_type_expression_type(actual),
        },
    }
}

fn custom_assignment_value_must_be_custom(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Custom,
            actual: value_type_expression_type(actual),
        },
    }
}

fn tuple_assignment_value_must_be_tuple(actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Tuple,
            actual: value_type_expression_type(actual),
        },
    }
}

fn value_type_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Parameter(_) => InvalidExpressionType::TypeParameter,
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::BitArray => InvalidExpressionType::BitArray,
        ValueType::UtfCodepoint => InvalidExpressionType::UtfCodepoint,
        ValueType::Custom(_) => InvalidExpressionType::Custom,
        ValueType::External(_) => InvalidExpressionType::External,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::List(_) => InvalidExpressionType::List,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

pub(in crate::planner) fn plan_variable_runtime_step(
    name: EcoString,
    value: crate::plan::Expr,
    context: &mut PlanContext<'_>,
) -> Step {
    plan_variable_runtime_step_and_return(name, value, context).0
}

fn plan_variable_runtime_step_and_return(
    name: EcoString,
    value: crate::plan::Expr,
    context: &mut PlanContext<'_>,
) -> (Step, Expr) {
    match value.into_kind() {
        ExprKind::Generic(value) => {
            let local = context.define_generic_local(name.clone(), value.parameter());
            (
                Step::let_generic(local, name.clone(), value),
                Expr::generic(crate::plan::GenericExpr::local_get(local, name)),
            )
        }
        ExprKind::Int(value) => {
            let local = context.define_int_local(name.clone());
            (
                Step::let_int(local, name.clone(), value),
                Expr::int(IntExpr::local_get(local, name)),
            )
        }
        ExprKind::String(value) => {
            let local = context.define_string_local(name.clone());
            (
                Step::let_string(local, name.clone(), value),
                Expr::string(StringExpr::local_get(local, name)),
            )
        }
        ExprKind::BitArray(value) => {
            let local = context.define_bit_array_local(name.clone());
            (
                Step::let_bit_array(local, name.clone(), value),
                Expr::bit_array(BitArrayExpr::local_get(local, name)),
            )
        }
        ExprKind::UtfCodepoint(value) => {
            let local = context.define_utf_codepoint_local(name.clone());
            (
                Step::let_utf_codepoint(local, name.clone(), value),
                Expr::utf_codepoint(UtfCodepointExpr::local_get(local, name)),
            )
        }
        ExprKind::Custom(value) => {
            let shape = value.shape().clone();
            let local = context.define_custom_local_shape(name.clone(), shape.clone());
            let typed_local = crate::plan::CustomLocal::from_shape(local, shape);
            (
                Step::let_custom(local, name.clone(), value),
                Expr::custom(CustomExpr::local_get(typed_local, name)),
            )
        }
        ExprKind::External(value) => {
            let shape = value.shape().clone();
            let local = context.define_external_local_shape(name.clone(), shape.clone());
            let typed_local = ExternalLocal::from_shape(local, shape);
            (
                Step::let_external(typed_local.clone(), name.clone(), value),
                Expr::external(ExternalExpr::local_get(typed_local, name)),
            )
        }
        ExprKind::Float(value) => {
            let local = context.define_float_local(name.clone());
            (
                Step::let_float(local, name.clone(), value),
                Expr::float(FloatExpr::local_get(local, name)),
            )
        }
        ExprKind::Bool(value) => {
            let local = context.define_bool_local(name.clone());
            (
                Step::let_bool(local, name.clone(), value),
                Expr::bool(BoolExpr::local_get(local, name)),
            )
        }
        ExprKind::Nil(value) => {
            let local = context.define_nil_local(name.clone());
            (
                Step::let_nil(local, name.clone(), value),
                Expr::nil(NilExpr::local_get(local, name)),
            )
        }
        ExprKind::Tuple(value) => {
            let shape = value.shape().to_vec().into_boxed_slice();
            let local = context.define_tuple_local_shape(name.clone(), shape.clone());
            let type_ = value.type_().to_vec();
            (
                Step::let_tuple(local, name.clone(), value),
                Expr::tuple(TupleExpr::local_get(local, name, type_).with_shape(shape)),
            )
        }
        ExprKind::List(value) => {
            let item_shape = value.item_shape().clone();
            let (local, value) = context.define_list_value(name.clone(), value);
            (
                Step::let_list_expr(name.clone(), value),
                Expr::list(ListExpr::local_get(local, name).with_item_shape(item_shape)),
            )
        }
        ExprKind::Function(value) => {
            let (step, expression) = match value.into_typed_kind() {
                TypedFunctionExprKind::Generic(value) => {
                    let shape = value.shape().clone();
                    let local = context.define_generic_function_local_shape(
                        name.clone(),
                        value.expression().type_().clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_generic_function_expr(local.clone(), name.clone(), value),
                        FunctionExpr::generic_with_shape(
                            crate::plan::GenericFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Int(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_int_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_int_function_expr(local, name.clone(), value),
                        FunctionExpr::int_with_shape(
                            IntFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::String(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_string_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_string_function_expr(local, name.clone(), value),
                        FunctionExpr::string_with_shape(
                            StringFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::BitArray(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_bit_array_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_bit_array_function_expr(local, name.clone(), value),
                        FunctionExpr::bit_array_with_shape(
                            BitArrayFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::UtfCodepoint(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_utf_codepoint_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_utf_codepoint_function_expr(local, name.clone(), value),
                        FunctionExpr::utf_codepoint_with_shape(
                            UtfCodepointFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Custom(value) => {
                    let shape = value.shape().clone();
                    let local = context.define_custom_function_local_shape(
                        name.clone(),
                        value.expression().custom_function_type().clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_custom_function_expr(local.id(), name.clone(), value),
                        FunctionExpr::custom(CustomFunctionExpr::local_get(local, name)),
                    )
                }
                TypedFunctionExprKind::External(value) => {
                    let shape = value.shape().clone();
                    let local = context.define_external_function_local_shape(
                        name.clone(),
                        value.expression().external_function_type().clone(),
                        shape,
                    );
                    (
                        Step::let_external_function_expr(local.id(), name.clone(), value),
                        FunctionExpr::external(ExternalFunctionExpr::local_get(local, name)),
                    )
                }
                TypedFunctionExprKind::Float(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_float_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_float_function_expr(local, name.clone(), value),
                        FunctionExpr::float_with_shape(
                            FloatFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Bool(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_bool_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_bool_function_expr(local, name.clone(), value),
                        FunctionExpr::bool_with_shape(
                            BoolFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Nil(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_nil_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_nil_function_expr(local, name.clone(), value),
                        FunctionExpr::nil_with_shape(
                            NilFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Tuple(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_tuple_function_local_shape(
                        name.clone(),
                        type_.clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_tuple_function_expr(local, name.clone(), value),
                        FunctionExpr::tuple_with_shape(
                            TupleFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::List(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_list_function_local_shape(
                        name.clone(),
                        type_,
                        value.expression().return_item_type(),
                        shape.clone(),
                    );
                    (
                        Step::let_list_function_expr(local.clone(), name.clone(), value),
                        FunctionExpr::list_with_shape(
                            ListFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Function(value) => {
                    let shape = value.shape().clone();
                    let local = context.define_function_function_local_shape(
                        name.clone(),
                        value.expression().function_function_type().clone(),
                        shape.clone(),
                    );
                    (
                        Step::let_function_function_expr(local.id(), name.clone(), value),
                        FunctionExpr::function_with_shape(
                            FunctionFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
            };
            (step, Expr::function(expression))
        }
    }
}

#[cfg(test)]
pub(super) fn plan_binding_pattern(pattern: TypedPattern) -> Result<BindingPattern, PlanError> {
    let module = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module, &functions, &mut anonymous);
    plan_binding_pattern_in_context(pattern, &context)
}

pub(super) fn plan_binding_pattern_in_context(
    pattern: TypedPattern,
    context: &PlanContext<'_>,
) -> Result<BindingPattern, PlanError> {
    plan_binding_pattern_in_context_with_alias(pattern, context, true)
}

pub(super) fn is_total_binding_pattern(
    pattern: &TypedPattern,
    context: &PlanContext<'_>,
) -> Result<bool, PlanError> {
    match pattern {
        Pattern::Variable { .. } | Pattern::Discard { .. } => Ok(true),
        Pattern::Assign { pattern, .. } => is_total_binding_pattern(pattern, context),
        Pattern::Tuple { elements, .. } => {
            for element in elements {
                if !is_total_binding_pattern(element, context)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Pattern::List { elements, tail, .. } => {
            if !elements.is_empty() {
                return Ok(false);
            }
            match tail.as_deref() {
                Some(gleam_core::ast::TailPattern {
                    pattern: Pattern::Variable { .. } | Pattern::Discard { .. },
                    ..
                }) => Ok(true),
                Some(_) | None => Ok(false),
            }
        }
        Pattern::BitArray { segments, .. } => Ok(is_total_bit_array_binding(segments)),
        Pattern::Constructor {
            arguments,
            constructor,
            type_,
            ..
        } if !type_.is_bool() && !type_.is_nil() => {
            let constructor = crate::planner::pattern::resolved_constructor(constructor)?;
            let field_types = arguments
                .iter()
                .map(|argument| {
                    crate::planner::pattern::pattern_value_shape(&argument.value, context)
                        .map(|shape| shape.value_type())
                })
                .collect::<Result<Vec<_>, _>>()?;
            let resolved =
                context.custom_pattern_constructor(type_.as_ref(), constructor, field_types)?;
            let source_is_exact = resolved.source_shape().constructor()
                == crate::plan::CustomConstructorRefinement::Exact(usize::from(
                    constructor.constructor_index,
                ));
            if !source_is_exact && resolved.constructor_count() != 1 {
                return Ok(false);
            }
            for argument in arguments {
                if !is_total_binding_pattern(&argument.value, context)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Constructor { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. } => Ok(false),
    }
}

fn plan_binding_pattern_in_context_with_alias(
    pattern: TypedPattern,
    context: &PlanContext<'_>,
    allow_alias: bool,
) -> Result<BindingPattern, PlanError> {
    let shape = crate::planner::pattern::pattern_value_shape(&pattern, context)?;
    match (pattern, shape) {
        (Pattern::Variable { name, .. }, _) => Ok(BindingPattern::Named(name)),
        (Pattern::Discard { .. }, _) => Ok(BindingPattern::Discard),
        (Pattern::Tuple { elements, .. }, _) => elements
            .into_iter()
            .map(|element| plan_binding_pattern_in_context_with_alias(element, context, true))
            .collect::<Result<Vec<_>, _>>()
            .map(BindingPattern::Tuple),
        (
            Pattern::List {
                location,
                elements,
                tail,
                type_,
            },
            shape,
        ) => plan_tail_only_list_binding_pattern_in_context(
            location, elements, tail, type_, shape, context,
        ),
        (Pattern::BitArray { location, segments }, shape) => {
            crate::planner::pattern::validate_pattern(
                &Pattern::BitArray {
                    location,
                    segments: segments.clone(),
                },
                &shape,
                context,
            )?;
            plan_total_bit_array_binding_pattern(segments)
        }
        (
            Pattern::Constructor {
                arguments,
                constructor,
                type_,
                ..
            },
            ValueShape::Custom(source_shape),
        ) if !type_.is_bool() && !type_.is_nil() => {
            let constructor = crate::planner::pattern::resolved_constructor(&constructor)?;
            let field_types = arguments
                .iter()
                .map(|argument| {
                    crate::planner::pattern::pattern_value_type_in_context(&argument.value, context)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let resolved =
                context.custom_pattern_constructor(type_.as_ref(), constructor, field_types)?;
            let constructor_count = resolved.constructor_count();
            let source_is_exact = resolved.source_shape().constructor()
                == crate::plan::CustomConstructorRefinement::Exact(usize::from(
                    constructor.constructor_index,
                ));
            let constructor_is_total = source_is_exact || constructor_count == 1;
            let fields_are_total = if constructor_is_total {
                let mut fields_are_total = true;
                for argument in &arguments {
                    fields_are_total &= is_total_binding_pattern(&argument.value, context)?;
                }
                fields_are_total
            } else {
                false
            };
            if !fields_are_total {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::PatternShape {
                        reason:
                            crate::planner::InvalidPatternShapeReason::RefutableBindingConstructor {
                                constructors: constructor_count,
                            },
                    },
                });
            }
            let constructor = resolved.into_constructor();
            let fields = arguments
                .into_iter()
                .map(|argument| {
                    plan_binding_pattern_in_context_with_alias(argument.value, context, true)
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(BindingPattern::Custom {
                source_shape,
                constructor_count,
                constructor,
                fields,
            })
        }
        (Pattern::Assign { name, pattern, .. }, _) if allow_alias => Ok(BindingPattern::Alias {
            pattern: Box::new(plan_binding_pattern_in_context_with_alias(
                *pattern, context, false,
            )?),
            name,
        }),
        (Pattern::Assign { .. }, _) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::NestedBindingAlias,
            },
        }),
        (pattern, shape) => {
            crate::planner::pattern::validate_pattern(&pattern, &shape, context)?;
            Err(non_variable_pattern_error(&pattern))
        }
    }
}

pub(super) fn non_variable_pattern_error(pattern: &TypedPattern) -> PlanError {
    match pattern {
        Pattern::List { .. } => PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::List,
        },
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArray { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Constructor { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }
        | Pattern::Discard { .. }
        | Pattern::Variable { .. }
        | Pattern::Assign { .. }
        | Pattern::Tuple { .. } => PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::pattern::pattern_kind(pattern),
                },
            },
        },
    }
}

fn plan_total_bit_array_binding_pattern(
    segments: Vec<
        gleam_core::ast::BitArraySegment<TypedPattern, std::sync::Arc<gleam_core::type_::Type>>,
    >,
) -> Result<BindingPattern, PlanError> {
    let segment = take_bit_array_binding_segment(segments)?;
    if !segment.type_.is_bit_array() {
        crate::planner::pattern::validate_pattern_value_type(
            ValueType::BitArray,
            crate::planner::pattern::pattern_value_type_from_gleam(segment.type_.as_ref())?,
        )?;
    }
    if segment.size().is_some() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentSize,
            },
        });
    }
    if !matches!(
        segment.options.as_slice(),
        [gleam_core::ast::BitArrayOption::Bits { .. }]
    ) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentOptions,
            },
        });
    }
    plan_total_bit_array_binding_value_pattern(*segment.value)
}

fn take_bit_array_binding_segment(
    segments: Vec<
        gleam_core::ast::BitArraySegment<TypedPattern, std::sync::Arc<gleam_core::type_::Type>>,
    >,
) -> Result<
    gleam_core::ast::BitArraySegment<TypedPattern, std::sync::Arc<gleam_core::type_::Type>>,
    PlanError,
> {
    let [segment] = match <[_; 1]>::try_from(segments) {
        Ok(segments) => segments,
        Err(segments) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason:
                        crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentCount {
                            actual: segments.len(),
                        },
                },
            });
        }
    };
    Ok(segment)
}

fn is_total_bit_array_binding(
    segments: &[gleam_core::ast::BitArraySegment<
        TypedPattern,
        std::sync::Arc<gleam_core::type_::Type>,
    >],
) -> bool {
    let [segment] = segments else {
        return false;
    };
    segment.type_.is_bit_array()
        && segment.size().is_none()
        && matches!(
            segment.options.as_slice(),
            [gleam_core::ast::BitArrayOption::Bits { .. }]
        )
        && is_total_bit_array_binding_value(segment.value.as_ref())
}

fn is_total_bit_array_binding_value(pattern: &TypedPattern) -> bool {
    match pattern {
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } => type_.is_bit_array(),
        Pattern::Assign { pattern, .. } => is_total_bit_array_binding_value(pattern),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. } => false,
    }
}

fn plan_total_bit_array_binding_value_pattern(
    pattern: TypedPattern,
) -> Result<BindingPattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if type_.is_bit_array() => {
            Ok(BindingPattern::Named(name))
        }
        Pattern::Discard { type_, .. } if type_.is_bit_array() => Ok(BindingPattern::Discard),
        Pattern::Assign { name, pattern, .. } => {
            plan_total_bit_array_binding_value_pattern(*pattern).map(|pattern| {
                BindingPattern::Alias {
                    pattern: Box::new(pattern),
                    name,
                }
            })
        }
        pattern => Err(non_variable_pattern_error(&pattern)),
    }
}

fn plan_tail_only_list_binding_pattern_in_context(
    location: gleam_core::ast::SrcSpan,
    elements: Vec<TypedPattern>,
    tail: Option<Box<gleam_core::ast::TailPattern<std::sync::Arc<gleam_core::type_::Type>>>>,
    type_: std::sync::Arc<gleam_core::type_::Type>,
    source_shape: ValueShape,
    context: &PlanContext<'_>,
) -> Result<BindingPattern, PlanError> {
    if !elements.is_empty() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape {
                reason: crate::planner::InvalidPatternShapeReason::ListBindingElements {
                    actual: elements.len(),
                },
            },
        });
    }

    let validated = crate::planner::pattern::validate_list_pattern(
        &Pattern::List {
            location,
            elements,
            tail,
            type_,
        },
        &source_shape,
        context,
    )?;
    let element_type = validated.item_shape.value_type();
    let tail = match validated.tail {
        Some(crate::planner::pattern::ValidatedListTail::Named(name)) => {
            ListTailBinding::Named(name)
        }
        Some(crate::planner::pattern::ValidatedListTail::Discard) => ListTailBinding::Discard,
        None => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::ListBindingTailMissing,
                },
            });
        }
    };

    Ok(BindingPattern::ListTail { tail, element_type })
}

#[cfg(test)]
fn pattern_shape_error(reason: crate::planner::InvalidPatternShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::PatternShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BindingPattern, ListTailBinding, is_total_binding_pattern,
        is_total_bit_array_binding_value, pattern_shape_error, plan_alias_assignment,
        plan_assignment_steps, plan_binding_pattern, plan_binding_pattern_in_context,
        plan_custom_assignment, plan_final_assignment,
        plan_tail_only_list_binding_pattern_in_context, plan_total_binding_pattern,
        plan_total_bit_array_binding_pattern, plan_tuple_assignment, total_custom_binding,
    };
    use crate::plan::{
        AssertBinding, BoolLocalId, CustomBindingPattern, CustomConstructor,
        CustomConstructorDefinition, CustomConstructorField, CustomExpr, CustomLocal,
        CustomLocalId, CustomType, CustomTypeDefinition, CustomTypeName, CustomTypePublicity,
        CustomValueShape, Expr, ExternalType, ExternalTypeName, FunctionType, IntListLocalId,
        IntLocalId, ListAssertTail, ListExpr, ListLocal, LocalId, NilLocalId, ParamLocal,
        StringLocalId, TotalBindingPattern, TypeParameterId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::dsl::{
        bool_, bool_case_int_function, bool_function_ref, equal, function, int, int_function_ref,
        let_bool_function_step, let_int_function_step, let_list_step, let_nil_function_step,
        let_string_function_step, let_tuple_step, list, local_bool, local_int, local_list,
        local_nil, local_string, local_tuple, module, nil_function_ref, string_function_ref, tuple,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionType, InvalidTypedAstReason, PlanError, UnsupportedPatternKind,
    };
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArrayOption, BitArraySegment, BitArraySize, Pattern,
        Statement, TailPattern, TypedAssignment, TypedExpr,
    };
    use gleam_core::exhaustiveness::CompiledCase;
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn total_binding_owner_preserves_recursive_custom_and_alias_shapes() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let constructor = custom_constructor("Boxed", vec![ValueType::Int]);
        let exact_shape = CustomValueShape::new(
            constructor.type_().type_name().clone(),
            Vec::new(),
            crate::plan::CustomConstructorRefinement::Exact(constructor.index()),
        );

        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: exact_shape.clone(),
                    constructor_count: 1,
                    constructor: constructor.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                ValueShape::Custom(exact_shape.clone()),
                &mut context,
            ),
            Ok(TotalBindingPattern::custom(CustomBindingPattern::exact(
                exact_shape,
                constructor.clone(),
                vec![TotalBindingPattern::discard(ValueType::Int)],
            ))),
        );

        let any_shape = CustomValueShape::any(constructor.type_().clone());
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: any_shape.clone(),
                    constructor_count: 1,
                    constructor: constructor.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                ValueShape::Custom(any_shape.clone()),
                &mut context,
            ),
            Ok(TotalBindingPattern::custom(
                CustomBindingPattern::only_constructor(
                    any_shape,
                    constructor,
                    vec![TotalBindingPattern::discard(ValueType::Int)],
                ),
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Alias {
                    pattern: Box::new(BindingPattern::Discard),
                    name: "alias".into(),
                },
                ValueShape::Int,
                &mut context,
            ),
            Ok(TotalBindingPattern::alias(
                TotalBindingPattern::discard(ValueType::Int),
                AssertBinding::new(
                    ParamLocal::int(IntLocalId(0)),
                    "alias".into(),
                    ValueShape::Int,
                ),
            )),
        );
    }

    #[test]
    fn total_binding_owner_preserves_tuple_and_list_tail_shapes() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Tuple(vec![
                    BindingPattern::Named("first".into()),
                    BindingPattern::Discard,
                ]),
                ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice()),
                &mut context,
            ),
            Ok(TotalBindingPattern::tuple(vec![
                TotalBindingPattern::bind(AssertBinding::new(
                    ParamLocal::int(IntLocalId(0)),
                    "first".into(),
                    ValueShape::Int,
                )),
                TotalBindingPattern::discard(ValueType::String),
            ])),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Named("rest".into()),
                    element_type: ValueType::Int,
                },
                ValueShape::List(Box::new(ValueShape::Int)),
                &mut context,
            ),
            Ok(TotalBindingPattern::list(
                ValueType::Int,
                ListAssertTail::bind(ListLocal::int(IntListLocalId(0)), "rest".into(),),
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Discard,
                    element_type: ValueType::String,
                },
                ValueShape::List(Box::new(ValueShape::String)),
                &mut context,
            ),
            Ok(TotalBindingPattern::list(
                ValueType::String,
                ListAssertTail::Ignore,
            )),
        );

        let total_list = Pattern::List {
            location: dummy_span(),
            elements: Vec::new(),
            tail: Some(Box::new(TailPattern {
                location: dummy_span(),
                pattern: Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::list(type_::int()),
                },
            })),
            type_: type_::list(type_::int()),
        };
        assert_eq!(is_total_binding_pattern(&total_list, &context), Ok(true));
        let non_total_list = Pattern::List {
            location: dummy_span(),
            elements: Vec::new(),
            tail: Some(Box::new(TailPattern {
                location: dummy_span(),
                pattern: Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
            })),
            type_: type_::list(type_::int()),
        };
        assert_eq!(
            is_total_binding_pattern(&non_total_list, &context),
            Ok(false),
        );
    }

    #[test]
    fn total_bit_array_binding_accepts_variables_discards_and_aliases() {
        let variable = Pattern::Variable {
            location: dummy_span(),
            name: "bits".into(),
            type_: type_::bit_array(),
            origin: VariableOrigin::generated(),
        };
        let discard = Pattern::Discard {
            location: dummy_span(),
            name: "_".into(),
            type_: type_::bit_array(),
        };
        let alias = Pattern::Assign {
            location: dummy_span(),
            name: "whole".into(),
            pattern: Box::new(discard.clone()),
        };

        assert!(is_total_bit_array_binding_value(&variable));
        assert!(is_total_bit_array_binding_value(&discard));
        assert!(is_total_bit_array_binding_value(&alias));
    }

    #[test]
    fn total_binding_owner_rejects_missing_contextual_list_and_bit_array_parts() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan_tail_only_list_binding_pattern_in_context(
                crate::planner::support::dummy_span(),
                Vec::new(),
                None,
                type_::list(type_::int()),
                ValueShape::List(Box::new(ValueShape::Int)),
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ListBindingTailMissing,
            )),
        );
        assert_eq!(
            plan_total_bit_array_binding_pattern(Vec::new()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentCount {
                    actual: 0,
                },
            )),
        );

        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Discard,
                    element_type: ValueType::String,
                },
                ValueShape::List(Box::new(ValueShape::Int)),
                &mut PlanContext::new(&module, &functions, &mut AnonymousFunctions::default(),),
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::String)),
                    actual: ValueType::List(Box::new(ValueType::Int)),
                },
            )),
        );

        let constructor = custom_constructor("Boxed", vec![ValueType::Int]);
        let source_shape = CustomValueShape::any(constructor.type_().clone());
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: source_shape.clone(),
                    constructor_count: 1,
                    constructor: constructor.clone(),
                    fields: Vec::new(),
                },
                ValueShape::Custom(source_shape),
                &mut PlanContext::new(&module, &functions, &mut AnonymousFunctions::default(),),
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ConstructorArity {
                    expected: 1,
                    actual: 0,
                },
            )),
        );

        let exact_other = CustomValueShape::new(
            constructor.type_().type_name().clone(),
            Vec::new(),
            crate::plan::CustomConstructorRefinement::Exact(1),
        );
        assert_eq!(
            total_custom_binding(
                exact_other,
                2,
                constructor.clone(),
                vec![TotalBindingPattern::discard(ValueType::Int)],
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingConstructorRefinement {
                    expected: constructor.index(),
                    actual: Some(1),
                },
            )),
        );

        let bit_array_discard = || Pattern::Discard {
            location: dummy_span(),
            name: "_".into(),
            type_: type_::bit_array(),
        };
        assert_eq!(
            plan_total_bit_array_binding_pattern(vec![BitArraySegment {
                location: dummy_span(),
                value: Box::new(bit_array_discard()),
                options: vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_: type_::int(),
            }]),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::BitArray,
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            plan_total_bit_array_binding_pattern(vec![BitArraySegment {
                location: dummy_span(),
                value: Box::new(bit_array_discard()),
                options: vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_: type_::generic_var(0),
            }]),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::UnsupportedType,
            )),
        );
        assert_eq!(
            plan_total_bit_array_binding_pattern(vec![BitArraySegment {
                location: dummy_span(),
                value: Box::new(bit_array_discard()),
                options: vec![
                    BitArrayOption::Bits {
                        location: dummy_span(),
                    },
                    BitArrayOption::Size {
                        location: dummy_span(),
                        value: Box::new(Pattern::BitArraySize(BitArraySize::Int {
                            location: dummy_span(),
                            value: "8".into(),
                            int_value: BigInt::from(8),
                        })),
                        short_form: false,
                    },
                ],
                type_: type_::bit_array(),
            }]),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentSize,
            )),
        );
        assert_eq!(
            plan_total_bit_array_binding_pattern(vec![BitArraySegment {
                location: dummy_span(),
                value: Box::new(Pattern::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                }),
                options: vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_: type_::bit_array(),
            }]),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::PatternKind::BitArray,
                },
            )),
        );
    }

    #[test]
    fn plan_let_and_integer_binop() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
  x + 2
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x").add_int(int(2))).let_int(0, "x", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_discard_assignment_evaluates_value() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ = 1
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(42)).evaluate(int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn custom_assignment_rejects_malformed_value_and_nested_binding_shapes() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let boxed = custom_constructor("Boxed", vec![ValueType::Int]);
        let boxed_shape = crate::plan::CustomValueShape::new(
            boxed.type_().type_name().clone(),
            Vec::new(),
            crate::plan::CustomConstructorRefinement::Exact(boxed.index()),
        );

        assert_eq!(
            plan_custom_assignment(
                1,
                boxed.clone(),
                vec![BindingPattern::Discard],
                Expr::from(int(1)),
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Custom,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        let other = custom_constructor("Other", Vec::new());
        assert_eq!(
            plan_custom_assignment(
                1,
                boxed.clone(),
                vec![BindingPattern::Discard],
                Expr::custom(
                    CustomExpr::try_constructor(other, Vec::new())
                        .expect("test custom construction should be valid"),
                ),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Custom(custom_type("Boxed")),
                    actual: ValueType::Custom(custom_type("Other")),
                },
            )),
        );
        assert_eq!(
            plan_custom_assignment(
                2,
                boxed.clone(),
                vec![BindingPattern::Discard],
                Expr::custom(CustomExpr::local_get(
                    CustomLocal::new(CustomLocalId(99), boxed.type_().clone()),
                    "boxed".into(),
                )),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingConstructorRefinement {
                    expected: 0,
                    actual: None,
                },
            )),
        );

        let invalid_custom_value = Expr::custom(
            CustomExpr::try_constructor(custom_constructor("Other", Vec::new()), Vec::new())
                .expect("test custom construction should be valid"),
        );
        assert_eq!(
            plan_assignment_steps(
                BindingPattern::Custom {
                    source_shape: boxed_shape.clone(),
                    constructor_count: 1,
                    constructor: boxed.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                invalid_custom_value.clone(),
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Custom(custom_type("Boxed")),
                    actual: ValueType::Custom(custom_type("Other")),
                },
            )),
        );
        assert_eq!(
            plan_alias_assignment(
                BindingPattern::Custom {
                    source_shape: boxed_shape.clone(),
                    constructor_count: 1,
                    constructor: boxed.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                "alias".into(),
                invalid_custom_value,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Custom(custom_type("Boxed")),
                    actual: ValueType::Custom(custom_type("Other")),
                },
            )),
        );

        let tuple_field =
            custom_constructor("TupleBox", vec![ValueType::Tuple(vec![ValueType::Int])]);
        assert_eq!(
            plan_custom_assignment(
                1,
                tuple_field.clone(),
                vec![BindingPattern::Tuple(vec![
                    BindingPattern::Discard,
                    BindingPattern::Discard,
                ])],
                Expr::custom(
                    CustomExpr::try_constructor(
                        tuple_field.clone(),
                        vec![Expr::tuple(crate::plan::TupleExpr::value(
                            vec![Expr::from(int(1))],
                            vec![ValueType::Int],
                        ))],
                    )
                    .expect("test custom construction should be valid"),
                ),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TupleArity {
                    expected: 1,
                    actual: 2,
                },
            )),
        );
        assert_eq!(
            plan_custom_assignment(
                1,
                boxed.clone(),
                Vec::new(),
                Expr::custom(
                    CustomExpr::try_constructor(boxed.clone(), vec![Expr::from(int(1))])
                        .expect("test custom construction should be valid"),
                ),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ConstructorArity {
                    expected: 1,
                    actual: 0,
                },
            )),
        );

        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Tuple(vec![BindingPattern::Discard]),
                ValueShape::Int,
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShape {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::Tuple,
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Tuple(vec![BindingPattern::Discard]),
                ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice()),
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TupleArity {
                    expected: 2,
                    actual: 1,
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Discard,
                    element_type: ValueType::Int,
                },
                ValueShape::String,
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShape {
                    expected: ValueType::String,
                    actual: crate::planner::PatternKind::List,
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: boxed_shape.clone(),
                    constructor_count: 1,
                    constructor: boxed.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                ValueShape::Custom(crate::plan::CustomValueShape::any(custom_type("Other"))),
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Custom(custom_type("Boxed")),
                    actual: ValueType::Custom(custom_type("Other")),
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: boxed_shape.clone(),
                    constructor_count: 1,
                    constructor: boxed.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                ValueShape::Int,
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShape {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::Constructor,
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: boxed_shape.clone(),
                    constructor_count: 2,
                    constructor: boxed.clone(),
                    fields: vec![BindingPattern::Discard],
                },
                ValueShape::Custom(CustomValueShape::new(
                    boxed.type_().type_name().clone(),
                    Vec::new(),
                    crate::plan::CustomConstructorRefinement::Exact(1),
                )),
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShapeConflict {
                    type_: ValueType::Custom(custom_type("Boxed")),
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Custom {
                    source_shape: boxed_shape,
                    constructor_count: 1,
                    constructor: boxed,
                    fields: vec![BindingPattern::Tuple(vec![BindingPattern::Discard])],
                },
                ValueShape::Custom(crate::plan::CustomValueShape::any(custom_type("Boxed"))),
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShape {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::Tuple,
                },
            )),
        );
        assert_eq!(
            plan_total_binding_pattern(
                BindingPattern::Alias {
                    pattern: Box::new(BindingPattern::Tuple(vec![BindingPattern::Discard])),
                    name: "alias".into(),
                },
                ValueShape::Int,
                &mut context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BindingShape {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::Tuple,
                },
            )),
        );
    }

    #[test]
    fn custom_binding_pattern_requires_known_total_constructor_metadata() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = dummy_span();
        let type_ = type_::named(
            "geam",
            "main",
            "Choice",
            gleam_core::ast::Publicity::Public,
            Vec::new(),
        );

        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Choice".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Unknown,
                    spread: None,
                    type_: type_.clone(),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        let unknown_result = || Pattern::Constructor {
            location: span,
            name_location: span,
            name: "Ok".into(),
            arguments: Vec::new(),
            module: None,
            constructor: Inferred::Unknown,
            spread: None,
            type_: type_::result(type_::int(), type_::string()),
        };
        assert_eq!(
            is_total_binding_pattern(
                &Pattern::Tuple {
                    location: span,
                    elements: vec![unknown_result()],
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        let result_constructor = || gleam_core::type_::PatternConstructor {
            name: "Ok".into(),
            field_map: None,
            documentation: None,
            module: "gleam".into(),
            location: span,
            constructor_index: 0,
        };
        let nested_result = type_::result(type_::int(), type_::string());
        let mut exact_outer_result = type_::result(nested_result.clone(), type_::string());
        std::sync::Arc::make_mut(&mut exact_outer_result).set_custom_type_variant(0);
        assert_eq!(
            is_total_binding_pattern(
                &Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::Constructor {
                            location: span,
                            name_location: span,
                            name: "Ok".into(),
                            arguments: Vec::new(),
                            module: None,
                            constructor: Inferred::Unknown,
                            spread: None,
                            type_: nested_result,
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(result_constructor()),
                    spread: None,
                    type_: exact_outer_result,
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        let nested_result = type_::result(type_::int(), type_::string());
        let mut exact_outer_result = type_::result(nested_result.clone(), type_::string());
        std::sync::Arc::make_mut(&mut exact_outer_result).set_custom_type_variant(0);
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::Constructor {
                            location: span,
                            name_location: span,
                            name: "Ok".into(),
                            arguments: Vec::new(),
                            module: None,
                            constructor: Inferred::Unknown,
                            spread: None,
                            type_: nested_result,
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(result_constructor()),
                    spread: None,
                    type_: exact_outer_result,
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        let mut exact_result = type_::result(type_::int(), type_::string());
        std::sync::Arc::make_mut(&mut exact_result).set_custom_type_variant(0);
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::Invalid {
                            location: span,
                            type_: type_::int(),
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(result_constructor()),
                    spread: None,
                    type_: exact_result.clone(),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::InvalidNode,
            )),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Known(result_constructor()),
                    spread: None,
                    type_: exact_result.clone(),
                },
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "gleam".into(),
                    name: "Result".into(),
                    reason: Box::new(crate::planner::InvalidCustomTypeReason::ConstructorArity {
                        expected: 1,
                        actual: 0,
                    }),
                },
            }),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::Assign {
                            location: span,
                            name: "outer".into(),
                            pattern: Box::new(Pattern::Assign {
                                location: span,
                                name: "inner".into(),
                                pattern: Box::new(Pattern::Discard {
                                    location: span,
                                    name: "_".into(),
                                    type_: type_::int(),
                                }),
                            }),
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(result_constructor()),
                    spread: None,
                    type_: exact_result,
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::NestedBindingAlias,
            )),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Invalid".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Known(gleam_core::type_::PatternConstructor {
                        name: "Invalid".into(),
                        field_map: None,
                        documentation: None,
                        module: "main".into(),
                        location: span,
                        constructor_index: 0,
                    }),
                    spread: None,
                    type_: type_::generic_var(0),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ConstructorType {
                    type_: ValueType::Parameter(TypeParameterId(0)),
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Choice".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Known(gleam_core::type_::PatternConstructor {
                        name: "Choice".into(),
                        field_map: None,
                        documentation: None,
                        module: "main".into(),
                        location: span,
                        constructor_index: 0,
                    }),
                    spread: None,
                    type_: type_.clone(),
                },
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Choice".into(),
                    reason: Box::new(crate::planner::InvalidCustomTypeReason::MissingDefinition),
                },
            }),
        );

        let mut inferred_type = type_.clone();
        std::sync::Arc::make_mut(&mut inferred_type).set_custom_type_variant(0);
        let pattern_constructor = || gleam_core::type_::PatternConstructor {
            name: "Choice".into(),
            field_map: None,
            documentation: None,
            module: "main".into(),
            location: span,
            constructor_index: 0,
        };
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Choice".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::BitArraySize(BitArraySize::Int {
                            location: span,
                            value: "1".into(),
                            int_value: BigInt::from(1),
                        }),
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(pattern_constructor()),
                    spread: None,
                    type_: inferred_type.clone(),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
            )),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Choice".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Known(pattern_constructor()),
                    spread: None,
                    type_: inferred_type,
                },
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Choice".into(),
                    reason: Box::new(crate::planner::InvalidCustomTypeReason::MissingDefinition),
                },
            }),
        );

        let mut result_type = type_::result(type_::int(), type_::string());
        std::sync::Arc::make_mut(&mut result_type).set_custom_type_variant(0);
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: vec![gleam_core::ast::CallArg {
                        label: None,
                        location: span,
                        value: Pattern::Int {
                            location: span,
                            value: "1".into(),
                            int_value: BigInt::from(1),
                        },
                        implicit: None,
                    }],
                    module: None,
                    constructor: Inferred::Known(gleam_core::type_::PatternConstructor {
                        name: "Ok".into(),
                        field_map: None,
                        documentation: None,
                        module: "gleam".into(),
                        location: span,
                        constructor_index: 0,
                    }),
                    spread: None,
                    type_: result_type,
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::RefutableBindingConstructor {
                    constructors: 2,
                },
            )),
        );

        let definitions = vec![CustomTypeDefinition::new(
            custom_type("Choice").type_name().clone(),
            CustomTypePublicity::Public,
            false,
            Vec::new(),
            vec![
                CustomConstructorDefinition::new("First".into(), 0, Vec::new()),
                CustomConstructorDefinition::new("Second".into(), 1, Vec::new()),
            ],
        )];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);
        let first_pattern = Pattern::Constructor {
            location: span,
            name_location: span,
            name: "First".into(),
            arguments: Vec::new(),
            module: None,
            constructor: Inferred::Known(gleam_core::type_::PatternConstructor {
                name: "First".into(),
                field_map: None,
                documentation: None,
                module: "main".into(),
                location: span,
                constructor_index: 0,
            }),
            spread: None,
            type_: type_.clone(),
        };
        assert_eq!(
            is_total_binding_pattern(&first_pattern, &context),
            Ok(false)
        );
        assert_eq!(
            plan_binding_pattern_in_context(first_pattern, &context),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::RefutableBindingConstructor {
                    constructors: 2,
                },
            )),
        );
    }

    fn custom_constructor(name: &str, fields: Vec<ValueType>) -> CustomConstructor {
        CustomConstructor::new(
            custom_type(name),
            name.into(),
            0,
            fields
                .into_iter()
                .map(|type_| CustomConstructorField::new(None, type_))
                .collect(),
        )
    }

    fn custom_type(name: &str) -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    fn external_type(name: &str) -> ExternalType {
        ExternalType::new(
            ExternalTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    #[test]
    fn plan_tuple_assignment_binds_projected_elements_from_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, two) = #(1, 2)
  one + two
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function("main", local_int(0, "one").add_int(local_int(1, "two")))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]).index_int(0),
                )
                .let_int(1, "two", tuple_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_tuple_assignment_discard_evaluates_projected_element_without_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(_, value) = #(1, 2)
  value
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function("main", local_int(0, "value"))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .evaluate(
                    local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]).index_int(0),
                )
                .let_int(0, "value", tuple_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_tail_assignment_binds_whole_list() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let [..rest] = [1, 2]
  rest
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_list(0, "rest", ValueType::Int)).step(let_list_step(
                0,
                "rest",
                list([int(1), int(2)], ValueType::Int),
            )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_tail_discard_assignment_evaluates_whole_list() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let [..] = [1]
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(42)).evaluate(list([int(1)], ValueType::Int)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_variable_alias_assignment_binds_inner_name_then_alias() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value as alias = 1
  value + alias
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "value").add_int(local_int(1, "alias")))
                .let_int(0, "value", int(1))
                .let_int(1, "alias", local_int(0, "value")),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_discard_alias_assignment_binds_alias_without_discard_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ as alias = 1
  alias
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "alias")).let_int(0, "alias", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_list_tail_alias_assignment_binds_tail_then_alias() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let [..rest] as values = [1]
  values
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_list(1, "values", ValueType::Int))
                .step(let_list_step(0, "rest", list([int(1)], ValueType::Int)))
                .step(let_list_step(
                    1,
                    "values",
                    local_list(0, "rest", ValueType::Int),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_nested_alias_pattern_is_invalid() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Assign {
                        location: dummy_span(),
                        name: "inner".into(),
                        pattern: Box::new(Pattern::Variable {
                            location: dummy_span(),
                            name: "value".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }),
                    }),
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment(
                    "value".into(),
                    type_::int(),
                ),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::NestedBindingAlias,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_alias_tuple_pattern_requires_tuple_value() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(Pattern::Tuple {
                        location: dummy_span(),
                        elements: vec![Pattern::Variable {
                            location: dummy_span(),
                            name: "value".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }],
                    }),
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment(
                    "value".into(),
                    type_::int(),
                ),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(vec![ValueType::Int]),
                    },
                },
            }),
        );
    }

    #[test]
    fn plan_tuple_alias_assignment_binds_projected_elements_and_alias_from_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, _) as pair = #(1, 2)
  one == pair.0
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let internal_tuple = local_tuple(0, "<tuple:0>", type_.clone());
        let alias_tuple = local_tuple(1, "pair", type_.clone());
        let expected = module(
            "main",
            function("main", equal(local_int(0, "one"), alias_tuple.index_int(0)))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", type_.clone()).index_int(0),
                )
                .evaluate(local_tuple(0, "<tuple:0>", type_.clone()).index_int(1))
                .step(let_tuple_step(1, "pair", internal_tuple)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_tuple_assignment_binds_nested_internal_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, #(two, three)) = #(1, #(2, 3))
  one + two + three
}
"#,
        ))
        .expect("source should plan");
        let outer_type = [
            ValueType::Int,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        ];
        let outer_local = local_tuple(0, "<tuple:0>", outer_type.clone());
        let inner_local = local_tuple(1, "<tuple:1>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "one")
                    .add_int(local_int(1, "two"))
                    .add_int(local_int(2, "three")),
            )
            .step(let_tuple_step(
                0,
                "<tuple:0>",
                tuple([
                    Expr::from(int(1)),
                    Expr::from(tuple([Expr::from(int(2)), Expr::from(int(3))])),
                ]),
            ))
            .let_int(
                0,
                "one",
                local_tuple(0, "<tuple:0>", outer_type).index_int(0),
            )
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                outer_local.index_tuple(1, [ValueType::Int, ValueType::Int]),
            ))
            .let_int(
                1,
                "two",
                local_tuple(1, "<tuple:1>", [ValueType::Int, ValueType::Int]).index_int(0),
            )
            .let_int(2, "three", inner_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_tuple_alias_assignment_binds_nested_aliases_in_step_order() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, #(two, _) as inner) as pair = #(1, #(2, 3))
  one + two + inner.0 + pair.0
}
"#,
        ))
        .expect("source should plan");
        let outer_type = [
            ValueType::Int,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Int]),
        ];
        let inner_type = [ValueType::Int, ValueType::Int];
        let outer_internal = local_tuple(0, "<tuple:0>", outer_type.clone());
        let inner_internal = local_tuple(1, "<tuple:1>", inner_type.clone());
        let inner_alias = local_tuple(2, "inner", inner_type.clone());
        let pair_alias = local_tuple(3, "pair", outer_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "one")
                    .add_int(local_int(1, "two"))
                    .add_int(inner_alias.index_int(0))
                    .add_int(pair_alias.index_int(0)),
            )
            .step(let_tuple_step(
                0,
                "<tuple:0>",
                tuple([
                    Expr::from(int(1)),
                    Expr::from(tuple([Expr::from(int(2)), Expr::from(int(3))])),
                ]),
            ))
            .let_int(
                0,
                "one",
                local_tuple(0, "<tuple:0>", outer_type.clone()).index_int(0),
            )
            .step(let_tuple_step(
                1,
                "<tuple:1>",
                outer_internal.index_tuple(1, inner_type.clone()),
            ))
            .let_int(
                1,
                "two",
                local_tuple(1, "<tuple:1>", inner_type.clone()).index_int(0),
            )
            .evaluate(local_tuple(1, "<tuple:1>", inner_type.clone()).index_int(1))
            .step(let_tuple_step(2, "inner", inner_internal))
            .step(let_tuple_step(
                3,
                "pair",
                local_tuple(0, "<tuple:0>", outer_type),
            )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_tuple_assignment_arity_mismatch() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![typed_int_expr(1)],
                },
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![
                        Pattern::Variable {
                            location: dummy_span(),
                            name: "one".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        },
                        Pattern::Variable {
                            location: dummy_span(),
                            name: "two".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        },
                    ],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TupleArity {
                    expected: 1,
                    actual: 2,
                },
            )),
        );

        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        assert_eq!(
            plan_tuple_assignment(
                vec![BindingPattern::Discard, BindingPattern::Discard],
                Expr::tuple(crate::plan::TupleExpr::value(
                    vec![Expr::from(int(1))],
                    vec![ValueType::Int],
                )),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TupleArity {
                    expected: 1,
                    actual: 2,
                },
            )),
        );
    }

    #[test]
    fn reject_margin_tuple_assignment_value_must_be_tuple() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Variable {
                        location: dummy_span(),
                        name: "one".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    }],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(vec![ValueType::Int]),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_nested_tuple_assignment_value_must_be_tuple() {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![typed_int_expr(1)],
                },
                pattern: Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![Pattern::Tuple {
                        location: dummy_span(),
                        elements: vec![Pattern::Variable {
                            location: dummy_span(),
                            name: "one".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }],
                    }],
                },
                kind: AssignmentKind::Let,
                compiled_case: CompiledCase::simple_variable_assignment("one".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::Int,
                        actual: ValueType::Tuple(vec![ValueType::Int]),
                    },
                },
            }),
        );
    }

    #[test]
    fn tuple_assignment_rejects_conflicting_element_shape() {
        let module = ecow::EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let value = Expr::tuple(
            crate::plan::TupleExpr::local_get(
                crate::plan::TupleLocalId(0),
                "pair".into(),
                vec![ValueType::Int],
            )
            .with_shape(vec![crate::plan::ValueShape::String].into_boxed_slice()),
        );

        assert_eq!(
            super::plan_tuple_assignment(vec![super::BindingPattern::Discard], value, &mut context)
                .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );

        let expected_tuple_error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Tuple,
                actual: InvalidExpressionType::Int,
            },
        };
        assert_eq!(
            super::plan_assignment_steps(
                BindingPattern::Tuple(Vec::new()),
                Expr::from(int(1)),
                &mut context,
            ),
            Err(expected_tuple_error.clone()),
        );
        assert_eq!(
            super::plan_alias_assignment(
                BindingPattern::Tuple(Vec::new()),
                "whole".into(),
                Expr::from(int(1)),
                &mut context,
            )
            .err(),
            Some(expected_tuple_error),
        );

        assert_eq!(
            super::plan_tuple_assignment(
                vec![BindingPattern::ListTail {
                    tail: ListTailBinding::Discard,
                    element_type: ValueType::Int,
                }],
                Expr::tuple(crate::plan::TupleExpr::value(
                    vec![Expr::from(int(1))],
                    vec![ValueType::Int],
                )),
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_tuple_assignment_value_type_error_preserves_actual_family() {
        let cases = [
            (
                ValueType::Parameter(TypeParameterId(0)),
                InvalidExpressionType::TypeParameter,
            ),
            (ValueType::Int, InvalidExpressionType::Int),
            (ValueType::String, InvalidExpressionType::String),
            (ValueType::BitArray, InvalidExpressionType::BitArray),
            (ValueType::UtfCodepoint, InvalidExpressionType::UtfCodepoint),
            (
                ValueType::Custom(custom_type("Boxed")),
                InvalidExpressionType::Custom,
            ),
            (
                ValueType::External(external_type("Token")),
                InvalidExpressionType::External,
            ),
            (ValueType::Float, InvalidExpressionType::Float),
            (ValueType::Bool, InvalidExpressionType::Bool),
            (ValueType::Nil, InvalidExpressionType::Nil),
            (
                ValueType::Tuple(vec![ValueType::Int]),
                InvalidExpressionType::Tuple,
            ),
            (
                ValueType::List(Box::new(ValueType::Int)),
                InvalidExpressionType::List,
            ),
            (
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
                InvalidExpressionType::Function,
            ),
        ];

        for (actual_type, actual) in cases {
            assert_eq!(
                super::tuple_assignment_value_must_be_tuple(actual_type),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual,
                    },
                },
            );
        }
    }

    #[test]
    fn reject_margin_list_tail_assignment_value_type_error_preserves_actual_family() {
        assert_eq!(
            super::list_assignment_value_must_be_list(ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            },
        );

        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();
        let mut context =
            crate::planner::context::PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_list_tail_assignment(
                ListTailBinding::Named("rest".into()),
                ValueType::Int,
                int(1).into(),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_tail_assignment_element_type_mismatch() {
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();
        let mut context =
            crate::planner::context::PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_list_tail_assignment(
                ListTailBinding::Named("rest".into()),
                ValueType::String,
                Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::List(Box::new(ValueType::String)),
                        actual: ValueType::List(Box::new(ValueType::Int)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_assignment_pattern_error_is_propagated() {
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();
        let mut context =
            crate::planner::context::PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assignment(
                TypedAssignment {
                    location: dummy_span(),
                    value: typed_int_expr(1),
                    pattern: Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                    kind: AssignmentKind::Let,
                    compiled_case: CompiledCase::simple_variable_assignment(
                        "value".into(),
                        type_::int(),
                    ),
                    annotation: None,
                },
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::BindingKind {
                        actual: crate::planner::PatternKind::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_tail_assignment_step_error_is_propagated() {
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();
        let mut context =
            crate::planner::context::PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_assignment_steps(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Named("rest".into()),
                    element_type: ValueType::String,
                },
                Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::List(Box::new(ValueType::String)),
                        actual: ValueType::List(Box::new(ValueType::Int)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_list_tail_alias_assignment_error_is_propagated() {
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();
        let mut context =
            crate::planner::context::PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_alias_assignment(
                BindingPattern::ListTail {
                    tail: ListTailBinding::Named("rest".into()),
                    element_type: ValueType::String,
                },
                "values".into(),
                Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::List(Box::new(ValueType::String)),
                        actual: ValueType::List(Box::new(ValueType::Int)),
                    },
                },
            }),
        );
        assert_eq!(
            super::plan_assignment_steps(
                BindingPattern::Alias {
                    pattern: Box::new(BindingPattern::Alias {
                        pattern: Box::new(BindingPattern::ListTail {
                            tail: ListTailBinding::Named("rest".into()),
                            element_type: ValueType::String,
                        }),
                        name: "inner".into(),
                    }),
                    name: "values".into(),
                },
                Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                &mut context,
            )
            .err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::TypeMismatch {
                        expected: ValueType::List(Box::new(ValueType::String)),
                        actual: ValueType::List(Box::new(ValueType::Int)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_profile_discard_assignment_value_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let _ = {
    <<1:native>>
    1
  }
  42
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn plan_function_valued_assignment() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

pub fn main() {
  let function = case True {
    True -> add_one
    False -> add_one
  }
  let string = string_identity
  let bool = bool_identity
  let nil = nil_identity
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1))
                .step(let_int_function_step(
                    0,
                    "function",
                    bool_case_int_function(
                        bool_(true),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ),
                ))
                .step(let_string_function_step(
                    0,
                    "string",
                    string_function_ref(2, [LocalId::String(StringLocalId(0))]),
                ))
                .step(let_bool_function_step(
                    0,
                    "bool",
                    bool_function_ref(3, [LocalId::Bool(BoolLocalId(0))]),
                ))
                .step(let_nil_function_step(
                    0,
                    "nil",
                    nil_function_ref(4, [LocalId::Nil(NilLocalId(0))]),
                )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function("bool_identity", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_assignment_returns_assigned_value_from_binding_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x")).let_int(0, "x", int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_discard_assignment_returns_assigned_value_without_binding_step() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let _ = 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_tuple_assignment_returns_internal_tuple_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, two) = #(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let tuple_local = local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]);
        let expected = module(
            "main",
            function(
                "main",
                local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]),
            )
            .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
            .let_int(
                0,
                "one",
                local_tuple(0, "<tuple:0>", [ValueType::Int, ValueType::Int]).index_int(0),
            )
            .let_int(1, "two", tuple_local.index_int(1)),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_pattern_alias_assignment_returns_alias_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let #(one, _) as pair = #(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let type_ = [ValueType::Int, ValueType::Int];
        let expected = module(
            "main",
            function("main", local_tuple(1, "pair", type_.clone()))
                .step(let_tuple_step(0, "<tuple:0>", tuple([int(1), int(2)])))
                .let_int(
                    0,
                    "one",
                    local_tuple(0, "<tuple:0>", type_.clone()).index_int(0),
                )
                .evaluate(local_tuple(0, "<tuple:0>", type_.clone()).index_int(1))
                .step(let_tuple_step(
                    1,
                    "pair",
                    local_tuple(0, "<tuple:0>", type_),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_list_tail_assignment_returns_list_local() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let [..rest] = [1]
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_list(0, "rest", ValueType::Int)).step(let_list_step(
                0,
                "rest",
                list([int(1)], ValueType::Int),
            )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_final_assignment_value_is_validated() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let x = {
    <<1:native>>
    1
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
    fn final_assignment_validates_the_pattern_against_the_value_shape() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            plan_final_assignment(
                TypedAssignment {
                    location: dummy_span(),
                    value: typed_int_expr(1),
                    pattern: Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::string(),
                    },
                    kind: AssignmentKind::Let,
                    compiled_case: CompiledCase::simple_variable_assignment(
                        "_".into(),
                        type_::string(),
                    ),
                    annotation: None,
                },
                &mut context,
            )
            .err(),
            Some(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );
    }

    #[test]
    fn reject_margin_generated_assignment() {
        let mut generated = compile_minimal_module();
        generated.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "x".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                kind: AssignmentKind::Generated,
                compiled_case: CompiledCase::simple_variable_assignment("x".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];
        assert_eq!(
            plan_module(generated),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            }),
        );

        let mut final_generated = compile_minimal_module();
        final_generated.definitions.functions[0].body =
            vec![Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "x".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                kind: AssignmentKind::Generated,
                compiled_case: CompiledCase::simple_variable_assignment("x".into(), type_::int()),
                annotation: None,
            }))];
        assert_eq!(
            plan_module(final_generated),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            }),
        );
    }

    #[test]
    fn plan_binding_pattern_accepts_supported_shapes() {
        let variable = |name: &str| Pattern::Variable {
            location: dummy_span(),
            name: name.into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };

        assert_eq!(
            plan_binding_pattern(variable("x")),
            Ok(BindingPattern::Named("x".into())),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_::int(),
            }),
            Ok(BindingPattern::Discard),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Tuple {
                location: dummy_span(),
                elements: vec![variable("x"), variable("y")],
            }),
            Ok(BindingPattern::Tuple(vec![
                BindingPattern::Named("x".into()),
                BindingPattern::Named("y".into()),
            ])),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(variable("x")),
            }),
            Ok(BindingPattern::Alias {
                pattern: Box::new(BindingPattern::Named("x".into())),
                name: "alias".into(),
            }),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Variable {
                        location: dummy_span(),
                        name: "rest".into(),
                        type_: type_::list(type_::int()),
                        origin: VariableOrigin::generated(),
                    },
                })),
                type_: type_::list(type_::int()),
            }),
            Ok(BindingPattern::ListTail {
                tail: ListTailBinding::Named("rest".into()),
                element_type: ValueType::Int,
            }),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::list(type_::int()),
                    },
                })),
                type_: type_::list(type_::int()),
            }),
            Ok(BindingPattern::ListTail {
                tail: ListTailBinding::Discard,
                element_type: ValueType::Int,
            }),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::BitArray {
                location: dummy_span(),
                segments: vec![BitArraySegment {
                    location: dummy_span(),
                    value: Box::new(Pattern::Variable {
                        location: dummy_span(),
                        name: "bits".into(),
                        type_: type_::bit_array(),
                        origin: VariableOrigin::generated(),
                    }),
                    options: vec![BitArrayOption::Bits {
                        location: dummy_span(),
                    }],
                    type_: type_::bit_array(),
                }],
            }),
            Ok(BindingPattern::Named("bits".into())),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::BitArray {
                location: dummy_span(),
                segments: vec![BitArraySegment {
                    location: dummy_span(),
                    value: Box::new(Pattern::Assign {
                        location: dummy_span(),
                        name: "alias".into(),
                        pattern: Box::new(Pattern::Discard {
                            location: dummy_span(),
                            name: "_".into(),
                            type_: type_::bit_array(),
                        }),
                    }),
                    options: vec![BitArrayOption::Bits {
                        location: dummy_span(),
                    }],
                    type_: type_::bit_array(),
                }],
            }),
            Ok(BindingPattern::Alias {
                pattern: Box::new(BindingPattern::Discard),
                name: "alias".into(),
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_pattern_shapes() {
        let variable = |name: &str| Pattern::Variable {
            location: dummy_span(),
            name: name.into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };

        assert_eq!(
            plan_binding_pattern(Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(Pattern::List {
                    location: dummy_span(),
                    elements: vec![variable("x")],
                    tail: None,
                    type_: type_::list(type_::int()),
                }),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ListBindingElements { actual: 1 },
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Variable {
                        location: dummy_span(),
                        name: "rest".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    },
                })),
                type_: type_::list(type_::int()),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::int(),
                    },
                })),
                type_: type_::list(type_::int()),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::Int,
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: None,
                type_: type_::list(type_::int()),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ListBindingTailMissing,
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Variable {
                        location: dummy_span(),
                        name: "rest".into(),
                        type_: type_::list(type_::int()),
                        origin: VariableOrigin::generated(),
                    },
                })),
                type_: type_::int(),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::List,
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: dummy_span(),
                    pattern: Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                })),
                type_: type_::list(type_::int()),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ListTailKind {
                    actual: crate::planner::PatternKind::Int,
                },
            )),
        );
        let patterns = [
            (
                Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
                crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::PatternKind::Int,
                },
            ),
            (
                Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::PatternKind::Float,
                },
            ),
            (
                Pattern::String {
                    location: dummy_span(),
                    value: "a".into(),
                },
                crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::PatternKind::String,
                },
            ),
            (
                Pattern::BitArraySize(BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                crate::planner::InvalidPatternShapeReason::BitArraySizeNode,
            ),
            (
                Pattern::Constructor {
                    location: dummy_span(),
                    name_location: dummy_span(),
                    name: "Boxed".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Unknown,
                    spread: None,
                    type_: type_::int(),
                },
                crate::planner::InvalidPatternShapeReason::UnresolvedConstructor,
            ),
            (
                Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "pre".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                },
                crate::planner::InvalidPatternShapeReason::BindingKind {
                    actual: crate::planner::PatternKind::StringPrefix,
                },
            ),
            (
                Pattern::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                crate::planner::InvalidPatternShapeReason::InvalidNode,
            ),
        ];

        for (pattern, reason) in patterns {
            assert_eq!(
                plan_binding_pattern(pattern),
                Err(pattern_shape_error(reason)),
            );
        }
        assert_eq!(
            plan_binding_pattern(Pattern::BitArray {
                location: dummy_span(),
                segments: Vec::new(),
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentCount {
                    actual: 0,
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern(Pattern::BitArray {
                location: dummy_span(),
                segments: vec![BitArraySegment {
                    location: dummy_span(),
                    value: Box::new(Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::bit_array(),
                    }),
                    options: vec![
                        BitArrayOption::Bits {
                            location: dummy_span(),
                        },
                        BitArrayOption::Bytes {
                            location: dummy_span(),
                        },
                    ],
                    type_: type_::bit_array(),
                }],
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::BitArrayBindingSegmentOptions,
            )),
        );
        for (segment_type, binding_type, reason) in [
            (
                type_::int(),
                type_::bit_array(),
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::BitArray,
                },
            ),
            (
                type_::bit_array(),
                type_::int(),
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::BitArray,
                    actual: ValueType::Int,
                },
            ),
        ] {
            assert_eq!(
                plan_binding_pattern(Pattern::BitArray {
                    location: dummy_span(),
                    segments: vec![BitArraySegment {
                        location: dummy_span(),
                        value: Box::new(Pattern::Variable {
                            location: dummy_span(),
                            name: "bits".into(),
                            type_: binding_type,
                            origin: VariableOrigin::generated(),
                        }),
                        options: vec![BitArrayOption::Bits {
                            location: dummy_span(),
                        }],
                        type_: segment_type,
                    }],
                }),
                Err(pattern_shape_error(reason)),
            );
        }
        assert_eq!(
            plan_binding_pattern(Pattern::BitArray {
                location: dummy_span(),
                segments: vec![BitArraySegment {
                    location: dummy_span(),
                    value: Box::new(Pattern::Assign {
                        location: dummy_span(),
                        name: "alias".into(),
                        pattern: Box::new(Pattern::Variable {
                            location: dummy_span(),
                            name: "bits".into(),
                            type_: type_::int(),
                            origin: VariableOrigin::generated(),
                        }),
                    }),
                    options: vec![BitArrayOption::Bits {
                        location: dummy_span(),
                    }],
                    type_: type_::bit_array(),
                }],
            }),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::BitArray,
                    actual: ValueType::Int,
                },
            )),
        );
    }

    #[test]
    fn reject_margin_contextual_list_binding_shapes() {
        let module = "main".into();
        let functions = HashMap::<ecow::EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let tail = |pattern| {
            Some(Box::new(TailPattern {
                location: dummy_span(),
                pattern,
            }))
        };

        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: tail(Pattern::Discard {
                        location: dummy_span(),
                        name: "_".into(),
                        type_: type_::int(),
                    }),
                    type_: type_::int(),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Int,
                    actual: crate::planner::PatternKind::List,
                },
            )),
        );
        assert_eq!(
            plan_binding_pattern_in_context(
                Pattern::List {
                    location: dummy_span(),
                    elements: Vec::new(),
                    tail: tail(Pattern::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    }),
                    type_: type_::list(type_::int()),
                },
                &context,
            ),
            Err(pattern_shape_error(
                crate::planner::InvalidPatternShapeReason::ListTailKind {
                    actual: crate::planner::PatternKind::Int,
                },
            )),
        );
    }

    #[test]
    fn tail_only_list_binding_preserves_parameter_element_type() {
        let list_type = type_::list(type_::generic_var(0));
        let tail = TailPattern {
            location: dummy_span(),
            pattern: Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: list_type.clone(),
            },
        };

        assert_eq!(
            plan_binding_pattern(Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: Some(Box::new(tail)),
                type_: list_type,
            }),
            Ok(BindingPattern::ListTail {
                tail: ListTailBinding::Discard,
                element_type: ValueType::Parameter(crate::plan::TypeParameterId(0)),
            }),
        );
    }

    #[test]
    fn non_variable_pattern_error_reports_list_profile_boundary() {
        assert_eq!(
            super::non_variable_pattern_error(&Pattern::List {
                location: dummy_span(),
                elements: Vec::new(),
                tail: None,
                type_: type_::list(type_::int()),
            }),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::List,
            },
        );
    }

    fn typed_int_expr(value: i64) -> TypedExpr {
        TypedExpr::Int {
            location: dummy_span(),
            type_: type_::int(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }
}
