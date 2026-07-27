mod adapter;
mod argument;
mod return_;
mod type_;

use crate::host::{HostProfile, HostProvider};
use crate::plan::FunctionType;
use ecow::EcoString;
use std::fmt;

#[cfg(test)]
pub(in crate::host) use argument::CallArguments;
pub(crate) use argument::{
    HostBoolArgumentSlot, HostCallArguments, HostIntArgumentSlot, HostParameter,
};
pub(crate) use return_::{HostBoolFunction, HostFunctionImplementation, HostIntFunction};
pub(crate) use type_::HostValueType;

#[derive(Clone, PartialEq, Eq)]
pub struct HostFunctionSchema {
    name: EcoString,
    parameters: Box<[HostParameter]>,
    return_: HostValueType,
    type_: FunctionType,
}

/// A Rust function that can be registered as a Geam host function.
///
/// Host functions accept zero through seven `BigInt` or `bool` arguments and
/// return either `BigInt` or `bool`.
///
/// ```compile_fail
/// use geam::HostModule;
/// use num_bigint::BigInt;
///
/// let _ = HostModule::new("host_support", "host/math")
///     .unwrap()
///     .with_function(
///         "too_many",
///         |_: BigInt,
///          _: BigInt,
///          _: BigInt,
///          _: BigInt,
///          _: BigInt,
///          _: BigInt,
///          _: BigInt,
///          _: BigInt|
///          -> BigInt { 0.into() },
///     );
/// ```
pub trait HostFunction<Arguments, Return>:
    adapter::HostFunction<Arguments, Return> + Send + Sync + 'static
{
}

impl<Function, Arguments, Return> HostFunction<Arguments, Return> for Function where
    Function: adapter::HostFunction<Arguments, Return> + Send + Sync + 'static
{
}

pub trait FallibleHostFunction<Arguments, Return>:
    adapter::FallibleHostFunction<Arguments, Return> + Send + Sync + 'static
{
}

impl<Function, Arguments, Return> FallibleHostFunction<Arguments, Return> for Function where
    Function: adapter::FallibleHostFunction<Arguments, Return> + Send + Sync + 'static
{
}

pub trait ScopedHostFunction<Profile, Provider, Arguments, Return>:
    adapter::ScopedHostFunction<Profile, Provider, Arguments, Return> + Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

impl<Profile, Provider, Function, Arguments, Return>
    ScopedHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function:
        adapter::ScopedHostFunction<Profile, Provider, Arguments, Return> + Send + Sync + 'static,
{
}

pub(crate) struct HostFunctionDefinition<Profile: HostProfile> {
    schema: HostFunctionSchema,
    implementation: HostFunctionImplementation<Profile>,
}

impl HostFunctionSchema {
    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn parameters(&self) -> &[HostParameter] {
        &self.parameters
    }

    pub(crate) fn return_type(&self) -> HostValueType {
        self.return_
    }
}

impl fmt::Debug for HostFunctionSchema {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostFunctionSchema")
            .field("name", &self.name)
            .field("type_", &self.type_)
            .finish()
    }
}

impl<Profile: HostProfile> HostFunctionDefinition<Profile> {
    pub(crate) fn new<Arguments, Return, Function>(name: EcoString, function: Function) -> Self
    where
        Function: HostFunction<Arguments, Return>,
    {
        let registration =
            <Function as adapter::HostFunction<Arguments, Return>>::register::<Profile>(function);
        Self::from_registration(name, registration)
    }

    pub(crate) fn new_fallible<Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Self
    where
        Function: FallibleHostFunction<Arguments, Return>,
    {
        let registration = <Function as adapter::FallibleHostFunction<Arguments, Return>>::register::<
            Profile,
        >(function);
        Self::from_registration(name, registration)
    }

