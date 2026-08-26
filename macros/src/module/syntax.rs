use super::custom_value::{
    CustomModel, custom_input_model, custom_output_type, custom_output_type_with_index,
};
use super::list::validate_list_return;
use super::list_model::register_list_decoder;
use super::signature::{
    callback_signature_type, function_output_from_root, function_output_rust_type,
    generic_external_input_signature_type, generic_external_output_signature_type,
    generic_value_signature_type, host_return_type, list_signature_type,
    provider_input_signature_type, provider_value_from_input_root,
};
use super::type_syntax::{
    collection_item, collection_item_with_path, external_type, host_result_value,
    is_advanced_external, is_collection, is_declared_provider_type, is_non_empty_tuple,
    is_qualified_type_path, is_type_application_named, source_wrapper,
};
use super::{
    CallAccess, CallbackType, ClassifiedGenericHostType, CollectionType, DeclaredInput,
    ExternalArguments, ExternalModel, ExternalSemantics, FunctionArgumentType, FunctionArguments,
    FunctionInputType, FunctionInputValueType, FunctionModel, FunctionOutputCollectionType,
    FunctionOutputLeafType, FunctionOutputValueType, FunctionReturnType,
    FunctionRootOutputValueType, GenericExternalModel, GenericExternalStorage, GenericExternalType,
    GenericHostType, GenericInputSource, GenericParameterScope, GenericValueType, ListDecoderModel,
    ListType, ModuleArguments, ModuleProfile, PartialExternalArguments, PartialModuleArguments,
    ProviderValueType, SourceWrapper, StaticValueType, StoredExternalField, is_marker,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Fields, FnArg, GenericArgument, GenericParam, Ident, ItemFn, ItemStruct, Meta, Path,
    PathArguments, ReturnType, Token, Type, TypePath,
};

impl Parse for ModuleArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut partial = PartialModuleArguments::default();
        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            match field.to_string().as_str() {
                "path" => {
                    if partial.path.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("duplicate module argument `{field}`"),
                        ));
                    }
                }
                "crate_path" => {
                    if partial.crate_path.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("duplicate module argument `{field}`"),
                        ));
                    }
                }
                "profile" => {
                    if partial.profile.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("duplicate module argument `{field}`"),
                        ));
                    }
                }
                "component" => {
                    if partial.component.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("duplicate module argument `{field}`"),
                        ));
                    }
                }
                "stores" => {
                    if partial.stores.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("duplicate module argument `{field}`"),
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        format!("unknown module argument `{field}`"),
                    ));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        let Some(path) = partial.path else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing required module argument `path`",
            ));
        };
        let profile = match (partial.profile, partial.component) {
            (None, None) => ModuleProfile::Component,
            (Some(bound), Some(component)) => {
                if partial.crate_path.is_none() {
                    return Err(syn::Error::new_spanned(
                        bound,
                        "module `profile` and `component` require explicit `crate_path`",
                    ));
                }
                ModuleProfile::Explicit {
                    bound,
                    component: Box::new(component),
                }
            }
            (Some(bound), None) => {
                return Err(syn::Error::new_spanned(
                    bound,
                    "module `profile` requires `component`",
                ));
            }
            (None, Some(component)) => {
                return Err(syn::Error::new_spanned(
                    component,
                    "module `component` requires `profile`",
                ));
            }
        };
        if matches!(profile, ModuleProfile::Component)
            && let Some(stores) = &partial.stores
        {
            return Err(syn::Error::new_spanned(
                stores,
                "module `stores` requires `profile` and `component`",
            ));
        }
        Ok(Self {
            path,
            crate_path: partial.crate_path,
            profile,
            stores: partial.stores,
        })
    }
}

impl Parse for FunctionArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut arguments = Self::default();
        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            match field.to_string().as_str() {
                "profile" => {
                    input.parse::<Token![=]>()?;
                    if arguments.profile.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate function argument `profile`",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        format!("unknown function argument `{field}`"),
                    ));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(arguments)
    }
}

impl Parse for ExternalArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut partial = PartialExternalArguments::default();
        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            match field.to_string().as_str() {
                "name" => {
                    input.parse::<Token![=]>()?;
                    if partial.name.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `name`",
                        ));
                    }
                }
                "manual" => {
                    if partial.manual.replace(field.clone()).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `manual`",
                        ));
                    }
                    if !input.is_empty() && !input.peek(Token![,]) {
                        return Err(syn::Error::new(
                            field.span(),
                            "external argument `manual` does not accept a value",
                        ));
                    }
                }
                "retained" => {
                    if partial.retained.replace(field.clone()).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `retained`",
                        ));
                    }
                    if !input.is_empty() && !input.peek(Token![,]) {
                        return Err(syn::Error::new(
                            field.span(),
                            "external argument `retained` does not accept a value",
                        ));
                    }
                }
                "parameters" => {
                    input.parse::<Token![=]>()?;
                    let content;
                    syn::bracketed!(content in input);
                    let parameters = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect::<Vec<_>>();
                    if partial.parameters.replace(parameters).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `parameters`",
                        ));
                    }
                }
                "input" => {
                    input.parse::<Token![=]>()?;
                    if partial.input.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `input`",
                        ));
                    }
                }
                "payload" => {
                    input.parse::<Token![=]>()?;
                    if partial.payload.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new(
                            field.span(),
                            "duplicate external argument `payload`",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        format!("unknown external argument `{field}`"),
                    ));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        let Some(name) = partial.name else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing required external argument `name`",
            ));
        };
        if let (Some(_), Some(retained)) = (&partial.manual, &partial.retained) {
            return Err(syn::Error::new(
                retained.span(),
                "external arguments `manual` and `retained` cannot be combined",
            ));
        }
        Ok(Self {
            name,
            manual: partial.manual.is_some(),
            retained: partial.retained.is_some(),
            parameters: partial.parameters.unwrap_or_default(),
            input: partial.input,
            payload: partial.payload,
        })
    }
}

fn take_marker(attributes: &mut Vec<Attribute>, name: &str) -> syn::Result<bool> {
    let mut retained = Vec::with_capacity(attributes.len());
    let mut found = false;
    for attribute in std::mem::take(attributes) {
        let is_marker = attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == name);
        if !is_marker {
            retained.push(attribute);
            continue;
        }
        if !matches!(attribute.meta, Meta::Path(_)) {
            return Err(syn::Error::new_spanned(
                attribute,
                format!("`#[geam::{name}]` does not accept arguments"),
            ));
        }
        if found {
            return Err(syn::Error::new_spanned(
                attribute,
                format!("duplicate `#[geam::{name}]` attribute"),
            ));
        }
        found = true;
    }
    *attributes = retained;
    Ok(found)
}

pub(super) fn take_function_marker(
    attributes: &mut Vec<Attribute>,
) -> syn::Result<Option<FunctionArguments>> {
    let mut retained = Vec::with_capacity(attributes.len());
    let mut found = None;
    for attribute in std::mem::take(attributes) {
        if !is_marker(&attribute, "function") {
            retained.push(attribute);
            continue;
        }
        if found.is_some() {
            return Err(syn::Error::new_spanned(
                attribute,
                "duplicate `#[geam::function]` attribute",
            ));
        }
        let arguments = match &attribute.meta {
            Meta::Path(_) => FunctionArguments::default(),
            Meta::List(_) => attribute.parse_args::<FunctionArguments>()?,
            Meta::NameValue(_) => {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "`#[geam::function]` accepts only `profile = Name`",
                ));
            }
        };
        found = Some(arguments);
    }
    *attributes = retained;
    Ok(found)
}

pub(super) fn take_external_marker(
    attributes: &mut Vec<Attribute>,
) -> syn::Result<Option<ExternalArguments>> {
    let mut retained = Vec::with_capacity(attributes.len());
    let mut found = None;
    for attribute in std::mem::take(attributes) {
        if !is_marker(&attribute, "external") {
            retained.push(attribute);
            continue;
        }
        if found.is_some() {
            return Err(syn::Error::new_spanned(
                attribute,
                "duplicate `#[geam::external]` attribute",
            ));
        }
        let arguments = match &attribute.meta {
            Meta::List(_) => attribute.parse_args::<ExternalArguments>()?,
            Meta::Path(_) => syn::parse2::<ExternalArguments>(TokenStream::new())?,
            Meta::NameValue(_) => {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "`#[geam::external]` requires `name = \"...\"` arguments",
                ));
            }
        };
        found = Some(arguments);
    }
    *attributes = retained;
    Ok(found)
}

