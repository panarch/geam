use geam_core::compile_typed_project;
use gleam_compiler_core::type_::printer::Printer;
use std::collections::BTreeSet;
use std::fmt::Write;

#[test]
fn official_public_surface_and_every_private_external_are_pinned() {
    let program = compile_typed_project(super::project_root(), "native_values").unwrap();
    let mut modules = program
        .modules()
        .filter(|module| module.type_info.package == "gleam_erlang")
        .collect::<Vec<_>>();
    modules.sort_by(|a, b| a.name.cmp(&b.name));
    let mut snapshot = String::new();
    let mut externals = BTreeSet::new();
    for module in &modules {
        writeln!(snapshot, "# {}", module.name).unwrap();
        let mut declarations = Vec::new();
        for (name, type_) in &module.type_info.types {
            if type_.publicity.is_public() {
                let mut printer = Printer::new_without_type_variables(&module.names);
                declarations.push(format!("type {name}: {}", printer.print_type(&type_.type_)));
            }
        }
        for (name, value) in &module.type_info.values {
            if value.publicity.is_public() {
                let mut printer = Printer::new_without_type_variables(&module.names);
                declarations.push(format!(
                    "value {name}: {}",
                    printer.print_type(&value.type_)
                ));
            }
        }
        for function in &module.definitions.functions {
            let (_, name) = function.name.as_ref().unwrap();
            let mut printer = Printer::new_without_type_variables(&module.names);
            let arguments = function
                .arguments
                .iter()
                .map(|argument| {
                    let type_ = printer.print_type(&argument.type_);
                    match argument.names.get_label() {
                        Some(label) => format!("{label}: {type_}"),
                        None => type_.to_string(),
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let signature = format!(
                "{name}: fn({arguments}) -> {}",
                printer.print_type(&function.return_type)
            );
            if function.publicity.is_public() {
                declarations.push(format!("function {signature}"));
            }
            if let Some((native_module, native_function, _)) = &function.external_erlang
                && function.body.is_empty()
            {
                declarations.push(format!(
                    "external {signature} = {native_module}:{native_function}"
                ));
                externals.insert((module.name.to_string(), name.to_string()));
            }
        }
        declarations.sort();
        for declaration in declarations {
            writeln!(snapshot, "{declaration}").unwrap();
        }
        snapshot.push('\n');
    }
    assert_eq!(modules.len(), 7);
    assert_eq!(externals.len(), 48);
    let providers = geam_erlang::host_providers::<geam_erlang::GleamErlangProfile>().unwrap();
    let registered = providers
        .iter()
        .flat_map(|provider| {
            provider
                .functions()
                .map(|function| (provider.module().to_string(), function.name().to_string()))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(registered, externals);
    assert_eq!(
        snapshot.trim(),
        include_str!("../fixtures/gleam_erlang-1.3.0.surface").trim()
    );
}
