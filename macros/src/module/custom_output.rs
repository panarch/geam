use super::StaticValueType;
use super::custom_value::{CustomFieldValueType, CustomModel, custom_field_models};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemEnum};

pub(super) struct OutputField {
    pub(super) index: usize,
    pub(super) parameter: Ident,
    pub(super) value_type: TokenStream,
}

pub(super) fn fields(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Vec<OutputField> {
    custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .enumerate()
        .filter_map(|(index, field)| {
            let (value, list) = match &field.value {
                CustomFieldValueType::Value(value) => (value.as_ref(), false),
                CustomFieldValueType::List(list) => (&list.collection.value, true),
            };
            (changes_representation(value, customs)
                || (index == 0 && !custom.parameters.is_empty()))
            .then(|| {
                let value = output_type(value, customs, support);
                OutputField {
                    index,
                    parameter: format_ident!("__GeamOutputField{index}"),
                    value_type: if list {
                        quote!(::std::vec::Vec<#value>)
                    } else {
                        value
                    },
                }
            })
        })
        .collect()
}

// Parameterize only representation-dependent fields. Rust still sees the
// original enum name and derives its requested traits from the actual fields.
pub(super) fn parameterize(item: &mut ItemEnum, custom: &CustomModel, fields: &[OutputField]) {
    for (index, field) in item
        .variants
        .iter_mut()
        .flat_map(|variant| variant.fields.iter_mut())
        .enumerate()
    {
        if let Some(output) = fields.iter().find(|output| output.index == index) {
            let parameter = &output.parameter;
            let default = &field.ty;
            item.generics
                .params
                .push(syn::parse_quote!(#parameter = #default));
            field.ty = if index == 0 && !custom.parameters.is_empty() {
                let shape = format_ident!("__Geam{}Field", custom.ident);
                let parameters = &custom.parameters;
                syn::parse_quote!(<#parameter as #shape<#(#parameters),*>>::Value)
            } else {
                syn::parse_quote!(#parameter)
            };
        }
    }
    if let Some(marker) = phantom_variant(custom) {
        item.variants.push(marker);
    }
}

// Fieldless generic declarations still carry their nominal source parameters.
// The uninhabited variant adds no source constructor and needs no match arm.
pub(super) fn phantom_variant(custom: &CustomModel) -> Option<syn::Variant> {
    if custom.parameters.is_empty()
        || custom
            .constructors
            .iter()
            .any(|constructor| !custom_field_models(&constructor.fields).is_empty())
    {
        return None;
    }
    let mut name = format_ident!("__GeamParameters");
    while custom
        .constructors
        .iter()
        .any(|constructor| constructor.ident == name)
    {
        name = format_ident!("{name}_");
    }
    let parameters = &custom.parameters;
    Some(syn::parse_quote! {
        #[doc(hidden)]
        #[allow(dead_code, reason = "uninhabited nominal type parameter marker")]
        #name(::core::convert::Infallible, ::core::marker::PhantomData<fn() -> (#(#parameters,)*)>)
    })
}

fn changes_representation(value: &StaticValueType, customs: &[CustomModel]) -> bool {
    match value {
        StaticValueType::List(list) => changes_representation(&list.collection.value, customs),
        StaticValueType::Future(_) | StaticValueType::Callback(_) => true,
        StaticValueType::Scalar(_) => false,
        StaticValueType::Declared { .. } => true,
        StaticValueType::External {
            declaration,
            payload,
            ..
        } => *payload != syn::parse_quote!(#declaration),
        StaticValueType::Custom { index } => customs[*index]
            .constructors
            .iter()
            .flat_map(|constructor| custom_field_models(&constructor.fields))
            .any(|field| match &field.value {
                CustomFieldValueType::Value(value) => changes_representation(value, customs),
                CustomFieldValueType::List(list) => {
                    changes_representation(&list.collection.value, customs)
                }
            }),
        StaticValueType::Tuple(elements) => elements
            .iter()
            .any(|value| changes_representation(value, customs)),
        StaticValueType::Result { success, failure } => {
            changes_representation(success, customs) || changes_representation(failure, customs)
        }
        StaticValueType::Option { value } => changes_representation(value, customs),
    }
}

fn output_type(
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match value {
        StaticValueType::List(list) => {
            let item = output_type(&list.collection.value, customs, support);
            quote!(::std::vec::Vec<#item>)
        }
        StaticValueType::Future(future) => {
            let source = &future.source;
            quote!(#support::ProviderFuture<#source>)
        }
        StaticValueType::Callback(callback) => {
            let signature = &callback.signature;
            quote!(#support::Callback<#signature>)
        }
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_ } => {
            quote!(<#type_ as #support::ProviderValueForms>::Output)
        }
        StaticValueType::External { payload, .. } => quote!(#payload),
        StaticValueType::Custom { index } => {
            let ident = &customs[*index].ident;
            quote!(<#ident as #support::ProviderValueForms>::Output)
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|value| output_type(value, customs, support));
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = output_type(success, customs, support);
            let failure = output_type(failure, customs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = output_type(value, customs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}
