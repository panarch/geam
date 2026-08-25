use super::{
    CollectionType, ExternalModel, ListDecoderModel, ListType, SourceWrapper, StaticValueType,
    collection_item, external_type, is_collection, is_marker, is_qualified_type_path,
    register_list_decoder, source_wrapper,
};
use quote::format_ident;
use std::collections::BTreeSet;
use syn::ext::IdentExt;
use syn::parse::ParseStream;
use syn::{Attribute, Fields, Ident, Item, ItemEnum, Meta, Token, Type, TypePath, Visibility};

pub(super) struct CustomArguments {
    pub(super) input: Option<Ident>,
}

#[derive(Default)]
struct PartialCustomArguments {
    input: Option<Ident>,
}

struct CustomHeader {
    ident: Ident,
    input: Option<Ident>,
    visibility: Visibility,
    schema: Ident,
    decoder: Ident,
    item: ItemEnum,
}

pub(super) struct CustomModel {
    pub(super) ident: Ident,
    pub(super) input: Option<Ident>,
    pub(super) visibility: Visibility,
    pub(super) schema: Ident,
    pub(super) decoder: Ident,
    pub(super) constructors: Vec<CustomConstructorModel>,
}

pub(super) struct CustomConstructorModel {
    pub(super) ident: Ident,
    pub(super) definition: Ident,
    pub(super) marker: Ident,
    pub(super) fields: CustomFields,
}

pub(super) enum CustomFields {
    Unit,
    Unnamed(Vec<CustomFieldModel>),
    Named(Vec<CustomFieldModel>),
}

pub(super) struct CustomFieldModel {
    pub(super) ident: Option<Ident>,
    pub(super) definition: Ident,
    pub(super) value: CustomFieldValueType,
}

pub(super) enum CustomFieldValueType {
    Value(Box<StaticValueType>),
    List(Box<ListType>),
}

pub(super) struct CustomDeclarations {
    pub(super) models: Vec<CustomModel>,
    pub(super) list_decoders: Vec<Option<Ident>>,
}

