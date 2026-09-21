use super::custom_value::{CustomFieldValueType, CustomModel, custom_field_models};
use super::{
    CallbackType, GeneratedCallback, GeneratedConstruction, GeneratedNames, InputOwnership,
    StaticValueType,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

pub(super) fn generics(custom: &CustomModel) -> Vec<super::FunctionGeneric> {
    custom
        .parameters
        .iter()
        .enumerate()
        .map(|(index, ident)| super::FunctionGeneric {
            ident: ident.clone(),
            index,
            nominal: true,
        })
        .collect()
}

pub(super) fn source_type(custom: &CustomModel) -> TokenStream {
    named_type(&custom.ident, &custom.parameters)
}

pub(super) fn named_type(name: &syn::Ident, parameters: &[syn::Ident]) -> TokenStream {
    if parameters.is_empty() {
        quote!(#name)
    } else {
        quote!(#name<#(#parameters),*>)
    }
}

pub(super) fn schema_type(custom: &CustomModel, customs: &[CustomModel]) -> TokenStream {
    let schema = &custom.schema;
    if has_profile(custom, customs) {
        quote!(#schema<__GeamProfile>)
    } else {
        quote!(#schema)
    }
}

pub(super) fn has_profile(custom: &CustomModel, customs: &[CustomModel]) -> bool {
    custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .any(|field| {
            let value = match &field.value {
                CustomFieldValueType::Value(value) => value.as_ref(),
                CustomFieldValueType::List(list) => &list.collection.value,
            };
            !super::list_capability::capabilities(value, customs).is_empty()
                || !super::list::list_declared_accesses(value, customs).is_empty()
        })
}

pub(super) fn host_type(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let schema = schema_type(custom, customs);
    if custom.parameters.is_empty() {
        return quote!(#support::HostCustomType<#schema>);
    }
    let parameters = custom
        .parameters
        .iter()
        .map(|parameter| quote!(<#parameter as #support::ProviderValue>::Host))
        .collect::<Vec<_>>();
    let arguments = super::signature::host_type_token_sequence(&parameters, support);
    quote!(#support::HostCustomType<#schema, #arguments>)
}

pub(super) fn constructor_type(
    custom: &CustomModel,
    marker: &syn::Ident,
    customs: &[CustomModel],
) -> TokenStream {
    let parameters = &custom.parameters;
    if has_profile(custom, customs) {
        quote!(#marker<__GeamProfile, #(#parameters,)*>)
    } else {
        named_type(marker, parameters)
    }
}

pub(super) fn list_marker_type(
    custom: &CustomModel,
    marker: &syn::Ident,
    decoder: &TokenStream,
) -> TokenStream {
    let parameters = &custom.parameters;
    quote!(#marker<#(#parameters,)* #decoder>)
}

pub(super) fn callbacks(index: usize, customs: &[CustomModel]) -> Vec<&CallbackType> {
    let mut found = Vec::new();
    for field in customs[index]
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
    {
        let value = match &field.value {
            CustomFieldValueType::Value(value) => value.as_ref(),
            CustomFieldValueType::List(list) => &list.collection.value,
        };
        found.extend(super::list_capability::callbacks(value, customs));
    }
    found
}

pub(super) fn is_contextual(index: usize, customs: &[CustomModel]) -> bool {
    !super::list_capability::capabilities(&StaticValueType::Custom { index }, customs).is_empty()
        || !super::list::list_declared_accesses(&StaticValueType::Custom { index }, customs)
            .is_empty()
}

pub(super) fn invocation_requirements(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    if !super::list_capability::capabilities(&StaticValueType::Custom { index }, customs).is_empty()
    {
        return quote!(#support::ProviderInvocationRequired);
    }
    super::list::list_declared_accesses(&StaticValueType::Custom { index }, customs)
        .iter()
        .rev()
        .fold(quote!(()), |tail, access| {
            let type_ = &access.type_;
            quote!((<#type_ as #support::ProviderValueForms>::InvocationRequirements, #tail))
        })
}

pub(super) fn constructions(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
) -> (
    Vec<GeneratedConstruction>,
    BTreeMap<String, syn::Ident>,
    Vec<TokenStream>,
) {
    let mut names = GeneratedNames::default();
    let mut constructions = Vec::new();
    let mut bindings = BTreeMap::new();
    let mut bounds = Vec::new();
    for codec in super::callback::custom_codecs(&customs[index], customs) {
        let generated = codec.generate(&generics(&customs[index]), customs, support);
        if generated.has_constructions && !bindings.contains_key(&codec.key()) {
            let binding = names.next("custom_constructions");
            constructions.push(GeneratedConstruction {
                requirement: generated.requirements,
                binding: binding.clone(),
            });
            bindings.insert(codec.key(), binding);
        }
        bounds.extend(generated.bounds);
    }
    (constructions, bindings, bounds)
}

pub(super) fn requirements(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
) -> GeneratedCallback {
    let (constructions, _, bounds) = constructions(index, customs, support);
    GeneratedCallback {
        definition: TokenStream::new(),
        requirements: super::function::provider_requirement_sequence(&constructions, support),
        has_constructions: !constructions.is_empty(),
        bounds,
    }
}

pub(super) fn forms(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    profile: &TokenStream,
) -> TokenStream {
    if is_contextual(index, customs) {
        quote!(#support::ProviderContextualValueForms<#profile>)
    } else {
        quote!(#support::ProviderValueForms)
    }
}

pub(super) fn input_type(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
) -> TokenStream {
    let source = source_type(&customs[index]);
    let forms = forms(index, customs, support, profile);
    let member = match flavor {
        InputOwnership::Borrowed => quote!(ImmediateInput),
        InputOwnership::Owned => quote!(OwnedInput),
    };
    quote!(<#source as #forms>::#member)
}

pub(super) fn defined_input_type(
    index: usize,
    input: &syn::Ident,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
) -> TokenStream {
    let custom = &customs[index];
    let alias = super::list::custom_input_ident(input, flavor);
    if !is_contextual(index, customs) {
        return named_type(&alias, &custom.parameters);
    }
    let fields = custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .map(|field| {
            field_type(
                &field.value,
                customs,
                support,
                flavor,
                profile,
                ValueForm::Input,
            )
        });
    let parameters = &custom.parameters;
    quote!(#alias<#(#parameters,)* #(#fields,)* #profile>)
}

pub(super) fn output_type(
    index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    profile: &TokenStream,
) -> TokenStream {
    let custom = &customs[index];
    let name = &custom.ident;
    if !is_contextual(index, customs) {
        let source = source_type(custom);
        return quote!(<#source as #support::ProviderValueForms>::Output);
    }
    let fields = custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .collect::<Vec<_>>();
    let parameters = super::custom_output::fields(custom, customs, support)
        .into_iter()
        .map(|field| {
            field_type(
                &fields[field.index].value,
                customs,
                support,
                InputOwnership::Owned,
                profile,
                ValueForm::Output,
            )
        })
        .collect::<Vec<_>>();
    let source_parameters = &custom.parameters;
    quote!(#name<#(#source_parameters,)* #(#parameters,)*>)
}

pub(super) fn list_marker_ident(input: &syn::Ident, flavor: InputOwnership) -> syn::Ident {
    let input = super::list::custom_input_ident(input, flavor);
    quote::format_ident!("{input}List")
}

pub(super) fn marker_fields(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Vec<TokenStream> {
    custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .map(|field| {
            field_type(
                &field.value,
                customs,
                support,
                flavor,
                &TokenStream::new(),
                ValueForm::Marker,
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
enum ValueForm {
    Input,
    Output,
    Marker,
}

fn field_type(
    field: &CustomFieldValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
    form: ValueForm,
) -> TokenStream {
    match field {
        CustomFieldValueType::Value(value) => {
            value_type(value, customs, support, flavor, profile, form)
        }
        CustomFieldValueType::List(list) => {
            if matches!(form, ValueForm::Marker) {
                if let StaticValueType::Declared { type_ } = &list.collection.value {
                    let member = match flavor {
                        InputOwnership::Borrowed => quote!(Immediate),
                        InputOwnership::Owned => quote!(Owned),
                    };
                    return quote!(<#type_ as #support::ProviderMarkerListForms>::#member);
                }
                if super::list_capability::capabilities(&list.collection.value, customs).is_empty()
                    && !super::list_capability::has_profile(&list.collection.value, customs)
                    && super::list::list_declared_accesses(&list.collection.value, customs)
                        .is_empty()
                {
                    let item = super::signature::static_list_item_type(
                        &list.collection.value,
                        customs,
                        support,
                        flavor,
                        profile,
                    );
                    let host = super::signature::host_static_value_type(
                        &list.collection.value,
                        customs,
                        support,
                    );
                    let decoder = super::list::default_list_decoder_type(list, customs, flavor);
                    return quote!(#support::List<#item, #support::ProviderListContext<#host, #decoder>>);
                }
                let item = value_type(
                    &list.collection.value,
                    customs,
                    support,
                    flavor,
                    profile,
                    form,
                );
                quote!(#support::List<#item>)
            } else if matches!(form, ValueForm::Output) {
                let item = value_type(
                    &list.collection.value,
                    customs,
                    support,
                    flavor,
                    profile,
                    ValueForm::Output,
                );
                quote!(::std::vec::Vec<#item>)
            } else {
                let value =
                    super::signature::list_signature_type(list, customs, support, flavor, profile);
                quote!(#value)
            }
        }
    }
}

fn value_type(
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
    form: ValueForm,
) -> TokenStream {
    match value {
        StaticValueType::Future(future) => {
            if matches!(form, ValueForm::Marker) {
                let source = &future.source;
                return quote!(#support::ProviderFuture<#source>);
            }
            let value =
                super::signature::future_input_signature_type(future, customs, support, profile);
            quote!(#value)
        }
        StaticValueType::Callback(callback) => {
            if matches!(form, ValueForm::Marker) {
                let signature = &callback.signature;
                return quote!(#support::Callback<#signature>);
            }
            let value =
                super::signature::callback_signature_type(callback, customs, profile, support);
            quote!(#value)
        }
        StaticValueType::Custom { index } => match form {
            ValueForm::Output => output_type(*index, customs, support, profile),
            ValueForm::Input => input_type(*index, customs, support, flavor, profile),
            ValueForm::Marker => {
                let source = source_type(&customs[*index]);
                match flavor {
                    InputOwnership::Borrowed => {
                        quote!(<#source as #support::ProviderValueForms>::ImmediateInput)
                    }
                    InputOwnership::Owned => {
                        quote!(<#source as #support::ProviderValueForms>::OwnedInput)
                    }
                }
            }
        },
        StaticValueType::List(list) => field_type(
            &CustomFieldValueType::List(list.clone()),
            customs,
            support,
            flavor,
            profile,
            form,
        ),
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|value| value_type(value, customs, support, flavor, profile, form));
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = value_type(success, customs, support, flavor, profile, form);
            let failure = value_type(failure, customs, support, flavor, profile, form);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = value_type(value, customs, support, flavor, profile, form);
            quote!(::core::option::Option<#value>)
        }
        StaticValueType::Scalar(value) => quote!(#value),
        StaticValueType::Declared { type_ } => {
            let member = if matches!(form, ValueForm::Output) {
                quote!(Output)
            } else {
                match flavor {
                    InputOwnership::Borrowed => quote!(ImmediateInput),
                    InputOwnership::Owned => quote!(OwnedInput),
                }
            };
            let forms = if matches!(form, ValueForm::Marker) {
                quote!(#support::ProviderValueForms)
            } else {
                quote!(#support::ProviderContextualValueForms<#profile>)
            };
            quote!(<#type_ as #forms>::#member)
        }
        StaticValueType::External { payload, .. } => {
            if matches!(form, ValueForm::Output) {
                quote!(#payload)
            } else {
                match flavor {
                    InputOwnership::Borrowed => quote!(#support::ProviderExternalView<#payload>),
                    InputOwnership::Owned => quote!(#support::ProviderOwnedExternal<#payload>),
                }
            }
        }
    }
}

pub(super) fn construction_proof(
    key: &str,
    constructions: &BTreeMap<String, syn::Ident>,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    if let Some(proof) = constructions.get(key) {
        return quote!(#proof);
    }
    for (index, custom) in customs.iter().enumerate() {
        if let Some(proof) = constructions.get(&custom.schema.to_string())
            && let Some(selected) =
                select_construction_proof(key, index, quote!(#proof), customs, support)
        {
            return selected;
        }
    }
    quote!(#support::ProviderConstructions::<#support::ProviderNoConstructions>::none())
}

fn select_construction_proof(
    key: &str,
    index: usize,
    proof: TokenStream,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Option<TokenStream> {
    let mut seen = std::collections::BTreeSet::new();
    let mut position = 0;
    for codec in super::callback::custom_codecs(&customs[index], customs) {
        if !seen.insert(codec.key()) || !codec.generate(&[], customs, support).has_constructions {
            continue;
        }
        let selector = super::function::provider_construction_index(position, support);
        let selected = quote!(#proof.select::<#selector>());
        if codec.key() == key {
            return Some(selected);
        }
        if let super::callback::InputCodec::Custom { index, .. } = codec
            && let Some(selected) =
                select_construction_proof(key, index, selected, customs, support)
        {
            return Some(selected);
        }
        position += 1;
    }
    None
}
