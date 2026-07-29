use ecow::EcoString;
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomConstructorSchema,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema,
    HostCustomSchema, HostCustomType, HostCustomTypeSchema, HostModule, HostProvider,
    HostProviderLinkReason, HostProviderModule, HostProviderSet, HostSchemaType, HostTypeList,
    HostTypeListEnd, HostTypeParameter, HostedExecution, ModuleSource, PackageSource, PlanError,
    StatelessHostProfile, Value, compile_typed_host_program, plan_host_program,
};
#[test]
fn executes_external_gleam_fallback_without_a_provider() {
    let source = r#"
@external(erlang, "host", "increment")
fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  increment(41)
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new(Vec::<HostModule>::new()).expect("empty host set should be valid"),
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("fallback body should plan");
    assert!(plan.modules()[0].functions()[0].gleam_body().is_some());
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

struct SchemaProvider;

impl HostProvider<StatelessHostProfile> for SchemaProvider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

#[test]
fn source_less_host_custom_type_must_have_a_planned_definition() {
    struct MissingSchema;

    impl HostCustomSchema for MissingSchema {
        const PACKAGE: &'static str = "host_support";
        const MODULE: &'static str = "host/custom";
        const NAME: &'static str = "Missing";
        const PARAMETER_COUNT: usize = 0;

        type Constructors = HostCustomConstructorListEnd;
    }

    type Missing = HostCustomType<MissingSchema>;

    fn accept<'call>(
        call: HostCall<'call, StatelessHostProfile, SchemaProvider, bool>,
        _value: HostCustom<'call, Missing>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.return_value(true))
    }

    let host = HostModule::new("host_support", "host/custom")
        .expect("host module should be valid")
        .with_scoped_function::<SchemaProvider, (Missing,), bool, _>("accept", accept)
        .expect("host function should be valid");
    let source = r#"
import host/custom

pub fn main() {
  1
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("host program should compile");

    let Err(PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    }) = plan_host_program(typed)
    else {
        panic!("missing custom type should reject host linkage");
    };
    let HostProviderLinkReason::MissingCustomType { custom_type } = *reason else {
        panic!("missing custom type should preserve its exact reason");
    };

    assert_eq!(package, "host_support");
    assert_eq!(module, "host/custom");
    assert_eq!(function, "accept");
    assert_eq!(custom_type.package(), "host_support");
    assert_eq!(custom_type.module(), "host/custom");
    assert_eq!(custom_type.name(), "Missing");
}

#[test]
fn source_less_host_custom_type_must_be_public_and_non_opaque() {
    struct InternalMarkerDefinition;

    impl HostCustomConstructorDefinition for InternalMarkerDefinition {
        const NAME: &'static str = "Marker";

        type Fields = HostCustomFieldListEnd;
    }

    struct InternalMarkerSchema;

    impl HostCustomSchema for InternalMarkerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/marker";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<InternalMarkerDefinition, HostCustomConstructorListEnd>;
    }

    type InternalMarker = HostCustomType<InternalMarkerSchema>;

    fn accept<'call>(
        call: HostCall<'call, StatelessHostProfile, SchemaProvider, bool>,
        _value: HostCustom<'call, InternalMarker>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.return_value(true))
    }

    let host = HostModule::new("domain", "host/custom")
        .expect("host module should be valid")
        .with_scoped_function::<SchemaProvider, (InternalMarker,), bool, _>("accept", accept)
        .expect("host function should be valid");
    let domain = r#"
@internal
pub type Marker {
  Marker
}
"#;
    let main = r#"
import host/custom

pub fn main() {
  1
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "domain",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "domain/marker",
                    "domain/marker.gleam",
                    domain,
                )],
            ),
            PackageSource::new(
                "application",
                ["domain"],
                [ModuleSource::new("main", "main.gleam", main)],
            ),
        ],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("host program should compile");

    let Err(PlanError::HostProviderLink {
        package,
        module,
        function,
        reason,
    }) = plan_host_program(typed)
    else {
        panic!("internal custom type should not enter a source-less public host surface");
    };
    let HostProviderLinkReason::CustomTypeVisibility { custom_type } = *reason else {
        panic!("internal custom type should preserve its exact visibility reason");
    };

    assert_eq!(package, "domain");
    assert_eq!(module, "host/custom");
    assert_eq!(function, "accept");
    assert_eq!(custom_type.package(), "domain");
    assert_eq!(custom_type.module(), "domain/marker");
    assert_eq!(custom_type.name(), "Marker");
}

