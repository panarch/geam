use super::super::super::value::ExplainLocal;
use crate::plan::execution::{ParamLocal, StringLocalId};

pub(super) fn write_let_assert_panic(
    output: &mut String,
    subject: &ParamLocal,
    message: Option<&StringLocalId>,
) {
    output.push_str("let_assert_panic subject=");
    subject.write_local(output);
    output.push_str(" message=");
    match message {
        Some(message) => message.write_local(output),
        None => output.push_str("none"),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_let_assert_panic() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        let expected = "let_assert_panic subject=%list.int#0 message=none";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let (subject, message) = let_assert_panic(&terminators);
            super::write_let_assert_panic(output, subject, message.as_ref());
        });
    }

    fn let_assert_panic<'a>(
        terminators: &[&'a Terminator],
    ) -> (
        &'a crate::plan::execution::ParamLocal,
        Option<crate::plan::execution::StringLocalId>,
    ) {
        let mut panics = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::LetAssertPanic(panic) => Some((panic.subject(), panic.message())),
                _ => None,
            });
        let Some(panic) = panics.next() else {
            panic!("source should lower one let-assert panic terminator");
        };
        if panics.next().is_some() {
            panic!("source should lower one let-assert panic terminator");
        }
        panic
    }

    #[test]
    #[should_panic(expected = "source should lower one let-assert panic terminator")]
    fn let_assert_panic_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let_assert_panic(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one let-assert panic terminator")]
    fn let_assert_panic_uniqueness_guard_is_visible() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::LetAssertPanic(_)))
                .expect("source should lower a let-assert panic");
            let_assert_panic(&[terminator, terminator]);
        });
    }
}
