use crate::host::{
    AsyncHostCallError, HostCall, HostCallCompletion, HostCallError, HostConstruction,
    HostConstructions, HostProfile, HostProvider, HostType, HostTypeList, HostTypeListEnd,
    HostTypeSequence, TransferHostCall,
};
use std::marker::PhantomData;

/// Static host type selected by one generated provider value declaration.
#[doc(hidden)]
pub trait ProviderValue {
    type Host: HostType;
    type Input;
    type ListInput;
    type OutputRequirements: ProviderConstructionRequirements;
    type RootRequirements: ProviderConstructionRequirements;
}

/// Transferable input forms selected beside one ordinary provider value.
#[doc(hidden)]
pub trait ProviderTransferValue: ProviderValue {
    type Output: ProviderTransferValue<Host = Self::Host, Output = Self::Output> + 'static;
    type ImmediateInput;
    type ImmediateListInput;
    type TransferInput;
    type TransferListInput;
}

/// Defers one payload's transfer requirement until a transferable profile is
/// actually selected.
///
/// The profile parameter keeps macro-generated local-only providers valid even
/// when their payload is not `Send`; selecting an async composition requires
/// this bound and therefore rejects that payload statically.
#[doc(hidden)]
pub trait ProviderTransferPayload<Profile>: Send + 'static {}

impl<Profile, Payload> ProviderTransferPayload<Profile> for Payload where Payload: Send + 'static {}

/// Owned conversion from one transferable host value into a provider input.
#[doc(hidden)]
pub trait ProviderTransferInputValue<Profile, Provider, Return>: Sized
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host: HostType;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self;
}

/// Conversion from an owned provider value into transferable execution.
#[doc(hidden)]
pub trait ProviderTransferOutputValue<Profile, Provider, Return>: ProviderValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    fn into_host<'call>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call>;
}

/// Root completion for an owned provider value in transferable execution.
#[doc(hidden)]
pub trait ProviderTransferRootOutputValue<Profile, Provider>: ProviderValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn complete<'call>(
        self,
        call: TransferHostCall<'call, Profile, Provider, Self::Host>,
        constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, AsyncHostCallError>;
}

/// Transferable List item view and decoder selected by one declaration.
#[doc(hidden)]
pub trait ProviderTransferListInputValue: Sized {
    type Host: HostType;
    type View;
    type Decoder: super::ProviderTransferListItemDecoder<Self, View = Self::View> + Clone;
}

/// Construction of one transferable List item decoder.
#[doc(hidden)]
pub trait ProviderTransferListInputCodec<Profile, Provider>:
    ProviderTransferListInputValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn decoder<Return>(call: &TransferHostCall<'_, Profile, Provider, Return>) -> Self::Decoder
    where
        Return: HostType;
}

/// Call-scoped conversion from one host value into its Rust input representation.
#[doc(hidden)]
pub trait ProviderInputValue<Profile, Provider, Return>: ProviderValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self;
}

/// Static conversion from one owned Rust value into its call-scoped host value.
#[doc(hidden)]
pub trait ProviderOutputValue<Profile, Provider, Return>: ProviderValue
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call>;
}

/// Static conversion from one owned Rust value into a function's root return.
#[doc(hidden)]
pub trait ProviderRootOutputValue<Profile, Provider>: ProviderValue
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

/// Profile-specific external input access generated beside one declaration.
#[doc(hidden)]
pub trait ProviderExternalCodec<Profile>: ProviderValue + Sized + 'static
where
    Profile: HostProfile,
{
    fn input<'call, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> super::ProviderExternalItem<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn output<'call, Provider, Return>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: super::ProviderExternalItem<Self>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;
}

/// Profile-specific transferable external access generated beside one declaration.
#[doc(hidden)]
pub trait ProviderTransferExternalCodec<Profile>: ProviderValue + Sized + Send + 'static
where
    Profile: HostProfile,
{
    fn immediate_input<'call, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> super::ProviderTransferExternalView<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn transfer_input<'call, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> super::ProviderTransferExternalItem<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn immediate_list_decoder<Provider, Return>(
        call: &TransferHostCall<'_, Profile, Provider, Return>,
    ) -> super::ProviderTransferExternalViewListDecoder<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn transfer_list_decoder<Provider, Return>(
        call: &TransferHostCall<'_, Profile, Provider, Return>,
    ) -> super::ProviderTransferExternalListDecoder<Self>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn immediate_output<'call, Provider, Return>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: super::ProviderTransferExternalView<Self>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;

    fn transfer_output<'call, Provider, Return>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: super::ProviderTransferExternalItem<Self>,
    ) -> <Self::Host as HostType>::Value<'call>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;
}

