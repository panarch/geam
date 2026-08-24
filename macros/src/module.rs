mod custom_value;

use crate::path::support_path;
use custom_value::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use std::collections::{BTreeMap, BTreeSet};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, FnArg, GenericArgument, GenericParam, Ident, Item, ItemFn, ItemMod, ItemStruct,
    LitStr, Meta, Path, PathArguments, ReturnType, Token, Type, TypePath,
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
struct GenericValueType {
    source: Type,
    instantiated: Type,
    path: TypePath,
    host: GenericHostType,
}

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
    index: usize,
}

#[derive(Clone)]
enum ProviderValueType {
    Scalar(Type),
    Declared {
        type_: Type,
        input: DeclaredInput,
    },
    External {
        payload: Ident,
        schema: Ident,
        store_field: Ident,
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
enum FunctionOutputValueType {
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
    Declared {
        type_: Type,
        input: DeclaredInput,
    },
    External {
        payload: Ident,
        schema: Ident,
        store_field: Ident,
    },
    Custom {
        index: usize,
        rust: Type,
    },
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
    value: ProviderValueType,
}

#[derive(Clone)]
struct ListType {
    collection: CollectionType,
    decoder: Ident,
}

enum FunctionArgumentType {
    Value(Box<ProviderValueType>),
    Generic(Box<GenericValueType>),
    List(Box<ListType>),
}

enum FunctionReturnType {
    Value(FunctionOutputValueType),
    Generic(Box<GenericValueType>),
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
    value: ProviderValueType,
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
    wrapper: TokenStream,
    registration: TokenStream,
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
            generics.push(FunctionGeneric { index });
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
    let custom_declarations =
        collect_custom_declarations(items, &mut source_names, &externals, &mut list_decoders)?;
    let customs = custom_declarations.models;
    let custom_list_decoders = custom_declarations.list_decoders;

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
                Profile: __GeamModuleProfile,
            {
                type Storage = #storage;
            }
        }
    });
    let mut custom_declarations = Vec::with_capacity(customs.len());
    for (index, custom) in customs.iter().enumerate() {
        custom_declarations.push(generate_custom_declaration(
            index,
            custom,
            &customs,
            custom_list_decoders[index].as_ref(),
            &support,
            &module_path,
        )?);
    }
    let generated_list_decoders = list_decoders
        .iter()
        .map(|decoder| generate_list_decoder(decoder, &customs, &support))
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

