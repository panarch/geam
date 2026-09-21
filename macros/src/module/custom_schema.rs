use super::signature::host_type_token_sequence;
use super::type_syntax::{collection_item, source_wrapper};
use super::{CustomModel, ExternalModel, SourceWrapper};
use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::{VisitMut, visit_type_mut};
use syn::{GenericArgument, Ident, PathArguments, ReturnType, Type, TypePath};

pub(super) fn static_requirements(requirements: &TokenStream) -> TokenStream {
    struct Static;
    impl VisitMut for Static {
        fn visit_path_segment_mut(&mut self, segment: &mut syn::PathSegment) {
            if segment.ident == "ProviderContextualValueForms" {
                segment.ident = Ident::new("ProviderValue", segment.ident.span());
                segment.arguments = PathArguments::None;
            } else {
                syn::visit_mut::visit_path_segment_mut(self, segment);
            }
        }
    }
    let mut type_: Type = syn::parse_quote!(#requirements);
    Static.visit_type_mut(&mut type_);
    quote!(#type_)
}

// A custom schema's parameters belong to that declaration, independently of any
// provider function's return-first generic numbering.
pub(super) fn field_type(
    type_: TokenStream,
    parameters: &[Ident],
    support: &TokenStream,
    customs: &[CustomModel],
    externals: &[ExternalModel],
) -> syn::Result<TokenStream> {
    struct Fields<'a> {
        parameters: &'a [Ident],
        support: &'a TokenStream,
        customs: &'a [CustomModel],
        externals: &'a [ExternalModel],
        error: Option<syn::Error>,
    }
    impl VisitMut for Fields<'_> {
        fn visit_type_mut(&mut self, type_: &mut Type) {
            if self.error.is_some() {
                return;
            }
            if let Type::Path(TypePath {
                qself: Some(owner),
                path,
            }) = type_
                && path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "Host")
                && path.segments.iter().any(|segment| {
                    segment.ident == "ProviderValue"
                        || segment.ident == "ProviderTypedValue"
                        || segment.ident == "ProviderContextualValueForms"
                })
            {
                match source_field(
                    &owner.ty,
                    self.parameters,
                    self.support,
                    self.customs,
                    self.externals,
                ) {
                    Ok(field) => *type_ = syn::parse_quote!(#field),
                    Err(error) => self.error = Some(error),
                }
                return;
            }
            visit_type_mut(self, type_);
        }
    }
    let mut type_: Type = syn::parse_quote!(#type_);
    let mut fields = Fields {
        parameters,
        support,
        customs,
        externals,
        error: None,
    };
    fields.visit_type_mut(&mut type_);
    if let Some(error) = fields.error {
        return Err(error);
    }
    Ok(quote!(#type_))
}

fn source_field(
    type_: &Type,
    parameters: &[Ident],
    support: &TokenStream,
    customs: &[CustomModel],
    externals: &[ExternalModel],
) -> syn::Result<TokenStream> {
    if let Type::Path(TypePath { qself: None, path }) = type_
        && let Some(ident) = path.get_ident()
        && let Some(index) = parameters.iter().position(|parameter| parameter == ident)
    {
        let mut position = quote!(#support::HostTypeIndex0);
        for _ in 0..index {
            position = quote!(#support::HostTypeIndexNext<#position>);
        }
        return Ok(quote!(#support::HostCustomTypeArgument<#position>));
    }
    if super::type_syntax::is_type_application_named(type_, "Value")
        && let Some(value) = collection_item(type_, "Value")?
    {
        return source_field(&value, parameters, support, customs, externals);
    }
    if super::type_syntax::is_type_application_named(type_, "List")
        && let Some(value) = collection_item(type_, "List")?
    {
        let value = source_field(&value, parameters, support, customs, externals)?;
        return Ok(quote!(#support::HostListType<#value>));
    }
    match source_wrapper(type_)? {
        SourceWrapper::Result {
            success, failure, ..
        } => {
            let success = source_field(success, parameters, support, customs, externals)?;
            let failure = source_field(failure, parameters, support, customs, externals)?;
            return Ok(quote!(#support::ProviderResult<#success, #failure>));
        }
        SourceWrapper::Option { value, .. } => {
            let value = source_field(value, parameters, support, customs, externals)?;
            return Ok(quote!(#support::ProviderOption<#value>));
        }
        SourceWrapper::Other => {}
    }
    if let Type::Tuple(tuple) = type_
        && !tuple.elems.is_empty()
    {
        let fields = tuple
            .elems
            .iter()
            .map(|type_| source_field(type_, parameters, support, customs, externals))
            .collect::<syn::Result<Vec<_>>>()?;
        let fields = host_type_token_sequence(&fields, support);
        return Ok(quote!(#support::HostTupleType<#fields>));
    }
    if let Type::BareFn(function) = type_ {
        let arguments = function
            .inputs
            .iter()
            .map(|input| source_field(&input.ty, parameters, support, customs, externals))
            .collect::<syn::Result<Vec<_>>>()?;
        let arguments = host_type_token_sequence(&arguments, support);
        let returned = match &function.output {
            ReturnType::Default => quote!(()),
            ReturnType::Type(_, returned) => {
                source_field(returned, parameters, support, customs, externals)?
            }
        };
        return Ok(quote!(#support::HostOpaqueFunctionType<#arguments, #returned>));
    }
    if let Type::Path(TypePath { qself: None, path }) = type_
        && let Some(segment) = path.segments.last()
        && let PathArguments::AngleBracketed(arguments) = &segment.arguments
    {
        let fields = arguments
            .args
            .iter()
            .map(|argument| match argument {
                GenericArgument::Type(type_) => {
                    source_field(type_, parameters, support, customs, externals)
                }
                _ => Err(syn::Error::new_spanned(
                    argument,
                    "nominal source arguments must be types",
                )),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let fields = host_type_token_sequence(&fields, support);
        if let Some((index, _)) = super::custom_value::generic_custom_index(type_, customs) {
            let schema = super::custom_context::schema_type(&customs[index], customs);
            return Ok(quote!(#support::HostCustomType<#schema, #fields>));
        }
        if path.segments.len() == 1
            && let Some(external) = externals
                .iter()
                .find(|external| external.ident == segment.ident)
        {
            let schema = &external.schema;
            return Ok(quote!(#support::HostExternalType<#schema, #fields>));
        }
        let source = instantiate(type_.clone(), parameters, support);
        return Ok(
            quote!(<<#source as #support::ProviderContextualValueForms<__GeamProfile>>::Host as #support::HostNominalCustomField<#fields>>::Field),
        );
    }
    Ok(quote!(<#type_ as #support::ProviderContextualValueForms<__GeamProfile>>::Host))
}

fn instantiate(mut type_: Type, parameters: &[Ident], support: &TokenStream) -> Type {
    struct Parameters<'a> {
        parameters: &'a [Ident],
        support: &'a TokenStream,
    }
    impl VisitMut for Parameters<'_> {
        fn visit_type_mut(&mut self, type_: &mut Type) {
            if let Type::Path(TypePath { qself: None, path }) = type_
                && let Some(ident) = path.get_ident()
                && let Some(index) = self
                    .parameters
                    .iter()
                    .position(|parameter| parameter == ident)
            {
                let support = self.support;
                *type_ = syn::parse_quote!(#support::HostTypeParameter<#index>);
                return;
            }
            visit_type_mut(self, type_);
        }
    }
    Parameters {
        parameters,
        support,
    }
    .visit_type_mut(&mut type_);
    type_
}

#[cfg(test)]
mod tests {
    use super::{field_type, source_field};
    use quote::quote;
    use syn::{Ident, Type, parse_quote};

    #[test]
    fn recursive_schema_fields_propagate_invalid_child_declarations() {
        for (source, expected) in [
            (
                parse_quote!(Value<bool, bool>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(List<bool, bool>),
                "List requires exactly one type argument",
            ),
            (
                parse_quote!(List<Value<bool, bool>>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(Result<Value<bool, bool>, bool>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(Result<bool, Value<bool, bool>>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(Option<Value<bool, bool>>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!((bool, Value<bool, bool>)),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(fn(Value<bool, bool>)),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(fn() -> Value<bool, bool>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(types::Box<Value<bool, bool>>),
                "Value requires exactly one type argument",
            ),
            (
                parse_quote!(Result<bool>),
                "Result requires exactly 2 type arguments",
            ),
        ] {
            assert_eq!(
                source_field(&source, &[], &quote!(support), &[], &[])
                    .unwrap_err()
                    .to_string(),
                expected
            );
        }
        let actual =
            source_field(&parse_quote!((bool, ())), &[], &quote!(support), &[], &[]).unwrap();
        assert_eq!(
            actual.to_string().replace(" , >", " >"),
            quote!(
                support::HostTupleType<
                    support::HostTypeList<
                        <bool as support::ProviderContextualValueForms<__GeamProfile>>::Host,
                        support::HostTypeList<
                            <() as support::ProviderContextualValueForms<__GeamProfile>>::Host,
                            support::HostTypeListEnd,
                        >,
                    >,
                >
            )
            .to_string()
            .replace(" , >", " >")
        );
    }

    #[test]
    fn schema_fields_keep_declaration_order_through_recursive_nominal_arguments() {
        let parameters: [Ident; 2] = [parse_quote!(Left), parse_quote!(Right)];
        let support = quote!(support);
        let cases: [(Type, proc_macro2::TokenStream); 4] = [
            (
                parse_quote!(Result<List<Value<Right>>, Option<Value<Left>>>),
                quote!(
                    support::ProviderResult<
                        support::HostListType<
                            support::HostCustomTypeArgument<
                                support::HostTypeIndexNext<support::HostTypeIndex0>,
                            >,
                        >,
                        support::ProviderOption<
                            support::HostCustomTypeArgument<support::HostTypeIndex0>,
                        >,
                    >
                ),
            ),
            (
                parse_quote!(fn(Value<Left>) -> Value<Right>),
                quote!(
                    support::HostOpaqueFunctionType<
                        support::HostTypeList<
                            support::HostCustomTypeArgument<support::HostTypeIndex0>,
                            support::HostTypeListEnd,
                        >,
                        support::HostCustomTypeArgument<
                            support::HostTypeIndexNext<support::HostTypeIndex0>,
                        >,
                    >
                ),
            ),
            (
                parse_quote!(fn()),
                quote!(support::HostOpaqueFunctionType<support::HostTypeListEnd, ()>),
            ),
            (
                parse_quote!(types::Record<Left, Right>),
                quote!(
                    <<types::Record<
                        support::HostTypeParameter<0usize>,
                        support::HostTypeParameter<1usize>,
                    > as support::ProviderContextualValueForms<__GeamProfile>>::Host as support::HostNominalCustomField<
                        support::HostTypeList<
                            support::HostCustomTypeArgument<support::HostTypeIndex0>,
                            support::HostTypeList<
                                support::HostCustomTypeArgument<
                                    support::HostTypeIndexNext<support::HostTypeIndex0>,
                                >,
                                support::HostTypeListEnd,
                            >,
                        >,
                    >>::Field
                ),
            ),
        ];
        for (source, expected) in cases {
            let actual = source_field(&source, &parameters, &support, &[], &[]).unwrap();
            assert_eq!(
                actual.to_string().replace(" , >", " >"),
                expected.to_string().replace(" , >", " >"),
            );
        }
        let host = quote!(support::HostListType<<Value<Right> as support::ProviderValue>::Host>);
        let actual = field_type(host, &parameters, &support, &[], &[]).unwrap();
        assert_eq!(
            actual.to_string().replace(" , >", " >"),
            quote!(
                support::HostListType<
                    support::HostCustomTypeArgument<
                        support::HostTypeIndexNext<support::HostTypeIndex0>,
                    >,
                >
            )
            .to_string()
            .replace(" , >", " >"),
        );
    }

    #[test]
    fn local_custom_and_external_fields_use_the_declaration_parameter_position() {
        let support = quote!(support);
        let mut items: syn::File = syn::parse2(quote! {
            #[geam::custom(input = RecordInput)]
            enum Record<Item> { Record(Value<Item>) }
        })
        .unwrap();
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
        let mut payload = parse_quote! {
            struct Parcel<Item> { #[geam::stored] value: Stored<Item> }
        };
        let arguments = syn::parse2(quote!(
            name = "Parcel",
            parameters = [Item],
            input = ParcelInput
        ))
        .unwrap();
        let external =
            crate::module::syntax::build_external_model(0, &mut payload, arguments, &support)
                .unwrap();
        let parameters = [parse_quote!(Left), parse_quote!(Right)];
        for (source, expected) in [
            (
                parse_quote!(Record<Right>),
                quote!(
                    support::HostCustomType<
                        __GeamCustomSchema0<__GeamProfile>,
                        support::HostTypeList<
                            support::HostCustomTypeArgument<
                                support::HostTypeIndexNext<support::HostTypeIndex0>,
                            >,
                            support::HostTypeListEnd,
                        >,
                    >
                ),
            ),
            (
                parse_quote!(Parcel<Left>),
                quote!(support::HostExternalType<__GeamExternalSchema0,
                support::HostTypeList<support::HostCustomTypeArgument<support::HostTypeIndex0>, support::HostTypeListEnd>>),
            ),
        ] {
            let actual = source_field(
                &source,
                &parameters,
                &support,
                &customs,
                std::slice::from_ref(&external),
            )
            .unwrap();
            let actual = syn::parse2::<Type>(actual).unwrap();
            let expected = syn::parse2::<Type>(expected).unwrap();
            assert_eq!(
                quote!(#actual).to_string().replace(" , >", " >"),
                quote!(#expected).to_string().replace(" , >", " >"),
            );
        }
    }

    #[test]
    fn invalid_nominal_source_arguments_are_expansion_errors() {
        for source in [
            parse_quote!(types::Record<'static>),
            parse_quote!(types::Record<3>),
            parse_quote!(types::Record<Item = bool>),
        ] {
            let error = source_field(&source, &[], &quote!(support), &[], &[]).unwrap_err();
            assert_eq!(error.to_string(), "nominal source arguments must be types");
        }
        let error = field_type(
            quote!((
                <types::Record<'static> as support::ProviderValue>::Host,
                <Result<bool> as support::ProviderValue>::Host,
            )),
            &[],
            &quote!(support),
            &[],
            &[],
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "nominal source arguments must be types");
    }
}