#[test]
fn source_provider_custom_schema_must_exactly_match_the_planned_definition() {
    struct MismatchedBoxedSchema;

    struct EnabledField;

    impl HostCustomField for EnabledField {
        const LABEL: Option<&'static str> = Some("enabled");

        type Type = bool;
    }

    struct MismatchedBoxedConstructor;

    impl HostCustomConstructorDefinition for MismatchedBoxedConstructor {
        const NAME: &'static str = "Boxed";

        type Fields = HostCustomFieldList<EnabledField, HostCustomFieldListEnd>;
    }

    impl HostCustomSchema for MismatchedBoxedSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Boxed";
        const PARAMETER_COUNT: usize = 1;

        type Constructors =
            HostCustomConstructorList<MismatchedBoxedConstructor, HostCustomConstructorListEnd>;
    }

    type BoxedArguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
    type Boxed = HostCustomType<MismatchedBoxedSchema, BoxedArguments>;

    fn accept<'call>(
        call: HostCall<'call, StatelessHostProfile, SchemaProvider, bool>,
        _value: HostCustom<'call, Boxed>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.return_value(true))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<SchemaProvider, (Boxed,), bool, _>("accept", accept)
        .expect("provider function should be valid");
    let source = r#"
pub type Boxed(value) {
  Boxed(value: value)
}

@external(erlang, "host", "accept")
fn accept(value: Boxed(value)) -> Bool

pub fn main() {
  accept(Boxed(1))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<EcoString>::new(),
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host program should compile");

    assert_eq!(
        plan_host_program(typed).err(),
        Some(PlanError::HostProviderLink {
            package: "application".into(),
            module: "main".into(),
            function: "accept".into(),
            reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                expected: HostCustomTypeSchema::new(
                    "application",
                    "main",
                    "Boxed",
                    1,
                    [HostCustomConstructorSchema::new(
                        "Boxed",
                        [HostCustomFieldSchema::new(
                            Some("value"),
                            HostSchemaType::parameter(0),
                        )],
                    )],
                ),
                actual: HostCustomTypeSchema::of::<MismatchedBoxedSchema>(),
            }),
        }),
    );
}

#[test]
fn source_provider_validates_custom_schemas_referenced_by_constructor_fields() {
    struct MismatchedInnerField;

    impl HostCustomField for MismatchedInnerField {
        const LABEL: Option<&'static str> = Some("value");

        type Type = bool;
    }

    struct InnerConstructor;

    impl HostCustomConstructorDefinition for InnerConstructor {
        const NAME: &'static str = "Inner";

        type Fields = HostCustomFieldList<MismatchedInnerField, HostCustomFieldListEnd>;
    }

    struct InnerSchema;

    impl HostCustomSchema for InnerSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Inner";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<InnerConstructor, HostCustomConstructorListEnd>;
    }

    struct OuterInnerField;

    impl HostCustomField for OuterInnerField {
        const LABEL: Option<&'static str> = Some("inner");

        type Type = HostCustomType<InnerSchema>;
    }

    struct OuterConstructor;

    impl HostCustomConstructorDefinition for OuterConstructor {
        const NAME: &'static str = "Outer";

        type Fields = HostCustomFieldList<OuterInnerField, HostCustomFieldListEnd>;
    }

    struct OuterSchema;

    impl HostCustomSchema for OuterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Outer";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<OuterConstructor, HostCustomConstructorListEnd>;
    }

    type Outer = HostCustomType<OuterSchema>;

    fn accept<'call>(
        call: HostCall<'call, StatelessHostProfile, SchemaProvider, bool>,
        _value: HostCustom<'call, Outer>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.return_value(true))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<SchemaProvider, (Outer,), bool, _>("accept", accept)
        .expect("provider function should be valid");
    let source = r#"
pub type Inner {
  Inner(value: Int)
}

pub type Outer {
  Outer(inner: Inner)
}

@external(erlang, "host", "accept")
fn accept(value: Outer) -> Bool

pub fn main() {
  accept(Outer(Inner(1)))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<EcoString>::new(),
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host program should compile");

    assert_eq!(
        plan_host_program(typed).err(),
        Some(PlanError::HostProviderLink {
            package: "application".into(),
            module: "main".into(),
            function: "accept".into(),
            reason: Box::new(HostProviderLinkReason::CustomSchemaMismatch {
                expected: HostCustomTypeSchema::new(
                    "application",
                    "main",
                    "Inner",
                    0,
                    [HostCustomConstructorSchema::new(
                        "Inner",
                        [HostCustomFieldSchema::new(
                            Some("value"),
                            HostSchemaType::Int,
                        )],
                    )],
                ),
                actual: HostCustomTypeSchema::of::<InnerSchema>(),
            }),
        }),
    );
}
