use futures_util::FutureExt;
use geam_core::embedding::{FunctionDeclaration, List, WorkModuleBuilder, with_execution_scope};
use geam_core::frontend::compile_typed_transfer_host_program;
use geam_core::host::{
    AsyncHostComponentProfile, HostExternalSchema, HostFutureStore, HostProfile, HostProvider,
    HostProviderComponent, HostWorkProfile, TransferHostProviderSet,
};
use geam_core::{EchoOutput, ModuleSource, PackageSource};
use geam_runtime_api::embedding::FutureType;
use geam_runtime_api::{FutureComponent, HostFutureSchema};
use num_bigint::BigInt;

struct Profile;
impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = HostFutureStore;
}
impl HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl AsyncHostComponentProfile<FutureComponent> for Profile {
    fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn package_surface_has_one_nominal_type_and_four_public_combinators() {
    let module = geam_core::frontend::compile_typed_module(
        "geam/future",
        "src/geam/future.gleam",
        include_str!("../gleam/src/geam/future.gleam"),
    )
    .expect("official frontend accepts the package source");
    let mut values = module
        .type_info
        .values
        .iter()
        .filter(|(_, value)| value.publicity.is_public())
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    values.sort_unstable();
    assert_eq!(values, ["all", "map", "ready", "then"]);
    let types = module
        .definitions
        .custom_types
        .iter()
        .filter(|type_| type_.publicity.is_public())
        .map(|type_| {
            (
                type_.name.as_str(),
                type_.parameters.len(),
                type_.constructors.len(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(types, [("Future", 1, 0)]);
    assert!(module.type_info.type_aliases.is_empty());
    let mut functions = module
        .definitions
        .functions
        .iter()
        .map(|function| {
            let name = &function.name.as_ref().expect("named package function").1;
            let mut printer =
                gleam_compiler_core::type_::printer::Printer::new_without_type_variables(
                    &module.names,
                );
            let arguments = function
                .arguments
                .iter()
                .map(|argument| printer.print_type(&argument.type_))
                .collect::<Vec<_>>();
            format!(
                "{}: fn({}) -> {}",
                name,
                arguments.join(", "),
                printer.print_type(&function.return_type)
            )
        })
        .collect::<Vec<_>>();
    functions.sort_unstable();
    assert_eq!(
        functions.join("\n"),
        "all: fn(List(Future(a))) -> Future(List(a))\nflatten: fn(Future(Future(a))) -> Future(a)\nmap: fn(Future(a), fn(a) -> b) -> Future(b)\nready: fn(value) -> Future(value)\nthen: fn(Future(a), fn(a) -> Future(b)) -> Future(b)"
    );
}

#[test]
fn ordinary_package_api_preserves_composition_identity_order_and_shared_completion() {
    assert_eq!(FutureComponent::ID, "geam");
    assert_eq!(HostFutureSchema::PACKAGE, "geam");
    assert_eq!(HostFutureSchema::MODULE, "geam/future");
    assert_eq!(HostFutureSchema::NAME, "Future");
    assert_eq!(HostFutureSchema::PARAMETER_COUNT, 1);
    let providers = FutureComponent::providers::<Profile>().expect("built-in registration");
    let source = r#"import geam/future
pub fn composed(value: Int) {
  let original = future.ready(value)
  let alias = original
  let distinct = future.ready(value)
  echo #(original == alias, original == distinct)
  use value <- future.then(original)
  echo value
  future.ready(value * 2)
}
pub fn batch() {
  let shared = future.map(future.ready(7), fn(value) { echo value value + 1 })
  future.all([shared, future.ready(9), shared])
}
pub fn empty() -> future.Future(List(Int)) { future.all([]) }
"#;
    let program = compile_typed_transfer_host_program(
        "application",
        "library",
        [
            PackageSource::new(
                "geam",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "geam/future",
                    "src/geam/future.gleam",
                    include_str!("../gleam/src/geam/future.gleam"),
                )],
            ),
            PackageSource::new(
                "application",
                ["geam"],
                [ModuleSource::new("library", "src/library.gleam", source)],
            ),
        ],
        TransferHostProviderSet::new(providers).expect("static providers"),
    )
    .expect("unchanged ordinary package source");
    let (mut bindings, composed) = WorkModuleBuilder::new(program)
        .expect("plan")
        .function(FunctionDeclaration::<(BigInt,), FutureType<BigInt>>::new(
            "composed",
        ))
        .expect("composition");
    let batch = bindings
        .function(FunctionDeclaration::<(), FutureType<List<BigInt>>>::new(
            "batch",
        ))
        .expect("batch");
    let empty = bindings
        .function(FunctionDeclaration::<(), FutureType<List<BigInt>>>::new(
            "empty",
        ))
        .expect("empty");
    let mut module = bindings.seal().expect("seal all API functions");
    let mut state = ();
    assert!(std::ptr::eq(
        <FutureComponent as HostProvider<Profile>>::project(&mut state),
        &state
    ));
    let mut output = Vec::new();
    let mut echo = |value: EchoOutput| output.push(value.to_string());
    with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let work = scope
            .call(&composed, (21.into(),))
            .expect("construct composition");
        let first = scope.observe(&work).await.expect("first result");
        let again = scope.observe(&work).await.expect("same operation");
        first.read(|a| {
            again.read(|b| {
                assert!(std::ptr::eq(a, b));
                assert_eq!(a, &BigInt::from(42));
            })
        });
        let work = scope.call(&batch, ()).expect("construct ordered batch");
        scope
            .observe(&work)
            .await
            .expect("batch completion")
            .read(|values| {
                assert_eq!(values.len(), 3);
                assert_eq!(
                    (0..3)
                        .map(|i| values.read_item(i, Clone::clone).expect("item"))
                        .collect::<Vec<_>>(),
                    [8.into(), 9.into(), 8.into()]
                );
            });
        let work = scope.call(&empty, ()).expect("empty batch");
        scope
            .observe(&work)
            .await
            .expect("empty completion")
            .read(|values| assert!(values.is_empty()));
    })
    .now_or_never()
    .expect("caller drives all ready work");
    assert_eq!(
        output,
        [
            "src/library.gleam:6\n#(True, False)",
            "src/library.gleam:8\n21",
            "src/library.gleam:12\n7"
        ]
    );
}