fn parse_custom_arguments(input: ParseStream<'_>) -> syn::Result<CustomArguments> {
    let mut partial = PartialCustomArguments::default();
    while !input.is_empty() {
        let field = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        match field.to_string().as_str() {
            "input" => {
                if partial.input.replace(input.parse()?).is_some() {
                    return Err(syn::Error::new(
                        field.span(),
                        "duplicate custom argument `input`",
                    ));
                }
            }
            _ => {
                return Err(syn::Error::new(
                    field.span(),
                    format!("unknown custom argument `{field}`"),
                ));
            }
        }
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(CustomArguments {
        input: partial.input,
    })
}

pub(super) fn collect_custom_declarations(
    items: &mut [Item],
    source_names: &mut BTreeSet<String>,
    generated_input_names: &mut BTreeSet<String>,
    provider_type_names: &BTreeSet<String>,
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<CustomDeclarations> {
    let mut headers = Vec::new();
    for item in items {
        let Item::Enum(custom) = item else {
            continue;
        };
        let Some(arguments) = take_custom_marker(&mut custom.attrs)? else {
            continue;
        };
        validate_custom(custom)?;
        let source_name = custom.ident.unraw().to_string();
        if !source_names.insert(source_name.clone()) {
            return Err(syn::Error::new(
                custom.ident.span(),
                format!("duplicate source type `{source_name}`"),
            ));
        }
        if let Some(input) = &arguments.input {
            let input_name = input.unraw().to_string();
            if provider_type_names.contains(&input_name) {
                return Err(syn::Error::new(
                    input.span(),
                    format!(
                        "generated custom input type `{input_name}` conflicts with provider value type `{input_name}`"
                    ),
                ));
            }
            if !generated_input_names.insert(input_name.clone()) {
                return Err(syn::Error::new(
                    input.span(),
                    format!("duplicate generated input type `{input_name}`"),
                ));
            }
        }
        let index = headers.len();
        headers.push(CustomHeader {
            ident: custom.ident.clone(),
            input: arguments.input,
            visibility: custom.vis.clone(),
            schema: format_ident!("__GeamCustomSchema{index}"),
            decoder: format_ident!("__geam_decode_custom_{index}"),
            item: custom.clone(),
        });
    }

    let mut models = Vec::with_capacity(headers.len());
    for (index, header) in headers.iter().enumerate() {
        models.push(build_custom_model(
            index,
            header,
            &headers,
            externals,
            list_decoders,
        )?);
    }
    validate_custom_cycles(&models)?;

    let mut custom_list_decoders = Vec::with_capacity(models.len());
    for (index, custom) in models.iter().enumerate() {
        let decoder = if let Some(input) = &custom.input {
            let item: Type = syn::parse_quote!(#input);
            Some(register_list_decoder(
                &CollectionType {
                    source: item.clone(),
                    item: item.clone(),
                    value: StaticValueType::Custom { index },
                },
                list_decoders,
            ))
        } else {
            None
        };
        custom_list_decoders.push(decoder);
    }

    Ok(CustomDeclarations {
        models,
        list_decoders: custom_list_decoders,
    })
}

fn take_custom_marker(attributes: &mut Vec<Attribute>) -> syn::Result<Option<CustomArguments>> {
    let mut retained = Vec::with_capacity(attributes.len());
    let mut found = None;
    for attribute in std::mem::take(attributes) {
        if !is_marker(&attribute, "custom") {
            retained.push(attribute);
            continue;
        }
        if found.is_some() {
            return Err(syn::Error::new_spanned(
                attribute,
                "duplicate `#[geam::custom]` attribute",
            ));
        }
        let arguments = match &attribute.meta {
            Meta::List(_) => attribute.parse_args_with(parse_custom_arguments)?,
            Meta::Path(_) => CustomArguments { input: None },
            Meta::NameValue(_) => {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "`#[geam::custom]` accepts only `input = Type` arguments",
                ));
            }
        };
        found = Some(arguments);
    }
    *attributes = retained;
    Ok(found)
}

fn validate_custom(custom: &ItemEnum) -> syn::Result<()> {
    if !custom.generics.params.is_empty() || custom.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &custom.generics,
            "custom value enums must not have generics",
        ));
    }
    if custom.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &custom.ident,
            "custom value enums must declare at least one constructor",
        ));
    }
    for variant in &custom.variants {
        if let Some((_, discriminant)) = &variant.discriminant {
            return Err(syn::Error::new_spanned(
                discriminant,
                "custom value constructors must not have Rust discriminants",
            ));
        }
    }
    Ok(())
}

