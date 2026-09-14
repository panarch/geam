use super::{MatchIntBindingId, MatchPatternBinding};
use crate::plan::Text;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::IntegerLiteral;
use crate::plan::execution::graph::{Endianness, IntLocalId, StringEncoding};
use crate::plan::execution::graph::{LocalLabel, endianness, string_encoding};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Clone)]
pub struct BitArrayPattern {
    pub segments: Table<BitArrayPatternSegment>,
}

#[derive(Clone)]
pub enum BitArrayPatternSegment {
    Int {
        pattern: BitArrayPatternValue<IntegerLiteral>,
        size: BitArrayPatternSize,
        endianness: Endianness,
        signedness: Signedness,
    },
    Float {
        pattern: BitArrayPatternValue<f64>,
        size: BitArrayPatternSize,
        endianness: Endianness,
    },
    Bits {
        pattern: BitArrayBindingPattern,
        size: Option<BitArrayPatternSize>,
        unit: u8,
    },
    String {
        pattern: BitArrayStringPattern,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        pattern: BitArrayBindingPattern,
        encoding: StringEncoding,
    },
}

#[derive(Clone)]
pub struct BitArrayPatternSize {
    pub value: BitArrayPatternSizeExpr,
    pub unit: u8,
}

#[derive(Clone)]
pub enum BitArrayPatternSizeExpr {
    Value(IntegerLiteral),
    Local(IntLocalId),
    Binding(MatchIntBindingId),
    Add { left: Node<Self>, right: Node<Self> },
    Subtract { left: Node<Self>, right: Node<Self> },
    Multiply { left: Node<Self>, right: Node<Self> },
    Divide { left: Node<Self>, right: Node<Self> },
    Remainder { left: Node<Self>, right: Node<Self> },
}

#[derive(Clone)]
pub enum BitArrayPatternValue<Value: 'static> {
    Literal(Value),
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Node<Self>,
        binding: MatchPatternBinding,
    },
}

#[derive(Clone)]
pub enum BitArrayStringPattern {
    Literal(Text),
    Discard,
}

#[derive(Clone)]
pub enum BitArrayBindingPattern {
    Bind(MatchPatternBinding),
    Discard,
    Alias {
        pattern: Node<Self>,
        binding: MatchPatternBinding,
    },
}

impl BitArrayPattern {
    pub(in crate::plan::execution) fn new(segments: Vec<BitArrayPatternSegment>) -> Self {
        Self {
            segments: segments.into(),
        }
    }

    pub(crate) fn segments(&self) -> &[BitArrayPatternSegment] {
        &self.segments
    }
}

impl BitArrayPatternSize {
    pub(in crate::plan::execution) fn new(value: BitArrayPatternSizeExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &BitArrayPatternSizeExpr {
        &self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

impl Explain for BitArrayPattern {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("<<");
        for (index, segment) in self.segments().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            context.write(segment);
        }
        context.push_str(">>");
    }
}

impl Explain for BitArrayPatternSegment {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Int {
                pattern,
                size,
                endianness: order,
                signedness,
            } => {
                context.push_str("int(");
                write_value(context, pattern, |context, value| {
                    context.push_str(&value.to_string());
                });
                context.push_str(", size=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push_str(", ");
                context.push_str(match signedness {
                    Signedness::Signed => "signed",
                    Signedness::Unsigned => "unsigned",
                });
                context.push(')');
            }
            Self::Float {
                pattern,
                size,
                endianness: order,
            } => {
                context.push_str("float(");
                write_value(context, pattern, |context, value| {
                    context.push_str(&format!("{value:?}"));
                });
                context.push_str(", size=");
                context.write(size);
                context.push_str(", ");
                context.push_str(endianness(*order));
                context.push(')');
            }
            Self::Bits {
                pattern,
                size,
                unit,
            } => {
                context.push_str("bits(");
                context.write(pattern);
                context.push_str(", size=");
                match size {
                    Some(size) => context.write(size),
                    None => context.push_str("rest"),
                }
                context.push_str(", unit=");
                context.push_str(&unit.to_string());
                context.push(')');
            }
            Self::String { pattern, encoding } => {
                context.push_str("string(");
                match pattern {
                    BitArrayStringPattern::Literal(value) => {
                        context.push_str(&format!("{value:?}"));
                    }
                    BitArrayStringPattern::Discard => context.push('_'),
                }
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
            Self::UtfCodepoint { pattern, encoding } => {
                context.push_str("utf_codepoint(");
                context.write(pattern);
                context.push_str(", ");
                context.push_str(string_encoding(*encoding));
                context.push(')');
            }
        }
    }
}

impl Explain for BitArrayBindingPattern {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Bind(binding) => context.write(binding),
            Self::Discard => context.push('_'),
            Self::Alias { pattern, binding } => {
                context.push_str("alias(");
                context.write(pattern.as_ref());
                context.push_str(", ");
                context.write(binding);
                context.push(')');
            }
        }
    }
}

