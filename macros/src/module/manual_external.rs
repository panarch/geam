use super::signature::host_type_token_sequence;
use super::syntax::{host_type_index, retained_parameter_accessor};
use super::{ExternalModel, GenericExternalModel};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

pub(super) fn transfer_declaration(
    external: &ExternalModel,
    generic: &GenericExternalModel,
    owner: &Path,
    payload: &Path,
    store: &TokenStream,
    support: &TokenStream,
) -> TokenStream {
    let parameters = &generic.parameters;
    let input = &generic.input;
    let output = &external.ident;
    let schema = &external.schema;
    let storage = &external.storage;
    let visibility = &generic.visibility;
    let arguments = parameters
        .iter()
        .map(|parameter| quote!(<#parameter as #support::ProviderValue>::Host))
        .collect::<Vec<_>>();
    let arguments = host_type_token_sequence(&arguments, support);
    let output_type =
        quote!(#output<#(#parameters,)* #support::ProviderTransferExternalOutput<#payload>>);
    let local_output_type =
        quote!(#output<#(#parameters,)* #support::ProviderExternalOutput<#owner>>);
    let accessors = parameters.iter().enumerate().map(|(index, parameter)| {
        let method = retained_parameter_accessor(parameter);
        let index = host_type_index(index, support);
        quote! {
            #visibility fn #method<'__geam_value>(
                &'__geam_value self,
                select: impl ::core::ops::FnOnce(&'__geam_value #payload)
                    -> &'__geam_value #support::ProviderTransferRetained<#owner, #index>,
            ) -> #support::Stored<#parameter, #support::ProviderTransferStoredInput<
                '__geam_value, #owner, #index,
                <__GeamArguments as #support::HostTypeAt<#index>>::Type,
            >>
            where __GeamArguments: #support::HostTypeAt<#index>,
            {
                #support::Stored::from_transfer_retained(select(self.__geam_context.payload()))
            }
        }
    });
    let owned_accessors = parameters.iter().enumerate().map(|(index, parameter)| {
        let method = retained_parameter_accessor(parameter);
        let index = host_type_index(index, support);
        quote! {
            #visibility fn #method(
                &self,
                select: impl for<'__geam_value> ::core::ops::FnOnce(&'__geam_value #payload)
                    -> &'__geam_value #support::ProviderTransferRetained<#owner, #index>,
            ) -> #support::Stored<#parameter, #support::ProviderAsyncStoredInput<
                #owner, #index, <__GeamArguments as #support::HostTypeAt<#index>>::Type,
            >>
            where __GeamArguments: #support::HostTypeAt<#index>,
            {
                self.__geam_context.stored(select)
            }
        }
    });

    quote! {
        impl<#(#parameters,)*> #support::ProviderTransferValue for #local_output_type
        where #(#parameters: #support::ProviderValue + 'static,)*
        {
            type Output = #output_type;
            type ImmediateInput = #input<#(#parameters,)* #support::ProviderTransferExternalInputContext<#payload, #arguments>>;
            type ImmediateListInput = Self::ImmediateInput;
            type TransferInput = #input<#(#parameters,)* #support::ProviderAsyncExternalInputContext<#payload, #arguments>>;
            type TransferListInput = Self::TransferInput;
        }

        impl<#(#parameters,)*> #support::ProviderTransferValue for #output_type
        where #(#parameters: #support::ProviderValue + 'static,)*
        {
            type Output = Self;
            type ImmediateInput = #input<#(#parameters,)* #support::ProviderTransferExternalInputContext<#payload, #arguments>>;
            type ImmediateListInput = Self::ImmediateInput;
            type TransferInput = #input<#(#parameters,)* #support::ProviderAsyncExternalInputContext<#payload, #arguments>>;
            type TransferListInput = Self::TransferInput;
        }

        impl<#(#parameters,)* Profile, Provider, Return>
            #support::ProviderTransferDynamicInput<Profile, Provider, Return>
            for #output<#(#parameters,)*>
        where
            Profile: __GeamAsyncModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#parameters: #support::ProviderValue,)*
            #payload: #support::ProviderTransferPayload<Profile>,
        {
            type Host = #support::HostExternalType<#schema, #arguments>;
            type View = #input<#(#parameters,)* #support::ProviderTransferExternalInputContext<
                #payload, #arguments,
            >>;

            fn from_host<'__geam_call>(
                call: &mut #support::TransferHostCall<'__geam_call, Profile, Provider, Return>,
                value: <Self::Host as #support::HostType>::Value<'__geam_call>,
            ) -> Self::View {
                let value = call.provider_transfer_external_view_with::<
                    __GeamAsyncProvider, #schema, #arguments,
                >(value);
                #input::__geam_from_transfer_host(
                    #support::ProviderTransferExternalInputContext::from_host(value),
                )
            }
        }

        impl<#(#parameters,)* __GeamArguments> #input<
            #(#parameters,)* #support::ProviderTransferExternalInputContext<#payload, __GeamArguments>,
        >
        where
            __GeamArguments: #support::HostTypeSequence,
            #payload: ::core::marker::Send + 'static,
        {
            fn __geam_from_transfer_host(context: #support::ProviderTransferExternalInputContext<
                #payload, __GeamArguments,
            >) -> Self {
                Self { __geam_context: context, __geam_parameters: ::core::marker::PhantomData }
            }

            #visibility fn payload(&self) -> &#payload { self.__geam_context.payload() }

            #visibility fn into_value(self) -> #output_type {
                #output {
                    __geam_context: self.__geam_context.into_output(),
                    __geam_parameters: ::core::marker::PhantomData,
                }
            }

            #(#accessors)*
        }

        impl<#(#parameters,)* __GeamArguments> #input<
            #(#parameters,)* #support::ProviderAsyncExternalInputContext<#payload, __GeamArguments>,
        >
        where
            __GeamArguments: #support::HostTypeSequence,
            #payload: ::core::marker::Send + 'static,
        {
            fn __geam_from_async_host(context: #support::ProviderAsyncExternalInputContext<
                #payload, __GeamArguments,
            >) -> Self {
                Self { __geam_context: context, __geam_parameters: ::core::marker::PhantomData }
            }

            #visibility fn with_payload<__GeamOutput>(
                &self, read: impl ::core::ops::FnOnce(&#payload) -> __GeamOutput,
            ) -> __GeamOutput { self.__geam_context.with_payload(read) }

            #visibility fn into_value(self) -> #output_type {
                #output {
                    __geam_context: self.__geam_context.into_output(),
                    __geam_parameters: ::core::marker::PhantomData,
                }
            }

            #(#owned_accessors)*
        }

        impl<#(#parameters,)*> #support::ProviderValue for #output_type
        where #(#parameters: #support::ProviderValue,)*
        {
            type Host = #support::HostExternalType<#schema, #arguments>;
            type Input = Self;
            type ListInput = Self;
            type OutputRequirements = #support::ProviderConstruction<Self::Host>;
            type RootRequirements = #support::ProviderNoConstructions;
        }

        impl<#(#parameters,)* Profile, Provider, Return>
            #support::ProviderTransferOutputValue<Profile, Provider, Return> for #output_type
        where
            Profile: __GeamAsyncModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#parameters: #support::ProviderValue,)*
            #payload: #support::ProviderTransferPayload<Profile>,
        {
            fn into_host<'__geam_call>(
                self,
                call: &mut #support::TransferHostCall<'__geam_call, Profile, Provider, Return>,
                construction: &#support::ProviderConstructions<'__geam_call, Self::OutputRequirements>,
            ) -> <Self::Host as #support::HostType>::Value<'__geam_call> {
                match self.__geam_context.into_value() {
                    ::core::result::Result::Ok(payload) => call.construct_external_with_binding::<
                        __GeamAsyncProvider, #schema, #arguments,
                    >(construction.token(), payload),
                    ::core::result::Result::Err(value) => call.provider_transfer_external_from_return::<
                        #schema, #arguments, _,
                    >(value),
                }
            }
        }

        impl<#(#parameters,)* Profile, Provider>
            #support::ProviderTransferRootOutputValue<Profile, Provider> for #output_type
        where
            Profile: __GeamAsyncModuleProfile,
            Provider: #support::HostProvider<Profile>,
            #(#parameters: #support::ProviderValue,)*
            #payload: #support::ProviderTransferPayload<Profile>,
        {
            fn complete<'__geam_call>(
                self,
                mut call: #support::TransferHostCall<'__geam_call, Profile, Provider, Self::Host>,
                _constructions: &#support::ProviderConstructions<'__geam_call, Self::RootRequirements>,
            ) -> ::core::result::Result<#support::HostCallCompletion<'__geam_call, Self::Host>, #support::AsyncHostCallError> {
                let value = match self.__geam_context.into_value() {
                    ::core::result::Result::Ok(payload) => call.create_external_with_binding::<__GeamAsyncProvider>(payload),
                    ::core::result::Result::Err(value) => call.provider_transfer_external_from_return::<
                        #schema, #arguments, _,
                    >(value),
                };
                ::core::result::Result::Ok(call.return_value(value))
            }
        }

        impl<Profile> #support::AsyncHostExternalStorage<Profile, #schema> for #storage
        where Profile: __GeamAsyncModuleProfile,
        {
            type Payload = #payload;
            fn store(stores: &Profile::ExternalStores) -> &#support::AsyncHostExternalStore<Self::Payload> {
                #store
            }
            fn source_equal(
                context: &#support::AsyncHostExternalEquality<'_>, left: &Self::Payload, right: &Self::Payload,
            ) -> bool {
                <#payload as #support::RetainedExternalPayload<#support::ProviderTransferRetainedContext>>::source_equal(left, context, right)
            }
            fn source_hash(context: &#support::AsyncHostExternalHashing<'_>, value: &Self::Payload) -> u64 {
                <#payload as #support::RetainedExternalPayload<#support::ProviderTransferRetainedContext>>::source_hash(value, context)
            }
            fn inspect(context: &#support::AsyncHostExternalInspection<'_>, value: &Self::Payload) -> #support::EcoString {
                <#payload as #support::RetainedExternalPayload<#support::ProviderTransferRetainedContext>>::inspect(value, context)
            }
        }
    }
}
