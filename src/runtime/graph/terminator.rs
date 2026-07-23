use super::environment::{BlockEnvironment, RetainedValues};
use super::pattern;
use crate::plan::execution::{
    BlockId, Edge, ExecutionPlan, GraphExitId, MatchEdge, MatchEdgeArgument, NeverCallTarget,
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
    Exit(GraphExitId),
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
        Terminator::Jump(edge) => Ok(transition(environment, edge)),
        Terminator::BoolBranch {
            subject,
            true_,
            false_,
        } => {
            let edge = if environment.bool(*subject) {
                true_
            } else {
                false_
            };
            Ok(transition(environment, edge))
        }
        Terminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => {
            let subject = environment.int(*subject);
            let selected = clauses
                .iter()
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => fallback,
            };
            Ok(transition(environment, edge))
        }
        Terminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => {
            let subject = environment.float(*subject);
            let selected = clauses
                .iter()
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => fallback,
            };
            Ok(transition(environment, edge))
        }
        Terminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => {
            let subject = environment.string(*subject);
            let selected = clauses
                .iter()
                .find_map(|(pattern, edge)| (pattern == &subject).then_some(edge));
            let edge = match selected {
                Some(edge) => edge,
                None => fallback,
            };
            Ok(transition(environment, edge))
        }
        Terminator::Match {
            subject,
            pattern: matcher,
            success,
            failure,
        } => {
            let subject = environment.value(subject);
            let matched = pattern::match_pattern(plan, state, environment, matcher, &subject);
            drop(subject);
            matched.map(|matched| match matched {
                Some(bindings) => transition_match(environment, success, bindings),
                None => transition(environment, failure),
            })
        }
        Terminator::Exit(exit) => Ok(GraphAction::Exit(*exit)),
        Terminator::SourceStop {
            kind,
            message,
            site,
        } => {
            let message = message.map(|message| environment.string(message));
            Err(ExecutionError::source_panic(
                plan.source_context(),
                panic_kind(*kind),
                message,
                site.clone(),
            ))
        }
        Terminator::LetAssertPanic {
            subject,
            message,
            site,
            pattern_span,
        } => {
            let subject = environment.value(subject);
            let message = message.map(|message| environment.string(message));
            let subject = crate::runtime::materialize::value(plan, state, subject);
            Err(ExecutionError::let_assert_panic(
                plan.source_context(),
                message,
                site.clone(),
                subject,
                *pattern_span,
            ))
        }
        Terminator::NeverCall { function, args } => {
            let inputs = environment.retain(args);
            let function = match function {
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
