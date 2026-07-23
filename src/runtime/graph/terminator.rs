use super::environment::{BlockEnvironment, RetainedValues};
use super::pattern;
use crate::plan::execution::{
    BlockGraphExitId, BlockId, Edge, ExecutionPlan, MatchEdge, MatchEdgeArgument, NeverCallTarget,
    NeverFunctionId, SourceStopKind, Terminator,
};
use crate::runtime::ExecutionError;
use crate::runtime::error::{ExecutionResult, PanicKind};
use crate::runtime::evaluated::EvaluatedNeverFunction;
use crate::runtime::state::RuntimeState;

pub(super) enum GraphAction {
    Continue {
        block: BlockId,
        inputs: RetainedValues,
    },
    Exit(BlockGraphExitId),
    NeverCall {
        function: NeverCall,
        inputs: RetainedValues,
    },
}

pub(super) enum NeverCall {
    Direct(NeverFunctionId),
    Value(EvaluatedNeverFunction),
}

pub(super) fn terminator_action(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    environment: &BlockEnvironment,
    terminator: &Terminator,
) -> ExecutionResult<GraphAction> {
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
            let matched =
                pattern::match_pattern(plan, state, environment, matcher.pattern(), &subject);
            drop(subject);
            matched.map(|matched| match matched {
                Some(bindings) => transition_match(environment, matcher.success(), bindings),
                None => transition(environment, matcher.failure()),
            })
        }
        Terminator::Exit(exit) => Ok(GraphAction::Exit(*exit)),
        Terminator::SourceStop(stop) => {
            let message = stop.message().map(|message| environment.string(message));
            Err(ExecutionError::source_panic(
                plan.source_context(),
                panic_kind(stop.kind()),
                message,
                stop.site().clone(),
            ))
        }
        Terminator::LetAssertPanic(panic) => {
            let subject = environment.value(panic.subject());
            let message = panic.message().map(|message| environment.string(message));
            let subject = crate::runtime::materialize::value(plan, state, subject);
            Err(ExecutionError::let_assert_panic(
                plan.source_context(),
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
            Ok(GraphAction::NeverCall { function, inputs })
        }
    }
}

fn transition(environment: &BlockEnvironment, edge: &Edge) -> GraphAction {
    GraphAction::Continue {
        block: edge.target(),
        inputs: environment.retain(edge.args()),
    }
}

fn transition_match(
    environment: &BlockEnvironment,
    edge: &MatchEdge,
    bindings: pattern::MatchBindings,
) -> GraphAction {
    let mut inputs = RetainedValues::empty();
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
