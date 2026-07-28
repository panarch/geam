mod adapter;
mod argument;
mod return_;
mod type_;

use crate::host::{HostProfile, HostProvider};
use crate::plan::{FunctionType, TypeScheme};
use ecow::EcoString;
use std::fmt;

#[cfg(test)]
pub(in crate::host) use argument::CallArguments;
pub(crate) use argument::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostFloatArgumentSlot,
    HostIntArgumentSlot, HostNilArgumentSlot, HostParameter, HostStringArgumentSlot,
    HostUtfCodepointArgumentSlot,
};
pub(crate) use return_::{
    HostBitArrayFunction, HostBoolFunction, HostFloatFunction, HostFunctionImplementation,
    HostIntFunction, HostNeverFunction, HostNilFunction, HostStringFunction,
    HostUtfCodepointFunction, HostValueFunctionImplementation,
};
pub(crate) use type_::HostValueType;

#[derive(Clone, PartialEq, Eq)]
pub struct HostFunctionSchema {
    name: EcoString,
    scheme: TypeScheme,
    parameters: Box<[HostParameter]>,
    return_: HostValueType,
    type_: FunctionType,
}

/// A Rust function that can be registered as a Geam host function.
///
/// Host functions accept zero through seven scalar arguments. Supported Rust
/// values are `BigInt`, `f64`, `EcoString`, `BitArrayValue`, `char`, `bool`,
/// and `()`. A host function returns one value from the same set, or
/// `Infallible` when it never returns successfully.
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
///
/// ```compile_fail
/// use geam::HostModule;
///
/// let _ = HostModule::new("host_support", "host/math")
///     .unwrap()
///     .with_function("unsupported", |value: i64| value);
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

    pub fn scheme(&self) -> &TypeScheme {
        &self.scheme
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
            .field("scheme", &self.scheme)
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
        let parameter_count = registration
            .parameters
            .iter()
            .map(|parameter| parameter.type_().type_parameter_count())
            .chain([registration.return_.type_parameter_count()])
            .max()
            .unwrap_or(0);
        Self {
            schema: HostFunctionSchema {
                name,
                scheme: TypeScheme::new(parameter_count),
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
        HostNilFunction, HostValueType,
    };
    use crate::BitArrayValue;
    use crate::host::HostFailure;
    use crate::host::function::argument::CallArguments;
    use crate::plan::{TypeParameterId, ValueType};
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::convert::Infallible;

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
                &CallArguments::new(vec![10.into(), 20.into()], vec![false]),
            ),
            Ok(BigInt::from(20)),
        );
        assert_eq!(
            implementation.call(
                &mut (),
                &CallArguments::new(vec![10.into(), 20.into()], vec![true]),
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
            bool_implementation(implementation)
                .call(&mut (), &CallArguments::new(vec![1.into()], Vec::new()),),
            Ok(true),
        );
    }

    #[test]
    fn definition_assembles_every_scalar_parameter_from_one_layout() {
        let definition: HostFunctionDefinition<crate::host::StatelessHostProfile> =
            HostFunctionDefinition::new(
                "consume".into(),
                |_: BigInt, _: f64, _: EcoString, _: BitArrayValue, _: char, _: bool, (): ()| (),
            );

        assert_eq!(
            definition.schema().type_().argument_types(),
            [
                ValueType::Int,
                ValueType::Float,
                ValueType::String,
                ValueType::BitArray,
                ValueType::UtfCodepoint,
                ValueType::Bool,
                ValueType::Nil,
            ],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Nil);
        assert_eq!(definition.schema().return_type(), HostValueType::Nil);

        let (_, implementation) = definition.into_parts();
        let implementation = nil_implementation(implementation);
        let arguments = CallArguments::new(vec![1.into()], vec![true]).with_scalar_values(
            vec![1.5],
            vec!["one".into()],
            vec![BitArrayValue::from_bytes(vec![1])],
            vec!['A'],
            1,
        );
        assert_eq!(implementation.call(&mut (), &arguments), Ok(()));
    }

    #[test]
    fn definition_assembles_generic_never_scheme_and_callback_together() {
        let definition: HostFunctionDefinition<crate::host::StatelessHostProfile> =
            HostFunctionDefinition::new_fallible(
                "stop".into(),
                |value: BigInt| -> Result<Infallible, HostFailure> {
                    Err(HostFailure::new(format!("stopped at {value}")))
                },
            );

        assert_eq!(definition.schema().name(), "stop");
        assert_eq!(
            definition.schema().scheme().parameters(),
            [TypeParameterId(0)],
        );
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Int],
        );
        assert_eq!(
            definition.schema().type_().return_(),
            &ValueType::Parameter(TypeParameterId(0)),
        );
        assert_eq!(
            definition.schema().return_type(),
            HostValueType::Parameter(TypeParameterId(0)),
        );

        let (_, implementation) = definition.into_parts();
        assert_eq!(
            never_implementation(implementation)
                .call(
                    &mut (),
                    &CallArguments::new(vec![BigInt::from(7)], Vec::new()),
                )
                .expect_err("stop should fail")
                .to_string(),
            "stopped at 7",
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
            r#"HostFunctionSchema { name: "negate", scheme: TypeScheme { parameters: [] }, type_: FunctionType { arguments: [Bool], return_: Bool } }"#,
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

    #[test]
    #[should_panic(expected = "consume should retain a Nil implementation")]
    fn nil_definition_shape_guard_is_visible() {
        let definition = HostFunctionDefinition::new("consume".into(), || true);
        let (_, implementation) = definition.into_parts();
        nil_implementation(implementation);
    }

    #[test]
    #[should_panic(expected = "stop should retain a Never implementation")]
    fn never_definition_shape_guard_is_visible() {
        let definition = HostFunctionDefinition::new("stop".into(), || true);
        let (_, implementation) = definition.into_parts();
        never_implementation(implementation);
    }

    fn int_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostIntFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Value(super::HostValueFunctionImplementation::Int(
            implementation,
        )) = implementation
        else {
            panic!("choose should retain an Int implementation");
        };
        implementation
    }

    fn bool_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostBoolFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Value(super::HostValueFunctionImplementation::Bool(
            implementation,
        )) = implementation
        else {
            panic!("is_positive should retain a Bool implementation");
        };
        implementation
    }

    fn nil_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostNilFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Value(super::HostValueFunctionImplementation::Nil(
            implementation,
        )) = implementation
        else {
            panic!("consume should retain a Nil implementation");
        };
        implementation
    }

    fn never_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> super::HostNeverFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Never(implementation) = implementation else {
            panic!("stop should retain a Never implementation");
        };
        implementation
    }
}
