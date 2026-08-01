use super::{FrontendError, ModuleSource, PackageSource};
use crate::host::{
    HostFunctionSchema, HostProfile, HostProviderSet, HostTypeDescriptor,
    RegisteredHostImplementations, RegisteredHostModule, RegisteredHostProviderModule,
};
use camino::Utf8PathBuf;
use ecow::EcoString;
use gleam_core::analyse::{ModuleAnalyzerConstructor, TargetSupport};
use gleam_core::ast::{Publicity, SrcSpan, TypedModule, UntypedModule};
use gleam_core::build::{Origin, Target};
use gleam_core::config::PackageConfig;
use gleam_core::line_numbers::LineNumbers;
use gleam_core::parse;
use gleam_core::type_::error::VariableOrigin;
use gleam_core::type_::{
    Deprecation, ModuleInterface, PRELUDE_MODULE_NAME, References, ValueConstructor,
    ValueConstructorVariant, bit_array, bool as bool_type, build_prelude, float, fn_, generic_var,
    int, list, named, nil, string, tuple, utf_codepoint,
};
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

pub struct HostedTypedProgram<Profile: HostProfile> {
    program: HostedProgram,
    implementations: RegisteredHostImplementations<Profile>,
}

struct HostedProgram {
    root_package: EcoString,
    root_module: EcoString,
    root_index: usize,
    modules: Vec<HostedTypedProgramModule>,
    providers: Vec<RegisteredHostProviderModule>,
}

#[derive(Debug)]
pub(crate) struct TypedProgramModule {
    pub(crate) module: TypedModule,
    pub(crate) path: Utf8PathBuf,
    pub(crate) source: String,
}

pub(crate) enum HostedTypedProgramModule {
    Source(Box<TypedProgramModule>),
    Host(RegisteredHostModule),
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

impl<Profile: HostProfile> HostedTypedProgram<Profile> {
    pub fn root_package(&self) -> &EcoString {
        &self.program.root_package
    }

