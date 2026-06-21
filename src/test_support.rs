use crate::analyse::{AnalyseError, analyse_module};
use crate::ast::{Publicity, TypedModule, UntypedModule};
use crate::parse::parse_module;
use crate::type_::{ImportableModules, ModuleInterface, ValueConstructor, fn_, nil, string};
use camino::Utf8PathBuf;

pub(crate) fn parse(src: &str) -> UntypedModule {
    parse_module(Utf8PathBuf::from("main.gleam"), src).expect("parse error")
}

pub(crate) fn analyse_result(src: &str) -> Result<TypedModule, AnalyseError> {
    analyse_with(src, ImportableModules::new())
}

pub(crate) fn analyse_with(
    src: &str,
    importable_modules: ImportableModules,
) -> Result<TypedModule, AnalyseError> {
    analyse_module(parse(src), &importable_modules)
}

pub(crate) fn io_imports() -> ImportableModules {
    let mut imports = ImportableModules::new();
    let mut io = ModuleInterface::new("gleam/io");
    io.values.insert(
        "println".into(),
        ValueConstructor::module_fn(Publicity::Public, fn_(vec![string()], nil())),
    );
    imports.insert("gleam/io".into(), io);
    imports
}
