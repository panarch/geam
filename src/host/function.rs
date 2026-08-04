mod adapter;
mod argument;
mod return_;

use crate::host::{HostProfile, HostProvider};
use crate::plan::{FunctionType, TypeScheme};
use ecow::EcoString;
use std::collections::BTreeSet;
use std::fmt;

#[cfg(test)]
pub(crate) use argument::CallArguments;
pub(crate) use argument::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostCustomArgumentSlot,
    HostExternalArgumentSlot, HostFloatArgumentSlot, HostFunctionArgumentSlot, HostIntArgumentSlot,
    HostListArgumentSlot, HostNilArgumentSlot, HostParameter, HostStringArgumentSlot,
    HostTupleArgumentSlot, HostUtfCodepointArgumentSlot, HostValueArgumentSlot,
};
pub(crate) use return_::HostNeverFunction;
pub(crate) use return_::{HostFunctionImplementation, HostValueFunction};
#[cfg(test)]
pub(crate) use return_::{expect_never_implementation, expect_value_implementation};

/// A Rust function that can be registered as a Geam host function.
///
/// Owned host functions accept zero through seven scalar arguments. Supported
/// Rust values are `BigInt`, `f64`, `EcoString`, `BitArrayValue`, `char`,
/// `bool`, and `()`. A host function returns one value from the same set, or
/// `Infallible` when it cannot return successfully.
///
/// Scoped host functions use the same arity boundary with the typed
/// `HostTypeParameter`, `HostListType`, `HostTupleType`, and `HostCustomType`
/// language. Their compound values remain borrowed through one `HostCall`.
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
    adapter::HostFunctionAdapter<Arguments, Return> + Send + Sync + 'static
{
}

pub trait FallibleHostFunction<Arguments, Return>:
    adapter::FallibleHostFunctionAdapter<Arguments, Return> + Send + Sync + 'static
{
}

pub trait ScopedHostFunction<Profile, Provider, Arguments, Return>:
    adapter::ScopedHostFunctionAdapter<Profile, Provider, Arguments, Return> + Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

/// A scoped host function with statically registered intermediate value types.
///
/// Implementations receive [`crate::HostConstructions`] after the active
/// [`crate::HostCall`] and before the source arguments.
pub trait ScopedConstructingHostFunction<Profile, Provider, Arguments, Return, Constructions>:
    adapter::ScopedConstructingHostFunctionAdapter<
        Profile,
        Provider,
        Arguments,
        Return,
        Constructions,
    > + Send
    + Sync
    + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: crate::host::HostTypeSequence,
{
}

pub trait ScopedDivergingHostFunction<Profile, Provider, Arguments, Return>:
    adapter::ScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>
    + Send
    + Sync
    + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

impl<Function, Arguments, Return> HostFunction<Arguments, Return> for Function where
    Function: adapter::HostFunctionAdapter<Arguments, Return> + Send + Sync + 'static
{
}

impl<Function, Arguments, Return> FallibleHostFunction<Arguments, Return> for Function where
    Function: adapter::FallibleHostFunctionAdapter<Arguments, Return> + Send + Sync + 'static
{
}

impl<Profile, Provider, Function, Arguments, Return>
    ScopedHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: adapter::ScopedHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

impl<Profile, Provider, Function, Arguments, Return, Constructions>
    ScopedConstructingHostFunction<Profile, Provider, Arguments, Return, Constructions> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: crate::host::HostTypeSequence,
    Function: adapter::ScopedConstructingHostFunctionAdapter<
            Profile,
            Provider,
            Arguments,
            Return,
            Constructions,
        > + Send
        + Sync
        + 'static,
{
}

impl<Profile, Provider, Function, Arguments, Return>
    ScopedDivergingHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: adapter::ScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

#[derive(Clone, PartialEq, Eq)]
pub struct HostFunctionSchema {
    name: EcoString,
    scheme: TypeScheme,
    layout: Box<[HostParameter]>,
    parameters: Box<[crate::host::HostTypeDescriptor]>,
    return_: crate::host::HostTypeDescriptor,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    external_schemas: Box<[crate::host::HostExternalTypeSchema]>,
    type_: FunctionType,
}

struct HostFunctionSchemaRegistration {
    layout: Box<[HostParameter]>,
    parameters: Box<[crate::host::HostTypeDescriptor]>,
    return_: crate::host::HostTypeDescriptor,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
}

pub(crate) struct HostFunctionDefinition<Profile: HostProfile> {
    schema: HostFunctionSchema,
    constructions: RegisteredHostConstructions,
    implementation: HostFunctionImplementation<Profile>,
}

