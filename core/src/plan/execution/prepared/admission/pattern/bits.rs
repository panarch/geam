use super::{BindingValue, Bindings, PatternError};
use crate::plan::execution::graph::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, MatchPatternBinding,
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
                    self.bit_value(pattern, &ValueType::Int, true, |value| {
                        super::super::literal::integer(value).map_err(PatternError::Integer)
                    })?;
                }
                BitArrayPatternSegment::Float { pattern, size, .. } => {
                    self.size(size, locals)?;
                    self.bit_value(pattern, &ValueType::Float, false, |_| Ok(()))?;
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
        if size.unit == 0 {
            return Err(PatternError::ZeroUnit);
        }
        enum Visit<'a> {
            Enter(&'a BitArrayPatternSizeExpr),
            Leave(*const BitArrayPatternSizeExpr),
        }
        let mut pending = vec![Visit::Enter(&size.value)];
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
                    if !self.ints.contains(&binding.0) {
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

    fn bit_value<Value>(
        &mut self,
        root: &BitArrayPatternValue<Value>,
        type_: &'static ValueType,
        integer: bool,
        validate: fn(&Value) -> Result<(), PatternError>,
    ) -> Result<(), PatternError> {
        let mut aliases = Vec::new();
        let mut visited = HashSet::new();
        let mut pattern = root;
        loop {
            if !visited.insert(pattern as *const BitArrayPatternValue<Value>) {
                return Err(PatternError::RecursivePattern);
            }
            match pattern {
                BitArrayPatternValue::Literal(value) => {
                    validate(value)?;
                    break;
                }
                BitArrayPatternValue::Discard => break,
                BitArrayPatternValue::Bind(binding) => {
                    self.bit_bind(binding, type_, integer)?;
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
            self.bit_bind(binding, type_, integer)?;
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
                    self.bit_bind(binding, type_, false)?;
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
            self.bit_bind(binding, type_, false)?;
        }
        Ok(())
    }

    fn bit_bind(
        &mut self,
        binding: &MatchPatternBinding,
        type_: &'static ValueType,
        integer: bool,
    ) -> Result<(), PatternError> {
        self.bind(binding, BindingValue::Scalar(type_))?;
        if integer {
            self.ints.insert(binding.index);
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
        StringEncoding,
    };
    use crate::plan::execution::prepared::admission::{literal::IntegerError, local::LocalError};
    use crate::plan::execution::storage::{Node, Table};
    use num_bigint::{BigInt, Sign};
    use std::collections::HashSet;

    const FLOAT_LITERAL: fn(&f64) -> Result<(), PatternError> = |_| Ok(());
    const INTEGER_LITERAL: fn(&IntegerLiteral) -> Result<(), PatternError> =
        |value| super::super::super::literal::integer(value).map_err(PatternError::Integer);

    #[test]
    fn aliases_preserve_binding_order_and_only_integer_values_become_size_bindings() {
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: HashSet::new(),
        };
        let int: BitArrayPatternValue<IntegerLiteral> = BitArrayPatternValue::Alias {
            pattern: Box::new(BitArrayPatternValue::Bind(MatchPatternBinding::new(0))).into(),
            binding: MatchPatternBinding::new(1),
        };
        assert_eq!(
            bindings.bit_value(&int, &ValueType::Int, true, INTEGER_LITERAL),
            Ok(())
        );
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Literal(BigInt::from(42).into()),
                &ValueType::Int,
                true,
                INTEGER_LITERAL,
            ),
            Ok(())
        );
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Literal(IntegerLiteral {
                    sign: Sign::Plus,
                    digits: Table::Static(&[]),
                }),
                &ValueType::Int,
                true,
                INTEGER_LITERAL,
            ),
            Err(PatternError::Integer(IntegerError::EmptyMagnitude))
        );
        assert_eq!(bindings.ints, HashSet::from([0, 1]));
        assert_eq!(
            bindings.bit_value(
                &BitArrayPatternValue::Discard,
                &ValueType::Int,
                true,
                INTEGER_LITERAL
            ),
            Ok(())
        );
        for pattern in [
            BitArrayPatternValue::Bind(MatchPatternBinding::new(3)),
            BitArrayPatternValue::Alias {
                pattern: Box::new(BitArrayPatternValue::Discard).into(),
                binding: MatchPatternBinding::new(3),
            },
        ] {
            assert_eq!(
                bindings.bit_value(&pattern, &ValueType::Int, true, INTEGER_LITERAL),
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
            bindings.bit_value(&float, &ValueType::Float, false, FLOAT_LITERAL),
            Ok(())
        );
        let discard = BitArrayPatternValue::<f64>::Discard;
        assert_eq!(
            bindings.bit_value(&discard, &ValueType::Float, false, FLOAT_LITERAL),
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
        assert_eq!(bindings.ints, HashSet::from([0, 1]));
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
            bindings.bit_value(&value, &ValueType::Float, false, FLOAT_LITERAL),
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
            bindings.bit_value(&alias, &ValueType::Float, false, FLOAT_LITERAL),
            Err(PatternError::BindingOrder {
                expected: 6,
                found: 7
            })
        );
    }

    #[test]
    fn cyclic_literal_and_capture_aliases_are_rejected_before_binding() {
        static INT: BitArrayPatternValue<IntegerLiteral> = BitArrayPatternValue::Alias {
            pattern: Node::Static(&INT),
            binding: MatchPatternBinding { index: 0 },
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
            ints: HashSet::new(),
        };
        assert_eq!(
            bindings.bit_value(&INT, &ValueType::Int, true, INTEGER_LITERAL),
            Err(PatternError::RecursivePattern)
        );
        assert_eq!(
            bindings.bit_value(&FLOAT, &ValueType::Float, false, FLOAT_LITERAL),
            Err(PatternError::RecursivePattern)
        );
        assert_eq!(
            bindings.bit_binding(&CAPTURE, &ValueType::BitArray),
            Err(PatternError::RecursivePattern)
        );
        assert!(bindings.values.is_empty());
        assert!(bindings.ints.is_empty());
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
            ints: HashSet::from([0]),
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
                bindings.size(&BitArrayPatternSize { value, unit: 1 }, &locals),
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
                bindings.size(&BitArrayPatternSize { value, unit }, &locals),
                Err(expected)
            );
        }
    }

    #[test]
    fn segment_validation_preserves_literal_size_and_capture_diagnostics() {
        let size = BitArrayPatternSize {
            value: BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
            unit: 1,
        };
        let mut bindings = Bindings {
            values: Vec::new(),
            ints: HashSet::new(),
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
                    size: BitArrayPatternSize {
                        unit: 0,
                        ..size.clone()
                    },
                    endianness: Endianness::Big,
                    signedness: crate::plan::execution::graph::Signedness::Unsigned,
                },
                PatternError::ZeroUnit,
            ),
            (
                BitArrayPatternSegment::Float {
                    pattern: BitArrayPatternValue::Discard,
                    size: BitArrayPatternSize {
                        unit: 0,
                        ..size.clone()
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
                    size: Some(BitArrayPatternSize {
                        unit: 0,
                        ..size.clone()
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
