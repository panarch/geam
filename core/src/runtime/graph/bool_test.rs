use super::{BlockEnvironment, RuntimeGraphState, integer};
use crate::plan::execution::graph::BoolTest;
use crate::runtime::evaluated::value_refs_equal;

pub(super) fn evaluate<State: RuntimeGraphState>(
    state: &State,
    environment: &BlockEnvironment,
    test: &BoolTest,
) -> bool {
    use BoolTest as I;
    match test {
        I::Not(value) => !environment.bool(*value),
        I::EqualInt { left, right } => integer::equal(environment, *left, *right),
        I::NotEqualInt { left, right } => !integer::equal(environment, *left, *right),
        I::LtInt { left, right } => integer::compare(environment, *left, *right).is_lt(),
        I::LtEqInt { left, right } => integer::compare(environment, *left, *right).is_le(),
        I::GtInt { left, right } => integer::compare(environment, *left, *right).is_gt(),
        I::GtEqInt { left, right } => integer::compare(environment, *left, *right).is_ge(),
        I::LtFloat { left, right } => environment.float(*left) < environment.float(*right),
        I::LtEqFloat { left, right } => environment.float(*left) <= environment.float(*right),
        I::GtFloat { left, right } => environment.float(*left) > environment.float(*right),
        I::GtEqFloat { left, right } => environment.float(*left) >= environment.float(*right),
        I::Equal { left, right } => value_refs_equal(
            state.lists(),
            &environment.value_ref(left),
            &environment.value_ref(right),
        ),
        I::NotEqual { left, right } => !value_refs_equal(
            state.lists(),
            &environment.value_ref(left),
            &environment.value_ref(right),
        ),
        I::StringStartsWith { value, prefix } => {
            environment.string(*value).starts_with(prefix.as_str())
        }
        I::ListLengthEquals { value, length } => environment.list_len(value) == *length,
        I::ListLengthAtLeast { value, length } => environment.list_len(value) >= *length,
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::runtime::{plan_src, run_main};

    #[test]
    fn integer_value_and_direct_branches_preserve_all_comparisons() {
        let source = r#"
fn checks(left: Int, right: Int) {
  #(
    left == right, case left == right { True -> True False -> False },
    left != right, case left != right { True -> True False -> False },
    left < right, case left < right { True -> True False -> False },
    left <= right, case left <= right { True -> True False -> False },
    left > right, case left > right { True -> True False -> False },
    left >= right, case left >= right { True -> True False -> False }
  )
}
pub fn main() { #(checks(-9223372036854775809, 9223372036854775808), checks(9223372036854775808, -9223372036854775809), checks(-1, -1)) }
"#;
        let expected = [
            [false, true, true, true, false, false],
            [false, true, false, false, true, true],
            [true, false, false, true, false, true],
        ];
        assert_eq!(
            run_main(&plan_src(source), &mut Vec::new()),
            Ok(Value::Tuple(
                expected
                    .into_iter()
                    .map(|values| Value::Tuple(
                        values
                            .into_iter()
                            .flat_map(|value| [Value::Bool(value), Value::Bool(value)])
                            .collect()
                    ))
                    .collect()
            ))
        );
    }

    #[test]
    fn float_value_and_direct_branches_preserve_all_comparisons() {
        let source = r#"
fn checks(left: Float, right: Float) {
  #(
    left <. right, case left <. right { True -> True False -> False },
    left <=. right, case left <=. right { True -> True False -> False },
    left >. right, case left >. right { True -> True False -> False },
    left >=. right, case left >=. right { True -> True False -> False }
  )
}
pub fn main() { #(checks(-1.5, 2.5), checks(2.5, -1.5), checks(-0.0, 0.0)) }
"#;
        let expected = [
            [true, true, false, false],
            [false, false, true, true],
            [false, true, false, true],
        ];
        assert_eq!(
            run_main(&plan_src(source), &mut Vec::new()),
            Ok(Value::Tuple(
                expected
                    .into_iter()
                    .map(|values| Value::Tuple(
                        values
                            .into_iter()
                            .flat_map(|value| [Value::Bool(value), Value::Bool(value)])
                            .collect()
                    ))
                    .collect()
            ))
        );
    }

    #[test]
    fn not_and_structural_equality_keep_values_and_direct_truth() {
        let source = r#"
pub type Box { Box(Int, List(String)) }
fn checks(left, right, flag) {
  #(left == right, case left == right { True -> True False -> False },
    left != right, case left != right { True -> True False -> False },
    !flag, case !flag { True -> True False -> False })
}
pub fn main() {
  #(checks(Box(4, ["x", "y"]), Box(4, ["x", "y"]), False),
    checks(Box(4, ["x", "y"]), Box(4, ["y", "x"]), True),
    checks(#([True], <<1, 2>>, Nil, 1.5), #([True], <<1, 2>>, Nil, 1.5), False))
}
"#;
        let expected = [
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ];
        assert_eq!(
            run_main(&plan_src(source), &mut Vec::new()),
            Ok(Value::Tuple(
                expected
                    .into_iter()
                    .map(|values| Value::Tuple(
                        values
                            .into_iter()
                            .flat_map(|value| [Value::Bool(value), Value::Bool(value)])
                            .collect()
                    ))
                    .collect()
            ))
        );
    }
}
