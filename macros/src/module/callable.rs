use super::syntax::{FunctionValidationContext, classify_input, classify_return};
use super::{
    CallbackType, FunctionArguments, FunctionParameter, GenericParameterScope, ListDecoderModel,
    SourceCompletion,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::{Attribute, Ident, ItemFn, Token, Type, Visibility};

#[derive(Clone)]
pub(super) struct CallableModel {
    pub(super) factory: Ident,
    pub(super) schema: Ident,
    pub(super) visibility: Visibility,
    pub(super) captures: Vec<usize>,
    pub(super) capture_encoder: CallbackType,
    pub(super) borrowed_capture_encoder: CallbackType,
    pub(super) returned: CallbackType,
}

pub(super) fn bindings_type(function: &super::FunctionModel, profile: &TokenStream) -> TokenStream {
    let name = format_ident!("__GeamFactories_{}", function.ident.unraw());
    let parameters = function.generics.iter().map(|generic| &generic.ident);
    quote!(#name<#profile, #(#parameters),*>)
}

pub(super) fn bindings_definition(
    function: &super::FunctionModel,
    requirements: &TokenStream,
    offset: usize,
    bounds: &[TokenStream],
    support: &TokenStream,
    flavor: super::InputOwnership,
) -> TokenStream {
    let mode = capture_mode(flavor, support);
    let name = format_ident!("__GeamFactories_{}", function.ident.unraw());
    let parameters = function
        .generics
        .iter()
        .map(|generic| &generic.ident)
        .collect::<Vec<_>>();
    let type_ = bindings_type(function, &quote!(__GeamProfile));
    let selections = function.factories.iter().enumerate().map(|(index, factory)| {
        let index = super::function::provider_construction_index(offset + index, support);
        let required = quote!(<#factory as #support::ProviderFactoryCodec<__GeamProfile, #mode>>::Requirements);
        quote! {
            impl<__GeamProfile, #(#parameters,)*> #support::ProviderFactoryBinding<#factory, #required> for #type_
            where
                __GeamProfile: __GeamModuleProfile,
                #(#parameters: #support::ProviderValue + 'static,)*
                #(#bounds,)*
            {
                fn select<'call>(proof: &#support::ProviderConstructions<'call, Self::Requirements>)
                    -> #support::ProviderConstructions<'call, #required>
                { proof.select::<#index>() }
            }
        }
    });
    quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct #name<__GeamProfile, #(#parameters,)*>(::core::marker::PhantomData<fn() -> (__GeamProfile, #(#parameters,)*)>);
        impl<__GeamProfile, #(#parameters,)*> #support::ProviderFactoryBindings for #type_
        where
            __GeamProfile: __GeamModuleProfile,
            #(#parameters: #support::ProviderValue + 'static,)*
            #(#bounds,)*
        { type Requirements = #requirements; type CaptureMode = #mode; }
        #(#selections)*
    }
}

pub(super) fn capture_mode(flavor: super::InputOwnership, support: &TokenStream) -> TokenStream {
    match flavor {
        super::InputOwnership::Borrowed => quote!(#support::ProviderImmediateCaptures),
        super::InputOwnership::Owned => quote!(#support::ProviderOwnedCaptures),
    }
}

pub(super) fn invocation_names<'a>(
    function: &super::FunctionModel,
    names: &'a [Ident],
) -> Vec<&'a Ident> {
    names
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            !function
                .callable
                .as_ref()
                .is_some_and(|callable| callable.captures.contains(index))
        })
        .map(|(_, name)| name)
        .collect()
}

pub(super) fn capture_parameters(
    function: &super::FunctionModel,
    names: &[Ident],
    customs: &[super::CustomModel],
    support: &TokenStream,
    lifetime: &TokenStream,
) -> (TokenStream, TokenStream) {
    let Some(callable) = &function.callable else {
        return (TokenStream::new(), TokenStream::new());
    };
    let types = callable
        .captures
        .iter()
        .map(|index| {
            super::signature::host_argument_type(&function.arguments[*index], customs, support)
        })
        .collect::<Vec<_>>();
    let types = super::signature::host_type_token_sequence(&types, support);
    let names = callable
        .captures
        .iter()
        .map(|index| {
            let name = &names[*index];
            quote!(#name)
        })
        .collect::<Vec<_>>();
    let pattern = super::function::host_value_sequence(&names);
    (
        quote!(__geam_captures: #support::HostCaptures<#lifetime, #types>,),
        quote!(let #pattern = call.captures(__geam_captures);),
    )
}

pub(super) fn schema_type(
    function: &super::FunctionModel,
    profile: &TokenStream,
) -> Option<TokenStream> {
    function.callable.as_ref().map(|callable| {
        let schema = &callable.schema;
        let parameters = function.generics.iter().map(|generic| &generic.ident);
        quote!(#schema<#profile, #(#parameters),*>)
    })
}

pub(super) struct Declaration<'model> {
    pub(super) function: &'model super::FunctionModel,
    pub(super) callable: &'model CallableModel,
    pub(super) customs: &'model [super::CustomModel],
    pub(super) support: &'model TokenStream,
    pub(super) component: &'model TokenStream,
    pub(super) module: &'model syn::LitStr,
    pub(super) constructions: &'model TokenStream,
    pub(super) registered_return: &'model TokenStream,
    pub(super) bounds: &'model [TokenStream],
}

impl Declaration<'_> {
    pub(super) fn generate(self) -> TokenStream {
        let Self {
            function,
            callable,
            customs,
            support,
            component,
            module,
            constructions,
            registered_return,
            bounds,
        } = self;
        let factory = &callable.factory;
        let schema = &callable.schema;
        let visibility = &callable.visibility;
        let name = function.ident.unraw().to_string();
        let parameters = function
            .generics
            .iter()
            .map(|generic| &generic.ident)
            .collect::<Vec<_>>();
        let schema_type = quote!(#schema<__GeamProfile, #(#parameters),*>);
        let generic_arguments = parameters
            .iter()
            .map(|ident| quote!(#ident))
            .collect::<Vec<_>>();
        let factory_type =
            super::signature::callback_codec_type(factory, generic_arguments.clone());
        let result_codec = super::signature::callback_codec_type(
            &callable.returned.codec,
            generic_arguments.clone(),
        );
        let capture_types = callable
            .captures
            .iter()
            .map(|index| {
                super::signature::host_argument_type(&function.arguments[*index], customs, support)
            })
            .collect::<Vec<_>>();
        let capture_types = super::signature::host_type_token_sequence(&capture_types, support);
        let argument_types = function
            .arguments
            .iter()
            .enumerate()
            .filter(|(index, _)| !callable.captures.contains(index))
            .map(|(_, argument)| super::signature::host_argument_type(argument, customs, support))
            .collect::<Vec<_>>();
        let argument_types = super::signature::host_type_token_sequence(&argument_types, support);
        let output = super::signature::callback_signature_type(
            &callable.returned,
            customs,
            &quote!(__GeamProfile),
            support,
        );
        let result_requirement = super::function::generate_callback_codec(
            &callable.returned,
            &function.generics,
            customs,
            support,
        )
        .requirements;
        let first = super::function::provider_construction_index(0, support);
        let second = super::function::provider_construction_index(1, support);
        let third = super::function::provider_construction_index(2, support);
        let factory_codecs = [
            (&callable.borrowed_capture_encoder, super::InputOwnership::Borrowed),
            (&callable.capture_encoder, super::InputOwnership::Owned),
        ].into_iter().map(|(encoder, flavor)| {
            let mode = capture_mode(flavor, support);
            let capture_codec = super::signature::callback_codec_type(
                &encoder.codec, generic_arguments.clone(),
            );
            let captures = encoder.arguments.iter().map(|argument| {
                super::signature::callback_output_signature_type(
                    argument, customs, support, flavor, &quote!(__GeamProfile),
                )
            });
            let capture_requirement = super::function::generate_callback_codec_with_flavor(
                encoder, &function.generics, customs, support, flavor,
            ).requirements;
            let requirements = quote!(#support::ProviderConstructionList<
                #support::ProviderConstruction<#support::HostCreatedFunction<#schema_type>>,
                #support::ProviderConstructionList<#capture_requirement,
                    #support::ProviderConstructionList<#result_requirement, #support::ProviderNoConstructions>>
            >);
            quote! {
                impl<__GeamProfile, #(#parameters,)*> #support::ProviderFactoryCodec<__GeamProfile, #mode> for #factory_type
                where
                    __GeamProfile: __GeamModuleProfile,
                    #(#parameters: #support::ProviderValue + 'static,)*
                    #(#bounds,)*
                {
                    type Requirements = #requirements;
                    type Captures = (#(#captures,)*);
                    type Output = #output;
                    fn create<'call, __GeamCaller, __GeamReturn>(
                        call: &mut #support::HostCall<'call, __GeamProfile, __GeamCaller, __GeamReturn>,
                        constructions: &#support::ProviderConstructions<'call, Self::Requirements>,
                        captures: Self::Captures,
                    ) -> ::core::result::Result<Self::Output, #support::HostCallError>
                    where __GeamCaller: #support::HostProvider<__GeamProfile>, __GeamReturn: #support::HostType
                    {
                        constructions.with_call::<__GeamProfile, __GeamCaller, __GeamReturn, __GeamProvider, _>(call, |mut call, proof| {
                            let captures = <#capture_codec as #support::ProviderCallbackCodec<__GeamProfile, __GeamProvider, ()>>::into_host_arguments(captures, &mut call, &proof.select::<#second>())?;
                            let callable = call.construct_function(proof.select::<#first>().token(), captures);
                            ::core::result::Result::Ok(#support::Callback::from_owned_host::<#result_codec, _, _>(&call, callable, proof.select::<#third>()))
                        })
                    }
                }
            }
        });
        quote! {
            #visibility struct #factory<#(#parameters,)*>(::core::marker::PhantomData<fn() -> (#(#parameters,)*)>);
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #visibility struct #schema<__GeamProfile, #(#parameters,)*>(::core::marker::PhantomData<fn() -> (__GeamProfile, #(#parameters,)*)>);
            impl<__GeamProfile, #(#parameters,)*> #support::HostCallableSchema for #schema_type
            where
                __GeamProfile: __GeamModuleProfile,
                #(#parameters: #support::ProviderValue + 'static,)*
                #(#bounds,)*
            {
                const PACKAGE: &'static str = <#component as #support::ProviderPackage>::PACKAGE;
                const MODULE: &'static str = #module;
                const NAME: &'static str = #name;
                type Arguments = #argument_types;
                type Return = #registered_return;
                type Captures = #capture_types;
                type Constructions = #constructions;
                type Completion = #support::HostReturns;
            }
            #(#factory_codecs)*
        }
    }
}

pub(super) fn parse_arguments(attribute: &Attribute) -> syn::Result<FunctionArguments> {
    attribute.parse_args_with(|input: syn::parse::ParseStream<'_>| {
        let mut factory = None;
        let mut ordinary = TokenStream::new();
        while !input.is_empty() {
            let key = input.call(Ident::parse_any)?;
            if key == "factory" {
                input.parse::<Token![=]>()?;
                if factory.replace(input.parse::<Ident>()?).is_some() {
                    return Err(syn::Error::new_spanned(
                        key,
                        "duplicate callable argument `factory`",
                    ));
                }
            } else {
                ordinary.extend(quote!(#key));
                if input.parse::<Token![=]>().is_ok() {
                    let value = input.parse::<Ident>()?;
                    ordinary.extend(quote!(= #value));
                }
                ordinary.extend(quote!(,));
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        let factory = factory.ok_or_else(|| {
            syn::Error::new_spanned(attribute, "callable declarations require `factory = Name`")
        })?;
        let mut arguments = syn::parse2::<FunctionArguments>(ordinary)?;
        arguments.callable = Some(factory);
        Ok(arguments)
    })
}

pub(super) fn factory_parameter(type_: &Type) -> syn::Result<Type> {
    let Some((declaration, _)) = super::type_syntax::collection_item_with_path(type_, "Factory")?
    else {
        return Err(syn::Error::new_spanned(
            type_,
            "factory parameters require `Factory<Declaration>`",
        ));
    };
    Ok(declaration)
}

pub(super) struct CallableParser<'parser, 'model> {
    pub(super) list_decoders: &'parser mut Vec<ListDecoderModel>,
    pub(super) generics: &'parser mut GenericParameterScope,
    pub(super) context: &'parser FunctionValidationContext<'model>,
}

impl CallableParser<'_, '_> {
    pub(super) fn model(
        self,
        function: &ItemFn,
        factory: Ident,
        captures: Vec<usize>,
        parameters: &[FunctionParameter],
        completion: SourceCompletion,
        returned: &Type,
    ) -> syn::Result<CallableModel> {
        let Self {
            list_decoders,
            generics,
            context,
        } = self;
        let support = context.support;
        let source = parameters
            .iter()
            .filter_map(|parameter| match parameter {
                FunctionParameter::Source(source) => Some(source.syntax.ty.as_ref()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut captured_types = Vec::new();
        let mut argument_types = Vec::new();
        let returned = directional_type(returned, Direction::Input, context);
        let returned: Type = if completion == SourceCompletion::Work {
            syn::parse_quote!(#support::Future<#returned>)
        } else {
            returned
        };
        let return_ = classify_input(
            &returned,
            context.externals,
            context.customs,
            list_decoders,
            generics,
            support,
            true,
        )?;
        let mut arguments = Vec::new();
        let mut capture_arguments = Vec::new();
        // Keep the declaration's return-first, then source-order generic indices.
        // Capture and invocation codecs share this one directional classification.
        for (index, source) in source.iter().enumerate() {
            let type_ = directional_type(source, Direction::Output, context);
            let argument = classify_return(
                &type_,
                context.externals,
                context.customs,
                list_decoders,
                generics,
                support,
            )?;
            if captures.get(captured_types.len()) == Some(&index) {
                captured_types.push(type_);
                capture_arguments.push(argument);
            } else {
                argument_types.push(type_);
                arguments.push(argument);
            }
        }
        let ident = &function.sig.ident;
        let returned = CallbackType {
            signature: syn::parse_quote!(fn(#(#argument_types),*) -> #returned),
            path: syn::parse_quote!(#support::Callback),
            arguments,
            return_: Box::new(return_),
            codec: format_ident!("__GeamCallableCallback_{}", ident.unraw()),
            generics: generics.declared.clone(),
        };
        // A capture pack is a type sequence, so it has no invocation-arity limit.
        let ident = &function.sig.ident;
        let capture_encoder = CallbackType {
            signature: syn::parse_quote!(fn(#(#captured_types),*) -> ()),
            path: syn::parse_quote!(#support::Callback),
            arguments: capture_arguments,
            return_: Box::new(super::FunctionInputType::Value(Box::new(
                super::FunctionInputValueType::Scalar(syn::parse_quote!(())),
            ))),
            codec: format_ident!("__GeamCaptures_{}", ident.unraw()),
            generics: generics.declared.clone(),
        };
        let mut borrowed_capture_encoder = capture_encoder.clone();
        borrowed_capture_encoder.codec = format_ident!("__GeamBorrowedCaptures_{}", ident.unraw());
        Ok(CallableModel {
            factory,
            schema: format_ident!("__GeamCallable_{}", ident.unraw()),
            visibility: function.vis.clone(),
            captures,
            capture_encoder,
            borrowed_capture_encoder,
            returned,
        })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Input,
    Output,
}

fn directional_type(
    type_: &Type,
    direction: Direction,
    context: &FunctionValidationContext<'_>,
) -> Type {
    struct Directional<'context> {
        direction: Direction,
        context: &'context FunctionValidationContext<'context>,
    }
    impl syn::visit_mut::VisitMut for Directional<'_> {
        fn visit_type_mut(&mut self, type_: &mut Type) {
            if let Type::Reference(reference) = type_ {
                *type_ = (*reference.elem).clone();
            }
            if let Type::Path(path) = type_ {
                let single = path.path.segments.len() == 1;
                let mut replacement = None;
                for segment in path.path.segments.iter_mut().rev().take(1) {
                    if matches!(
                        segment.ident.to_string().as_str(),
                        "Value" | "Callback" | "List"
                    ) {
                        return;
                    }
                    if single {
                        for custom in self.context.customs {
                            if let Some(input) = &custom.input {
                                let (from, to) = match self.direction {
                                    Direction::Input => (&custom.ident, &input.ident),
                                    Direction::Output => (&input.ident, &custom.ident),
                                };
                                if segment.ident == *from {
                                    segment.ident = to.clone();
                                    break;
                                }
                            }
                        }
                        for external in self.context.externals {
                            if let Some(generic) = &external.generic {
                                let (from, to) = match self.direction {
                                    Direction::Input => (&external.ident, &generic.input),
                                    Direction::Output => (&generic.input, &external.ident),
                                };
                                if segment.ident == *from {
                                    segment.ident = to.clone();
                                    break;
                                }
                            }
                        }
                    }
                    if path.qself.is_none()
                        && segment.ident == "Vec"
                        && matches!(self.direction, Direction::Input)
                    {
                        let arguments = segment.arguments.clone();
                        let support = self.context.support;
                        replacement = Some(syn::parse_quote!(#support::List #arguments));
                    }
                }
                if let Some(replacement) = replacement {
                    path.path = replacement;
                }
            }
            syn::visit_mut::visit_type_mut(self, type_);
        }
    }
    let mut result = type_.clone();
    syn::visit_mut::VisitMut::visit_type_mut(&mut Directional { direction, context }, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::parse_arguments;
    use crate::module::expand;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn callable_option_syntax_reports_the_exact_invalid_token() {
        for (attribute, expected) in [
            (
                syn::parse_quote!(#[geam::callable(= Factory)]),
                "expected ident",
            ),
            (
                syn::parse_quote!(#[geam::callable(factory Factory)]),
                "expected `=`",
            ),
            (
                syn::parse_quote!(#[geam::callable(factory = "Factory")]),
                "expected identifier",
            ),
            (
                syn::parse_quote!(#[geam::callable(factory = Factory profile = Host)]),
                "expected `,`",
            ),
            (
                syn::parse_quote!(#[geam::callable(factory = Factory, profile = "Host")]),
                "expected identifier",
            ),
            (
                syn::parse_quote!(#[geam::callable(factory = Factory, unknown)]),
                "unknown function argument `unknown`",
            ),
        ] {
            assert_eq!(
                parse_arguments(&attribute).err().unwrap().to_string(),
                expected
            );
        }
        assert_eq!(
            declaration_error(quote! {
                #[geam::callable(factory = "Factory")]
                fn body() -> bool { true }
            }),
            "expected identifier"
        );
        assert_eq!(
            declaration_error(quote! {
                #[geam::function]
                fn body(#[geam::call] call: &mut Call<()>, #[geam::factory] #[geam::factory] factory: Factory<Other>) -> bool { true }
            }),
            "duplicate `#[geam::factory]` attribute"
        );
        assert_eq!(
            super::factory_parameter(&syn::parse_quote!(Factory<bool, bool>))
                .err()
                .unwrap()
                .to_string(),
            "Factory requires exactly one type argument"
        );
    }

    #[test]
    fn directional_views_qualify_owned_vectors_and_preserve_associated_types() {
        let support = quote!(::geam::__macro_support);
        let context = super::FunctionValidationContext::new(None, &[], &[], &support);
        for (original, input) in [
            (
                quote!(Vec<bool>),
                quote!(::geam::__macro_support::List<bool>),
            ),
            (
                quote!(std::vec::Vec<Vec<bool>>),
                quote!(::geam::__macro_support::List<::geam::__macro_support::List<bool>>),
            ),
            (
                quote!((bool, Vec<bool>)),
                quote!((bool, ::geam::__macro_support::List<bool>)),
            ),
            (quote!(<Item as Trait>::Vec), quote!(<Item as Trait>::Vec)),
            (quote!(Value<Vec<bool>>), quote!(Value<Vec<bool>>)),
            (quote!(List<Vec<bool>>), quote!(List<Vec<bool>>)),
            (
                quote!(Callback<fn() -> Vec<bool>>),
                quote!(Callback<fn() -> Vec<bool>>),
            ),
        ] {
            let original: syn::Type = syn::parse2(original).unwrap();
            let input: syn::Type = syn::parse2(input).unwrap();
            assert_eq!(
                super::directional_type(&original, super::Direction::Input, &context),
                input
            );
            assert_eq!(
                super::directional_type(&original, super::Direction::Output, &context),
                original
            );
        }
        let borrowed: syn::Type = syn::parse_quote!(&bool);
        assert_eq!(
            super::directional_type(&borrowed, super::Direction::Input, &context),
            syn::parse_quote!(bool)
        );
    }

    #[test]
    fn directional_nominal_views_preserve_the_selected_declaration() {
        let support = quote!(support);
        let mut items: syn::File = syn::parse_quote! {
            #[geam::custom]
            enum OutputOnly { OutputOnly }
            #[geam::custom(input = FirstInput)]
            enum First { First(bool) }
            #[geam::custom(input = SecondInput)]
            enum Second { Second(bool) }
        };
        let customs = crate::module::custom_value::collect_custom_declarations(
            &mut items.items,
            &mut Default::default(),
            &mut Default::default(),
            &Default::default(),
            &[],
            &mut Vec::new(),
            &support,
        )
        .unwrap()
        .models;
        let mut externals = Vec::new();
        for (payload, arguments) in [
            (
                quote!(
                    struct Token {
                        value: bool,
                    }
                ),
                quote!(name = "Token"),
            ),
            (
                quote!(
                    struct Parcel<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }
                ),
                quote!(name = "Parcel", parameters = [Item], input = ParcelInput),
            ),
            (
                quote!(
                    struct Package<Item> {
                        #[geam::stored]
                        value: Stored<Item>,
                    }
                ),
                quote!(name = "Package", parameters = [Item], input = PackageInput),
            ),
        ] {
            externals.push(
                crate::module::syntax::build_external_model(
                    externals.len(),
                    &mut syn::parse2(payload).unwrap(),
                    syn::parse2(arguments).unwrap(),
                    &support,
                )
                .unwrap(),
            );
        }
        let context = super::FunctionValidationContext::new(None, &externals, &customs, &support);
        for (original, input, output) in [
            (quote!(OutputOnly), quote!(OutputOnly), quote!(OutputOnly)),
            (quote!(First), quote!(FirstInput), quote!(First)),
            (quote!(SecondInput), quote!(SecondInput), quote!(Second)),
            (quote!(Token), quote!(Token), quote!(Token)),
            (
                quote!(Parcel<bool>),
                quote!(ParcelInput<bool>),
                quote!(Parcel<bool>),
            ),
            (
                quote!(PackageInput<bool>),
                quote!(PackageInput<bool>),
                quote!(Package<bool>),
            ),
            (
                quote!(remote::First),
                quote!(remote::First),
                quote!(remote::First),
            ),
        ] {
            let original = syn::parse2(original).unwrap();
            for (direction, expected) in [
                (super::Direction::Input, input),
                (super::Direction::Output, output),
            ] {
                assert_eq!(
                    super::directional_type(&original, direction, &context),
                    syn::parse2::<syn::Type>(expected).unwrap(),
                );
            }
        }
    }

    #[test]
    fn callable_options_preserve_factory_profile_and_completion() {
        let arguments = parse_arguments(&syn::parse_quote!(
            #[geam::callable(factory = Transform, profile = Host, await)]
        ))
        .unwrap();
        assert_eq!(arguments.callable.unwrap(), "Transform");
        assert_eq!(arguments.profile.unwrap(), "Host");
        assert_eq!(arguments.await_.unwrap(), "await");
        assert_eq!(
            parse_arguments(&syn::parse_quote!(#[geam::callable()]))
                .err()
                .unwrap()
                .to_string(),
            "callable declarations require `factory = Name`",
        );
        assert_eq!(
            parse_arguments(
                &syn::parse_quote!(#[geam::callable(factory = First, factory = Second)])
            )
            .err()
            .unwrap()
            .to_string(),
            "duplicate callable argument `factory`",
        );
    }

    #[test]
    fn captures_are_explicit_and_distinct_from_call_and_factory_parameters() {
        for (function, expected) in [
            (
                quote! {
                    #[geam::function]
                    fn source(#[geam::capture] value: bool) -> bool { value }
                },
                "`#[geam::capture]` requires a callable declaration",
            ),
            (
                quote! {
                    #[geam::callable(factory = Bad)]
                    fn body(#[geam::call] #[geam::capture] call: &mut Call<()>) -> bool { true }
                },
                "call, factory and capture parameters are distinct",
            ),
            (
                quote! {
                    #[geam::callable(factory = Bad)]
                    fn body(#[geam::factory] #[geam::capture] factory: Factory<Other>) -> bool { true }
                },
                "call, factory and capture parameters are distinct",
            ),
            (
                quote! {
                    #[geam::callable(factory = Bad)]
                    fn body(#[geam::capture] #[geam::capture] value: bool) -> bool { value }
                },
                "duplicate `#[geam::capture]` attribute",
            ),
        ] {
            assert_eq!(declaration_error(function), expected);
        }
    }

    #[test]
    fn factory_permissions_have_one_position_owner_and_unique_declarations() {
        for arguments in [
            quote!(#[geam::factory] factory: Factory<Other>),
            quote!(#[geam::call] call: &Call<()>, #[geam::factory] factory: Factory<Other>),
            quote!(#[geam::call] call: &mut Call<()>, value: bool, #[geam::factory] factory: Factory<Other>),
        ] {
            assert_eq!(
                declaration_error(quote! {
                    #[geam::function]
                    fn source(#arguments) -> bool { true }
                }),
                "factory parameters must follow a mutable Call and precede source arguments"
            );
        }
        assert_eq!(
            declaration_error(quote! {
                #[geam::function]
                fn source(#[geam::call] call: &mut Call<()>, #[geam::factory] factory: bool) -> bool { true }
            }),
            "factory parameters require `Factory<Declaration>`"
        );
        assert_eq!(
            declaration_error(quote! {
                #[geam::function]
                fn source(
                    #[geam::call] call: &mut Call<()>,
                    #[geam::factory] first: Factory<Other>,
                    #[geam::factory] second: Factory<Other>,
                ) -> bool { true }
            }),
            "duplicate factory declaration parameter"
        );
    }

    #[test]
    fn captures_do_not_relax_the_seven_invocation_argument_boundary() {
        assert_eq!(
            declaration_error(quote! {
                #[geam::callable(factory = TooMany)]
                fn body(
                    #[geam::capture] capture: bool,
                    a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool,
                ) -> bool { capture }
            }),
            "provider functions support at most seven source arguments"
        );
    }

    fn declaration_error(function: TokenStream) -> String {
        expand(
            quote!(path = "callable_test", crate_path = geam_core),
            quote!(mod callable_test { #function }),
        )
        .unwrap_err()
        .to_string()
    }
    #[test]
    fn callable_factory_interfaces_validate_raw_input_capture_and_result_syntax() {
        for declaration in [
            quote! { #[geam::callable(factory = Invalid)] fn body(value: Callback<bool>) -> bool { true } },
            quote! { #[geam::callable(factory = Invalid)] fn body(#[geam::capture] value: Callback<bool>) -> bool { true } },
            quote! { #[geam::callable(factory = Invalid)] fn body() -> Callback<bool> { todo!() } },
        ] {
            assert_eq!(
                declaration_error(declaration),
                "Callback<T> requires a safe non-variadic Rust fn signature"
            );
        }
    }

    #[test]
    fn generated_captures_do_not_count_toward_invocation_arity() {
        for (asynchronous, marker) in [(false, quote!()), (true, quote!(async))] {
            let expanded = expand(
                quote!(path = "callable_test", crate_path = geam_core),
                quote! {
                    mod callable_test {
                        #[geam::callable(factory = Keep)]
                        #marker fn keep(
                            #[geam::capture] a: bool, #[geam::capture] b: bool,
                            #[geam::capture] c: bool, #[geam::capture] d: bool,
                            #[geam::capture] e: bool, #[geam::capture] f: bool,
                            #[geam::capture] g: bool, #[geam::capture] h: bool,
                            value: bool,
                        ) -> bool { a && b && c && d && e && f && g && h && value }
                    }
                },
            )
            .unwrap();
            let module: syn::ItemMod = syn::parse2(expanded.clone()).unwrap();
            let keep = module
                .content
                .unwrap()
                .1
                .into_iter()
                .filter_map(|item| {
                    if let syn::Item::Fn(function) = item {
                        Some(function)
                    } else {
                        None
                    }
                })
                .find(|function| function.sig.ident == "keep")
                .unwrap();
            assert_eq!(keep.sig.inputs.len(), 9);
            assert_eq!(keep.sig.asyncness.is_some(), asynchronous);
            assert!(keep.attrs.iter().any(|attribute| {
                quote!(#attribute)
                    .to_string()
                    .contains("clippy :: too_many_arguments")
            }));
            assert!(expanded.to_string().contains("struct Keep"));
            assert_eq!(
                expanded.to_string().contains("HostFutureType"),
                asynchronous
            );
        }
    }
    #[test]
    fn custom_work_fields_emit_nested_codecs_and_propagate_work_requirements() {
        let expanded = expand(
            quote!(path = "custom_work", crate_path = geam_core),
            quote! {
                mod custom_work {
                    #[geam::custom(input = PendingInput)]
                    enum Pending { Pending(Future<Callback<fn(bool) -> bool>>) }
                    #[geam::custom(input = ReplyInput)]
                    enum Reply { Reply(Callback<fn(bool) -> bool>) }
                    #[geam::function(await)]
                    async fn inspect(value: PendingInput) -> bool { true }
                    #[geam::function]
                    fn wrapped() -> (Reply, bool) { todo!() }
                }
            },
        )
        .unwrap()
        .to_string();
        assert!(expanded.contains("ProviderCallbackCodec"));
        assert!(expanded.contains("ProviderFutureCodec"));
        assert!(expanded.contains("HostWorkProfile"));
        assert!(expanded.contains("enum PendingInput"));
        assert!(expanded.contains("enum ReplyInput"));
    }

    #[test]
    fn phantom_parameter_markers_do_not_shadow_author_declared_variants() {
        let expanded = expand(
            quote!(path = "nominal", crate_path = geam_core),
            quote! {
                mod nominal {
                    pub struct Ordinary;
                    #[geam::custom]
                    enum Flag<Item> { __GeamParameters, __GeamParameters_ }
                }
            },
        )
        .unwrap();
        let module: syn::ItemMod = syn::parse2(expanded).unwrap();
        let flag = module
            .content
            .unwrap()
            .1
            .into_iter()
            .filter_map(|item| {
                if let syn::Item::Enum(item) = item {
                    Some(item)
                } else {
                    None
                }
            })
            .find(|item| item.ident == "Flag")
            .unwrap();
        assert_eq!(
            flag.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
            [
                "__GeamParameters",
                "__GeamParameters_",
                "__GeamParameters__"
            ]
        );
        assert_eq!(flag.variants[0].fields, syn::Fields::Unit);
        assert_eq!(flag.variants[1].fields, syn::Fields::Unit);
        assert_eq!(flag.variants[2].fields.len(), 2);
    }
}
