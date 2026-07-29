use crate::host::HostFailure;
use crate::plan::{FunctionType, HostCallSite, SourceContext};
use camino::Utf8PathBuf;
use ecow::EcoString;
use miette::NamedSource;
use std::fmt;

#[derive(Debug, Clone)]
pub struct HostError {
    package: EcoString,
    module: EcoString,
    function: EcoString,
    signature: FunctionType,
    failure: HostFailure,
    location: HostLocation,
    source: Option<Box<NamedSource<String>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostLocation {
    Resolved {
        site: HostCallSite,
        path: Utf8PathBuf,
        line: usize,
    },
    Site(HostCallSite),
    Host {
        caller: HostOrigin,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostOrigin {
    package: EcoString,
    module: EcoString,
    function: EcoString,
    signature: FunctionType,
}

#[derive(Clone)]
pub(crate) enum HostCallOrigin {
    Entry,
    Source(HostCallSite),
    Host(HostOrigin),
}

impl HostError {
    pub(crate) fn new(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
        failure: HostFailure,
        site: HostCallSite,
        source_context: Option<&SourceContext>,
    ) -> Self {
        let location = HostLocation::from_context(site, source_context);
        let source = source_context
            .map(SourceContext::named_source)
            .map(Box::new);
        Self {
            package,
            module,
            function,
            signature,
            failure,
            location,
            source,
        }
    }

    pub(crate) fn new_from_host(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
        failure: HostFailure,
        caller: HostOrigin,
    ) -> Self {
        Self {
            package,
            module,
            function,
            signature,
            failure,
            location: HostLocation::Host { caller },
            source: None,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn signature(&self) -> &FunctionType {
        &self.signature
    }

    pub fn failure(&self) -> &HostFailure {
        &self.failure
    }

    pub fn location(&self) -> &HostLocation {
        &self.location
    }

    pub(in crate::runtime::error) fn source(&self) -> Option<&NamedSource<String>> {
        self.source.as_deref()
    }

    pub(in crate::runtime::error) fn primary_label(&self) -> String {
        format!(
            "host function {}::{}.{} failed",
            self.package, self.module, self.function,
        )
    }
}

impl HostLocation {
    pub fn site(&self) -> Option<&HostCallSite> {
        match self {
            Self::Resolved { site, .. } | Self::Site(site) => Some(site),
            Self::Host { .. } => None,
        }
    }

    pub fn path(&self) -> Option<&Utf8PathBuf> {
        match self {
            Self::Resolved { path, .. } => Some(path),
            Self::Site(_) | Self::Host { .. } => None,
        }
    }

    pub fn line(&self) -> Option<usize> {
        match self {
            Self::Resolved { line, .. } => Some(*line),
            Self::Site(_) | Self::Host { .. } => None,
        }
    }

    pub fn caller(&self) -> Option<&HostOrigin> {
        match self {
            Self::Host { caller } => Some(caller),
            Self::Resolved { .. } | Self::Site(_) => None,
        }
    }

    fn from_context(site: HostCallSite, context: Option<&SourceContext>) -> Self {
        match context {
            Some(context) => {
                let line = context
                    .source()
                    .as_bytes()
                    .iter()
                    .take(site.span().start())
                    .filter(|byte| **byte == b'\n')
                    .count()
                    + 1;
                Self::Resolved {
                    site,
                    path: context.path().clone(),
                    line,
                }
            }
            None => Self::Site(site),
        }
    }
}

impl HostOrigin {
    fn new(
        package: EcoString,
        module: EcoString,
        function: EcoString,
        signature: FunctionType,
    ) -> Self {
        Self {
            package,
            module,
            function,
            signature,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn signature(&self) -> &FunctionType {
        &self.signature
    }
}

impl HostCallOrigin {
    pub(crate) fn source(site: HostCallSite) -> Self {
        Self::Source(site)
    }

    pub(crate) fn host(function: &crate::plan::execution::host::HostedFunctionMetadata) -> Self {
        Self::Host(HostOrigin::new(
            function.package().clone(),
            function.module().clone(),
            function.name().clone(),
            function.signature().clone(),
        ))
    }

    pub(crate) fn into_source_site(
        self,
        declaration: &HostCallSite,
    ) -> Result<HostCallSite, HostOrigin> {
        match self {
            Self::Entry => Ok(declaration.clone()),
            Self::Source(site) => Ok(site),
            Self::Host(caller) => Err(caller),
        }
    }
}

impl PartialEq for HostError {
    fn eq(&self, other: &Self) -> bool {
        self.package == other.package
            && self.module == other.module
            && self.function == other.function
            && self.signature == other.signature
            && self.failure == other.failure
            && self.location == other.location
            && named_source_eq(self.source(), other.source())
    }
}

impl Eq for HostError {}

fn named_source_eq(
    left: Option<&NamedSource<String>>,
    right: Option<&NamedSource<String>>,
) -> bool {
    left.map(|source| (source.name(), source.inner()))
        == right.map(|source| (source.name(), source.inner()))
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "host function {}::{}.{} failed: {}",
            self.package, self.module, self.function, self.failure,
        )
    }
}

impl std::error::Error for HostError {}

#[cfg(test)]
mod tests {
    use super::{HostCallOrigin, HostError, HostLocation};
    use crate::host::HostFailure;
    use crate::plan::{FunctionType, HostCallSite, SourceContext, SourceSpan, ValueType};

    #[test]
    fn host_error_owns_identity_signature_failure_and_resolved_location() {
        let source = SourceContext::new("src/main.gleam", "pub fn main() {\n  math.add(1, 2)\n}");
        let site = HostCallSite::new("main".into(), "main".into(), SourceSpan::new(18, 32));
        let error = HostError::new(
            "host_support".into(),
            "host/math".into(),
            "add".into(),
            FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
            HostFailure::new("unavailable"),
            site.clone(),
            Some(&source),
        );

        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/math");
        assert_eq!(error.function(), "add");
        assert_eq!(
            error.signature(),
            &FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
        );
        assert_eq!(error.failure().message(), "unavailable");
        assert_eq!(error.location().site(), Some(&site));
        assert_eq!(error.location().caller(), None);
        assert_eq!(error.location().path(), Some(source.path()));
        assert_eq!(error.location().line(), Some(2));
        assert_eq!(
            error.to_string(),
            "host function host_support::host/math.add failed: unavailable",
        );
    }

    #[test]
    fn source_less_host_error_retains_site_only_location() {
        let site = HostCallSite::new("main".into(), "main".into(), SourceSpan::new(4, 8));
        let error = HostError::new(
            "host_support".into(),
            "host/math".into(),
            "add".into(),
            FunctionType::new(Vec::new(), ValueType::Int),
            HostFailure::new("unavailable"),
            site.clone(),
            None,
        );

        assert_eq!(error.location(), &HostLocation::Site(site.clone()));
        assert_eq!(error.location().site(), Some(&site));
        assert_eq!(error.location().caller(), None);
        assert_eq!(error.location().path(), None);
        assert_eq!(error.location().line(), None);
    }

    #[test]
    fn host_call_origin_uses_the_declaration_or_exact_source_site() {
        let declaration =
            HostCallSite::new("host/math".into(), "add".into(), SourceSpan::new(2, 5));
        let source = HostCallSite::new("main".into(), "main".into(), SourceSpan::new(20, 34));

        assert_eq!(
            HostCallOrigin::Entry.into_source_site(&declaration),
            Ok(declaration.clone()),
        );
        assert_eq!(
            HostCallOrigin::source(source.clone()).into_source_site(&declaration),
            Ok(source),
        );
    }

    #[test]
    fn host_origin_location_preserves_the_invoking_host_identity() {
        let caller = super::HostOrigin::new(
            "application".into(),
            "host/outer".into(),
            "apply".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let error = HostError::new_from_host(
            "application".into(),
            "host/inner".into(),
            "increment".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            HostFailure::new("unavailable"),
            caller.clone(),
        );

        assert_eq!(
            error.location(),
            &HostLocation::Host {
                caller: caller.clone(),
            },
        );
        assert_eq!(error.location().site(), None);
        assert_eq!(error.location().path(), None);
        assert_eq!(error.location().line(), None);
        assert_eq!(error.location().caller(), Some(&caller));
        assert_eq!(caller.package(), "application");
        assert_eq!(caller.module(), "host/outer");
        assert_eq!(caller.function(), "apply");
        assert_eq!(
            caller.signature(),
            &FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let declaration = HostCallSite::new(
            "host/inner".into(),
            "increment".into(),
            SourceSpan::new(0, 0),
        );
        assert_eq!(
            HostCallOrigin::Host(caller.clone()).into_source_site(&declaration),
            Err(caller),
        );
    }

    #[test]
    fn host_error_equality_includes_owned_source_context() {
        let site = HostCallSite::new("main".into(), "main".into(), SourceSpan::new(18, 32));
        let source = SourceContext::new("src/main.gleam", "pub fn main() {\n  fail()\n}");
        let different_source =
            SourceContext::new("src/other.gleam", "pub fn main() {\n  fail()\n}");
        let resolved = HostError::new(
            "host_support".into(),
            "host/math".into(),
            "fail".into(),
            FunctionType::new(Vec::new(), ValueType::Int),
            HostFailure::new("unavailable"),
            site.clone(),
            Some(&source),
        );
        let different = HostError::new(
            "host_support".into(),
            "host/math".into(),
            "fail".into(),
            FunctionType::new(Vec::new(), ValueType::Int),
            HostFailure::new("unavailable"),
            site.clone(),
            Some(&different_source),
        );
        let site_only = HostError::new(
            "host_support".into(),
            "host/math".into(),
            "fail".into(),
            FunctionType::new(Vec::new(), ValueType::Int),
            HostFailure::new("unavailable"),
            site,
            None,
        );

        assert_eq!(resolved, resolved.clone());
        assert_ne!(resolved, different);
        assert_eq!(site_only, site_only.clone());
        assert_ne!(resolved, site_only);
        assert_ne!(site_only, resolved);
    }
}
