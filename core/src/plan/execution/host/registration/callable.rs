use super::{RegistrationType, same};
use crate::host::{HostFunctionBinding, RegisteredCallableConstruction};
use crate::plan::Text;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallableRegistration {
    pub package: Text,
    pub module: Text,
    pub name: Text,
    pub arguments: Table<RegistrationType>,
    pub captures: Table<RegistrationType>,
    pub return_: RegistrationType,
    pub returns_value: bool,
}

impl CallableRegistration {
    pub(in crate::plan::execution) fn from_registered(
        declaration: &RegisteredCallableConstruction,
    ) -> Self {
        Self {
            package: declaration.identity.package.clone().into(),
            module: declaration.identity.module.clone().into(),
            name: declaration.identity.name.clone().into(),
            arguments: declaration
                .arguments
                .iter()
                .map(RegistrationType::from_descriptor)
                .collect(),
            captures: declaration
                .captures
                .iter()
                .map(RegistrationType::from_descriptor)
                .collect(),
            return_: RegistrationType::from_descriptor(&declaration.return_),
            returns_value: matches!(declaration.completion, HostFunctionBinding::Value(())),
        }
    }

    pub(in crate::plan::execution) fn matches(
        &self,
        declaration: &RegisteredCallableConstruction,
    ) -> bool {
        self.package.as_ref() == declaration.identity.package.as_str()
            && self.module.as_ref() == declaration.identity.module.as_str()
            && self.name.as_ref() == declaration.identity.name.as_str()
            && self.returns_value
                == matches!(declaration.completion, HostFunctionBinding::Value(()))
            && same(
                &self.arguments,
                &declaration.arguments,
                RegistrationType::matches,
            )
            && same(
                &self.captures,
                &declaration.captures,
                RegistrationType::matches,
            )
            && self.return_.matches(&declaration.return_)
    }
}

impl Emit for CallableRegistration {
    fn emit(&self, output: &mut Rust) {
        let Self {
            package,
            module,
            name,
            arguments,
            captures,
            return_,
            returns_value,
        } = self;
        output.structure(
            "host::CallableRegistration",
            &[
                ("package", package),
                ("module", module),
                ("name", name),
                ("arguments", arguments),
                ("captures", captures),
                ("return_", return_),
                ("returns_value", returns_value),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::CallableRegistration;
    use crate::host::RegisteredCallableConstruction;
    use crate::plan::execution::prepared::rust::Rust;
    use crate::{HostCallableSchema, HostDiverges, HostTypeList, HostTypeListEnd};

    #[test]
    fn emitted_native_registration_preserves_identity_captures_and_completion() {
        struct Answer;
        impl HostCallableSchema for Answer {
            const PACKAGE: &'static str = "example";
            const MODULE: &'static str = "callbacks/private";
            const NAME: &'static str = "answer";
            type Arguments = HostTypeList<num_bigint::BigInt, HostTypeListEnd>;
            type Captures = HostTypeList<bool, HostTypeListEnd>;
            type Return = crate::StringValue;
            type Constructions = HostTypeListEnd;
            type Completion = HostDiverges;
        }
        let declaration = RegisteredCallableConstruction::of::<Answer>();
        let registration = CallableRegistration::from_registered(&declaration);
        assert!(registration.matches(&declaration));
        assert_eq!(
            Rust::expression(&registration),
            r#"
data::host::CallableRegistration {
    package: data::Text::Static("example"),
    module: data::Text::Static("callbacks/private"),
    name: data::Text::Static("answer"),
    arguments: data::Storage::Static(&[
        data::host::RegistrationType::Int,
    ]),
    captures: data::Storage::Static(&[
        data::host::RegistrationType::Bool,
    ]),
    return_: data::host::RegistrationType::String,
    returns_value: false,
}"#
            .trim_start_matches('\n')
        );

        for changed in [
            CallableRegistration {
                package: "another".into(),
                ..registration.clone()
            },
            CallableRegistration {
                module: "another".into(),
                ..registration.clone()
            },
            CallableRegistration {
                name: "another".into(),
                ..registration.clone()
            },
            CallableRegistration {
                arguments: vec![super::RegistrationType::Bool].into(),
                ..registration.clone()
            },
            CallableRegistration {
                captures: vec![super::RegistrationType::Int].into(),
                ..registration.clone()
            },
            CallableRegistration {
                return_: super::RegistrationType::Int,
                ..registration.clone()
            },
            CallableRegistration {
                returns_value: true,
                ..registration.clone()
            },
        ] {
            assert!(!changed.matches(&declaration));
        }
    }
}
