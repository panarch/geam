use bitvec::vec::BitVec;
use num_bigint::BigInt;
use std::collections::HashMap;

use super::bit_array;
use super::environment::BlockEnvironment;
use crate::plan::execution::graph::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, MatchIntBindingId,
    MatchPattern, MatchPatternBinding, MatchPatternListTail,
};
use crate::runtime::InvariantError;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};

pub(super) struct MatchBindings {
    values: Vec<EvaluatedValue>,
    ints: HashMap<MatchIntBindingId, BigInt>,
}

impl MatchBindings {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            ints: HashMap::new(),
        }
    }

    fn bind(&mut self, _binding: &MatchPatternBinding, value: EvaluatedValue) {
        self.values.push(value);
    }

    fn bind_int(&mut self, binding: &MatchPatternBinding, value: &BigInt) {
        self.ints.insert(binding.int_id(), value.clone());
        self.bind(binding, EvaluatedValue::Int(value.clone()));
    }

    fn int(&self, binding: MatchIntBindingId) -> BigInt {
        self.ints[&binding].clone()
    }

    pub(super) fn value(&self, index: usize) -> EvaluatedValue {
        self.values[index].clone()
    }
}

pub(super) fn match_pattern<Plan>(
    plan: &Plan,
    lists: &mut crate::runtime::RuntimeListStorage,
    environment: &BlockEnvironment,
    pattern: &MatchPattern,
    subject: &EvaluatedValue,
) -> Result<Option<MatchBindings>, InvariantError>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
{
    let mut bindings = MatchBindings::new();
    if matches(plan, lists, environment, pattern, subject, &mut bindings)? {
        Ok(Some(bindings))
    } else {
        Ok(None)
    }
}

