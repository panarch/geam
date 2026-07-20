use crate::plan::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArraySegment, Endianness, Expr,
    FloatBitSize, IntExpr, PanicSite, StringEncoding, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError, UnsupportedBitArraySegmentReason,
};
use gleam_core::ast::{
    BitArrayOption, BitArraySegment as GleamBitArraySegment, Constant, TypedExpr,
};
use gleam_core::type_::Type;
use num_bigint::BigInt;
use std::sync::Arc;

pub(super) fn plan_expression(
    segments: Vec<GleamBitArraySegment<TypedExpr, Arc<Type>>>,
    context: &mut PlanContext<'_>,
) -> Result<BitArrayExpr, PlanError> {
    let mut planned = Vec::with_capacity(segments.len());
    for segment in segments {
        let value = super::plan_expr(*segment.value, context)?;
        let options = plan_options(segment.options, |size| plan_expression_size(size, context))?;
        planned.push(plan_segment(
            value,
            options,
            context.panic_site(segment.location),
        )?);
    }
    Ok(BitArrayExpr::value(planned))
}

pub(super) fn plan_constant(
    segments: Vec<GleamBitArraySegment<Constant<Arc<Type>>, Arc<Type>>>,
    context: &PlanContext<'_>,
) -> Result<BitArrayExpr, PlanError> {
    let mut planned = Vec::with_capacity(segments.len());
    for segment in segments {
        let value = super::constant::plan(*segment.value, context)?;
        let options = plan_options(segment.options, plan_constant_size)?;
        planned.push(plan_segment(
            value,
            options,
            context.panic_site(segment.location),
        )?);
    }
    Ok(BitArrayExpr::value(planned))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentKind {
    Int,
    Float,
    Bits,
    String(StringEncoding),
    UtfCodepoint(StringEncoding),
}

enum SegmentSize {
    Fixed(BigInt),
    Evaluated(IntExpr),
}

struct SegmentOptions {
    kind: Option<SegmentKind>,
    endianness: Endianness,
    explicit_endianness: bool,
    size: Option<SegmentSize>,
    unit: u8,
    explicit_unit: bool,
}

fn plan_options<Value>(
    options: Vec<BitArrayOption<Value>>,
    mut plan_size: impl FnMut(Value) -> Result<SegmentSize, PlanError>,
) -> Result<SegmentOptions, PlanError> {
    let mut kind = None;
    let mut endianness = Endianness::Big;
    let mut explicit_endianness = false;
    let mut size = None;
    let mut unit = 1;
    let mut explicit_unit = false;

    for option in options {
        match option {
            BitArrayOption::Int { .. } => set_kind(&mut kind, SegmentKind::Int)?,
            BitArrayOption::Float { .. } => set_kind(&mut kind, SegmentKind::Float)?,
            BitArrayOption::Bits { .. } => set_kind(&mut kind, SegmentKind::Bits)?,
            BitArrayOption::Utf8 { .. } => {
                set_kind(&mut kind, SegmentKind::String(StringEncoding::Utf8))?
            }
            BitArrayOption::Utf16 { .. } => set_kind(
                &mut kind,
                SegmentKind::String(StringEncoding::Utf16(Endianness::Big)),
            )?,
            BitArrayOption::Utf32 { .. } => set_kind(
                &mut kind,
                SegmentKind::String(StringEncoding::Utf32(Endianness::Big)),
            )?,
            BitArrayOption::Utf8Codepoint { .. } => {
                set_kind(&mut kind, SegmentKind::UtfCodepoint(StringEncoding::Utf8))?
            }
            BitArrayOption::Utf16Codepoint { .. } => set_kind(
                &mut kind,
                SegmentKind::UtfCodepoint(StringEncoding::Utf16(Endianness::Big)),
            )?,
            BitArrayOption::Utf32Codepoint { .. } => set_kind(
                &mut kind,
                SegmentKind::UtfCodepoint(StringEncoding::Utf32(Endianness::Big)),
            )?,
            BitArrayOption::Big { .. } => {
                endianness = Endianness::Big;
                explicit_endianness = true;
            }
            BitArrayOption::Little { .. } => {
                endianness = Endianness::Little;
                explicit_endianness = true;
            }
            BitArrayOption::Unit { value, .. } => {
                unit = value;
                explicit_unit = true;
            }
            BitArrayOption::Size { value, .. } => size = Some(plan_size(*value)?),
            BitArrayOption::Native { .. } => {
                return unsupported(UnsupportedBitArraySegmentReason::NativeEndianness);
            }
            BitArrayOption::Bytes { .. }
            | BitArrayOption::Signed { .. }
            | BitArrayOption::Unsigned { .. } => return invalid_segment_option(),
        }
    }

    Ok(SegmentOptions {
        kind,
        endianness,
        explicit_endianness,
        size,
        unit,
        explicit_unit,
    })
}

fn plan_segment(
    value: Expr,
    options: SegmentOptions,
    site: PanicSite,
) -> Result<BitArraySegment, PlanError> {
    let value_type = value.value_type();
    let SegmentOptions {
        kind,
        endianness,
        explicit_endianness,
        size,
        unit,
        explicit_unit,
    } = options;

    let kind = match kind {
        Some(kind) => kind,
        None => match value_type {
            ValueType::Int => SegmentKind::Int,
            ValueType::Float => SegmentKind::Float,
            ValueType::Parameter(_)
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Custom(_)
            | ValueType::Bool
            | ValueType::Nil
            | ValueType::Tuple(_)
            | ValueType::List(_)
            | ValueType::Function(_) => return invalid_segment_option(),
        },
    };

    match kind {
        SegmentKind::Int => {
            let Some(value) = value.into_int() else {
                return invalid_segment_option();
            };
            match size {
                None => Ok(BitArraySegment::Int {
                    value,
                    bit_size: 8 * usize::from(unit),
                    endianness,
                }),
                Some(SegmentSize::Fixed(size)) => Ok(BitArraySegment::Int {
                    value,
                    bit_size: fixed_bit_size(size, unit)?,
                    endianness,
                }),
                Some(SegmentSize::Evaluated(size)) => Ok(BitArraySegment::EvaluatedInt {
                    value,
                    size: BitArrayEvaluatedSize::new(size, unit),
                    endianness,
                    site,
                }),
            }
        }
        SegmentKind::Float => {
            let Some(value) = value.into_float() else {
                return invalid_segment_option();
            };
            match size {
                None => Ok(BitArraySegment::Float {
                    value,
                    bit_size: FloatBitSize::SixtyFour,
                    endianness,
                }),
                Some(SegmentSize::Fixed(size)) => Ok(BitArraySegment::Float {
                    value,
                    bit_size: float_bit_size(fixed_bit_size(size, unit)?)?,
                    endianness,
                }),
                Some(SegmentSize::Evaluated(size)) => Ok(BitArraySegment::EvaluatedFloat {
                    value,
                    size: BitArrayEvaluatedSize::new(size, unit),
                    endianness,
                    site,
                }),
            }
        }
        SegmentKind::Bits => {
            let Some(value) = value.into_bit_array() else {
                return invalid_segment_option();
            };
            match size {
                None => Ok(BitArraySegment::Bits(value)),
                Some(SegmentSize::Fixed(size)) => Ok(BitArraySegment::SizedBits {
                    value,
                    size: BitArrayBitsSize::Fixed(fixed_bit_size(size, unit)?),
                    site,
                }),
                Some(SegmentSize::Evaluated(size)) => Ok(BitArraySegment::SizedBits {
                    value,
                    size: BitArrayBitsSize::Evaluated(BitArrayEvaluatedSize::new(size, unit)),
                    site,
                }),
            }
        }
        SegmentKind::String(encoding) => {
            if size.is_some() || unit != 1 {
                return invalid_segment_option();
            }
            let Some(value) = value.into_string() else {
                return invalid_segment_option();
            };
            let encoding = match encoding {
                StringEncoding::Utf8 => StringEncoding::Utf8,
                StringEncoding::Utf16(_) => StringEncoding::Utf16(endianness),
                StringEncoding::Utf32(_) => StringEncoding::Utf32(endianness),
            };
            Ok(BitArraySegment::String { value, encoding })
        }
        SegmentKind::UtfCodepoint(encoding) => {
            if size.is_some()
                || explicit_unit
                || matches!(encoding, StringEncoding::Utf8) && explicit_endianness
            {
                return invalid_segment_option();
            }
            let Some(value) = value.into_utf_codepoint() else {
                return invalid_segment_option();
            };
            let encoding = match encoding {
                StringEncoding::Utf8 => StringEncoding::Utf8,
                StringEncoding::Utf16(_) => StringEncoding::Utf16(endianness),
                StringEncoding::Utf32(_) => StringEncoding::Utf32(endianness),
            };
            Ok(BitArraySegment::UtfCodepoint { value, encoding })
        }
    }
}

fn plan_expression_size(
    value: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<SegmentSize, PlanError> {
    if let Some(value) = typed_expr_int_literal(&value) {
        return Ok(SegmentSize::Fixed(value));
    }
    let value = super::plan_expr(value, context)?;
    let Some(value) = value.into_int() else {
        return invalid_segment_option();
    };
    Ok(SegmentSize::Evaluated(value))
}

fn plan_constant_size(value: Constant<Arc<Type>>) -> Result<SegmentSize, PlanError> {
    constant_int_literal(&value)
        .map(SegmentSize::Fixed)
        .ok_or_else(invalid_segment_option_error)
}

fn fixed_bit_size(value: BigInt, unit: u8) -> Result<usize, PlanError> {
    let value = if value < BigInt::from(0) {
        BigInt::from(0)
    } else {
        value
    };
    usize::try_from(value * BigInt::from(unit))
        .map_err(|_| unsupported_error(UnsupportedBitArraySegmentReason::SizeOutOfRange))
}

fn float_bit_size(bit_size: usize) -> Result<FloatBitSize, PlanError> {
    match bit_size {
        16 => Ok(FloatBitSize::Sixteen),
        32 => Ok(FloatBitSize::ThirtyTwo),
        64 => Ok(FloatBitSize::SixtyFour),
        _ => invalid_segment_option(),
    }
}

fn set_kind(kind: &mut Option<SegmentKind>, value: SegmentKind) -> Result<(), PlanError> {
    if kind.replace(value).is_some() {
        return invalid_segment_option();
    }
    Ok(())
}

fn typed_expr_int_literal(value: &TypedExpr) -> Option<BigInt> {
    match value {
        TypedExpr::Int { int_value, .. } => Some(int_value.clone()),
        _ => None,
    }
}

fn constant_int_literal(value: &Constant<Arc<Type>>) -> Option<BigInt> {
    match value {
        Constant::Int { int_value, .. } => Some(int_value.clone()),
        _ => None,
    }
}

fn invalid_segment_option<T>() -> Result<T, PlanError> {
    Err(invalid_segment_option_error())
}

fn invalid_segment_option_error() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::BitArraySegmentOption,
        },
    }
}

