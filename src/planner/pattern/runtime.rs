use crate::plan::{
    AssertBinding, AssertPattern, BitArrayBindingPattern, BitArrayPatternSegment,
    CustomBindingPattern, CustomConstructor, CustomConstructorRefinement, CustomPattern,
    CustomValueShape, ListAssertPattern, ListAssertTail, ParamLocal, TotalBindingPattern,
    ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_core::analyse::Inferred;
use gleam_core::ast::{AssignName, Pattern, TailPattern, TypedPattern};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(in crate::planner) struct PlannedRuntimePattern {
    pub(in crate::planner) pattern: AssertPattern,
    pub(in crate::planner) is_total: bool,
    pub(in crate::planner) total_binding: Option<TotalBindingPattern>,
    pub(in crate::planner) custom_binding: Option<PlannedCustomBinding>,
}

#[derive(Clone)]
pub(in crate::planner) struct PlannedCustomBinding {
    constructor: CustomConstructor,
    fields: Vec<TotalBindingPattern>,
    source_shape: CustomValueShape,
    constructor_count: usize,
}

impl PlannedCustomBinding {
    pub(in crate::planner) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    pub(in crate::planner) fn constructor_count(&self) -> usize {
        self.constructor_count
    }

    pub(in crate::planner) fn intrinsic_binding(&self) -> Option<CustomBindingPattern> {
        match self.source_shape.constructor() {
            CustomConstructorRefinement::Exact(index) if index == self.constructor.index() => {
                Some(CustomBindingPattern::exact(
                    self.source_shape.clone(),
                    self.constructor.clone(),
                    self.fields.clone(),
                ))
            }
            CustomConstructorRefinement::Any if self.constructor_count == 1 => {
                Some(CustomBindingPattern::only_constructor(
                    self.source_shape.clone(),
                    self.constructor.clone(),
                    self.fields.clone(),
                ))
            }
            CustomConstructorRefinement::Any | CustomConstructorRefinement::Exact(_) => None,
        }
    }

    pub(in crate::planner) fn into_intrinsic_binding(self) -> Option<CustomBindingPattern> {
        match self.source_shape.constructor() {
            CustomConstructorRefinement::Exact(index) if index == self.constructor.index() => Some(
                CustomBindingPattern::exact(self.source_shape, self.constructor, self.fields),
            ),
            CustomConstructorRefinement::Any if self.constructor_count == 1 => {
                Some(CustomBindingPattern::only_constructor(
                    self.source_shape,
                    self.constructor,
                    self.fields,
                ))
            }
            CustomConstructorRefinement::Any | CustomConstructorRefinement::Exact(_) => None,
        }
    }

    pub(in crate::planner) fn into_remainder_binding(
        self,
        excluded: Vec<usize>,
    ) -> CustomBindingPattern {
        CustomBindingPattern::exhaustive_remainder(
            self.source_shape,
            excluded,
            self.constructor,
            self.fields,
        )
    }
}

pub(in crate::planner) fn plan_runtime_pattern(
    pattern: TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } => {
            let binding = define_binding(name, type_.as_ref(), context);
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::Bind(binding.clone()),
                is_total: true,
                total_binding: Some(TotalBindingPattern::bind(binding)),
                custom_binding: None,
            })
        }
        Pattern::Discard { type_, .. } => {
            let type_ = context.value_type(type_.as_ref());
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::Discard,
                is_total: true,
                total_binding: Some(TotalBindingPattern::discard(type_)),
                custom_binding: None,
            })
        }
        Pattern::Int { int_value, .. } => Ok(PlannedRuntimePattern {
            pattern: AssertPattern::Int(int_value),
            is_total: false,
            total_binding: None,
            custom_binding: None,
        }),
        Pattern::Float { float_value, .. } => Ok(PlannedRuntimePattern {
            pattern: AssertPattern::Float(float_value.value()),
            is_total: false,
            total_binding: None,
            custom_binding: None,
        }),
        Pattern::String { value, .. } => Ok(PlannedRuntimePattern {
            pattern: AssertPattern::String(value),
            is_total: false,
            total_binding: None,
            custom_binding: None,
        }),
        Pattern::Tuple { elements, .. } => {
            let mut patterns = Vec::with_capacity(elements.len());
            let mut bindings = Vec::with_capacity(elements.len());
            let mut is_total = true;
            for element in elements {
                let element = plan_runtime_pattern(element, context)?;
                is_total &= element.is_total;
                if let Some(binding) = element.total_binding {
                    bindings.push(binding);
                }
                patterns.push(element.pattern);
            }
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::Tuple(patterns),
                is_total,
                total_binding: is_total.then(|| TotalBindingPattern::tuple(bindings)),
                custom_binding: None,
            })
        }
        Pattern::List {
            elements,
            tail,
            type_,
            ..
        } => plan_list_pattern(elements, tail.map(|tail| *tail), type_, context),
        Pattern::BitArray { segments, .. } => {
            let (pattern, is_total) = super::plan_bit_array_pattern(segments, context)?;
            Ok(PlannedRuntimePattern {
                total_binding: if is_total {
                    total_bit_array_binding(&pattern)
                } else {
                    None
                },
                pattern: AssertPattern::bit_array(pattern),
                is_total,
                custom_binding: None,
            })
        }
        Pattern::Constructor {
            arguments,
            constructor,
            type_,
            ..
        } if type_.is_bool() => plan_bool_pattern(constructor, arguments),
        Pattern::Constructor {
            arguments,
            constructor,
            type_,
            ..
        } if type_.is_nil() => plan_nil_pattern(constructor, arguments),
        Pattern::Constructor {
            arguments,
            constructor,
            type_,
            ..
        } => {
            let ValueShape::Custom(source_shape) = context.value_shape(type_.as_ref()) else {
                return Err(invalid_pattern());
            };
            plan_custom_pattern(arguments, constructor, type_, source_shape, context)
        }
        Pattern::StringPrefix {
            left_side_string,
            left_side_assignment,
            right_side_assignment,
            ..
        } => {
            let left = left_side_assignment.map(|(name, _)| define_string_binding(name, context));
            let right = match right_side_assignment {
                AssignName::Variable(name) => Some(define_string_binding(name, context)),
                AssignName::Discard(_) => None,
            };
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::StringPrefix {
                    prefix: left_side_string,
                    left,
                    right,
                },
                is_total: false,
                total_binding: None,
                custom_binding: None,
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let shape = pattern_value_shape(&pattern, context)?;
            let planned = plan_runtime_pattern(*pattern, context)?;
            let binding = define_value_binding(name, shape, context);
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::alias(planned.pattern, binding.clone()),
                is_total: planned.is_total,
                total_binding: planned
                    .total_binding
                    .map(|pattern| TotalBindingPattern::alias(pattern, binding)),
                custom_binding: planned.custom_binding,
            })
        }
        Pattern::BitArraySize(_) | Pattern::Invalid { .. } => Err(invalid_pattern()),
    }
}