impl Explain for BitArrayPatternSize {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.write(self.value());
        context.push('*');
        context.push_str(&self.unit().to_string());
    }
}

impl Explain for BitArrayPatternSizeExpr {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Value(value) => context.push_str(&value.to_string()),
            Self::Local(local) => local.write_local_label(context.output()),
            Self::Binding(binding) => {
                context.push_str("binding#");
                context.push_str(&binding.index().to_string());
            }
            Self::Add { left, right } => write_binary(context, "+", left, right),
            Self::Subtract { left, right } => write_binary(context, "-", left, right),
            Self::Multiply { left, right } => write_binary(context, "*", left, right),
            Self::Divide { left, right } => write_binary(context, "/", left, right),
            Self::Remainder { left, right } => write_binary(context, "%", left, right),
        }
    }
}

fn write_binary(
    context: &mut ExplainContext<'_, '_>,
    operator: &str,
    left: &BitArrayPatternSizeExpr,
    right: &BitArrayPatternSizeExpr,
) {
    context.push('(');
    context.write(left);
    context.push(' ');
    context.push_str(operator);
    context.push(' ');
    context.write(right);
    context.push(')');
}

fn write_value<Value>(
    context: &mut ExplainContext<'_, '_>,
    pattern: &BitArrayPatternValue<Value>,
    write_literal: impl Copy + Fn(&mut ExplainContext<'_, '_>, &Value),
) {
    match pattern {
        BitArrayPatternValue::Literal(value) => write_literal(context, value),
        BitArrayPatternValue::Bind(binding) => context.write(binding),
        BitArrayPatternValue::Discard => context.push('_'),
        BitArrayPatternValue::Alias { pattern, binding } => {
            context.push_str("alias(");
            write_value(context, pattern, write_literal);
            context.push_str(", ");
            context.write(binding);
            context.push(')');
        }
    }
}

impl Emit for Signedness {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Signed => output.path("graph::Signedness::Signed"),
            Self::Unsigned => output.path("graph::Signedness::Unsigned"),
        }
    }
}

impl Emit for BitArrayPattern {
    fn emit(&self, output: &mut Rust) {
        let Self { segments } = self;
        output.structure("graph::BitArrayPattern", &[("segments", segments)]);
    }
}

impl Emit for BitArrayPatternSegment {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int {
                pattern,
                size,
                endianness,
                signedness,
            } => output.structure(
                "graph::BitArrayPatternSegment::Int",
                &[
                    ("pattern", pattern),
                    ("size", size),
                    ("endianness", endianness),
                    ("signedness", signedness),
                ],
            ),
            Self::Float {
                pattern,
                size,
                endianness,
            } => output.structure(
                "graph::BitArrayPatternSegment::Float",
                &[
                    ("pattern", pattern),
                    ("size", size),
                    ("endianness", endianness),
                ],
            ),
            Self::Bits {
                pattern,
                size,
                unit,
            } => output.structure(
                "graph::BitArrayPatternSegment::Bits",
                &[("pattern", pattern), ("size", size), ("unit", unit)],
            ),
            Self::String { pattern, encoding } => output.structure(
                "graph::BitArrayPatternSegment::String",
                &[("pattern", pattern), ("encoding", encoding)],
            ),
            Self::UtfCodepoint { pattern, encoding } => output.structure(
                "graph::BitArrayPatternSegment::UtfCodepoint",
                &[("pattern", pattern), ("encoding", encoding)],
            ),
        }
    }
}

