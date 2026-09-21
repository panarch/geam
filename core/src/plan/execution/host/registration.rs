mod callable;
mod schema;
mod type_;

pub use callable::CallableRegistration;
pub use schema::{ConstructorSchema, CustomSchema, ExternalSchema, FieldSchema, SchemaType};
pub use type_::RegistrationType;

use crate::host::{HostFunctionSchema, HostParameter, RegisteredHostConstructions};
use crate::plan::HostFunctionTemplate;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationContract {
    pub parameter_count: usize,
    pub parameters: Table<RegistrationType>,
    pub captures: Table<RegistrationType>,
    pub callable: bool,
    pub callable_constructions: Table<CallableRegistration>,
    pub return_: RegistrationType,
    pub layout: Table<RegistrationParameter>,
    pub custom_schemas: Table<CustomSchema>,
    pub external_schemas: Table<ExternalSchema>,
    pub constructions: Table<RegistrationType>,
    pub construction_customs: Table<CustomSchema>,
    pub construction_externals: Table<ExternalSchema>,
    pub native_rules: Option<Table<RegistrationType>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationParameter {
    Int(usize),
    Float(usize),
    String(usize),
    BitArray(usize),
    UtfCodepoint(usize),
    Bool(usize),
    Nil(usize),
    Value(usize),
    List(usize),
    Tuple(usize),
    Custom(usize),
    External(usize),
    Function { slot: usize, arity: usize },
}

impl RegistrationContract {
    pub(in crate::plan::execution) fn from_template(
        template: &HostFunctionTemplate,
        constructions: &RegisteredHostConstructions,
    ) -> Self {
        Self {
            parameter_count: template.scheme().parameters().len(),
            callable: template.is_callable(),
            callable_constructions: constructions
                .callables()
                .iter()
                .map(CallableRegistration::from_registered)
                .collect(),
            parameters: template
                .parameters()
                .iter()
                .map(RegistrationType::from_descriptor)
                .collect(),
            captures: template
                .captures()
                .iter()
                .map(RegistrationType::from_descriptor)
                .collect(),
            return_: RegistrationType::from_descriptor(template.return_type()),
            layout: template
                .layout()
                .iter()
                .copied()
                .map(RegistrationParameter::from_parameter)
                .collect(),
            custom_schemas: template
                .custom_schemas()
                .iter()
                .map(CustomSchema::from_schema)
                .collect(),
            external_schemas: template
                .external_schemas()
                .iter()
                .map(ExternalSchema::from_schema)
                .collect(),
            constructions: constructions
                .types()
                .iter()
                .map(RegistrationType::from_descriptor)
                .collect(),
            construction_customs: constructions
                .custom_schemas()
                .iter()
                .map(CustomSchema::from_schema)
                .collect(),
            construction_externals: constructions
                .external_schemas()
                .iter()
                .map(ExternalSchema::from_schema)
                .collect(),
            native_rules: constructions.native_rules().map(|rules| {
                rules
                    .iter()
                    .map(RegistrationType::from_descriptor)
                    .collect()
            }),
        }
    }

    pub(in crate::plan::execution) fn matches(
        &self,
        schema: &HostFunctionSchema,
        constructions: &RegisteredHostConstructions,
    ) -> bool {
        self.callable == schema.is_callable()
            && same(
                &self.callable_constructions,
                constructions.callables(),
                CallableRegistration::matches,
            )
            && self.parameter_count == schema.scheme().parameters().len()
            && same(
                &self.parameters,
                schema.parameters(),
                RegistrationType::matches,
            )
            && same(&self.captures, schema.captures(), RegistrationType::matches)
            && self.return_.matches(schema.return_type())
            && self.layout.len() == schema.layout().len()
            && self
                .layout
                .iter()
                .zip(schema.layout())
                .all(|(left, right)| *left == RegistrationParameter::from_parameter(*right))
            && same(
                &self.custom_schemas,
                schema.custom_schemas(),
                CustomSchema::matches,
            )
            && same(
                &self.external_schemas,
                schema.external_schemas(),
                ExternalSchema::matches,
            )
            && same(
                &self.constructions,
                constructions.types(),
                RegistrationType::matches,
            )
            && same(
                &self.construction_customs,
                constructions.custom_schemas(),
                CustomSchema::matches,
            )
            && same(
                &self.construction_externals,
                constructions.external_schemas(),
                ExternalSchema::matches,
            )
            && match (&self.native_rules, constructions.native_rules()) {
                (None, None) => true,
                (Some(left), Some(right)) => same(left, right, RegistrationType::matches),
                _ => false,
            }
    }
}

impl RegistrationParameter {
    pub(in crate::plan::execution) fn from_parameter(parameter: HostParameter) -> Self {
        match parameter {
            HostParameter::Int(slot) => Self::Int(slot.index()),
            HostParameter::Float(slot) => Self::Float(slot.index()),
            HostParameter::String(slot) => Self::String(slot.index()),
            HostParameter::BitArray(slot) => Self::BitArray(slot.index()),
            HostParameter::UtfCodepoint(slot) => Self::UtfCodepoint(slot.index()),
            HostParameter::Bool(slot) => Self::Bool(slot.index()),
            HostParameter::Nil(slot) => Self::Nil(slot.index()),
            HostParameter::Value(slot) => Self::Value(slot.index()),
            HostParameter::List(slot) => Self::List(slot.index()),
            HostParameter::Tuple(slot) => Self::Tuple(slot.index()),
            HostParameter::Custom(slot) => Self::Custom(slot.index()),
            HostParameter::External(slot) => Self::External(slot.index()),
            HostParameter::Function { slot, arity } => Self::Function {
                slot: slot.index(),
                arity,
            },
        }
    }
}

fn same<Left, Right>(
    left: &[Left],
    right: &[Right],
    matches: impl Fn(&Left, &Right) -> bool,
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| matches(left, right))
}

