use crate::plan::{
    BitArrayExpr, BitArraySegment, Endianness, Expr, FloatBitSize, StringEncoding, ValueType,
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
    let segments = segments
        .into_iter()
        .map(|segment| {
            let value = super::plan_expr(*segment.value, context)?;
            plan_segment(value, segment.options, typed_expr_int_literal)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BitArrayExpr::value(segments))
}

pub(super) fn plan_constant(
    segments: Vec<GleamBitArraySegment<Constant<Arc<Type>>, Arc<Type>>>,
    context: &PlanContext<'_>,
) -> Result<BitArrayExpr, PlanError> {
    let segments = segments
        .into_iter()
        .map(|segment| {
            let value = super::constant::plan(*segment.value, context)?;
            plan_segment(value, segment.options, constant_int_literal)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BitArrayExpr::value(segments))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentKind {
    Int,
    Float,
    Bits,
    String(StringEncoding),
    UtfCodepoint(StringEncoding),
}

fn plan_segment<Value>(
    value: Expr,
    options: Vec<BitArrayOption<Value>>,
    int_literal: fn(&Value) -> Option<BigInt>,
) -> Result<BitArraySegment, PlanError> {
    let value_type = value.value_type();
    let mut kind = None;
    let mut endianness = Endianness::Big;
    let mut explicit_endianness = false;
    let mut size = None;
    let mut unit = 1usize;
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
                unit = usize::from(value);
                explicit_unit = true;
            }
            BitArrayOption::Size { value, .. } => {
                let Some(value) = int_literal(value.as_ref()) else {
                    return unsupported(UnsupportedBitArraySegmentReason::DynamicSize);
                };
                size = Some(usize::try_from(value).map_err(|_| {
                    unsupported_error(UnsupportedBitArraySegmentReason::SizeOutOfRange)
                })?);
            }
            BitArrayOption::Native { .. } => {
                return unsupported(UnsupportedBitArraySegmentReason::NativeEndianness);
            }
            BitArrayOption::Bytes { .. }
            | BitArrayOption::Signed { .. }
            | BitArrayOption::Unsigned { .. } => return invalid_segment_option(),
        }
    }

    let kind = match kind {
        Some(kind) => kind,
        None => match value_type {
            ValueType::Int => SegmentKind::Int,
            ValueType::Float => SegmentKind::Float,
            ValueType::String
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
            let bit_size = size.unwrap_or(8).checked_mul(unit).ok_or_else(|| {
                unsupported_error(UnsupportedBitArraySegmentReason::SizeOutOfRange)
            })?;
            Ok(BitArraySegment::Int {
                value,
                bit_size,
                endianness,
            })
        }
        SegmentKind::Float => {
            let Some(value) = value.into_float() else {
                return invalid_segment_option();
            };
            let bit_size = size.unwrap_or(64).checked_mul(unit).ok_or_else(|| {
                unsupported_error(UnsupportedBitArraySegmentReason::SizeOutOfRange)
            })?;
            let bit_size = match bit_size {
                16 => FloatBitSize::Sixteen,
                32 => FloatBitSize::ThirtyTwo,
                64 => FloatBitSize::SixtyFour,
                _ => return invalid_segment_option(),
            };
            Ok(BitArraySegment::Float {
                value,
                bit_size,
                endianness,
            })
        }
        SegmentKind::Bits => {
            if size.is_some() {
                return unsupported(UnsupportedBitArraySegmentReason::SizedBits);
            }
            let Some(value) = value.into_bit_array() else {
                return invalid_segment_option();
            };
            Ok(BitArraySegment::Bits(value))
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
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::BitArraySegmentOption,
        },
    })
}

fn unsupported<T>(reason: UnsupportedBitArraySegmentReason) -> Result<T, PlanError> {
    Err(unsupported_error(reason))
}

fn unsupported_error(reason: UnsupportedBitArraySegmentReason) -> PlanError {
    PlanError::UnsupportedBitArraySegment { reason }
}

