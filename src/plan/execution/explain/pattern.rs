mod bit_array;

use self::bit_array::write_bit_array;
use super::super::graph::{
    MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail,
};

pub(super) fn write_pattern(output: &mut String, pattern: &MatchPattern) {
    match pattern {
        MatchPattern::Bind(binding) => write_binding(output, binding),
        MatchPattern::Discard => output.push('_'),
        MatchPattern::Int(value) => output.push_str(&value.to_string()),
        MatchPattern::Float(value) => output.push_str(&format!("{value:?}")),
        MatchPattern::String(value) => output.push_str(&format!("{value:?}")),
        MatchPattern::Bool(value) => output.push_str(if *value { "True" } else { "False" }),
        MatchPattern::Nil => output.push_str("Nil"),
        MatchPattern::Tuple(elements) => {
            output.push_str("#(");
            write_patterns(output, elements);
            output.push(')');
        }
        MatchPattern::List(list) => write_list(output, list),
        MatchPattern::BitArray(pattern) => write_bit_array(output, pattern),
        MatchPattern::Custom {
            constructor,
            fields,
        } => {
            output.push_str("custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
            output.push('(');
            write_patterns(output, fields);
            output.push(')');
        }
        MatchPattern::StringPrefix {
            prefix,
            left,
            right,
        } => {
            output.push_str("string_prefix(");
            output.push_str(&format!("{prefix:?}"));
            output.push_str(", left=");
            write_optional_binding(output, left.as_ref());
            output.push_str(", right=");
            write_optional_binding(output, right.as_ref());
            output.push(')');
        }
        MatchPattern::Alias { pattern, binding } => {
            output.push_str("alias(");
            write_pattern(output, pattern);
            output.push_str(", ");
            write_binding(output, binding);
            output.push(')');
        }
    }
}

fn write_patterns(output: &mut String, patterns: &[MatchPattern]) {
    for (index, pattern) in patterns.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_pattern(output, pattern);
    }
}

fn write_list(output: &mut String, list: &MatchPatternList) {
    output.push('[');
    let mut separator = "";
    for pattern in list.elements() {
        output.push_str(separator);
        write_pattern(output, pattern);
        separator = ", ";
    }
    if let Some(tail) = list.tail() {
        output.push_str(separator);
        output.push_str("..");
        match tail {
            MatchPatternListTail::Ignore => output.push('_'),
            MatchPatternListTail::Bind(binding) => write_binding(output, binding),
        }
    }
    output.push(']');
}

fn write_optional_binding(output: &mut String, binding: Option<&MatchPatternBinding>) {
    match binding {
        Some(binding) => write_binding(output, binding),
        None => output.push('_'),
    }
}

pub(super) fn write_binding(output: &mut String, binding: &MatchPatternBinding) {
    output.push_str("binding#");
    output.push_str(&binding.index().to_string());
}

#[cfg(test)]
mod tests {
    use super::super::super::{IntFunctionId, Terminator};
    use super::MatchPattern;

    #[test]
    fn writes_nested_patterns_from_a_lowered_match() {
        let source = r#"
pub type Payload { Payload(Int) }

fn select(value: #(List(Int), Payload, String)) {
  let assert #([1, ..rest], Payload(number), "pre" <> suffix) as whole = value
  number
}

pub fn main() { select(#([1, 2], Payload(3), "prefix")) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let terminator = plan.int_function(IntFunctionId(1)).graph().blocks()[0].terminator();
        let pattern = match_pattern(terminator);
        let mut output = String::new();

        super::write_pattern(&mut output, pattern);

        assert_eq!(
            output,
            "alias(#([1, ..binding#0], custom_type#0.constructor#0(binding#1), string_prefix(\"pre\", left=_, right=binding#2)), binding#3)",
        );
    }

    #[test]
    fn list_pattern_explanation_separates_elements_from_the_tail() {
        let source = r#"
fn bind_tail(values: List(Int)) {
  let assert [head, ..tail] = values
  head
}

fn ignore_tail(values: List(Int)) {
  let assert [head, ..] = values
  head
}

pub fn main() { bind_tail([1]) + ignore_tail([2]) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();

        let bind_tail = plan.int_function(IntFunctionId(1)).graph();
        let pattern = match_pattern(bind_tail.blocks()[0].terminator());
        super::write_pattern(&mut output, pattern);

        assert_eq!(output, "[binding#0, ..binding#1]");

        output.clear();
        let ignore_tail = plan.int_function(IntFunctionId(2)).graph();
        let pattern = match_pattern(ignore_tail.blocks()[0].terminator());
        super::write_pattern(&mut output, pattern);

        assert_eq!(output, "[binding#0, .._]");
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_pattern_shape_guard_is_visible() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let terminator = plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator();

        match_pattern(terminator);
    }

    fn match_pattern(terminator: &Terminator) -> &MatchPattern {
        let Terminator::Match { pattern, .. } = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        pattern
    }
}
