use super::argument::{HostArgument, HostParameter, HostParameterLayout, HostScopedArgument};
use super::return_::{HostFunctionImplementation, HostReturn, OwnedHostFunctionImplementation};
use crate::host::{
    HostAbiType, HostCall, HostCallCompletion, HostCallError, HostConstructions, HostFailure,
    HostProfile, HostProvider, HostTypeDescriptor, HostTypeSequence,
};

pub trait HostFunctionAdapter<Arguments, Return>: Send + Sync + 'static {
    fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile>;
}

pub trait FallibleHostFunctionAdapter<Arguments, Return>: Send + Sync + 'static {
    fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile>;
}

pub trait ScopedHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> ScopedHostFunctionRegistration<Profile>;
}

pub trait ScopedConstructingHostFunctionAdapter<Profile, Provider, Arguments, Return, Constructions>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    fn register(self) -> ScopedHostFunctionRegistration<Profile>;
}

pub trait ScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> ScopedHostFunctionRegistration<Profile>;
}

pub trait ResumableHostFunctionAdapter<Profile, Provider, Arguments, Return, Constructions>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    fn register(self) -> ScopedHostFunctionRegistration<Profile>;
}

macro_rules! resumable_function {
    (@layout $layout:ident;) => {
        let $layout = HostParameterLayout::default();
    };
    (@layout $layout:ident; $($argument:ident),+) => {
        let mut $layout = HostParameterLayout::default();
    };
    ($($argument:ident => $slot:ident),*) => {
        impl<Profile, Provider, Function, Return, Constructions, $($argument,)*>
            ResumableHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return, Constructions> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Constructions: HostTypeSequence,
            Return: HostAbiType,
            $($argument: HostScopedArgument,)*
            Function: for<'call> Fn(
                HostCall<'call, Profile, Provider, Return>,
                HostConstructions<'call, Constructions>,
                $(<$argument as crate::host::HostType>::Value<'call>,)*
            ) -> Result<crate::host::HostCallContinuation<'call, Return>, HostCallError> + Send + Sync + 'static,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                resumable_function!(@layout layout; $($argument),*);
                $(let $slot = <$argument as HostScopedArgument>::register(&mut layout);)*
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                $(<$argument as HostAbiType>::collect_custom_schemas(&mut custom_schemas, &mut visited);)*
                <Return as HostAbiType>::collect_custom_schemas(&mut custom_schemas, &mut visited);
                let implementation = HostFunctionImplementation::continuing(move |runtime| {
                    let call = HostCall::new(runtime);
                    $(let $slot = <$argument as HostScopedArgument>::read(&call, $slot);)*
                    self(call, HostConstructions::new(), $($slot,)*)
                        .map(|completion| completion.continuation)
                });
                ScopedHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }
    };
}

resumable_function!();
resumable_function!(A => a);
resumable_function!(A => a, B => b);
resumable_function!(A => a, B => b, C => c);
resumable_function!(A => a, B => b, C => c, D => d);
resumable_function!(A => a, B => b, C => c, D => d, E => e);
resumable_function!(A => a, B => b, C => c, D => d, E => e, F => f);
resumable_function!(A => a, B => b, C => c, D => d, E => e, F => f, G => g);

pub struct HostFunctionRegistration<Profile: HostProfile> {
    pub(super) parameters: Box<[HostParameter]>,
    pub(super) parameter_types: Box<[HostTypeDescriptor]>,
    pub(super) return_type: HostTypeDescriptor,
    pub(super) custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    pub(super) implementation: OwnedHostFunctionImplementation<Profile>,
}

pub struct ScopedHostFunctionRegistration<Profile: HostProfile> {
    pub(super) parameters: Box<[HostParameter]>,
    pub(super) parameter_types: Box<[HostTypeDescriptor]>,
    pub(super) return_type: HostTypeDescriptor,
    pub(super) custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    pub(super) implementation: HostFunctionImplementation<Profile>,
}

impl<Profile: HostProfile> HostFunctionRegistration<Profile> {
    pub(super) fn into_immediate(self) -> ScopedHostFunctionRegistration<Profile> {
        ScopedHostFunctionRegistration {
            parameters: self.parameters,
            parameter_types: self.parameter_types,
            return_type: self.return_type,
            custom_schemas: self.custom_schemas,
            implementation: self.implementation.into_immediate(),
        }
    }
}

