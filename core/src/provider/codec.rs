use crate::host::{
    HostCall, HostCallCompletion, HostCallError, HostConstruction, HostConstructions, HostProfile,
    HostProvider, HostType, HostTypeList, HostTypeListEnd, HostTypeSequence,
};
use std::marker::PhantomData;

/// Static host type selected by one generated provider value declaration.
#[doc(hidden)]
pub trait ProviderValue {
    type Host: HostType;
    type OutputRequirements: ProviderConstructionRequirements;
    type RootRequirements: ProviderConstructionRequirements;
}

/// Host representation selected after the execution profile is known.
/// A declaration containing explicit work cannot select its nominal work
/// schema until this point.
#[doc(hidden)]
pub trait ProviderTypedValue<Profile: HostProfile> {
    type Host: HostType;
    type OutputRequirements: ProviderConstructionRequirements;
    type RootRequirements: ProviderConstructionRequirements;
}

impl<Profile: HostProfile, Value: ProviderValue> ProviderTypedValue<Profile> for Value {
    type Host = Value::Host;
    type OutputRequirements = Value::OutputRequirements;
    type RootRequirements = Value::RootRequirements;
}

impl<const INDEX: usize> ProviderValue for crate::HostTypeParameter<INDEX> {
    type Host = Self;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

/// Transferable input forms selected beside one ordinary provider value.
#[doc(hidden)]
pub trait ProviderValueForms {
    type InvocationRequirements;
    type Runtime<Profile: HostProfile>;
    type Output: ProviderValueForms<Output = Self::Output> + 'static;
    type ImmediateInput;
    type ImmediateListInput;
    type OwnedInput;
    type OwnedListInput;
    type ImmediateListDecoder;
    type OwnedListDecoder;
}

/// List forms that can be named before choosing an execution profile.
/// Contextual declarations supply marker lists until their runtime family is
/// selected; static declarations keep their ordinary retained-list forms.
#[doc(hidden)]
pub trait ProviderMarkerListForms {
    type Immediate;
    type Owned;
}

impl<Value: ProviderValueForms + ProviderValue> ProviderMarkerListForms for Value {
    type Immediate = super::List<
        Value::ImmediateListInput,
        super::ProviderListContext<Value::Host, Value::ImmediateListDecoder>,
    >;
    type Owned = super::List<
        Value::OwnedListInput,
        super::ProviderListContext<Value::Host, Value::OwnedListDecoder>,
    >;
}

/// Profile-dependent forms for declarations that retain callable values.
#[doc(hidden)]
pub trait ProviderContextualValueForms<Profile: HostProfile> {
    type Host: HostType;
    type Output;
    type OutputRequirements: ProviderConstructionRequirements;
    type RootRequirements: ProviderConstructionRequirements;
    type ImmediateInput;
    type ImmediateListInput;
    type OwnedInput;
    type OwnedListInput;
    type InputRequirements: ProviderConstructionRequirements;
}

/// The declaration selects one runtime form family without requiring its
/// execution profile when the declaration itself is named.
#[doc(hidden)]
pub trait ProviderRuntimeValueForms<Profile: HostProfile> {
    type Host: HostType;
    type Output;
    type OutputRequirements: ProviderConstructionRequirements;
    type RootRequirements: ProviderConstructionRequirements;
    type ImmediateInput;
    type ImmediateListInput;
    type OwnedInput;
    type OwnedListInput;
    type InputRequirements: ProviderConstructionRequirements;
}

#[doc(hidden)]
pub struct ProviderStaticValueForms<Value>(PhantomData<fn() -> Value>);

impl<Profile: HostProfile, Value> ProviderRuntimeValueForms<Profile>
    for ProviderStaticValueForms<Value>
where
    Value: ProviderValueForms + ProviderValue,
    Value::Output: ProviderValue,
{
    type Host = Value::Host;
    type Output = Value::Output;
    type OutputRequirements = <Value::Output as ProviderValue>::OutputRequirements;
    type RootRequirements = <Value::Output as ProviderValue>::RootRequirements;
    type ImmediateInput = Value::ImmediateInput;
    type ImmediateListInput = Value::ImmediateListInput;
    type OwnedInput = Value::OwnedInput;
    type OwnedListInput = Value::OwnedListInput;
    type InputRequirements = ProviderNoConstructions;
}

impl<Profile, Value> ProviderContextualValueForms<Profile> for Value
where
    Profile: HostProfile,
    Value: ProviderValueForms,
    Value::Runtime<Profile>: ProviderRuntimeValueForms<Profile>,
{
    type Host = <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::Host;
    type Output = <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::Output;
    type OutputRequirements =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::OutputRequirements;
    type RootRequirements =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::RootRequirements;
    type ImmediateInput =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::ImmediateInput;
    type ImmediateListInput =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::ImmediateListInput;
    type OwnedInput = <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::OwnedInput;
    type OwnedListInput =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::OwnedListInput;
    type InputRequirements =
        <Value::Runtime<Profile> as ProviderRuntimeValueForms<Profile>>::InputRequirements;
}

/// Owned conversion from one transferable host value into a provider input.
#[doc(hidden)]
pub trait ProviderInputValue<Profile, Provider, Return>: Sized
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;
    type Requirements: ProviderConstructionRequirements;

    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self
    where
        Self::Requirements:
            ProviderConstructionRequirements<Types<HostTypeListEnd> = HostTypeListEnd>,
    {
        Self::from_host_with(call, value, &ProviderConstructions::empty())
    }

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self;
}

