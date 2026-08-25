use super::custom_value::{
    CustomConstructorModel, CustomFieldValueType, CustomFields, CustomModel, custom_field_models,
};
use super::list_model::static_value_key;
use super::{
    FunctionArgumentType, FunctionInputType, FunctionModel, FunctionReturnType, GeneratedNames,
    GeneratedValue, ListDeclaredAccess, ListDecoderModel, ListExternalAccess, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::{Ident, Type};

pub(super) fn validate_list_return(function: &FunctionModel) -> syn::Result<()> {
    if let FunctionReturnType::List(returned) = &function.return_
        && !function.arguments.iter().any(|argument| {
            matches!(
                argument,
                FunctionArgumentType::Input(FunctionInputType::List(argument))
                    if static_value_key(&argument.collection.value)
                        == static_value_key(&returned.collection.value)
            )
        })
    {
        return Err(syn::Error::new_spanned(
            &returned.collection.source,
            "a returned geam::List<T> must match a List argument",
        ));
    }
    Ok(())
}

pub(super) fn generate_list_decoder(
    decoder: &ListDecoderModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    let ident = &decoder.ident;
    let item = validated_list_item_type(&decoder.value, custom_inputs, support);
    let accesses = list_external_accesses(&decoder.value, customs);
    let declared = list_declared_accesses(&decoder.value, customs);
    let fields = accesses.iter().map(|access| {
        let field = &access.field;
        let payload = &access.payload;
        quote!(#field: #support::ProviderExternalPayloadAccess<#payload>,)
    });
    let declared_fields = declared.iter().map(|access| {
        let field = &access.field;
        let type_ = &access.type_;
        quote!(
            #field: <<#type_ as #support::ProviderValue>::ListInput as
                #support::ProviderListInputValue>::Decoder,
        )
    });
    let definition = if accesses.is_empty() && declared.is_empty() {
        quote! {
            #[doc(hidden)]
            #[derive(Clone, Copy)]
            pub struct #ident;
        }
    } else {
        quote! {
            #[doc(hidden)]
            #[derive(Clone)]
            pub struct #ident {
                #(#fields)*
                #(#declared_fields)*
            }
        }
    };
    let mut names = GeneratedNames::default();
    let decoded = decode_list_item(
        &decoder.value,
        quote!(__geam_value),
        customs,
        custom_inputs,
        support,
        &mut names,
    );
    let statements = decoded.statements;
    let value = decoded.value;
    let view = list_item_view_type(&decoder.value, custom_inputs, support);
    quote! {
        #definition

        impl #support::ProviderListItemDecoder<#item> for #ident {
            type View = #view;

            fn decode(
                &self,
                __geam_value: #support::ProviderListItemValue<'_>,
            ) -> Self::View {
                #statements
                #value
            }
        }
    }
}