    pub(crate) fn new_scoped<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Self
    where
        Provider: HostProvider<Profile>,
        Function: ScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        let registration = <Function as adapter::ScopedHostFunction<
            Profile,
            Provider,
            Arguments,
            Return,
        >>::register(function);
        Self::from_registration(name, registration)
    }

    fn from_registration(
        name: EcoString,
        registration: adapter::HostFunctionRegistration<Profile>,
    ) -> Self {
        let argument_types = registration
            .parameters
            .iter()
            .map(|parameter| parameter.type_().value_type())
            .collect();
        let return_type = registration.return_.value_type();
        Self {
            schema: HostFunctionSchema {
                name,
                parameters: registration.parameters,
                return_: registration.return_,
                type_: FunctionType::new(argument_types, return_type),
            },
            implementation: registration.implementation,
        }
    }

    pub(crate) fn schema(&self) -> &HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(self) -> (HostFunctionSchema, HostFunctionImplementation<Profile>) {
        (self.schema, self.implementation)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostBoolFunction, HostFunctionDefinition, HostFunctionImplementation, HostIntFunction,
        HostValueType,
    };
    use crate::host::function::argument::{
        HostBoolArgumentSlot, HostCallArguments, HostIntArgumentSlot,
    };
    use crate::plan::ValueType;
    use num_bigint::BigInt;

    struct Arguments {
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    }

    impl HostCallArguments for Arguments {
        fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
            self.ints[slot.index()].clone()
        }

        fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
            self.bools[slot.index()]
        }
    }

    #[test]
    fn definition_assembles_schema_and_int_implementation_together() {
        let definition = HostFunctionDefinition::new(
            "choose".into(),
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        );

        assert_eq!(definition.schema().name(), "choose");
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Bool, ValueType::Int, ValueType::Int],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Int);
        assert_eq!(definition.schema().return_type(), HostValueType::Int);

        let (_, implementation) = definition.into_parts();
        let implementation = int_implementation(implementation);
        assert_eq!(
            implementation.call(
                &mut (),
                &Arguments {
                    ints: vec![10.into(), 20.into()],
                    bools: vec![false],
                },
            ),
            Ok(BigInt::from(20)),
        );
        assert_eq!(
            implementation.call(
                &mut (),
                &Arguments {
                    ints: vec![10.into(), 20.into()],
                    bools: vec![true],
                },
            ),
            Ok(BigInt::from(10)),
        );
    }

    #[test]
    fn definition_assembles_schema_and_bool_implementation_together() {
        let definition =
            HostFunctionDefinition::new("is_positive".into(), |value: BigInt| value > 0.into());

        assert_eq!(definition.schema().name(), "is_positive");
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Int],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Bool);
        assert_eq!(definition.schema().return_type(), HostValueType::Bool);

        let (_, implementation) = definition.into_parts();
        assert_eq!(
            bool_implementation(implementation).call(
                &mut (),
                &Arguments {
                    ints: vec![1.into()],
                    bools: Vec::new(),
                },
            ),
            Ok(true),
        );
    }

    #[test]
    fn schema_clone_contains_only_the_registered_signature() {
        let definition: HostFunctionDefinition<crate::host::StatelessHostProfile> =
            HostFunctionDefinition::new("negate".into(), <bool as std::ops::Not>::not);
        let schema = definition.schema().clone();

        assert_eq!(schema, *definition.schema());
        assert_eq!(schema.name(), "negate");
        assert_eq!(schema.type_().argument_types(), [ValueType::Bool],);
        assert_eq!(schema.type_().return_(), &ValueType::Bool);
        assert_eq!(
            format!("{schema:?}"),
            r#"HostFunctionSchema { name: "negate", type_: FunctionType { arguments: [Bool], return_: Bool } }"#,
        );
    }

    #[test]
    #[should_panic(expected = "choose should retain an Int implementation")]
    fn int_definition_shape_guard_is_visible() {
        let definition = HostFunctionDefinition::new("choose".into(), || true);
        let (_, implementation) = definition.into_parts();
        int_implementation(implementation);
    }

    #[test]
    #[should_panic(expected = "is_positive should retain a Bool implementation")]
    fn bool_definition_shape_guard_is_visible() {
        let definition =
            HostFunctionDefinition::new("is_positive".into(), <BigInt as Default>::default);
        let (_, implementation) = definition.into_parts();
        bool_implementation(implementation);
    }

    fn int_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostIntFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("choose should retain an Int implementation");
        };
        implementation
    }

    fn bool_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostBoolFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("is_positive should retain a Bool implementation");
        };
        implementation
    }
}
