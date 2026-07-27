use super::{FrontendError, ModuleSource, PackageSource};
use camino::Utf8PathBuf;
use ecow::EcoString;
use gleam_core::analyse::{ModuleAnalyzerConstructor, TargetSupport};
use gleam_core::ast::{TypedModule, UntypedModule};
use gleam_core::build::{Origin, Target};
use gleam_core::config::PackageConfig;
use gleam_core::line_numbers::LineNumbers;
use gleam_core::parse;
use gleam_core::type_::{PRELUDE_MODULE_NAME, build_prelude};
use gleam_core::uid::UniqueIdGenerator;
use gleam_core::warning::{TypeWarningEmitter, WarningEmitter};
use im::HashMap as ImHashMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

const SINGLE_PACKAGE: &str = "geam";

#[derive(Debug)]
pub struct TypedProgram {
    root_package: EcoString,
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
    pub fn root_package(&self) -> &EcoString {
        &self.root_package
    }

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
    compile_typed_program(
        module_name.clone(),
        [ModuleSource::new(module_name, path, src)],
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
    compile_typed_package_program(
        SINGLE_PACKAGE,
        root_module,
        [PackageSource::new(
            SINGLE_PACKAGE,
            Vec::<EcoString>::new(),
            modules,
        )],
    )
}

pub fn compile_typed_package_program(
    root_package: impl Into<EcoString>,
    root_module: impl Into<EcoString>,
    packages: impl IntoIterator<Item = PackageSource>,
) -> Result<TypedProgram, FrontendError> {
    compile_package_sources(
        root_package.into(),
        root_module.into(),
        packages.into_iter().collect(),
    )
}

fn compile_package_sources(
    root_package: EcoString,
    root_module: EcoString,
    packages: Vec<PackageSource>,
) -> Result<TypedProgram, FrontendError> {
    let mut package_names = BTreeSet::new();
    for package in &packages {
        if !package_names.insert(package.package().clone()) {
            return Err(FrontendError::DuplicatePackage {
                package: package.package().clone(),
            });
        }
    }
    if !package_names.contains(&root_package) {
        return Err(FrontendError::MissingRootPackage {
            package: root_package,
        });
    }

    let warnings = WarningEmitter::null();
    let mut parsed_modules = Vec::new();
    for package in packages {
        let (package, direct_dependencies, modules) = package.into_parts();
        for source in modules {
            parsed_modules.push(parse_module(
                package.clone(),
                direct_dependencies.clone(),
                source,
                &warnings,
            )?);
        }
    }

    compile_parsed_package_program(root_package, root_module, parsed_modules, warnings)
}

pub(super) fn parse_module(
    package: EcoString,
    direct_dependencies: Box<[EcoString]>,
    source: ModuleSource,
    warnings: &WarningEmitter,
) -> Result<ParsedModule, FrontendError> {
    let (module_name, path, source) = source.into_parts();
    let parsed = parse::parse_module(path.clone(), &source, warnings).map_err(|error| {
        FrontendError::Parse {
            path: path.clone(),
            error: Box::new(error),
        }
    })?;
    let mut module = parsed.module;
    module.name = module_name;
    Ok(ParsedModule {
        package,
        direct_dependencies,
        path,
        source,
        module,
    })
}

pub(super) fn compile_parsed_package_program(
    root_package: EcoString,
    root_module: EcoString,
    mut parsed_modules: Vec<ParsedModule>,
    warnings: WarningEmitter,
) -> Result<TypedProgram, FrontendError> {
    let mut module_owners = BTreeMap::new();
    for parsed in &parsed_modules {
        if let Some((first_package, first_path)) = module_owners.insert(
            parsed.module.name.clone(),
            (parsed.package.clone(), parsed.path.clone()),
        ) {
            return Err(FrontendError::DuplicateModule {
                module: parsed.module.name.clone(),
                first_package,
                first_path,
                second_package: parsed.package.clone(),
                second_path: parsed.path.clone(),
            });
        }
    }

    if !parsed_modules
        .iter()
        .any(|module| module.package == root_package && module.module.name == root_module)
    {
        return Err(FrontendError::MissingRootModule {
            package: root_package,
            module: root_module,
        });
    }

    let order = dependency_order(&parsed_modules)?;
    let positions = order
        .iter()
        .enumerate()
        .map(|(index, module)| (module.clone(), index))
        .collect::<BTreeMap<_, _>>();
    parsed_modules.sort_by_key(|module| positions[&module.module.name]);

    let root_index = order
        .iter()
        .take_while(|module| *module != &root_module)
        .count();
    let ids = UniqueIdGenerator::new();
    let mut importable_modules = ImHashMap::new();
    importable_modules.insert(PRELUDE_MODULE_NAME.into(), build_prelude(&ids));
    let dev_dependencies = HashSet::new();
    let mut typed_modules = Vec::with_capacity(order.len());

    for parsed in parsed_modules {
        let direct_dependencies = parsed
            .direct_dependencies
            .iter()
            .cloned()
            .map(|package| (package, ()))
            .collect::<HashMap<_, _>>();
        let config = PackageConfig {
            name: parsed.package,
            ..PackageConfig::default()
        };
        let path = parsed.path;
        let source = parsed.source;
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
        root_package,
        root_module,
        root_index,
        modules: typed_modules,
    })
}

