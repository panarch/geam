mod callable;
mod entry;
mod execution;
mod list;
mod main;
mod returning_function;
mod value;

pub(in crate::runtime) use callable::{InvocableFunctionValue, prepare_callable};
pub(in crate::runtime) use entry::EntryTarget;
use execution::run;
pub(in crate::runtime) use execution::{EntryValue, Execution};
#[cfg(test)]
pub(in crate::runtime) use list::run_int_list;
pub(in crate::runtime) use list::run_list;
pub(in crate::runtime) use main::prepare_main;
pub(in crate::runtime) use returning_function::run_core_function;
pub(in crate::runtime) use value::{
    run_bit_array, run_bool, run_custom, run_float, run_int, run_never, run_nil, run_string,
    run_tuple, run_utf_codepoint,
};
