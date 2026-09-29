use super::environment::{BlockEnvironment, RetainedValues};
use super::pattern;
use crate::plan::execution::function::NeverFunctionId;
use crate::plan::execution::graph::{
    BlockGraphExitId, BlockId, Edge, IntSwitch, MatchEdge, NeverCallTarget, SourceStopKind,
    Terminator,
};
use crate::runtime::ExecutionError;

use crate::runtime::captures::Captures;
use crate::runtime::error::PanicKind;
use crate::runtime::evaluated::EvaluatedValue;

use crate::runtime::state::RuntimeState;

pub(in crate::runtime) enum GraphAction {
    Continue {
        block: BlockId,
        inputs: RetainedValues,
    },
    Exit {
        exit: BlockGraphExitId,
        environment: BlockEnvironment,
    },
    NeverCall {
        function: NeverCall,
        inputs: RetainedValues,
        site: crate::plan::HostCallSite,
    },
}

pub(in crate::runtime) enum NeverCall {
    Direct(NeverFunctionId),
    Value {
        function: NeverFunctionId,
        captures: Captures,
    },
}

pub(in crate::runtime) trait RuntimeGraphState {
    type Error: From<crate::runtime::InvariantError>;

    fn lists(&self) -> &crate::runtime::RuntimeListStorage;
    fn lists_mut(&mut self) -> &mut crate::runtime::RuntimeListStorage;
    fn captures(&self) -> &crate::runtime::CaptureStorage;
    fn emit_echo(&mut self, output: crate::runtime::EchoOutput);

    fn source_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        kind: PanicKind,
        message: Option<ecow::EcoString>,
        site: crate::plan::PanicSite,
    ) -> Self::Error;

    fn let_assert_panic<Plan>(
        &self,
        plan: &Plan,
        source: Option<&crate::plan::SourceContext>,
        message: Option<ecow::EcoString>,
        site: crate::plan::PanicSite,
        subject: crate::runtime::EvaluatedValue,
        pattern_span: crate::plan::SourceSpan,
    ) -> Self::Error
    where
        Plan: crate::plan::execution::runtime::RuntimeExecutionPlan;

    fn bit_array_segment_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        reason: crate::runtime::BitArraySegmentPanicReason,
        site: crate::plan::PanicSite,
    ) -> Self::Error;
}

impl<Host> RuntimeGraphState for RuntimeState<'_, Host> {
    type Error = ExecutionError<crate::PanicValue>;

    fn lists(&self) -> &crate::runtime::RuntimeListStorage {
        self.lists()
    }

    fn lists_mut(&mut self) -> &mut crate::runtime::RuntimeListStorage {
        self.lists_mut()
    }

    fn captures(&self) -> &crate::runtime::CaptureStorage {
        self.captures()
    }

    fn emit_echo(&mut self, output: crate::runtime::EchoOutput) {
        self.emit_echo(output);
    }

    fn source_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        kind: PanicKind,
        message: Option<ecow::EcoString>,
        site: crate::plan::PanicSite,
    ) -> Self::Error {
        ExecutionError::source_panic(source, kind, message, site)
    }

    fn let_assert_panic<Plan>(
        &self,
        plan: &Plan,
        source: Option<&crate::plan::SourceContext>,
        message: Option<ecow::EcoString>,
        site: crate::plan::PanicSite,
        subject: crate::runtime::EvaluatedValue,
        pattern_span: crate::plan::SourceSpan,
    ) -> Self::Error
    where
        Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    {
        let subject = crate::runtime::error::PanicValue::new(plan, self.lists(), subject);
        ExecutionError::let_assert_panic(source, message, site, subject, pattern_span)
    }

    fn bit_array_segment_panic(
        &self,
        source: Option<&crate::plan::SourceContext>,
        reason: crate::runtime::BitArraySegmentPanicReason,
        site: crate::plan::PanicSite,
    ) -> Self::Error {
        ExecutionError::bit_array_segment_panic(source, reason, site)
    }
}

