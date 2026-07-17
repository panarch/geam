use crate::plan::{execution, module};

pub(super) fn bit_array_pattern(
    pattern: &module::BitArrayPattern,
    context: &mut super::LoweringContext,
) -> execution::BitArrayPattern {
    execution::BitArrayPattern::new(
        pattern
            .segments()
            .iter()
            .map(|segment| bit_array_segment(segment, context))
            .collect(),
    )
}

fn bit_array_segment(
    segment: &module::BitArrayPatternSegment,
    context: &mut super::LoweringContext,
) -> execution::BitArrayPatternSegment {
    use execution::BitArrayPatternSegment as E;
    use module::BitArrayPatternSegment as M;

    match segment {
        M::Int {
            pattern,
            size,
            endianness,
            signedness,
        } => E::Int {
            pattern: pattern_value(pattern, &mut |local| {
                execution::IntLocalId(context.mapped_local(super::frame::LocalKind::Int, local.0))
            }),
            size: pattern_size(size, context),
            endianness: (*endianness).into(),
            signedness: (*signedness).into(),
        },
        M::Float {
            pattern,
            size,
            endianness,
        } => E::Float {
            pattern: pattern_value(pattern, &mut |local| {
                execution::FloatLocalId(
                    context.mapped_local(super::frame::LocalKind::Float, local.0),
                )
            }),
            size: pattern_size(size, context),
            endianness: (*endianness).into(),
        },
        M::Bits {
            pattern,
            size,
            unit,
        } => E::Bits {
            pattern: binding_pattern(pattern, &mut |local| {
                execution::BitArrayLocalId(
                    context.mapped_local(super::frame::LocalKind::BitArray, local.0),
                )
            }),
            size: size.as_ref().map(|size| pattern_size(size, context)),
            unit: *unit,
        },
        M::String { pattern, encoding } => E::String {
            pattern: match pattern {
                module::BitArrayStringPattern::Literal(value) => {
                    execution::BitArrayStringPattern::Literal(value.clone())
                }
                module::BitArrayStringPattern::Discard => execution::BitArrayStringPattern::Discard,
            },
            encoding: (*encoding).into(),
        },
        M::UtfCodepoint { pattern, encoding } => E::UtfCodepoint {
            pattern: binding_pattern(pattern, &mut |local| {
                execution::UtfCodepointLocalId(
                    context.mapped_local(super::frame::LocalKind::UtfCodepoint, local.0),
                )
            }),
            encoding: (*encoding).into(),
        },
    }
}

fn pattern_size(
    size: &module::BitArrayPatternSize,
    context: &super::LoweringContext,
) -> execution::BitArrayPatternSize {
    execution::BitArrayPatternSize::new(pattern_size_expr(size.value(), context), size.unit())
}

fn pattern_size_expr(
    value: &module::BitArrayPatternSizeExpr,
    context: &super::LoweringContext,
) -> execution::BitArrayPatternSizeExpr {
    use execution::BitArrayPatternSizeExpr as E;
    use module::BitArrayPatternSizeExpr as M;

    match value {
        M::Value(value) => E::Value(value.clone()),
        M::LocalGet { local, .. } => E::LocalGet(execution::IntLocalId(
            context.mapped_local(super::frame::LocalKind::Int, local.0),
        )),
        M::Add { left, right } => E::Add {
            left: Box::new(pattern_size_expr(left, context)),
            right: Box::new(pattern_size_expr(right, context)),
        },
        M::Subtract { left, right } => E::Subtract {
            left: Box::new(pattern_size_expr(left, context)),
            right: Box::new(pattern_size_expr(right, context)),
        },
        M::Multiply { left, right } => E::Multiply {
            left: Box::new(pattern_size_expr(left, context)),
            right: Box::new(pattern_size_expr(right, context)),
        },
        M::Divide { left, right } => E::Divide {
            left: Box::new(pattern_size_expr(left, context)),
            right: Box::new(pattern_size_expr(right, context)),
        },
        M::Remainder { left, right } => E::Remainder {
            left: Box::new(pattern_size_expr(left, context)),
            right: Box::new(pattern_size_expr(right, context)),
        },
    }
}

fn pattern_value<Value: Clone, ModuleLocal: Copy, ExecutionLocal>(
    pattern: &module::BitArrayPatternValue<Value, ModuleLocal>,
    local: &mut impl FnMut(ModuleLocal) -> ExecutionLocal,
) -> execution::BitArrayPatternValue<Value, ExecutionLocal> {
    match pattern {
        module::BitArrayPatternValue::Literal(value) => {
            execution::BitArrayPatternValue::Literal(value.clone())
        }
        module::BitArrayPatternValue::Bind(binding) => {
            execution::BitArrayPatternValue::Bind(pattern_binding(binding, local))
        }
        module::BitArrayPatternValue::Discard => execution::BitArrayPatternValue::Discard,
        module::BitArrayPatternValue::Alias { pattern, binding } => {
            execution::BitArrayPatternValue::Alias {
                pattern: Box::new(pattern_value(pattern, local)),
                binding: pattern_binding(binding, local),
            }
        }
    }
}

fn binding_pattern<ModuleLocal: Copy, ExecutionLocal>(
    pattern: &module::BitArrayBindingPattern<ModuleLocal>,
    local: &mut impl FnMut(ModuleLocal) -> ExecutionLocal,
) -> execution::BitArrayBindingPattern<ExecutionLocal> {
    match pattern {
        module::BitArrayBindingPattern::Bind(binding) => {
            execution::BitArrayBindingPattern::Bind(pattern_binding(binding, local))
        }
        module::BitArrayBindingPattern::Discard => execution::BitArrayBindingPattern::Discard,
        module::BitArrayBindingPattern::Alias { pattern, binding } => {
            execution::BitArrayBindingPattern::Alias {
                pattern: Box::new(binding_pattern(pattern, local)),
                binding: pattern_binding(binding, local),
            }
        }
    }
}

fn pattern_binding<ModuleLocal: Copy, ExecutionLocal>(
    binding: &module::PatternBinding<ModuleLocal>,
    local: &mut impl FnMut(ModuleLocal) -> ExecutionLocal,
) -> execution::PatternBinding<ExecutionLocal> {
    execution::PatternBinding::new(local(*binding.local()))
}

impl From<module::Signedness> for execution::Signedness {
    fn from(value: module::Signedness) -> Self {
        match value {
            module::Signedness::Signed => Self::Signed,
            module::Signedness::Unsigned => Self::Unsigned,
        }
    }
}

impl From<module::Endianness> for execution::Endianness {
    fn from(value: module::Endianness) -> Self {
        match value {
            module::Endianness::Big => Self::Big,
            module::Endianness::Little => Self::Little,
        }
    }
}

impl From<module::StringEncoding> for execution::StringEncoding {
    fn from(value: module::StringEncoding) -> Self {
        match value {
            module::StringEncoding::Utf8 => Self::Utf8,
            module::StringEncoding::Utf16(endianness) => Self::Utf16(endianness.into()),
            module::StringEncoding::Utf32(endianness) => Self::Utf32(endianness.into()),
        }
    }
}
