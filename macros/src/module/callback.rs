use super::{
    CallbackType, FunctionInputType, FunctionInputValueType, FunctionModel,
    FunctionOutputValueType, FunctionReturnType, FunctionRootOutputValueType, ProviderValueType,
};

#[derive(Clone, Copy)]
pub(super) enum InputCodec<'model> {
    Callback(&'model CallbackType),
    Captures(&'model CallbackType, super::InputOwnership),
    Declared(&'model syn::Type),
    Future(&'model super::FutureInputType),
    Custom {
        index: usize,
        model: &'model super::CustomModel,
    },
}

impl InputCodec<'_> {
    pub(super) fn key(&self) -> String {
        match self {
            Self::Callback(value) | Self::Captures(value, _) => value.codec.to_string(),
            Self::Declared(type_) => {
                use quote::quote;
                quote!(#type_).to_string()
            }
            Self::Future(value) => value.codec.to_string(),
            Self::Custom { model, .. } => model.schema.to_string(),
        }
    }

    pub(super) fn requires_work(self, customs: &[super::CustomModel]) -> bool {
        match self {
            Self::Future(_) => true,
            Self::Custom { index, .. } => super::list_capability::capabilities(
                &super::StaticValueType::Custom { index },
                customs,
            )
            .iter()
            .any(|capability| capability.requires_work(customs)),
            _ => false,
        }
    }

    pub(super) fn generate(
        &self,
        generics: &[super::FunctionGeneric],
        customs: &[super::CustomModel],
        support: &proc_macro2::TokenStream,
    ) -> super::GeneratedCallback {
        match self {
            Self::Declared(type_) => {
                use quote::quote;
                super::GeneratedCallback {
                    definition: proc_macro2::TokenStream::new(),
                    requirements: quote!(<#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::InputRequirements),
                    has_constructions: true,
                    bounds: Vec::new(),
                }
            }
            Self::Callback(value) => {
                super::function::generate_callback_codec(value, generics, customs, support)
            }
            Self::Captures(value, flavor) => super::function::generate_callback_codec_with_flavor(
                value, generics, customs, support, *flavor,
            ),
            Self::Future(value) => {
                super::function::generate_future_codec(value, generics, customs, support)
            }
            Self::Custom { index, .. } => {
                super::custom_context::requirements(*index, customs, support)
            }
        }
    }
}

pub(super) fn callbacks<'model>(
    function: &'model FunctionModel,
    customs: &'model [super::CustomModel],
) -> Vec<&'model CallbackType> {
    function_codecs(function, customs)
        .into_iter()
        .flat_map(|codec| match codec {
            InputCodec::Callback(value) => vec![value],
            InputCodec::Future(_) | InputCodec::Declared(_) | InputCodec::Captures(_, _) => {
                Vec::new()
            }
            InputCodec::Custom { index, .. } => super::custom_context::callbacks(index, customs),
        })
        .collect()
}

pub(super) fn requires_work(callback: &CallbackType, customs: &[super::CustomModel]) -> bool {
    let mut found = Vec::new();
    collect(callback, &mut found, true, customs);
    found.iter().any(|codec| codec.requires_work(customs))
}

pub(super) fn instantiated_codec_type(
    codec: &syn::Ident,
    parameters: &[syn::Ident],
    environment: &super::InputEnvironment<'_>,
) -> proc_macro2::TokenStream {
    use quote::quote;
    let support = environment.support;
    let arguments = environment
        .function_generics
        .iter()
        .filter(|generic| parameters.contains(&generic.ident))
        .map(|generic| match environment.generic_source {
            super::GenericInputSource::Instantiated => {
                let index = generic.index;
                quote!(#support::HostTypeParameter<#index>)
            }
            super::GenericInputSource::Declared => {
                let ident = &generic.ident;
                quote!(#ident)
            }
        })
        .collect();
    super::signature::callback_codec_type(codec, arguments)
}

pub(super) fn codecs<'model>(
    function: &'model FunctionModel,
    customs: &'model [super::CustomModel],
) -> Vec<InputCodec<'model>> {
    let mut found = function_codecs(function, customs);
    if let Some(callable) = &function.callable {
        for (encoder, flavor) in [
            (&callable.capture_encoder, super::InputOwnership::Owned),
            (
                &callable.borrowed_capture_encoder,
                super::InputOwnership::Borrowed,
            ),
        ] {
            found.push(InputCodec::Captures(encoder, flavor));
            for argument in &encoder.arguments {
                collect_return(argument, &mut found, customs);
            }
        }
        collect(&callable.returned, &mut found, true, customs);
    }
    found
}

pub(super) fn function_codecs<'model>(
    function: &'model FunctionModel,
    customs: &'model [super::CustomModel],
) -> Vec<InputCodec<'model>> {
    let mut found = Vec::new();
    collect_return(&function.return_, &mut found, customs);
    for argument in &function.arguments {
        collect_input(argument, &mut found, true, customs);
    }
    found
}

