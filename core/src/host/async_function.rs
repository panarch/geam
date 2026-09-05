use crate::host::{
    AsyncHostCall, AsyncHostCallError, AsyncHostCallable, AsyncHostCallbackArguments,
    AsyncHostFuture, AsyncHostRequestContext, HostArgument, HostCallArguments, HostFailure,
    HostFunctionType, HostParameter, HostParameterLayout, HostProfile, HostProvider, HostType,
    HostTypeDescriptor,
};
use crate::runtime::{ProfiledRetainedValues, TransferValues};
use crate::{BitArrayValue, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// An infallible host function that returns an owned, worker-transferable Future.
///
/// Compatible Rust functions and closures implement this trait automatically;
/// use [`crate::AsyncHostModule::with_async_function`] or
/// [`crate::AsyncHostProviderModule::with_async_function`] to register one.
pub trait AsyncHostFunction<Arguments, Return, HostFuture>:
    AsyncHostFunctionAdapter<Arguments, Return, HostFuture> + Send + Sync + 'static
where
    HostFuture: Future<Output = Return> + Send + 'static,
{
}

/// A fallible host function that returns an owned, worker-transferable Future.
///
/// The Future reports application failures as [`HostFailure`]. Compatible Rust
/// functions and closures implement this trait automatically.
pub trait FallibleAsyncHostFunction<Arguments, Return, HostFuture>:
    FallibleAsyncHostFunctionAdapter<Arguments, Return, HostFuture> + Send + Sync + 'static
where
    HostFuture: Future<Output = Result<Return, HostFailure>> + Send + 'static,
{
}

/// An async host function with bounded access to caller-owned state and Gleam callbacks.
///
/// Compatible higher-ranked Rust functions implement this trait automatically.
/// They receive an [`AsyncHostCall`] and return an [`AsyncHostFuture`] whose
/// lifetime is limited to that host invocation.
pub trait ScopedAsyncHostFunction<Profile, Provider, Arguments, Return>:
    ScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return> + Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

/// A fallible scoped async host function.
///
/// This combines [`ScopedAsyncHostFunction`] capabilities with an
/// [`AsyncHostCallError`] result for explicit host failures or nested callback
/// failures.
pub trait FallibleScopedAsyncHostFunction<Profile, Provider, Arguments, Return>:
    FallibleScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return> + Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

impl<Function, Arguments, Return, HostFuture> AsyncHostFunction<Arguments, Return, HostFuture>
    for Function
where
    Function: AsyncHostFunctionAdapter<Arguments, Return, HostFuture> + Send + Sync + 'static,
    HostFuture: Future<Output = Return> + Send + 'static,
{
}

impl<Function, Arguments, Return, HostFuture>
    FallibleAsyncHostFunction<Arguments, Return, HostFuture> for Function
where
    Function:
        FallibleAsyncHostFunctionAdapter<Arguments, Return, HostFuture> + Send + Sync + 'static,
    HostFuture: Future<Output = Result<Return, HostFailure>> + Send + 'static,
{
}

impl<Profile, Provider, Function, Arguments, Return>
    ScopedAsyncHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: ScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

impl<Profile, Provider, Function, Arguments, Return>
    FallibleScopedAsyncHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: FallibleScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

#[doc(hidden)]
pub trait AsyncHostFunctionAdapter<Arguments, Return, HostFuture>: Send + Sync + 'static
where
    HostFuture: Future<Output = Return> + Send + 'static,
{
    fn register<Profile: HostProfile>(self) -> AsyncHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub trait FallibleAsyncHostFunctionAdapter<Arguments, Return, HostFuture>:
    Send + Sync + 'static
where
    HostFuture: Future<Output = Result<Return, HostFailure>> + Send + 'static,
{
    fn register<Profile: HostProfile>(self) -> AsyncHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub trait ScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> AsyncHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub trait FallibleScopedAsyncHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> AsyncHostFunctionRegistration<Profile>;
}

pub(crate) type BoxAsyncHostFuture<'call, Return> =
    Pin<Box<dyn Future<Output = Result<Return, AsyncHostCallError>> + Send + 'call>>;

pub(crate) type AsyncHostCallback<Return> =
    dyn Fn(&dyn HostCallArguments) -> BoxAsyncHostFuture<'static, Return> + Send + Sync;

pub(crate) type ScopedAsyncHostCallback<Profile, Return> =
    dyn for<'call> Fn(
            AsyncHostRequestContext<'call, Profile>,
            &ProfiledRetainedValues<TransferValues>,
        ) -> ScopedAsyncHostFuture<'call, Return>
        + Send
        + Sync;

pub(crate) enum AsyncHostFunctionCallback<Profile: HostProfile, Return> {
    Owned(Arc<AsyncHostCallback<Return>>),
    Scoped(Arc<ScopedAsyncHostCallback<Profile, Return>>),
}

pub(crate) enum AsyncHostFunctionKind<Profile: HostProfile> {
    Int(AsyncHostFunctionCallback<Profile, BigInt>),
    Float(AsyncHostFunctionCallback<Profile, f64>),
    String(AsyncHostFunctionCallback<Profile, EcoString>),
    BitArray(AsyncHostFunctionCallback<Profile, BitArrayValue>),
    UtfCodepoint(AsyncHostFunctionCallback<Profile, char>),
    Bool(AsyncHostFunctionCallback<Profile, bool>),
    Nil(AsyncHostFunctionCallback<Profile, ()>),
    External(Arc<ScopedAsyncHostCallback<Profile, crate::runtime::TransferExternalPayloadLease>>),
}

pub(crate) struct AsyncHostFunctionImplementation<Profile: HostProfile> {
    kind: AsyncHostFunctionKind<Profile>,
}

impl<Profile: HostProfile> AsyncHostFunctionImplementation<Profile> {
    pub(crate) fn kind(&self) -> &AsyncHostFunctionKind<Profile> {
        &self.kind
    }
}

/// Inferred registration data for a compatible async Rust function.
///
/// Registration methods consume this type internally; application code does
/// not construct it directly.
#[doc(hidden)]
pub struct AsyncHostFunctionRegistration<Profile: HostProfile> {
    parameters: Box<[HostParameter]>,
    parameter_types: Box<[HostTypeDescriptor]>,
    return_type: HostTypeDescriptor,
    implementation: AsyncHostFunctionKind<Profile>,
}

pub(crate) struct AsyncHostFunctionDefinition<Profile: HostProfile> {
    schema: super::HostFunctionSchema,
    constructions: super::RegisteredHostConstructions,
    implementation: AsyncHostFunctionImplementation<Profile>,
}

pub(crate) enum ScopedAsyncHostFuture<'call, Return> {
    Infallible(AsyncHostFuture<'call, Return>),
    Fallible(AsyncHostFuture<'call, Result<Return, AsyncHostCallError>>),
}

impl<Return> Future for ScopedAsyncHostFuture<'_, Return> {
    type Output = Result<Return, AsyncHostCallError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Self::Infallible(future) => Pin::new(future).poll(context).map(Ok),
            Self::Fallible(future) => Pin::new(future).poll(context),
        }
    }
}