macro_rules! native_function {
    ($($argument:ident => $slot:ident),*) => {
        impl<Profile, Provider, Return, Targets, Function, $($argument,)*>
            ScopedConstructingHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return, Targets>
            for crate::host::native::NativeFunction<Profile, Provider, Return, Targets, Function>
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostAbiType,
            Targets: HostTypeSequence,
            $($argument: HostScopedArgument,)*
            Function: for<'call> Fn(
                crate::host::native::NativeCall<'call, Profile, Provider, Return, Targets>,
                $(<$argument as crate::host::HostType>::Value<'call>),*
            ) -> Result<HostCallCompletion<'call, Return>, HostCallError> + Send + Sync + 'static,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                // Tie decoded arguments and completion to the same call lifetime.
                fn callback<Profile, Provider, Return, Targets, Function, $($argument,)*>(function: Function) -> Function
                where
                    Profile: HostProfile,
                    Provider: HostProvider<Profile>,
                    Return: HostAbiType,
                    Targets: HostTypeSequence,
                    $($argument: HostScopedArgument,)*
                    Function: for<'call> Fn(
                        HostCall<'call, Profile, Provider, Return>,
                        HostConstructions<'call, Targets>,
                        $(<$argument as crate::host::HostType>::Value<'call>),*
                    ) -> Result<HostCallCompletion<'call, Return>, HostCallError>,
                {
                    function
                }
                <_ as ScopedConstructingHostFunctionAdapter<
                    Profile, Provider, ($($argument,)*), Return, Targets,
                >>::register(callback::<Profile, Provider, Return, Targets, _, $($argument,)*>(move |call, _, $($slot),*| {
                        (self.function)(
                            crate::host::native::NativeCall::new(call, std::sync::Arc::clone(&self.rules)),
                            $($slot),*
                        )
                    }))
            }
        }

        impl<Profile, Provider, Return, Targets, Function, $($argument,)*>
            ResumableHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return, Targets>
            for crate::host::native::NativeFunction<Profile, Provider, Return, Targets, Function>
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Return: HostAbiType,
            Targets: HostTypeSequence,
            $($argument: HostScopedArgument,)*
            Function: for<'call> Fn(
                crate::host::native::NativeCall<'call, Profile, Provider, Return, Targets>,
                $(<$argument as crate::host::HostType>::Value<'call>),*
            ) -> Result<crate::host::HostCallContinuation<'call, Return>, HostCallError> + Send + Sync + 'static,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                fn callback<Profile, Provider, Return, Targets, Function, $($argument,)*>(function: Function) -> Function
                where
                    Profile: HostProfile,
                    Provider: HostProvider<Profile>,
                    Return: HostAbiType,
                    Targets: HostTypeSequence,
                    $($argument: HostScopedArgument,)*
                    Function: for<'call> Fn(
                        HostCall<'call, Profile, Provider, Return>,
                        HostConstructions<'call, Targets>,
                        $(<$argument as crate::host::HostType>::Value<'call>),*
                    ) -> Result<crate::host::HostCallContinuation<'call, Return>, HostCallError>,
                {
                    function
                }
                <_ as ResumableHostFunctionAdapter<
                    Profile, Provider, ($($argument,)*), Return, Targets,
                >>::register(callback::<Profile, Provider, Return, Targets, _, $($argument,)*>(move |call, _, $($slot),*| {
                    (self.function)(
                        crate::host::native::NativeCall::new(call, std::sync::Arc::clone(&self.rules)),
                        $($slot),*
                    )
                }))
            }
        }
    };
}

native_function!();
native_function!(A => a);
native_function!(A => a, B => b);
native_function!(A => a, B => b, C => c);
native_function!(A => a, B => b, C => c, D => d);
native_function!(A => a, B => b, C => c, D => d, E => e);
native_function!(A => a, B => b, C => c, D => d, E => e, F => f);
native_function!(A => a, B => b, C => c, D => d, E => e, F => f, G => g);

