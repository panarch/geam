use super::expression::{
    bit_array_expr, bit_array_function_expr, bool_expr, custom_expr, expr, float_expr,
    function_function_expr, int_expr, int_function_expr, list_function_expr, nil_expr,
    nil_function_expr, string_expr, string_function_expr, symbolic_bit_array_function_expr,
    symbolic_bool_function_expr, symbolic_float_function_expr, symbolic_function_function_expr,
    symbolic_int_function_expr, symbolic_list_function_expr, symbolic_nil_function_expr,
    symbolic_string_function_expr, symbolic_utf_codepoint_function_expr, tuple_expr,
    typed_function_expr, utf_codepoint_expr, utf_codepoint_function_expr,
};
use super::id::{custom_local, function_function_local, list_function_local, list_local};
use super::specialization::Representability;
use crate::plan::{execution, module};

macro_rules! primitive_function_step {
    (
        $context:expr,
        $local:expr,
        $value:expr,
        $local_kind:ident,
        $lower:expr,
        $symbolic:expr,
        $step:ident,
        $execution_local:ident
    ) => {{
        let shape = $context.concrete_function_shape($value.shape());
        if matches!(
            $context.function_representation(&shape),
            super::specialization::FunctionRepresentation::Symbolic
        ) {
            super::expression::symbolic_function_step(
                super::frame::LocalKey::new(super::frame::LocalKind::$local_kind, $local.0),
                $value.shape(),
                $value.expression(),
                $context,
                $symbolic,
            )
        } else {
            typed_function_expr($value, $context, $lower).map(|value| execution::StepKind::$step {
                local: execution::$execution_local(
                    $context.mapped_local(super::frame::LocalKind::$local_kind, $local.0),
                ),
                value,
            })
        }
    }};
}

pub(super) fn steps(
    steps: &[module::Step],
    context: &mut super::LoweringContext,
) -> Representability<Vec<execution::Step>> {
    Representability::collect(steps.iter().map(|step| lower_step(step, context)))
}

pub(super) enum StepsUntilNever {
    Complete(Vec<execution::Step>),
    Diverging {
        prefix: Vec<execution::Step>,
        expression: execution::NeverExpr,
    },
}

pub(super) fn steps_until_never(
    steps: &[module::Step],
    context: &mut super::LoweringContext,
) -> Representability<StepsUntilNever> {
    let mut prefix = Vec::with_capacity(steps.len());
    for step in steps {
        if let Some(expression) = guaranteed_let_assert_failure(step, context) {
            return expression.map(|expression| StepsUntilNever::Diverging { prefix, expression });
        }
        let kind = match lower_step_kind(step, context) {
            Representability::Inhabited(kind) => kind,
            Representability::Uninhabited => return Representability::Uninhabited,
        };
        match kind {
            execution::StepKind::Evaluate(expression) => match expression.into_kind() {
                execution::ExprKind::Never(expression) => {
                    return Representability::Inhabited(StepsUntilNever::Diverging {
                        prefix,
                        expression,
                    });
                }
                kind => prefix.push(execution::Step::from_kind(execution::StepKind::Evaluate(
                    execution::Expr::from_kind(kind),
                ))),
            },
            kind => prefix.push(execution::Step::from_kind(kind)),
        }
    }
    Representability::Inhabited(StepsUntilNever::Complete(prefix))
}

fn guaranteed_let_assert_failure(
    step: &module::Step,
    context: &mut super::LoweringContext,
) -> Option<Representability<execution::NeverExpr>> {
    let module::StepKind::AssertPattern {
        subject,
        pattern,
        message,
        site,
        pattern_span,
    } = step.kind()
    else {
        return None;
    };
    if !parameter_list_is_empty(subject, context) || !list_pattern_requires_item(pattern) {
        return None;
    }

    Some(
        Representability::transpose_option(
            message
                .as_ref()
                .map(|message| string_expr(message, context)),
        )
        .map(|message| {
            execution::NeverExpr::from_kind(execution::NeverExprKind::LetAssert {
                subject: assert_subject(subject, context),
                message: message.map(Box::new),
                site: site.clone(),
                pattern_span: *pattern_span,
            })
        }),
    )
}

