use super::{
    HostCustomConstructor, HostCustomConstructorAt, HostCustomConstructorBranch,
    HostCustomConstructorDefinition, HostCustomConstructorLeaf, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomConstructorSchema, HostCustomField,
    HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema, HostCustomIndexHere,
    HostCustomIndexLeft, HostCustomIndexRight, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostCustomTypeSchema, HostSchemaType,
};
use crate::embedding::HostPreparation;
use crate::host::type_::custom_constructor_index;
use crate::host::{HostAbiType, HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeSequence};
use crate::{
    HostDeclarations, HostFunctionDeclaration, HostProviderModuleDeclaration, ModuleSource,
    PackageSource,
};
use num_bigint::BigInt;
use std::collections::HashSet;
use std::marker::PhantomData;

struct Outer<Inner>(PhantomData<Inner>);
struct OuterConstructor<Inner>(PhantomData<Inner>);
struct OuterField<Inner>(PhantomData<Inner>);
impl<Inner: HostCustomSchema> HostCustomSchema for Outer<Inner> {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Outer";
    const PARAMETER_COUNT: usize = 0;
    type Constructors =
        HostCustomConstructorList<OuterConstructor<Inner>, HostCustomConstructorListEnd>;
}
impl<Inner: HostCustomSchema> HostCustomConstructorDefinition for OuterConstructor<Inner> {
    const NAME: &'static str = "Outer";
    type Fields = HostCustomFieldList<OuterField<Inner>, HostCustomFieldListEnd>;
}
impl<Inner: HostCustomSchema> HostCustomField for OuterField<Inner> {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomType<Inner>;
}
struct First;
struct Second;
struct FirstConstructor;
struct SecondConstructor;
struct FirstField;
struct SecondField;
impl HostCustomSchema for First {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Inner";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<FirstConstructor, HostCustomConstructorListEnd>;
}
impl HostCustomSchema for Second {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Inner";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<SecondConstructor, HostCustomConstructorListEnd>;
}
impl HostCustomConstructorDefinition for FirstConstructor {
    const NAME: &'static str = "Inner";
    type Fields = HostCustomFieldList<FirstField, HostCustomFieldListEnd>;
}
impl HostCustomConstructorDefinition for SecondConstructor {
    const NAME: &'static str = "Inner";
    type Fields = HostCustomFieldList<SecondField, HostCustomFieldListEnd>;
}
impl HostCustomField for FirstField {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}
impl HostCustomField for SecondField {
    const LABEL: Option<&'static str> = None;
    type Type = geam_core::StringValue;
}

struct Shared;
impl HostCustomSchema for Shared {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Inner";
    const PARAMETER_COUNT: usize = 0;
    const SHARED: bool = true;
    type Constructors = HostCustomConstructorList<FirstConstructor, HostCustomConstructorListEnd>;
}

