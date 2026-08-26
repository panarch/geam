use super::custom_value::{
    CustomConstructorModel, CustomFieldValueType, CustomFields, CustomModel, custom_field_models,
};
use super::list::{list_declared_accesses, list_decoder_value};
use super::signature::{
    callback_codec_type, callback_host_arguments, callback_input_signature_type,
    callback_output_signature_type, function_output_from_root, function_output_host_type,
    generate_generic_external_payload, generic_external_host_type,
    generic_external_input_signature_type, generic_host_type, host_argument_type,
    host_custom_field_type, host_input_type, host_return_type, host_static_value_type,
    host_type_token_sequence, host_value_type, instantiated_generic_source_type,
    provider_value_from_input_root, wrapper_argument_type,
};
use super::{
    CallAccess, CallbackType, DeclaredInput, FunctionArgumentType, FunctionGeneric,
    FunctionInputType, FunctionModel, FunctionOutputLeafType, FunctionOutputValueType,
    FunctionReturnType, FunctionRootOutputValueType, GeneratedCallback, GeneratedConstruction,
    GeneratedFunction, GeneratedNames, GeneratedReturn, GeneratedValue, GenericInputSource,
    InputEnvironment, OutputEnvironment, OutputState, ProviderValueType, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::Ident;
use syn::ext::IdentExt;

fn generate_callback_codec(
    callback: &CallbackType,
    generics: &[FunctionGeneric],
    customs: &[CustomModel],
    support: &TokenStream,
    outer_return: &TokenStream,
) -> GeneratedCallback {
    let codec = &callback.codec;
    let generic_idents = generics
        .iter()
        .map(|generic| generic.ident.clone())
        .collect::<Vec<_>>();
    let codec_type = callback_codec_type(
        codec,
        generic_idents.iter().map(|ident| quote!(#ident)).collect(),
    );
    let host_arguments = callback_host_arguments(callback, customs, support);
    let host_return = host_input_type(&callback.return_, customs, support);
    let rust_arguments = callback
        .arguments
        .iter()
        .map(|argument| callback_output_signature_type(argument, customs, support))
        .collect::<Vec<_>>();
    let rust_return = callback_input_signature_type(&callback.return_, customs, support);
    let argument_names = (0..callback.arguments.len())
        .map(|index| format_ident!("__geam_callback_argument_{index}"))
        .collect::<Vec<_>>();
    let mut names = GeneratedNames::default();
    let mut constructions = Vec::new();
    let environment = OutputEnvironment {
        customs,
        support,
        provider: &quote!(__GeamProvider),
        return_type: outer_return,
    };
    let encoded_arguments = callback
        .arguments
        .iter()
        .zip(&argument_names)
        .map(|(type_, argument)| {
            encode_callback_argument(
                type_,
                quote!(#argument),
                &environment,
                &mut names,
                &mut constructions,
            )
        })
        .collect::<Vec<_>>();
    let argument_statements = encoded_arguments
        .iter()
        .map(|argument| &argument.statements);
    let argument_values = encoded_arguments
        .iter()
        .map(|argument| argument.value.clone())
        .collect::<Vec<_>>();
    let host_values = host_value_sequence(&argument_values);
    let unused_unit = callback
        .arguments
        .is_empty()
        .then(|| quote!(#[allow(clippy::unused_unit)]));
    let requirements = provider_requirement_sequence(&constructions, support);
    let construction_setup = if constructions.is_empty() {
        TokenStream::new()
    } else {
        provider_construction_bindings(&constructions, quote!(constructions), support)
    };
    let input_environment = InputEnvironment {
        customs,
        support,
        return_type: outer_return,
        function_generics: &[],
        generic_source: GenericInputSource::Declared,
    };
    let decoded_return = decode_input(
        &callback.return_,
        quote!(value),
        &input_environment,
        &mut names,
    );
    let return_statements = decoded_return.statements;
    let returned = decoded_return.value;

    let mut bounds = Vec::new();
    for argument in &callback.arguments {
        collect_function_return_bounds(argument, customs, support, outer_return, &mut bounds);
    }
    collect_function_input_type_bounds(
        &callback.return_,
        customs,
        support,
        outer_return,
        &mut bounds,
    );
    if !constructions.is_empty() {
        bounds.push(quote! {
            #requirements: #support::ProviderConstructionRequirements
        });
        bounds.extend(provider_requirement_selection_bounds(
            &requirements,
            &constructions,
            support,
        ));
    }
    let mut bound_keys = BTreeSet::new();
    bounds.retain(|bound| bound_keys.insert(bound.to_string()));

    let codec_definition = if generic_idents.is_empty() {
        quote!(struct #codec;)
    } else {
        quote! {
            struct #codec<#(#generic_idents),*>(
                ::core::marker::PhantomData<fn() -> (#(#generic_idents,)*)>,
            );
        }
    };
    let definition = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #codec_definition

        impl<'__geam_call, #(#generic_idents,)* Profile>
            #support::ProviderCallbackCodec<
                '__geam_call,
                Profile,
                __GeamProvider,
                #outer_return,
            > for #codec_type
        where
            Profile: __GeamModuleProfile,
            #(#bounds,)*
        {
            type HostArguments = #host_arguments;
            type HostReturn = #host_return;
            type Arguments = (#(#rust_arguments,)*);
            type Returned = #rust_return;
            type Requirements = #requirements;

            #unused_unit
            fn into_host_arguments(
                arguments: Self::Arguments,
                mut call: &mut #support::HostCall<
                    '__geam_call,
                    Profile,
                    __GeamProvider,
                    #outer_return,
                >,
                constructions: &#support::ProviderConstructions<
                    '__geam_call,
                    Self::Requirements,
                >,
            ) -> <Self::HostArguments as #support::HostTypeSequence>::Values<'__geam_call> {
                let (#(#argument_names,)*) = arguments;
                #construction_setup
                #(#argument_statements)*
                #host_values
            }

            fn from_host_return(
                value: <Self::HostReturn as #support::HostType>::Value<'__geam_call>,
                mut call: &mut #support::HostCall<
                    '__geam_call,
                    Profile,
                    __GeamProvider,
                    #outer_return,
                >,
            ) -> Self::Returned {
                #return_statements
                #returned
            }
        }
    };

    GeneratedCallback {
        definition,
        requirements,
        has_constructions: !constructions.is_empty(),
        bounds,
    }
}

pub(super) fn generate_function(
    function: &FunctionModel,
    customs: &[CustomModel],
    support: &TokenStream,
) -> GeneratedFunction {
    let FunctionModel {
        ident,
        generics,
        arguments: function_arguments,
        return_,
        call: call_access,
        host_result,
        profile,
    } = function;
    let wrapper = format_ident!("__geam_host_{}", ident.unraw());
    let arguments = (0..function_arguments.len())
        .map(|index| format_ident!("__geam_argument_{index}"))
        .collect::<Vec<_>>();
    let argument_types = function_arguments
        .iter()
        .map(|type_| wrapper_argument_type(type_, customs, support))
        .collect::<Vec<_>>();
    let mut names = GeneratedNames::default();
    let return_type = host_return_type(return_, customs, support);
    let mut codec_bounds = function_codec_bounds(function, customs, support, &return_type);
    let generated_callbacks = function_arguments
        .iter()
        .map(|argument| match argument {
            FunctionArgumentType::Callback(callback) => Some(generate_callback_codec(
                callback,
                generics,
                customs,
                support,
                &return_type,
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    for callback in generated_callbacks.iter().flatten() {
        codec_bounds.extend(callback.bounds.iter().cloned());
    }
    let mut generated_return = generate_return(
        return_,
        customs,
        support,
        &quote!(__GeamProvider),
        &return_type,
        &mut names,
    );
    let mut constructions = Vec::new();
    let mut callback_construction_bindings = Vec::with_capacity(function_arguments.len());
    for callback in &generated_callbacks {
        let binding = callback.as_ref().and_then(|callback| {
            callback.has_constructions.then(|| {
                let binding = names.next("callback_constructions");
                constructions.push(GeneratedConstruction {
                    requirement: callback.requirements.clone(),
                    binding: binding.clone(),
                });
                binding
            })
        });
        callback_construction_bindings.push(binding);
    }
    constructions.append(&mut generated_return.constructions);
    let input_environment = InputEnvironment {
        customs,
        support,
        return_type: &return_type,
        function_generics: generics,
        generic_source: GenericInputSource::Instantiated,
    };
    let decoded_arguments = function_arguments
        .iter()
        .zip(&arguments)
        .zip(&callback_construction_bindings)
        .map(|((type_, argument), callback_constructions)| {
            decode_argument(
                type_,
                quote!(#argument),
                &input_environment,
                &mut names,
                callback_constructions.as_ref(),
            )
        })
        .collect::<Vec<_>>();
    let argument_statements = decoded_arguments
        .iter()
        .map(|argument| &argument.statements);
    let (call_setup, call_argument, call_recovery) = match call_access {
        CallAccess::None => (None, None, None),
        CallAccess::Shared => (
            Some(quote! {
                let __geam_state = &*call.state();
                let __geam_provider_call = #support::Call::from_shared_state(__geam_state);
            }),
            Some(quote!(&__geam_provider_call)),
            None,
        ),
        CallAccess::Mutable => (
            Some(quote! {
                let mut __geam_provider_call = #support::Call::from_host_call(call);
            }),
            Some(quote!(&mut __geam_provider_call)),
            Some(quote! {
                let mut call = __geam_provider_call.into_host_call();
            }),
        ),
    };
    let call_arguments = call_argument
        .into_iter()
        .chain(
            decoded_arguments
                .iter()
                .map(|argument| argument.value.clone()),
        )
        .collect::<Vec<_>>();
    let host_result_unwrap = host_result.then(|| {
        quote! {
            let returned = returned?;
        }
    });
    let mut generic_arguments = Vec::with_capacity(generics.len());
    for generic in generics {
        let index = generic.index;
        generic_arguments.push(quote!(#support::HostTypeParameter<#index>));
    }
    let function = match (generic_arguments.is_empty(), *profile) {
        (true, true) => quote!(#ident::<Profile>),
        (true, false) => quote!(#ident),
        (false, true) => quote!(#ident::<#(#generic_arguments,)* Profile>),
        (false, false) if matches!(call_access, CallAccess::Mutable) => {
            quote!(#ident::<#(#generic_arguments,)* Profile>)
        }
        (false, false) => quote!(#ident::<#(#generic_arguments),*>),
    };
    let return_statements = &generated_return.statements;
    let completion = &generated_return.completion;
    let requirements = provider_requirement_sequence(&constructions, support);
    if !constructions.is_empty() {
        codec_bounds.push(quote! {
            #requirements: #support::ProviderConstructionRequirements
        });
        codec_bounds.extend(provider_requirement_selection_bounds(
            &requirements,
            &constructions,
            support,
        ));
    }
    let construction_parameter = (!constructions.is_empty()).then(|| {
        quote! {
            __geam_constructions: #support::HostConstructions<
                '__geam_call,
                <#requirements as #support::ProviderConstructionRequirements>::Types<
                    #support::HostTypeListEnd,
                >,
            >,
        }
    });
    let construction_setup = (!constructions.is_empty()).then(|| {
        let bindings = provider_construction_bindings(
            &constructions,
            quote!(&__geam_provider_constructions),
            support,
        );
        quote! {
            let __geam_provider_constructions =
                #support::ProviderConstructions::<#requirements>::new(
                    __geam_constructions,
                );
            #bindings
        }
    });
    let wrapper_definition = quote! {
        fn #wrapper<'__geam_call, Profile>(
            call: #support::HostCall<
                '__geam_call,
                Profile,
                __GeamProvider,
                #return_type,
            >,
            #construction_parameter
            #(#arguments: #argument_types,)*
        ) -> ::core::result::Result<
            #support::HostCallCompletion<'__geam_call, #return_type>,
            #support::HostCallError,
        >
        where
            Profile: __GeamModuleProfile,
            #(#codec_bounds,)*
        {
            #[allow(unused_mut)]
            let mut call = call;
            #construction_setup
            #(#argument_statements)*
            #call_setup
            let returned = #function(#(#call_arguments),*);
            #call_recovery
            #host_result_unwrap
            #return_statements
            #completion
        }
    };

    let name = ident.unraw().to_string();
    let host_arguments = function_arguments
        .iter()
        .map(|type_| host_argument_type(type_, customs, support));
    let registration = if constructions.is_empty() {
        quote! {
            let provider = provider.with_scoped_function::<
                __GeamProvider,
                (#(#host_arguments,)*),
                #return_type,
                _,
            >(#name, #wrapper::<Profile>)?;
        }
    } else {
        quote! {
            let provider = provider.with_scoped_function_and_constructions::<
                __GeamProvider,
                (#(#host_arguments,)*),
                #return_type,
                <#requirements as #support::ProviderConstructionRequirements>::Types<
                    #support::HostTypeListEnd,
                >,
                _,
            >(#name, #wrapper::<Profile>)?;
        }
    };

    GeneratedFunction {
        callback_codecs: generated_callbacks
            .into_iter()
            .flatten()
            .map(|callback| callback.definition)
            .collect(),
        wrapper: wrapper_definition,
        registration,
        bounds: codec_bounds,
    }
}

fn function_codec_bounds(
    function: &FunctionModel,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for argument in &function.arguments {
        match argument {
            FunctionArgumentType::Input(input) => collect_function_input_type_bounds(
                input,
                customs,
                support,
                return_type,
                &mut bounds,
            ),
            FunctionArgumentType::Callback(_) => {}
        }
    }
    collect_function_return_bounds(
        &function.return_,
        customs,
        support,
        return_type,
        &mut bounds,
    );

    let mut keys = BTreeSet::new();
    bounds
        .into_iter()
        .filter(|bound| keys.insert(bound.to_string()))
        .collect()
}

fn collect_function_input_bounds(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::Owned,
        } => bounds.push(quote! {
            #type_: #support::ProviderInputValue<Profile, __GeamProvider, #return_type>
        }),
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::BorrowedExternal,
        } => bounds.push(quote! {
            <#type_ as #support::ProviderValue>::Input:
                #support::ProviderInputValue<Profile, __GeamProvider, #return_type>
        }),
        ProviderValueType::Tuple(elements) => {
            for element in elements {
                collect_function_input_bounds(element, customs, support, return_type, bounds);
            }
        }
        ProviderValueType::Result { success, failure } => {
            collect_function_input_bounds(success, customs, support, return_type, bounds);
            collect_function_input_bounds(failure, customs, support, return_type, bounds);
        }
        ProviderValueType::Option { value } => {
            collect_function_input_bounds(value, customs, support, return_type, bounds);
        }
        ProviderValueType::Custom { rust: input, .. } => {
            bounds.push(quote! {
                #input: #support::ProviderInputValue<
                    Profile,
                    __GeamProvider,
                    #return_type,
                >
            });
        }
        ProviderValueType::List(list) => {
            for access in list_declared_accesses(&list.collection.value, customs) {
                let type_ = access.type_;
                bounds.push(quote! {
                    <#type_ as #support::ProviderValue>::ListInput:
                        #support::ProviderListInputCodec<Profile>
                });
            }
        }
        ProviderValueType::Scalar(_)
        | ProviderValueType::Generic(_)
        | ProviderValueType::External { .. } => {}
    }
}

fn collect_function_input_type_bounds(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        FunctionInputType::Value(type_) => {
            let type_ = provider_value_from_input_root(type_);
            collect_function_input_bounds(&type_, customs, support, return_type, bounds);
        }
        FunctionInputType::Generic(_) => {}
        FunctionInputType::External(_) => {}
        FunctionInputType::List(list) => {
            for access in list_declared_accesses(&list.collection.value, customs) {
                let type_ = access.type_;
                bounds.push(quote! {
                    <#type_ as #support::ProviderValue>::ListInput:
                        #support::ProviderListInputCodec<Profile>
                });
            }
        }
    }
}

fn collect_function_return_bounds(
    type_: &FunctionReturnType,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        FunctionReturnType::Value(value) => {
            collect_function_root_output_bounds(value, customs, support, return_type, bounds)
        }
        FunctionReturnType::Generic(_)
        | FunctionReturnType::External(_)
        | FunctionReturnType::List(_) => {}
    }
}

fn collect_function_root_output_bounds(
    type_: &FunctionRootOutputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        FunctionRootOutputValueType::Value(value) => match value.as_ref() {
            FunctionOutputLeafType::Declared { type_, .. } => {
                bounds.push(quote! {
                    #type_: #support::ProviderRootOutputValue<Profile, __GeamProvider>
                });
            }
            FunctionOutputLeafType::Custom { index, .. } => {
                let type_ = &customs[*index].ident;
                bounds.push(quote! {
                    #type_: #support::ProviderRootOutputValue<Profile, __GeamProvider>
                });
            }
            FunctionOutputLeafType::Scalar(_) | FunctionOutputLeafType::External { .. } => {}
        },
        FunctionRootOutputValueType::Tuple(elements) => {
            for element in elements {
                collect_function_output_intermediate_bounds(
                    element,
                    customs,
                    support,
                    return_type,
                    bounds,
                );
            }
        }
        FunctionRootOutputValueType::Result { success, failure } => {
            collect_function_output_intermediate_bounds(
                success,
                customs,
                support,
                return_type,
                bounds,
            );
            collect_function_output_intermediate_bounds(
                failure,
                customs,
                support,
                return_type,
                bounds,
            );
        }
        FunctionRootOutputValueType::Option { value } => {
            collect_function_output_intermediate_bounds(
                value,
                customs,
                support,
                return_type,
                bounds,
            );
        }
        FunctionRootOutputValueType::Vec(collection) => {
            collect_function_output_intermediate_bounds(
                &collection.value,
                customs,
                support,
                return_type,
                bounds,
            );
        }
    }
}

fn collect_function_output_intermediate_bounds(
    type_: &FunctionOutputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        FunctionOutputValueType::Value(value) => match value.as_ref() {
            FunctionOutputLeafType::Declared { type_, .. } => {
                bounds.push(quote! {
                    #type_: #support::ProviderOutputValue<
                        Profile,
                        __GeamProvider,
                        #return_type,
                    >
                });
            }
            FunctionOutputLeafType::Custom { index, .. } => {
                let type_ = &customs[*index].ident;
                bounds.push(quote! {
                    #type_: #support::ProviderOutputValue<
                        Profile,
                        __GeamProvider,
                        #return_type,
                    >
                });
            }
            FunctionOutputLeafType::Scalar(_) | FunctionOutputLeafType::External { .. } => {}
        },
        FunctionOutputValueType::Generic(_) => {}
        FunctionOutputValueType::Tuple(elements) => {
            for element in elements {
                collect_function_output_intermediate_bounds(
                    element,
                    customs,
                    support,
                    return_type,
                    bounds,
                );
            }
        }
        FunctionOutputValueType::Result { success, failure } => {
            collect_function_output_intermediate_bounds(
                success,
                customs,
                support,
                return_type,
                bounds,
            );
            collect_function_output_intermediate_bounds(
                failure,
                customs,
                support,
                return_type,
                bounds,
            );
        }
        FunctionOutputValueType::Option { value } => {
            collect_function_output_intermediate_bounds(
                value,
                customs,
                support,
                return_type,
                bounds,
            );
        }
        FunctionOutputValueType::Vec(collection) => {
            collect_function_output_intermediate_bounds(
                &collection.value,
                customs,
                support,
                return_type,
                bounds,
            );
        }
    }
}

fn decode_argument(
    type_: &FunctionArgumentType,
    input: TokenStream,
    environment: &InputEnvironment<'_>,
    names: &mut GeneratedNames,
    callback_constructions: Option<&Ident>,
) -> GeneratedValue {
    match type_ {
        FunctionArgumentType::Input(type_) => decode_input(type_, input, environment, names),
        FunctionArgumentType::Callback(callback) => {
            let InputEnvironment {
                support,
                return_type,
                function_generics,
                ..
            } = environment;
            let value = names.next("callback");
            let codec = callback_codec_type(
                &callback.codec,
                function_generics
                    .iter()
                    .map(|generic| {
                        let index = generic.index;
                        quote!(#support::HostTypeParameter<#index>)
                    })
                    .collect(),
            );
            let constructions = if let Some(constructions) = callback_constructions {
                quote!(#constructions)
            } else {
                quote!(#support::ProviderConstructions::<
                    #support::ProviderNoConstructions,
                >::none())
            };
            GeneratedValue {
                statements: quote! {
                    let #value = #support::Callback::<
                        _,
                        #support::ProviderCallbackContext<
                            '__geam_call,
                            Profile,
                            __GeamProvider,
                            #return_type,
                            #codec,
                        >,
                    >::from_host(#input, #constructions);
                },
                value: quote!(#value),
            }
        }
    }
}

fn decode_input(
    type_: &FunctionInputType,
    input: TokenStream,
    environment: &InputEnvironment<'_>,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let InputEnvironment {
        customs,
        support,
        return_type,
        generic_source,
        ..
    } = environment;
    match type_ {
        FunctionInputType::Value(type_) => {
            let type_ = provider_value_from_input_root(type_);
            decode_value_argument(&type_, input, customs, support, return_type, names, false)
        }
        FunctionInputType::Generic(value) => {
            let source = match generic_source {
                GenericInputSource::Declared => value.source.clone(),
                GenericInputSource::Instantiated => instantiated_generic_source_type(value),
            };
            let host = generic_host_type(&value.host, customs, support);
            let value = names.next("generic_input");
            GeneratedValue {
                statements: quote! {
                    let #value = #support::Value::<
                        #source,
                        #support::ProviderValueContext<'__geam_call, #host>,
                    >::from_host(#input);
                },
                value: quote!(#value),
            }
        }
        FunctionInputType::External(external) => {
            let value = names.next("external_input");
            let payload = names.next("external_payload");
            let input_type =
                generic_external_input_signature_type(external, customs, support, *generic_source);
            let schema = &external.schema;
            let arguments = external
                .arguments
                .iter()
                .map(|argument| generic_host_type(&argument.host, customs, support))
                .collect::<Vec<_>>();
            let arguments = host_type_token_sequence(&arguments, support);
            GeneratedValue {
                statements: quote! {
                    let #payload = call.provider_external_item_with::<
                        __GeamProvider,
                        #schema,
                        #arguments,
                    >(#input);
                    let #value: #input_type = <#input_type>::__geam_from_host(
                        #support::ProviderExternalInputContext::from_host(#payload),
                    );
                },
                value: quote!(#value),
            }
        }
        FunctionInputType::List(list) => {
            let decoder_value =
                list_decoder_value(&list.decoder, &list.collection.value, customs, support);
            let value = names.next("list");
            GeneratedValue {
                statements: quote! {
                    let #value = call.provider_list(#input, #decoder_value);
                },
                value: quote!(#value),
            }
        }
    }
}

fn decode_value_argument(
    type_: &ProviderValueType,
    input: TokenStream,
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    names: &mut GeneratedNames,
    nested: bool,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        ProviderValueType::Generic(value) => {
            let source = &value.source;
            let host = generic_host_type(&value.host, customs, support);
            let value = names.next("generic_input");
            GeneratedValue {
                statements: quote! {
                    let #value = #support::Value::<
                        #source,
                        #support::ProviderValueContext<'__geam_call, #host>,
                    >::from_host(#input);
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::Owned,
        } => {
            let value = names.next("declared_input");
            GeneratedValue {
                statements: quote! {
                    let #value: #type_ = <#type_ as #support::ProviderInputValue<
                        Profile,
                        __GeamProvider,
                        #return_type,
                    >>::from_host(&mut call, #input);
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::BorrowedExternal,
        } => {
            let value = names.next("declared_external_input");
            GeneratedValue {
                statements: quote! {
                    let #value: <#type_ as #support::ProviderValue>::Input =
                        <<#type_ as #support::ProviderValue>::Input as
                            #support::ProviderInputValue<
                                Profile,
                                __GeamProvider,
                                #return_type,
                            >>::from_host(&mut call, #input);
                },
                value: if nested {
                    quote!(#value)
                } else {
                    quote!(&*#value)
                },
            }
        }
        ProviderValueType::External { schema, .. } => {
            let view = names.next("payload");
            if nested {
                GeneratedValue {
                    statements: quote! {
                        let #view = call.provider_external_item_with::<
                            __GeamProvider,
                            #schema,
                            #support::HostTypeListEnd,
                        >(#input);
                    },
                    value: quote!(#view),
                }
            } else {
                GeneratedValue {
                    statements: quote! {
                        let #view = call.external_payload(#input);
                    },
                    value: quote!(&*#view),
                }
            }
        }
        ProviderValueType::Custom {
            rust: input_type, ..
        } => {
            let value = names.next("custom_input");
            GeneratedValue {
                statements: quote! {
                    let #value = <#input_type as #support::ProviderInputValue<
                        Profile,
                        __GeamProvider,
                        #return_type,
                    >>::from_host(&mut call, #input);
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::List(list) => {
            let decoder =
                list_decoder_value(&list.decoder, &list.collection.value, customs, support);
            let value = names.next("nested_list");
            GeneratedValue {
                statements: quote! {
                    let #value = call.provider_list(#input, #decoder);
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::Tuple(elements) => {
            let host_elements = elements
                .iter()
                .map(|_| names.next("tuple_element"))
                .collect::<Vec<_>>();
            let host_element_tokens = host_elements
                .iter()
                .map(|element| quote!(#element))
                .collect::<Vec<_>>();
            let host_values = host_value_sequence(&host_element_tokens);
            let mut statements = quote! {
                let #host_values = call.tuple_values(#input);
            };
            let decoded = elements
                .iter()
                .zip(host_elements)
                .map(|(element, value)| {
                    decode_value_argument(
                        element,
                        quote!(#value),
                        customs,
                        support,
                        return_type,
                        names,
                        nested,
                    )
                })
                .collect::<Vec<_>>();
            for element in &decoded {
                statements.extend(element.statements.clone());
            }
            let values = decoded.iter().map(|element| &element.value);
            GeneratedValue {
                statements,
                value: quote!((#(#values,)*)),
            }
        }
        ProviderValueType::Result { success, failure } => {
            let success_host = host_value_type(success, customs, support);
            let failure_host = host_value_type(failure, customs, support);
            let success_value = names.next("result_success_host");
            let failure_value = names.next("result_failure_host");
            let decoded_success = decode_value_argument(
                success,
                quote!(#success_value),
                customs,
                support,
                return_type,
                names,
                true,
            );
            let decoded_failure = decode_value_argument(
                failure,
                quote!(#failure_value),
                customs,
                support,
                return_type,
                names,
                true,
            );
            let success_statements = decoded_success.statements;
            let success = decoded_success.value;
            let failure_statements = decoded_failure.statements;
            let failure = decoded_failure.value;
            let value = names.next("result_input");
            GeneratedValue {
                statements: quote! {
                    let #value = if let ::core::option::Option::Some((#success_value, ())) =
                        call.provider_custom_fields::<
                            #support::ProviderOk<#success_host, #failure_host>
                        >(#input)
                    {
                        #success_statements
                        ::core::result::Result::Ok(#success)
                    } else {
                        let (#failure_value, ()) = call.provider_remaining_custom_fields::<
                            #support::ProviderError<#success_host, #failure_host>
                        >(#input);
                        #failure_statements
                        ::core::result::Result::Err(#failure)
                    };
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::Option { value } => {
            let host = host_value_type(value, customs, support);
            let some_host = names.next("option_some_host");
            let decoded = decode_value_argument(
                value,
                quote!(#some_host),
                customs,
                support,
                return_type,
                names,
                true,
            );
            let decoded_statements = decoded.statements;
            let decoded_value = decoded.value;
            let result = names.next("option_input");
            GeneratedValue {
                statements: quote! {
                    let #result = if let ::core::option::Option::Some((#some_host, ())) =
                        call.provider_custom_fields::<#support::ProviderSome<#host>>(#input)
                    {
                        #decoded_statements
                        ::core::option::Option::Some(#decoded_value)
                    } else {
                        ::core::option::Option::None
                    };
                },
                value: quote!(#result),
            }
        }
    }
}

fn generate_return(
    type_: &FunctionReturnType,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    match type_ {
        FunctionReturnType::Generic(_) => GeneratedReturn {
            statements: quote! {
                let returned = returned.into_host();
            },
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        FunctionReturnType::External(external) => {
            let generated =
                generate_generic_external_payload(external, quote!(returned), support, names);
            let statements = generated.statements;
            let payload = generated.value;
            let schema = &external.schema;
            let arguments = external
                .arguments
                .iter()
                .map(|argument| generic_host_type(&argument.host, customs, support))
                .collect::<Vec<_>>();
            let arguments = host_type_token_sequence(&arguments, support);
            GeneratedReturn {
                statements: quote! {
                    #statements
                    let returned = match #payload {
                        ::core::result::Result::Ok(payload) => {
                            call.create_external_with_binding::<#provider>(payload)
                        }
                        ::core::result::Result::Err(value) => {
                            call.provider_external_from_item::<#schema, #arguments, _>(value)
                        }
                    };
                },
                completion: quote! {
                    ::core::result::Result::Ok(call.return_value(returned))
                },
                constructions: Vec::new(),
            }
        }
        FunctionReturnType::Value(type_) => {
            generate_function_value_return(type_, customs, support, provider, return_type, names)
        }
        FunctionReturnType::List(_) => GeneratedReturn {
            statements: quote! {
                let returned = returned.__geam_into_context().into_host();
            },
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
    }
}

fn generate_function_value_return(
    type_: &FunctionRootOutputValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    let mut constructions = Vec::new();
    let environment = OutputEnvironment {
        customs,
        support,
        provider,
        return_type,
    };
    let mut state = OutputState {
        names,
        constructions: &mut constructions,
    };
    match type_ {
        FunctionRootOutputValueType::Value(value) => {
            generate_output_leaf_return(value, customs, support, provider, state.names)
        }
        FunctionRootOutputValueType::Tuple(elements) => {
            let generated = encode_function_output_tuple_elements(
                elements,
                quote!(returned),
                &environment,
                &mut state,
            );
            let values = generated.value;
            GeneratedReturn {
                statements: generated.statements,
                completion: quote! {
                    ::core::result::Result::Ok(call.return_tuple(#values))
                },
                constructions,
            }
        }
        FunctionRootOutputValueType::Result { success, failure } => {
            let success_value = state.names.next("result_success");
            let failure_value = state.names.next("result_failure");
            let success_output = encode_function_output_intermediate(
                success,
                quote!(#success_value),
                &environment,
                &mut state,
            );
            let failure_output = encode_function_output_intermediate(
                failure,
                quote!(#failure_value),
                &environment,
                &mut state,
            );
            let success_statements = success_output.statements;
            let success_output = success_output.value;
            let failure_statements = failure_output.statements;
            let failure_output = failure_output.value;
            let success_host = function_output_host_type(success, customs, support);
            let failure_host = function_output_host_type(failure, customs, support);
            GeneratedReturn {
                statements: TokenStream::new(),
                completion: quote! {
                    ::core::result::Result::Ok(match returned {
                        ::core::result::Result::Ok(#success_value) => {
                            #success_statements
                            call.return_custom::<
                                #support::ProviderOk<#success_host, #failure_host>
                            >((#success_output, ()))
                        }
                        ::core::result::Result::Err(#failure_value) => {
                            #failure_statements
                            call.return_custom::<
                                #support::ProviderError<#success_host, #failure_host>
                            >((#failure_output, ()))
                        }
                    })
                },
                constructions,
            }
        }
        FunctionRootOutputValueType::Option { value } => {
            let some_value = state.names.next("option_some");
            let output = encode_function_output_intermediate(
                value,
                quote!(#some_value),
                &environment,
                &mut state,
            );
            let statements = output.statements;
            let output = output.value;
            let host = function_output_host_type(value, customs, support);
            GeneratedReturn {
                statements: TokenStream::new(),
                completion: quote! {
                    ::core::result::Result::Ok(match returned {
                        ::core::option::Option::Some(#some_value) => {
                            #statements
                            call.return_custom::<#support::ProviderSome<#host>>((#output, ()))
                        }
                        ::core::option::Option::None => {
                            call.return_custom::<#support::ProviderNone<#host>>(())
                        }
                    })
                },
                constructions,
            }
        }
        FunctionRootOutputValueType::Vec(collection) => {
            let item = state.names.next("returned_list_item");
            let generated = encode_function_output_intermediate(
                &collection.value,
                quote!(#item),
                &environment,
                &mut state,
            );
            let statements = generated.statements;
            let value = generated.value;
            let values = state.names.next("returned_list_values");
            GeneratedReturn {
                statements: quote! {
                    let mut #values = ::std::vec::Vec::with_capacity(returned.len());
                    for #item in returned {
                        #statements
                        #values.push(#value);
                    }
                },
                completion: quote! {
                    ::core::result::Result::Ok(call.return_list(#values))
                },
                constructions,
            }
        }
    }
}

fn generate_output_leaf_return(
    type_: &FunctionOutputLeafType,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    match type_ {
        FunctionOutputLeafType::Scalar(_) => GeneratedReturn {
            statements: TokenStream::new(),
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        FunctionOutputLeafType::Declared { type_, .. } => {
            let mut constructions = Vec::new();
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::RootRequirements
            );
            let construction =
                register_provider_requirement(requirement, names, &mut constructions);
            GeneratedReturn {
                statements: TokenStream::new(),
                completion: quote! {
                    <#type_ as #support::ProviderRootOutputValue<
                        Profile,
                        #provider,
                    >>::complete(returned, call, &#construction)
                },
                constructions,
            }
        }
        FunctionOutputLeafType::External { .. } => GeneratedReturn {
            statements: quote! {
                let returned = call.create_external(returned);
            },
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        FunctionOutputLeafType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            let mut constructions = Vec::new();
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::RootRequirements
            );
            let construction =
                register_provider_requirement(requirement, names, &mut constructions);
            GeneratedReturn {
                statements: TokenStream::new(),
                completion: quote! {
                    <#type_ as #support::ProviderRootOutputValue<
                        Profile,
                        #provider,
                    >>::complete(returned, call, &#construction)
                },
                constructions,
            }
        }
    }
}

pub(super) fn generate_custom_return(
    custom_index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    let custom = &customs[custom_index];
    let custom_ident = &custom.ident;
    let mut constructions = Vec::new();
    let environment = OutputEnvironment {
        customs,
        support,
        provider,
        return_type,
    };
    let mut state = OutputState {
        names,
        constructions: &mut constructions,
    };
    let mut arms = Vec::new();
    for constructor in &custom.constructors {
        let (pattern, bindings) = custom_output_pattern(custom_ident, constructor, state.names);
        let fields = custom_field_models(&constructor.fields);
        let mut statements = Vec::with_capacity(fields.len());
        let mut values = Vec::with_capacity(fields.len());
        for (field, binding) in fields.iter().zip(bindings) {
            let encoded =
                encode_custom_field(&field.value, quote!(#binding), &environment, &mut state);
            statements.push(encoded.statements);
            values.push(encoded.value);
        }
        let host_fields = host_value_sequence(&values);
        let marker = &constructor.marker;
        arms.push(quote! {
            #pattern => {
                #(#statements)*
                call.return_custom::<#marker>(#host_fields)
            }
        });
    }
    let completion = state.names.next("custom_completion");
    GeneratedReturn {
        statements: quote! {
            let #completion = match returned {
                #(#arms,)*
            };
        },
        completion: quote! {
            ::core::result::Result::Ok(#completion)
        },
        constructions,
    }
}

pub(super) fn generate_custom_intermediate(
    custom_index: usize,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    let custom = &environment.customs[custom_index];
    let custom_ident = &custom.ident;
    let mut generated_arms = Vec::new();
    for constructor in &custom.constructors {
        let (pattern, bindings) = custom_output_pattern(custom_ident, constructor, state.names);
        let fields = custom_field_models(&constructor.fields);
        let mut statements = Vec::with_capacity(fields.len());
        let mut values = Vec::with_capacity(fields.len());
        for (field, binding) in fields.iter().zip(bindings) {
            let encoded = encode_custom_field(&field.value, quote!(#binding), environment, state);
            statements.push(encoded.statements);
            values.push(encoded.value);
        }
        generated_arms.push((constructor, pattern, statements, values));
    }

    let construction = register_host_construction(
        host_value_type(
            &ProviderValueType::Custom {
                index: custom_index,
                rust: syn::parse_quote!(#custom_ident),
            },
            environment.customs,
            environment.support,
        ),
        environment.support,
        state.names,
        state.constructions,
    );
    let mut arms = Vec::with_capacity(generated_arms.len());
    for (constructor, pattern, statements, values) in generated_arms {
        let fields = host_value_sequence(&values);
        let marker = &constructor.marker;
        arms.push(quote! {
            #pattern => {
                #(#statements)*
                call.construct_custom::<#marker>(
                    #construction.token(),
                    #fields,
                )
            }
        });
    }
    let value = state.names.next("returned_custom");
    GeneratedValue {
        statements: quote! {
            let #value = match #input {
                #(#arms,)*
            };
        },
        value: quote!(#value),
    }
}

fn custom_output_pattern(
    custom: &Ident,
    constructor: &CustomConstructorModel,
    names: &mut GeneratedNames,
) -> (TokenStream, Vec<Ident>) {
    let variant = &constructor.ident;
    match &constructor.fields {
        CustomFields::Unit => (quote!(#custom::#variant), Vec::new()),
        CustomFields::Unnamed(fields) => {
            let mut bindings = Vec::with_capacity(fields.len());
            for _ in fields {
                bindings.push(names.next("custom_field"));
            }
            (quote!(#custom::#variant(#(#bindings),*)), bindings)
        }
        CustomFields::Named(fields) => {
            let mut bindings = Vec::with_capacity(fields.len());
            let mut members = Vec::with_capacity(fields.len());
            for field in fields {
                bindings.push(names.next("custom_field"));
                members.push(&field.ident);
            }
            (
                quote!(#custom::#variant { #(#members: #bindings),* }),
                bindings,
            )
        }
    }
}

fn encode_static_tuple_elements(
    elements: &[StaticValueType],
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    let native_elements = elements
        .iter()
        .map(|_| state.names.next("returned_element"))
        .collect::<Vec<_>>();
    let mut statements = quote! {
        let (#(#native_elements,)*) = #input;
    };
    let encoded = elements
        .iter()
        .zip(native_elements)
        .map(|(element, value)| {
            encode_static_intermediate(element, quote!(#value), environment, state)
        })
        .collect::<Vec<_>>();
    for element in &encoded {
        statements.extend(element.statements.clone());
    }
    let values = encoded
        .iter()
        .map(|element| element.value.clone())
        .collect::<Vec<_>>();
    GeneratedValue {
        statements,
        value: host_value_sequence(&values),
    }
}

fn encode_function_output_tuple_elements(
    elements: &[FunctionOutputValueType],
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    let native_elements = elements
        .iter()
        .map(|_| state.names.next("returned_element"))
        .collect::<Vec<_>>();
    let mut statements = quote! {
        let (#(#native_elements,)*) = #input;
    };
    let encoded = elements
        .iter()
        .zip(native_elements)
        .map(|(element, value)| {
            encode_function_output_intermediate(element, quote!(#value), environment, state)
        })
        .collect::<Vec<_>>();
    for element in &encoded {
        statements.extend(element.statements.clone());
    }
    let values = encoded
        .iter()
        .map(|element| element.value.clone())
        .collect::<Vec<_>>();
    GeneratedValue {
        statements,
        value: host_value_sequence(&values),
    }
}

fn encode_custom_field(
    type_: &CustomFieldValueType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            encode_static_intermediate(type_, input, environment, state)
        }
        CustomFieldValueType::List(list) => {
            let item = state.names.next("returned_custom_list_item");
            let generated = encode_static_intermediate(
                &list.collection.value,
                quote!(#item),
                environment,
                state,
            );
            let statements = generated.statements;
            let item_value = generated.value;
            let values = state.names.next("returned_custom_list_values");
            let construction = register_host_construction(
                host_custom_field_type(type_, environment.customs, environment.support),
                environment.support,
                state.names,
                state.constructions,
            );
            let value = state.names.next("returned_custom_list");
            GeneratedValue {
                statements: quote! {
                    let mut #values = ::std::vec::Vec::with_capacity(#input.len());
                    for #item in #input {
                        #statements
                        #values.push(#item_value);
                    }
                    let #value = call.construct_list(
                        #construction.token(),
                        #values,
                    );
                },
                value: quote!(#value),
            }
        }
    }
}

fn encode_static_intermediate(
    type_: &StaticValueType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    match type_ {
        StaticValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        StaticValueType::Declared { type_, .. } => {
            let support = environment.support;
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::OutputRequirements
            );
            let construction =
                register_provider_requirement(requirement, state.names, state.constructions);
            let value = state.names.next("returned_declared");
            let provider = environment.provider;
            let return_type = environment.return_type;
            GeneratedValue {
                statements: quote! {
                    let #value = <#type_ as #support::ProviderOutputValue<
                        Profile,
                        #provider,
                        #return_type,
                    >>::into_host(#input, &mut call, &#construction);
                },
                value: quote!(#value),
            }
        }
        StaticValueType::External { schema, .. } => {
            let support = environment.support;
            let construction = register_host_construction(
                host_static_value_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let value = state.names.next("returned_external");
            GeneratedValue {
                statements: quote! {
                    let #value = call.construct_external_with_binding::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(
                        #construction.token(),
                        #input,
                    );
                },
                value: quote!(#value),
            }
        }
        StaticValueType::Custom { index, .. } => {
            let type_ = &environment.customs[*index].ident;
            let support = environment.support;
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::OutputRequirements
            );
            let construction =
                register_provider_requirement(requirement, state.names, state.constructions);
            let value = state.names.next("returned_custom");
            let provider = environment.provider;
            let return_type = environment.return_type;
            GeneratedValue {
                statements: quote! {
                    let #value = <#type_ as #support::ProviderOutputValue<
                        Profile,
                        #provider,
                        #return_type,
                    >>::into_host(#input, &mut call, &#construction);
                },
                value: quote!(#value),
            }
        }
        StaticValueType::Tuple(elements) => {
            let mut generated = encode_static_tuple_elements(elements, input, environment, state);
            let support = environment.support;
            let construction = register_host_construction(
                host_static_value_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let value = state.names.next("returned_tuple");
            let elements = generated.value;
            generated.statements.extend(quote! {
                let #value = call.construct_tuple(
                    #construction.token(),
                    #elements,
                );
            });
            generated.value = quote!(#value);
            generated
        }
        StaticValueType::Result { success, failure } => {
            let success_value = state.names.next("result_success");
            let failure_value = state.names.next("result_failure");
            let success_output =
                encode_static_intermediate(success, quote!(#success_value), environment, state);
            let failure_output =
                encode_static_intermediate(failure, quote!(#failure_value), environment, state);
            let success_statements = success_output.statements;
            let success_output = success_output.value;
            let failure_statements = failure_output.statements;
            let failure_output = failure_output.value;
            let support = environment.support;
            let success_host = host_static_value_type(success, environment.customs, support);
            let failure_host = host_static_value_type(failure, environment.customs, support);
            let construction = register_host_construction(
                host_static_value_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let output = state.names.next("returned_result");
            GeneratedValue {
                statements: quote! {
                    let #output = match #input {
                        ::core::result::Result::Ok(#success_value) => {
                            #success_statements
                            call.construct_custom::<
                                #support::ProviderOk<#success_host, #failure_host>
                            >(#construction.token(), (#success_output, ()))
                        }
                        ::core::result::Result::Err(#failure_value) => {
                            #failure_statements
                            call.construct_custom::<
                                #support::ProviderError<#success_host, #failure_host>
                            >(#construction.token(), (#failure_output, ()))
                        }
                    };
                },
                value: quote!(#output),
            }
        }
        StaticValueType::Option { value } => {
            let some_value = state.names.next("option_some");
            let encoded =
                encode_static_intermediate(value, quote!(#some_value), environment, state);
            let statements = encoded.statements;
            let encoded = encoded.value;
            let support = environment.support;
            let host = host_static_value_type(value, environment.customs, support);
            let construction = register_host_construction(
                host_static_value_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let output = state.names.next("returned_option");
            GeneratedValue {
                statements: quote! {
                    let #output = match #input {
                        ::core::option::Option::Some(#some_value) => {
                            #statements
                            call.construct_custom::<#support::ProviderSome<#host>>(
                                #construction.token(),
                                (#encoded, ()),
                            )
                        }
                        ::core::option::Option::None => {
                            call.construct_custom::<#support::ProviderNone<#host>>(
                                #construction.token(),
                                (),
                            )
                        }
                    };
                },
                value: quote!(#output),
            }
        }
    }
}

fn encode_function_output_intermediate(
    type_: &FunctionOutputValueType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    match type_ {
        FunctionOutputValueType::Value(value) => {
            encode_function_output_leaf_intermediate(value, input, environment, state)
        }
        FunctionOutputValueType::Generic(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_host()),
        },
        FunctionOutputValueType::Tuple(elements) => {
            let mut generated =
                encode_function_output_tuple_elements(elements, input, environment, state);
            let support = environment.support;
            let construction = register_host_construction(
                function_output_host_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let value = state.names.next("returned_tuple");
            let elements = generated.value;
            generated.statements.extend(quote! {
                let #value = call.construct_tuple(
                    #construction.token(),
                    #elements,
                );
            });
            generated.value = quote!(#value);
            generated
        }
        FunctionOutputValueType::Result { success, failure } => {
            let success_value = state.names.next("result_success");
            let failure_value = state.names.next("result_failure");
            let success_output = encode_function_output_intermediate(
                success,
                quote!(#success_value),
                environment,
                state,
            );
            let failure_output = encode_function_output_intermediate(
                failure,
                quote!(#failure_value),
                environment,
                state,
            );
            let success_statements = success_output.statements;
            let success_output = success_output.value;
            let failure_statements = failure_output.statements;
            let failure_output = failure_output.value;
            let support = environment.support;
            let success_host = function_output_host_type(success, environment.customs, support);
            let failure_host = function_output_host_type(failure, environment.customs, support);
            let construction = register_host_construction(
                function_output_host_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let output = state.names.next("returned_result");
            GeneratedValue {
                statements: quote! {
                    let #output = match #input {
                        ::core::result::Result::Ok(#success_value) => {
                            #success_statements
                            call.construct_custom::<
                                #support::ProviderOk<#success_host, #failure_host>
                            >(#construction.token(), (#success_output, ()))
                        }
                        ::core::result::Result::Err(#failure_value) => {
                            #failure_statements
                            call.construct_custom::<
                                #support::ProviderError<#success_host, #failure_host>
                            >(#construction.token(), (#failure_output, ()))
                        }
                    };
                },
                value: quote!(#output),
            }
        }
        FunctionOutputValueType::Option { value } => {
            let some_value = state.names.next("option_some");
            let encoded =
                encode_function_output_intermediate(value, quote!(#some_value), environment, state);
            let statements = encoded.statements;
            let encoded = encoded.value;
            let support = environment.support;
            let host = function_output_host_type(value, environment.customs, support);
            let construction = register_host_construction(
                function_output_host_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let output = state.names.next("returned_option");
            GeneratedValue {
                statements: quote! {
                    let #output = match #input {
                        ::core::option::Option::Some(#some_value) => {
                            #statements
                            call.construct_custom::<#support::ProviderSome<#host>>(
                                #construction.token(),
                                (#encoded, ()),
                            )
                        }
                        ::core::option::Option::None => {
                            call.construct_custom::<#support::ProviderNone<#host>>(
                                #construction.token(),
                                (),
                            )
                        }
                    };
                },
                value: quote!(#output),
            }
        }
        FunctionOutputValueType::Vec(collection) => {
            let item = state.names.next("returned_list_item");
            let generated = encode_function_output_intermediate(
                &collection.value,
                quote!(#item),
                environment,
                state,
            );
            let item_statements = generated.statements;
            let item_value = generated.value;
            let values = state.names.next("returned_list_values");
            let support = environment.support;
            let construction = register_host_construction(
                function_output_host_type(type_, environment.customs, support),
                support,
                state.names,
                state.constructions,
            );
            let value = state.names.next("returned_list");
            GeneratedValue {
                statements: quote! {
                    let mut #values = ::std::vec::Vec::with_capacity(#input.len());
                    for #item in #input {
                        #item_statements
                        #values.push(#item_value);
                    }
                    let #value = call.construct_list(
                        #construction.token(),
                        #values,
                    );
                },
                value: quote!(#value),
            }
        }
    }
}

fn encode_function_output_leaf_intermediate(
    type_: &FunctionOutputLeafType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    match type_ {
        FunctionOutputLeafType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        FunctionOutputLeafType::Declared { type_, .. } => {
            let support = environment.support;
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::OutputRequirements
            );
            let construction =
                register_provider_requirement(requirement, state.names, state.constructions);
            let value = state.names.next("returned_declared");
            let provider = environment.provider;
            let return_type = environment.return_type;
            GeneratedValue {
                statements: quote! {
                    let #value = <#type_ as #support::ProviderOutputValue<
                        Profile,
                        #provider,
                        #return_type,
                    >>::into_host(#input, &mut call, &#construction);
                },
                value: quote!(#value),
            }
        }
        FunctionOutputLeafType::External { schema, .. } => {
            let support = environment.support;
            let host = quote!(#support::HostExternalType<#schema>);
            let construction =
                register_host_construction(host, support, state.names, state.constructions);
            let value = state.names.next("returned_external");
            GeneratedValue {
                statements: quote! {
                    let #value = call.construct_external_with_binding::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(
                        #construction.token(),
                        #input,
                    );
                },
                value: quote!(#value),
            }
        }
        FunctionOutputLeafType::Custom { index, .. } => {
            let type_ = &environment.customs[*index].ident;
            let support = environment.support;
            let requirement = quote!(
                <#type_ as #support::ProviderValue>::OutputRequirements
            );
            let construction =
                register_provider_requirement(requirement, state.names, state.constructions);
            let value = state.names.next("returned_custom");
            let provider = environment.provider;
            let return_type = environment.return_type;
            GeneratedValue {
                statements: quote! {
                    let #value = <#type_ as #support::ProviderOutputValue<
                        Profile,
                        #provider,
                        #return_type,
                    >>::into_host(#input, &mut call, &#construction);
                },
                value: quote!(#value),
            }
        }
    }
}

fn encode_callback_argument(
    type_: &FunctionReturnType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    names: &mut GeneratedNames,
    constructions: &mut Vec<GeneratedConstruction>,
) -> GeneratedValue {
    match type_ {
        FunctionReturnType::Generic(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_host()),
        },
        FunctionReturnType::External(external) => {
            let generated =
                generate_generic_external_payload(external, input, environment.support, names);
            let statements = generated.statements;
            let payload = generated.value;
            let host =
                generic_external_host_type(external, environment.customs, environment.support);
            let construction =
                register_host_construction(host, environment.support, names, constructions);
            let value = names.next("returned_external");
            let provider = environment.provider;
            let schema = &external.schema;
            let arguments = external
                .arguments
                .iter()
                .map(|argument| {
                    generic_host_type(&argument.host, environment.customs, environment.support)
                })
                .collect::<Vec<_>>();
            let arguments = host_type_token_sequence(&arguments, environment.support);
            GeneratedValue {
                statements: quote! {
                    #statements
                    let #value = match #payload {
                        ::core::result::Result::Ok(payload) => {
                            call.construct_external_with_binding::<
                                #provider,
                                #schema,
                                #arguments,
                            >(#construction.token(), payload)
                        }
                        ::core::result::Result::Err(value) => {
                            call.provider_external_from_item::<#schema, #arguments, _>(value)
                        }
                    };
                },
                value: quote!(#value),
            }
        }
        FunctionReturnType::List(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.__geam_into_context().into_host()),
        },
        FunctionReturnType::Value(value) => {
            let mut state = OutputState {
                names,
                constructions,
            };
            let value = function_output_from_root(value);
            encode_function_output_intermediate(&value, input, environment, &mut state)
        }
    }
}

pub(super) fn host_value_sequence(values: &[TokenStream]) -> TokenStream {
    values
        .iter()
        .rev()
        .fold(quote!(()), |tail, head| quote!((#head, #tail)))
}

fn register_host_construction(
    type_: TokenStream,
    support: &TokenStream,
    names: &mut GeneratedNames,
    constructions: &mut Vec<GeneratedConstruction>,
) -> Ident {
    register_provider_requirement(
        quote!(#support::ProviderConstruction<#type_>),
        names,
        constructions,
    )
}

fn register_provider_requirement(
    requirement: TokenStream,
    names: &mut GeneratedNames,
    constructions: &mut Vec<GeneratedConstruction>,
) -> Ident {
    let binding = names.next("construction");
    constructions.push(GeneratedConstruction {
        requirement,
        binding: binding.clone(),
    });
    binding
}

pub(super) fn provider_requirement_sequence(
    constructions: &[GeneratedConstruction],
    support: &TokenStream,
) -> TokenStream {
    constructions.iter().rev().fold(
        quote!(#support::ProviderNoConstructions),
        |tail, construction| {
            let head = &construction.requirement;
            quote!(#support::ProviderConstructionList<#head, #tail>)
        },
    )
}

pub(super) fn provider_construction_bindings(
    constructions: &[GeneratedConstruction],
    requirements: TokenStream,
    support: &TokenStream,
) -> TokenStream {
    let mut statements = TokenStream::new();
    for (index, construction) in constructions.iter().enumerate() {
        let binding = &construction.binding;
        let index = provider_construction_index(index, support);
        statements.extend(quote! {
            let #binding = #support::ProviderConstructions::select::<#index>(#requirements);
        });
    }
    statements
}

pub(super) fn provider_requirement_selection_bounds(
    requirements: &TokenStream,
    constructions: &[GeneratedConstruction],
    support: &TokenStream,
) -> Vec<TokenStream> {
    constructions
        .iter()
        .enumerate()
        .map(|(index, construction)| {
            let index = provider_construction_index(index, support);
            let requirement = &construction.requirement;
            quote! {
                #requirements:
                    #support::ProviderConstructionRequirementAt<
                        #index,
                        Requirement = #requirement,
                    >
            }
        })
        .collect()
}

fn provider_construction_index(index: usize, support: &TokenStream) -> TokenStream {
    (0..index).fold(
        quote!(#support::ProviderConstructionIndex0),
        |index, _| quote!(#support::ProviderConstructionIndexNext<#index>),
    )
}