/// Conversion from an owned provider value into transferable execution.
#[doc(hidden)]
pub trait ProviderOutputValue<Profile, Provider, Return>: ProviderTypedValue<Profile>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error: Into<HostCallError>;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error>;

    fn store_dynamic<'call, Owner: super::ProviderStoredOwner>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<super::advanced::StoredDynamic<Owner>, Self::Error>
    where
        Self: Sized,
    {
        let value = self.into_host(call, constructions)?;
        Ok(super::advanced::StoredDynamic::from_runtime_value(
            call.retain_value::<Self::Host>(value),
        ))
    }

    fn into_host_infallible<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Self: Sized
            + ProviderOutputValue<Profile, Provider, Return, Error = std::convert::Infallible>,
    {
        self.into_host(call, constructions)
            .unwrap_or_else(|never| match never {})
    }
}

/// Root completion for an owned provider value in transferable execution.
#[doc(hidden)]
pub trait ProviderRootOutputValue<Profile, Provider>: ProviderTypedValue<Profile>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn complete<'call>(
        self,
        call: HostCall<'call, Profile, Provider, Self::Host>,
        constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError>;
}

/// Transferable List item view and decoder selected by one declaration.
#[doc(hidden)]
pub trait ProviderListInputValue: Sized {
    type Host: HostType;
    type View;
    type Decoder: super::ProviderTypedListItemDecoder<Self, Host = Self::Host, View = Self::View>
        + Clone
        + Send
        + 'static;
}

/// Construction of one transferable List item decoder.
#[doc(hidden)]
pub trait ProviderListInputCodec<Profile, Provider>: ProviderListInputValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    type Requirements: ProviderConstructionRequirements;

    fn decoder<Return>(call: &HostCall<'_, Profile, Provider, Return>) -> Self::Decoder
    where
        Return: HostType,
        Self::Requirements:
            ProviderConstructionRequirements<Types<HostTypeListEnd> = HostTypeListEnd>,
    {
        Self::decoder_with(call, &ProviderConstructions::empty())
    }

    fn decoder_with<'call, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder
    where
        Return: HostType;
}

/// Profile-specific transferable external access generated beside one declaration.
#[doc(hidden)]
pub trait ProviderExternalCodec<Profile>: ProviderValue + Sized + Send + 'static
where
    Profile: HostProfile,
{
    fn immediate_input<'call, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> super::ProviderExternalView<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn owned_input<'call, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> super::ProviderOwnedExternal<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn immediate_list_decoder<Provider, Return>(
        call: &HostCall<'_, Profile, Provider, Return>,
    ) -> super::ProviderExternalListDecoder<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn owned_list_decoder<Provider, Return>(
        call: &HostCall<'_, Profile, Provider, Return>,
    ) -> super::ProviderOwnedExternalListDecoder<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn immediate_output<'call, Provider, Return>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: super::ProviderExternalView<Self>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn owned_output<'call, Provider, Return>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: super::ProviderOwnedExternal<Self>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;
}