pub(in crate::planner) fn plan_custom_subject_pattern(
    pattern: TypedPattern,
    source_shape: CustomValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let Pattern::Constructor {
        arguments,
        constructor,
        type_,
        ..
    } = pattern
    else {
        return Err(invalid_pattern());
    };
    plan_custom_pattern(arguments, constructor, type_, source_shape, context)
}

fn plan_list_pattern(
    elements: Vec<TypedPattern>,
    tail: Option<TailPattern<Arc<Type>>>,
    type_: Arc<Type>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let ValueShape::List(item_shape) = context.value_shape(type_.as_ref()) else {
        return Err(invalid_pattern());
    };
    let element_type = item_shape.value_type();
    let has_no_elements = elements.is_empty();
    let mut patterns = Vec::with_capacity(elements.len());
    for element in elements {
        patterns.push(plan_runtime_pattern(element, context)?.pattern);
    }
    let tail = tail
        .map(|tail| plan_list_tail(tail, item_shape.as_ref(), context))
        .transpose()?;
    let is_total = has_no_elements && tail.is_some();
    let total_binding = if is_total {
        tail.clone()
            .map(|tail| TotalBindingPattern::list(element_type.clone(), tail))
    } else {
        None
    };
    Ok(PlannedRuntimePattern {
        pattern: AssertPattern::list(ListAssertPattern::new(element_type, patterns, tail)),
        is_total,
        total_binding,
        custom_binding: None,
    })
}

