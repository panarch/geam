use camino::Utf8PathBuf;
use ecow::EcoString;
use gleam_compiler_core::ast::SrcSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContext {
    path: Utf8PathBuf,
    source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicSite {
    module: EcoString,
    function: EcoString,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoSite {
    module: EcoString,
    function: EcoString,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCallSite {
    module: EcoString,
    function: EcoString,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionCallTarget<Function> {
    function: Function,
    site: HostCallSite,
}

impl SourceContext {
    pub fn new(path: impl Into<Utf8PathBuf>, source: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            source: source.into(),
        }
    }

    pub fn path(&self) -> &Utf8PathBuf {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub(crate) fn named_source(&self) -> miette::NamedSource<String> {
        miette::NamedSource::new(self.path.as_str(), self.source.clone()).with_language("gleam")
    }
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) fn to_miette(self) -> miette::SourceSpan {
        (self.start, self.len()).into()
    }
}

impl From<SrcSpan> for SourceSpan {
    fn from(span: SrcSpan) -> Self {
        Self {
            start: span.start as usize,
            end: span.end as usize,
        }
    }
}

impl PanicSite {
    pub fn new(module: EcoString, function: EcoString, span: SourceSpan) -> Self {
        Self {
            module,
            function,
            span,
        }
    }

    #[cfg(test)]
    pub(crate) fn unknown() -> Self {
        Self {
            module: "<unknown>".into(),
            function: "<unknown>".into(),
            span: SourceSpan::new(0, 0),
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

impl EchoSite {
    pub fn new(module: EcoString, function: EcoString, span: SourceSpan) -> Self {
        Self {
            module,
            function,
            span,
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

impl HostCallSite {
    pub fn new(module: EcoString, function: EcoString, span: SourceSpan) -> Self {
        Self {
            module,
            function,
            span,
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }

    #[cfg(test)]
    pub(crate) fn unknown() -> Self {
        Self::new(
            "<unknown>".into(),
            "<unknown>".into(),
            SourceSpan::new(0, 0),
        )
    }
}

impl<Function> FunctionCallTarget<Function> {
    pub(crate) fn new(function: Function, site: HostCallSite) -> Self {
        Self { function, site }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn site(&self) -> &HostCallSite {
        &self.site
    }
}

#[cfg(test)]
impl From<crate::plan::FunctionInstantiation>
    for FunctionCallTarget<crate::plan::FunctionInstantiation>
{
    fn from(function: crate::plan::FunctionInstantiation) -> Self {
        Self::new(function, HostCallSite::unknown())
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoSite, HostCallSite, PanicSite, SourceContext, SourceSpan};
    use gleam_compiler_core::ast::SrcSpan;

    #[test]
    fn source_context_preserves_path_and_source() {
        let context = SourceContext::new("main.gleam", "pub fn main() { 1 }");

        assert_eq!(context.path().as_str(), "main.gleam");
        assert_eq!(context.source(), "pub fn main() { 1 }");
        assert_eq!(context.named_source().name(), "main.gleam");
    }

    #[test]
    fn source_span_converts_from_gleam_span() {
        let span = SourceSpan::from(SrcSpan::new(3, 9));

        assert_eq!(span.start(), 3);
        assert_eq!(span.end(), 9);
        assert_eq!(span.len(), 6);
        assert!(!span.is_empty());
        assert_eq!(span.to_miette(), (3, 6).into());

        let empty = SourceSpan::new(4, 4);
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());
    }

    #[test]
    fn panic_site_preserves_module_function_and_span() {
        let site = PanicSite::new("main".into(), "run".into(), SourceSpan::new(4, 8));

        assert_eq!(site.module(), "main");
        assert_eq!(site.function(), "run");
        assert_eq!(site.span(), SourceSpan::new(4, 8));
    }

    #[test]
    fn echo_site_preserves_module_function_and_span() {
        let site = EchoSite::new("main".into(), "run".into(), SourceSpan::new(4, 8));

        assert_eq!(site.module(), "main");
        assert_eq!(site.function(), "run");
        assert_eq!(site.span(), SourceSpan::new(4, 8));
    }

    #[test]
    fn host_call_site_preserves_module_function_and_span() {
        let site = HostCallSite::new("main".into(), "run".into(), SourceSpan::new(4, 8));

        assert_eq!(site.module(), "main");
        assert_eq!(site.function(), "run");
        assert_eq!(site.span(), SourceSpan::new(4, 8));
    }
}