macro_rules! host_function {
    () => {
        impl<Function, Return> HostFunctionAdapter<(), Return> for Function
        where
            Function: Fn() -> Return + Send + Sync + 'static,
            Return: HostReturn,
        {
            fn register<Profile: HostProfile>(
                self,
            ) -> HostFunctionRegistration<Profile> {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation: Return::implementation(move |_, _| Ok(self())),
                }
            }
        }

        impl<Function, Return> FallibleHostFunctionAdapter<(), Return> for Function
        where
            Function: Fn() -> Result<Return, HostFailure> + Send + Sync + 'static,
            Return: HostReturn,
        {
            fn register<Profile: HostProfile>(
                self,
            ) -> HostFunctionRegistration<Profile> {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation: Return::implementation(move |_, _| self()),
                }
            }
        }

        impl<Profile, Provider, Function, Return>
            ScopedHostFunctionAdapter<Profile, Provider, (), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    HostCall<'call, Profile, Provider, Return>,
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                ScopedHostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation: HostFunctionImplementation::scoped(move |runtime| {
                        self(HostCall::new(runtime)).map(|completion| completion.token)
                    }),
                }
            }
        }

        impl<Profile, Provider, Function, Return, Constructions>
            ScopedConstructingHostFunctionAdapter<Profile, Provider, (), Return, Constructions>
            for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Constructions: HostTypeSequence,
            Function: for<'call> Fn(
                    HostCall<'call, Profile, Provider, Return>,
                    HostConstructions<'call, Constructions>,
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                ScopedHostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation: HostFunctionImplementation::scoped(move |runtime| {
                        self(HostCall::new(runtime), HostConstructions::new())
                            .map(|completion| completion.token)
                    }),
                }
            }
        }

        impl<Profile, Provider, Function, Return>
            ScopedDivergingHostFunctionAdapter<Profile, Provider, (), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    HostCall<'call, Profile, Provider, Return>,
                ) -> Result<std::convert::Infallible, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                ScopedHostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation: HostFunctionImplementation::scoped_never(move |runtime| {
                        self(HostCall::new(runtime))
                    }),
                }
            }
        }
    };
    ($($argument:ident => $slot:ident),+) => {
        impl<Function, Return, $($argument,)*> HostFunctionAdapter<($($argument,)*), Return> for Function
        where
            Function: Fn($($argument),*) -> Return + Send + Sync + 'static,
            Return: HostReturn,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(
                self,
            ) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |_, arguments| {
                    Ok(self($($argument::read(arguments, $slot)),*))
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation,
                }
            }
        }

        impl<Function, Return, $($argument,)*>
            FallibleHostFunctionAdapter<($($argument,)*), Return> for Function
        where
            Function: Fn($($argument),*) -> Result<Return, HostFailure> + Send + Sync + 'static,
            Return: HostReturn,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(
                self,
            ) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |_, arguments| {
                    self($($argument::read(arguments, $slot)),*)
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            ScopedHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                HostCall<'call, Profile, Provider, Return>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: HostScopedArgument,)*
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as HostScopedArgument>::register(&mut layout);)*
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                $(<$argument as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );)*
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                let implementation = HostFunctionImplementation::scoped(move |runtime| {
                    let call = HostCall::new(runtime);
                    $(let $slot = <$argument as HostScopedArgument>::read(&call, $slot);)*
                    self(
                        call,
                        $($slot),*
                    )
                    .map(|completion| completion.token)
                });
                ScopedHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, Constructions, $($argument,)*>
            ScopedConstructingHostFunctionAdapter<
                Profile,
                Provider,
                ($($argument,)*),
                Return,
                Constructions,
            > for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Constructions: HostTypeSequence,
            Function: for<'call> Fn(
                HostCall<'call, Profile, Provider, Return>,
                HostConstructions<'call, Constructions>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: HostScopedArgument,)*
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as HostScopedArgument>::register(&mut layout);)*
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                $(<$argument as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );)*
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                let implementation = HostFunctionImplementation::scoped(move |runtime| {
                    let call = HostCall::new(runtime);
                    $(let $slot = <$argument as HostScopedArgument>::read(&call, $slot);)*
                    self(
                        call,
                        HostConstructions::new(),
                        $($slot),*
                    )
                    .map(|completion| completion.token)
                });
                ScopedHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            ScopedDivergingHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                HostCall<'call, Profile, Provider, Return>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<std::convert::Infallible, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: HostScopedArgument,)*
        {
            fn register(self) -> ScopedHostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as HostScopedArgument>::register(&mut layout);)*
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                $(<$argument as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );)*
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                let implementation = HostFunctionImplementation::scoped_never(move |runtime| {
                    let call = HostCall::new(runtime);
                    $(let $slot = <$argument as HostScopedArgument>::read(&call, $slot);)*
                    self(
                        call,
                        $($slot),*
                    )
                });
                ScopedHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }
    };
}

