mod custom_value;

use crate::path::support_path;
use custom_value::{
    CustomConstructorModel, CustomFieldModel, CustomFieldValueType, CustomFields, CustomInputModel,
    CustomModel, collect_custom_declarations, custom_input_model, custom_output_type,
    custom_output_type_with_index,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use std::collections::{BTreeMap, BTreeSet};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Fields, FnArg, GenericArgument, GenericParam, Ident, Item, ItemFn, ItemMod,
    ItemStruct, LitStr, Meta, Path, PathArguments, ReturnType, Token, Type, TypeBareFn, TypePath,
};

struct ModuleArguments {
    path: LitStr,
    crate_path: Option<Path>,
    profile: ModuleProfile,
    stores: Option<Path>,
}

#[derive(Default)]
struct PartialModuleArguments {
    path: Option<LitStr>,
    crate_path: Option<Path>,
    profile: Option<Path>,
    component: Option<Type>,
    stores: Option<Path>,
}

enum ModuleProfile {
    Component,
    Explicit { bound: Path, component: Box<Type> },
}

#[derive(Default)]
struct FunctionArguments {
    profile: Option<Ident>,
}

struct ExternalArguments {
    name: LitStr,
    manual: bool,
    retained: bool,
    parameters: Vec<Ident>,
    input: Option<Ident>,
    payload: Option<Type>,
}

#[derive(Default)]
struct PartialExternalArguments {
    name: Option<LitStr>,
    manual: Option<Ident>,
    retained: Option<Ident>,
    parameters: Option<Vec<Ident>>,
    input: Option<Ident>,
    payload: Option<Type>,
}

enum ExternalSemantics {
    Default,
    Manual,
    Retained,
}

struct ExternalModel {
    ident: Ident,
    name: LitStr,
    semantics: ExternalSemantics,
    schema: Ident,
    storage: Ident,
    store_field: Ident,
    generic: Option<GenericExternalModel>,
}

struct GenericExternalModel {
    parameters: Vec<Ident>,
    input: Ident,
    visibility: syn::Visibility,
    storage: GenericExternalStorage,
}

#[derive(Clone)]
enum GenericExternalStorage {
    StoredFields {
        payload: Ident,
        owner: Ident,
        fields: Vec<StoredExternalField>,
    },
    ManualPayload {
        payload: Type,
    },
}

#[derive(Clone)]
struct StoredExternalField {
    ident: Ident,
    parameter: Ident,
    parameter_index: usize,
    index: TokenStream,
}

#[derive(Clone)]
struct GenericValueType {
    source: Type,
    instantiated: Type,
    path: TypePath,
    host: GenericHostType,
}

struct CallbackType {
    signature: TypeBareFn,
    path: TypePath,
    arguments: Vec<FunctionReturnType>,
    return_: Box<FunctionInputType>,
    codec: Ident,
}

struct GenericExternalType {
    output: Ident,
    input: Ident,
    schema: Ident,
    storage: GenericExternalStorage,
    source_arguments: Vec<Type>,
    arguments: Vec<ClassifiedGenericHostType>,
}

#[derive(Clone)]
struct ClassifiedGenericHostType {
    host: GenericHostType,
    instantiated: Type,
}

#[derive(Clone)]
enum GenericHostType {
    Parameter {
        index: usize,
    },
    Scalar(Type),
    Declared(Type),
    External {
        schema: Ident,
    },
    Custom {
        index: usize,
    },
    Tuple(Vec<GenericHostType>),
    List(Box<GenericHostType>),
    Result {
        success: Box<GenericHostType>,
        failure: Box<GenericHostType>,
    },
    Option(Box<GenericHostType>),
    Function {
        arguments: Vec<GenericHostType>,
        return_: Box<GenericHostType>,
    },
}

struct GenericParameterScope {
    declared: Vec<Ident>,
    indices: BTreeMap<String, usize>,
}

struct FunctionGeneric {
    ident: Ident,
    index: usize,
}

#[derive(Clone)]
enum ProviderValueType {
    Scalar(Type),
    Generic(Box<GenericValueType>),
    Declared {
        type_: Type,
        input: DeclaredInput,
    },
    External {
        payload: Ident,
        schema: Ident,
    },
    Custom {
        index: usize,
        rust: Type,
    },
    List(Box<ListType>),
    Tuple(Vec<ProviderValueType>),
    Result {
        success: Box<ProviderValueType>,
        failure: Box<ProviderValueType>,
    },
    Option {
        value: Box<ProviderValueType>,
    },
}

#[derive(Clone)]
enum FunctionInputValueType {
    Scalar(Type),
    Declared {
        type_: Type,
        input: DeclaredInput,
    },
    External {
        payload: Ident,
        schema: Ident,
    },
    Custom {
        index: usize,
        rust: Type,
    },
    Tuple(Vec<ProviderValueType>),
    Result {
        success: Box<ProviderValueType>,
        failure: Box<ProviderValueType>,
    },
    Option {
        value: Box<ProviderValueType>,
    },
}

#[derive(Clone)]
enum StaticValueType {
    Scalar(Type),
    Declared {
        type_: Type,
    },
    External {
        payload: Ident,
        schema: Ident,
        store_field: Ident,
    },
    Custom {
        index: usize,
    },
    Tuple(Vec<StaticValueType>),
    Result {
        success: Box<StaticValueType>,
        failure: Box<StaticValueType>,
    },
    Option {
        value: Box<StaticValueType>,
    },
}

#[derive(Clone)]
enum FunctionOutputValueType {
    Value(Box<FunctionOutputLeafType>),
    Generic(Box<GenericValueType>),
    Tuple(Vec<FunctionOutputValueType>),
    Result {
        success: Box<FunctionOutputValueType>,
        failure: Box<FunctionOutputValueType>,
    },
    Option {
        value: Box<FunctionOutputValueType>,
    },
    Vec(FunctionOutputCollectionType),
}

#[derive(Clone)]
enum FunctionRootOutputValueType {
    Value(Box<FunctionOutputLeafType>),
    Tuple(Vec<FunctionOutputValueType>),
    Result {
        success: Box<FunctionOutputValueType>,
        failure: Box<FunctionOutputValueType>,
    },
    Option {
        value: Box<FunctionOutputValueType>,
    },
    Vec(FunctionOutputCollectionType),
}

#[derive(Clone)]
enum FunctionOutputLeafType {
    Scalar(Type),
    Declared { type_: Type, input: DeclaredInput },
    External { payload: Ident, schema: Ident },
    Custom { index: usize, rust: Type },
}

#[derive(Clone)]
struct FunctionOutputCollectionType {
    value: Box<FunctionOutputValueType>,
}

enum SourceWrapper<'type_> {
    Result {
        path: &'type_ TypePath,
        success: &'type_ Type,
        failure: &'type_ Type,
    },
    Option {
        path: &'type_ TypePath,
        value: &'type_ Type,
    },
    Other,
}

enum SourceWrapperArguments<'arguments> {
    Result(&'arguments PathArguments),
    Option(&'arguments PathArguments),
    Other,
}

#[derive(Clone, Copy)]
enum DeclaredInput {
    Owned,
    BorrowedExternal,
}

#[derive(Clone)]
struct CollectionType {
    source: Type,
    item: Type,
    value: StaticValueType,
}

#[derive(Clone)]
struct ListType {
    collection: CollectionType,
    decoder: Ident,
}

enum FunctionInputType {
    Value(Box<FunctionInputValueType>),
    Generic(Box<GenericValueType>),
    External(Box<GenericExternalType>),
    List(Box<ListType>),
}

enum FunctionArgumentType {
    Input(FunctionInputType),
    Callback(Box<CallbackType>),
}

enum FunctionReturnType {
    Value(FunctionRootOutputValueType),
    Generic(Box<GenericValueType>),
    External(Box<GenericExternalType>),
    List(Box<ListType>),
}

enum CallAccess {
    None,
    Shared,
    Mutable,
}

struct FunctionModel {
    ident: Ident,
    generics: Vec<FunctionGeneric>,
    arguments: Vec<FunctionArgumentType>,
    return_: FunctionReturnType,
    call: CallAccess,
    host_result: bool,
    profile: bool,
}

struct ListDecoderModel {
    ident: Ident,
    value: StaticValueType,
    key: String,
}

struct ListExternalAccess {
    payload: Ident,
    schema: Ident,
    field: Ident,
}

struct ListDeclaredAccess {
    type_: Type,
    field: Ident,
}

struct GeneratedFunction {
    callback_codecs: Vec<TokenStream>,
    wrapper: TokenStream,
    registration: TokenStream,
    bounds: Vec<TokenStream>,
}

struct GeneratedCallback {
    definition: TokenStream,
    requirements: TokenStream,
    has_constructions: bool,
    bounds: Vec<TokenStream>,
}

struct GeneratedValue {
    statements: TokenStream,
    value: TokenStream,
}

struct GeneratedReturn {
    statements: TokenStream,
    completion: TokenStream,
    constructions: Vec<GeneratedConstruction>,
}

struct GeneratedConstruction {
    requirement: TokenStream,
    binding: Ident,
}

struct OutputEnvironment<'model> {
    customs: &'model [CustomModel],
    support: &'model TokenStream,
    provider: &'model TokenStream,
    return_type: &'model TokenStream,
}

struct OutputState<'output> {
    names: &'output mut GeneratedNames,
    constructions: &'output mut Vec<GeneratedConstruction>,
}

#[derive(Clone, Copy)]
enum GenericInputSource {
    Declared,
    Instantiated,
}

struct InputEnvironment<'model> {
    customs: &'model [CustomModel],
    support: &'model TokenStream,
    return_type: &'model TokenStream,
    function_generics: &'model [FunctionGeneric],
    generic_source: GenericInputSource,
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

impl GenericParameterScope {
    fn new(generics: &syn::Generics) -> syn::Result<Self> {
        if let Some(where_clause) = &generics.where_clause {
            return Err(syn::Error::new_spanned(
                where_clause,
                "provider functions must not have where clauses",
            ));
        }

        let mut declared = Vec::new();
        for parameter in &generics.params {
            let parameter = match parameter {
                GenericParam::Type(parameter) => parameter,
                GenericParam::Lifetime(parameter) => {
                    return Err(syn::Error::new_spanned(
                        parameter,
                        "provider functions must not have lifetime generics",
                    ));
                }
                GenericParam::Const(parameter) => {
                    return Err(syn::Error::new_spanned(
                        parameter,
                        "provider functions must not have const generics",
                    ));
                }
            };
            if !parameter.bounds.is_empty() {
                return Err(syn::Error::new_spanned(
                    &parameter.bounds,
                    "provider function type generics must not have bounds",
                ));
            }
            if parameter.default.is_some() {
                return Err(syn::Error::new_spanned(
                    parameter,
                    "provider function type generics must not have defaults",
                ));
            }
            declared.push(parameter.ident.clone());
        }

        Ok(Self {
            declared,
            indices: BTreeMap::new(),
        })
    }

    fn declared_ident(&self, type_: &Type) -> Option<Ident> {
        let Type::Path(TypePath { qself: None, path }) = type_ else {
            return None;
        };
        let ident = path.get_ident()?;
        for declared in &self.declared {
            if declared == ident {
                return Some(declared.clone());
            }
        }
        None
    }

    fn parameter_index(&mut self, ident: Ident) -> usize {
        let key = ident.to_string();
        let next = self.indices.len();
        *self.indices.entry(key).or_insert(next)
    }

    fn finish(self) -> syn::Result<Vec<FunctionGeneric>> {
        let mut generics = Vec::with_capacity(self.declared.len());
        for ident in self.declared {
            let Some(index) = self.indices.get(&ident.to_string()).copied() else {
                return Err(syn::Error::new_spanned(
                    &ident,
                    format!(
                        "generic parameter `{ident}` must appear inside a Value<...> source shape"
                    ),
                ));
            };
            generics.push(FunctionGeneric { ident, index });
        }
        Ok(generics)
    }
}