fn generate_custom_declaration(
    custom_index: usize,
    custom: &CustomModel,
    customs: &[CustomModel],
    list_decoder: Option<&Ident>,
    support: &TokenStream,
    module_path: &LitStr,
) -> syn::Result<TokenStream> {
    let custom_ident = &custom.ident;
    let schema = &custom.schema;
    let source_name = custom.ident.unraw().to_string();
    let mut field_definitions = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            let definition = &field.definition;
            let label = if let Some(ident) = &field.ident {
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
    let input_declaration = if let Some(input) = &custom.input {
        let visibility = &custom.visibility;
        let mut variants = Vec::with_capacity(custom.constructors.len());
        for constructor in &custom.constructors {
            let ident = &constructor.ident;
            let variant = match &constructor.fields {
                CustomFields::Unit => quote!(#ident),
                CustomFields::Unnamed(fields) => {
                    let mut types = Vec::with_capacity(fields.len());
                    for field in fields {
                        types.push(custom_input_type(&field.value, customs, support)?);
                    }
                    quote!(#ident(#(#types),*))
                }
                CustomFields::Named(fields) => {
                    let mut members = Vec::with_capacity(fields.len());
                    for field in fields {
                        let field_ident = field
                            .ident
                            .as_ref()
                            .expect("named custom fields must retain identifiers");
                        let type_ = custom_input_type(&field.value, customs, support)?;
                        members.push(quote!(#field_ident: #type_));
                    }
                    quote!(#ident { #(#members),* })
                }
            };
            variants.push(variant);
        }
        let input_codec_bounds = custom_input_codec_bounds(custom, customs, support);
        let decoder_input_codec_bounds = input_codec_bounds.clone();
        let list_codec_bounds = custom_list_codec_bounds(custom_index, customs, support);
        let decoder_definition = generate_custom_decoder(
            custom_index,
            custom,
            customs,
            support,
            &decoder_input_codec_bounds,
        );
        let decoder_ident = &custom.decoder;
        let list_decoder = list_decoder.expect("custom input must own a List decoder");
        let list_decoder_value = list_decoder_value(
            list_decoder,
            &ProviderValueType::Custom {
                index: custom_index,
                rust: syn::parse_quote!(#input),
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
        quote!(#input)
    } else {
        quote!(#support::NoCustomInput)
    };

    Ok(quote! {
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
    })
}

fn generate_custom_decoder(
    _custom_index: usize,
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
    codec_bounds: &[TokenStream],
) -> TokenStream {
    let decoder = &custom.decoder;
    let schema = &custom.schema;
    let input = custom
        .input
        .as_ref()
        .expect("custom decoder requires an input declaration");
    let mut names = GeneratedNames::default();
    let mut branches = Vec::with_capacity(custom.constructors.len());
    for constructor in &custom.constructors {
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
        branches.push(quote! {
            if let ::core::option::Option::Some(#host_pattern) =
                call.provider_custom_fields::<#marker>(value)
            {
                #(#statements)*
                #(#declarations)*
                return #expression;
            }
        });
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
            ::core::unreachable!("typed custom constructor index is out of range")
        }
    }
}

fn decode_custom_field_value(
    type_: &CustomFieldValueType,
    input: TokenStream,
    customs: &[CustomModel],
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_custom_input_value(type_, input, customs, support, names)
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
    type_: &ProviderValueType,
    input: TokenStream,
    customs: &[CustomModel],
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        ProviderValueType::Declared { type_, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(
                <<#type_ as #support::ProviderValue>::Input as
                    #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                        call,
                        #input,
                    )
            ),
        },
        ProviderValueType::External { payload, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(
                <#support::ProviderExternalItem<#payload> as
                    #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                        call,
                        #input,
                    )
            ),
        },
        ProviderValueType::Custom { index, .. } => {
            let input_type = customs[*index]
                .input
                .as_ref()
                .expect("custom source inputs require a generated input type");
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
        ProviderValueType::Tuple(elements) => {
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
                let decoded =
                    decode_custom_input_value(element, quote!(#host), customs, support, names);
                statements.extend(decoded.statements);
                values.push(decoded.value);
            }
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
            let decoded_success =
                decode_custom_input_value(success, quote!(#success_value), customs, support, names);
            let decoded_failure =
                decode_custom_input_value(failure, quote!(#failure_value), customs, support, names);
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
                    } else if let ::core::option::Option::Some((#failure_value, ())) =
                        call.provider_custom_fields::<
                            #support::ProviderError<#success_host, #failure_host>
                        >(#input)
                    {
                        #failure_statements
                        ::core::result::Result::Err(#failure)
                    } else {
                        ::core::unreachable!("typed Result constructor index is out of range")
                    }
                }),
            }
        }
        ProviderValueType::Option { value } => {
            let host = host_value_type(value, customs, support);
            let some_host = names.next("option_some_host");
            let decoded =
                decode_custom_input_value(value, quote!(#some_host), customs, support, names);
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
                    } else if call
                        .provider_custom_fields::<#support::ProviderNone<#host>>(#input)
                        .is_some()
                    {
                        ::core::option::Option::None
                    } else {
                        ::core::unreachable!("typed Option constructor index is out of range")
                    }
                }),
            }
        }
    }
}

fn custom_input_type(
    type_: &CustomFieldValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<TokenStream> {
    match type_ {
        CustomFieldValueType::Value(type_) => custom_input_value_type(type_, customs, support),
        CustomFieldValueType::List(list) => {
            let item = custom_list_input_value_type(&list.collection.value, customs, support)?;
            let decoder = &list.decoder;
            Ok(quote! {
                #support::List<
                    #item,
                    #support::ProviderInputListContext<#decoder>,
                >
            })
        }
    }
}

fn custom_input_value_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<TokenStream> {
    match type_ {
        ProviderValueType::Scalar(type_) => Ok(quote!(#type_)),
        ProviderValueType::Declared { type_, .. } => {
            Ok(quote!(<#type_ as #support::ProviderValue>::Input))
        }
        ProviderValueType::External { payload, .. } => {
            Ok(quote!(#support::ProviderExternalItem<#payload>))
        }
        ProviderValueType::Custom { index, .. } => {
            let Some(input) = &customs[*index].input else {
                return Err(syn::Error::new(
                    customs[*index].ident.span(),
                    format!(
                        "custom value `{}` is nested in an input declaration but has no `input = ...`",
                        customs[*index].ident
                    ),
                ));
            };
            Ok(quote!(#input))
        }
        ProviderValueType::Tuple(elements) => {
            let mut types = Vec::with_capacity(elements.len());
            for element in elements {
                types.push(custom_input_value_type(element, customs, support)?);
            }
            Ok(quote!((#(#types,)*)))
        }
        ProviderValueType::Result { success, failure } => {
            let success = custom_input_value_type(success, customs, support)?;
            let failure = custom_input_value_type(failure, customs, support)?;
            Ok(quote!(::core::result::Result<#success, #failure>))
        }
        ProviderValueType::Option { value } => {
            let value = custom_input_value_type(value, customs, support)?;
            Ok(quote!(::core::option::Option<#value>))
        }
    }
}

fn custom_list_input_value_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> syn::Result<TokenStream> {
    match type_ {
        ProviderValueType::Scalar(type_) => Ok(quote!(#type_)),
        ProviderValueType::Declared { type_, .. } => {
            Ok(quote!(<#type_ as #support::ProviderValue>::ListInput))
        }
        ProviderValueType::External { payload, .. } => Ok(quote!(#payload)),
        ProviderValueType::Custom { index, .. } => {
            let Some(input) = &customs[*index].input else {
                return Err(syn::Error::new(
                    customs[*index].ident.span(),
                    format!(
                        "custom value `{}` is nested in an input declaration but has no `input = ...`",
                        customs[*index].ident
                    ),
                ));
            };
            Ok(quote!(#input))
        }
        ProviderValueType::Tuple(elements) => {
            let mut types = Vec::with_capacity(elements.len());
            for element in elements {
                types.push(custom_list_input_value_type(element, customs, support)?);
            }
            Ok(quote!((#(#types,)*)))
        }
        ProviderValueType::Result { success, failure } => {
            let success = custom_list_input_value_type(success, customs, support)?;
            let failure = custom_list_input_value_type(failure, customs, support)?;
            Ok(quote!(::core::result::Result<#success, #failure>))
        }
        ProviderValueType::Option { value } => {
            let value = custom_list_input_value_type(value, customs, support)?;
            Ok(quote!(::core::option::Option<#value>))
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
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    provider: &TokenStream,
    return_type: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        ProviderValueType::Declared { type_, .. } => {
            bounds.push(quote! {
                #type_: #support::ProviderOutputValue<
                    Profile,
                    #provider,
                    #return_type,
                >
            });
        }
        ProviderValueType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            bounds.push(quote! {
                #type_: #support::ProviderOutputValue<
                    Profile,
                    #provider,
                    #return_type,
                >
            });
        }
        ProviderValueType::Tuple(elements) => {
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
        ProviderValueType::Result { success, failure } => {
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
        ProviderValueType::Option { value } => collect_custom_output_codec_bounds(
            value,
            customs,
            support,
            provider,
            return_type,
            bounds,
        ),
        ProviderValueType::Scalar(_) | ProviderValueType::External { .. } => {}
    }
}

fn custom_input_codec_bounds(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            match &field.value {
                CustomFieldValueType::Value(value) => {
                    collect_custom_input_codec_bounds(value, customs, support, &mut bounds);
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
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
    bounds: &mut Vec<TokenStream>,
) {
    match type_ {
        ProviderValueType::Declared { type_, .. } => {
            bounds.push(quote! {
                <#type_ as #support::ProviderValue>::Input:
                    #support::ProviderInputValue<Profile, Provider, Return>
            });
        }
        ProviderValueType::Custom { index, .. } => {
            let input = customs[*index]
                .input
                .as_ref()
                .expect("nested custom source inputs require a generated input type");
            bounds.push(quote! {
                #input: #support::ProviderInputValue<Profile, Provider, Return>
            });
        }
        ProviderValueType::Tuple(elements) => {
            for element in elements {
                collect_custom_input_codec_bounds(element, customs, support, bounds);
            }
        }
        ProviderValueType::Result { success, failure } => {
            collect_custom_input_codec_bounds(success, customs, support, bounds);
            collect_custom_input_codec_bounds(failure, customs, support, bounds);
        }
        ProviderValueType::Option { value } => {
            collect_custom_input_codec_bounds(value, customs, support, bounds);
        }
        ProviderValueType::Scalar(_) | ProviderValueType::External { .. } => {}
    }
}

fn custom_list_codec_bounds(
    custom_index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    let custom_type = &customs[custom_index].ident;
    for access in list_declared_accesses(
        &ProviderValueType::Custom {
            index: custom_index,
            rust: syn::parse_quote!(#custom_type),
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
    let decoded_arguments = function_arguments
        .iter()
        .zip(&arguments)
        .map(|(type_, argument)| {
            decode_argument(
                type_,
                quote!(#argument),
                customs,
                support,
                &return_type,
                &mut names,
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
    let generated_return = generate_return(
        return_,
        customs,
        support,
        &quote!(__GeamProvider),
        &return_type,
        &mut names,
    );
    let return_statements = &generated_return.statements;
    let completion = &generated_return.completion;
    let requirements = provider_requirement_sequence(&generated_return.constructions, support);
    if !generated_return.constructions.is_empty() {
        codec_bounds.push(quote! {
            #requirements: #support::ProviderConstructionRequirements
        });
        codec_bounds.extend(provider_requirement_selection_bounds(
            &requirements,
            &generated_return.constructions,
            support,
        ));
    }
    let construction_parameter = (!generated_return.constructions.is_empty()).then(|| {
        quote! {
            __geam_constructions: #support::HostConstructions<
                '__geam_call,
                <#requirements as #support::ProviderConstructionRequirements>::Types<
                    #support::HostTypeListEnd,
                >,
            >,
        }
    });
    let construction_setup = (!generated_return.constructions.is_empty()).then(|| {
        let bindings = provider_construction_bindings(
            &generated_return.constructions,
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
            FunctionArgumentType::Value(type_) => {
                collect_function_input_bounds(type_, customs, support, return_type, &mut bounds);
            }
            FunctionArgumentType::Generic(_) => {}
            FunctionArgumentType::List(list) => {
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
        ProviderValueType::Custom { index, .. } => {
            let input = customs[*index]
                .input
                .as_ref()
                .expect("custom source inputs require a generated input type");
            bounds.push(quote! {
                #input: #support::ProviderInputValue<
                    Profile,
                    __GeamProvider,
                    #return_type,
                >
            });
        }
        ProviderValueType::Scalar(_) | ProviderValueType::External { .. } => {}
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
        FunctionReturnType::Value(FunctionOutputValueType::Value(value)) => match value.as_ref() {
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
        FunctionReturnType::Value(
            value @ (FunctionOutputValueType::Tuple(_)
            | FunctionOutputValueType::Result { .. }
            | FunctionOutputValueType::Option { .. }
            | FunctionOutputValueType::Vec(_)),
        ) => collect_function_output_intermediate_bounds(
            value,
            customs,
            support,
            return_type,
            bounds,
        ),
        FunctionReturnType::Generic(_) | FunctionReturnType::List(_) => {}
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
    customs: &[CustomModel],
    support: &TokenStream,
    return_type: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        FunctionArgumentType::Value(type_) => {
            decode_value_argument(type_, input, customs, support, return_type, names, false)
        }
        FunctionArgumentType::Generic(value) => {
            let source = instantiated_generic_source_type(value);
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
        FunctionArgumentType::List(list) => {
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
        ProviderValueType::Custom { index, .. } => {
            let input_type = customs[*index]
                .input
                .as_ref()
                .expect("custom source inputs require a generated input type");
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
                    } else if let ::core::option::Option::Some((#failure_value, ())) =
                        call.provider_custom_fields::<
                            #support::ProviderError<#success_host, #failure_host>
                        >(#input)
                    {
                        #failure_statements
                        ::core::result::Result::Err(#failure)
                    } else {
                        ::core::unreachable!("typed Result constructor index is out of range")
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
                    } else if call
                        .provider_custom_fields::<#support::ProviderNone<#host>>(#input)
                        .is_some()
                    {
                        ::core::option::Option::None
                    } else {
                        ::core::unreachable!("typed Option constructor index is out of range")
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
    type_: &FunctionOutputValueType,
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
        FunctionOutputValueType::Value(value) => {
            generate_output_leaf_return(value, customs, support, provider, state.names)
        }
        FunctionOutputValueType::Tuple(elements) => {
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
        FunctionOutputValueType::Result { success, failure } => {
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
        FunctionOutputValueType::Option { value } => {
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
        FunctionOutputValueType::Vec(collection) => {
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
        FunctionOutputLeafType::External {
            payload,
            schema,
            store_field,
        } => ProviderValueType::External {
            payload: payload.clone(),
            schema: schema.clone(),
            store_field: store_field.clone(),
        },
        FunctionOutputLeafType::Custom { index, rust } => ProviderValueType::Custom {
            index: *index,
            rust: rust.clone(),
        },
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
                members.push(
                    field
                        .ident
                        .as_ref()
                        .expect("named custom fields must retain their identifiers"),
                );
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

fn encode_tuple_elements(
    elements: &[ProviderValueType],
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
        .map(|(element, value)| encode_intermediate(element, quote!(#value), environment, state))
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
        CustomFieldValueType::Value(type_) => encode_intermediate(type_, input, environment, state),
        CustomFieldValueType::List(list) => {
            let item = state.names.next("returned_custom_list_item");
            let generated =
                encode_intermediate(&list.collection.value, quote!(#item), environment, state);
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

fn encode_intermediate(
    type_: &ProviderValueType,
    input: TokenStream,
    environment: &OutputEnvironment<'_>,
    state: &mut OutputState<'_>,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        ProviderValueType::Declared { type_, .. } => {
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
        ProviderValueType::External { schema, .. } => {
            let support = environment.support;
            let construction = register_host_construction(
                host_value_type(type_, environment.customs, support),
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
        ProviderValueType::Custom { index, .. } => {
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
        ProviderValueType::Tuple(elements) => {
            let mut generated = encode_tuple_elements(elements, input, environment, state);
            let support = environment.support;
            let construction = register_host_construction(
                host_value_type(type_, environment.customs, support),
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
        ProviderValueType::Result { success, failure } => {
            let success_value = state.names.next("result_success");
            let failure_value = state.names.next("result_failure");
            let success_output =
                encode_intermediate(success, quote!(#success_value), environment, state);
            let failure_output =
                encode_intermediate(failure, quote!(#failure_value), environment, state);
            let success_statements = success_output.statements;
            let success_output = success_output.value;
            let failure_statements = failure_output.statements;
            let failure_output = failure_output.value;
            let support = environment.support;
            let success_host = host_value_type(success, environment.customs, support);
            let failure_host = host_value_type(failure, environment.customs, support);
            let construction = register_host_construction(
                host_value_type(type_, environment.customs, support),
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
        ProviderValueType::Option { value } => {
            let some_value = state.names.next("option_some");
            let encoded = encode_intermediate(value, quote!(#some_value), environment, state);
            let statements = encoded.statements;
            let encoded = encoded.value;
            let support = environment.support;
            let host = host_value_type(value, environment.customs, support);
            let construction = register_host_construction(
                host_value_type(type_, environment.customs, support),
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
        FunctionOutputValueType::Value(value) => encode_intermediate(
            &provider_value_from_output_leaf(value),
            input,
            environment,
            state,
        ),
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

fn host_argument_type(
    type_: &FunctionArgumentType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        FunctionArgumentType::Value(type_) => host_value_type(type_, customs, support),
        FunctionArgumentType::Generic(value) => generic_host_type(&value.host, customs, support),
        FunctionArgumentType::List(list) => {
            let item = host_value_type(&list.collection.value, customs, support);
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
        FunctionReturnType::Value(type_) => function_output_host_type(type_, customs, support),
        FunctionReturnType::Generic(value) => generic_host_type(&value.host, customs, support),
        FunctionReturnType::List(list) => {
            let item = host_value_type(&list.collection.value, customs, support);
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
        CustomFieldValueType::Value(type_) => host_value_type(type_, customs, support),
        CustomFieldValueType::List(list) => {
            let item = host_value_type(&list.collection.value, customs, support);
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
        FunctionArgumentType::Value(type_) => wrapper_value_type(type_, customs, support),
        FunctionArgumentType::Generic(value) => {
            let host = generic_host_type(&value.host, customs, support);
            quote!(<#host as #support::HostType>::Value<'__geam_call>)
        }
        FunctionArgumentType::List(list) => {
            let item = host_value_type(&list.collection.value, customs, support);
            quote!(#support::HostList<'__geam_call, #item>)
        }
    }
}

fn wrapper_value_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::Declared { type_, .. } => {
            quote!(
                <<#type_ as #support::ProviderValue>::Host as
                    #support::HostType>::Value<'__geam_call>
            )
        }
        ProviderValueType::External { schema, .. } => {
            quote!(#support::HostExternal<'__geam_call, #support::HostExternalType<#schema>>)
        }
        ProviderValueType::Custom { index, .. } => {
            let schema = &customs[*index].schema;
            quote!(#support::HostCustom<'__geam_call, #support::HostCustomType<#schema>>)
        }
        ProviderValueType::Tuple(elements) => {
            let elements = host_value_type_sequence(elements, customs, support);
            quote!(#support::HostTuple<'__geam_call, #elements>)
        }
        ProviderValueType::Result { success, failure } => {
            let success = host_value_type(success, customs, support);
            let failure = host_value_type(failure, customs, support);
            quote!(#support::HostCustom<'__geam_call, #support::ProviderResult<#success, #failure>>)
        }
        ProviderValueType::Option { value } => {
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
        ProviderValueType::Declared { type_, .. } => {
            format!("declared:{}", quote!(#type_))
        }
        ProviderValueType::External { schema, .. } => format!("external:{schema}"),
        ProviderValueType::Custom { index, .. } => format!("custom:{index}"),
        ProviderValueType::Tuple(elements) => format!(
            "tuple:({})",
            elements
                .iter()
                .map(provider_value_key)
                .collect::<Vec<_>>()
                .join(",")
        ),
        ProviderValueType::Result { success, failure } => format!(
            "result:<{},{}>",
            provider_value_key(success),
            provider_value_key(failure),
        ),
        ProviderValueType::Option { value } => {
            format!("option:<{}>", provider_value_key(value))
        }
    }
}

fn generate_list_decoder(
    decoder: &ListDecoderModel,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let ident = &decoder.ident;
    let item = custom_list_input_value_type(&decoder.value, customs, support)
        .expect("validated List input values must have an input marker");
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
        support,
        &mut names,
    );
    let statements = decoded.statements;
    let value = decoded.value;
    let view = list_item_view_type(&decoder.value, customs, support);
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

fn list_decoder_value(
    ident: &Ident,
    value: &ProviderValueType,
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
    type_: &ProviderValueType,
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
        type_: &ProviderValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListDeclaredAccess>,
    ) {
        match type_ {
            ProviderValueType::Declared { type_, .. } => {
                let field = declared_access_field(type_);
                if accesses.iter().any(|access| access.field == field) {
                    return;
                }
                accesses.push(ListDeclaredAccess {
                    type_: type_.clone(),
                    field,
                });
            }
            ProviderValueType::Tuple(elements) => {
                for element in elements {
                    collect(element, customs, accesses);
                }
            }
            ProviderValueType::Result { success, failure } => {
                collect(success, customs, accesses);
                collect(failure, customs, accesses);
            }
            ProviderValueType::Option { value } => collect(value, customs, accesses),
            ProviderValueType::Custom { index, .. } => {
                for constructor in &customs[*index].constructors {
                    for field in custom_field_models(&constructor.fields) {
                        collect_custom_field(&field.value, customs, accesses);
                    }
                }
            }
            ProviderValueType::Scalar(_) | ProviderValueType::External { .. } => {}
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
    type_: &ProviderValueType,
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
        type_: &ProviderValueType,
        customs: &[CustomModel],
        accesses: &mut Vec<ListExternalAccess>,
    ) {
        match type_ {
            ProviderValueType::Scalar(_) | ProviderValueType::Declared { .. } => {}
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
                    collect(element, customs, accesses);
                }
            }
            ProviderValueType::Result { success, failure } => {
                collect(success, customs, accesses);
                collect(failure, customs, accesses);
            }
            ProviderValueType::Option { value } => collect(value, customs, accesses),
            ProviderValueType::Custom { index, .. } => {
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
    type_: &ProviderValueType,
    input: TokenStream,
    customs: &[CustomModel],
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        ProviderValueType::Scalar(type_) => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_scalar::<#type_>()),
        },
        ProviderValueType::Declared { type_, .. } => {
            let field = declared_access_field(type_);
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(#support::ProviderListItemDecoder::decode(&self.#field, #input)),
            }
        }
        ProviderValueType::External { store_field, .. } => GeneratedValue {
            statements: TokenStream::new(),
            value: quote!(#input.into_external(&self.#store_field)),
        },
        ProviderValueType::Custom { index, .. } => {
            decode_list_custom(*index, input, customs, support, names)
        }
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
                let generated = decode_list_item(element, quote!(#host), customs, support, names);
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
        ProviderValueType::Result { success, failure } => {
            let custom = names.next("list_result");
            let field = names.next("list_result_field");
            let decoded_success =
                decode_list_item(success, quote!(#field), customs, support, names);
            let success_statements = decoded_success.statements;
            let success_value = decoded_success.value;
            let decoded_failure =
                decode_list_item(failure, quote!(#field), customs, support, names);
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
                        1 => {
                            let #field = #custom.take_field(0);
                            #failure_statements
                            ::core::result::Result::Err(#failure_value)
                        }
                        _ => ::core::unreachable!(
                            "typed Result constructor index is out of range"
                        ),
                    }
                },
            }
        }
        ProviderValueType::Option { value } => {
            let custom = names.next("list_option");
            let field = names.next("list_option_field");
            let decoded = decode_list_item(value, quote!(#field), customs, support, names);
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
                        1 => ::core::option::Option::None,
                        _ => ::core::unreachable!(
                            "typed Option constructor index is out of range"
                        ),
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
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_list_item(type_, input, customs, support, names)
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
    support: &TokenStream,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let custom = &customs[custom_index];
    let input_type = custom
        .input
        .as_ref()
        .expect("custom list inputs require a generated input type");
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
            let generated =
                decode_list_custom_field(&field.value, quote!(#host), customs, support, names);
            let generated_statements = generated.statements;
            let generated_value = generated.value;
            statements.extend(quote! {
                let #host = #custom_value.take_field(#field_index);
                #generated_statements
                let #decoded = #generated_value;
            });
        }
        let expression = custom_input_expression(input_type, constructor, &decoded_names);
        arms.push(quote! {
            #constructor_index => {
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
                _ => ::core::unreachable!("typed custom constructor index is out of range"),
            }
        },
    }
}

fn nested_list_decoder_value(
    ident: &Ident,
    value: &ProviderValueType,
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
                names.push(
                    field
                        .ident
                        .as_ref()
                        .expect("named custom fields must retain their identifiers"),
                );
            }
            quote!(#input::#variant { #(#names: #values),* })
        }
    }
}

fn list_item_view_type(
    type_: &ProviderValueType,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    match type_ {
        ProviderValueType::Scalar(type_) => quote!(#type_),
        ProviderValueType::Declared { type_, .. } => {
            quote!(<<#type_ as #support::ProviderValue>::ListInput as
                #support::ProviderListInputValue>::View)
        }
        ProviderValueType::External { payload, .. } => {
            quote!(#support::ProviderExternalItem<#payload>)
        }
        ProviderValueType::Custom { index, .. } => {
            let input = customs[*index]
                .input
                .as_ref()
                .expect("custom List inputs require a generated input type");
            quote!(#input)
        }
        ProviderValueType::Tuple(elements) => {
            let mut types = Vec::with_capacity(elements.len());
            for element in elements {
                types.push(list_item_view_type(element, customs, support));
            }
            quote!((#(#types,)*))
        }
        ProviderValueType::Result { success, failure } => {
            let success = list_item_view_type(success, customs, support);
            let failure = list_item_view_type(failure, customs, support);
            quote!(::core::result::Result<#success, #failure>)
        }
        ProviderValueType::Option { value } => {
            let value = list_item_view_type(value, customs, support);
            quote!(::core::option::Option<#value>)
        }
    }
}

fn list_signature_type(list: &ListType, customs: &[CustomModel], support: &TokenStream) -> Type {
    let item = &list.collection.item;
    let host_item = host_value_type(&list.collection.value, customs, support);
    let decoder = &list.decoder;
    syn::parse_quote! {
        #support::List<
            #item,
            #support::ProviderListContext<'__geam_list, #host_item, #decoder>,
        >
    }
}

fn provider_input_signature_type(type_: &ProviderValueType, support: &TokenStream) -> Type {
    match type_ {
        ProviderValueType::Scalar(type_) => type_.clone(),
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
        ProviderValueType::Tuple(elements) => {
            let mut types = Vec::with_capacity(elements.len());
            for element in elements {
                types.push(provider_input_signature_type(element, support));
            }
            syn::parse_quote!((#(#types,)*))
        }
        ProviderValueType::Result { success, failure } => {
            let success = provider_input_signature_type(success, support);
            let failure = provider_input_signature_type(failure, support);
            syn::parse_quote!(::core::result::Result<#success, #failure>)
        }
        ProviderValueType::Option { value } => {
            let value = provider_input_signature_type(value, support);
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
    path.path
        .segments
        .last_mut()
        .expect("Value path must contain one segment")
        .arguments = PathArguments::AngleBracketed(syn::parse_quote! {
        <#source, #support::ProviderValueContext<'__geam_call, #host>>
    });
    Type::Path(path)
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
                "path" => set_once(&mut partial.path, input.parse()?, &field)?,
                "crate_path" => set_once(&mut partial.crate_path, input.parse()?, &field)?,
                "profile" => set_once(&mut partial.profile, input.parse()?, &field)?,
                "component" => set_once(&mut partial.component, input.parse()?, &field)?,
                "stores" => set_once(&mut partial.stores, input.parse()?, &field)?,
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
    reject_unwrapped_generic(&rust_return_type, &generic_scope)?;
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
            reject_unwrapped_generic(&argument.ty, &generic_scope)?;
            let type_ = classify_argument(
                &argument.ty,
                externals,
                customs,
                list_decoders,
                &mut generic_scope,
                support,
            )?;
            match &type_ {
                FunctionArgumentType::List(list) => {
                    *argument.ty = list_signature_type(list, customs, support);
                    has_list = true;
                }
                FunctionArgumentType::Generic(value) => {
                    *argument.ty = generic_value_signature_type(value, customs, support);
                }
                FunctionArgumentType::Value(value) if contains_source_wrapper(value) => {
                    *argument.ty = provider_input_signature_type(value, support);
                }
                FunctionArgumentType::Value(_) => {}
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
    validate_list_return(&model)?;
    if let FunctionReturnType::List(list) = &model.return_ {
        let list = list_signature_type(list, customs, support);
        let output = if let Some((_, host_result)) = &host_result {
            let host_result = collection_type_with_item(host_result, &list);
            syn::parse_quote!(#host_result)
        } else {
            list
        };
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
        has_list = true;
    } else if let FunctionReturnType::Generic(value) = &model.return_ {
        let output = generic_value_signature_type(value, customs, support);
        let output = if let Some((_, host_result)) = &host_result {
            let host_result = collection_type_with_item(host_result, &output);
            syn::parse_quote!(#host_result)
        } else {
            output
        };
        function.sig.output =
            ReturnType::Type(Token![->](proc_macro2::Span::call_site()), Box::new(output));
    }
    if !matches!(model.call, CallAccess::None) || function_contains_generic_value(&model) {
        prepend_function_lifetime(&mut function.sig.generics, syn::parse_quote!('__geam_call));
    }
    if has_list {
        prepend_function_lifetime(&mut function.sig.generics, syn::parse_quote!('__geam_list));
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

fn contains_source_wrapper(type_: &ProviderValueType) -> bool {
    match type_ {
        ProviderValueType::Result { .. } | ProviderValueType::Option { .. } => true,
        ProviderValueType::Tuple(elements) => elements.iter().any(contains_source_wrapper),
        ProviderValueType::Scalar(_)
        | ProviderValueType::Declared { .. }
        | ProviderValueType::External { .. }
        | ProviderValueType::Custom { .. } => false,
    }
}

fn function_contains_generic_value(function: &FunctionModel) -> bool {
    if matches!(function.return_, FunctionReturnType::Generic(_)) {
        return true;
    }
    for argument in &function.arguments {
        if matches!(argument, FunctionArgumentType::Generic(_)) {
            return true;
        }
    }
    false
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

fn reject_unwrapped_generic(type_: &Type, generics: &GenericParameterScope) -> syn::Result<()> {
    if let Some(ident) = find_unwrapped_generic(type_, generics) {
        return Err(syn::Error::new_spanned(
            type_,
            format!("generic source type `{ident}` must be written as Value<{ident}>",),
        ));
    }
    Ok(())
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
            "Value<...> source shapes must not contain Rust references",
        ));
    }
    if is_type_application_named(type_, "Value") {
        return Err(syn::Error::new_spanned(
            type_,
            "Value<...> must not be nested inside another Value",
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
            "Vec<T> is not a source type inside Value<...>; use geam::List<T>",
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
        let mut host_arguments = Vec::with_capacity(function.inputs.len());
        let mut instantiated_arguments = Vec::with_capacity(function.inputs.len());
        for argument in &function.inputs {
            let argument =
                classify_generic_host_type(&argument.ty, generics, externals, customs, support)?;
            host_arguments.push(argument.host);
            instantiated_arguments.push(argument.instantiated);
        }
        let return_type = match &function.output {
            ReturnType::Default => syn::parse_quote!(()),
            ReturnType::Type(_, type_) => (**type_).clone(),
        };
        let return_type =
            classify_generic_host_type(&return_type, generics, externals, customs, support)?;
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
    if let Some((index, _)) = custom_input_model(type_, customs) {
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
                    "generic declared source types are not supported inside Value<...>",
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

fn classify_argument(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    list_decoders: &mut Vec<ListDecoderModel>,
    generics: &mut GenericParameterScope,
    support: &TokenStream,
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
        if reference.mutability.is_none() && is_qualified_type_path(&reference.elem) {
            return Ok(FunctionArgumentType::Value(Box::new(
                ProviderValueType::Declared {
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
        return Ok(FunctionArgumentType::Generic(Box::new(value)));
    }
    if let Some(item) = collection_item(type_, "List")? {
        let value = classify_collection_input_item(&item, externals, customs, "List")?;
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
        classify_argument_value(type_, externals, customs, generics, support)?,
    )))
}

fn classify_argument_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<ProviderValueType> {
    if generic_value_type(type_, generics, externals, customs, support)?.is_some() {
        return Err(syn::Error::new_spanned(
            type_,
            "Value<...> must be the complete source argument",
        ));
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
                store_field: external.store_field.clone(),
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
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(ProviderValueType::Result {
                success: Box::new(classify_argument_value(
                    success, externals, customs, generics, support,
                )?),
                failure: Box::new(classify_argument_value(
                    failure, externals, customs, generics, support,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(ProviderValueType::Option {
                value: Box::new(classify_argument_value(
                    value, externals, customs, generics, support,
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
            elements.push(classify_argument_value(
                element, externals, customs, generics, support,
            )?);
        }
        return Ok(ProviderValueType::Tuple(elements));
    }
    if let Some((index, custom)) = custom_input_model(type_, customs) {
        debug_assert!(custom.input.is_some());
        return Ok(ProviderValueType::Custom {
            index,
            rust: type_.clone(),
        });
    }
    if let Some(custom) = custom_output_type(type_, customs) {
        let input = if let Some(input) = &custom.input {
            format!("`{input}`")
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
    if is_qualified_type_path(type_) {
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
        return Ok(FunctionReturnType::Value(FunctionOutputValueType::Vec(
            FunctionOutputCollectionType {
                value: Box::new(value),
            },
        )));
    }
    Ok(FunctionReturnType::Value(classify_function_output_value(
        type_, externals, customs, generics, support,
    )?))
}

fn classify_function_output_value(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    generics: &mut GenericParameterScope,
    support: &TokenStream,
) -> syn::Result<FunctionOutputValueType> {
    if generic_value_type(type_, generics, externals, customs, support)?.is_some() {
        return Err(syn::Error::new_spanned(
            type_,
            "Value<...> must be the complete source return",
        ));
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
    if let Some(external) = external_type(type_, externals) {
        return Ok(FunctionOutputValueType::Value(Box::new(
            FunctionOutputLeafType::External {
                payload: external.ident.clone(),
                schema: external.schema.clone(),
                store_field: external.store_field.clone(),
            },
        )));
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
    if let Some((index, _)) = custom_output_type_with_index(type_, customs) {
        return Ok(FunctionOutputValueType::Value(Box::new(
            FunctionOutputLeafType::Custom {
                index,
                rust: type_.clone(),
            },
        )));
    }
    if let Some((_, custom)) = custom_input_model(type_, customs) {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "custom input `{}` cannot be returned; return `{}`",
                custom.input.as_ref().expect("matched input must exist"),
                custom.ident
            ),
        ));
    }
    if is_qualified_type_path(type_) {
        return Ok(FunctionOutputValueType::Value(Box::new(
            FunctionOutputLeafType::Declared {
                type_: type_.clone(),
                input: DeclaredInput::Owned,
            },
        )));
    }
    Ok(FunctionOutputValueType::Value(Box::new(
        FunctionOutputLeafType::Scalar(type_.clone()),
    )))
}

fn classify_collection_input_item(
    type_: &Type,
    externals: &[ExternalModel],
    customs: &[CustomModel],
    collection: &str,
) -> syn::Result<ProviderValueType> {
    if is_type_application_named(type_, "Value") {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "Value<...> cannot be a {collection} item; wrap the complete generic source shape in Value<...>"
            ),
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
            return Ok(ProviderValueType::Result {
                success: Box::new(classify_collection_input_item(
                    success, externals, customs, collection,
                )?),
                failure: Box::new(classify_collection_input_item(
                    failure, externals, customs, collection,
                )?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(ProviderValueType::Option {
                value: Box::new(classify_collection_input_item(
                    value, externals, customs, collection,
                )?),
            });
        }
        SourceWrapper::Other => {}
    }
    if let Some(external) = external_type(type_, externals) {
        return Ok(ProviderValueType::External {
            payload: external.ident.clone(),
            schema: external.schema.clone(),
            store_field: external.store_field.clone(),
        });
    }
    if let Some((index, _)) = custom_input_model(type_, customs) {
        return Ok(ProviderValueType::Custom {
            index,
            rust: type_.clone(),
        });
    }
    if let Some(custom) = custom_output_type(type_, customs) {
        let input = if let Some(input) = &custom.input {
            format!("`{input}`")
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
        return Ok(ProviderValueType::Tuple(elements));
    }
    if is_qualified_type_path(type_) {
        return Ok(ProviderValueType::Declared {
            type_: type_.clone(),
            input: DeclaredInput::Owned,
        });
    }
    Ok(ProviderValueType::Scalar(type_.clone()))
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
    use super::{
        ExternalArguments, GenericParameterScope, ModuleArguments, classify_collection_input_item,
        expand, find_unwrapped_generic, list_item_view_type, provider_value_key,
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
                "Value<...> must not be nested inside another Value",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<&Item>) -> Value<Item> { todo!() }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Option<&Item>>) -> bool { true }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Vec<Item>>) -> Value<Item> { todo!() }
                    }
                },
                "Vec<T> is not a source type inside Value<...>; use geam::List<T>",
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
                "generic declared source types are not supported inside Value<...>",
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
                        fn identity<Item>(value: Value<Item>) -> Option<Value<Item>> { todo!() }
                    }
                },
                "Value<...> must be the complete source return",
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
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result<&Item, Item>>) -> bool { true }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Result<Item, &Item>>) -> bool { true }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<fn() -> &Item>) -> bool { true }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<sibling::Box<Item, bool>>) -> bool { true }
                    }
                },
                "generic declared source types are not supported inside Value<...>",
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
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<fn(&Item) -> Item>) -> bool { true }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
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
                "Value<...> source shapes must not contain Rust references",
            ),
            (
                quote! {
                    mod counter {
                        #[geam::function]
                        fn identity<Item>(value: Value<Item>) -> Option<Value<&Item>> { todo!() }
                    }
                },
                "Value<...> source shapes must not contain Rust references",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
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
                "duplicate custom input type `Status`",
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
                "duplicate custom input type `SharedInput`",
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
            provider_value_key(&input),
            "result:<option:<scalar:BigInt>,option:<scalar:EcoString>>",
        );
        assert_eq!(
            list_item_view_type(&input, &[], &quote!(geam_core)).to_string(),
            ":: core :: result :: Result < :: core :: option :: Option < BigInt > , :: core :: option :: Option < EcoString > >",
        );
    }

    #[test]
    fn source_result_and_option_reject_nested_collections_at_the_exact_direction() {
        let argument_cases = [
            (
                quote! {
                    mod values {
                        #[geam::function]
                        fn inspect(
                            value: Result<geam_core::List<BigInt>, EcoString>,
                        ) -> bool { true }
                    }
                },
                "geam::List<T> is supported only as a top-level source argument",
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
