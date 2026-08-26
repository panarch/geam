use super::{ExternalModel, SourceWrapper, SourceWrapperArguments};
use syn::{GenericArgument, PathArguments, Type, TypePath};

pub(super) fn collection_item(type_: &Type, name: &str) -> syn::Result<Option<Type>> {
    Ok(collection_item_with_path(type_, name)?.map(|(item, _)| item))
}

pub(super) fn collection_item_with_path(
    type_: &Type,
    name: &str,
) -> syn::Result<Option<(Type, TypePath)>> {
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

pub(super) fn source_wrapper(type_: &Type) -> syn::Result<SourceWrapper<'_>> {
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

pub(super) fn host_result_value(type_: &Type) -> syn::Result<Option<(Type, TypePath)>> {
    collection_item_with_path(type_, "HostResult")
}

pub(super) fn is_type_named(type_: &Type, name: &str) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path })
            if path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

pub(super) fn is_collection(type_: &Type, name: &str) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path })
            if path.segments.last().is_some_and(|segment| segment.ident == name)
    )
}

pub(super) fn is_type_application_named(type_: &Type, name: &str) -> bool {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return false;
    };
    path.segments.last().is_some_and(|segment| {
        segment.ident == name && matches!(segment.arguments, PathArguments::AngleBracketed(_))
    })
}

pub(super) fn is_non_empty_tuple(type_: &Type) -> bool {
    matches!(type_, Type::Tuple(tuple) if !tuple.elems.is_empty())
}

pub(super) fn is_qualified_type_path(type_: &Type) -> bool {
    matches!(
        type_,
        Type::Path(TypePath { qself: None, path }) if path.segments.len() > 1
    )
}

pub(super) fn is_declared_provider_type(type_: &Type) -> syn::Result<bool> {
    is_advanced_external(type_).map(|external| external | is_qualified_type_path(type_))
}

pub(super) fn is_advanced_external(type_: &Type) -> syn::Result<bool> {
    if !is_type_application_named(type_, "External") {
        return Ok(false);
    }
    collection_item(type_, "External")?;
    Ok(true)
}

pub(super) fn external_type<'external>(
    type_: &Type,
    externals: &'external [ExternalModel],
) -> Option<&'external ExternalModel> {
    let Type::Path(TypePath { qself: None, path }) = type_ else {
        return None;
    };
    let ident = path.get_ident()?;
    externals.iter().find(|external| &external.ident == ident)
}
