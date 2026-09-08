use super::argument::{HostArgument, HostParameter, HostParameterLayout};
use super::{HostFunctionSchema, HostFunctionSchemaRegistration, RegisteredHostConstructions};
use crate::host::{
    AsyncHostCallError, HostAbiType, HostCallCompletion, HostConstructions, HostProfile,
    HostProvider, HostTypeSequence, HostValueToken, TransferHostCall, TransferHostCallRuntime,
};
use ecow::EcoString;
use std::sync::Arc;

type TransferHostScopedCallback<Profile> = dyn Fn(&mut dyn TransferHostCallRuntime<Profile>) -> Result<HostValueToken, AsyncHostCallError>
    + Send
    + Sync;

type TransferHostNeverCallback<Profile> = dyn Fn(
        &mut dyn TransferHostCallRuntime<Profile>,
    ) -> Result<std::convert::Infallible, AsyncHostCallError>
    + Send
    + Sync;

/// An immediate scoped host function specialized for transferable execution.
///
/// This hidden trait is implemented by generated provider adapters. Provider
/// authors continue to declare one ordinary Rust function through the Geam
/// attribute API.
#[doc(hidden)]
pub trait TransferScopedHostFunction<Profile, Provider, Arguments, Return>:
    TransferScopedHostFunctionAdapter<Profile, Provider, Arguments, Return> + Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

/// A transferable immediate host function with registered construction types.
#[doc(hidden)]
pub trait TransferScopedConstructingHostFunction<
    Profile,
    Provider,
    Arguments,
    Return,
    Constructions,
>:
    TransferScopedConstructingHostFunctionAdapter<
        Profile,
        Provider,
        Arguments,
        Return,
        Constructions,
    > + Send
    + Sync
    + 'static where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
}

/// A transferable immediate host function that cannot return successfully.
#[doc(hidden)]
pub trait TransferScopedDivergingHostFunction<Profile, Provider, Arguments, Return>:
    TransferScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>
    + Send
    + Sync
    + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
}

impl<Profile, Provider, Function, Arguments, Return>
    TransferScopedHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: TransferScopedHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

impl<Profile, Provider, Function, Arguments, Return, Constructions>
    TransferScopedConstructingHostFunction<Profile, Provider, Arguments, Return, Constructions>
    for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
    Function: TransferScopedConstructingHostFunctionAdapter<
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
    TransferScopedDivergingHostFunction<Profile, Provider, Arguments, Return> for Function
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Function: TransferScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>
        + Send
        + Sync
        + 'static,
{
}

#[doc(hidden)]
pub trait TransferScopedHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> TransferHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub trait TransferScopedConstructingHostFunctionAdapter<
    Profile,
    Provider,
    Arguments,
    Return,
    Constructions,
>: Send + Sync + 'static where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Constructions: HostTypeSequence,
{
    fn register(self) -> TransferHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub trait TransferScopedDivergingHostFunctionAdapter<Profile, Provider, Arguments, Return>:
    Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> TransferHostFunctionRegistration<Profile>;
}

#[doc(hidden)]
pub struct TransferHostFunctionRegistration<Profile: HostProfile> {
    parameters: Box<[HostParameter]>,
    parameter_types: Box<[crate::host::HostTypeDescriptor]>,
    return_type: crate::host::HostTypeDescriptor,
    custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    implementation: TransferHostFunctionImplementation<Profile>,
}

pub(crate) enum TransferHostFunctionImplementation<Profile: HostProfile> {
    Value(TransferHostValueFunction<Profile>),
    Never(TransferHostNeverFunction<Profile>),
}

pub(crate) struct TransferHostValueFunction<Profile: HostProfile> {
    callback: Arc<TransferHostScopedCallback<Profile>>,
}

pub(crate) struct TransferHostNeverFunction<Profile: HostProfile> {
    callback: Arc<TransferHostNeverCallback<Profile>>,
}

impl<Profile: HostProfile> Clone for TransferHostValueFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
        }
    }
}

impl<Profile: HostProfile> Clone for TransferHostNeverFunction<Profile> {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
        }
    }
}