pub(crate) fn expand(arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let arguments = syn::parse2::<ModuleArguments>(arguments)?;
    let mut module = syn::parse2::<ItemMod>(item)?;
    let support = support_path(arguments.crate_path.as_ref())?;
    let has_explicit_profile = matches!(&arguments.profile, ModuleProfile::Explicit { .. });
    let profile_visibility = has_explicit_profile.then(|| quote!(pub(super)));
    let (profile_bound, component) = match &arguments.profile {
        ModuleProfile::Component => (
            quote!(#support::HostComponentProfile<super::Component>),
            quote!(super::Component),
        ),
        ModuleProfile::Explicit { bound, component } => (quote!(#bound), quote!(#component)),
    };
    let inline_module_error =
        syn::Error::new_spanned(&module, "`#[geam::module]` requires an inline Rust module");
    let (_, items) = module.content.as_mut().ok_or(inline_module_error)?;
    let provider_type_names = items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(item)
                if item
                    .attrs
                    .iter()
                    .any(|attribute| is_marker(attribute, "external")) =>
            {
                Some(item.ident.unraw().to_string())
            }
            Item::Enum(item)
                if item
                    .attrs
                    .iter()
                    .any(|attribute| is_marker(attribute, "custom")) =>
            {
                Some(item.ident.unraw().to_string())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let mut externals = Vec::new();
    let mut source_names = BTreeSet::new();
    let mut generated_input_names = BTreeSet::new();
    for item in items.iter_mut() {
        let Item::Struct(payload) = item else {
            continue;
        };
        let Some(external) = take_external_marker(&mut payload.attrs)? else {
            continue;
        };
        if !source_names.insert(external.name.value()) {
            return Err(syn::Error::new(
                external.name.span(),
                format!("duplicate external source type `{}`", external.name.value()),
            ));
        }
        let index = externals.len();
        if let Some(input) = &external.input {
            let input_name = input.unraw().to_string();
            if provider_type_names.contains(&input_name) {
                return Err(syn::Error::new(
                    input.span(),
                    format!(
                        "generated external input type `{input}` conflicts with provider value type `{input}`"
                    ),
                ));
            }
            if !generated_input_names.insert(input_name) {
                return Err(syn::Error::new(
                    input.span(),
                    format!("duplicate generated input type `{input}`"),
                ));
            }
        }
        externals.push(build_external_model(index, payload, external, &support)?);
    }
    match (&arguments.profile, &arguments.stores, externals.is_empty()) {
        (ModuleProfile::Explicit { .. }, None, false) => {
            return Err(syn::Error::new_spanned(
                &module.ident,
                "built-in modules with external declarations require a `stores` projection",
            ));
        }
        (ModuleProfile::Explicit { .. }, Some(stores), true) => {
            return Err(syn::Error::new_spanned(
                stores,
                "module `stores` is used only by external declarations",
            ));
        }
        _ => {}
    }
    let (stores_visibility, storage_visibility) = if arguments.stores.is_some() {
        (quote!(pub), quote!(pub))
    } else {
        (quote!(pub(super)), quote!())
    };

    let mut list_decoders = Vec::new();
    let custom_declarations = collect_custom_declarations(
        items,
        &mut source_names,
        &mut generated_input_names,
        &provider_type_names,
        &externals,
        &mut list_decoders,
    )?;
    let customs = custom_declarations.models;

    let mut functions = Vec::new();
    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        if let Some(function_arguments) = take_function_marker(&mut function.attrs)? {
            let model = validate_function(
                function,
                function_arguments,
                match &arguments.profile {
                    ModuleProfile::Component => None,
                    ModuleProfile::Explicit { bound, .. } => Some(bound),
                },
                &externals,
                &customs,
                &mut list_decoders,
                &support,
            )?;
            functions.push(model);
        }
    }

    let stores_projection = arguments.stores.as_ref();
    let module_path = arguments.path;
    let module_ident = &module.ident;
    let payload_semantics = externals.iter().filter_map(|external| {
        if external.generic.is_some() || !matches!(external.semantics, ExternalSemantics::Default) {
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

                fn inspect(&self) -> #support::EcoString {
                    #support::EcoString::from(::core::concat!(#source_name, "(<opaque>)"))
                }
            }
        })
    });
    let store_fields = externals.iter().map(|external| {
        let payload = external
            .generic
            .as_ref()
            .map(|generic| match &generic.storage {
                GenericExternalStorage::StoredFields { payload, .. } => quote!(#payload),
                GenericExternalStorage::ManualPayload { payload } => quote!(#payload),
            })
            .unwrap_or_else(|| {
                let payload = &external.ident;
                quote!(#payload)
            });
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
        let store = if let Some(stores) = stores_projection {
            quote! {
                &#stores::<Profile>(stores).#store_field
            }
        } else {
            quote! {
                &<Profile as #support::HostComponentProfile<#component>>::component_stores(
                    stores,
                ).#module_ident.#store_field
            }
        };
        let semantics = match external.semantics {
            ExternalSemantics::Default | ExternalSemantics::Manual => quote! {
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
                ) -> #support::EcoString {
                    <#payload as #support::ExternalPayload>::inspect(value)
                }
            },
            ExternalSemantics::Retained => quote! {
                fn source_equal(
                    context: &#support::HostExternalEquality<'_>,
                    left: &Self::Payload,
                    right: &Self::Payload,
                ) -> bool {
                    <#payload as #support::RetainedExternalPayload>::source_equal(
                        left,
                        context,
                        right,
                    )
                }

                fn source_hash(
                    context: &#support::HostExternalHashing<'_>,
                    value: &Self::Payload,
                ) -> u64 {
                    <#payload as #support::RetainedExternalPayload>::source_hash(value, context)
                }

                fn inspect(
                    context: &#support::HostExternalInspection<'_>,
                    value: &Self::Payload,
                ) -> #support::EcoString {
                    <#payload as #support::RetainedExternalPayload>::inspect(value, context)
                }
            },
        };
        if let Some(generic) = &external.generic {
            let parameter_count = generic.parameters.len();
            let parameters = &generic.parameters;
            let input = &generic.input;
            let output = &external.ident;
            let visibility = &generic.visibility;
            let host_parameters = parameters
                .iter()
                .map(|parameter| quote!(<#parameter as #support::ProviderValue>::Host))
                .collect::<Vec<_>>();
            let host_arguments = host_type_token_sequence(&host_parameters, &support);
            match &generic.storage {
                GenericExternalStorage::StoredFields {
                    payload: generic_payload,
                    owner,
                    fields,
                } => {
                    let output_contexts = fields.iter().map(|field| {
                        let parameter = &parameters[field.parameter_index];
                        let index = &field.index;
                        quote! {
                            #support::ProviderStoredOutput<
                                '__geam_output,
                                #owner,
                                #index,
                                <#parameter as #support::ProviderValue>::Host,
                            >
                        }
                    });
                    let output_type = quote! {
                        #output<#(#parameters,)* #(#output_contexts,)*>
                    };
                    let output_impl_parameters = ::core::iter::once(quote!('__geam_output))
                        .chain(parameters.iter().map(|parameter| quote!(#parameter)))
                        .collect::<Vec<_>>();
                    let output_patterns = fields.iter().map(|field| {
                        let ident = &field.ident;
                        quote!(#ident)
                    });
                    let output_payload_fields = fields.iter().map(|field| {
                        let ident = &field.ident;
                        quote!(#ident: #ident.into_host())
                    });
                    let output_payload = quote! {
                        ::core::result::Result::<
                            #generic_payload,
                            #support::ProviderExternalItem<#generic_payload>,
                        >::Ok({
                            let #output { #(#output_patterns,)* } = self;
                            #generic_payload { #(#output_payload_fields,)* }
                        })
                    };
                    let output_codec = generic_external_output_codec(
                        &output_impl_parameters,
                        parameters,
                        output_type,
                        output_payload,
                        schema,
                        &host_arguments,
                        &support,
                    );
                    let payload_fields = fields.iter().map(|field| {
                        let ident = &field.ident;
                        let index = &field.index;
                        quote!(#ident: #support::HostStoredValue<#support::HostStoredType<#index>>)
                    });
                    let input_accessors = fields.iter().map(|field| {
                        let ident = &field.ident;
                        let parameter = &field.parameter;
                        let index = &field.index;
                        quote! {
                            #visibility fn #ident(&self) -> #support::Stored<
                                #parameter,
                                #support::ProviderStoredInput<
                                    '_,
                                    #owner,
                                    #index,
                                    <__GeamArguments as #support::HostTypeAt<#index>>::Type,
                                >,
                            >
                            where
                                __GeamArguments: #support::HostTypeAt<#index>,
                            {
                                #support::Stored::from_input(&self.__geam_context.payload().#ident)
                            }
                        }
                    });
                    let equal_fields = fields.iter().map(|field| {
                        let ident = &field.ident;
                        quote!(context.stored_values_equal(&left.#ident, &right.#ident))
                    });
                    let hash_fields = fields.iter().map(|field| {
                        let ident = &field.ident;
                        quote!(context.stored_value_hash(&value.#ident))
                    });
                    let contexts = fields
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format_ident!("__GeamStoredContext{index}"))
                        .collect::<Vec<_>>();
                    return quote! {
                        #[doc(hidden)]
                        pub struct #schema;

                        impl #support::HostExternalSchema for #schema {
                            const PACKAGE: &'static str =
                                <super::Component as #support::ProviderPackage>::PACKAGE;
                            const MODULE: &'static str = #module_path;
                            const NAME: &'static str = #source_name;
                            const PARAMETER_COUNT: usize = #parameter_count;
                        }

                        impl<#(#parameters,)* #(#contexts,)*>
                            #support::ProviderExternalDeclaration
                            for #payload<#(#parameters,)* #(#contexts,)*>
                        {
                            type Schema = #schema;
                        }

                        impl<#(#parameters,)* Profile, Provider, Return>
                            #support::ProviderDynamicInput<Profile, Provider, Return>
                            for #payload<#(#parameters,)*>
                        where
                            Profile: __GeamModuleProfile,
                            Provider: #support::HostProvider<Profile>,
                            Return: #support::HostType,
                            #(#parameters: #support::ProviderValue,)*
                        {
                            type Host = #support::HostExternalType<#schema, #host_arguments>;
                            type View<'__geam_call> = #input<
                                #(#parameters,)*
                                #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #generic_payload,
                                    #host_arguments,
                                >,
                            >;

                            fn from_host<'__geam_call>(
                                call: &mut #support::HostCall<
                                    '__geam_call,
                                    Profile,
                                    Provider,
                                    Return,
                                >,
                                value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                            ) -> Self::View<'__geam_call> {
                                let value = call.provider_external_item_with::<
                                    __GeamProvider,
                                    #schema,
                                    #host_arguments,
                                >(value);
                                #input::__geam_from_host(
                                    #support::ProviderExternalInputContext::from_host(value),
                                )
                            }
                        }

                        #output_codec

                        #[doc(hidden)]
                        pub struct #owner;

                        impl #support::ProviderStoredOwner for #owner {}

                        #[doc(hidden)]
                        pub struct #generic_payload {
                            #(#payload_fields,)*
                        }

                        #visibility struct #input<
                            #(#parameters,)*
                            __GeamContext = #support::MissingExternalInputContext,
                        > {
                            __geam_context: __GeamContext,
                            __geam_parameters: ::core::marker::PhantomData<
                                fn() -> (#(#parameters,)*)
                            >,
                        }

                        impl<'__geam_call, #(#parameters,)* __GeamArguments>
                            #input<
                                #(#parameters,)*
                                #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #generic_payload,
                                    __GeamArguments,
                                >,
                            >
                        where
                            __GeamArguments: #support::HostTypeSequence,
                        {
                            fn __geam_from_host(
                                context: #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #generic_payload,
                                    __GeamArguments,
                                >,
                            ) -> Self {
                                Self {
                                    __geam_context: context,
                                    __geam_parameters: ::core::marker::PhantomData,
                                }
                            }

                            #(#input_accessors)*
                        }

                        #[doc(hidden)]
                        #storage_visibility struct #storage;

                        impl<Profile> #support::HostExternalStorage<Profile, #schema> for #storage
                        where
                            Profile: __GeamModuleProfile,
                        {
                            type Payload = #generic_payload;

                            fn store(
                                stores: &Profile::ExternalStores,
                            ) -> &#support::HostExternalStore<Self::Payload> {
                                #store
                            }

                            fn source_equal(
                                context: &#support::HostExternalEquality<'_>,
                                left: &Self::Payload,
                                right: &Self::Payload,
                            ) -> bool {
                                true #(&& #equal_fields)*
                            }

                            fn source_hash(
                                context: &#support::HostExternalHashing<'_>,
                                value: &Self::Payload,
                            ) -> u64 {
                                #support::external_payload_hash(&(#(#hash_fields,)*))
                            }

                            fn inspect(
                                _context: &#support::HostExternalInspection<'_>,
                                _value: &Self::Payload,
                            ) -> #support::EcoString {
                                #support::EcoString::from(::core::concat!(#source_name, "(<opaque>)"))
                            }
                        }
                    };
                }
                GenericExternalStorage::ManualPayload { payload: retained_payload } => {
                    let output_type = quote! {
                        #output<
                            #(#parameters,)*
                            #support::ProviderExternalOutput<#retained_payload>,
                        >
                    };
                    let output_impl_parameters = parameters
                        .iter()
                        .map(|parameter| quote!(#parameter))
                        .collect::<Vec<_>>();
                    let output_codec = generic_external_output_codec(
                        &output_impl_parameters,
                        parameters,
                        output_type,
                        quote!(self.__geam_context.into_value()),
                        schema,
                        &host_arguments,
                        &support,
                    );
                    let retained_accessors = parameters.iter().enumerate().map(|(index, parameter)| {
                        let method = retained_parameter_accessor(parameter);
                        let index = host_type_index(index, &support);
                        quote! {
                            #visibility fn #method<'__geam_value>(
                                &'__geam_value self,
                                select: impl ::core::ops::FnOnce(
                                    &'__geam_value #retained_payload,
                                ) -> &'__geam_value #support::Retained<#retained_payload, #index>,
                            ) -> #support::Stored<
                                #parameter,
                                #support::ProviderStoredInput<
                                    '__geam_value,
                                    #retained_payload,
                                    #index,
                                    <__GeamArguments as #support::HostTypeAt<#index>>::Type,
                                >,
                            >
                            where
                                __GeamArguments: #support::HostTypeAt<#index>,
                            {
                                #support::Stored::from_retained(select(self.__geam_context.payload()))
                            }
                        }
                    });
                    return quote! {
                        #[doc(hidden)]
                        pub struct #schema;

                        impl #support::HostExternalSchema for #schema {
                            const PACKAGE: &'static str =
                                <super::Component as #support::ProviderPackage>::PACKAGE;
                            const MODULE: &'static str = #module_path;
                            const NAME: &'static str = #source_name;
                            const PARAMETER_COUNT: usize = #parameter_count;
                        }

                        impl<#(#parameters,)* __GeamExternalContext>
                            #support::ProviderExternalDeclaration
                            for #output<#(#parameters,)* __GeamExternalContext>
                        {
                            type Schema = #schema;
                        }

                        impl<#(#parameters,)* Profile, Provider, Return>
                            #support::ProviderDynamicInput<Profile, Provider, Return>
                            for #output<#(#parameters,)*>
                        where
                            Profile: __GeamModuleProfile,
                            Provider: #support::HostProvider<Profile>,
                            Return: #support::HostType,
                            #(#parameters: #support::ProviderValue,)*
                        {
                            type Host = #support::HostExternalType<#schema, #host_arguments>;
                            type View<'__geam_call> = #input<
                                #(#parameters,)*
                                #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #retained_payload,
                                    #host_arguments,
                                >,
                            >;

                            fn from_host<'__geam_call>(
                                call: &mut #support::HostCall<
                                    '__geam_call,
                                    Profile,
                                    Provider,
                                    Return,
                                >,
                                value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                            ) -> Self::View<'__geam_call> {
                                let value = call.provider_external_item_with::<
                                    __GeamProvider,
                                    #schema,
                                    #host_arguments,
                                >(value);
                                #input::__geam_from_host(
                                    #support::ProviderExternalInputContext::from_host(value),
                                )
                            }
                        }

                        #output_codec

                        impl<#(#parameters,)*> #output<#(#parameters,)*> {
                            #visibility fn from_payload(
                                payload: #retained_payload,
                            ) -> #output<
                                #(#parameters,)*
                                #support::ProviderExternalOutput<#retained_payload>,
                            > {
                                #output {
                                    __geam_context: #support::ProviderExternalOutput::new(payload),
                                    __geam_parameters: ::core::marker::PhantomData,
                                }
                            }
                        }

                        #visibility struct #input<
                            #(#parameters,)*
                            __GeamContext = #support::MissingExternalInputContext,
                        > {
                            __geam_context: __GeamContext,
                            __geam_parameters: ::core::marker::PhantomData<
                                fn() -> (#(#parameters,)*)
                            >,
                        }

                        impl<'__geam_call, #(#parameters,)* __GeamArguments>
                            #input<
                                #(#parameters,)*
                                #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #retained_payload,
                                    __GeamArguments,
                                >,
                            >
                        where
                            __GeamArguments: #support::HostTypeSequence,
                        {
                            fn __geam_from_host(
                                context: #support::ProviderExternalInputContext<
                                    '__geam_call,
                                    #retained_payload,
                                    __GeamArguments,
                                >,
                            ) -> Self {
                                Self {
                                    __geam_context: context,
                                    __geam_parameters: ::core::marker::PhantomData,
                                }
                            }

                            #visibility fn payload(&self) -> &#retained_payload {
                                self.__geam_context.payload()
                            }

                            #visibility fn into_value(
                                self,
                            ) -> #output<
                                #(#parameters,)*
                                #support::ProviderExternalOutput<#retained_payload>,
                            > {
                                #output {
                                    __geam_context: self.__geam_context.into_output(),
                                    __geam_parameters: ::core::marker::PhantomData,
                                }
                            }

                            #(#retained_accessors)*
                        }

                        #[doc(hidden)]
                        #storage_visibility struct #storage;

                        impl<Profile> #support::HostExternalStorage<Profile, #schema> for #storage
                        where
                            Profile: __GeamModuleProfile,
                        {
                            type Payload = #retained_payload;

                            fn store(
                                stores: &Profile::ExternalStores,
                            ) -> &#support::HostExternalStore<Self::Payload> {
                                #store
                            }

                            fn source_equal(
                                context: &#support::HostExternalEquality<'_>,
                                left: &Self::Payload,
                                right: &Self::Payload,
                            ) -> bool {
                                <#retained_payload as #support::RetainedExternalPayload>::source_equal(
                                    left,
                                    context,
                                    right,
                                )
                            }

                            fn source_hash(
                                context: &#support::HostExternalHashing<'_>,
                                value: &Self::Payload,
                            ) -> u64 {
                                <#retained_payload as #support::RetainedExternalPayload>::source_hash(
                                    value,
                                    context,
                                )
                            }

                            fn inspect(
                                context: &#support::HostExternalInspection<'_>,
                                value: &Self::Payload,
                            ) -> #support::EcoString {
                                <#retained_payload as #support::RetainedExternalPayload>::inspect(
                                    value,
                                    context,
                                )
                            }
                        }
                    };
                }
            }
        }
        quote! {
            #[doc(hidden)]
            pub struct #schema;

            impl #support::HostExternalSchema for #schema {
                const PACKAGE: &'static str =
                    <super::Component as #support::ProviderPackage>::PACKAGE;
                const MODULE: &'static str = #module_path;
                const NAME: &'static str = #source_name;
                const PARAMETER_COUNT: usize = 0;
            }

            impl #support::ProviderExternalDeclaration for #payload {
                type Schema = #schema;
            }

            impl #support::ProviderStoredOwner for #payload {}

            impl #support::ProviderValue for #payload {
                type Host = #support::HostExternalType<#schema>;
                type Input = #support::ProviderExternalItem<Self>;
                type ListInput = Self;
                type OutputRequirements = #support::ProviderConstruction<Self::Host>;
                type RootRequirements = #support::ProviderNoConstructions;
            }

            impl<Profile> #support::ProviderExternalCodec<Profile> for #payload
            where
                Profile: __GeamModuleProfile,
            {
                fn input<'__geam_call, Provider, Return>(
                    call: &#support::HostCall<'__geam_call, Profile, Provider, Return>,
                    value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                ) -> #support::ProviderExternalItem<Self>
                where
                    Provider: #support::HostProvider<Profile>,
                    Return: #support::HostType,
                {
                    call.provider_external_item_with::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(value)
                }

                fn output<'__geam_call, Provider, Return>(
                    call: &mut #support::HostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Return,
                    >,
                    value: #support::ProviderExternalItem<Self>,
                ) -> <Self::Host as #support::HostType>::Value<'__geam_call>
                where
                    Provider: #support::HostProvider<Profile>,
                    Return: #support::HostType,
                {
                    call.provider_external_from_item::<#schema, #support::HostTypeListEnd, _>(value)
                }
            }

            impl #support::ProviderListInputValue for #payload {
                type View = #support::ProviderExternalItem<Self>;
                type Decoder = #support::ProviderExternalListDecoder<Self>;
            }

            impl<Profile> #support::ProviderListInputCodec<Profile> for #payload
            where
                Profile: __GeamModuleProfile,
            {
                fn decoder<'__geam_call, Provider, Return>(
                    call: &#support::HostCall<'__geam_call, Profile, Provider, Return>,
                ) -> Self::Decoder
                where
                    Provider: #support::HostProvider<Profile>,
                    Return: #support::HostType,
                {
                    #support::ProviderExternalListDecoder::new(
                        call.provider_external_payload_access_with::<
                            __GeamProvider,
                            #schema,
                        >(),
                    )
                }
            }

            impl<Profile, Provider, Return>
                #support::ProviderOutputValue<Profile, Provider, Return> for #payload
            where
                Profile: __GeamModuleProfile,
                Provider: #support::HostProvider<Profile>,
                Return: #support::HostType,
            {
                fn into_host<'__geam_call>(
                    self,
                    call: &mut #support::HostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Return,
                    >,
                    construction: &#support::ProviderConstructions<
                        '__geam_call,
                        Self::OutputRequirements,
                    >,
                ) -> <Self::Host as #support::HostType>::Value<'__geam_call> {
                    call.construct_external_with_binding::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(construction.token(), self)
                }
            }

            impl<Profile, Provider> #support::ProviderRootOutputValue<Profile, Provider>
                for #payload
            where
                Profile: __GeamModuleProfile,
                Provider: #support::HostProvider<Profile>,
            {
                fn complete<'__geam_call>(
                    self,
                    mut call: #support::HostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Self::Host,
                    >,
                    _constructions: &#support::ProviderConstructions<
                        '__geam_call,
                        Self::RootRequirements,
                    >,
                ) -> ::core::result::Result<
                    #support::HostCallCompletion<'__geam_call, Self::Host>,
                    #support::HostCallError,
                > {
                    let value = call.create_external_with_binding::<__GeamProvider>(self);
                    ::core::result::Result::Ok(call.return_value(value))
                }
            }

            #[doc(hidden)]
            #storage_visibility struct #storage;

            impl<Profile> #support::HostExternalStorage<Profile, #schema> for #storage
            where
                Profile: __GeamModuleProfile,
            {
                type Payload = #payload;

                fn store(
                    stores: &Profile::ExternalStores,
                ) -> &#support::HostExternalStore<Self::Payload> {
                    #store
                }

                #semantics
            }
        }
    });
    // An explicit payload type is also the retention owner. Multiple source
    // declarations may intentionally share that domain while retaining their
    // own schemas and stores, as Dict and TransientDict do.
    let mut retained_owner_keys = BTreeSet::new();
    let retained_owners = externals
        .iter()
        .filter_map(|external| {
            let generic = external.generic.as_ref()?;
            let GenericExternalStorage::ManualPayload { payload } = &generic.storage else {
                return None;
            };
            retained_owner_keys
                .insert(quote!(#payload).to_string())
                .then(|| quote!(impl #support::ProviderStoredOwner for #payload {}))
        })
        .collect::<Vec<_>>();
    let bindings = externals.iter().map(|external| {
        let schema = &external.schema;
        let storage = &external.storage;
        quote! {
            impl<Profile> #support::HostExternalBinding<Profile, #schema> for __GeamProvider
            where
                Profile: __GeamModuleProfile,
            {
                type Storage = #storage;
            }
        }
    });
    let custom_inputs = customs
        .iter()
        .enumerate()
        .filter_map(|(index, custom)| {
            custom
                .input
                .as_ref()
                .map(|input| (index, input.ident.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let custom_declarations = customs
        .iter()
        .enumerate()
        .map(|(index, custom)| {
            generate_custom_declaration(
                index,
                custom,
                &customs,
                &custom_inputs,
                &support,
                &module_path,
            )
        })
        .collect::<Vec<_>>();
    let generated_list_decoders = list_decoders
        .iter()
        .map(|decoder| generate_list_decoder(decoder, &customs, &custom_inputs, &support))
        .collect::<Vec<_>>();
    let generated_functions = functions
        .iter()
        .map(|function| generate_function(function, &customs, &support))
        .collect::<Vec<_>>();
    let mut module_bound_keys = BTreeSet::new();
    let module_bounds = generated_functions
        .iter()
        .flat_map(|function| function.bounds.iter().cloned())
        .filter(|bound| module_bound_keys.insert(bound.to_string()))
        .collect::<Vec<_>>();
    let callback_codecs = generated_functions
        .iter()
        .flat_map(|function| function.callback_codecs.iter());
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
        #profile_visibility trait __GeamModuleProfile: #profile_bound {}

        impl<Profile> __GeamModuleProfile for Profile
        where
            Profile: #profile_bound,
        {}
    }));
    items.push(Item::Verbatim(quote! {
        #[doc(hidden)]
        #[derive(Default)]
        #stores_visibility struct __GeamStores {
            #(#store_fields)*
        }
    }));
    for semantics in payload_semantics {
        items.push(Item::Verbatim(semantics));
    }
    for schema in schemas {
        items.push(Item::Verbatim(schema));
    }
    for owner in retained_owners {
        items.push(Item::Verbatim(owner));
    }
    for declaration in custom_declarations {
        items.push(Item::Verbatim(declaration));
    }
    items.push(Item::Verbatim(quote! {
        #profile_visibility struct __GeamProvider;
    }));
    items.push(Item::Verbatim(quote! {
        impl<Profile> #support::HostProvider<Profile> for __GeamProvider
        where
            Profile: __GeamModuleProfile,
        {
            type State = <#component as #support::HostProviderComponent>::RunState;

            fn project(state: &mut Profile::RunState) -> &mut Self::State {
                <Profile as #support::HostComponentProfile<#component>>::component_state(state)
            }
        }
    }));
    for binding in bindings {
        items.push(Item::Verbatim(binding));
    }
    for decoder in generated_list_decoders {
        items.push(Item::Verbatim(decoder));
    }
    for codec in callback_codecs {
        items.push(Item::Verbatim(codec.clone()));
    }
    for wrapper in wrappers {
        items.push(Item::Verbatim(wrapper.clone()));
    }
    let registration = quote! {
        let provider = #support::HostProviderModule::<Profile>::new(
            <super::Component as #support::ProviderPackage>::PACKAGE,
            #module_path,
        )?;
        #(#external_registrations)*
        #(#registrations)*
        ::core::result::Result::Ok(provider)
    };
    let module_span = module.ident.span();
    let registrar = if has_explicit_profile {
        quote_spanned! {module_span=>
            pub(super) fn __geam_module<Profile>() -> ::core::result::Result<
                #support::HostProviderModule<Profile>,
                #support::HostRegistrationError,
            >
            where
                Profile: __GeamModuleProfile,
                #(#module_bounds,)*
            {
                #registration
            }
        }
    } else {
        quote_spanned! {module_span=>
            pub(super) struct __GeamModule;

            impl<Profile> #support::ProviderModuleRegistration<Profile> for __GeamModule
            where
                Profile: __GeamModuleProfile,
                #(#module_bounds,)*
            {
                fn module() -> ::core::result::Result<
                    #support::HostProviderModule<Profile>,
                    #support::HostRegistrationError,
                > {
                    #registration
                }
            }
        }
    };
    items.push(Item::Verbatim(registrar));

    Ok(quote!(#module))
}

fn generic_external_output_codec(
    impl_parameters: &[TokenStream],
    parameters: &[Ident],
    output: TokenStream,
    output_payload: TokenStream,
    schema: &Ident,
    host_arguments: &TokenStream,
    support: &TokenStream,
) -> TokenStream {
    quote! {
        impl<#(#impl_parameters,)*> #support::ProviderValue for #output
        where
            #(#parameters: #support::ProviderValue,)*
        {
            type Host = #support::HostExternalType<#schema, #host_arguments>;
            type Input = Self;
            type ListInput = Self;
            type OutputRequirements = #support::ProviderConstruction<Self::Host>;
            type RootRequirements = #support::ProviderNoConstructions;
        }

        impl<#(#impl_parameters,)* Profile, Provider, Return>
            #support::ProviderOutputValue<Profile, Provider, Return>
            for #output
        where
            Profile: __GeamModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#parameters: #support::ProviderValue,)*
        {
            fn into_host<'__geam_call>(
                self,
                call: &mut #support::HostCall<'__geam_call, Profile, Provider, Return>,
                construction: &#support::ProviderConstructions<
                    '__geam_call,
                    Self::OutputRequirements,
                >,
            ) -> <Self::Host as #support::HostType>::Value<'__geam_call> {
                match #output_payload {
                    ::core::result::Result::Ok(payload) => {
                        call.construct_external_with_binding::<
                            __GeamProvider,
                            #schema,
                            #host_arguments,
                        >(construction.token(), payload)
                    }
                    ::core::result::Result::Err(value) => {
                        call.provider_external_from_item::<#schema, #host_arguments, _>(value)
                    }
                }
            }
        }

        impl<#(#impl_parameters,)* Profile, Provider>
            #support::ProviderRootOutputValue<Profile, Provider>
            for #output
        where
            Profile: __GeamModuleProfile,
            Provider: #support::HostProvider<Profile>,
            #(#parameters: #support::ProviderValue,)*
        {
            fn complete<'__geam_call>(
                self,
                mut call: #support::HostCall<'__geam_call, Profile, Provider, Self::Host>,
                _constructions: &#support::ProviderConstructions<
                    '__geam_call,
                    Self::RootRequirements,
                >,
            ) -> ::core::result::Result<
                #support::HostCallCompletion<'__geam_call, Self::Host>,
                #support::HostCallError,
            > {
                let value = match #output_payload {
                    ::core::result::Result::Ok(payload) => {
                        call.create_external_with_binding::<__GeamProvider>(payload)
                    }
                    ::core::result::Result::Err(value) => {
                        call.provider_external_from_item::<#schema, #host_arguments, _>(value)
                    }
                };
                ::core::result::Result::Ok(call.return_value(value))
            }
        }
    }
}