trait AsyncHostReturn: Sized {
    fn descriptor() -> HostTypeDescriptor;

    fn owned<Profile: HostProfile>(
        function: impl Fn(&dyn HostCallArguments) -> BoxAsyncHostFuture<'static, Self>
        + Send
        + Sync
        + 'static,
    ) -> AsyncHostFunctionKind<Profile>;

    fn scoped<Profile: HostProfile>(
        function: impl for<'call> Fn(
            AsyncHostRequestContext<'call, Profile>,
            &ProfiledRetainedValues<TransferValues>,
        ) -> ScopedAsyncHostFuture<'call, Self>
        + Send
        + Sync
        + 'static,
    ) -> AsyncHostFunctionKind<Profile>;
}

macro_rules! async_host_return {
    ($type:ty, $variant:ident) => {
        impl AsyncHostReturn for $type {
            fn descriptor() -> HostTypeDescriptor {
                <$type as crate::host::HostAbiType>::descriptor()
            }

            fn owned<Profile: HostProfile>(
                function: impl Fn(&dyn HostCallArguments) -> BoxAsyncHostFuture<'static, Self>
                + Send
                + Sync
                + 'static,
            ) -> AsyncHostFunctionKind<Profile> {
                AsyncHostFunctionKind::<Profile>::$variant(AsyncHostFunctionCallback::Owned(
                    Arc::new(function),
                ))
            }

            fn scoped<Profile: HostProfile>(
                function: impl for<'call> Fn(
                    AsyncHostRequestContext<'call, Profile>,
                    &ProfiledRetainedValues<TransferValues>,
                ) -> ScopedAsyncHostFuture<'call, Self>
                + Send
                + Sync
                + 'static,
            ) -> AsyncHostFunctionKind<Profile> {
                AsyncHostFunctionKind::<Profile>::$variant(AsyncHostFunctionCallback::Scoped(
                    Arc::new(function),
                ))
            }
        }
    };
}