fn parameter_list_is_empty(
    subject: &module::AssertSubject,
    context: &super::LoweringContext,
) -> bool {
    let module::AssertSubject::List(module::ListLocal::Generic { parameter, .. }) = subject else {
        return false;
    };
    matches!(
        context.concrete_parameter(*parameter),
        super::specialization::SpecializedValueShape::Parameter(_)
    )
}

fn list_pattern_requires_item(pattern: &module::AssertPattern) -> bool {
    match pattern {
        module::AssertPattern::List(pattern) => !pattern.elements().is_empty(),
        module::AssertPattern::Alias { pattern, .. } => list_pattern_requires_item(pattern),
        _ => false,
    }
}

fn lower_step(
    step: &module::Step,
    context: &mut super::LoweringContext,
) -> Representability<execution::Step> {
    lower_step_kind(step, context).map(execution::Step::from_kind)
}

fn lower_step_kind(
    step: &module::Step,
    context: &mut super::LoweringContext,
) -> Representability<execution::StepKind> {
    use execution::StepKind as E;
    use module::StepKind as M;

    match step.kind() {
        M::LetGeneric { local, value, .. } => {
            super::expression::generic_step(*local, value, context)
        }
        M::LetInt {
            local,
            name: _,
            value,
        } => int_expr(value, context).map(|value| E::LetInt {
            local: execution::IntLocalId(
                context.mapped_local(super::frame::LocalKind::Int, local.0),
            ),
            value,
        }),
        M::LetFloat {
            local,
            name: _,
            value,
        } => float_expr(value, context).map(|value| E::LetFloat {
            local: execution::FloatLocalId(
                context.mapped_local(super::frame::LocalKind::Float, local.0),
            ),
            value,
        }),
        M::LetString {
            local,
            name: _,
            value,
        } => string_expr(value, context).map(|value| E::LetString {
            local: execution::StringLocalId(
                context.mapped_local(super::frame::LocalKind::String, local.0),
            ),
            value,
        }),
        M::LetBitArray {
            local,
            name: _,
            value,
        } => bit_array_expr(value, context).map(|value| E::LetBitArray {
            local: execution::BitArrayLocalId(
                context.mapped_local(super::frame::LocalKind::BitArray, local.0),
            ),
            value,
        }),
        M::LetUtfCodepoint {
            local,
            name: _,
            value,
        } => utf_codepoint_expr(value, context).map(|value| E::LetUtfCodepoint {
            local: execution::UtfCodepointLocalId(
                context.mapped_local(super::frame::LocalKind::UtfCodepoint, local.0),
            ),
            value,
        }),
        M::LetCustom { binding, name: _ } => custom_expr(binding.value(), context).map(|value| {
            E::LetCustom(execution::CustomLocalExpr::new(
                custom_local(binding.local(), context),
                value,
            ))
        }),
        M::LetBool {
            local,
            name: _,
            value,
        } => bool_expr(value, context).map(|value| E::LetBool {
            local: execution::BoolLocalId(
                context.mapped_local(super::frame::LocalKind::Bool, local.0),
            ),
            value,
        }),
        M::LetNil {
            local,
            name: _,
            value,
        } => nil_expr(value, context).map(|value| E::LetNil {
            local: execution::NilLocalId(
                context.mapped_local(super::frame::LocalKind::Nil, local.0),
            ),
            value,
        }),
        M::LetTuple {
            local,
            name: _,
            value,
        } => tuple_expr(value, context).map(|value| E::LetTuple {
            local: execution::TupleLocalId(
                context.mapped_local(super::frame::LocalKind::Tuple, local.0),
            ),
            value,
        }),
        M::LetList { name: _, value } => {
            super::expression::list_local_expr(value, context).map(|value| E::LetList { value })
        }
        M::LetIntFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            IntFunction,
            int_function_expr,
            symbolic_int_function_expr,
            LetIntFunction,
            IntFunctionLocalId
        ),
        M::LetFloatFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            FloatFunction,
            super::expression::float_function_expr,
            symbolic_float_function_expr,
            LetFloatFunction,
            FloatFunctionLocalId
        ),
        M::LetStringFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            StringFunction,
            string_function_expr,
            symbolic_string_function_expr,
            LetStringFunction,
            StringFunctionLocalId
        ),
        M::LetBitArrayFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            BitArrayFunction,
            bit_array_function_expr,
            symbolic_bit_array_function_expr,
            LetBitArrayFunction,
            BitArrayFunctionLocalId
        ),
        M::LetUtfCodepointFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            UtfCodepointFunction,
            utf_codepoint_function_expr,
            symbolic_utf_codepoint_function_expr,
            LetUtfCodepointFunction,
            UtfCodepointFunctionLocalId
        ),
        M::LetCustomFunction {
            local,
            name: _,
            value,
        } => super::expression::specialized_typed_custom_function_binding(
            context.mapped_local(super::frame::LocalKind::CustomFunction, local.id().0),
            value,
            context,
        )
        .map(super::expression::specialized_function_step),
        M::LetBoolFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            BoolFunction,
            super::expression::bool_function_expr,
            symbolic_bool_function_expr,
            LetBoolFunction,
            BoolFunctionLocalId
        ),
        M::LetNilFunction {
            local,
            name: _,
            value,
        } => primitive_function_step!(
            context,
            local,
            value,
            NilFunction,
            nil_function_expr,
            symbolic_nil_function_expr,
            LetNilFunction,
            NilFunctionLocalId
        ),
        M::LetTupleFunction {
            local,
            name: _,
            value,
        } => super::expression::specialized_typed_tuple_function_binding(
            context.mapped_local(super::frame::LocalKind::TupleFunction, local.0),
            value,
            context,
        )
        .map(super::expression::specialized_function_step),
        M::LetListFunction {
            local,
            name: _,
            value,
        } => {
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                super::expression::symbolic_function_step(
                    super::frame::list_function_local_key(local),
                    value.shape(),
                    value.expression(),
                    context,
                    symbolic_list_function_expr,
                )
            } else {
                typed_function_expr(value, context, list_function_expr).map(|value| {
                    E::LetListFunction {
                        local: list_function_local(local, context),
                        value,
                    }
                })
            }
        }
        M::LetFunctionFunction {
            local,
            name: _,
            value,
        } => {
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                super::expression::symbolic_function_step(
                    super::frame::LocalKey::new(
                        super::frame::LocalKind::FunctionFunction,
                        local.id().0,
                    ),
                    value.shape(),
                    value.expression(),
                    context,
                    symbolic_function_function_expr,
                )
            } else {
                typed_function_expr(value, context, function_function_expr).map(|value| {
                    E::LetFunctionFunction {
                        local: function_function_local(local, context),
                        value,
                    }
                })
            }
        }
        M::LetGenericFunction { local, value, .. } => {
            super::expression::generic_function_step(local, value, context)
        }
        M::AssertPattern {
            subject,
            pattern,
            message,
            site,
            pattern_span,
        } => Representability::transpose_option(
            message
                .as_ref()
                .map(|message| string_expr(message, context)),
        )
        .map(|message| E::AssertPattern {
            subject: assert_subject(subject, context),
            pattern: assert_pattern(pattern, context),
            message,
            site: site.clone(),
            pattern_span: *pattern_span,
        }),
        M::BindCustomFields { local, pattern } => {
            custom_binding_pattern(pattern, context).map(|pattern| E::BindCustomFields {
                local: custom_local(local, context),
                pattern,
            })
        }
        M::AssertBool {
            condition,
            message,
            site,
        } => bool_expr(condition, context).and_then(|condition| {
            Representability::transpose_option(
                message
                    .as_ref()
                    .map(|message| string_expr(message, context)),
            )
            .map(|message| E::AssertBool {
                condition,
                message,
                site: site.clone(),
            })
        }),
        M::Evaluate(value) => expr(value, context).map(E::Evaluate),
    }
}