fn build_custom_model(
    custom_index: usize,
    header: &CustomHeader,
    headers: &[CustomHeader],
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<CustomModel> {
    let mut constructors = Vec::with_capacity(header.item.variants.len());
    for (constructor_index, variant) in header.item.variants.iter().enumerate() {
        let fields = match &variant.fields {
            Fields::Unit => CustomFields::Unit,
            Fields::Unnamed(fields) => {
                let mut models = Vec::with_capacity(fields.unnamed.len());
                for (field_index, field) in fields.unnamed.iter().enumerate() {
                    models.push(build_custom_field(
                        custom_index,
                        constructor_index,
                        field_index,
                        None,
                        &field.ty,
                        headers,
                        externals,
                        list_decoders,
                    )?);
                }
                CustomFields::Unnamed(models)
            }
            Fields::Named(fields) => {
                let mut models = Vec::with_capacity(fields.named.len());
                for (field_index, field) in fields.named.iter().enumerate() {
                    models.push(build_custom_field(
                        custom_index,
                        constructor_index,
                        field_index,
                        field.ident.clone(),
                        &field.ty,
                        headers,
                        externals,
                        list_decoders,
                    )?);
                }
                CustomFields::Named(models)
            }
        };
        constructors.push(CustomConstructorModel {
            ident: variant.ident.clone(),
            definition: format_ident!(
                "__GeamCustom{custom_index}Constructor{constructor_index}Definition"
            ),
            marker: format_ident!("__GeamCustom{custom_index}Constructor{constructor_index}"),
            fields,
        });
    }

    Ok(CustomModel {
        ident: header.ident.clone(),
        input: header.input.clone(),
        visibility: header.visibility.clone(),
        schema: header.schema.clone(),
        decoder: header.decoder.clone(),
        constructors,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_custom_field(
    custom_index: usize,
    constructor_index: usize,
    field_index: usize,
    ident: Option<Ident>,
    type_: &Type,
    headers: &[CustomHeader],
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<CustomFieldModel> {
    Ok(CustomFieldModel {
        ident,
        definition: format_ident!(
            "__GeamCustom{custom_index}Constructor{constructor_index}Field{field_index}"
        ),
        value: classify_custom_output_value(type_, headers, externals, list_decoders)?,
    })
}

fn classify_custom_output_value(
    type_: &Type,
    headers: &[CustomHeader],
    externals: &[ExternalModel],
    list_decoders: &mut Vec<ListDecoderModel>,
) -> syn::Result<CustomFieldValueType> {
    if let Some(item) = collection_item(type_, "Vec")? {
        if let Type::Reference(_) = &item {
            return Err(syn::Error::new_spanned(
                &item,
                "custom List item outputs must be owned values",
            ));
        }
        if is_collection(&item, "List") || is_collection(&item, "Vec") {
            return Err(syn::Error::new_spanned(
                &item,
                "nested List values are not supported in custom declarations",
            ));
        }
        let value = classify_custom_value(&item, headers, externals)?;
        let collection = CollectionType {
            source: type_.clone(),
            item,
            value,
        };
        let decoder = register_list_decoder(&collection, list_decoders);
        return Ok(CustomFieldValueType::List(Box::new(ListType {
            collection,
            decoder,
        })));
    }
    if is_collection(type_, "List") {
        return Err(syn::Error::new_spanned(
            type_,
            "custom output List fields use Vec<T>; generated input values use geam::List<T>",
        ));
    }
    Ok(CustomFieldValueType::Value(Box::new(
        classify_custom_value(type_, headers, externals)?,
    )))
}

fn classify_custom_value(
    type_: &Type,
    headers: &[CustomHeader],
    externals: &[ExternalModel],
) -> syn::Result<StaticValueType> {
    if let Type::Reference(_) = type_ {
        return Err(syn::Error::new_spanned(
            type_,
            "custom output fields must be owned values",
        ));
    }
    if is_collection(type_, "List") || is_collection(type_, "Vec") {
        return Err(syn::Error::new_spanned(
            type_,
            "List values are not supported inside custom tuple fields",
        ));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            return Ok(StaticValueType::Result {
                success: Box::new(classify_custom_value(success, headers, externals)?),
                failure: Box::new(classify_custom_value(failure, headers, externals)?),
            });
        }
        SourceWrapper::Option { value, .. } => {
            return Ok(StaticValueType::Option {
                value: Box::new(classify_custom_value(value, headers, externals)?),
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
    if let Some((index, _)) = custom_header_output_type(type_, headers) {
        return Ok(StaticValueType::Custom { index });
    }
    if let Some(header) = custom_header_input_type(type_, headers) {
        return Err(syn::Error::new_spanned(
            type_,
            format!(
                "custom input `{}` cannot be stored in an output; use `{}`",
                header.input.as_ref().expect("matched input must exist"),
                header.ident
            ),
        ));
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let mut elements = Vec::with_capacity(tuple.elems.len());
        for element in &tuple.elems {
            elements.push(classify_custom_value(element, headers, externals)?);
        }
        return Ok(StaticValueType::Tuple(elements));
    }
    if is_qualified_type_path(type_) {
        return Ok(StaticValueType::Declared {
            type_: type_.clone(),
        });
    }
    Ok(StaticValueType::Scalar(type_.clone()))
}

fn custom_header_output_type<'custom>(
    type_: &Type,
    customs: &'custom [CustomHeader],
) -> Option<(usize, &'custom CustomHeader)> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    for (index, custom) in customs.iter().enumerate() {
        if &custom.ident == ident {
            return Some((index, custom));
        }
    }
    None
}

fn custom_header_input_type<'custom>(
    type_: &Type,
    customs: &'custom [CustomHeader],
) -> Option<&'custom CustomHeader> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    customs
        .iter()
        .find(|custom| custom.input.as_ref() == Some(ident))
}

fn validate_custom_cycles(customs: &[CustomModel]) -> syn::Result<()> {
    fn visit(
        index: usize,
        customs: &[CustomModel],
        visiting: &mut Vec<usize>,
        visited: &mut BTreeSet<usize>,
    ) -> syn::Result<()> {
        let mut cycle_start = None;
        for (position, current) in visiting.iter().enumerate() {
            if *current == index {
                cycle_start = Some(position);
                break;
            }
        }
        if let Some(position) = cycle_start {
            let mut cycle = Vec::with_capacity(visiting.len() - position + 1);
            for nested in &visiting[position..] {
                cycle.push(customs[*nested].ident.to_string());
            }
            cycle.push(customs[index].ident.to_string());
            return Err(syn::Error::new(
                customs[index].ident.span(),
                format!(
                    "recursive custom values are not supported: {}",
                    cycle.join(" -> ")
                ),
            ));
        }
        if !visited.insert(index) {
            return Ok(());
        }
        visiting.push(index);
        for nested in custom_dependencies(&customs[index]) {
            visit(nested, customs, visiting, visited)?;
        }
        visiting.pop();
        Ok(())
    }

    let mut visited = BTreeSet::new();
    for index in 0..customs.len() {
        visit(index, customs, &mut Vec::new(), &mut visited)?;
    }
    Ok(())
}

fn custom_dependencies(custom: &CustomModel) -> Vec<usize> {
    fn collect(type_: &StaticValueType, output: &mut Vec<usize>) {
        match type_ {
            StaticValueType::Custom { index, .. } => output.push(*index),
            StaticValueType::Tuple(elements) => {
                for element in elements {
                    collect(element, output);
                }
            }
            StaticValueType::Result { success, failure } => {
                collect(success, output);
                collect(failure, output);
            }
            StaticValueType::Option { value } => collect(value, output),
            StaticValueType::Scalar(_)
            | StaticValueType::Declared { .. }
            | StaticValueType::External { .. } => {}
        }
    }

    let mut output = Vec::new();
    for constructor in &custom.constructors {
        let fields = match &constructor.fields {
            CustomFields::Unit => continue,
            CustomFields::Unnamed(fields) | CustomFields::Named(fields) => fields,
        };
        for field in fields {
            match &field.value {
                CustomFieldValueType::Value(value) => collect(value, &mut output),
                CustomFieldValueType::List(list) => collect(&list.collection.value, &mut output),
            }
        }
    }
    output
}

pub(super) fn custom_output_type<'custom>(
    type_: &Type,
    customs: &'custom [CustomModel],
) -> Option<&'custom CustomModel> {
    let (_, custom) = custom_output_type_with_index(type_, customs)?;
    Some(custom)
}

pub(super) fn custom_output_type_with_index<'custom>(
    type_: &Type,
    customs: &'custom [CustomModel],
) -> Option<(usize, &'custom CustomModel)> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    for (index, custom) in customs.iter().enumerate() {
        if &custom.ident == ident {
            return Some((index, custom));
        }
    }
    None
}