async_host_return!(BigInt, Int);
async_host_return!(f64, Float);
async_host_return!(EcoString, String);
async_host_return!(BitArrayValue, BitArray);
async_host_return!(char, UtfCodepoint);
async_host_return!(bool, Bool);
async_host_return!((), Nil);

trait AsyncHostScopedReturn<Profile: HostProfile, Provider: HostProvider<Profile>> {
    type Value<'call>;

    fn descriptor() -> HostTypeDescriptor;

    fn scoped(
        function: impl for<'call> Fn(
            AsyncHostRequestContext<'call, Profile>,
            &ProfiledRetainedValues<TransferValues>,
        ) -> ScopedAsyncHostFuture<'call, Self::Value<'call>>
        + Send
        + Sync
        + 'static,
    ) -> AsyncHostFunctionKind<Profile>;
}

impl<Profile, Provider, Return> AsyncHostScopedReturn<Profile, Provider> for Return
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: AsyncHostReturn,
{
    type Value<'call> = Return;

    fn descriptor() -> HostTypeDescriptor {
        Return::descriptor()
    }

    fn scoped(
        function: impl for<'call> Fn(
            AsyncHostRequestContext<'call, Profile>,
            &ProfiledRetainedValues<TransferValues>,
        ) -> ScopedAsyncHostFuture<'call, Return>
        + Send
        + Sync
        + 'static,
    ) -> AsyncHostFunctionKind<Profile> {
        Return::scoped(function)
    }
}

impl<Profile, Provider, Schema, Arguments> AsyncHostScopedReturn<Profile, Provider>
    for super::HostExternalType<Schema, Arguments>
where
    Profile: HostProfile,
    Schema: super::HostExternalSchema,
    Arguments: super::HostAbiTypeSequence,
    Provider: super::AsyncHostExternalBinding<Profile, Schema>,
{
    type Value<'call> = super::AsyncHostExternalReturn<'call, Schema, Arguments>;

    fn descriptor() -> HostTypeDescriptor {
        <Self as super::HostAbiType>::descriptor()
    }

    fn scoped(
        function: impl for<'call> Fn(
            AsyncHostRequestContext<'call, Profile>,
            &ProfiledRetainedValues<TransferValues>,
        ) -> ScopedAsyncHostFuture<'call, Self::Value<'call>>
        + Send
        + Sync
        + 'static,
    ) -> AsyncHostFunctionKind<Profile> {
        AsyncHostFunctionKind::External(Arc::new(move |context, arguments| {
            let future = function(context, arguments);
            ScopedAsyncHostFuture::Fallible(AsyncHostFuture::new(async move {
                future.await.map(super::AsyncHostExternalReturn::into_lease)
            }))
        }))
    }
}