pub(super) fn source_codecs<'model>(
    function: &'model FunctionModel,
    customs: &'model [super::CustomModel],
) -> Vec<InputCodec<'model>> {
    let mut found = Vec::new();
    for argument in &function.arguments {
        collect_input(argument, &mut found, false, customs);
    }
    found
}

pub(super) fn input_codecs<'model>(
    input: &'model FunctionInputType,
    customs: &'model [super::CustomModel],
) -> Vec<InputCodec<'model>> {
    let mut found = Vec::new();
    collect_input(input, &mut found, false, customs);
    found
}

fn collect<'model>(
    callback: &'model CallbackType,
    found: &mut Vec<InputCodec<'model>>,
    nested: bool,
    customs: &'model [super::CustomModel],
) {
    found.push(InputCodec::Callback(callback));
    if nested {
        collect_input(&callback.return_, found, true, customs);
        for argument in &callback.arguments {
            collect_return(argument, found, customs);
        }
    }
}

fn collect_input<'model>(
    input: &'model FunctionInputType,
    found: &mut Vec<InputCodec<'model>>,
    nested: bool,
    customs: &'model [super::CustomModel],
) {
    match input {
        FunctionInputType::Callback(callback) => collect(callback, found, nested, customs),
        FunctionInputType::Future(future) => {
            found.push(InputCodec::Future(future));
            if nested {
                collect_input(&future.value, found, true, customs);
            }
        }
        FunctionInputType::Value(value) => match value.as_ref() {
            FunctionInputValueType::Tuple(elements) => {
                for element in elements {
                    collect_value(element, found, nested, customs);
                }
            }
            FunctionInputValueType::Result { success, failure } => {
                collect_value(success, found, nested, customs);
                collect_value(failure, found, nested, customs);
            }
            FunctionInputValueType::Option { value } => {
                collect_value(value, found, nested, customs)
            }
            FunctionInputValueType::Scalar(_) | FunctionInputValueType::External { .. } => {}
            FunctionInputValueType::Declared { type_, .. } => {
                found.push(InputCodec::Declared(type_))
            }
            FunctionInputValueType::Custom { index, .. } => collect_custom(*index, found, customs),
        },
        FunctionInputType::List(list) => {
            collect_static(&list.collection.value, found, nested, customs)
        }
        FunctionInputType::Generic(_) | FunctionInputType::External(_) => {}
    }
}

fn collect_value<'model>(
    value: &'model ProviderValueType,
    found: &mut Vec<InputCodec<'model>>,
    nested: bool,
    customs: &'model [super::CustomModel],
) {
    match value {
        ProviderValueType::Future(future) => {
            found.push(InputCodec::Future(future));
            if nested {
                collect_input(&future.value, found, true, customs);
            }
        }
        ProviderValueType::Callback(callback) => collect(callback, found, nested, customs),
        ProviderValueType::Tuple(elements) => {
            for element in elements {
                collect_value(element, found, nested, customs);
            }
        }
        ProviderValueType::Result { success, failure } => {
            collect_value(success, found, nested, customs);
            collect_value(failure, found, nested, customs);
        }
        ProviderValueType::Option { value } => collect_value(value, found, nested, customs),
        ProviderValueType::Scalar(_)
        | ProviderValueType::Generic(_)
        | ProviderValueType::External { .. } => {}
        ProviderValueType::Declared { type_, .. } => found.push(InputCodec::Declared(type_)),
        ProviderValueType::Custom { index, .. } => collect_custom(*index, found, customs),
        ProviderValueType::List(list) => {
            collect_static(&list.collection.value, found, nested, customs)
        }
    }
}

fn collect_return<'model>(
    output: &'model FunctionReturnType,
    found: &mut Vec<InputCodec<'model>>,
    customs: &'model [super::CustomModel],
) {
    match output {
        FunctionReturnType::Callback(callback) => collect(callback, found, true, customs),
        FunctionReturnType::Future(future) => {
            found.push(InputCodec::Future(future));
            collect_input(&future.value, found, true, customs);
        }
        FunctionReturnType::Value(value) => match value {
            FunctionRootOutputValueType::Tuple(elements) => {
                for element in elements {
                    collect_output(element, found, customs);
                }
            }
            FunctionRootOutputValueType::Result { success, failure } => {
                collect_output(success, found, customs);
                collect_output(failure, found, customs);
            }
            FunctionRootOutputValueType::Option { value } => collect_output(value, found, customs),
            FunctionRootOutputValueType::Vec(collection) => {
                collect_output(&collection.value, found, customs)
            }
            FunctionRootOutputValueType::Value(value) => {
                if let super::FunctionOutputLeafType::Custom { index, .. } = value.as_ref() {
                    collect_custom(*index, found, customs);
                }
            }
        },
        FunctionReturnType::Generic(_) | FunctionReturnType::External(_) => {}
        FunctionReturnType::List(list) => {
            collect_static(&list.collection.value, found, true, customs)
        }
    }
}