fn matches<Plan>(
    plan: &Plan,
    lists: &mut crate::runtime::RuntimeListStorage,
    environment: &BlockEnvironment,
    pattern: &MatchPattern,
    value: &EvaluatedValue,
    bindings: &mut MatchBindings,
) -> Result<bool, InvariantError>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
{
    match pattern {
        MatchPattern::Bind(binding) => {
            bindings.bind(binding, value.clone());
            Ok(true)
        }
        MatchPattern::Discard => Ok(true),
        MatchPattern::Int(pattern) => {
            Ok(matches!(value, EvaluatedValue::Int(value) if value == pattern))
        }
        MatchPattern::Float(pattern) => {
            Ok(matches!(value, EvaluatedValue::Float(value) if value == pattern))
        }
        MatchPattern::String(pattern) => {
            Ok(matches!(value, EvaluatedValue::String(value) if value == pattern))
        }
        MatchPattern::Bool(pattern) => {
            Ok(matches!(value, EvaluatedValue::Bool(value) if value == pattern))
        }
        MatchPattern::Nil => Ok(matches!(value, EvaluatedValue::Nil)),
        MatchPattern::Tuple(patterns) => {
            let EvaluatedValue::Tuple(values) = value else {
                return Ok(false);
            };
            if patterns.len() != values.len() {
                return Ok(false);
            }
            for (index, pattern) in patterns.iter().enumerate() {
                let value = &values[index];
                if !matches(plan, lists, environment, pattern, value, bindings)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        MatchPattern::List(pattern) => {
            let value = match value {
                EvaluatedValue::ParameterList(value) => {
                    if !pattern.elements().is_empty() {
                        return Ok(false);
                    }
                    if let Some(MatchPatternListTail::Bind(binding)) = pattern.tail() {
                        bindings.bind(binding, EvaluatedValue::ParameterList(*value));
                    }
                    return Ok(true);
                }
                EvaluatedValue::List(value) => value,
                _ => return Ok(false),
            };
            let values = lists.evaluated_values(value);
            let element_count = pattern.elements().len();
            if pattern.tail().is_some() {
                if values.len() < element_count {
                    return Ok(false);
                }
            } else if values.len() != element_count {
                return Ok(false);
            }
            for (index, pattern) in pattern.elements().iter().enumerate() {
                let value = &values[index];
                if !matches(plan, lists, environment, pattern, value, bindings)? {
                    return Ok(false);
                }
            }
            if let Some(MatchPatternListTail::Bind(binding)) = pattern.tail() {
                let tail = lists.drop_first(value, element_count);
                bindings.bind(binding, EvaluatedValue::List(tail));
            }
            Ok(true)
        }
        MatchPattern::BitArray(pattern) => {
            let EvaluatedValue::BitArray(value) = value else {
                return Ok(false);
            };
            Ok(match_bit_array(environment, value, pattern, bindings))
        }
        MatchPattern::Custom {
            constructor,
            fields,
        } => {
            let EvaluatedValue::Custom(value) = value else {
                return Ok(false);
            };
            if value.constructor() != *constructor {
                return Ok(false);
            }
            let descriptor = plan.custom_constructor(*constructor);
            for (index, pattern) in fields.iter().enumerate() {
                let value = &value.fields()[index];
                let expected = descriptor.fields()[index].type_();
                if plan.value_type(expected) != value.value_type(plan.value_metadata()) {
                    return Err(InvariantError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: index,
                        expected: plan.value_type(expected),
                        actual: value.value_type(plan.value_metadata()),
                    });
                }
                if !matches(plan, lists, environment, pattern, value, bindings)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        MatchPattern::StringPrefix {
            prefix,
            left,
            right,
        } => {
            let EvaluatedValue::String(value) = value else {
                return Ok(false);
            };
            let Some(suffix) = value.strip_prefix(prefix.as_str()) else {
                return Ok(false);
            };
            if let Some(binding) = left {
                bindings.bind(binding, EvaluatedValue::String(prefix.clone()));
            }
            if let Some(binding) = right {
                bindings.bind(binding, EvaluatedValue::String(suffix.into()));
            }
            Ok(true)
        }
        MatchPattern::Alias { pattern, binding } => {
            if !matches(plan, lists, environment, pattern, value, bindings)? {
                return Ok(false);
            }
            bindings.bind(binding, value.clone());
            Ok(true)
        }
    }
}

fn match_bit_array(
    environment: &BlockEnvironment,
    subject: &EvaluatedBitArray,
    pattern: &BitArrayPattern,
    bindings: &mut MatchBindings,
) -> bool {
    let mut cursor = 0;
    for segment in pattern.segments() {
        let matched = match segment {
            BitArrayPatternSegment::Int {
                pattern,
                size,
                endianness,
                signedness,
            } => {
                let Some(bit_size) = evaluate_size(environment, bindings, size) else {
                    return false;
                };
                let Some(bits) = bit_array::take_bits(subject.bits(), &mut cursor, bit_size) else {
                    return false;
                };
                let value = bit_array::decode_integer(bits, *endianness, *signedness);
                match_int(pattern, &value, bindings)
            }
            BitArrayPatternSegment::Float {
                pattern,
                size,
                endianness,
            } => {
                let Some(bit_size) = evaluate_size(environment, bindings, size) else {
                    return false;
                };
                let width = match bit_size {
                    16 => crate::plan::execution::graph::FloatBitSize::Sixteen,
                    32 => crate::plan::execution::graph::FloatBitSize::ThirtyTwo,
                    64 => crate::plan::execution::graph::FloatBitSize::SixtyFour,
                    _ => return false,
                };
                let Some(bits) = bit_array::take_bits(subject.bits(), &mut cursor, bit_size) else {
                    return false;
                };
                let value = bit_array::decode_float(bits, width, *endianness);
                match_float(pattern, value, bindings)
            }
            BitArrayPatternSegment::Bits {
                pattern,
                size,
                unit,
            } => {
                let bit_size = match size {
                    Some(size) => {
                        let Some(size) = evaluate_size(environment, bindings, size) else {
                            return false;
                        };
                        size
                    }
                    None => {
                        let remaining = subject.bits().len() - cursor;
                        if !remaining.is_multiple_of(usize::from(*unit)) {
                            return false;
                        }
                        remaining
                    }
                };
                let Some(bits) = bit_array::take_bits(subject.bits(), &mut cursor, bit_size) else {
                    return false;
                };
                let value = EvaluatedBitArray::new(BitVec::from_bitslice(bits));
                bind_bit_array(pattern, &value, bindings);
                true
            }
            BitArrayPatternSegment::String { pattern, encoding } => match pattern {
                BitArrayStringPattern::Literal(literal) => {
                    let encoded = bit_array::encode_string(literal, *encoding);
                    let Some(bits) =
                        bit_array::take_bits(subject.bits(), &mut cursor, encoded.len())
                    else {
                        return false;
                    };
                    bits == encoded.as_bitslice()
                }
                BitArrayStringPattern::Discard => {
                    let Some((_, bit_size)) =
                        bit_array::decode_codepoint(&subject.bits()[cursor..], *encoding)
                    else {
                        return false;
                    };
                    cursor += bit_size;
                    true
                }
            },
            BitArrayPatternSegment::UtfCodepoint { pattern, encoding } => {
                let Some((value, bit_size)) =
                    bit_array::decode_codepoint(&subject.bits()[cursor..], *encoding)
                else {
                    return false;
                };
                cursor += bit_size;
                bind_utf_codepoint(pattern, value, bindings);
                true
            }
        };
        if !matched {
            return false;
        }
    }
    cursor == subject.bits().len()
}

fn evaluate_size(
    environment: &BlockEnvironment,
    bindings: &MatchBindings,
    size: &BitArrayPatternSize,
) -> Option<usize> {
    let value = evaluate_size_expression(environment, bindings, size.value());
    let Ok(value) = usize::try_from(value) else {
        return None;
    };
    if value == 0 {
        return None;
    }
    value.checked_mul(usize::from(size.unit()))
}

fn evaluate_size_expression(
    environment: &BlockEnvironment,
    bindings: &MatchBindings,
    expression: &BitArrayPatternSizeExpr,
) -> BigInt {
    match expression {
        BitArrayPatternSizeExpr::Value(value) => value.clone(),
        BitArrayPatternSizeExpr::Local(local) => environment.int(*local),
        BitArrayPatternSizeExpr::Binding(binding) => bindings.int(*binding),
        BitArrayPatternSizeExpr::Add { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                + evaluate_size_expression(environment, bindings, right)
        }
        BitArrayPatternSizeExpr::Subtract { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                - evaluate_size_expression(environment, bindings, right)
        }
        BitArrayPatternSizeExpr::Multiply { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                * evaluate_size_expression(environment, bindings, right)
        }
        BitArrayPatternSizeExpr::Divide { left, right } => {
            let right = evaluate_size_expression(environment, bindings, right);
            if right == BigInt::from(0) {
                BigInt::from(0)
            } else {
                evaluate_size_expression(environment, bindings, left) / right
            }
        }
        BitArrayPatternSizeExpr::Remainder { left, right } => {
            let right = evaluate_size_expression(environment, bindings, right);
            if right == BigInt::from(0) {
                BigInt::from(0)
            } else {
                evaluate_size_expression(environment, bindings, left) % right
            }
        }
    }
}

fn match_int(
    pattern: &BitArrayPatternValue<BigInt>,
    value: &BigInt,
    bindings: &mut MatchBindings,
) -> bool {
    match pattern {
        BitArrayPatternValue::Literal(expected) => expected == value,
        BitArrayPatternValue::Bind(binding) => {
            bindings.bind_int(binding, value);
            true
        }
        BitArrayPatternValue::Discard => true,
        BitArrayPatternValue::Alias { pattern, binding } => {
            if !match_int(pattern, value, bindings) {
                return false;
            }
            bindings.bind_int(binding, value);
            true
        }
    }
}

fn match_float(
    pattern: &BitArrayPatternValue<f64>,
    value: f64,
    bindings: &mut MatchBindings,
) -> bool {
    match pattern {
        BitArrayPatternValue::Literal(expected) => *expected == value,
        BitArrayPatternValue::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::Float(value));
            true
        }
        BitArrayPatternValue::Discard => true,
        BitArrayPatternValue::Alias { pattern, binding } => {
            if !match_float(pattern, value, bindings) {
                return false;
            }
            bindings.bind(binding, EvaluatedValue::Float(value));
            true
        }
    }
}

fn bind_bit_array(
    pattern: &BitArrayBindingPattern,
    value: &EvaluatedBitArray,
    bindings: &mut MatchBindings,
) {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::BitArray(value.clone()));
        }
        BitArrayBindingPattern::Discard => {}
        BitArrayBindingPattern::Alias { pattern, binding } => {
            bind_bit_array(pattern, value, bindings);
            bindings.bind(binding, EvaluatedValue::BitArray(value.clone()));
        }
    }
}

