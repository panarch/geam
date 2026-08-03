use crate::plan::{
    BitArrayBindingPattern, BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment,
    BitArrayPatternSize, BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern,
    Endianness, FloatLocalId, IntLocalId, LocalId, PatternBinding, Signedness, StringEncoding,
    UtfCodepointLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidTypedAstReason, PlanError, UnsupportedBitArraySegmentReason};
use gleam_core::ast::{
    BitArrayOption, BitArraySegment, BitArraySize, Constant, IntOperator, Pattern,
};
use gleam_core::strings::convert_string_escape_chars;
use gleam_core::type_::{Type, ValueConstructor, ValueConstructorVariant};
use std::sync::Arc;

pub(in crate::planner) fn plan_bit_array_pattern(
    segments: Vec<BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>>,
    context: &mut PlanContext<'_>,
) -> Result<(BitArrayPattern, bool), PlanError> {
    let is_total = is_total_pattern(&segments);
    let mut planned = Vec::with_capacity(segments.len());
    let segment_count = segments.len();
    for (index, segment) in segments.into_iter().enumerate() {
        let segment = plan_segment(segment, context)?;
        if index + 1 < segment_count
            && matches!(segment, BitArrayPatternSegment::Bits { size: None, .. })
        {
            return Err(invalid_pattern());
        }
        planned.push(segment);
    }
    Ok((BitArrayPattern::new(planned), is_total))
}

