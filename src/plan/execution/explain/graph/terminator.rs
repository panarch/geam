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
        Terminator::Jump(jump) => write_jump(output, jump.edge()),
        Terminator::BoolBranch(branch) => {
            write_bool_branch(output, &branch.subject(), branch.true_(), branch.false_())
        }
        Terminator::IntSwitch(switch) => write_int_switch(
            output,
            &switch.subject(),
            switch.clauses(),
            switch.fallback(),
        ),
        Terminator::FloatSwitch(switch) => write_float_switch(
            output,
            &switch.subject(),
            switch.clauses(),
            switch.fallback(),
        ),
        Terminator::StringSwitch(switch) => write_string_switch(
            output,
            &switch.subject(),
            switch.clauses(),
            switch.fallback(),
        ),
        Terminator::Match(matcher) => write_match(
            output,
            matcher.subject(),
            matcher.pattern(),
            matcher.success(),
            matcher.failure(),
        ),
        Terminator::Exit(exit) => write_graph_exit(output, *exit),
        Terminator::SourceStop(stop) => {
            write_source_stop(output, stop.kind(), stop.message().as_ref())
        }
        Terminator::LetAssertPanic(panic) => {
            write_let_assert_panic(output, panic.subject(), panic.message().as_ref())
        }
        Terminator::NeverCall(call) => write_never_call(output, call.function(), call.args()),
    }
}