// Keep block dispatch in the graph loop rather than passing a GraphAction at every transition.
#[inline]
pub(in crate::runtime) fn terminator_action<Plan, State>(
    plan: &Plan,
    state: &mut State,
    environment: BlockEnvironment,
    terminator: &Terminator,
    match_results: &mut Vec<EvaluatedValue>,
) -> Result<GraphAction, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
{
    match terminator {
        Terminator::Jump(jump) => Ok(transition(environment, jump.edge())),
        Terminator::BoolBranch(branch) => {
            let edge = if environment.bool(branch.subject()) {
                branch.true_()
            } else {
                branch.false_()
            };
            Ok(transition(environment, edge))
        }
        Terminator::IntSwitch(switch) => {
            let edge = select_int_edge(&environment, switch);
            Ok(transition(environment, edge))
        }
        Terminator::FloatSwitch(switch) => {
            let subject = environment.float(switch.subject());
            let selected = switch
                .clauses()
                .iter()
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => switch.fallback(),
            };
            Ok(transition(environment, edge))
        }
        Terminator::StringSwitch(switch) => {
            let subject = environment.string(switch.subject());
            let selected = switch
                .clauses()
                .iter()
                .find_map(|(pattern, edge)| (pattern.as_str() == subject.as_str()).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => switch.fallback(),
            };
            Ok(transition(environment, edge))
        }
        Terminator::Match(matcher) => {
            let matched = {
                let subject = environment.value_ref(matcher.subject());
                pattern::match_pattern(
                    plan,
                    state.lists(),
                    &environment,
                    matcher.pattern(),
                    &subject,
                    match_results,
                )
            };
            matched
                .map_err(State::Error::from)
                .map(|matched| match matched {
                    Some(bindings) => {
                        transition_match(environment, matcher.success(), bindings, match_results)
                    }
                    None => transition(environment, matcher.failure()),
                })
        }
        Terminator::Echo(echo) => {
            let subject = environment.value(echo.subject());
            let message = echo
                .message()
                .map(|message| environment.string(message).into_ecostring());
            let value =
                crate::runtime::materialize::value(plan.value_metadata(), state.lists(), subject);
            let location = crate::runtime::EchoLocation::from_context(
                echo.site().clone(),
                plan.source_context_for(echo.site().module()),
            );
            state.emit_echo(crate::runtime::EchoOutput::new(location, message, value));
            Ok(transition(environment, echo.next()))
        }
        Terminator::Exit(exit) => Ok(GraphAction::Exit {
            exit: *exit,
            environment,
        }),
        Terminator::SourceStop(stop) => {
            let message = stop
                .message()
                .map(|message| environment.string(message).into_ecostring());
            Err(state.source_panic(
                plan.source_context_for(stop.site().module()),
                panic_kind(stop.kind()),
                message,
                stop.site().clone(),
            ))
        }
        Terminator::LetAssertPanic(panic) => {
            let subject = environment.value(panic.subject());
            let message = panic
                .message()
                .map(|message| environment.string(message).into_ecostring());
            Err(state.let_assert_panic(
                plan,
                plan.source_context_for(panic.site().module()),
                message,
                panic.site().clone(),
                subject,
                *panic.pattern_span(),
            ))
        }
        Terminator::NeverCall(call) => {
            let function = match call.function() {
                NeverCallTarget::Direct(function) => NeverCall::Direct(*function),
                NeverCallTarget::Value(function) => {
                    let function = environment.never_function(function);
                    NeverCall::Value {
                        function: function.runtime_id(),
                        captures: function.capture_frame().clone(),
                    }
                }
            };
            let inputs = environment.into_retained(&call.transfer);
            Ok(GraphAction::NeverCall {
                function,
                inputs,
                site: call.site().clone(),
            })
        }
    }
}

fn transition(environment: BlockEnvironment, edge: &Edge) -> GraphAction {
    GraphAction::Continue {
        block: edge.target(),
        inputs: environment.into_retained(&edge.transfer),
    }
}