#[test]
fn balanced_constructors_preserve_metadata_membership_fields_and_nested_schema_visits() {
    struct Schema;
    struct GenericConstructor;
    struct GenericField;
    impl HostCustomSchema for Schema {
        const PACKAGE: &'static str = "app";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Balanced";
        const PARAMETER_COUNT: usize = 1;
        type Constructors = HostCustomConstructorBranch<
            HostCustomConstructorLeaf<OuterConstructor<First>>,
            HostCustomConstructorBranch<
                HostCustomConstructorLeaf<FirstConstructor>,
                HostCustomConstructorLeaf<GenericConstructor>,
            >,
        >;
    }
    impl HostCustomConstructorDefinition for GenericConstructor {
        const NAME: &'static str = "Generic";
        type Fields = HostCustomFieldList<GenericField, HostCustomFieldListEnd>;
    }
    impl HostCustomField for GenericField {
        const LABEL: Option<&'static str> = Some("value");
        type Type = HostCustomTypeArgument<HostTypeIndex0>;
    }
    type Custom = HostCustomType<Schema, HostTypeList<bool, HostTypeListEnd>>;
    type FirstSelection = HostCustomConstructorAt<
        Custom,
        HostCustomIndexLeft<HostCustomIndexHere>,
        OuterConstructor<First>,
    >;
    type MiddleSelection = HostCustomConstructorAt<
        Custom,
        HostCustomIndexRight<HostCustomIndexLeft<HostCustomIndexHere>>,
        FirstConstructor,
    >;
    type LastSelection = HostCustomConstructorAt<
        Custom,
        HostCustomIndexRight<HostCustomIndexRight<HostCustomIndexHere>>,
        GenericConstructor,
    >;

    let schema = HostCustomTypeSchema::of::<Schema>();
    assert_eq!(
        schema,
        HostCustomTypeSchema::new(
            "app",
            "main",
            "Balanced",
            1,
            [
                HostCustomConstructorSchema::new(
                    "Outer",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Custom {
                            package: "app".into(),
                            module: "main".into(),
                            name: "Inner".into(),
                            arguments: Box::new([]),
                        }
                    )]
                ),
                HostCustomConstructorSchema::new(
                    "Inner",
                    [HostCustomFieldSchema::new(
                        None::<&str>,
                        HostSchemaType::Int
                    )]
                ),
                HostCustomConstructorSchema::new(
                    "Generic",
                    [HostCustomFieldSchema::new(
                        Some("value"),
                        HostSchemaType::Parameter(0)
                    )]
                ),
            ]
        )
    );
    assert_eq!(custom_constructor_index::<FirstSelection>(), 0);
    assert_eq!(custom_constructor_index::<MiddleSelection>(), 1);
    assert_eq!(custom_constructor_index::<LastSelection>(), 2);
    type Fields = <LastSelection as HostCustomConstructor>::Fields;
    let fields: <Fields as HostTypeSequence>::Values<'_> = (true, ());
    assert!(fields.0);

    let mut schemas = Vec::new();
    let mut visited = HashSet::new();
    <Custom as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    <Custom as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    assert_eq!(schemas, [schema, HostCustomTypeSchema::of::<First>()]);
    assert_eq!(visited.len(), 2);
}

#[test]
fn recursive_aliases_visit_each_rust_declaration_and_terminate() {
    struct Start;
    struct Loop;
    struct RecursiveConstructor<Child, Next>(PhantomData<(Child, Next)>);
    struct NextField<Next>(PhantomData<Next>);
    impl HostCustomSchema for Start {
        const PACKAGE: &'static str = "app";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Outer";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorList<
            RecursiveConstructor<First, Loop>,
            HostCustomConstructorListEnd,
        >;
    }
    impl HostCustomSchema for Loop {
        const PACKAGE: &'static str = "app";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Outer";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorList<
            RecursiveConstructor<Second, Self>,
            HostCustomConstructorListEnd,
        >;
    }
    impl<Child: HostCustomSchema, Next: HostCustomSchema> HostCustomConstructorDefinition
        for RecursiveConstructor<Child, Next>
    {
        const NAME: &'static str = "Outer";
        type Fields = HostCustomFieldList<
            OuterField<Child>,
            HostCustomFieldList<NextField<Next>, HostCustomFieldListEnd>,
        >;
    }
    impl<Next: HostCustomSchema> HostCustomField for NextField<Next> {
        const LABEL: Option<&'static str> = None;
        type Type = crate::HostListType<HostCustomType<Next>>;
    }
    let mut schemas = Vec::new();
    let mut visited = HashSet::new();
    <HostCustomType<Start> as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    assert_eq!(
        schemas,
        [
            HostCustomTypeSchema::of::<Start>(),
            HostCustomTypeSchema::of::<First>(),
            HostCustomTypeSchema::of::<Second>()
        ]
    );
    assert_eq!(visited.len(), 4);
}

