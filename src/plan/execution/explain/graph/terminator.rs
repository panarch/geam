mod bool_branch;
mod jump;
mod let_assert_panic;
mod match_;
mod never_call;
mod source_stop;
mod switch;

use self::bool_branch::write_bool_branch;
use self::jump::write_jump;
use self::let_assert_panic::write_let_assert_panic;
use self::match_::write_match;
use self::never_call::write_never_call;
use self::source_stop::write_source_stop;
use self::switch::{write_float_switch, write_int_switch, write_string_switch};
use super::super::super::{GraphExitId, Terminator};

pub(super) fn write_terminator(
    output: &mut String,
    terminator: &Terminator,
    write_graph_exit: &mut dyn FnMut(&mut String, GraphExitId),
) {
    match terminator {
        Terminator::Jump(edge) => write_jump(output, edge),
        Terminator::BoolBranch {
            subject,
            true_,
            false_,
        } => write_bool_branch(output, subject, true_, false_),
        Terminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => write_int_switch(output, subject, clauses, fallback),
        Terminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => write_float_switch(output, subject, clauses, fallback),
        Terminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => write_string_switch(output, subject, clauses, fallback),
        Terminator::Match {
            subject,
            pattern,
            success,
            failure,
        } => write_match(output, subject, pattern, success, failure),
        Terminator::Exit(exit) => write_graph_exit(output, *exit),
        Terminator::SourceStop { kind, message, .. } => {
            write_source_stop(output, *kind, message.as_ref())
        }
        Terminator::LetAssertPanic {
            subject, message, ..
        } => write_let_assert_panic(output, subject, message.as_ref()),
        Terminator::NeverCall { function, args } => write_never_call(output, function, args),
    }
}
