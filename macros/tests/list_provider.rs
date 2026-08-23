use ecow::EcoString;
use geam_core::BitArrayValue;
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentRegistration, HostProviderSet, HostedExecution, ModuleSource,
    PackageSource, PlanError, Value, ValueType, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;
use std::sync::atomic::{AtomicUsize, Ordering};

static PAYLOAD_CLONES: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RunState {
    selections: usize,
}

#[geam_macros::provider(
    package = "lists",
    state = RunState,
    modules = [lists],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "lists", crate_path = geam_core)]
mod lists {
    use super::{BigInt, BitArrayValue, EcoString, Ordering, PAYLOAD_CLONES, RunState};

    #[geam_macros::external(name = "Tag")]
    #[derive(PartialEq, Eq, Hash)]
    struct Tag(EcoString);

    impl Clone for Tag {
        fn clone(&self) -> Self {
            PAYLOAD_CLONES.fetch_add(1, Ordering::SeqCst);
            Self(self.0.clone())
        }
    }

    #[geam_macros::function]
    fn length(values: geam_core::List<BigInt>) -> BigInt {
        values.len().into()
    }

    #[geam_macros::function]
    fn first_or(values: geam_core::List<EcoString>, fallback: EcoString) -> EcoString {
        values.get(0).unwrap_or(fallback)
    }

    #[geam_macros::function]
    fn identity(values: geam_core::List<BigInt>) -> geam_core::List<BigInt> {
        values
    }

    #[geam_macros::function]
    fn reverse(values: geam_core::List<EcoString>) -> Vec<EcoString> {
        (0..values.len())
            .rev()
            .map(|index| values.get(index).expect("index comes from the List length"))
            .collect()
    }

    #[geam_macros::function]
    fn labels(values: geam_core::List<(EcoString, BigInt)>) -> Vec<EcoString> {
        (0..values.len())
            .map(|index| {
                let (label, _) = values.get(index).expect("index comes from the List length");
                label
            })
            .collect()
    }

    #[geam_macros::function]
    fn scalar_items_match(values: geam_core::List<(f64, BitArrayValue, char, bool, ())>) -> bool {
        let Some((float, bits, codepoint, bool_, ())) = values.get(0) else {
            return false;
        };
        float == 1.5 && bits == BitArrayValue::from_bytes(vec![1]) && codepoint == 'A' && bool_
    }

    #[geam_macros::function]
    fn nested_items_match(values: geam_core::List<(EcoString, (BigInt, bool))>) -> bool {
        let Some((label, (count, enabled))) = values.get(0) else {
            return false;
        };
        label == "nested" && count == BigInt::from(7) && enabled
    }

    #[geam_macros::function]
    fn tag(label: EcoString) -> Tag {
        Tag(label)
    }

    #[geam_macros::function]
    fn first_tag(values: geam_core::List<Tag>) -> EcoString {
        values
            .get(0)
            .map_or_else(|| "missing".into(), |tag| tag.0.clone())
    }

    #[geam_macros::function]
    fn tags() -> Vec<Tag> {
        vec![Tag("one".into()), Tag("two".into())]
    }

    #[geam_macros::function]
    fn tagged() -> Vec<(EcoString, Tag)> {
        vec![("label".into(), Tag("value".into()))]
    }

    #[geam_macros::function]
    fn choose(
        #[geam_macros::state] state: &mut RunState,
        first: geam_core::List<BigInt>,
        second: geam_core::List<BigInt>,
        choose_second: bool,
    ) -> geam_core::List<BigInt> {
        state.selections += 1;
        if choose_second { second } else { first }
    }

    #[geam_macros::function]
    fn selections(#[geam_macros::state] state: &RunState) -> BigInt {
        state.selections.into()
    }