pub(super) fn build_external_model(
    index: usize,
    payload: &mut ItemStruct,
    arguments: ExternalArguments,
    support: &TokenStream,
) -> syn::Result<ExternalModel> {
    let generic = if arguments.parameters.is_empty() {
        if arguments.input.is_some() {
            return Err(syn::Error::new_spanned(
                arguments.input,
                "external argument `input` requires non-empty `parameters`",
            ));
        }
        if arguments.payload.is_some() {
            return Err(syn::Error::new_spanned(
                arguments.payload,
                "external argument `payload` requires non-empty `parameters`",
            ));
        }
        if !payload.generics.params.is_empty() || payload.generics.where_clause.is_some() {
            return Err(syn::Error::new_spanned(
                &payload.generics,
                "external payload structs must not have generics; declare `parameters = [...]` for retained generic externals",
            ));
        }
        for field in &mut payload.fields {
            if take_marker(&mut field.attrs, "stored")? {
                return Err(syn::Error::new_spanned(
                    field,
                    "`#[geam::stored]` fields require non-empty external `parameters`",
                ));
            }
        }
        None
    } else {
        if arguments.retained {
            return Err(syn::Error::new_spanned(
                &payload.ident,
                "external argument `retained` is only supported for non-generic payloads",
            ));
        }
        let Some(input) = arguments.input.clone() else {
            return Err(syn::Error::new_spanned(
                &payload.ident,
                "generic external declarations require `input = Type`",
            ));
        };
        if payload.generics.where_clause.is_some() {
            return Err(syn::Error::new_spanned(
                &payload.generics,
                "generic external declarations must not have where clauses",
            ));
        }
        let mut declared = Vec::new();
        for parameter in &payload.generics.params {
            let GenericParam::Type(parameter) = parameter else {
                return Err(syn::Error::new_spanned(
                    parameter,
                    "generic external declarations support only type parameters",
                ));
            };
            if !parameter.bounds.is_empty() || parameter.default.is_some() {
                return Err(syn::Error::new_spanned(
                    parameter,
                    "generic external type parameters must not have bounds or defaults",
                ));
            }
            declared.push(parameter.ident.clone());
        }
        if declared != arguments.parameters {
            return Err(syn::Error::new_spanned(
                &payload.generics,
                "external `parameters` must list every Rust type parameter once in declaration order",
            ));
        }
        let mut unique = BTreeSet::new();
        for parameter in &arguments.parameters {
            if !unique.insert(parameter.unraw().to_string()) {
                return Err(syn::Error::new(
                    parameter.span(),
                    format!("duplicate external parameter `{parameter}`"),
                ));
            }
        }
        if let Some(explicit_payload) = arguments.payload.clone() {
            let mut retained_accessors = BTreeSet::new();
            for parameter in &arguments.parameters {
                let accessor = retained_parameter_accessor(parameter);
                if !retained_accessors.insert(accessor.to_string()) {
                    return Err(syn::Error::new(
                        parameter.span(),
                        format!(
                            "external parameter `{parameter}` generates duplicate retained accessor `{accessor}`"
                        ),
                    ));
                }
            }
            if !arguments.manual {
                return Err(syn::Error::new_spanned(
                    &explicit_payload,
                    "generic external `payload` requires bare `manual` semantics",
                ));
            }
            let Type::Path(TypePath {
                qself: None,
                path: payload_path,
            }) = &explicit_payload
            else {
                return Err(syn::Error::new_spanned(
                    &explicit_payload,
                    "generic external `payload` must be a non-generic type path",
                ));
            };
            for segment in &payload_path.segments {
                if !matches!(segment.arguments, PathArguments::None) {
                    return Err(syn::Error::new_spanned(
                        &explicit_payload,
                        "generic external `payload` must be a non-generic type path",
                    ));
                }
            }
            if !matches!(payload.fields, Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    &payload.fields,
                    "generic external declarations with an explicit payload require a unit marker struct",
                ));
            }
            let context = format_ident!("__GeamExternalContext");
            let parameters = &arguments.parameters;
            payload
                .generics
                .params
                .push(GenericParam::Type(syn::parse_quote! {
                    #context = #support::MissingExternalOutputContext
                }));
            payload.fields = Fields::Named(syn::parse_quote!({
                __geam_context: #context,
                __geam_parameters: ::core::marker::PhantomData<fn() -> (#(#parameters,)*)>,
            }));
            payload.semi_token = None;
            Some(GenericExternalModel {
                parameters: arguments.parameters.clone(),
                input,
                visibility: payload.vis.clone(),
                storage: GenericExternalStorage::ManualPayload {
                    payload: explicit_payload,
                },
            })
        } else {
            if arguments.manual {
                return Err(syn::Error::new_spanned(
                    &payload.ident,
                    "generic external `manual` semantics require an explicit retained `payload = Type`",
                ));
            }
            let Fields::Named(fields) = &mut payload.fields else {
                return Err(syn::Error::new_spanned(
                    &payload.fields,
                    "generic external declarations require named `#[geam::stored]` fields",
                ));
            };
            if fields.named.is_empty() {
                return Err(syn::Error::new_spanned(
                    &payload.ident,
                    "generic external declarations require at least one `#[geam::stored]` field",
                ));
            }
            let mut stored_fields = Vec::with_capacity(fields.named.len());
            let mut used = BTreeSet::new();
            for (field_index, field) in fields.named.iter_mut().enumerate() {
                if !take_marker(&mut field.attrs, "stored")? {
                    return Err(syn::Error::new_spanned(
                        field,
                        "generic external fields must be marked `#[geam::stored]`",
                    ));
                }
                let Some((stored, mut stored_path)) =
                    collection_item_with_path(&field.ty, "Stored")?
                else {
                    return Err(syn::Error::new_spanned(
                        &field.ty,
                        "`#[geam::stored]` fields must use `Stored<Parameter>`",
                    ));
                };
                let Type::Path(TypePath { qself: None, path }) = stored else {
                    return Err(syn::Error::new_spanned(
                        &field.ty,
                        "`#[geam::stored]` fields must name one declared external parameter",
                    ));
                };
                let Some(parameter) = path.get_ident().cloned() else {
                    return Err(syn::Error::new_spanned(
                        path,
                        "`#[geam::stored]` fields must name one declared external parameter",
                    ));
                };
                let Some(parameter_index) = arguments
                    .parameters
                    .iter()
                    .position(|declared| declared == &parameter)
                else {
                    return Err(syn::Error::new_spanned(
                        parameter,
                        "`#[geam::stored]` fields must name one declared external parameter",
                    ));
                };
                used.insert(parameter_index);
                let context = format_ident!("__GeamStoredContext{field_index}");
                for segment in stored_path.path.segments.iter_mut().rev().take(1) {
                    segment.arguments =
                        PathArguments::AngleBracketed(syn::parse_quote!(<#parameter, #context>));
                }
                field.ty = Type::Path(stored_path);
                let Some(ident) = field.ident.clone() else {
                    return Err(syn::Error::new_spanned(
                        field,
                        "generic external declarations require named `#[geam::stored]` fields",
                    ));
                };
                stored_fields.push(StoredExternalField {
                    ident,
                    parameter,
                    parameter_index,
                    index: host_type_index(parameter_index, support),
                });
                payload
                    .generics
                    .params
                    .push(GenericParam::Type(syn::parse_quote! {
                        #context = #support::MissingStoredContext
                    }));
            }
            for (parameter_index, parameter) in arguments.parameters.iter().enumerate() {
                if !used.contains(&parameter_index) {
                    return Err(syn::Error::new(
                        parameter.span(),
                        format!(
                            "external parameter `{parameter}` must own at least one `#[geam::stored]` field"
                        ),
                    ));
                }
            }
            let generated_payload = format_ident!("__GeamExternalPayload{index}");
            let generated_owner = format_ident!("__GeamExternalOwner{index}");
            Some(GenericExternalModel {
                parameters: arguments.parameters.clone(),
                input,
                visibility: payload.vis.clone(),
                storage: GenericExternalStorage::StoredFields {
                    payload: generated_payload,
                    owner: generated_owner,
                    fields: stored_fields,
                },
            })
        }
    };

    Ok(ExternalModel {
        ident: payload.ident.clone(),
        name: arguments.name,
        semantics: if arguments.retained {
            ExternalSemantics::Retained
        } else if arguments.manual {
            ExternalSemantics::Manual
        } else {
            ExternalSemantics::Default
        },
        schema: format_ident!("__GeamExternalSchema{index}"),
        storage: format_ident!("__GeamExternalStorage{index}"),
        store_field: format_ident!("__geam_external_{index}"),
        generic,
    })
}

pub(super) fn host_type_index(index: usize, support: &TokenStream) -> TokenStream {
    let mut value = quote!(#support::HostTypeIndex0);
    for _ in 0..index {
        value = quote!(#support::HostTypeIndexNext<#value>);
    }
    value
}

pub(super) fn retained_parameter_accessor(parameter: &Ident) -> Ident {
    let span = parameter.span();
    let parameter = parameter.unraw().to_string();
    let characters = parameter.chars().collect::<Vec<_>>();
    let mut suffix = String::with_capacity(parameter.len());
    for (index, character) in characters.iter().copied().enumerate() {
        let previous_is_lowercase = index
            .checked_sub(1)
            .and_then(|previous| characters.get(previous))
            .is_some_and(|previous| previous.is_ascii_lowercase() || previous.is_ascii_digit());
        let next_is_lowercase = characters
            .get(index + 1)
            .is_some_and(char::is_ascii_lowercase);
        if character.is_ascii_uppercase()
            && index > 0
            && (previous_is_lowercase || next_is_lowercase)
        {
            suffix.push('_');
        }
        suffix.push(character.to_ascii_lowercase());
    }
    format_ident!("stored_{}", suffix, span = span,)
}

pub(super) fn validate_function(
    function: &mut ItemFn,
    arguments: FunctionArguments,
    module_profile: Option<&Path>,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    support: &TokenStream,
) -> syn::Result<FunctionModel> {
    if function.sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.constness,
            "provider functions must not be const",
        ));
    }
    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.asyncness,
            "provider functions must not be async",
        ));
    }
    if function.sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.unsafety,
            "provider functions must be safe",
        ));
    }
    if function.sig.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "provider functions must use the ordinary Rust ABI",
        ));
    }
    let mut generic_scope = GenericParameterScope::new(&function.sig.generics)?;
    let profile = match (arguments.profile, module_profile) {
        (None, _) => None,
        (Some(profile), Some(bound)) => Some((profile, bound)),
        (Some(profile), None) => {
            return Err(syn::Error::new_spanned(
                profile,
                "function `profile` requires module `profile` and `component`",
            ));
        }
    };
    let ReturnType::Type(_, return_type) = &function.sig.output else {
        return Err(syn::Error::new_spanned(
            &function.sig.output,
            "provider functions require an explicit return type",
        ));
    };
    let declared_return_type = (**return_type).clone();
    let host_result = host_result_value(&declared_return_type)?;
    let rust_return_type = host_result
        .as_ref()
        .map(|(value, _)| value.clone())
        .unwrap_or_else(|| declared_return_type.clone());
    reject_unwrapped_generic(&rust_return_type, &generic_scope, externals)?;
    if matches!(&rust_return_type, Type::Tuple(tuple) if tuple.elems.is_empty()) {
        function
            .attrs
            .push(syn::parse_quote!(#[allow(clippy::unused_unit)]));
    }
    let return_ = classify_return(
        &rust_return_type,
        externals,
        customs,
        list_decoders,
        &mut generic_scope,
        support,
    )?;

    let mut call = CallAccess::None;
    let mut arguments = Vec::new();
    let mut has_list = false;
    let host_return = host_return_type(&return_, customs, support);
    let active_profile = profile
        .as_ref()
        .map(|(profile, _)| quote!(#profile))
        .unwrap_or_else(|| quote!(__GeamProfile));
    for (index, argument) in function.sig.inputs.iter_mut().enumerate() {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(
                argument,
                "provider functions must be free functions",
            ));
        };
        let is_call = take_marker(&mut argument.attrs, "call")?;
        if is_call {
            if index != 0 {
                return Err(syn::Error::new_spanned(
                    argument,
                    "the `#[geam::call]` parameter must be first",
                ));
            }
            let (mutable, state, call_type) = call_parameter(&argument.ty)?;
            if mutable {
                let call_type = call_context_type(
                    &call_type,
                    &state,
                    syn::parse_quote! {
                        #support::ProviderActiveCall<
                            '__geam_call,
                            #active_profile,
                            __GeamProvider,
                            #host_return,
                        >
                    },
                );
                *argument.ty = syn::parse_quote! {
                    &mut #call_type
                };
                call = CallAccess::Mutable;
            } else {
                let call_type = call_context_type(
                    &call_type,
                    &state,
                    syn::parse_quote! {
                        #support::ProviderSharedCall<'__geam_call, #state>
                    },
                );
                *argument.ty = syn::parse_quote! {
                    &#call_type
                };
                call = CallAccess::Shared;
            }
        } else {
            if is_call_type(&argument.ty) {
                return Err(syn::Error::new_spanned(
                    &argument.ty,
                    "Call<State> parameters require `#[geam::call]`",
                ));
            }
            let callback_index = arguments.len();
            let type_ = if let Some(callback) = callback_type(
                &argument.ty,
                &function.sig.ident,
                callback_index,
                externals,
                customs,
                list_decoders,
                &mut generic_scope,
                support,
            )? {
                FunctionArgumentType::Callback(Box::new(callback))
            } else {
                reject_unwrapped_generic(&argument.ty, &generic_scope, externals)?;
                FunctionArgumentType::Input(classify_input(
                    &argument.ty,
                    externals,
                    customs,
                    list_decoders,
                    &mut generic_scope,
                    support,
                    false,
                )?)
            };
            match &type_ {
                FunctionArgumentType::Input(FunctionInputType::List(list)) => {
                    *argument.ty = list_signature_type(list, customs, support);
                    has_list = true;
                }
                FunctionArgumentType::Input(FunctionInputType::Generic(value)) => {
                    *argument.ty = generic_value_signature_type(value, customs, support);
                }
                FunctionArgumentType::Input(FunctionInputType::External(external)) => {
                    *argument.ty = generic_external_input_signature_type(
                        external,
                        customs,
                        support,
                        GenericInputSource::Declared,
                    );
                }
                FunctionArgumentType::Callback(callback) => {
                    *argument.ty = callback_signature_type(
                        callback,
                        &generic_scope.declared,
                        &active_profile,
                        &host_return,
                        support,
                    );
                }
                FunctionArgumentType::Input(FunctionInputType::Value(value))
                    if function_input_contains_source_wrapper(value) =>
                {
                    let value = provider_value_from_input_root(value);
                    *argument.ty = provider_input_signature_type(&value, customs, support);
                }
                FunctionArgumentType::Input(FunctionInputType::Value(_)) => {}
            }
            arguments.push(type_);
        }
    }
    if arguments.len() > 7 {
        return Err(syn::Error::new_spanned(
            &function.sig.inputs,
            "provider functions support at most seven source arguments",
        ));
    }

    let generics = generic_scope.finish()?;
    let model = FunctionModel {
        ident: function.sig.ident.clone(),
        generics,
        arguments,
        return_,
        call,
        host_result: host_result.is_some(),
        profile: profile.is_some(),
    };
    if function_contains_callback(&model) && !matches!(model.call, CallAccess::Mutable) {
        return Err(syn::Error::new_spanned(
            &function.sig.inputs,
            "Callback arguments require a first `#[geam::call]` parameter using `&mut Call<State>`",
        ));
    }
    validate_list_return(&model)?;
    if let FunctionReturnType::List(list) = &model.return_ {
        let list = list_signature_type(list, customs, support);
        let output = wrap_host_result_type(list, host_result.as_ref().map(|(_, path)| path));
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
        has_list = true;
    } else if let FunctionReturnType::Generic(value) = &model.return_ {
        let output = generic_value_signature_type(value, customs, support);
        let output = wrap_host_result_type(output, host_result.as_ref().map(|(_, path)| path));
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
    } else if let FunctionReturnType::External(external) = &model.return_ {
        let output = generic_external_output_signature_type(external, customs, support);
        let output = wrap_host_result_type(output, host_result.as_ref().map(|(_, path)| path));
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
    } else if let FunctionReturnType::Value(value) = &model.return_
        && function_root_output_contains_generic(value)
    {
        let value = function_output_from_root(value);
        let output = function_output_rust_type(&value, customs, support);
        let output = wrap_host_result_type(output, host_result.as_ref().map(|(_, path)| path));
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
    }
    if !matches!(model.call, CallAccess::None)
        || function_contains_generic_value(&model)
        || function_contains_nested_list(&model)
        || function_contains_call_scoped_generic_external(&model)
        || function_contains_callback(&model)
    {
        prepend_function_lifetime(&mut function.sig.generics, syn::parse_quote!('__geam_call));
    }
    if has_list {
        prepend_function_lifetime(&mut function.sig.generics, syn::parse_quote!('__geam_list));
    }
    if has_list && !matches!(model.call, CallAccess::None) {
        function
            .sig
            .generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote!('__geam_list: '__geam_call));
    }
    if let Some((profile, bound)) = &profile {
        function
            .sig
            .generics
            .params
            .push(GenericParam::Type(syn::parse_quote!(#profile)));
        function
            .sig
            .generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote!(#profile: #bound));
    }
    if matches!(model.call, CallAccess::Mutable) && profile.is_none() {
        function
            .sig
            .generics
            .params
            .push(GenericParam::Type(syn::parse_quote!(__GeamProfile)));
        function
            .sig
            .generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote! {
                __GeamProfile: __GeamModuleProfile
            });
    }
    Ok(model)
}

fn prepend_function_lifetime(generics: &mut syn::Generics, lifetime: syn::LifetimeParam) {
    let existing = std::mem::take(&mut generics.params);
    generics.params.push(GenericParam::Lifetime(lifetime));
    generics.params.extend(existing);
}

fn provider_value_contains_source_wrapper(type_: &ProviderValueType) -> bool {
    match type_ {
        ProviderValueType::Result { .. }
        | ProviderValueType::Option { .. }
        | ProviderValueType::List(_) => true,
        ProviderValueType::Tuple(elements) => {
            elements.iter().any(provider_value_contains_source_wrapper)
        }
        ProviderValueType::Scalar(_)
        | ProviderValueType::Generic(_)
        | ProviderValueType::Declared { .. }
        | ProviderValueType::External { .. }
        | ProviderValueType::Custom { .. } => false,
    }
}

fn function_input_contains_source_wrapper(type_: &FunctionInputValueType) -> bool {
    match type_ {
        FunctionInputValueType::Result { .. } | FunctionInputValueType::Option { .. } => true,
        FunctionInputValueType::Tuple(elements) => {
            elements.iter().any(provider_value_contains_source_wrapper)
        }
        FunctionInputValueType::Scalar(_)
        | FunctionInputValueType::Declared { .. }
        | FunctionInputValueType::External { .. }
        | FunctionInputValueType::Custom { .. } => false,
    }
}

fn provider_value_nested_generic_source(value: &ProviderValueType) -> Option<&Type> {
    match value {
        ProviderValueType::Generic(value) => Some(&value.source),
        ProviderValueType::Tuple(elements) => elements
            .iter()
            .find_map(provider_value_nested_generic_source),
        ProviderValueType::Result { success, failure } => {
            provider_value_nested_generic_source(success)
                .or_else(|| provider_value_nested_generic_source(failure))
        }
        ProviderValueType::Option { value } => provider_value_nested_generic_source(value),
        ProviderValueType::Scalar(_)
        | ProviderValueType::Declared { .. }
        | ProviderValueType::External { .. }
        | ProviderValueType::Custom { .. }
        | ProviderValueType::List(_) => None,
    }
}

fn function_input_nested_generic_source(value: &FunctionInputValueType) -> Option<&Type> {
    match value {
        FunctionInputValueType::Tuple(elements) => elements
            .iter()
            .find_map(provider_value_nested_generic_source),
        FunctionInputValueType::Result { success, failure } => {
            provider_value_nested_generic_source(success)
                .or_else(|| provider_value_nested_generic_source(failure))
        }
        FunctionInputValueType::Option { value } => provider_value_nested_generic_source(value),
        FunctionInputValueType::Scalar(_)
        | FunctionInputValueType::Declared { .. }
        | FunctionInputValueType::External { .. }
        | FunctionInputValueType::Custom { .. } => None,
    }
}

fn function_output_contains_generic(value: &FunctionOutputValueType) -> bool {
    match value {
        FunctionOutputValueType::Generic(_) => true,
        FunctionOutputValueType::Tuple(elements) => {
            elements.iter().any(function_output_contains_generic)
        }
        FunctionOutputValueType::Result { success, failure } => {
            function_output_contains_generic(success) || function_output_contains_generic(failure)
        }
        FunctionOutputValueType::Option { value } => function_output_contains_generic(value),
        FunctionOutputValueType::Vec(collection) => {
            function_output_contains_generic(&collection.value)
        }
        FunctionOutputValueType::Value(_) => false,
    }
}

fn function_root_output_contains_generic(value: &FunctionRootOutputValueType) -> bool {
    match value {
        FunctionRootOutputValueType::Tuple(elements) => {
            elements.iter().any(function_output_contains_generic)
        }
        FunctionRootOutputValueType::Result { success, failure } => {
            function_output_contains_generic(success) || function_output_contains_generic(failure)
        }
        FunctionRootOutputValueType::Option { value } => function_output_contains_generic(value),
        FunctionRootOutputValueType::Vec(collection) => {
            function_output_contains_generic(&collection.value)
        }
        FunctionRootOutputValueType::Value(_) => false,
    }
}

fn function_contains_generic_value(function: &FunctionModel) -> bool {
    if matches!(function.return_, FunctionReturnType::Generic(_))
        || matches!(
            &function.return_,
            FunctionReturnType::Value(value) if function_root_output_contains_generic(value)
        )
    {
        return true;
    }
    for argument in &function.arguments {
        if matches!(
            argument,
            FunctionArgumentType::Input(FunctionInputType::Generic(_))
        ) {
            return true;
        }
    }
    false
}

fn provider_value_contains_list(value: &ProviderValueType) -> bool {
    match value {
        ProviderValueType::List(_) => true,
        ProviderValueType::Tuple(elements) => elements.iter().any(provider_value_contains_list),
        ProviderValueType::Result { success, failure } => {
            provider_value_contains_list(success) || provider_value_contains_list(failure)
        }
        ProviderValueType::Option { value } => provider_value_contains_list(value),
        ProviderValueType::Scalar(_)
        | ProviderValueType::Generic(_)
        | ProviderValueType::Declared { .. }
        | ProviderValueType::External { .. }
        | ProviderValueType::Custom { .. } => false,
    }
}

fn function_input_contains_list(value: &FunctionInputValueType) -> bool {
    match value {
        FunctionInputValueType::Tuple(elements) => {
            elements.iter().any(provider_value_contains_list)
        }
        FunctionInputValueType::Result { success, failure } => {
            provider_value_contains_list(success) || provider_value_contains_list(failure)
        }
        FunctionInputValueType::Option { value } => provider_value_contains_list(value),
        FunctionInputValueType::Scalar(_)
        | FunctionInputValueType::Declared { .. }
        | FunctionInputValueType::External { .. }
        | FunctionInputValueType::Custom { .. } => false,
    }
}

fn function_contains_nested_list(function: &FunctionModel) -> bool {
    function.arguments.iter().any(|argument| {
        matches!(
            argument,
            FunctionArgumentType::Input(FunctionInputType::Value(value))
                if function_input_contains_list(value)
        )
    })
}

fn function_contains_call_scoped_generic_external(function: &FunctionModel) -> bool {
    if matches!(
        &function.return_,
        FunctionReturnType::External(external)
            if matches!(external.storage, GenericExternalStorage::StoredFields { .. })
    ) {
        return true;
    }
    function.arguments.iter().any(|argument| {
        matches!(
            argument,
            FunctionArgumentType::Input(FunctionInputType::External(_))
        )
    })
}

fn function_contains_callback(function: &FunctionModel) -> bool {
    function
        .arguments
        .iter()
        .any(|argument| matches!(argument, FunctionArgumentType::Callback(_)))
}

fn call_parameter(type_: &Type) -> syn::Result<(bool, Type, TypePath)> {
    let Type::Reference(reference) = type_ else {
        return Err(syn::Error::new_spanned(
            type_,
            "the `#[geam::call]` parameter must be `&Call<State>` or `&mut Call<State>`",
        ));
    };
    let Some((state, call_type)) = collection_item_with_path(&reference.elem, "Call")? else {
        return Err(syn::Error::new_spanned(
            type_,
            "the `#[geam::call]` parameter must be `&Call<State>` or `&mut Call<State>`",
        ));
    };
    Ok((reference.mutability.is_some(), state, call_type))
}

fn call_context_type(call_type: &TypePath, state: &Type, context: Type) -> TypePath {
    let mut call_type = call_type.clone();
    for segment in call_type.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote!(<#state, #context>));
    }
    call_type
}

fn collection_type_with_item(collection: &TypePath, item: &Type) -> TypePath {
    let mut collection = collection.clone();
    for segment in collection.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote!(<#item>));
    }
    collection
}

fn wrap_host_result_type(output: Type, host_result: Option<&TypePath>) -> Type {
    if let Some(host_result) = host_result {
        Type::Path(collection_type_with_item(host_result, &output))
    } else {
        output
    }
}

fn collection_type_with_items(collection: &TypePath, items: &[Type]) -> TypePath {
    let mut collection = collection.clone();
    for segment in collection.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote!(<#(#items),*>));
    }
    collection
}

fn is_call_type(type_: &Type) -> bool {
    let type_ = if let Type::Reference(reference) = type_ {
        &reference.elem
    } else {
        type_
    };
    is_collection(type_, "Call")
}

fn reject_unwrapped_generic(
    type_: &Type,
    generics: &GenericParameterScope,
    externals: &[ExternalModel],
) -> syn::Result<()> {
    if is_generic_external_application(type_, externals) {
        return Ok(());
    }
    if let Some(ident) = find_unwrapped_generic(type_, generics) {
        return Err(syn::Error::new_spanned(
            type_,
            format!("generic source type `{ident}` must be written as Value<{ident}>",),
        ));
    }
    Ok(())
}

fn is_generic_external_application(type_: &Type, externals: &[ExternalModel]) -> bool {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return false;
    };
    if path.segments.len() != 1 {
        return false;
    }
    let ident = &path.segments[0].ident;
    externals.iter().any(|external| {
        external
            .generic
            .as_ref()
            .is_some_and(|generic| &external.ident == ident || &generic.input == ident)
    })
}

pub(super) fn find_unwrapped_generic(
    type_: &Type,
    generics: &GenericParameterScope,
) -> Option<Ident> {
    if is_type_application_named(type_, "Value") {
        return None;
    }
    if let Some(ident) = generics.declared_ident(type_) {
        return Some(ident);
    }

    if let Type::BareFn(function) = type_ {
        for argument in &function.inputs {
            if let Some(ident) = find_unwrapped_generic(&argument.ty, generics) {
                return Some(ident);
            }
        }
        if let ReturnType::Type(_, return_) = &function.output {
            return find_unwrapped_generic(return_, generics);
        }
        return None;
    }
    if let Type::Paren(paren) = type_ {
        return find_unwrapped_generic(&paren.elem, generics);
    }
    if let Type::Path(path) = type_ {
        for segment in &path.path.segments {
            if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                for argument in &arguments.args {
                    if let GenericArgument::Type(type_) = argument
                        && let Some(ident) = find_unwrapped_generic(type_, generics)
                    {
                        return Some(ident);
                    }
                }
            }
        }
        return None;
    }
    if let Type::Reference(reference) = type_ {
        return find_unwrapped_generic(&reference.elem, generics);
    }
    if let Type::Tuple(tuple) = type_ {
        for element in &tuple.elems {
            if let Some(ident) = find_unwrapped_generic(element, generics) {
                return Some(ident);
            }
        }
        return None;
    }
    None
}

fn generic_value_type(
    type_: &Type,
    generics: &mut GenericParameterScope,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<Option<GenericValueType>> {
    let is_value_application = is_type_application_named(type_, "Value");
    if !is_value_application
        && (external_type(type_, externals).is_some()
            || custom_output_type(type_, customs).is_some()
            || custom_input_model(type_, customs).is_some()
            || is_qualified_type_path(type_))
    {
        return Ok(None);
    }
    let Some((source, path)) = collection_item_with_path(type_, "Value")? else {
        return Ok(None);
    };
    let classified = classify_generic_host_type(&source, generics, externals, customs, support)?;
    if !generic_host_contains_parameter(&classified.host)
        && !matches!(classified.host, GenericHostType::Function { .. })
    {
        return Err(syn::Error::new_spanned(
            type_,
            "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
        ));
    }
    Ok(Some(GenericValueType {
        source,
        instantiated: classified.instantiated,
        path,
        host: classified.host,
    }))
}

#[allow(clippy::too_many_arguments)]
fn callback_type(
    type_: &Type,
    function: &Ident,
    argument_index: usize,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<Option<CallbackType>> {
    if let Type::Reference(reference) = type_
        && is_collection(&reference.elem, "Callback")
    {
        return Err(syn::Error::new_spanned(
            type_,
            "Callback arguments must be passed by value",
        ));
    }
    let Some((signature, path)) = collection_item_with_path(type_, "Callback")? else {
        return Ok(None);
    };
    let Type::BareFn(signature) = signature else {
        return Err(syn::Error::new_spanned(
            type_,
            "Callback<T> requires a safe non-variadic Rust fn signature",
        ));
    };
    if signature.lifetimes.is_some()
        || signature.unsafety.is_some()
        || signature.abi.is_some()
        || signature.variadic.is_some()
    {
        return Err(syn::Error::new_spanned(
            &signature,
            "Callback<T> requires a safe non-variadic Rust fn signature without lifetimes",
        ));
    }
    if signature.inputs.len() > 7 {
        return Err(syn::Error::new_spanned(
            &signature.inputs,
            "provider callbacks support at most seven source arguments",
        ));
    }

    // Source function parameters are indexed from the return shape first.
    let return_type = match &signature.output {
        ReturnType::Default => syn::parse_quote!(()),
        ReturnType::Type(_, type_) => (**type_).clone(),
    };
    if is_collection(&return_type, "Callback") {
        return Err(syn::Error::new_spanned(
            &return_type,
            "callbacks returned by a callback are opaque values; use Value<fn(...) -> ...>",
        ));
    }
    reject_unwrapped_generic(&return_type, generics, externals)?;
    let return_ = classify_input(
        &return_type,
        externals,
        customs,
        list_decoders,
        generics,
        support,
        true,
    )?;

    let mut arguments = Vec::with_capacity(signature.inputs.len());
    for argument in &signature.inputs {
        if is_collection(&argument.ty, "Callback") {
            return Err(syn::Error::new_spanned(
                &argument.ty,
                "callback arguments that are functions must use Value<fn(...) -> ...>",
            ));
        }
        reject_unwrapped_generic(&argument.ty, generics, externals)?;
        arguments.push(classify_return(
            &argument.ty,
            externals,
            customs,
            list_decoders,
            generics,
            support,
        )?);
    }

    Ok(Some(CallbackType {
        signature,
        path,
        arguments,
        return_: Box::new(return_),
        codec: format_ident!(
            "__GeamCallbackCodec_{}_{}",
            function.unraw(),
            argument_index,
        ),
    }))
}

fn classify_generic_host_type(
    type_: &Type,
    generics: &mut GenericParameterScope,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<ClassifiedGenericHostType> {
    if let Some(ident) = generics.declared_ident(type_) {
        let index = generics.parameter_index(ident);
        return Ok(ClassifiedGenericHostType {
            host: GenericHostType::Parameter { index },
            instantiated: syn::parse_quote!(#support::HostTypeParameter<#index>),
        });
    }
    if let Type::Reference(_) = type_ {
        return Err(syn::Error::new_spanned(
            type_,
            "generic source shapes must not contain Rust references",
        ));
    }
    if is_type_application_named(type_, "Value") {
        return Err(syn::Error::new_spanned(
            type_,
            "Value<...> wrappers must not be nested inside generic source shapes",
        ));
    }
    if let Some((item, path)) = collection_item_with_path(type_, "List")? {
        let item = classify_generic_host_type(&item, generics, externals, customs, support)?;
        return Ok(ClassifiedGenericHostType {
            instantiated: Type::Path(collection_type_with_item(&path, &item.instantiated)),
            host: GenericHostType::List(Box::new(item.host)),
        });
    }
    if collection_item(type_, "Vec")?.is_some() {
        return Err(syn::Error::new_spanned(
            type_,
            "Vec<T> is not a generic source type; use geam::List<T>",
        ));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            path,
            success,
            failure,
        } => {
            let success =
                classify_generic_host_type(success, generics, externals, customs, support)?;
            let failure =
                classify_generic_host_type(failure, generics, externals, customs, support)?;
            let instantiated = collection_type_with_items(
                path,
                &[success.instantiated.clone(), failure.instantiated.clone()],
            );
            return Ok(ClassifiedGenericHostType {
                instantiated: Type::Path(instantiated),
                host: GenericHostType::Result {
                    success: Box::new(success.host),
                    failure: Box::new(failure.host),
                },
            });
        }
        SourceWrapper::Option { path, value } => {
            let value = classify_generic_host_type(value, generics, externals, customs, support)?;
            return Ok(ClassifiedGenericHostType {
                instantiated: Type::Path(collection_type_with_item(path, &value.instantiated)),
                host: GenericHostType::Option(Box::new(value.host)),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Some(external) = external_type(type_, externals) {
        return Ok(ClassifiedGenericHostType {
            instantiated: type_.clone(),
            host: GenericHostType::External {
                schema: external.schema.clone(),
            },
        });
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut host_elements = Vec::with_capacity(tuple.elems.len());
        let mut instantiated_elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            let element =
                classify_generic_host_type(element, generics, externals, customs, support)?;
            host_elements.push(element.host);
            instantiated_elements.push(element.instantiated);
        }
        return Ok(ClassifiedGenericHostType {
            host: GenericHostType::Tuple(host_elements),
            instantiated: syn::parse_quote!((#(#instantiated_elements,)*)),
        });
    }
    if let Type::BareFn(function) = type_ {
        if function.lifetimes.is_some() {
            return Err(syn::Error::new_spanned(
                function,
                "opaque source function shapes must not declare lifetimes",
            ));
        }
        if function.unsafety.is_some() || function.abi.is_some() || function.variadic.is_some() {
            return Err(syn::Error::new_spanned(
                function,
                "opaque source function shapes must use safe non-variadic Rust fn syntax",
            ));
        }
        let return_type = match &function.output {
            ReturnType::Default => syn::parse_quote!(()),
            ReturnType::Type(_, type_) => (**type_).clone(),
        };
        let return_type =
            classify_generic_host_type(&return_type, generics, externals, customs, support)?;
        let mut host_arguments = Vec::with_capacity(function.inputs.len());
        let mut instantiated_arguments = Vec::with_capacity(function.inputs.len());
        for argument in &function.inputs {
            let argument =
                classify_generic_host_type(&argument.ty, generics, externals, customs, support)?;
            host_arguments.push(argument.host);
            instantiated_arguments.push(argument.instantiated);
        }
        let instantiated_return = &return_type.instantiated;
        return Ok(ClassifiedGenericHostType {
            host: GenericHostType::Function {
                arguments: host_arguments,
                return_: Box::new(return_type.host),
            },
            instantiated: syn::parse_quote! {
                fn(#(#instantiated_arguments),*) -> #instantiated_return
            },
        });
    }
    if let Some((index, _, _)) = custom_input_model(type_, customs) {
        return Ok(ClassifiedGenericHostType {
            instantiated: type_.clone(),
            host: GenericHostType::Custom { index },
        });
    }
    if let Type::Path(TypePath { qself: None, path }) = type_
        && path.segments.len() > 1
    {
        for segment in &path.segments {
            if !matches!(segment.arguments, PathArguments::None) {
                return Err(syn::Error::new_spanned(
                    type_,
                    "generic declared source types are not supported inside generic source shapes",
                ));
            }
        }
        return Ok(ClassifiedGenericHostType {
            host: GenericHostType::Declared(type_.clone()),
            instantiated: type_.clone(),
        });
    }
    Ok(ClassifiedGenericHostType {
        host: GenericHostType::Scalar(type_.clone()),
        instantiated: type_.clone(),
    })
}

fn generic_host_contains_parameter(type_: &GenericHostType) -> bool {
    match type_ {
        GenericHostType::Parameter { .. } => true,
        GenericHostType::Tuple(elements) => {
            for element in elements {
                if generic_host_contains_parameter(element) {
                    return true;
                }
            }
            false
        }
        GenericHostType::List(item) | GenericHostType::Option(item) => {
            generic_host_contains_parameter(item)
        }
        GenericHostType::Result {
            success, failure, ..
        } => generic_host_contains_parameter(success) || generic_host_contains_parameter(failure),
        GenericHostType::Function { arguments, return_ } => {
            for argument in arguments {
                if generic_host_contains_parameter(argument) {
                    return true;
                }
            }
            generic_host_contains_parameter(return_)
        }
        GenericHostType::Scalar(_)
        | GenericHostType::Declared(_)
        | GenericHostType::External { .. }
        | GenericHostType::Custom { .. } => false,
    }
}

fn classify_input(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
    allow_nested_generic: bool,
) -> syn::Result<FunctionInputType> {
    if let Some(external) =
        generic_external_type(type_, true, externals, generics, customs, support)?
    {
        return Ok(FunctionInputType::External(Box::new(external)));
    }
    if let Some(external) =
        generic_external_type(type_, false, externals, generics, customs, support)?
    {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "generic external output `{}` cannot be used as input; use `{}<...>`",
                external.output, external.input,
            ),
        ));
    }
    if let Type::Reference(reference) = type_ {
        if is_advanced_external(&reference.elem)? {
            return Err(syn::Error::new_spanned(
                type_,
                "provider::advanced::External<T> arguments are already retained views and must be passed by value",
            ));
        }
        if is_collection(&reference.elem, "List") {
            return Err(syn::Error::new_spanned(
                type_,
                "geam::List<T> arguments must be passed by value",
            ));
        }
        if is_collection(&reference.elem, "Vec") {
            return Err(syn::Error::new_spanned(
                type_,
                "Vec<T> arguments are not supported; use geam::List<T>",
            ));
        }
        if let Some(external) = external_type(&reference.elem, externals) {
            if reference.mutability.is_some() {
                return Err(syn::Error::new_spanned(
                    type_,
                    format!(
                        "external payload `{}` arguments must be immutable references",
                        external.ident
                    ),
                ));
            }
            return Ok(FunctionInputType::Value(Box::new(
                FunctionInputValueType::External {
                    payload: external.ident.clone(),
                    schema: external.schema.clone(),
                },
            )));
        }
        if is_non_empty_tuple(&reference.elem) {
            return Err(syn::Error::new_spanned(
                type_,
                "tuple arguments must be passed by value",
            ));
        }
        if reference.mutability.is_none() && is_qualified_type_path(&reference.elem) {
            return Ok(FunctionInputType::Value(Box::new(
                FunctionInputValueType::Declared {
                    type_: (*reference.elem).clone(),
                    input: DeclaredInput::BorrowedExternal,
                },
            )));
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source arguments may borrow only declared external payloads",
        ));
    }
    if let Some(value) = generic_value_type(type_, generics, externals, customs, support)? {
        return Ok(FunctionInputType::Generic(Box::new(value)));
    }
    if let Some(item) = collection_item(type_, "List")? {
        let value = classify_collection_input_item(&item, externals, customs, "List")?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(FunctionInputType::List(Box::new(ListType {
            collection,
            decoder,
        })));
    }
    if collection_item(type_, "Vec")?.is_some() {
        return Err(syn::Error::new_spanned(
            type_,
            "Vec<T> arguments are not supported; use geam::List<T>",
        ));
    }
    let value =
        classify_function_input_value(type_, externals, customs, list_decoders, generics, support)?;
    if !allow_nested_generic && let Some(source) = function_input_nested_generic_source(&value) {
        return Err(syn::Error::new_spanned(
            source,
            "Value<...> must be the complete source argument",
        ));
    }
    Ok(FunctionInputType::Value(Box::new(value)))
}

fn classify_function_input_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<FunctionInputValueType> {
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(FunctionInputValueType::Result {
                success: Box::new(classify_argument_value(
                    success,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
                failure: Box::new(classify_argument_value(
                    failure,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(FunctionInputValueType::Option {
                value: Box::new(classify_argument_value(
                    value,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Some(external) = external_type(type_, externals) {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "external payload `{}` arguments must be immutable references",
                external.ident
            ),
        ));
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            let element = classify_argument_value(
                element,
                externals,
                customs,
                list_decoders,
                generics,
                support,
            )?;
            elements.push(element);
        }
        return Ok(FunctionInputValueType::Tuple(elements));
    }
    if let Some((index, _, _)) = custom_input_model(type_, customs) {
        return Ok(FunctionInputValueType::Custom {
            index,
            rust: type_.clone(),
        });
    }
    if let Some(custom) = custom_output_type(type_, customs) {
        let input = if let Some(input) = &custom.input {
            format!("`{}`", input.ident)
        } else {
            "an explicit generated input type".to_owned()
        };
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "custom output `{}` cannot be used as a source argument; use {input}",
                custom.ident
            ),
        ));
    }
    if is_declared_provider_type(type_)? {
        return Ok(FunctionInputValueType::Declared {
            type_: type_.clone(),
            input: DeclaredInput::Owned,
        });
    }
    Ok(FunctionInputValueType::Scalar(type_.clone()))
}

fn classify_argument_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<ProviderValueType> {
    if let Some(value) = generic_value_type(type_, generics, externals, customs, support)? {
        return Ok(ProviderValueType::Generic(Box::new(value)));
    }
    if let Type::Reference(reference) = type_ {
        if let Some(external) = external_type(&reference.elem, externals) {
            if reference.mutability.is_some() {
                return Err(syn::Error::new_spanned(
                    type_,
                    format!(
                        "external payload `{}` arguments must be immutable references",
                        external.ident
                    ),
                ));
            }
            return Ok(ProviderValueType::External {
                payload: external.ident.clone(),
                schema: external.schema.clone(),
            });
        }
        if reference.mutability.is_none() && is_qualified_type_path(&reference.elem) {
            return Ok(ProviderValueType::Declared {
                type_: (*reference.elem).clone(),
                input: DeclaredInput::BorrowedExternal,
            });
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source arguments may borrow only declared external payloads",
        ));
    }
    if let Some(item) = collection_item(type_, "List")? {
        let value = classify_collection_input_item(&item, externals, customs, "List")?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(ProviderValueType::List(Box::new(ListType {
            collection,
            decoder,
        })));
    }
    if is_collection(type_, "Vec") {
        return Err(syn::Error::new_spanned(
            type_,
            "Vec<T> arguments are not supported; use geam::List<T>",
        ));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(ProviderValueType::Result {
                success: Box::new(classify_argument_value(
                    success,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
                failure: Box::new(classify_argument_value(
                    failure,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(ProviderValueType::Option {
                value: Box::new(classify_argument_value(
                    value,
                    externals,
                    customs,
                    list_decoders,
                    generics,
                    support,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Some(external) = external_type(type_, externals) {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "external payload `{}` arguments must be immutable references",
                external.ident
            ),
        ));
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            let element = classify_argument_value(
                element,
                externals,
                customs,
                list_decoders,
                generics,
                support,
            )?;
            elements.push(element);
        }
        return Ok(ProviderValueType::Tuple(elements));
    }
    if let Some((index, _, _)) = custom_input_model(type_, customs) {
        return Ok(ProviderValueType::Custom {
            index,
            rust: type_.clone(),
        });
    }
    if let Some(custom) = custom_output_type(type_, customs) {
        let input = if let Some(input) = &custom.input {
            format!("`{}`", input.ident)
        } else {
            "an explicit generated input type".to_owned()
        };
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "custom output `{}` cannot be used as a source argument; use {input}",
                custom.ident
            ),
        ));
    }
    if is_declared_provider_type(type_)? {
        return Ok(ProviderValueType::Declared {
            type_: type_.clone(),
            input: DeclaredInput::Owned,
        });
    }
    Ok(ProviderValueType::Scalar(type_.clone()))
}

fn classify_return(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<FunctionReturnType> {
    if let Some(external) =
        generic_external_type(type_, false, externals, generics, customs, support)?
    {
        return Ok(FunctionReturnType::External(Box::new(external)));
    }
    if let Some(external) =
        generic_external_type(type_, true, externals, generics, customs, support)?
    {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "generic external input `{}` cannot be returned; return `{}<...>`",
                external.input, external.output,
            ),
        ));
    }
    if let Some(value) = generic_value_type(type_, generics, externals, customs, support)? {
        return Ok(FunctionReturnType::Generic(Box::new(value)));
    }
    if let Type::Reference(reference) = type_ {
        if is_collection(&reference.elem, "List") {
            return Err(syn::Error::new_spanned(
                type_,
                "geam::List<T> returns must be owned",
            ));
        }
        if is_collection(&reference.elem, "Vec") {
            return Err(syn::Error::new_spanned(
                type_,
                "Vec<T> returns must be owned",
            ));
        }
        if let Some(external) = external_type(&reference.elem, externals) {
            return Err(syn::Error::new_spanned(
                type_,
                format!(
                    "external payload `{}` returns must be owned",
                    external.ident
                ),
            ));
        }
        if is_non_empty_tuple(&reference.elem) {
            return Err(syn::Error::new_spanned(
                type_,
                "tuple returns must be owned",
            ));
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source returns must be owned values",
        ));
    }
    if let Some(item) = collection_item(type_, "List")? {
        let value = classify_collection_input_item(&item, externals, customs, "List")?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(FunctionReturnType::List(Box::new(ListType {
            collection,
            decoder,
        })));
    }
    if let Some(item) = collection_item(type_, "Vec")? {
        let value = classify_function_output_value(&item, externals, customs, generics, support)?;
        return Ok(FunctionReturnType::Value(FunctionRootOutputValueType::Vec(
            FunctionOutputCollectionType {
                value: Box::new(value),
            },
        )));
    }
    Ok(FunctionReturnType::Value(
        classify_function_root_output_value(type_, externals, customs, generics, support)?,
    ))
}

fn classify_function_root_output_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<FunctionRootOutputValueType> {
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(FunctionRootOutputValueType::Result {
                success: Box::new(classify_function_output_value(
                    success, externals, customs, generics, support,
                )?),
                failure: Box::new(classify_function_output_value(
                    failure, externals, customs, generics, support,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(FunctionRootOutputValueType::Option {
                value: Box::new(classify_function_output_value(
                    value, externals, customs, generics, support,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            elements.push(classify_function_output_value(
                element, externals, customs, generics, support,
            )?);
        }
        return Ok(FunctionRootOutputValueType::Tuple(elements));
    }
    Ok(FunctionRootOutputValueType::Value(Box::new(
        classify_function_output_leaf(type_, externals, customs)?,
    )))
}

fn classify_function_output_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<FunctionOutputValueType> {
    if let Some(value) = generic_value_type(type_, generics, externals, customs, support)? {
        return Ok(FunctionOutputValueType::Generic(Box::new(value)));
    }
    if let Type::Reference(reference) = type_ {
        if let Some(external) = external_type(&reference.elem, externals) {
            return Err(syn::Error::new_spanned(
                type_,
                format!(
                    "external payload `{}` returns must be owned",
                    external.ident
                ),
            ));
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source returns must be owned values",
        ));
    }
    if is_collection(type_, "List") {
        return Err(syn::Error::new_spanned(
            type_,
            "geam::List<T> is supported only as a top-level source return",
        ));
    }
    if let Some(item) = collection_item(type_, "Vec")? {
        let value = classify_function_output_value(&item, externals, customs, generics, support)?;
        return Ok(FunctionOutputValueType::Vec(FunctionOutputCollectionType {
            value: Box::new(value),
        }));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(FunctionOutputValueType::Result {
                success: Box::new(classify_function_output_value(
                    success, externals, customs, generics, support,
                )?),
                failure: Box::new(classify_function_output_value(
                    failure, externals, customs, generics, support,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(FunctionOutputValueType::Option {
                value: Box::new(classify_function_output_value(
                    value, externals, customs, generics, support,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            elements.push(classify_function_output_value(
                element, externals, customs, generics, support,
            )?);
        }
        return Ok(FunctionOutputValueType::Tuple(elements));
    }
    Ok(FunctionOutputValueType::Value(Box::new(
        classify_function_output_leaf(type_, externals, customs)?,
    )))
}

fn classify_function_output_leaf(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
) -> syn::Result<FunctionOutputLeafType> {
    if let Some(external) = external_type(type_, externals) {
        return Ok(FunctionOutputLeafType::External {
            payload: external.ident.clone(),
            schema: external.schema.clone(),
        });
    }
    if let Some((index, _)) = custom_output_type_with_index(type_, customs) {
        return Ok(FunctionOutputLeafType::Custom {
            index,
            rust: type_.clone(),
        });
    }
    if let Some((_, custom, input)) = custom_input_model(type_, customs) {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "custom input `{}` cannot be returned; return `{}`",
                input, custom.ident
            ),
        ));
    }
    if is_declared_provider_type(type_)? {
        return Ok(FunctionOutputLeafType::Declared {
            type_: type_.clone(),
            input: DeclaredInput::Owned,
        });
    }
    Ok(FunctionOutputLeafType::Scalar(type_.clone()))
}

fn classify_collection_input_item(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    collection: &str,
) -> syn::Result<StaticValueType> {
    if is_type_application_named(type_, "Value") {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "Value<...> cannot be a {collection} item; wrap the complete generic source shape in Value<...>"
            ),
        ));
    }
    if is_advanced_external(type_)? {
        return Err(syn::Error::new_spanned(
            type_,
            "provider::advanced::External<T> is a call-scoped pass-through and cannot be a List item; use the declared payload input type",
        ));
    }
    if let Type::Reference(reference) = type_ {
        if let Some(external) = external_type(&reference.elem, externals) {
            return Err(syn::Error::new_spanned(
                type_,
                format!(
                    "{collection} item external payload `{}` must be owned",
                    external.ident
                ),
            ));
        }
        return Err(syn::Error::new_spanned(
            type_,
            format!("{collection} items must be owned values"),
        ));
    }
    if is_collection(type_, "List") || is_collection(type_, "Vec") {
        return Err(syn::Error::new_spanned(
            type_,
            "nested List and Vec item values are not supported",
        ));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(StaticValueType::Result {
                success: Box::new(classify_collection_input_item(
                    success, externals, customs, collection,
                )?),
                failure: Box::new(classify_collection_input_item(
                    failure, externals, customs, collection,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(StaticValueType::Option {
                value: Box::new(classify_collection_input_item(
                    value, externals, customs, collection,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Some(external) = external_type(type_, externals) {
        return Ok(StaticValueType::External {
            payload: external.ident.clone(),
            schema: external.schema.clone(),
            store_field: external.store_field.clone(),
        });
    }
    if let Some((index, _, _)) = custom_input_model(type_, customs) {
        return Ok(StaticValueType::Custom { index });
    }
    if let Some(custom) = custom_output_type(type_, customs) {
        let input = if let Some(input) = &custom.input {
            format!("`{}`", input.ident)
        } else {
            "an `input = ...` declaration".to_owned()
        };
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "{collection} item custom output `{}` cannot be used as input; use {input}",
                custom.ident
            ),
        ));
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let elements = tuple
            .elems
            .iter()
            .map(|element| classify_collection_input_item(element, externals, customs, collection))
            .collect::<syn::Result<Vec<_>>>()?;
        return Ok(StaticValueType::Tuple(elements));
    }
    if is_qualified_type_path(type_) {
        return Ok(StaticValueType::Declared {
            type_: type_.clone(),
        });
    }
    Ok(StaticValueType::Scalar(type_.clone()))
}

fn generic_external_type(
    type_: &Type,
    input: bool,
    externals: &[ExternalModel],
    generics: &mut GenericParameterScope,
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<Option<GenericExternalType>> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return Ok(None);
    };
    if path.segments.len() != 1 {
        return Ok(None);
    }
    let segment = &path.segments[0];
    let Some((external, generic)) = externals.iter().find_map(|external| {
        let generic = external.generic.as_ref()?;
        let matches = if input {
            generic.input == segment.ident
        } else {
            external.ident == segment.ident
        };
        matches.then_some((external, generic))
    }) else {
        return Ok(None);
    };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "generic external `{}` requires exactly {} type arguments",
                segment.ident,
                generic.parameters.len(),
            ),
        ));
    };
    let mut classified = Vec::with_capacity(arguments.args.len());
    let mut source_arguments = Vec::with_capacity(arguments.args.len());
    for argument in &arguments.args {
        let GenericArgument::Type(argument) = argument else {
            return Err(syn::Error::new_spanned(
                argument,
                "generic external arguments must be source types",
            ));
        };
        source_arguments.push(argument.clone());
        classified.push(classify_generic_host_type(
            argument, generics, externals, customs, support,
        )?);
    }
    if classified.len() != generic.parameters.len() {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "generic external `{}` requires exactly {} type arguments",
                segment.ident,
                generic.parameters.len(),
            ),
        ));
    }
    Ok(Some(GenericExternalType {
        output: external.ident.clone(),
        input: generic.input.clone(),
        schema: external.schema.clone(),
        storage: generic.storage.clone(),
        source_arguments,
        arguments: classified,
    }))
}

#[cfg(test)]
mod tests {
    use super::{build_external_model, find_unwrapped_generic, retained_parameter_accessor};
    use crate::module::{ExternalArguments, GenericParameterScope, ModuleArguments};
    use quote::quote;
    use syn::{ItemFn, Type};
    #[test]
    fn module_arguments_require_one_path_and_reject_unknown_fields() {
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!())
                .err()
                .expect("missing path should fail")
                .to_string(),
            "missing required module argument `path`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(path = "first", path = "second"))
                .err()
                .expect("duplicate path should fail")
                .to_string(),
            "duplicate module argument `path`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(other = "counter"))
                .err()
                .expect("unknown field should fail")
                .to_string(),
            "unknown module argument `other`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                crate_path = geam_core,
                crate_path = other_core
            ))
            .err()
            .expect("duplicate crate path should fail")
            .to_string(),
            "duplicate module argument `crate_path`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(path = "counter", profile = crate::Profile))
                .err()
                .expect("profile without component should fail")
                .to_string(),
            "module `profile` requires `component`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                stores = crate::counter_stores
            ))
            .err()
            .expect("stores without a built-in profile should fail")
            .to_string(),
            "module `stores` requires `profile` and `component`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                profile = crate::First,
                profile = crate::Second
            ))
            .err()
            .expect("duplicate profile should fail")
            .to_string(),
            "duplicate module argument `profile`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                component = crate::Component<Profile::Io>
            ))
            .err()
            .expect("component without profile should fail")
            .to_string(),
            "module `component` requires `profile`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                component = crate::First,
                component = crate::Second
            ))
            .err()
            .expect("duplicate component should fail")
            .to_string(),
            "duplicate module argument `component`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                stores = crate::first_stores,
                stores = crate::second_stores
            ))
            .err()
            .expect("duplicate stores projection should fail")
            .to_string(),
            "duplicate module argument `stores`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                profile = crate::Profile,
                component = crate::Component<Profile::Io>
            ))
            .err()
            .expect("built-in profile wiring should require an explicit support crate")
            .to_string(),
            "module `profile` and `component` require explicit `crate_path`",
        );
        syn::parse2::<ModuleArguments>(quote!(
            path = "counter",
            crate_path = geam_core,
            profile = crate::Profile,
            component = crate::Component<Profile::Io>,
            stores = crate::counter_stores
        ))
        .expect("explicit built-in profile, component, and stores should parse together");
    }
    #[test]
    fn module_argument_syntax_errors_are_preserved() {
        let cases = [
            (quote!(= "counter"), "expected identifier"),
            (quote!(path "counter"), "expected `=`"),
            (quote!(path = counter), "expected string literal"),
            (
                quote!(path = "counter", crate_path = "geam"),
                "expected identifier",
            ),
            (
                quote!(path = "counter" crate_path = geam_core),
                "expected `,`",
            ),
            (
                quote!(path = "counter", profile =),
                "unexpected end of input, expected identifier",
            ),
            (
                quote!(path = "counter", component =),
                "unexpected end of input, expected one of: `for`, parentheses, `fn`, `unsafe`, `extern`, identifier, `::`, `<`, `dyn`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime",
            ),
            (
                quote!(path = "counter", stores = "counter"),
                "expected identifier",
            ),
        ];

        for (arguments, expected) in cases {
            assert_eq!(
                syn::parse2::<ModuleArguments>(arguments)
                    .err()
                    .expect("module argument syntax should fail")
                    .to_string(),
                expected,
            );
        }
    }
    #[test]
    fn generic_parameter_scan_ignores_shapes_without_declared_parameters() {
        let function: ItemFn = syn::parse_quote! {
            fn identity<Item>() {}
        };
        let generics = GenericParameterScope::new(&function.sig.generics)
            .expect("ordinary type generics should define a scan scope");
        let types: [Type; 4] = [
            syn::parse_quote!(fn()),
            syn::parse_quote!(Option<bool>),
            syn::parse_quote!((bool,)),
            syn::parse_quote!(!),
        ];

        for type_ in types {
            assert_eq!(find_unwrapped_generic(&type_, &generics), None);
        }
    }
    #[test]
    fn external_arguments_require_a_name_and_bare_semantics_flags() {
        let automatic = syn::parse2::<ExternalArguments>(quote!(name = "Metrics"))
            .expect("default semantics should parse");
        assert_eq!(automatic.name.value(), "Metrics");
        assert!(!automatic.manual);
        assert!(!automatic.retained);
        assert!(automatic.parameters.is_empty());
        assert!(automatic.input.is_none());
        assert!(automatic.payload.is_none());

        let manual = syn::parse2::<ExternalArguments>(quote!(manual, name = "Metrics"))
            .expect("manual semantics should parse in either field order");
        assert_eq!(manual.name.value(), "Metrics");
        assert!(manual.manual);
        assert!(!manual.retained);

        let retained = syn::parse2::<ExternalArguments>(quote!(retained, name = "Dynamic"))
            .expect("retained semantics should parse in either field order");
        assert_eq!(retained.name.value(), "Dynamic");
        assert!(!retained.manual);
        assert!(retained.retained);

        let generic = syn::parse2::<ExternalArguments>(quote!(
            name = "Box",
            parameters = [Item, Metadata],
            input = BoxInput,
        ))
        .expect("generic retained arguments should parse");
        assert_eq!(
            generic
                .parameters
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["Item", "Metadata"],
        );
        assert_eq!(generic.input.expect("input should be retained"), "BoxInput");

        let retained = syn::parse2::<ExternalArguments>(quote!(
            name = "Queue",
            parameters = [Item],
            input = QueueInput,
            payload = storage::QueuePayload,
            manual,
        ))
        .expect("advanced retained arguments should parse");
        assert!(retained.manual);
        let retained_payload = retained.payload.expect("payload should be retained");
        assert_eq!(
            quote!(#retained_payload).to_string(),
            "storage :: QueuePayload",
        );

        let cases = [
            (quote!(), "missing required external argument `name`"),
            (quote!(manual), "missing required external argument `name`"),
            (
                quote!(retained),
                "missing required external argument `name`",
            ),
            (
                quote!(name = "First", name = "Second"),
                "duplicate external argument `name`",
            ),
            (
                quote!(name = "Metrics", manual, manual),
                "duplicate external argument `manual`",
            ),
            (
                quote!(name = "Metrics", manual = true),
                "external argument `manual` does not accept a value",
            ),
            (
                quote!(name = "Metrics", manual("custom")),
                "external argument `manual` does not accept a value",
            ),
            (
                quote!(name = "Dynamic", retained, retained),
                "duplicate external argument `retained`",
            ),
            (
                quote!(name = "Dynamic", retained = true),
                "external argument `retained` does not accept a value",
            ),
            (
                quote!(name = "Dynamic", retained("context")),
                "external argument `retained` does not accept a value",
            ),
            (
                quote!(name = "Dynamic", manual, retained),
                "external arguments `manual` and `retained` cannot be combined",
            ),
            (
                quote!(name = "Box", parameters = [Item], parameters = [Other]),
                "duplicate external argument `parameters`",
            ),
            (
                quote!(name = "Box", input = First, input = Second),
                "duplicate external argument `input`",
            ),
            (
                quote!(name = "Box", payload = First, payload = Second),
                "duplicate external argument `payload`",
            ),
            (
                quote!(other = "Metrics"),
                "unknown external argument `other`",
            ),
            (quote!(= "Metrics"), "expected identifier"),
            (quote!(name "Metrics"), "expected `=`"),
            (quote!(name = Metrics), "expected string literal"),
            (quote!(name = "Box", parameters[Item]), "expected `=`"),
            (
                quote!(name = "Box", parameters = Item),
                "expected square brackets",
            ),
            (
                quote!(name = "Box", parameters = ["Item"]),
                "expected identifier",
            ),
            (quote!(name = "Box", input BoxInput), "expected `=`"),
            (
                quote!(name = "Box", input = "BoxInput"),
                "expected identifier",
            ),
            (quote!(name = "Box", payload QueuePayload), "expected `=`"),
            (
                quote!(name = "Box", payload = const),
                "expected one of: `for`, parentheses, `fn`, `unsafe`, `extern`, identifier, `::`, `<`, `dyn`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime",
            ),
            (quote!(name = "Metrics" other = "Value"), "expected `,`"),
        ];

        for (arguments, expected) in cases {
            assert_eq!(
                syn::parse2::<ExternalArguments>(arguments)
                    .err()
                    .expect("external arguments should be rejected")
                    .to_string(),
                expected,
            );
        }
    }
    #[test]
    fn retained_parameter_accessors_follow_rust_snake_case() {
        assert_eq!(
            retained_parameter_accessor(&syn::parse_quote!(Item)),
            "stored_item"
        );
        assert_eq!(
            retained_parameter_accessor(&syn::parse_quote!(EntryValue)),
            "stored_entry_value",
        );
        assert_eq!(
            retained_parameter_accessor(&syn::parse_quote!(URLValue)),
            "stored_url_value",
        );
    }

    #[test]
    fn generic_external_named_field_identity_is_validated_at_the_syn_boundary() {
        let mut payload = syn::parse2::<syn::ItemStruct>(quote! {
            struct BoxValue<Item> {
                #[geam::stored]
                value: Stored<Item>,
            }
        })
        .expect("generic external payload should parse");
        payload
            .fields
            .iter_mut()
            .next()
            .expect("fixture should have one field")
            .ident = None;
        let arguments = syn::parse2::<ExternalArguments>(quote! {
            name = "Box",
            parameters = [Item],
            input = BoxInput,
        })
        .expect("generic external arguments should parse");

        assert_eq!(
            build_external_model(0, &mut payload, arguments, &quote!(geam_core))
                .err()
                .expect("missing named-field identity should fail")
                .to_string(),
            "generic external declarations require named `#[geam::stored]` fields",
        );
    }
}
