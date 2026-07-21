use bitvec::vec::BitVec;
use num_bigint::BigInt;
use std::collections::HashMap;

use super::bit_array;
use crate::plan::execution::{
    ExecutionPlan, GraphBitArrayBindingPattern, GraphBitArrayPattern, GraphBitArrayPatternSegment,
    GraphBitArrayPatternSize, GraphBitArrayPatternSizeExpr, GraphBitArrayPatternValue,
    GraphBitArrayStringPattern, MatchIntBindingId, MatchPattern, MatchPatternBinding,
    MatchPatternListTail,
};
use crate::runtime::environment::BlockEnvironment;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, InvariantError};

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

pub(super) fn match_pattern(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    pattern: &MatchPattern,
    subject: &EvaluatedValue,
) -> ExecutionResult<Option<MatchBindings>> {
    let mut bindings = MatchBindings::new();
    if matches(plan, state, environment, pattern, subject, &mut bindings)? {
        Ok(Some(bindings))
    } else {
        Ok(None)
    }
}

fn matches(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    pattern: &MatchPattern,
    value: &EvaluatedValue,
    bindings: &mut MatchBindings,
) -> ExecutionResult<bool> {
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
                if !matches(plan, state, environment, pattern, value, bindings)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        MatchPattern::List(pattern) => {
            let EvaluatedValue::List(value) = value else {
                return Ok(false);
            };
            let values = state.evaluated_values(value);
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
                if !matches(plan, state, environment, pattern, value, bindings)? {
                    return Ok(false);
                }
            }
            if let Some(MatchPatternListTail::Bind(binding)) = pattern.tail() {
                let tail = state.drop_first(value, element_count);
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
                if plan.value_type(expected) != value.value_type(plan) {
                    return Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: index,
                            expected: plan.value_type(expected),
                            actual: value.value_type(plan),
                        },
                    ));
                }
                if !matches(plan, state, environment, pattern, value, bindings)? {
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
            if !matches(plan, state, environment, pattern, value, bindings)? {
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
    pattern: &GraphBitArrayPattern,
    bindings: &mut MatchBindings,
) -> bool {
    let mut cursor = 0;
    for segment in pattern.segments() {
        let matched = match segment {
            GraphBitArrayPatternSegment::Int {
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
            GraphBitArrayPatternSegment::Float {
                pattern,
                size,
                endianness,
            } => {
                let Some(bit_size) = evaluate_size(environment, bindings, size) else {
                    return false;
                };
                let width = match bit_size {
                    16 => crate::plan::execution::FloatBitSize::Sixteen,
                    32 => crate::plan::execution::FloatBitSize::ThirtyTwo,
                    64 => crate::plan::execution::FloatBitSize::SixtyFour,
                    _ => return false,
                };
                let Some(bits) = bit_array::take_bits(subject.bits(), &mut cursor, bit_size) else {
                    return false;
                };
                let value = bit_array::decode_float(bits, width, *endianness);
                match_float(pattern, value, bindings)
            }
            GraphBitArrayPatternSegment::Bits {
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
            GraphBitArrayPatternSegment::String { pattern, encoding } => match pattern {
                GraphBitArrayStringPattern::Literal(literal) => {
                    let encoded = bit_array::encode_string(literal, *encoding);
                    let Some(bits) =
                        bit_array::take_bits(subject.bits(), &mut cursor, encoded.len())
                    else {
                        return false;
                    };
                    bits == encoded.as_bitslice()
                }
                GraphBitArrayStringPattern::Discard => {
                    let Some((_, bit_size)) =
                        bit_array::decode_codepoint(&subject.bits()[cursor..], *encoding)
                    else {
                        return false;
                    };
                    cursor += bit_size;
                    true
                }
            },
            GraphBitArrayPatternSegment::UtfCodepoint { pattern, encoding } => {
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
    size: &GraphBitArrayPatternSize,
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
    expression: &GraphBitArrayPatternSizeExpr,
) -> BigInt {
    match expression {
        GraphBitArrayPatternSizeExpr::Value(value) => value.clone(),
        GraphBitArrayPatternSizeExpr::Local(local) => environment.int(*local),
        GraphBitArrayPatternSizeExpr::Binding(binding) => bindings.int(*binding),
        GraphBitArrayPatternSizeExpr::Add { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                + evaluate_size_expression(environment, bindings, right)
        }
        GraphBitArrayPatternSizeExpr::Subtract { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                - evaluate_size_expression(environment, bindings, right)
        }
        GraphBitArrayPatternSizeExpr::Multiply { left, right } => {
            evaluate_size_expression(environment, bindings, left)
                * evaluate_size_expression(environment, bindings, right)
        }
        GraphBitArrayPatternSizeExpr::Divide { left, right } => {
            let right = evaluate_size_expression(environment, bindings, right);
            if right == BigInt::from(0) {
                BigInt::from(0)
            } else {
                evaluate_size_expression(environment, bindings, left) / right
            }
        }
        GraphBitArrayPatternSizeExpr::Remainder { left, right } => {
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
    pattern: &GraphBitArrayPatternValue<BigInt>,
    value: &BigInt,
    bindings: &mut MatchBindings,
) -> bool {
    match pattern {
        GraphBitArrayPatternValue::Literal(expected) => expected == value,
        GraphBitArrayPatternValue::Bind(binding) => {
            bindings.bind_int(binding, value);
            true
        }
        GraphBitArrayPatternValue::Discard => true,
        GraphBitArrayPatternValue::Alias { pattern, binding } => {
            if !match_int(pattern, value, bindings) {
                return false;
            }
            bindings.bind_int(binding, value);
            true
        }
    }
}

fn match_float(
    pattern: &GraphBitArrayPatternValue<f64>,
    value: f64,
    bindings: &mut MatchBindings,
) -> bool {
    match pattern {
        GraphBitArrayPatternValue::Literal(expected) => *expected == value,
        GraphBitArrayPatternValue::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::Float(value));
            true
        }
        GraphBitArrayPatternValue::Discard => true,
        GraphBitArrayPatternValue::Alias { pattern, binding } => {
            if !match_float(pattern, value, bindings) {
                return false;
            }
            bindings.bind(binding, EvaluatedValue::Float(value));
            true
        }
    }
}

fn bind_bit_array(
    pattern: &GraphBitArrayBindingPattern,
    value: &EvaluatedBitArray,
    bindings: &mut MatchBindings,
) {
    match pattern {
        GraphBitArrayBindingPattern::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::BitArray(value.clone()));
        }
        GraphBitArrayBindingPattern::Discard => {}
        GraphBitArrayBindingPattern::Alias { pattern, binding } => {
            bind_bit_array(pattern, value, bindings);
            bindings.bind(binding, EvaluatedValue::BitArray(value.clone()));
        }
    }
}

fn bind_utf_codepoint(
    pattern: &GraphBitArrayBindingPattern,
    value: char,
    bindings: &mut MatchBindings,
) {
    match pattern {
        GraphBitArrayBindingPattern::Bind(binding) => {
            bindings.bind(binding, EvaluatedValue::UtfCodepoint(value));
        }
        GraphBitArrayBindingPattern::Discard => {}
        GraphBitArrayBindingPattern::Alias { pattern, binding } => {
            bind_utf_codepoint(pattern, value, bindings);
            bindings.bind(binding, EvaluatedValue::UtfCodepoint(value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MatchPattern, match_pattern};
    use crate::plan::ValueType;
    use crate::plan::execution::{ExecutionPlan, RuntimeFunctionId, Terminator};
    use crate::runtime::environment::{BlockEnvironment, RetainedValues};
    use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};
    use crate::runtime::state::{CustomListAllocation, ListValueId, RuntimeState};
    use crate::runtime::{ExecutionError, InvariantError, Value};

    #[test]
    fn recursive_matcher_executes_every_supported_pattern_family() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/bindings/let_assert_pattern_families.gleam"
            )),
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
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/control_flow/case/pattern_alias_families.gleam"
            )),
            Value::Bool(true),
        );
    }

    #[test]
    fn recursive_matcher_exports_list_tails() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/bindings/let_assert_list_destructuring.gleam"
            )),
            Value::Bool(true),
        );
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
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/control_flow/case/bit_array_pattern_integers.gleam"
            )),
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
            "pub fn main() { let assert Nil = Nil 1 }",
            EvaluatedValue::Int(1.into()),
        );
        assert_pattern_miss(
            "fn flag() { True } pub fn main() { let value = case flag() { True -> 1 False -> 2 } let assert 1 = value 1 }",
            EvaluatedValue::Int(2.into()),
        );
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
        let mut state = RuntimeState::new();
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let error = exact_match_error(match_pattern(
            &plan,
            &mut state,
            &environment,
            pattern,
            &subject,
        ));

        assert_eq!(
            error,
            ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            },),
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
        let mut tuple_state = RuntimeState::new();
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
        let mut state = RuntimeState::new();
        let values = state.custom(CustomListAllocation::new(
            list_plan.custom_list_function_id(0).type_id(),
            vec![corrupted_boxed],
        ));
        assert_custom_field_corruption(
            &list_plan,
            &mut state,
            main_pattern(&list_plan),
            EvaluatedValue::List(ListValueId::Custom(values)),
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
        let mut custom_state = RuntimeState::new();
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
        let pattern = main_pattern(&plan);
        let mut state = RuntimeState::new();
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());

        let matched = match_pattern(&plan, &mut state, &environment, pattern, &subject)
            .expect("refutable mismatch should not be an execution error");
        assert!(matched.is_none());
    }

    fn exact_match_error(
        result: Result<Option<super::MatchBindings>, ExecutionError>,
    ) -> ExecutionError {
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
        constructor: crate::plan::execution::CustomConstructorId,
    ) {
        let descriptor = plan.custom_constructor(constructor);
        let environment = BlockEnvironment::from_retained(RetainedValues::empty());
        assert_eq!(
            exact_match_error(match_pattern(plan, state, &environment, pattern, &subject,)),
            ExecutionError::Invariant(InvariantError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                field_index: 0,
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
    }

    fn custom_pattern_constructor(
        pattern: &MatchPattern,
    ) -> crate::plan::execution::CustomConstructorId {
        match pattern {
            MatchPattern::Custom { constructor, .. } => *constructor,
            _ => panic!("fixture pattern should be a custom constructor"),
        }
    }

    fn nested_custom_constructors(
        pattern: &MatchPattern,
    ) -> (
        crate::plan::execution::CustomConstructorId,
        crate::plan::execution::CustomConstructorId,
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
            RuntimeFunctionId::Int(id) => id,
            _ => panic!("fixture main should return Int"),
        };
        plan.int_function(main)
            .graph()
            .blocks()
            .iter()
            .find_map(|block| {
                if let Terminator::Match { pattern, .. } = block.terminator() {
                    Some(pattern)
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
