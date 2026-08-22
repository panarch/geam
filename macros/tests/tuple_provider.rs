use ecow::EcoString;
use geam_core::provider::Configuration;
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration, HostProviderSet,
    HostedExecution, ModuleSource, PackageSource, PlanError, Value, ValueType,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RunState {
    transformations: usize,
}

#[geam_macros::provider(
    package = "tuples",
    state = RunState,
    modules = [tuples],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "tuples", crate_path = geam_core)]
mod tuples {
    use super::{BigInt, EcoString, RunState};

    #[geam_macros::external(name = "Tag")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Tag(EcoString);

    #[geam_macros::function]
    fn make_tagged(label: EcoString) -> (Tag, (EcoString,)) {
        (Tag(label.clone()), (label,))
    }

    #[geam_macros::function]
    fn read_tagged((tag, (label,)): (&Tag, (EcoString,))) -> EcoString {
        format!("{}:{label}", tag.0).into()
    }

    #[geam_macros::function]
    fn reassociate(
        #[geam_macros::state] state: &mut RunState,
        (label, (count, enabled)): (EcoString, (BigInt, bool)),
    ) -> ((EcoString, BigInt), bool) {
        state.transformations += 1;
        ((label, count), enabled)
    }

    #[geam_macros::function]
    fn issued(#[geam_macros::state] state: &RunState) -> (BigInt,) {
        (state.transformations.into(),)
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const TUPLE_SOURCE: &str = r#"
@external(erlang, "macro_tuples", "Tag")
pub type Tag

@external(erlang, "macro_tuples", "make_tagged")
pub fn make_tagged(label: String) -> #(Tag, #(String))

@external(erlang, "macro_tuples", "read_tagged")
pub fn read_tagged(value: #(Tag, #(String))) -> String

@external(erlang, "macro_tuples", "reassociate")
pub fn reassociate(value: #(String, #(Int, Bool))) -> #(#(String, Int), Bool)

@external(erlang, "macro_tuples", "issued")
pub fn issued() -> #(Int)

pub fn main() {
  let tagged = make_tagged("alpha")
  let equivalent = tagged == make_tagged("alpha")
  let regrouped = reassociate(#("alpha", #(7, True)))
  #(tagged, equivalent, read_tagged(tagged), regrouped, issued())
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored tuple component should register")
}

fn execution(source: &str) -> Result<HostedExecution<Profile>, PlanError> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored tuple module should be unique");
    let typed = compile_typed_host_program(
        "tuples",
        "tuples",
        [PackageSource::new(
            "tuples",
            Vec::<&str>::new(),
            [ModuleSource::new("tuples", "src/tuples.gleam", source)],
        )],
        hosts,
    )
    .expect("complete tuple provider source should compile");
    let plan = plan_host_program(typed)?;
    Ok(HostedExecution::try_from_module_plan(plan).expect("matching tuple provider should seal"))
}

#[test]
fn macro_authored_tuple_schema_preserves_nested_and_external_shapes() {
    assert_eq!(
        Component::initialize(&Configuration::empty()),
        Ok(RunState::default())
    );
    let providers = providers();
    assert_eq!(providers.len(), 1);
    let provider = &providers[0];
    assert_eq!(provider.external_types().count(), 1);
    let functions = provider.functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        ["make_tagged", "read_tagged", "reassociate", "issued"],
    );
    let ValueType::Tuple(tagged_elements) = functions[0].type_().return_() else {
        panic!("make_tagged should return a tuple");
    };
    assert!(matches!(tagged_elements[0], ValueType::External(_)));
    let tagged = ValueType::Tuple(tagged_elements.clone());
    assert_eq!(functions[0].type_().argument_types(), &[ValueType::String]);
    assert_eq!(functions[0].type_().return_(), &tagged);
    assert_eq!(functions[1].type_().argument_types(), &[tagged]);
    assert_eq!(functions[1].type_().return_(), &ValueType::String);
    assert_eq!(
        functions[2].type_().argument_types(),
        &[ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Bool]),
        ])],
    );
    assert_eq!(
        functions[2].type_().return_(),
        &ValueType::Tuple(vec![
            ValueType::Tuple(vec![ValueType::String, ValueType::Int]),
            ValueType::Bool,
        ]),
    );
    assert_eq!(functions[3].type_().argument_types(), []);
    assert_eq!(
        functions[3].type_().return_(),
        &ValueType::Tuple(vec![ValueType::Int]),
    );
}