fn plan_segment(
    segment: BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayPatternSegment, PlanError> {
    if segment
        .options
        .iter()
        .any(|option| matches!(option, BitArrayOption::Native { .. }))
    {
        return unsupported_segment(UnsupportedBitArraySegmentReason::NativeEndianness);
    }
    let kind = segment_kind(&segment)?;
    validate_segment_options(&segment, kind)?;
    let endianness = segment_endianness(&segment);
    let unit = segment.unit();
    let explicit_size = segment
        .size()
        .map(|size| plan_segment_size(size, unit, context))
        .transpose()?;
    match kind {
        SegmentKind::Int => Ok(BitArrayPatternSegment::Int {
            pattern: plan_int_pattern(*segment.value, context)?,
            size: explicit_size.unwrap_or_else(|| BitArrayPatternSize::new(size_value(8), 1)),
            endianness,
            signedness: if segment
                .options
                .iter()
                .any(|option| matches!(option, BitArrayOption::Signed { .. }))
            {
                Signedness::Signed
            } else {
                Signedness::Unsigned
            },
        }),
        SegmentKind::Float => Ok(BitArrayPatternSegment::Float {
            pattern: plan_float_pattern(*segment.value, context)?,
            size: explicit_size.unwrap_or_else(|| BitArrayPatternSize::new(size_value(64), 1)),
            endianness,
        }),
        SegmentKind::Bits => Ok(BitArrayPatternSegment::Bits {
            pattern: plan_bits_pattern(*segment.value, context)?,
            size: explicit_size,
            unit,
        }),
        SegmentKind::String(encoding) => Ok(BitArrayPatternSegment::String {
            pattern: plan_string_pattern(*segment.value)?,
            encoding,
        }),
        SegmentKind::UtfCodepoint(encoding) => Ok(BitArrayPatternSegment::UtfCodepoint {
            pattern: plan_utf_codepoint_pattern(*segment.value, context)?,
            encoding,
        }),
    }
}

#[derive(Clone, Copy)]
enum SegmentKind {
    Int,
    Float,
    Bits,
    String(StringEncoding),
    UtfCodepoint(StringEncoding),
}

fn segment_kind(
    segment: &BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>,
) -> Result<SegmentKind, PlanError> {
    let mut kind = None;
    for option in &segment.options {
        let next = match option {
            BitArrayOption::Bytes { .. } | BitArrayOption::Bits { .. } => Some(SegmentKind::Bits),
            BitArrayOption::Int { .. } => Some(SegmentKind::Int),
            BitArrayOption::Float { .. } => Some(SegmentKind::Float),
            BitArrayOption::Utf8 { .. } => Some(SegmentKind::String(StringEncoding::Utf8)),
            BitArrayOption::Utf16 { .. } => Some(SegmentKind::String(StringEncoding::Utf16(
                segment_endianness(segment),
            ))),
            BitArrayOption::Utf32 { .. } => Some(SegmentKind::String(StringEncoding::Utf32(
                segment_endianness(segment),
            ))),
            BitArrayOption::Utf8Codepoint { .. } => {
                Some(SegmentKind::UtfCodepoint(StringEncoding::Utf8))
            }
            BitArrayOption::Utf16Codepoint { .. } => Some(SegmentKind::UtfCodepoint(
                StringEncoding::Utf16(segment_endianness(segment)),
            )),
            BitArrayOption::Utf32Codepoint { .. } => Some(SegmentKind::UtfCodepoint(
                StringEncoding::Utf32(segment_endianness(segment)),
            )),
            BitArrayOption::Signed { .. }
            | BitArrayOption::Unsigned { .. }
            | BitArrayOption::Big { .. }
            | BitArrayOption::Little { .. }
            | BitArrayOption::Native { .. }
            | BitArrayOption::Size { .. }
            | BitArrayOption::Unit { .. } => None,
        };
        if let Some(next) = next {
            if kind.is_some() {
                return Err(invalid_pattern());
            }
            kind = Some(next);
        }
    }
    if let Some(kind) = kind {
        return Ok(kind);
    }
    if matches!(segment.value_unwrapping_assign(), Pattern::String { .. }) {
        Ok(SegmentKind::String(StringEncoding::Utf8))
    } else {
        Ok(SegmentKind::Int)
    }
}

fn validate_segment_options(
    segment: &BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>,
    kind: SegmentKind,
) -> Result<(), PlanError> {
    let mut kind_count = 0;
    let mut signedness_count = 0;
    let mut endianness_count = 0;
    let mut size_count = 0;
    let mut unit_count = 0;
    let mut zero_unit = false;

    for option in &segment.options {
        match option {
            BitArrayOption::Bytes { .. }
            | BitArrayOption::Bits { .. }
            | BitArrayOption::Int { .. }
            | BitArrayOption::Float { .. }
            | BitArrayOption::Utf8 { .. }
            | BitArrayOption::Utf16 { .. }
            | BitArrayOption::Utf32 { .. }
            | BitArrayOption::Utf8Codepoint { .. }
            | BitArrayOption::Utf16Codepoint { .. }
            | BitArrayOption::Utf32Codepoint { .. } => kind_count += 1,
            BitArrayOption::Signed { .. } | BitArrayOption::Unsigned { .. } => {
                signedness_count += 1;
            }
            BitArrayOption::Big { .. }
            | BitArrayOption::Little { .. }
            | BitArrayOption::Native { .. } => endianness_count += 1,
            BitArrayOption::Size { .. } => size_count += 1,
            BitArrayOption::Unit { value, .. } => {
                unit_count += 1;
                zero_unit |= *value == 0;
            }
        }
    }

    let duplicate_option = kind_count
        .max(signedness_count)
        .max(endianness_count)
        .max(size_count)
        .max(unit_count)
        > 1;
    let invalid_unit = [unit_count > size_count, zero_unit].contains(&true);
    let incompatible_modifier = match kind {
        SegmentKind::Int => false,
        SegmentKind::Float => signedness_count != 0,
        SegmentKind::Bits => signedness_count + endianness_count != 0,
        SegmentKind::String(StringEncoding::Utf8) => {
            signedness_count + endianness_count + size_count + unit_count != 0
        }
        SegmentKind::String(StringEncoding::Utf16(_) | StringEncoding::Utf32(_)) => {
            signedness_count + size_count + unit_count != 0
        }
        SegmentKind::UtfCodepoint(StringEncoding::Utf8) => {
            signedness_count + endianness_count + size_count + unit_count != 0
        }
        SegmentKind::UtfCodepoint(StringEncoding::Utf16(_) | StringEncoding::Utf32(_)) => {
            signedness_count + size_count + unit_count != 0
        }
    };

    if [duplicate_option, invalid_unit, incompatible_modifier].contains(&true) {
        Err(invalid_pattern())
    } else {
        Ok(())
    }
}

fn segment_endianness(segment: &BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>) -> Endianness {
    if segment
        .options
        .iter()
        .any(|option| matches!(option, BitArrayOption::Little { .. }))
    {
        Endianness::Little
    } else {
        Endianness::Big
    }
}

fn plan_segment_size(
    pattern: &Pattern<Arc<Type>>,
    unit: u8,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayPatternSize, PlanError> {
    let Pattern::BitArraySize(size) = pattern else {
        return Err(invalid_pattern());
    };
    Ok(BitArrayPatternSize::new(plan_size(size, context)?, unit))
}

fn plan_size(
    size: &BitArraySize<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayPatternSizeExpr, PlanError> {
    match size {
        BitArraySize::Int { int_value, .. } => {
            Ok(BitArrayPatternSizeExpr::value(int_value.clone()))
        }
        BitArraySize::Variable {
            name, constructor, ..
        } => {
            let constructor = constructor
                .as_deref()
                .cloned()
                .ok_or_else(invalid_pattern)?;
            plan_size_variable(name.clone(), constructor, context)
        }
        BitArraySize::BinaryOperator {
            operator,
            left,
            right,
            ..
        } => {
            let left = plan_size(left, context)?;
            let right = plan_size(right, context)?;
            Ok(match operator {
                IntOperator::Add => BitArrayPatternSizeExpr::add(left, right),
                IntOperator::Subtract => BitArrayPatternSizeExpr::subtract(left, right),
                IntOperator::Multiply => BitArrayPatternSizeExpr::multiply(left, right),
                IntOperator::Divide => BitArrayPatternSizeExpr::divide(left, right),
                IntOperator::Remainder => BitArrayPatternSizeExpr::remainder(left, right),
            })
        }
        BitArraySize::Block { inner, .. } => plan_size(inner, context),
    }
}

fn plan_size_variable(
    name: ecow::EcoString,
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<BitArrayPatternSizeExpr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => match context.lookup_local(&name) {
            Some((LocalId::Int(local), ValueType::Int)) => {
                Ok(BitArrayPatternSizeExpr::local_get(local, name))
            }
            Some(_) => Err(invalid_pattern()),
            None => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal { name },
            }),
        },
        ValueConstructorVariant::ModuleConstant {
            module,
            name: constant_name,
            literal,
            ..
        } => {
            context.module_constant_instantiation(
                &module,
                &constant_name,
                &crate::plan::ValueShape::Int,
            )?;
            plan_size_constant(literal, context)
        }
        _ => Err(invalid_pattern()),
    }
}

