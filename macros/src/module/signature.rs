use super::custom_value::{CustomFieldValueType, CustomModel};
use super::list::list_decoder_ident;
use super::{
    CallbackType, DeclaredInput, FunctionArgumentType, FunctionInputType, FunctionInputValueType,
    FunctionOutputLeafType, FunctionOutputValueType, FunctionReturnType,
    FunctionRootOutputValueType, GeneratedNames, GeneratedValue, GenericExternalStorage,
    GenericExternalType, GenericHostType, GenericInputSource, GenericValueType, InputOwnership,
    ListType, ProviderValueType, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, PathArguments, Type};

fn static_list_item_type(
    type_: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => match flavor {
            InputOwnership::Borrowed => {
                quote!(<#type_ as #support::ProviderValueForms>::ImmediateListInput)
            }
            InputOwnership::Owned => {
                quote!(<#type_ as #support::ProviderValueForms>::OwnedListInput)
            }
        },
        StaticValueType::External { payload, .. } => match flavor {
            InputOwnership::Borrowed => {
                quote!(#support::ProviderExternalView<#payload>)
            }
            InputOwnership::Owned => {
                quote!(#support::ProviderOwnedExternal<#payload>)
            }
        },
        StaticValueType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            match flavor {
                InputOwnership::Borrowed => {
                    quote!(<#type_ as #support::ProviderValueForms>::ImmediateListInput)
                }
                InputOwnership::Owned => {
                    quote!(<#type_ as #support::ProviderValueForms>::OwnedListInput)
                }
            }
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| static_list_item_type(element, customs, support, flavor))
                .collect::<Vec<_>>();
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = static_list_item_type(success, customs, support, flavor);
            let failure = static_list_item_type(failure, customs, support, flavor);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = static_list_item_type(value, customs, support, flavor);
            quote!(::core::option::Option<#value>)
        }
    }
}

pub(super) fn list_signature_type(
    list: &ListType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Type {
    let item = static_list_item_type(&list.collection.value, customs, support, flavor);
    let host_item = host_static_value_type(&list.collection.value, customs, support);
    let decoder = list_decoder_ident(&list.decoder, flavor);
    syn::parse_quote! {
        #support::List<
            #item,
            #support::ProviderListContext<#host_item, #decoder>,
        >
    }
}

pub(super) fn provider_input_signature_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Type {
    match type_ {
        ProviderValueType::Scalar(type_) => type_.clone(),
        ProviderValueType::Generic(value) => generic_value_signature_type(value, customs, support),
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::Owned,
            ..
        } => match flavor {
            InputOwnership::Borrowed => syn::parse_quote!(
                <#type_ as #support::ProviderValueForms>::ImmediateInput
            ),
            InputOwnership::Owned => syn::parse_quote!(
                <#type_ as #support::ProviderValueForms>::OwnedInput
            ),
        },
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::BorrowedExternal,
            ..
        } => match flavor {
            InputOwnership::Borrowed => syn::parse_quote!(
                <#type_ as #support::ProviderValueForms>::ImmediateInput
            ),
            InputOwnership::Owned => syn::parse_quote!(
                <#type_ as #support::ProviderValueForms>::OwnedInput
            ),
        },
        ProviderValueType::External { payload, .. } => match flavor {
            InputOwnership::Borrowed => {
                syn::parse_quote!(#support::ProviderExternalView<#payload>)
            }
            InputOwnership::Owned => {
                syn::parse_quote!(#support::ProviderOwnedExternal<#payload>)
            }
        },
        ProviderValueType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            match flavor {
                InputOwnership::Borrowed => {
                    syn::parse_quote!(<#type_ as #support::ProviderValueForms>::ImmediateInput)
                }
                InputOwnership::Owned => {
                    syn::parse_quote!(<#type_ as #support::ProviderValueForms>::OwnedInput)
                }
            }
        }
        ProviderValueType::List(list) => list_signature_type(list, customs, support, flavor),
        ProviderValueType::Tuple(elements) => {
            let types = elements
                .iter()
                .map(|element| provider_input_signature_type(element, customs, support, flavor))
                .collect::<Vec<_>>();
            syn::parse_quote!((#(#types,)*))
        }
        ProviderValueType::Result { success, failure } => {
            let success = provider_input_signature_type(success, customs, support, flavor);
            let failure = provider_input_signature_type(failure, customs, support, flavor);
            syn::parse_quote!(::core::result::Result<#success, #failure>)
        }
        ProviderValueType::Option { value } => {
            let value = provider_input_signature_type(value, customs, support, flavor);
            syn::parse_quote!(::core::option::Option<#value>)
        }
    }
}

pub(super) fn generic_value_signature_type(
    value: &GenericValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    let source = &value.source;
    let host = generic_host_type(&value.host, customs, support);
    let mut path = value.path.clone();
    for segment in path.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote! {
            <#source, #support::ProviderValueContext<#host>>
        });
    }
    Type::Path(path)
}

pub(super) fn generic_external_host_type(
    external: &GenericExternalType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let arguments = external
        .arguments
        .iter()
        .map(|argument| generic_host_type(&argument.host, customs, support))
        .collect::<Vec<_>>();
    let arguments = host_type_token_sequence(&arguments, support);
    let schema = &external.schema;
    quote!(#support::HostExternalType<#schema, #arguments>)
}

pub(super) fn generate_generic_external_payload(
    external: &GenericExternalType,
    input: TokenStream,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let output = &external.output;
    let payload = names.next("external_payload");
    match &external.storage {
        GenericExternalStorage::StoredFields {
            payload: payload_type,

            fields,
            ..
        } => {
            let fields = fields
                .iter()
                .map(|field| {
                    let ident = &field.ident;
                    let value = names.next("stored_field");
                    (ident, value)
                })
                .collect::<Vec<_>>();
            let patterns = fields.iter().map(|(ident, value)| quote!(#ident: #value));
            let (payload_type, item_type, values) = (
                quote!(#payload_type),
                quote!(#support::ProviderOwnedExternal<#payload_type>),
                fields
                    .iter()
                    .map(|(ident, value)| quote!(#ident: #value.into_retained()))
                    .collect::<Vec<_>>(),
            );
            GeneratedValue {
                statements: quote! {
                    let #output { #(#patterns,)* } = #input;
                    let #payload = ::core::result::Result::<
                        #payload_type,
                        #item_type,
                    >::Ok(
                        #payload_type { #(#values,)* },
                    );
                },
                value: quote!(#payload),
            }
        }
        GenericExternalStorage::ManualPayload { .. } => {
            let context = names.next("external_output");
            GeneratedValue {
                statements: quote! {
                    let #output { __geam_context: #context, .. } = #input;
                    let #payload = #context.into_value();
                },
                value: quote!(#payload),
            }
        }
    }
}

pub(super) fn generic_external_input_signature_type(
    external: &GenericExternalType,
    customs: &[CustomModel],
    support: &TokenStream,
    source: GenericInputSource,
    flavor: InputOwnership,
) -> Type {
    let mut arguments = match source {
        GenericInputSource::Declared => external.source_arguments.clone(),
        GenericInputSource::Instantiated => external
            .arguments
            .iter()
            .map(|argument| argument.instantiated.clone())
            .collect(),
    };
    let host_arguments = external
        .arguments
        .iter()
        .map(|argument| generic_host_type(&argument.host, customs, support))
        .collect::<Vec<_>>();
    let host_arguments = host_type_token_sequence(&host_arguments, support);
    let payload = match &external.storage {
        GenericExternalStorage::StoredFields { payload, .. } => quote!(#payload),
        GenericExternalStorage::ManualPayload { payload, .. } => quote!(#payload),
    };
    let context = match flavor {
        InputOwnership::Borrowed => quote! {
            #support::ProviderExternalInputContext<#payload, #host_arguments>
        },
        InputOwnership::Owned => quote! {
            #support::ProviderOwnedExternalInputContext<#payload, #host_arguments>
        },
    };
    arguments.push(syn::parse_quote!(#context));
    let input = &external.input;
    syn::parse_quote!(#input<#(#arguments),*>)
}

pub(super) fn generic_external_output_signature_type(
    external: &GenericExternalType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    let mut arguments = external.source_arguments.clone();
    match &external.storage {
        GenericExternalStorage::StoredFields { owner, fields, .. } => {
            for field in fields {
                let index = &field.index;
                let host = generic_host_type(
                    &external.arguments[field.parameter_index].host,
                    customs,
                    support,
                );
                arguments.push(syn::parse_quote! {
                    #support::ProviderStoredOutput<#owner, #index, #host>
                });
            }
        }
        GenericExternalStorage::ManualPayload { payload, .. } => {
            arguments.push(syn::parse_quote! {
                #support::ProviderExternalOutput<#payload>
            });
        }
    }
    let output = &external.output;
    syn::parse_quote!(#output<#(#arguments),*>)
}

pub(super) fn callback_signature_type(
    callback: &CallbackType,
    generics: &[Ident],
    profile: &TokenStream,
    support: &TokenStream,
) -> Type {
    let signature = &callback.signature;
    let codec = callback_codec_type(
        &callback.codec,
        generics.iter().map(|ident| quote!(#ident)).collect(),
    );
    let context = quote!(#support::ProviderOwnedCallbackContext<#profile, __GeamProvider, #codec>);
    let mut path = callback.path.clone();
    for segment in path.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote! {
            <#signature, #context>
        });
    }
    Type::Path(path)
}

pub(super) fn callback_output_signature_type(
    type_: &FunctionReturnType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Type {
    match type_ {
        FunctionReturnType::Value(value) => {
            let value = function_output_from_root(value);
            function_output_rust_type(&value, customs, support)
        }
        FunctionReturnType::Generic(value) => generic_value_signature_type(value, customs, support),
        FunctionReturnType::External(external) => {
            generic_external_output_signature_type(external, customs, support)
        }
        FunctionReturnType::List(list) => list_signature_type(list, customs, support, flavor),
    }
}

pub(super) fn callback_codec_type(codec: &Ident, arguments: Vec<TokenStream>) -> TokenStream {
    if arguments.is_empty() {
        quote!(#codec)
    } else {
        quote!(#codec<#(#arguments),*>)
    }
}

pub(super) fn callback_input_signature_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    profile: &TokenStream,
) -> Type {
    match type_ {
        FunctionInputType::Future(value) => {
            future_input_signature_type(value, customs, support, profile)
        }
        FunctionInputType::Value(value) => {
            let value = provider_value_from_input_root(value);
            provider_input_signature_type(&value, customs, support, flavor)
        }
        FunctionInputType::Generic(value) => generic_value_signature_type(value, customs, support),
        FunctionInputType::External(external) => generic_external_input_signature_type(
            external,
            customs,
            support,
            GenericInputSource::Declared,
            flavor,
        ),
        FunctionInputType::List(list) => list_signature_type(list, customs, support, flavor),
    }
}

pub(super) fn future_input_signature_type(
    input: &super::FutureInputType,
    customs: &[CustomModel],
    support: &TokenStream,
    profile: &TokenStream,
) -> Type {
    let host = host_input_type(&input.value, customs, support, profile);
    let source = &input.source;
    let returned = callback_input_signature_type(
        &input.value,
        customs,
        support,
        InputOwnership::Owned,
        profile,
    );
    let context =
        quote!(#support::ProviderFutureValueContext<#profile, __GeamProvider, #host, #returned>);
    let mut path = input.path.clone();
    for segment in path.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote!(<#source, #context>));
    }
    Type::Path(path)
}

pub(super) fn function_output_rust_type(
    type_: &FunctionOutputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    match type_ {
        FunctionOutputValueType::Value(value) => match value.as_ref() {
            FunctionOutputLeafType::Scalar(type_)
            | FunctionOutputLeafType::Declared { type_, .. } => type_.clone(),
            FunctionOutputLeafType::External { payload, .. } => syn::parse_quote!(#payload),
            FunctionOutputLeafType::Custom { rust, .. } => rust.clone(),
        },
        FunctionOutputValueType::Generic(value) => {
            generic_value_signature_type(value, customs, support)
        }
        FunctionOutputValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| function_output_rust_type(element, customs, support))
                .collect::<Vec<_>>();
            syn::parse_quote!((#(#elements,)*))
        }
        FunctionOutputValueType::Result { success, failure } => {
            let success = function_output_rust_type(success, customs, support);
            let failure = function_output_rust_type(failure, customs, support);
            syn::parse_quote!(::core::result::Result<#success, #failure>)
        }
        FunctionOutputValueType::Option { value } => {
            let value = function_output_rust_type(value, customs, support);
            syn::parse_quote!(::core::option::Option<#value>)
        }
        FunctionOutputValueType::Vec(collection) => {
            let value = function_output_rust_type(&collection.value, customs, support);
            syn::parse_quote!(::std::vec::Vec<#value>)
        }
    }
}

pub(super) fn instantiated_generic_source_type(value: &GenericValueType) -> Type {
    value.instantiated.clone()
}
fn provider_value_from_output_leaf(type_: &FunctionOutputLeafType) -> ProviderValueType {
    match type_ {
        FunctionOutputLeafType::Scalar(type_) => ProviderValueType::Scalar(type_.clone()),
        FunctionOutputLeafType::Declared { type_, input } => ProviderValueType::Declared {
            type_: type_.clone(),
            input: *input,
        },
        FunctionOutputLeafType::External { payload, schema } => ProviderValueType::External {
            payload: payload.clone(),
            schema: schema.clone(),
        },
        FunctionOutputLeafType::Custom { index, rust } => ProviderValueType::Custom {
            index: *index,
            rust: rust.clone(),
        },
    }
}

pub(super) fn provider_value_from_input_root(type_: &FunctionInputValueType) -> ProviderValueType {
    match type_ {
        FunctionInputValueType::Scalar(type_) => ProviderValueType::Scalar(type_.clone()),
        FunctionInputValueType::Declared { type_, input } => ProviderValueType::Declared {
            type_: type_.clone(),
            input: *input,
        },
        FunctionInputValueType::External { payload, schema } => ProviderValueType::External {
            payload: payload.clone(),
            schema: schema.clone(),
        },
        FunctionInputValueType::Custom { index, rust } => ProviderValueType::Custom {
            index: *index,
            rust: rust.clone(),
        },
        FunctionInputValueType::Tuple(elements) => ProviderValueType::Tuple(elements.clone()),
        FunctionInputValueType::Result { success, failure } => ProviderValueType::Result {
            success: success.clone(),
            failure: failure.clone(),
        },
        FunctionInputValueType::Option { value } => ProviderValueType::Option {
            value: value.clone(),
        },
    }
}

pub(super) fn function_output_from_root(
    type_: &FunctionRootOutputValueType,
) -> FunctionOutputValueType {
    match type_ {
        FunctionRootOutputValueType::Value(value) => FunctionOutputValueType::Value(value.clone()),
        FunctionRootOutputValueType::Tuple(elements) => {
            FunctionOutputValueType::Tuple(elements.clone())
        }
        FunctionRootOutputValueType::Result { success, failure } => {
            FunctionOutputValueType::Result {
                success: success.clone(),
                failure: failure.clone(),
            }
        }
        FunctionRootOutputValueType::Option { value } => FunctionOutputValueType::Option {
            value: value.clone(),
        },
        FunctionRootOutputValueType::Vec(collection) => {
            FunctionOutputValueType::Vec(collection.clone())
        }
    }
}

pub(super) fn host_argument_type(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionArgumentType::Input(type_) => {
            host_input_type(type_, customs, support, &quote!(Profile))
        }
        FunctionArgumentType::Callback(callback) => callback_host_type(callback, customs, support),
    }
}

pub(super) fn host_input_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
    profile: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionInputType::Future(value) => {
            let value = host_input_type(&value.value, customs, support, profile);
            quote!(#support::HostFutureType<#value, #support::HostWorkSchema<#profile>>)
        }
        FunctionInputType::Value(type_) => {
            let type_ = provider_value_from_input_root(type_);
            host_value_type(&type_, customs, support)
        }
        FunctionInputType::Generic(value) => generic_host_type(&value.host, customs, support),
        FunctionInputType::External(external) => {
            generic_external_host_type(external, customs, support)
        }
        FunctionInputType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

pub(super) fn host_return_type(
    type_: &FunctionReturnType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionReturnType::Value(type_) => {
            let type_ = function_output_from_root(type_);
            function_output_host_type(&type_, customs, support)
        }
        FunctionReturnType::Generic(value) => generic_host_type(&value.host, customs, support),
        FunctionReturnType::External(external) => {
            generic_external_host_type(external, customs, support)
        }
        FunctionReturnType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

pub(super) fn function_output_host_type(
    type_: &FunctionOutputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionOutputValueType::Value(value) => {
            host_value_type(&provider_value_from_output_leaf(value), customs, support)
        }
        FunctionOutputValueType::Generic(value) => generic_host_type(&value.host, customs, support),
        FunctionOutputValueType::Tuple(elements) => {
            let elements = function_output_host_type_sequence(elements, customs, support);
            quote!(#support::HostTupleType<#elements>)
        }
        FunctionOutputValueType::Result { success, failure } => {
            let success = function_output_host_type(success, customs, support);
            let failure = function_output_host_type(failure, customs, support);
            quote!(#support::ProviderResult<#success, #failure>)
        }
        FunctionOutputValueType::Option { value } => {
            let value = function_output_host_type(value, customs, support);
            quote!(#support::ProviderOption<#value>)
        }
        FunctionOutputValueType::Vec(collection) => {
            let item = function_output_host_type(&collection.value, customs, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

fn function_output_host_type_sequence(
    elements: &[FunctionOutputValueType],
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    elements
        .iter()
        .rev()
        .fold(quote!(#support::HostTypeListEnd), |tail, element| {
            let element = function_output_host_type(element, customs, support);
            quote!(#support::HostTypeList<#element, #tail>)
        })
}

pub(super) fn host_value_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::Generic(value) => generic_host_type(&value.host, customs, support),
        ProviderValueType::Declared { type_, .. } => {
            quote!(<#type_ as #support::ProviderValue>::Host)
        }
        ProviderValueType::External { schema, .. } => {
            quote!(#support::HostExternalType<#schema>)
        }
        ProviderValueType::Custom { index, .. } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustomType<#schema>)
        }
        ProviderValueType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostListType<#item>)
        }
        ProviderValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, customs, support);
            quote!(#support::HostTupleType<#elements>)
        }
        ProviderValueType::Result { success, failure } => {
            let success = host_value_type(success, customs, support);
            let failure = host_value_type(failure, customs, support);
            quote!(#support::ProviderResult<#success, #failure>)
        }
        ProviderValueType::Option { value } => {
            let value = host_value_type(value, customs, support);
            quote!(#support::ProviderOption<#value>)
        }
    }
}

pub(super) fn host_static_value_type(
    type_: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            quote!(<#type_ as #support::ProviderValue>::Host)
        }
        StaticValueType::External { schema, .. } => {
            quote!(#support::HostExternalType<#schema>)
        }
        StaticValueType::Custom { index, .. } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustomType<#schema>)
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| host_static_value_type(element, customs, support))
                .collect::<Vec<_>>();
            let elements = host_type_token_sequence(&elements, support);
            quote!(#support::HostTupleType<#elements>)
        }
        StaticValueType::Result { success, failure } => {
            let success = host_static_value_type(success, customs, support);
            let failure = host_static_value_type(failure, customs, support);
            quote!(#support::ProviderResult<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = host_static_value_type(value, customs, support);
            quote!(#support::ProviderOption<#value>)
        }
    }
}

pub(super) fn generic_host_type(
    type_: &GenericHostType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        GenericHostType::Parameter { index } => {
            quote!(#support::HostTypeParameter<#index>)
        }
        GenericHostType::Scalar(type_) => quote!(#type_),
        GenericHostType::Declared(type_) => {
            quote!(<#type_ as #support::ProviderValue>::Host)
        }
        GenericHostType::External { schema } => {
            quote!(#support::HostExternalType<#schema>)
        }
        GenericHostType::Custom { index } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustomType<#schema>)
        }
        GenericHostType::Tuple(elements) => {
            let mut host_elements = Vec::with_capacity(elements.len());
            for element in elements {
                host_elements.push(generic_host_type(element, customs, support));
            }
            let elements = host_type_token_sequence(&host_elements, support);
            quote!(#support::HostTupleType<#elements>)
        }
        GenericHostType::List(item) => {
            let item = generic_host_type(item, customs, support);
            quote!(#support::HostListType<#item>)
        }
        GenericHostType::Result {
            success, failure, ..
        } => {
            let success = generic_host_type(success, customs, support);
            let failure = generic_host_type(failure, customs, support);
            quote!(#support::ProviderResult<#success, #failure>)
        }
        GenericHostType::Option(value) => {
            let value = generic_host_type(value, customs, support);
            quote!(#support::ProviderOption<#value>)
        }
        GenericHostType::Function { arguments, return_ } => {
            let mut host_arguments = Vec::with_capacity(arguments.len());
            for argument in arguments {
                host_arguments.push(generic_host_type(argument, customs, support));
            }
            let arguments = host_type_token_sequence(&host_arguments, support);
            let return_ = generic_host_type(return_, customs, support);
            quote!(#support::HostOpaqueFunctionType<#arguments, #return_>)
        }
    }
}

pub(super) fn host_custom_field_type(
    type_: &CustomFieldValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        CustomFieldValueType::Value(type_) => host_static_value_type(type_, customs, support),
        CustomFieldValueType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

pub(super) fn wrapper_argument_type(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    wrapper_argument_type_with_lifetime(type_, customs, support, &quote!('__geam_call))
}

pub(super) fn wrapper_argument_type_with_lifetime(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
    lifetime: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionArgumentType::Input(type_) => wrapper_input_type(type_, customs, support, lifetime),
        FunctionArgumentType::Callback(callback) => {
            let arguments = callback_host_arguments(callback, customs, support);
            let return_ = host_input_type(&callback.return_, customs, support, &quote!(Profile));
            quote!(#support::HostCallable<#lifetime, #arguments, #return_>)
        }
    }
}

fn wrapper_input_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
    lifetime: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionInputType::Future(value) => {
            let value = host_input_type(&value.value, customs, support, &quote!(Profile));
            quote!(#support::HostExternal<#lifetime, #support::HostFutureType<#value, #support::HostWorkSchema<Profile>>>)
        }
        FunctionInputType::Value(type_) => {
            wrapper_input_value_type(type_, customs, support, lifetime)
        }
        FunctionInputType::Generic(value) => {
            let host = generic_host_type(&value.host, customs, support);
            quote!(<#host as #support::HostType>::Value<#lifetime>)
        }
        FunctionInputType::External(external) => {
            let host = generic_external_host_type(external, customs, support);
            quote!(#support::HostExternal<#lifetime, #host>)
        }
        FunctionInputType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostList<#lifetime, #item>)
        }
    }
}

fn callback_host_type(
    callback: &CallbackType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let arguments = callback_host_arguments(callback, customs, support);
    let return_ = host_input_type(&callback.return_, customs, support, &quote!(Profile));
    quote!(#support::HostFunctionType<#arguments, #return_>)
}

pub(super) fn callback_host_arguments(
    callback: &CallbackType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    callback
        .arguments
        .iter()
        .rev()
        .fold(quote!(#support::HostTypeListEnd), |tail, argument| {
            let argument = host_return_type(argument, customs, support);
            quote!(#support::HostTypeList<#argument, #tail>)
        })
}

fn wrapper_input_value_type(
    type_: &FunctionInputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    lifetime: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionInputValueType::Scalar(type_) => quote!(#type_),
        FunctionInputValueType::Declared { type_, .. } => {
            quote!(
                <<#type_ as #support::ProviderValue>::Host as
                    #support::HostType>::Value<#lifetime>
            )
        }
        FunctionInputValueType::External { schema, .. } => {
            quote!(#support::HostExternal<#lifetime, #support::HostExternalType<#schema>>)
        }
        FunctionInputValueType::Custom { index, .. } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustom<#lifetime, #support::HostCustomType<#schema>>)
        }
        FunctionInputValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, customs, support);
            quote!(#support::HostTuple<#lifetime, #elements>)
        }
        FunctionInputValueType::Result { success, failure } => {
            let success = host_value_type(success, customs, support);
            let failure = host_value_type(failure, customs, support);
            quote!(#support::HostCustom<#lifetime, #support::ProviderResult<#success, #failure>>)
        }
        FunctionInputValueType::Option { value } => {
            let value = host_value_type(value, customs, support);
            quote!(#support::HostCustom<#lifetime, #support::ProviderOption<#value>>)
        }
    }
}

fn host_value_type_sequence(
    elements: &[ProviderValueType],
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let elements = elements
        .iter()
        .map(|element| host_value_type(element, customs, support))
        .collect::<Vec<_>>();
    host_type_token_sequence(&elements, support)
}

pub(super) fn host_type_token_sequence(
    elements: &[TokenStream],
    support: &TokenStream,
) -> TokenStream {
    elements.iter().rev().fold(
        quote!(#support::HostTypeListEnd),
        |tail, head| quote!(#support::HostTypeList<#head, #tail>),
    )
}