    pub fn root_module(&self) -> &EcoString {
        &self.program.root_module
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        usize,
        Vec<HostedTypedProgramModule>,
        Vec<RegisteredHostProviderModule>,
        RegisteredHostImplementations<Profile>,
    ) {
        (
            self.program.root_index,
            self.program.modules,
            self.program.providers,
            self.implementations,
        )
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

pub fn compile_typed_host_program<Profile: HostProfile>(
    root_package: impl Into<EcoString>,
    root_module: impl Into<EcoString>,
    packages: impl IntoIterator<Item = PackageSource>,
    hosts: HostProviderSet<Profile>,
) -> Result<HostedTypedProgram<Profile>, FrontendError> {
    let (modules, providers, implementations) = hosts.into_registered();
    compile_host_package_sources(
        root_package.into(),
        root_module.into(),
        packages.into_iter().collect(),
        modules,
        providers,
    )
    .map(|program| HostedTypedProgram {
        program,
        implementations,
    })
}

fn compile_package_sources(
    root_package: EcoString,
    root_module: EcoString,
    packages: Vec<PackageSource>,
) -> Result<TypedProgram, FrontendError> {
    let warnings = WarningEmitter::null();
    let parsed_modules = parse_package_sources(&root_package, packages, &warnings)?;

    compile_parsed_package_program(root_package, root_module, parsed_modules, warnings)
}

fn compile_host_package_sources(
    root_package: EcoString,
    root_module: EcoString,
    packages: Vec<PackageSource>,
    host_modules: Vec<RegisteredHostModule>,
    providers: Vec<RegisteredHostProviderModule>,
) -> Result<HostedProgram, FrontendError> {
    let warnings = WarningEmitter::null();
    let parsed_modules = parse_package_sources(&root_package, packages, &warnings)?;

    compile_parsed_host_program(
        root_package,
        root_module,
        parsed_modules,
        host_modules,
        providers,
        warnings,
    )
}

fn parse_package_sources(
    root_package: &EcoString,
    packages: Vec<PackageSource>,
    warnings: &WarningEmitter,
) -> Result<Vec<ParsedModule>, FrontendError> {
    let mut package_names = BTreeSet::new();
    for package in &packages {
        if !package_names.insert(package.package().clone()) {
            return Err(FrontendError::DuplicatePackage {
                package: package.package().clone(),
            });
        }
    }
    if !package_names.contains(root_package) {
        return Err(FrontendError::MissingRootPackage {
            package: root_package.clone(),
        });
    }

    let mut parsed_modules = Vec::new();
    for package in packages {
        let (package, direct_dependencies, modules) = package.into_parts();
        for source in modules {
            parsed_modules.push(parse_module(
                package.clone(),
                direct_dependencies.clone(),
                source,
                warnings,
            )?);
        }
    }
    Ok(parsed_modules)
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
    source_module_owners(&parsed_modules)?;
    ensure_root_source_module(&parsed_modules, &root_package, &root_module)?;

    let order = dependency_order(&parsed_modules, &[])?;
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
    let mut importable_modules = ImHashMap::<EcoString, ModuleInterface>::new();
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

fn compile_parsed_host_program(
    root_package: EcoString,
    root_module: EcoString,
    mut parsed_modules: Vec<ParsedModule>,
    host_modules: Vec<RegisteredHostModule>,
    providers: Vec<RegisteredHostProviderModule>,
    warnings: WarningEmitter,
) -> Result<HostedProgram, FrontendError> {
    let module_owners = source_module_owners(&parsed_modules)?;
    for host in &host_modules {
        if let Some((source_package, source_path)) = module_owners.get(host.module()) {
            return Err(FrontendError::SourceHostModuleCollision {
                module: host.module().clone(),
                source_package: source_package.clone(),
                source_path: source_path.clone(),
                host_package: host.package().clone(),
            });
        }
    }

    ensure_root_source_module(&parsed_modules, &root_package, &root_module)?;

    let order = dependency_order(&parsed_modules, &host_modules)?;
    let positions = order
        .iter()
        .enumerate()
        .map(|(index, module)| (module.clone(), index))
        .collect::<BTreeMap<_, _>>();
    parsed_modules.sort_by_key(|module| positions[&module.module.name]);
    let mut modules = parsed_modules
        .into_iter()
        .map(|module| HostedParsedModule::Source(Box::new(module)))
        .chain(host_modules.into_iter().map(HostedParsedModule::Host))
        .collect::<Vec<_>>();
    modules.sort_by_key(|module| positions[module.module()]);

    let root_index = order
        .iter()
        .take_while(|module| *module != &root_module)
        .count();
    let ids = UniqueIdGenerator::new();
    let prelude = build_prelude(&ids);
    let mut importable_modules = ImHashMap::<EcoString, ModuleInterface>::new();
    importable_modules.insert(PRELUDE_MODULE_NAME.into(), prelude.clone());
    let dev_dependencies = HashSet::new();
    let mut typed_modules = Vec::with_capacity(order.len());

    for module in modules {
        match module {
            HostedParsedModule::Source(parsed) => {
                let parsed = *parsed;
                let direct_dependencies = parsed
                    .direct_dependencies
                    .iter()
                    .cloned()
                    .map(|package| (package, ()))
                    .collect::<HashMap<_, _>>();
                let visible_modules = importable_modules
                    .iter()
                    .filter(|(name, interface)| {
                        name.as_str() == PRELUDE_MODULE_NAME
                            || interface.package == parsed.package
                            || direct_dependencies.contains_key(&interface.package)
                    })
                    .map(|(name, interface)| (name.clone(), interface.clone()))
                    .collect::<ImHashMap<_, _>>();
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
                    importable_modules: &visible_modules,
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
                typed_modules.push(HostedTypedProgramModule::Source(Box::new(
                    TypedProgramModule {
                        module,
                        path,
                        source,
                    },
                )));
            }
            HostedParsedModule::Host(host) => {
                let interface = host_module_interface(&host, &prelude);
                importable_modules.insert(host.module().clone(), interface);
                typed_modules.push(HostedTypedProgramModule::Host(host));
            }
        }
    }

    Ok(HostedProgram {
        root_package,
        root_module,
        root_index,
        modules: typed_modules,
        providers,
    })
}

fn source_module_owners(
    modules: &[ParsedModule],
) -> Result<BTreeMap<EcoString, (EcoString, Utf8PathBuf)>, FrontendError> {
    let mut owners = BTreeMap::new();
    for parsed in modules {
        if let Some((first_package, first_path)) = owners.insert(
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
    Ok(owners)
}

fn ensure_root_source_module(
    modules: &[ParsedModule],
    root_package: &EcoString,
    root_module: &EcoString,
) -> Result<(), FrontendError> {
    if modules
        .iter()
        .any(|module| &module.package == root_package && &module.module.name == root_module)
    {
        return Ok(());
    }

    Err(FrontendError::MissingRootModule {
        package: root_package.clone(),
        module: root_module.clone(),
    })
}

pub(super) struct ParsedModule {
    pub(super) package: EcoString,
    pub(super) direct_dependencies: Box<[EcoString]>,
    pub(super) path: Utf8PathBuf,
    pub(super) source: String,
    pub(super) module: UntypedModule,
}

enum HostedParsedModule {
    Source(Box<ParsedModule>),
    Host(RegisteredHostModule),
}

impl HostedParsedModule {
    fn module(&self) -> &EcoString {
        match self {
            Self::Source(module) => &module.module.name,
            Self::Host(module) => module.module(),
        }
    }
}

fn host_module_interface(
    module: &RegisteredHostModule,
    prelude: &ModuleInterface,
) -> ModuleInterface {
    let mut interface = prelude.clone();
    interface.name = module.module().clone();
    interface.package = module.package().clone();
    interface.types.clear();
    interface.types_value_constructors.clear();
    interface.values = module
        .functions()
        .map(|schema| host_value_constructor(module.module(), schema))
        .collect();
    interface.accessors.clear();
    interface.line_numbers = LineNumbers::new("");
    interface.src_path = Utf8PathBuf::new();
    interface.warnings.clear();
    interface.type_aliases.clear();
    interface.documentation.clear();
    interface.contains_echo = false;
    interface.references = References::default();
    interface.inline_functions.clear();
    interface
}

fn host_value_constructor(
    module: &EcoString,
    schema: &HostFunctionSchema,
) -> (EcoString, ValueConstructor) {
    let type_ = fn_(
        schema.parameters().iter().map(host_type).collect(),
        host_type(schema.return_type()),
    );
    // Gleam keeps these metadata types crate-private, but exposes their
    // canonical values through ValueConstructor accessors.
    let frontend_metadata = ValueConstructor::local_variable(
        SrcSpan::new(0, 0),
        VariableOrigin::generated(),
        type_.clone(),
    );
    (
        schema.name().clone(),
        ValueConstructor {
            publicity: Publicity::Public,
            deprecation: Deprecation::NotDeprecated,
            variant: ValueConstructorVariant::ModuleFn {
                name: schema.name().clone(),
                field_map: None,
                module: module.clone(),
                arity: schema.parameters().len(),
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: frontend_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: frontend_metadata.called_function_purity(),
            },
            type_,
        },
    )
}

fn host_type(type_: &HostTypeDescriptor) -> std::sync::Arc<gleam_core::type_::Type> {
    match type_ {
        HostTypeDescriptor::Parameter(index) => generic_var(*index as u64),
        HostTypeDescriptor::Int => int(),
        HostTypeDescriptor::Float => float(),
        HostTypeDescriptor::String => string(),
        HostTypeDescriptor::BitArray => bit_array(),
        HostTypeDescriptor::UtfCodepoint => utf_codepoint(),
        HostTypeDescriptor::Bool => bool_type(),
        HostTypeDescriptor::Nil => nil(),
        HostTypeDescriptor::List(item) => list(host_type(item)),
        HostTypeDescriptor::Tuple(elements) => {
            tuple(elements.iter().map(host_type).collect::<Vec<_>>())
        }
        HostTypeDescriptor::Function { arguments, return_ } => fn_(
            arguments.iter().map(host_type).collect(),
            host_type(return_),
        ),
        HostTypeDescriptor::Custom { schema, arguments } => named(
            schema.package(),
            schema.module(),
            schema.name(),
            Publicity::Public,
            arguments.iter().map(host_type).collect(),
        ),
        HostTypeDescriptor::External { schema, arguments } => named(
            schema.package(),
            schema.module(),
            schema.name(),
            Publicity::Public,
            arguments.iter().map(host_type).collect(),
        ),
    }
}

fn dependency_order(
    modules: &[ParsedModule],
    hosts: &[RegisteredHostModule],
) -> Result<Vec<EcoString>, FrontendError> {
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
        .chain(hosts.iter().map(|module| module.module().clone()))
        .collect::<BTreeSet<_>>();
    let package_modules = modules
        .iter()
        .map(|module| (&module.package, &module.module.name))
        .chain(
            hosts
                .iter()
                .map(|module| (module.package(), module.module())),
        )
        .fold(
            BTreeMap::<EcoString, BTreeSet<EcoString>>::new(),
            |mut packages, (package, module)| {
                packages
                    .entry(package.clone())
                    .or_default()
                    .insert(module.clone());
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
        .chain(
            hosts
                .iter()
                .map(|module| (module.module().clone(), BTreeSet::new())),
        )
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
        FrontendError, HostedTypedProgramModule, ModuleSource, PackageSource,
        compile_typed_host_program, compile_typed_module, compile_typed_package_program,
        compile_typed_program, host_module_interface, host_type,
    };
    use crate::host::{HostCustomTypeSchema, HostModule, HostProviderSet, HostTypeDescriptor};
    use crate::plan_host_program;
    use crate::planner::{InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::{Publicity, SrcSpan};
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{
        Deprecation, ValueConstructor, ValueConstructorVariant, bit_array, bool as bool_type,
        build_prelude, float, fn_, generic_var, int, list, named, nil, string, tuple,
        utf_codepoint,
    };
    use gleam_core::uid::UniqueIdGenerator;
    use num_bigint::BigInt;

    #[test]
    fn builds_exact_source_less_host_function_interfaces() {
        let choose = |condition: bool, left: BigInt, right: BigInt| {
            if condition { left } else { right }
        };
        assert_eq!(
            choose(false, BigInt::from(10), BigInt::from(20)),
            BigInt::from(20),
        );
        assert_eq!(
            choose(true, BigInt::from(10), BigInt::from(20)),
            BigInt::from(10),
        );
        let all = |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool| {
            a && b && c && d && e && f && g
        };
        assert!(all(true, true, true, true, true, true, true));

        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("ready", <bool as Default>::default)
            .expect("host function should be valid")
            .with_function("choose", choose)
            .expect("host function should be valid")
            .with_function("all", all)
            .expect("host function should be valid")
            .with_function(
                "consume",
                |_: BigInt,
                 _: f64,
                 _: EcoString,
                 _: crate::BitArrayValue,
                 _: char,
                 _: bool,
                 (): ()| (),
            )
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let (mut modules, providers, _) = hosts.into_registered();
        assert!(providers.is_empty());
        let host = modules.pop().expect("host module should exist");
        let ids = UniqueIdGenerator::new();
        let prelude = build_prelude(&ids);
        let next_id = ids.next();
        let interface = host_module_interface(&host, &prelude);
        let constructor = &interface.values["add"];
        let expected_type = fn_(vec![int(), int()], int());
        let expected_metadata = ValueConstructor::local_variable(
            SrcSpan::new(0, 0),
            VariableOrigin::generated(),
            expected_type.clone(),
        );
        let expected_constructor = ValueConstructor {
            publicity: Publicity::Public,
            deprecation: Deprecation::NotDeprecated,
            variant: ValueConstructorVariant::ModuleFn {
                name: "add".into(),
                field_map: None,
                module: "host/math".into(),
                arity: 2,
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: expected_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: expected_metadata.called_function_purity(),
            },
            type_: expected_type,
        };

        assert_eq!(ids.next(), next_id + 1);
        assert_eq!(interface.name, "host/math");
        assert_eq!(interface.package, "host_support");
        assert_eq!(interface.origin, prelude.origin);
        assert_eq!(
            interface.minimum_required_version,
            prelude.minimum_required_version,
        );
        assert!(!interface.is_internal);
        assert!(interface.types.is_empty());
        assert!(interface.types_value_constructors.is_empty());
        assert_eq!(interface.values.len(), 5);
        assert!(interface.accessors.is_empty());
        assert!(interface.warnings.is_empty());
        assert!(interface.type_aliases.is_empty());
        assert!(interface.documentation.is_empty());
        assert!(!interface.contains_echo);
        assert_eq!(interface.references, Default::default());
        assert!(interface.inline_functions.is_empty());
        assert_eq!(constructor, &expected_constructor);
        assert_eq!(interface.values["ready"].type_, fn_(vec![], bool_type()));
        assert_eq!(
            &interface.values["ready"].variant,
            &ValueConstructorVariant::ModuleFn {
                name: "ready".into(),
                field_map: None,
                module: "host/math".into(),
                arity: 0,
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: expected_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: expected_metadata.called_function_purity(),
            },
        );
        assert_eq!(
            interface.values["choose"].type_,
            fn_(vec![bool_type(), int(), int()], int()),
        );
        assert_eq!(
            &interface.values["choose"].variant,
            &ValueConstructorVariant::ModuleFn {
                name: "choose".into(),
                field_map: None,
                module: "host/math".into(),
                arity: 3,
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: expected_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: expected_metadata.called_function_purity(),
            },
        );
        assert_eq!(
            interface.values["consume"].type_,
            fn_(
                vec![
                    int(),
                    float(),
                    string(),
                    bit_array(),
                    utf_codepoint(),
                    bool_type(),
                    nil(),
                ],
                nil(),
            ),
        );
        assert_eq!(
            &interface.values["consume"].variant,
            &ValueConstructorVariant::ModuleFn {
                name: "consume".into(),
                field_map: None,
                module: "host/math".into(),
                arity: 7,
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: expected_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: expected_metadata.called_function_purity(),
            },
        );
        assert_eq!(
            interface.values["all"].type_,
            fn_(
                vec![
                    bool_type(),
                    bool_type(),
                    bool_type(),
                    bool_type(),
                    bool_type(),
                    bool_type(),
                    bool_type(),
                ],
                bool_type(),
            ),
        );
        assert_eq!(
            &interface.values["all"].variant,
            &ValueConstructorVariant::ModuleFn {
                name: "all".into(),
                field_map: None,
                module: "host/math".into(),
                arity: 7,
                location: SrcSpan::new(0, 0),
                documentation: None,
                implementations: expected_metadata.variant.implementations(),
                external_erlang: None,
                external_javascript: None,
                purity: expected_metadata.called_function_purity(),
            },
        );
        let implementations = constructor.variant.implementations();
        assert!(implementations.gleam);
        assert!(implementations.can_run_on_erlang);
        assert!(implementations.can_run_on_javascript);
        assert!(!implementations.uses_erlang_externals);
        assert!(!implementations.uses_javascript_externals);
        assert_eq!(
            format!("{:?}", constructor.called_function_purity()),
            "Unknown",
        );
    }

    #[test]
    fn maps_recursive_host_types_into_exact_gleam_interface_types() {
        let custom = HostCustomTypeSchema::new("domain", "domain/tree", "Tree", 1, Vec::new());
        let cases = [
            (
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Parameter(0))),
                list(generic_var(0)),
            ),
            (
                HostTypeDescriptor::Tuple(
                    vec![HostTypeDescriptor::Int, HostTypeDescriptor::Bool].into_boxed_slice(),
                ),
                tuple(vec![int(), bool_type()]),
            ),
            (
                HostTypeDescriptor::Custom {
                    schema: custom,
                    arguments: vec![HostTypeDescriptor::String].into_boxed_slice(),
                },
                named(
                    "domain",
                    "domain/tree",
                    "Tree",
                    Publicity::Public,
                    vec![string()],
                ),
            ),
        ];

        for (host, expected) in cases {
            assert_eq!(host_type(&host), expected);
        }
    }

    #[test]
    fn compiles_same_package_host_imports_without_source_bodies() {
        let hosts = HostProviderSet::new([HostModule::new("application", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let program = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import host/math.{add}

pub fn main() {
  math.add(1, add(2, 3))
}
"#,
                )],
            )],
            hosts,
        )
        .expect("host imports should compile");

        assert_eq!(program.root_package(), "application");
        assert_eq!(program.root_module(), "main");
        assert_eq!(
            program
                .program
                .modules
                .iter()
                .map(|module| match module {
                    HostedTypedProgramModule::Source(module) => (
                        module.module.type_info.package.as_str(),
                        module.module.name.as_str(),
                    ),
                    HostedTypedProgramModule::Host(module) => {
                        (module.package().as_str(), module.module().as_str())
                    }
                })
                .collect::<Vec<_>>(),
            [("application", "host/math"), ("application", "main")],
        );
        let root = program
            .program
            .modules
            .iter()
            .find_map(|module| match module {
                HostedTypedProgramModule::Source(module) if module.module.name == "main" => {
                    Some(module)
                }
                HostedTypedProgramModule::Source(_) | HostedTypedProgramModule::Host(_) => None,
            })
            .expect("root source module should exist");
        assert_eq!(root.module.definitions.imports[0].package, "application");
    }

    #[test]
    fn compiles_direct_dependency_host_imports_in_dependency_order() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let program = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "import host/math\npub fn main() { math.add(1, 2) }",
                )],
            )],
            hosts,
        )
        .expect("declared host dependency should compile");

        assert_eq!(
            program
                .program
                .modules
                .iter()
                .map(|module| match module {
                    HostedTypedProgramModule::Source(module) => (
                        module.module.type_info.package.as_str(),
                        module.module.name.as_str(),
                    ),
                    HostedTypedProgramModule::Host(module) => {
                        (module.package().as_str(), module.module().as_str())
                    }
                })
                .collect::<Vec<_>>(),
            [("host_support", "host/math"), ("application", "main")],
        );
    }

    #[test]
    fn orders_host_modules_deterministically_before_dependent_source_modules() {
        let hosts = HostProviderSet::new([
            HostModule::new("host_support", "host/zeta").expect("host module should be valid"),
            HostModule::new("host_support", "host/alpha").expect("host module should be valid"),
        ])
        .expect("host modules should be unique");
        let program = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            hosts,
        )
        .expect("host modules should compile in dependency order");

        assert_eq!(
            program
                .program
                .modules
                .iter()
                .map(|module| match module {
                    HostedTypedProgramModule::Source(module) => (
                        module.module.type_info.package.as_str(),
                        module.module.name.as_str(),
                    ),
                    HostedTypedProgramModule::Host(module) => {
                        (module.package().as_str(), module.module().as_str())
                    }
                })
                .collect::<Vec<_>>(),
            [
                ("host_support", "host/alpha"),
                ("host_support", "host/zeta"),
                ("application", "main"),
            ],
        );
    }

    #[test]
    fn leaves_undeclared_and_unknown_host_imports_to_gleam_analysis() {
        let undeclared_hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let undeclared = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "import host/math\npub fn main() { 1 }",
                )],
            )],
            undeclared_hosts,
        )
        .err()
        .expect("undeclared package should fail analysis");
        let unknown_hosts = HostProviderSet::new(Vec::<HostModule>::new())
            .expect("empty host modules should be valid");
        let unknown = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "import host/math\npub fn main() { 1 }",
                )],
            )],
            unknown_hosts,
        )
        .err()
        .expect("unknown host module should fail analysis");

        assert!(matches!(
            &undeclared,
            FrontendError::Analyse { errors }
                if matches!(
                    errors.as_slice(),
                    [gleam_core::type_::Error::UnknownModule { name, .. }]
                        if name == "host/math"
                )
        ));
        assert!(matches!(
            &unknown,
            FrontendError::Analyse { errors }
                if matches!(
                    errors.as_slice(),
                    [gleam_core::type_::Error::UnknownModule { name, .. }]
                        if name == "host/math"
                )
        ));
    }

    #[test]
    fn rejects_source_and_host_module_identity_collisions() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let error = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["host_support"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "source_support",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "host/math",
                        "host/math.gleam",
                        "pub fn add(left, right) { left + right }",
                    )],
                ),
            ],
            hosts,
        )
        .err()
        .expect("source and host modules should not share an identity");

        assert_eq!(
            format!("{error:?}"),
            "SourceHostModuleCollision { module: \"host/math\", source_package: \"source_support\", source_path: \"host/math.gleam\", host_package: \"host_support\" }",
        );
    }

    #[test]
    fn rejects_host_modules_as_root_source() {
        let hosts = HostProviderSet::new([HostModule::new("application", "main")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let error = compile_typed_host_program(
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
            hosts,
        )
        .err()
        .expect("host module should not satisfy the source root");

        assert_eq!(
            format!("{error:?}"),
            "MissingRootModule { package: \"application\", module: \"main\" }",
        );
    }

    #[test]
    fn hosted_program_preserves_shared_frontend_error_ownership() {
        let duplicate_package = compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    Vec::<ModuleSource>::new(),
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .err()
        .expect("duplicate package should fail");
        assert_eq!(
            format!("{duplicate_package:?}"),
            "DuplicatePackage { package: \"application\" }",
        );

        let duplicate_module = compile_typed_host_program(
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
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .err()
        .expect("duplicate module should fail");
        assert_eq!(
            format!("{duplicate_module:?}"),
            "DuplicateModule { module: \"main\", first_package: \"application\", first_path: \"first.gleam\", second_package: \"library\", second_path: \"second.gleam\" }",
        );

        let missing_root = compile_typed_host_program(
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
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .err()
        .expect("missing root module should fail");
        assert_eq!(
            format!("{missing_root:?}"),
            "MissingRootModule { package: \"application\", module: \"main\" }",
        );

        let import_cycle = compile_typed_host_program(
            "application",
            "one",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
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
            )],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .err()
        .expect("import cycle should fail");
        assert_eq!(
            format!("{import_cycle:?}"),
            "ImportCycle { modules: [\"one\", \"two\", \"one\"] }",
        );
    }

    #[test]
    fn reject_margin_hosted_typed_program_invalid_constant_shape() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let mut program = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "const value = 1\npub fn main() { value }",
                )],
            )],
            hosts,
        )
        .expect("host program should compile");
        let source_module = program
            .program
            .modules
            .iter_mut()
            .find_map(|module| match module {
                HostedTypedProgramModule::Source(module) => Some(module),
                HostedTypedProgramModule::Host(_) => None,
            })
            .expect("source module should exist");
        source_module.module.definitions.constants[0].type_ = gleam_core::type_::generic_var(0);

        assert_eq!(
            plan_host_program(program).err(),
            Some(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

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