fn collect_output<'model>(
    output: &'model FunctionOutputValueType,
    found: &mut Vec<InputCodec<'model>>,
    customs: &'model [super::CustomModel],
) {
    match output {
        FunctionOutputValueType::Future(future) => {
            found.push(InputCodec::Future(future));
            collect_input(&future.value, found, true, customs);
        }
        FunctionOutputValueType::Callback(callback) => collect(callback, found, true, customs),
        FunctionOutputValueType::Tuple(elements) => {
            for element in elements {
                collect_output(element, found, customs);
            }
        }
        FunctionOutputValueType::Result { success, failure } => {
            collect_output(success, found, customs);
            collect_output(failure, found, customs);
        }
        FunctionOutputValueType::Option { value } => collect_output(value, found, customs),
        FunctionOutputValueType::Vec(collection) => {
            collect_output(&collection.value, found, customs)
        }
        FunctionOutputValueType::Value(value) => {
            if let super::FunctionOutputLeafType::Custom { index, .. } = value.as_ref() {
                collect_custom(*index, found, customs);
            }
        }
        FunctionOutputValueType::Generic(_) => {}
    }
}

fn collect_static<'model>(
    value: &'model super::StaticValueType,
    found: &mut Vec<InputCodec<'model>>,
    nested: bool,
    customs: &'model [super::CustomModel],
) {
    use super::StaticValueType as V;
    match value {
        V::List(list) => collect_static(&list.collection.value, found, nested, customs),
        V::Future(future) => {
            found.push(InputCodec::Future(future));
            if nested {
                collect_input(&future.value, found, true, customs);
            }
        }
        V::Callback(callback) => collect(callback, found, nested, customs),
        V::Tuple(elements) => {
            for element in elements {
                collect_static(element, found, nested, customs);
            }
        }
        V::Result { success, failure } => {
            collect_static(success, found, nested, customs);
            collect_static(failure, found, nested, customs);
        }
        V::Option { value } => collect_static(value, found, nested, customs),
        V::Custom { index } => collect_custom(*index, found, customs),
        V::Declared { type_ } => found.push(InputCodec::Declared(type_)),
        V::Scalar(_) | V::External { .. } => {}
    }
}

fn collect_custom<'model>(
    index: usize,
    found: &mut Vec<InputCodec<'model>>,
    customs: &'model [super::CustomModel],
) {
    if super::custom_context::is_contextual(index, customs) {
        found.push(InputCodec::Custom {
            index,
            model: &customs[index],
        });
    }
}

pub(super) fn custom_codecs<'model>(
    custom: &'model super::CustomModel,
    customs: &'model [super::CustomModel],
) -> Vec<InputCodec<'model>> {
    use super::custom_value::{CustomFieldValueType, custom_field_models};
    let mut found = Vec::new();
    for field in custom
        .constructors
        .iter()
        .flat_map(|constructor| custom_field_models(&constructor.fields))
    {
        let value = match &field.value {
            CustomFieldValueType::Value(value) => value,
            CustomFieldValueType::List(list) => &list.collection.value,
        };
        collect_static(value, &mut found, false, customs);
    }
    found
}

pub(super) fn custom_definitions(
    customs: &[super::CustomModel],
    support: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    let mut definitions = std::collections::BTreeMap::new();
    for custom in customs {
        let mut found = Vec::new();
        for codec in custom_codecs(custom, customs) {
            match codec {
                InputCodec::Callback(callback) => collect(callback, &mut found, true, customs),
                InputCodec::Future(future) => {
                    found.push(codec);
                    collect_input(&future.value, &mut found, true, customs);
                }
                InputCodec::Custom { .. }
                | InputCodec::Declared(_)
                | InputCodec::Captures(_, _) => {}
            }
        }
        let generics = super::custom_context::generics(custom);
        for codec in found
            .into_iter()
            .filter(|codec| matches!(codec, InputCodec::Callback(_) | InputCodec::Future(_)))
        {
            definitions.insert(
                codec.key(),
                codec.generate(&generics, customs, support).definition,
            );
        }
    }
    definitions.into_values().collect()
}
