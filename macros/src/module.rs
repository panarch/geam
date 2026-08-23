use crate::path::support_path;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, FnArg, GenericArgument, GenericParam, Ident, Item, ItemFn, ItemMod, ItemStruct,
    LitStr, Meta, Path, PathArguments, ReturnType, Token, Type, TypePath,
};

struct ModuleArguments {
    path: LitStr,
    crate_path: Option<Path>,
}

#[derive(Default)]
struct PartialModuleArguments {
    path: Option<LitStr>,
    crate_path: Option<Path>,
}

struct ExternalArguments {
    name: LitStr,
    manual: bool,
}

#[derive(Default)]
struct PartialExternalArguments {
    name: Option<LitStr>,
    manual: Option<Ident>,
}

enum ExternalSemantics {
    Default,
    Manual,
}

struct ExternalModel {
    ident: Ident,
    name: LitStr,
    semantics: ExternalSemantics,
    schema: Ident,
    storage: Ident,
    store_field: Ident,
}

#[derive(Clone)]
enum ProviderValueType {
    Scalar(Type),
    External {
        payload: Ident,
        schema: Ident,
        store_field: Ident,
    },
    Tuple(Vec<ProviderValueType>),
}

#[derive(Clone)]
struct CollectionType {
    source: Type,
    item: Type,
    value: ProviderValueType,
}

struct ListType {
    collection: CollectionType,
    decoder: Ident,
}

enum FunctionArgumentType {
    Value(Box<ProviderValueType>),
    List(Box<ListType>),
}

enum FunctionReturnType {
    Value(ProviderValueType),
    List(ListType),
    Vec(CollectionType),
}

enum StateAccess {
    None,
    Shared,
    Mutable,
}

struct FunctionModel {
    ident: Ident,
    arguments: Vec<FunctionArgumentType>,
    return_: FunctionReturnType,
    state: StateAccess,
}

struct ListDecoderModel {
    ident: Ident,
    item: Type,
    value: ProviderValueType,
    key: String,
}

struct ListExternalAccess {
    payload: Ident,
    schema: Ident,
    field: Ident,
}

struct GeneratedFunction {
    wrapper: TokenStream,
    registration: TokenStream,
}

struct GeneratedValue {
    statements: TokenStream,
    value: TokenStream,
}

struct GeneratedReturn {
    statements: TokenStream,
    completion: TokenStream,
    constructions: Vec<TokenStream>,
}

#[derive(Default)]
struct GeneratedNames {
    next: usize,
}

impl GeneratedNames {
    fn next(&mut self, role: &str) -> Ident {
        let index = self.next;
        self.next += 1;
        format_ident!("__geam_{role}_{index}")
    }
}