pub(super) struct ParsedModule {
    pub(super) package: EcoString,
    pub(super) direct_dependencies: Box<[EcoString]>,
    pub(super) path: Utf8PathBuf,
    pub(super) source: String,
    pub(super) module: UntypedModule,
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
        .map(|module| module.module.name.clone())
        .collect::<BTreeSet<_>>();
    let package_modules = modules.iter().fold(
        BTreeMap::<EcoString, BTreeSet<EcoString>>::new(),
        |mut packages, module| {
            packages
                .entry(module.package.clone())
                .or_default()
                .insert(module.module.name.clone());
            packages
        },
    );
    let dependencies = modules
        .iter()
        .map(|module| {
            let mut internal = module
                .module
                .dependencies(Target::Erlang)
                .into_iter()
                .map(|(dependency, _)| dependency)
                .filter(|dependency| supplied.contains(dependency))
                .collect::<BTreeSet<_>>();
            for package in &module.direct_dependencies {
                if let Some(dependencies) = package_modules.get(package) {
                    internal.extend(dependencies.iter().cloned());
                }
            }
            (module.module.name.clone(), internal)
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
    use super::{
        ModuleSource, PackageSource, compile_typed_module, compile_typed_package_program,
        compile_typed_program,
    };
    use ecow::EcoString;

    #[test]
    fn compiles_single_modules_through_the_default_package() {
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
        assert_eq!(module.type_info.package, "geam");
        assert_eq!(module.definitions.functions.len(), 1);
        assert!(module.type_info.values.contains_key("main"));
    }

    #[test]
    fn preserves_the_single_package_program_surface() {
        let program = compile_typed_program(
            "main",
            [
                ModuleSource::new(
                    "main",
                    "main.gleam",
                    "import support\npub fn main() { support.value() }",
                ),
                ModuleSource::new("unrelated", "unrelated.gleam", "pub fn value() { 2 }"),
                ModuleSource::new("support", "support.gleam", "pub fn value() { 1 }"),
            ],
        )
        .expect("program should compile");

        assert_eq!(program.root_package(), "geam");
        assert_eq!(program.root_module(), "main");
        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["support", "main", "unrelated"],
        );
    }

    #[test]
    fn compiles_qualified_and_unqualified_cross_package_imports() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import support.{identity}

pub fn main() {
  #(support.answer(), identity(2))
}
"#,
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        r#"
pub fn answer() {
  1
}

pub fn identity(value) {
  value
}
"#,
                    )],
                ),
            ],
        )
        .expect("package program should compile");

        assert_eq!(program.root_package(), "application");
        assert_eq!(
            program
                .modules()
                .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
                .collect::<Vec<_>>(),
            [("library", "support"), ("application", "main")],
        );
    }

    #[test]
    fn orders_all_direct_dependency_modules_before_root_package_modules() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [
                        ModuleSource::new("zebra", "zebra.gleam", "pub fn value() { 1 }"),
                        ModuleSource::new("alpha", "alpha.gleam", "pub fn value() { 2 }"),
                    ],
                ),
            ],
        )
        .expect("package program should compile");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["alpha", "zebra", "main"],
        );
    }

    #[test]
    fn accepts_supplied_direct_dependency_packages_without_modules() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["metadata_only"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "metadata_only",
                    Vec::<EcoString>::new(),
                    Vec::<ModuleSource>::new(),
                ),
            ],
        )
        .expect("empty dependency package should not add a module edge");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["main"],
        );
    }

    #[test]
    fn keeps_same_item_names_independent_across_packages() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["first", "second"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import first/value as first
import second/value as second

pub fn main() {
  #(first.answer(), second.answer())
}
"#,
                    )],
                ),
                PackageSource::new(
                    "first",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "first/value",
                        "first/value.gleam",
                        "pub fn answer() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "second",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "second/value",
                        "second/value.gleam",
                        "pub fn answer() { 2 }",
                    )],
                ),
            ],
        )
        .expect("same item names should remain module-qualified");

        assert_eq!(program.modules().len(), 3);
    }

    #[test]
    fn preserves_declared_dependency_identity_on_imports() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "import support\npub fn main() { support.answer() }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        "pub fn answer() { 42 }",
                    )],
                ),
            ],
        )
        .expect("declared package dependency should compile");

        let root = program
            .modules()
            .find(|module| module.name == "main")
            .expect("root module should be present");
        assert_eq!(root.definitions.imports[0].package, "library");
    }

    #[test]
    fn validates_every_supplied_package_module() {
        let error = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [
                        ModuleSource::new("support", "support.gleam", "pub fn answer() { 42 }"),
                        ModuleSource::new(
                            "broken",
                            "broken.gleam",
                            "pub fn broken() { 1 + \"bad\" }",
                        ),
                    ],
                ),
            ],
        )
        .expect_err("unused package modules should still be analysed");

        assert_eq!(error.to_string(), "failed to analyse Gleam module");
    }

    #[test]
    fn rejects_parse_and_analysis_failures_separately() {
        let parse = compile_typed_module("main", "main.gleam", "pub fn main(")
            .expect_err("invalid syntax should fail");
        let analyse = compile_typed_module("main", "main.gleam", "pub fn main() { 1 + \"bad\" }")
            .expect_err("invalid types should fail");

        assert_eq!(parse.to_string(), "failed to parse Gleam module main.gleam");
        assert_eq!(analyse.to_string(), "failed to analyse Gleam module");
    }

    #[test]
    fn rejects_duplicate_packages_before_parsing_sources() {
        let error = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new("main", "main.gleam", "pub fn main(")],
                ),
                PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    Vec::<ModuleSource>::new(),
                ),
            ],
        )
        .expect_err("duplicate package should fail first");

        assert_eq!(
            format!("{error:?}"),
            "DuplicatePackage { package: \"application\" }",
        );
    }

    #[test]
    fn rejects_missing_root_package_before_parsing_sources() {
        let error = compile_typed_package_program(
            "application",
            "main",
            [PackageSource::new(
                "library",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "support",
                    "support.gleam",
                    "pub fn value(",
                )],
            )],
        )
        .expect_err("missing root package should fail first");

        assert_eq!(
            format!("{error:?}"),
            "MissingRootPackage { package: \"application\" }",
        );
    }

    #[test]
    fn rejects_duplicate_module_names_with_both_package_owners() {
        let error = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "first.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "main",
                        "second.gleam",
                        "pub fn other() { 2 }",
                    )],
                ),
            ],
        )
        .expect_err("duplicate module should fail");

        assert_eq!(
            format!("{error:?}"),
            "DuplicateModule { module: \"main\", first_package: \"application\", first_path: \"first.gleam\", second_package: \"library\", second_path: \"second.gleam\" }",
        );
    }

    #[test]
    fn rejects_missing_root_module_after_parsing_sources() {
        let error = compile_typed_package_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "support",
                    "support.gleam",
                    "pub fn value() { 1 }",
                )],
            )],
        )
        .expect_err("missing root module should fail");

        assert_eq!(
            format!("{error:?}"),
            "MissingRootModule { package: \"application\", module: \"main\" }",
        );
    }

    #[test]
    fn rejects_import_cycles_with_a_non_empty_module_path() {
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

        assert_eq!(
            format!("{error:?}"),
            "ImportCycle { modules: [\"one\", \"two\", \"one\"] }",
        );
    }

    #[test]
    fn leaves_unknown_imports_to_gleam_analysis() {
        let error = compile_typed_program(
            "main",
            [ModuleSource::new(
                "main",
                "main.gleam",
                "import unknown\npub fn main() { 1 }",
            )],
        )
        .expect_err("unknown import should fail in analysis");

        assert_eq!(error.to_string(), "failed to analyse Gleam module");
    }
}
