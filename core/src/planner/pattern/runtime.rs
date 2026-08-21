use crate::plan::{
    AssertBinding, AssertPattern, BitArrayBindingPattern, BitArrayPatternSegment,
    CustomBindingPattern, CustomConstructor, CustomConstructorRefinement, CustomPattern,
    CustomValueShape, ListAssertPattern, ListAssertTail, ParamLocal, TotalBindingPattern,
    ValueShape, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_compiler_core::analyse::Inferred;
use gleam_compiler_core::ast::{AssignName, Pattern, TailPattern, TypedPattern};
use gleam_compiler_core::strings::convert_string_escape_chars;
use gleam_compiler_core::type_::Type;
use std::sync::Arc;

pub(in crate::planner) struct PlannedRuntimePattern {
    pub(in crate::planner) pattern: AssertPattern,
    pub(in crate::planner) is_total: bool,
    pub(in crate::planner) total_binding: Option<TotalBindingPattern>,
    pub(in crate::planner) custom_binding: Option<PlannedCustomBinding>,
}

pub(in crate::planner) struct PlannedCustomPattern {
    pub(in crate::planner) pattern: CustomPattern,
    pub(in crate::planner) is_total: bool,
    total_binding: Option<TotalBindingPattern>,
    pub(in crate::planner) custom_binding: Option<PlannedCustomBinding>,
}

impl PlannedCustomPattern {
    fn into_runtime(self) -> PlannedRuntimePattern {
        PlannedRuntimePattern {
            pattern: AssertPattern::custom(self.pattern),
            is_total: self.is_total,
            total_binding: self.total_binding,
            custom_binding: self.custom_binding,
        }
    }
}

#[derive(Clone)]
pub(in crate::planner) struct PlannedCustomBinding {
    constructor: CustomConstructor,
    fields: Vec<TotalBindingPattern>,
    source_shape: CustomValueShape,
    constructor_count: usize,
}

impl PlannedCustomBinding {
    #[cfg(test)]
    pub(in crate::planner) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    #[cfg(test)]
    pub(in crate::planner) fn constructor_count(&self) -> usize {
        self.constructor_count
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

    pub(in crate::planner) fn into_exhaustive_remainder_binding(self) -> CustomBindingPattern {
        let constructor = self.constructor.index();
        let excluded = (0..self.constructor_count)
            .filter(|index| *index != constructor)
            .collect();
        self.into_remainder_binding(excluded)
    }
}

#[cfg(test)]
fn plan_runtime_pattern(
    pattern: TypedPattern,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let source_shape = super::pattern_value_shape(&pattern, context)?;
    plan_runtime_pattern_with_source_shape(pattern, source_shape, context)
}

pub(in crate::planner) fn plan_runtime_pattern_with_source_shape(
    pattern: TypedPattern,
    source_shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let validate_before_planning = match &pattern {
        Pattern::Assign { .. }
        | Pattern::Tuple { .. }
        | Pattern::List { .. }
        | Pattern::BitArray { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Invalid { .. } => false,
        Pattern::Constructor { type_, .. } => type_.is_bool() || type_.is_nil(),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Variable { .. }
        | Pattern::Discard { .. }
        | Pattern::StringPrefix { .. } => true,
    };
    if validate_before_planning {
        super::validate_pattern(&pattern, &source_shape, context)?;
    }
    plan_validated_runtime_pattern(pattern, source_shape, context)
}

fn plan_validated_runtime_pattern(
    pattern: TypedPattern,
    source_shape: ValueShape,
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
            pattern: AssertPattern::String(convert_string_escape_chars(&value)),
            is_total: false,
            total_binding: None,
            custom_binding: None,
        }),
        Pattern::Tuple { location, elements } => {
            let ValueShape::Tuple(source_shapes) = &source_shape else {
                let pattern = Pattern::Tuple { location, elements };
                return Err(super::unexpected_pattern(&pattern, &source_shape, context));
            };
            let source_shapes = source_shapes.clone();
            super::validate_tuple_arity(source_shapes.len(), elements.len())?;
            let mut patterns = Vec::with_capacity(elements.len());
            let mut bindings = Vec::with_capacity(elements.len());
            let mut is_total = true;
            for (element, source_shape) in elements.into_iter().zip(source_shapes) {
                let element =
                    plan_runtime_pattern_with_source_shape(element, source_shape, context)?;
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
            location,
            elements,
            tail,
            type_,
        } => {
            let ValueShape::List(item_shape) = &source_shape else {
                let pattern = Pattern::List {
                    location,
                    elements,
                    tail,
                    type_,
                };
                return Err(super::unexpected_pattern(&pattern, &source_shape, context));
            };
            super::validation::validate_pattern_type(
                &source_shape,
                context.value_shape_in_scope(type_.as_ref()),
            )?;
            plan_list_pattern(
                elements,
                tail.map(|tail| *tail),
                item_shape.as_ref().clone(),
                context,
            )
        }
        Pattern::BitArray { ref segments, .. } => {
            super::validation::validate_pattern_type(&source_shape, ValueShape::BitArray)?;
            let (pattern, is_total) = super::plan_bit_array_pattern(segments.clone(), context)?;
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
            arguments: _,
            name,
            type_,
            ..
        } if type_.is_bool() => Ok(plan_bool_pattern(name == "True")),
        Pattern::Constructor {
            arguments: _,
            constructor: _,
            type_,
            ..
        } if type_.is_nil() => Ok(plan_nil_pattern()),
        ref pattern @ Pattern::Constructor {
            ref arguments,
            ref constructor,
            ref type_,
            ..
        } => {
            let ValueShape::Custom(custom_source_shape) = &source_shape else {
                return Err(super::unexpected_pattern(pattern, &source_shape, context));
            };
            plan_custom_pattern(
                arguments.clone(),
                constructor.clone(),
                type_.clone(),
                custom_source_shape.clone(),
                context,
            )
            .map(PlannedCustomPattern::into_runtime)
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
                    prefix: convert_string_escape_chars(&left_side_string),
                    left,
                    right,
                },
                is_total: false,
                total_binding: None,
                custom_binding: None,
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let planned =
                plan_runtime_pattern_with_source_shape(*pattern, source_shape.clone(), context)?;
            let binding = define_value_binding(name, source_shape, context);
            Ok(PlannedRuntimePattern {
                pattern: AssertPattern::alias(planned.pattern, binding.clone()),
                is_total: planned.is_total,
                total_binding: planned
                    .total_binding
                    .map(|pattern| TotalBindingPattern::alias(pattern, binding)),
                custom_binding: planned.custom_binding,
            })
        }
        pattern @ (Pattern::BitArraySize(_) | Pattern::Invalid { .. }) => {
            Err(super::unexpected_pattern(&pattern, &source_shape, context))
        }
    }
}