/// Profile-independent List item view and decoder selected by one declaration.
#[doc(hidden)]
pub trait ProviderListInputValue: ProviderValue + Sized {
    type View;
    type Decoder: super::ProviderListItemDecoder<Self, View = Self::View> + Clone;
}

/// Profile-specific construction of one statically selected List item decoder.
#[doc(hidden)]
pub trait ProviderListInputCodec<Profile>: ProviderListInputValue
where
    Profile: HostProfile,
{
    fn decoder<'call, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
    ) -> Self::Decoder
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
    type Requirement: ProviderConstructionRequirements;
}

impl<Head, Tail> ProviderConstructionRequirementAt<ProviderConstructionIndex0>
    for ProviderConstructionList<Head, Tail>
where
    Head: ProviderConstructionRequirements,
    Tail: ProviderConstructionRequirements,
{
    type Requirement = Head;
}

impl<Head, Tail, Index> ProviderConstructionRequirementAt<ProviderConstructionIndexNext<Index>>
    for ProviderConstructionList<Head, Tail>
where
    Head: ProviderConstructionRequirements,
    Tail: ProviderConstructionRequirementAt<Index>,
{
    type Requirement = Tail::Requirement;
}

/// Call-scoped proof of one exact generated construction requirement tree.
#[doc(hidden)]
pub struct ProviderConstructions<'call, Requirements>
where
    Requirements: ProviderConstructionRequirements,
{
    marker: PhantomData<fn(&'call ()) -> Requirements>,
}

impl<Requirements> Clone for ProviderConstructions<'_, Requirements>
where
    Requirements: ProviderConstructionRequirements,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Requirements> Copy for ProviderConstructions<'_, Requirements> where
    Requirements: ProviderConstructionRequirements
{
}

impl<'call, Requirements> ProviderConstructions<'call, Requirements>
where
    Requirements: ProviderConstructionRequirements,
{
    pub fn new(
        _constructions: &HostConstructions<'call, Requirements::Types<HostTypeListEnd>>,
    ) -> Self {
        Self {
            marker: PhantomData,
        }
    }

    pub(crate) fn host(&self) -> HostConstructions<'call, Requirements::Types<HostTypeListEnd>> {
        HostConstructions::new()
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
            marker: PhantomData,
        }
    }
}

impl<'call, Type> ProviderConstructions<'call, ProviderConstruction<Type>>
where
    Type: HostType,
{
    pub fn token(&self) -> HostConstruction<'call, Type> {
        HostConstructions::<HostTypeList<Type, HostTypeListEnd>>::new()
            .at::<crate::HostTypeIndex0>()
    }
}

impl<'call> ProviderConstructions<'call, ProviderNoConstructions> {
    pub fn none() -> Self {
        Self {
            marker: PhantomData,
        }
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
            type Input = Self;
            type ListInput = Self;
            type OutputRequirements = ProviderNoConstructions;
            type RootRequirements = ProviderNoConstructions;
        }

        impl ProviderTransferValue for $type {
            type Output = Self;
            type ImmediateInput = Self;
            type ImmediateListInput = Self;
            type TransferInput = Self;
            type TransferListInput = Self;
        }

