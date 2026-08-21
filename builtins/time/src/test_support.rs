use super::{GleamTimeProfile, TimeSource, host_providers};
use crate::{
    HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
    compile_typed_host_program, plan_host_program,
};
use ecow::EcoString;
use std::collections::VecDeque;
use std::time::SystemTime;

pub(super) const CALENDAR_SOURCE: &str = r#"
@external(erlang, "gleam_time_ffi", "local_time_offset_seconds")
@external(javascript, "../../gleam_time_ffi.mjs", "local_time_offset_seconds")
fn local_time_offset_seconds() -> Int

pub fn current_offset() -> Int {
  local_time_offset_seconds()
}
"#;

pub(super) const TIMESTAMP_SOURCE: &str = r#"
@external(erlang, "gleam_time_ffi", "system_time")
@external(javascript, "../../gleam_time_ffi.mjs", "system_time")
fn get_system_time() -> #(Int, Int)

pub fn current_parts() -> #(Int, Int) {
  get_system_time()
}
"#;

#[derive(Default)]
pub(super) struct ScriptedSource {
    pub(super) times: VecDeque<Result<SystemTime, crate::HostFailure>>,
    pub(super) offsets: VecDeque<Result<i32, crate::HostFailure>>,
}

impl TimeSource for ScriptedSource {
    fn system_time(&mut self) -> Result<SystemTime, crate::HostFailure> {
        self.times
            .pop_front()
            .unwrap_or_else(|| Err(crate::HostFailure::new("scripted system time exhausted")))
    }

    fn local_offset_seconds(&mut self) -> Result<i32, crate::HostFailure> {
        self.offsets
            .pop_front()
            .unwrap_or_else(|| Err(crate::HostFailure::new("scripted local offset exhausted")))
    }
}

pub(super) fn execution<Source>(
    source: &str,
    root_module: &str,
) -> HostedExecution<GleamTimeProfile<Source>>
where
    Source: TimeSource,
{
    let modules = if root_module == "main" {
        vec![
            ModuleSource::new(
                "gleam/time/calendar",
                "src/gleam/time/calendar.gleam",
                CALENDAR_SOURCE,
            ),
            ModuleSource::new(
                "gleam/time/timestamp",
                "src/gleam/time/timestamp.gleam",
                TIMESTAMP_SOURCE,
            ),
            ModuleSource::new("main", "src/main.gleam", source),
        ]
    } else {
        vec![
            ModuleSource::new(
                "gleam/time/calendar",
                "src/gleam/time/calendar.gleam",
                if root_module == "gleam/time/calendar" {
                    source
                } else {
                    CALENDAR_SOURCE
                },
            ),
            ModuleSource::new(
                "gleam/time/timestamp",
                "src/gleam/time/timestamp.gleam",
                if root_module == "gleam/time/timestamp" {
                    source
                } else {
                    TIMESTAMP_SOURCE
                },
            ),
        ]
    };
    let providers = host_providers::<GleamTimeProfile<Source>>()
        .expect("synthetic Time providers should register");
    let hosts = HostProviderSet::with_providers(
        Vec::<HostModule<GleamTimeProfile<Source>>>::new(),
        providers,
    )
    .expect("synthetic Time provider modules should be unique");
    let typed = compile_typed_host_program(
        "gleam_time",
        root_module,
        [PackageSource::new(
            "gleam_time",
            Vec::<EcoString>::new(),
            modules,
        )],
        hosts,
    )
    .expect("synthetic Time source should compile");
    let plan = plan_host_program(typed).expect("synthetic Time source should plan");
    HostedExecution::try_from_module_plan(plan).expect("synthetic Time execution should seal")
}

#[cfg(test)]
mod tests {
    use super::ScriptedSource;
    use crate::TimeSource;

    #[test]
    fn scripted_source_reports_exhausted_effect_queues() {
        let mut source = ScriptedSource::default();

        assert_eq!(
            source
                .system_time()
                .expect_err("empty time queue should fail")
                .message(),
            "scripted system time exhausted",
        );
        assert_eq!(
            source
                .local_offset_seconds()
                .expect_err("empty offset queue should fail")
                .message(),
            "scripted local offset exhausted",
        );
    }
}