pub(in crate::planner) fn plan_custom_subject_pattern(
    pattern: TypedPattern,
    source_shape: CustomValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedCustomPattern, PlanError> {
    let (arguments, constructor, type_) = match pattern {
        Pattern::Constructor {
            arguments,
            constructor,
            type_,
            ..
        } => (arguments, constructor, type_),
        pattern => {
            return Err(super::unexpected_pattern(
                &pattern,
                &ValueShape::Custom(source_shape),
                context,
            ));
        }
    };
    plan_custom_pattern(arguments, constructor, type_, source_shape, context)
}

pub(in crate::planner) fn pattern_value_type_in_context(
    pattern: &TypedPattern,
    context: &PlanContext<'_>,
) -> Result<ValueType, PlanError> {
    super::pattern_value_shape(pattern, context).map(|shape| shape.value_type())
}

fn plan_list_pattern(
    elements: Vec<TypedPattern>,
    tail: Option<TailPattern<Arc<Type>>>,
    item_shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedRuntimePattern, PlanError> {
    let element_type = item_shape.value_type();
    let has_no_elements = elements.is_empty();
    let mut patterns = Vec::with_capacity(elements.len());
    for element in elements {
        patterns.push(
            plan_runtime_pattern_with_source_shape(element, item_shape.clone(), context)?.pattern,
        );
    }
    let tail = tail
        .map(|tail| plan_list_tail(tail, &item_shape, context))
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
    let expected = ValueShape::List(Box::new(item_shape.clone()));
    match super::validate_list_tail(&tail.pattern, &expected, context)? {
        super::ValidatedListTail::Named(name) => Ok(ListAssertTail::bind(
            context.define_list_local_shape(name.clone(), item_shape.clone()),
            name,
        )),
        super::ValidatedListTail::Discard => Ok(ListAssertTail::Ignore),
    }
}

fn plan_bool_pattern(value: bool) -> PlannedRuntimePattern {
    PlannedRuntimePattern {
        pattern: AssertPattern::Bool(value),
        is_total: false,
        total_binding: None,
        custom_binding: None,
    }
}

fn plan_nil_pattern() -> PlannedRuntimePattern {
    PlannedRuntimePattern {
        pattern: AssertPattern::Nil,
        is_total: true,
        total_binding: Some(TotalBindingPattern::discard(ValueType::Nil)),
        custom_binding: None,
    }
}

fn plan_custom_pattern(
    arguments: Vec<gleam_compiler_core::ast::CallArg<TypedPattern>>,
    constructor: Inferred<gleam_compiler_core::type_::PatternConstructor>,
    type_: Arc<Type>,
    source_shape: CustomValueShape,
    context: &mut PlanContext<'_>,
) -> Result<PlannedCustomPattern, PlanError> {
    let constructor = super::resolved_constructor(&constructor)?;
    let field_types = arguments
        .iter()
        .map(|argument| pattern_value_type_in_context(&argument.value, context))
        .collect::<Result<Vec<_>, _>>()?;
    let resolved_constructor =
        context.custom_pattern_constructor(type_.as_ref(), constructor, field_types)?;
    let pattern_source_shape = resolved_constructor.source_shape().clone();
    let constructor_count = resolved_constructor.constructor_count();
    let custom_constructor = resolved_constructor.into_constructor();
    let mut fields = Vec::with_capacity(arguments.len());
    let mut binding_fields = Vec::with_capacity(arguments.len());
    let mut fields_are_total = true;
    for (argument, source_field) in arguments.into_iter().zip(custom_constructor.fields()) {
        let source_shape = ValueShape::from_value_type(source_field.type_().clone());
        let field = plan_runtime_pattern_with_source_shape(argument.value, source_shape, context)?;
        fields_are_total &= field.is_total;
        if let Some(binding) = field.total_binding {
            binding_fields.push(binding);
        }
        fields.push(field.pattern);
    }
    let fields_are_bindable = binding_fields.len() == fields.len();
    let matches_exact_constructor = source_shape.constructor()
        == CustomConstructorRefinement::Exact(usize::from(constructor.constructor_index));
    let is_total = fields_are_total && (constructor_count == 1 || matches_exact_constructor);
    let binding_source_shape = source_shape.refine(&pattern_source_shape);
    let custom_binding =
        (fields_are_bindable && binding_source_shape.is_some()).then(|| PlannedCustomBinding {
            constructor: custom_constructor.clone(),
            fields: binding_fields.clone(),
            source_shape: source_shape.clone(),
            constructor_count,
        });
    let total_binding = binding_source_shape
        .filter(|_| fields_are_bindable)
        .map(|binding_source_shape| PlannedCustomBinding {
            constructor: custom_constructor.clone(),
            fields: binding_fields.clone(),
            source_shape: binding_source_shape,
            constructor_count,
        })
        .and_then(PlannedCustomBinding::into_intrinsic_binding)
        .map(TotalBindingPattern::custom);
    Ok(PlannedCustomPattern {
        pattern: CustomPattern::new(
            custom_constructor,
            fields,
            fields_are_total.then_some(binding_fields),
        ),
        is_total,
        total_binding,
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

#[cfg(test)]
mod tests {
    use super::{CustomConstructorRefinement, CustomValueShape};
    use super::{
        pattern_value_type_in_context, plan_bool_pattern, plan_custom_pattern,
        plan_custom_subject_pattern, plan_runtime_pattern, plan_runtime_pattern_with_source_shape,
        total_bit_array_binding,
    };
    use crate::plan::{
        AssertBinding, AssertPattern, BitArrayBindingPattern, BitArrayLocalId, BitArrayPattern,
        BitArrayPatternSegment, CustomBindingPattern, CustomPattern, CustomTypeName, GenericLocal,
        GenericLocalId, IntListLocalId, ListAssertPattern, ListAssertTail, ListLocal, ParamLocal,
        PatternBinding, TotalBindingPattern, TypeParameterId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::{
        InvalidBitArraySegmentOptionsReason, InvalidCustomTypeReason, InvalidPatternShapeReason,
        InvalidTypedAstReason, PatternKind, PlanError,
    };
    use ecow::EcoString;
    use gleam_compiler_core::analyse::Inferred;
    use gleam_compiler_core::ast::{
        AssignName, BitArrayOption, BitArraySegment as AstBitArraySegment, BitArraySize, CallArg,
        Pattern, TailPattern,
    };
    use gleam_compiler_core::type_::{self, PatternConstructor, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn total_list_and_false_patterns_preserve_exact_runtime_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
        let tail = ListAssertTail::bind(ListLocal::int(IntListLocalId(0)), EcoString::from("tail"));

        let planned = plan_runtime_pattern(
            Pattern::List {
                location: span,
                elements: Vec::new(),
                tail: Some(Box::new(TailPattern {
                    location: span,
                    pattern: Pattern::Variable {
                        location: span,
                        name: "tail".into(),
                        type_: type_::list(type_::int()),
                        origin: VariableOrigin::generated(),
                    },
                })),
                type_: type_::list(type_::int()),
            },
            &mut context,
        )
        .expect("a tail-only list pattern should plan");
        assert_eq!(
            planned.pattern,
            AssertPattern::list(ListAssertPattern::new(
                ValueType::Int,
                Vec::new(),
                Some(tail.clone()),
            )),
        );
        assert!(planned.is_total);
        assert_eq!(
            planned.total_binding,
            Some(TotalBindingPattern::list(ValueType::Int, tail)),
        );
        assert!(planned.custom_binding.is_none());

        let planned = plan_bool_pattern(false);
        assert_eq!(planned.pattern, AssertPattern::Bool(false));
        assert!(!planned.is_total);
        assert!(planned.total_binding.is_none());
        assert!(planned.custom_binding.is_none());
    }

    #[test]
    fn total_bit_array_binding_preserves_bind_discard_and_alias_shapes() {
        let bits = PatternBinding::new(BitArrayLocalId(0), "bits".into());
        let alias = PatternBinding::new(BitArrayLocalId(1), "whole".into());

        assert_eq!(
            total_bit_array_binding(&BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Bind(bits.clone()),
                size: None,
                unit: 1,
            }])),
            Some(TotalBindingPattern::bind(AssertBinding::new(
                ParamLocal::bit_array(BitArrayLocalId(0)),
                "bits".into(),
                ValueShape::BitArray,
            ))),
        );
        assert_eq!(
            total_bit_array_binding(&BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Discard,
                size: None,
                unit: 1,
            }])),
            Some(TotalBindingPattern::discard(ValueType::BitArray)),
        );
        assert_eq!(
            total_bit_array_binding(&BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Alias {
                    pattern: Box::new(BitArrayBindingPattern::Bind(bits)),
                    binding: alias,
                },
                size: None,
                unit: 1,
            }])),
            Some(TotalBindingPattern::alias(
                TotalBindingPattern::bind(AssertBinding::new(
                    ParamLocal::bit_array(BitArrayLocalId(0)),
                    "bits".into(),
                    ValueShape::BitArray,
                )),
                AssertBinding::new(
                    ParamLocal::bit_array(BitArrayLocalId(1)),
                    "whole".into(),
                    ValueShape::BitArray,
                ),
            )),
        );
    }

    #[test]
    fn total_bit_array_runtime_pattern_preserves_its_binding_proof() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
        let binding = PatternBinding::new(BitArrayLocalId(0), "rest".into());
        let expected_pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
            pattern: BitArrayBindingPattern::Bind(binding),
            size: None,
            unit: 1,
        }]);

        let planned = plan_runtime_pattern(
            Pattern::BitArray {
                location: span,
                segments: vec![AstBitArraySegment {
                    location: span,
                    value: Box::new(Pattern::Variable {
                        location: span,
                        name: "rest".into(),
                        type_: type_::bit_array(),
                        origin: VariableOrigin::generated(),
                    }),
                    options: vec![BitArrayOption::Bits { location: span }],
                    type_: type_::bit_array(),
                }],
            },
            &mut context,
        )
        .expect("an unsized bits remainder should be a total runtime pattern");

        assert_eq!(planned.pattern, AssertPattern::bit_array(expected_pattern),);
        assert!(planned.is_total);
        assert_eq!(
            planned.total_binding,
            Some(TotalBindingPattern::bind(AssertBinding::new(
                ParamLocal::bit_array(BitArrayLocalId(0)),
                "rest".into(),
                ValueShape::BitArray,
            ))),
        );
        assert!(planned.custom_binding.is_none());
    }

    #[test]
    fn recursive_pattern_shape_preserves_nested_value_types() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
        let pattern = Pattern::Assign {
            location: span,
            name: "whole".into(),
            pattern: Box::new(Pattern::Tuple {
                location: span,
                elements: vec![
                    Pattern::List {
                        location: span,
                        elements: Vec::new(),
                        tail: None,
                        type_: type_::list(type_::int()),
                    },
                    Pattern::BitArray {
                        location: span,
                        segments: Vec::new(),
                    },
                    Pattern::StringPrefix {
                        location: span,
                        left_location: span,
                        left_side_assignment: None,
                        right_location: span,
                        left_side_string: "prefix".into(),
                        right_side_assignment: AssignName::Discard("_".into()),
                    },
                ],
            }),
        };
        let expected = ValueShape::Tuple(
            vec![
                ValueShape::List(Box::new(ValueShape::Int)),
                ValueShape::BitArray,
                ValueShape::String,
            ]
            .into_boxed_slice(),
        );

        assert_eq!(
            super::super::pattern_value_shape(&pattern, &context),
            Ok(expected.clone())
        );
        assert_eq!(
            pattern_value_type_in_context(&pattern, &context),
            Ok(expected.value_type()),
        );
    }

    #[test]
    fn runtime_pattern_preserves_generic_bindings_and_custom_resolution_errors() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
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

        let result_shape = CustomValueShape::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueShape::Int, ValueShape::String],
            CustomConstructorRefinement::Any,
        );
        assert_eq!(
            plan_custom_pattern(
                Vec::new(),
                Inferred::Known(pattern_constructor("Ok")),
                type_::generic_var(0),
                result_shape,
                &mut context,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "gleam".into(),
                    name: "Ok".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: ValueType::Parameter(parameter),
                    }),
                },
            }),
        );

        assert_eq!(
            total_bit_array_binding(&BitArrayPattern::new(Vec::new())),
            None
        );
        assert_eq!(
            pattern_value_type_in_context(
                &Pattern::Variable {
                    location: span,
                    name: "other".into(),
                    type_: type_::generic_var(0),
                    origin: VariableOrigin::generated(),
                },
                &context,
            ),
            Ok(ValueType::Parameter(parameter)),
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
                .clone()
                .expect("total fields should preserve the custom binding")
                .into_intrinsic_binding(),
            None,
        );
        let any_constructor = any
            .custom_binding
            .as_ref()
            .expect("total fields should preserve the custom binding")
            .constructor()
            .clone();
        assert_eq!(
            any.pattern,
            CustomPattern::new(
                any_constructor.clone(),
                vec![AssertPattern::Discard],
                Some(vec![TotalBindingPattern::discard(ValueType::Int)]),
            ),
        );
        assert_eq!(
            any.custom_binding
                .clone()
                .expect("total fields should preserve the custom binding")
                .into_remainder_binding(vec![0]),
            CustomBindingPattern::exhaustive_remainder(
                result_shape.clone(),
                vec![0],
                any_constructor,
                vec![TotalBindingPattern::discard(ValueType::Int)],
            ),
        );
        assert_eq!(
            any.custom_binding
                .clone()
                .expect("total fields should preserve the custom binding")
                .into_exhaustive_remainder_binding(),
            CustomBindingPattern::exhaustive_remainder(
                result_shape.clone(),
                vec![1],
                any.custom_binding
                    .as_ref()
                    .expect("total fields should preserve the custom binding")
                    .constructor()
                    .clone(),
                vec![TotalBindingPattern::discard(ValueType::Int)],
            ),
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
                .clone()
                .expect("inferred Result pattern should preserve its custom binding")
                .into_intrinsic_binding(),
            Some(CustomBindingPattern::exact(
                CustomValueShape::new(
                    result_shape.type_name().clone(),
                    result_shape.arguments().to_vec(),
                    CustomConstructorRefinement::Exact(0),
                ),
                planned
                    .custom_binding
                    .as_ref()
                    .expect("inferred Result pattern should preserve its custom binding")
                    .constructor()
                    .clone(),
                vec![TotalBindingPattern::discard(ValueType::Int)],
            )),
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

    #[test]
    fn runtime_pattern_boundaries_reject_malformed_structural_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let span = crate::planner::support::dummy_span();
        let result_type = crate::plan::CustomType::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueType::Int, ValueType::String],
        );
        let result_shape = CustomValueShape::any(result_type.clone());

        let malformed = [
            (
                Pattern::Tuple {
                    location: span,
                    elements: Vec::new(),
                },
                ValueShape::Int,
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::Tuple(Vec::new()),
                },
            ),
            (
                Pattern::List {
                    location: span,
                    elements: Vec::new(),
                    tail: None,
                    type_: type_::list(type_::int()),
                },
                ValueShape::Int,
                InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Int,
                    actual: PatternKind::List,
                },
            ),
            (
                Pattern::List {
                    location: span,
                    elements: Vec::new(),
                    tail: None,
                    type_: type_::list(type_::string()),
                },
                ValueShape::List(Box::new(ValueShape::Int)),
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::List(Box::new(ValueType::String)),
                },
            ),
            (
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "Ok".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Known(pattern_constructor("Ok")),
                    spread: None,
                    type_: type_::result(type_::int(), type_::string()),
                },
                ValueShape::Int,
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::Custom(result_type.clone()),
                },
            ),
        ];
        for (pattern, source_shape, reason) in malformed {
            assert_eq!(
                plan_runtime_pattern_with_source_shape(pattern, source_shape, &mut context)
                    .map(|_| ()),
                Err(pattern_shape_error(reason)),
            );
        }

        for (pattern, reason) in [
            (
                Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                },
                InvalidPatternShapeReason::InvalidNode,
            ),
            (
                Pattern::BitArraySize(BitArraySize::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                InvalidPatternShapeReason::BitArraySizeNode,
            ),
        ] {
            assert_eq!(
                plan_runtime_pattern_with_source_shape(pattern, ValueShape::Int, &mut context)
                    .map(|_| ()),
                Err(pattern_shape_error(reason)),
            );
        }

        assert_eq!(
            plan_custom_subject_pattern(
                Pattern::Discard {
                    name: "_".into(),
                    location: span,
                    type_: type_::result(type_::int(), type_::string()),
                },
                result_shape,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::KindMismatch {
                    expected: ValueType::Custom(result_type.clone()),
                    actual: PatternKind::Discard,
                }
            )),
        );

        assert_eq!(
            plan_runtime_pattern(
                Pattern::Invalid {
                    location: span,
                    type_: type_::int(),
                },
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(InvalidPatternShapeReason::InvalidNode,)),
        );

        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                Pattern::Variable {
                    location: span,
                    name: "value".into(),
                    type_: type_::string(),
                    origin: VariableOrigin::generated(),
                },
                ValueShape::Int,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );

        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                Pattern::Tuple {
                    location: span,
                    elements: Vec::new(),
                },
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(InvalidPatternShapeReason::TupleArity {
                expected: 1,
                actual: 0,
            },)),
        );

        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                Pattern::Tuple {
                    location: span,
                    elements: vec![Pattern::Variable {
                        location: span,
                        name: "value".into(),
                        type_: type_::string(),
                        origin: VariableOrigin::generated(),
                    }],
                },
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );

        let malformed_bits = Pattern::BitArray {
            location: span,
            segments: vec![AstBitArraySegment {
                location: span,
                value: Box::new(Pattern::Discard {
                    name: "_".into(),
                    location: span,
                    type_: type_::int(),
                }),
                options: vec![
                    BitArrayOption::Bits { location: span },
                    BitArrayOption::Bytes { location: span },
                ],
                type_: type_::int(),
            }],
        };
        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                malformed_bits.clone(),
                ValueShape::Int,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::BitArray,
                },
            )),
        );
        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                malformed_bits,
                ValueShape::BitArray,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::BitArraySegmentOptions {
                    reason: InvalidBitArraySegmentOptionsReason::MultipleKinds,
                },
            )),
        );

        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                Pattern::Constructor {
                    location: span,
                    name_location: span,
                    name: "True".into(),
                    arguments: Vec::new(),
                    module: None,
                    constructor: Inferred::Unknown,
                    spread: None,
                    type_: type_::bool(),
                },
                ValueShape::Bool,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                Pattern::Assign {
                    location: span,
                    name: "whole".into(),
                    pattern: Box::new(Pattern::Invalid {
                        location: span,
                        type_: type_::int(),
                    }),
                },
                ValueShape::Int,
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(InvalidPatternShapeReason::InvalidNode,)),
        );

        let malformed_list = Pattern::List {
            location: span,
            elements: vec![Pattern::Variable {
                location: span,
                name: "item".into(),
                type_: type_::string(),
                origin: VariableOrigin::generated(),
            }],
            tail: Some(Box::new(TailPattern {
                location: span,
                pattern: Pattern::Int {
                    location: span,
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
            })),
            type_: type_::list(type_::int()),
        };
        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                malformed_list,
                ValueShape::List(Box::new(ValueShape::Int)),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );

        let malformed_tail = Pattern::List {
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
        };
        assert_eq!(
            plan_runtime_pattern_with_source_shape(
                malformed_tail,
                ValueShape::List(Box::new(ValueShape::Int)),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::ListTailKind {
                    actual: PatternKind::Int,
                },
            )),
        );

        assert_eq!(
            plan_custom_pattern(
                Vec::new(),
                Inferred::Unknown,
                type_::result(type_::int(), type_::string()),
                CustomValueShape::any(result_type.clone()),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::UnresolvedConstructor,
            )),
        );

        assert_eq!(
            plan_custom_pattern(
                vec![CallArg {
                    label: None,
                    location: span,
                    value: Pattern::List {
                        location: span,
                        elements: vec![Pattern::String {
                            location: span,
                            value: "wrong".into(),
                        }],
                        tail: None,
                        type_: type_::list(type_::int()),
                    },
                    implicit: None,
                }],
                Inferred::Known(pattern_constructor("Ok")),
                type_::result(type_::list(type_::int()), type_::string()),
                CustomValueShape::any(crate::plan::CustomType::new(
                    CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                    vec![ValueType::List(Box::new(ValueType::Int)), ValueType::String],
                )),
                &mut context,
            )
            .map(|_| ()),
            Err(pattern_shape_error(
                InvalidPatternShapeReason::TypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
            )),
        );
    }

    fn pattern_shape_error(reason: InvalidPatternShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::PatternShape { reason },
        }
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