/// Exact registered construction sequence required by one generated conversion.
#[doc(hidden)]
#[allow(private_bounds)]
pub trait ProviderConstructionRequirements: private::Requirements {
    type Types<Tail: HostTypeSequence>: HostTypeSequence;
}

/// A conversion that constructs no intermediate host value.
#[doc(hidden)]
pub struct ProviderNoConstructions;

/// One exact intermediate host type constructed by a generated conversion.
#[doc(hidden)]
pub struct ProviderConstruction<Type>(PhantomData<fn() -> Type>);

/// Ordered composition of two exact generated construction requirements.
#[doc(hidden)]
pub struct ProviderConstructionList<Head, Tail>(PhantomData<fn() -> (Head, Tail)>);

/// First generated construction requirement in one exact static list.
#[doc(hidden)]
pub struct ProviderConstructionIndex0;

/// Next generated construction requirement in one exact static list.
#[doc(hidden)]
pub struct ProviderConstructionIndexNext<Index>(PhantomData<fn() -> Index>);

/// Selects one exact requirement from a generated construction list.
#[doc(hidden)]
pub trait ProviderConstructionRequirementAt<Index>: ProviderConstructionRequirements {
    const CALLABLE_OFFSET: usize;
    type Requirement: ProviderConstructionRequirements;
}

impl<Head, Tail> ProviderConstructionRequirementAt<ProviderConstructionIndex0>
    for ProviderConstructionList<Head, Tail>
where
    Head: ProviderConstructionRequirements,
    Tail: ProviderConstructionRequirements,
{
    const CALLABLE_OFFSET: usize = 0;
    type Requirement = Head;
}

impl<Head, Tail, Index> ProviderConstructionRequirementAt<ProviderConstructionIndexNext<Index>>
    for ProviderConstructionList<Head, Tail>
where
    Head: ProviderConstructionRequirements,
    Tail: ProviderConstructionRequirementAt<Index>,
{
    const CALLABLE_OFFSET: usize = crate::host::construction_callable_count::<
        Head::Types<HostTypeListEnd>,
    >() + Tail::CALLABLE_OFFSET;
    type Requirement = Tail::Requirement;
}

/// Call-scoped proof of one exact generated construction requirement tree.
///
/// The saved construction positions belong to the call that granted them.
///
/// ```compile_fail
/// use geam_core::__macro_support::{
///     ProviderConstructionRequirements, ProviderConstructions,
/// };
///
/// fn escape<'call, Requirements: ProviderConstructionRequirements>(
///     constructions: ProviderConstructions<'call, Requirements>,
/// ) -> ProviderConstructions<'static, Requirements> {
///     constructions
/// }
/// ```
#[doc(hidden)]
pub struct ProviderConstructions<'call, Requirements>
where
    Requirements: ProviderConstructionRequirements,
{
    callable_base: usize,
    marker: ConstructionMarker<'call, Requirements>,
}

type ConstructionMarker<'call, Requirements> =
    PhantomData<fn(&'call ()) -> (&'call (), Requirements)>;

impl<'call, Requirements> ProviderConstructions<'call, Requirements>
where
    Requirements: ProviderConstructionRequirements,
{
    /// Creates a proof only when the complete static requirement tree is empty.
    pub fn empty() -> Self
    where
        Requirements: ProviderConstructionRequirements<Types<HostTypeListEnd> = HostTypeListEnd>,
    {
        Self {
            callable_base: 0,
            marker: PhantomData,
        }
    }

    pub fn new(
        constructions: &HostConstructions<'call, Requirements::Types<HostTypeListEnd>>,
    ) -> Self {
        Self {
            callable_base: constructions.callable_base(),
            marker: PhantomData,
        }
    }

    pub(crate) fn host(&self) -> HostConstructions<'call, Requirements::Types<HostTypeListEnd>> {
        HostConstructions::with_base(self.callable_base)
    }

    pub fn select<Index>(
        &self,
    ) -> ProviderConstructions<
        'call,
        <Requirements as ProviderConstructionRequirementAt<Index>>::Requirement,
    >
    where
        Requirements: ProviderConstructionRequirementAt<Index>,
    {
        ProviderConstructions {
            callable_base: self.callable_base
                + <Requirements as ProviderConstructionRequirementAt<Index>>::CALLABLE_OFFSET,
            marker: PhantomData,
        }
    }
}