fn plan_list_tail(
    tail: TailPattern<Arc<Type>>,
    item_shape: &ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<ListAssertTail, PlanError> {
    match tail.pattern {
        Pattern::Variable { name, type_, .. }
            if context.value_shape(type_.as_ref())
                == ValueShape::List(Box::new(item_shape.clone())) =>
        {
            Ok(ListAssertTail::bind(
                context.define_list_local_shape(name.clone(), item_shape.clone()),
                name,
            ))
        }
        Pattern::Discard { type_, .. }
            if context.value_shape(type_.as_ref())
                == ValueShape::List(Box::new(item_shape.clone())) =>
        {
            Ok(ListAssertTail::Ignore)
        }
        _ => Err(invalid_pattern()),
    }
}

fn plan_bool_pattern(
    constructor: Inferred<gleam_core::type_::PatternConstructor>,
    arguments: Vec<gleam_core::ast::CallArg<TypedPattern>>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let Inferred::Known(constructor) = constructor else {
        return Err(invalid_pattern());
    };
    if !arguments.is_empty() {
        return Err(invalid_pattern());
    }
    let value = match constructor.name.as_str() {
        "True" => true,
        "False" => false,
        _ => return Err(invalid_pattern()),
    };
    Ok(PlannedRuntimePattern {
        pattern: AssertPattern::Bool(value),
        is_total: false,
        total_binding: None,
        custom_binding: None,
    })
}

fn plan_nil_pattern(
    constructor: Inferred<gleam_core::type_::PatternConstructor>,
    arguments: Vec<gleam_core::ast::CallArg<TypedPattern>>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let Inferred::Known(constructor) = constructor else {
        return Err(invalid_pattern());
    };
    if constructor.name != "Nil" || !arguments.is_empty() {
        return Err(invalid_pattern());
    }
    Ok(PlannedRuntimePattern {
        pattern: AssertPattern::Nil,
        is_total: true,
        total_binding: Some(TotalBindingPattern::discard(ValueType::Nil)),
        custom_binding: None,
    })
}

fn plan_custom_pattern(
    arguments: Vec<gleam_core::ast::CallArg<TypedPattern>>,
    constructor: Inferred<gleam_core::type_::PatternConstructor>,
    type_: Arc<Type>,
    source_shape: CustomValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let Inferred::Known(constructor) = constructor else {
        return Err(invalid_pattern());
    };
    let field_types = arguments
        .iter()
        .map(|argument| pattern_value_type(&argument.value, context))
        .collect::<Result<Vec<_>, _>>()?;
    let resolved_constructor =
        context.custom_pattern_constructor(type_.as_ref(), &constructor, field_types)?;
    let constructor_count = resolved_constructor.constructor_count();
    let custom_constructor = resolved_constructor.into_constructor();
    let mut fields = Vec::with_capacity(arguments.len());
    let mut total_fields = Vec::with_capacity(arguments.len());
    for argument in arguments {
        let field = plan_runtime_pattern(argument.value, context)?;
        if let Some(binding) = field.total_binding {
            total_fields.push(binding);
        }
        fields.push(field.pattern);
    }
    let fields_are_total = total_fields.len() == fields.len();
    let matches_exact_constructor = source_shape.constructor()
        == CustomConstructorRefinement::Exact(usize::from(constructor.constructor_index));
    let is_total = fields_are_total && (constructor_count == 1 || matches_exact_constructor);
    let custom_binding = fields_are_total.then(|| PlannedCustomBinding {
        constructor: custom_constructor.clone(),
        fields: total_fields,
        source_shape,
        constructor_count,
    });
    Ok(PlannedRuntimePattern {
        pattern: AssertPattern::custom(CustomPattern::new(custom_constructor, fields)),
        is_total,
        total_binding: if is_total {
            custom_binding
                .as_ref()
                .and_then(PlannedCustomBinding::intrinsic_binding)
                .map(TotalBindingPattern::custom)
        } else {
            None
        },
        custom_binding,
    })
}

fn total_bit_array_binding(pattern: &crate::plan::BitArrayPattern) -> Option<TotalBindingPattern> {
    let [
        BitArrayPatternSegment::Bits {
            pattern,
            size: None,
            ..
        },
    ] = pattern.segments()
    else {
        return None;
    };
    Some(total_bits_binding(pattern))
}

fn total_bits_binding(
    pattern: &BitArrayBindingPattern<crate::plan::BitArrayLocalId>,
) -> TotalBindingPattern {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => {
            let (local, name) = binding.clone().into_parts();
            TotalBindingPattern::bind(AssertBinding::new(
                ParamLocal::bit_array(local),
                name,
                ValueShape::BitArray,
            ))
        }
        BitArrayBindingPattern::Discard => TotalBindingPattern::discard(ValueType::BitArray),
        BitArrayBindingPattern::Alias { pattern, binding } => {
            let (local, name) = binding.clone().into_parts();
            TotalBindingPattern::alias(
                total_bits_binding(pattern),
                AssertBinding::new(ParamLocal::bit_array(local), name, ValueShape::BitArray),
            )
        }
    }
}

