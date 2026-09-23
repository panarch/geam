use super::custom_value::{
    CustomFieldValueType, CustomFields, CustomInputModel, CustomModel, custom_field_models,
};
use super::function::{
    generate_custom_intermediate, generate_custom_return, host_value_sequence,
    provider_construction_bindings, provider_requirement_selection_bounds,
    provider_requirement_sequence,
};
use super::list::{
    custom_input_expression, custom_input_ident, list_declared_accesses, list_decoder_ident,
    list_decoder_value,
};
use super::signature::{host_custom_field_type, host_static_value_type};
use super::{
    GeneratedNames, GeneratedValue, InputOwnership, OutputEnvironment, OutputState, StaticValueType,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use syn::ext::IdentExt;
use syn::{Ident, LitStr};

pub(super) fn generate_custom_declaration(
    custom_index: usize,
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
    module_path: &LitStr,
    externals: &[super::ExternalModel],
) -> syn::Result<TokenStream> {
    let custom_ident = &custom.ident;
    let visibility = &custom.visibility;
    let parameters = &custom.parameters;
    let parameter_count = parameters.len();
    let source_type = super::custom_context::source_type(custom);
    let host_type = super::custom_context::host_type(custom, customs, support);
    let parameter_declarations = parameters
        .iter()
        .map(|parameter| quote!(#parameter: #support::ProviderValue + 'static))
        .collect::<Vec<_>>();
    let source_impl_generics =
        (!parameters.is_empty()).then(|| quote!(<#(#parameter_declarations),*>));
    let source_generics = (!parameters.is_empty()).then(|| quote!(<#(#parameters),*>));
    let phantom_field = (!parameters.is_empty()).then(|| {
        let shape = format_ident!("__Geam{}Field", custom.ident);
        quote! {
            #[doc(hidden)]
            #visibility trait #shape<#(#parameters),*> { type Value; }
            impl<#(#parameters,)* __GeamField> #shape<#(#parameters),*> for __GeamField { type Value = __GeamField; }
        }
    });
    let contextual = super::custom_context::is_contextual(custom_index, customs);
    let forms = quote!(#support::ProviderValueForms);
    let invocation_requirements =
        super::custom_context::invocation_requirements(custom_index, customs, support);
    let input_requirements = super::custom_context::requirements(custom_index, customs, support);
    let context_bounds = input_requirements.bounds;
    let input_requirements = input_requirements.requirements;
    let context_where =
        contextual.then(|| quote!(where __GeamProfile: __GeamModuleProfile, #(#context_bounds,)*));
    let output_fields = super::custom_output::fields(custom, customs, support);
    let output_parameters = output_fields
        .iter()
        .map(|field| &field.parameter)
        .collect::<Vec<_>>();
    let output_generics = (!parameters.is_empty() || !output_parameters.is_empty())
        .then(|| quote!(<#(#parameters,)* #(#output_parameters,)*>));
    let output_impl_generics = (!parameters.is_empty() || !output_parameters.is_empty())
        .then(|| quote!(<#(#parameter_declarations,)* #(#output_parameters,)*>));
    let output_type = quote!(#custom_ident #output_generics);
    let forms_generics = &output_impl_generics;
    let runtime_family = format_ident!("__Geam{}RuntimeForms", custom.ident);
    let runtime_type = if contextual {
        quote!(#runtime_family<__GeamProfile, #(#parameters,)*>)
    } else {
        quote!(#support::ProviderStaticValueForms<Self>)
    };
    let source_output = if output_fields.is_empty() {
        quote!(#source_type)
    } else {
        let fields = output_fields.iter().map(|field| &field.value_type);
        quote!(#custom_ident<#(#parameters,)* #(#fields,)*>)
    };
    let owned_output = if contextual {
        super::custom_context::output_type(custom_index, customs, support, &quote!(__GeamProfile))
    } else if output_fields.is_empty() {
        quote!(#source_type)
    } else {
        let fields = output_fields.iter().map(|field| &field.value_type);
        quote!(#custom_ident<#(#parameters,)* #(#fields,)*>)
    };
    let source_list_decoder = |flavor| {
        custom.input.as_ref().filter(|_| !contextual).map_or_else(
            || quote!(#support::MissingListContext),
            |input| {
                super::list::default_decoder_type_for(
                    &input.list_decoder,
                    &StaticValueType::Custom {
                        index: custom_index,
                    },
                    customs,
                    flavor,
                )
            },
        )
    };
    let immediate_list_decoder = source_list_decoder(InputOwnership::Borrowed);
    let owned_list_decoder = source_list_decoder(InputOwnership::Owned);
    let source_list_marker = |flavor, decoder: &TokenStream| {
        custom.input.as_ref().map_or_else(
            || quote!(#support::NoCustomInput),
            |input| {
                let marker = super::custom_context::list_marker_ident(&input.ident, flavor);
                super::custom_context::list_marker_type(custom, &marker, &quote!(#decoder))
            },
        )
    };
    let source_immediate_list =
        source_list_marker(InputOwnership::Borrowed, &immediate_list_decoder);
    let source_owned_list = source_list_marker(InputOwnership::Owned, &owned_list_decoder);
    let schema = &custom.schema;
    let schema_type = super::custom_context::schema_type(custom, customs);
    let schema_generics = contextual.then(|| quote!(<__GeamProfile>));
    let schema_field =
        contextual.then(|| quote!((::core::marker::PhantomData<fn() -> __GeamProfile>)));
    let work_bound = super::list_capability::capabilities(
        &StaticValueType::Custom {
            index: custom_index,
        },
        customs,
    )
    .iter()
    .any(|callback| callback.requires_work(customs))
    .then(|| quote!(__GeamProfile: #support::HostWorkProfile,));
    let schema_where =
        contextual.then(|| quote!(where __GeamProfile: __GeamModuleProfile, #work_bound));
    let marker_generics = if contextual {
        Some(quote!(<__GeamProfile, #(#parameters,)*>))
    } else {
        source_generics.clone()
    };
    let value_generics = if contextual {
        Some(quote!(<__GeamProfile, #(#parameter_declarations,)* #(#output_parameters,)*>))
    } else {
        output_impl_generics.clone()
    };
    let value_trait = if contextual {
        quote!(#support::ProviderTypedValue<__GeamProfile>)
    } else {
        quote!(#support::ProviderValue)
    };
    let source_value_generics = if contextual {
        Some(quote!(<__GeamProfile, #(#parameter_declarations,)*>))
    } else {
        source_impl_generics.clone()
    };
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
            let type_ = super::custom_schema::field_type(
                host_custom_field_type(&field.value, customs, support),
                parameters,
                support,
                customs,
                externals,
            )?;
            field_definitions.push(quote! {
                #[doc(hidden)]
                pub struct #definition #schema_generics #schema_field;

                impl #schema_generics #support::HostCustomField for #definition #schema_generics #schema_where {
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
            fields.push(quote!(#definition #schema_generics));
        }
        let fields = host_custom_field_sequence(&fields, support);
        let index = host_custom_index(index, support);
        constructor_definitions.push(quote! {
            #[doc(hidden)]
            pub struct #definition #schema_generics #schema_field;

            impl #schema_generics #support::HostCustomConstructorDefinition for #definition #schema_generics #schema_where {
                const NAME: &'static str = #name;
                type Fields = #fields;
            }

            #[doc(hidden)]
            pub type #marker #marker_generics = #support::HostCustomConstructorAt<
                #host_type,
                #index,
                #definition #schema_generics,
            >;
        });
    }
    let mut constructors = Vec::with_capacity(custom.constructors.len());
    for constructor in &custom.constructors {
        let definition = &constructor.definition;
        constructors.push(quote!(#definition #schema_generics));
    }
    let constructors = host_custom_constructor_sequence(&constructors, support);

    let mut root_names = GeneratedNames::default();
    let root = generate_custom_return(
        custom_index,
        customs,
        support,
        &quote!(__GeamProviderBinding),
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
    let nested_provider = quote!(__GeamProviderBinding);
    let nested_return_type = quote!(__GeamReturn);
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
    let nested_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(__GeamProviderBinding),
        &quote!(__GeamReturn),
    );
    let root_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(__GeamProviderBinding),
        &quote!(Self::Host),
    );
    let input_declaration = if let Some(input_model) = &custom.input {
        let input = &input_model.ident;
        let definition =
            generate_custom_input_definition(custom_index, custom, input, customs, support);
        let input_declaration = {
            let immediate_ident = super::list::custom_input_ident(input, InputOwnership::Borrowed);
            let async_ident = super::list::custom_input_ident(input, InputOwnership::Owned);
            let immediate_input = super::custom_context::named_type(&immediate_ident, parameters);
            let async_input = super::custom_context::named_type(&async_ident, parameters);
            let input = super::custom_context::named_type(input, parameters);
            let immediate_declaration = generate_custom_input_declaration(
                custom_index,
                custom,
                input_model,
                customs,
                support,
                InputOwnership::Borrowed,
            );
            let async_declaration = generate_custom_input_declaration(
                custom_index,
                custom,
                input_model,
                customs,
                support,
                InputOwnership::Owned,
            );
            let marker_lists = contextual.then(|| {
                quote! {
                    impl #source_impl_generics #support::ProviderMarkerListForms for #input {
                        type Immediate = #support::List<#immediate_input>;
                        type Owned = #support::List<#async_input>;
                    }
                }
            });
            quote! {
                impl #source_impl_generics #forms for #input {
                    type InvocationRequirements = #invocation_requirements;
                    type ImmediateListDecoder = #immediate_list_decoder;
                    type OwnedListDecoder = #owned_list_decoder;
                    type Runtime<__GeamProfile: #support::HostProfile> = #runtime_type;
                    type Output = #source_output;
                    type ImmediateInput = #immediate_input;
                    type ImmediateListInput = #source_immediate_list;
                    type OwnedInput = #async_input;
                    type OwnedListInput = #source_owned_list;
                }

                #marker_lists

                #immediate_declaration
                #async_declaration
            }
        };
        let input = super::custom_context::named_type(input, parameters);
        quote! {
            #definition

            impl #source_value_generics #value_trait for #input #context_where {
                type Host = #host_type;
                type OutputRequirements = #support::ProviderNoConstructions;
                type RootRequirements = #support::ProviderNoConstructions;
            }

            impl<__GeamProfile, #(#parameter_declarations,)*> #support::ProviderCustomInputDeclaration<__GeamProfile> for #input
            where __GeamProfile: __GeamModuleProfile, #(#context_bounds,)* {
                type Schema = #schema_type;
                type Output = #source_type;
            }

            #input_declaration
        }
    } else {
        TokenStream::new()
    };
    let input = if let Some(input) = &custom.input {
        let input = &input.ident;
        super::custom_context::named_type(input, parameters)
    } else {
        quote!(#support::NoCustomInput)
    };
    let immediate_input = if let Some(input) = &custom.input {
        super::custom_context::defined_input_type(
            custom_index,
            &input.ident,
            customs,
            support,
            InputOwnership::Borrowed,
            &quote!(__GeamProfile),
        )
    } else {
        quote!(#support::NoCustomInput)
    };
    let owned_input = if let Some(input) = &custom.input {
        super::custom_context::defined_input_type(
            custom_index,
            &input.ident,
            customs,
            support,
            InputOwnership::Owned,
            &quote!(__GeamProfile),
        )
    } else {
        quote!(#support::NoCustomInput)
    };
    let contextual_list_marker = |flavor| {
        custom.input.as_ref().map_or_else(
            || quote!(#support::NoCustomInput),
            |input| {
                let marker = super::custom_context::list_marker_ident(&input.ident, flavor);
                let decoder = super::list_capability::decoder_type_for(
                    &input.list_decoder,
                    &StaticValueType::Custom {
                        index: custom_index,
                    },
                    customs,
                    support,
                    flavor,
                    &quote!(__GeamProfile),
                );
                super::custom_context::list_marker_type(custom, &marker, &quote!(#decoder))
            },
        )
    };
    let contextual_immediate_list = contextual_list_marker(InputOwnership::Borrowed);
    let contextual_owned_list = contextual_list_marker(InputOwnership::Owned);
    let runtime_declaration = contextual.then(|| {
        quote! {
            #[doc(hidden)]
            #visibility struct #runtime_family<__GeamProfile, #(#parameter_declarations,)*>(::core::marker::PhantomData<fn() -> (__GeamProfile, #(#parameters,)*)>);

            impl<__GeamProfile, #(#parameter_declarations,)*> #support::ProviderRuntimeValueForms<__GeamProfile> for #runtime_family<__GeamProfile, #(#parameters,)*>
            #context_where
            {
                type Host = #host_type;
                type Output = #owned_output;
                type OutputRequirements = #nested_requirements;
                type RootRequirements = #root_requirements;
                type ImmediateInput = #immediate_input;
                type ImmediateListInput = #contextual_immediate_list;
                type OwnedInput = #owned_input;
                type OwnedListInput = #contextual_owned_list;
                type InputRequirements = #input_requirements;
            }
        }
    });
    let marker_immediate_input = custom
        .input
        .as_ref()
        .map(|input| super::list::custom_input_ident(&input.ident, InputOwnership::Borrowed))
        .map_or_else(
            || quote!(#support::NoCustomInput),
            |input| super::custom_context::named_type(&input, parameters),
        );
    let marker_owned_input = custom
        .input
        .as_ref()
        .map(|input| super::list::custom_input_ident(&input.ident, InputOwnership::Owned))
        .map_or_else(
            || quote!(#support::NoCustomInput),
            |input| super::custom_context::named_type(&input, parameters),
        );
    let value_declaration = {
        let marker_lists = contextual.then(|| {
            quote! {
                impl #forms_generics #support::ProviderMarkerListForms for #output_type {
                    type Immediate = #support::List<#marker_immediate_input>;
                    type Owned = #support::List<#marker_owned_input>;
                }
            }
        });
        quote! {
            impl #forms_generics #forms for #output_type {
                type InvocationRequirements = #invocation_requirements;
                type ImmediateListDecoder = #immediate_list_decoder;
                type OwnedListDecoder = #owned_list_decoder;
                type Runtime<__GeamProfile: #support::HostProfile> = #runtime_type;
                type Output = #source_output;
                type ImmediateInput = #marker_immediate_input;
                type ImmediateListInput = #source_immediate_list;
                type OwnedInput = #marker_owned_input;
                type OwnedListInput = #source_owned_list;
            }

            #marker_lists

            #runtime_declaration

            impl<__GeamProfile, __GeamProviderBinding, __GeamReturn, #(#parameter_declarations,)*>
                #support::ProviderOutputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn>
                for #owned_output
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                __GeamReturn: #support::HostType,
                #(#context_bounds,)*
                #(#nested_codec_bounds,)*
                #(#nested_requirement_bounds,)*
            {
                type Error = #support::HostCallError;

                fn into_host<'__geam_call>(
                    self,
                    mut call: &mut #support::HostCall<
                        '__geam_call,
                        __GeamProfile,
                        __GeamProviderBinding,
                        __GeamReturn,
                    >,
                    constructions: &#support::ProviderConstructions<
                        '__geam_call,
                        Self::OutputRequirements,
                    >,
                ) -> ::core::result::Result<<Self::Host as #support::HostType>::Value<'__geam_call>, Self::Error> {
                    #nested_bindings
                    #nested_statements
                    ::core::result::Result::Ok(#nested_value)
                }
            }

            impl<__GeamProfile, __GeamProviderBinding, #(#parameter_declarations,)*>
                #support::ProviderRootOutputValue<__GeamProfile, __GeamProviderBinding>
                for #owned_output
            where
                __GeamProfile: __GeamModuleProfile,
                __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
                #(#context_bounds,)*
                #(#root_codec_bounds,)*
                #(#root_requirement_bounds,)*
            {
                fn complete<'__geam_call>(
                    self,
                    mut call: #support::HostCall<
                        '__geam_call,
                        __GeamProfile,
                        __GeamProviderBinding,
                        Self::Host,
                    >,
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
        }
    };

    let (value_nested_requirements, value_root_requirements) = if contextual {
        (nested_requirements.clone(), root_requirements.clone())
    } else {
        (
            super::custom_schema::static_requirements(&nested_requirements),
            super::custom_schema::static_requirements(&root_requirements),
        )
    };
    Ok(quote! {
        #phantom_field
        #(#field_definitions)*
        #(#constructor_definitions)*

        #[doc(hidden)]
        pub struct #schema #schema_generics #schema_field;

        impl #schema_generics #support::HostCustomSchema for #schema #schema_generics #schema_where {
            const PACKAGE: &'static str =
                <super::Component as #support::ProviderPackage>::PACKAGE;
            const MODULE: &'static str = #module_path;
            const NAME: &'static str = #source_name;
            const PARAMETER_COUNT: usize = #parameter_count;
            type Constructors = #constructors;
        }

        impl<__GeamProfile, #(#parameter_declarations,)* #(#output_parameters,)*> #support::ProviderCustomDeclaration<__GeamProfile> for #output_type
        where __GeamProfile: __GeamModuleProfile, #(#context_bounds,)* {
            type Schema = #schema_type;
            type Input = #input;
        }

        impl #value_generics #value_trait for #output_type #context_where {
            type Host = #host_type;
            type OutputRequirements = #value_nested_requirements;
            type RootRequirements = #value_root_requirements;
        }

        #value_declaration

        #input_declaration
    })
}

fn generate_custom_input_definition(
    custom_index: usize,
    custom: &CustomModel,
    input: &Ident,
    customs: &[CustomModel],
    support: &TokenStream,
) -> TokenStream {
    let visibility = &custom.visibility;
    let source_parameters = &custom.parameters;
    let source_bounds = source_parameters
        .iter()
        .map(|parameter| quote!(#parameter: #support::ProviderValue + 'static))
        .collect::<Vec<_>>();
    let fields = custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .collect::<Vec<_>>();
    let has_context = !fields.is_empty();
    let context_trait = format_ident!("__Geam{}Shape", input);
    let immediate = format_ident!("__GeamImmediate{}Shape", input);
    let owned = format_ident!("__GeamOwned{}Shape", input);
    let context_type = super::custom_context::named_type(&context_trait, source_parameters);
    let immediate_type = super::custom_context::named_type(&immediate, source_parameters);
    let names = (0..fields.len())
        .map(|index| format_ident!("Field{index}"))
        .collect::<Vec<_>>();
    let mut index = 0;
    let variants = custom
        .constructors
        .iter()
        .map(|constructor| {
            let ident = &constructor.ident;
            let members = custom_field_models(&constructor.fields)
                .iter()
                .map(|field| {
                    let name = &names[index];
                    let type_ = if source_parameters.is_empty() {
                        quote!(__GeamContext::#name)
                    } else {
                        quote!(<__GeamContext as #context_type>::#name)
                    };
                    index += 1;
                    if field.named {
                        let ident = &field.ident;
                        quote!(#ident: #type_)
                    } else {
                        type_
                    }
                })
                .collect::<Vec<_>>();
            match &constructor.fields {
                CustomFields::Unit => quote!(#ident),
                CustomFields::Unnamed(_) => quote!(#ident(#(#members),*)),
                CustomFields::Named(_) => quote!(#ident { #(#members),* }),
            }
        })
        .collect::<Vec<_>>();
    let parameters = if has_context {
        Some(quote!(<#(#source_bounds,)* __GeamContext: #context_type = #immediate_type>))
    } else {
        (!source_parameters.is_empty()).then(|| quote!(<#(#source_bounds),*>))
    };
    let phantom = super::custom_output::phantom_variant(custom);
    let contexts = [
        (&immediate, InputOwnership::Borrowed),
        (&owned, InputOwnership::Owned),
    ]
    .into_iter()
    .map(|(context, flavor)| {
        let alias = custom_input_ident(input, flavor);
        if !has_context {
            let source_generics = (!source_parameters.is_empty()).then(|| quote!(<#(#source_bounds),*>));
            let source_type = super::custom_context::named_type(input, source_parameters);
            return quote! {
                #[doc(hidden)]
                #visibility type #alias #source_generics = #source_type;
            };
        }
        let contextual = super::custom_context::is_contextual(custom_index, customs);
        let profile_default = contextual.then(|| quote!(__GeamInputProfile = (),));
        let profile_parameter = contextual.then(|| quote!(__GeamInputProfile,));
        let types = super::custom_context::marker_fields(custom, customs, support, flavor);
        quote! {
            #[doc(hidden)]
            #visibility struct #context<#(#source_bounds,)* #(#names = #types,)* #profile_default>(
                ::core::marker::PhantomData<fn() -> (#(#source_parameters,)* #(#names,)* #profile_parameter)>,
            );

            impl<#(#source_bounds,)* #(#names,)* #profile_parameter> #context_type for #context<#(#source_parameters,)* #(#names,)* #profile_parameter> {
                #(type #names = #names;)*
            }

            #[doc(hidden)]
            #visibility type #alias<#(#source_bounds,)* #(#names = #types,)* #profile_default> = #input<#(#source_parameters,)* #context<#(#source_parameters,)* #(#names,)* #profile_parameter>>;
        }
    });
    let context_definition = has_context.then(|| {
        let source_generics =
            (!source_parameters.is_empty()).then(|| quote!(<#(#source_bounds),*>));
        quote! {
            #[doc(hidden)]
            #visibility trait #context_trait #source_generics { #(type #names;)* }
        }
    });
    quote! {
        #visibility enum #input #parameters { #(#variants,)* #phantom }
        #context_definition
        #(#contexts)*
    }
}

fn generate_custom_input_declaration(
    custom_index: usize,
    custom: &CustomModel,
    input_model: &CustomInputModel,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> TokenStream {
    let input = super::custom_context::defined_input_type(
        custom_index,
        &input_model.ident,
        customs,
        support,
        flavor,
        &quote!(__GeamProfile),
    );
    let contextual = super::custom_context::is_contextual(custom_index, customs);
    let generated = super::custom_context::requirements(custom_index, customs, support);
    let requirements = generated.requirements;
    let function_flavor = flavor;
    let parameters = &custom.parameters;
    let parameter_declarations = parameters
        .iter()
        .map(|parameter| quote!(#parameter: #support::ProviderValue + 'static))
        .collect::<Vec<_>>();
    let host_type = super::custom_context::host_type(custom, customs, support);
    let mut input_codec_bounds =
        custom_input_codec_bounds(custom, customs, support, function_flavor);
    input_codec_bounds.extend(generated.bounds);
    let decoder_definition = generate_custom_decoder(
        custom_index,
        custom,
        input_model,
        customs,
        support,
        &input_codec_bounds,
        function_flavor,
    );
    let decoder = match flavor {
        InputOwnership::Borrowed => {
            format_ident!("__GeamImmediate{}", input_model.decoder)
        }
        InputOwnership::Owned => format_ident!("__GeamOwned{}", input_model.decoder),
    };
    let list_value = StaticValueType::Custom {
        index: custom_index,
    };
    let contextual_decoder = super::list_capability::decoder_type_for(
        &input_model.list_decoder,
        &list_value,
        customs,
        support,
        flavor,
        &quote!(__GeamProfile),
    );
    let list_decoder = if contextual {
        contextual_decoder
    } else {
        let decoder = list_decoder_ident(&input_model.list_decoder, flavor);
        super::custom_context::named_type(&decoder, parameters)
    };
    let (list_constructions, list_bindings, list_context_bounds) =
        super::custom_context::constructions(custom_index, customs, support);
    let list_setup =
        provider_construction_bindings(&list_constructions, quote!(_constructions), support);
    let list_return = quote!(__GeamReturn);
    let function_generics = super::custom_context::generics(custom);
    let list_environment = super::InputEnvironment {
        customs,
        support,
        return_type: &list_return,
        function_generics: &function_generics,
        generic_source: super::GenericInputSource::Declared,
        flavor,
        callback_constructions: &list_bindings,
    };
    let list_decoder_value = list_decoder_value(
        &input_model.list_decoder,
        &StaticValueType::Custom {
            index: custom_index,
        },
        customs,
        support,
        flavor,
        super::list::ListDecoderContext {
            provider: &quote!(__GeamProviderBinding),
            call: &quote!(&*call),
            capabilities: super::list_capability::decoder_fields(&list_value, &list_environment),
            constructions: &list_bindings,
        },
    );
    let list_codec_bounds =
        custom_list_codec_bounds(custom_index, customs, support, function_flavor);
    let marker = super::custom_context::list_marker_ident(&input_model.ident, flavor);
    let visibility = &custom.visibility;
    let list_declaration = quote! {
        #[doc(hidden)]
        #visibility struct #marker<#(#parameters,)* __GeamDecoder>(::core::marker::PhantomData<fn() -> (#(#parameters,)* __GeamDecoder)>);

        impl<#(#parameter_declarations,)* __GeamDecoder> #support::ProviderListInputValue for #marker<#(#parameters,)* __GeamDecoder>
        where __GeamDecoder: #support::ProviderTypedListItemDecoder<Self>
            + ::core::clone::Clone + ::core::marker::Send + 'static,
        {
            type Host = <__GeamDecoder as #support::ProviderTypedListItemDecoder<Self>>::Host;
            type View = <__GeamDecoder as #support::ProviderListItemDecoder<Self>>::View;
            type Decoder = __GeamDecoder;
        }

        impl<__GeamProfile, __GeamProviderBinding, #(#parameter_declarations,)*> #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding> for #marker<#(#parameters,)* #list_decoder>
        where __GeamProfile: __GeamModuleProfile, __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
            #(#list_context_bounds,)* #(#list_codec_bounds,)*
        {
            type Requirements = #requirements;
            fn decoder_with<'__geam_call, __GeamReturn>(
                call: &#support::HostCall<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn>,
                _constructions: &#support::ProviderConstructions<'__geam_call, Self::Requirements>,
            ) -> Self::Decoder where __GeamReturn: #support::HostType {
                #list_setup
                #list_decoder_value
            }
        }
    };
    let direct = (matches!(flavor, InputOwnership::Borrowed) || custom.constructors.iter().any(|constructor| !custom_field_models(&constructor.fields).is_empty())).then(|| quote! {
        impl<__GeamProfile, __GeamProviderBinding, __GeamReturn, #(#parameter_declarations,)*>
            #support::ProviderInputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn> for #input
        where
            __GeamProfile: __GeamModuleProfile,
            __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
            __GeamReturn: #support::HostType,
            #(#input_codec_bounds,)*
        {
            type Host = #host_type;

            type Requirements = #requirements;

            fn from_host_with<'__geam_call>(
                call: &mut #support::HostCall<
                    '__geam_call,
                    __GeamProfile,
                    __GeamProviderBinding,
                    __GeamReturn,
                >,
                value: <Self::Host as #support::HostType>::Value<'__geam_call>,
                constructions: &#support::ProviderConstructions<'__geam_call, Self::Requirements>,
            ) -> Self {
                #decoder::<__GeamProfile, __GeamProviderBinding, __GeamReturn, #(#parameters,)*>(call, value, constructions)
            }
        }

        #decoder_definition
    });
    quote! { #direct #list_declaration }
}

fn generate_custom_decoder(
    custom_index: usize,
    custom: &CustomModel,
    input_model: &CustomInputModel,
    customs: &[CustomModel],
    support: &TokenStream,
    codec_bounds: &[TokenStream],
    flavor: InputOwnership,
) -> TokenStream {
    let decoder = match flavor {
        InputOwnership::Borrowed => {
            format_ident!("__GeamImmediate{}", input_model.decoder)
        }
        InputOwnership::Owned => format_ident!("__GeamOwned{}", input_model.decoder),
    };
    let parameters = &custom.parameters;
    let parameter_declarations = parameters
        .iter()
        .map(|parameter| quote!(#parameter: #support::ProviderValue + 'static))
        .collect::<Vec<_>>();
    let host_type = super::custom_context::host_type(custom, customs, support);
    let input = super::custom_context::defined_input_type(
        custom_index,
        &input_model.ident,
        customs,
        support,
        flavor,
        &quote!(__GeamProfile),
    );
    let input_constructor = custom_input_ident(&input_model.ident, flavor);
    let (constructions, callback_constructions, _) =
        super::custom_context::constructions(custom_index, customs, support);
    let requirements = provider_requirement_sequence(&constructions, support);
    let bindings = provider_construction_bindings(&constructions, quote!(constructions), support);
    let function_generics = super::custom_context::generics(custom);
    let environment = super::InputEnvironment {
        customs,
        support,
        return_type: &quote!(__GeamReturn),
        function_generics: &function_generics,
        generic_source: super::GenericInputSource::Declared,
        flavor,
        callback_constructions: &callback_constructions,
    };
    let mut names = GeneratedNames::default();
    let mut branches = Vec::with_capacity(custom.constructors.len());
    let mut remaining = TokenStream::new();
    for (constructor_index, constructor) in custom.constructors.iter().enumerate() {
        let marker = super::custom_context::constructor_type(custom, &constructor.marker, customs);
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
                &environment,
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
        let expression = custom_input_expression(&input_constructor, constructor, &value_names);
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
    let call = { quote!(#support::HostCall) };
    let profile = { quote!(__GeamModuleProfile) };
    quote! {
        fn #decoder<'__geam_call, __GeamProfile, __GeamProviderBinding, __GeamReturn, #(#parameter_declarations,)*>(
            call: &mut #call<
                '__geam_call,
                __GeamProfile,
                __GeamProviderBinding,
                __GeamReturn,
            >,
            value: #support::HostCustom<
                '__geam_call,
                #host_type,
            >,
            constructions: &#support::ProviderConstructions<'__geam_call, #requirements>,
        ) -> #input
        where
            __GeamProfile: #profile,
            __GeamProviderBinding: #support::HostProvider<__GeamProfile>,
            __GeamReturn: #support::HostType,
            #(#codec_bounds,)*
        {
            #bindings
            #(#branches)*
            #remaining
        }
    }
}

fn decode_custom_field_value(
    type_: &CustomFieldValueType,
    input: TokenStream,
    environment: &super::InputEnvironment<'_>,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let customs = environment.customs;
    let support = environment.support;
    let flavor = environment.flavor;
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_custom_input_value(type_, input, environment, names)
        }
        CustomFieldValueType::List(list) => {
            let decoder = match flavor {
                InputOwnership::Borrowed => list_decoder_value(
                    &list.decoder,
                    &list.collection.value,
                    customs,
                    support,
                    InputOwnership::Borrowed,
                    super::list::ListDecoderContext {
                        provider: &quote!(__GeamProviderBinding),
                        call: &quote!(&*call),
                        constructions: environment.callback_constructions,
                        capabilities: super::list_capability::decoder_fields(
                            &list.collection.value,
                            environment,
                        ),
                    },
                ),
                InputOwnership::Owned => list_decoder_value(
                    &list.decoder,
                    &list.collection.value,
                    customs,
                    support,
                    InputOwnership::Owned,
                    super::list::ListDecoderContext {
                        provider: &quote!(__GeamProviderBinding),
                        call: &quote!(&*call),
                        constructions: environment.callback_constructions,
                        capabilities: super::list_capability::decoder_fields(
                            &list.collection.value,
                            environment,
                        ),
                    },
                ),
            };

            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(call.provider_retained_list(#input, #decoder)),
            }
        }
    }
}

fn decode_custom_input_value(
    type_: &StaticValueType,
    input: TokenStream,
    environment: &super::InputEnvironment<'_>,
    names: &mut GeneratedNames,
) -> GeneratedValue {
    let customs = environment.customs;
    let support = environment.support;
    let flavor = environment.flavor;
    let callback_constructions = environment.callback_constructions;
    match type_ {
        StaticValueType::List(list) => decode_custom_field_value(
            &CustomFieldValueType::List(list.clone()),
            input,
            environment,
            names,
        ),
        StaticValueType::Future(future) => super::function::decode_input(
            &super::FunctionInputType::Future(future.clone()),
            input,
            environment,
            names,
        ),
        StaticValueType::Callback(callback) => super::function::decode_input(
            &super::FunctionInputType::Callback(callback.clone()),
            input,
            environment,
            names,
        ),
        StaticValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        StaticValueType::Declared { type_, .. } => {
            let member = match flavor {
                InputOwnership::Borrowed => quote!(ImmediateInput),
                InputOwnership::Owned => quote!(OwnedInput),
            };
            let key = quote!(#type_).to_string();
            let proof = super::custom_context::construction_proof(
                &key,
                callback_constructions,
                customs,
                support,
            );
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(<<#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::#member as #support::ProviderInputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn>>::from_host_with(call, #input, &#proof)),
            }
        }
        StaticValueType::External {
            payload: _, schema, ..
        } => {
            let value = match flavor {
                InputOwnership::Borrowed => quote!(
                    call.provider_external_view_with::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(#input)
                ),
                InputOwnership::Owned => quote!(
                    call.provider_external_item_with::<
                        __GeamProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(#input)
                ),
            };
            GeneratedValue {
                statements: TokenStream::new(),
                value,
            }
        }
        StaticValueType::Custom { index, .. } => {
            let input_type = super::custom_context::input_type(
                *index,
                customs,
                support,
                flavor,
                &quote!(__GeamProfile),
            );
            let key = customs[*index].schema.to_string();
            let proof = callback_constructions.get(&key).map_or_else(|| quote!(#support::ProviderConstructions::<#support::ProviderNoConstructions>::none()), |binding| quote!(#binding));
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(
                    <#input_type as #support::ProviderInputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn>>::from_host_with(call, #input, &#proof)
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
                let decoded = decode_custom_input_value(element, quote!(#host), environment, names);
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
            let decoded_success =
                decode_custom_input_value(success, quote!(#success_value), environment, names);
            let decoded_failure =
                decode_custom_input_value(failure, quote!(#failure_value), environment, names);
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
            let decoded = decode_custom_input_value(value, quote!(#some_host), environment, names);
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
    let output_trait = quote!(#support::ProviderOutputValue);
    match type_ {
        StaticValueType::List(list) => collect_custom_output_codec_bounds(
            &list.collection.value,
            customs,
            support,
            provider,
            return_type,
            bounds,
        ),
        StaticValueType::Declared { type_, .. } => {
            bounds.push(quote! {
                <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Output: #output_trait<
                    __GeamProfile, #provider, #return_type,
                    Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host,
                    OutputRequirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::OutputRequirements,
                >
            });
        }
        StaticValueType::Custom { index, .. } => {
            let type_ = super::custom_context::output_type(
                *index,
                customs,
                support,
                &quote!(__GeamProfile),
            );
            let declaration = &customs[*index].ident;
            let schema = super::custom_context::schema_type(&customs[*index], customs);
            bounds.push(quote!(#type_: #output_trait<
                __GeamProfile, #provider, #return_type,
                Host = #support::HostCustomType<#schema>,
                OutputRequirements = <#declaration as #support::ProviderContextualValueForms<__GeamProfile>>::OutputRequirements,
            >));
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
        StaticValueType::Future(_) => bounds.push(quote!(__GeamProfile: #support::HostWorkProfile)),
        StaticValueType::Callback(_)
        | StaticValueType::Scalar(_)
        | StaticValueType::External { .. } => {}
    }
}

fn custom_input_codec_bounds(
    custom: &CustomModel,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            match &field.value {
                CustomFieldValueType::Value(value) => {
                    collect_custom_input_codec_bounds(value, customs, support, &mut bounds, flavor);
                }
                CustomFieldValueType::List(list) => {
                    for access in list_declared_accesses(&list.collection.value, customs) {
                        let type_ = access.type_;
                        bounds.push(match flavor {
                            InputOwnership::Borrowed => quote! {
                                <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::ImmediateListInput:
                                    #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding, Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host, Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements>
                            },
                            InputOwnership::Owned => quote! {
                                <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::OwnedListInput:
                                    #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding, Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host, Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements>
                            },
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
    customs: &[CustomModel],
    support: &TokenStream,
    bounds: &mut Vec<TokenStream>,
    flavor: InputOwnership,
) {
    match type_ {
        StaticValueType::List(list) => collect_custom_input_codec_bounds(
            &list.collection.value,
            customs,
            support,
            bounds,
            flavor,
        ),
        StaticValueType::Declared { type_, .. } => {
            bounds.push(match flavor {
                InputOwnership::Borrowed => quote! {
                    <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::ImmediateInput:
                        #support::ProviderInputValue<
                            __GeamProfile,
                            __GeamProviderBinding,
                            __GeamReturn,
                            Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host,
                        Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements,
                        >
                },
                InputOwnership::Owned => quote! {
                    <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::OwnedInput:
                        #support::ProviderInputValue<
                            __GeamProfile,
                            __GeamProviderBinding,
                            __GeamReturn,
                            Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host,
                        Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements,
                        >
                },
            });
        }
        StaticValueType::Custom { index, .. } => {
            let input = super::custom_context::input_type(
                *index,
                customs,
                support,
                flavor,
                &quote!(__GeamProfile),
            );
            let schema = super::custom_context::schema_type(&customs[*index], customs);
            let requirements =
                super::custom_context::requirements(*index, customs, support).requirements;
            bounds.push(quote!(#input: #support::ProviderInputValue<__GeamProfile, __GeamProviderBinding, __GeamReturn, Host = #support::HostCustomType<#schema>, Requirements = #requirements>));
        }
        StaticValueType::Tuple(elements) => {
            for element in elements {
                collect_custom_input_codec_bounds(element, customs, support, bounds, flavor);
            }
        }
        StaticValueType::Result { success, failure } => {
            collect_custom_input_codec_bounds(success, customs, support, bounds, flavor);
            collect_custom_input_codec_bounds(failure, customs, support, bounds, flavor);
        }
        StaticValueType::Option { value } => {
            collect_custom_input_codec_bounds(value, customs, support, bounds, flavor);
        }
        StaticValueType::Future(_) => bounds.push(quote!(__GeamProfile: #support::HostWorkProfile)),
        StaticValueType::Callback(_)
        | StaticValueType::Scalar(_)
        | StaticValueType::External { .. } => {}
    }
}

fn custom_list_codec_bounds(
    custom_index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: InputOwnership,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for access in list_declared_accesses(
        &StaticValueType::Custom {
            index: custom_index,
        },
        customs,
    ) {
        let type_ = access.type_;
        bounds.push(match flavor {
            InputOwnership::Borrowed => quote! {
                <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::ImmediateListInput:
                    #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding, Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host, Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements>
            },
            InputOwnership::Owned => quote! {
                <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::OwnedListInput:
                    #support::ProviderListInputCodec<__GeamProfile, __GeamProviderBinding, Host = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host, Requirements = <#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements>
            },
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

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn custom_field_schemas_reject_non_type_arguments_in_qualified_nominals() {
        for (field, expected) in [
            (
                quote!(remote::Record<42>),
                "generic declared arguments must be source types",
            ),
            (
                quote!(remote::Record<'static>),
                "generic declared arguments must be source types",
            ),
            (
                quote!(remote::Record<Local<42>>),
                "nominal source arguments must be types",
            ),
            (
                quote!(remote::Record<Local<'static>>),
                "nominal source arguments must be types",
            ),
        ] {
            let result = crate::module::expand(
                quote!(path = "native", crate_path = geam_core),
                quote! {
                    mod native {
                        #[geam::custom(input = EnvelopeInput)]
                        pub enum Envelope { Envelope(#field) }
                    }
                },
            );
            assert_eq!(result.unwrap_err().to_string(), expected);
        }
    }
}