        impl<Profile, Provider, Return> ProviderTransferInputValue<Profile, Provider, Return>
            for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostType,
        {
            type Host = Self;

            fn from_host<'call>(
                _call: &mut TransferHostCall<'call, Profile, Provider, Return>,
                value: <Self::Host as HostType>::Value<'call>,
            ) -> Self {
                value
            }
        }

        impl<Profile, Provider, Return> ProviderTransferOutputValue<Profile, Provider, Return>
            for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostType,
        {
            fn into_host<'call>(
                self,
                _call: &mut TransferHostCall<'call, Profile, Provider, Return>,
                _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
            ) -> <Self::Host as HostType>::Value<'call> {
                self
            }
        }

        impl<Profile, Provider> ProviderTransferRootOutputValue<Profile, Provider> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
        {
            fn complete<'call>(
                self,
                call: TransferHostCall<'call, Profile, Provider, Self::Host>,
                _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
            ) -> Result<HostCallCompletion<'call, Self::Host>, AsyncHostCallError> {
                Ok(call.return_value(self))
            }
        }

        impl ProviderTransferListInputValue for $type {
            type Host = Self;
            type View = Self;
            type Decoder = super::ProviderScalarListDecoder<Self>;
        }

        impl<Profile, Provider> ProviderTransferListInputCodec<Profile, Provider> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
        {
            fn decoder<Return>(
                _call: &TransferHostCall<'_, Profile, Provider, Return>,
            ) -> Self::Decoder
            where
                Return: HostType,
            {
                super::ProviderScalarListDecoder::new()
            }
        }

        impl<Profile, Provider, Return> ProviderInputValue<Profile, Provider, Return> for $type
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostType,
        {
            fn from_host<'call>(
                _call: &mut HostCall<'call, Profile, Provider, Return>,
                value: <Self::Host as HostType>::Value<'call>,
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
            fn into_host<'call>(
                self,
                _call: &mut HostCall<'call, Profile, Provider, Return>,
                _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
            ) -> <Self::Host as HostType>::Value<'call> {
                self
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
            type View = Self;
            type Decoder = super::ProviderScalarListDecoder<Self>;
        }

        impl<Profile> ProviderListInputCodec<Profile> for $type
        where
            Profile: HostProfile,
        {
            fn decoder<'call, Provider, Return>(
                _call: &HostCall<'call, Profile, Provider, Return>,
            ) -> Self::Decoder
            where
                Provider: HostProvider<Profile>,
                Return: HostType,
            {
                super::ProviderScalarListDecoder::new()
            }
        }
    };
}

provider_scalar!(num_bigint::BigInt);
provider_scalar!(f64);
provider_scalar!(ecow::EcoString);
provider_scalar!(crate::BitArrayValue);
provider_scalar!(char);
provider_scalar!(bool);
provider_scalar!(());

impl<Payload> ProviderValue for super::ProviderExternalItem<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Host = Payload::Host;
    type Input = Self;
    type ListInput = Self;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Payload> ProviderTransferValue for super::ProviderExternalItem<Payload>
where
    Payload: ProviderTransferValue + 'static,
{
    type Output = super::ProviderTransferExternalItem<Payload::Output>;
    type ImmediateInput = super::ProviderTransferExternalView<Payload::Output>;
    type ImmediateListInput = super::ProviderTransferExternalView<Payload::Output>;
    type TransferInput = super::ProviderTransferExternalItem<Payload::Output>;
    type TransferListInput = super::ProviderTransferExternalItem<Payload::Output>;
}

impl<Payload> ProviderValue for super::ProviderTransferExternalView<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Host = Payload::Host;
    type Input = Self;
    type ListInput = Self;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Payload> ProviderValue for super::ProviderTransferExternalItem<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Host = Payload::Host;
    type Input = Self;
    type ListInput = Self;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Payload> ProviderTransferValue for super::ProviderTransferExternalView<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type TransferInput = super::ProviderTransferExternalItem<Payload>;
    type TransferListInput = Self::TransferInput;
}

impl<Payload> ProviderTransferValue for super::ProviderTransferExternalItem<Payload>
where
    Payload: ProviderValue + 'static,
{
    type Output = Self;
    type ImmediateInput = super::ProviderTransferExternalView<Payload>;
    type ImmediateListInput = Self::ImmediateInput;
    type TransferInput = Self;
    type TransferListInput = Self;
}

impl<Profile, Provider, Return, Payload> ProviderTransferInputValue<Profile, Provider, Return>
    for super::ProviderTransferExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    type Host = Payload::Host;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self {
        Payload::immediate_input(call, value)
    }
}

impl<Profile, Provider, Return, Payload> ProviderTransferInputValue<Profile, Provider, Return>
    for super::ProviderTransferExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    type Host = Payload::Host;

    fn from_host<'call>(
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self {
        Payload::transfer_input(call, value)
    }
}