fn define_binding(name: EcoString, type_: &Type, context: &mut PlanContext<'_>) -> AssertBinding {
    let shape = context.value_shape(type_);
    define_value_binding(name, shape, context)
}

fn define_value_binding(
    name: EcoString,
    shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> AssertBinding {
    AssertBinding::new(
        context.define_param_local_shape(name.clone(), shape.clone()),
        name,
        shape,
    )
}

fn define_string_binding(
    name: EcoString,
    context: &mut PlanContext<'_>,
) -> crate::plan::StringAssertBinding {
    crate::plan::StringAssertBinding::new(context.define_string_local(name.clone()), name)
}

pub(in crate::planner) fn pattern_value_type(
    pattern: &TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<ValueType, PlanError> {
    let mut value_shape = |type_: &Type| context.value_shape(type_);
    pattern_value_shape_with(pattern, &mut value_shape).map(|shape| shape.value_type())
}

fn pattern_value_shape(
    pattern: &TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<ValueShape, PlanError> {
    let mut value_shape = |type_: &Type| context.value_shape(type_);
    pattern_value_shape_with(pattern, &mut value_shape)
}

pub(in crate::planner) fn pattern_value_type_in_context(
    pattern: &TypedPattern,
    context: &PlanContext<'_>,
) -> Result<ValueType, PlanError> {
    let mut value_shape = |type_: &Type| context.value_shape_in_scope(type_);
    pattern_value_shape_with(pattern, &mut value_shape).map(|shape| shape.value_type())
}

fn pattern_value_shape_with(
    pattern: &TypedPattern,
    value_shape: &mut impl FnMut(&Type) -> ValueShape,
) -> Result<ValueShape, PlanError> {
    let shape = match pattern {
        Pattern::Int { .. } => ValueShape::Int,
        Pattern::Float { .. } => ValueShape::Float,
        Pattern::String { .. } | Pattern::StringPrefix { .. } => ValueShape::String,
        Pattern::Variable { type_, .. }
        | Pattern::Discard { type_, .. }
        | Pattern::List { type_, .. }
        | Pattern::Constructor { type_, .. }
        | Pattern::Invalid { type_, .. } => value_shape(type_.as_ref()),
        Pattern::Tuple { elements, .. } => ValueShape::Tuple(
            elements
                .iter()
                .map(|element| pattern_value_shape_with(element, value_shape))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        ),
        Pattern::BitArray { .. } => ValueShape::BitArray,
        Pattern::Assign { pattern, .. } => pattern_value_shape_with(pattern, value_shape)?,
        Pattern::BitArraySize(_) => return Err(invalid_pattern()),
    };
    Ok(shape)
}

fn invalid_pattern() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::InvalidPattern,
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomConstructorRefinement, CustomValueShape};
    use super::{
        PlannedRuntimePattern, invalid_pattern, pattern_value_type, plan_bool_pattern,
        plan_custom_pattern, plan_list_tail, plan_nil_pattern, plan_runtime_pattern,
        total_bit_array_binding,
    };
    use crate::plan::{
        AssertBinding, AssertPattern, BitArrayPattern, CustomBindingPattern, CustomTypeName,
        GenericLocal, GenericLocalId, ParamLocal, TotalBindingPattern, TypeParameterId, ValueShape,
        ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::{InvalidCustomTypeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{AssignName, BitArraySize, CallArg, Pattern, TailPattern};
    use gleam_core::type_::{self, PatternConstructor, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn runtime_pattern_rejects_invalid_typed_ast_shapes_exactly() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();

        assert_invalid(plan_runtime_pattern(
            Pattern::Invalid {
                location: span,
                type_: type_::int(),
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::BitArraySize(BitArraySize::Int {
                location: span,
                value: "1".into(),
                int_value: BigInt::from(1),
            }),
            &mut context,
        ));
        let parameter = TypeParameterId(0);
        let generic_binding = plan_runtime_pattern(
            Pattern::Variable {
                location: span,
                name: "value".into(),
                type_: type_::generic_var(0),
                origin: VariableOrigin::generated(),
            },
            &mut context,
        )
        .expect("generic variable pattern should plan");
        let binding = AssertBinding::new(
            ParamLocal::generic(GenericLocal::new(GenericLocalId(0), parameter)),
            "value".into(),
            ValueShape::Parameter(parameter),
        );
        assert_eq!(
            generic_binding.pattern,
            AssertPattern::Bind(binding.clone())
        );
        assert!(generic_binding.is_total);
        assert_eq!(
            generic_binding.total_binding,
            Some(TotalBindingPattern::bind(binding)),
        );
        assert!(generic_binding.custom_binding.is_none());

        let generic_discard = plan_runtime_pattern(
            Pattern::Discard {
                name: "_".into(),
                location: span,
                type_: type_::generic_var(0),
            },
            &mut context,
        )
        .expect("generic discard pattern should plan");
        assert_eq!(generic_discard.pattern, AssertPattern::Discard);
        assert!(generic_discard.is_total);
        assert_eq!(
            generic_discard.total_binding,
            Some(TotalBindingPattern::discard(ValueType::Parameter(
                parameter
            ))),
        );
        assert!(generic_discard.custom_binding.is_none());
        assert_invalid(plan_runtime_pattern(
            Pattern::Tuple {
                location: span,
                elements: vec![Pattern::BitArraySize(BitArraySize::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                })],
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::Assign {
                location: span,
                name: "alias".into(),
                pattern: Box::new(Pattern::BitArraySize(BitArraySize::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                })),
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::Assign {
                location: span,
                name: "alias".into(),
                pattern: Box::new(Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                }),
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::List {
                location: span,
                elements: Vec::new(),
                tail: None,
                type_: type_::int(),
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::List {
                location: span,
                elements: vec![Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                }],
                tail: None,
                type_: type_::list(type_::int()),
            },
            &mut context,
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::List {
                location: span,
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: span,
                    pattern: Pattern::Int {
                        location: span,
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                })),
                type_: type_::list(type_::int()),
            },
            &mut context,
        ));
        assert_eq!(
            plan_list_tail(
                TailPattern {
                    location: span,
                    pattern: Pattern::Int {
                        location: span,
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                },
                &ValueShape::Int,
                &mut context,
            ),
            Err(invalid_pattern()),
        );

        let argument = CallArg {
            label: None,
            location: span,
            value: Pattern::Discard {
                name: "_".into(),
                location: span,
                type_: type_::bool(),
            },
            implicit: None,
        };
        assert_invalid(plan_bool_pattern(Inferred::Unknown, Vec::new()));
        assert_invalid(plan_bool_pattern(
            Inferred::Known(pattern_constructor("True")),
            vec![argument],
        ));
        assert_invalid(plan_bool_pattern(
            Inferred::Known(pattern_constructor("Other")),
            Vec::new(),
        ));
        assert_invalid(plan_nil_pattern(Inferred::Unknown, Vec::new()));
        assert_invalid(plan_nil_pattern(
            Inferred::Known(pattern_constructor("Other")),
            Vec::new(),
        ));
        assert_invalid(plan_runtime_pattern(
            Pattern::Constructor {
                location: span,
                name_location: span,
                name: "Invalid".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Inferred::Known(pattern_constructor("Invalid")),
                spread: None,
                type_: type_::generic_var(0),
            },
            &mut context,
        ));
        let result_shape = CustomValueShape::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueShape::Int, ValueShape::String],
            CustomConstructorRefinement::Any,
        );
        assert_invalid(plan_custom_pattern(
            Vec::new(),
            Inferred::Unknown,
            type_::result(type_::int(), type_::string()),
            result_shape.clone(),
            &mut context,
        ));
        assert_eq!(
            plan_custom_pattern(
                Vec::new(),
                Inferred::Known(pattern_constructor("Ok")),
                type_::generic_var(0),
                result_shape.clone(),
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Ok".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_invalid(plan_custom_pattern(
            vec![CallArg {
                label: None,
                location: span,
                value: Pattern::BitArraySize(BitArraySize::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                implicit: None,
            }],
            Inferred::Known(pattern_constructor("Ok")),
            type_::result(type_::int(), type_::string()),
            result_shape.clone(),
            &mut context,
        ));
        assert_invalid(plan_custom_pattern(
            vec![CallArg {
                label: None,
                location: span,
                value: Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                },
                implicit: None,
            }],
            Inferred::Known(pattern_constructor("Ok")),
            type_::result(type_::int(), type_::string()),
            result_shape,
            &mut context,
        ));

        assert_eq!(
            total_bit_array_binding(&BitArrayPattern::new(Vec::new())),
            None
        );
        assert_eq!(
            pattern_value_type(
                &Pattern::Variable {
                    location: span,
                    name: "value".into(),
                    type_: type_::generic_var(0),
                    origin: VariableOrigin::generated(),
                },
                &mut context
            ),
            Ok(ValueType::Parameter(parameter)),
        );
        assert_eq!(
            pattern_value_type(
                &Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                },
                &mut context
            ),
            Ok(ValueType::Int),
        );
        assert_eq!(
            pattern_value_type(
                &Pattern::BitArraySize(BitArraySize::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                &mut context
            ),
            Err(invalid_pattern()),
        );
        assert_eq!(
            pattern_value_type(
                &Pattern::Tuple {
                    location: span,
                    elements: vec![Pattern::BitArraySize(BitArraySize::Int {
                        location: span,
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    })],
                },
                &mut context
            ),
            Err(invalid_pattern()),
        );
        assert_eq!(
            pattern_value_type(
                &Pattern::Assign {
                    location: span,
                    name: "alias".into(),
                    pattern: Box::new(Pattern::BitArraySize(BitArraySize::Int {
                        location: span,
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    })),
                },
                &mut context
            ),
            Err(invalid_pattern()),
        );
    }

    #[test]
    fn inferred_custom_variant_is_a_total_runtime_pattern() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
        let result_shape = CustomValueShape::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueShape::Int, ValueShape::String],
            CustomConstructorRefinement::Any,
        );
        let any = plan_custom_pattern(
            vec![CallArg {
                label: None,
                location: span,
                value: Pattern::Discard {
                    name: "_".into(),
                    location: span,
                    type_: type_::int(),
                },
                implicit: None,
            }],
            Inferred::Known(pattern_constructor("Ok")),
            type_::result(type_::int(), type_::string()),
            result_shape.clone(),
            &mut context,
        )
        .expect("non-inferred Result variant should plan");
        assert_eq!(
            any.custom_binding
                .as_ref()
                .expect("total fields should preserve the custom binding")
                .intrinsic_binding(),
            None,
        );
        let exact_result_shape = CustomValueShape::new(
            result_shape.type_name().clone(),
            result_shape.arguments().to_vec(),
            CustomConstructorRefinement::Exact(0),
        );
        let planned = plan_custom_pattern(
            vec![CallArg {
                label: None,
                location: span,
                value: Pattern::Discard {
                    name: "_".into(),
                    location: span,
                    type_: type_::int(),
                },
                implicit: None,
            }],
            Inferred::Known(pattern_constructor("Ok")),
            type_::result(type_::int(), type_::string()),
            exact_result_shape.clone(),
            &mut context,
        )
        .expect("inferred Result variant should plan");

        assert!(planned.is_total);
        let constructor = planned
            .custom_binding
            .as_ref()
            .expect("inferred Result pattern should preserve its custom binding")
            .constructor()
            .clone();
        assert_eq!(
            planned.total_binding,
            Some(TotalBindingPattern::custom(CustomBindingPattern::exact(
                exact_result_shape,
                constructor,
                vec![TotalBindingPattern::discard(ValueType::Int)],
            ))),
        );
        assert_eq!(
            planned
                .custom_binding
                .as_ref()
                .map(super::PlannedCustomBinding::constructor_count),
            Some(2),
        );
    }

    #[test]
    fn string_prefix_discard_preserves_the_exact_runtime_pattern() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();

        let planned = plan_runtime_pattern(
            Pattern::StringPrefix {
                location: span,
                left_location: span,
                left_side_assignment: None,
                right_location: span,
                left_side_string: "prefix".into(),
                right_side_assignment: AssignName::Discard("_".into()),
            },
            &mut context,
        );

        assert_eq!(
            planned.map(|planned| planned.pattern),
            Ok(AssertPattern::StringPrefix {
                prefix: "prefix".into(),
                left: None,
                right: None,
            }),
        );
    }

    fn assert_invalid(result: Result<PlannedRuntimePattern, crate::planner::PlanError>) {
        assert_eq!(result.map(|_| ()), Err(invalid_pattern()));
    }

    fn pattern_constructor(name: &str) -> PatternConstructor {
        PatternConstructor {
            name: name.into(),
            field_map: None,
            documentation: None,
            module: "gleam".into(),
            location: crate::planner::support::dummy_span(),
            constructor_index: 0,
        }
    }
}