fn assert_subject(
    subject: &module::AssertSubject,
    context: &mut super::LoweringContext,
) -> execution::AssertSubject {
    match subject {
        module::AssertSubject::Int(local) => execution::AssertSubject::Int(execution::IntLocalId(
            context.mapped_local(super::frame::LocalKind::Int, local.0),
        )),
        module::AssertSubject::Float(local) => execution::AssertSubject::Float(
            execution::FloatLocalId(context.mapped_local(super::frame::LocalKind::Float, local.0)),
        ),
        module::AssertSubject::String(local) => {
            execution::AssertSubject::String(execution::StringLocalId(
                context.mapped_local(super::frame::LocalKind::String, local.0),
            ))
        }
        module::AssertSubject::BitArray(local) => {
            execution::AssertSubject::BitArray(execution::BitArrayLocalId(
                context.mapped_local(super::frame::LocalKind::BitArray, local.0),
            ))
        }
        module::AssertSubject::Custom(local) => {
            execution::AssertSubject::Custom(custom_local(local, context))
        }
        module::AssertSubject::Bool(local) => execution::AssertSubject::Bool(
            execution::BoolLocalId(context.mapped_local(super::frame::LocalKind::Bool, local.0)),
        ),
        module::AssertSubject::Nil(local) => execution::AssertSubject::Nil(execution::NilLocalId(
            context.mapped_local(super::frame::LocalKind::Nil, local.0),
        )),
        module::AssertSubject::Tuple(local) => execution::AssertSubject::Tuple(
            execution::TupleLocalId(context.mapped_local(super::frame::LocalKind::Tuple, local.0)),
        ),
        module::AssertSubject::List(local) => {
            execution::AssertSubject::List(list_local(local, context))
        }
    }
}