// Keep clause traversal outside the shared block dispatch.
#[inline(never)]
fn select_int_edge<'plan>(environment: &BlockEnvironment, switch: &'plan IntSwitch) -> &'plan Edge {
    let subject = environment.int_ref(switch.subject());
    let selected = switch
        .clauses()
        .iter()
        .find_map(|(pattern, edge)| pattern.matches(subject).then_some(edge));
    match selected {
        Some(edge) => edge,
        None => switch.fallback(),
    }
}

fn transition_match(
    environment: BlockEnvironment,
    edge: &MatchEdge,
    bindings: pattern::MatchBindings,
    match_results: &mut Vec<EvaluatedValue>,
) -> GraphAction {
    let inputs =
        environment.into_match_retained(&edge.transfer, &edge.bindings, bindings, match_results);
    GraphAction::Continue {
        block: edge.target(),
        inputs,
    }
}

fn panic_kind(kind: SourceStopKind) -> PanicKind {
    match kind {
        SourceStopKind::Panic => PanicKind::Panic,
        SourceStopKind::Todo => PanicKind::Todo,
        SourceStopKind::Assert => PanicKind::Assert,
        SourceStopKind::EmptyFunction => PanicKind::EmptyFunction,
        SourceStopKind::EmptyBlock => PanicKind::EmptyBlock,
        SourceStopKind::IncompleteUse => PanicKind::IncompleteUse,
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::{Value, run_src};
    use num_bigint::BigInt;

    #[test]
    fn integer_switches_preserve_digit_boundaries_and_values_on_the_selected_edge() {
        for (number, label) in [
            ("-18446744073709551617", "other"),
            ("-18446744073709551616", "negative wide"),
            ("-4294967296", "negative limb"),
            ("-1", "negative one"),
            ("0", "zero"),
            ("1", "one"),
            ("2", "other"),
            ("4294967295", "limb maximum"),
            ("4294967296", "next limb"),
            ("4294967297", "other"),
            ("18446744073709551615", "two limbs"),
            ("18446744073709551616", "wide"),
            ("1208925819614629174706176", "large"),
            ("1208925819614629174706177", "other"),
        ] {
            let source = format!(
                r#"
fn classify(value: Int) {{
  let label = case value {{
    0 -> "zero"
    1208925819614629174706176 -> "large"
    -1 -> "negative one"
    4294967296 -> "next limb"
    -18446744073709551616 -> "negative wide"
    1 -> "one"
    4294967295 -> "limb maximum"
    18446744073709551616 -> "wide"
    -4294967296 -> "negative limb"
    18446744073709551615 -> "two limbs"
    _ -> "other"
  }}
  #(value, label, value)
}}
pub fn main() {{ classify({number}) }}
"#,
            );
            let expected = number.parse::<BigInt>().unwrap();
            assert_eq!(
                run_src(&source),
                Value::Tuple(vec![
                    Value::Int(expected.clone()),
                    Value::String(label.into()),
                    Value::Int(expected),
                ]),
                "{number}",
            );
        }
    }

    #[test]
    fn integer_switches_preserve_first_matching_guarded_clause_and_fallback() {
        let source = r#"
fn select(value, enabled) {
  let selected = case value {
    0 if enabled -> 10
    0 -> 20
    4294967296 if enabled -> 30
    4294967296 -> 40
    _ -> 50
  }
  #(value, selected)
}
pub fn main() {
  #(
    select(0, True),
    select(0, False),
    select(4294967296, True),
    select(4294967296, False),
    select(-1, True),
  )
}
"#;
        assert_eq!(
            run_src(source),
            Value::Tuple(vec![
                Value::Tuple(vec![Value::Int(0.into()), Value::Int(10.into())]),
                Value::Tuple(vec![Value::Int(0.into()), Value::Int(20.into())]),
                Value::Tuple(vec![
                    Value::Int(4_294_967_296_u64.into()),
                    Value::Int(30.into())
                ]),
                Value::Tuple(vec![
                    Value::Int(4_294_967_296_u64.into()),
                    Value::Int(40.into())
                ]),
                Value::Tuple(vec![Value::Int((-1).into()), Value::Int(50.into())]),
            ]),
        );
    }
}