#[test]
fn macro_authored_tuples_execute_with_persistent_external_and_independent_state() {
    let (first, repeated, independent) = {
        let execution = execution(TUPLE_SOURCE).expect("matching tuple provider should plan");
        let mut first_state = ProfileState {
            component: RunState::default(),
        };
        let mut second_state = ProfileState {
            component: RunState::default(),
        };
        let first = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("tuple provider should execute");
        let repeated = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("tuple provider should execute repeatedly");
        let independent = execution
            .run_main(&mut second_state, &mut Vec::new())
            .expect("tuple provider should execute independently");
        assert_eq!(first_state.component.transformations, 2);
        assert_eq!(second_state.component.transformations, 1);
        (first, repeated, independent)
    };

    assert_tuple_result(&first, 1);
    assert_tuple_result(&repeated, 2);
    assert_tuple_result(&independent, 1);
}

fn assert_tuple_result(value: &Value, issued: i64) {
    let Value::Tuple(values) = value else {
        panic!("main should return the tuple result");
    };
    let [
        Value::Tuple(tagged),
        Value::Bool(equivalent),
        Value::String(label),
        Value::Tuple(regrouped),
        Value::Tuple(count),
    ] = values.as_slice()
    else {
        panic!("tuple result should preserve every nested family: {value:?}");
    };
    let [Value::External(tag), Value::Tuple(tagged_label)] = tagged.as_slice() else {
        panic!("tagged tuple should preserve its external and one-tuple: {tagged:?}");
    };
    assert!(equivalent);
    assert_eq!(tag.inspection(), "Tag(<opaque>)");
    assert_eq!(tagged_label, &[Value::String("alpha".into())]);
    assert_eq!(label, "alpha:alpha");
    assert_eq!(
        regrouped,
        &[
            Value::Tuple(vec![Value::String("alpha".into()), Value::Int(7.into())]),
            Value::Bool(true),
        ],
    );
    assert_eq!(count, &[Value::Int(issued.into())]);
}

#[test]
fn tuple_shape_mismatch_remains_a_structured_link_error() {
    let mismatched = TUPLE_SOURCE
        .replace(
            "#(String, #(Int, Bool))) -> #(#(String, Int), Bool)",
            "#(String, #(Bool, Int))) -> #(#(String, Int), Bool)",
        )
        .replace("#(\"alpha\", #(7, True))", "#(\"alpha\", #(True, 7))");
    let error = match execution(&mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched tuple should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("tuple mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "tuples");
    assert_eq!(module.as_str(), "tuples");
    assert_eq!(function.as_str(), "reassociate");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("tuple linkage error should preserve the exact scheme mismatch");
    };
    assert!(expected_scheme.parameters().is_empty());
    assert_eq!(
        expected_type.argument_types(),
        &[ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Bool, ValueType::Int]),
        ])],
    );
    assert!(actual_scheme.parameters().is_empty());
    assert_eq!(
        actual_type.argument_types(),
        &[ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Bool]),
        ])],
    );
    assert_eq!(expected_type.return_(), actual_type.return_());
}

#[test]
fn tuple_arity_mismatch_remains_a_structured_link_error() {
    let mismatched = TUPLE_SOURCE
        .replace(
            "#(String, #(Int, Bool))) -> #(#(String, Int), Bool)",
            "#(String, #(Int, Bool, String))) -> #(#(String, Int), Bool)",
        )
        .replace(
            "#(\"alpha\", #(7, True))",
            "#(\"alpha\", #(7, True, \"extra\"))",
        );
    let error = match execution(&mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched tuple arity should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("tuple arity mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "tuples");
    assert_eq!(module.as_str(), "tuples");
    assert_eq!(function.as_str(), "reassociate");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_scheme,
        expected_type,
        actual_scheme,
        actual_type,
    } = *reason
    else {
        panic!("tuple arity error should preserve the exact scheme mismatch");
    };
    assert!(expected_scheme.parameters().is_empty());
    assert_eq!(
        expected_type.argument_types(),
        &[ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Bool, ValueType::String,]),
        ])],
    );
    assert!(actual_scheme.parameters().is_empty());
    assert_eq!(
        actual_type.argument_types(),
        &[ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Bool]),
        ])],
    );
    assert_eq!(expected_type.return_(), actual_type.return_());
}