fn generate_custom_declaration(
    custom_index: usize,
    custom: &CustomModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    module_path: &LitStr,
) -> TokenStream {
    let custom_ident = &custom.ident;
    let schema = &custom.schema;
    let source_name = custom.ident.unraw().to_string();
    let mut field_definitions = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            let definition = &field.definition;
            let label = if field.named {
                let ident = &field.ident;
                let label = ident.unraw().to_string();
                quote!(::core::option::Option::Some(#label))
            } else {
                quote!(::core::option::Option::None)
            };
            let type_ = host_custom_field_type(&field.value, customs, support);
            field_definitions.push(quote! {
                #[doc(hidden)]
                pub struct #definition;

                impl #support::HostCustomField for #definition {
                    const LABEL: ::core::option::Option<&'static str> = #label;
                    type Type = #type_;
                }
            });
        }
    }
    let mut constructor_definitions = Vec::with_capacity(custom.constructors.len());
    for (index, constructor) in custom.constructors.iter().enumerate() {
        let definition = &constructor.definition;
        let marker = &constructor.marker;
        let name = constructor.ident.unraw().to_string();
        let mut fields = Vec::new();
        for field in custom_field_models(&constructor.fields) {
            let definition = &field.definition;
            fields.push(quote!(#definition));
        }
        let fields = host_custom_field_sequence(&fields, support);
        let index = host_custom_index(index, support);
        constructor_definitions.push(quote! {
            #[doc(hidden)]
            pub struct #definition;

            impl #support::HostCustomConstructorDefinition for #definition {
                const NAME: &'static str = #name;
                type Fields = #fields;
            }

            #[doc(hidden)]
            pub type #marker = #support::HostCustomConstructorAt<
                #support::HostCustomType<#schema>,
                #index,
                #definition,
            >;
        });
    }
    let mut constructors = Vec::with_capacity(custom.constructors.len());
    for constructor in &custom.constructors {
        let definition = &constructor.definition;
        constructors.push(quote!(#definition));
    }
    let constructors = host_custom_constructor_sequence(&constructors, support);

    let mut root_names = GeneratedNames::default();
    let root = generate_custom_return(
        custom_index,
        customs,
        support,
        &quote!(Provider),
        &quote!(Self::Host),
        &mut root_names,
    );
    let root_requirements = provider_requirement_sequence(&root.constructions, support);
    let mut root_requirement_bounds = Vec::new();
    if !root.constructions.is_empty() {
        root_requirement_bounds.push(quote! {
            #root_requirements: #support::ProviderConstructionRequirements
        });
        root_requirement_bounds.extend(provider_requirement_selection_bounds(
            &root_requirements,
            &root.constructions,
            support,
        ));
    }
    let root_bindings =
        provider_construction_bindings(&root.constructions, quote!(&constructions), support);
    let root_statements = root.statements;
    let root_completion = root.completion;

    let mut nested_names = GeneratedNames::default();
    let mut nested_constructions = Vec::new();
    let nested_provider = quote!(Provider);
    let nested_return_type = quote!(Return);
    let nested_environment = OutputEnvironment {
        customs,
        support,
        provider: &nested_provider,
        return_type: &nested_return_type,
    };
    let mut nested_state = OutputState {
        names: &mut nested_names,
        constructions: &mut nested_constructions,
    };
    let nested = generate_custom_intermediate(
        custom_index,
        quote!(self),
        &nested_environment,
        &mut nested_state,
    );
    let nested_requirements = provider_requirement_sequence(&nested_constructions, support);
    let mut nested_requirement_bounds = vec![quote! {
        #nested_requirements: #support::ProviderConstructionRequirements
    }];
    nested_requirement_bounds.extend(provider_requirement_selection_bounds(
        &nested_requirements,
        &nested_constructions,
        support,
    ));
    let nested_bindings =
        provider_construction_bindings(&nested_constructions, quote!(&constructions), support);
    let nested_statements = nested.statements;
    let nested_value = nested.value;
    let nested_codec_bounds =
        custom_output_codec_bounds(custom, customs, support, &quote!(Provider), &quote!(Return));
    let root_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(Provider),
        &quote!(Self::Host),
    );
    let input_declaration = if let Some(input_model) = &custom.input {
        let input = &input_model.ident;
        let visibility = &custom.visibility;
        let mut variants = Vec::with_capacity(custom.constructors.len());
        for constructor in &custom.constructors {
            let ident = &constructor.ident;
            let variant = match &constructor.fields {
                CustomFields::Unit => quote!(#ident),
                CustomFields::Unnamed(fields) => {
                    let mut types = Vec::with_capacity(fields.len());
                    for field in fields {
                        types.push(custom_input_type(&field.value, custom_inputs, support));
                    }
                    quote!(#ident(#(#types),*))
                }
                CustomFields::Named(fields) => {
                    let mut members = Vec::with_capacity(fields.len());
                    for field in fields {
                        let field_ident = &field.ident;
                        let type_ = custom_input_type(&field.value, custom_inputs, support);
                        members.push(quote!(#field_ident: #type_));
                    }
                    quote!(#ident { #(#members),* })
                }
            };
            variants.push(variant);
        }
        let input_codec_bounds = custom_input_codec_bounds(custom, customs, custom_inputs, support);
        let decoder_input_codec_bounds = input_codec_bounds.clone();
        let list_codec_bounds = custom_list_codec_bounds(custom_index, customs, support);
        let decoder_definition = generate_custom_decoder(
            custom,
            input_model,
            customs,
            custom_inputs,
            support,
            &decoder_input_codec_bounds,
        );
        let decoder_ident = &input_model.decoder;
        let list_decoder = &input_model.list_decoder;
        let list_decoder_value = list_decoder_value(
            list_decoder,
            &StaticValueType::Custom {
                index: custom_index,
            },
            customs,
            support,
        );
        quote! {
            #visibility enum #input {
                #(#variants,)*
            }

            impl #support::ProviderValue for #input {
                type Host = #support::HostCustomType<#schema>;
                type Input = Self;
                type ListInput = Self;
                type OutputRequirements = #support::ProviderNoConstructions;
                type RootRequirements = #support::ProviderNoConstructions;
            }

            impl #support::ProviderCustomInputDeclaration for #input {
                type Schema = #schema;
                type Output = #custom_ident;
            }

            impl<Profile, Provider, Return>
                #support::ProviderInputValue<Profile, Provider, Return> for #input
            where
                Profile: __GeamModuleProfile,
                Provider: #support::HostProvider<Profile>,
                Return: #support::HostType,
                #(#input_codec_bounds,)*
            {
                fn from_host<'__geam_call>(
                    call: &mut #support::HostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Return,
                    >,
                    value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                ) -> Self {
                    #decoder_ident(call, value)
                }
            }

            impl #support::ProviderListInputValue for #input {
                type View = Self;
                type Decoder = #list_decoder;
            }

            impl<Profile> #support::ProviderListInputCodec<Profile> for #input
            where
                Profile: __GeamModuleProfile,
                #(#list_codec_bounds,)*
            {
                fn decoder<'__geam_call, Provider, Return>(
                    call: &#support::HostCall<'__geam_call, Profile, Provider, Return>,
                ) -> Self::Decoder
                where
                    Provider: #support::HostProvider<Profile>,
                    Return: #support::HostType,
                {
                    #list_decoder_value
                }
            }

            #decoder_definition
        }
    } else {
        TokenStream::new()
    };
    let input = if let Some(input) = &custom.input {
        let input = &input.ident;
        quote!(#input)
    } else {
        quote!(#support::NoCustomInput)
    };

    quote! {
        #(#field_definitions)*
        #(#constructor_definitions)*

        #[doc(hidden)]
        pub struct #schema;

        impl #support::HostCustomSchema for #schema {
            const PACKAGE: &'static str =
                <super::Component as #support::ProviderPackage>::PACKAGE;
            const MODULE: &'static str = #module_path;
            const NAME: &'static str = #source_name;
            const PARAMETER_COUNT: usize = 0;
            type Constructors = #constructors;
        }

        impl #support::ProviderCustomDeclaration for #custom_ident {
            type Schema = #schema;
            type Input = #input;
        }

        impl #support::ProviderValue for #custom_ident {
            type Host = #support::HostCustomType<#schema>;
            type Input = #input;
            type ListInput = #input;
            type OutputRequirements = #nested_requirements;
            type RootRequirements = #root_requirements;
        }

        impl<Profile, Provider, Return>
            #support::ProviderOutputValue<Profile, Provider, Return> for #custom_ident
        where
            Profile: __GeamModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#nested_codec_bounds,)*
            #(#nested_requirement_bounds,)*
        {
            fn into_host<'__geam_call>(
                self,
                mut call: &mut #support::HostCall<'__geam_call, Profile, Provider, Return>,
                constructions: &#support::ProviderConstructions<
                    '__geam_call,
                    Self::OutputRequirements,
                >,
            ) -> <Self::Host as #support::HostType>::Value<'__geam_call> {
                #nested_bindings
                #nested_statements
                #nested_value
            }
        }

        impl<Profile, Provider> #support::ProviderRootOutputValue<Profile, Provider>
            for #custom_ident
        where
            Profile: __GeamModuleProfile,
            Provider: #support::HostProvider<Profile>,
            #(#root_codec_bounds,)*
            #(#root_requirement_bounds,)*
        {
            fn complete<'__geam_call>(
                self,
                mut call: #support::HostCall<'__geam_call, Profile, Provider, Self::Host>,
                constructions: &#support::ProviderConstructions<
                    '__geam_call,
                    Self::RootRequirements,
                >,
            ) -> ::core::result::Result<
                #support::HostCallCompletion<'__geam_call, Self::Host>,
                #support::HostCallError,
            > {
                #root_bindings
                let returned = self;
                #root_statements
                #root_completion
            }
        }

        #input_declaration
    }
}