#[derive(Default)]
struct AsyncHostParameterLayout {
    schema: HostParameterLayout,
    next_external: usize,
    next_int_callback: usize,
    next_float_callback: usize,
    next_string_callback: usize,
    next_bit_array_callback: usize,
    next_utf_codepoint_callback: usize,
    next_bool_callback: usize,
    next_nil_callback: usize,
}

#[derive(Clone, Copy)]
struct AsyncHostCallbackArgumentSlot {
    index: usize,
}

trait AsyncHostScopedArgument: HostType {
    type Slot: Copy + Send + Sync + 'static;
    type ScopedValue<'call, Profile: HostProfile>;

    fn register(layout: &mut AsyncHostParameterLayout) -> Self::Slot;

    fn read<'call, Profile: HostProfile>(
        arguments: &ProfiledRetainedValues<TransferValues>,
        slot: Self::Slot,
    ) -> Self::ScopedValue<'call, Profile>;
}

impl AsyncHostParameterLayout {
    fn register<Argument: AsyncHostScopedArgument>(&mut self) -> Argument::Slot {
        Argument::register(self)
    }

    fn finish(self) -> Box<[HostParameter]> {
        self.schema.finish()
    }
}

impl<T> AsyncHostScopedArgument for T
where
    T: HostArgument,
    for<'call> T: crate::host::HostAbiType<Value<'call> = T>,
{
    type Slot = T::Slot;
    type ScopedValue<'call, Profile: HostProfile> = T;

    fn register(layout: &mut AsyncHostParameterLayout) -> Self::Slot {
        layout.schema.register::<T>()
    }

    fn read<'call, Profile: HostProfile>(
        arguments: &ProfiledRetainedValues<TransferValues>,
        slot: Self::Slot,
    ) -> Self::ScopedValue<'call, Profile> {
        T::read(arguments, slot)
    }
}

macro_rules! async_host_callback_argument {
    ($return:ty, $next:ident, $read:ident) => {
        impl<Arguments> AsyncHostScopedArgument for HostFunctionType<Arguments, $return>
        where
            Arguments: AsyncHostCallbackArguments,
        {
            type Slot = AsyncHostCallbackArgumentSlot;
            type ScopedValue<'call, Profile: HostProfile> =
                AsyncHostCallable<'call, Profile, Arguments, $return>;

            fn register(layout: &mut AsyncHostParameterLayout) -> Self::Slot {
                let slot = AsyncHostCallbackArgumentSlot {
                    index: layout.$next,
                };
                layout.$next += 1;
                layout.schema.register_function_parameter();
                slot
            }

            fn read<'call, Profile: HostProfile>(
                arguments: &ProfiledRetainedValues<TransferValues>,
                slot: Self::Slot,
            ) -> Self::ScopedValue<'call, Profile> {
                AsyncHostCallable::new(arguments.$read(slot.index))
            }
        }
    };
}

async_host_callback_argument!(BigInt, next_int_callback, async_int_function);
async_host_callback_argument!(f64, next_float_callback, async_float_function);
async_host_callback_argument!(EcoString, next_string_callback, async_string_function);
async_host_callback_argument!(
    BitArrayValue,
    next_bit_array_callback,
    async_bit_array_function
);
async_host_callback_argument!(
    char,
    next_utf_codepoint_callback,
    async_utf_codepoint_function
);
async_host_callback_argument!(bool, next_bool_callback, async_bool_function);
async_host_callback_argument!((), next_nil_callback, async_nil_function);

impl<Schema, Arguments> AsyncHostScopedArgument for super::HostExternalType<Schema, Arguments>
where
    Schema: super::HostExternalSchema,
    Arguments: super::HostAbiTypeSequence,
{
    type Slot = usize;
    type ScopedValue<'call, Profile: HostProfile> =
        super::AsyncHostExternal<'call, Schema, Arguments>;

    fn register(layout: &mut AsyncHostParameterLayout) -> Self::Slot {
        let slot = layout.next_external;
        layout.next_external += 1;
        layout.schema.register_external_parameter();
        slot
    }

    fn read<'call, Profile: HostProfile>(
        arguments: &ProfiledRetainedValues<TransferValues>,
        index: Self::Slot,
    ) -> Self::ScopedValue<'call, Profile> {
        super::AsyncHostExternal::new(arguments.async_external(index))
    }
}