impl Emit for BitArrayPatternSize {
    fn emit(&self, output: &mut Rust) {
        let Self { value, unit } = self;
        output.structure(
            "graph::BitArrayPatternSize",
            &[("value", value), ("unit", unit)],
        );
    }
}

impl Emit for BitArrayPatternSizeExpr {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => {
                output.call("graph::BitArrayPatternSizeExpr::Value", &[field_0])
            }
            Self::Local(field_0) => {
                output.call("graph::BitArrayPatternSizeExpr::Local", &[field_0])
            }
            Self::Binding(field_0) => {
                output.call("graph::BitArrayPatternSizeExpr::Binding", &[field_0])
            }
            Self::Add { left, right } => output.structure(
                "graph::BitArrayPatternSizeExpr::Add",
                &[("left", left), ("right", right)],
            ),
            Self::Subtract { left, right } => output.structure(
                "graph::BitArrayPatternSizeExpr::Subtract",
                &[("left", left), ("right", right)],
            ),
            Self::Multiply { left, right } => output.structure(
                "graph::BitArrayPatternSizeExpr::Multiply",
                &[("left", left), ("right", right)],
            ),
            Self::Divide { left, right } => output.structure(
                "graph::BitArrayPatternSizeExpr::Divide",
                &[("left", left), ("right", right)],
            ),
            Self::Remainder { left, right } => output.structure(
                "graph::BitArrayPatternSizeExpr::Remainder",
                &[("left", left), ("right", right)],
            ),
        }
    }
}

impl<Value: 'static> Emit for BitArrayPatternValue<Value>
where
    Value: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Literal(field_0) => {
                output.call("graph::BitArrayPatternValue::Literal", &[field_0])
            }
            Self::Bind(field_0) => output.call("graph::BitArrayPatternValue::Bind", &[field_0]),
            Self::Discard => output.path("graph::BitArrayPatternValue::Discard"),
            Self::Alias { pattern, binding } => output.structure(
                "graph::BitArrayPatternValue::Alias",
                &[("pattern", pattern), ("binding", binding)],
            ),
        }
    }
}

impl Emit for BitArrayStringPattern {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Literal(field_0) => {
                output.call("graph::BitArrayStringPattern::Literal", &[field_0])
            }
            Self::Discard => output.path("graph::BitArrayStringPattern::Discard"),
        }
    }
}

impl Emit for BitArrayBindingPattern {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Bind(field_0) => output.call("graph::BitArrayBindingPattern::Bind", &[field_0]),
            Self::Discard => output.path("graph::BitArrayBindingPattern::Discard"),
            Self::Alias { pattern, binding } => output.structure(
                "graph::BitArrayBindingPattern::Alias",
                &[("pattern", pattern), ("binding", binding)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Endianness,
        IntLocalId, IntegerLiteral, MatchIntBindingId, MatchPatternBinding, Node, Rust, Signedness,
        StringEncoding,
    };

    #[test]
    fn emits_bit_pattern_sizes_and_every_arithmetic_operation() {
        let left = Node::Static(&BitArrayPatternSizeExpr::Local(IntLocalId(1)));
        let right = Node::Static(&BitArrayPatternSizeExpr::Binding(MatchIntBindingId(2)));
        let cases = [
            (
                BitArrayPatternSizeExpr::Value(num_bigint::BigInt::from(8).into()),
                r#"
data::graph::BitArrayPatternSizeExpr::Value(data::graph::IntegerLiteral {
    sign: data::Sign::Plus,
    digits: data::Storage::Static(&[
        8,
    ]),
})"#.trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSizeExpr::Local(IntLocalId(1)),
                "data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))",
            ),
            (
                BitArrayPatternSizeExpr::Binding(MatchIntBindingId(2)),
                "data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))",
            ),
            (
                BitArrayPatternSizeExpr::Add {
                    left: left.clone(),
                    right: right.clone(),
                },
                r#"
data::graph::BitArrayPatternSizeExpr::Add {
    left: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))),
    right: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSizeExpr::Subtract {
                    left: left.clone(),
                    right: right.clone(),
                },
                r#"
data::graph::BitArrayPatternSizeExpr::Subtract {
    left: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))),
    right: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSizeExpr::Multiply {
                    left: left.clone(),
                    right: right.clone(),
                },
                r#"
data::graph::BitArrayPatternSizeExpr::Multiply {
    left: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))),
    right: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSizeExpr::Divide {
                    left: left.clone(),
                    right: right.clone(),
                },
                r#"
