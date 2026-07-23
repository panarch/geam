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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

    #[error("module {module} was supplied more than once")]
    DuplicateModule {
        module: EcoString,
        first_path: Utf8PathBuf,
        second_path: Utf8PathBuf,
    },

    #[error("root module {module} was not supplied")]
    MissingRoot { module: EcoString },

    #[error("module import cycle: {modules:?}")]
    ImportCycle { modules: Vec<EcoString> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSource {
    module: EcoString,
    path: Utf8PathBuf,
    source: String,
}

impl ModuleSource {
    pub fn new(
        module: impl Into<EcoString>,
        path: impl Into<Utf8PathBuf>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            path: path.into(),
            source: source.into(),
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn path(&self) -> &Utf8PathBuf {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Debug)]
pub struct TypedProgram {
    root_module: EcoString,
    root_index: usize,
    modules: Vec<TypedProgramModule>,
}

#[derive(Debug)]
pub(crate) struct TypedProgramModule {
    pub(crate) module: TypedModule,
    pub(crate) path: Utf8PathBuf,
    pub(crate) source: String,
}

impl TypedProgram {
    pub fn root_module(&self) -> &EcoString {
        &self.root_module
    }

    pub fn modules(&self) -> impl ExactSizeIterator<Item = &TypedModule> {
        self.modules.iter().map(|module| &module.module)
    }

    pub(crate) fn into_parts(self) -> (usize, Vec<TypedProgramModule>) {
        (self.root_index, self.modules)
    }
}

pub fn compile_typed_module(
    module_name: impl Into<EcoString>,
    path: impl Into<Utf8PathBuf>,
    src: &str,
) -> Result<TypedModule, FrontendError> {
    let module_name = module_name.into();
    compile_typed_program_sources(
        module_name.clone(),
        vec![ModuleSource::new(module_name, path, src)],
    )
    .map(|program| {
        let (root_index, mut modules) = program.into_parts();
        modules.swap_remove(root_index).module
    })
}

pub fn compile_typed_program(
    root_module: impl Into<EcoString>,
    modules: impl IntoIterator<Item = ModuleSource>,
) -> Result<TypedProgram, FrontendError> {
    compile_typed_program_sources(root_module.into(), modules.into_iter().collect())
}

fn compile_typed_program_sources(
    root_module: EcoString,
    modules: Vec<ModuleSource>,
) -> Result<TypedProgram, FrontendError> {
    let warnings = WarningEmitter::null();
    let mut parsed_modules = Vec::new();

    for source in modules {
        let parsed = parse::parse_module(source.path.clone(), &source.source, &warnings).map_err(
            |error| FrontendError::Parse {
                path: source.path.clone(),
                error: Box::new(error),
            },
        )?;
        let mut module = parsed.module;
        module.name = source.module.clone();
        parsed_modules.push(ParsedModule { source, module });
    }

    let mut module_paths = BTreeMap::new();
    for parsed in &parsed_modules {
        if let Some(first_path) =
            module_paths.insert(parsed.source.module.clone(), parsed.source.path.clone())
        {
            return Err(FrontendError::DuplicateModule {
                module: parsed.source.module.clone(),
                first_path,
                second_path: parsed.source.path.clone(),
            });
        }
    }

    if !module_paths.contains_key(&root_module) {
        return Err(FrontendError::MissingRoot {
            module: root_module,
        });
    }

    let order = dependency_order(&parsed_modules)?;
    let positions = order
        .iter()
        .enumerate()
        .map(|(index, module)| (module.clone(), index))
        .collect::<BTreeMap<_, _>>();
    parsed_modules.sort_by_key(|module| positions[&module.source.module]);

    let ids = UniqueIdGenerator::new();
    let mut importable_modules = ImHashMap::new();
    importable_modules.insert(PRELUDE_MODULE_NAME.into(), build_prelude(&ids));

    let direct_dependencies = HashMap::<EcoString, ()>::new();
    let dev_dependencies = HashSet::new();
    let config = PackageConfig {
        name: "geam".into(),
        ..PackageConfig::default()
    };
    let root_index = order
        .iter()
        .take_while(|module| *module != &root_module)
        .count();
    let mut typed_modules = Vec::with_capacity(order.len());

    for parsed in parsed_modules {
        let path = parsed.source.path;
        let source = parsed.source.source;

        let module = ModuleAnalyzerConstructor::<()> {
            target: Target::Erlang,
            ids: &ids,
            origin: Origin::Src,
            importable_modules: &importable_modules,
            warnings: &TypeWarningEmitter::new(
                path.clone(),
                source.clone().into(),
                warnings.clone(),
            ),
            direct_dependencies: &direct_dependencies,
            dev_dependencies: &dev_dependencies,
            target_support: TargetSupport::Enforced,
            package_config: &config,
        }
        .infer_module(parsed.module, LineNumbers::new(&source), path.clone())
        .into_result()
        .map_err(|errors| FrontendError::Analyse {
            errors: errors.into_iter().collect(),
        })?;

        importable_modules.insert(module.name.clone(), module.type_info.clone());
        typed_modules.push(TypedProgramModule {
            module,
            path,
            source,
        });
    }

    Ok(TypedProgram {
        root_module,
        root_index,
        modules: typed_modules,
    })
}

struct ParsedModule {
    source: ModuleSource,
    module: gleam_core::ast::UntypedModule,
}

fn dependency_order(modules: &[ParsedModule]) -> Result<Vec<EcoString>, FrontendError> {
    #[derive(Clone, Copy)]
    enum Visit {
        Visiting(usize),
        Visited,
    }

    fn visit(
        module: &EcoString,
        dependencies: &BTreeMap<EcoString, BTreeSet<EcoString>>,
        visits: &mut BTreeMap<EcoString, Visit>,
        path: &mut Vec<EcoString>,
        order: &mut Vec<EcoString>,
    ) -> Result<(), FrontendError> {
        match visits.get(module).copied() {
            Some(Visit::Visited) => return Ok(()),
            Some(Visit::Visiting(position)) => {
                let mut modules = path[position..].to_vec();
                modules.push(module.clone());
                return Err(FrontendError::ImportCycle { modules });
            }
            None => {}
        }

        visits.insert(module.clone(), Visit::Visiting(path.len()));
        path.push(module.clone());
        for dependency in &dependencies[module] {
            visit(dependency, dependencies, visits, path, order)?;
        }
        path.pop();
        visits.insert(module.clone(), Visit::Visited);
        order.push(module.clone());
        Ok(())
    }

    let supplied = modules
        .iter()
        .map(|module| module.source.module.clone())
        .collect::<BTreeSet<_>>();
    let dependencies = modules
        .iter()
        .map(|module| {
            let internal = module
                .module
                .dependencies(Target::Erlang)
                .into_iter()
                .map(|(dependency, _)| dependency)
                .filter(|dependency| supplied.contains(dependency))
                .collect::<BTreeSet<_>>();
            (module.source.module.clone(), internal)
        })
        .collect::<BTreeMap<_, _>>();
    let mut order = Vec::with_capacity(modules.len());
    let mut visits = BTreeMap::new();
    let mut path = Vec::new();
    for module in dependencies.keys() {
        visit(module, &dependencies, &mut visits, &mut path, &mut order)?;
    }
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::{FrontendError, ModuleSource, compile_typed_module, compile_typed_program};

    #[derive(Debug, PartialEq)]
    enum FrontendErrorKind {
        Parse,
        Analyse,
        DuplicateModule,
        MissingRoot,
        ImportCycle,
    }

    fn frontend_error_kind(error: &FrontendError) -> FrontendErrorKind {
        match error {
            FrontendError::Parse { .. } => FrontendErrorKind::Parse,
            FrontendError::Analyse { .. } => FrontendErrorKind::Analyse,
            FrontendError::DuplicateModule { .. } => FrontendErrorKind::DuplicateModule,
            FrontendError::MissingRoot { .. } => FrontendErrorKind::MissingRoot,
            FrontendError::ImportCycle { .. } => FrontendErrorKind::ImportCycle,
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

    #[test]
    fn compile_typed_program_orders_dependencies_before_dependants() {
        let root = ModuleSource::new(
            "main",
            "main.gleam",
            "import support\npub fn main() { support.value() }",
        );
        assert_eq!(root.module(), "main");
        assert_eq!(root.path().as_str(), "main.gleam");
        assert_eq!(
            root.source(),
            "import support\npub fn main() { support.value() }",
        );
        let program = compile_typed_program(
            "main",
            [
                root,
                ModuleSource::new("unrelated", "unrelated.gleam", "pub fn value() { 2 }"),
                ModuleSource::new("support", "support.gleam", "pub fn value() { 1 }"),
            ],
        )
        .expect("program should compile");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["support", "main", "unrelated"],
        );
        assert_eq!(program.root_module(), "main");
    }

    #[test]
    fn compile_typed_program_rejects_duplicate_module_names() {
        let error = compile_typed_program(
            "main",
            [
                ModuleSource::new("main", "first.gleam", "pub fn main() { 1 }"),
                ModuleSource::new("main", "second.gleam", "pub fn other() { 2 }"),
            ],
        )
        .expect_err("duplicate module should fail");

        assert_eq!(
            frontend_error_kind(&error),
            FrontendErrorKind::DuplicateModule
        );
        assert_eq!(error.to_string(), "module main was supplied more than once",);
        assert_eq!(
            format!("{error:?}"),
            "DuplicateModule { module: \"main\", first_path: \"first.gleam\", second_path: \"second.gleam\" }",
        );
    }

    #[test]
    fn compile_typed_program_rejects_missing_root() {
        let error = compile_typed_program(
            "main",
            [ModuleSource::new(
                "support",
                "support.gleam",
                "pub fn value() { 1 }",
            )],
        )
        .expect_err("missing root should fail");

        assert_eq!(frontend_error_kind(&error), FrontendErrorKind::MissingRoot);
        assert_eq!(error.to_string(), "root module main was not supplied");
    }

    #[test]
    fn compile_typed_program_rejects_import_cycles_with_a_module_path() {
        let error = compile_typed_program(
            "one",
            [
                ModuleSource::new(
                    "one",
                    "one.gleam",
                    "import two\npub fn value() { two.value() }",
                ),
                ModuleSource::new(
                    "two",
                    "two.gleam",
                    "import one\npub fn value() { one.value() }",
                ),
            ],
        )
        .expect_err("cycle should fail");

        assert_eq!(frontend_error_kind(&error), FrontendErrorKind::ImportCycle);
        assert_eq!(
            format!("{error:?}"),
            "ImportCycle { modules: [\"one\", \"two\", \"one\"] }",
        );
    }

    #[test]
    fn compile_typed_program_leaves_unknown_imports_to_gleam_analysis() {
        let error = compile_typed_program(
            "main",
            [ModuleSource::new(
                "main",
                "main.gleam",
                "import unknown\npub fn main() { 1 }",
            )],
        )
        .expect_err("unknown import should fail in analysis");

        assert_eq!(frontend_error_kind(&error), FrontendErrorKind::Analyse);
    }
}