#[test]
fn separate_rust_declarations_cannot_hide_conflicting_nested_layouts() {
    type A = HostCustomType<Outer<First>>;
    type B = HostCustomType<Outer<Second>>;
    let mut schemas = Vec::new();
    let mut visited = HashSet::new();
    <A as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    <B as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    assert_eq!(
        schemas,
        [
            HostCustomTypeSchema::of::<Outer<First>>(),
            HostCustomTypeSchema::of::<First>(),
            HostCustomTypeSchema::of::<Second>()
        ]
    );
    assert_eq!(visited.len(), 4);
    <A as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    assert_eq!(schemas.len(), 3);
    assert_eq!(visited.len(), 4);
    let declarations = HostProviderModuleDeclaration::new("app", "main")
        .unwrap()
        .with_function(HostFunctionDeclaration::<(A, B), bool>::new("check"))
        .unwrap();
    let typed = crate::compile_declared_host_program(
        "app",
        "main",
        [PackageSource::new(
            "app",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "main",
                "main.gleam",
                r#"
pub type Inner { Inner(Int) }
pub type Outer { Outer(Inner) }
@external(erlang, "native", "check")
fn check(first: Outer, second: Outer) -> Bool
pub fn main() { check(Outer(Inner(1)), Outer(Inner(2))) }
"#,
            )],
        )],
        HostDeclarations::from_providers([declarations]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        HostPreparation::new(typed).err(),
        Some(crate::PlanError::HostProviderLink {
            package: "app".into(),
            module: "main".into(),
            function: "check".into(),
            reason: Box::new(crate::HostProviderLinkReason::CustomSchemaMismatch {
                expected: HostCustomTypeSchema::of::<First>(),
                actual: HostCustomTypeSchema::of::<Second>(),
            }),
        })
    );
}

#[test]
fn sharing_requirements_are_collected_through_equal_outer_schemas() {
    type A = HostCustomType<Outer<First>>;
    type B = HostCustomType<Outer<Shared>>;
    let mut schemas = Vec::new();
    let mut visited = HashSet::new();
    <A as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    <B as HostAbiType>::collect_custom_schemas(&mut schemas, &mut visited);
    assert_eq!(
        schemas,
        [
            HostCustomTypeSchema::of::<Outer<First>>(),
            HostCustomTypeSchema::of::<First>(),
            HostCustomTypeSchema::of::<Shared>()
        ]
    );
    let declarations = HostProviderModuleDeclaration::new("app", "main")
        .unwrap()
        .with_function(HostFunctionDeclaration::<(A, B), bool>::new("check"))
        .unwrap();
    let typed = crate::compile_declared_host_program(
        "app",
        "main",
        [PackageSource::new(
            "app",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "main",
                "main.gleam",
                r#"
pub type Inner { Inner(Int) }
pub type Outer { Outer(Inner) }
@external(erlang, "native", "check")
fn check(first: Outer, second: Outer) -> Bool
pub fn main() { check(Outer(Inner(1)), Outer(Inner(2))) }
"#,
            )],
        )],
        HostDeclarations::from_providers([declarations]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        HostPreparation::new(typed).err(),
        Some(crate::PlanError::HostProviderLink {
            package: "app".into(),
            module: "main".into(),
            function: "check".into(),
            reason: Box::new(crate::HostProviderLinkReason::MissingSharedCustomType {
                custom_type: crate::plan::CustomTypeName::new(
                    "app".into(),
                    "main".into(),
                    "Inner".into()
                ),
            }),
        })
    );
}

#[test]
fn captures_collect_their_own_rust_schema_graph() {
    use crate::HostCallableSchema;
    struct Callable;
    impl HostCallableSchema for Callable {
        const PACKAGE: &'static str = "app";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "capture";
        type Arguments = HostTypeList<HostCustomType<Outer<First>>, HostTypeListEnd>;
        type Return = bool;
        type Captures = HostTypeList<HostCustomType<Outer<Second>>, HostTypeListEnd>;
        type Constructions = HostTypeListEnd;
        type Completion = crate::HostReturns;
    }
    let (_, _, callables, _) = HostDeclarations::new([])
        .unwrap()
        .with_callable::<Callable>()
        .unwrap()
        .into_registered();
    assert_eq!(
        callables[0].function.schema().custom_schemas(),
        [
            HostCustomTypeSchema::of::<Outer<First>>(),
            HostCustomTypeSchema::of::<First>(),
            HostCustomTypeSchema::of::<Second>(),
        ]
    );
}