pub(crate) struct RegisteredHostConstructions {
    types: Box<[crate::host::HostTypeDescriptor]>,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    external_schemas: Box<[crate::host::HostExternalTypeSchema]>,
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

    pub(crate) fn parameters(&self) -> &[crate::host::HostTypeDescriptor] {
        &self.parameters
    }

    pub(crate) fn layout(&self) -> &[HostParameter] {
        &self.layout
    }

    pub(crate) fn return_type(&self) -> &crate::host::HostTypeDescriptor {
        &self.return_
    }

    pub(crate) fn custom_schemas(&self) -> &[crate::host::HostCustomTypeSchema] {
        &self.custom_schemas
    }

    pub(crate) fn external_schemas(&self) -> &[crate::host::HostExternalTypeSchema] {
        &self.external_schemas
    }

    fn from_registration(
        name: EcoString,
        registration: HostFunctionSchemaRegistration,
    ) -> Result<Self, crate::HostRegistrationError> {
        let argument_types = registration
            .parameters
            .iter()
            .map(crate::host::HostTypeDescriptor::value_type)
            .collect();
        let return_type = registration.return_.value_type();
        let mut type_parameters = BTreeSet::new();
        for parameter in &registration.parameters {
            parameter.collect_type_parameters(&mut type_parameters);
        }
        registration
            .return_
            .collect_type_parameters(&mut type_parameters);
        let type_parameters = type_parameters.into_iter().collect::<Vec<_>>();
        if type_parameters.iter().copied().ne(0..type_parameters.len()) {
            return Err(crate::HostRegistrationError::NonContiguousTypeParameters {
                function: name,
                parameters: type_parameters.into_boxed_slice(),
            });
        }
        let mut external_schemas = Vec::new();
        let mut external_identities = std::collections::HashSet::new();
        for parameter in &registration.parameters {
            parameter.collect_external_schemas(&mut external_schemas, &mut external_identities);
        }
        registration
            .return_
            .collect_external_schemas(&mut external_schemas, &mut external_identities);
        Ok(Self {
            name,
            scheme: TypeScheme::new(type_parameters.len()),
            layout: registration.layout,
            parameters: registration.parameters,
            return_: registration.return_,
            custom_schemas: registration.custom_schemas,
            external_schemas: external_schemas.into_boxed_slice(),
            type_: FunctionType::new(argument_types, return_type),
        })
    }
}

impl RegisteredHostConstructions {
    fn new(
        types: Box<[crate::host::HostTypeDescriptor]>,
        custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    ) -> Self {
        let mut external_schemas = Vec::new();
        let mut external_identities = std::collections::HashSet::new();
        for type_ in &types {
            type_.collect_external_schemas(&mut external_schemas, &mut external_identities);
        }
        Self {
            types,
            custom_schemas,
            external_schemas: external_schemas.into_boxed_slice(),
        }
    }

    pub(crate) fn empty() -> Self {
        Self::new(Box::new([]), Box::new([]))
    }

    pub(crate) fn types(&self) -> &[crate::host::HostTypeDescriptor] {
        &self.types
    }

    pub(crate) fn custom_schemas(&self) -> &[crate::host::HostCustomTypeSchema] {
        &self.custom_schemas
    }

    pub(crate) fn external_schemas(&self) -> &[crate::host::HostExternalTypeSchema] {
        &self.external_schemas
    }

    fn unbound_type_parameters(&self, parameter_count: usize) -> Box<[usize]> {
        let mut parameters = BTreeSet::new();
        for type_ in &self.types {
            type_.collect_type_parameters(&mut parameters);
        }
        parameters
            .into_iter()
            .filter(|parameter| *parameter >= parameter_count)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

impl fmt::Debug for HostFunctionSchema {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_struct("HostFunctionSchema");
        debug
            .field("name", &self.name)
            .field("scheme", &self.scheme)
            .field("type_", &self.type_);
        if !self.custom_schemas.is_empty() {
            debug.field("custom_schemas", &self.custom_schemas);
        }
        if !self.external_schemas.is_empty() {
            debug.field("external_schemas", &self.external_schemas);
        }
        debug.finish()
    }
}

impl<Profile: HostProfile> HostFunctionDefinition<Profile> {
    pub(crate) fn new<Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Function: HostFunction<Arguments, Return>,
    {
        let registration = <Function as adapter::HostFunctionAdapter<Arguments, Return>>::register::<
            Profile,
        >(function);
        Self::from_registration(name, registration)
    }

