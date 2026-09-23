use super::signature::host_type_token_sequence;
use super::{ExternalModel, GenericExternalModel, GenericExternalStorage, InputOwnership};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

struct InputCodecs<'model> {
    generic: &'model GenericExternalModel,
    payload: &'model TokenStream,
    host: &'model TokenStream,
    schema: &'model Ident,
    arguments: &'model TokenStream,
    support: &'model TokenStream,
}

pub(super) fn declaration(
    external: &ExternalModel,
    generic: &GenericExternalModel,
    support: &TokenStream,
) -> TokenStream {
    let parameters = &generic.parameters;
    let output = &external.ident;
    let input = &generic.input;
    let schema = &external.schema;
    let arguments = parameters
        .iter()
        .map(|parameter| quote!(<#parameter as #support::ProviderValue>::Host))
        .collect::<Vec<_>>();
    let arguments = host_type_token_sequence(&arguments, support);
    let host = quote!(#support::HostExternalType<#schema, #arguments>);
    let (payload, contexts, output_contexts, into_payload) = match &generic.storage {
        GenericExternalStorage::StoredFields {
            payload,
            owner,
            fields,
        } => {
            let contexts = fields
                .iter()
                .enumerate()
                .map(|(index, _)| format_ident!("__GeamStoredContext{index}"))
                .collect::<Vec<_>>();
            let output_contexts = fields.iter().map(|field| {
                let parameter = &parameters[field.parameter_index];
                let index = &field.index;
                quote!(#support::ProviderStoredOutput<#owner, #index, <#parameter as #support::ProviderValue>::Host>)
            }).collect::<Vec<_>>();
            let names = fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
            let into_payload = quote!({
                let #output { #(#names,)* } = self;
                #payload { #(#names: #names.into_retained(),)* }
            });
            (
                quote!(#payload),
                contexts,
                output_contexts,
                Some(into_payload),
            )
        }
        GenericExternalStorage::ManualPayload { payload } => (
            quote!(#payload),
            vec![format_ident!("__GeamExternalContext")],
            vec![quote!(#support::ProviderExternalOutput<#payload>)],
            None,
        ),
    };
    let output_type = quote!(#output<#(#parameters,)* #(#output_contexts,)*>);
    let immediate = quote!(#input<#(#parameters,)* #support::ProviderExternalInputContext<#payload, #arguments>>);
    let owned = quote!(#input<#(#parameters,)* #support::ProviderOwnedExternalInputContext<#payload, #arguments>>);
    let immediate_decoder = format_ident!("__Geam{output}ListDecoder");
    let owned_decoder = format_ident!("__Geam{output}OwnedListDecoder");
    let forms = [
        (quote!(#(#contexts,)*), quote!(#output<#(#parameters,)* #(#contexts,)*>)),
        (quote!(__GeamInputContext,), quote!(#input<#(#parameters,)* __GeamInputContext>)),
    ].into_iter().map(|(contexts, type_)| quote! {
        impl<#(#parameters,)* #contexts> #support::ProviderValue for #type_
        where #(#parameters: #support::ProviderValue,)*
        {
            type Host = #host;
            type OutputRequirements = #support::ProviderConstruction<Self::Host>;
            type RootRequirements = #support::ProviderNoConstructions;
        }

        impl<#(#parameters,)* #contexts> #support::ProviderValueForms for #type_
        where #(#parameters: #support::ProviderValue + 'static,)*
        {
            type InvocationRequirements = ();
            type Runtime<__GeamProfile: #support::HostProfile> = #support::ProviderStaticValueForms<Self>;
            type Output = #output_type;
            type ImmediateInput = #immediate;
            type ImmediateListInput = #immediate;
            type OwnedInput = #owned;
            type OwnedListInput = #owned;
            type ImmediateListDecoder = #immediate_decoder<#(#parameters,)*>;
            type OwnedListDecoder = #owned_decoder<#(#parameters,)*>;
        }
    });
    let input_codecs = InputCodecs {
        generic,
        payload: &payload,
        host: &host,
        schema,
        arguments: &arguments,
        support,
    };
    let immediate_codecs =
        input_codecs.generate(&immediate, &immediate_decoder, InputOwnership::Borrowed);
    let owned_codecs = input_codecs.generate(&owned, &owned_decoder, InputOwnership::Owned);
    let output_codecs = into_payload.map(|into_payload| quote! {
        impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding, __GeamReturn>
            #support::ProviderOutputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn> for #output_type
        where
            __GeamProfile: __GeamModuleProfile,
            __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
            __GeamReturn: #support::HostType,
            #(#parameters: #support::ProviderValue,)*
        {
            type Error = ::core::convert::Infallible;

            fn into_host<'__geam_call>(
                self,
                call: &mut #support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn>,
                constructions: &#support::ProviderConstructions<'__geam_call, Self::OutputRequirements>,
            ) -> ::core::result::Result<<Self::Host as #support::HostType>::Value<'__geam_call>, Self::Error> {
                ::core::result::Result::Ok(call.construct_external_with_binding::<__GeamProvider, #schema, #arguments>(constructions.token(), #into_payload))
            }
        }

        impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding>
            #support::ProviderRootOutputValue<__GeamProfile, __GeamProviderBinding> for #output_type
        where
            __GeamProfile: __GeamModuleProfile,
            __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
            #(#parameters: #support::ProviderValue,)*
        {
            fn complete<'__geam_call>(
                self,
                mut call: #support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, Self::Host>,
                _constructions: &#support::ProviderConstructions<'__geam_call, Self::RootRequirements>,
            ) -> ::core::result::Result<#support::HostCallCompletion<'__geam_call, Self::Host>, #support::HostCallError> {
                let value = call.create_external_with_binding::<__GeamProvider>(#into_payload);
                ::core::result::Result::Ok(call.return_value(value))
            }
        }
    });
    quote! {
        #(#forms)*
        #immediate_codecs
        #owned_codecs
        #output_codecs
    }
}

impl InputCodecs<'_> {
    fn generate(
        &self,
        view: &TokenStream,
        decoder: &Ident,
        ownership: InputOwnership,
    ) -> TokenStream {
        let Self {
            generic,
            payload,
            host,
            schema,
            arguments,
            support,
        } = self;
        let parameters = &generic.parameters;
        let input = &generic.input;
        let (access, decode, context, construct) = match ownership {
            InputOwnership::Borrowed => (
                quote!(provider_external_view_with),
                quote!(into_external_view),
                quote!(ProviderExternalInputContext),
                quote!(__geam_from_host),
            ),
            InputOwnership::Owned => (
                quote!(provider_external_item_with),
                quote!(into_external),
                quote!(ProviderOwnedExternalInputContext),
                quote!(__geam_from_async_host),
            ),
        };
        quote! {
            impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding, __GeamReturn>
                #support::ProviderInputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn> for #view
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                __GeamReturn: #support::HostType,
                #(#parameters: #support::ProviderValue,)*
            {
                type Host = #host;
                type Requirements = #support::ProviderNoConstructions;

                fn from_host_with<'__geam_call>(
                    call: &mut #support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn>,
                    value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                    _constructions: &#support::ProviderConstructions<'__geam_call, Self::Requirements>,
                ) -> Self {
                    #input::#construct(#support::#context::from_host(call.#access::<__GeamProvider, #schema, #arguments>(value)))
                }
            }

            #[doc(hidden)]
            pub struct #decoder<#(#parameters,)*> {
                access: #support::ProviderExternalPayloadAccess<#payload>,
                parameters: ::core::marker::PhantomData<fn() -> (#(#parameters,)*)>,
            }

            impl<#(#parameters,)*> ::core::clone::Clone for #decoder<#(#parameters,)*> {
                fn clone(&self) -> Self {
                    Self { access: self.access.clone(), parameters: ::core::marker::PhantomData }
                }
            }

            impl<#(#parameters,)*> #support::ProviderTypedListItemDecoder<#view> for #decoder<#(#parameters,)*>
            where #(#parameters: #support::ProviderValue,)*
            { type Host = #host; }

            impl<#(#parameters,)*> #support::ProviderListItemDecoder<#view> for #decoder<#(#parameters,)*>
            where #(#parameters: #support::ProviderValue,)*
            {
                type View = #view;

                fn decode(&self, value: #support::ProviderListItemValue<'_>) -> Self::View {
                    #input::#construct(#support::#context::from_host(value.#decode(&self.access)))
                }
            }

            impl<#(#parameters,)*> #support::ProviderListInputValue for #view
            where #(#parameters: #support::ProviderValue + 'static,)*
            {
                type Host = #host;
                type View = Self;
                type Decoder = #decoder<#(#parameters,)*>;
            }

            impl<#(#parameters,)* __GeamProfile, __GeamProviderBinding>
                #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding> for #view
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                #(#parameters: #support::ProviderValue + 'static,)*
            {
                type Requirements = #support::ProviderNoConstructions;

                fn decoder_with<'__geam_call, __GeamReturn: #support::HostType>(
                    call: &#support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn>,
                    _constructions: &#support::ProviderConstructions<'__geam_call, Self::Requirements>,
                ) -> Self::Decoder {
                    #decoder {
                        access: call.provider_external_payload_access_with::<__GeamProvider, #schema>(),
                        parameters: ::core::marker::PhantomData,
                    }
                }
            }
        }
    }
}
