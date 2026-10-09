mod build;
mod cargo;
mod control;
mod generator;
mod source;

pub(super) use build::{BuildProfile, BuildSession, ExecutableBuilder};
pub(super) use cargo::{CargoLock, RunnerChecker, RunnerExecutor, SystemCargo, reconcile_lock};
pub(super) use source::reconcile_source;

pub(super) fn binary_name(root_package: &str) -> String {
    format!("{root_package}-geam-runner")
}
