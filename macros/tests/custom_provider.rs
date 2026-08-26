use ecow::EcoString;
use geam_core::{
    HostComponentProfile, HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema,
    HostModule, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderLinkReason, HostProviderSet, HostSchemaType, HostedExecution, ModuleSource,
    PackageSource, PlanError, Value, ValueType, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(
    package = "customs",
    modules = [customs],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "customs", crate_path = geam_core)]
mod customs {
    use super::{BigInt, EcoString};

    #[geam_macros::external(name = "Tag")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Tag(EcoString);

    #[geam_macros::custom(input = StatusInput)]
    enum Status {
        Idle,
        Code(BigInt),
        Detail { label: EcoString, enabled: bool },
        Qualified(std::primitive::bool),
    }

    #[geam_macros::custom(input = EnvelopeInput)]
    enum Envelope {
        Wrapped(Status),
        Labels(Vec<EcoString>),
        Tagged(Tag),
        Paired((Status, Tag)),
        Batch(Vec<(Status, Tag)>),
    }

    #[geam_macros::custom]
    enum OutputOnly {
        Ready,
    }

    #[geam_macros::function]
    fn idle() -> Status {
        Status::Idle
    }

    #[geam_macros::function]
    fn code(value: BigInt) -> Status {
        Status::Code(value)
    }

    #[geam_macros::function]
    fn detail(label: EcoString, enabled: bool) -> Status {
        Status::Detail { label, enabled }
    }

    #[geam_macros::function]
    fn qualified(value: bool) -> Status {
        Status::Qualified(value)
    }

    #[geam_macros::function]
    fn status_text(value: StatusInput) -> EcoString {
        match value {
            StatusInput::Idle => "idle".into(),
            StatusInput::Code(value) => format!("code:{value}").into(),
            StatusInput::Detail { label, enabled } => format!("detail:{label}:{enabled}").into(),
            StatusInput::Qualified(value) => format!("qualified:{value}").into(),
        }
    }

    #[geam_macros::function]
    fn wrap(value: BigInt) -> Envelope {
        Envelope::Wrapped(Status::Code(value))
    }

    #[geam_macros::function]
    fn labels(values: geam_core::List<EcoString>) -> Envelope {
        Envelope::Labels(
            (0..values.len())
                .map(|index| values.get(index).expect("index comes from the List length"))
                .collect(),
        )
    }

    #[geam_macros::function]
    fn tag(value: EcoString) -> Envelope {
        Envelope::Tagged(Tag(value))
    }

    #[geam_macros::function]
    fn batch(value: BigInt, tag: EcoString) -> Envelope {
        Envelope::Batch(vec![(Status::Code(value), Tag(tag))])
    }

    #[geam_macros::function]
    fn pair(value: BigInt, tag: EcoString) -> Envelope {
        Envelope::Paired((Status::Code(value), Tag(tag)))
    }

    #[geam_macros::function]
    fn ready() -> OutputOnly {
        OutputOnly::Ready
    }

    #[geam_macros::function]
    fn envelope_text(value: EnvelopeInput) -> EcoString {
        match value {
            EnvelopeInput::Wrapped(StatusInput::Idle) => "wrapped:idle".into(),
            EnvelopeInput::Wrapped(StatusInput::Code(value)) => {
                format!("wrapped:code:{value}").into()
            }
            EnvelopeInput::Wrapped(StatusInput::Detail { label, enabled }) => {
                format!("wrapped:detail:{label}:{enabled}").into()
            }
            EnvelopeInput::Wrapped(StatusInput::Qualified(value)) => {
                format!("wrapped:qualified:{value}").into()
            }
            EnvelopeInput::Labels(values) => {
                let first = values.get(0).unwrap_or_else(|| "empty".into());
                format!("labels:{}:{first}", values.len()).into()
            }
            EnvelopeInput::Tagged(value) => format!("tagged:{}", value.0).into(),
            EnvelopeInput::Paired((status, tag)) => {
                let status = match status {
                    StatusInput::Idle => "idle".into(),
                    StatusInput::Code(value) => value.to_string(),
                    StatusInput::Detail { label, enabled } => format!("{label}:{enabled}"),
                    StatusInput::Qualified(value) => format!("qualified:{value}"),
                };
                format!("paired:{status}:{}", tag.0).into()
            }
            EnvelopeInput::Batch(values) => {
                let Some((status, tag)) = values.get(0) else {
                    return "batch:empty".into();
                };
                let status = match status {
                    StatusInput::Idle => "idle".into(),
                    StatusInput::Code(value) => value.to_string(),
                    StatusInput::Detail { label, enabled } => format!("{label}:{enabled}"),
                    StatusInput::Qualified(value) => format!("qualified:{value}"),
                };
                format!("batch:{status}:{}", tag.0).into()
            }
        }
    }

