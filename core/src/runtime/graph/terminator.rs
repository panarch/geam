use super::environment::{BlockEnvironment, RetainedValues};
use super::pattern;
use crate::plan::execution::function::NeverFunctionId;
use crate::plan::execution::graph::{
    BlockGraphExitId, BlockId, Edge, MatchEdge, NeverCallTarget, SourceStopKind, Terminator,
};
use crate::runtime::ExecutionError;

use crate::runtime::error::PanicKind;
use crate::runtime::evaluated::EvaluatedNeverFunction;

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
    Value(EvaluatedNeverFunction),
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

pub(in crate::runtime) fn terminator_action<Plan, State>(
    plan: &Plan,
    state: &mut State,
    environment: BlockEnvironment,
    terminator: &Terminator,
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
            let subject = environment.int(switch.subject());
            let selected = switch
                .clauses()
                .iter()
                .find_map(|(pattern, edge)| pattern.matches(&subject).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => switch.fallback(),
            };
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
            let subject = environment.value(matcher.subject());
            let matched = pattern::match_pattern(
                plan,
                state.lists_mut(),
                &environment,
                matcher.pattern(),
                &subject,
            );
            drop(subject);
            matched
                .map_err(State::Error::from)
                .map(|matched| match matched {
                    Some(bindings) => transition_match(environment, matcher.success(), bindings),
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
                    NeverCall::Value(environment.never_function(function))
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

fn transition_match(
    environment: BlockEnvironment,
    edge: &MatchEdge,
    bindings: pattern::MatchBindings,
) -> GraphAction {
    let inputs = environment.into_match_retained(&edge.transfer, &edge.bindings, bindings);
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