pub(super) fn custom_input_model<'custom>(
    type_: &Type,
    customs: &'custom [CustomModel],
) -> Option<(usize, &'custom CustomModel, &'custom Ident)> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    for (index, custom) in customs.iter().enumerate() {
        if let Some(input) = custom.input.as_ref()
            && input == ident
        {
            return Some((index, custom, input));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::parse_custom_arguments;
    use quote::quote;
    use syn::parse::Parser;

    #[test]
    fn arguments_accept_only_one_generated_input_name() {
        let output_only = parse_custom_arguments
            .parse2(quote!())
            .expect("output-only custom declaration should parse");
        assert!(output_only.input.is_none());

        let directional = parse_custom_arguments
            .parse2(quote!(input = StatusInput))
            .expect("generated custom input name should parse");
        assert_eq!(
            directional.input.expect("input should exist"),
            "StatusInput"
        );

        let cases = [
            (
                quote!(input = FirstInput, input = SecondInput),
                "duplicate custom argument `input`",
            ),
            (
                quote!(other = StatusInput),
                "unknown custom argument `other`",
            ),
            (quote!(= StatusInput), "expected identifier"),
            (quote!(input StatusInput), "expected `=`"),
            (quote!(input = "StatusInput"), "expected identifier"),
            (
                quote!(input = StatusInput other = OtherInput),
                "expected `,`",
            ),
        ];

        for (arguments, expected) in cases {
            assert_eq!(
                parse_custom_arguments
                    .parse2(arguments)
                    .err()
                    .expect("custom arguments should be rejected")
                    .to_string(),
                expected,
            );
        }
    }
}