fn generate_custom_decoder(
    custom: &CustomModel,
    input_model: &CustomInputModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    codec_bounds: &[TokenStream],
) -> TokenStream {
    let decoder = &input_model.decoder;
    let schema = &custom.schema;
    let input = &input_model.ident;
    let mut names = GeneratedNames::default();
    let mut branches = Vec::with_capacity(custom.constructors.len());
    let mut remaining = TokenStream::new();
    for (constructor_index, constructor) in custom.constructors.iter().enumerate() {
        let marker = &constructor.marker;
        let fields = custom_field_models(&constructor.fields);
        let mut host_fields = Vec::with_capacity(fields.len());
        let mut host_field_tokens = Vec::with_capacity(fields.len());
        for _ in fields {
            let field = names.next("custom_input_host_field");
            host_field_tokens.push(quote!(#field));
            host_fields.push(field);
        }
        let host_pattern = host_value_sequence(&host_field_tokens);
        let mut decoded = Vec::with_capacity(fields.len());
        for (field, host) in fields.iter().zip(&host_fields) {
            decoded.push(decode_custom_field_value(
                &field.value,
                quote!(#host),
                customs,
                custom_inputs,
                support,
                &mut names,
            ));
        }
        let mut statements = Vec::with_capacity(decoded.len());
        let mut declarations = Vec::with_capacity(decoded.len());
        let mut value_names = Vec::with_capacity(decoded.len());
        for field in decoded {
            statements.push(field.statements);
            let name = names.next("custom_input_field");
            let value = field.value;
            declarations.push(quote!(let #name = #value;));
            value_names.push(name);
        }
        let expression = custom_input_expression(input, constructor, &value_names);
        let body = quote! {
                #(#statements)*
                #(#declarations)*
                #expression
        };
        if constructor_index + 1 == custom.constructors.len() {
            remaining = quote! {
                let #host_pattern =
                    call.provider_remaining_custom_fields::<#marker>(value);
                #body
            };
        } else {
            branches.push(quote! {
                if let ::core::option::Option::Some(#host_pattern) =
                    call.provider_custom_fields::<#marker>(value)
                {
                    return { #body };
                }
            });
        }
    }
    quote! {
        fn #decoder<'__geam_call, Profile, Provider, Return>(
            call: &mut #support::HostCall<
                '__geam_call,
                Profile,
                Provider,
                Return,
            >,
            value: #support::HostCustom<
                '__geam_call,
                #support::HostCustomType<#schema>,
            >,
        ) -> #input
        where
            Profile: __GeamModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#codec_bounds,)*
        {
            #(#branches)*
            #remaining
        }
    }
}

fn decode_custom_field_value(
    type_: &CustomFieldValueType,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_custom_input_value(type_, input, customs, custom_inputs, support, names)
        }
        CustomFieldValueType::List(list) => {
            let decoder =
                list_decoder_value(&list.decoder, &list.collection.value, customs, support);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(call.provider_input_list(#input, #decoder)),
            }
        }
    }
}

fn decode_custom_input_value(
    type_: &StaticValueType,
    input: TokenStream,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        StaticValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        StaticValueType::Declared { type_, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(
                <<#type_ as #support::ProviderValue>::Input as
                    #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                        call,
                        #input,
                    )
            ),
        },
        StaticValueType::External { payload, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(
                <#support::ProviderExternalItem<#payload> as
                    #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                        call,
                        #input,
                    )
            ),
        },
        StaticValueType::Custom { index, .. } => {
            let input_type = &custom_inputs[index];
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(
                    <#input_type as #support::ProviderInputValue<
                        Profile,
                        Provider,
                        Return,
                    >>::from_host(call, #input)
                ),
            }
        }
        StaticValueType::Tuple(elements) => {
            let mut host_elements = Vec::with_capacity(elements.len());
            let mut host_element_tokens = Vec::with_capacity(elements.len());
            for _ in elements {
                let element = names.next("custom_input_tuple_host");
                host_element_tokens.push(quote!(#element));
                host_elements.push(element);
            }
            let host_values = host_value_sequence(&host_element_tokens);
            let mut statements = quote! {
                let #host_values = call.tuple_values(#input);
            };
            let mut values = Vec::with_capacity(elements.len());
            for (element, host) in elements.iter().zip(host_elements) {
                let decoded = decode_custom_input_value(
                    element,
                    quote!(#host),
                    customs,
                    custom_inputs,
                    support,
                    names,
                );
                statements.extend(decoded.statements);
                values.push(decoded.value);
            }
            GeneratedValue {
                statements,
                value: quote!((#(#values,)*)),
            }
        }
        StaticValueType::Result { success, failure } => {
            let success_host = host_static_value_type(success, customs, support);
            let failure_host = host_static_value_type(failure, customs, support);
            let success_value = names.next("result_success_host");
            let failure_value = names.next("result_failure_host");
            let decoded_success = decode_custom_input_value(
                success,
                quote!(#success_value),
                customs,
                custom_inputs,
                support,
                names,
            );
            let decoded_failure = decode_custom_input_value(
                failure,
                quote!(#failure_value),
                customs,
                custom_inputs,
                support,
                names,
            );
            let success_statements = decoded_success.statements;
            let success = decoded_success.value;
            let failure_statements = decoded_failure.statements;
            let failure = decoded_failure.value;
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!({
                    if let ::core::option::Option::Some((#success_value, ())) =
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
                    }
                }),
            }
        }
        StaticValueType::Option { value } => {
            let host = host_static_value_type(value, customs, support);
            let some_host = names.next("option_some_host");
            let decoded = decode_custom_input_value(
                value,
                quote!(#some_host),
                customs,
                custom_inputs,
                support,
                names,
            );
            let statements = decoded.statements;
            let value = decoded.value;
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!({
                    if let ::core::option::Option::Some((#some_host, ())) =
                        call.provider_custom_fields::<#support::ProviderSome<#host>>(#input)
                    {
                        #statements
                        ::core::option::Option::Some(#value)
                    } else {
                        ::core::option::Option::None
                    }
                }),
            }
        }
    }
}

fn custom_input_type(
    type_: &CustomFieldValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            custom_input_value_type(type_, custom_inputs, support)
        }
        CustomFieldValueType::List(list) => {
            let item = custom_list_input_value_type(&list.collection.value, custom_inputs, support);
            let decoder = &list.decoder;
            quote! {
                #support::List<
                    #item,
                    #support::ProviderInputListContext<#decoder>,
                >
            }
        }
    }
}

fn custom_input_value_type(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => {
            quote!(<#type_ as #support::ProviderValue>::Input)
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
                .map(|element| custom_input_value_type(element, custom_inputs, support))
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = custom_input_value_type(success, custom_inputs, support);
            let failure = custom_input_value_type(failure, custom_inputs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = custom_input_value_type(value, custom_inputs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}

fn custom_list_input_value_type(
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
            let types = elements
                .iter()
                .map(|element| custom_list_input_value_type(element, custom_inputs, support))
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = custom_list_input_value_type(success, custom_inputs, support);
            let failure = custom_list_input_value_type(failure, custom_inputs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = custom_list_input_value_type(value, custom_inputs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}

fn custom_output_codec_bounds(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            let value = match &field.value {
                CustomFieldValueType::Value(value) => value,
                CustomFieldValueType::List(list) => &list.collection.value,
            };
            collect_custom_output_codec_bounds(
                value,
                customs,
                support,
                provider,
                return_type,
                &mut bounds,
            );
        }
    }
    deduplicate_bounds(bounds)
}

fn collect_custom_output_codec_bounds(
    type_: &StaticValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        StaticValueType::Declared { type_, .. } => {
            bounds.push(quote! {
                #type_: #support::ProviderOutputValue<
                    Profile,
                    #provider,
                    #return_type,
                >
            });
        }
        StaticValueType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            bounds.push(quote! {
                #type_: #support::ProviderOutputValue<
                    Profile,
                    #provider,
                    #return_type,
                >
            });
        }
        StaticValueType::Tuple(elements) => {
            for element in elements {
                collect_custom_output_codec_bounds(
                    element,
                    customs,
                    support,
                    provider,
                    return_type,
                    bounds,
                );
            }
        }
        StaticValueType::Result { success, failure } => {
            collect_custom_output_codec_bounds(
                success,
                customs,
                support,
                provider,
                return_type,
                bounds,
            );
            collect_custom_output_codec_bounds(
                failure,
                customs,
                support,
                provider,
                return_type,
                bounds,
            );
        }
        StaticValueType::Option { value } => collect_custom_output_codec_bounds(
            value,
            customs,
            support,
            provider,
            return_type,
            bounds,
        ),
        StaticValueType::Scalar(_) | StaticValueType::External { .. } => {}
    }
}

fn custom_input_codec_bounds(
    custom: &CustomModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            match &field.value {
                CustomFieldValueType::Value(value) => {
                    collect_custom_input_codec_bounds(value, custom_inputs, support, &mut bounds);
                }
                CustomFieldValueType::List(list) => {
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
    }
    deduplicate_bounds(bounds)
}

fn collect_custom_input_codec_bounds(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        StaticValueType::Declared { type_, .. } => {
            bounds.push(quote! {
                <#type_ as #support::ProviderValue>::Input:
                    #support::ProviderInputValue<Profile, Provider, Return>
            });
        }
        StaticValueType::Custom { index, .. } => {
            let input = &custom_inputs[index];
            bounds.push(quote! {
                #input: #support::ProviderInputValue<Profile, Provider, Return>
            });
        }
        StaticValueType::Tuple(elements) => {
            for element in elements {
                collect_custom_input_codec_bounds(element, custom_inputs, support, bounds);
            }
        }
        StaticValueType::Result { success, failure } => {
            collect_custom_input_codec_bounds(success, custom_inputs, support, bounds);
            collect_custom_input_codec_bounds(failure, custom_inputs, support, bounds);
        }
        StaticValueType::Option { value } => {
            collect_custom_input_codec_bounds(value, custom_inputs, support, bounds);
        }
        StaticValueType::Scalar(_) | StaticValueType::External { .. } => {}
    }
}

fn custom_list_codec_bounds(
    custom_index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for access in list_declared_accesses(
        &StaticValueType::Custom {
            index: custom_index,
        },
        customs,
    ) {
        let type_ = access.type_;
        bounds.push(quote! {
            <#type_ as #support::ProviderValue>::ListInput:
                #support::ProviderListInputCodec<Profile>
        });
    }
    deduplicate_bounds(bounds)
}

fn deduplicate_bounds(bounds: Vec<TokenStream>) -> Vec<TokenStream> {
    let mut keys = BTreeSet::new();
    let mut deduplicated = Vec::new();
    for bound in bounds {
        if keys.insert(bound.to_string()) {
            deduplicated.push(bound);
        }
    }
    deduplicated
}

fn host_custom_field_sequence(fields: &[TokenStream], support: &TokenStream) -> TokenStream {
    let mut tail = quote!(#support::HostCustomFieldListEnd);
    for head in fields.iter().rev() {
        tail = quote!(#support::HostCustomFieldList<#head, #tail>);
    }
    tail
}

fn host_custom_constructor_sequence(
    constructors: &[TokenStream],
    support: &TokenStream,
) -> TokenStream {
    let mut tail = quote!(#support::HostCustomConstructorListEnd);
    for head in constructors.iter().rev() {
        tail = quote!(#support::HostCustomConstructorList<#head, #tail>);
    }
    tail
}

fn host_custom_index(index: usize, support: &TokenStream) -> TokenStream {
    let mut output = quote!(#support::HostCustomIndex0);
    for _ in 0..index {
        output = quote!(#support::HostCustomIndexNext<#output>);
    }
    output
}

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

fn generate_function(
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

fn provider_value_from_input_root(type_: &FunctionInputValueType) -> ProviderValueType {
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

fn function_output_from_root(type_: &FunctionRootOutputValueType) -> FunctionOutputValueType {
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

fn generate_custom_return(
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

fn generate_custom_intermediate(
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

fn custom_field_models(fields: &CustomFields) -> &[CustomFieldModel] {
    match fields {
        CustomFields::Unit => &[],
        CustomFields::Unnamed(fields) | CustomFields::Named(fields) => fields,
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

fn host_argument_type(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionArgumentType::Input(type_) => host_input_type(type_, customs, support),
        FunctionArgumentType::Callback(callback) => callback_host_type(callback, customs, support),
    }
}

fn host_input_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
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

fn host_return_type(
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

fn function_output_host_type(
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

fn host_value_type(
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

fn host_static_value_type(
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

fn generic_host_type(
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

fn host_custom_field_type(
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

fn wrapper_argument_type(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionArgumentType::Input(type_) => wrapper_input_type(type_, customs, support),
        FunctionArgumentType::Callback(callback) => {
            let arguments = callback_host_arguments(callback, customs, support);
            let return_ = host_input_type(&callback.return_, customs, support);
            quote!(#support::HostCallable<'__geam_call, #arguments, #return_>)
        }
    }
}

fn wrapper_input_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionInputType::Value(type_) => wrapper_input_value_type(type_, customs, support),
        FunctionInputType::Generic(value) => {
            let host = generic_host_type(&value.host, customs, support);
            quote!(<#host as #support::HostType>::Value<'__geam_call>)
        }
        FunctionInputType::External(external) => {
            let host = generic_external_host_type(external, customs, support);
            quote!(#support::HostExternal<'__geam_call, #host>)
        }
        FunctionInputType::List(list) => {
            let item = host_static_value_type(&list.collection.value, customs, support);
            quote!(#support::HostList<'__geam_call, #item>)
        }
    }
}

fn callback_host_type(
    callback: &CallbackType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let arguments = callback_host_arguments(callback, customs, support);
    let return_ = host_input_type(&callback.return_, customs, support);
    quote!(#support::HostFunctionType<#arguments, #return_>)
}

fn callback_host_arguments(
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
) -> TokenStream {
    match type_ {
        FunctionInputValueType::Scalar(type_) => quote!(#type_),
        FunctionInputValueType::Declared { type_, .. } => {
            quote!(
                <<#type_ as #support::ProviderValue>::Host as
                    #support::HostType>::Value<'__geam_call>
            )
        }
        FunctionInputValueType::External { schema, .. } => {
            quote!(#support::HostExternal<'__geam_call, #support::HostExternalType<#schema>>)
        }
        FunctionInputValueType::Custom { index, .. } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustom<'__geam_call, #support::HostCustomType<#schema>>)
        }
        FunctionInputValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, customs, support);
            quote!(#support::HostTuple<'__geam_call, #elements>)
        }
        FunctionInputValueType::Result { success, failure } => {
            let success = host_value_type(success, customs, support);
            let failure = host_value_type(failure, customs, support);
            quote!(#support::HostCustom<'__geam_call, #support::ProviderResult<#success, #failure>>)
        }
        FunctionInputValueType::Option { value } => {
            let value = host_value_type(value, customs, support);
            quote!(#support::HostCustom<'__geam_call, #support::ProviderOption<#value>>)
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

fn provider_requirement_sequence(
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

fn provider_construction_bindings(
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

fn provider_requirement_selection_bounds(
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

fn register_list_decoder(list: &CollectionType, decoders: &mut Vec<ListDecoderModel>) -> Ident {
    let key = static_value_key(&list.value);
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

fn static_value_key(type_: &StaticValueType) -> String {
    match type_ {
        StaticValueType::Scalar(type_) => format!("scalar:{}", quote!(#type_)),
        StaticValueType::Declared { type_, .. } => {
            format!("declared:{}", quote!(#type_))
        }
        StaticValueType::External { schema, .. } => format!("external:{schema}"),
        StaticValueType::Custom { index, .. } => format!("custom:{index}"),
        StaticValueType::Tuple(elements) => format!(
            "tuple:({})",
            elements
                .iter()
                .map(static_value_key)
                .collect::<Vec<_>>()
                .join(",")
        ),
        StaticValueType::Result { success, failure } => format!(
            "result:<{},{}>",
            static_value_key(success),
            static_value_key(failure),
        ),
        StaticValueType::Option { value } => {
            format!("option:<{}>", static_value_key(value))
        }
    }
}

fn generate_list_decoder(
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

fn list_decoder_value(
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

fn list_declared_accesses(
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

fn custom_input_expression(
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

fn list_item_view_type(
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

fn list_signature_type(list: &ListType, customs: &[CustomModel], support: &TokenStream) -> Type {
    let item = &list.collection.item;
    let host_item = host_static_value_type(&list.collection.value, customs, support);
    let decoder = &list.decoder;
    syn::parse_quote! {
        #support::List<
            #item,
            #support::ProviderListContext<'__geam_list, #host_item, #decoder>,
        >
    }
}

fn provider_input_signature_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    match type_ {
        ProviderValueType::Scalar(type_) => type_.clone(),
        ProviderValueType::Generic(value) => generic_value_signature_type(value, customs, support),
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::Owned,
        } => type_.clone(),
        ProviderValueType::Declared {
            type_,
            input: DeclaredInput::BorrowedExternal,
        } => syn::parse_quote!(<#type_ as #support::ProviderValue>::Input),
        ProviderValueType::External { payload, .. } => {
            syn::parse_quote!(#support::ProviderExternalItem<#payload>)
        }
        ProviderValueType::Custom { rust, .. } => rust.clone(),
        ProviderValueType::List(list) => {
            let item = &list.collection.item;
            let host_item = host_static_value_type(&list.collection.value, customs, support);
            let decoder = &list.decoder;
            syn::parse_quote! {
                #support::List<
                    #item,
                    #support::ProviderListContext<'__geam_call, #host_item, #decoder>,
                >
            }
        }
        ProviderValueType::Tuple(elements) => {
            let mut types = Vec::with_capacity(elements.len());
            for element in elements {
                types.push(provider_input_signature_type(element, customs, support));
            }
            syn::parse_quote!((#(#types,)*))
        }
        ProviderValueType::Result { success, failure } => {
            let success = provider_input_signature_type(success, customs, support);
            let failure = provider_input_signature_type(failure, customs, support);
            syn::parse_quote!(::core::result::Result<#success, #failure>)
        }
        ProviderValueType::Option { value } => {
            let value = provider_input_signature_type(value, customs, support);
            syn::parse_quote!(::core::option::Option<#value>)
        }
    }
}

fn generic_value_signature_type(
    value: &GenericValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    let source = &value.source;
    let host = generic_host_type(&value.host, customs, support);
    let mut path = value.path.clone();
    for segment in path.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote! {
            <#source, #support::ProviderValueContext<'__geam_call, #host>>
        });
    }
    Type::Path(path)
}

fn generic_external_host_type(
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

fn generate_generic_external_payload(
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
            let values = fields
                .iter()
                .map(|(ident, value)| quote!(#ident: #value.into_host()));
            GeneratedValue {
                statements: quote! {
                    let #output { #(#patterns,)* } = #input;
                    let #payload = ::core::result::Result::<
                        #payload_type,
                        #support::ProviderExternalItem<#payload_type>,
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

fn generic_external_input_signature_type(
    external: &GenericExternalType,
    customs: &[CustomModel],
    support: &TokenStream,
    source: GenericInputSource,
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
        GenericExternalStorage::ManualPayload { payload } => quote!(#payload),
    };
    arguments.push(syn::parse_quote! {
        #support::ProviderExternalInputContext<'__geam_call, #payload, #host_arguments>
    });
    let input = &external.input;
    syn::parse_quote!(#input<#(#arguments),*>)
}

fn generic_external_output_signature_type(
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
                    #support::ProviderStoredOutput<'__geam_call, #owner, #index, #host>
                });
            }
        }
        GenericExternalStorage::ManualPayload { payload } => {
            arguments.push(syn::parse_quote! {
                #support::ProviderExternalOutput<#payload>
            });
        }
    }
    let output = &external.output;
    syn::parse_quote!(#output<#(#arguments),*>)
}

fn callback_signature_type(
    callback: &CallbackType,
    generics: &[Ident],
    profile: &TokenStream,
    return_type: &TokenStream,
    support: &TokenStream,
) -> Type {
    let signature = &callback.signature;
    let codec = callback_codec_type(
        &callback.codec,
        generics.iter().map(|ident| quote!(#ident)).collect(),
    );
    let mut path = callback.path.clone();
    for segment in path.path.segments.iter_mut().rev().take(1) {
        segment.arguments = PathArguments::AngleBracketed(syn::parse_quote! {
            <
                #signature,
                #support::ProviderCallbackContext<
                    '__geam_call,
                    #profile,
                    __GeamProvider,
                    #return_type,
                    #codec,
                >,
            >
        });
    }
    Type::Path(path)
}

fn callback_codec_type(codec: &Ident, arguments: Vec<TokenStream>) -> TokenStream {
    if arguments.is_empty() {
        quote!(#codec)
    } else {
        quote!(#codec<#(#arguments),*>)
    }
}

fn callback_output_signature_type(
    type_: &FunctionReturnType,
    customs: &[CustomModel],
    support: &TokenStream,
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
        FunctionReturnType::List(list) => callback_list_signature_type(list, customs, support),
    }
}

fn callback_input_signature_type(
    type_: &FunctionInputType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    match type_ {
        FunctionInputType::Value(value) => {
            let value = provider_value_from_input_root(value);
            provider_input_signature_type(&value, customs, support)
        }
        FunctionInputType::Generic(value) => generic_value_signature_type(value, customs, support),
        FunctionInputType::External(external) => generic_external_input_signature_type(
            external,
            customs,
            support,
            GenericInputSource::Declared,
        ),
        FunctionInputType::List(list) => callback_list_signature_type(list, customs, support),
    }
}

fn callback_list_signature_type(
    list: &ListType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Type {
    let item = &list.collection.item;
    let host_item = host_static_value_type(&list.collection.value, customs, support);
    let decoder = &list.decoder;
    syn::parse_quote! {
        #support::List<
            #item,
            #support::ProviderListContext<'__geam_call, #host_item, #decoder>,
        >
    }
}

fn function_output_rust_type(
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

fn instantiated_generic_source_type(value: &GenericValueType) -> Type {
    value.instantiated.clone()
}

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

fn take_function_marker(attributes: &mut Vec<Attribute>) -> syn::Result<Option<FunctionArguments>> {
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

fn build_external_model(
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

fn host_type_index(index: usize, support: &TokenStream) -> TokenStream {
    let mut value = quote!(#support::HostTypeIndex0);
    for _ in 0..index {
        value = quote!(#support::HostTypeIndexNext<#value>);
    }
    value
}

fn retained_parameter_accessor(parameter: &Ident) -> Ident {
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

fn validate_function(
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

fn find_unwrapped_generic(type_: &Type, generics: &GenericParameterScope) -> Option<Ident> {
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

fn collection_item(type_: &Type, name: &str) -> syn::Result<Option<Type>> {
    Ok(collection_item_with_path(type_, name)?.map(|(item, _)| item))
}

fn collection_item_with_path(type_: &Type, name: &str) -> syn::Result<Option<(Type, TypePath)>> {
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
    Ok(Some((
        item.clone(),
        TypePath {
            qself: None,
            path: path.clone(),
        },
    )))
}

fn source_wrapper(type_: &Type) -> syn::Result<SourceWrapper<'_>> {
    if host_result_value(type_)?.is_some() {
        return Err(syn::Error::new_spanned(
            type_,
            "HostResult<T> is supported only as the outer provider return",
        ));
    }
    let Type::Path(type_path @ TypePath { qself: None, .. }) = type_ else {
        return Ok(SourceWrapper::Other);
    };
    let mut arguments = SourceWrapperArguments::Other;
    for segment in &type_path.path.segments {
        arguments = if segment.ident == "Result" {
            SourceWrapperArguments::Result(&segment.arguments)
        } else if segment.ident == "Option" {
            SourceWrapperArguments::Option(&segment.arguments)
        } else {
            SourceWrapperArguments::Other
        };
    }
    match arguments {
        SourceWrapperArguments::Result(PathArguments::AngleBracketed(arguments)) => {
            let mut arguments = arguments.args.iter();
            match (arguments.next(), arguments.next(), arguments.next()) {
                (
                    Some(GenericArgument::Type(success)),
                    Some(GenericArgument::Type(failure)),
                    None,
                ) => {
                    if is_type_named(failure, "HostFailure") {
                        Err(syn::Error::new_spanned(
                            type_,
                            "Result<T, HostFailure> is not a source Result; use HostResult<T>",
                        ))
                    } else {
                        Ok(SourceWrapper::Result {
                            path: type_path,
                            success,
                            failure,
                        })
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    type_,
                    "Result requires exactly 2 type arguments",
                )),
            }
        }
        SourceWrapperArguments::Option(PathArguments::AngleBracketed(arguments)) => {
            let mut arguments = arguments.args.iter();
            match (arguments.next(), arguments.next()) {
                (Some(GenericArgument::Type(value)), None) => Ok(SourceWrapper::Option {
                    path: type_path,
                    value,
                }),
                _ => Err(syn::Error::new_spanned(
                    type_,
                    "Option requires exactly 1 type argument",
                )),
            }
        }
        SourceWrapperArguments::Result(_) => Err(syn::Error::new_spanned(
            type_,
            "Result requires exactly 2 type arguments",
        )),
        SourceWrapperArguments::Option(_) => Err(syn::Error::new_spanned(
            type_,
            "Option requires exactly 1 type argument",
        )),
        SourceWrapperArguments::Other => Ok(SourceWrapper::Other),
    }
}

fn host_result_value(type_: &Type) -> syn::Result<Option<(Type, TypePath)>> {
    collection_item_with_path(type_, "HostResult")
}

fn is_type_named(type_: &Type, name: &str) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path })
            if path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

fn is_collection(type_: &Type, name: &str) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path })
            if path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

fn is_type_application_named(type_: &Type, name: &str) -> bool {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return false;
    };
    path.segments.last().is_some_and(|segment| {
        segment.ident == name && matches!(segment.arguments, PathArguments::AngleBracketed(_))
    })
}

fn is_non_empty_tuple(type_: &Type) -> bool {
    matches!(type_, Type::Tuple(tuple) if !tuple.elems.is_empty())
}

fn is_qualified_type_path(type_: &Type) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path }) if path.segments.len() > 1
    )
}

fn is_declared_provider_type(type_: &Type) -> syn::Result<bool> {
    is_advanced_external(type_).map(|external| external | is_qualified_type_path(type_))
}

fn is_advanced_external(type_: &Type) -> syn::Result<bool> {
    if !is_type_application_named(type_, "External") {
        return Ok(false);
    }
    collection_item(type_, "External")?;
    Ok(true)
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
    use super::{
        ExternalArguments, GenericParameterScope, ModuleArguments, build_external_model,
        classify_collection_input_item, expand, find_unwrapped_generic, list_item_view_type,
        retained_parameter_accessor, static_value_key,
    };
    use quote::quote;
    use syn::{ItemFn, Type};

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
    fn explicit_module_profile_wires_one_static_component_projection() {
        let expansion = expand(
            quote!(
                path = "random",
                crate_path = geam_core,
                profile = crate::BuiltInProfile,
                component = crate::Component<Profile::Io>
            ),
            quote! {
                mod random {
                    #[geam::function(profile = Profile)]
                    fn next(
                        #[geam::call] call: &mut Call<RunState<Profile::Io>>,
                    ) -> f64 {
                        call.state_mut().next()
                    }
                }
            },
        )
        .expect("built-in profile wiring should expand")
        .to_string();

        assert!(
            expansion.contains("pub (super) trait __GeamModuleProfile : crate :: BuiltInProfile")
        );
        assert!(expansion.contains("pub (super) fn __geam_module < Profile >"));
        assert!(!expansion.contains("ProviderModuleRegistration"));
        assert!(expansion.contains(
            "Profile as geam_core :: __macro_support :: HostComponentProfile < crate :: Component < Profile :: Io > >> :: component_state"
        ));
        assert!(expansion.contains("fn next < '__geam_call , Profile >"));
        assert!(expansion.contains("Profile : crate :: BuiltInProfile"));
        assert!(expansion.contains("next :: < Profile >"));
        assert!(!expansion.contains("__GeamProfile"));
        assert!(
            expansion
                .contains("# [doc (hidden)] # [derive (Default)] pub (super) struct __GeamStores")
        );
    }

    #[test]
    fn explicit_external_modules_require_one_static_store_projection() {
        let item = quote! {
            mod token {
                #[geam::external(name = "Token")]
                #[derive(PartialEq, Eq, Hash)]
                struct Token;
            }
        };
        let missing = expand(
            quote!(
                path = "token",
                crate_path = geam_core,
                profile = crate::BuiltInProfile,
                component = crate::Component<Profile::Io>
            ),
            item.clone(),
        )
        .expect_err("built-in external storage must have an explicit projection");
        assert_eq!(
            missing.to_string(),
            "built-in modules with external declarations require a `stores` projection",
        );

        let unused = expand(
            quote!(
                path = "token",
                crate_path = geam_core,
                profile = crate::BuiltInProfile,
                component = crate::Component<Profile::Io>,
                stores = crate::token_stores
            ),
            quote!(
                mod token {}
            ),
        )
        .expect_err("store projections without external declarations should fail");
        assert_eq!(
            unused.to_string(),
            "module `stores` is used only by external declarations",
        );

        let expansion = expand(
            quote!(
                path = "token",
                crate_path = geam_core,
                profile = crate::BuiltInProfile,
                component = crate::Component<Profile::Io>,
                stores = crate::token_stores
            ),
            item,
        )
        .expect("one explicit store projection should expand")
        .to_string();
        assert!(
            expansion
                .contains("& crate :: token_stores :: < Profile > (stores) . __geam_external_0")
        );
        assert!(
            expansion.contains("# [doc (hidden)] # [derive (Default)] pub struct __GeamStores")
        );
        assert!(expansion.contains("# [doc (hidden)] pub struct __GeamExternalStorage0"));
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
                "generic parameter `T` must appear inside a Value<...> source shape",
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
                "provider functions must not have where clauses",
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
    fn call_injection_is_reference_only_unique_and_first() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(label: String, #[geam::call] call: &mut Call<RunState>) -> String {
                        label
                    }
                }
            )),
            "the `#[geam::call]` parameter must be first",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::call] call: Call<RunState>) -> String {
                        String::new()
                    }
                }
            )),
            "the `#[geam::call]` parameter must be `&Call<State>` or `&mut Call<State>`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::call(value)] call: &mut Call<RunState>) -> String {
                        String::new()
                    }
                }
            )),
            "`#[geam::call]` does not accept arguments",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(
                        #[geam::call]
                        #[geam::call]
                        call: &mut Call<RunState>,
                    ) -> String {
                        String::new()
                    }
                }
            )),
            "duplicate `#[geam::call]` attribute",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(call: &Call<RunState>) -> String {
                        String::new()
                    }
                }
            )),
            "Call<State> parameters require `#[geam::call]`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::call] call: &Other) -> String {
                        String::new()
                    }
                }
            )),
            "the `#[geam::call]` parameter must be `&Call<State>` or `&mut Call<State>`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::call] call: &Call) -> String {
                        String::new()
                    }
                }
            )),
            "Call requires exactly one type argument",
        );

        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    #[geam::function]
                    fn next(
                        #[allow(unused_variables)]
                        #[geam::call]
                        call: &Call<RunState>,
                    ) -> bool {
                        true
                    }
                }
            },
        )
        .expect("non-call parameter attributes should be retained")
        .to_string();
        assert!(expansion.contains("# [allow (unused_variables)] call"));
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::call] call: &Call<RunState, Other>) -> String {
                        String::new()
                    }
                }
            )),
            "Call requires exactly one type argument",
        );
    }

    #[test]
    fn callback_shape_and_active_call_diagnostics_are_exact() {
        let cases = [
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(callback: Callback<fn() -> bool>) -> bool { true }
                    }
                },
                "Callback arguments require a first `#[geam::call]` parameter using `&mut Call<State>`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &Call<RunState>,
                            callback: Callback<fn() -> bool>,
                        ) -> bool { true }
                    }
                },
                "Callback arguments require a first `#[geam::call]` parameter using `&mut Call<State>`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: &Callback<fn() -> bool>,
                        ) -> bool { true }
                    }
                },
                "Callback arguments must be passed by value",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<bool>,
                        ) -> bool { true }
                    }
                },
                "Callback<T> requires a safe non-variadic Rust fn signature",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<unsafe fn() -> bool>,
                        ) -> bool { true }
                    }
                },
                "Callback<T> requires a safe non-variadic Rust fn signature without lifetimes",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn(
                                bool, bool, bool, bool, bool, bool, bool, bool,
                            ) -> bool>,
                        ) -> bool { true }
                    }
                },
                "provider callbacks support at most seven source arguments",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn() -> Callback<fn() -> bool>>,
                        ) -> bool { true }
                    }
                },
                "callbacks returned by a callback are opaque values; use Value<fn(...) -> ...>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn(Callback<fn() -> bool>) -> bool>,
                        ) -> bool { true }
                    }
                },
                "callback arguments that are functions must use Value<fn(...) -> ...>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<bool, bool>,
                        ) -> bool { true }
                    }
                },
                "Callback requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn() -> List>,
                        ) -> bool { true }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn(Vec) -> bool>,
                        ) -> bool { true }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke<Item>(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn() -> Item>,
                        ) -> bool { true }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke<Item>(
                            #[geam::call] call: &mut Call<RunState>,
                            callback: Callback<fn(Item) -> bool>,
                        ) -> bool { true }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn callbacks_generate_one_static_directional_codec_and_callable_abi() {
        let expansion = expand(
            quote!(path = "callbacks", crate_path = geam_core),
            quote! {
                mod callbacks {
                    #[geam::external(name = "Token")]
                    struct Token;

                    #[geam::custom(input = StatusInput)]
                    enum Status {
                        Ready(String),
                    }

                    #[geam::function]
                    fn around<Item>(
                        #[geam::call] call: &mut Call<RunState>,
                        callback: Callback<fn(Value<Item>, Token, Status) -> Value<Item>>,
                    ) -> HostResult<Value<Item>> {
                        todo!()
                    }
                }
            },
        )
        .expect("typed callback declaration should expand")
        .to_string();

        assert!(expansion.contains("struct __GeamCallbackCodec_around_0 < Item >"));
        assert!(expansion.contains("ProviderCallbackCodec < '__geam_call"));
        assert!(
            expansion.contains("type HostArguments = geam_core :: __macro_support :: HostTypeList")
        );
        assert!(expansion.contains(
            "type HostReturn = geam_core :: __macro_support :: HostTypeParameter < 0usize >"
        ));
        assert!(expansion.contains("HostFunctionType <"));
        assert!(!expansion.contains("HostOpaqueFunctionType < geam_core :: __macro_support :: HostTypeList < geam_core :: __macro_support :: HostExternalType"));
        assert!(expansion.contains("call . construct_external_with_binding"));
        assert!(expansion.contains("ProviderOutputValue"));
        assert!(expansion.contains("__geam_callback_argument_0 . into_host ()"));
        assert!(
            expansion.contains(
                "Callback :: < _ , geam_core :: __macro_support :: ProviderCallbackContext"
            )
        );
        assert!(expansion.contains("with_scoped_function_and_constructions"));
    }

    #[test]
    fn callback_native_compounds_and_lists_keep_directional_rust_signatures() {
        let expansion = expand(
            quote!(path = "callbacks", crate_path = geam_core),
            quote! {
                mod callbacks {
                    #[geam::external(name = "Token")]
                    struct Token;

                    #[geam::custom(input = StatusInput)]
                    enum Status {
                        Ready(String),
                    }

                    #[geam::function]
                    fn invoke(
                        #[geam::call] call: &mut Call<RunState>,
                        callback: Callback<fn(
                            ((String, Token), other::Payload),
                            Result<String, Status>,
                            Option<String>,
                            List<Token>,
                            Vec<(Token, other::Payload)>,
                            other::Payload,
                        ) -> List<Token>>,
                        values: List<Token>,
                    ) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn notify(
                        #[geam::call] call: &mut Call<RunState>,
                        callback: Callback<fn()>,
                    ) -> bool {
                        todo!()
                    }
                }
            },
        )
        .expect("native compound callback declaration should expand")
        .to_string();

        assert!(expansion.contains(
            "type Arguments = (((String , Token ,) , other :: Payload ,) , :: core :: result :: Result < String , Status > , :: core :: option :: Option < String > , geam_core :: __macro_support :: List < Token"
        ));
        assert!(expansion.contains(":: std :: vec :: Vec < (Token , other :: Payload ,) >"));
        assert!(expansion.contains("other :: Payload"));
        assert!(expansion.contains("type Returned = geam_core :: __macro_support :: List < Token"));
        assert!(expansion.contains("__geam_into_context () . into_host ()"));
        assert!(expansion.contains("ProviderListInputCodec < Profile >"));
        assert!(expansion.contains("type HostReturn = ()"));
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
            "unknown function argument `value`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(profile = Profile)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "function `profile` requires module `profile` and `component`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(profile = First, profile = Second)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "duplicate function argument `profile`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(profile)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "expected `=`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(= Profile)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "expected identifier",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(profile =)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "unexpected end of input, expected identifier",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(profile = First profile = Second)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "expected `,`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function = "profile"]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "`#[geam::function]` accepts only `profile = Name`",
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
                    fn shared(#[geam::call] call: &Call<RunState>, value: bool) -> bool { value }

                    #[geam::function]
                    fn mutable(#[geam::call] call: &mut Call<RunState>, value: bool) -> bool { value }
                }
            },
        )
        .expect("module should expand")
        .to_string();

        assert!(expansion.contains("fn helper"));
        assert!(expansion.contains("const ENABLED"));
        assert_eq!(expansion.matches("call . state ()").count(), 1);
        assert!(expansion.contains("let __geam_state = & * call . state ()"));
        assert!(expansion.contains("Call :: from_shared_state (__geam_state)"));
        assert!(expansion.contains("shared (& __geam_provider_call , __geam_argument_0)"));
        assert!(expansion.contains("Call :: from_host_call (call)"));
        assert!(expansion.contains("mutable (& mut __geam_provider_call , __geam_argument_0)"));
        assert!(expansion.contains("__geam_provider_call . into_host_call ()"));
        let first = expansion
            .find("\"first\"")
            .expect("first registration should exist");
        let second = expansion
            .find("\"second\"")
            .expect("second registration should exist");
        assert!(first < second);
    }

    #[test]
    fn generic_value_expansion_preserves_author_generics_and_return_first_indices() {
        let expansion = expand(
            quote!(path = "generic_values", crate_path = geam_core),
            quote! {
                mod generic_values {
                    fn helper() {}

                    #[geam::external(name = "Token")]
                    struct Token;

                    #[geam::custom(input = ProblemInput)]
                    enum Problem {
                        Missing,
                    }

                    #[geam::function]
                    fn select<First, Second>(
                        first: Value<First>,
                        second: Value<Second>,
                    ) -> Value<Second> {
                        let _ = first;
                        second
                    }

                    #[geam::function]
                    fn concrete<Item>(value: Value<(Item, EcoString)>) -> Value<(Item, EcoString)> {
                        value
                    }

                    #[geam::function]
                    fn declared<Item>(
                        value: Value<(Item, sibling::MarkerInput)>,
                    ) -> Value<(Item, sibling::MarkerInput)> {
                        value
                    }

                    #[geam::function]
                    fn external<Item>(
                        value: Value<(Item, Token)>,
                    ) -> Value<(Item, Token)> {
                        value
                    }

                    #[geam::function]
                    fn custom<Item>(
                        value: Value<(Item, ProblemInput)>,
                    ) -> Value<(Item, ProblemInput)> {
                        value
                    }

                    #[geam::function]
                    fn fallible<Item>(value: Value<Item>) -> HostResult<Value<Item>> {
                        Ok(value)
                    }

                    #[geam::function]
                    fn nullary<Item>(value: Value<fn() -> Item>) -> Value<fn() -> Item> {
                        value
                    }

                    #[geam::function]
                    fn unary<Item>(value: Value<fn(Item) -> bool>) -> Value<fn(Item) -> bool> {
                        value
                    }

                    #[geam::function]
                    fn binary<Item>(
                        value: Value<fn(bool, Item) -> bool>,
                    ) -> Value<fn(bool, Item) -> bool> {
                        value
                    }

                    #[geam::function]
                    fn nullary_unit(value: Value<fn()>) -> Value<fn()> {
                        value
                    }

                    #[geam::function]
                    fn list_value<Item>(value: Value<List<Item>>) -> Value<List<Item>> {
                        value
                    }

                    #[geam::function]
                    fn failure_value<Item>(
                        value: Value<Result<EcoString, Item>>,
                    ) -> Value<Result<EcoString, Item>> {
                        value
                    }

                    #[geam::function]
                    fn optional<Item>(
                        value: Value<Option<Item>>,
                    ) -> Value<Option<Item>> {
                        value
                    }

                    #[geam::function]
                    fn mutable<Item>(
                        #[geam::call] call: &mut Call<RunState>,
                        value: Value<Item>,
                    ) -> Value<Item> {
                        let _ = call;
                        value
                    }

                    #[geam::function]
                    fn present<Item>(value: Value<Item>) -> bool {
                        let _ = value;
                        true
                    }
                }
            },
        )
        .expect("generic source values should expand");
        let module = syn::parse2::<syn::ItemMod>(expansion.clone())
            .expect("expanded provider module should remain valid Rust syntax");
        let (_, items) = module.content.expect("expanded module must remain inline");
        let function = items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(function) if function.sig.ident == "select" => Some(function),
                _ => None,
            })
            .expect("author function should remain in the expanded module");
        let generic_names = function
            .sig
            .generics
            .type_params()
            .map(|parameter| parameter.ident.to_string())
            .collect::<Vec<_>>();

        assert_eq!(generic_names, ["First", "Second"]);
        assert_eq!(function.sig.generics.lifetimes().count(), 1);

        let expansion = expansion.to_string();
        assert!(expansion.contains(
            "select :: < geam_core :: __macro_support :: HostTypeParameter < 1usize > , geam_core :: __macro_support :: HostTypeParameter < 0usize > >"
        ));
        assert!(expansion.contains(
            "< sibling :: MarkerInput as geam_core :: __macro_support :: ProviderValue > :: Host"
        ));
        assert!(expansion.contains("__GeamExternalSchema0"));
        assert!(expansion.contains("__GeamCustom0Constructor0"));
        assert!(expansion.contains("let returned = returned ?"));
        assert!(!expansion.contains("HostConstructions"));
    }

    #[test]
    fn generic_value_expansion_composes_with_explicit_profiles() {
        let expansion = expand(
            quote!(
                path = "generic_values",
                crate_path = geam_core,
                profile = crate::BuiltInProfile,
                component = crate::Component<Profile::Io>
            ),
            quote! {
                mod generic_values {
                    #[geam::function(profile = Profile)]
                    fn identity<Item>(value: Value<Item>) -> Value<Item> {
                        value
                    }
                }
            },
        )
        .expect("generic source values should compose with a built-in profile")
        .to_string();

        assert!(expansion.contains(
            "identity :: < geam_core :: __macro_support :: HostTypeParameter < 0usize > , Profile >"
        ));
    }

    #[test]
    fn generic_value_declaration_diagnostics_are_exact() {
        let cases = [
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<'value>(value: Value<bool>) -> Value<bool> { value }
                    }
                },
                "provider functions must not have lifetime generics",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<const INDEX: usize>(value: bool) -> bool { value }
                    }
                },
                "provider functions must not have const generics",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item: Clone>(value: Value<Item>) -> Value<Item> { value }
                    }
                },
                "provider function type generics must not have bounds",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item = bool>(value: Value<Item>) -> Value<Item> { value }
                    }
                },
                "provider function type generics must not have defaults",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Item>) -> Value<Item>
                        where
                            Item: Clone,
                        {
                            value
                        }
                    }
                },
                "provider functions must not have where clauses",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Item) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke<Item>(callback: fn(bool, Item) -> bool) -> Value<Item> {
                            todo!()
                        }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke<Item>(callback: fn(Item) -> bool) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn invoke<Item>(callback: fn() -> Item) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Option<Item>) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: (bool, Item)) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: (Item)) -> Value<Item> { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Item>) -> Item { todo!() }
                    }
                },
                "generic source type `Item` must be written as Value<Item>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn constant<Item>(value: bool) -> bool { value }
                    }
                },
                "generic parameter `Item` must appear inside a Value<...> source shape",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity(value: Value<bool>) -> Value<bool> { value }
                    }
                },
                "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity(value: Value<(EcoString, bool)>) -> bool { true }
                    }
                },
                "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Value<Item>>) -> Value<Item> { todo!() }
                    }
                },
                "Value<...> wrappers must not be nested inside generic source shapes",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<&Item>) -> Value<Item> { todo!() }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Option<&Item>>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Vec<Item>>) -> Value<Item> { todo!() }
                    }
                },
                "Vec<T> is not a generic source type; use geam::List<T>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<sibling::Box<Item>>) -> Value<Item> {
                            todo!()
                        }
                    }
                },
                "generic declared source types are not supported inside generic source shapes",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(values: geam::List<Value<Item>>) -> bool { true }
                    }
                },
                "Value<...> cannot be a List item; wrap the complete generic source shape in Value<...>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: (Value<Item>, bool)) -> bool { true }
                    }
                },
                "Value<...> must be the complete source argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity(value: Value) -> bool { true }
                    }
                },
                "Value requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<First, Second>(value: Value<First, Second>) -> bool { true }
                    }
                },
                "Value requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<for<'value> fn(Item) -> Item>) -> bool {
                            true
                        }
                    }
                },
                "opaque source function shapes must not declare lifetimes",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<unsafe fn(Item) -> Item>) -> bool { true }
                    }
                },
                "opaque source function shapes must use safe non-variadic Rust fn syntax",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<List<&Item>>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result<&Item, Item>>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result<Item, &Item>>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<fn() -> &Item>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<sibling::Box<Item, bool>>) -> bool { true }
                    }
                },
                "generic declared source types are not supported inside generic source shapes",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<List>) -> Value<Item> { todo!() }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Vec>) -> Value<Item> { todo!() }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result>) -> Value<Item> { todo!() }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<(Item, &Item)>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<fn(&Item) -> Item>) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<List<EcoString>>) -> Value<Item> { todo!() }
                    }
                },
                "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Option<EcoString>>) -> Value<Item> { todo!() }
                    }
                },
                "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result<EcoString, bool>>) -> Value<Item> { todo!() }
                    }
                },
                "Value<T> is reserved for generic source shapes and opaque function values; use the concrete provider type directly",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: (Value<&Item>, bool)) -> bool { true }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Item>) -> Option<Value<&Item>> { todo!() }
                    }
                },
                "generic source shapes must not contain Rust references",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }

        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    #[geam::function]
                    fn identity<Item>(value: Value<Item>) -> Option<Value<Item>> { todo!() }
                }
            },
        )
        .expect("generic values should compose inside source output shapes")
        .to_string();
        assert!(expansion.contains(
            "ProviderOption < geam_core :: __macro_support :: HostTypeParameter < 0usize > >"
        ));
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
    fn generic_external_declarations_require_one_static_stored_contract() {
        let cases = [
            (
                quote! {
                    mod generic {
                        #[geam::external(name = "Box", input = BoxInput)]
                        struct BoxValue;
                    }
                },
                "external argument `input` requires non-empty `parameters`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(name = "Box", payload = Payload)]
                        struct BoxValue;
                    }
                },
                "external argument `payload` requires non-empty `parameters`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                            retained,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "external argument `retained` is only supported for non-generic payloads",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(name = "Box", parameters = [Item])]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external declarations require `input = Type`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                            manual,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external `manual` semantics require an explicit retained `payload = Type`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Queue",
                            parameters = [Item],
                            input = QueueInput,
                            payload = QueuePayload,
                        )]
                        struct Queue<Item>;
                    }
                },
                "generic external `payload` requires bare `manual` semantics",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Queue",
                            parameters = [Item],
                            input = QueueInput,
                            payload = QueuePayload<Item>,
                            manual,
                        )]
                        struct Queue<Item>;
                    }
                },
                "generic external `payload` must be a non-generic type path",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Queue",
                            parameters = [Item],
                            input = QueueInput,
                            payload = (QueuePayload, OtherPayload),
                            manual,
                        )]
                        struct Queue<Item>;
                    }
                },
                "generic external `payload` must be a non-generic type path",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Queue",
                            parameters = [Item],
                            input = QueueInput,
                            payload = QueuePayload,
                            manual,
                        )]
                        struct Queue<Item> { marker: PhantomData<Item> }
                    }
                },
                "generic external declarations with an explicit payload require a unit marker struct",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<'a> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external declarations support only type parameters",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item: Clone> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external type parameters must not have bounds or defaults",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Other],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "external `parameters` must list every Rust type parameter once in declaration order",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item, Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item, Other> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "external `parameters` must list every Rust type parameter once in declaration order",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item>(Stored<Item>);
                    }
                },
                "generic external declarations require named `#[geam::stored]` fields",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {}
                    }
                },
                "generic external declarations require at least one `#[geam::stored]` field",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external fields must be marked `#[geam::stored]`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Item,
                        }
                    }
                },
                "`#[geam::stored]` fields must use `Stored<Parameter>`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Vec<Item>>,
                        }
                    }
                },
                "`#[geam::stored]` fields must name one declared external parameter",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Pair",
                            parameters = [Left, Right],
                            input = PairInput,
                        )]
                        struct Pair<Left, Right> {
                            #[geam::stored]
                            left: Stored<Left>,
                        }
                    }
                },
                "external parameter `Right` must own at least one `#[geam::stored]` field",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(name = "Token")]
                        struct Token {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "`#[geam::stored]` fields require non-empty external `parameters`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(name = "Token")]
                        struct Token {
                            #[geam::stored(value)]
                            value: bool,
                        }
                    }
                },
                "`#[geam::stored]` does not accept arguments",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored(value)]
                            value: Stored<Item>,
                        }
                    }
                },
                "`#[geam::stored]` does not accept arguments",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item, Item>,
                        }
                    }
                },
                "Stored requires exactly one type argument",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item>
                        where
                            Item: Sized,
                        {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generic external declarations must not have where clauses",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item, Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item, Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "duplicate external parameter `Item`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<(Item,)>,
                        }
                    }
                },
                "`#[geam::stored]` fields must name one declared external parameter",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Other>,
                        }
                    }
                },
                "`#[geam::stored]` fields must name one declared external parameter",
            ),
            (
                quote! {
                    mod generic {
                        struct QueuePayload;

                        #[geam::external(
                            name = "Queue",
                            parameters = [URLValue, UrlValue],
                            input = QueueInput,
                            payload = QueuePayload,
                            manual,
                        )]
                        struct Queue<URLValue, UrlValue>;
                    }
                },
                "external parameter `UrlValue` generates duplicate retained accessor `stored_url_value`",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
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

    #[test]
    fn generic_external_functions_keep_call_access_optional() {
        let expansion = expand(
            quote!(path = "generic", crate_path = geam_core),
            quote! {
                mod generic {
                    #[geam::external(
                        name = "Box",
                        parameters = [Item],
                        input = BoxInput,
                    )]
                    struct BoxValue<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }

                    #[geam::function]
                    fn present<Item>(boxed: BoxInput<Item>) -> bool {
                        let _ = boxed;
                        true
                    }

                    #[geam::function]
                    fn present_with_state<Item>(
                        #[geam::call] call: &Call<RunState>,
                        boxed: BoxInput<Item>,
                    ) -> bool {
                        let _ = (call, boxed);
                        true
                    }
                }
            },
        )
        .expect("generic external inputs should not require mutable call access")
        .to_string();

        assert!(expansion.contains("fn present < '__geam_call , Item >"));
        assert!(expansion.contains("fn present_with_state < '__geam_call , Item >"));
        assert!(expansion.contains("Call :: from_shared_state"));
    }

    #[test]
    fn generic_external_callbacks_keep_input_and_output_directions_distinct() {
        let expansion = expand(
            quote!(path = "generic", crate_path = geam_core),
            quote! {
                mod generic {
                    #[geam::external(
                        name = "Box",
                        parameters = [Item],
                        input = BoxInput,
                    )]
                    struct BoxValue<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }

                    #[geam::function]
                    fn output_only<Item>() -> BoxValue<Item> { todo!() }

                    #[geam::function]
                    fn send<Input, Output>(
                        #[geam::call] call: &mut Call<()>,
                        value: Value<Input>,
                        callback: Callback<fn(BoxValue<Input>) -> Value<Output>>,
                    ) -> HostResult<Value<Output>> { todo!() }

                    #[geam::function]
                    fn receive<Item>(
                        #[geam::call] call: &mut Call<()>,
                        callback: Callback<fn() -> BoxInput<Item>>,
                    ) -> HostResult<Value<Item>> { todo!() }
                }
            },
        )
        .expect("generic external callbacks should expand")
        .to_string();

        assert!(expansion.contains("fn output_only < '__geam_call , Item >"));
        assert!(expansion.contains("construct_external_with_binding"));
        assert!(expansion.contains("ProviderExternalInputContext < '__geam_call"));
        assert!(expansion.contains("fn send < '__geam_call"));
        assert!(expansion.contains("fn receive < '__geam_call"));
    }

    #[test]
    fn generic_external_functions_require_directional_types() {
        let declaration = quote! {
            #[geam::external(
                name = "Box",
                parameters = [Item],
                input = BoxInput,
            )]
            struct BoxValue<Item> {
                #[geam::stored]
                value: Stored<Item>,
            }
        };
        let cases = [
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(
                            #[geam::call] call: &mut Call<()>,
                            boxed: BoxValue<Item>,
                        ) -> Value<Item> { todo!() }
                    }
                },
                "generic external output `BoxValue` cannot be used as input; use `BoxInput<...>`",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(
                            #[geam::call] call: &mut Call<()>,
                            boxed: BoxInput<Item>,
                        ) -> BoxInput<Item> { todo!() }
                    }
                },
                "generic external input `BoxInput` cannot be returned; return `BoxValue<...>`",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong(
                            #[geam::call] call: &mut Call<()>,
                            boxed: BoxInput,
                        ) -> bool { true }
                    }
                },
                "generic external `BoxInput` requires exactly 1 type arguments",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item, Other>(
                            #[geam::call] call: &mut Call<()>,
                            boxed: BoxInput<Item, Other>,
                        ) -> bool { true }
                    }
                },
                "generic external `BoxInput` requires exactly 1 type arguments",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong(
                            boxed: BoxInput<'static>,
                        ) -> bool { true }
                    }
                },
                "generic external arguments must be source types",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(boxed: BoxValue<'static>) -> bool { true }
                    }
                },
                "generic external arguments must be source types",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(boxed: Value<Item>) -> BoxInput<'static> { todo!() }
                    }
                },
                "generic external arguments must be source types",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(boxed: Value<Item>) -> BoxValue<'static> { todo!() }
                    }
                },
                "generic external arguments must be source types",
            ),
            (
                quote! {
                    mod generic {
                        #declaration
                        #[geam::function]
                        fn wrong<Item>(
                            boxed: BoxInput<Vec<Item>>,
                        ) -> bool { true }
                    }
                },
                "Vec<T> is not a generic source type; use geam::List<T>",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn generic_external_expansion_owns_one_store_and_positional_retention() {
        let expansion = expand(
            quote!(path = "generic_box", crate_path = geam_core),
            quote! {
                mod generic_box {
                    #[geam::external(
                        name = "Box",
                        parameters = [Item],
                        input = BoxInput,
                    )]
                    pub struct BoxValue<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }

                    #[geam::function]
                    fn replace<Old, New>(
                        #[geam::call] call: &mut Call<()>,
                        old: BoxInput<Old>,
                        value: Value<New>,
                    ) -> BoxValue<New> {
                        todo!()
                    }

                    #[geam::function]
                    fn try_replace<Old, New>(
                        #[geam::call] call: &mut Call<()>,
                        old: BoxInput<Old>,
                        value: Value<New>,
                    ) -> HostResult<BoxValue<New>> {
                        todo!()
                    }
                }
            },
        )
        .expect("generic external declaration should expand")
        .to_string();

        assert!(expansion.contains("const PARAMETER_COUNT : usize = 1usize"));
        assert_eq!(
            expansion
                .matches("HostExternalStore < __GeamExternalPayload0 >")
                .count(),
            1,
        );
        assert!(expansion.contains("type Payload = __GeamExternalPayload0"));
        assert!(expansion.contains("ProviderStoredInput < '_ , __GeamExternalOwner0"));
        assert!(expansion.contains("ProviderStoredOutput < '__geam_call , __GeamExternalOwner0"));
        assert!(expansion.contains(
            "-> HostResult < BoxValue < New , geam_core :: __macro_support :: ProviderStoredOutput < '__geam_call"
        ));
        assert!(expansion.contains("HostTypeParameter < 1usize >"));
        assert!(expansion.contains("HostTypeParameter < 0usize >"));
        assert!(expansion.contains("context . stored_values_equal"));
        assert!(expansion.contains("context . stored_value_hash"));
        assert!(expansion.contains("concat ! (\"Box\" , \"(<opaque>)\")"));
        assert_eq!(
            expansion
                .matches("call . provider_external_item_with")
                .count(),
            3,
        );
        assert!(!expansion.contains("call . external_payload_with"));
        assert!(expansion.contains(
            "ProviderExternalDeclaration for BoxValue < Item , __GeamStoredContext0 , >"
        ));
        assert!(expansion.contains(
            "ProviderDynamicInput < Profile , Provider , Return > for BoxValue < Item , >"
        ));
        assert!(expansion.contains("type View < '__geam_call > = BoxInput < Item ,"));
        assert_eq!(
            expansion
                .matches("call . create_external_with_binding :: < __GeamProvider >")
                .count(),
            3,
        );
        assert!(!expansion.contains("HostExternalPayloadBuilder"));
        assert!(!expansion.contains("payload . clone"));
        assert_eq!(
            expansion
                .matches("with_external_type :: < __GeamProvider , __GeamExternalSchema0 >")
                .count(),
            1,
        );
    }

    #[test]
    fn source_identity_and_generated_input_names_have_distinct_owners() {
        expand(
            quote!(path = "generic", crate_path = geam_core),
            quote! {
                mod generic {
                    #[geam::external(
                        name = "BoxInput",
                        parameters = [Item],
                        input = BoxInput,
                    )]
                    struct BoxValue<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }
                }
            },
        )
        .expect("source identities may match generated Rust input names");

        let cases = [
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = BoxValue,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "generated external input type `BoxValue` conflicts with provider value type `BoxValue`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = Status,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }

                        #[geam::custom]
                        enum Status { Ready }
                    }
                },
                "generated external input type `Status` conflicts with provider value type `Status`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "Box",
                            parameters = [Item],
                            input = SharedInput,
                        )]
                        struct BoxValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }

                        #[geam::custom(input = SharedInput)]
                        enum Status { Ready }
                    }
                },
                "duplicate generated input type `SharedInput`",
            ),
            (
                quote! {
                    mod generic {
                        #[geam::external(
                            name = "First",
                            parameters = [Item],
                            input = SharedInput,
                        )]
                        struct FirstValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }

                        #[geam::external(
                            name = "Second",
                            parameters = [Item],
                            input = SharedInput,
                        )]
                        struct SecondValue<Item> {
                            #[geam::stored]
                            value: Stored<Item>,
                        }
                    }
                },
                "duplicate generated input type `SharedInput`",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn retained_payload_expansion_uses_one_explicit_owner_across_declarations() {
        let expansion = expand(
            quote!(path = "priority_queue", crate_path = geam_core),
            quote! {
                mod priority_queue {
                    struct QueuePayload;

                    #[geam::external(
                        name = "PriorityQueue",
                        parameters = [Item],
                        input = PriorityQueueInput,
                        payload = QueuePayload,
                        manual,
                    )]
                    pub struct PriorityQueue<Item>;

                    #[geam::external(
                        name = "TransientPriorityQueue",
                        parameters = [Item],
                        input = TransientPriorityQueueInput,
                        payload = QueuePayload,
                        manual,
                    )]
                    pub struct TransientPriorityQueue<Item>;

                    #[geam::function]
                    fn empty<Item>() -> PriorityQueue<Item> {
                        todo!()
                    }

                    #[geam::function]
                    fn replace<Item>(
                        #[geam::call] call: &mut Call<()>,
                        queue: PriorityQueueInput<Item>,
                        value: Value<Item>,
                    ) -> PriorityQueue<Item> {
                        todo!()
                    }
                }
            },
        )
        .expect("advanced retained declaration should expand")
        .to_string();

        assert!(expansion.contains("const PARAMETER_COUNT : usize = 1usize"));
        assert_eq!(
            expansion
                .matches("HostExternalStore < QueuePayload >")
                .count(),
            2,
        );
        assert_eq!(
            expansion
                .matches("ProviderStoredOwner for QueuePayload")
                .count(),
            1,
        );
        assert!(expansion.contains("ProviderExternalOutput < QueuePayload >"));
        assert!(expansion.contains(
            "Retained < QueuePayload , geam_core :: __macro_support :: HostTypeIndex0 >"
        ),);
        assert!(expansion.contains("fn stored_item"));
        assert!(expansion.contains("fn payload (& self) -> & QueuePayload"));
        assert!(expansion.contains("RetainedExternalPayload > :: source_equal"));
        assert!(expansion.contains("RetainedExternalPayload > :: source_hash"));
        assert!(expansion.contains("RetainedExternalPayload > :: inspect"));
        assert!(!expansion.contains("pub struct __GeamExternalPayload0"));
        assert!(!expansion.contains("pub struct __GeamExternalOwner0"));
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
            "external payload structs must not have generics; declare `parameters = [...]` for retained generic externals",
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
    fn output_only_customs_generate_no_input_surface() {
        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    #[geam::custom]
                    enum Status {
                        Ready,
                    }

                    #[geam::function]
                    fn ready() -> Status {
                        Status::Ready
                    }
                }
            },
        )
        .expect("output-only custom values should expand")
        .to_string();

        assert!(expansion.contains("ProviderRootOutputValue"));
        assert!(!expansion.contains("pub enum StatusInput"));
    }

    #[test]
    fn directional_customs_generate_one_static_recursive_codec() {
        let expansion = expand(
            quote!(path = "customs", crate_path = geam_core),
            quote! {
                mod customs {
                    #[derive(Clone)]
                    enum Helper {
                        Value,
                    }

                    #[geam::external(name = "Tag")]
                    struct Tag;

                    #[geam::custom(input = StatusInput)]
                    enum Status {
                        Idle,
                        Code(BigInt),
                        Pair((BigInt, bool)),
                        Detail {
                            label: EcoString,
                            tag: Tag,
                            sibling: sibling::Value,
                        },
                    }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope {
                        Wrapped(Status),
                        Batch(Vec<(Status, Tag, sibling::Value)>),
                    }

                    #[geam::custom(input = FlagInput)]
                    enum Flag {
                        Enabled(bool),
                    }

                    #[geam::function]
                    fn inspect(
                        value: EnvelopeInput,
                        values: geam::List<StatusInput>,
                        sibling: &sibling::External,
                    ) -> EcoString {
                        todo!()
                    }

                    #[geam::function]
                    fn wrap(value: BigInt) -> Envelope {
                        todo!()
                    }

                    #[geam::function]
                    fn declared(value: sibling::Value) -> sibling::Value {
                        value
                    }

                    #[geam::function]
                    fn declared_values(
                        values: geam::List<sibling::Value>,
                    ) -> Vec<sibling::Value> {
                        Vec::new()
                    }

                    #[geam::function]
                    fn flags(values: geam::List<FlagInput>) -> EcoString {
                        todo!()
                    }

                    #[geam::function]
                    fn same_statuses(
                        values: geam::List<StatusInput>,
                    ) -> geam::List<StatusInput> {
                        values
                    }

                    #[geam::function]
                    fn envelopes() -> Vec<Envelope> {
                        Vec::new()
                    }
                }
            },
        )
        .expect("directional custom declarations should expand")
        .to_string();

        assert!(expansion.contains("enum StatusInput"));
        assert!(expansion.contains("enum EnvelopeInput"));
        assert!(expansion.contains("ProviderInputValue"));
        assert!(expansion.contains("ProviderRootOutputValue"));
        assert!(expansion.contains("ProviderListInputCodec"));
        assert!(expansion.contains("ProviderExternalCodec"));
        assert!(expansion.contains("sibling :: External"));
        assert!(expansion.contains("sibling :: Value"));
        assert_eq!(expansion.matches("HostCustomSchema for").count(), 3);
        assert_eq!(
            expansion
                .matches("HostCustomConstructorDefinition for")
                .count(),
            7,
        );
    }

    #[test]
    fn custom_declarations_are_non_generic_nonempty_enums_with_unique_names() {
        let cases = [
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status<T> { Value(T) }
                    }
                },
                "custom value enums must not have generics",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status where BigInt: Clone { Ready }
                    }
                },
                "custom value enums must not have generics",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status {}
                    }
                },
                "custom value enums must declare at least one constructor",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Ready = 1 }
                    }
                },
                "custom value constructors must not have Rust discriminants",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }
                    }
                },
                "duplicate `#[geam::custom]` attribute",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom = StatusInput]
                        enum Status { Ready }
                    }
                },
                "`#[geam::custom]` accepts only `input = Type` arguments",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(other = StatusInput)]
                        enum Status { Ready }
                    }
                },
                "unknown custom argument `other`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::external(name = "Status")]
                        struct Payload;

                        #[geam::custom]
                        enum Status { Ready }
                    }
                },
                "duplicate source type `Status`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Ready }

                        #[geam::custom]
                        enum Status { Waiting }
                    }
                },
                "duplicate source type `Status`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = Status)]
                        enum Status { Ready }
                    }
                },
                "generated custom input type `Status` conflicts with provider value type `Status`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = SharedInput)]
                        enum First { Value }

                        #[geam::custom(input = SharedInput)]
                        enum Second { Value }
                    }
                },
                "duplicate generated input type `SharedInput`",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn custom_fields_preserve_directional_list_and_nested_custom_contracts() {
        let cases = [
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Borrowed(&BigInt) }
                    }
                },
                "custom output fields must be owned values",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values(geam::List<BigInt>) }
                    }
                },
                "custom output List fields use Vec<T>; generated input values use geam::List<T>",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values(Vec<&BigInt>) }
                    }
                },
                "custom List item outputs must be owned values",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values(Vec<Vec<BigInt>>) }
                    }
                },
                "nested List values are not supported in custom declarations",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values(Vec<(bool, Vec<BigInt>)>) }
                    }
                },
                "List values are not supported inside custom tuple fields",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values((bool, Vec<BigInt>)) }
                    }
                },
                "List values are not supported inside custom tuple fields",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Values { values: Vec } }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = InnerInput)]
                        enum Inner { Value }

                        #[geam::custom]
                        enum Outer { Value(InnerInput) }
                    }
                },
                "custom input `InnerInput` cannot be stored in an output; use `Inner`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Inner { Value }

                        #[geam::custom(input = OuterInput)]
                        enum Outer { Value(Inner) }
                    }
                },
                "custom value `Inner` is nested in an input declaration but has no `input = ...`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Inner { Value }

                        #[geam::custom(input = OuterInput)]
                        enum Outer { Values(Vec<(bool, Inner)>) }
                    }
                },
                "custom value `Inner` is nested in an input declaration but has no `input = ...`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Inner { Value }

                        #[geam::custom(input = OuterInput)]
                        enum Outer { Values(Vec<Inner>) }
                    }
                },
                "custom value `Inner` is nested in an input declaration but has no `input = ...`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Inner { Value }

                        #[geam::custom(input = OuterInput)]
                        enum Outer { Value { inner: Inner } }
                    }
                },
                "custom value `Inner` is nested in an input declaration but has no `input = ...`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Inner { Value }

                        #[geam::custom(input = OuterInput)]
                        enum Outer { Value((bool, Inner)) }
                    }
                },
                "custom value `Inner` is nested in an input declaration but has no `input = ...`",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn custom_function_signatures_require_directional_input_and_output_types() {
        let cases = [
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(value: Status) -> bool { true }
                    }
                },
                "custom output `Status` cannot be used as a source argument; use an explicit generated input type",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(value: Status) -> bool { true }
                    }
                },
                "custom output `Status` cannot be used as a source argument; use `StatusInput`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(value: (bool, Status)) -> bool { true }
                    }
                },
                "custom output `Status` cannot be used as a source argument; use an explicit generated input type",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(value: (bool, Status)) -> bool { true }
                    }
                },
                "custom output `Status` cannot be used as a source argument; use `StatusInput`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }

                        #[geam::function]
                        fn create() -> StatusInput { todo!() }
                    }
                },
                "custom input `StatusInput` cannot be returned; return `Status`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(values: geam::List<Status>) -> bool { true }
                    }
                },
                "List item custom output `Status` cannot be used as input; use an `input = ...` declaration",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }

                        #[geam::function]
                        fn read(values: geam::List<Status>) -> bool { true }
                    }
                },
                "List item custom output `Status` cannot be used as input; use `StatusInput`",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::custom(input = StatusInput)]
                        enum Status { Ready }

                        #[geam::function]
                        fn create() -> Vec<StatusInput> { Vec::new() }
                    }
                },
                "custom input `StatusInput` cannot be returned; return `Status`",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn recursive_custom_declarations_are_rejected_with_the_exact_cycle() {
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::custom]
                    enum Node { Next(Node) }
                }
            }),
            "recursive custom values are not supported: Node -> Node",
        );
        assert_eq!(
            expansion_error(quote! {
                mod counter {
                    #[geam::custom]
                    enum First { Next((bool, Second)) }

                    #[geam::custom]
                    enum Second { Next(Vec<First>) }
                }
            }),
            "recursive custom values are not supported: First -> Second -> First",
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
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn input(
                            value: &geam_core::provider::advanced::External<Token>,
                        ) -> bool { true }
                    }
                },
                "provider::advanced::External<T> arguments are already retained views and must be passed by value",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn input(
                            values: geam::List<geam_core::provider::advanced::External<Token>>,
                        ) -> bool { true }
                    }
                },
                "provider::advanced::External<T> is a call-scoped pass-through and cannot be a List item; use the declared payload input type",
            ),
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
    fn advanced_external_generic_arity_diagnostics_are_exact() {
        let cases = [
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        value: &geam_core::provider::advanced::External<BigInt, bool>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        value: geam_core::provider::advanced::External<BigInt, bool>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        value: Result<
                            geam_core::provider::advanced::External<BigInt, bool>,
                            bool,
                        >,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        values: geam::List<
                            geam_core::provider::advanced::External<BigInt, bool>,
                        >,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect()
                        -> geam_core::provider::advanced::External<BigInt, bool>
                    {
                        unreachable!()
                    }
                }
            },
        ];

        for item in cases {
            assert_eq!(
                expansion_error(item),
                "External requires exactly one type argument",
            );
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
                "external payload `Token` returns must be owned",
            ),
            (
                quote! {
                    mod lists {
                        #[geam::external(name = "Token")]
                        struct Token;

                        #[geam::function]
                        fn output() -> Vec<(EcoString, &Token)> { Vec::new() }
                    }
                },
                "external payload `Token` returns must be owned",
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
                        fn nested(values: (Vec<BigInt>, bool)) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }

        let expansion = expand(
            quote!(path = "lists", crate_path = geam_core),
            quote! {
                mod lists {
                    #[geam::function]
                    fn nested(values: (geam::List<BigInt>, bool)) -> bool { true }
                }
            },
        )
        .expect("lazy Lists should compose inside ordinary input values")
        .to_string();
        assert!(expansion.contains(
            "HostTupleType < geam_core :: __macro_support :: HostTypeList < geam_core :: __macro_support :: HostListType < BigInt >"
        ));
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
            2,
        );
        assert!(expansion.contains("ProviderExternalItem < Token >"));
        assert!(expansion.contains("into_external (& self . __geam_external_0)"));
        assert_eq!(expansion.matches("call . construct_external").count(), 2);
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
        assert!(expansion.contains("ProviderConstructionList <"));
        assert!(expansion.contains("ProviderConstructionRequirements > :: Types <"));
        assert!(
            expansion.contains(
                "ProviderConstruction < geam_core :: __macro_support :: HostExternalType"
            )
        );
        assert!(
            expansion
                .contains("ProviderConstruction < geam_core :: __macro_support :: HostTupleType")
        );
        assert!(!expansion.contains("HostTypeIndex0"));
        assert!(!expansion.contains("HostTypeIndexNext <"));
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
                        #[geam::call] call: &Call<RunState>,
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
        assert!(
            expansion.contains("fn inspect (& self) -> geam_core :: __macro_support :: EcoString")
        );
        assert!(!expansion.contains(":: ecow :: EcoString"));
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
        assert!(expansion.contains("Call :: from_shared_state (__geam_state)"));
        assert!(
            expansion.contains(
                "copy (& __geam_provider_call , & * __geam_payload_0 , __geam_argument_1)"
            )
        );
        assert!(expansion.contains("let returned = call . create_external (returned)"));
        assert!(expansion.contains(
            "< super :: Component as geam_core :: __macro_support :: ProviderPackage > :: PACKAGE"
        ));
    }

    #[test]
    fn retained_external_expansion_uses_only_the_context_aware_semantics_contract() {
        let expansion = expand(
            quote!(path = "dynamic", crate_path = geam_core),
            quote! {
                mod dynamic {
                    #[geam::external(name = "Snapshot", manual)]
                    struct Snapshot;

                    #[geam::external(name = "Dynamic", retained)]
                    struct Dynamic;
                }
            },
        )
        .expect("manual and retained external semantics should remain distinct")
        .to_string();

        assert!(expansion.contains(
            "< Snapshot as geam_core :: __macro_support :: ExternalPayload > :: source_equal"
        ));
        assert!(expansion.contains(
            "< Dynamic as geam_core :: __macro_support :: RetainedExternalPayload > :: source_equal"
        ));
        assert!(!expansion.contains(
            "< Dynamic as geam_core :: __macro_support :: ExternalPayload > :: source_equal"
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

    #[test]
    fn qualified_declarations_use_static_directional_codecs() {
        let expansion = expand(
            quote!(path = "consumer", crate_path = geam_core),
            quote! {
                mod consumer {
                    #[geam::function]
                    fn token_text(value: &declarations::Token) -> EcoString {
                        value.0.clone()
                    }

                    #[geam::function]
                    fn tuple_token_text(
                        value: (&declarations::Token, bool),
                    ) -> EcoString {
                        value.0.0.clone()
                    }

                    #[geam::function]
                    fn pair() -> (declarations::Status, bool) {
                        todo!()
                    }

                    #[geam::function]
                    fn statuses() -> Vec<declarations::Status> {
                        Vec::new()
                    }
                }
            },
        )
        .expect("qualified declarations should expand through their static codecs")
        .to_string();

        assert!(expansion.contains(
            "< declarations :: Token as geam_core :: __macro_support :: ProviderValue > :: Input"
        ));
        assert!(expansion.contains(
            "declarations :: Status : geam_core :: __macro_support :: ProviderOutputValue"
        ));
        assert!(expansion.contains(
            "< declarations :: Status as geam_core :: __macro_support :: ProviderValue > :: OutputRequirements"
        ));
        assert!(expansion.contains(
            "HostListType < < declarations :: Status as geam_core :: __macro_support :: ProviderValue > :: Host >"
        ));
    }

    #[test]
    fn source_result_and_option_use_core_owned_custom_schemas() {
        let expansion = expand(
            quote!(path = "values", crate_path = geam_core),
            quote! {
                mod values {
                    #[geam::external(name = "LocalToken")]
                    struct LocalToken;

                    #[geam::custom(input = ProblemInput)]
                    enum Problem {
                        Invalid(String),
                    }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope {
                        ResultField(Result<Option<BigInt>, Problem>),
                        OptionField(Option<Result<BigInt, Problem>>),
                        ResultItems(Vec<Result<Option<BigInt>, Problem>>),
                        OptionItems(Vec<Option<Result<BigInt, Problem>>>),
                    }

                    #[geam::function]
                    fn inspect(
                        value: Result<Option<(BigInt, ProblemInput)>, ProblemInput>,
                    ) -> Result<Option<(BigInt, Problem)>, Problem> {
                        todo!()
                    }

                    #[geam::function]
                    fn inspect_declared(
                        value: Option<Result<&declarations::Token, ProblemInput>>,
                    ) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn inspect_declared_custom(
                        value: Option<declarations::ProblemInput>,
                    ) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn inspect_local_external(value: Option<&LocalToken>) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn inspect_results(
                        values: geam_core::List<Result<Option<BigInt>, ProblemInput>>,
                    ) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn inspect_options(
                        values: geam_core::List<Option<Result<BigInt, ProblemInput>>>,
                    ) -> bool {
                        todo!()
                    }

                    #[geam::function]
                    fn results() -> Vec<Result<Option<BigInt>, Problem>> {
                        todo!()
                    }

                    #[geam::function]
                    fn options() -> Vec<Option<Result<BigInt, Problem>>> {
                        todo!()
                    }
                }
            },
        )
        .expect("Result and Option should expand through core schemas")
        .to_string();

        assert!(expansion.contains("ProviderResult <"));
        assert!(expansion.contains("ProviderOption <"));
        assert!(expansion.contains("ProviderOk <"));
        assert!(expansion.contains("ProviderError <"));
        assert!(expansion.contains("ProviderSome <"));
        assert!(expansion.contains("ProviderNone <"));
        assert!(expansion.contains("provider_remaining_custom_fields"));
        assert!(!expansion.contains("unreachable"));
        assert!(
            expansion.contains(
                "value : :: core :: result :: Result < :: core :: option :: Option < (BigInt , ProblemInput ,) > , ProblemInput >"
            )
        );
        assert!(expansion.contains(
            "value : :: core :: option :: Option < :: core :: result :: Result < < declarations :: Token as geam_core :: __macro_support :: ProviderValue > :: Input , ProblemInput > >"
        ));
        assert!(
            expansion
                .contains("value : :: core :: option :: Option < declarations :: ProblemInput >")
        );
        assert!(expansion.contains(
            "value : :: core :: option :: Option < geam_core :: __macro_support :: ProviderExternalItem < LocalToken > >"
        ));
        assert!(expansion.contains(
            ":: core :: result :: Result < :: core :: option :: Option < BigInt > , ProblemInput >"
        ));
        assert!(expansion.contains(
            ":: core :: option :: Option < :: core :: result :: Result < BigInt , ProblemInput > >"
        ));
        assert!(expansion.contains("ResultField"));
        assert!(expansion.contains("OptionField"));
        assert!(expansion.contains("ResultItems"));
        assert!(expansion.contains("OptionItems"));
    }

    #[test]
    fn collection_input_models_preserve_recursive_result_and_option_direction() {
        let input: syn::Type = syn::parse_quote!(Result<Option<BigInt>, Option<EcoString>>);
        let input = classify_collection_input_item(&input, &[], &[], "List")
            .expect("nested Result and Option List input should classify");
        assert_eq!(
            static_value_key(&input),
            "result:<option:<scalar:BigInt>,option:<scalar:EcoString>>",
        );
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

    #[test]
    fn source_result_and_option_preserve_nested_collection_direction() {
        let argument_cases = [
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Option<(bool, Vec<BigInt>)>,
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: (Result<Vec<EcoString>, BigInt>, bool),
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: (Result<BigInt, Vec<EcoString>>, bool),
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: (Option<Vec<BigInt>>, bool),
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: (bool, Vec<BigInt>)) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Result<Vec<EcoString>, BigInt>,
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Result<BigInt, Vec<EcoString>>,
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Option<Vec<BigInt>>,
                        ) -> bool { true }
                    }
                },
                "Vec<T> arguments are not supported; use geam::List<T>",
            ),
        ];

        for (item, expected) in argument_cases {
            assert_eq!(expansion_error(item), expected);
        }

        let expansion = expand(
            quote!(path = "values", crate_path = geam_core),
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        value: Result<geam_core::List<BigInt>, EcoString>,
                    ) -> bool { true }
                }
            },
        )
        .expect("lazy Lists should compose inside source Result input")
        .to_string();
        assert!(expansion.contains(
            "ProviderResult < geam_core :: __macro_support :: HostListType < BigInt > , EcoString >"
        ));

        let return_cases = [
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Result<geam_core::List<BigInt>, EcoString> {
                            unreachable!()
                        }
                    }
                },
                "geam::List<T> is supported only as a top-level source return",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Option<geam_core::List<BigInt>> {
                            unreachable!()
                        }
                    }
                },
                "geam::List<T> is supported only as a top-level source return",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Result<BigInt> { unreachable!() }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Option<BigInt, EcoString> { unreachable!() }
                    }
                },
                "Option requires exactly 1 type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Result { unreachable!() }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Option { unreachable!() }
                    }
                },
                "Option requires exactly 1 type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Result<Vec<BigInt, EcoString>, ()> {
                            unreachable!()
                        }
                    }
                },
                "Vec requires exactly one type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::external(name = "Token")]
                        #[derive(PartialEq, Eq, Hash)]
                        struct Token;

                        #[geam::function]
                        fn inspect() -> Result<Vec<&Token>, ()> {
                            unreachable!()
                        }
                    }
                },
                "external payload `Token` returns must be owned",
            ),
            (
                quote! {
                    mod values {
                        #[geam::external(name = "Token")]
                        #[derive(PartialEq, Eq, Hash)]
                        struct Token;

                        #[geam::function]
                        fn inspect() -> Result<BigInt, &Token> {
                            unreachable!()
                        }
                    }
                },
                "external payload `Token` returns must be owned",
            ),
            (
                quote! {
                    mod values {
                        #[geam::external(name = "Token")]
                        #[derive(PartialEq, Eq, Hash)]
                        struct Token;

                        #[geam::function]
                        fn inspect() -> Option<Result<BigInt, &Token>> {
                            unreachable!()
                        }
                    }
                },
                "external payload `Token` returns must be owned",
            ),
        ];

        for (item, expected) in return_cases {
            assert_eq!(expansion_error(item), expected);
        }

        let collection_cases = [
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        values: geam_core::List<Result<geam_core::List<BigInt>, EcoString>>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        values: geam_core::List<Result<BigInt, Vec<EcoString>>>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        values: geam_core::List<Option<geam_core::List<BigInt>>>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect() -> Vec<Result<geam_core::List<BigInt>, EcoString>> {
                        Vec::new()
                    }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect() -> Vec<Option<geam_core::List<BigInt>>> {
                        Vec::new()
                    }
                }
            },
        ];

        for (index, item) in collection_cases.into_iter().enumerate() {
            assert_eq!(
                expansion_error(item),
                if index < 3 {
                    "nested List and Vec item values are not supported"
                } else {
                    "geam::List<T> is supported only as a top-level source return"
                },
            );
        }

        let expansion = expand(
            quote!(path = "values", crate_path = geam_core),
            quote! {
                mod values {
                    #[geam::function]
                    fn parse_query() -> Result<Vec<(EcoString, EcoString)>, ()> {
                        Ok(Vec::new())
                    }
                }
            },
        )
        .expect("Vec output nested in source Result should expand")
        .to_string();
        assert!(
            expansion.contains("ProviderResult < geam_core :: __macro_support :: HostListType")
        );
        assert!(expansion.contains("call . construct_list"));
        assert!(expansion.contains("call . return_custom"));

        let collection_arity_cases = [
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(values: geam_core::List<Result<BigInt>>) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect(
                        values: geam_core::List<Option<BigInt, EcoString>>,
                    ) -> bool { true }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect() -> Vec<Result<BigInt>> { Vec::new() }
                }
            },
            quote! {
                mod values {
                    #[geam::function]
                    fn inspect() -> Vec<Option<BigInt, EcoString>> { Vec::new() }
                }
            },
        ];

        let expected = [
            "Result requires exactly 2 type arguments",
            "Option requires exactly 1 type argument",
            "Result requires exactly 2 type arguments",
            "Option requires exactly 1 type argument",
        ];
        for (item, expected) in collection_arity_cases.into_iter().zip(expected) {
            assert_eq!(expansion_error(item), expected);
        }

        let custom_cases = [
            quote! {
                mod values {
                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope {
                        Invalid(Result<Vec<BigInt>, EcoString>),
                    }
                }
            },
            quote! {
                mod values {
                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope {
                        Invalid(Result<BigInt, Vec<EcoString>>),
                    }
                }
            },
            quote! {
                mod values {
                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope {
                        Invalid(Option<Vec<BigInt>>),
                    }
                }
            },
        ];

        for item in custom_cases {
            assert_eq!(
                expansion_error(item),
                "List values are not supported inside custom tuple fields",
            );
        }

        let custom_arity_cases = [
            (
                quote! {
                    mod values {
                        #[geam::custom(input = EnvelopeInput)]
                        enum Envelope {
                            Invalid(Result<BigInt>),
                        }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod values {
                        #[geam::custom(input = EnvelopeInput)]
                        enum Envelope {
                            Invalid(Option<BigInt, EcoString>),
                        }
                    }
                },
                "Option requires exactly 1 type argument",
            ),
        ];

        for (item, expected) in custom_arity_cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn custom_inputs_reject_output_only_customs_nested_in_source_wrappers() {
        let value_cases = [
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Result<OutputOnly, BigInt>) }
                }
            },
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Result<BigInt, OutputOnly>) }
                }
            },
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Option<OutputOnly>) }
                }
            },
        ];

        for item in value_cases {
            assert_eq!(
                expansion_error(item),
                "custom value `OutputOnly` is nested in an input declaration but has no `input = ...`",
            );
        }

        let list_cases = [
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Vec<Result<OutputOnly, BigInt>>) }
                }
            },
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Vec<Result<BigInt, OutputOnly>>) }
                }
            },
            quote! {
                mod values {
                    #[geam::custom]
                    enum OutputOnly { Value }

                    #[geam::custom(input = EnvelopeInput)]
                    enum Envelope { Value(Vec<Option<OutputOnly>>) }
                }
            },
        ];

        for item in list_cases {
            assert_eq!(
                expansion_error(item),
                "custom value `OutputOnly` is nested in an input declaration but has no `input = ...`",
            );
        }
    }

    #[test]
    fn source_result_and_option_generic_arity_diagnostics_are_exact() {
        let cases = [
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: Result<BigInt>) -> bool { true }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: Option) -> bool { true }
                    }
                },
                "Option requires exactly 1 type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: Result<'static, BigInt>) -> bool { true }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: Option<'static>) -> bool { true }
                    }
                },
                "Option requires exactly 1 type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Result<geam::List<BigInt, bool>, bool>,
                        ) -> bool { true }
                    }
                },
                "List requires exactly one type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Result<geam::List<Vec<BigInt>>, bool>,
                        ) -> bool { true }
                    }
                },
                "nested List and Vec item values are not supported",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: (Result<BigInt>, bool)) -> bool { true }
                    }
                },
                "Result requires exactly 2 type arguments",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn host_result_is_only_the_outer_execution_envelope() {
        let cases = [
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(value: HostResult<BigInt>) -> bool { true }
                    }
                },
                "HostResult<T> is supported only as the outer provider return",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> (HostResult<BigInt>, bool) { todo!() }
                    }
                },
                "HostResult<T> is supported only as the outer provider return",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> Result<BigInt, HostFailure> { todo!() }
                    }
                },
                "Result<T, HostFailure> is not a source Result; use HostResult<T>",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> HostResult { todo!() }
                    }
                },
                "HostResult requires exactly one type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> HostResult<BigInt, Problem> { todo!() }
                    }
                },
                "HostResult requires exactly one type argument",
            ),
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect() -> (HostResult, bool) { todo!() }
                    }
                },
                "HostResult requires exactly one type argument",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }

        let expansion = expand(
            quote!(path = "values", crate_path = geam_core),
            quote! {
                mod values {
                    #[geam::custom]
                    enum Problem { Invalid }

                    #[geam::function]
                    fn inspect() -> HostResult<Result<BigInt, Problem>> { todo!() }

                    #[geam::function]
                    fn keep(
                        values: geam_core::List<BigInt>,
                    ) -> HostResult<geam_core::List<BigInt>> { Ok(values) }
                }
            },
        )
        .expect("HostResult should wrap an ordinary source Result")
        .to_string();

        assert!(expansion.contains("let returned = returned ?"));
        assert!(expansion.contains("ProviderResult <"));
        assert!(
            expansion.contains(
                "HostResult < geam_core :: __macro_support :: List < BigInt , geam_core :: __macro_support :: ProviderListContext < '__geam_list"
            )
        );
    }
}
