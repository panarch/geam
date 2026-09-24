use std::ffi::OsString;

pub struct State {
    pub arguments: Vec<OsString>,
}

impl Default for State {
    fn default() -> Self {
        println!("arguments-initialized");
        Self {
            arguments: std::env::args_os().skip(1).collect(),
        }
    }
}

#[geam::provider(package = "application_arguments", state = State, modules = [arguments])]
pub struct Component;

#[geam::module(path = "application_arguments")]
mod arguments {
    use super::State;
    use geam::provider::{BigInt, Call, StringValue};

    #[geam::function]
    fn strings(#[geam::call] call: &Call<State>) -> Vec<StringValue> {
        call.state()
            .arguments
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned().into())
            .collect()
    }

    #[geam::function]
    fn native_units(#[geam::call] call: &Call<State>) -> Vec<Vec<BigInt>> {
        call.state()
            .arguments
            .iter()
            .map(|argument| {
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;
                    argument
                        .as_bytes()
                        .iter()
                        .copied()
                        .map(BigInt::from)
                        .collect()
                }
                #[cfg(windows)]
                {
                    use std::os::windows::ffi::OsStrExt;
                    argument.encode_wide().map(BigInt::from).collect()
                }
            })
            .collect()
    }
}