impl<'call, Type> ProviderConstructions<'call, ProviderConstruction<Type>>
where
    Type: HostType,
{
    pub fn token(&self) -> HostConstruction<'call, Type> {
        HostConstructions::<HostTypeList<Type, HostTypeListEnd>>::with_base(self.callable_base)
            .at::<crate::HostTypeIndex0>()
    }
}

impl<'call> ProviderConstructions<'call, ProviderNoConstructions> {
    pub fn none() -> Self {
        Self::empty()
    }
}

impl ProviderConstructionRequirements for ProviderNoConstructions {
    type Types<Tail: HostTypeSequence> = Tail;
}

impl<Type> ProviderConstructionRequirements for ProviderConstruction<Type>
where
    Type: HostType,
{
    type Types<Tail: HostTypeSequence> = HostTypeList<Type, Tail>;
}

impl<Head, Tail> ProviderConstructionRequirements for ProviderConstructionList<Head, Tail>
where
    Head: ProviderConstructionRequirements,
    Tail: ProviderConstructionRequirements,
{
    type Types<End: HostTypeSequence> = Head::Types<Tail::Types<End>>;
}

macro_rules! provider_scalar {
    ($type:ty) => {
        impl ProviderValue for $type {
            type Host = Self;
            type OutputRequirements = ProviderNoConstructions;
            type RootRequirements = ProviderNoConstructions;
        }

        impl ProviderValueForms for $type {
            type InvocationRequirements = ();
            type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
            type Output = Self;
            type ImmediateInput = Self;
            type ImmediateListInput = Self;
            type OwnedInput = Self;
            type OwnedListInput = Self;
            type ImmediateListDecoder = super::ProviderScalarListDecoder<Self>;
            type OwnedListDecoder = Self::ImmediateListDecoder;
        }

        impl<Profile, Provider, Return> ProviderInputValue<Profile, Provider, Return> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostType,
        {
            type Host = Self;

            type Requirements = ProviderNoConstructions;

            fn from_host_with<'call>(
                _call: &mut HostCall<'call, Profile, Provider, Return>,
                value: <Self::Host as HostType>::Value<'call>,
                _constructions: &ProviderConstructions<'call, Self::Requirements>,
            ) -> Self {
                value
            }
        }

        impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostType,
        {
            type Error = std::convert::Infallible;

            fn into_host<'call>(
                self,
                _call: &mut HostCall<'call, Profile, Provider, Return>,
                _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
            ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
                Ok(self)
            }
        }

        impl<Profile, Provider> ProviderRootOutputValue<Profile, Provider> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
        {
            fn complete<'call>(
                self,
                call: HostCall<'call, Profile, Provider, Self::Host>,
                _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
            ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
                Ok(call.return_value(self))
            }
        }

        impl ProviderListInputValue for $type {
            type Host = Self;
            type View = Self;
            type Decoder = super::ProviderScalarListDecoder<Self>;
        }

        impl<Profile, Provider> ProviderListInputCodec<Profile, Provider> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
        {
            type Requirements = ProviderNoConstructions;

            fn decoder_with<'call, Return>(
                _call: &HostCall<'call, Profile, Provider, Return>,
                _constructions: &ProviderConstructions<'call, Self::Requirements>,
            ) -> Self::Decoder
            where
                Return: HostType,
            {
                super::ProviderScalarListDecoder::new()
            }
        }
    };
}

provider_scalar!(num_bigint::BigInt);
provider_scalar!(f64);
provider_scalar!(crate::StringValue);
provider_scalar!(crate::BitArrayValue);
provider_scalar!(char);
provider_scalar!(bool);
provider_scalar!(());

impl<Payload> ProviderValue for super::ProviderExternalView<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Host = Payload::Host;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Payload> ProviderValue for super::ProviderOwnedExternal<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Host = Payload::Host;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Payload> ProviderValueForms for super::ProviderExternalView<Payload>
where
    Payload: ProviderValue + 'static,
{
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type OwnedInput = super::ProviderOwnedExternal<Payload>;
    type OwnedListInput = Self::OwnedInput;
    type ImmediateListDecoder = super::ProviderExternalListDecoder<Payload>;
    type OwnedListDecoder = super::ProviderOwnedExternalListDecoder<Payload>;
}

