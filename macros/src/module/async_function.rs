use super::function::{
    decode_argument, generate_callback_codec, generate_return, provider_construction_bindings,
    provider_requirement_selection_bounds, provider_requirement_sequence,
    transfer_function_codec_bounds,
};
use super::signature::{
    host_argument_type, host_return_type, wrapper_argument_type,
    wrapper_argument_type_with_lifetime,
};
use super::{
    AsyncCallAccess, CallAccess, CustomModel, FunctionArgumentType, FunctionModel,
    GeneratedConstruction, GeneratedNames, GeneratedTransferFunction, GenericInputSource,
    InputEnvironment, ProviderRepresentation, TransferFlavor, TransferFunction,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(super) fn generate_transfer_function(
    transfer: &TransferFunction,
    customs: &[CustomModel],
    support: &TokenStream,
    module_external_bounds: &[TokenStream],
) -> GeneratedTransferFunction {
    let (function, flavor, call_is_mutable) = match transfer {
        TransferFunction::Immediate { model, call } => (
            model,
            TransferFlavor::Immediate,
            matches!(call, CallAccess::Mutable),
        ),
        TransferFunction::Async { model, call } => (
            model,
            TransferFlavor::Async,
            matches!(call, AsyncCallAccess::Mutable),
        ),
    };
    let ident = &function.ident;
    let wrapper = format_ident!("__geam_transfer_host_{}", ident);
    let is_async = matches!(flavor, TransferFlavor::Async);
    let function_flavor = flavor.function();
    let argument_names = (0..function.arguments.len())
        .map(|index| format_ident!("__geam_argument_{index}"))
        .collect::<Vec<_>>();
    let host_arguments = function
        .arguments
        .iter()
        .map(|argument| host_argument_type(argument, customs, support))
        .collect::<Vec<_>>();
    let return_type = host_return_type(&function.return_, customs, support);
    let registered_return = if is_async {
        quote!(#support::HostFutureType<#return_type, #support::HostWorkSchema<Profile>>)
    } else {
        return_type.clone()
    };
    let callback_return = if is_async {
        quote!(())
    } else {
        return_type.clone()
    };
    let mut names = GeneratedNames::default();
    let generated_callbacks = function
        .arguments
        .iter()
        .map(|argument| match argument {
            FunctionArgumentType::Callback(callback) => Some(generate_callback_codec(
                callback,
                &function.generics,
                customs,
                support,
                &callback_return,
                function_flavor,
                is_async,
            )),
            FunctionArgumentType::Input(_) => None,
        })
        .collect::<Vec<_>>();
    let provider = quote!(__GeamAsyncProvider);
    let mut generated_return = generate_return(
        &function.return_,
        customs,
        support,
        &provider,
        &return_type,
        &mut names,
        ProviderRepresentation::Transfer,
    );
    let mut constructions = Vec::new();
    let mut callback_construction_bindings = Vec::with_capacity(function.arguments.len());
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
        return_type: &registered_return,
        function_generics: &function.generics,
        generic_source: GenericInputSource::Instantiated,
        flavor: function_flavor,
    };
    let decoded_arguments = function
        .arguments
        .iter()
        .zip(&argument_names)
        .zip(&callback_construction_bindings)
        .map(|((argument, name), callback_constructions)| {
            decode_argument(
                argument,
                quote!(#name),
                &input_environment,
                &mut names,
                callback_constructions.as_ref(),
            )
        })
        .collect::<Vec<_>>();
    let argument_statements = decoded_arguments
        .iter()
        .map(|argument| &argument.statements)
        .collect::<Vec<_>>();
    let decoded_argument_values = decoded_arguments
        .iter()
        .map(|argument| argument.value.clone())
        .collect::<Vec<_>>();
    let requirements = provider_requirement_sequence(&constructions, support);
    let construction_types = quote! {
        <#requirements as #support::ProviderConstructionRequirements>::Types<
            #support::HostTypeListEnd,
        >
    };
    let mut bounds = module_external_bounds.to_vec();
    bounds.extend(transfer_function_codec_bounds(
        function,
        customs,
        support,
        &return_type,
        &registered_return,
        flavor,
    ));
    bounds.extend(
        generated_callbacks
            .iter()
            .flatten()
            .flat_map(|callback| callback.bounds.iter().cloned()),
    );
    if is_async && generated_callbacks.iter().any(Option::is_some) {
        bounds.push(quote! {
            Profile::ExternalStores: ::core::marker::Send
        });
    }
    if is_async {
        bounds.push(quote! {
            Profile: #support::HostWorkProfile
        });
    }
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
    let construction_bindings = provider_construction_bindings(
        &constructions,
        quote!(&__geam_provider_constructions),
        support,
    );
    let construction_setup = (!constructions.is_empty()).then(|| {
        quote! {
            let __geam_provider_constructions =
                #support::ProviderConstructions::<#requirements>::new(
                    &__geam_constructions,
                );
            #construction_bindings
        }
    });
    let return_statements = std::mem::take(&mut generated_return.statements);
    let completion = generated_return.completion;
    let function_path = function_path(
        function,
        support,
        call_is_mutable || super::syntax::function_contains_future_input(function),
    );
    let name = ident.to_string();
    let host_result_unwrap = function
        .host_result
        .then(|| quote!(let returned = returned?;));
    let callback_codecs = generated_callbacks
        .into_iter()
        .flatten()
        .map(|callback| callback.definition)
        .collect::<Vec<_>>();

    match transfer {
        TransferFunction::Async {
            call: call_access, ..
        } => {
            let call_access = *call_access;
            let argument_types = function
                .arguments
                .iter()
                .map(|argument| {
                    wrapper_argument_type_with_lifetime(
                        argument,
                        customs,
                        support,
                        &quote!('__geam_runtime),
                    )
                })
                .collect::<Vec<_>>();
            let decoded_arguments = if argument_names.is_empty() {
                TokenStream::new()
            } else {
                let decoded_tuple = quote!((#(#decoded_argument_values,)*));
                let decoded_pattern = quote!((#(#argument_names,)*));
                quote! {
                    let #decoded_pattern = {
                        #construction_setup
                        #(#argument_statements)*
                        #decoded_tuple
                    };
                }
            };
            let (call_setup, call_argument) = match call_access {
                AsyncCallAccess::None => (TokenStream::new(), None),
                AsyncCallAccess::Mutable => (
                    quote! {
                        let mut __geam_provider_call =
                            #support::Call::from_future_context(__geam_future_context);
                    },
                    Some(quote!(&mut __geam_provider_call)),
                ),
            };
            let call_arguments = call_argument
                .into_iter()
                .chain(argument_names.iter().map(|argument| quote!(#argument)))
                .collect::<Vec<_>>();
            let wrapper_definition = quote! {
                #[allow(clippy::too_many_arguments)]
                fn #wrapper<'__geam_runtime, Profile>(
                    mut call: #support::TransferHostCall<
                        '__geam_runtime,
                        Profile,
                        __GeamAsyncProvider,
                        #registered_return,
                    >,
                    __geam_constructions: #support::HostConstructions<
                        '__geam_runtime,
                        #construction_types,
                    >,
                    #(#argument_names: #argument_types,)*
                ) -> ::core::result::Result<
                    #support::HostCallCompletion<'__geam_runtime, #registered_return>,
                    #support::AsyncHostCallError,
                >
                where
                    Profile: __GeamAsyncModuleProfile,
                    Profile::RunState: ::core::marker::Send,
                    #(#bounds,)*
                {
                    #decoded_arguments
                    ::core::result::Result::Ok(call.return_future(__geam_constructions, move |__geam_future_context| ::std::boxed::Box::pin(async move {
                        #call_setup
                        let returned = #function_path(#(#call_arguments),*).await;
                        #host_result_unwrap
                        ::core::result::Result::Ok(#support::HostFutureCompletion::<
                            Profile, __GeamAsyncProvider, #return_type, #construction_types,
                        >::new(move |mut call, __geam_constructions| {
                            #construction_setup
                            #return_statements
                            #completion
                        }))
                    })))
                }
            };
            let registration = quote! {
                let provider = provider.with_scoped_function_and_constructions::<
                    __GeamAsyncProvider,
                    (#(#host_arguments,)*),
                    #registered_return,
                    #construction_types,
                    _,
                >(#name, #wrapper::<Profile>)?;
            };
            GeneratedTransferFunction {
                callback_codecs,
                wrapper: wrapper_definition,
                registration,
                bounds,
            }
        }
        TransferFunction::Immediate {
            call: call_access, ..
        } => {
            let call_access = *call_access;
            let argument_types = function
                .arguments
                .iter()
                .map(|argument| wrapper_argument_type(argument, customs, support))
                .collect::<Vec<_>>();
            let (call_setup, call_argument, call_recovery) = match call_access {
                CallAccess::None => (TokenStream::new(), None, TokenStream::new()),
                CallAccess::Mutable => (
                    quote! {
                        let mut __geam_provider_call =
                            #support::Call::from_transfer_host_call(call);
                    },
                    Some(quote!(&mut __geam_provider_call)),
                    quote! {
                        let mut call = __geam_provider_call.into_transfer_host_call();
                    },
                ),
                CallAccess::Shared => (
                    quote! {
                        let __geam_state = &*call.state();
                        let __geam_provider_call =
                            #support::Call::from_shared_state(__geam_state);
                    },
                    Some(quote!(&__geam_provider_call)),
                    TokenStream::new(),
                ),
            };
            let call_arguments = call_argument
                .into_iter()
                .chain(decoded_argument_values)
                .collect::<Vec<_>>();
            let construction_parameter = (!constructions.is_empty()).then(|| {
                quote! {
                    __geam_constructions: #support::HostConstructions<
                        '__geam_call,
                        #construction_types,
                    >,
                }
            });
            let wrapper_definition = quote! {
                #[allow(clippy::too_many_arguments)]
                fn #wrapper<'__geam_call, Profile>(
                    call: #support::TransferHostCall<
                        '__geam_call,
                        Profile,
                        __GeamAsyncProvider,
                        #return_type,
                    >,
                    #construction_parameter
                    #(#argument_names: #argument_types,)*
                ) -> ::core::result::Result<
                    #support::HostCallCompletion<'__geam_call, #return_type>,
                    #support::AsyncHostCallError,
                >
                where
                    Profile: __GeamAsyncModuleProfile,
                    #(#bounds,)*
                {
                    #[allow(unused_mut)]
                    let mut call = call;
                    #construction_setup
                    #(#argument_statements)*
                    #call_setup
                    let returned = #function_path(#(#call_arguments),*);
                    #call_recovery
                    #host_result_unwrap
                    #return_statements
                    #completion
                }
            };
            let registration = if constructions.is_empty() {
                quote! {
                    let provider = provider.with_scoped_function::<
                        __GeamAsyncProvider,
                        (#(#host_arguments,)*),
                        #return_type,
                        _,
                    >(#name, #wrapper::<Profile>)?;
                }
            } else {
                quote! {
                    let provider = provider.with_scoped_function_and_constructions::<
                        __GeamAsyncProvider,
                        (#(#host_arguments,)*),
                        #return_type,
                        #construction_types,
                        _,
                    >(#name, #wrapper::<Profile>)?;
                }
            };
            GeneratedTransferFunction {
                callback_codecs,
                wrapper: wrapper_definition,
                registration,
                bounds,
            }
        }
    }
}

fn function_path(
    function: &FunctionModel,
    support: &TokenStream,
    call_is_mutable: bool,
) -> TokenStream {
    let ident = &function.ident;
    let generic_arguments = function
        .generics
        .iter()
        .map(|generic| {
            let index = generic.index;
            quote!(#support::HostTypeParameter<#index>)
        })
        .collect::<Vec<_>>();
    match (
        generic_arguments.is_empty(),
        function.profile || call_is_mutable,
    ) {
        (true, true) => quote!(#ident::<Profile>),
        (true, false) => quote!(#ident),
        (false, true) => quote!(#ident::<#(#generic_arguments,)* Profile>),
        (false, false) => quote!(#ident::<#(#generic_arguments),*>),
    }
}