fn plan_size_constant(
    constant: Constant<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<BitArrayPatternSizeExpr, PlanError> {
    match constant {
        Constant::Int { int_value, .. } => Ok(BitArrayPatternSizeExpr::value(int_value)),
        Constant::Var {
            name, constructor, ..
        } => {
            let constructor = constructor
                .map(|constructor| *constructor)
                .ok_or_else(invalid_pattern)?;
            plan_size_variable(name, constructor, context)
        }
        _ => Err(invalid_pattern()),
    }
}

fn size_value(value: u8) -> BitArrayPatternSizeExpr {
    BitArrayPatternSizeExpr::value(value.into())
}

fn plan_int_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayPatternValue<num_bigint::BigInt, IntLocalId>, PlanError> {
    match pattern {
        Pattern::Int { int_value, .. } => Ok(BitArrayPatternValue::Literal(int_value)),
        Pattern::Variable { name, type_, .. } if type_.is_int() => {
            let local = context.define_int_local(name.clone());
            Ok(BitArrayPatternValue::Bind(PatternBinding::new(local, name)))
        }
        Pattern::Discard { type_, .. } if type_.is_int() => Ok(BitArrayPatternValue::Discard),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_int_pattern(*pattern, context)?;
            let local = context.define_int_local(name.clone());
            Ok(BitArrayPatternValue::Alias {
                pattern: Box::new(pattern),
                binding: PatternBinding::new(local, name),
            })
        }
        _ => Err(invalid_pattern()),
    }
}

fn plan_float_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayPatternValue<f64, FloatLocalId>, PlanError> {
    match pattern {
        Pattern::Float { float_value, .. } => {
            Ok(BitArrayPatternValue::Literal(float_value.value()))
        }
        Pattern::Variable { name, type_, .. } if type_.is_float() => {
            let local = context.define_float_local(name.clone());
            Ok(BitArrayPatternValue::Bind(PatternBinding::new(local, name)))
        }
        Pattern::Discard { type_, .. } if type_.is_float() => Ok(BitArrayPatternValue::Discard),
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_float_pattern(*pattern, context)?;
            let local = context.define_float_local(name.clone());
            Ok(BitArrayPatternValue::Alias {
                pattern: Box::new(pattern),
                binding: PatternBinding::new(local, name),
            })
        }
        _ => Err(invalid_pattern()),
    }
}

fn plan_bits_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayBindingPattern<BitArrayLocalId>, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if type_.is_bit_array() => {
            let local = context.define_bit_array_local(name.clone());
            Ok(BitArrayBindingPattern::Bind(PatternBinding::new(
                local, name,
            )))
        }
        Pattern::Discard { type_, .. } if type_.is_bit_array() => {
            Ok(BitArrayBindingPattern::Discard)
        }
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_bits_pattern(*pattern, context)?;
            let local = context.define_bit_array_local(name.clone());
            Ok(BitArrayBindingPattern::Alias {
                pattern: Box::new(pattern),
                binding: PatternBinding::new(local, name),
            })
        }
        _ => Err(invalid_pattern()),
    }
}

fn plan_string_pattern(pattern: Pattern<Arc<Type>>) -> Result<BitArrayStringPattern, PlanError> {
    match pattern {
        Pattern::String { value, .. } => Ok(BitArrayStringPattern::Literal(
            convert_string_escape_chars(&value),
        )),
        Pattern::Discard { .. } => Ok(BitArrayStringPattern::Discard),
        Pattern::Variable { .. } | Pattern::Assign { .. } => Err(invalid_pattern()),
        _ => Err(invalid_pattern()),
    }
}

fn plan_utf_codepoint_pattern(
    pattern: Pattern<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayBindingPattern<UtfCodepointLocalId>, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if type_.is_utf_codepoint() => {
            let local = context.define_utf_codepoint_local(name.clone());
            Ok(BitArrayBindingPattern::Bind(PatternBinding::new(
                local, name,
            )))
        }
        Pattern::Discard { type_, .. } if type_.is_utf_codepoint() => {
            Ok(BitArrayBindingPattern::Discard)
        }
        Pattern::Assign { name, pattern, .. } => {
            let pattern = plan_utf_codepoint_pattern(*pattern, context)?;
            let local = context.define_utf_codepoint_local(name.clone());
            Ok(BitArrayBindingPattern::Alias {
                pattern: Box::new(pattern),
                binding: PatternBinding::new(local, name),
            })
        }
        _ => Err(invalid_pattern()),
    }
}