macro_rules! async_host_function {
    ($($argument:ident => $slot:ident),*) => {
        impl<Function, Return, HostFuture, $($argument,)*>
            AsyncHostFunctionAdapter<($($argument,)*), Return, HostFuture> for Function
        where
            Function: Fn($($argument),*) -> HostFuture + Send + Sync + 'static,
            Return: AsyncHostReturn,
            HostFuture: Future<Output = Return> + Send + 'static,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(self) -> AsyncHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::owned::<Profile>(move |_arguments| {
                    $(let $slot = $argument::read(_arguments, $slot);)*
                    let future = self($($slot),*);
                    Box::pin(async move { Ok(future.await) })
                });
                AsyncHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as crate::host::HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: Return::descriptor(),
                    implementation,
                }
            }
        }

        impl<Function, Return, HostFuture, $($argument,)*>
            FallibleAsyncHostFunctionAdapter<($($argument,)*), Return, HostFuture> for Function
        where
            Function: Fn($($argument),*) -> HostFuture + Send + Sync + 'static,
            Return: AsyncHostReturn,
            HostFuture: Future<Output = Result<Return, HostFailure>> + Send + 'static,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(self) -> AsyncHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::owned::<Profile>(move |_arguments| {
                    $(let $slot = $argument::read(_arguments, $slot);)*
                    let future = self($($slot),*);
                    Box::pin(async move { future.await.map_err(AsyncHostCallError::from) })
                });
                AsyncHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as crate::host::HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: Return::descriptor(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            ScopedAsyncHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return>
            for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    AsyncHostCall<'call, Profile, Provider, Return>,
                    $(<$argument as AsyncHostScopedArgument>::ScopedValue<'call, Profile>),*
                ) -> AsyncHostFuture<'call, <Return as AsyncHostScopedReturn<Profile, Provider>>::Value<'call>>
                + Send
                + Sync
                + 'static,
            Return: AsyncHostScopedReturn<Profile, Provider>,
            $($argument: AsyncHostScopedArgument,)*
        {
            fn register(self) -> AsyncHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = AsyncHostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::scoped(move |context, _arguments| {
                    $(let $slot = <$argument as AsyncHostScopedArgument>::read::<Profile>(_arguments, $slot);)*
                    ScopedAsyncHostFuture::Infallible(self(
                        AsyncHostCall::new(context),
                        $($slot),*
                    ))
                });
                AsyncHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as crate::host::HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: Return::descriptor(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            FallibleScopedAsyncHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return>
            for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    AsyncHostCall<'call, Profile, Provider, Return>,
                    $(<$argument as AsyncHostScopedArgument>::ScopedValue<'call, Profile>),*
                ) -> AsyncHostFuture<'call, Result<<Return as AsyncHostScopedReturn<Profile, Provider>>::Value<'call>, AsyncHostCallError>>
                + Send
                + Sync
                + 'static,
            Return: AsyncHostScopedReturn<Profile, Provider>,
            $($argument: AsyncHostScopedArgument,)*
        {
            fn register(self) -> AsyncHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = AsyncHostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::scoped(move |context, _arguments| {
                    $(let $slot = <$argument as AsyncHostScopedArgument>::read::<Profile>(_arguments, $slot);)*
                    ScopedAsyncHostFuture::Fallible(self(
                        AsyncHostCall::new(context),
                        $($slot),*
                    ))
                });
                AsyncHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as crate::host::HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: Return::descriptor(),
                    implementation,
                }
            }
        }
    };
}

async_host_function!();
async_host_function!(A => a);
async_host_function!(A => a, B => b);
async_host_function!(A => a, B => b, C => c);
async_host_function!(A => a, B => b, C => c, D => d);
async_host_function!(A => a, B => b, C => c, D => d, E => e);
async_host_function!(A => a, B => b, C => c, D => d, E => e, F => f);
async_host_function!(A => a, B => b, C => c, D => d, E => e, F => f, G => g);