fn custom_binding_pattern(
    pattern: &module::CustomBindingPattern,
    context: &mut super::LoweringContext,
) -> Representability<execution::CustomBindingPattern> {
    custom_field_binding_pattern(pattern.constructor(), pattern.fields(), context)
}

pub(super) fn custom_field_binding_pattern(
    constructor: &crate::plan::CustomConstructor,
    fields: &[module::TotalBindingPattern],
    context: &mut super::LoweringContext,
) -> Representability<execution::CustomBindingPattern> {
    Representability::collect(
        fields
            .iter()
            .map(|field| total_binding_pattern(field, context)),
    )
    .map(|fields| {
        execution::CustomBindingPattern::new(
            context.custom_constructor(constructor.clone()),
            fields,
        )
    })
}

fn total_binding_pattern(
    pattern: &module::TotalBindingPattern,
    context: &mut super::LoweringContext,
) -> Representability<execution::TotalBindingPattern> {
    let kind = match pattern.kind() {
        module::TotalBindingPatternKind::Bind(binding) => match assert_binding(binding, context) {
            super::specialization::StorageErasure::Stored(binding) => {
                Representability::Inhabited(execution::TotalBindingPatternKind::Bind(binding))
            }
            super::specialization::StorageErasure::Erased => Representability::Uninhabited,
        },
        module::TotalBindingPatternKind::Discard => {
            Representability::Inhabited(execution::TotalBindingPatternKind::Discard)
        }
        module::TotalBindingPatternKind::Tuple(elements) => Representability::collect(
            elements
                .iter()
                .map(|element| total_binding_pattern(element, context)),
        )
        .map(execution::TotalBindingPatternKind::Tuple),
        module::TotalBindingPatternKind::List(tail) => Representability::Inhabited(
            execution::TotalBindingPatternKind::List(assert_tail(tail, context)),
        ),
        module::TotalBindingPatternKind::Custom(pattern) => {
            custom_binding_pattern(pattern, context).map(execution::TotalBindingPatternKind::Custom)
        }
        module::TotalBindingPatternKind::Alias { pattern, binding } => {
            total_binding_pattern(pattern, context).and_then(|pattern| {
                match assert_binding(binding, context) {
                    super::specialization::StorageErasure::Stored(binding) => {
                        Representability::Inhabited(execution::TotalBindingPatternKind::Alias {
                            pattern: Box::new(pattern),
                            binding,
                        })
                    }
                    super::specialization::StorageErasure::Erased => Representability::Uninhabited,
                }
            })
        }
    };
    kind.map(|kind| {
        execution::TotalBindingPattern::new(context.value_type(pattern.type_().clone()), kind)
    })
}