fn validated_list_item_type(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            quote!(<#type_ as #support::ProviderValue>::ListInput)
        }
        StaticValueType::External { payload, .. } => quote!(#payload),
        StaticValueType::Custom { index, .. } => {
            let input = &custom_inputs[index];
            quote!(#input)
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| validated_list_item_type(element, custom_inputs, support))
                .collect::<Vec<_>>();
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = validated_list_item_type(success, custom_inputs, support);
            let failure = validated_list_item_type(failure, custom_inputs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = validated_list_item_type(value, custom_inputs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}

pub(super) fn list_decoder_value(
    ident: &Ident,
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let accesses = list_external_accesses(value, customs);
    let declared = list_declared_accesses(value, customs);
    if accesses.is_empty() && declared.is_empty() {
        quote!(#ident)
    } else {
        let fields = accesses.iter().map(|access| {
            let field = &access.field;
            let schema = &access.schema;
            quote!(
                #field: call.provider_external_payload_access_with::<
                    __GeamProvider,
                    #schema,
                >(),
            )
        });
        let declared_fields = declared.iter().map(|access| {
            let field = &access.field;
            let type_ = &access.type_;
            quote!(
                #field: <<#type_ as #support::ProviderValue>::ListInput as
                    #support::ProviderListInputCodec<Profile>>::decoder(&call),
            )
        });
        quote! {
            #ident {
                #(#fields)*
                #(#declared_fields)*
            }
        }
    }
}

pub(super) fn list_declared_accesses(
    type_: &StaticValueType,
    customs: &[CustomModel],
) -> Vec<ListDeclaredAccess> {
    fn collect_custom_field(
        type_: &CustomFieldValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListDeclaredAccess>,
    ) {
        match type_ {
            CustomFieldValueType::Value(type_) => collect(type_, customs, accesses),
            CustomFieldValueType::List(list) => collect(&list.collection.value, customs, accesses),
        }
    }

    fn collect(
        type_: &StaticValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListDeclaredAccess>,
    ) {
        match type_ {
            StaticValueType::Declared { type_, .. } => {
                let field = declared_access_field(type_);
                if accesses.iter().any(|access| access.field == field) {
                    return;
                }
                accesses.push(ListDeclaredAccess {
                    type_: type_.clone(),
                    field,
                });
            }
            StaticValueType::Tuple(elements) => {
                for element in elements {
                    collect(element, customs, accesses);
                }
            }
            StaticValueType::Result { success, failure } => {
                collect(success, customs, accesses);
                collect(failure, customs, accesses);
            }
            StaticValueType::Option { value } => collect(value, customs, accesses),
            StaticValueType::Custom { index, .. } => {
                for constructor in &customs[*index].constructors {
                    for field in custom_field_models(&constructor.fields) {
                        collect_custom_field(&field.value, customs, accesses);
                    }
                }
            }
            StaticValueType::Scalar(_) | StaticValueType::External { .. } => {}
        }
    }

    let mut accesses = Vec::new();
    collect(type_, customs, &mut accesses);
    accesses
}

fn declared_access_field(type_: &Type) -> Ident {
    let encoded = quote!(#type_)
        .to_string()
        .bytes()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format_ident!("__geam_declared_{encoded}")
}

fn list_external_accesses(
    type_: &StaticValueType,
    customs: &[CustomModel],
) -> Vec<ListExternalAccess> {
    fn collect_custom_field(
        type_: &CustomFieldValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListExternalAccess>,
    ) {
        match type_ {
            CustomFieldValueType::Value(type_) => collect(type_, customs, accesses),
            CustomFieldValueType::List(list) => {
                collect(&list.collection.value, customs, accesses);
            }
        }
    }

    fn collect(
        type_: &StaticValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListExternalAccess>,
    ) {
        match type_ {
            StaticValueType::Scalar(_) | StaticValueType::Declared { .. } => {}
            StaticValueType::External {
                payload,
                schema,
                store_field,
            } => {
                if accesses.iter().any(|access| access.schema == *schema) {
                    return;
                }
                accesses.push(ListExternalAccess {
                    payload: payload.clone(),
                    schema: schema.clone(),
                    field: store_field.clone(),
                });
            }
            StaticValueType::Tuple(elements) => {
                for element in elements {
                    collect(element, customs, accesses);
                }
            }
            StaticValueType::Result { success, failure } => {
                collect(success, customs, accesses);
                collect(failure, customs, accesses);
            }
            StaticValueType::Option { value } => collect(value, customs, accesses),
            StaticValueType::Custom { index, .. } => {
                for constructor in &customs[*index].constructors {
                    for field in custom_field_models(&constructor.fields) {
                        collect_custom_field(&field.value, customs, accesses);
                    }
                }
            }
        }
    }

    let mut accesses = Vec::new();
    collect(type_, customs, &mut accesses);
    accesses
}

fn decode_list_item(
    type_: &StaticValueType,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        StaticValueType::Scalar(type_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_scalar::<#type_>()),
        },
        StaticValueType::Declared { type_, .. } => {
            let field = declared_access_field(type_);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(#support::ProviderListItemDecoder::decode(&self.#field, #input)),
            }
        }
        StaticValueType::External { store_field, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_external(&self.#store_field)),
        },
        StaticValueType::Custom { index, .. } => {
            decode_list_custom(*index, input, customs, custom_inputs, support, names)
        }
        StaticValueType::Tuple(elements) => {
            let tuple = names.next("list_tuple");
            let decoded_elements = elements
                .iter()
                .map(|_| names.next("decoded_list_tuple_element"))
                .collect::<Vec<_>>();
            let mut statements = quote! {
                let mut #tuple = #input.into_tuple();
            };
            for (index, (element, decoded)) in
                elements.iter().zip(&decoded_elements).enumerate().rev()
            {
                let host = names.next("list_tuple_element");
                let generated = decode_list_item(
                    element,
                    quote!(#host),
                    customs,
                    custom_inputs,
                    support,
                    names,
                );
                let generated_statements = generated.statements;
                let generated_value = generated.value;
                statements.extend(quote! {
                    let #host = #tuple.take_item(#index);
                    #generated_statements
                    let #decoded = #generated_value;
                });
            }
            GeneratedValue {
                statements,
                value: quote!((#(#decoded_elements,)*)),
            }
        }
        StaticValueType::Result { success, failure } => {
            let custom = names.next("list_result");
            let field = names.next("list_result_field");
            let decoded_success = decode_list_item(
                success,
                quote!(#field),
                customs,
                custom_inputs,
                support,
                names,
            );
            let success_statements = decoded_success.statements;
            let success_value = decoded_success.value;
            let decoded_failure = decode_list_item(
                failure,
                quote!(#field),
                customs,
                custom_inputs,
                support,
                names,
            );
            let failure_statements = decoded_failure.statements;
            let failure_value = decoded_failure.value;
            GeneratedValue {
                statements: quote! {
                    let mut #custom = #input.into_custom();
                },
                value: quote! {
                    match #custom.constructor() {
                        0 => {
                            let #field = #custom.take_field(0);
                            #success_statements
                            ::core::result::Result::Ok(#success_value)
                        }
                        _ => {
                            let #field = #custom.take_field(0);
                            #failure_statements
                            ::core::result::Result::Err(#failure_value)
                        }
                    }
                },
            }
        }
        StaticValueType::Option { value } => {
            let custom = names.next("list_option");
            let field = names.next("list_option_field");
            let decoded = decode_list_item(
                value,
                quote!(#field),
                customs,
                custom_inputs,
                support,
                names,
            );
            let statements = decoded.statements;
            let value = decoded.value;
            GeneratedValue {
                statements: quote! {
                    let mut #custom = #input.into_custom();
                },
                value: quote! {
                    match #custom.constructor() {
                        0 => {
                            let #field = #custom.take_field(0);
                            #statements
                            ::core::option::Option::Some(#value)
                        }
                        _ => ::core::option::Option::None,
                    }
                },
            }
        }
    }
}

fn decode_list_custom_field(
    type_: &CustomFieldValueType,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_list_item(type_, input, customs, custom_inputs, support, names)
        }
        CustomFieldValueType::List(list) => {
            let decoder = nested_list_decoder_value(&list.decoder, &list.collection.value, customs);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(#input.into_list(#decoder)),
            }
        }
    }
}

fn decode_list_custom(
    custom_index: usize,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let custom = &customs[custom_index];
    let input_type = &custom_inputs[&custom_index];
    let custom_value = names.next("list_custom");
    let mut arms = Vec::new();
    for (constructor_index, constructor) in custom.constructors.iter().enumerate() {
        let fields = custom_field_models(&constructor.fields);
        let mut decoded_names = Vec::with_capacity(fields.len());
        for _ in fields {
            decoded_names.push(names.next("decoded_list_custom_field"));
        }
        let mut statements = TokenStream::new();
        for (field_index, (field, decoded)) in fields.iter().zip(&decoded_names).enumerate().rev() {
            let host = names.next("list_custom_field");
            let generated = decode_list_custom_field(
                &field.value,
                quote!(#host),
                customs,
                custom_inputs,
                support,
                names,
            );
            let generated_statements = generated.statements;
            let generated_value = generated.value;
            statements.extend(quote! {
                let #host = #custom_value.take_field(#field_index);
                #generated_statements
                let #decoded = #generated_value;
            });
        }
        let expression = custom_input_expression(input_type, constructor, &decoded_names);
        let pattern = if constructor_index + 1 == custom.constructors.len() {
            quote!(_)
        } else {
            quote!(#constructor_index)
        };
        arms.push(quote! {
            #pattern => {
                #statements
                #expression
            }
        });
    }
    GeneratedValue {
        statements: quote! {
            let mut #custom_value = #input.into_custom();
        },
        value: quote! {
            match #custom_value.constructor() {
                #(#arms,)*
            }
        },
    }
}

fn nested_list_decoder_value(
    ident: &Ident,
    value: &StaticValueType,
    customs: &[CustomModel],
) -> TokenStream {
    let accesses = list_external_accesses(value, customs);
    let declared = list_declared_accesses(value, customs);
    if accesses.is_empty() && declared.is_empty() {
        quote!(#ident)
    } else {
        let mut fields = Vec::with_capacity(accesses.len());
        for access in &accesses {
            let field = &access.field;
            fields.push(quote!(#field: self.#field.clone(),));
        }
        let mut declared_fields = Vec::with_capacity(declared.len());
        for access in &declared {
            let field = &access.field;
            declared_fields.push(quote!(#field: self.#field.clone(),));
        }
        quote! {
            #ident {
                #(#fields)*
                #(#declared_fields)*
            }
        }
    }
}

pub(super) fn custom_input_expression(
    input: &Ident,
    constructor: &CustomConstructorModel,
    values: &[Ident],
) -> TokenStream {
    let variant = &constructor.ident;
    match &constructor.fields {
        CustomFields::Unit => quote!(#input::#variant),
        CustomFields::Unnamed(_) => quote!(#input::#variant(#(#values),*)),
        CustomFields::Named(fields) => {
            let mut names = Vec::with_capacity(fields.len());
            for field in fields {
                names.push(&field.ident);
            }
            quote!(#input::#variant { #(#names: #values),* })
        }
    }
}

pub(super) fn list_item_view_type(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            quote!(<<#type_ as #support::ProviderValue>::ListInput as
                #support::ProviderListInputValue>::View)
        }
        StaticValueType::External { payload, .. } => {
            quote!(#support::ProviderExternalItem<#payload>)
        }
        StaticValueType::Custom { index, .. } => {
            let input = &custom_inputs[index];
            quote!(#input)
        }
        StaticValueType::Tuple(elements) => {
            let types = elements
                .iter()
                .map(|element| list_item_view_type(element, custom_inputs, support))
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = list_item_view_type(success, custom_inputs, support);
            let failure = list_item_view_type(failure, custom_inputs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = list_item_view_type(value, custom_inputs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::list_item_view_type;
    use crate::module::StaticValueType;
    use quote::quote;

    #[test]
    fn item_views_preserve_recursive_result_and_option_direction() {
        let input = StaticValueType::Result {
            success: Box::new(StaticValueType::Option {
                value: Box::new(StaticValueType::Scalar(syn::parse_quote!(BigInt))),
            }),
            failure: Box::new(StaticValueType::Option {
                value: Box::new(StaticValueType::Scalar(syn::parse_quote!(EcoString))),
            }),
        };

        assert_eq!(
            list_item_view_type(
                &input,
                &std::collections::BTreeMap::new(),
                &quote!(geam_core),
            )
            .to_string(),
            ":: core :: result :: Result < :: core :: option :: Option < BigInt > , :: core :: option :: Option < EcoString > >",
        );
    }
}