    #[geam_macros::function]
    fn combined_length(
        #[geam_macros::state] state: &RunState,
        numbers: geam_core::List<BigInt>,
        labels: geam_core::List<EcoString>,
    ) -> BigInt {
        (state.selections + numbers.len() + labels.len()).into()
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

const LIST_SOURCE: &str = r#"
@external(erlang, "macro_lists", "length")
pub fn length(values: List(Int)) -> Int

@external(erlang, "macro_lists", "first_or")
pub fn first_or(values: List(String), fallback: String) -> String

@external(erlang, "macro_lists", "identity")
pub fn identity(values: List(Int)) -> List(Int)

@external(erlang, "macro_lists", "reverse")
pub fn reverse(values: List(String)) -> List(String)

@external(erlang, "macro_lists", "labels")
pub fn labels(values: List(#(String, Int))) -> List(String)

@external(erlang, "macro_lists", "scalar_items_match")
pub fn scalar_items_match(
  values: List(#(Float, BitArray, UtfCodepoint, Bool, Nil)),
) -> Bool

@external(erlang, "macro_lists", "nested_items_match")
pub fn nested_items_match(values: List(#(String, #(Int, Bool)))) -> Bool

@external(erlang, "macro_lists", "Tag")
pub type Tag

@external(erlang, "macro_lists", "tag")
pub fn tag(label: String) -> Tag

@external(erlang, "macro_lists", "first_tag")
pub fn first_tag(values: List(Tag)) -> String

@external(erlang, "macro_lists", "tags")
pub fn tags() -> List(Tag)

@external(erlang, "macro_lists", "tagged")
pub fn tagged() -> List(#(String, Tag))

@external(erlang, "macro_lists", "choose")
pub fn choose(first: List(Int), second: List(Int), choose_second: Bool) -> List(Int)

@external(erlang, "macro_lists", "selections")
pub fn selections() -> Int

@external(erlang, "macro_lists", "combined_length")
pub fn combined_length(numbers: List(Int), labels: List(String)) -> Int

pub fn main() {
  let numbers = [1, 2, 3]
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  #(
    length([]),
    length(numbers),
    first_or([], "fallback"),
    first_or(["first", "second"], "fallback"),
    identity(numbers),
    reverse(["first", "second", "third"]),
    labels([#("alpha", 1), #("beta", 2)]),
    scalar_items_match([#(1.5, <<1>>, codepoint, True, Nil)]),
    nested_items_match([#("nested", #(7, True))]),
    first_tag([tag("selected"), tag("ignored")]),
    tags(),
    tagged(),
    choose([1], [2, 3], True),
    selections(),
    combined_length([1, 2], ["a", "b", "c"]),
  )
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored List component should register")
}

fn execution(source: &str) -> Result<HostedExecution<Profile>, PlanError> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored List module should be unique");
    let typed = compile_typed_host_program(
        "lists",
        "lists",
        [PackageSource::new(
            "lists",
            Vec::<&str>::new(),
            [ModuleSource::new("lists", "src/lists.gleam", source)],
        )],
        hosts,
    )
    .expect("complete List provider source should compile");
    let plan = plan_host_program(typed)?;
    Ok(HostedExecution::try_from_module_plan(plan).expect("matching List provider should seal"))
}

#[test]
fn macro_authored_list_schema_preserves_item_shapes() {
    let providers = providers();
    assert_eq!(providers[0].external_types().count(), 1);
    let functions = providers[0].functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        [
            "length",
            "first_or",
            "identity",
            "reverse",
            "labels",
            "scalar_items_match",
            "nested_items_match",
            "tag",
            "first_tag",
            "tags",
            "tagged",
            "choose",
            "selections",
            "combined_length",
        ],
    );
    assert_eq!(
        functions[0].type_().argument_types(),
        &[ValueType::List(Box::new(ValueType::Int))],
    );
    assert_eq!(functions[0].type_().return_(), &ValueType::Int);
    assert_eq!(
        functions[1].type_().argument_types(),
        &[
            ValueType::List(Box::new(ValueType::String)),
            ValueType::String,
        ],
    );
    assert_eq!(functions[1].type_().return_(), &ValueType::String);
    assert_eq!(
        functions[2].type_().return_(),
        &ValueType::List(Box::new(ValueType::Int)),
    );
    assert_eq!(
        functions[3].type_().return_(),
        &ValueType::List(Box::new(ValueType::String)),
    );
    assert_eq!(
        functions[4].type_().argument_types(),
        &[ValueType::List(Box::new(ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Int,
        ])))],
    );
    assert_eq!(
        functions[5].type_().argument_types(),
        &[ValueType::List(Box::new(ValueType::Tuple(vec![
            ValueType::Float,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Bool,
            ValueType::Nil,
        ])))],
    );
    assert_eq!(
        functions[6].type_().argument_types(),
        &[ValueType::List(Box::new(ValueType::Tuple(vec![
            ValueType::String,
            ValueType::Tuple(vec![ValueType::Int, ValueType::Bool]),
        ])))],
    );
    assert_eq!(functions[6].type_().return_(), &ValueType::Bool);
    let ValueType::External(tag) = functions[7].type_().return_() else {
        panic!("tag should return the declared external type");
    };
    let tag = ValueType::External(tag.clone());
    assert_eq!(
        functions[8].type_().argument_types(),
        &[ValueType::List(Box::new(tag.clone()))],
    );
    assert_eq!(
        functions[9].type_().return_(),
        &ValueType::List(Box::new(tag.clone())),
    );
    assert_eq!(
        functions[10].type_().return_(),
        &ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::String, tag,]))),
    );
    assert_eq!(
        functions[11].type_().argument_types(),
        &[
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Bool,
        ],
    );
    assert_eq!(functions[12].type_().argument_types(), []);
    assert_eq!(
        functions[13].type_().argument_types(),
        &[
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::String)),
        ],
    );
}

#[test]
fn macro_authored_lists_execute_lazily_and_construct_vec_returns() {
    PAYLOAD_CLONES.store(0, Ordering::SeqCst);
    let (value, state_selections) = {
        let execution = execution(LIST_SOURCE).expect("matching List provider should plan");
        let mut state = ProfileState {
            component: RunState::default(),
        };
        let value = execution
            .run_main(&mut state, &mut Vec::new())
            .expect("List provider should execute");
        (value, state.component.selections)
    };
    let Value::Tuple(values) = value else {
        panic!("List provider should return one tuple");
    };
    let [
        empty_length,
        length,
        empty_first,
        first,
        identity,
        reversed,
        labels,
        scalar_items_match,
        nested_items_match,
        selected_tag,
        tags,
        tagged,
        chosen,
        selections,
        combined_length,
    ] = values.as_slice()
    else {
        panic!("List provider should return every List result: {values:?}");
    };
    assert_eq!(empty_length, &Value::Int(0.into()));
    assert_eq!(length, &Value::Int(3.into()));
    assert_eq!(empty_first, &Value::String("fallback".into()));
    assert_eq!(first, &Value::String("first".into()));
    assert_eq!(
        identity,
        &Value::List(geam_core::ListValue::int(vec![
            1.into(),
            2.into(),
            3.into()
        ])),
    );
    assert_eq!(
        reversed,
        &Value::List(geam_core::ListValue::string(vec![
            "third".into(),
            "second".into(),
            "first".into(),
        ])),
    );
    assert_eq!(
        labels,
        &Value::List(geam_core::ListValue::string(vec![
            "alpha".into(),
            "beta".into(),
        ])),
    );
    assert_eq!(scalar_items_match, &Value::Bool(true));
    assert_eq!(nested_items_match, &Value::Bool(true));
    assert_eq!(selected_tag, &Value::String("selected".into()));
    let Value::List(tags) = tags else {
        panic!("external Vec should become a Gleam List");
    };
    assert_eq!(tags.len(), 2);
    assert_eq!(
        Value::List(tags.clone()).inspect().to_string(),
        "[Tag(<opaque>), Tag(<opaque>)]",
    );
    let Value::List(tagged) = tagged else {
        panic!("tuple Vec should become a Gleam List");
    };
    assert_eq!(tagged.len(), 1);
    assert_eq!(
        Value::List(tagged.clone()).inspect().to_string(),
        "[#(\"label\", Tag(<opaque>))]",
    );
    assert_eq!(
        chosen,
        &Value::List(geam_core::ListValue::int(vec![2.into(), 3.into()])),
    );
    assert_eq!(selections, &Value::Int(1.into()));
    assert_eq!(combined_length, &Value::Int(6.into()));
    assert_eq!(state_selections, 1);
    assert_eq!(PAYLOAD_CLONES.load(Ordering::SeqCst), 0);
}

#[test]
fn list_item_mismatch_remains_a_structured_link_error() {
    let mismatched = LIST_SOURCE
        .replace(
            "pub fn length(values: List(Int)) -> Int",
            "pub fn length(values: List(String)) -> Int",
        )
        .replace("length(numbers)", "length([\"one\"])");
    let error = match execution(&mismatched) {
        Err(error) => error,
        Ok(_) => panic!("mismatched List item should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("List mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "lists");
    assert_eq!(module.as_str(), "lists");
    assert_eq!(function.as_str(), "length");
    let geam_core::HostProviderLinkReason::SchemeMismatch {
        expected_type,
        actual_type,
        ..
    } = *reason
    else {
        panic!("List linkage error should preserve the exact scheme mismatch");
    };
    assert_eq!(
        expected_type.argument_types(),
        &[ValueType::List(Box::new(ValueType::String))],
    );
    assert_eq!(
        actual_type.argument_types(),
        &[ValueType::List(Box::new(ValueType::Int))],
    );
}