fn bind_utf_codepoint(pattern: &BitArrayBindingPattern, value: char, bindings: &mut MatchBindings) {
    match pattern {
        BitArrayBindingPattern::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::UtfCodepoint(value));
        }
        BitArrayBindingPattern::Discard => {}
        BitArrayBindingPattern::Alias { pattern, binding } => {
            bind_utf_codepoint(pattern, value, bindings);
            bindings.bind(binding, EvaluatedValue::UtfCodepoint(value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::environment::{BlockEnvironment, RetainedValues};
    use super::{MatchPattern, match_pattern};
    use crate::plan::ValueType;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::{CoreRuntimeFunctionId, RuntimeFunctionId};
    use crate::plan::execution::graph::Terminator;
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::{CustomListAllocation, ListValueId, ParameterListValueId};
    use crate::runtime::{InvariantError, Value};

    #[test]
    fn recursive_matcher_executes_every_supported_pattern_family() {
        assert_eq!(
            crate::runtime::run_src(
                r#"pub type Payload {
  Payload(Int, BitArray, String, fn(Int) -> Int)
  Empty
}

fn add(captured: Int) {
  fn(value) { captured + value }
}

fn final_literal(value: Int) {
  let assert 42 = value
}

pub fn main() {
  let assert 1 as one = 1
  let assert 1.5 = 1.5
  let assert "ready" = "ready"
  let assert Nil = Nil

  let function = add(10)
  let subject = #(
    [1],
    <<2>>,
    Payload(3, <<4>>, "prefix", function),
  )
  let assert #(
    [first],
    <<second>>,
    Payload(third, <<fourth>>, "pre" <> suffix, nested_function) as payload,
  ) as whole = subject
  let assert #(
    [whole_first],
    <<whole_second>>,
    Payload(whole_third, _, _, whole_function),
  ) = whole
  let assert Payload(payload_number, _, _, payload_function) = payload

  let captured = 5
  let message = "unused"
  let closure = fn(value) {
    let assert #(captured_value, [item]) = #(captured, [value]) as message
    captured_value + item
  }

  #(
    first,
    one,
    second,
    third,
    fourth,
    suffix,
    nested_function(1),
    whole_first + whole_second + whole_third + whole_function(1),
    payload_number + payload_function(1),
    closure(6),
    final_literal(42),
  )
}

// @geam:expect Tuple([Int(1), Int(1), Int(2), Int(3), Int(4), String("fix"), Int(11), Int(17), Int(14), Int(11), Int(42)])
"#
            ),
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Int(1.into()),
                Value::Int(2.into()),
                Value::Int(3.into()),
                Value::Int(4.into()),
                Value::String("fix".into()),
                Value::Int(11.into()),
                Value::Int(17.into()),
                Value::Int(14.into()),
                Value::Int(11.into()),
                Value::Int(42.into()),
            ]),
        );
    }

    #[test]
    fn recursive_matcher_preserves_aliases_across_literal_and_prefix_patterns() {
        assert_eq!(
            crate::runtime::run_src(
                r#"pub fn main() {
  let bool_literal = case True {
    True as alias -> alias
    False -> False
  }

  let bool_variable = case True {
    value as alias -> value && alias
  }

  let string_variable = case "one" {
    value as alias -> value <> alias
  }

  let string_literal = case "one" {
    "one" as alias -> alias
    _ -> ""
  }

  let float_literal = case 1.5 {
    1.5 as alias -> alias +. 0.5
    _ -> 0.0
  }

  let float_variable = case 1.5 {
    value as alias -> value +. alias
  }

  bool_literal
  && bool_variable
  && string_variable == "oneone"
  && string_literal == "one"
  && float_literal == 2.0
  && float_variable == 3.0
}

// @geam:expect Bool(true)
"#
            ),
            Value::Bool(true),
        );
    }

    #[test]
    fn recursive_matcher_discards_a_string_prefix_suffix() {
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub fn main() {
  let assert "pre" <> _ = "prefix"
  True
}
"#,
            ),
            Value::Bool(true),
        );
    }

    #[test]
    fn recursive_matcher_exports_list_tails() {
        assert_eq!(
            crate::runtime::run_src(
                r#"pub fn main() {
  let assert [first, ..rest] = [1, 2]
  first == 1 && rest == [2]
}

// @geam:expect Bool(true)
"#
            ),
            Value::Bool(true),
        );
    }

    #[test]
    fn recursive_matcher_accepts_an_unresolved_empty_list() {
        let source = r#"
fn empty() -> List(value) {
  []
}

pub fn main() {
  let values = empty()
  let assert [] = values
  True
}
"#;

        assert_eq!(crate::runtime::run_src(source), Value::Bool(true));
    }

    #[test]
    fn recursive_matcher_rejects_a_nonempty_pattern_for_an_unresolved_empty_list() {
        let source = r#"
fn empty() -> List(value) {
  []
}

pub fn main() {
  let _ = empty()
  let assert [value] = [1]
  value
}
"#;
        let plan = execution_plan(source);
        let subject = EvaluatedValue::ParameterList(ParameterListValueId::new(
            plan.parameter_list_function_id(0).type_id(),
        ));

        assert_plan_pattern_miss(&plan, subject);
    }

    #[test]
    fn recursive_matcher_binds_an_unresolved_empty_list_tail() {
        let source = r#"
pub type Boxed(value) {
  Boxed(List(value))
  Empty
}

fn boxed() -> Boxed(value) {
  Boxed([])
}

pub fn main() {
  let assert Boxed([..tail]) = boxed()
  let assert [] = tail
  1
}
"#;

        assert_eq!(crate::runtime::run_src(source), Value::Int(1.into()));
    }

    #[test]
    fn source_matcher_handles_every_bit_array_binding_family_and_miss() {
        let source = r#"
pub fn main() {
  #(
    case <<1>> { <<value>> -> value _ -> 0 },
    case <<1>> { <<1 as alias>> -> alias _ -> 0 },
    case <<1>> { <<_>> -> True _ -> False },
    case <<1.5:float-size(16)>> {
      <<value:float-size(16)>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(16)>> {
      <<1.5 as alias:float-size(16)>> -> alias
      _ -> 0.0
    },
    case <<1.5:float-size(16)>> {
      <<_:float-size(16)>> -> True
      _ -> False
    },
    case <<1, 2>> {
      <<first:bytes-size(1), _ as rest:bits>> -> #(first, rest)
      _ -> #(<<>>, <<>>)
    },
    case <<"A":utf16-big>> { <<_:utf16-big>> -> True _ -> False },
    case <<"A":utf8>> { <<"A":utf8>> -> True _ -> False },
    case <<255>> { <<_:utf8>> -> True _ -> False },
    case <<65>> { <<value:utf8_codepoint>> -> value _ -> panic },
    case <<65>> { <<_ as alias:utf8_codepoint>> -> alias _ -> panic },
    case <<65>> { <<_:utf8_codepoint>> -> True _ -> False },
    case <<255>> { <<_:utf8_codepoint>> -> True _ -> False },
    case "prefix" { "pre" as left <> _ -> left == "pre" _ -> False },
    case "prefix" { "pre" <> _ -> True _ -> False },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Int(1.into()),
                Value::Bool(true),
                Value::Float(1.5),
                Value::Float(1.5),
                Value::Bool(true),
                Value::Tuple(vec![
                    Value::BitArray(crate::BitArrayValue::from_bytes(vec![1])),
                    Value::BitArray(crate::BitArrayValue::from_bytes(vec![2])),
                ]),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(false),
                Value::UtfCodepoint('A'),
                Value::UtfCodepoint('A'),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(true),
            ]),
        );
    }

    #[test]
    fn source_matcher_evaluates_every_size_operator_and_boundary() {
        assert_eq!(
            crate::runtime::run_src(
                r#"const base_pattern_size = 8
const pattern_size = base_pattern_size

pub fn main() {
  let outer_size = 12
  let negative_size = -1
  let zero_size = 0
  let huge_size = 184467440737095516160

  #(
    case <<-2:size(12)>> {
      <<value:signed-size(12)>> -> value
      _ -> 0
    },
    case <<-2:size(12)>> {
      <<value:unsigned-size(12)>> -> value
      _ -> 0
    },
    case <<0x234:little-size(12)>> {
      <<value:little-size(12)>> -> value
      _ -> 0
    },
    case <<0x234:size(12)>> {
      <<value:size(outer_size)>> -> value
      _ -> 0
    },
    case <<12, 0x234:size(12)>> {
      <<size, value:size(size)>> -> value
      _ -> 0
    },
    case <<>> {
      <<_:bits-size(negative_size)>> -> 1
      _ -> 0
    },
    case <<>> {
      <<_:bits-size(huge_size)>> -> 1
      _ -> 0
    },
    case <<1>> {
      <<_:bits-size(16)>> -> 1
      _ -> 0
    },
    case <<1>> {
      <<_:size(16)>> -> 1
      _ -> 0
    },
    case <<>> {
      <<value:size(zero_size)>> if value == 0 -> 1
      _ -> 0
    },
    case <<1, 2, 3, 4, 5>> {
      <<
        one:size(pattern_size),
        two:size(outer_size - 4),
        three:size(outer_size * 2 / 3),
        four:size(outer_size % 5 + 6),
        five:size({ outer_size - 4 }),
      >> -> one + two + three + four + five
      _ -> 0
    },
  )
}

// @geam:expect Tuple([Int(-2), Int(4094), Int(564), Int(564), Int(564), Int(0), Int(0), Int(0), Int(0), Int(0), Int(15)])
"#
            ),
            Value::Tuple(vec![
                Value::Int((-2).into()),
                Value::Int(4094.into()),
                Value::Int(564.into()),
                Value::Int(564.into()),
                Value::Int(564.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(15.into()),
            ]),
        );
        assert_eq!(
            crate::runtime::run_src(
                r#"
pub fn main() {
  let overflow = 9223372036854775808
  #(
    case <<>> { <<_:bits-size(1 / 0)>> -> 1 _ -> 0 },
    case <<>> { <<_:bits-size(1 % 0)>> -> 1 _ -> 0 },
    case <<>> { <<_:bits-size(overflow)-unit(2)>> -> 1 _ -> 0 },
  )
}
"#,
            ),
            Value::Tuple(vec![
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(0.into()),
            ]),
        );
    }

    #[test]
    fn recursive_matcher_keeps_wrong_root_families_and_nested_misses_refutable() {
        assert_pattern_miss(
            "pub fn main() { let assert 1.5 = 1.5 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert \"one\" = \"one\" 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert True = True 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert True = True 1 }",
            EvaluatedValue::Bool(false),
        );
        assert_pattern_miss(
            "pub fn main() { let assert #(1) = #(1) 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert #(1) = #(1) 1 }",
            EvaluatedValue::Tuple(Vec::new()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert #(1) = #(1) 1 }",
            EvaluatedValue::Tuple(vec![EvaluatedValue::Int(2.into())]),
        );
        assert_pattern_miss(
            "pub fn main() { let assert [1] = [1] 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_int_list_pattern_miss(
            "fn ints() -> List(Int) { [] } pub fn main() { let _ = ints() let assert [1, ..tail] = [1] let _ = tail 1 }",
            Vec::new(),
        );
        assert_int_list_pattern_miss(
            "fn ints() -> List(Int) { [] } pub fn main() { let _ = ints() let assert [1] = [1] 1 }",
            vec![1.into(), 2.into()],
        );
        assert_int_list_pattern_miss(
            "fn ints() -> List(Int) { [] } pub fn main() { let _ = ints() let assert [1] = [1] 1 }",
            vec![2.into()],
        );
        assert_pattern_miss(
            "pub fn main() { let assert <<1>> = <<1>> 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub type Boxed { Boxed(Int) Empty } fn boxed() { Boxed(1) } pub fn main() { let assert Boxed(1) = boxed() 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert \"pre\" <> _ = \"prefix\" 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert 1 as selected = 1 selected }",
            EvaluatedValue::Int(2.into()),
        );
        assert_pattern_miss(
            "pub fn main() { let assert #(1, Nil) = #(1, Nil) 1 }",
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int(1.into()),
                EvaluatedValue::Int(1.into()),
            ]),
        );
        assert_pattern_miss(
            "fn flag() { True } pub fn main() { let value = case flag() { True -> 1 False -> 2 } let assert 1 = value 1 }",
            EvaluatedValue::Int(2.into()),
        );
    }

    #[test]
    fn recursive_matcher_keeps_custom_and_string_prefix_misses_refutable() {
        let custom_source = "pub type Boxed { Boxed(Int) Empty } fn boxed(flag: Bool) -> Boxed { case flag { True -> Boxed(1) False -> Empty } } pub fn main() { let assert Boxed(1) = boxed(True) 1 }";
        let custom_plan = execution_plan(custom_source);
        let boxed = custom_plan.custom_constructor_id(0, 0);
        let empty = custom_plan.custom_constructor_id(0, 1);

        assert_plan_pattern_miss(
            &custom_plan,
            EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(empty, Box::new([]))),
        );
        assert_plan_pattern_miss(
            &custom_plan,
            EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                boxed,
                vec![EvaluatedValue::Int(2.into())].into_boxed_slice(),
            )),
        );

        assert_pattern_miss(
            "pub fn main() { let assert \"pre\" <> _ = \"prefix\" 1 }",
            EvaluatedValue::String("other".into()),
        );
    }

    #[test]
    fn recursive_matcher_exports_a_string_prefix_left_binding() {
        let plan = execution_plan(
            "pub fn main() { let assert \"pre\" as left <> _ = \"prefix\" let _ = left 1 }",
        );
        let pattern = main_pattern(&plan);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let bindings = match_pattern(
            &plan,
            state.lists_mut(),
            &environment,
            pattern,
            &EvaluatedValue::String("prefix".into()),
        )
        .expect("string-prefix matching should not be an execution error")
        .expect("the prefix should match");

        assert_eq!(bindings.value(0), EvaluatedValue::String("pre".into()));
    }

    #[test]
    fn recursive_matcher_exports_a_bool_alias_binding() {
        let plan = execution_plan(
            "pub fn main() { let assert True as selected = True let _ = selected 1 }",
        );
        let pattern = main_pattern(&plan);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let bindings = match_pattern(
            &plan,
            state.lists_mut(),
            &environment,
            pattern,
            &EvaluatedValue::Bool(true),
        )
        .expect("Bool matching should not be an execution error")
        .expect("the Bool pattern should match");

        assert_eq!(bindings.value(0), EvaluatedValue::Bool(true));
    }

    #[test]
    fn custom_pattern_reports_exact_field_family_corruption() {
        let plan = execution_plan(
            "pub type Boxed { Boxed(Int) Empty } fn boxed() { Boxed(1) } pub fn main() { let assert Boxed(value) = boxed() value }",
        );
        let pattern = main_pattern(&plan);
        let constructor = custom_pattern_constructor(pattern);
        let descriptor = plan.custom_constructor(constructor);
        let subject = EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
        ));
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let error = exact_match_error(match_pattern(
            &plan,
            state.lists_mut(),
            &environment,
            pattern,
            &subject,
        ));

        assert_eq!(
            error,
            InvariantError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            },
        );
    }

    #[test]
    fn nested_patterns_propagate_custom_field_corruption() {
        let tuple_plan = execution_plan(
            "pub type Boxed { Boxed(Int) Empty } fn boxed(flag: Bool) -> Boxed { case flag { True -> Boxed(1) False -> Empty } } pub fn main() { let assert #(Boxed(value)) = #(boxed(True)) value }",
        );
        let boxed_constructor = tuple_plan.custom_constructor_id(0, 0);
        let corrupted_boxed = EvaluatedCustomValue::from_fields(
            boxed_constructor,
            vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
        );
        let mut tuple_echo = Vec::new();
        let mut tuple_state = RuntimeState::new(&mut tuple_echo);
        assert_custom_field_corruption(
            &tuple_plan,
            &mut tuple_state,
            main_pattern(&tuple_plan),
            EvaluatedValue::Tuple(vec![EvaluatedValue::Custom(corrupted_boxed.clone())]),
            boxed_constructor,
        );

        let list_plan = execution_plan(
            "pub type Boxed { Boxed(Int) Empty } fn boxed(flag: Bool) -> Boxed { case flag { True -> Boxed(1) False -> Empty } } fn boxes() -> List(Boxed) { [] } pub fn main() { let _ = boxes() let assert [Boxed(value)] = [boxed(True)] value }",
        );
        let boxed_constructor = list_plan.custom_constructor_id(0, 0);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let values = state.lists_mut().custom(CustomListAllocation::new(
            list_plan.custom_list_function_id(0).type_id(),
            vec![corrupted_boxed],
        ));
        assert_custom_field_corruption(
            &list_plan,
            &mut state,
            main_pattern(&list_plan),
            EvaluatedValue::from(ListValueId::Custom(values)),
            boxed_constructor,
        );

        let custom_plan = execution_plan(
            "pub type Boxed { Boxed(Int) Empty } pub type Outer { Outer(Boxed) Other } fn outer(flag: Bool) -> Outer { case flag { True -> Outer(Boxed(1)) False -> Other } } pub fn main() { let assert Outer(Boxed(value)) as whole = outer(True) let _ = whole value }",
        );
        let pattern = main_pattern(&custom_plan);
        let (outer_constructor, boxed_constructor) = nested_custom_constructors(pattern);
        let corrupted_outer = EvaluatedCustomValue::from_fields(
            outer_constructor,
            vec![EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                boxed_constructor,
                vec![EvaluatedValue::String("wrong".into())].into_boxed_slice(),
            ))]
            .into_boxed_slice(),
        );
        let mut custom_echo = Vec::new();
        let mut custom_state = RuntimeState::new(&mut custom_echo);
        assert_custom_field_corruption(
            &custom_plan,
            &mut custom_state,
            pattern,
            EvaluatedValue::Custom(corrupted_outer),
            boxed_constructor,
        );
    }

    fn assert_pattern_miss(source: &str, subject: EvaluatedValue) {
        let plan = execution_plan(source);
        assert_plan_pattern_miss(&plan, subject);
    }

    fn assert_plan_pattern_miss(plan: &ExecutionPlan, subject: EvaluatedValue) {
        let pattern = main_pattern(plan);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let matched = match_pattern(plan, state.lists_mut(), &environment, pattern, &subject)
            .expect("refutable mismatch should not be an execution error");
        assert!(matched.is_none());
    }

    fn assert_int_list_pattern_miss(source: &str, values: Vec<num_bigint::BigInt>) {
        let plan = execution_plan(source);
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let list = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), values);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let matched = match_pattern(
            &plan,
            state.lists_mut(),
            &environment,
            main_pattern(&plan),
            &EvaluatedValue::List(list.into()),
        )
        .expect("list mismatch should not be an execution error");

        assert!(matched.is_none());
    }

    fn exact_match_error(
        result: Result<Option<super::MatchBindings>, InvariantError>,
    ) -> InvariantError {
        match result {
            Err(error) => error,
            Ok(_) => panic!("expected pattern matching to report an execution error"),
        }
    }

    #[test]
    #[should_panic(expected = "expected pattern matching to report an execution error")]
    fn exact_match_error_guard_rejects_success() {
        let _ = exact_match_error(Ok(None));
    }

    #[test]
    #[should_panic(expected = "fixture pattern should be a custom constructor")]
    fn custom_pattern_constructor_guard_rejects_other_patterns() {
        custom_pattern_constructor(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture pattern should contain nested custom constructors")]
    fn nested_custom_constructors_guard_rejects_other_patterns() {
        nested_custom_constructors(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture pattern should contain nested custom constructors")]
    fn nested_custom_constructors_guard_rejects_a_noncustom_alias_body() {
        let plan = execution_plan(
            "fn value(flag: Bool) { case flag { True -> 1 False -> 2 } } pub fn main() { let assert 1 as selected = value(True) selected }",
        );
        nested_custom_constructors(main_pattern(&plan));
    }

    #[test]
    #[should_panic(expected = "fixture pattern should contain nested custom constructors")]
    fn nested_custom_constructors_guard_rejects_a_noncustom_field() {
        let plan = execution_plan(
            "pub type Boxed { Boxed(Int) Empty } fn boxed(flag: Bool) -> Boxed { case flag { True -> Boxed(1) False -> Empty } } pub fn main() { let assert Boxed(1) as whole = boxed(True) let _ = whole 0 }",
        );
        nested_custom_constructors(main_pattern(&plan));
    }

    #[test]
    #[should_panic(expected = "fixture main should return Int")]
    fn main_pattern_guard_rejects_other_function_tables() {
        main_pattern(&execution_plan("pub fn main() { Nil }"));
    }

    fn assert_custom_field_corruption(
        plan: &ExecutionPlan,
        state: &mut RuntimeState,
        pattern: &MatchPattern,
        subject: EvaluatedValue,
        constructor: crate::plan::execution::type_::CustomConstructorId,
    ) {
        let descriptor = plan.custom_constructor(constructor);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());
        assert_eq!(
            exact_match_error(match_pattern(
                plan,
                state.lists_mut(),
                &environment,
                pattern,
                &subject,
            )),
            InvariantError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            },
        );
    }

    fn custom_pattern_constructor(
        pattern: &MatchPattern,
    ) -> crate::plan::execution::type_::CustomConstructorId {
        match pattern {
            MatchPattern::Custom { constructor, .. } => *constructor,
            _ => panic!("fixture pattern should be a custom constructor"),
        }
    }

    fn nested_custom_constructors(
        pattern: &MatchPattern,
    ) -> (
        crate::plan::execution::type_::CustomConstructorId,
        crate::plan::execution::type_::CustomConstructorId,
    ) {
        match pattern {
            MatchPattern::Alias { pattern, .. } => match pattern.as_ref() {
                MatchPattern::Custom {
                    constructor: outer,
                    fields,
                } => match &fields[0] {
                    MatchPattern::Custom {
                        constructor: inner, ..
                    } => (*outer, *inner),
                    _ => panic!("fixture pattern should contain nested custom constructors"),
                },
                _ => panic!("fixture pattern should contain nested custom constructors"),
            },
            _ => panic!("fixture pattern should contain nested custom constructors"),
        }
    }

    fn main_pattern(plan: &ExecutionPlan) -> &MatchPattern {
        let main = match plan.main_runtime() {
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(id)) => id,
            _ => panic!("fixture main should return Int"),
        };
        plan.int_function(main)
            .body()
            .block_graph()
            .blocks()
            .iter()
            .find_map(|block| {
                if let Terminator::Match(matcher) = block.terminator() {
                    Some(matcher.pattern())
                } else {
                    None
                }
            })
            .expect("fixture graph should contain a match terminator")
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module)
    }
}