    #[geam_macros::function]
    fn first_status(values: geam_core::List<StatusInput>) -> EcoString {
        values.get(0).map_or_else(|| "missing".into(), status_text)
    }

    #[geam_macros::function]
    fn first_envelope(values: geam_core::List<EnvelopeInput>) -> EcoString {
        values
            .get(0)
            .map_or_else(|| "missing".into(), envelope_text)
    }

    #[geam_macros::function]
    fn statuses() -> Vec<Status> {
        vec![Status::Idle, Status::Code(17.into())]
    }

    #[geam_macros::function]
    fn pair_text(left: StatusInput, right: StatusInput) -> EcoString {
        format!("{}|{}", status_text(left), status_text(right)).into()
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

const SOURCE: &str = r#"
@external(erlang, "customs", "Tag")
pub type Tag

pub type Status {
  Idle
  Code(Int)
  Detail(label: String, enabled: Bool)
  Qualified(Bool)
}

pub type Envelope {
  Wrapped(Status)
  Labels(List(String))
  Tagged(Tag)
  Paired(#(Status, Tag))
  Batch(List(#(Status, Tag)))
}

pub type OutputOnly {
  Ready
}

@external(erlang, "customs", "idle")
fn idle() -> Status

@external(erlang, "customs", "code")
fn code(value: Int) -> Status

@external(erlang, "customs", "detail")
fn detail(label: String, enabled: Bool) -> Status

@external(erlang, "customs", "qualified")
fn qualified(value: Bool) -> Status

@external(erlang, "customs", "status_text")
fn status_text(value: Status) -> String

@external(erlang, "customs", "wrap")
fn wrap(value: Int) -> Envelope

@external(erlang, "customs", "labels")
fn labels(values: List(String)) -> Envelope

@external(erlang, "customs", "tag")
fn tag(value: String) -> Envelope

@external(erlang, "customs", "batch")
fn batch(value: Int, tag: String) -> Envelope

@external(erlang, "customs", "pair")
fn pair(value: Int, tag: String) -> Envelope

@external(erlang, "customs", "ready")
fn ready() -> OutputOnly

@external(erlang, "customs", "envelope_text")
fn envelope_text(value: Envelope) -> String

@external(erlang, "customs", "first_status")
fn first_status(values: List(Status)) -> String

@external(erlang, "customs", "first_envelope")
fn first_envelope(values: List(Envelope)) -> String

@external(erlang, "customs", "statuses")
fn statuses() -> List(Status)

@external(erlang, "customs", "pair_text")
fn pair_text(left: Status, right: Status) -> String

pub fn main() {
  assert status_text(idle()) == "idle"
  assert status_text(code(7)) == "code:7"
  assert status_text(detail("ready", True)) == "detail:ready:true"
  assert status_text(qualified(True)) == "qualified:true"
  assert envelope_text(wrap(9)) == "wrapped:code:9"
  assert envelope_text(labels([])) == "labels:0:empty"
  assert envelope_text(labels(["first", "second"])) == "labels:2:first"
  assert envelope_text(tag("blue")) == "tagged:blue"
  assert envelope_text(batch(13, "green")) == "batch:13:green"
  assert envelope_text(pair(14, "violet")) == "paired:14:violet"
  assert first_status([]) == "missing"
  assert first_status([code(11)]) == "code:11"
  assert first_status(statuses()) == "idle"
  assert first_envelope([]) == "missing"
  assert first_envelope([labels(["nested"])]) == "labels:1:nested"
  assert pair_text(code(2), detail("ready", True)) == "code:2|detail:ready:true"
  let shared = detail("shared", False)
  assert pair_text(shared, shared) == "detail:shared:false|detail:shared:false"
  assert ready() == Ready
  True
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored custom component should register")
}

#[test]
fn generated_custom_schema_preserves_constructor_field_and_function_order() {
    let providers = providers();
    assert_eq!(providers.len(), 1);
    let provider = &providers[0];
    assert_eq!(provider.package().as_str(), "customs");
    assert_eq!(provider.module().as_str(), "customs");
    let functions = provider.functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        [
            "idle",
            "code",
            "detail",
            "qualified",
            "status_text",
            "wrap",
            "labels",
            "tag",
            "batch",
            "pair",
            "ready",
            "envelope_text",
            "first_status",
            "first_envelope",
            "statuses",
            "pair_text",
        ],
    );
    let status = functions[0].type_().return_().clone();
    let ValueType::Custom(_) = &status else {
        panic!("idle should return the Status custom type")
    };
    let status_schema = geam_core::HostCustomTypeSchema::of::<customs::__GeamCustomSchema0>();
    assert_eq!(status_schema.name(), "Status");
    assert_eq!(status_schema.constructors().len(), 4);
    assert_eq!(status_schema.constructors()[0].name(), "Idle");
    assert_eq!(status_schema.constructors()[1].name(), "Code");
    assert_eq!(status_schema.constructors()[2].name(), "Detail");
    assert_eq!(status_schema.constructors()[3].name(), "Qualified");
    assert_eq!(
        status_schema.constructors()[2].fields()[0].label(),
        Some(&EcoString::from("label")),
    );
    assert_eq!(
        status_schema.constructors()[2].fields()[1].label(),
        Some(&EcoString::from("enabled")),
    );
    assert_eq!(
        functions[4].type_().argument_types(),
        std::slice::from_ref(&status)
    );
    assert_eq!(functions[4].type_().return_(), &ValueType::String);
    assert_eq!(
        functions[12].type_().argument_types(),
        &[ValueType::List(Box::new(status))],
    );
}

#[test]
fn custom_inputs_decode_only_active_values_and_nested_lists_lazily() {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored module should be unique");
    let typed = compile_typed_host_program(
        "customs",
        "customs",
        [PackageSource::new(
            "customs",
            Vec::<&str>::new(),
            [ModuleSource::new("customs", "src/customs.gleam", SOURCE)],
        )],
        hosts,
    )
    .expect("complete custom provider source should compile");
    let plan = plan_host_program(typed).expect("matching custom provider should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("matching custom provider should seal");

    assert_eq!(
        execution.run_main(&mut ProfileState { component: () }, &mut Vec::new(),),
        Ok(Value::Bool(true)),
    );
}

#[test]
fn custom_schema_mismatch_preserves_exact_source_and_generated_context() {
    let mismatched = SOURCE.replace("  Code(Int)", "  Code(String)");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored module should be unique");
    let typed = compile_typed_host_program(
        "customs",
        "customs",
        [PackageSource::new(
            "customs",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "customs",
                "src/customs.gleam",
                mismatched,
            )],
        )],
        hosts,
    )
    .expect("mismatched custom schema should still compile");

    let error = match plan_host_program(typed) {
        Err(error) => error,
        Ok(_) => panic!("mismatched custom schema should fail during linkage"),
    };
    let PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    } = error
    else {
        panic!("custom mismatch should remain a host provider linkage error");
    };
    assert_eq!(package.as_str(), "customs");
    assert_eq!(module.as_str(), "customs");
    assert_eq!(function.as_str(), "idle");
    let HostProviderLinkReason::CustomSchemaMismatch { expected, actual } = *reason else {
        panic!("custom linkage error should preserve the exact schema mismatch");
    };
    assert_eq!(expected, status_schema(HostSchemaType::String));
    assert_eq!(actual, status_schema(HostSchemaType::Int));
}

fn status_schema(code_type: HostSchemaType) -> HostCustomTypeSchema {
    HostCustomTypeSchema::new(
        "customs",
        "customs",
        "Status",
        0,
        [
            HostCustomConstructorSchema::new("Idle", []),
            HostCustomConstructorSchema::new(
                "Code",
                [HostCustomFieldSchema::new(None::<EcoString>, code_type)],
            ),
            HostCustomConstructorSchema::new(
                "Detail",
                [
                    HostCustomFieldSchema::new(Some("label"), HostSchemaType::String),
                    HostCustomFieldSchema::new(Some("enabled"), HostSchemaType::Bool),
                ],
            ),
            HostCustomConstructorSchema::new(
                "Qualified",
                [HostCustomFieldSchema::new(
                    None::<EcoString>,
                    HostSchemaType::Bool,
                )],
            ),
        ],
    )
}
