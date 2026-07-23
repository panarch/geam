use super::super::super::{GraphExitId, NeverCallTarget, SourceStopKind, Terminator};
use super::super::label::FunctionLabel;
use super::super::pattern::write_pattern;
use super::super::value::{ExplainLocal, write_locals};
use super::edge::{write_edge, write_match_edge};

pub(super) fn write_terminator(
    output: &mut String,
    terminator: &Terminator,
    write_exit: &mut dyn FnMut(&mut String, GraphExitId),
) {
    match terminator {
        Terminator::Jump(edge) => {
            output.push_str("jump ");
            write_edge(output, edge);
        }
        Terminator::BoolBranch {
            subject,
            true_,
            false_,
        } => {
            output.push_str("branch ");
            subject.write_local(output);
            output.push_str(" true=");
            write_edge(output, true_);
            output.push_str(" false=");
            write_edge(output, false_);
        }
        Terminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.int ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&pattern.to_string());
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.float ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.string ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::Match {
            subject,
            pattern,
            success,
            failure,
        } => {
            output.push_str("match ");
            subject.write_local(output);
            output.push_str(" pattern=");
            write_pattern(output, pattern);
            output.push_str(" success=");
            write_match_edge(output, success);
            output.push_str(" failure=");
            write_edge(output, failure);
        }
        Terminator::Exit(exit) => write_exit(output, *exit),
        Terminator::SourceStop { kind, message, .. } => {
            output.push_str("source_stop kind=");
            output.push_str(source_stop_kind(*kind));
            output.push_str(" message=");
            match message {
                Some(message) => message.write_local(output),
                None => output.push_str("none"),
            }
        }
        Terminator::LetAssertPanic {
            subject, message, ..
        } => {
            output.push_str("let_assert_panic subject=");
            subject.write_local(output);
            output.push_str(" message=");
            match message {
                Some(message) => message.write_local(output),
                None => output.push_str("none"),
            }
        }
        Terminator::NeverCall { function, args } => {
            output.push_str("never_call ");
            match function {
                NeverCallTarget::Direct(function) => {
                    FunctionLabel::new("never", function.0).push_to(output);
                }
                NeverCallTarget::Value(function) => function.write_local(output),
            }
            output.push_str(" args=");
            write_locals(output, args);
        }
    }
}

fn source_stop_kind(kind: SourceStopKind) -> &'static str {
    match kind {
        SourceStopKind::Panic => "panic",
        SourceStopKind::Todo => "todo",
        SourceStopKind::Assert => "assert",
        SourceStopKind::EmptyFunction => "empty_function",
        SourceStopKind::EmptyBlock => "empty_block",
        SourceStopKind::IncompleteUse => "incomplete_use",
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{GraphExitId, IntFunctionId};

    #[test]
    fn writes_branch_and_switch_terminators() {
        let cases = [
            (
                r#"
fn choose(value: Bool) { case value { True -> 1 False -> 0 } }
pub fn main() { choose(True) }
"#,
                "branch %bool#0 true=b1() false=b2()",
            ),
            (
                r#"
fn choose(value: Int) { case value { 1 -> 1 _ -> 0 } }
pub fn main() { choose(1) }
"#,
                "switch.int %int#0 clauses=[1->b1()] fallback=b2()",
            ),
            (
                r#"
fn choose(value: Float) { case value { 1.0 -> 1 _ -> 0 } }
pub fn main() { choose(1.0) }
"#,
                "switch.float %float#0 clauses=[1.0->b1()] fallback=b2()",
            ),
            (
                r#"
fn choose(value: String) { case value { "one" -> 1 _ -> 0 } }
pub fn main() { choose("one") }
"#,
                "switch.string %string#0 clauses=[\"one\"->b1()] fallback=b2()",
            ),
        ];

        for (source, expected) in cases {
            let plan = execution_plan(source);
            let terminator = plan.int_function(IntFunctionId(1)).graph().blocks()[0].terminator();
            let mut output = String::new();
            super::write_terminator(&mut output, terminator, &mut write_exit_id);

            assert_eq!(output, expected);
        }
    }

    #[test]
    fn writes_match_and_let_assert_panic_terminators() {
        let source = r#"
fn head(values: List(Int)) {
  let assert [value, ..] = values
  value
}

pub fn main() { head([1]) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let graph = plan.int_function(IntFunctionId(1)).graph();
        let mut output = String::new();
        let mut write_exit = write_exit_id;

        super::write_terminator(&mut output, graph.blocks()[0].terminator(), &mut write_exit);
        output.push_str(" | ");
        super::write_terminator(&mut output, graph.blocks()[2].terminator(), &mut write_exit);

        assert_eq!(
            output,
            "match %list.int#0 pattern=[binding#0, .._] success=b1(binding#0) failure=b2(%list.int#0) | let_assert_panic subject=%list.int#0 message=none",
        );
    }

    #[test]
    fn writes_jump_exit_source_stop_and_never_call_terminators() {
        let source = r#"
fn stopped() -> Int { panic as "stopped" }
fn stop() -> value { panic }
fn indirect_never() -> Int {
  let function = stop
  let _ = function()
  1
}
fn direct_never() -> Int {
  let _ = stop()
  1
}
fn join(flag: Bool) -> Int {
  let selected = case flag { True -> 1 False -> 2 }
  selected + 3
}

pub fn main() {
  let _ = #(stopped, indirect_never, direct_never)
  join(True)
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();
        let mut write_exit = write_exit_id;

        for (function, block) in [(1, 0), (2, 0), (3, 0), (4, 1), (4, 2)] {
            if !output.is_empty() {
                output.push_str(" | ");
            }
            let terminator =
                plan.int_function(IntFunctionId(function)).graph().blocks()[block].terminator();
            super::write_terminator(&mut output, terminator, &mut write_exit);
        }

        assert_eq!(
            output,
            concat!(
                "source_stop kind=panic message=%string#0 | ",
                "never_call %function.never#0 args=[] | ",
                "never_call never#0 args=[] | ",
                "jump b2(%int#0) | exit#0",
            ),
        );
    }

    #[test]
    fn writes_every_source_stop_kind_token() {
        let cases = [
            (super::SourceStopKind::Panic, "panic"),
            (super::SourceStopKind::Todo, "todo"),
            (super::SourceStopKind::Assert, "assert"),
            (super::SourceStopKind::EmptyFunction, "empty_function"),
            (super::SourceStopKind::EmptyBlock, "empty_block"),
            (super::SourceStopKind::IncompleteUse, "incomplete_use"),
        ];

        for (kind, expected) in cases {
            assert_eq!(super::source_stop_kind(kind), expected);
        }
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }

    fn write_exit_id(output: &mut String, exit: GraphExitId) {
        output.push_str("exit#");
        output.push_str(&exit.index().to_string());
    }
}