data::graph::BitArrayPatternSizeExpr::Divide {
    left: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))),
    right: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))),
}"#.trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSizeExpr::Remainder { left, right },
                r#"
data::graph::BitArrayPatternSizeExpr::Remainder {
    left: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1))),
    right: data::Storage::Static(&data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(2))),
}"#.trim_start_matches('\n'),
            ),
        ];
        for (value, expected) in cases {
            assert_eq!(Rust::expression(&value), expected);
        }
        let size = BitArrayPatternSize::new(BitArrayPatternSizeExpr::Local(IntLocalId(1)), 8);
        assert_eq!(
            Rust::expression(&size),
            r#"
data::graph::BitArrayPatternSize {
    value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
    unit: 8,
}"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_integer_and_float_literal_binding_and_alias_patterns() {
        let integers = [
            (
                BitArrayPatternValue::Literal(num_bigint::BigInt::from(-3).into()),
                r#"
data::graph::BitArrayPatternValue::Literal(data::graph::IntegerLiteral {
    sign: data::Sign::Minus,
    digits: data::Storage::Static(&[
        3,
    ]),
})"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternValue::Bind(MatchPatternBinding::new(2)),
                r#"
data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
    index: 2,
})"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternValue::Discard,
                "data::graph::BitArrayPatternValue::Discard",
            ),
            (
                BitArrayPatternValue::<IntegerLiteral>::Alias {
                    pattern: Node::Static(&BitArrayPatternValue::Discard),
                    binding: MatchPatternBinding::new(2),
                },
                r#"
data::graph::BitArrayPatternValue::Alias {
    pattern: data::Storage::Static(&data::graph::BitArrayPatternValue::Discard),
    binding: data::graph::MatchPatternBinding {
        index: 2,
    },
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (pattern, expected) in integers {
            assert_eq!(Rust::expression(&pattern), expected);
        }
        let floats = [
            (
                BitArrayPatternValue::Literal(-0.0f64),
                "data::graph::BitArrayPatternValue::Literal(f64::from_bits(9223372036854775808))",
            ),
            (
                BitArrayPatternValue::Bind(MatchPatternBinding::new(3)),
                r#"
data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
    index: 3,
})"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternValue::Discard,
                "data::graph::BitArrayPatternValue::Discard",
            ),
            (
                BitArrayPatternValue::Alias {
                    pattern: Node::Static(&BitArrayPatternValue::Discard),
                    binding: MatchPatternBinding::new(3),
                },
                r#"
data::graph::BitArrayPatternValue::Alias {
    pattern: data::Storage::Static(&data::graph::BitArrayPatternValue::Discard),
    binding: data::graph::MatchPatternBinding {
        index: 3,
    },
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (pattern, expected) in floats {
            assert_eq!(Rust::expression(&pattern), expected);
        }
    }

    #[test]
    fn emits_string_and_bit_binding_patterns() {
        for (pattern, expected) in [
            (
                BitArrayBindingPattern::Bind(MatchPatternBinding::new(2)),
                r#"
data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
    index: 2,
})"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayBindingPattern::Discard,
                "data::graph::BitArrayBindingPattern::Discard",
            ),
            (
                BitArrayBindingPattern::Alias {
                    pattern: Node::Static(&BitArrayBindingPattern::Discard),
                    binding: MatchPatternBinding::new(2),
                },
                r#"
data::graph::BitArrayBindingPattern::Alias {
    pattern: data::Storage::Static(&data::graph::BitArrayBindingPattern::Discard),
    binding: data::graph::MatchPatternBinding {
        index: 2,
    },
}"#
                .trim_start_matches('\n'),
            ),
        ] {
            assert_eq!(Rust::expression(&pattern), expected);
        }
        for (pattern, expected) in [
            (
                BitArrayStringPattern::Literal("a\n".into()),
                "data::graph::BitArrayStringPattern::Literal(data::Text::Static(\"a\\n\"))",
            ),
            (
                BitArrayStringPattern::Discard,
                "data::graph::BitArrayStringPattern::Discard",
            ),
        ] {
            assert_eq!(Rust::expression(&pattern), expected);
        }
    }

    #[test]
    fn emits_all_segment_kinds_with_explicit_size_unit_and_encoding() {
        assert_eq!(
            Rust::expression(&Signedness::Signed),
            "data::graph::Signedness::Signed"
        );
        assert_eq!(
            Rust::expression(&Signedness::Unsigned),
            "data::graph::Signedness::Unsigned"
        );
        let size = BitArrayPatternSize::new(BitArrayPatternSizeExpr::Local(IntLocalId(1)), 8);
        let cases = [
            (
                BitArrayPatternSegment::Int {
                    pattern: BitArrayPatternValue::Discard,
                    size: size.clone(),
                    endianness: Endianness::Little,
                    signedness: Signedness::Signed,
                },
                r#"
data::graph::BitArrayPatternSegment::Int {
    pattern: data::graph::BitArrayPatternValue::Discard,
    size: data::graph::BitArrayPatternSize {
        value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
        unit: 8,
    },
    endianness: data::graph::Endianness::Little,
    signedness: data::graph::Signedness::Signed,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSegment::Float {
                    pattern: BitArrayPatternValue::Discard,
                    size: size.clone(),
                    endianness: Endianness::Big,
                },
                r#"
data::graph::BitArrayPatternSegment::Float {
    pattern: data::graph::BitArrayPatternValue::Discard,
    size: data::graph::BitArrayPatternSize {
        value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
        unit: 8,
    },
    endianness: data::graph::Endianness::Big,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: Some(size),
                    unit: 1,
                },
                r#"
data::graph::BitArrayPatternSegment::Bits {
    pattern: data::graph::BitArrayBindingPattern::Discard,
    size: Some(data::graph::BitArrayPatternSize {
        value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
        unit: 8,
    }),
    unit: 1,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Discard,
                    size: None,
                    unit: 8,
                },
                r#"
data::graph::BitArrayPatternSegment::Bits {
    pattern: data::graph::BitArrayBindingPattern::Discard,
    size: None,
    unit: 8,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSegment::String {
                    pattern: BitArrayStringPattern::Discard,
                    encoding: StringEncoding::Utf8,
                },
                r#"
data::graph::BitArrayPatternSegment::String {
    pattern: data::graph::BitArrayStringPattern::Discard,
    encoding: data::graph::StringEncoding::Utf8,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BitArrayPatternSegment::UtfCodepoint {
                    pattern: BitArrayBindingPattern::Discard,
                    encoding: StringEncoding::Utf16(Endianness::Little),
                },
                r#"
data::graph::BitArrayPatternSegment::UtfCodepoint {
    pattern: data::graph::BitArrayBindingPattern::Discard,
    encoding: data::graph::StringEncoding::Utf16(data::graph::Endianness::Little),
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (segment, expected) in cases {
            assert_eq!(Rust::expression(&segment), expected);
        }
        let pattern = BitArrayPattern::new(vec![BitArrayPatternSegment::String {
            pattern: BitArrayStringPattern::Discard,
            encoding: StringEncoding::Utf8,
        }]);
        assert_eq!(
            Rust::expression(&pattern),
            r#"
data::graph::BitArrayPattern {
    segments: data::Storage::Static(&[
        data::graph::BitArrayPatternSegment::String {
            pattern: data::graph::BitArrayStringPattern::Discard,
            encoding: data::graph::StringEncoding::Utf8,
        },
    ]),
}"#
            .trim_start_matches('\n')
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::super::Terminator;
    use super::{
        BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, Signedness,
    };
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        Endianness, IntLocalId, MatchPattern, MatchPatternBinding,
    };
    use num_bigint::BigInt;

    #[test]
    fn writes_dynamic_and_remainder_bit_array_segments() {
        let source = r#"
fn identity(bits: BitArray) { bits }

pub fn main() {
  let bits = identity(<<1, 2>>)
  let size = 8
  let assert <<value:size(size), rest:bits>> = bits
  value
}
"#;
        let expected =
            "<<int(binding#0, size=%int#2*1, big, unsigned), bits(binding#1, size=rest, unit=1)>>";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_bit_array_pattern_segment() {
        let source = "pub fn main() { 1 }";
        let expected = "int(1, size=8*1, big, unsigned)";

        explain::assert_rendered(source, expected, |plan, output| {
            let segment = BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Literal(BigInt::from(1).into()),
                size: BitArrayPatternSize::new(
                    BitArrayPatternSizeExpr::Value(BigInt::from(8).into()),
                    1,
                ),
                endianness: Endianness::Big,
                signedness: Signedness::Unsigned,
            };
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&segment);
        });
    }

    #[test]
    fn writes_bit_array_binding_pattern() {
        let source = "pub fn main() { 1 }";
        let expected = "alias(_, binding#2)";

        explain::assert_rendered(source, expected, |plan, output| {
            let pattern = BitArrayBindingPattern::Alias {
                pattern: Box::new(BitArrayBindingPattern::Discard).into(),
                binding: MatchPatternBinding::new(2),
            };
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&pattern);
        });
    }

    #[test]
    fn writes_bit_array_pattern_size() {
        let source = "pub fn main() { 1 }";
        let expected = "%int#2*4";

        explain::assert_rendered(source, expected, |plan, output| {
            let size = BitArrayPatternSize::new(BitArrayPatternSizeExpr::Local(IntLocalId(2)), 4);
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&size);
        });
    }

    #[test]
    fn writes_bit_array_pattern_size_expression() {
        let source = "pub fn main() { 1 }";
        let expected = "(8 + binding#2)";

        explain::assert_rendered(source, expected, |plan, output| {
            let expression = BitArrayPatternSizeExpr::Add {
                left: Box::new(BitArrayPatternSizeExpr::Value(BigInt::from(8).into())).into(),
                right: Box::new(BitArrayPatternSizeExpr::Binding(
                    super::MatchIntBindingId::new(2),
                ))
                .into(),
            };
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&expression);
        });
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            match_pattern(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower a BitArray match pattern")]
    fn bit_array_pattern_shape_guard_is_visible() {
        let source = r#"
fn select(value: Int) {
  let assert 1 = value
  value
}
pub fn main() { select(1) }
"#;
        explain::with_execution_plan(source, |plan| {
            bit_array_pattern(match_pattern(&terminators_for(plan, IntFunctionId(1))));
        });
    }

    #[test]
    #[should_panic(expected = "let assert should lower to one match terminator")]
    fn match_pattern_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(bits: BitArray) { bits }

pub fn main() {
  let bits = identity(<<1>>)
  let assert <<value, rest:bits>> = bits
  value
}
"#;
        explain::with_execution_plan(source, |plan| {
            let pattern = match_pattern(&terminators(plan));
            match_pattern_from_nodes(&[pattern, pattern]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        terminators_for(plan, IntFunctionId(0))
    }

    fn terminators_for(
        plan: &crate::plan::execution::ExecutionPlan,
        function: IntFunctionId,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        plan.int_function(function)
            .body()
            .block_graph()
            .blocks()
            .map(|block| block.terminator())
            .collect()
    }

    fn match_pattern<'a>(terminators: &[&'a Terminator]) -> &'a MatchPattern {
        let patterns = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Match(matcher) => Some(matcher.pattern()),
                _ => None,
            })
            .collect::<Vec<_>>();
        match_pattern_from_nodes(&patterns)
    }

    fn match_pattern_from_nodes<'a>(patterns: &[&'a MatchPattern]) -> &'a MatchPattern {
        let [pattern] = patterns else {
            if patterns.is_empty() {
                panic!("let assert should lower to a match terminator");
            }
            panic!("let assert should lower to one match terminator");
        };
        pattern
    }

    fn bit_array_pattern(pattern: &MatchPattern) -> &BitArrayPattern {
        let MatchPattern::BitArray(pattern) = pattern else {
            panic!("source should lower a BitArray match pattern");
        };
        pattern
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let pattern = bit_array_pattern(match_pattern(&terminators(plan)));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(pattern);
        });
    }
}