impl<Profile: HostProfile> TransferHostValueFunction<Profile> {
    pub(crate) fn new(
        callback: impl Fn(
            &mut dyn TransferHostCallRuntime<Profile>,
        ) -> Result<HostValueToken, AsyncHostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            callback: Arc::new(callback),
        }
    }

    pub(crate) fn call(
        &self,
        runtime: &mut dyn TransferHostCallRuntime<Profile>,
    ) -> Result<HostValueToken, AsyncHostCallError> {
        (self.callback)(runtime)
    }
}

impl<Profile: HostProfile> TransferHostNeverFunction<Profile> {
    pub(crate) fn new(
        callback: impl Fn(
            &mut dyn TransferHostCallRuntime<Profile>,
        ) -> Result<std::convert::Infallible, AsyncHostCallError>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        Self {
            callback: Arc::new(callback),
        }
    }

    pub(crate) fn call(
        &self,
        runtime: &mut dyn TransferHostCallRuntime<Profile>,
    ) -> Result<std::convert::Infallible, AsyncHostCallError> {
        (self.callback)(runtime)
    }
}

pub(crate) struct TransferHostFunctionDefinition<Profile: HostProfile> {
    schema: HostFunctionSchema,
    constructions: RegisteredHostConstructions,
    implementation: TransferHostFunctionImplementation<Profile>,
}

pub(crate) trait TransferHostScopedArgument: HostAbiType {
    type Slot: Copy + Send + Sync + 'static;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot;

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType;
}

impl<T> TransferHostScopedArgument for T
where
    T: HostArgument,
    for<'call> T: HostAbiType<Value<'call> = T>,
{
    type Slot = T::Slot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register::<T>()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        T::read(call.arguments(), slot)
    }
}

impl<const INDEX: usize> TransferHostScopedArgument for crate::host::HostTypeParameter<INDEX> {
    type Slot = crate::host::HostValueArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_value_parameter()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.value(slot)
    }
}

impl<Item: HostAbiType> TransferHostScopedArgument for crate::host::HostListType<Item> {
    type Slot = crate::host::HostListArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_list_parameter()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.list(slot)
    }
}

impl<Elements: crate::host::HostAbiTypeSequence> TransferHostScopedArgument
    for crate::host::HostTupleType<Elements>
{
    type Slot = crate::host::HostTupleArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_tuple_parameter()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.tuple(slot)
    }
}

impl<Schema, Arguments> TransferHostScopedArgument
    for crate::host::HostCustomType<Schema, Arguments>
where
    Schema: crate::host::HostCustomSchema,
    Arguments: crate::host::HostAbiTypeSequence,
{
    type Slot = crate::host::HostCustomArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_custom_parameter()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.custom(slot)
    }
}

impl<Schema, Arguments> TransferHostScopedArgument
    for crate::host::HostExternalType<Schema, Arguments>
where
    Schema: crate::host::HostExternalSchema,
    Arguments: crate::host::HostAbiTypeSequence,
{
    type Slot = crate::host::HostExternalArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_external_parameter_slot()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.external(slot)
    }
}

impl<Arguments, FunctionReturn> TransferHostScopedArgument
    for crate::host::HostFunctionType<Arguments, FunctionReturn>
where
    Arguments: crate::host::HostAbiTypeSequence,
    FunctionReturn: HostAbiType,
{
    type Slot = crate::host::HostFunctionArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_function_parameter_slot(
            <Arguments as crate::host::HostAbiTypeSequence>::descriptors().len(),
        )
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.function(slot)
    }
}

impl<Arguments, FunctionReturn> TransferHostScopedArgument
    for crate::host::HostOpaqueFunctionType<Arguments, FunctionReturn>
where
    Arguments: crate::host::HostAbiTypeSequence,
    FunctionReturn: HostAbiType,
{
    type Slot = crate::host::HostValueArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        layout.register_value_parameter()
    }

    fn read<'call, Profile, Provider, Return>(
        call: &TransferHostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.value(slot)
    }
}