pub(super) fn assert_pattern(
    pattern: &module::AssertPattern,
    context: &mut super::LoweringContext,
) -> execution::AssertPattern {
    match pattern {
        module::AssertPattern::Bind(binding) => match assert_binding(binding, context) {
            super::specialization::StorageErasure::Stored(binding) => {
                execution::AssertPattern::Bind(binding)
            }
            super::specialization::StorageErasure::Erased => execution::AssertPattern::Discard,
        },
        module::AssertPattern::Discard => execution::AssertPattern::Discard,
        module::AssertPattern::Int(value) => execution::AssertPattern::Int(value.clone()),
        module::AssertPattern::Float(value) => execution::AssertPattern::Float(*value),
        module::AssertPattern::String(value) => execution::AssertPattern::String(value.clone()),
        module::AssertPattern::Bool(value) => execution::AssertPattern::Bool(*value),
        module::AssertPattern::Nil => execution::AssertPattern::Nil,
        module::AssertPattern::Tuple(elements) => execution::AssertPattern::Tuple(
            elements
                .iter()
                .map(|element| assert_pattern(element, context))
                .collect(),
        ),
        module::AssertPattern::List(pattern) => {
            execution::AssertPattern::List(execution::ListAssertPattern::new(
                pattern
                    .elements()
                    .iter()
                    .map(|element| assert_pattern(element, context))
                    .collect(),
                pattern.tail().map(|tail| assert_tail(tail, context)),
            ))
        }
        module::AssertPattern::BitArray(pattern) => {
            execution::AssertPattern::BitArray(super::pattern::bit_array_pattern(pattern, context))
        }
        module::AssertPattern::Custom(pattern) => {
            execution::AssertPattern::Custom(execution::CustomPattern::new(
                context.custom_constructor(pattern.constructor().clone()),
                pattern
                    .fields()
                    .iter()
                    .map(|field| assert_pattern(field, context))
                    .collect(),
            ))
        }
        module::AssertPattern::StringPrefix {
            prefix,
            left,
            right,
        } => execution::AssertPattern::StringPrefix {
            prefix: prefix.clone(),
            left: left
                .as_ref()
                .map(|binding| string_assert_binding(binding, context)),
            right: right
                .as_ref()
                .map(|binding| string_assert_binding(binding, context)),
        },
        module::AssertPattern::Alias { pattern, binding } => {
            match assert_binding(binding, context) {
                super::specialization::StorageErasure::Stored(binding) => {
                    execution::AssertPattern::Alias {
                        pattern: Box::new(assert_pattern(pattern, context)),
                        binding,
                    }
                }
                super::specialization::StorageErasure::Erased => assert_pattern(pattern, context),
            }
        }
    }
}

fn string_assert_binding(
    binding: &module::StringAssertBinding,
    context: &super::LoweringContext,
) -> execution::StringAssertBinding {
    execution::StringAssertBinding::new(execution::StringLocalId(
        context.mapped_local(super::frame::LocalKind::String, binding.local().0),
    ))
}

