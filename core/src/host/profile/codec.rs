#[derive(Clone)]
pub(crate) struct HostCodecScope {
    function: std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata>,
}

impl HostCodecScope {
    pub(crate) fn new(
        function: std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata>,
    ) -> Self {
        Self { function }
    }

    pub(crate) fn function(
        &self,
    ) -> &std::sync::Arc<crate::plan::execution::host::HostedFunctionMetadata> {
        &self.function
    }
}