macro_rules! transfer_host_function {
    ($($argument:ident => $slot:ident),*) => {
        impl<Profile, Provider, Function, Return, $($argument,)*>
            TransferScopedHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return>
            for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    TransferHostCall<'call, Profile, Provider, Return>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<HostCallCompletion<'call, Return>, AsyncHostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: TransferHostScopedArgument,)*
        {
            fn register(self) -> TransferHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as TransferHostScopedArgument>::register(&mut layout);)*
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
                let implementation = TransferHostFunctionImplementation::Value(
                    TransferHostValueFunction::new(move |runtime| {
                        let call = TransferHostCall::new(runtime);
                        $(let $slot = <$argument as TransferHostScopedArgument>::read(&call, $slot);)*
                        self(call, $($slot),*).map(|completion| completion.token)
                    }),
                );
                TransferHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, Constructions, $($argument,)*>
            TransferScopedConstructingHostFunctionAdapter<
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
                    TransferHostCall<'call, Profile, Provider, Return>,
                    HostConstructions<'call, Constructions>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<HostCallCompletion<'call, Return>, AsyncHostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: TransferHostScopedArgument,)*
        {
            fn register(self) -> TransferHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as TransferHostScopedArgument>::register(&mut layout);)*
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
                let implementation = TransferHostFunctionImplementation::Value(
                    TransferHostValueFunction::new(move |runtime| {
                        let call = TransferHostCall::new(runtime);
                        $(let $slot = <$argument as TransferHostScopedArgument>::read(&call, $slot);)*
                        self(call, HostConstructions::new(), $($slot),*)
                            .map(|completion| completion.token)
                    }),
                );
                TransferHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            TransferScopedDivergingHostFunctionAdapter<Profile, Provider, ($($argument,)*), Return>
            for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    TransferHostCall<'call, Profile, Provider, Return>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<std::convert::Infallible, AsyncHostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: TransferHostScopedArgument,)*
        {
            fn register(self) -> TransferHostFunctionRegistration<Profile> {
                #[allow(unused_mut)]
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as TransferHostScopedArgument>::register(&mut layout);)*
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
                let implementation = TransferHostFunctionImplementation::Never(
                    TransferHostNeverFunction::new(move |runtime| {
                        let call = TransferHostCall::new(runtime);
                        $(let $slot = <$argument as TransferHostScopedArgument>::read(&call, $slot);)*
                        self(call, $($slot),*)
                    }),
                );
                TransferHostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*]
                        .into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }
    };
}

transfer_host_function!();
transfer_host_function!(A0 => a0);
transfer_host_function!(A0 => a0, A1 => a1);
transfer_host_function!(A0 => a0, A1 => a1, A2 => a2);
transfer_host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3);
transfer_host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4);
transfer_host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
transfer_host_function!(
    A0 => a0,
    A1 => a1,
    A2 => a2,
    A3 => a3,
    A4 => a4,
    A5 => a5,
    A6 => a6
);

