use super::function::{
    decode_input, function_codec_bounds, generate_return, provider_construction_bindings,
    provider_requirement_selection_bounds, provider_requirement_sequence,
};
use super::signature::{
    host_argument_type, host_return_type, wrapper_argument_type,
    wrapper_argument_type_with_lifetime,
};
use super::{
    CallAccess, CustomModel, FunctionModel, GeneratedConstruction, GeneratedFunction,
    GeneratedNames, GenericInputSource, InputEnvironment, InputOwnership, OwnedCallAccess,
    ProviderFunction, SourceCompletion,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;

pub(super) fn generate_function_adapter(
    declaration: &ProviderFunction,
    customs: &[CustomModel],
    support: &TokenStream,
    module_external_bounds: &[TokenStream],
    component: &TokenStream,
    module: &syn::LitStr,
) -> GeneratedFunction {
    let (function, flavor, call_is_mutable, source_completion) = match declaration {
        ProviderFunction::Immediate { model, call } => (
            model,
            InputOwnership::Borrowed,
            matches!(call, CallAccess::Mutable),
            SourceCompletion::Ordinary,
        ),
        ProviderFunction::Owned {
            model,
            call,
            completion,
        } => (
            model,
            InputOwnership::Owned,
            matches!(call, OwnedCallAccess::Mutable),
            *completion,
        ),
    };
    let nominal = function.generics.iter().any(|generic| generic.nominal);
    let wrapper_parameters = function
        .generics
        .iter()
        .filter(|_| nominal)
        .map(|generic| {
            let ident = &generic.ident;
            quote!(#ident: #support::ProviderValue + 'static)
        })
        .collect::<Vec<_>>();
    let wrapper_arguments = function
        .generics
        .iter()
        .filter(|_| nominal)
        .map(|generic| {
            let index = generic.index;
            quote!(#support::HostTypeParameter<#index>)
        })
        .collect::<Vec<_>>();
    let ident = &function.ident;
    let wrapper = format_ident!("__geam_host_{}", ident);
    let argument_names = (0..function.arguments.len())
        .map(|index| format_ident!("__geam_argument_{index}"))
        .collect::<Vec<_>>();
    let host_arguments = function
        .arguments
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            !function
                .callable
                .as_ref()
                .is_some_and(|callable| callable.captures.contains(index))
        })
        .map(|(_, argument)| argument)
        .map(|argument| host_argument_type(argument, customs, support))
        .collect::<Vec<_>>();
    let return_type = host_return_type(&function.return_, customs, support);
    let registered_return = if source_completion == SourceCompletion::Work {
        quote!(#support::HostFutureType<#return_type, #support::HostWorkSchema<__GeamProfile>>)
    } else {
        return_type.clone()
    };
    let mut names = GeneratedNames::default();
    let generated_callbacks = super::callback::codecs(function, customs)
        .into_iter()
        .map(|callback| {
            (
                callback.key(),
                callback.generate(&function.generics, customs, support),
            )
        })
        .collect::<::std::collections::BTreeMap<_, _>>();
    let provider = quote!(__GeamProvider);
    let mut generated_return = generate_return(
        &function.return_,
        customs,
        support,
        &provider,
        &return_type,
        &mut names,
    );
    let mut constructions = Vec::new();
    let mut callback_constructions = ::std::collections::BTreeMap::new();
    for callback in super::callback::source_codecs(function, customs) {
        let generated = &generated_callbacks[&callback.key()];
        if generated.has_constructions {
            let binding = names.next("callback_constructions");
            constructions.push(GeneratedConstruction {
                requirement: generated.requirements.clone(),
                binding: binding.clone(),
            });
            callback_constructions.insert(callback.key(), binding);
        }
    }
    constructions.append(&mut generated_return.constructions);
    let factory_offset = constructions.len();
    let capture_mode = super::callable::capture_mode(flavor, support);
    for factory in &function.factories {
        constructions.push(GeneratedConstruction {
            requirement: quote!(<#factory as #support::ProviderFactoryCodec<__GeamProfile, #capture_mode>>::Requirements),
            binding: names.next("factory_constructions"),
        });
    }
    let input_environment = InputEnvironment {
        customs,
        support,
        return_type: &registered_return,
        function_generics: &function.generics,
        generic_source: if nominal {
            GenericInputSource::Declared
        } else {
            GenericInputSource::Instantiated
        },
        flavor,
        callback_constructions: &callback_constructions,
    };
    let decoded_arguments = function
        .arguments
        .iter()
        .zip(&argument_names)
        .map(|(argument, name)| {
            decode_input(argument, quote!(#name), &input_environment, &mut names)
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
    bounds.extend(function_codec_bounds(
        function,
        customs,
        support,
        &return_type,
        &registered_return,
        flavor,
    ));
    bounds.extend(
        generated_callbacks
            .values()
            .flat_map(|callback| callback.bounds.iter().cloned()),
    );
    if source_completion == SourceCompletion::Work {
        bounds.push(quote! {
            __GeamProfile: #support::HostWorkProfile
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
        &constructions[..factory_offset],
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
        call_is_mutable
            || super::syntax::function_contains_future_input(function, customs)
            || super::syntax::function_contains_callback(function, customs)
            || super::syntax::function_uses_contextual_forms(function, customs, support),
    );
    let name = ident.unraw().to_string();
    let host_result_unwrap = function
        .host_result
        .then(|| quote!(let returned = returned?;));
    let mut callback_codecs = generated_callbacks
        .into_values()
        .map(|callback| callback.definition)
        .collect::<Vec<_>>();
    if !function.factories.is_empty() {
        callback_codecs.push(super::callable::bindings_definition(
            function,
            &requirements,
            factory_offset,
            &bounds,
            support,
            flavor,
        ));
    }
    let lifetime = if flavor == InputOwnership::Owned {
        quote!('__geam_runtime)
    } else {
        quote!('__geam_call)
    };
    let captures =
        super::callable::capture_parameters(function, &argument_names, customs, support, &lifetime);
    if let Some(callable) = &function.callable {
        callback_codecs.push(
            super::callable::Declaration {
                function,
                callable,
                customs,
                support,
                component,
                module,
                constructions: &construction_types,
                registered_return: &registered_return,
                bounds: &bounds,
            }
            .generate(),
        );
    }
    let callable_schema = super::callable::schema_type(function, &quote!(__GeamProfile));
    let factory_arguments = function
        .factories
        .iter()
        .map(|factory| quote!(#support::Factory::<#factory>::declaration()))
        .collect::<Vec<_>>();
    let has_factories = !function.factories.is_empty();
    let factory_bindings = super::callable::bindings_type(function, &quote!(__GeamProfile));

    match declaration {
        ProviderFunction::Owned {
            call: call_access,
            completion: source_completion,
            ..
        } => {
            let call_access = *call_access;
            let (completion_type, start, register, call_context) = match source_completion {
                SourceCompletion::Ordinary => (
                    quote!(#support::HostCallContinuation),
                    quote!(resume),
                    quote!(with_resumable_function),
                    quote!(from_execution_context),
                ),
                SourceCompletion::Work => (
                    quote!(#support::HostCallCompletion),
                    quote!(return_future),
                    quote!(with_scoped_function_and_constructions),
                    quote!(from_future_context),
                ),
            };
            let argument_types = function
                .arguments
                .iter()
                .enumerate()
                .filter(|(index, _)| {
                    !function
                        .callable
                        .as_ref()
                        .is_some_and(|callable| callable.captures.contains(index))
                })
                .map(|(_, argument)| argument)
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
                OwnedCallAccess::None => (TokenStream::new(), None),
                OwnedCallAccess::Mutable => (
                    if has_factories {
                        let constructor = match source_completion {
                            SourceCompletion::Ordinary => {
                                quote!(from_execution_context_with_factories)
                            }
                            SourceCompletion::Work => quote!(from_future_context_with_factories),
                        };
                        let context = match source_completion {
                            SourceCompletion::Ordinary => {
                                quote!(#support::ProviderExecutionCall<'_, __GeamProfile, __GeamProvider, (), #factory_bindings>)
                            }
                            SourceCompletion::Work => {
                                quote!(#support::ProviderFutureCall<'_, __GeamProfile, __GeamProvider, #factory_bindings>)
                            }
                        };
                        quote!(let mut __geam_provider_call = #support::Call::<_, #context>::#constructor(__geam_execution_context);)
                    } else {
                        quote! {
                            let mut __geam_provider_call =
                                #support::Call::#call_context(__geam_execution_context);
                        }
                    },
                    Some(quote!(&mut __geam_provider_call)),
                ),
            };
            let call_arguments = call_argument
                .into_iter()
                .chain(factory_arguments.clone())
                .chain(argument_names.iter().map(|argument| quote!(#argument)))
                .collect::<Vec<_>>();
            let wrapper_argument_names =
                super::callable::invocation_names(function, &argument_names);
            let (capture_parameter, capture_setup) = &captures;
            let wrapper_definition = quote! {
                #[allow(clippy::too_many_arguments)]
                fn #wrapper<'__geam_runtime, __GeamProfile, #(#wrapper_parameters,)*>(
                    mut call: #support::HostCall<
                        '__geam_runtime,
                        __GeamProfile,
                        __GeamProvider,
                        #registered_return,
                    >,
                    #capture_parameter
                    __geam_constructions: #support::HostConstructions<
                        '__geam_runtime,
                        #construction_types,
                    >,
                    #(#wrapper_argument_names: #argument_types,)*
                ) -> ::core::result::Result<
                    #completion_type<'__geam_runtime, #registered_return>,
                    #support::HostCallError,
                >
                where
                    __GeamProfile: __GeamModuleProfile,
                    #(#bounds,)*
                {
                    #capture_setup
                    #decoded_arguments
                    ::core::result::Result::Ok(call.#start(__geam_constructions, move |__geam_execution_context| ::std::boxed::Box::pin(async move {
                        #call_setup
                        let returned = #function_path(#(#call_arguments),*).await;
                        #host_result_unwrap
                        ::core::result::Result::Ok(#support::HostOwnedCompletion::<
                            __GeamProfile, __GeamProvider, #return_type, #construction_types,
                        >::new(move |mut call, __geam_constructions| {
                            #construction_setup
                            #return_statements
                            #completion
                        }))
                    })))
                }
            };
            let registration: syn::Stmt = if let Some(schema) = &callable_schema {
                let register = if *source_completion == SourceCompletion::Ordinary {
                    quote!(with_resumable_callable)
                } else {
                    quote!(with_callable)
                };
                syn::parse_quote! {
                    let provider = provider.#register::<__GeamProvider, #schema, (#(#host_arguments,)*), _>(#wrapper::<__GeamProfile, #(#wrapper_arguments,)*>)?;
                }
            } else {
                syn::parse_quote! {
                    let provider = provider.#register::<
                        __GeamProvider,
                        (#(#host_arguments,)*),
                        #registered_return,
                        #construction_types,
                        _,
                    >(#name, #wrapper::<__GeamProfile, #(#wrapper_arguments,)*>)?;
                }
            };
            GeneratedFunction {
                callback_codecs,
                wrapper: wrapper_definition,
                registration: instantiate_nominal_registration(registration, function, support),
                bounds: bounds
                    .into_iter()
                    .map(|bound| {
                        instantiate_nominal_bound(
                            syn::parse_quote!(where #bound),
                            function,
                            support,
                        )
                    })
                    .collect(),
            }
        }
        ProviderFunction::Immediate {
            call: call_access, ..
        } => {
            let call_access = *call_access;
            let argument_types = function
                .arguments
                .iter()
                .enumerate()
                .filter(|(index, _)| {
                    !function
                        .callable
                        .as_ref()
                        .is_some_and(|callable| callable.captures.contains(index))
                })
                .map(|(_, argument)| argument)
                .map(|argument| wrapper_argument_type(argument, customs, support))
                .collect::<Vec<_>>();
            let (call_setup, call_argument, call_recovery) = match call_access {
                CallAccess::None => (TokenStream::new(), None, TokenStream::new()),
                CallAccess::Mutable => (
                    if has_factories {
                        quote! {
                            let mut __geam_provider_call = #support::Call::<_, #support::ProviderActiveCall<'_, __GeamProfile, __GeamProvider, #return_type, #factory_bindings>>::from_host_call_with_factories(call, #support::ProviderConstructions::new(&__geam_constructions));
                        }
                    } else {
                        quote! {
                            let mut __geam_provider_call =
                                #support::Call::from_host_call(call);
                        }
                    },
                    Some(quote!(&mut __geam_provider_call)),
                    quote! {
                        let mut call = __geam_provider_call.into_host_call();
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
                .chain(factory_arguments)
                .chain(decoded_argument_values)
                .collect::<Vec<_>>();
            let construction_parameter = (!constructions.is_empty() || function.callable.is_some())
                .then(|| {
                    quote! {
                        __geam_constructions: #support::HostConstructions<
                            '__geam_call,
                            #construction_types,
                        >,
                    }
                });
            let wrapper_argument_names =
                super::callable::invocation_names(function, &argument_names);
            let (capture_parameter, capture_setup) = &captures;
            let wrapper_definition = quote! {
                #[allow(clippy::too_many_arguments)]
                fn #wrapper<'__geam_call, __GeamProfile, #(#wrapper_parameters,)*>(
                    call: #support::HostCall<
                        '__geam_call,
                        __GeamProfile,
                        __GeamProvider,
                        #return_type,
                    >,
                    #capture_parameter
                    #construction_parameter
                    #(#wrapper_argument_names: #argument_types,)*
                ) -> ::core::result::Result<
                    #support::HostCallCompletion<'__geam_call, #return_type>,
                    #support::HostCallError,
                >
                where
                    __GeamProfile: __GeamModuleProfile,
                    #(#bounds,)*
                {
                    #[allow(unused_mut)]
                    let mut call = call;
                    #capture_setup
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
            let registration: syn::Stmt = if let Some(schema) = &callable_schema {
                syn::parse_quote! {
                    let provider = provider.with_callable::<__GeamProvider, #schema, (#(#host_arguments,)*), _>(#wrapper::<__GeamProfile, #(#wrapper_arguments,)*>)?;
                }
            } else if constructions.is_empty() {
                syn::parse_quote! {
                    let provider = provider.with_scoped_function::<
                        __GeamProvider,
                        (#(#host_arguments,)*),
                        #return_type,
                        _,
                    >(#name, #wrapper::<__GeamProfile, #(#wrapper_arguments,)*>)?;
                }
            } else {
                syn::parse_quote! {
                    let provider = provider.with_scoped_function_and_constructions::<
                        __GeamProvider,
                        (#(#host_arguments,)*),
                        #return_type,
                        #construction_types,
                        _,
                    >(#name, #wrapper::<__GeamProfile, #(#wrapper_arguments,)*>)?;
                }
            };
            GeneratedFunction {
                callback_codecs,
                wrapper: wrapper_definition,
                registration: instantiate_nominal_registration(registration, function, support),
                bounds: bounds
                    .into_iter()
                    .map(|bound| {
                        instantiate_nominal_bound(
                            syn::parse_quote!(where #bound),
                            function,
                            support,
                        )
                    })
                    .collect(),
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
            if generic.nominal {
                let ident = &generic.ident;
                quote!(#ident)
            } else {
                let index = generic.index;
                quote!(#support::HostTypeParameter<#index>)
            }
        })
        .collect::<Vec<_>>();
    match (
        generic_arguments.is_empty(),
        function.profile || call_is_mutable,
    ) {
        (true, true) => quote!(#ident::<__GeamProfile>),
        (true, false) => quote!(#ident),
        (false, true) => quote!(#ident::<#(#generic_arguments,)* __GeamProfile>),
        (false, false) => quote!(#ident::<#(#generic_arguments),*>),
    }
}

// Registration instantiates source types once with scheme markers. Visit type
// positions so a source parameter named Host cannot rewrite an associated Host
// projection, declaration name, field, or generated method identifier.
fn instantiate_nominal_registration(
    mut statement: syn::Stmt,
    function: &FunctionModel,
    support: &TokenStream,
) -> TokenStream {
    if !function.generics.iter().any(|generic| generic.nominal) {
        return quote!(#statement);
    }
    syn::visit_mut::VisitMut::visit_stmt_mut(
        &mut NominalParameters { function, support },
        &mut statement,
    );
    quote!(#statement)
}

fn instantiate_nominal_bound(
    mut bounds: syn::WhereClause,
    function: &FunctionModel,
    support: &TokenStream,
) -> TokenStream {
    if !function.generics.iter().any(|generic| generic.nominal) {
        let predicates = bounds.predicates;
        return quote!(#predicates);
    }
    syn::visit_mut::VisitMut::visit_where_clause_mut(
        &mut NominalParameters { function, support },
        &mut bounds,
    );
    let predicates = bounds.predicates;
    quote!(#predicates)
}

struct NominalParameters<'model> {
    function: &'model FunctionModel,
    support: &'model TokenStream,
}

impl syn::visit_mut::VisitMut for NominalParameters<'_> {
    fn visit_type_mut(&mut self, type_: &mut syn::Type) {
        if let syn::Type::Path(syn::TypePath { qself: None, path }) = type_
            && let Some(ident) = path.get_ident()
            && let Some(generic) = self
                .function
                .generics
                .iter()
                .find(|generic| generic.ident == *ident)
        {
            let index = generic.index;
            let support = self.support;
            *type_ = syn::parse_quote!(#support::HostTypeParameter<#index>);
            return;
        }
        syn::visit_mut::visit_type_mut(self, type_);
    }
}