host_function!();
host_function!(A0 => a0);
host_function!(A0 => a0, A1 => a1);
host_function!(A0 => a0, A1 => a1, A2 => a2);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
host_function!(
    A0 => a0,
    A1 => a1,
    A2 => a2,
    A3 => a3,
    A4 => a4,
    A5 => a5,
    A6 => a6
);

#[cfg(test)]
mod tests {
    use super::{
        FallibleHostFunctionAdapter, HostFunctionAdapter, ScopedConstructingHostFunctionAdapter,
        ScopedDivergingHostFunctionAdapter, ScopedHostFunctionAdapter,
    };
    use crate::BitArrayValue;
    use crate::host::function::HostFunctionImplementation;
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostFailure, HostListType,
        HostProvider, HostScopedValue, HostTypeDescriptor, HostTypeIndex0, HostTypeList,
        HostTypeListEnd, expect_never_implementation, expect_value_implementation,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::convert::Infallible;

    struct ScopedProvider;

    type Scoped0 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped1 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped2 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped3 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped4 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped5 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped6 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped7 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type ConstructionTypes = HostTypeList<HostListType<BigInt>, HostTypeListEnd>;
    type Constructing0 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing1 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing2 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing3 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing4 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing5 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
        (),
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing6 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
        (),
        (),
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Constructing7 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        HostConstructions<'call, ConstructionTypes>,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    )
        -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Diverging0 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
    ) -> Result<Infallible, HostCallError>;
    type Diverging1 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging2 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging3 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging4 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging5 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging6 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<Infallible, HostCallError>;
    type Diverging7 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<Infallible, HostCallError>;

    impl HostProvider<TestHostProfile> for ScopedProvider {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    #[test]
    fn supports_zero_arguments() {
        let registration = <_ as HostFunctionAdapter<(), BigInt>>::register(|| BigInt::from(7));
        let registration = registration.into_immediate();

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_type, HostTypeDescriptor::Int);
        assert_eq!(
            call_int(&registration.implementation, Vec::new(), Vec::new()),
            BigInt::from(7),
        );
    }

    #[test]
    fn supports_zero_argument_fallible_functions() {
        let registration = <_ as FallibleHostFunctionAdapter<(), BigInt>>::register::<
            TestHostProfile,
        >(|| Err(HostFailure::new("unavailable")));
        let registration = registration.into_immediate();
        let implementation = expect_value_implementation(&registration.implementation);

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_type, HostTypeDescriptor::Int);
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        assert_eq!(
            crate::host::expect_immediate_call(implementation, &mut runtime)
                .expect_err("fallible function should preserve its failure")
                .to_string(),
            "unavailable",
        );
    }

    #[test]
    fn supports_every_fallible_argument_arity() {
        let registrations = vec![
            <_ as FallibleHostFunctionAdapter<(), BigInt>>::register::<TestHostProfile>(|| {
                Ok::<_, HostFailure>(BigInt::from(0))
            }),
            <_ as FallibleHostFunctionAdapter<((),), BigInt>>::register::<TestHostProfile>(
                |_: ()| Ok::<_, HostFailure>(BigInt::from(1)),
            ),
            <_ as FallibleHostFunctionAdapter<((), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: ()| Ok::<_, HostFailure>(BigInt::from(2)),
            ),
            <_ as FallibleHostFunctionAdapter<((), (), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: (), _: ()| Ok::<_, HostFailure>(BigInt::from(3)),
            ),
            <_ as FallibleHostFunctionAdapter<((), (), (), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: (), _: (), _: ()| Ok::<_, HostFailure>(BigInt::from(4)),
            ),
            <_ as FallibleHostFunctionAdapter<((), (), (), (), ()), BigInt>>::register::<
                TestHostProfile,
            >(|_: (), _: (), _: (), _: (), _: ()| {
                Ok::<_, HostFailure>(BigInt::from(5))
            }),
            <_ as FallibleHostFunctionAdapter<((), (), (), (), (), ()), BigInt>>::register::<
                TestHostProfile,
            >(|_: (), _: (), _: (), _: (), _: (), _: ()| {
                Ok::<_, HostFailure>(BigInt::from(6))
            }),
            <_ as FallibleHostFunctionAdapter<((), (), (), (), (), (), ()), BigInt>>::register::<
                TestHostProfile,
            >(|_: (), _: (), _: (), _: (), _: (), _: (), _: ()| {
                Ok::<_, HostFailure>(BigInt::from(7))
            }),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            let registration = registration.into_immediate();
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            crate::host::expect_immediate_call(
                expect_value_implementation(&registration.implementation),
                &mut runtime,
            )
            .expect("fallible callback should succeed");
            assert_eq!(
                runtime.completed(),
                Some(&HostScopedValue::Int(BigInt::from(arity))),
            );
        }
    }

    #[test]
    fn supports_every_scoped_argument_arity() {
        let zero: Scoped0 = |mut call| {
            *call.state() += 1;
            Ok(call.return_value(BigInt::from(0)))
        };
        let one: Scoped1 = |call, _| Ok(call.return_value(BigInt::from(1)));
        let two: Scoped2 = |call, _, _| Ok(call.return_value(BigInt::from(2)));
        let three: Scoped3 = |call, _, _, _| Ok(call.return_value(BigInt::from(3)));
        let four: Scoped4 = |call, _, _, _, _| Ok(call.return_value(BigInt::from(4)));
        let five: Scoped5 = |call, _, _, _, _, _| Ok(call.return_value(BigInt::from(5)));
        let six: Scoped6 = |call, _, _, _, _, _, _| Ok(call.return_value(BigInt::from(6)));
        let seven: Scoped7 = |call, _, _, _, _, _, _, _| Ok(call.return_value(BigInt::from(7)));
        let registrations = vec![
            <_ as ScopedHostFunctionAdapter<TestHostProfile, ScopedProvider, (), BigInt>>::register(
                zero,
            ),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((),),
                BigInt,
            >>::register(one),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), ()),
                BigInt,
            >>::register(two),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), ()),
                BigInt,
            >>::register(three),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), ()),
                BigInt,
            >>::register(four),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), ()),
                BigInt,
            >>::register(five),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), ()),
                BigInt,
            >>::register(six),
            <_ as ScopedHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), (), ()),
                BigInt,
            >>::register(seven),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            crate::host::expect_immediate_call(
                expect_value_implementation(&registration.implementation),
                &mut runtime,
            )
            .expect("scoped callback should succeed");
            assert_eq!(
                runtime.completed(),
                Some(&HostScopedValue::Int(BigInt::from(arity))),
            );
            drop(runtime);
            assert_eq!(state.counter, usize::from(arity == 0));
        }
    }

    #[test]
    fn supports_every_scoped_constructing_argument_arity() {
        let zero: Constructing0 = |call, constructions| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(0)))
        };
        let one: Constructing1 = |call, constructions, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(1)))
        };
        let two: Constructing2 = |call, constructions, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(2)))
        };
        let three: Constructing3 = |call, constructions, _, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(3)))
        };
        let four: Constructing4 = |call, constructions, _, _, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(4)))
        };
        let five: Constructing5 = |call, constructions, _, _, _, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(5)))
        };
        let six: Constructing6 = |call, constructions, _, _, _, _, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(6)))
        };
        let seven: Constructing7 = |call, constructions, _, _, _, _, _, _, _| {
            let _ = constructions.at::<HostTypeIndex0>();
            Ok(call.return_value(BigInt::from(7)))
        };
        let registrations = vec![
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                (),
                BigInt,
                ConstructionTypes,
            >>::register(zero),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((),),
                BigInt,
                ConstructionTypes,
            >>::register(one),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), ()),
                BigInt,
                ConstructionTypes,
            >>::register(two),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), ()),
                BigInt,
                ConstructionTypes,
            >>::register(three),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), ()),
                BigInt,
                ConstructionTypes,
            >>::register(four),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), ()),
                BigInt,
                ConstructionTypes,
            >>::register(five),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), ()),
                BigInt,
                ConstructionTypes,
            >>::register(six),
            <_ as ScopedConstructingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), (), ()),
                BigInt,
                ConstructionTypes,
            >>::register(seven),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            crate::host::expect_immediate_call(
                expect_value_implementation(&registration.implementation),
                &mut runtime,
            )
            .expect("scoped constructing callback should succeed");
            assert_eq!(
                runtime.completed(),
                Some(&HostScopedValue::Int(BigInt::from(arity))),
            );
        }
    }

    #[test]
    fn supports_every_native_argument_arity_without_a_second_parameter_layout() {
        use crate::host::native::{NativeCall, NativeRules};
        type Call<'call> =
            NativeCall<'call, TestHostProfile, ScopedProvider, BigInt, HostTypeListEnd>;
        let provider = crate::HostProviderModule::new("application", "main").unwrap()
            .with_native_function::<ScopedProvider, (), BigInt, HostTypeListEnd, _>(
                "zero", NativeRules::default(), |mut call: Call<'_>| {
                    *call.call().state() += 1;
                    Ok(call.finish(0.into()))
                },
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt,), BigInt, HostTypeListEnd, _>(
                "one", NativeRules::default(), |call: Call<'_>, a| Ok(call.finish(a)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "two", NativeRules::default(), |call: Call<'_>, a, b| Ok(call.finish(a * 10 + b)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "three", NativeRules::default(), |call: Call<'_>, a, b, c| Ok(call.finish(a * 100 + b * 10 + c)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt, BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "four", NativeRules::default(), |call: Call<'_>, a, b, c, d| Ok(call.finish(a * 1000 + b * 100 + c * 10 + d)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt, BigInt, BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "five", NativeRules::default(), |call: Call<'_>, a, b, c, d, e| Ok(call.finish(a * 10000 + b * 1000 + c * 100 + d * 10 + e)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt, BigInt, BigInt, BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "six", NativeRules::default(), |call: Call<'_>, a, b, c, d, e, f| Ok(call.finish(a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f)),
            ).unwrap()
            .with_native_function::<ScopedProvider, (BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt), BigInt, HostTypeListEnd, _>(
                "seven", NativeRules::default(), |call: Call<'_>, a, b, c, d, e, f, g| Ok(call.finish(a * 1000000 + b * 100000 + c * 10000 + d * 1000 + e * 100 + f * 10 + g)),
            ).unwrap();
        let source = r#"
@external(erlang, "native", "zero") fn zero() -> Int
@external(erlang, "native", "one") fn one(a: Int) -> Int
@external(erlang, "native", "two") fn two(a: Int, b: Int) -> Int
@external(erlang, "native", "three") fn three(a: Int, b: Int, c: Int) -> Int
@external(erlang, "native", "four") fn four(a: Int, b: Int, c: Int, d: Int) -> Int
@external(erlang, "native", "five") fn five(a: Int, b: Int, c: Int, d: Int, e: Int) -> Int
@external(erlang, "native", "six") fn six(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int) -> Int
@external(erlang, "native", "seven") fn seven(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int, g: Int) -> Int
pub fn main() {
  #(zero(), one(1), two(1, 2), three(1, 2, 3), four(1, 2, 3, 4),
    five(1, 2, 3, 4, 5), six(1, 2, 3, 4, 5, 6), seven(1, 2, 3, 4, 5, 6, 7))
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
            )],
            crate::HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut state = TestRunState::default();
        let mut echo = Vec::new();
        let result = crate::execution_fixture::run(&mut execution, &mut state, &mut echo).unwrap();
        assert_eq!(
            result.inspect().to_string(),
            "#(0, 1, 12, 123, 1234, 12345, 123456, 1234567)"
        );
        assert_eq!(state.counter, 1);
        assert!(echo.is_empty());
    }

    #[test]
    fn supports_every_scoped_diverging_argument_arity() {
        let zero: Diverging0 = |_| Err(HostFailure::new("0").into());
        let one: Diverging1 = |_, _| Err(HostFailure::new("1").into());
        let two: Diverging2 = |_, _, _| Err(HostFailure::new("2").into());
        let three: Diverging3 = |_, _, _, _| Err(HostFailure::new("3").into());
        let four: Diverging4 = |_, _, _, _, _| Err(HostFailure::new("4").into());
        let five: Diverging5 = |_, _, _, _, _, _| Err(HostFailure::new("5").into());
        let six: Diverging6 = |_, _, _, _, _, _, _| Err(HostFailure::new("6").into());
        let seven: Diverging7 = |_, _, _, _, _, _, _, _| Err(HostFailure::new("7").into());
        let registrations = vec![
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                (),
                BigInt,
            >>::register(zero),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((),),
                BigInt,
            >>::register(one),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), ()),
                BigInt,
            >>::register(two),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), ()),
                BigInt,
            >>::register(three),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), ()),
                BigInt,
            >>::register(four),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), ()),
                BigInt,
            >>::register(five),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), ()),
                BigInt,
            >>::register(six),
            <_ as ScopedDivergingHostFunctionAdapter<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), (), ()),
                BigInt,
            >>::register(seven),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            assert_eq!(registration.return_type, HostTypeDescriptor::Int);
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            assert_eq!(
                expect_never_implementation(&registration.implementation)
                    .call(&mut runtime)
                    .expect_err("scoped diverging callback should not return")
                    .to_string(),
                arity.to_string(),
            );
        }
    }

    #[test]
    fn supports_one_argument() {
        let registration =
            <_ as HostFunctionAdapter<(BigInt,), BigInt>>::register(|a: BigInt| a + 1);
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [HostTypeDescriptor::Int],
        );
        assert_eq!(
            call_int(&registration.implementation, vec![1.into()], Vec::new()),
            BigInt::from(2),
        );
    }

    #[test]
    fn supports_two_arguments() {
        let registration = <_ as HostFunctionAdapter<(BigInt, BigInt), BigInt>>::register(
            |a: BigInt, b: BigInt| a - b,
        );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [HostTypeDescriptor::Int, HostTypeDescriptor::Int],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 3.into()],
                Vec::new(),
            ),
            BigInt::from(7),
        );
    }

    #[test]
    fn supports_three_arguments() {
        let registration = <_ as HostFunctionAdapter<(bool, BigInt, BigInt), BigInt>>::register(
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 20.into()],
                vec![false],
            ),
            BigInt::from(20),
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 20.into()],
                vec![true],
            ),
            BigInt::from(10),
        );
    }

    #[test]
    fn supports_four_arguments() {
        let registration = <_ as HostFunctionAdapter<(BigInt, bool, BigInt, bool), bool>>::register(
            |left: BigInt, first: bool, right: BigInt, second: bool| {
                left < right && first && !second
            },
        );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
            ],
        );
        assert!(call_bool(
            registration.implementation,
            vec![1.into(), 2.into()],
            vec![true, false],
        ));
    }

    #[test]
    fn supports_five_arguments() {
        let registration =
            <_ as HostFunctionAdapter<(BigInt, BigInt, BigInt, BigInt, BigInt), BigInt>>::register(
                |a: BigInt, b: BigInt, c: BigInt, d: BigInt, e: BigInt| a + b + c + d + e,
            );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()],
                Vec::new(),
            ),
            BigInt::from(15),
        );
    }

    #[test]
    fn supports_six_arguments() {
        let registration =
            <_ as HostFunctionAdapter<(bool, bool, bool, bool, bool, bool), bool>>::register(
                |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool| {
                    a && !b && c && !d && e && !f
                },
            );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
            ],
        );
        assert!(call_bool(
            registration.implementation,
            Vec::new(),
            vec![true, false, true, false, true, false],
        ));
    }

    #[test]
    fn supports_seven_arguments() {
        let registration = <_ as HostFunctionAdapter<
            (BigInt, bool, BigInt, bool, BigInt, bool, BigInt),
            BigInt,
        >>::register::<TestHostProfile>(
            |a: BigInt, b: bool, c: BigInt, d: bool, e: BigInt, f: bool, g: BigInt| {
                let c = if b { c } else { BigInt::from(0) };
                let e = if d { e } else { BigInt::from(0) };
                let g = if f { g } else { BigInt::from(0) };
                a + c + e + g
            },
        );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 4.into(), 8.into()],
                vec![true, false, true],
            ),
            BigInt::from(11),
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 4.into(), 8.into()],
                vec![false, true, false],
            ),
            BigInt::from(5),
        );
    }

    #[test]
    fn supports_every_scalar_argument_family_in_one_sealed_layout() {
        let registration = <_ as HostFunctionAdapter<
            (BigInt, f64, EcoString, BitArrayValue, char, bool, ()),
            EcoString,
        >>::register::<TestHostProfile>(
            |int: BigInt,
             float: f64,
             string: EcoString,
             bits: BitArrayValue,
             codepoint: char,
             bool_: bool,
             (): ()| {
                format!(
                    "{int}:{float}:{string}:{}:{codepoint}:{bool_}",
                    bits.bit_len(),
                )
                .into()
            },
        );
        let registration = registration.into_immediate();

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Float,
                HostTypeDescriptor::String,
                HostTypeDescriptor::BitArray,
                HostTypeDescriptor::UtfCodepoint,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Nil,
            ],
        );
        assert_eq!(registration.return_type, HostTypeDescriptor::String);
        let arguments = CallArguments::new(vec![1.into()], vec![true]).with_scalar_values(
            vec![1.5],
            vec!["one".into()],
            vec![BitArrayValue::from_bytes(vec![0xff])],
            vec!['A'],
            1,
        );

        assert_eq!(
            call_string(registration.implementation, arguments),
            EcoString::from("1:1.5:one:8:A:true"),
        );
    }

    #[test]
    #[should_panic(expected = "test function should return Int")]
    fn int_return_shape_guard_is_visible() {
        let registration =
            <_ as HostFunctionAdapter<(), bool>>::register(<bool as Default>::default);
        let registration = registration.into_immediate();
        call_int(&registration.implementation, Vec::new(), Vec::new());
    }

    #[test]
    #[should_panic(expected = "test function should return Bool")]
    fn bool_return_shape_guard_is_visible() {
        call_bool(
            <_ as HostFunctionAdapter<(), BigInt>>::register(<BigInt as Default>::default)
                .into_immediate()
                .implementation,
            Vec::new(),
            Vec::new(),
        );
    }

    #[test]
    #[should_panic(expected = "all-scalar test function should return String")]
    fn string_return_shape_guard_is_visible() {
        call_string(
            <_ as HostFunctionAdapter<(), BigInt>>::register(<BigInt as Default>::default)
                .into_immediate()
                .implementation,
            CallArguments::new(Vec::new(), Vec::new()),
        );
    }

    fn call_int(
        implementation: &HostFunctionImplementation<TestHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> BigInt {
        let implementation = expect_value_implementation(implementation);
        let arguments = CallArguments::new(ints, bools);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        crate::host::expect_immediate_call(implementation, &mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::Int(value)) = runtime.completed() else {
            panic!("test function should return Int");
        };
        value.clone()
    }

    fn call_bool(
        implementation: HostFunctionImplementation<TestHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> bool {
        let implementation = expect_value_implementation(&implementation);
        let arguments = CallArguments::new(ints, bools);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        crate::host::expect_immediate_call(implementation, &mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::Bool(value)) = runtime.completed() else {
            panic!("test function should return Bool");
        };
        *value
    }

    fn call_string(
        implementation: HostFunctionImplementation<TestHostProfile>,
        arguments: CallArguments,
    ) -> EcoString {
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        crate::host::expect_immediate_call(implementation, &mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::String(value)) = runtime.completed() else {
            panic!("all-scalar test function should return String");
        };
        value.clone()
    }
}