impl<Profile: HostProfile> TransferHostFunctionDefinition<Profile> {
    pub(crate) fn new_scoped<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: TransferScopedHostFunction<Profile, Provider, Arguments, Return>,
    {
        let registration = <Function as TransferScopedHostFunctionAdapter<
            Profile,
            Provider,
            Arguments,
            Return,
        >>::register(function);
        Self::from_registration(name, registration, RegisteredHostConstructions::empty())
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
        Constructions: HostTypeSequence,
        Function: TransferScopedConstructingHostFunction<
                Profile,
                Provider,
                Arguments,
                Return,
                Constructions,
            >,
    {
        let registration = <Function as TransferScopedConstructingHostFunctionAdapter<
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
        Self::from_registration(
            name,
            registration,
            RegisteredHostConstructions::new(construction_types, custom_schemas.into_boxed_slice()),
        )
    }

    pub(crate) fn new_scoped_diverging<Provider, Arguments, Return, Function>(
        name: EcoString,
        function: Function,
    ) -> Result<Self, crate::HostRegistrationError>
    where
        Provider: HostProvider<Profile>,
        Function: TransferScopedDivergingHostFunction<Profile, Provider, Arguments, Return>,
    {
        let registration = <Function as TransferScopedDivergingHostFunctionAdapter<
            Profile,
            Provider,
            Arguments,
            Return,
        >>::register(function);
        Self::from_registration(name, registration, RegisteredHostConstructions::empty())
    }

    fn from_registration(
        name: EcoString,
        registration: TransferHostFunctionRegistration<Profile>,
        constructions: RegisteredHostConstructions,
    ) -> Result<Self, crate::HostRegistrationError> {
        HostFunctionSchema::from_registration(
            name,
            HostFunctionSchemaRegistration {
                layout: registration.parameters,
                parameters: registration.parameter_types,
                return_: registration.return_type,
                custom_schemas: registration.custom_schemas,
            },
        )
        .and_then(|schema| {
            constructions.validate_for(&schema)?;
            Ok(Self {
                schema,
                constructions,
                implementation: registration.implementation,
            })
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
        TransferHostFunctionImplementation<Profile>,
    ) {
        (self.schema, self.constructions, self.implementation)
    }
}

#[cfg(test)]
mod tests {
    use super::TransferHostFunctionDefinition;
    use crate::host::test::{TestHostProfile, TestRunState};
    use crate::host::{
        AsyncHostCallError, HostCallCompletion, HostConstructions, HostProvider,
        HostRegistrationError, HostType, HostTypeList, HostTypeListEnd, HostTypeParameter,
        TransferHostCall,
    };

    struct ConstructionProvider;

    impl HostProvider<TestHostProfile> for ConstructionProvider {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    type UnboundConstructions = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;

    fn ready_with_construction<'call, Value: HostType>(
        mut call: TransferHostCall<'call, TestHostProfile, ConstructionProvider, bool>,
        _constructions: HostConstructions<'call, HostTypeList<Value, HostTypeListEnd>>,
    ) -> Result<HostCallCompletion<'call, bool>, AsyncHostCallError> {
        *call.state() += 1;
        Ok(call.return_value(true))
    }

    #[test]
    fn construction_types_must_be_bound_by_the_transfer_function_scheme() {
        let error = TransferHostFunctionDefinition::new_scoped_with_constructions::<
            ConstructionProvider,
            (),
            bool,
            UnboundConstructions,
            _,
        >(
            "ready".into(),
            ready_with_construction::<HostTypeParameter<0>>,
        )
        .err()
        .expect("unbound transfer construction should be rejected");

        assert_eq!(
            error,
            HostRegistrationError::UnboundConstructionTypeParameters {
                function: "ready".into(),
                parameters: vec![0].into_boxed_slice(),
            },
        );
    }

    #[test]
    fn a_concrete_construction_uses_the_original_projected_state() {
        use crate::host::{TransferHostProviderModule, TransferHostProviderSet};
        use crate::plan::execution::TransferHostedExecution;
        use crate::plan::{LibraryEntry, LibraryValueType};
        use crate::runtime::TransferInputs;
        use crate::runtime::work::driver::Driver;
        let provider = TransferHostProviderModule::<TestHostProfile>::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_function_and_constructions::<ConstructionProvider, (), bool,
                HostTypeList<bool, HostTypeListEnd>, _>("ready", ready_with_construction::<bool>)
            .expect("concrete construction is closed");
        let program = crate::frontend::compile_typed_transfer_host_program("application", "library", [
            crate::PackageSource::new("application", Vec::<String>::new(), [
                crate::ModuleSource::new("library", "src/library.gleam",
                    "@external(erlang, \"native\", \"ready\") fn ready() -> Bool\npub fn run() { ready() }"),
            ]),
        ], TransferHostProviderSet::new([provider]).expect("provider set")).expect("source");
        let plan = crate::planner::plan_transfer_host_library_program(program).expect("plan");
        let entry = plan
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("entry")
            .gleam_body()
            .expect("source body");
        let entry = LibraryEntry::new(entry.id(), LibraryValueType::Bool, Vec::new(), Vec::new());
        let (plan, entries) = TransferHostedExecution::from_library_plan(plan, entry, Vec::new())
            .expect("sealed executable");
        let mut state = TestRunState {
            counter: 4,
            unrelated: true,
        };
        let mut stores = ();
        let mut echo = drop;
        let mut driver = Driver::new(&plan, &mut state, &mut stores, &mut echo);
        assert!(
            driver
                .run_bool(*entries.bools[0].function(), TransferInputs::empty())
                .expect("native ready")
        );
        drop(driver);
        assert_eq!(state.counter, 5);
        assert!(state.unrelated);
    }
}
