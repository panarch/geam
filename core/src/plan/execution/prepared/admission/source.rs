use crate::plan::execution::ExecutionModuleContext;
use crate::plan::{ModuleId, SourceContext, SourceSpan};
use std::collections::HashMap;

pub(super) struct Sources<'data> {
    modules: HashMap<&'data str, &'data Option<SourceContext>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum SourceError {
    Root { index: usize, modules: usize },
    DuplicateModule(String),
    MissingModule(String),
    SpanOrder(SourceSpan),
    SpanBounds { module: String, span: SourceSpan },
}

impl<'data> Sources<'data> {
    pub(super) fn admit(
        root: ModuleId,
        modules: &'data [ExecutionModuleContext],
    ) -> Result<Self, SourceError> {
        if root.index() >= modules.len() {
            return Err(SourceError::Root {
                index: root.index(),
                modules: modules.len(),
            });
        }
        let mut contexts = HashMap::with_capacity(modules.len());
        for module in modules {
            if contexts
                .insert(module.module.as_str(), &module.source_context)
                .is_some()
            {
                return Err(SourceError::DuplicateModule(
                    module.module.as_str().to_owned(),
                ));
            }
        }
        Ok(Self { modules: contexts })
    }

    pub(super) fn span(&self, module: &str, span: SourceSpan) -> Result<(), SourceError> {
        let context = self
            .modules
            .get(module)
            .ok_or_else(|| SourceError::MissingModule(module.to_owned()))?;
        if span.start() > span.end() {
            return Err(SourceError::SpanOrder(span));
        }
        if let Some(context) = context
            && context.source().get(span.start()..span.end()).is_none()
        {
            return Err(SourceError::SpanBounds {
                module: module.to_owned(),
                span,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionModuleContext, ModuleId, SourceContext, SourceError, SourceSpan, Sources,
    };
    use crate::plan::Text;

    #[test]
    fn checks_embedded_utf8_spans_without_accessing_source_paths() {
        let modules = [ExecutionModuleContext {
            module: Text::Static("example"),
            source_context: Some(SourceContext::from_static(
                "/does/not/exist/example.gleam",
                "a\u{e9}\u{1f642}",
            )),
        }];
        let sources = Sources::admit(ModuleId::new(0), &modules).unwrap();
        for (start, end) in [(0, 1), (1, 3), (3, 7), (0, 7), (7, 7)] {
            assert_eq!(sources.span("example", SourceSpan::new(start, end)), Ok(()));
        }
        for (start, end) in [(2, 3), (3, 6), (0, 8), (8, 8)] {
            let span = SourceSpan::new(start, end);
            assert_eq!(
                sources.span("example", span),
                Err(SourceError::SpanBounds {
                    module: "example".into(),
                    span
                })
            );
        }
        assert_eq!(
            sources.span("example", SourceSpan::new(4, 2)),
            Err(SourceError::SpanOrder(SourceSpan::new(4, 2)))
        );
        assert_eq!(
            sources.span("missing", SourceSpan::new(0, 0)),
            Err(SourceError::MissingModule("missing".into()))
        );
        assert!(std::ptr::eq(
            sources.modules["example"],
            &modules[0].source_context
        ));
    }

    #[test]
    fn rejects_missing_roots_and_duplicate_module_owners() {
        assert_eq!(
            Sources::admit(ModuleId::new(0), &[]).err(),
            Some(SourceError::Root {
                index: 0,
                modules: 0
            })
        );
        let modules = [
            ExecutionModuleContext {
                module: Text::Static("example"),
                source_context: None,
            },
            ExecutionModuleContext {
                module: Text::Static("example"),
                source_context: None,
            },
        ];
        assert_eq!(
            Sources::admit(ModuleId::new(2), &modules).err(),
            Some(SourceError::Root {
                index: 2,
                modules: 2
            })
        );
        assert_eq!(
            Sources::admit(ModuleId::new(0), &modules).err(),
            Some(SourceError::DuplicateModule("example".into()))
        );
        let sources = Sources::admit(ModuleId::new(0), &modules[..1]).unwrap();
        assert_eq!(sources.span("example", SourceSpan::new(3, 9)), Ok(()));
        assert_eq!(
            sources.span("example", SourceSpan::new(9, 3)),
            Err(SourceError::SpanOrder(SourceSpan::new(9, 3)))
        );
    }
}