impl Emit for RegistrationContract {
    fn emit(&self, output: &mut Rust) {
        let Self {
            parameter_count,
            parameters,
            captures,
            callable,
            callable_constructions,
            return_,
            layout,
            custom_schemas,
            external_schemas,
            constructions,
            construction_customs,
            construction_externals,
            native_rules,
        } = self;
        output.structure(
            "host::RegistrationContract",
            &[
                ("parameter_count", parameter_count),
                ("parameters", parameters),
                ("captures", captures),
                ("callable", callable),
                ("callable_constructions", callable_constructions),
                ("return_", return_),
                ("layout", layout),
                ("custom_schemas", custom_schemas),
                ("external_schemas", external_schemas),
                ("constructions", constructions),
                ("construction_customs", construction_customs),
                ("construction_externals", construction_externals),
                ("native_rules", native_rules),
            ],
        );
    }
}

impl Emit for RegistrationParameter {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(slot) => output.call("host::RegistrationParameter::Int", &[slot]),
            Self::Float(slot) => output.call("host::RegistrationParameter::Float", &[slot]),
            Self::String(slot) => output.call("host::RegistrationParameter::String", &[slot]),
            Self::BitArray(slot) => output.call("host::RegistrationParameter::BitArray", &[slot]),
            Self::UtfCodepoint(slot) => {
                output.call("host::RegistrationParameter::UtfCodepoint", &[slot])
            }
            Self::Bool(slot) => output.call("host::RegistrationParameter::Bool", &[slot]),
            Self::Nil(slot) => output.call("host::RegistrationParameter::Nil", &[slot]),
            Self::Value(slot) => output.call("host::RegistrationParameter::Value", &[slot]),
            Self::List(slot) => output.call("host::RegistrationParameter::List", &[slot]),
            Self::Tuple(slot) => output.call("host::RegistrationParameter::Tuple", &[slot]),
            Self::Custom(slot) => output.call("host::RegistrationParameter::Custom", &[slot]),
            Self::External(slot) => output.call("host::RegistrationParameter::External", &[slot]),
            Self::Function { slot, arity } => output.structure(
                "host::RegistrationParameter::Function",
                &[("slot", slot), ("arity", arity)],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostFunctionTemplate, RegisteredHostConstructions, RegistrationContract,
        RegistrationParameter, RegistrationType, Rust,
    };
    use crate::host::{HostProviderModule, HostProviderSet, HostTypeDescriptor};
    use crate::plan::{
        FunctionShape, FunctionTemplateId, FunctionTemplateSignature, HostCallSite, SourceSpan,
    };
    use crate::{HostProvider, StatelessHostProfile};
    use num_bigint::BigInt;