impl<Payload> ProviderValueForms for super::ProviderOwnedExternal<Payload>
where
    Payload: ProviderValue + 'static,
{
    type InvocationRequirements = ();
    type Runtime<Profile: HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = super::ProviderExternalView<Payload>;
    type ImmediateListInput = Self::ImmediateInput;
    type OwnedInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = super::ProviderExternalListDecoder<Payload>;
    type OwnedListDecoder = super::ProviderOwnedExternalListDecoder<Payload>;
}

impl<Profile, Provider, Return, Payload> ProviderInputValue<Profile, Provider, Return>
    for super::ProviderExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    type Host = Payload::Host;

    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
        _constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Payload::immediate_input(call, value)
    }
}

impl<Profile, Provider, Return, Payload> ProviderInputValue<Profile, Provider, Return>
    for super::ProviderOwnedExternal<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    type Host = Payload::Host;

    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
        _constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Payload::owned_input(call, value)
    }
}

impl<Payload> ProviderListInputValue for super::ProviderExternalView<Payload>
where
    Payload: ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
    type View = Self;
    type Decoder = super::ProviderExternalListDecoder<Payload>;
}

impl<Payload> ProviderListInputValue for super::ProviderOwnedExternal<Payload>
where
    Payload: ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
    type View = Self;
    type Decoder = super::ProviderOwnedExternalListDecoder<Payload>;
}

impl<Profile, Provider, Payload> ProviderListInputCodec<Profile, Provider>
    for super::ProviderExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderExternalCodec<Profile>,
{
    type Requirements = ProviderNoConstructions;

    fn decoder_with<'call, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder
    where
        Return: HostType,
    {
        Payload::immediate_list_decoder(call)
    }
}

impl<Profile, Provider, Payload> ProviderListInputCodec<Profile, Provider>
    for super::ProviderOwnedExternal<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderExternalCodec<Profile>,
{
    type Requirements = ProviderNoConstructions;

    fn decoder_with<'call, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self::Decoder
    where
        Return: HostType,
    {
        Payload::owned_list_decoder(call)
    }
}

impl<Profile, Provider, Return, Payload> ProviderOutputValue<Profile, Provider, Return>
    for super::ProviderExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    type Error = std::convert::Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
        Ok(Payload::immediate_output(call, self))
    }
}

impl<Profile, Provider, Return, Payload> ProviderOutputValue<Profile, Provider, Return>
    for super::ProviderOwnedExternal<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    type Error = std::convert::Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
        Ok(Payload::owned_output(call, self))
    }
}

impl<Profile, Provider, Payload> ProviderRootOutputValue<Profile, Provider>
    for super::ProviderExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderExternalCodec<Profile>,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, Self::Host>,
        _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
        let value = Payload::immediate_output(&mut call, self);
        Ok(call.return_value(value))
    }
}

impl<Profile, Provider, Payload> ProviderRootOutputValue<Profile, Provider>
    for super::ProviderOwnedExternal<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderExternalCodec<Profile>,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, Self::Host>,
        _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, HostCallError> {
        let value = Payload::owned_output(&mut call, self);
        Ok(call.return_value(value))
    }
}

mod private {
    pub trait Requirements {}

    impl Requirements for super::ProviderNoConstructions {}

    impl<Type> Requirements for super::ProviderConstruction<Type> where Type: crate::HostType {}

