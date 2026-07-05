use camino::Utf8PathBuf;
use ecow::EcoString;
use gleam_core::analyse::{ModuleAnalyzerConstructor, TargetSupport};
use gleam_core::ast::TypedModule;
use gleam_core::build::{Origin, Target};
use gleam_core::config::PackageConfig;
use gleam_core::line_numbers::LineNumbers;
use gleam_core::parse;
use gleam_core::parse::error::ParseError;
use gleam_core::type_::{Error as TypeError, PRELUDE_MODULE_NAME, build_prelude};
use gleam_core::uid::UniqueIdGenerator;
use gleam_core::warning::{TypeWarningEmitter, WarningEmitter};
use im::HashMap as ImHashMap;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("failed to parse Gleam module {path}")]
    Parse {
        path: Utf8PathBuf,
        error: Box<ParseError>,
    },

    #[error("failed to analyse Gleam module")]
    Analyse { errors: Vec<TypeError> },
}

pub fn compile_typed_module(
    module_name: impl Into<EcoString>,
    path: impl Into<Utf8PathBuf>,
    src: &str,
) -> Result<TypedModule, FrontendError> {
    let module_name = module_name.into();
    let path = path.into();
    let warnings = WarningEmitter::null();
    let parsed = parse::parse_module(path.clone(), src, &warnings).map_err(|error| {
        FrontendError::Parse {
            path: path.clone(),
            error: Box::new(error),
        }
    })?;

    let ids = UniqueIdGenerator::new();
    let mut importable_modules = ImHashMap::new();
    importable_modules.insert(PRELUDE_MODULE_NAME.into(), build_prelude(&ids));

    let direct_dependencies = HashMap::<EcoString, ()>::new();
    let dev_dependencies = HashSet::new();
    let config = PackageConfig {
        name: "geam".into(),
        ..PackageConfig::default()
    };

    let mut module = parsed.module;
    module.name = module_name;

    ModuleAnalyzerConstructor::<()> {
        target: Target::Erlang,
        ids: &ids,
        origin: Origin::Src,
        importable_modules: &importable_modules,
        warnings: &TypeWarningEmitter::new(path.clone(), src.into(), warnings),
        direct_dependencies: &direct_dependencies,
        dev_dependencies: &dev_dependencies,
        target_support: TargetSupport::Enforced,
        package_config: &config,
    }
    .infer_module(module, LineNumbers::new(src), path)
    .into_result()
    .map_err(|errors| FrontendError::Analyse {
        errors: errors.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::{FrontendError, compile_typed_module};

    #[derive(Debug, PartialEq)]
    enum FrontendErrorKind {
        Parse,
        Analyse,
    }

    fn frontend_error_kind(error: &FrontendError) -> FrontendErrorKind {
        match error {
            FrontendError::Parse { .. } => FrontendErrorKind::Parse,
            FrontendError::Analyse { .. } => FrontendErrorKind::Analyse,
        }
    }

    #[test]
    fn compile_typed_module_returns_gleam_typed_module() {
        let module = compile_typed_module(
            "main",
            "main.gleam",
            r#"
pub fn main() {
  1
}
"#,
        )
        .expect("module should compile");

        assert_eq!(module.name, "main");
        assert_eq!(module.definitions.functions.len(), 1);
        assert_eq!(
            module.definitions.functions[0]
                .name
                .as_ref()
                .map(|(_, name)| name.as_str()),
            Some("main"),
        );
        assert!(module.type_info.values.contains_key("main"));
    }

    #[test]
    fn compile_typed_module_returns_gleam_parse_errors() {
        let error = compile_typed_module("main", "main.gleam", "pub fn main(")
            .expect_err("invalid syntax should fail in Gleam parse");

        assert_eq!(frontend_error_kind(&error), FrontendErrorKind::Parse);
        assert_eq!(error.to_string(), "failed to parse Gleam module main.gleam");
    }

    #[test]
    fn compile_typed_module_returns_gleam_analyse_errors() {
        let error = compile_typed_module(
            "main",
            "main.gleam",
            r#"
pub fn main() {
  1 + "bad"
}
"#,
        )
        .expect_err("invalid types should fail in Gleam analyse");

        assert_eq!(frontend_error_kind(&error), FrontendErrorKind::Analyse);
        assert_eq!(error.to_string(), "failed to analyse Gleam module");
    }
}
