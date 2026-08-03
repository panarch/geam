use camino::{Utf8Path, Utf8PathBuf};
use geam::{
    ExecutionPlan, HostProfile, HostProviderSet, HostedExecution, TypedProgram, Value,
    compile_typed_host_project, compile_typed_project, plan_host_program, plan_program, run_main,
};

#[path = "support/upstream_surface.rs"]
mod upstream_surface;

use upstream_surface::{ExpectedSurface, assert_module_surface};

#[path = "gleam_stdlib/gleam_bit_array.rs"]
mod gleam_bit_array;
#[path = "gleam_stdlib/gleam_bool.rs"]
mod gleam_bool;
#[path = "gleam_stdlib/gleam_bytes_tree.rs"]
mod gleam_bytes_tree;
#[path = "gleam_stdlib/gleam_dict.rs"]
mod gleam_dict;
#[path = "gleam_stdlib/gleam_dynamic.rs"]
mod gleam_dynamic;
#[path = "gleam_stdlib/gleam_dynamic_decode.rs"]
mod gleam_dynamic_decode;
#[path = "gleam_stdlib/gleam_float.rs"]
mod gleam_float;
#[path = "gleam_stdlib/gleam_function.rs"]
mod gleam_function;
#[path = "gleam_stdlib/gleam_int.rs"]
mod gleam_int;
#[path = "gleam_stdlib/gleam_io.rs"]
mod gleam_io;
#[path = "gleam_stdlib/gleam_list.rs"]
mod gleam_list;
#[path = "gleam_stdlib/gleam_option.rs"]
mod gleam_option;
#[path = "gleam_stdlib/gleam_order.rs"]
mod gleam_order;
#[path = "gleam_stdlib/gleam_pair.rs"]
mod gleam_pair;
#[path = "gleam_stdlib/gleam_result.rs"]
mod gleam_result;
#[path = "gleam_stdlib/gleam_set.rs"]
mod gleam_set;
#[path = "gleam_stdlib/gleam_string.rs"]
mod gleam_string;
#[path = "gleam_stdlib/gleam_string_tree.rs"]
mod gleam_string_tree;
#[path = "gleam_stdlib/gleam_uri.rs"]
mod gleam_uri;

fn assert_surface(
    root_module: &str,
    dependency_module: &str,
    dependency_modules: &[&str],
    expected: &ExpectedSurface,
) {
    let program = compile_fixture(root_module, dependency_modules);
    assert_module_surface(&program, "gleam_stdlib", dependency_module, expected);
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