    pub(crate) fn new_fallible<Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Function: FallibleHostFunction<Arguments, Return>,
    {
        let registration =
            <Function as adapter::FallibleHostFunctionAdapter<Arguments, Return>>::register::<
                Profile,
            >(function);
        Self::from_registration(name, registration)
    }

    pub(crate) fn new_scoped<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        let registration = <Function as adapter::ScopedHostFunctionAdapter<
            Profile,
            Provider,
            Arguments,
            Return,
        >>::register(function);
        Self::from_registration(name, registration)
    }

    pub(crate) fn new_scoped_with_constructions<
        Provider,
        Arguments,
        Return,
        Constructions,
        Function,
    >(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Constructions: crate::host::HostTypeSequence,
        Function:
            ScopedConstructingHostFunction<Profile, Provider, Arguments, Return, Constructions>,
    {
        let registration = <Function as adapter::ScopedConstructingHostFunctionAdapter<
            Profile,
            Provider,
            Arguments,
            Return,
            Constructions,
        >>::register(function);
        let construction_types =
            <Constructions as crate::host::HostAbiTypeSequence>::descriptors().into_boxed_slice();
        let mut custom_schemas = Vec::new();
        let mut visited = std::collections::HashSet::new();
        <Constructions as crate::host::HostAbiTypeSequence>::collect_custom_schemas(
            &mut custom_schemas,
            &mut visited,
        );
        let constructions =
            RegisteredHostConstructions::new(construction_types, custom_schemas.into_boxed_slice());
        Self::from_registration_with_constructions(name, registration, constructions)
    }

    pub(crate) fn new_scoped_diverging<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedDivergingHostFunction<Profile, Provider, Arguments, Return>,
    {
        let registration = <Function as adapter::ScopedDivergingHostFunctionAdapter<
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
    ) -> Result<Self, crate::HostRegistrationError> {
        Self::from_registration_with_constructions(
            name,
            registration,
            RegisteredHostConstructions::empty(),
        )
    }

    fn from_registration_with_constructions(
        name: EcoString,
        registration: adapter::HostFunctionRegistration<Profile>,
        constructions: RegisteredHostConstructions,
    ) -> Result<Self, crate::HostRegistrationError> {
        let schema = HostFunctionSchemaRegistration {
            layout: registration.parameters,
            parameters: registration.parameter_types,
            return_: registration.return_type,
            custom_schemas: registration.custom_schemas,
        };
        let schema = HostFunctionSchema::from_registration(name, schema)?;
        let unbound = constructions.unbound_type_parameters(schema.scheme().parameters().len());
        if !unbound.is_empty() {
            return Err(
                crate::HostRegistrationError::UnboundConstructionTypeParameters {
                    function: schema.name().clone(),
                    parameters: unbound,
                },
            );
        }
        Ok(Self {
            schema,
            constructions,
            implementation: registration.implementation,
        })
    }

    pub(crate) fn schema(&self) -> &HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        HostFunctionSchema,
        RegisteredHostConstructions,
        HostFunctionImplementation<Profile>,
    ) {
        (self.schema, self.constructions, self.implementation)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostFunctionDefinition, HostFunctionSchema, RegisteredHostConstructions};
    use crate::BitArrayValue;
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCustomConstructorSchema,
        HostCustomFieldSchema, HostCustomTypeSchema, HostExternalTypeSchema, HostListType,
        HostProvider, HostRegistrationError, HostSchemaType, HostScopedValue, HostTypeDescriptor,
        HostTypeIndex0, HostTypeList, HostTypeListEnd, HostValueFamily,
        expect_value_implementation,
    };
    use crate::plan::ValueType;
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct ConstructionProvider;

    impl HostProvider<TestHostProfile> for ConstructionProvider {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    fn ready<'call>(
        call: HostCall<'call, TestHostProfile, ConstructionProvider, bool>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.return_value(true))
    }

    type ConstructionTypes = HostTypeList<HostListType<BigInt>, HostTypeListEnd>;

    fn ready_with_constructions<'call>(
        call: HostCall<'call, TestHostProfile, ConstructionProvider, bool>,
        constructions: crate::HostConstructions<'call, ConstructionTypes>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let _ = constructions.at::<HostTypeIndex0>();
        Ok(call.return_value(true))
    }

    #[test]
    fn definition_assembles_schema_and_int_implementation_together() {
        let definition = HostFunctionDefinition::new(
            "choose".into(),
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        )
        .expect("contiguous scalar function should register");

        assert_eq!(definition.schema().name(), "choose");
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Bool, ValueType::Int, ValueType::Int],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Int);
        assert_eq!(definition.schema().return_type(), &HostTypeDescriptor::Int);

        let (_, _, implementation) = definition.into_parts();
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let arguments = CallArguments::new(vec![10.into(), 20.into()], vec![false]);
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            implementation.call(&mut runtime).map(|token| token.family),
            Ok(HostValueFamily::Int),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(20))),
        );
        let arguments = CallArguments::new(vec![10.into(), 20.into()], vec![true]);
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            implementation.call(&mut runtime).map(|token| token.family),
            Ok(HostValueFamily::Int),
        );
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(10))),
        );
    }

    #[test]
    fn definition_assembles_schema_and_bool_implementation_together() {
        let definition =
            HostFunctionDefinition::new("is_positive".into(), |value: BigInt| value > 0.into())
                .expect("monomorphic function should register");

        assert_eq!(definition.schema().name(), "is_positive");
        assert_eq!(
            definition.schema().type_().argument_types(),
            [ValueType::Int],
        );
        assert_eq!(definition.schema().type_().return_(), &ValueType::Bool);
        assert_eq!(definition.schema().return_type(), &HostTypeDescriptor::Bool);

        let (_, _, implementation) = definition.into_parts();
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let arguments = CallArguments::new(vec![1.into()], Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            implementation.call(&mut runtime).map(|token| token.family),
            Ok(HostValueFamily::Bool),
        );
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Bool(true)));
    }

    #[test]
    fn definition_assembles_every_scalar_parameter_from_one_layout() {
        let definition: HostFunctionDefinition<TestHostProfile> = HostFunctionDefinition::new(
            "consume".into(),
            |_: BigInt, _: f64, _: EcoString, _: BitArrayValue, _: char, _: bool, (): ()| (),
        )
        .expect("monomorphic scalar function should register");

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
        assert_eq!(definition.schema().return_type(), &HostTypeDescriptor::Nil);

        let (_, _, implementation) = definition.into_parts();
        let implementation = expect_value_implementation(&implementation);
        let arguments = CallArguments::new(vec![1.into()], vec![true]).with_scalar_values(
            vec![1.5],
            vec!["one".into()],
            vec![BitArrayValue::from_bytes(vec![1])],
            vec!['A'],
            1,
        );
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        assert_eq!(
            implementation.call(&mut runtime).map(|token| token.family),
            Ok(HostValueFamily::Nil),
        );
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Nil));
    }

    #[test]
    fn schema_clone_contains_only_the_registered_signature() {
        let definition: HostFunctionDefinition<TestHostProfile> =
            HostFunctionDefinition::new("negate".into(), <bool as std::ops::Not>::not)
                .expect("monomorphic function should register");
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
    fn hidden_construction_types_stay_outside_the_public_function_schema() {
        let plain = HostFunctionDefinition::new_scoped::<ConstructionProvider, (), bool, _>(
            "ready".into(),
            ready,
        )
        .expect("plain scoped function should register");
        let with_constructions = HostFunctionDefinition::new_scoped_with_constructions::<
            ConstructionProvider,
            (),
            bool,
            ConstructionTypes,
            _,
        >("ready".into(), ready_with_constructions)
        .expect("scoped function with hidden constructions should register");
        let (schema, constructions, constructing_implementation) = with_constructions.into_parts();

        assert_eq!(schema, *plain.schema());
        assert_eq!(schema.scheme(), &crate::plan::TypeScheme::new(0));
        assert_eq!(schema.type_(), plain.schema().type_());
        assert_eq!(
            constructions.types(),
            [HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Int))],
        );
        assert!(constructions.custom_schemas().is_empty());
        assert!(constructions.external_schemas().is_empty());

        let (_, _, plain_implementation) = plain.into_parts();
        for implementation in [&plain_implementation, &constructing_implementation] {
            let implementation = expect_value_implementation(implementation);
            let mut state = TestRunState::default();
            assert!(std::ptr::eq(
                ConstructionProvider::project(&mut state),
                &state.counter,
            ));
            let arguments = CallArguments::new(Vec::new(), Vec::new());
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            assert_eq!(
                implementation.call(&mut runtime).map(|token| token.family),
                Ok(HostValueFamily::Bool),
            );
            assert_eq!(runtime.completed(), Some(&HostScopedValue::Bool(true)));
        }
    }

    #[test]
    fn registered_constructions_report_parameters_outside_the_function_scheme() {
        let constructions = RegisteredHostConstructions::new(
            vec![
                HostTypeDescriptor::List(Box::new(HostTypeDescriptor::Parameter(0))),
                HostTypeDescriptor::Parameter(2),
                HostTypeDescriptor::Parameter(2),
            ]
            .into_boxed_slice(),
            Box::new([]),
        );

        assert_eq!(
            constructions.unbound_type_parameters(0),
            vec![0, 2].into_boxed_slice(),
        );
        assert_eq!(
            constructions.unbound_type_parameters(1),
            vec![2].into_boxed_slice(),
        );
        assert_eq!(
            constructions.unbound_type_parameters(3),
            Vec::<usize>::new().into_boxed_slice(),
        );
    }

    #[test]
    fn schema_debug_includes_custom_definitions_not_derived_from_the_function_type() {
        let custom_schema = HostCustomTypeSchema::new(
            "host_shapes",
            "host/shape",
            "Shape",
            0,
            [HostCustomConstructorSchema::new(
                "Circle",
                [HostCustomFieldSchema::new(
                    Some("radius"),
                    HostSchemaType::Float,
                )],
            )],
        );
        let return_ = HostTypeDescriptor::Custom {
            schema: custom_schema.clone(),
            arguments: Box::new([]),
        };
        let schema = HostFunctionSchema {
            name: "origin".into(),
            scheme: crate::plan::TypeScheme::new(0),
            layout: Box::new([]),
            parameters: Box::new([]),
            type_: crate::plan::FunctionType::new(Vec::new(), return_.value_type()),
            return_,
            custom_schemas: vec![custom_schema].into_boxed_slice(),
            external_schemas: Box::new([]),
        };

        assert_eq!(
            format!("{schema:?}"),
            r#"HostFunctionSchema { name: "origin", scheme: TypeScheme { parameters: [] }, type_: FunctionType { arguments: [], return_: Custom(CustomType { name: CustomTypeName { package: "host_shapes", module: "host/shape", name: "Shape" }, arguments: [] }) }, custom_schemas: [HostCustomTypeSchema { package: "host_shapes", module: "host/shape", name: "Shape", parameter_count: 0, constructors: [HostCustomConstructorSchema { name: "Circle", fields: [HostCustomFieldSchema { label: Some("radius"), type_: Float }] }] }] }"#,
        );
    }

    #[test]
    fn schema_debug_includes_external_definitions_not_derived_from_the_function_type() {
        let external_schema =
            HostExternalTypeSchema::new("host_shapes", "host/resource", "Resource", 1);
        let return_ = HostTypeDescriptor::External {
            schema: external_schema.clone(),
            arguments: vec![HostTypeDescriptor::Parameter(0)].into_boxed_slice(),
        };
        let schema = HostFunctionSchema {
            name: "resource".into(),
            scheme: crate::plan::TypeScheme::new(1),
            layout: Box::new([]),
            parameters: Box::new([]),
            type_: crate::plan::FunctionType::new(Vec::new(), return_.value_type()),
            return_,
            custom_schemas: Box::new([]),
            external_schemas: vec![external_schema].into_boxed_slice(),
        };

        assert_eq!(
            format!("{schema:?}"),
            r#"HostFunctionSchema { name: "resource", scheme: TypeScheme { parameters: [TypeParameterId(0)] }, type_: FunctionType { arguments: [], return_: External(ExternalType { name: ExternalTypeName { package: "host_shapes", module: "host/resource", name: "Resource" }, arguments: [Parameter(TypeParameterId(0))] }) }, external_schemas: [HostExternalTypeSchema { package: "host_shapes", module: "host/resource", name: "Resource", parameter_count: 1 }] }"#,
        );
    }

    #[test]
    fn definition_rejects_non_contiguous_type_parameter_indices_before_allocating_a_scheme() {
        let mut registration = <_ as super::adapter::HostFunctionAdapter<(), bool>>::register::<
            TestHostProfile,
        >(|| true);
        registration.return_type = HostTypeDescriptor::Parameter(2);
        let error = HostFunctionDefinition::from_registration("identity".into(), registration)
            .err()
            .expect("sparse type parameters should be rejected");

        assert_eq!(
            error,
            HostRegistrationError::NonContiguousTypeParameters {
                function: "identity".into(),
                parameters: vec![2].into_boxed_slice(),
            },
        );
        assert_eq!(
            error.to_string(),
            "host function identity uses type parameter indices [2]; indices must be contiguous from zero",
        );
    }
}