fn is_total_pattern(segments: &[BitArraySegment<Pattern<Arc<Type>>, Arc<Type>>]) -> bool {
    segments.len() == 1
        && segments[0].size().is_none()
        && matches!(
            segments[0].options.as_slice(),
            [BitArrayOption::Bits { .. }]
        )
}

fn unsupported_segment<T>(reason: UnsupportedBitArraySegmentReason) -> Result<T, PlanError> {
    Err(PlanError::UnsupportedBitArraySegment { reason })
}

fn invalid_pattern() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::InvalidPattern,
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BitArrayBindingPattern, BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment,
        BitArrayPatternSizeExpr, BitArrayStringPattern, Endianness, FloatLocalId, IntLocalId,
        PatternBinding, Signedness, StringEncoding, UtfCodepointLocalId,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::error::{
        InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason,
    };
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use gleam_core::ast::Publicity;
    use gleam_core::ast::{BitArrayOption, BitArraySegment, Constant, Pattern};
    use gleam_core::type_::{
        self, Deprecation, ValueConstructor, ValueConstructorVariant, error::VariableOrigin,
    };

    #[test]
    fn reject_profile_bit_array_pattern_native_endianness() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  case <<1>> {
    <<value:native>> -> value
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }

    #[test]
    fn reject_margin_bit_array_pattern_invalid_typed_ast_shapes() {
        let discard = |type_| Pattern::Discard {
            location: dummy_span(),
            name: "_".into(),
            type_,
        };
        let segment = |value, options, type_| BitArraySegment {
            location: dummy_span(),
            value: Box::new(value),
            options,
            type_,
        };
        let alias = |name: &str, pattern: Pattern<std::sync::Arc<gleam_core::type_::Type>>| {
            Pattern::Assign {
                name: name.into(),
                location: dummy_span(),
                pattern: Box::new(pattern),
            }
        };
        let local_constructor = |type_| ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_,
            variant: ValueConstructorVariant::LocalVariable {
                location: dummy_span(),
                origin: VariableOrigin::generated(),
            },
        };
        let size = |value| BitArrayOption::Size {
            location: dummy_span(),
            value: Box::new(Pattern::BitArraySize(value)),
            short_form: false,
        };
        let int_size = || gleam_core::ast::BitArraySize::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        };
        let missing_constructor = || gleam_core::ast::BitArraySize::Variable {
            location: dummy_span(),
            name: "missing".into(),
            constructor: None,
            type_: type_::int(),
        };
        let invalid = || {
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidPattern,
            })
        };
        let mut cases = Vec::new();

        cases.push((
            vec![segment(
                discard(type_::int()),
                vec![BitArrayOption::Utf8Codepoint {
                    location: dummy_span(),
                }],
                type_::int(),
            )],
            invalid(),
        ));
        cases.push((
            vec![segment(
                alias("codepoint_alias", discard(type_::int())),
                vec![BitArrayOption::Utf8Codepoint {
                    location: dummy_span(),
                }],
                type_::utf_codepoint(),
            )],
            invalid(),
        ));
        for options in [
            vec![
                BitArrayOption::Utf8Codepoint {
                    location: dummy_span(),
                },
                BitArrayOption::Big {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf16Codepoint {
                    location: dummy_span(),
                },
                BitArrayOption::Signed {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf32Codepoint {
                    location: dummy_span(),
                },
                size(int_size()),
            ],
        ] {
            cases.push((
                vec![segment(
                    discard(type_::utf_codepoint()),
                    options,
                    type_::utf_codepoint(),
                )],
                invalid(),
            ));
        }

        for options in [
            vec![
                BitArrayOption::Int {
                    location: dummy_span(),
                },
                BitArrayOption::Int {
                    location: dummy_span(),
                },
            ],
            vec![BitArrayOption::Unit {
                location: dummy_span(),
                value: 2,
            }],
            vec![
                BitArrayOption::Size {
                    location: dummy_span(),
                    value: Box::new(Pattern::BitArraySize(gleam_core::ast::BitArraySize::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    })),
                    short_form: false,
                },
                BitArrayOption::Unit {
                    location: dummy_span(),
                    value: 0,
                },
            ],
            vec![
                BitArrayOption::Float {
                    location: dummy_span(),
                },
                BitArrayOption::Signed {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Bits {
                    location: dummy_span(),
                },
                BitArrayOption::Little {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf8 {
                    location: dummy_span(),
                },
                BitArrayOption::Big {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf16 {
                    location: dummy_span(),
                },
                BitArrayOption::Signed {
                    location: dummy_span(),
                },
            ],
        ] {
            cases.push((
                vec![segment(discard(type_::int()), options, type_::int())],
                invalid(),
            ));
        }
        cases.push((
            vec![segment(
                Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::string(),
                    origin: gleam_core::type_::error::VariableOrigin::generated(),
                },
                vec![BitArrayOption::Utf8 {
                    location: dummy_span(),
                }],
                type_::string(),
            )],
            invalid(),
        ));
        cases.push((
            vec![
                segment(
                    discard(type_::bit_array()),
                    vec![BitArrayOption::Bits {
                        location: dummy_span(),
                    }],
                    type_::bit_array(),
                ),
                segment(
                    discard(type_::int()),
                    vec![BitArrayOption::Int {
                        location: dummy_span(),
                    }],
                    type_::int(),
                ),
            ],
            invalid(),
        ));

        for (value, options, type_, expected) in [
            (
                discard(type_::int()),
                vec![BitArrayOption::Size {
                    location: dummy_span(),
                    value: Box::new(discard(type_::int())),
                    short_form: false,
                }],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::string()),
                vec![BitArrayOption::Int {
                    location: dummy_span(),
                }],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![BitArrayOption::Float {
                    location: dummy_span(),
                }],
                type_::float(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_::bit_array(),
                super::invalid_pattern(),
            ),
            (
                Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                },
                vec![BitArrayOption::Utf8 {
                    location: dummy_span(),
                }],
                type_::string(),
                super::invalid_pattern(),
            ),
            (
                alias("int_alias", discard(type_::string())),
                vec![BitArrayOption::Int {
                    location: dummy_span(),
                }],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                alias("float_alias", discard(type_::int())),
                vec![BitArrayOption::Float {
                    location: dummy_span(),
                }],
                type_::float(),
                super::invalid_pattern(),
            ),
            (
                alias("bits_alias", discard(type_::int())),
                vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_::bit_array(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![size(missing_constructor())],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![size(gleam_core::ast::BitArraySize::Variable {
                    location: dummy_span(),
                    name: "unknown".into(),
                    constructor: Some(Box::new(local_constructor(type_::int()))),
                    type_: type_::int(),
                })],
                type_::int(),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::UnknownLocal {
                        name: "unknown".into(),
                    },
                },
            ),
            (
                discard(type_::int()),
                vec![size(gleam_core::ast::BitArraySize::Variable {
                    location: dummy_span(),
                    name: "string_size".into(),
                    constructor: Some(Box::new(local_constructor(type_::string()))),
                    type_: type_::int(),
                })],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![size(gleam_core::ast::BitArraySize::BinaryOperator {
                    location: dummy_span(),
                    operator: gleam_core::ast::IntOperator::Add,
                    left: Box::new(missing_constructor()),
                    right: Box::new(int_size()),
                })],
                type_::int(),
                super::invalid_pattern(),
            ),
            (
                discard(type_::int()),
                vec![size(gleam_core::ast::BitArraySize::BinaryOperator {
                    location: dummy_span(),
                    operator: gleam_core::ast::IntOperator::Add,
                    left: Box::new(int_size()),
                    right: Box::new(missing_constructor()),
                })],
                type_::int(),
                super::invalid_pattern(),
            ),
        ] {
            cases.push((vec![segment(value, options, type_)], Err(expected)));
        }

        for (segments, expected) in cases {
            let module_name = "main".into();
            let functions = std::collections::HashMap::new();
            let mut anonymous = AnonymousFunctions::default();
            let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
            context.define_int_local("int_size".into());
            context.define_string_local("string_size".into());

            assert_eq!(
                super::plan_bit_array_pattern(segments, &mut context),
                expected,
            );
        }
    }

    #[test]
    fn plan_bit_array_pattern_preserves_supported_segment_shapes() {
        let discard = |type_| Pattern::Discard {
            location: dummy_span(),
            name: "_".into(),
            type_,
        };
        let segment = |value, options, type_| BitArraySegment {
            location: dummy_span(),
            value: Box::new(value),
            options,
            type_,
        };
        let alias = |name: &str, pattern: Pattern<std::sync::Arc<gleam_core::type_::Type>>| {
            Pattern::Assign {
                name: name.into(),
                location: dummy_span(),
                pattern: Box::new(pattern),
            }
        };
        let local_constructor = |type_| ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_,
            variant: ValueConstructorVariant::LocalVariable {
                location: dummy_span(),
                origin: VariableOrigin::generated(),
            },
        };
        let size = |value| BitArrayOption::Size {
            location: dummy_span(),
            value: Box::new(Pattern::BitArraySize(value)),
            short_form: false,
        };
        let int_size = || gleam_core::ast::BitArraySize::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
        };
        let mut cases = Vec::new();

        cases.push((
            vec![segment(
                discard(type_::bit_array()),
                vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                type_::bit_array(),
            )],
            Ok((
                BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: None,
                    unit: 1,
                }]),
                true,
            )),
        ));
        cases.push((
            vec![
                segment(
                    Pattern::Variable {
                        location: dummy_span(),
                        name: "codepoint".into(),
                        type_: type_::utf_codepoint(),
                        origin: VariableOrigin::generated(),
                    },
                    vec![BitArrayOption::Utf8Codepoint {
                        location: dummy_span(),
                    }],
                    type_::utf_codepoint(),
                ),
                segment(
                    discard(type_::utf_codepoint()),
                    vec![
                        BitArrayOption::Utf16Codepoint {
                            location: dummy_span(),
                        },
                        BitArrayOption::Little {
                            location: dummy_span(),
                        },
                    ],
                    type_::utf_codepoint(),
                ),
                segment(
                    alias("codepoint_alias", discard(type_::utf_codepoint())),
                    vec![BitArrayOption::Utf32Codepoint {
                        location: dummy_span(),
                    }],
                    type_::utf_codepoint(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::UtfCodepoint {
                        pattern: BitArrayBindingPattern::Bind(PatternBinding::new(
                            UtfCodepointLocalId(0),
                            "codepoint".into(),
                        )),
                        encoding: StringEncoding::Utf8,
                    },
                    BitArrayPatternSegment::UtfCodepoint {
                        pattern: BitArrayBindingPattern::Discard,
                        encoding: StringEncoding::Utf16(Endianness::Little),
                    },
                    BitArrayPatternSegment::UtfCodepoint {
                        pattern: BitArrayBindingPattern::Alias {
                            pattern: Box::new(BitArrayBindingPattern::Discard),
                            binding: PatternBinding::new(
                                UtfCodepointLocalId(1),
                                "codepoint_alias".into(),
                            ),
                        },
                        encoding: StringEncoding::Utf32(Endianness::Big),
                    },
                ]),
                false,
            )),
        ));
        cases.push((
            vec![
                segment(
                    Pattern::Float {
                        location: dummy_span(),
                        value: "1.0".into(),
                        float_value: gleam_core::parse::LiteralFloatValue::ONE,
                    },
                    vec![BitArrayOption::Float {
                        location: dummy_span(),
                    }],
                    type_::float(),
                ),
                segment(
                    Pattern::String {
                        location: dummy_span(),
                        value: "A".into(),
                    },
                    Vec::new(),
                    type_::string(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::Float {
                        pattern: crate::plan::BitArrayPatternValue::Literal(1.0),
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(64.into()),
                            1,
                        ),
                        endianness: crate::plan::Endianness::Big,
                    },
                    BitArrayPatternSegment::String {
                        pattern: BitArrayStringPattern::Literal("A".into()),
                        encoding: StringEncoding::Utf8,
                    },
                ]),
                false,
            )),
        ));
        let explicit_size = |value| {
            vec![BitArrayOption::Size {
                location: dummy_span(),
                value: Box::new(Pattern::BitArraySize(value)),
                short_form: false,
            }]
        };
        cases.push((
            vec![
                segment(
                    Pattern::Int {
                        location: dummy_span(),
                        value: "258".into(),
                        int_value: 258.into(),
                    },
                    vec![
                        BitArrayOption::Int {
                            location: dummy_span(),
                        },
                        BitArrayOption::Signed {
                            location: dummy_span(),
                        },
                        BitArrayOption::Little {
                            location: dummy_span(),
                        },
                        size(int_size()),
                    ],
                    type_::int(),
                ),
                segment(
                    Pattern::Variable {
                        location: dummy_span(),
                        name: "integer".into(),
                        type_: type_::int(),
                        origin: VariableOrigin::generated(),
                    },
                    Vec::new(),
                    type_::int(),
                ),
                segment(discard(type_::int()), Vec::new(), type_::int()),
                segment(
                    alias("integer_alias", discard(type_::int())),
                    Vec::new(),
                    type_::int(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::Int {
                        pattern: crate::plan::BitArrayPatternValue::Literal(258.into()),
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(1.into()),
                            1,
                        ),
                        endianness: Endianness::Little,
                        signedness: Signedness::Signed,
                    },
                    BitArrayPatternSegment::Int {
                        pattern: crate::plan::BitArrayPatternValue::Bind(PatternBinding::new(
                            IntLocalId(1),
                            "integer".into(),
                        )),
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(8.into()),
                            1,
                        ),
                        endianness: Endianness::Big,
                        signedness: Signedness::Unsigned,
                    },
                    BitArrayPatternSegment::Int {
                        pattern: crate::plan::BitArrayPatternValue::Discard,
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(8.into()),
                            1,
                        ),
                        endianness: Endianness::Big,
                        signedness: Signedness::Unsigned,
                    },
                    BitArrayPatternSegment::Int {
                        pattern: crate::plan::BitArrayPatternValue::Alias {
                            pattern: Box::new(crate::plan::BitArrayPatternValue::Discard),
                            binding: PatternBinding::new(IntLocalId(2), "integer_alias".into()),
                        },
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(8.into()),
                            1,
                        ),
                        endianness: Endianness::Big,
                        signedness: Signedness::Unsigned,
                    },
                ]),
                false,
            )),
        ));
        cases.push((
            vec![
                segment(
                    Pattern::Variable {
                        location: dummy_span(),
                        name: "float".into(),
                        type_: type_::float(),
                        origin: VariableOrigin::generated(),
                    },
                    vec![
                        BitArrayOption::Float {
                            location: dummy_span(),
                        },
                        BitArrayOption::Little {
                            location: dummy_span(),
                        },
                        size(int_size()),
                    ],
                    type_::float(),
                ),
                segment(
                    discard(type_::float()),
                    vec![BitArrayOption::Float {
                        location: dummy_span(),
                    }],
                    type_::float(),
                ),
                segment(
                    alias("float_alias", discard(type_::float())),
                    vec![BitArrayOption::Float {
                        location: dummy_span(),
                    }],
                    type_::float(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::Float {
                        pattern: crate::plan::BitArrayPatternValue::Bind(PatternBinding::new(
                            FloatLocalId(0),
                            "float".into(),
                        )),
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(1.into()),
                            1,
                        ),
                        endianness: Endianness::Little,
                    },
                    BitArrayPatternSegment::Float {
                        pattern: crate::plan::BitArrayPatternValue::Discard,
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(64.into()),
                            1,
                        ),
                        endianness: Endianness::Big,
                    },
                    BitArrayPatternSegment::Float {
                        pattern: crate::plan::BitArrayPatternValue::Alias {
                            pattern: Box::new(crate::plan::BitArrayPatternValue::Discard),
                            binding: PatternBinding::new(FloatLocalId(1), "float_alias".into()),
                        },
                        size: crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(64.into()),
                            1,
                        ),
                        endianness: Endianness::Big,
                    },
                ]),
                false,
            )),
        ));
        cases.push((
            vec![
                segment(
                    Pattern::Variable {
                        location: dummy_span(),
                        name: "head".into(),
                        type_: type_::bit_array(),
                        origin: VariableOrigin::generated(),
                    },
                    vec![
                        BitArrayOption::Bits {
                            location: dummy_span(),
                        },
                        size(int_size()),
                    ],
                    type_::bit_array(),
                ),
                segment(
                    discard(type_::bit_array()),
                    vec![
                        BitArrayOption::Bytes {
                            location: dummy_span(),
                        },
                        size(int_size()),
                    ],
                    type_::bit_array(),
                ),
                segment(
                    alias("tail", discard(type_::bit_array())),
                    vec![BitArrayOption::Bits {
                        location: dummy_span(),
                    }],
                    type_::bit_array(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::Bits {
                        pattern: BitArrayBindingPattern::Bind(PatternBinding::new(
                            BitArrayLocalId(0),
                            "head".into(),
                        )),
                        size: Some(crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(1.into()),
                            1,
                        )),
                        unit: 1,
                    },
                    BitArrayPatternSegment::Bits {
                        pattern: BitArrayBindingPattern::Discard,
                        size: Some(crate::plan::BitArrayPatternSize::new(
                            BitArrayPatternSizeExpr::value(1.into()),
                            8,
                        )),
                        unit: 8,
                    },
                    BitArrayPatternSegment::Bits {
                        pattern: BitArrayBindingPattern::Alias {
                            pattern: Box::new(BitArrayBindingPattern::Discard),
                            binding: PatternBinding::new(BitArrayLocalId(1), "tail".into()),
                        },
                        size: None,
                        unit: 1,
                    },
                ]),
                false,
            )),
        ));
        cases.push((
            vec![
                segment(
                    discard(type_::string()),
                    vec![BitArrayOption::Utf8 {
                        location: dummy_span(),
                    }],
                    type_::string(),
                ),
                segment(
                    Pattern::String {
                        location: dummy_span(),
                        value: "A".into(),
                    },
                    vec![
                        BitArrayOption::Utf16 {
                            location: dummy_span(),
                        },
                        BitArrayOption::Little {
                            location: dummy_span(),
                        },
                    ],
                    type_::string(),
                ),
                segment(
                    Pattern::String {
                        location: dummy_span(),
                        value: "B".into(),
                    },
                    vec![
                        BitArrayOption::Utf32 {
                            location: dummy_span(),
                        },
                        BitArrayOption::Big {
                            location: dummy_span(),
                        },
                    ],
                    type_::string(),
                ),
            ],
            Ok((
                BitArrayPattern::new(vec![
                    BitArrayPatternSegment::String {
                        pattern: BitArrayStringPattern::Discard,
                        encoding: StringEncoding::Utf8,
                    },
                    BitArrayPatternSegment::String {
                        pattern: BitArrayStringPattern::Literal("A".into()),
                        encoding: StringEncoding::Utf16(Endianness::Little),
                    },
                    BitArrayPatternSegment::String {
                        pattern: BitArrayStringPattern::Literal("B".into()),
                        encoding: StringEncoding::Utf32(Endianness::Big),
                    },
                ]),
                false,
            )),
        ));

        let arithmetic_size = gleam_core::ast::BitArraySize::Block {
            location: dummy_span(),
            inner: Box::new(gleam_core::ast::BitArraySize::BinaryOperator {
                location: dummy_span(),
                operator: gleam_core::ast::IntOperator::Add,
                left: Box::new(gleam_core::ast::BitArraySize::BinaryOperator {
                    location: dummy_span(),
                    operator: gleam_core::ast::IntOperator::Subtract,
                    left: Box::new(gleam_core::ast::BitArraySize::Variable {
                        location: dummy_span(),
                        name: "int_size".into(),
                        constructor: Some(Box::new(local_constructor(type_::int()))),
                        type_: type_::int(),
                    }),
                    right: Box::new(int_size()),
                }),
                right: Box::new(gleam_core::ast::BitArraySize::BinaryOperator {
                    location: dummy_span(),
                    operator: gleam_core::ast::IntOperator::Multiply,
                    left: Box::new(gleam_core::ast::BitArraySize::BinaryOperator {
                        location: dummy_span(),
                        operator: gleam_core::ast::IntOperator::Divide,
                        left: Box::new(int_size()),
                        right: Box::new(int_size()),
                    }),
                    right: Box::new(gleam_core::ast::BitArraySize::BinaryOperator {
                        location: dummy_span(),
                        operator: gleam_core::ast::IntOperator::Remainder,
                        left: Box::new(int_size()),
                        right: Box::new(int_size()),
                    }),
                }),
            }),
        };
        cases.push((
            vec![segment(
                discard(type_::int()),
                explicit_size(arithmetic_size),
                type_::int(),
            )],
            Ok((
                BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
                    pattern: crate::plan::BitArrayPatternValue::Discard,
                    size: crate::plan::BitArrayPatternSize::new(
                        BitArrayPatternSizeExpr::add(
                            BitArrayPatternSizeExpr::subtract(
                                BitArrayPatternSizeExpr::local_get(
                                    IntLocalId(0),
                                    "int_size".into(),
                                ),
                                BitArrayPatternSizeExpr::value(1.into()),
                            ),
                            BitArrayPatternSizeExpr::multiply(
                                BitArrayPatternSizeExpr::divide(
                                    BitArrayPatternSizeExpr::value(1.into()),
                                    BitArrayPatternSizeExpr::value(1.into()),
                                ),
                                BitArrayPatternSizeExpr::remainder(
                                    BitArrayPatternSizeExpr::value(1.into()),
                                    BitArrayPatternSizeExpr::value(1.into()),
                                ),
                            ),
                        ),
                        1,
                    ),
                    endianness: Endianness::Big,
                    signedness: Signedness::Unsigned,
                }]),
                false,
            )),
        ));

        for (segments, expected) in cases {
            let module_name = "main".into();
            let functions = std::collections::HashMap::new();
            let mut anonymous = AnonymousFunctions::default();
            let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
            context.define_int_local("int_size".into());
            context.define_string_local("string_size".into());

            assert_eq!(
                super::plan_bit_array_pattern(segments, &mut context),
                expected,
            );
        }
    }

    #[test]
    fn reject_margin_bit_array_pattern_size_invalid_constant_constructors() {
        let module = compile(
            r#"
const size = 8
const alias = size
pub fn main() { 0 }
"#,
        );
        let module_constant = |name: &str, module_name: &str, literal| {
            let constant = module
                .definitions
                .constants
                .iter()
                .find(|constant| constant.name == name)
                .expect("module constant should exist");

            ValueConstructor {
                publicity: constant.publicity,
                deprecation: constant.deprecation.clone(),
                type_: constant.type_.clone(),
                variant: ValueConstructorVariant::ModuleConstant {
                    documentation: None,
                    location: constant.location,
                    module: module_name.into(),
                    name: name.into(),
                    literal,
                    implementations: constant.implementations,
                },
            }
        };
        let constant_value = |name: &str| {
            module
                .definitions
                .constants
                .iter()
                .find(|constant| constant.name == name)
                .expect("module constant should exist")
                .value
                .as_ref()
                .clone()
        };
        let missing_alias = module_constant(
            "alias",
            "main",
            Constant::Var {
                location: dummy_span(),
                module: None,
                name: "missing".into(),
                constructor: None,
                type_: type_::int(),
            },
        );
        let string = module_constant(
            "size",
            "main",
            Constant::String {
                location: dummy_span(),
                value: "wrong".into(),
            },
        );
        let external = module_constant("size", "other", constant_value("size"));
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_size_variable("missing_alias".into(), missing_alias, &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "alias".into(),
                    reason: InvalidModuleReferenceReason::MissingConstant,
                },
            }),
        );
        assert_eq!(
            super::plan_size_variable("string".into(), string, &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "size".into(),
                    reason: InvalidModuleReferenceReason::MissingConstant,
                },
            }),
        );
        assert_eq!(
            super::plan_size_variable("external".into(), external, &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "size".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
        assert_eq!(
            super::plan_size_variable(
                "record".into(),
                ValueConstructor {
                    publicity: Publicity::Private,
                    deprecation: Deprecation::NotDeprecated,
                    type_: type_::int(),
                    variant: ValueConstructorVariant::Record {
                        name: "Record".into(),
                        arity: 0,
                        field_map: None,
                        location: dummy_span(),
                        module: "main".into(),
                        variants_count: 1,
                        variant_index: 0,
                        documentation: None,
                    },
                },
                &context,
            ),
            Err(super::invalid_pattern()),
        );
        assert_eq!(
            super::plan_size_constant(
                Constant::String {
                    location: dummy_span(),
                    value: "wrong".into(),
                },
                &context,
            ),
            Err(super::invalid_pattern()),
        );
        assert_eq!(
            super::plan_size_constant(
                Constant::Var {
                    location: dummy_span(),
                    module: None,
                    name: "missing".into(),
                    constructor: None,
                    type_: type_::int(),
                },
                &context,
            ),
            Err(super::invalid_pattern()),
        );
    }
}
