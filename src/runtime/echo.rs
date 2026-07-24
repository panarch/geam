use std::fmt::{self, Display, Formatter};

use camino::Utf8PathBuf;
use ecow::EcoString;

use crate::plan::{EchoSite, SourceContext};
use crate::runtime::Value;

pub trait EchoSink {
    fn emit(&mut self, output: EchoOutput);
}

#[derive(Debug, Clone, PartialEq)]
pub struct EchoOutput {
    location: EchoLocation,
    message: Option<EcoString>,
    value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EchoLocation {
    Resolved {
        site: EchoSite,
        path: Utf8PathBuf,
        line: usize,
    },
    Site(EchoSite),
}

impl EchoOutput {
    pub fn new(location: EchoLocation, message: Option<EcoString>, value: Value) -> Self {
        Self {
            location,
            message,
            value,
        }
    }

    pub fn location(&self) -> &EchoLocation {
        &self.location
    }

    pub fn message(&self) -> Option<&EcoString> {
        self.message.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl EchoLocation {
    pub fn resolved(site: EchoSite, path: impl Into<Utf8PathBuf>, line: usize) -> Self {
        Self::Resolved {
            site,
            path: path.into(),
            line,
        }
    }

    pub fn site(site: EchoSite) -> Self {
        Self::Site(site)
    }

    pub fn echo_site(&self) -> &EchoSite {
        match self {
            Self::Resolved { site, .. } | Self::Site(site) => site,
        }
    }

    pub fn path(&self) -> Option<&Utf8PathBuf> {
        match self {
            Self::Resolved { path, .. } => Some(path),
            Self::Site(_) => None,
        }
    }

    pub fn line(&self) -> Option<usize> {
        match self {
            Self::Resolved { line, .. } => Some(*line),
            Self::Site(_) => None,
        }
    }

    pub(crate) fn from_context(site: EchoSite, context: Option<&SourceContext>) -> Self {
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
                Self::resolved(site, context.path().clone(), line)
            }
            None => Self::site(site),
        }
    }
}

impl EchoSink for Vec<EchoOutput> {
    fn emit(&mut self, output: EchoOutput) {
        self.push(output);
    }
}

impl Display for EchoOutput {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        match &self.location {
            EchoLocation::Resolved { path, line, .. } => {
                output.push_str(path.as_str());
                output.push(':');
                output.push_str(&line.to_string());
            }
            EchoLocation::Site(site) => {
                output.push_str(site.module());
                output.push_str("::");
                output.push_str(site.function());
                output.push('@');
                output.push_str(&site.span().start().to_string());
                output.push_str("..");
                output.push_str(&site.span().end().to_string());
            }
        }
        if let Some(message) = &self.message {
            output.push(' ');
            output.push_str(message);
        }
        output.push('\n');
        self.value.inspect().write_to(&mut output);
        formatter.write_str(&output)
    }
}

#[cfg(test)]
mod tests {
    use super::{EchoLocation, EchoOutput, EchoSink};
    use crate::{EchoSite, SourceContext, SourceSpan, Value};

    #[test]
    fn resolved_location_preserves_site_path_and_one_based_line() {
        let site = EchoSite::new("main".into(), "run".into(), SourceSpan::new(13, 17));
        let context = SourceContext::new("src/main.gleam", "first\nsecond\nthird");
        let location = EchoLocation::from_context(site.clone(), Some(&context));

        assert_eq!(location.echo_site(), &site);
        assert_eq!(
            location.path().map(|path| path.as_str()),
            Some("src/main.gleam"),
        );
        assert_eq!(location.line(), Some(3));
    }

    #[test]
    fn site_location_preserves_unresolved_site() {
        let site = EchoSite::new("main".into(), "run".into(), SourceSpan::new(4, 8));
        let location = EchoLocation::from_context(site.clone(), None);

        assert_eq!(location.echo_site(), &site);
        assert_eq!(location.path(), None);
        assert_eq!(location.line(), None);
    }

    #[test]
    fn output_preserves_structured_fields_and_resolved_display() {
        let site = EchoSite::new("main".into(), "run".into(), SourceSpan::new(4, 8));
        let output = EchoOutput::new(
            EchoLocation::resolved(site.clone(), "src/main.gleam", 12),
            Some("selected".into()),
            Value::Bool(true),
        );

        assert_eq!(output.location().echo_site(), &site);
        assert_eq!(
            output.message().map(|message| message.as_str()),
            Some("selected")
        );
        assert_eq!(output.value(), &Value::Bool(true));
        assert_eq!(output.to_string(), "src/main.gleam:12 selected\nTrue",);
    }

    #[test]
    fn output_formats_site_fallback_without_message() {
        let output = EchoOutput::new(
            EchoLocation::site(EchoSite::new(
                "main".into(),
                "run".into(),
                SourceSpan::new(4, 8),
            )),
            None,
            Value::Int(1.into()),
        );

        assert_eq!(output.message(), None);
        assert_eq!(output.to_string(), "main::run@4..8\n1");
    }

    #[test]
    fn vector_collects_owned_echo_outputs() {
        let output = EchoOutput::new(
            EchoLocation::site(EchoSite::new(
                "main".into(),
                "run".into(),
                SourceSpan::new(0, 1),
            )),
            None,
            Value::Nil,
        );
        let mut outputs = Vec::new();

        outputs.emit(output.clone());

        assert_eq!(outputs, vec![output]);
    }

    #[test]
    fn run_without_source_context_emits_site_location() {
        let typed = crate::compile_typed_module(
            "main",
            "main.gleam",
            "pub fn main() { echo 1 as \"fallback\" }",
        )
        .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module);
        let mut outputs = Vec::new();

        assert_eq!(
            crate::run_main(&plan, &mut outputs),
            Ok(Value::Int(1.into()))
        );
        assert_eq!(
            outputs,
            vec![EchoOutput::new(
                EchoLocation::site(EchoSite::new(
                    "main".into(),
                    "main".into(),
                    SourceSpan::new(16, 36),
                )),
                Some("fallback".into()),
                Value::Int(1.into()),
            )],
        );
    }
}