fn assert_binding(
    binding: &module::AssertBinding,
    context: &mut super::LoweringContext,
) -> super::specialization::StorageErasure<execution::AssertBinding> {
    super::param::param_slot(binding.slot(), context).map(execution::AssertBinding::new)
}

fn assert_tail(
    tail: &module::ListAssertTail,
    context: &mut super::LoweringContext,
) -> execution::ListAssertTail {
    match tail {
        module::ListAssertTail::Ignore => execution::ListAssertTail::Ignore,
        module::ListAssertTail::Bind(binding) => {
            execution::ListAssertTail::bind(list_local(binding.local(), context))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::specialization::{
        Representability, RepresentationContext, SpecializationKey, SpecializedTypeSubstitution,
    };
    use super::super::{FunctionTemplates, LoweringContext};
    use crate::plan::execution::{
        AssertBinding, AssertPattern, AssertSubject, BitArrayBindingPattern, BitArrayLocalId,
        BitArrayPatternSegment, BitArrayPatternSizeExpr, BitArrayPatternValue, ExecutionPlan,
        IntFunctionId, IntListLocalId, IntLocalId, ListAssertPattern, ListAssertTail,
        ListFunctionId, ListListFunctionId, ListListTypeId, ListLocal, ListLocalExpr, ParamLocal,
        RuntimeFunctionId, Step, StepKind,
    };
    use std::collections::HashSet;

    #[test]
    fn provisional_parameter_bindings_propagate_storage_erasure() {
        let parameter = crate::plan::TypeParameterId(0);
        let main_id = crate::plan::FunctionTemplateId::new(0);
        let source_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(0), parameter);
        let binding_local =
            crate::plan::GenericLocal::new(crate::plan::GenericLocalId(1), parameter);
        let subject = crate::plan::AssertSubject::List(crate::plan::ListLocal::generic(
            crate::plan::GenericListLocalId(0),
            parameter,
        ));
        let step = crate::plan::Step::let_generic(
            binding_local,
            "bound".into(),
            crate::plan::GenericExpr::local_get(source_local, "source".into()),
        );
        let guaranteed_failure = crate::plan::Step::assert_pattern_at(
            subject.clone(),
            crate::plan::AssertPattern::list(crate::plan::ListAssertPattern::new(
                crate::plan::ValueType::Parameter(parameter),
                vec![crate::plan::AssertPattern::Discard],
                None,
            )),
            Some(crate::plan::StringExpr::value("message".into())),
            crate::plan::PanicSite::unknown(),
            crate::plan::SourceSpan::new(1, 2),
        );
        let main = crate::plan::FunctionTemplate::new(
            main_id,
            "main".into(),
            Vec::new(),
            vec![step.clone(), guaranteed_failure.clone()],
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );
        let templates = FunctionTemplates::new(main, Vec::new(), Vec::new());
        let mut context = LoweringContext::new(
            &templates,
            SpecializationKey::monomorphic(main_id),
            RepresentationContext::new(Vec::new()),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        );
        context.reserve_locals(&SpecializationKey::monomorphic(main_id));
        let binding = crate::plan::AssertBinding::new(
            crate::plan::ParamLocal::generic(binding_local),
            "bound".into(),
            crate::plan::ValueShape::Parameter(parameter),
        );
        let direct = crate::plan::TotalBindingPattern::bind(binding.clone());
        let alias = crate::plan::TotalBindingPattern::alias(
            crate::plan::TotalBindingPattern::discard(crate::plan::ValueType::Parameter(parameter)),
            binding.clone(),
        );
        let list_alias = crate::plan::AssertPattern::alias(
            crate::plan::AssertPattern::list(crate::plan::ListAssertPattern::new(
                crate::plan::ValueType::Parameter(parameter),
                vec![crate::plan::AssertPattern::Discard],
                None,
            )),
            binding,
        );
        assert!(super::parameter_list_is_empty(&subject, &context));
        context.substitution = SpecializedTypeSubstitution::instantiate(
            &crate::plan::TypeSubstitution::from_arguments(vec![crate::plan::ValueShape::Int]),
            &SpecializedTypeSubstitution::empty(),
        );
        assert!(!super::parameter_list_is_empty(&subject, &context));
        context.substitution = SpecializedTypeSubstitution::empty();
        assert!(!super::list_pattern_requires_item(
            &crate::plan::AssertPattern::Discard,
        ));
        assert!(super::list_pattern_requires_item(&list_alias));
        assert_eq!(
            super::steps_until_never(&[step], &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::total_binding_pattern(&direct, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::total_binding_pattern(&alias, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::steps_until_never(&[guaranteed_failure], &mut context).map(|_| ()),
            Representability::Inhabited(()),
        );
    }

    #[test]
    fn lowering_removes_bit_array_pattern_names_and_preserves_typed_bindings() {
        let source = r#"
pub fn main() {
  let assert <<1 as alias, rest:bits>> = <<1, 2>>
  alias
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let function = plan.int_function(IntFunctionId(0));
        assert_eq!(function.steps().len(), 2);
        assert_eq!(
            expect_bit_array_assert_shape(&function.steps()[1]),
            (
                BitArrayLocalId(0),
                1.into(),
                IntLocalId(0),
                1,
                8.into(),
                BitArrayLocalId(1),
                1,
            ),
        );
    }

    #[test]
    #[should_panic(expected = "expected a lowered BitArray assert shape")]
    fn bit_array_assert_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() { let value = 1 value }");
        let _ = expect_bit_array_assert_shape(&plan.int_function(IntFunctionId(0)).steps()[0]);
    }

    #[test]
    fn lowering_preserves_parent_and_child_list_types_through_assert_bindings() {
        let source = r#"
pub fn main() {
  let values: List(List(Int)) = []
  let assert [first, ..rest] = values
  rest
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let main = expect_list_list_main(&plan);
        let function = plan.list_list_function(main);
        let value = expect_nested_list_binding(&function.steps()[0]);
        let parent_type = value.item().type_id();
        let assert_value = expect_nested_list_binding(&function.steps()[1]);
        let (local, pattern) = expect_list_assert(&function.steps()[2]);
        let subject_type = expect_nested_list_local(local);
        let pattern = expect_list_pattern(pattern);
        let first = expect_single_binding(pattern.elements());
        let first_type = expect_int_list_binding(first);
        let rest = expect_tail_binding(pattern.tail());
        let rest_type = expect_nested_list_local(rest);

        assert_eq!(subject_type, parent_type);
        assert_eq!(rest_type, parent_type);
        assert_eq!(assert_value.item().type_id(), parent_type);
        assert_eq!(parent_type.item_type(), first_type.list_type());
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list binding step")]
    fn nested_list_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(Int) { let value = 1 [] }");
        let main = plan.int_list_function_id(0);
        let _ = expect_nested_list_binding(&plan.int_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a list-assert step")]
    fn list_assert_fixture_guard_rejects_binding() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let _ = expect_list_assert(&plan.list_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list local")]
    fn nested_list_local_fixture_guard_rejects_int_list_local() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let first = expect_single_binding(pattern.elements());
        let first_type = expect_int_list_binding(first);
        let local = ListLocal::Int {
            local: IntListLocalId(0),
            type_id: first_type,
        };
        let _ = expect_nested_list_local(&local);
    }

    #[test]
    #[should_panic(expected = "expected a list assert pattern")]
    fn list_pattern_fixture_guard_rejects_binding_pattern() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let _ = expect_list_pattern(&pattern.elements()[0]);
    }

    #[test]
    #[should_panic(expected = "expected one assert binding")]
    fn single_binding_fixture_guard_rejects_empty_elements() {
        let _ = expect_single_binding(&[]);
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) binding")]
    fn int_list_binding_fixture_guard_rejects_nested_list_binding() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let rest = expect_tail_binding(pattern.tail());
        let local = ParamLocal::List(rest.clone());
        let _ = expect_int_list_local(&local);
    }

    #[test]
    #[should_panic(expected = "expected a bound assert tail")]
    fn tail_binding_fixture_guard_rejects_missing_tail() {
        let _ = expect_tail_binding(None);
    }

    fn assert_execution_plan() -> ExecutionPlan {
        execution_plan(
            r#"
pub fn main() {
  let values: List(List(Int)) = []
  let assert [first, ..rest] = values
  rest
}
"#,
        )
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_list_list_main(plan: &ExecutionPlan) -> ListListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::List(main)) => main,
            _ => panic!("expected a List(List) main function"),
        }
    }

    fn expect_bit_array_assert_shape(
        step: &Step,
    ) -> (
        BitArrayLocalId,
        num_bigint::BigInt,
        IntLocalId,
        u8,
        num_bigint::BigInt,
        BitArrayLocalId,
        u8,
    ) {
        if let StepKind::AssertPattern {
            subject: AssertSubject::BitArray(local),
            pattern: AssertPattern::BitArray(pattern),
            ..
        } = step.kind()
            && let [
                BitArrayPatternSegment::Int {
                    pattern: BitArrayPatternValue::Alias { pattern, binding },
                    size,
                    ..
                },
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Bind(rest),
                    size: None,
                    unit,
                },
            ] = pattern.segments()
            && let BitArrayPatternValue::Literal(value) = pattern.as_ref()
            && let BitArrayPatternSizeExpr::Value(size_value) = size.value()
        {
            return (
                *local,
                value.clone(),
                *binding.local(),
                size.unit(),
                size_value.clone(),
                *rest.local(),
                *unit,
            );
        }
        panic!("expected a lowered BitArray assert shape");
    }

    fn expect_nested_list_binding(step: &Step) -> &crate::plan::execution::ListListExpr {
        match step.kind() {
            StepKind::LetList {
                value: ListLocalExpr::List { value, .. },
            } => value,
            _ => panic!("expected a nested-list binding step"),
        }
    }

    fn expect_list_assert(step: &Step) -> (&ListLocal, &AssertPattern) {
        match step.kind() {
            StepKind::AssertPattern {
                subject: AssertSubject::List(local),
                pattern,
                ..
            } => (local, pattern),
            _ => panic!("expected a list-assert step"),
        }
    }

    fn expect_nested_list_local(local: &ListLocal) -> ListListTypeId {
        match local {
            ListLocal::List { type_id, .. } => *type_id,
            _ => panic!("expected a nested-list local"),
        }
    }

    fn expect_list_pattern(pattern: &AssertPattern) -> &ListAssertPattern {
        match pattern {
            AssertPattern::List(pattern) => pattern,
            _ => panic!("expected a list assert pattern"),
        }
    }

    fn expect_single_binding(elements: &[AssertPattern]) -> &AssertBinding {
        match elements {
            [AssertPattern::Bind(binding)] => binding,
            _ => panic!("expected one assert binding"),
        }
    }

    fn expect_int_list_binding(binding: &AssertBinding) -> crate::plan::execution::IntListTypeId {
        expect_int_list_local(binding.local())
    }

    fn expect_int_list_local(local: &ParamLocal) -> crate::plan::execution::IntListTypeId {
        match local {
            ParamLocal::List(ListLocal::Int { type_id, .. }) => *type_id,
            _ => panic!("expected a List(Int) binding"),
        }
    }

    fn expect_tail_binding(tail: Option<&ListAssertTail>) -> &ListLocal {
        match tail {
            Some(ListAssertTail::Bind(binding)) => binding.local(),
            _ => panic!("expected a bound assert tail"),
        }
    }
}
