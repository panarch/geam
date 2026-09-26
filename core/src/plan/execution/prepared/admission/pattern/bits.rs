use super::{BindingValue, Bindings, PatternError};
use crate::plan::execution::graph::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, MatchIntPatternBinding, MatchPatternBinding,
};
use crate::plan::execution::prepared::admission::{local::Locals, operand::Operand};
use crate::plan::execution::type_::ValueType;
use std::collections::HashSet;

impl Bindings {
    pub(super) fn bits<'data>(
        &mut self,
        pattern: &BitArrayPattern,
        locals: &Locals<'data>,
    ) -> Result<(), PatternError> {
        for segment in pattern.segments.iter() {
            match segment {
                BitArrayPatternSegment::Int { pattern, size, .. } => {
                    self.size(size, locals)?;
                    self.bit_value(
                        pattern,
                        |value| {
                            super::super::literal::integer(value).map_err(PatternError::Integer)
                        },
                        Self::int_binding,
                    )?;
                }
                BitArrayPatternSegment::Float { pattern, size, .. } => {
                    self.size(size, locals)?;
                    self.bit_value(
                        pattern,
                        |_| Ok(()),
                        |bindings, binding| bindings.bit_bind(binding, &ValueType::Float),
                    )?;
                }
                BitArrayPatternSegment::Bits {
                    pattern,
                    size,
                    unit,
                } => {
                    if *unit == 0 {
                        return Err(PatternError::ZeroUnit);
                    }
                    if let Some(size) = size {
                        self.size(size, locals)?;
                    }
                    self.bit_binding(pattern, &ValueType::BitArray)?;
                }
                BitArrayPatternSegment::String { .. } => {}
                BitArrayPatternSegment::UtfCodepoint { pattern, .. } => {
                    self.bit_binding(pattern, &ValueType::UtfCodepoint)?
                }
            }
        }
        Ok(())
    }

    fn size<'data>(
        &self,
        size: &BitArrayPatternSize,
        locals: &Locals<'data>,
    ) -> Result<(), PatternError> {
        let BitArrayPatternSize::Dynamic { value, unit } = size else {
            return Ok(());
        };
        if *unit == 0 {
            return Err(PatternError::ZeroUnit);
        }
        enum Visit<'a> {
            Enter(&'a BitArrayPatternSizeExpr),
            Leave(*const BitArrayPatternSizeExpr),
        }
        let mut pending = vec![Visit::Enter(value)];
        let mut active = HashSet::new();
        let mut complete = HashSet::new();
        while let Some(visit) = pending.pop() {
            let value = match visit {
                Visit::Enter(value) => value,
                Visit::Leave(key) => {
                    active.remove(&key);
                    complete.insert(key);
                    continue;
                }
            };
            let key = value as *const BitArrayPatternSizeExpr;
            if complete.contains(&key) {
                continue;
            }
            if !active.insert(key) {
                return Err(PatternError::RecursiveSize);
            }
            pending.push(Visit::Leave(key));
            match value {
                BitArrayPatternSizeExpr::Value(value) => {
                    super::super::literal::integer(value).map_err(PatternError::Integer)?;
                }
                BitArrayPatternSizeExpr::Local(local) => {
                    local.read(locals).map_err(PatternError::Local)?;
                }
                BitArrayPatternSizeExpr::Binding(binding) => {
                    if binding.index() >= self.ints {
                        return Err(PatternError::IntBinding { index: binding.0 });
                    }
                }
                BitArrayPatternSizeExpr::Add { left, right }
                | BitArrayPatternSizeExpr::Subtract { left, right }
                | BitArrayPatternSizeExpr::Multiply { left, right }
                | BitArrayPatternSizeExpr::Divide { left, right }
                | BitArrayPatternSizeExpr::Remainder { left, right } => {
                    pending.push(Visit::Enter(right));
                    pending.push(Visit::Enter(left));
                }
            }
        }
        Ok(())
    }

    fn bit_value<Value, Binding>(
        &mut self,
        root: &BitArrayPatternValue<Value, Binding>,
        validate: fn(&Value) -> Result<(), PatternError>,
        bind: fn(&mut Self, &Binding) -> Result<(), PatternError>,
    ) -> Result<(), PatternError> {
        let mut aliases = Vec::new();
        let mut visited = HashSet::new();
        let mut pattern = root;
        loop {
            if !visited.insert(pattern as *const BitArrayPatternValue<Value, Binding>) {
                return Err(PatternError::RecursivePattern);
            }
            match pattern {
                BitArrayPatternValue::Literal(value) => {
                    validate(value)?;
                    break;
                }
                BitArrayPatternValue::Discard => break,
                BitArrayPatternValue::Bind(binding) => {
                    bind(self, binding)?;
                    break;
                }
                BitArrayPatternValue::Alias {
                    pattern: inner,
                    binding,
                } => {
                    aliases.push(binding);
                    pattern = inner;
                }
            }
        }
        for binding in aliases.into_iter().rev() {
            bind(self, binding)?;
        }
        Ok(())
    }

    fn bit_binding(
        &mut self,
        root: &BitArrayBindingPattern,
        type_: &'static ValueType,
    ) -> Result<(), PatternError> {
        let mut aliases = Vec::new();
        let mut visited = HashSet::new();
        let mut pattern = root;
        loop {
            if !visited.insert(pattern as *const BitArrayBindingPattern) {
                return Err(PatternError::RecursivePattern);
            }
            match pattern {
                BitArrayBindingPattern::Discard => break,
                BitArrayBindingPattern::Bind(binding) => {
                    self.bit_bind(binding, type_)?;
                    break;
                }
                BitArrayBindingPattern::Alias {
                    pattern: inner,
                    binding,
                } => {
                    aliases.push(binding);
                    pattern = inner;
                }
            }
        }
        for binding in aliases.into_iter().rev() {
            self.bit_bind(binding, type_)?;
        }
        Ok(())
    }

    fn bit_bind(
        &mut self,
        binding: &MatchPatternBinding,
        type_: &'static ValueType,
    ) -> Result<(), PatternError> {
        self.bind(binding, BindingValue::Scalar(type_))
    }

    fn int_binding(&mut self, binding: &MatchIntPatternBinding) -> Result<(), PatternError> {
        self.bit_bind(&binding.binding, &ValueType::Int)?;
        if let Some(slot) = binding.size {
            if slot.index() != self.ints {
                return Err(PatternError::IntBindingOrder {
                    expected: self.ints,
                    found: slot.index(),
                });
            }
            self.ints += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BindingValue, Bindings, BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment,
        BitArrayPatternSize, BitArrayPatternSizeExpr, BitArrayPatternValue, Locals,
        MatchPatternBinding, PatternError, ValueType,
    };
    use crate::plan::execution::graph::{
        BitArrayStringPattern, Endianness, IntLocalId, IntegerLiteral, MatchIntBindingId,
        MatchIntPatternBinding, StringEncoding,
    };
    use crate::plan::execution::prepared::admission::{literal::IntegerError, local::LocalError};
    use crate::plan::execution::storage::{Node, Table};
    use num_bigint::{BigInt, Sign};

    const FLOAT_LITERAL: fn(&f64) -> Result<(), PatternError> = |_| Ok(());
    const FLOAT_BINDING: fn(&mut Bindings, &MatchPatternBinding) -> Result<(), PatternError> =
        |bindings, binding| bindings.bit_bind(binding, &ValueType::Float);
    const INTEGER_LITERAL: fn(&IntegerLiteral) -> Result<(), PatternError> =
        |value| super::super::super::literal::integer(value).map_err(PatternError::Integer);

    #[test]
    fn fixed_sizes_admit_target_independent_bits_without_dynamic_state() {
        let bindings = Bindings {
            values: Vec::new(),
            ints: 0,
        };
        let locals = Locals::default();
        for bits in [0, u64::from(u32::MAX), u64::from(u32::MAX) + 1, u64::MAX] {
            assert_eq!(
                bindings.size(&BitArrayPatternSize::Fixed(bits), &locals),
                Ok(())
            );
        }
        assert_eq!(
            bindings.size(
                &BitArrayPatternSize::Dynamic {
                    value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                    unit: 1,
                },
                &locals
            ),
            Ok(())
        );
    }

    #[test]
    fn integer_size_slots_are_dense_and_distinct_from_general_results() {
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: 0,
        };
        for (index, size) in [(0, None), (1, Some(MatchIntBindingId(0))), (2, None)] {
            assert_eq!(
                bindings.int_binding(&MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(index),
                    size,
                }),
                Ok(())
            );
        }
        assert_eq!(bindings.values.len(), 3);
        assert_eq!(bindings.ints, 1);
        for found in [0, 2] {
            let mut invalid = Bindings {
                values: Vec::new(),
                ints: 0,
            };
            assert_eq!(
                invalid.int_binding(&MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(0),
                    size: Some(MatchIntBindingId(0)),
                }),
                Ok(())
            );
            assert_eq!(
                invalid.int_binding(&MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(1),
                    size: Some(MatchIntBindingId(found)),
                }),
                Err(PatternError::IntBindingOrder { expected: 1, found })
            );
        }
    }

    #[test]
    fn aliases_preserve_binding_order_and_only_integer_values_become_size_bindings() {
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: 0,
        };
        let int: BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding> =
            BitArrayPatternValue::Alias {
                pattern: Box::new(BitArrayPatternValue::Bind(MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(0),
                    size: Some(MatchIntBindingId(0)),
                }))
                .into(),
                binding: MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(1),
                    size: Some(MatchIntBindingId(1)),
                },
            };
        assert_eq!(
            bindings.bit_value(&int, INTEGER_LITERAL, Bindings::int_binding),
            Ok(())
        );
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Literal(BigInt::from(42).into()),
                INTEGER_LITERAL,
                Bindings::int_binding,
            ),
            Ok(())
        );
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Literal(IntegerLiteral {
                    sign: Sign::Plus,
                    digits: Table::Static(&[]),
                }),
                INTEGER_LITERAL,
                Bindings::int_binding,
            ),
            Err(PatternError::Integer(IntegerError::EmptyMagnitude))
        );
        assert_eq!(bindings.ints, 2);
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Discard,
                INTEGER_LITERAL,
                Bindings::int_binding
            ),
            Ok(())
        );
        for pattern in [
            BitArrayPatternValue::Bind(MatchIntPatternBinding {
                binding: MatchPatternBinding::new(3),
                size: None,
            }),
            BitArrayPatternValue::Alias {
                pattern: Box::new(BitArrayPatternValue::Discard).into(),
                binding: MatchIntPatternBinding {
                    binding: MatchPatternBinding::new(3),
                    size: None,
                },
            },
        ] {
            assert_eq!(
                bindings.bit_value(&pattern, INTEGER_LITERAL, Bindings::int_binding),
                Err(PatternError::BindingOrder {
                    expected: 2,
                    found: 3
                })
            );
        }
        assert!(matches!(
            bindings.values.as_slice(),
            [
                BindingValue::Scalar(ValueType::Int),
                BindingValue::Scalar(ValueType::Int)
            ]
        ));

        let float = BitArrayPatternValue::Alias {
            pattern: Box::new(BitArrayPatternValue::Literal(1.5)).into(),
            binding: MatchPatternBinding::new(2),
        };
        assert_eq!(
            bindings.bit_value(&float, FLOAT_LITERAL, FLOAT_BINDING),
            Ok(())
        );
        let discard = BitArrayPatternValue::<f64>::Discard;
        assert_eq!(
            bindings.bit_value(&discard, FLOAT_LITERAL, FLOAT_BINDING),
            Ok(())
        );
        let bits = BitArrayBindingPattern::Alias {
            pattern: Box::new(BitArrayBindingPattern::Bind(MatchPatternBinding::new(3))).into(),
            binding: MatchPatternBinding::new(4),
        };
        assert_eq!(bindings.bit_binding(&bits, &ValueType::BitArray), Ok(()));
        let point = BitArrayBindingPattern::Alias {
            pattern: Box::new(BitArrayBindingPattern::Discard).into(),
            binding: MatchPatternBinding::new(5),
        };
        assert_eq!(
            bindings.bit_binding(&point, &ValueType::UtfCodepoint),
            Ok(())
        );
        assert_eq!(bindings.ints, 2);
        assert!(matches!(
            bindings.values[2..],
            [
                BindingValue::Scalar(ValueType::Float),
                BindingValue::Scalar(ValueType::BitArray),
                BindingValue::Scalar(ValueType::BitArray),
                BindingValue::Scalar(ValueType::UtfCodepoint)
            ]
        ));
        assert_eq!(
            bindings.bit_binding(&bits, &ValueType::BitArray),
            Err(PatternError::BindingOrder {
                expected: 6,
                found: 3
            })
        );
        let alias = BitArrayBindingPattern::Alias {
            pattern: Box::new(BitArrayBindingPattern::Discard).into(),
            binding: MatchPatternBinding::new(7),
        };
        assert_eq!(
            bindings.bit_binding(&alias, &ValueType::BitArray),
            Err(PatternError::BindingOrder {
                expected: 6,
                found: 7
            })
        );
        let value = BitArrayPatternValue::<f64>::Bind(MatchPatternBinding::new(7));
        assert_eq!(
            bindings.bit_value(&value, FLOAT_LITERAL, FLOAT_BINDING),
            Err(PatternError::BindingOrder {
                expected: 6,
                found: 7
            })
        );
        let alias = BitArrayPatternValue::Alias {
            pattern: Box::new(BitArrayPatternValue::<f64>::Discard).into(),
            binding: MatchPatternBinding::new(7),
        };
        assert_eq!(
            bindings.bit_value(&alias, FLOAT_LITERAL, FLOAT_BINDING),
            Err(PatternError::BindingOrder {
                expected: 6,
                found: 7
            })
        );
    }

    #[test]
    fn cyclic_literal_and_capture_aliases_are_rejected_before_binding() {
        static INT: BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding> =
            BitArrayPatternValue::Alias {
                pattern: Node::Static(&INT),
                binding: MatchIntPatternBinding {
                    binding: MatchPatternBinding { index: 0 },
                    size: Some(MatchIntBindingId(0)),
                },
            };
        static FLOAT: BitArrayPatternValue<f64> = BitArrayPatternValue::Alias {
            pattern: Node::Static(&FLOAT),
            binding: MatchPatternBinding { index: 0 },
        };
        static CAPTURE: BitArrayBindingPattern = BitArrayBindingPattern::Alias {
            pattern: Node::Static(&CAPTURE),
            binding: MatchPatternBinding { index: 0 },
        };
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: 0,
        };
        assert_eq!(
            bindings.bit_value(&INT, INTEGER_LITERAL, Bindings::int_binding),
            Err(PatternError::RecursivePattern)
        );
        assert_eq!(
            bindings.bit_value(&FLOAT, FLOAT_LITERAL, FLOAT_BINDING),
            Err(PatternError::RecursivePattern)
        );
        assert_eq!(
            bindings.bit_binding(&CAPTURE, &ValueType::BitArray),
            Err(PatternError::RecursivePattern)
        );
        assert!(bindings.values.is_empty());
        assert_eq!(bindings.ints, 0);
    }

    #[test]
    fn sizes_distinguish_shared_arithmetic_nodes_cycles_and_unavailable_bindings() {
        static ZERO: BitArrayPatternSizeExpr = BitArrayPatternSizeExpr::Value(IntegerLiteral {
            sign: Sign::NoSign,
            digits: Table::Static(&[]),
        });
        static CYCLE: BitArrayPatternSizeExpr = BitArrayPatternSizeExpr::Add {
            left: Node::Static(&ZERO),
            right: Node::Static(&CYCLE),
        };
        let locals = Locals::default();
        let bindings = Bindings {
            values: vec![BindingValue::Scalar(&ValueType::Int)],
            ints: 1,
        };
        for value in [
            BitArrayPatternSizeExpr::Add {
                left: Node::Static(&ZERO),
                right: Node::Static(&ZERO),
            },
            BitArrayPatternSizeExpr::Subtract {
                left: Node::Static(&ZERO),
                right: Node::Static(&ZERO),
            },
            BitArrayPatternSizeExpr::Multiply {
                left: Node::Static(&ZERO),
                right: Node::Static(&ZERO),
            },
            BitArrayPatternSizeExpr::Divide {
                left: Node::Static(&ZERO),
                right: Node::Static(&ZERO),
            },
            BitArrayPatternSizeExpr::Remainder {
                left: Node::Static(&ZERO),
                right: Node::Static(&ZERO),
            },
            BitArrayPatternSizeExpr::Binding(MatchIntBindingId(0)),
        ] {
            assert_eq!(
                bindings.size(&BitArrayPatternSize::Dynamic { value, unit: 1 }, &locals),
                Ok(())
            );
        }
        for (value, unit, expected) in [
            (
                BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                0,
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSizeExpr::Add {
                    left: Node::Static(&ZERO),
                    right: Node::Static(&CYCLE),
                },
                1,
                PatternError::RecursiveSize,
            ),
            (
                BitArrayPatternSizeExpr::Binding(MatchIntBindingId(1)),
                1,
                PatternError::IntBinding { index: 1 },
            ),
            (
                BitArrayPatternSizeExpr::Local(IntLocalId(0)),
                1,
                PatternError::Local(LocalError::Missing(IntLocalId(0).into())),
            ),
            (
                BitArrayPatternSizeExpr::Value(IntegerLiteral {
                    sign: Sign::Plus,
                    digits: Table::Static(&[]),
                }),
                1,
                PatternError::Integer(IntegerError::EmptyMagnitude),
            ),
        ] {
            assert_eq!(
                bindings.size(&BitArrayPatternSize::Dynamic { value, unit }, &locals),
                Err(expected)
            );
        }
    }

    #[test]
    fn segment_validation_preserves_literal_size_and_capture_diagnostics() {
        let size = BitArrayPatternSize::Dynamic {
            value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
            unit: 1,
        };
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: 0,
        };
        let pattern = BitArrayPattern {
            segments: vec![
                BitArrayPatternSegment::Float {
                    pattern: BitArrayPatternValue::Literal(1.0),
                    size: size.clone(),
                    endianness: Endianness::Big,
                },
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: Some(size.clone()),
                    unit: 1,
                },
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: None,
                    unit: 8,
                },
                BitArrayPatternSegment::String {
                    pattern: BitArrayStringPattern::Literal("x".into()),
                    encoding: StringEncoding::Utf8,
                },
                BitArrayPatternSegment::UtfCodepoint {
                    pattern: BitArrayBindingPattern::Bind(MatchPatternBinding::new(0)),
                    encoding: StringEncoding::Utf8,
                },
            ]
            .into(),
        };
        assert_eq!(bindings.bits(&pattern, &Locals::default()), Ok(()));
        assert!(matches!(
            bindings.values.as_slice(),
            [BindingValue::Scalar(ValueType::UtfCodepoint)]
        ));
        for (segment, expected) in [
            (
                BitArrayPatternSegment::Int {
                    pattern: BitArrayPatternValue::Discard,
                    size: BitArrayPatternSize::Dynamic {
                        value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                        unit: 0,
                    },
                    endianness: Endianness::Big,
                    signedness: crate::plan::execution::graph::Signedness::Unsigned,
                },
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSegment::Float {
                    pattern: BitArrayPatternValue::Discard,
                    size: BitArrayPatternSize::Dynamic {
                        value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                        unit: 0,
                    },
                    endianness: Endianness::Big,
                },
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSegment::Int {
                    pattern: BitArrayPatternValue::Literal(IntegerLiteral {
                        sign: Sign::Plus,
                        digits: Table::Static(&[]),
                    }),
                    size: size.clone(),
                    endianness: Endianness::Big,
                    signedness: crate::plan::execution::graph::Signedness::Unsigned,
                },
                PatternError::Integer(IntegerError::EmptyMagnitude),
            ),
            (
                BitArrayPatternSegment::Float {
                    pattern: BitArrayPatternValue::Bind(MatchPatternBinding::new(9)),
                    size: size.clone(),
                    endianness: Endianness::Big,
                },
                PatternError::BindingOrder {
                    expected: 1,
                    found: 9,
                },
            ),
            (
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: None,
                    unit: 0,
                },
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: Some(BitArrayPatternSize::Dynamic {
                        value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                        unit: 0,
                    }),
                    unit: 1,
                },
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Bind(MatchPatternBinding::new(9)),
                    size: None,
                    unit: 1,
                },
                PatternError::BindingOrder {
                    expected: 1,
                    found: 9,
                },
            ),
            (
                BitArrayPatternSegment::UtfCodepoint {
                    pattern: BitArrayBindingPattern::Bind(MatchPatternBinding::new(9)),
                    encoding: StringEncoding::Utf8,
                },
                PatternError::BindingOrder {
                    expected: 1,
                    found: 9,
                },
            ),
        ] {
            assert_eq!(
                bindings.bits(
                    &BitArrayPattern {
                        segments: vec![segment].into()
                    },
                    &Locals::default()
                ),
                Err(expected)
            );
        }
    }
}