fn unsupported<T>(reason: UnsupportedBitArraySegmentReason) -> Result<T, PlanError> {
    Err(unsupported_error(reason))
}

fn unsupported_error(reason: UnsupportedBitArraySegmentReason) -> PlanError {
    PlanError::UnsupportedBitArraySegment { reason }
}

#[cfg(test)]
mod tests {
    use super::{
        SegmentKind, SegmentOptions, SegmentSize, constant_int_literal, plan_constant,
        plan_expression, plan_options, plan_segment as plan_segment_with_options,
    };
    use crate::plan::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArraySegment, Endianness, Expr,
        FloatBitSize, FloatExpr, FunctionExpr, FunctionReference, IntExpr, IntLocalId, PanicSite,
        StringEncoding, StringExpr, UtfCodepointExpr, UtfCodepointLocalId,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason,
    };
    use gleam_core::ast::{
        BitArrayOption, BitArraySegment as GleamBitArraySegment, Constant, SrcSpan, TypedExpr,
    };
    use gleam_core::type_::{self, ValueConstructor, ValueConstructorVariant};
    use num_bigint::BigInt;
    use std::sync::Arc;

    fn plan_fixed_segment_fixture(
        value: Expr,
        options: Vec<BitArrayOption<()>>,
        size: Option<BigInt>,
        site: &PanicSite,
    ) -> Result<BitArraySegment, PlanError> {
        let options = plan_options(options, |_| {
            size.clone()
                .map(SegmentSize::Fixed)
                .ok_or_else(invalid_segment_option_error)
        })?;
        plan_segment_with_options(value, options, site.clone())
    }

    fn invalid_segment_option_error() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::BitArraySegmentOption,
            },
        }
    }

    #[test]
    fn plan_supported_segment_options_build_exact_plan_segments() {
        let location = SrcSpan::new(0, 0);
        let site = PanicSite::unknown();
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::int(IntExpr::value(0x12.into())),
                vec![
                    BitArrayOption::Int { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                    BitArrayOption::Little { location },
                ],
                Some(4.into()),
                &site,
            ),
            Ok(BitArraySegment::Int {
                value: IntExpr::value(0x12.into()),
                bit_size: 8,
                endianness: Endianness::Little,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.5)),
                vec![
                    BitArrayOption::Float { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Big { location },
                ],
                Some(16.into()),
                &site,
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::Sixteen,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.5)),
                vec![
                    BitArrayOption::Float { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                    BitArrayOption::Little { location },
                ],
                Some(8.into()),
                &site,
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::Sixteen,
                endianness: Endianness::Little,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.5)),
                vec![
                    BitArrayOption::Float { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Big { location },
                ],
                Some(32.into()),
                &site,
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::ThirtyTwo,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.5)),
                vec![BitArrayOption::Float { location }],
                None,
                &site,
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::SixtyFour,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![BitArrayOption::Bits { location }],
                None,
                &site,
            ),
            Ok(BitArraySegment::Bits(BitArrayExpr::value(Vec::new()))),
        );

        let evaluated_size = IntExpr::local_get(IntLocalId(0), "size".into());
        assert_eq!(
            plan_segment_with_options(
                Expr::int(IntExpr::value(0x12.into())),
                SegmentOptions {
                    kind: Some(SegmentKind::Int),
                    endianness: Endianness::Big,
                    explicit_endianness: false,
                    size: Some(SegmentSize::Evaluated(evaluated_size.clone())),
                    unit: 2,
                    explicit_unit: true,
                },
                site.clone(),
            ),
            Ok(BitArraySegment::EvaluatedInt {
                value: IntExpr::value(0x12.into()),
                size: BitArrayEvaluatedSize::new(evaluated_size.clone(), 2),
                endianness: Endianness::Big,
                site: site.clone(),
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![
                    BitArrayOption::Bits { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                Some(BigInt::from(usize::MAX)),
                &site,
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_segment_with_options(
                Expr::float(FloatExpr::value(1.5)),
                SegmentOptions {
                    kind: Some(SegmentKind::Float),
                    endianness: Endianness::Little,
                    explicit_endianness: true,
                    size: Some(SegmentSize::Evaluated(evaluated_size.clone())),
                    unit: 1,
                    explicit_unit: false,
                },
                site.clone(),
            ),
            Ok(BitArraySegment::EvaluatedFloat {
                value: FloatExpr::value(1.5),
                size: BitArrayEvaluatedSize::new(evaluated_size.clone(), 1),
                endianness: Endianness::Little,
                site: site.clone(),
            }),
        );
        assert_eq!(
            plan_segment_with_options(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                SegmentOptions {
                    kind: Some(SegmentKind::Bits),
                    endianness: Endianness::Big,
                    explicit_endianness: false,
                    size: Some(SegmentSize::Evaluated(evaluated_size.clone())),
                    unit: 1,
                    explicit_unit: false,
                },
                site.clone(),
            ),
            Ok(BitArraySegment::SizedBits {
                value: BitArrayExpr::value(Vec::new()),
                size: BitArrayBitsSize::Evaluated(BitArrayEvaluatedSize::new(evaluated_size, 1)),
                site: site.clone(),
            }),
        );

        for (option, expected) in [
            (BitArrayOption::Utf8 { location }, StringEncoding::Utf8),
            (
                BitArrayOption::Utf16 { location },
                StringEncoding::Utf16(Endianness::Big),
            ),
            (
                BitArrayOption::Utf32 { location },
                StringEncoding::Utf32(Endianness::Big),
            ),
        ] {
            assert_eq!(
                plan_fixed_segment_fixture(
                    Expr::string(StringExpr::value("a".into())),
                    vec![option],
                    None,
                    &site,
                ),
                Ok(BitArraySegment::String {
                    value: StringExpr::value("a".into()),
                    encoding: expected,
                }),
            );
        }

        let codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        for (options, expected) in [
            (
                vec![BitArrayOption::Utf8Codepoint { location }],
                StringEncoding::Utf8,
            ),
            (
                vec![
                    BitArrayOption::Utf16Codepoint { location },
                    BitArrayOption::Little { location },
                ],
                StringEncoding::Utf16(Endianness::Little),
            ),
            (
                vec![
                    BitArrayOption::Utf32Codepoint { location },
                    BitArrayOption::Big { location },
                ],
                StringEncoding::Utf32(Endianness::Big),
            ),
        ] {
            assert_eq!(
                plan_fixed_segment_fixture(
                    Expr::utf_codepoint(codepoint.clone()),
                    options,
                    Some(1.into()),
                    &site,
                ),
                Ok(BitArraySegment::UtfCodepoint {
                    value: codepoint.clone(),
                    encoding: expected,
                }),
            );
        }
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::utf_codepoint(codepoint.clone()),
                Vec::new(),
                Some(1.into()),
                &site,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::utf_codepoint(codepoint.clone()),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                ],
                Some(1.into()),
                &site,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::utf_codepoint(codepoint.clone()),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Little { location },
                ],
                Some(1.into()),
                &site,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::utf_codepoint(codepoint),
                vec![
                    BitArrayOption::Utf16Codepoint { location },
                    BitArrayOption::Unit { location, value: 1 },
                ],
                Some(1.into()),
                &site,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_expression_segments_preserve_value_and_size_failures() {
        let location = SrcSpan::new(0, 0);
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let invalid = Err(invalid_segment_option_error());

        assert_eq!(
            plan_expression(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(TypedExpr::String {
                        location,
                        type_: type_::string(),
                        value: "wrong".into(),
                    }),
                    options: Vec::new(),
                    type_: type_::string(),
                }],
                &mut context,
            ),
            invalid,
        );
        assert_eq!(
            plan_expression(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(TypedExpr::Invalid {
                        location,
                        type_: type_::int(),
                        extra_information: None,
                    }),
                    options: Vec::new(),
                    type_: type_::int(),
                }],
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_expression(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(crate::planner::expression::typed_int_expr(1)),
                    options: vec![BitArrayOption::Size {
                        location,
                        value: Box::new(TypedExpr::Invalid {
                            location,
                            type_: type_::int(),
                            extra_information: None,
                        }),
                        short_form: false,
                    }],
                    type_: type_::int(),
                }],
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_expression(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(crate::planner::expression::typed_int_expr(1)),
                    options: vec![BitArrayOption::Size {
                        location,
                        value: Box::new(TypedExpr::String {
                            location,
                            type_: type_::string(),
                            value: "wrong".into(),
                        }),
                        short_form: false,
                    }],
                    type_: type_::int(),
                }],
                &mut context,
            ),
            invalid,
        );
    }

    #[test]
    fn reject_margin_constant_segments_preserve_value_and_size_failures() {
        let location = SrcSpan::new(0, 0);
        let module = crate::planner::support::compile(
            r#"
const size = 4
const wrong = "wrong"
pub fn main() { 0 }
"#,
        );
        let module_constant = |name: &str| {
            let definition = module
                .definitions
                .constants
                .iter()
                .find(|constant| constant.name == name)
                .expect("module constant should exist");
            Constant::Var {
                location,
                module: None,
                name: name.into(),
                constructor: Some(Box::new(ValueConstructor {
                    publicity: definition.publicity,
                    deprecation: definition.deprecation.clone(),
                    type_: definition.type_.clone(),
                    variant: ValueConstructorVariant::ModuleConstant {
                        documentation: None,
                        location: definition.location,
                        module: "main".into(),
                        name: name.into(),
                        literal: definition.value.as_ref().clone(),
                        implementations: definition.implementations,
                    },
                })),
                type_: definition.type_.clone(),
            }
        };
        let module_name = "main".into();
        let functions = std::collections::HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            plan_constant(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(Constant::Int {
                        location,
                        value: "1".into(),
                        int_value: 1.into(),
                    }),
                    options: vec![BitArrayOption::Size {
                        location,
                        value: Box::new(module_constant("size")),
                        short_form: false,
                    }],
                    type_: type_::int(),
                }],
                &context,
            ),
            Err(invalid_segment_option_error()),
        );
        assert_eq!(
            plan_constant(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(Constant::String {
                        location,
                        value: "wrong".into(),
                    }),
                    options: Vec::new(),
                    type_: type_::string(),
                }],
                &context,
            ),
            Err(invalid_segment_option_error()),
        );
        assert_eq!(
            plan_constant(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(Constant::Invalid {
                        location,
                        type_: type_::int(),
                        extra_information: None,
                    }),
                    options: Vec::new(),
                    type_: type_::int(),
                }],
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(Constant::Int {
                        location,
                        value: "1".into(),
                        int_value: 1.into(),
                    }),
                    options: vec![BitArrayOption::Size {
                        location,
                        value: Box::new(Constant::Invalid {
                            location,
                            type_: type_::int(),
                            extra_information: None,
                        }),
                        short_form: false,
                    }],
                    type_: type_::int(),
                }],
                &context,
            ),
            Err(invalid_segment_option_error()),
        );
        assert_eq!(
            plan_constant(
                vec![GleamBitArraySegment {
                    location,
                    value: Box::new(Constant::Int {
                        location,
                        value: "1".into(),
                        int_value: 1.into(),
                    }),
                    options: vec![BitArrayOption::Size {
                        location,
                        value: Box::new(module_constant("wrong")),
                        short_form: false,
                    }],
                    type_: type_::int(),
                }],
                &context,
            ),
            Err(invalid_segment_option_error()),
        );
    }

    #[test]
    fn reject_margin_invalid_segment_combinations() {
        let location = SrcSpan::new(0, 0);
        let site = PanicSite::unknown();
        let invalid = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::BitArraySegmentOption,
            },
        });

        for options in [
            vec![
                BitArrayOption::Int { location },
                BitArrayOption::Int { location },
            ],
            vec![
                BitArrayOption::Int { location },
                BitArrayOption::Float { location },
            ],
            vec![
                BitArrayOption::Bits { location },
                BitArrayOption::Bits { location },
            ],
            vec![
                BitArrayOption::Utf8 { location },
                BitArrayOption::Utf8 { location },
            ],
            vec![
                BitArrayOption::Int { location },
                BitArrayOption::Utf16 { location },
            ],
            vec![BitArrayOption::Float { location }],
            vec![BitArrayOption::Bits { location }],
            vec![BitArrayOption::Utf8 { location }],
            vec![
                BitArrayOption::Utf16 { location },
                BitArrayOption::Unit { location, value: 2 },
            ],
            vec![
                BitArrayOption::Utf16 { location },
                BitArrayOption::Utf32 { location },
            ],
            vec![
                BitArrayOption::Utf8Codepoint { location },
                BitArrayOption::Utf8Codepoint { location },
            ],
            vec![
                BitArrayOption::Utf16Codepoint { location },
                BitArrayOption::Utf16Codepoint { location },
            ],
            vec![
                BitArrayOption::Utf32Codepoint { location },
                BitArrayOption::Utf32Codepoint { location },
            ],
        ] {
            assert_eq!(
                plan_fixed_segment_fixture(
                    Expr::int(IntExpr::value(1.into())),
                    options,
                    None,
                    &site,
                ),
                invalid,
            );
        }
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.0)),
                vec![BitArrayOption::Size {
                    location,
                    value: Box::new(()),
                    short_form: false,
                }],
                Some(24.into()),
                &site,
            ),
            invalid,
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::string(StringExpr::value("wrong".into())),
                vec![BitArrayOption::Int { location }],
                None,
                &site,
            ),
            invalid,
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::utf_codepoint(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                )),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                None,
                &site,
            ),
            invalid,
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::int(IntExpr::value(1.into())),
                vec![BitArrayOption::Utf8Codepoint { location }],
                None,
                &site,
            ),
            invalid,
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::function(FunctionExpr::reference(FunctionReference::new(
                    crate::plan::monomorphic_function_instantiation(
                        0,
                        crate::plan::FunctionShape::new(Vec::new(), crate::plan::ValueShape::Int,),
                    )
                ))),
                Vec::new(),
                None,
                &site,
            ),
            invalid,
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::float(FloatExpr::value(1.0)),
                vec![
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                Some(BigInt::from(usize::MAX)),
                &site,
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::int(IntExpr::value(1.into())),
                vec![
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                Some(BigInt::from(usize::MAX)),
                &site,
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![
                    BitArrayOption::Bits { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                ],
                Some(8.into()),
                &site,
            ),
            Ok(BitArraySegment::SizedBits {
                value: BitArrayExpr::value(Vec::new()),
                size: BitArrayBitsSize::Fixed(8),
                site: site.clone(),
            }),
        );
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::int(IntExpr::value(1.into())),
                vec![BitArrayOption::Size {
                    location,
                    value: Box::new(()),
                    short_form: false,
                }],
                Some(BigInt::from(-1)),
                &site,
            ),
            Ok(BitArraySegment::Int {
                value: IntExpr::value(1.into()),
                bit_size: 0,
                endianness: Endianness::Big,
            }),
        );
    }

    #[test]
    fn reject_profile_native_endianness() {
        let cases = [
            (
                "pub fn main() { <<1:native>> }",
                UnsupportedBitArraySegmentReason::NativeEndianness,
            ),
            (
                "const value = <<1:native>> pub fn main() { value }",
                UnsupportedBitArraySegmentReason::NativeEndianness,
            ),
        ];

        for (source, reason) in cases {
            assert_eq!(
                crate::planner::support::expect_plan_error(source),
                PlanError::UnsupportedBitArraySegment { reason },
            );
        }

        let site = PanicSite::unknown();
        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::string(StringExpr::value("a".into())),
                vec![BitArrayOption::Native {
                    location: SrcSpan::new(0, 0),
                }],
                None,
                &site,
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

    #[test]
    fn reject_profile_bit_array_segment_expression_error() {
        assert_eq!(
            crate::planner::support::expect_plan_error("pub fn main() { <<echo 1>> }"),
            PlanError::UnsupportedExpression {
                kind: crate::planner::error::UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_expression_only_bit_array_options() {
        let site = PanicSite::unknown();
        for option in [
            BitArrayOption::Bytes {
                location: SrcSpan::new(0, 0),
            },
            BitArrayOption::Signed {
                location: SrcSpan::new(0, 0),
            },
            BitArrayOption::Unsigned {
                location: SrcSpan::new(0, 0),
            },
        ] {
            assert_eq!(
                plan_fixed_segment_fixture(
                    Expr::bit_array(BitArrayExpr::value(Vec::new())),
                    vec![option],
                    None,
                    &site,
                ),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                    },
                }),
            );
        }

        assert_eq!(
            plan_fixed_segment_fixture(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![BitArrayOption::Size {
                    location: SrcSpan::new(0, 0),
                    value: Box::new(()),
                    short_form: false,
                }],
                None,
                &site,
            ),
            Err(invalid_segment_option_error()),
        );
    }

    #[test]
    fn reject_margin_constant_size_requires_an_integer_literal() {
        let integer: Constant<Arc<gleam_core::type_::Type>> = Constant::Int {
            location: SrcSpan::new(0, 0),
            value: "8".into(),
            int_value: 8.into(),
        };
        let string: Constant<Arc<gleam_core::type_::Type>> = Constant::String {
            location: SrcSpan::new(0, 0),
            value: "8".into(),
        };

        assert_eq!(constant_int_literal(&integer), Some(8.into()));
        assert_eq!(constant_int_literal(&string), None);
    }
}
