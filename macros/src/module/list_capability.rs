use super::custom_value::{CustomFieldValueType, CustomModel, custom_field_models};
use super::{
    CallbackType, FutureInputType, InputEnvironment, InputOwnership, ListType, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub(super) fn has_profile(value: &StaticValueType, customs: &[CustomModel]) -> bool {
    match value {
        StaticValueType::Custom { index } => {
            super::custom_context::has_profile(&customs[*index], customs)
        }
        StaticValueType::List(list) => has_profile(&list.collection.value, customs),
        StaticValueType::Tuple(elements) => {
            elements.iter().any(|value| has_profile(value, customs))
        }
        StaticValueType::Result { success, failure } => {
            has_profile(success, customs) || has_profile(failure, customs)
        }
        StaticValueType::Option { value } => has_profile(value, customs),
        _ => false,
    }
}

#[derive(Clone, Copy)]
pub(super) enum Capability<'model> {
    Callback(&'model CallbackType),
    Future(&'model FutureInputType),
}

impl Capability<'_> {
    pub(super) fn names(self) -> (Ident, Ident, Ident) {
        match self {
            Self::Callback(callback) => names(callback),
            Self::Future(future) => future_names(future),
        }
    }

    pub(super) fn source(self) -> TokenStream {
        match self {
            Self::Callback(callback) => {
                let signature = &callback.signature;
                quote!(#signature)
            }
            Self::Future(future) => {
                let source = &future.source;
                quote!(#source)
            }
        }
    }

    pub(super) fn marker(self, support: &TokenStream) -> TokenStream {
        let (source, _, _) = self.names();
        match self {
            Self::Callback(_) => quote!(#support::Callback<#source>),
            Self::Future(_) => quote!(#support::ProviderFuture<#source>),
        }
    }

    pub(super) fn requires_work(self, customs: &[CustomModel]) -> bool {
        match self {
            Self::Callback(callback) => super::callback::requires_work(callback, customs),
            Self::Future(_) => true,
        }
    }

    fn decoder(
        self,
        customs: &[CustomModel],
        support: &TokenStream,
        profile: &TokenStream,
    ) -> TokenStream {
        match self {
            Self::Callback(callback) => {
                let forms =
                    super::signature::callback_runtime_forms(callback, customs, support, profile);
                quote!(#support::ProviderCallbackListDecoder<#profile, #(#forms),*>)
            }
            Self::Future(future) => {
                let host =
                    super::signature::host_input_type(&future.value, customs, support, profile);
                let output = super::signature::callback_input_signature_type(
                    &future.value,
                    customs,
                    support,
                    InputOwnership::Owned,
                    profile,
                );
                quote!(#support::ProviderFutureListDecoder<#profile, #host, #output>)
            }
        }
    }
}

pub(super) fn capabilities<'model>(
    value: &'model StaticValueType,
    customs: &'model [CustomModel],
) -> Vec<Capability<'model>> {
    fn collect<'model>(
        value: &'model StaticValueType,
        customs: &'model [CustomModel],
        found: &mut Vec<Capability<'model>>,
    ) {
        match value {
            StaticValueType::Callback(callback) => found.push(Capability::Callback(callback)),
            StaticValueType::Future(future) => found.push(Capability::Future(future)),
            StaticValueType::List(list) => collect(&list.collection.value, customs, found),
            StaticValueType::Tuple(elements) => {
                for value in elements {
                    collect(value, customs, found);
                }
            }
            StaticValueType::Result { success, failure } => {
                collect(success, customs, found);
                collect(failure, customs, found);
            }
            StaticValueType::Option { value } => collect(value, customs, found),
            StaticValueType::Custom { index } => {
                for field in customs[*index]
                    .constructors
                    .iter()
                    .flat_map(|constructor| custom_field_models(&constructor.fields))
                {
                    let value = match &field.value {
                        CustomFieldValueType::Value(value) => value.as_ref(),
                        CustomFieldValueType::List(list) => &list.collection.value,
                    };
                    collect(value, customs, found);
                }
            }
            StaticValueType::Scalar(_)
            | StaticValueType::Declared { .. }
            | StaticValueType::External { .. } => {}
        }
    }
    let mut found = Vec::new();
    collect(value, customs, &mut found);
    let mut names = ::std::collections::BTreeSet::new();
    found.retain(|value| names.insert(value.names().0.to_string()));
    found
}

pub(super) fn callbacks<'model>(
    value: &'model StaticValueType,
    customs: &'model [CustomModel],
) -> Vec<&'model CallbackType> {
    capabilities(value, customs)
        .into_iter()
        .filter_map(|value| match value {
            Capability::Callback(callback) => Some(callback),
            Capability::Future(_) => None,
        })
        .collect()
}

pub(super) fn future_names(future: &FutureInputType) -> (Ident, Ident, Ident) {
    let source = &future.source;
    let encoded = quote!(#source)
        .to_string()
        .bytes()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    (
        format_ident!("__GeamListFuture{encoded}"),
        format_ident!("__GeamWorkDecoder{encoded}"),
        format_ident!("__geam_work_{encoded}"),
    )
}

// Encode the complete source signature, avoiding collisions or dependence on
// the nominal codec name of the provider function that first registered a list.
pub(super) fn names(callback: &CallbackType) -> (Ident, Ident, Ident) {
    let signature = &callback.signature;
    let encoded = quote!(#signature)
        .to_string()
        .bytes()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    (
        format_ident!("__GeamListSignature{encoded}"),
        format_ident!("__GeamListDecoder{encoded}"),
        format_ident!("__geam_callable_{encoded}"),
    )
}

pub(super) fn source_parameters<'model>(
    value: &StaticValueType,
    customs: &'model [CustomModel],
) -> &'model [Ident] {
    match value {
        StaticValueType::Custom { index } => &customs[*index].parameters,
        _ => &[],
    }
}

pub(super) fn decoder_type(
    list: &ListType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
) -> TokenStream {
    decoder_type_for(
        &list.decoder,
        &list.collection.value,
        customs,
        support,
        flavor,
        profile,
    )
}

pub(super) fn decoder_type_for(
    ident: &Ident,
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
) -> TokenStream {
    if let StaticValueType::Declared { type_, .. } = value {
        let member = match flavor {
            InputOwnership::Borrowed => quote!(ImmediateListInput),
            InputOwnership::Owned => quote!(OwnedListInput),
        };
        return quote!(<<#type_ as #support::ProviderContextualValueForms<#profile>>::#member as #support::ProviderListInputValue>::Decoder);
    }
    let ident = super::list::list_decoder_ident(ident, flavor);
    let retained = capabilities(value, customs);
    let declared = super::list::list_declared_accesses(value, customs);
    let source_parameters = source_parameters(value, customs);
    let profile_argument = has_profile(value, customs).then(|| quote!(#profile,));
    if source_parameters.is_empty() && retained.is_empty() && declared.is_empty() {
        return quote!(#ident);
    }
    let arguments = retained.iter().flat_map(|capability| [capability.source(), capability.decoder(customs, support, profile)])
        .chain(declared.iter().flat_map(|access| {
            let type_ = &access.type_;
            let item = match flavor {
                InputOwnership::Borrowed => {
                    quote!(<#type_ as #support::ProviderContextualValueForms<#profile>>::ImmediateListInput)
                }
                InputOwnership::Owned => {
                    quote!(<#type_ as #support::ProviderContextualValueForms<#profile>>::OwnedListInput)
                }
            };
            let decoder = quote!(<#item as #support::ProviderListInputValue>::Decoder);
            [item, decoder]
        }));
    quote!(#ident<#profile_argument #(#source_parameters,)* #(#arguments),*>)
}

pub(super) fn nested_decoder_type(
    list: &ListType,
    customs: &[CustomModel],
    flavor: InputOwnership,
) -> TokenStream {
    if let StaticValueType::Declared { type_, .. } = &list.collection.value {
        let (_, decoder) = super::list::declared_names(type_);
        return quote!(#decoder);
    }
    let ident = super::list::list_decoder_ident(&list.decoder, flavor);
    let source_parameters = source_parameters(&list.collection.value, customs);
    let profile_argument =
        has_profile(&list.collection.value, customs).then(|| quote!(__GeamProfile,));
    let parameters = capabilities(&list.collection.value, customs)
        .iter()
        .flat_map(|capability| {
            let (signature, decoder, _) = capability.names();
            [signature, decoder]
        })
        .chain(
            super::list::list_declared_accesses(&list.collection.value, customs)
                .iter()
                .flat_map(|access| {
                    let (item, decoder) = super::list::declared_names(&access.type_);
                    [item, decoder]
                }),
        )
        .collect::<Vec<_>>();
    if source_parameters.is_empty() && parameters.is_empty() {
        quote!(#ident)
    } else {
        quote!(#ident<#profile_argument #(#source_parameters,)* #(#parameters),*>)
    }
}

pub(super) fn decoder_fields(
    value: &StaticValueType,
    environment: &InputEnvironment<'_>,
) -> TokenStream {
    let support = environment.support;
    capabilities(value, environment.customs).iter().map(|capability| {
        let (_, _, field) = capability.names();
        let (codec, generics) = match capability {
            Capability::Callback(callback) => (&callback.codec, &callback.generics),
            Capability::Future(future) => (&future.codec, &future.generics),
        };
        let proof = super::custom_context::construction_proof(&codec.to_string(), environment.callback_constructions, environment.customs, support);
        let codec = super::callback::instantiated_codec_type(codec, generics, environment);
        match capability {
            Capability::Callback(_) => quote!(#field: #support::ProviderOwnedCallbackListDecoder::<__GeamProfile, __GeamProvider, #codec>::from_host_with::<#codec, __GeamProvider, _, _>(&call, #proof),),
            Capability::Future(_) => quote!(#field: #support::ProviderFutureListDecoder::from_host_with::<#codec, __GeamProvider, _, _>(&call, #proof),),
        }
    }).collect()
}
