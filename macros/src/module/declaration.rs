use super::custom_value::{
    CustomFieldValueType, CustomFields, CustomInputModel, CustomModel, custom_field_models,
};
use super::function::{
    generate_custom_intermediate, generate_custom_return, host_value_sequence,
    provider_construction_bindings, provider_requirement_selection_bounds,
    provider_requirement_sequence,
};
use super::list::{
    custom_input_expression, list_declared_accesses, list_decoder_value,
    transfer_custom_input_ident, transfer_list_decoder_ident, transfer_list_decoder_value,
};
use super::signature::{host_custom_field_type, host_static_value_type};
use super::{
    FunctionFlavor, GeneratedNames, GeneratedValue, OutputEnvironment, OutputState,
    ProviderRepresentation, StaticValueType, TransferFlavor,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use syn::ext::IdentExt;
use syn::{Ident, LitStr};

pub(super) fn generic_external_output_codec(
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

pub(super) fn generate_custom_declaration(
    custom_index: usize,
    custom: &CustomModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    module_path: &LitStr,
    transfer_enabled: bool,
) -> TokenStream {
    let custom_ident = &custom.ident;
    let output_fields = if transfer_enabled {
        super::custom_output::fields(custom, customs, support)
    } else {
        Vec::new()
    };
    let output_parameters = output_fields
        .iter()
        .map(|field| &field.parameter)
        .collect::<Vec<_>>();
    let output_generics =
        (!output_parameters.is_empty()).then(|| quote!(<#(#output_parameters,)*>));
    let output_type = quote!(#custom_ident #output_generics);
    let transfer_output = if output_fields.is_empty() {
        quote!(#custom_ident)
    } else {
        let fields = output_fields.iter().map(|field| &field.transfer);
        quote!(#custom_ident<#(#fields,)*>)
    };
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
        ProviderRepresentation::Local,
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

    let mut transfer_root_names = GeneratedNames::default();
    let transfer_root = generate_custom_return(
        custom_index,
        customs,
        support,
        &quote!(Provider),
        &quote!(Self::Host),
        &mut transfer_root_names,
        ProviderRepresentation::Transfer,
    );
    let transfer_root_requirements =
        provider_requirement_sequence(&transfer_root.constructions, support);
    let mut transfer_root_requirement_bounds = Vec::new();
    if !transfer_root.constructions.is_empty() {
        transfer_root_requirement_bounds.push(quote! {
            #transfer_root_requirements: #support::ProviderConstructionRequirements
        });
        transfer_root_requirement_bounds.extend(provider_requirement_selection_bounds(
            &transfer_root_requirements,
            &transfer_root.constructions,
            support,
        ));
    }
    let transfer_root_bindings = provider_construction_bindings(
        &transfer_root.constructions,
        quote!(&constructions),
        support,
    );
    let transfer_root_statements = transfer_root.statements;
    let transfer_root_completion = transfer_root.completion;

    let mut nested_names = GeneratedNames::default();
    let mut nested_constructions = Vec::new();
    let nested_provider = quote!(Provider);
    let nested_return_type = quote!(Return);
    let nested_environment = OutputEnvironment {
        customs,
        support,
        provider: &nested_provider,
        return_type: &nested_return_type,
        representation: ProviderRepresentation::Local,
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

    let mut transfer_nested_names = GeneratedNames::default();
    let mut transfer_nested_constructions = Vec::new();
    let transfer_nested_provider = quote!(Provider);
    let transfer_nested_return_type = quote!(Return);
    let transfer_nested_environment = OutputEnvironment {
        customs,
        support,
        provider: &transfer_nested_provider,
        return_type: &transfer_nested_return_type,
        representation: ProviderRepresentation::Transfer,
    };
    let mut transfer_nested_state = OutputState {
        names: &mut transfer_nested_names,
        constructions: &mut transfer_nested_constructions,
    };
    let transfer_nested = generate_custom_intermediate(
        custom_index,
        quote!(self),
        &transfer_nested_environment,
        &mut transfer_nested_state,
    );
    let transfer_nested_requirements =
        provider_requirement_sequence(&transfer_nested_constructions, support);
    let mut transfer_nested_requirement_bounds = vec![quote! {
        #transfer_nested_requirements: #support::ProviderConstructionRequirements
    }];
    transfer_nested_requirement_bounds.extend(provider_requirement_selection_bounds(
        &transfer_nested_requirements,
        &transfer_nested_constructions,
        support,
    ));
    let transfer_nested_bindings = provider_construction_bindings(
        &transfer_nested_constructions,
        quote!(&constructions),
        support,
    );
    let transfer_nested_statements = transfer_nested.statements;
    let transfer_nested_value = transfer_nested.value;
    let nested_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(Provider),
        &quote!(Return),
        ProviderRepresentation::Local,
    );
    let root_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(Provider),
        &quote!(Self::Host),
        ProviderRepresentation::Local,
    );
    let transfer_nested_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(Provider),
        &quote!(Return),
        ProviderRepresentation::Transfer,
    );
    let transfer_root_codec_bounds = custom_output_codec_bounds(
        custom,
        customs,
        support,
        &quote!(Provider),
        &quote!(Self::Host),
        ProviderRepresentation::Transfer,
    );
    let input_declaration = if let Some(input_model) = &custom.input {
        let input = &input_model.ident;
        let definition = generate_custom_input_definition(
            custom,
            input,
            custom_inputs,
            support,
            transfer_enabled,
        );
        let input_codec_bounds = custom_input_codec_bounds(
            custom,
            customs,
            custom_inputs,
            support,
            FunctionFlavor::Local,
        );
        let decoder_input_codec_bounds = input_codec_bounds.clone();
        let list_codec_bounds =
            custom_list_codec_bounds(custom_index, customs, support, FunctionFlavor::Local);
        let decoder_definition = generate_custom_decoder(
            custom,
            input_model,
            customs,
            custom_inputs,
            support,
            &decoder_input_codec_bounds,
            FunctionFlavor::Local,
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
        let transfer_input_declaration = transfer_enabled.then(|| {
            let immediate_input = transfer_custom_input_ident(input, TransferFlavor::Immediate);
            let async_input = transfer_custom_input_ident(input, TransferFlavor::Async);
            let immediate_declaration = generate_transfer_custom_input_declaration(
                custom_index,
                custom,
                input_model,
                customs,
                custom_inputs,
                support,
                TransferFlavor::Immediate,
            );
            let async_declaration = custom
                .constructors
                .iter()
                .any(|constructor| !custom_field_models(&constructor.fields).is_empty())
                .then(|| {
                    generate_transfer_custom_input_declaration(
                        custom_index,
                        custom,
                        input_model,
                        customs,
                        custom_inputs,
                        support,
                        TransferFlavor::Async,
                    )
                });
            quote! {
                impl #support::ProviderTransferValue for #input {
                    type Output = #transfer_output;
                    type ImmediateInput = #immediate_input;
                    type ImmediateListInput = #immediate_input;
                    type TransferInput = #async_input;
                    type TransferListInput = #async_input;
                }

                #immediate_declaration
                #async_declaration
            }
        });
        quote! {
            #definition

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
            #transfer_input_declaration
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
    let transfer_immediate_input = if let Some(input) = &custom.input {
        let input = transfer_custom_input_ident(&input.ident, TransferFlavor::Immediate);
        quote!(#input)
    } else {
        quote!(#support::NoCustomInput)
    };
    let transfer_async_input = if let Some(input) = &custom.input {
        let input = transfer_custom_input_ident(&input.ident, TransferFlavor::Async);
        quote!(#input)
    } else {
        quote!(#support::NoCustomInput)
    };
    let transfer_value_declaration = transfer_enabled.then(|| {
        quote! {
            impl #output_generics #support::ProviderTransferValue for #output_type {
                type Output = #transfer_output;
                type ImmediateInput = #transfer_immediate_input;
                type ImmediateListInput = #transfer_immediate_input;
                type TransferInput = #transfer_async_input;
                type TransferListInput = #transfer_async_input;
            }

            impl<Profile, Provider, Return>
                #support::ProviderTransferOutputValue<Profile, Provider, Return>
                for #transfer_output
            where
                Profile: __GeamAsyncModuleProfile,
                Provider: #support::HostProvider<Profile>,
                Return: #support::HostType,
                #(#transfer_nested_codec_bounds,)*
                #(#transfer_nested_requirement_bounds,)*
            {
                fn into_host<'__geam_call>(
                    self,
                    mut call: &mut #support::TransferHostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Return,
                    >,
                    constructions: &#support::ProviderConstructions<
                        '__geam_call,
                        Self::OutputRequirements,
                    >,
                ) -> <Self::Host as #support::HostType>::Value<'__geam_call> {
                    #transfer_nested_bindings
                    #transfer_nested_statements
                    #transfer_nested_value
                }
            }

            impl<Profile, Provider>
                #support::ProviderTransferRootOutputValue<Profile, Provider>
                for #transfer_output
            where
                Profile: __GeamAsyncModuleProfile,
                Provider: #support::HostProvider<Profile>,
                #(#transfer_root_codec_bounds,)*
                #(#transfer_root_requirement_bounds,)*
            {
                fn complete<'__geam_call>(
                    self,
                    mut call: #support::TransferHostCall<
                        '__geam_call,
                        Profile,
                        Provider,
                        Self::Host,
                    >,
                    constructions: &#support::ProviderConstructions<
                        '__geam_call,
                        Self::RootRequirements,
                    >,
                ) -> ::core::result::Result<
                    #support::HostCallCompletion<'__geam_call, Self::Host>,
                    #support::AsyncHostCallError,
                > {
                    #transfer_root_bindings
                    let returned = self;
                    #transfer_root_statements
                    #transfer_root_completion
                }
            }
        }
    });

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

        impl #output_generics #support::ProviderCustomDeclaration for #output_type {
            type Schema = #schema;
            type Input = #input;
        }

        impl #output_generics #support::ProviderValue for #output_type {
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

        #transfer_value_declaration

        #input_declaration
    }
}

fn generate_custom_input_definition(
    custom: &CustomModel,
    input: &Ident,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    transferable: bool,
) -> TokenStream {
    let visibility = &custom.visibility;
    let fields = custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
        .collect::<Vec<_>>();
    let has_context = transferable && !fields.is_empty();
    let context_trait = format_ident!("__Geam{}Shape", input);
    let local = format_ident!("__GeamLocal{}Shape", input);
    let immediate = format_ident!("__GeamImmediate{}Shape", input);
    let owned = format_ident!("__GeamOwned{}Shape", input);
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
                    let type_ = if has_context {
                        let name = &names[index];
                        quote!(__GeamContext::#name)
                    } else {
                        custom_input_type(
                            &field.value,
                            custom_inputs,
                            support,
                            FunctionFlavor::Local,
                        )
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
    let parameters = has_context.then(|| quote!(<__GeamContext: #context_trait = #local>));
    let context_definition = has_context.then(|| {
        let implementations = [
            (&local, FunctionFlavor::Local),
            (&immediate, FunctionFlavor::TransferImmediate),
            (&owned, FunctionFlavor::Async),
        ]
        .into_iter()
        .map(|(context, flavor)| {
            let types = fields
                .iter()
                .map(|field| custom_input_type(&field.value, custom_inputs, support, flavor));
            quote! {
                #[doc(hidden)]
                #visibility struct #context;
                impl #context_trait for #context { #(type #names = #types;)* }
            }
        });
        quote! {
            #[doc(hidden)]
            #visibility trait #context_trait { #(type #names;)* }
            #(#implementations)*
        }
    });
    let transfer_aliases = transferable.then(|| {
        let immediate_input = transfer_custom_input_ident(input, TransferFlavor::Immediate);
        let owned_input = transfer_custom_input_ident(input, TransferFlavor::Async);
        let immediate_type = if has_context {
            quote!(#input<#immediate>)
        } else {
            quote!(#input)
        };
        let owned_type = if has_context {
            quote!(#input<#owned>)
        } else {
            quote!(#input)
        };
        quote! {
            #[doc(hidden)]
            #visibility type #immediate_input = #immediate_type;
            #[doc(hidden)]
            #visibility type #owned_input = #owned_type;
        }
    });
    quote! {
        #context_definition
        #visibility enum #input #parameters { #(#variants,)* }
        #transfer_aliases
    }
}

fn generate_transfer_custom_input_declaration(
    custom_index: usize,
    custom: &CustomModel,
    input_model: &CustomInputModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: TransferFlavor,
) -> TokenStream {
    let input = transfer_custom_input_ident(&input_model.ident, flavor);
    let function_flavor = flavor.function();
    let schema = &custom.schema;
    let input_codec_bounds =
        custom_input_codec_bounds(custom, customs, custom_inputs, support, function_flavor);
    let decoder_definition = generate_custom_decoder(
        custom,
        input_model,
        customs,
        custom_inputs,
        support,
        &input_codec_bounds,
        function_flavor,
    );
    let decoder = match flavor {
        TransferFlavor::Immediate => {
            format_ident!("__GeamTransferImmediate{}", input_model.decoder)
        }
        TransferFlavor::Async => format_ident!("__GeamTransfer{}", input_model.decoder),
    };
    let list_decoder = transfer_list_decoder_ident(&input_model.list_decoder, flavor);
    let list_decoder_value = transfer_list_decoder_value(
        &input_model.list_decoder,
        &StaticValueType::Custom {
            index: custom_index,
        },
        customs,
        support,
        flavor,
        &quote!(Provider),
        &quote!(&*call),
    );
    let list_codec_bounds =
        custom_list_codec_bounds(custom_index, customs, support, function_flavor);
    quote! {
        impl<Profile, Provider, Return>
            #support::ProviderTransferInputValue<Profile, Provider, Return> for #input
        where
            Profile: __GeamAsyncModuleProfile,
            Provider: #support::HostProvider<Profile>,
            Return: #support::HostType,
            #(#input_codec_bounds,)*
        {
            type Host = #support::HostCustomType<#schema>;

            fn from_host<'__geam_call>(
                call: &mut #support::TransferHostCall<
                    '__geam_call,
                    Profile,
                    Provider,
                    Return,
                >,
                value: <Self::Host as #support::HostType>::Value<'__geam_call>,
            ) -> Self {
                #decoder(call, value)
            }
        }

        impl #support::ProviderTransferListInputValue for #input {
            type Host = #support::HostCustomType<#schema>;
            type View = Self;
            type Decoder = #list_decoder;
        }

        impl<Profile, Provider> #support::ProviderTransferListInputCodec<Profile, Provider>
            for #input
        where
            Profile: __GeamAsyncModuleProfile,
            Provider: #support::HostProvider<Profile>,
            #(#list_codec_bounds,)*
        {
            fn decoder<Return>(
                call: &#support::TransferHostCall<'_, Profile, Provider, Return>,
            ) -> Self::Decoder
            where
                Return: #support::HostType,
            {
                #list_decoder_value
            }
        }

        #decoder_definition
    }
}

fn generate_custom_decoder(
    custom: &CustomModel,
    input_model: &CustomInputModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    codec_bounds: &[TokenStream],
    flavor: FunctionFlavor,
) -> TokenStream {
    let decoder = match flavor {
        FunctionFlavor::Local => input_model.decoder.clone(),
        FunctionFlavor::TransferImmediate => {
            format_ident!("__GeamTransferImmediate{}", input_model.decoder)
        }
        FunctionFlavor::Async => format_ident!("__GeamTransfer{}", input_model.decoder),
    };
    let schema = &custom.schema;
    let input = match flavor {
        FunctionFlavor::Local => input_model.ident.clone(),
        FunctionFlavor::TransferImmediate => {
            transfer_custom_input_ident(&input_model.ident, TransferFlavor::Immediate)
        }
        FunctionFlavor::Async => {
            transfer_custom_input_ident(&input_model.ident, TransferFlavor::Async)
        }
    };
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
                flavor,
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
        let expression = custom_input_expression(&input, constructor, &value_names);
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
    let call = match flavor {
        FunctionFlavor::Local => quote!(#support::HostCall),
        FunctionFlavor::TransferImmediate | FunctionFlavor::Async => {
            quote!(#support::TransferHostCall)
        }
    };
    let profile = match flavor {
        FunctionFlavor::Local => quote!(__GeamModuleProfile),
        FunctionFlavor::TransferImmediate | FunctionFlavor::Async => {
            quote!(__GeamAsyncModuleProfile)
        }
    };
    quote! {
        fn #decoder<'__geam_call, Profile, Provider, Return>(
            call: &mut #call<
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
            Profile: #profile,
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
    flavor: FunctionFlavor,
) -> GeneratedValue {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            decode_custom_input_value(type_, input, customs, custom_inputs, support, names, flavor)
        }
        CustomFieldValueType::List(list) => {
            let decoder = match flavor {
                FunctionFlavor::Local => {
                    list_decoder_value(&list.decoder, &list.collection.value, customs, support)
                }
                FunctionFlavor::TransferImmediate => transfer_list_decoder_value(
                    &list.decoder,
                    &list.collection.value,
                    customs,
                    support,
                    TransferFlavor::Immediate,
                    &quote!(Provider),
                    &quote!(&*call),
                ),
                FunctionFlavor::Async => transfer_list_decoder_value(
                    &list.decoder,
                    &list.collection.value,
                    customs,
                    support,
                    TransferFlavor::Async,
                    &quote!(Provider),
                    &quote!(&*call),
                ),
            };
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
    flavor: FunctionFlavor,
) -> GeneratedValue {
    match type_ {
        StaticValueType::Scalar(_) => GeneratedValue {
            statements: TokenStream::new(),
            value: input,
        },
        StaticValueType::Declared { type_, .. } => {
            let value = match flavor {
                FunctionFlavor::Local => quote!(
                    <<#type_ as #support::ProviderValue>::Input as
                        #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                            call,
                            #input,
                        )
                ),
                FunctionFlavor::TransferImmediate => quote!(
                    <<#type_ as #support::ProviderTransferValue>::ImmediateInput as
                        #support::ProviderTransferInputValue<Profile, Provider, Return>>::from_host(
                            call,
                            #input,
                        )
                ),
                FunctionFlavor::Async => quote!(
                    <<#type_ as #support::ProviderTransferValue>::TransferInput as
                        #support::ProviderTransferInputValue<Profile, Provider, Return>>::from_host(
                            call,
                            #input,
                        )
                ),
            };
            GeneratedValue {
                statements: TokenStream::new(),
                value,
            }
        }
        StaticValueType::External {
            payload, schema, ..
        } => {
            let value = match flavor {
                FunctionFlavor::Local => quote!(
                    <#support::ProviderExternalItem<#payload> as
                        #support::ProviderInputValue<Profile, Provider, Return>>::from_host(
                            call,
                            #input,
                        )
                ),
                FunctionFlavor::TransferImmediate => quote!(
                    call.provider_transfer_external_view_with::<
                        __GeamAsyncProvider,
                        #schema,
                        #support::HostTypeListEnd,
                    >(#input)
                ),
                FunctionFlavor::Async => quote!(
                    call.provider_transfer_external_item_with::<
                        __GeamAsyncProvider,
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
            let input_type = match flavor {
                FunctionFlavor::Local => custom_inputs[index].clone(),
                FunctionFlavor::TransferImmediate => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Immediate)
                }
                FunctionFlavor::Async => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Async)
                }
            };
            let input_trait = match flavor {
                FunctionFlavor::Local => quote!(#support::ProviderInputValue),
                FunctionFlavor::TransferImmediate | FunctionFlavor::Async => {
                    quote!(#support::ProviderTransferInputValue)
                }
            };
            GeneratedValue {
                statements: TokenStream::new(),
                value: quote!(
                    <#input_type as #input_trait<
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
                    flavor,
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
                flavor,
            );
            let decoded_failure = decode_custom_input_value(
                failure,
                quote!(#failure_value),
                customs,
                custom_inputs,
                support,
                names,
                flavor,
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
                flavor,
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
    flavor: FunctionFlavor,
) -> TokenStream {
    match type_ {
        CustomFieldValueType::Value(type_) => {
            custom_input_value_type(type_, custom_inputs, support, flavor)
        }
        CustomFieldValueType::List(list) => {
            let item = custom_list_input_value_type(
                &list.collection.value,
                custom_inputs,
                support,
                flavor,
            );
            let decoder = match flavor {
                FunctionFlavor::Local => list.decoder.clone(),
                FunctionFlavor::TransferImmediate => {
                    transfer_list_decoder_ident(&list.decoder, TransferFlavor::Immediate)
                }
                FunctionFlavor::Async => {
                    transfer_list_decoder_ident(&list.decoder, TransferFlavor::Async)
                }
            };
            let context = match flavor {
                FunctionFlavor::Local => quote!(#support::ProviderInputListContext),
                FunctionFlavor::TransferImmediate | FunctionFlavor::Async => {
                    quote!(#support::ProviderTransferInputListContext)
                }
            };
            quote! { #support::List<#item, #context<#decoder>> }
        }
    }
}

fn custom_input_value_type(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: FunctionFlavor,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => match flavor {
            FunctionFlavor::Local => quote!(<#type_ as #support::ProviderValue>::Input),
            FunctionFlavor::TransferImmediate => {
                quote!(<#type_ as #support::ProviderTransferValue>::ImmediateInput)
            }
            FunctionFlavor::Async => {
                quote!(<#type_ as #support::ProviderTransferValue>::TransferInput)
            }
        },
        StaticValueType::External {
            payload,
            transfer_payload,
            ..
        } => match flavor {
            FunctionFlavor::Local => quote!(#support::ProviderExternalItem<#payload>),
            FunctionFlavor::TransferImmediate => {
                quote!(#support::ProviderTransferExternalView<#transfer_payload>)
            }
            FunctionFlavor::Async => {
                quote!(#support::ProviderTransferExternalItem<#transfer_payload>)
            }
        },
        StaticValueType::Custom { index, .. } => {
            let input = match flavor {
                FunctionFlavor::Local => custom_inputs[index].clone(),
                FunctionFlavor::TransferImmediate => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Immediate)
                }
                FunctionFlavor::Async => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Async)
                }
            };
            quote!(#input)
        }
        StaticValueType::Tuple(elements) => {
            let types = elements
                .iter()
                .map(|element| custom_input_value_type(element, custom_inputs, support, flavor))
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = custom_input_value_type(success, custom_inputs, support, flavor);
            let failure = custom_input_value_type(failure, custom_inputs, support, flavor);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = custom_input_value_type(value, custom_inputs, support, flavor);
            quote!(::core::option::Option<#value>)
        }
    }
}

fn custom_list_input_value_type(
    type_: &StaticValueType,
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: FunctionFlavor,
) -> TokenStream {
    match type_ {
        StaticValueType::Scalar(type_) => quote!(#type_),
        StaticValueType::Declared { type_, .. } => match flavor {
            FunctionFlavor::Local => {
                quote!(<#type_ as #support::ProviderValue>::ListInput)
            }
            FunctionFlavor::TransferImmediate => {
                quote!(<#type_ as #support::ProviderTransferValue>::ImmediateListInput)
            }
            FunctionFlavor::Async => {
                quote!(<#type_ as #support::ProviderTransferValue>::TransferListInput)
            }
        },
        StaticValueType::External {
            payload,
            transfer_payload,
            ..
        } => match flavor {
            FunctionFlavor::Local => quote!(#payload),
            FunctionFlavor::TransferImmediate => {
                quote!(#support::ProviderTransferExternalView<#transfer_payload>)
            }
            FunctionFlavor::Async => {
                quote!(#support::ProviderTransferExternalItem<#transfer_payload>)
            }
        },
        StaticValueType::Custom { index, .. } => {
            let input = match flavor {
                FunctionFlavor::Local => custom_inputs[index].clone(),
                FunctionFlavor::TransferImmediate => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Immediate)
                }
                FunctionFlavor::Async => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Async)
                }
            };
            quote!(#input)
        }
        StaticValueType::Tuple(elements) => {
            let types = elements
                .iter()
                .map(|element| {
                    custom_list_input_value_type(element, custom_inputs, support, flavor)
                })
                .collect::<Vec<_>>();
            quote!((#(#types,)*))
        }
        StaticValueType::Result { success, failure } => {
            let success = custom_list_input_value_type(success, custom_inputs, support, flavor);
            let failure = custom_list_input_value_type(failure, custom_inputs, support, flavor);
            quote!(::core::result::Result<#success, #failure>)
        }
        StaticValueType::Option { value } => {
            let value = custom_list_input_value_type(value, custom_inputs, support, flavor);
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
    representation: ProviderRepresentation,
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
                representation,
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
    representation: ProviderRepresentation,
) {
    let output_trait = match representation {
        ProviderRepresentation::Local => quote!(#support::ProviderOutputValue),
        ProviderRepresentation::Transfer => quote!(#support::ProviderTransferOutputValue),
    };
    match type_ {
        StaticValueType::Declared { type_, .. } => {
            let type_ = match representation {
                ProviderRepresentation::Local => quote!(#type_),
                ProviderRepresentation::Transfer => {
                    quote!(<#type_ as #support::ProviderTransferValue>::Output)
                }
            };
            bounds.push(quote! {
                #type_: #output_trait<
                    Profile,
                    #provider,
                    #return_type,
                >
            });
        }
        StaticValueType::Custom { index, .. } => {
            let type_ = &customs[*index].ident;
            let type_ = match representation {
                ProviderRepresentation::Local => quote!(#type_),
                ProviderRepresentation::Transfer => {
                    quote!(<#type_ as #support::ProviderTransferValue>::Output)
                }
            };
            bounds.push(quote! {
                #type_: #output_trait<
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
                    representation,
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
                representation,
            );
            collect_custom_output_codec_bounds(
                failure,
                customs,
                support,
                provider,
                return_type,
                bounds,
                representation,
            );
        }
        StaticValueType::Option { value } => collect_custom_output_codec_bounds(
            value,
            customs,
            support,
            provider,
            return_type,
            bounds,
            representation,
        ),
        StaticValueType::Scalar(_) | StaticValueType::External { .. } => {}
    }
}

fn custom_input_codec_bounds(
    custom: &CustomModel,
    customs: &[CustomModel],
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    flavor: FunctionFlavor,
) -> Vec<TokenStream> {
    let mut bounds = Vec::new();
    for constructor in &custom.constructors {
        for field in custom_field_models(&constructor.fields) {
            match &field.value {
                CustomFieldValueType::Value(value) => {
                    collect_custom_input_codec_bounds(
                        value,
                        customs,
                        custom_inputs,
                        support,
                        &mut bounds,
                        flavor,
                    );
                }
                CustomFieldValueType::List(list) => {
                    for access in list_declared_accesses(&list.collection.value, customs) {
                        let type_ = access.type_;
                        bounds.push(match flavor {
                            FunctionFlavor::Local => quote! {
                                <#type_ as #support::ProviderValue>::ListInput:
                                    #support::ProviderListInputCodec<Profile>
                            },
                            FunctionFlavor::TransferImmediate => quote! {
                                <#type_ as #support::ProviderTransferValue>::ImmediateListInput:
                                    #support::ProviderTransferListInputCodec<Profile, Provider>
                            },
                            FunctionFlavor::Async => quote! {
                                <#type_ as #support::ProviderTransferValue>::TransferListInput:
                                    #support::ProviderTransferListInputCodec<Profile, Provider>
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
    custom_inputs: &BTreeMap<usize, Ident>,
    support: &TokenStream,
    bounds: &mut Vec<TokenStream>,
    flavor: FunctionFlavor,
) {
    match type_ {
        StaticValueType::Declared { type_, .. } => {
            bounds.push(match flavor {
                FunctionFlavor::Local => quote! {
                    <#type_ as #support::ProviderValue>::Input:
                        #support::ProviderInputValue<Profile, Provider, Return>
                },
                FunctionFlavor::TransferImmediate => quote! {
                    <#type_ as #support::ProviderTransferValue>::ImmediateInput:
                        #support::ProviderTransferInputValue<
                            Profile,
                            Provider,
                            Return,
                            Host = <#type_ as #support::ProviderValue>::Host,
                        >
                },
                FunctionFlavor::Async => quote! {
                    <#type_ as #support::ProviderTransferValue>::TransferInput:
                        #support::ProviderTransferInputValue<
                            Profile,
                            Provider,
                            Return,
                            Host = <#type_ as #support::ProviderValue>::Host,
                        >
                },
            });
        }
        StaticValueType::Custom { index, .. } => {
            let input = match flavor {
                FunctionFlavor::Local => custom_inputs[index].clone(),
                FunctionFlavor::TransferImmediate => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Immediate)
                }
                FunctionFlavor::Async => {
                    transfer_custom_input_ident(&custom_inputs[index], TransferFlavor::Async)
                }
            };
            let input_trait = match flavor {
                FunctionFlavor::Local => quote! {
                    #support::ProviderInputValue<Profile, Provider, Return>
                },
                FunctionFlavor::TransferImmediate | FunctionFlavor::Async => {
                    let schema = &customs[*index].schema;
                    quote! {
                        #support::ProviderTransferInputValue<
                            Profile,
                            Provider,
                            Return,
                            Host = #support::HostCustomType<#schema>,
                        >
                    }
                }
            };
            bounds.push(quote! {
                #input: #input_trait
            });
        }
        StaticValueType::Tuple(elements) => {
            for element in elements {
                collect_custom_input_codec_bounds(
                    element,
                    customs,
                    custom_inputs,
                    support,
                    bounds,
                    flavor,
                );
            }
        }
        StaticValueType::Result { success, failure } => {
            collect_custom_input_codec_bounds(
                success,
                customs,
                custom_inputs,
                support,
                bounds,
                flavor,
            );
            collect_custom_input_codec_bounds(
                failure,
                customs,
                custom_inputs,
                support,
                bounds,
                flavor,
            );
        }
        StaticValueType::Option { value } => {
            collect_custom_input_codec_bounds(
                value,
                customs,
                custom_inputs,
                support,
                bounds,
                flavor,
            );
        }
        StaticValueType::Scalar(_) | StaticValueType::External { .. } => {}
    }
}

fn custom_list_codec_bounds(
    custom_index: usize,
    customs: &[CustomModel],
    support: &TokenStream,
    flavor: FunctionFlavor,
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
            FunctionFlavor::Local => quote! {
                <#type_ as #support::ProviderValue>::ListInput:
                    #support::ProviderListInputCodec<Profile>
            },
            FunctionFlavor::TransferImmediate => quote! {
                <#type_ as #support::ProviderTransferValue>::ImmediateListInput:
                    #support::ProviderTransferListInputCodec<Profile, Provider>
            },
            FunctionFlavor::Async => quote! {
                <#type_ as #support::ProviderTransferValue>::TransferListInput:
                    #support::ProviderTransferListInputCodec<Profile, Provider>
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
