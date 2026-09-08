use super::StaticValueType;
use super::custom_value::{CustomFieldValueType, CustomModel, custom_field_models};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemEnum};

pub(super) struct OutputField {
    index: usize,
    pub(super) parameter: Ident,
    pub(super) transfer: TokenStream,
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
            changes_representation(value, customs).then(|| {
                let value = transfer_type(value, customs, support);
                OutputField {
                    index,
                    parameter: format_ident!("__GeamOutputField{index}"),
                    transfer: if list {
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
pub(super) fn parameterize(item: &mut ItemEnum, fields: &[OutputField]) {
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
            field.ty = syn::parse_quote!(#parameter);
        }
    }
}

fn changes_representation(value: &StaticValueType, customs: &[CustomModel]) -> bool {
    match value {
        StaticValueType::Scalar(_) => false,
        StaticValueType::Declared { .. } => true,
        StaticValueType::External {
            payload,
            transfer_payload,
            ..
        } => *transfer_payload != syn::parse_quote!(#payload),
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

fn transfer_type(
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match value {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_ } => {
            quote!(<#type_ as #support::ProviderTransferValue>::Output)
        }
        StaticValueType::External {
            transfer_payload, ..
        } => quote!(#transfer_payload),
        StaticValueType::Custom { index } => {
            let ident = &customs[*index].ident;
            quote!(<#ident as #support::ProviderTransferValue>::Output)
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|value| transfer_type(value, customs, support));
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = transfer_type(success, customs, support);
            let failure = transfer_type(failure, customs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = transfer_type(value, customs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}