impl<Payload> ProviderTransferListInputValue for super::ProviderTransferExternalView<Payload>
where
    Payload: ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
    type View = Self;
    type Decoder = super::ProviderTransferExternalViewListDecoder<Payload>;
}

impl<Payload> ProviderTransferListInputValue for super::ProviderTransferExternalItem<Payload>
where
    Payload: ProviderValue + Send + 'static,
{
    type Host = Payload::Host;
    type View = Self;
    type Decoder = super::ProviderTransferExternalListDecoder<Payload>;
}

impl<Profile, Provider, Payload> ProviderTransferListInputCodec<Profile, Provider>
    for super::ProviderTransferExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn decoder<Return>(call: &TransferHostCall<'_, Profile, Provider, Return>) -> Self::Decoder
    where
        Return: HostType,
    {
        Payload::immediate_list_decoder(call)
    }
}

impl<Profile, Provider, Payload> ProviderTransferListInputCodec<Profile, Provider>
    for super::ProviderTransferExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn decoder<Return>(call: &TransferHostCall<'_, Profile, Provider, Return>) -> Self::Decoder
    where
        Return: HostType,
    {
        Payload::transfer_list_decoder(call)
    }
}

impl<Profile, Provider, Return, Payload> ProviderTransferOutputValue<Profile, Provider, Return>
    for super::ProviderTransferExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn into_host<'call>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call> {
        Payload::immediate_output(call, self)
    }
}

impl<Profile, Provider, Return, Payload> ProviderTransferOutputValue<Profile, Provider, Return>
    for super::ProviderTransferExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn into_host<'call>(
        self,
        call: &mut TransferHostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call> {
        Payload::transfer_output(call, self)
    }
}

impl<Profile, Provider, Payload> ProviderTransferRootOutputValue<Profile, Provider>
    for super::ProviderTransferExternalView<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn complete<'call>(
        self,
        mut call: TransferHostCall<'call, Profile, Provider, Self::Host>,
        _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, AsyncHostCallError> {
        let value = Payload::immediate_output(&mut call, self);
        Ok(call.return_value(value))
    }
}

impl<Profile, Provider, Payload> ProviderTransferRootOutputValue<Profile, Provider>
    for super::ProviderTransferExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Payload: ProviderTransferExternalCodec<Profile>,
{
    fn complete<'call>(
        self,
        mut call: TransferHostCall<'call, Profile, Provider, Self::Host>,
        _constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, Self::Host>, AsyncHostCallError> {
        let value = Payload::transfer_output(&mut call, self);
        Ok(call.return_value(value))
    }
}

impl<Profile, Provider, Return, Payload> ProviderOutputValue<Profile, Provider, Return>
    for super::ProviderExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        _constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> <Self::Host as HostType>::Value<'call> {
        Payload::output(call, self)
    }
}

impl<Profile, Provider, Payload> ProviderRootOutputValue<Profile, Provider>
    for super::ProviderExternalItem<Payload>
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
        let value = Payload::output(&mut call, self);
        Ok(call.return_value(value))
    }
}

impl<Profile, Provider, Return, Payload> ProviderInputValue<Profile, Provider, Return>
    for super::ProviderExternalItem<Payload>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Payload: ProviderExternalCodec<Profile>,
{
    fn from_host<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: <Self::Host as HostType>::Value<'call>,
    ) -> Self {
        Payload::input(call, value)
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
    use crate::{HostListType, HostTypeList, HostTypeListEnd};
    use ecow::EcoString;
    use num_bigint::BigInt;

    type Requirements = ProviderConstructionList<
        ProviderNoConstructions,
        ProviderConstructionList<
            ProviderConstruction<HostListType<BigInt>>,
            ProviderConstruction<HostListType<EcoString>>,
        >,
    >;
    type Expected =
        HostTypeList<HostListType<BigInt>, HostTypeList<HostListType<EcoString>, HostTypeListEnd>>;

    #[test]
    fn construction_requirements_preserve_exact_type_order() {
        fn assert_types<Requirements>()
        where
            Requirements: ProviderConstructionRequirements<Types<HostTypeListEnd> = Expected>,
        {
        }

        assert_types::<Requirements>();
    }
}
