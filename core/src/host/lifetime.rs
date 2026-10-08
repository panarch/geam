/// The owner required when a producer-owned value re-enters execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostValueLifetime {
    /// Immutable data can re-enter another execution of the same loaded owner.
    LoadedOwner,
    /// Hidden work or callbacks retain their original live execution.
    Execution,
}

impl HostValueLifetime {
    pub(crate) fn requires_execution(self) -> bool {
        self == Self::Execution
    }
}
