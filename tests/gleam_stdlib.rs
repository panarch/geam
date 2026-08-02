use camino::{Utf8Path, Utf8PathBuf};
use geam::{
    ExecutionPlan, HostProfile, HostProviderSet, HostedExecution, TypedProgram, Value,
    compile_typed_host_project, compile_typed_project, plan_host_program, plan_program, run_main,
};
use gleam_core::type_::printer::Printer;

#[path = "gleam_stdlib/gleam_bool.rs"]
mod gleam_bool;
#[path = "gleam_stdlib/gleam_dict.rs"]
mod gleam_dict;
#[path = "gleam_stdlib/gleam_dynamic.rs"]
mod gleam_dynamic;
#[path = "gleam_stdlib/gleam_float.rs"]
mod gleam_float;
#[path = "gleam_stdlib/gleam_int.rs"]
mod gleam_int;
#[path = "gleam_stdlib/gleam_list.rs"]
mod gleam_list;
#[path = "gleam_stdlib/gleam_option.rs"]
mod gleam_option;
#[path = "gleam_stdlib/gleam_order.rs"]
mod gleam_order;
#[path = "gleam_stdlib/gleam_string_tree.rs"]
mod gleam_string_tree;

struct ExpectedSurface {
    values: &'static [&'static str],
    types: &'static [(&'static str, usize)],
    type_aliases: &'static [&'static str],
    constructors: &'static [(&'static str, &'static str, usize)],
    functions: &'static str,
}

fn assert_surface(
    root_module: &str,
    dependency_module: &str,
    dependency_modules: &[&str],
    expected: &ExpectedSurface,
) {
    let program = compile_fixture(root_module, dependency_modules);
    let module = program
        .modules()
        .find(|module| {
            module.type_info.package == "gleam_stdlib" && module.name == dependency_module
        })
        .expect("stdlib dependency module should be loaded");

    let mut values = module
        .type_info
        .values
        .iter()
        .filter(|(_, value)| value.publicity.is_public())
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    values.sort_unstable();
    assert_eq!(values.as_slice(), expected.values);

    let mut types = module
        .type_info
        .types
        .iter()
        .filter(|(_, type_)| type_.publicity.is_public())
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    types.sort_unstable();
    assert_eq!(
        types,
        expected
            .types
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
    );

    let mut type_aliases = module
        .type_info
        .type_aliases
        .keys()
        .map(|name| name.as_str())
        .collect::<Vec<_>>();
    type_aliases.sort_unstable();
    assert_eq!(type_aliases.as_slice(), expected.type_aliases);

    let mut custom_types = module
        .definitions
        .custom_types
        .iter()
        .filter(|type_| type_.publicity.is_public())
        .map(|type_| (type_.name.as_str(), type_.parameters.len()))
        .collect::<Vec<_>>();
    custom_types.sort_unstable();
    assert_eq!(custom_types.as_slice(), expected.types);

    let mut constructors = module
        .definitions
        .custom_types
        .iter()
        .filter(|type_| type_.publicity.is_public())
        .flat_map(|type_| {
            type_.constructors.iter().map(|constructor| {
                (
                    type_.name.as_str(),
                    constructor.name.as_str(),
                    constructor.arguments.len(),
                )
            })
        })
        .collect::<Vec<_>>();
    constructors.sort_unstable();
    assert_eq!(constructors.as_slice(), expected.constructors);

    let mut functions = module
        .definitions
        .functions
        .iter()
        .filter(|function| function.publicity.is_public())
        .map(|function| {
            let (_, name) = function
                .name
                .as_ref()
                .expect("public module function should have a name");
            let mut printer = Printer::new_without_type_variables(&module.names);
            let mut signature = String::from(name.as_str());
            signature.push_str(": fn(");

            for (index, argument) in function.arguments.iter().enumerate() {
                if index > 0 {
                    signature.push_str(", ");
                }
                if let Some(label) = argument.names.get_label() {
                    signature.push_str(label);
                    signature.push_str(": ");
                }
                signature.push_str(&printer.print_type(&argument.type_));
            }

            signature.push_str(") -> ");
            signature.push_str(&printer.print_type(&function.return_type));
            signature
        })
        .collect::<Vec<_>>();
    functions.sort_unstable();
    assert_eq!(functions.join("\n"), expected.functions.trim());
}

fn run_fixture(root_module: &str, dependency_modules: &[&str]) -> Value {
    let source = std::fs::read_to_string(
        project_root()
            .join("src")
            .join(root_module)
            .with_extension("gleam"),
    )
    .expect("stdlib fixture source should be readable");
    let expected = source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("stdlib fixture should not be empty")
        .trim()
        .strip_prefix("// @geam:expect ")
        .expect("last non-empty stdlib fixture line should contain `// @geam:expect`");
    let program = compile_fixture(root_module, dependency_modules);
    let module_plan = plan_program(program).expect("stdlib fixture should plan");

    assert_eq!(
        module_plan
            .modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        expected_module_order(root_module, dependency_modules),
    );

    let plan = ExecutionPlan::from_module_plan(module_plan);
    let actual = run_main(&plan, &mut Vec::new()).expect("stdlib fixture should run");

    assert_eq!(actual.inspect().to_string(), expected);

    actual
}

fn run_hosted_fixture<Profile: HostProfile>(
    root_module: &str,
    dependency_modules: &[&str],
    hosts: HostProviderSet<Profile>,
    state: &mut Profile::RunState,
) -> Value {
    let root_path = project_root()
        .join("src")
        .join(root_module)
        .with_extension("gleam");
    let root_source = std::fs::read_to_string(&root_path)
        .expect("hosted stdlib fixture source should be readable");
    let expected = root_source
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("hosted stdlib fixture should not be empty")
        .trim()
        .strip_prefix("// @geam:expect ")
        .expect("last non-empty hosted stdlib fixture line should contain `// @geam:expect`")
        .to_string();
    let typed = compile_typed_host_project(project_root(), root_module, hosts)
        .expect("resolved hosted stdlib fixture should compile");
    let module_plan = plan_host_program(typed).expect("hosted stdlib fixture should plan");

    assert_eq!(
        module_plan
            .modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        expected_module_order(root_module, dependency_modules),
    );

    let execution = HostedExecution::try_from_module_plan(module_plan)
        .expect("hosted stdlib fixture should seal");
    let actual = execution
        .run_main(state, &mut Vec::new())
        .expect("hosted stdlib fixture should run");

    assert_eq!(actual.inspect().to_string(), expected);

    actual
}

fn compile_fixture(root_module: &str, dependency_modules: &[&str]) -> TypedProgram {
    let program =
        compile_typed_project(project_root(), root_module).expect("stdlib fixture should compile");

    assert_eq!(
        program
            .modules()
            .map(|module| (module.type_info.package.as_str(), module.name.as_str()))
            .collect::<Vec<_>>(),
        expected_module_order(root_module, dependency_modules),
    );

    program
}

fn expected_module_order<'name>(
    root_module: &'name str,
    dependency_modules: &'name [&'name str],
) -> Vec<(&'name str, &'name str)> {
    dependency_modules
        .iter()
        .map(|module| ("gleam_stdlib", *module))
        .chain([("geam_stdlib_test", root_module)])
        .collect()
}

fn project_root() -> Utf8PathBuf {
    Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projects/gleam_stdlib")
}