    impl<Head, Tail> Requirements for super::ProviderConstructionList<Head, Tail>
    where
        Head: super::ProviderConstructionRequirements,
        Tail: super::ProviderConstructionRequirements,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProviderConstruction, ProviderConstructionList, ProviderConstructionRequirements,
        ProviderNoConstructions,
    };
    use crate::{HostListType, HostTypeList, HostTypeListEnd, StringValue};
    use num_bigint::BigInt;

    type Requirements = ProviderConstructionList<
        ProviderNoConstructions,
        ProviderConstructionList<
            ProviderConstruction<HostListType<BigInt>>,
            ProviderConstruction<HostListType<StringValue>>,
        >,
    >;
    type Expected = HostTypeList<
        HostListType<BigInt>,
        HostTypeList<HostListType<StringValue>, HostTypeListEnd>,
    >;

    #[test]
    fn construction_requirements_preserve_exact_type_order() {
        fn assert_types<Requirements>()
        where
            Requirements: ProviderConstructionRequirements<Types<HostTypeListEnd> = Expected>,
        {
        }

        assert_types::<Requirements>();
    }

    #[test]
    fn empty_requirement_groups_preserve_zero_construction_permission() {
        use super::{ProviderConstructionIndex0, ProviderConstructions};
        type Empty = ProviderConstructionList<
            ProviderConstructionList<ProviderNoConstructions, ProviderNoConstructions>,
            ProviderNoConstructions,
        >;
        let proof = ProviderConstructions::<Empty>::empty();
        let selected = proof.select::<ProviderConstructionIndex0>();
        assert_eq!(proof.host().callable_base(), 0);
        assert_eq!(selected.host().callable_base(), 0);
    }

    #[test]
    fn selected_native_constructions_keep_their_body_through_retention_and_resume() {
        use super::{
            ProviderConstructionIndex0, ProviderConstructionIndexNext, ProviderConstructions,
        };
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostCallable, HostCallableSchema,
            HostCaptures, HostConstructions, HostCreatedFunction, HostFunctionType,
            HostOwnedCompletion, HostProvider, HostProviderModule, HostProviderSet, HostTypeIndex0,
            ModuleSource, PackageSource, StatelessHostProfile,
        };
        type End = HostTypeListEnd;
        type One<T> = HostTypeList<T, End>;
        type Thunk = HostFunctionType<End, BigInt>;
        struct Add<const OFFSET: usize>;
        impl<const OFFSET: usize> HostCallableSchema for Add<OFFSET> {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = if OFFSET == 10 { "first" } else { "second" };
            type Arguments = End;
            type Return = BigInt;
            type Captures = One<BigInt>;
            type Constructions = End;
            type Completion = crate::HostReturns;
        }
        type Second = ProviderConstruction<HostCreatedFunction<Add<100>>>;
        type Group = ProviderConstructionList<
            ProviderConstruction<HostListType<BigInt>>,
            ProviderConstructionList<Second, ProviderNoConstructions>,
        >;
        type Requirements = ProviderConstructionList<
            ProviderConstruction<HostCreatedFunction<Add<10>>>,
            ProviderConstructionList<Group, ProviderNoConstructions>,
        >;
        type Constructions = <Requirements as ProviderConstructionRequirements>::Types<End>;
        type Next = ProviderConstructionIndexNext<ProviderConstructionIndex0>;
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn body<'call, const OFFSET: usize>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            captures: HostCaptures<'call, One<BigInt>>,
            _: HostConstructions<'call, End>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            let (value, ()) = call.captures(captures);
            Ok(call.return_value(value + OFFSET))
        }
        fn immediate<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk>,
            constructions: HostConstructions<'call, Constructions>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, Thunk>, HostCallError> {
            let selected = ProviderConstructions::<Requirements>::new(&constructions)
                .select::<Next>()
                .select::<Next>();
            let callback = call.construct_function(selected.token(), (value, ()));
            Ok(call.return_value(callback))
        }
        fn resumable<'call>(
            call: HostCall<'call, StatelessHostProfile, Provider, Thunk>,
            constructions: HostConstructions<'call, Constructions>,
            value: BigInt,
        ) -> Result<crate::HostCallContinuation<'call, Thunk>, HostCallError> {
            let selected = ProviderConstructions::<Requirements>::new(&constructions)
                .select::<Next>()
                .select::<Next>();
            Ok(call.resume(selected.host(), move |_| {
                Box::pin(async move {
                    Ok(HostOwnedCompletion::new(move |mut call, constructions| {
                        let callback = call
                            .construct_function(constructions.at::<HostTypeIndex0>(), (value, ()));
                        Ok(call.return_value(callback))
                    }))
                })
            }))
        }
        fn retained<'call>(
            call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            constructions: HostConstructions<'call, Constructions>,
            callback: HostCallable<'call, One<Thunk>, BigInt>,
        ) -> Result<crate::HostCallContinuation<'call, BigInt>, HostCallError> {
            let selected = ProviderConstructions::<Requirements>::new(&constructions)
                .select::<Next>()
                .select::<Next>();
            let original = call.owned_callable(callback, &selected.host());
            let callback = original.clone();
            drop(original);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let value = callback
                        .invoke(
                            &context,
                            |mut call, constructions| {
                                let callback = call.construct_function(
                                    constructions.at::<HostTypeIndex0>(),
                                    (7.into(), ()),
                                );
                                (callback, ())
                            },
                            |_, _, value| Ok(value),
                        )
                        .await
                        .unwrap();
                    Ok(HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_scoped_function_and_constructions::<Provider, (BigInt,), Thunk, Constructions, _>(
                "immediate",
                immediate,
            )
            .unwrap()
            .with_resumable_function::<Provider, (BigInt,), Thunk, Constructions, _>(
                "resumable", resumable,
            )
            .unwrap()
            .with_resumable_function::<Provider, (HostFunctionType<One<Thunk>, BigInt>,), BigInt, Constructions, _>("retained", retained)
            .unwrap()
            .with_callable::<Provider, Add<10>, (), _>(body::<10>)
            .unwrap()
            .with_callable::<Provider, Add<100>, (), _>(body::<100>)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
@external(erlang, "native", "immediate") fn immediate(value: Int) -> fn() -> Int
@external(erlang, "native", "resumable") fn resumable(value: Int) -> fn() -> Int
@external(erlang, "native", "retained") fn retained(callback: fn(fn() -> Int) -> Int) -> Int
pub fn main() { #(immediate(5)(), resumable(6)(), retained(fn(next) { next() + 1 })) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echoes = Vec::new();
        let result = crate::execution_fixture::run(&mut execution, &mut (), &mut echoes).unwrap();
        assert_eq!(result.inspect().to_string(), "#(105, 106, 108)");
        assert!(echoes.is_empty());
    }
    #[test]
    fn dynamic_storage_preserves_conversion_failures_without_retaining_a_value() {
        use crate::host::test::{TestHostProfile, TestRunState};
        use crate::provider::{
            ProviderConstructions, ProviderOutputValue, ProviderStoredOwner, ProviderValue,
        };
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostFailure, HostProvider, HostType,
        };

        struct Provider;
        impl HostProvider<TestHostProfile> for Provider {
            type State = TestRunState;
            fn project(state: &mut TestRunState) -> &mut TestRunState {
                state
            }
        }
        struct Owner;
        impl ProviderStoredOwner for Owner {}
        struct Converted(bool);
        impl ProviderValue for Converted {
            type Host = BigInt;
            type OutputRequirements = ProviderNoConstructions;
            type RootRequirements = ProviderNoConstructions;
        }
        impl<Return: HostType> ProviderOutputValue<TestHostProfile, Provider, Return> for Converted {
            type Error = HostCallError;
            fn into_host<'call>(
                self,
                call: &mut HostCall<'call, TestHostProfile, Provider, Return>,
                _: &ProviderConstructions<'call, ProviderNoConstructions>,
            ) -> Result<BigInt, HostCallError> {
                call.state().counter += 1;
                if self.0 {
                    Err(HostFailure::new("conversion stopped").into())
                } else {
                    Ok(42.into())
                }
            }
        }
        fn convert<'call>(
            mut call: HostCall<'call, TestHostProfile, Provider, BigInt>,
            fails: bool,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let stored = Converted(fails)
                .store_dynamic::<Owner>(&mut call, &ProviderConstructions::none())?;
            let value = call.restore_value::<BigInt>(stored.stored());
            Ok(call.return_value(value))
        }
        for (argument, expected) in [
            ("False", Ok(crate::Value::Int(42.into()))),
            (
                "True",
                Err(
                    "host function application::main.convert failed: conversion stopped".to_owned(),
                ),
            ),
        ] {
            let provider = crate::HostProviderModule::new("application", "main")
                .unwrap()
                .with_scoped_function::<Provider, (bool,), BigInt, _>("convert", convert)
                .unwrap();
            let source = format!(
                r#"
@external(erlang, "native", "convert")
fn convert(fails: Bool) -> Int
pub fn main() {{ convert({argument}) }}
"#
            );
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [crate::ModuleSource::new("main", "main.gleam", source)],
                )],
                crate::HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let mut state = TestRunState::default();
            let mut echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut execution, &mut state, &mut echo)
                    .map_err(|error| error.to_string()),
                expected
            );
            assert_eq!(state.counter, 1);
            assert!(echo.is_empty());
        }
    }
}