impl<Profile: HostProfile> AsyncHostFunctionDefinition<Profile> {
    pub(crate) fn new<Arguments, Return, Function, HostFuture>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: AsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Return> + Send + 'static,
    {
        Self::from_registration(
            name,
            <Function as AsyncHostFunctionAdapter<Arguments, Return, HostFuture>>::register::<
                Profile,
            >(function),
        )
    }

    pub(crate) fn new_fallible<Arguments, Return, Function, HostFuture>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Function: FallibleAsyncHostFunction<Arguments, Return, HostFuture>,
        HostFuture: Future<Output = Result<Return, HostFailure>> + Send + 'static,
    {
        Self::from_registration(
            name,
            <Function as FallibleAsyncHostFunctionAdapter<Arguments, Return, HostFuture>>::register::<
                Profile,
            >(function),
        )
    }

    pub(crate) fn new_scoped<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: ScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        Self::from_registration(
            name,
            <Function as ScopedAsyncHostFunctionAdapter<
                Profile,
                Provider,
                Arguments,
                Return,
            >>::register(function),
        )
    }

    pub(crate) fn new_fallible_scoped<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: FallibleScopedAsyncHostFunction<Profile, Provider, Arguments, Return>,
    {
        Self::from_registration(
            name,
            <Function as FallibleScopedAsyncHostFunctionAdapter<
                Profile,
                Provider,
                Arguments,
                Return,
            >>::register(function),
        )
    }

    fn from_registration(
        name: EcoString,
        registration: AsyncHostFunctionRegistration<Profile>,
    ) -> Result<Self, HostRegistrationError> {
        let schema = super::function::HostFunctionSchemaRegistration {
            layout: registration.parameters,
            parameters: registration.parameter_types,
            return_: registration.return_type,
            custom_schemas: Box::new([]),
        };
        Ok(Self {
            schema: super::HostFunctionSchema::from_registration(name, schema)?,
            constructions: super::RegisteredHostConstructions::empty(),
            implementation: AsyncHostFunctionImplementation::<Profile> {
                kind: registration.implementation,
            },
        })
    }

    pub(crate) fn schema(&self) -> &super::HostFunctionSchema {
        &self.schema
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::HostFunctionSchema,
        super::RegisteredHostConstructions,
        AsyncHostFunctionImplementation<Profile>,
    ) {
        (self.schema, self.constructions, self.implementation)
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncHostFunctionDefinition;
    use crate::HostRegistrationError;
    use crate::host::{CallArguments, HostTypeDescriptor};
    use num_bigint::BigInt;
    use std::future::ready;

    #[test]
    fn owned_async_registration_assembles_the_exact_schema() {
        let definition = AsyncHostFunctionDefinition::<crate::StatelessHostProfile>::new(
            "identity".into(),
            ready::<BigInt>,
        )
        .expect("async function should register");
        let (schema, _, implementation) = definition.into_parts();

        assert_eq!(schema.name(), "identity");
        drop(implementation);
        let _arguments = CallArguments::new(vec![BigInt::from(1)], Vec::new());
    }

    #[test]
    fn owned_async_registration_rejects_non_contiguous_type_parameters() {
        let mut registration =
            <_ as super::AsyncHostFunctionAdapter<(BigInt,), BigInt, _>>::register::<
                crate::StatelessHostProfile,
            >(ready::<BigInt>);
        registration.return_type = HostTypeDescriptor::Parameter(2);
        let error = AsyncHostFunctionDefinition::from_registration("identity".into(), registration)
            .err()
            .expect("sparse type parameters should be rejected");

        assert_eq!(
            error,
            HostRegistrationError::NonContiguousTypeParameters {
                function: "identity".into(),
                parameters: vec![2].into_boxed_slice(),
            },
        );
    }
}
