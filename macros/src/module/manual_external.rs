use super::signature::host_type_token_sequence;
use super::syntax::{host_type_index, retained_parameter_accessor};
use super::{ExternalModel, GenericExternalModel};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

pub(super) fn declaration(
    external: &ExternalModel,
    generic: &GenericExternalModel,
    payload: &Path,
    support: &TokenStream,
) -> TokenStream {
    let owner = payload;
    let parameters = &generic.parameters;
    let input = &generic.input;
    let output = &external.ident;
    let schema = &external.schema;
    let visibility = &generic.visibility;
    let arguments = parameters
        .iter()
        .map(|parameter| quote!(<#parameter as #support::ProviderValue>::Host))
        .collect::<Vec<_>>();
    let arguments = host_type_token_sequence(&arguments, support);
    let output_type = quote!(#output<#(#parameters,)* #support::ProviderExternalOutput<#payload>>);
    let accessors = parameters.iter().enumerate().map(|(index, parameter)| {
        let method = retained_parameter_accessor(parameter);
        let index = host_type_index(index, support);
        quote! {
            #visibility fn #method<'__geam_value>(
                &'__geam_value self,
                select: impl ::core::ops::FnOnce(&'__geam_value #payload)
                    -> &'__geam_value #support::Retained<#owner, #index>,
            ) -> #support::Stored<#parameter, #support::ProviderStoredInput<
                '__geam_value, #owner, #index,
                <__GeamArguments as #support::HostTypeAt<#index>>::Type,
            >>
            where __GeamArguments: #support::HostTypeAt<#index>,
            {
                #support::Stored::from_retained(select(self.__geam_context.payload()))
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
                    -> &'__geam_value #support::Retained<#owner, #index>,
            ) -> #support::Stored<#parameter, #support::ProviderOwnedStoredInput<
                #owner, #index, <__GeamArguments as #support::HostTypeAt<#index>>::Type,
            >>
            where __GeamArguments: #support::HostTypeAt<#index>,
            {
                self.__geam_context.stored(select)
            }
        }
    });

    quote! {
            impl<#(#parameters,)* __GeamArguments> #input<
                #(#parameters,)* #support::ProviderExternalInputContext<#payload, __GeamArguments>,
            >
            where
                __GeamArguments: #support::HostTypeSequence,
                #payload: ::core::marker::Send + 'static,
            {
                fn __geam_from_host(context: #support::ProviderExternalInputContext<
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
                #(#parameters,)* #support::ProviderOwnedExternalInputContext<#payload, __GeamArguments>,
            >
            where
                __GeamArguments: #support::HostTypeSequence,
                #payload: ::core::marker::Send + 'static,
            {
                fn __geam_from_async_host(context: #support::ProviderOwnedExternalInputContext<
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

            impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding, __GeamReturn>
                #support::ProviderOutputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn> for #output_type
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                __GeamReturn: #support::HostType,
                #(#parameters: #support::ProviderValue,)*
                #payload: ::core::marker::Send + 'static,
            {
                type Error = ::core::convert::Infallible;

    fn into_host<'__geam_call>(
                    self,
                    call: &mut #support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn>,
                    construction: &#support::ProviderConstructions<'__geam_call, Self::OutputRequirements>,
                ) -> ::core::result::Result<<Self::Host as #support::HostType>::Value<'__geam_call>, Self::Error> {
                    ::core::result::Result::Ok(match self.__geam_context.into_value() {
                        ::core::result::Result::Ok(payload) => call.construct_external_with_binding::<
                            __GeamProvider, #schema, #arguments,
                        >(construction.token(), payload),
                        ::core::result::Result::Err(value) => call.provider_external_from_return::<
                            #schema, #arguments, _,
                        >(value),
                    })
                }
            }

            impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding>
                #support::ProviderRootOutputValue<__GeamProfile, __GeamProviderBinding> for #output_type
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                #(#parameters: #support::ProviderValue,)*
                #payload: ::core::marker::Send + 'static,
            {
                fn complete<'__geam_call>(
                    self,
                    mut call: #support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, Self::Host>,
                    _constructions: &#support::ProviderConstructions<'__geam_call, Self::RootRequirements>,
                ) -> ::core::result::Result<#support::HostCallCompletion<'__geam_call, Self::Host>, #support::HostCallError> {
                    let value = match self.__geam_context.into_value() {
                        ::core::result::Result::Ok(payload) => call.create_external_with_binding::<__GeamProvider>(payload),
                        ::core::result::Result::Err(value) => call.provider_external_from_return::<
                            #schema, #arguments, _,
                        >(value),
                    };
                    ::core::result::Result::Ok(call.return_value(value))
                }
            }

        }
}
