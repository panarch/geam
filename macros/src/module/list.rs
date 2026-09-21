use super::custom_value::{
    CustomConstructorModel, CustomFieldValueType, CustomFields, CustomModel, custom_field_models,
};
use super::{
    GeneratedNames, GeneratedValue, InputOwnership, ListDeclaredAccess, ListDecoderModel,
    ListExternalAccess, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::{Ident, Type};

pub(super) fn generate_list_decoders(
    decoder: &ListDecoderModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    [InputOwnership::Borrowed, InputOwnership::Owned]
        .into_iter()
        .map(|flavor| generate_list_decoder(decoder, customs, custom_inputs, support, flavor))
        .collect()
}

fn generate_list_decoder(
    decoder: &ListDecoderModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: InputOwnership,
) -> TokenStream {
    let ident = list_decoder_ident(&decoder.ident, flavor);
    let item = validated_list_item_type(&decoder.value, customs, custom_inputs, support, flavor);
    let accesses = list_external_accesses(&decoder.value, customs);
    let declared = list_declared_accesses(&decoder.value, customs);
    let declared_fields = declared.iter().map(|access| {
        let field = &access.field;
        let (_, decoder) = declared_names(&access.type_);
        quote!(#field: #decoder,)
    });
    let fields = accesses.iter().map(|access| {
        let field = &access.field;
        let payload = &access.payload;
        quote!(
            #field: #support::ProviderExternalPayloadAccess<#payload>,
        )
    });
    let capabilities = super::list_capability::capabilities(&decoder.value, customs);
    let capability_names = capabilities
        .iter()
        .map(|capability| capability.names())
        .collect::<Vec<_>>();
    let declared_names = declared
        .iter()
        .map(|access| declared_names(&access.type_))
        .collect::<Vec<_>>();
    let source_parameters = super::list_capability::source_parameters(&decoder.value, customs);
    let profile = super::list_capability::has_profile(&decoder.value, customs)
        .then(|| format_ident!("__GeamProfile"));
    let source_declarations = source_parameters
        .iter()
        .map(|parameter| quote!(#parameter: #support::ProviderValue + 'static))
        .collect::<Vec<_>>();
    let parameters = profile
        .iter()
        .chain(source_parameters.iter())
        .chain(
            capability_names
                .iter()
                .flat_map(|(signature, decoder, _)| [signature, decoder])
                .chain(
                    declared_names
                        .iter()
                        .flat_map(|(item, decoder)| [item, decoder]),
                ),
        )
        .collect::<Vec<_>>();
    let generics = (!parameters.is_empty()).then(|| quote!(<#(#parameters),*>));
    let implementation_parameters = profile
        .iter()
        .map(|ident| quote!(#ident))
        .chain(source_declarations.iter().cloned())
        .chain(
            parameters
                .iter()
                .skip(source_parameters.len() + usize::from(profile.is_some()))
                .map(|parameter| quote!(#parameter)),
        )
        .collect::<Vec<_>>();
    let implementation_generics =
        (!implementation_parameters.is_empty()).then(|| quote!(<#(#implementation_parameters),*>));
    let capability_parameters = capability_names
        .iter()
        .flat_map(|(signature, decoder, _)| [quote!(#signature), quote!(#decoder)]);
    let declared_parameters = declared_names
        .iter()
        .flat_map(|(item, decoder)| [quote!(#item), quote!(#decoder)]);
    let definition_parameters = profile
        .iter()
        .map(|ident| quote!(#ident))
        .chain(source_declarations.iter().cloned())
        .chain(capability_parameters)
        .chain(declared_parameters)
        .collect::<Vec<_>>();
    let definition_generics =
        (!definition_parameters.is_empty()).then(|| quote!(<#(#definition_parameters),*>));
    let capability_fields = capability_names
        .iter()
        .map(|(_, decoder, field)| quote!(#field: #decoder,))
        .collect::<Vec<_>>();
    let signature_parameters = profile
        .iter()
        .chain(source_parameters.iter())
        .chain(
            capability_names
                .iter()
                .map(|(signature, _, _)| signature)
                .chain(declared_names.iter().map(|(item, _)| item)),
        )
        .collect::<Vec<_>>();
    let mut decoder_bounds = capabilities.iter().map(|capability| {
        let (_, decoder, _) = capability.names();
        let marker = capability.marker(support);
        quote!(#decoder: #support::ProviderTypedListItemDecoder<#marker> + ::core::clone::Clone)
    })
        .chain(declared.iter().zip(&declared_names).map(|(access, (item, decoder))| {
            let _ = access;
            quote!(#decoder: #support::ProviderTypedListItemDecoder<#item> + ::core::clone::Clone)
        })).collect::<Vec<_>>();
    if profile.is_some() {
        decoder_bounds.push(quote!(__GeamProfile: __GeamModuleProfile));
        if capabilities
            .iter()
            .any(|capability| capability.requires_work(customs))
        {
            decoder_bounds.push(quote!(__GeamProfile: #support::HostWorkProfile));
        }
    }
    let where_clause = if decoder_bounds.is_empty() {
        TokenStream::new()
    } else {
        quote!(where #(#decoder_bounds,)*)
    };
    let definition = if source_parameters.is_empty()
        && capabilities.is_empty()
        && declared.is_empty()
    {
        if accesses.is_empty() && declared.is_empty() {
            quote! { #[doc(hidden)] #[derive(Clone, Copy)] pub struct #ident; }
        } else {
            quote! { #[doc(hidden)] #[derive(Clone)] pub struct #ident { #(#fields)* #(#declared_fields)* } }
        }
    } else {
        let clone_bounds = capability_names
            .iter()
            .map(|(_, decoder, _)| quote!(#decoder: ::core::clone::Clone))
            .chain(
                declared_names
                    .iter()
                    .map(|(_, decoder)| quote!(#decoder: ::core::clone::Clone)),
            );
        let clone_fields = accesses
            .iter()
            .map(|access| &access.field)
            .chain(declared.iter().map(|access| &access.field))
            .chain(capability_names.iter().map(|(_, _, field)| field))
            .map(|field| quote!(#field: self.#field.clone(),));
        quote! {
            #[doc(hidden)]
            pub struct #ident #definition_generics {
                #(#fields)* #(#declared_fields)* #(#capability_fields)*
                __geam_signatures: ::core::marker::PhantomData<fn() -> (#(#signature_parameters,)*)>,
            }
            impl #implementation_generics ::core::clone::Clone for #ident #generics where #(#clone_bounds,)* {
                fn clone(&self) -> Self {
                    Self { #(#clone_fields)* __geam_signatures: ::core::marker::PhantomData }
                }
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
        flavor,
    );
    let statements = decoded.statements;
    let value = decoded.value;
    let view =
        list_item_view_type_with_flavor(&decoder.value, customs, custom_inputs, support, flavor);
    let host = super::signature::host_static_value_type_with(
        &decoder.value,
        customs,
        support,
        &|callback| {
            let (signature, decoder, _) = super::list_capability::names(callback);
            quote!(<#decoder as #support::ProviderTypedListItemDecoder<#support::Callback<#signature>>>::Host)
        },
        &|future| {
            let (source, decoder, _) = super::list_capability::future_names(future);
            quote!(<#decoder as #support::ProviderTypedListItemDecoder<#support::ProviderFuture<#source>>>::Host)
        },
        &|type_| {
            let (item, decoder) = self::declared_names(type_);
            quote!(<#decoder as #support::ProviderTypedListItemDecoder<#item>>::Host)
        },
    );
    let declared_marker = if let StaticValueType::Custom { index } = &decoder.value {
        customs[*index].input.as_ref().map(|input| {
            let marker = super::custom_context::list_marker_ident(&input.ident, flavor);
            quote! {
                impl #implementation_generics #support::ProviderTypedListItemDecoder<#marker<#(#source_parameters,)* Self>> for #ident #generics #where_clause {
                    type Host = #host;
                }
                impl #implementation_generics #support::ProviderListItemDecoder<#marker<#(#source_parameters,)* Self>> for #ident #generics #where_clause {
                    type View = #view;
                    fn decode(&self, value: #support::ProviderListItemValue<'_>) -> Self::View {
                        <Self as #support::ProviderListItemDecoder<#item>>::decode(self, value)
                    }
                }
            }
        })
    } else {
        None
    };
    quote! {
        #definition
        #declared_marker

        impl #implementation_generics #support::ProviderTypedListItemDecoder<#item> for #ident #generics #where_clause {
            type Host = #host;
        }

        impl #implementation_generics #support::ProviderListItemDecoder<#item> for #ident #generics #where_clause {
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
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: InputOwnership,
) -> TokenStream {
    match type_ {
        StaticValueType::List(list) => {
            let item = validated_list_item_type(
                &list.collection.value,
                customs,
                custom_inputs,
                support,
                flavor,
            );
            quote!(#support::List<#item>)
        }
        StaticValueType::Future(future) => {
            let (source, _, _) = super::list_capability::future_names(future);
            quote!(#support::ProviderFuture<#source>)
        }
        StaticValueType::Callback(callback) => {
            let (signature, _, _) = super::list_capability::names(callback);
            quote!(#support::Callback<#signature>)
        }
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            let (item, _) = declared_names(type_);
            quote!(#item)
        }
        StaticValueType::External { payload, .. } => match flavor {
            InputOwnership::Borrowed => {
                quote!(#support::ProviderExternalView<#payload>)
            }
            InputOwnership::Owned => {
                quote!(#support::ProviderOwnedExternal<#payload>)
            }
        },
        StaticValueType::Custom { index, .. } => {
            let input = custom_input_ident(&custom_inputs[index], flavor);
            super::custom_context::named_type(&input, &customs[*index].parameters)
        }
        StaticValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| {
                    validated_list_item_type(element, customs, custom_inputs, support, flavor)
                })
                .collect::<Vec<_>>();
            quote!((#(#elements,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success =
                validated_list_item_type(success, customs, custom_inputs, support, flavor);
            let failure =
                validated_list_item_type(failure, customs, custom_inputs, support, flavor);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = validated_list_item_type(value, customs, custom_inputs, support, flavor);
            quote!(::core::option::Option<#value>)
        }
    }
}

pub(super) fn list_decoder_ident(ident: &Ident, flavor: InputOwnership) -> Ident {
    match flavor {
        InputOwnership::Borrowed => format_ident!("__GeamImmediate{}", ident),
        InputOwnership::Owned => format_ident!("__GeamOwned{}", ident),
    }
}

pub(super) fn default_list_decoder_type(
    list: &super::ListType,
    customs: &[CustomModel],
    flavor: InputOwnership,
) -> TokenStream {
    default_decoder_type_for(&list.decoder, &list.collection.value, customs, flavor)
}

pub(super) fn default_decoder_type_for(
    ident: &Ident,
    value: &StaticValueType,
    customs: &[CustomModel],
    flavor: InputOwnership,
) -> TokenStream {
    let ident = list_decoder_ident(ident, flavor);
    let parameters = super::list_capability::source_parameters(value, customs);
    if parameters.is_empty() {
        quote!(#ident)
    } else {
        quote!(#ident<#(#parameters),*>)
    }
}

pub(super) fn declared_names(type_: &Type) -> (Ident, Ident) {
    let field = declared_access_field(type_);
    (
        format_ident!("__GeamItem{field}"),
        format_ident!("__GeamDecoder{field}"),
    )
}

pub(super) struct ListDecoderContext<'context> {
    pub(super) provider: &'context TokenStream,
    pub(super) call: &'context TokenStream,
    pub(super) capabilities: TokenStream,
    pub(super) constructions: &'context BTreeMap<String, Ident>,
}

pub(super) fn list_decoder_value(
    ident: &Ident,
    value: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
    context: ListDecoderContext<'_>,
) -> TokenStream {
    let ListDecoderContext {
        provider,
        call,
        capabilities: capability_fields,
        constructions,
    } = context;
    if let StaticValueType::Declared { type_, .. } = value {
        let member = match flavor {
            InputOwnership::Borrowed => quote!(ImmediateListInput),
            InputOwnership::Owned => quote!(OwnedListInput),
        };
        let key = quote!(#type_).to_string();
        let proof =
            super::custom_context::construction_proof(&key, constructions, customs, support);
        return quote!(<<#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::#member as #support::ProviderListInputCodec<__GeamProfile, #provider>>::decoder_with(#call, &#proof));
    }
    let capabilities = super::list_capability::capabilities(value, customs);
    let ident = list_decoder_ident(ident, flavor);
    let accesses = list_external_accesses(value, customs);
    let declared = list_declared_accesses(value, customs);
    let source_parameters = super::list_capability::source_parameters(value, customs);
    if source_parameters.is_empty()
        && accesses.is_empty()
        && declared.is_empty()
        && capabilities.is_empty()
    {
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
            let member = match flavor { InputOwnership::Borrowed => quote!(ImmediateListInput), InputOwnership::Owned => quote!(OwnedListInput) };
            let input = quote!(<#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::#member);
            let key = quote!(#type_).to_string();
            let proof = super::custom_context::construction_proof(&key, constructions, customs, support);
            quote!(#field: <#input as #support::ProviderListInputCodec<__GeamProfile, #provider>>::decoder_with(#call, &#proof),)
        });
        let signatures =
            (!source_parameters.is_empty() || !capabilities.is_empty() || !declared.is_empty())
                .then(|| quote!(__geam_signatures: ::core::marker::PhantomData,));
        let generics = (!source_parameters.is_empty()).then(|| {
            let profile =
                super::list_capability::has_profile(value, customs).then(|| quote!(__GeamProfile,));
            let others = (0..(capabilities.len() + declared.len()) * 2).map(|_| quote!(_));
            quote!(::<#profile #(#source_parameters,)* #(#others),*>)
        });
        quote! {
            #ident #generics {
                #capability_fields
                #signatures
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
            StaticValueType::List(list) => collect(&list.collection.value, customs, accesses),
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
            StaticValueType::Future(_)
            | StaticValueType::Callback(_)
            | StaticValueType::Scalar(_)
            | StaticValueType::External { .. } => {}
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
            StaticValueType::List(list) => collect(&list.collection.value, customs, accesses),
            StaticValueType::Future(_)
            | StaticValueType::Callback(_)
            | StaticValueType::Scalar(_)
            | StaticValueType::Declared { .. } => {}
            StaticValueType::External {
                payload,
                schema,
                store_field,
                ..
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
    flavor: InputOwnership,
) -> GeneratedValue {
    match type_ {
        StaticValueType::List(list) => {
            let decoder =
                nested_list_decoder_value(&list.decoder, &list.collection.value, customs, flavor);
            let item = validated_list_item_type(
                &list.collection.value,
                customs,
                custom_inputs,
                support,
                flavor,
            );
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(#input.into_typed_list::<#item, _>(#decoder)),
            }
        }
        StaticValueType::Future(future) => {
            let (source, decoder, field) = super::list_capability::future_names(future);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(<#decoder as #support::ProviderListItemDecoder<#support::ProviderFuture<#source>>>::decode(&self.#field, #input)),
            }
        }
        StaticValueType::Callback(callback) => {
            let (signature, decoder, field) = super::list_capability::names(callback);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(<#decoder as #support::ProviderListItemDecoder<#support::Callback<#signature>>>::decode(&self.#field, #input)),
            }
        }

        StaticValueType::Scalar(type_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_scalar::<#type_>()),
        },
        StaticValueType::Declared { type_, .. } => {
            let field = declared_access_field(type_);
            let (item, decoder) = self::declared_names(type_);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(<#decoder as #support::ProviderListItemDecoder<#item>>::decode(&self.#field, #input)),
            }
        }
        StaticValueType::External { store_field, .. } => {
            let value = match flavor {
                InputOwnership::Owned => {
                    quote!(#input.into_external(&self.#store_field))
                }
                InputOwnership::Borrowed => {
                    quote!(#input.into_external_view(&self.#store_field))
                }
            };
            GeneratedValue {
                statements: TokenStream::new(),
                value,
            }
        }
        StaticValueType::Custom { index, .. } => decode_list_custom(
            *index,
            input,
            customs,
            custom_inputs,
            support,
            names,
            flavor,
        ),
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
                    flavor,
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
                flavor,
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
                flavor,
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
                flavor,
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
    flavor: InputOwnership,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_list_item(type_, input, customs, custom_inputs, support, names, flavor)
        }
        CustomFieldValueType::List(list) => decode_list_item(
            &StaticValueType::List(list.clone()),
            input,
            customs,
            custom_inputs,
            support,
            names,
            flavor,
        ),
    }
}

fn decode_list_custom(
    custom_index: usize,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
    flavor: InputOwnership,
) -> GeneratedValue {
    let custom = &customs[custom_index];
    let input_type = match flavor {
        InputOwnership::Borrowed => {
            custom_input_ident(&custom_inputs[&custom_index], InputOwnership::Borrowed)
        }
        InputOwnership::Owned => {
            custom_input_ident(&custom_inputs[&custom_index], InputOwnership::Owned)
        }
    };
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
                flavor,
            );
            let generated_statements = generated.statements;
            let generated_value = generated.value;
            statements.extend(quote! {
                let #host = #custom_value.take_field(#field_index);
                #generated_statements
                let #decoded = #generated_value;
            });
        }
        let expression = custom_input_expression(&input_type, constructor, &decoded_names);
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
    flavor: InputOwnership,
) -> TokenStream {
    if let StaticValueType::Declared { type_, .. } = value {
        let field = declared_access_field(type_);
        return quote!(self.#field.clone());
    }
    let ident = match flavor {
        InputOwnership::Borrowed => list_decoder_ident(ident, InputOwnership::Borrowed),
        InputOwnership::Owned => list_decoder_ident(ident, InputOwnership::Owned),
    };
    let accesses = list_external_accesses(value, customs);
    let declared = list_declared_accesses(value, customs);
    let capabilities = super::list_capability::capabilities(value, customs);
    // Generic nominal list items use the declared decoder returned above.
    // The remaining static custom items have no source type parameters.
    if accesses.is_empty() && declared.is_empty() && capabilities.is_empty() {
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
        let capability_fields = capabilities.iter().map(|capability| {
            let (_, _, field) = capability.names();
            quote!(#field: self.#field.clone(),)
        });
        let signatures = (!capabilities.is_empty() || !declared.is_empty())
            .then(|| quote!(__geam_signatures: ::core::marker::PhantomData,));
        quote! {
            #ident {
                #(#capability_fields)*
                #signatures
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

fn list_item_view_type_with_flavor(
    type_: &StaticValueType,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: InputOwnership,
) -> TokenStream {
    match type_ {
        StaticValueType::List(list) => {
            let item = validated_list_item_type(
                &list.collection.value,
                customs,
                custom_inputs,
                support,
                flavor,
            );
            let decoder = super::list_capability::nested_decoder_type(list, customs, flavor);
            quote!(#support::List<#item, #support::ProviderListContext<<#decoder as #support::ProviderTypedListItemDecoder<#item>>::Host, #decoder>>)
        }
        StaticValueType::Future(future) => {
            let (source, decoder, _) = super::list_capability::future_names(future);
            quote!(<#decoder as #support::ProviderListItemDecoder<#support::ProviderFuture<#source>>>::View)
        }
        StaticValueType::Callback(callback) => {
            let (signature, decoder, _) = super::list_capability::names(callback);
            quote!(<#decoder as #support::ProviderListItemDecoder<#support::Callback<#signature>>>::View)
        }
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            let (item, decoder) = self::declared_names(type_);
            quote!(<#decoder as #support::ProviderListItemDecoder<#item>>::View)
        }
        StaticValueType::External { payload, .. } => match flavor {
            InputOwnership::Borrowed => {
                quote!(#support::ProviderExternalView<#payload>)
            }
            InputOwnership::Owned => {
                quote!(#support::ProviderOwnedExternal<#payload>)
            }
        },
        StaticValueType::Custom { index, .. } => {
            let input = custom_input_ident(&custom_inputs[index], flavor);
            if customs[*index]
                .constructors
                .iter()
                .all(|constructor| custom_field_models(&constructor.fields).is_empty())
            {
                return super::custom_context::named_type(&input, &customs[*index].parameters);
            }
            let fields = customs[*index]
                .constructors
                .iter()
                .flat_map(|constructor| custom_field_models(&constructor.fields))
                .map(|field| match &field.value {
                    CustomFieldValueType::Value(value) => list_item_view_type_with_flavor(
                        value,
                        customs,
                        custom_inputs,
                        support,
                        flavor,
                    ),
                    CustomFieldValueType::List(list) => list_item_view_type_with_flavor(
                        &StaticValueType::List(list.clone()),
                        customs,
                        custom_inputs,
                        support,
                        flavor,
                    ),
                });
            let parameters = &customs[*index].parameters;
            let profile = super::custom_context::is_contextual(*index, customs)
                .then(|| quote!(__GeamProfile,));
            quote!(#input<#(#parameters,)* #(#fields,)* #profile>)
        }
        StaticValueType::Tuple(elements) => {
            let types = elements
                .iter()
                .map(|element| {
                    list_item_view_type_with_flavor(
                        element,
                        customs,
                        custom_inputs,
                        support,
                        flavor,
                    )
                })
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success =
                list_item_view_type_with_flavor(success, customs, custom_inputs, support, flavor);
            let failure =
                list_item_view_type_with_flavor(failure, customs, custom_inputs, support, flavor);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value =
                list_item_view_type_with_flavor(value, customs, custom_inputs, support, flavor);
            quote!(::core::option::Option<#value>)
        }
    }
}

pub(super) fn custom_input_ident(input: &Ident, flavor: InputOwnership) -> Ident {
    match flavor {
        InputOwnership::Borrowed => format_ident!("__GeamImmediate{}", input),
        InputOwnership::Owned => format_ident!("__GeamOwned{}", input),
    }
}

#[cfg(test)]
mod tests {
    use super::list_item_view_type_with_flavor;
    use crate::module::{InputOwnership, StaticValueType};
    use quote::quote;

    #[test]
    fn item_views_preserve_recursive_result_and_option_direction() {
        let input = StaticValueType::Result {
            success: Box::new(StaticValueType::Option {
                value: Box::new(StaticValueType::Scalar(syn::parse_quote!(BigInt))),
            }),
            failure: Box::new(StaticValueType::Option {
                value: Box::new(StaticValueType::Scalar(syn::parse_quote!(StringValue))),
            }),
        };

        assert_eq!(
            list_item_view_type_with_flavor(
                &input,
                &[],
                &std::collections::BTreeMap::new(),
                &quote!(geam_core),
                InputOwnership::Borrowed,
            )
            .to_string(),
            ":: core :: result :: Result < :: core :: option :: Option < BigInt > , :: core :: option :: Option < StringValue > >",
        );
    }
}