    #[test]
    fn emitted_native_contract_keeps_captures_outside_the_argument_layout() {
        use super::{RegistrationContract, RegistrationParameter, RegistrationType};
        use crate::plan::execution::prepared::rust::Rust;
        use crate::plan::execution::storage::Table;

        let contract = RegistrationContract {
            parameter_count: 0,
            parameters: vec![RegistrationType::Int].into(),
            captures: vec![RegistrationType::Bool].into(),
            callable: true,
            callable_constructions: Table::Static(&[]),
            return_: RegistrationType::String,
            layout: vec![RegistrationParameter::Int(0)].into(),
            custom_schemas: Table::Static(&[]),
            external_schemas: Table::Static(&[]),
            constructions: Table::Static(&[]),
            construction_customs: Table::Static(&[]),
            construction_externals: Table::Static(&[]),
            native_rules: None,
        };
        assert_eq!(
            Rust::expression(&contract),
            r#"
data::host::RegistrationContract {
    parameter_count: 0,
    parameters: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
    captures: data::Storage::Static(&[
        data::host::RegistrationType::Bool,
    ]),
    callable: true,
    callable_constructions: data::Storage::Static(&[]),
    return_: data::host::RegistrationType::String,
    layout: data::Storage::Static(&[
        data::host::RegistrationParameter::Int(0),
    ]),
    custom_schemas: data::Storage::Static(&[]),
    external_schemas: data::Storage::Static(&[]),
    constructions: data::Storage::Static(&[]),
    construction_customs: data::Storage::Static(&[]),
    construction_externals: data::Storage::Static(&[]),
    native_rules: None,
}"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_original_parameter_slots_for_every_storage_family() {
        use crate::BitArrayValue;
        use crate::StringValue;
        use crate::host::HostParameterLayout;

        let mut layout = HostParameterLayout::default();
        layout.register::<BigInt>();
        layout.register::<f64>();
        layout.register::<StringValue>();
        layout.register::<BitArrayValue>();
        layout.register::<char>();
        layout.register::<bool>();
        layout.register::<()>();
        layout.register_value_parameter();
        layout.register_list_parameter();
        layout.register_tuple_parameter();
        layout.register_custom_parameter();
        layout.register_external_parameter_slot();
        layout.register_function_parameter_slot(2);
        layout.register::<BigInt>();
        let parameters = layout.finish();
        let expected = [
            (
                RegistrationParameter::Int(0),
                "data::host::RegistrationParameter::Int(0)",
            ),
            (
                RegistrationParameter::Float(0),
                "data::host::RegistrationParameter::Float(0)",
            ),
            (
                RegistrationParameter::String(0),
                "data::host::RegistrationParameter::String(0)",
            ),
            (
                RegistrationParameter::BitArray(0),
                "data::host::RegistrationParameter::BitArray(0)",
            ),
            (
                RegistrationParameter::UtfCodepoint(0),
                "data::host::RegistrationParameter::UtfCodepoint(0)",
            ),
            (
                RegistrationParameter::Bool(0),
                "data::host::RegistrationParameter::Bool(0)",
            ),
            (
                RegistrationParameter::Nil(0),
                "data::host::RegistrationParameter::Nil(0)",
            ),
            (
                RegistrationParameter::Value(0),
                "data::host::RegistrationParameter::Value(0)",
            ),
            (
                RegistrationParameter::List(0),
                "data::host::RegistrationParameter::List(0)",
            ),
            (
                RegistrationParameter::Tuple(0),
                "data::host::RegistrationParameter::Tuple(0)",
            ),
            (
                RegistrationParameter::Custom(0),
                "data::host::RegistrationParameter::Custom(0)",
            ),
            (
                RegistrationParameter::External(0),
                "data::host::RegistrationParameter::External(0)",
            ),
            (
                RegistrationParameter::Function { slot: 0, arity: 2 },
                r#"
data::host::RegistrationParameter::Function {
    slot: 0,
    arity: 2,
}"#
                .trim_start_matches('\n'),
            ),
            (
                RegistrationParameter::Int(1),
                "data::host::RegistrationParameter::Int(1)",
            ),
        ];
        assert_eq!(parameters.len(), expected.len());
        for (parameter, (expected, expression)) in parameters.iter().zip(expected) {
            assert_eq!(RegistrationParameter::from_parameter(*parameter), expected);
            assert_eq!(Rust::expression(&expected), expression);
        }
    }

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn compares_original_layout_and_construction_permissions_not_just_signature() {
        let providers = || {
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "app", "native",
            )
            .unwrap()
            .with_function(
                "choose",
                |first: bool, value: BigInt, second: bool, _: ()| {
                    if first && second {
                        value
                    } else {
                        BigInt::from(0)
                    }
                },
            )
            .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "choose")
fn choose(first: Bool, value: Int, second: Bool, empty: Nil) -> Int
pub fn main() { #(choose(True, 42, True, Nil), choose(False, 42, True, Nil)) }
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "native",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "native",
                    "src/native.gleam",
                    source,
                )],
            )],
            providers(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Int(42.into()),
                crate::Value::Int(0.into())
            ])
        );
        let hosts = providers();
        let (_, mut providers, _, _) = hosts.into_registered();
        let (schema, _, _) = providers.remove(0).functions.remove(0).into_parts();
        let constructions = RegisteredHostConstructions::new(
            Box::new([HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))]),
            Box::new([]),
        );
        let template = HostFunctionTemplate::from_schema(
            FunctionTemplateSignature::new(
                FunctionTemplateId::new(0),
                schema.scheme().clone(),
                FunctionShape::from_function_type(schema.type_().clone()),
            ),
            "app".into(),
            HostCallSite::new("native".into(), "choose".into(), SourceSpan::new(0, 0)),
            schema.clone(),
        );
        let original = RegistrationContract::from_template(&template, &constructions);
        assert_eq!(original.parameter_count, 0);
        assert_eq!(
            original.parameters.as_ref(),
            &[
                RegistrationType::Bool,
                RegistrationType::Int,
                RegistrationType::Bool,
                RegistrationType::Nil,
            ]
        );
        assert_eq!(
            original.layout.as_ref(),
            &[
                RegistrationParameter::Bool(0),
                RegistrationParameter::Int(0),
                RegistrationParameter::Bool(1),
                RegistrationParameter::Nil(0),
            ]
        );
        assert_eq!(original.return_, RegistrationType::Int);
        assert_eq!(
            original.constructions.as_ref(),
            &[RegistrationType::List(
                Box::new(RegistrationType::Int).into(),
            )]
        );
        assert!(original.matches(&schema, &constructions));
        assert!(!original.matches(&schema, &RegisteredHostConstructions::empty()));
        assert!(!original.matches(
            &schema,
            &RegisteredHostConstructions::new(
                Box::new([HostTypeDescriptor::List(Box::new(
                    HostTypeDescriptor::String
                ))]),
                Box::new([]),
            )
        ));
        let mut changed = original.clone();
        changed.parameter_count = 1;
        assert!(!changed.matches(&schema, &constructions));
        changed = original.clone();
        changed.layout = vec![
            RegistrationParameter::Bool(0),
            RegistrationParameter::Int(0),
            RegistrationParameter::Bool(0),
            RegistrationParameter::Nil(0),
        ]
        .into();
        assert!(!changed.matches(&schema, &constructions));
        changed = original.clone();
        changed.parameters = vec![RegistrationType::Int].into();
        assert!(!changed.matches(&schema, &constructions));
        changed = original.clone();
        changed.return_ = RegistrationType::Bool;
        assert!(!changed.matches(&schema, &constructions));
        changed = original.clone();
        changed.native_rules = Some(Vec::new().into());
        assert!(!changed.matches(&schema, &constructions));
    }

    #[test]
    fn native_rule_absence_and_an_empty_registered_rule_set_are_distinct() {
        use crate::host::native::{NativeCall, NativeRules};
        use crate::{HostTypeList, HostTypeListEnd};
        type Targets = HostTypeList<BigInt, HostTypeListEnd>;
        let providers = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "native")
                .unwrap()
                .with_native_function::<Provider, (BigInt,), bool, Targets, _>(
                    "convert",
                    NativeRules::default(),
                    |mut call: NativeCall<'_, StatelessHostProfile, Provider, bool, Targets>, _| {
                        assert_eq!(call.call().state(), &mut ());
                        Ok(call.finish(true))
                    },
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "convert") fn convert(value: Int) -> Bool
pub fn main() { convert(42) }
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "native",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "native",
                    "src/native.gleam",
                    source,
                )],
            )],
            providers(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Bool(true)
        );
        let hosts = providers();
        let (_, mut providers, _, _) = hosts.into_registered();
        let (schema, constructions, _) = providers.remove(0).functions.remove(0).into_parts();
        let template = HostFunctionTemplate::from_schema(
            FunctionTemplateSignature::new(
                FunctionTemplateId::new(0),
                schema.scheme().clone(),
                FunctionShape::from_function_type(schema.type_().clone()),
            ),
            "app".into(),
            HostCallSite::new("native".into(), "convert".into(), SourceSpan::new(0, 0)),
            schema.clone(),
        );
        let original = RegistrationContract::from_template(&template, &constructions);
        assert_eq!(original.native_rules.as_deref(), Some([].as_slice()));
        assert!(original.matches(&schema, &constructions));
        let mut changed = original.clone();
        changed.native_rules = None;
        assert!(!changed.matches(&schema, &constructions));
        changed.native_rules = Some(vec![RegistrationType::String].into());
        assert!(!changed.matches(&schema, &constructions));
        assert_eq!(
            Rust::expression(&original.layout),
            r#"
data::Storage::Static(&[
    data::host::RegistrationParameter::Int(0),
])"#
            .trim_start_matches('\n')
        );
    }
}
