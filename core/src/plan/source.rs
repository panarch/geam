use super::Text;
use crate::plan::execution::prepared::rust::{Emit, Rust, TextBlock};
use camino::{Utf8Path, Utf8PathBuf};
use ecow::EcoString;
use gleam_compiler_core::ast::SrcSpan;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceContext {
    path: Text,
    source: Cow<'static, str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicSite {
    module: Text,
    function: Text,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoSite {
    module: Text,
    function: Text,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCallSite {
    module: Text,
    function: Text,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCallTarget<Function> {
    pub function: Function,
    pub site: HostCallSite,
}

impl SourceContext {
    pub fn new(path: impl Into<Utf8PathBuf>, source: impl Into<String>) -> Self {
        Self {
            path: Text::Owned(path.into().as_str().into()),
            source: Cow::Owned(source.into()),
        }
    }

    /// Retains source text already embedded in the application, without reading a file.
    pub const fn from_static(path: &'static str, source: &'static str) -> Self {
        Self {
            path: Text::Static(path),
            source: Cow::Borrowed(source),
        }
    }

    /// Retains source embedded as a text block, without reading a file. A leading
    /// line break only separates the literal's opening delimiter from the source,
    /// so that one line break is left out and the rest is borrowed unchanged.
    pub const fn from_static_block(path: &'static str, block: &'static str) -> Self {
        let source = match block.as_bytes() {
            [b'\n', ..] => block.split_at(1).1,
            _ => block,
        };
        Self::from_static(path, source)
    }

    pub fn path(&self) -> &Utf8Path {
        Utf8Path::new(&self.path)
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub(crate) fn named_source(&self) -> miette::NamedSource<String> {
        miette::NamedSource::new(self.path.as_ref(), self.source.to_string()).with_language("gleam")
    }
}

impl Emit for SourceContext {
    fn emit(&self, output: &mut Rust) {
        let path = self.path().as_str();
        match TextBlock::new(self.source()) {
            Some(block) => {
                output.call("source::SourceContext::from_static_block", &[&path, &block])
            }
            None => output.call(
                "source::SourceContext::from_static",
                &[&path, &self.source()],
            ),
        }
    }
}

impl Emit for SourceSpan {
    fn emit(&self, output: &mut Rust) {
        output.call("source::SourceSpan::new", &[&self.start, &self.end]);
    }
}

impl Emit for PanicSite {
    fn emit(&self, output: &mut Rust) {
        output.call(
            "source::PanicSite::from_static",
            &[&self.module(), &self.function(), &self.span],
        );
    }
}

impl Emit for EchoSite {
    fn emit(&self, output: &mut Rust) {
        output.call(
            "source::EchoSite::from_static",
            &[&self.module(), &self.function(), &self.span],
        );
    }
}

impl Emit for HostCallSite {
    fn emit(&self, output: &mut Rust) {
        output.call(
            "source::HostCallSite::from_static",
            &[&self.module(), &self.function(), &self.span],
        );
    }
}

impl SourceSpan {
    pub const fn new(start: usize, end: usize) -> Self {
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
            module: module.into(),
            function: function.into(),
            span,
        }
    }

    /// Uses names already embedded in the application.
    pub const fn from_static(
        module: &'static str,
        function: &'static str,
        span: SourceSpan,
    ) -> Self {
        Self {
            module: Text::Static(module),
            function: Text::Static(function),
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

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

impl EchoSite {
    pub fn new(module: EcoString, function: EcoString, span: SourceSpan) -> Self {
        Self {
            module: module.into(),
            function: function.into(),
            span,
        }
    }

    /// Uses names already embedded in the application.
    pub const fn from_static(
        module: &'static str,
        function: &'static str,
        span: SourceSpan,
    ) -> Self {
        Self {
            module: Text::Static(module),
            function: Text::Static(function),
            span,
        }
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

impl HostCallSite {
    pub fn new(module: EcoString, function: EcoString, span: SourceSpan) -> Self {
        Self {
            module: module.into(),
            function: function.into(),
            span,
        }
    }

    /// Uses names already embedded in the application.
    pub const fn from_static(
        module: &'static str,
        function: &'static str,
        span: SourceSpan,
    ) -> Self {
        Self {
            module: Text::Static(module),
            function: Text::Static(function),
            span,
        }
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn function(&self) -> &str {
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

impl<Function> Emit for FunctionCallTarget<Function>
where
    Function: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self { function, site } = self;
        output.structure(
            "source::FunctionCallTarget",
            &[("function", function), ("site", site)],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoSite, FunctionCallTarget, HostCallSite, PanicSite, SourceContext, SourceSpan};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::prepared::rust::Rust;
    use gleam_compiler_core::ast::SrcSpan;

    #[test]
    fn emits_embedded_source_and_sites_with_exact_names_and_byte_spans() {
        let source = SourceContext::new(
            "sources/example.gleam",
            "pub fn main() { \"line\\ntext\" }\n",
        );
        assert_eq!(
            Rust::expression(&source),
            r##"
data::source::SourceContext::from_static_block("sources/example.gleam", r#"
pub fn main() { "line\ntext" }
"#)"##
                .trim_start_matches('\n')
        );
        let span = SourceSpan::new(3, 12);
        assert_eq!(
            Rust::expression(&span),
            "data::source::SourceSpan::new(3, 12)"
        );
        assert_eq!(
            Rust::expression(&PanicSite::from_static("example", "main", span)),
            "data::source::PanicSite::from_static(\"example\", \"main\", data::source::SourceSpan::new(3, 12))"
        );
        assert_eq!(
            Rust::expression(&EchoSite::from_static("example", "main", span)),
            "data::source::EchoSite::from_static(\"example\", \"main\", data::source::SourceSpan::new(3, 12))"
        );
        let call = HostCallSite::from_static("example", "main", span);
        assert_eq!(
            Rust::expression(&call),
            "data::source::HostCallSite::from_static(\"example\", \"main\", data::source::SourceSpan::new(3, 12))"
        );
        assert_eq!(
            Rust::expression(&FunctionCallTarget::new(IntFunctionId(2), call.clone())),
            r#"
data::source::FunctionCallTarget {
    function: data::function::IntFunctionId(2),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
}"#.trim_start_matches('\n')
        );
        assert_eq!(
            Rust::expression(&FunctionCallTarget::new(2usize, call)),
            r#"
data::source::FunctionCallTarget {
    function: 2,
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 12)),
}"#.trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_single_line_and_carriage_return_source_as_escaped_strings() {
        let single_line = SourceContext::new("main.gleam", "pub fn main() { \"one\" }");
        assert_eq!(
            Rust::expression(&single_line),
            r#"data::source::SourceContext::from_static("main.gleam", "pub fn main() { \"one\" }")"#
        );
        let carriage_return = SourceContext::new("main.gleam", "pub fn main() {\r\n  1\r\n}\r\n");
        assert_eq!(
            Rust::expression(&carriage_return),
            r#"data::source::SourceContext::from_static("main.gleam", "pub fn main() {\r\n  1\r\n}\r\n")"#
        );
    }

    #[test]
    fn source_context_preserves_path_and_source() {
        let context = SourceContext::new("main.gleam", "pub fn main() { 1 }");

        assert_eq!(context.path().as_str(), "main.gleam");
        assert_eq!(context.source(), "pub fn main() { 1 }");
        assert_eq!(context.named_source().name(), "main.gleam");
    }

    #[test]
    fn static_source_context_borrows_paths_and_bytes_without_reading_files() {
        static PATH: &str = "not-on-disk/main.gleam";
        static SOURCE: &str = "pub fn main() { 42 }";
        static CONTEXT: SourceContext = SourceContext::from_static(PATH, SOURCE);

        assert!(std::ptr::eq(
            CONTEXT.path().as_str().as_ptr(),
            PATH.as_ptr()
        ));
        assert!(std::ptr::eq(CONTEXT.source().as_ptr(), SOURCE.as_ptr()));
        assert!(std::ptr::eq(
            CONTEXT.clone().source().as_ptr(),
            SOURCE.as_ptr()
        ));
        assert_eq!(CONTEXT.named_source().name(), PATH);
        assert_eq!(CONTEXT, SourceContext::new(PATH, SOURCE));
        assert_eq!(
            format!("{CONTEXT:?}"),
            "SourceContext { path: \"not-on-disk/main.gleam\", source: \"pub fn main() { 42 }\" }"
        );
    }

    #[test]
    fn static_source_block_excludes_only_its_leading_line_break() {
        static PATH: &str = "not-on-disk/main.gleam";
        static BLOCK: &str = "\npub fn main() { 42 }\n";
        static CONTEXT: SourceContext = SourceContext::from_static_block(PATH, BLOCK);

        assert_eq!(CONTEXT.source(), "pub fn main() { 42 }\n");
        assert!(std::ptr::eq(CONTEXT.source().as_ptr(), BLOCK[1..].as_ptr()));
        assert_eq!(CONTEXT, SourceContext::from_static(PATH, &BLOCK[1..]));
        assert_eq!(
            SourceContext::from_static_block(PATH, "\n\nsecond line\n").source(),
            "\nsecond line\n"
        );
        assert_eq!(
            SourceContext::from_static_block(PATH, "pub fn main() { 42 }").source(),
            "pub fn main() { 42 }"
        );
    }

    #[test]
    fn static_sites_preserve_the_same_names_and_spans() {
        static MODULE: &str = "application/module";
        static FUNCTION: &str = "work";
        const SPAN: SourceSpan = SourceSpan::new(3, 12);
        static PANIC: PanicSite = PanicSite::from_static(MODULE, FUNCTION, SPAN);
        static ECHO: EchoSite = EchoSite::from_static(MODULE, FUNCTION, SPAN);
        static CALL: HostCallSite = HostCallSite::from_static(MODULE, FUNCTION, SPAN);

        assert_eq!(PANIC, PanicSite::new(MODULE.into(), FUNCTION.into(), SPAN));
        assert_eq!(ECHO, EchoSite::new(MODULE.into(), FUNCTION.into(), SPAN));
        assert_eq!(
            CALL,
            HostCallSite::new(MODULE.into(), FUNCTION.into(), SPAN)
        );
        for (module, function, span) in [
            (PANIC.module(), PANIC.function(), PANIC.span()),
            (ECHO.module(), ECHO.function(), ECHO.span()),
            (CALL.module(), CALL.function(), CALL.span()),
        ] {
            assert!(std::ptr::eq(module.as_ptr(), MODULE.as_ptr()));
            assert!(std::ptr::eq(function.as_ptr(), FUNCTION.as_ptr()));
            assert_eq!(span, SPAN);
        }
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