pub(crate) fn expand(arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let arguments = syn::parse2::<ModuleArguments>(arguments)?;
    let mut module = syn::parse2::<ItemMod>(item)?;
    let support = support_path(arguments.crate_path.as_ref())?;
    let inline_module_error =
        syn::Error::new_spanned(&module, "`#[geam::module]` requires an inline Rust module");
    let (_, items) = module.content.as_mut().ok_or(inline_module_error)?;
    let mut externals = Vec::new();
    let mut source_names = BTreeSet::new();
    for item in items.iter_mut() {
        let Item::Struct(payload) = item else {
            continue;
        };
        let Some(external) = take_external_marker(&mut payload.attrs)? else {
            continue;
        };
        validate_external(payload)?;
        if !source_names.insert(external.name.value()) {
            return Err(syn::Error::new(
                external.name.span(),
                format!("duplicate external source type `{}`", external.name.value()),
            ));
        }
        let index = externals.len();
        externals.push(ExternalModel {
            ident: payload.ident.clone(),
            name: external.name,
            semantics: if external.manual {
                ExternalSemantics::Manual
            } else {
                ExternalSemantics::Default
            },
            schema: format_ident!("__GeamExternalSchema{index}"),
            storage: format_ident!("__GeamExternalStorage{index}"),
            store_field: format_ident!("__geam_external_{index}"),
        });
    }

    let mut functions = Vec::new();
    let mut list_decoders = Vec::new();
    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        if take_marker(&mut function.attrs, "function")? {
            let model = validate_function(function, &externals, &mut list_decoders, &support)?;
            functions.push(model);
        }
    }

    let module_path = arguments.path;
    let module_ident = &module.ident;
    let payload_semantics = externals.iter().filter_map(|external| {
        if matches!(external.semantics, ExternalSemantics::Manual) {
            return None;
        }
        let payload = &external.ident;
        let source_name = &external.name;
        Some(quote! {
            impl #support::ExternalPayload for #payload {
                fn source_equal(&self, other: &Self) -> bool {
                    <Self as ::core::cmp::PartialEq>::eq(self, other)
                }

                fn source_hash(&self) -> u64 {
                    #support::external_payload_hash(self)
                }

                fn inspect(&self) -> ::ecow::EcoString {
                    ::ecow::EcoString::from(::core::concat!(#source_name, "(<opaque>)"))
                }
            }
        })
    });
    let store_fields = externals.iter().map(|external| {
        let payload = &external.ident;
        let field = &external.store_field;
        quote! {
            #field: #support::HostExternalStore<#payload>,
        }
    });
    let schemas = externals.iter().map(|external| {
        let payload = &external.ident;
        let source_name = &external.name;
        let schema = &external.schema;
        let storage = &external.storage;
        let store_field = &external.store_field;
        quote! {
            struct #schema;

            impl #support::HostExternalSchema for #schema {
                const PACKAGE: &'static str =
                    <super::Component as #support::ProviderPackage>::PACKAGE;
                const MODULE: &'static str = #module_path;
                const NAME: &'static str = #source_name;
                const PARAMETER_COUNT: usize = 0;
            }

            struct #storage;

            impl<Profile> #support::HostExternalStorage<Profile, #schema> for #storage
            where
                Profile: #support::HostComponentProfile<super::Component>,
            {
                type Payload = #payload;

                fn store(
                    stores: &Profile::ExternalStores,
                ) -> &#support::HostExternalStore<Self::Payload> {
                    &<Profile as #support::HostComponentProfile<super::Component>>::component_stores(
                        stores,
                    ).#module_ident.#store_field
                }

                fn source_equal(
                    _context: &#support::HostExternalEquality<'_>,
                    left: &Self::Payload,
                    right: &Self::Payload,
                ) -> bool {
                    <#payload as #support::ExternalPayload>::source_equal(left, right)
                }

                fn source_hash(
                    _context: &#support::HostExternalHashing<'_>,
                    value: &Self::Payload,
                ) -> u64 {
                    <#payload as #support::ExternalPayload>::source_hash(value)
                }

                fn inspect(
                    _context: &#support::HostExternalInspection<'_>,
                    value: &Self::Payload,
                ) -> ::ecow::EcoString {
                    <#payload as #support::ExternalPayload>::inspect(value)
                }
            }
        }
    });
    let bindings = externals.iter().map(|external| {
        let schema = &external.schema;
        let storage = &external.storage;
        quote! {
            impl<Profile> #support::HostExternalBinding<Profile, #schema> for __GeamProvider
            where
                Profile: #support::HostComponentProfile<super::Component>,
            {
                type Storage = #storage;
            }
        }
    });
    let generated_list_decoders = list_decoders
        .iter()
        .map(|decoder| generate_list_decoder(decoder, &support))
        .collect::<Vec<_>>();
    let generated_functions = functions
        .iter()
        .map(|function| generate_function(function, &support))
        .collect::<Vec<_>>();
    let wrappers = generated_functions.iter().map(|function| &function.wrapper);
    let registrations = generated_functions
        .iter()
        .map(|function| &function.registration);
    let external_registrations = externals.iter().map(|external| {
        let schema = &external.schema;
        quote! {
            let provider = provider.with_external_type::<__GeamProvider, #schema>()?;
        }
    });

    items.push(Item::Verbatim(quote! {
        #[doc(hidden)]
        #[derive(Default)]
        pub(super) struct __GeamStores {
            #(#store_fields)*
        }
    }));
    for semantics in payload_semantics {
        items.push(Item::Verbatim(semantics));
    }
    for schema in schemas {
        items.push(Item::Verbatim(schema));
    }
    items.push(Item::Verbatim(quote! {
        struct __GeamProvider;
    }));
    items.push(Item::Verbatim(quote! {
        impl<Profile> #support::HostProvider<Profile> for __GeamProvider
        where
            Profile: #support::HostComponentProfile<super::Component>,
        {
            type State = <super::Component as #support::HostProviderComponent>::RunState;

            fn project(state: &mut Profile::RunState) -> &mut Self::State {
                <Profile as #support::HostComponentProfile<super::Component>>::component_state(
                    state,
                )
            }
        }
    }));
    for binding in bindings {
        items.push(Item::Verbatim(binding));
    }
    for decoder in generated_list_decoders {
        items.push(Item::Verbatim(decoder));
    }
    for wrapper in wrappers {
        items.push(Item::Verbatim(wrapper.clone()));
    }
    let registrar = quote! {
        pub(super) fn __geam_provider_module<Profile>() -> ::core::result::Result<
            #support::HostProviderModule<Profile>,
            #support::HostRegistrationError,
        >
        where
            Profile: #support::HostComponentProfile<super::Component>,
        {
            let provider = #support::HostProviderModule::<Profile>::new(
                <super::Component as #support::ProviderPackage>::PACKAGE,
                #module_path,
            )?;
            #(#external_registrations)*
            #(#registrations)*
            ::core::result::Result::Ok(provider)
        }
    };
    items.push(Item::Verbatim(registrar));

    Ok(quote!(#module))
}

fn generate_function(function: &FunctionModel, support: &TokenStream) -> GeneratedFunction {
    let FunctionModel {
        ident,
        arguments: function_arguments,
        return_,
        state,
    } = function;
    let wrapper = format_ident!("__geam_host_{}", ident.unraw());
    let arguments = (0..function_arguments.len())
        .map(|index| format_ident!("__geam_argument_{index}"))
        .collect::<Vec<_>>();
    let argument_types = function_arguments
        .iter()
        .map(|type_| wrapper_argument_type(type_, support))
        .collect::<Vec<_>>();
    let mut names = GeneratedNames::default();
    let decoded_arguments = function_arguments
        .iter()
        .zip(&arguments)
        .map(|(type_, argument)| decode_argument(type_, quote!(#argument), &mut names))
        .collect::<Vec<_>>();
    let argument_statements = decoded_arguments
        .iter()
        .map(|argument| &argument.statements);
    let (state_projection, state_argument) = match state {
        StateAccess::None => (None, None),
        StateAccess::Shared => (
            Some(quote! {
                let __geam_state = &*call.state();
            }),
            Some(quote!(__geam_state)),
        ),
        StateAccess::Mutable => (
            Some(quote! {
                let __geam_state = call.state();
            }),
            Some(quote!(__geam_state)),
        ),
    };
    let call_arguments = state_argument
        .into_iter()
        .chain(
            decoded_arguments
                .iter()
                .map(|argument| argument.value.clone()),
        )
        .collect::<Vec<_>>();
    let return_type = host_return_type(return_, support);
    let generated_return = generate_return(return_, support, &mut names);
    let return_statements = &generated_return.statements;
    let completion = &generated_return.completion;
    let construction_parameter = (!generated_return.constructions.is_empty()).then(|| {
        let constructions = host_type_token_sequence(&generated_return.constructions, support);
        quote! {
            __geam_constructions: #support::HostConstructions<
                '__geam_call,
                #constructions,
            >,
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
            Profile: #support::HostComponentProfile<super::Component>,
        {
            #[allow(unused_mut)]
            let mut call = call;
            #(#argument_statements)*
            #state_projection
            let returned = #ident(#(#call_arguments),*);
            #return_statements
            #completion
        }
    };

    let name = ident.unraw().to_string();
    let host_arguments = function_arguments
        .iter()
        .map(|type_| host_argument_type(type_, support));
    let registration = if generated_return.constructions.is_empty() {
        quote! {
            let provider = provider.with_scoped_function::<
                __GeamProvider,
                (#(#host_arguments,)*),
                #return_type,
                _,
            >(#name, #wrapper::<Profile>)?;
        }
    } else {
        let constructions = host_type_token_sequence(&generated_return.constructions, support);
        quote! {
            let provider = provider.with_scoped_function_and_constructions::<
                __GeamProvider,
                (#(#host_arguments,)*),
                #return_type,
                #constructions,
                _,
            >(#name, #wrapper::<Profile>)?;
        }
    };

    GeneratedFunction {
        wrapper: wrapper_definition,
        registration,
    }
}

fn decode_argument(
    type_: &FunctionArgumentType,
    input: TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        FunctionArgumentType::Value(type_) => decode_value_argument(type_, input, names),
        FunctionArgumentType::List(list) => {
            let decoder_value = list_decoder_value(&list.decoder, &list.collection.value);
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
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        ProviderValueType::External { .. } => {
            let view = names.next("payload");
            GeneratedValue {
                statements: quote! {
                    let #view = call.external_payload(#input);
                },
                value: quote!(&*#view),
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
                .map(|(element, value)| decode_value_argument(element, quote!(#value), names))
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
    }
}

fn generate_return(
    type_: &FunctionReturnType,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    match type_ {
        FunctionReturnType::Value(type_) => generate_value_return(type_, support, names),
        FunctionReturnType::List(_) => GeneratedReturn {
            statements: quote! {
                let returned = returned.__geam_into_context().into_host();
            },
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        FunctionReturnType::Vec(list) => {
            let mut constructions = Vec::new();
            let item = names.next("returned_list_item");
            let generated = encode_intermediate(
                &list.value,
                quote!(#item),
                support,
                names,
                &mut constructions,
            );
            let statements = generated.statements;
            let value = generated.value;
            let values = names.next("returned_list_values");
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

fn generate_value_return(
    type_: &ProviderValueType,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedReturn {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedReturn {
            statements: TokenStream::new(),
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        ProviderValueType::External { .. } => GeneratedReturn {
            statements: quote! {
                let returned = call.create_external(returned);
            },
            completion: quote! {
                ::core::result::Result::Ok(call.return_value(returned))
            },
            constructions: Vec::new(),
        },
        ProviderValueType::Tuple(elements) => {
            let mut constructions = Vec::new();
            let generated = encode_tuple_elements(
                elements,
                quote!(returned),
                support,
                names,
                &mut constructions,
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
    }
}

fn encode_tuple_elements(
    elements: &[ProviderValueType],
    input: TokenStream,
    support: &TokenStream,
    names: &mut GeneratedNames,
    constructions: &mut Vec<TokenStream>,
) -> GeneratedValue {
    let native_elements = elements
        .iter()
        .map(|_| names.next("returned_element"))
        .collect::<Vec<_>>();
    let mut statements = quote! {
        let (#(#native_elements,)*) = #input;
    };
    let encoded = elements
        .iter()
        .zip(native_elements)
        .map(|(element, value)| {
            encode_intermediate(element, quote!(#value), support, names, constructions)
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

fn encode_intermediate(
    type_: &ProviderValueType,
    input: TokenStream,
    support: &TokenStream,
    names: &mut GeneratedNames,
    constructions: &mut Vec<TokenStream>,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        ProviderValueType::External { .. } => {
            let index = host_index(constructions.len(), support);
            constructions.push(host_value_type(type_, support));
            let value = names.next("returned_external");
            GeneratedValue {
                statements: quote! {
                    let #value = call.construct_external(
                        __geam_constructions.at::<#index>(),
                        #input,
                    );
                },
                value: quote!(#value),
            }
        }
        ProviderValueType::Tuple(elements) => {
            let mut generated =
                encode_tuple_elements(elements, input, support, names, constructions);
            let index = host_index(constructions.len(), support);
            constructions.push(host_value_type(type_, support));
            let value = names.next("returned_tuple");
            let elements = generated.value;
            generated.statements.extend(quote! {
                let #value = call.construct_tuple(
                    __geam_constructions.at::<#index>(),
                    #elements,
                );
            });
            generated.value = quote!(#value);
            generated
        }
    }
}

fn host_argument_type(type_: &FunctionArgumentType, support: &TokenStream) -> TokenStream {
    match type_ {
        FunctionArgumentType::Value(type_) => host_value_type(type_, support),
        FunctionArgumentType::List(list) => {
            let item = host_value_type(&list.collection.value, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

fn host_return_type(type_: &FunctionReturnType, support: &TokenStream) -> TokenStream {
    match type_ {
        FunctionReturnType::Value(type_) => host_value_type(type_, support),
        FunctionReturnType::List(list) => {
            let item = host_value_type(&list.collection.value, support);
            quote!(#support::HostListType<#item>)
        }
        FunctionReturnType::Vec(list) => {
            let item = host_value_type(&list.value, support);
            quote!(#support::HostListType<#item>)
        }
    }
}

fn host_value_type(type_: &ProviderValueType, support: &TokenStream) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::External { schema, .. } => {
            quote!(#support::HostExternalType<#schema>)
        }
        ProviderValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, support);
            quote!(#support::HostTupleType<#elements>)
        }
    }
}

fn wrapper_argument_type(type_: &FunctionArgumentType, support: &TokenStream) -> TokenStream {
    match type_ {
        FunctionArgumentType::Value(type_) => wrapper_value_type(type_, support),
        FunctionArgumentType::List(list) => {
            let item = host_value_type(&list.collection.value, support);
            quote!(#support::HostList<'__geam_call, #item>)
        }
    }
}

fn wrapper_value_type(type_: &ProviderValueType, support: &TokenStream) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::External { schema, .. } => {
            quote!(#support::HostExternal<'__geam_call, #support::HostExternalType<#schema>>)
        }
        ProviderValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, support);
            quote!(#support::HostTuple<'__geam_call, #elements>)
        }
    }
}

fn host_value_type_sequence(elements: &[ProviderValueType], support: &TokenStream) -> TokenStream {
    let elements = elements
        .iter()
        .map(|element| host_value_type(element, support))
        .collect::<Vec<_>>();
    host_type_token_sequence(&elements, support)
}

fn host_type_token_sequence(elements: &[TokenStream], support: &TokenStream) -> TokenStream {
    elements.iter().rev().fold(
        quote!(#support::HostTypeListEnd),
        |tail, head| quote!(#support::HostTypeList<#head, #tail>),
    )
}

fn host_value_sequence(values: &[TokenStream]) -> TokenStream {
    values
        .iter()
        .rev()
        .fold(quote!(()), |tail, head| quote!((#head, #tail)))
}

fn host_index(index: usize, support: &TokenStream) -> TokenStream {
    (0..index).fold(
        quote!(#support::HostTypeIndex0),
        |index, _| quote!(#support::HostTypeIndexNext<#index>),
    )
}

fn register_list_decoder(list: &CollectionType, decoders: &mut Vec<ListDecoderModel>) -> Ident {
    let key = provider_value_key(&list.value);
    if let Some(decoder) = decoders
        .iter()
        .find(|decoder: &&ListDecoderModel| decoder.key == key)
    {
        return decoder.ident.clone();
    }
    let index = decoders.len();
    let ident = format_ident!("__GeamListDecoder{index}");
    decoders.push(ListDecoderModel {
        ident: ident.clone(),
        item: list.item.clone(),
        value: list.value.clone(),
        key,
    });
    ident
}

fn validate_list_return(function: &FunctionModel) -> syn::Result<()> {
    if let FunctionReturnType::List(returned) = &function.return_
        && !function.arguments.iter().any(|argument| {
            matches!(
                argument,
                FunctionArgumentType::List(argument)
                    if provider_value_key(&argument.collection.value)
                        == provider_value_key(&returned.collection.value)
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

fn provider_value_key(type_: &ProviderValueType) -> String {
    match type_ {
        ProviderValueType::Scalar(type_) => format!("scalar:{}", quote!(#type_)),
        ProviderValueType::External { schema, .. } => format!("external:{schema}"),
        ProviderValueType::Tuple(elements) => format!(
            "tuple:({})",
            elements
                .iter()
                .map(provider_value_key)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn generate_list_decoder(decoder: &ListDecoderModel, support: &TokenStream) -> TokenStream {
    let ident = &decoder.ident;
    let item = &decoder.item;
    let accesses = list_external_accesses(&decoder.value);
    let fields = accesses.iter().map(|access| {
        let field = &access.field;
        let payload = &access.payload;
        quote!(#field: #support::ProviderExternalPayloadAccess<#payload>,)
    });
    let definition = if accesses.is_empty() {
        quote!(struct #ident;)
    } else {
        quote! {
            struct #ident {
                #(#fields)*
            }
        }
    };
    let mut names = GeneratedNames::default();
    let decoded = decode_list_item(&decoder.value, quote!(__geam_value), &mut names);
    let statements = decoded.statements;
    let value = decoded.value;
    let view = list_item_view_type(&decoder.value, support);
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

fn list_decoder_value(ident: &Ident, value: &ProviderValueType) -> TokenStream {
    let accesses = list_external_accesses(value);
    if accesses.is_empty() {
        quote!(#ident)
    } else {
        let fields = accesses.iter().map(|access| {
            let field = &access.field;
            let schema = &access.schema;
            quote!(#field: call.provider_external_payload_access::<#schema>(),)
        });
        quote! {
            #ident {
                #(#fields)*
            }
        }
    }
}

fn list_external_accesses(type_: &ProviderValueType) -> Vec<ListExternalAccess> {
    fn collect(type_: &ProviderValueType, accesses: &mut Vec<ListExternalAccess>) {
        match type_ {
            ProviderValueType::Scalar(_) => {}
            ProviderValueType::External {
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
            ProviderValueType::Tuple(elements) => {
                for element in elements {
                    collect(element, accesses);
                }
            }
        }
    }

    let mut accesses = Vec::new();
    collect(type_, &mut accesses);
    accesses
}

fn decode_list_item(
    type_: &ProviderValueType,
    input: TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(type_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_scalar::<#type_>()),
        },
        ProviderValueType::External { store_field, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_external(&self.#store_field)),
        },
        ProviderValueType::Tuple(elements) => {
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
                let generated = decode_list_item(element, quote!(#host), names);
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
    }
}

fn list_item_view_type(type_: &ProviderValueType, support: &TokenStream) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::External { payload, .. } => {
            quote!(#support::ProviderExternalItem<#payload>)
        }
        ProviderValueType::Tuple(elements) => {
            let elements = elements
                .iter()
                .map(|element| list_item_view_type(element, support));
            quote!((#(#elements,)*))
        }
    }
}

fn list_signature_type(list: &ListType, support: &TokenStream) -> Type {
    let item = &list.collection.item;
    let host_item = host_value_type(&list.collection.value, support);
    let decoder = &list.decoder;
    syn::parse_quote! {
        #support::List<
            #item,
            #support::ProviderListContext<'__geam_list, #host_item, #decoder>,
        >
    }
}

impl Parse for ModuleArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut partial = PartialModuleArguments::default();
        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            match field.to_string().as_str() {
                "path" => set_once(&mut partial.path, input.parse()?, &field)?,
                "crate_path" => set_once(&mut partial.crate_path, input.parse()?, &field)?,
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
        Ok(Self {
            path,
            crate_path: partial.crate_path,
        })
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
        Ok(Self {
            name,
            manual: partial.manual.is_some(),
        })
    }
}

fn set_once<Value>(slot: &mut Option<Value>, value: Value, field: &Ident) -> syn::Result<()> {
    if slot.replace(value).is_some() {
        return Err(syn::Error::new(
            field.span(),
            format!("duplicate module argument `{field}`"),
        ));
    }
    Ok(())
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

fn take_external_marker(attributes: &mut Vec<Attribute>) -> syn::Result<Option<ExternalArguments>> {
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

fn is_marker(attribute: &Attribute, name: &str) -> bool {
    attribute
        .path()
        .segments
        .last()
        .is_some_and(|segment| segment.ident == name)
}

fn validate_external(payload: &ItemStruct) -> syn::Result<()> {
    if !payload.generics.params.is_empty() || payload.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &payload.generics,
            "external payload structs must not have generics",
        ));
    }
    Ok(())
}

fn validate_function(
    function: &mut ItemFn,
    externals: &[ExternalModel],
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
    if !function.sig.generics.params.is_empty() || function.sig.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "provider functions must not have generics",
        ));
    }
    let ReturnType::Type(_, return_type) = &function.sig.output else {
        return Err(syn::Error::new_spanned(
            &function.sig.output,
            "provider functions require an explicit return type",
        ));
    };
    let rust_return_type = (**return_type).clone();
    if matches!(&rust_return_type, Type::Tuple(tuple) if tuple.elems.is_empty()) {
        function
            .attrs
            .push(syn::parse_quote!(#[allow(clippy::unused_unit)]));
    }
    let return_ = classify_return(&rust_return_type, externals, list_decoders)?;

    let mut state = StateAccess::None;
    let mut arguments = Vec::new();
    let mut has_list = false;
    for (index, argument) in function.sig.inputs.iter_mut().enumerate() {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(
                argument,
                "provider functions must be free functions",
            ));
        };
        let is_state = take_marker(&mut argument.attrs, "state")?;
        if is_state {
            if index != 0 {
                return Err(syn::Error::new_spanned(
                    argument,
                    "the `#[geam::state]` parameter must be first",
                ));
            }
            let Type::Reference(reference) = argument.ty.as_ref() else {
                return Err(syn::Error::new_spanned(
                    &argument.ty,
                    "the `#[geam::state]` parameter must be `&State` or `&mut State`",
                ));
            };
            state = if reference.mutability.is_some() {
                StateAccess::Mutable
            } else {
                StateAccess::Shared
            };
        } else {
            let type_ = classify_argument(&argument.ty, externals, list_decoders)?;
            if let FunctionArgumentType::List(list) = &type_ {
                *argument.ty = list_signature_type(list, support);
                has_list = true;
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

    let model = FunctionModel {
        ident: function.sig.ident.clone(),
        arguments,
        return_,
        state,
    };
    validate_list_return(&model)?;
    if let FunctionReturnType::List(list) = &model.return_ {
        function.sig.output = ReturnType::Type(
            Token![->](proc_macro2::Span::call_site()),
            Box::new(list_signature_type(list, support)),
        );
        has_list = true;
    }
    if has_list {
        function
            .sig
            .generics
            .params
            .push(GenericParam::Lifetime(syn::parse_quote!('__geam_list)));
    }
    Ok(model)
}

fn classify_argument(
    type_: &Type,
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<FunctionArgumentType> {
    if let Type::Reference(reference) = type_ {
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
            return Ok(FunctionArgumentType::Value(Box::new(
                ProviderValueType::External {
                    payload: external.ident.clone(),
                    schema: external.schema.clone(),
                    store_field: external.store_field.clone(),
                },
            )));
        }
        if is_non_empty_tuple(&reference.elem) {
            return Err(syn::Error::new_spanned(
                type_,
                "tuple arguments must be passed by value",
            ));
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source arguments may borrow only declared external payloads",
        ));
    }
    if let Some(item) = collection_item(type_, "List")? {
        let value = classify_collection_item(&item, externals, "List")?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(FunctionArgumentType::List(Box::new(ListType {
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
    Ok(FunctionArgumentType::Value(Box::new(
        classify_argument_value(type_, externals)?,
    )))
}

fn classify_argument_value(
    type_: &Type,
    externals: &[ExternalModel],
) -> syn::Result<ProviderValueType> {
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
                store_field: external.store_field.clone(),
            });
        }
        return Err(syn::Error::new_spanned(
            type_,
            "provider source arguments may borrow only declared external payloads",
        ));
    }
    if is_collection(type_, "List") {
        return Err(syn::Error::new_spanned(
            type_,
            "geam::List<T> is supported only as a top-level source argument",
        ));
    }
    if is_collection(type_, "Vec") {
        return Err(syn::Error::new_spanned(
            type_,
            "Vec<T> arguments are not supported; use geam::List<T>",
        ));
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
        let elements = tuple
            .elems
            .iter()
            .map(|element| classify_argument_value(element, externals))
            .collect::<syn::Result<Vec<_>>>()?;
        return Ok(ProviderValueType::Tuple(elements));
    }
    Ok(ProviderValueType::Scalar(type_.clone()))
}

fn classify_return(
    type_: &Type,
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<FunctionReturnType> {
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
        let value = classify_collection_item(&item, externals, "List")?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(FunctionReturnType::List(ListType {
            collection,
            decoder,
        }));
    }
    if let Some(item) = collection_item(type_, "Vec")? {
        let value = classify_collection_item(&item, externals, "Vec")?;
        return Ok(FunctionReturnType::Vec(CollectionType {
            source: type_.clone(),
            item,
            value,
        }));
    }
    Ok(FunctionReturnType::Value(classify_return_value(
        type_, externals,
    )?))
}

fn classify_return_value(
    type_: &Type,
    externals: &[ExternalModel],
) -> syn::Result<ProviderValueType> {
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
    if is_collection(type_, "List") || is_collection(type_, "Vec") {
        return Err(syn::Error::new_spanned(
            type_,
            "List and Vec values are supported only as top-level source returns",
        ));
    }
    if let Some(external) = external_type(type_, externals) {
        return Ok(ProviderValueType::External {
            payload: external.ident.clone(),
            schema: external.schema.clone(),
            store_field: external.store_field.clone(),
        });
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let elements = tuple
            .elems
            .iter()
            .map(|element| classify_return_value(element, externals))
            .collect::<syn::Result<Vec<_>>>()?;
        return Ok(ProviderValueType::Tuple(elements));
    }
    Ok(ProviderValueType::Scalar(type_.clone()))
}

fn classify_collection_item(
    type_: &Type,
    externals: &[ExternalModel],
    collection: &str,
) -> syn::Result<ProviderValueType> {
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
    if let Some(external) = external_type(type_, externals) {
        return Ok(ProviderValueType::External {
            payload: external.ident.clone(),
            schema: external.schema.clone(),
            store_field: external.store_field.clone(),
        });
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let elements = tuple
            .elems
            .iter()
            .map(|element| classify_collection_item(element, externals, collection))
            .collect::<syn::Result<Vec<_>>>()?;
        return Ok(ProviderValueType::Tuple(elements));
    }
    Ok(ProviderValueType::Scalar(type_.clone()))
}

fn collection_item(type_: &Type, name: &str) -> syn::Result<Option<Type>> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return Ok(None);
    };
    let Some(segment) = path.segments.last().filter(|segment| segment.ident == name) else {
        return Ok(None);
    };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Err(syn::Error::new_spanned(
            type_,
            format!("{name} requires exactly one type argument"),
        ));
    };
    let Some(GenericArgument::Type(item)) = arguments.args.first() else {
        return Err(syn::Error::new_spanned(
            type_,
            format!("{name} requires exactly one type argument"),
        ));
    };
    if arguments.args.len() != 1 {
        return Err(syn::Error::new_spanned(
            type_,
            format!("{name} requires exactly one type argument"),
        ));
    }
    Ok(Some(item.clone()))
}

fn is_collection(type_: &Type, name: &str) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path })
            if path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

fn is_non_empty_tuple(type_: &Type) -> bool {
    matches!(type_, Type::Tuple(tuple) if !tuple.elems.is_empty())
}

fn external_type<'external>(
    type_: &Type,
    externals: &'external [ExternalModel],
) -> Option<&'external ExternalModel> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    externals.iter().find(|external| &external.ident == ident)
}

#[cfg(test)]
mod tests {
    use super::{ExternalArguments, ModuleArguments, expand};
    use quote::quote;

    fn expansion_error(item: proc_macro2::TokenStream) -> String {
        expand(quote!(path = "counter", crate_path = geam_core), item)
            .expect_err("module should be rejected")
            .to_string()
    }

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
    fn expansion_preserves_argument_item_and_crate_resolution_errors() {
        assert_eq!(
            expand(
                quote!(path),
                quote!(
                    mod counter {}
                )
            )
            .expect_err("malformed arguments should fail")
            .to_string(),
            "expected `=`",
        );
        assert_eq!(
            expand(quote!(path = "counter"), quote!(mod))
                .expect_err("malformed module should fail")
                .to_string(),
            "unexpected end of input, expected identifier",
        );
        let resolution_error = expand(
            quote!(path = "counter"),
            quote!(
                mod counter {}
            ),
        )
        .expect_err("missing default crate dependency should fail")
        .to_string();
        assert!(resolution_error.starts_with(
            "could not resolve the `geam` dependency: Could not find `geam` in `dependencies` or `dev-dependencies`"
        ));
    }

    #[test]
    fn module_must_be_inline() {
        assert_eq!(
            expansion_error(quote!(
                pub struct Counter;
            )),
            "expected `mod`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter;
            )),
            "`#[geam::module]` requires an inline Rust module",
        );
    }

    #[test]
    fn function_forms_are_rejected_at_the_declaration_owner() {
        let cases = [
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        const fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not be const",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        async fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not be async",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        unsafe fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must be safe",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        extern "C" fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must use the ordinary Rust ABI",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next<T>() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not have generics",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next() -> bool
                        where
                            bool: Copy,
                        {
                            true
                        }
                    }
                ),
                "provider functions must not have generics",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next() {}
                    }
                ),
                "provider functions require an explicit return type",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn state_injection_is_reference_only_unique_and_first() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(label: String, #[geam::state] state: &mut RunState) -> String {
                        label
                    }
                }
            )),
            "the `#[geam::state]` parameter must be first",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::state] state: RunState) -> String {
                        String::new()
                    }
                }
            )),
            "the `#[geam::state]` parameter must be `&State` or `&mut State`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::state(value)] state: &mut RunState) -> String {
                        String::new()
                    }
                }
            )),
            "`#[geam::state]` does not accept arguments",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(
                        #[geam::state]
                        #[geam::state]
                        state: &mut RunState,
                    ) -> String {
                        String::new()
                    }
                }
            )),
            "duplicate `#[geam::state]` attribute",
        );
    }

    #[test]
    fn function_marker_and_arity_diagnostics_are_exact() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(value)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "`#[geam::function]` does not accept arguments",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    #[geam::function]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "duplicate `#[geam::function]` attribute",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(
                        a: bool,
                        b: bool,
                        c: bool,
                        d: bool,
                        e: bool,
                        f: bool,
                        g: bool,
                        h: bool,
                    ) -> bool {
                        a
                    }
                }
            )),
            "provider functions support at most seven source arguments",
        );
    }

    #[test]
    fn receiver_parameters_are_rejected_as_non_free_function_shape() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(self) -> bool {
                        true
                    }
                }
            )),
            "provider functions must be free functions",
        );
    }

    #[test]
    fn expansion_preserves_lexical_function_order_and_ordinary_helpers() {
        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    const ENABLED: bool = true;

                    #[inline]
                    fn helper(value: bool) -> bool { value }

                    #[geam::function]
                    fn first(value: bool) -> bool { helper(value) }

                    #[geam::function]
                    fn second(value: bool) -> bool { value }

                    #[geam::function]
                    fn shared(#[geam::state] state: &RunState, value: bool) -> bool { value }

                    #[geam::function]
                    fn mutable(#[geam::state] state: &mut RunState, value: bool) -> bool { value }
                }
            },
        )
        .expect("module should expand")
        .to_string();

        assert!(expansion.contains("fn helper"));
        assert!(expansion.contains("const ENABLED"));
        assert_eq!(expansion.matches("call . state ()").count(), 2);
        assert!(expansion.contains("let __geam_state = & * call . state ()"));
        assert!(expansion.contains("shared (__geam_state , __geam_argument_0)"));
        assert!(expansion.contains("let __geam_state = call . state ()"));
        assert!(expansion.contains("mutable (__geam_state , __geam_argument_0)"));
        let first = expansion
            .find("\"first\"")
            .expect("first registration should exist");
        let second = expansion
            .find("\"second\"")
            .expect("second registration should exist");
        assert!(first < second);
    }

    #[test]
    fn explicit_unit_returns_preserve_nil_shape_without_clippy_conflict() {
        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    #[geam::function]
                    fn record(value: bool) -> () {}
                }
            },
        )
        .expect("explicit unit return should expand")
        .to_string();

        assert!(expansion.contains("allow (clippy :: unused_unit)"));
        assert!(expansion.contains("fn record (value : bool) -> ()"));
        assert!(!expansion.contains("HostTupleType"));
        assert!(!expansion.contains("HostConstructions"));
    }

    #[test]
    fn external_arguments_require_a_name_and_bare_manual_flag() {
        let automatic = syn::parse2::<ExternalArguments>(quote!(name = "Metrics"))
            .expect("default semantics should parse");
        assert_eq!(automatic.name.value(), "Metrics");
        assert!(!automatic.manual);

        let manual = syn::parse2::<ExternalArguments>(quote!(manual, name = "Metrics"))
            .expect("manual semantics should parse in either field order");
        assert_eq!(manual.name.value(), "Metrics");
        assert!(manual.manual);

        let cases = [
            (quote!(), "missing required external argument `name`"),
            (quote!(manual), "missing required external argument `name`"),
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
                quote!(other = "Metrics"),
                "unknown external argument `other`",
            ),
            (quote!(= "Metrics"), "expected identifier"),
            (quote!(name "Metrics"), "expected `=`"),
            (quote!(name = Metrics), "expected string literal"),
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
    fn external_declarations_are_struct_only_non_generic_and_unique() {
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "Metrics")]
                    struct Metrics<T>(T);
                }
            }),
            "external payload structs must not have generics",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "Metrics")]
                    struct First;

                    #[geam::external(name = "Metrics")]
                    struct Second;
                }
            }),
            "duplicate external source type `Metrics`",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external]
                    struct Metrics;
                }
            }),
            "missing required external argument `name`",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name)]
                    struct Metrics;
                }
            }),
            "expected `=`",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external = "Metrics"]
                    struct Metrics;
                }
            }),
            "`#[geam::external]` requires `name = \"...\"` arguments",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "First")]
                    #[geam::external(name = "Second")]
                    struct Metrics;
                }
            }),
            "duplicate `#[geam::external]` attribute",
        );
    }

    #[test]
    fn external_function_arguments_are_borrowed_and_returns_are_owned() {
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "Metrics")]
                    struct Metrics;

                    #[geam::function]
                    fn count(metrics: Metrics) -> bool { true }
                }
            }),
            "external payload `Metrics` arguments must be immutable references",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "Metrics")]
                    struct Metrics;

                    #[geam::function]
                    fn count(metrics: &mut Metrics) -> bool { true }
                }
            }),
            "external payload `Metrics` arguments must be immutable references",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::external(name = "Metrics")]
                    struct Metrics;

                    #[geam::function]
                    fn current(metrics: &Metrics) -> &Metrics { metrics }
                }
            }),
            "external payload `Metrics` returns must be owned",
        );
    }

    #[test]
    fn tuple_arguments_and_returns_reject_borrowed_or_mutable_shapes() {
        let cases = [
            (
                quote! {
                    mod tuples {
                        #[geam::function]
                        fn borrowed(value: &(EcoString, bool)) -> bool { true }
                    }
                },
                "tuple arguments must be passed by value",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::function]
                        fn borrowed(value: &mut (EcoString, bool)) -> bool { true }
                    }
                },
                "tuple arguments must be passed by value",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::function]
                        fn borrowed(value: EcoString) -> &(EcoString, bool) { todo!() }
                    }
                },
                "tuple returns must be owned",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::function]
                        fn borrowed(value: (EcoString, &EcoString)) -> bool { true }
                    }
                },
                "provider source arguments may borrow only declared external payloads",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::function]
                        fn borrowed(value: EcoString) -> (EcoString, &EcoString) { todo!() }
                    }
                },
                "provider source returns must be owned values",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn tuple_external_elements_preserve_argument_and_return_ownership() {
        let cases = [
            (
                quote! {
                    mod tuples {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn consume(value: (Token, EcoString)) -> bool { true }
                    }
                },
                "external payload `Token` arguments must be immutable references",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn consume(value: (&mut Token, EcoString)) -> bool { true }
                    }
                },
                "external payload `Token` arguments must be immutable references",
            ),
            (
                quote! {
                    mod tuples {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn create() -> (&Token, EcoString) { todo!() }
                    }
                },
                "external payload `Token` returns must be owned",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn list_and_vec_ownership_diagnostics_are_exact() {
        let cases = [
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed(values: &geam::List<BigInt>) -> bool { true }
                    }
                },
                "geam::List<T> arguments must be passed by value",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed(values: &mut geam::List<BigInt>) -> bool { true }
                    }
                },
                "geam::List<T> arguments must be passed by value",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn input(values: Vec<BigInt>) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn input(values: Vec) -> bool { true }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed(values: &Vec<BigInt>) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed(value: &BigInt) -> bool { true }
                    }
                },
                "provider source arguments may borrow only declared external payloads",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed() -> &geam::List<BigInt> { todo!() }
                    }
                },
                "geam::List<T> returns must be owned",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed() -> &Vec<BigInt> { todo!() }
                    }
                },
                "Vec<T> returns must be owned",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn borrowed() -> &BigInt { todo!() }
                    }
                },
                "provider source returns must be owned values",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn list_item_diagnostics_reject_borrowed_external_and_nested_collections() {
        let cases = [
            (
                quote! {
                    mod lists {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn input(values: geam::List<&Token>) -> bool { true }
                    }
                },
                "List item external payload `Token` must be owned",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn output() -> Vec<&Token> { Vec::new() }
                    }
                },
                "Vec item external payload `Token` must be owned",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn output() -> geam::List<&BigInt> { todo!() }
                    }
                },
                "List items must be owned values",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn input(values: geam::List<&BigInt>) -> bool { true }
                    }
                },
                "List items must be owned values",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn nested(values: geam::List<geam::List<BigInt>>) -> bool { true }
                    }
                },
                "nested List and Vec item values are not supported",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn nested(values: geam::List<(BigInt, &BigInt)>) -> bool { true }
                    }
                },
                "List items must be owned values",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn nested(values: (geam::List<BigInt>, bool)) -> bool { true }
                    }
                },
                "geam::List<T> is supported only as a top-level source argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn nested(values: (Vec<BigInt>, bool)) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn nested() -> (Vec<BigInt>, bool) { (Vec::new(), true) }
                    }
                },
                "List and Vec values are supported only as top-level source returns",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn list_type_arguments_and_pass_through_are_validated_at_expansion() {
        let cases = [
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn missing(values: geam::List) -> bool { true }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn missing() -> geam::List { todo!() }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn multiple() -> Vec<BigInt, bool> { todo!() }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn multiple(values: geam::List<BigInt, bool>) -> bool { true }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn non_type(values: geam::List<'static>) -> bool { true }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn fabricated() -> geam::List<BigInt> { todo!() }
                    }
                },
                "a returned geam::List<T> must match a List argument",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::function]
                        fn changed(values: geam::List<BigInt>) -> geam::List<bool> { todo!() }
                    }
                },
                "a returned geam::List<T> must match a List argument",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn list_expansion_decodes_lazily_and_distinguishes_pass_through_from_vec_construction() {
        let expansion = expand(
            quote!(path = "lists", crate_path = geam_core),
            quote! {
                mod lists {
                    #[geam::function]
                    fn identity(values: geam::List<BigInt>) -> geam::List<BigInt> {
                        values
                    }

                    #[geam::function]
                    fn reverse(values: geam::List<BigInt>) -> Vec<BigInt> {
                        (0..values.len())
                            .rev()
                            .map(|index| values.get(index).unwrap())
                            .collect()
                    }
                }
            },
        )
        .expect("List pass-through and Vec construction should expand")
        .to_string();

        assert_eq!(expansion.matches("struct __GeamListDecoder0").count(), 1);
        assert!(!expansion.contains("__GeamListDecoder1"));
        assert_eq!(expansion.matches("call . provider_list").count(), 2);
        assert_eq!(expansion.matches("call . return_list").count(), 1);
        assert_eq!(
            expansion.matches("call . return_value (returned)").count(),
            1
        );
        assert_eq!(
            expansion
                .matches("with_scoped_function_and_constructions")
                .count(),
            0,
        );
        assert_eq!(expansion.matches("with_scoped_function :: <").count(), 2);
        assert!(expansion.contains("ProviderListContext < '__geam_list"));
        assert!(expansion.contains("__geam_value . into_scalar :: < BigInt > ()"));
    }

    #[test]
    fn list_external_and_tuple_decoders_reuse_static_store_access_and_construction_tokens() {
        let expansion = expand(
            quote!(path = "lists", crate_path = geam_core),
            quote! {
                mod lists {
                    #[geam::external(name = "Token")]
                    #[derive(PartialEq, Eq, Hash)]
                    struct Token(EcoString);

                    #[geam::function]
                    fn labels(values: geam::List<(EcoString, Token, Token)>) -> Vec<EcoString> {
                        Vec::new()
                    }

                    #[geam::function]
                    fn created() -> Vec<(EcoString, Token)> {
                        Vec::new()
                    }
                }
            },
        )
        .expect("external tuple List items should expand")
        .to_string();

        assert_eq!(
            expansion
                .matches("call . provider_external_payload_access")
                .count(),
            1,
        );
        assert!(expansion.contains("ProviderExternalItem < Token >"));
        assert!(expansion.contains("into_external (& self . __geam_external_0)"));
        assert_eq!(expansion.matches("call . construct_external").count(), 1);
        assert_eq!(expansion.matches("call . construct_tuple").count(), 1);
        assert_eq!(expansion.matches("call . return_list").count(), 2);
    }

    #[test]
    fn top_level_scalar_tuple_returns_need_no_intermediate_constructions() {
        let expansion = expand(
            quote!(path = "tuples", crate_path = geam_core),
            quote! {
                mod tuples {
                    #[geam::function]
                    fn swap(value: (EcoString, BigInt)) -> (BigInt, EcoString) {
                        let (label, count) = value;
                        (count, label)
                    }
                }
            },
        )
        .expect("top-level scalar tuple should expand")
        .to_string();

        assert!(expansion.contains("call . return_tuple"));
        assert!(expansion.contains("with_scoped_function ::"));
        assert!(!expansion.contains("with_scoped_function_and_constructions"));
        assert!(!expansion.contains("HostConstructions"));
    }

    #[test]
    fn tuple_expansion_uses_native_values_and_sealed_post_order_constructions() {
        let expansion = expand(
            quote!(path = "tuples", crate_path = geam_core),
            quote! {
                mod tuples {
                    #[geam::external(name = "Token")]
                    #[derive(Clone, PartialEq, Eq, Hash)]
                    struct Token(EcoString);

                    #[geam::function]
                    fn consume(value: (&Token, (EcoString,))) -> EcoString { value.1.0 }

                    #[geam::function]
                    fn create(value: EcoString) -> (Token, (EcoString,)) {
                        (Token(value.clone()), (value,))
                    }

                    #[geam::function]
                    fn reassociate(
                        value: (EcoString, (BigInt, bool)),
                    ) -> ((EcoString, BigInt), bool) {
                        let (label, (count, enabled)) = value;
                        ((label, count), enabled)
                    }

                    #[geam::function]
                    fn wide(
                        value: (bool, bool, bool, bool, bool, bool, bool, bool),
                    ) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
                        value
                    }
                }
            },
        )
        .expect("native tuple declarations should expand")
        .to_string();

        assert!(expansion.contains("call . tuple_values (__geam_argument_0)"));
        assert!(expansion.contains("consume ((& * __geam_payload_"));
        assert!(expansion.contains("call . return_tuple"));
        let external = expansion
            .find("call . construct_external")
            .expect("tuple external return should be an intermediate construction");
        let one_tuple = expansion
            .find("call . construct_tuple")
            .expect("nested one-tuple should be an intermediate construction");
        assert!(external < one_tuple);
        assert!(expansion.contains("HostTypeIndex0"));
        assert!(expansion.contains("HostTypeIndexNext <"));
        assert!(expansion.contains("with_scoped_function_and_constructions"));
        assert!(
            expansion
                .contains("HostTupleType < geam_core :: __macro_support :: HostTypeList < bool")
        );
    }

    #[test]
    fn external_expansion_preserves_type_and_function_declaration_order() {
        let expansion = expand(
            quote!(path = "metrics", crate_path = geam_core),
            quote! {
                mod metrics {
                    struct Helper;

                    #[geam::function]
                    fn copy(
                        #[geam::state] state: &RunState,
                        value: &Metrics,
                        label: EcoString,
                    ) -> Metrics { value.clone() }

                    #[geam::external(name = "Metrics")]
                    #[derive(Clone)]
                    struct Metrics;

                    #[geam::external(name = "Snapshot", manual)]
                    struct Snapshot;

                    #[geam::function]
                    fn snapshot(value: &Metrics) -> Snapshot { Snapshot }
                }
            },
        )
        .expect("external module should expand")
        .to_string();

        let metrics = expansion
            .find("with_external_type :: < __GeamProvider , __GeamExternalSchema0 >")
            .expect("Metrics registration should exist");
        let snapshot = expansion
            .find("with_external_type :: < __GeamProvider , __GeamExternalSchema1 >")
            .expect("Snapshot registration should exist");
        let copy = expansion
            .find("\"copy\"")
            .expect("copy registration should exist");
        let snapshot_function = expansion
            .find("\"snapshot\"")
            .expect("snapshot registration should exist");

        assert!(metrics < snapshot);
        assert!(snapshot < copy);
        assert!(copy < snapshot_function);
        assert!(expansion.contains("struct Helper"));
        assert!(expansion.contains("derive (Clone)"));
        assert!(expansion.contains("HostExternalStore < Metrics >"));
        assert!(expansion.contains("HostExternalStore < Snapshot >"));
        assert!(
            expansion.contains("impl geam_core :: __macro_support :: ExternalPayload for Metrics")
        );
        assert!(
            !expansion
                .contains("impl geam_core :: __macro_support :: ExternalPayload for Snapshot")
        );
        assert!(expansion.contains("external_payload_hash (self)"));
        assert!(expansion.contains("concat ! (\"Metrics\" , \"(<opaque>)\")"));
        assert!(expansion.contains("type Payload = Metrics"));
        assert!(expansion.contains("HostExternalBinding < Profile , __GeamExternalSchema0 >"));
        assert!(expansion.contains("type Storage = __GeamExternalStorage0"));
        assert!(expansion.contains(
            "< Metrics as geam_core :: __macro_support :: ExternalPayload > :: source_equal"
        ));
        assert!(expansion.contains(
            "< Metrics as geam_core :: __macro_support :: ExternalPayload > :: source_hash"
        ));
        assert!(
            expansion.contains(
                "< Metrics as geam_core :: __macro_support :: ExternalPayload > :: inspect"
            )
        );
        assert!(expansion.contains("let __geam_payload_0 = call . external_payload"));
        assert_eq!(expansion.matches("call . state ()").count(), 1);
        assert!(
            expansion.contains("copy (__geam_state , & * __geam_payload_0 , __geam_argument_1)")
        );
        assert!(expansion.contains("let returned = call . create_external (returned)"));
        assert!(expansion.contains(
            "< super :: Component as geam_core :: __macro_support :: ProviderPackage > :: PACKAGE"
        ));
    }

    #[test]
    fn only_bare_declared_payload_types_receive_external_adapters() {
        let expansion = expand(
            quote!(path = "metrics", crate_path = geam_core),
            quote! {
                mod metrics {
                    #[geam::external(name = "Metrics")]
                    struct Metrics;

                    #[geam::function]
                    fn wrapped(value: Wrapper<Metrics>) -> bool { true }

                    #[geam::function]
                    fn associated(value: <Metrics as Trait>::Value) -> bool { true }
                }
            },
        )
        .expect("non-bare types should remain owned by the host type system")
        .to_string();

        assert!(expansion.contains("Wrapper < Metrics >"));
        assert!(expansion.contains("< Metrics as Trait > :: Value"));
        assert!(!expansion.contains("call . external_payload"));
    }
}