#[cfg(test)]
mod tests {
    use super::{constant_int_literal, plan_segment};
    use crate::plan::{
        BitArrayExpr, BitArraySegment, Endianness, Expr, FloatBitSize, FloatExpr, FunctionExpr,
        FunctionReference, IntExpr, IntFunctionId, ParamLocal, RuntimeFunctionId, StringEncoding,
        StringExpr, UtfCodepointExpr, UtfCodepointLocalId,
    };
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason,
    };
    use gleam_core::ast::{BitArrayOption, Constant, SrcSpan};
    use num_bigint::BigInt;
    use std::sync::Arc;

    #[test]
    fn supported_segment_options_build_exact_plan_segments() {
        let location = SrcSpan::new(0, 0);
        assert_eq!(
            plan_segment(
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
                |_: &()| Some(4.into()),
            ),
            Ok(BitArraySegment::Int {
                value: IntExpr::value(0x12.into()),
                bit_size: 8,
                endianness: Endianness::Little,
            }),
        );
        assert_eq!(
            plan_segment(
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
                |_: &()| Some(16.into()),
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::Sixteen,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_segment(
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
                |_: &()| Some(8.into()),
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::Sixteen,
                endianness: Endianness::Little,
            }),
        );
        assert_eq!(
            plan_segment(
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
                |_: &()| Some(32.into()),
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::ThirtyTwo,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::float(FloatExpr::value(1.5)),
                vec![BitArrayOption::Float { location }],
                |_: &()| None,
            ),
            Ok(BitArraySegment::Float {
                value: FloatExpr::value(1.5),
                bit_size: FloatBitSize::SixtyFour,
                endianness: Endianness::Big,
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![BitArrayOption::Bits { location }],
                |_: &()| None,
            ),
            Ok(BitArraySegment::Bits(BitArrayExpr::value(Vec::new()))),
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
                plan_segment(
                    Expr::string(StringExpr::value("a".into())),
                    vec![option],
                    |_: &()| None,
                ),
                Ok(BitArraySegment::String {
                    value: StringExpr::value("a".into()),
                    encoding: expected,
                }),
            );
        }

        let codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        let static_size = |_: &()| Some(BigInt::from(1));
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
                plan_segment(Expr::utf_codepoint(codepoint.clone()), options, static_size,),
                Ok(BitArraySegment::UtfCodepoint {
                    value: codepoint.clone(),
                    encoding: expected,
                }),
            );
        }
        assert_eq!(
            plan_segment(
                Expr::utf_codepoint(codepoint.clone()),
                Vec::new(),
                static_size,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::utf_codepoint(codepoint.clone()),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                ],
                static_size,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::utf_codepoint(codepoint.clone()),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Little { location },
                ],
                static_size,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::utf_codepoint(codepoint),
                vec![
                    BitArrayOption::Utf16Codepoint { location },
                    BitArrayOption::Unit { location, value: 1 },
                ],
                static_size,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                },
            }),
        );
    }

    #[test]
    fn invalid_segment_combinations_keep_construction_margins() {
        let location = SrcSpan::new(0, 0);
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
                plan_segment(Expr::int(IntExpr::value(1.into())), options, |_: &()| None,),
                invalid,
            );
        }
        assert_eq!(
            plan_segment(
                Expr::float(FloatExpr::value(1.0)),
                vec![BitArrayOption::Size {
                    location,
                    value: Box::new(()),
                    short_form: false,
                }],
                |_: &()| Some(24.into()),
            ),
            invalid,
        );
        assert_eq!(
            plan_segment(
                Expr::string(StringExpr::value("wrong".into())),
                vec![BitArrayOption::Int { location }],
                |_: &()| None,
            ),
            invalid,
        );
        assert_eq!(
            plan_segment(
                Expr::utf_codepoint(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                )),
                vec![
                    BitArrayOption::Utf8Codepoint { location },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                |_: &()| None,
            ),
            invalid,
        );
        assert_eq!(
            plan_segment(
                Expr::int(IntExpr::value(1.into())),
                vec![BitArrayOption::Utf8Codepoint { location }],
                |_: &()| None,
            ),
            invalid,
        );
        assert_eq!(
            plan_segment(
                Expr::function(FunctionExpr::reference(FunctionReference::new(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                ))),
                Vec::new(),
                |_: &()| None,
            ),
            invalid,
        );
        assert_eq!(
            plan_segment(
                Expr::float(FloatExpr::value(1.0)),
                vec![
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                |_: &()| Some(BigInt::from(usize::MAX)),
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::int(IntExpr::value(1.into())),
                vec![
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                    BitArrayOption::Unit { location, value: 2 },
                ],
                |_: &()| Some(BigInt::from(usize::MAX)),
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![
                    BitArrayOption::Bits { location },
                    BitArrayOption::Size {
                        location,
                        value: Box::new(()),
                        short_form: false,
                    },
                ],
                |_: &()| Some(8.into()),
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizedBits,
            }),
        );
        assert_eq!(
            plan_segment(
                Expr::int(IntExpr::value(1.into())),
                vec![BitArrayOption::Size {
                    location,
                    value: Box::new(()),
                    short_form: false,
                }],
                |_: &()| Some(BigInt::from(-1)),
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
    }

    #[test]
    fn source_exclusions_report_exact_segment_profile_reasons() {
        let cases = [
            (
                "pub fn main() { <<1:native>> }",
                UnsupportedBitArraySegmentReason::NativeEndianness,
            ),
            (
                "pub fn main() { let size = 8 <<1:size(size)>> }",
                UnsupportedBitArraySegmentReason::DynamicSize,
            ),
            (
                "pub fn main() { let bits = <<1>> <<bits:bits-size(8)>> }",
                UnsupportedBitArraySegmentReason::SizedBits,
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
    fn unsupported_segment_options_keep_distinct_profile_reasons() {
        let cases = [(
            BitArrayOption::Native {
                location: SrcSpan::new(0, 0),
            },
            UnsupportedBitArraySegmentReason::NativeEndianness,
        )];

        for (option, reason) in cases {
            assert_eq!(
                plan_segment(
                    Expr::string(StringExpr::value("a".into())),
                    vec![option],
                    |_: &()| None,
                ),
                Err(PlanError::UnsupportedBitArraySegment { reason }),
            );
        }

        assert_eq!(
            plan_segment(
                Expr::bit_array(BitArrayExpr::value(Vec::new())),
                vec![BitArrayOption::Size {
                    location: SrcSpan::new(0, 0),
                    value: Box::new(()),
                    short_form: false,
                }],
                |_: &()| None,
            ),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::DynamicSize,
            }),
        );
    }

    #[test]
    fn expression_only_impossible_options_are_typed_ast_margins() {
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
                plan_segment(
                    Expr::bit_array(BitArrayExpr::value(Vec::new())),
                    vec![option],
                    |_: &()| None,
                ),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::BitArraySegmentOption,
                    },
                }),
            );
        }
    }

    #[test]
    fn constant_size_literal_extraction_accepts_only_integer_constants() {
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
