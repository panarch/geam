use super::environment::{BlockEnvironment, ProfiledRetainedValues};
use super::pattern;
use crate::plan::execution::function::NeverFunctionId;
use crate::plan::execution::graph::{
    BlockGraphExitId, BlockId, Edge, MatchEdge, MatchEdgeArgument, NeverCallTarget, SourceStopKind,
    Terminator,
};
use crate::runtime::ExecutionError;
use crate::runtime::RuntimeValueProfile;
use crate::runtime::error::{PanicKind, PanicSubjectProfile};
use crate::runtime::evaluated::EvaluatedNeverFunction;
use crate::runtime::materialize::MaterializeProfile;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) enum GraphAction<Profile: RuntimeValueProfile> {
    Continue {
        block: BlockId,
        inputs: ProfiledRetainedValues<Profile>,
    },
    Exit(BlockGraphExitId),
    NeverCall {
        function: NeverCall<Profile>,
        inputs: ProfiledRetainedValues<Profile>,
        site: crate::plan::HostCallSite,
    },
}

pub(in crate::runtime) enum NeverCall<Profile: RuntimeValueProfile> {
    Direct(NeverFunctionId),
    Value(EvaluatedNeverFunction<Profile>),
}

pub(in crate::runtime) trait RuntimeGraphState<Profile: RuntimeValueProfile> {
    type Error: From<crate::runtime::InvariantError>;

    fn lists(&self) -> &Profile::ListStorage;
    fn lists_mut(&mut self) -> &mut Profile::ListStorage;
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
        subject: crate::runtime::EvaluatedValue<Profile>,
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

impl<Host, Values: PanicSubjectProfile> RuntimeGraphState<Values>
    for RuntimeState<'_, Host, Values>
{
    type Error = ExecutionError<Values::PanicSubject>;

    fn lists(&self) -> &Values::ListStorage {
        self.lists()
    }

    fn lists_mut(&mut self) -> &mut Values::ListStorage {
        self.lists_mut()
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
        subject: crate::runtime::EvaluatedValue<Values>,
        pattern_span: crate::plan::SourceSpan,
    ) -> Self::Error
    where
        Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    {
        let subject = Values::panic_subject(plan, self.lists(), subject);
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

pub(in crate::runtime) fn terminator_action<Plan, State, Profile>(
    plan: &Plan,
    state: &mut State,
    environment: &BlockEnvironment<Profile>,
    terminator: &Terminator,
) -> Result<GraphAction<Profile>, State::Error>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState<Profile>,
    Profile: MaterializeProfile,
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
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
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
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
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
                environment,
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
            let message = echo.message().map(|message| environment.string(message));
            let value =
                crate::runtime::materialize::value(plan.value_metadata(), state.lists(), subject);
            let location = crate::runtime::EchoLocation::from_context(
                echo.site().clone(),
                plan.source_context_for(echo.site().module()),
            );
            state.emit_echo(crate::runtime::EchoOutput::new(location, message, value));
            Ok(transition(environment, echo.next()))
        }
        Terminator::Exit(exit) => Ok(GraphAction::Exit(*exit)),
        Terminator::SourceStop(stop) => {
            let message = stop.message().map(|message| environment.string(message));
            Err(state.source_panic(
                plan.source_context_for(stop.site().module()),
                panic_kind(stop.kind()),
                message,
                stop.site().clone(),
            ))
        }
        Terminator::LetAssertPanic(panic) => {
            let subject = environment.value(panic.subject());
            let message = panic.message().map(|message| environment.string(message));
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
            let inputs = environment.retain(call.args());
            let function = match call.function() {
                NeverCallTarget::Direct(function) => NeverCall::Direct(*function),
                NeverCallTarget::Value(function) => {
                    NeverCall::Value(environment.never_function(function))
                }
            };
            Ok(GraphAction::NeverCall {
                function,
                inputs,
                site: call.site().clone(),
            })
        }
    }
}

fn transition<Profile: RuntimeValueProfile>(
    environment: &BlockEnvironment<Profile>,
    edge: &Edge,
) -> GraphAction<Profile> {
    GraphAction::Continue {
        block: edge.target(),
        inputs: environment.retain(edge.args()),
    }
}

fn transition_match<Profile: RuntimeValueProfile>(
    environment: &BlockEnvironment<Profile>,
    edge: &MatchEdge,
    bindings: pattern::MatchBindings<Profile>,
) -> GraphAction<Profile> {
    let mut inputs = ProfiledRetainedValues::empty();
    for argument in edge.args() {
        match argument {
            MatchEdgeArgument::Binding(index) => {
                inputs.push_evaluated(bindings.value(*index));
            }
            MatchEdgeArgument::Value(local) => {
                inputs.push_evaluated(environment.value(local));
            }
        }
    }
    drop(bindings);
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
