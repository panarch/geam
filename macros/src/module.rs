mod custom_value;
mod declaration;
mod function;
mod list;
mod list_model;
mod signature;
mod syntax;
mod type_syntax;

use crate::path::support_path;
use custom_value::{CustomModel, collect_custom_declarations};
use declaration::{generate_custom_declaration, generic_external_output_codec};
use function::generate_function;
use list::generate_list_decoder;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use signature::host_type_token_sequence;
use std::collections::{BTreeMap, BTreeSet};
use syn::ext::IdentExt;
use syn::{
    Attribute, GenericParam, Ident, Item, ItemMod, LitStr, Path, PathArguments, Type, TypeBareFn,
    TypePath,
};
use syntax::{
    build_external_model, host_type_index, retained_parameter_accessor, take_external_marker,
    take_function_marker, validate_function,
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

fn is_marker(attribute: &Attribute, name: &str) -> bool {
    attribute
        .path()
        .segments
        .last()
        .is_some_and(|segment| segment.ident == name)
}

#[cfg(test)]
mod tests {
    use super::expand;
    use quote::quote;

    fn expansion_error(item: proc_macro2::TokenStream) -> String {
        expand(quote!(path = "counter", crate_path = geam_core), item)
            .expect_err("module should be rejected")
            .to_string()
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
