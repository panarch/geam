use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Path;

pub(crate) fn support_path(explicit: Option<&Path>) -> syn::Result<TokenStream> {
    if let Some(path) = explicit {
        return Ok(quote!(#path::__macro_support));
    }

    resolved_support_path(crate_name("geam").map_err(|error| error.to_string()))
}

fn resolved_support_path(found: Result<FoundCrate, String>) -> syn::Result<TokenStream> {
    match found {
        Ok(FoundCrate::Itself) => Ok(quote!(crate::__macro_support)),
        Ok(FoundCrate::Name(name)) => {
            let name = format_ident!("{}", name.replace('-', "_"));
            Ok(quote!(::#name::__macro_support))
        }
        Err(error) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("could not resolve the `geam` dependency: {error}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{resolved_support_path, support_path};
    use proc_macro_crate::FoundCrate;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn explicit_crate_path_selects_its_hidden_support_module() {
        assert_eq!(
            support_path(Some(&parse_quote!(renamed_core)))
                .expect("explicit path should resolve")
                .to_string(),
            quote!(renamed_core::__macro_support).to_string(),
        );
    }

    #[test]
    fn dependency_resolution_supports_self_and_renamed_geam_crates() {
        assert_eq!(
            resolved_support_path(Ok(FoundCrate::Itself))
                .expect("self dependency should resolve")
                .to_string(),
            quote!(crate::__macro_support).to_string(),
        );
        assert_eq!(
            resolved_support_path(Ok(FoundCrate::Name("renamed-geam".into())))
                .expect("renamed dependency should resolve")
                .to_string(),
            quote!(::renamed_geam::__macro_support).to_string(),
        );
    }

    #[test]
    fn dependency_resolution_failure_is_an_authoring_diagnostic() {
        assert_eq!(
            resolved_support_path(Err("dependency is missing".into()))
                .expect_err("missing dependency should fail")
                .to_string(),
            "could not resolve the `geam` dependency: dependency is missing",
        );
        assert!(
            support_path(None)
                .expect_err("macro crate has no root geam dependency")
                .to_string()
                .starts_with("could not resolve the `geam` dependency:"),
        );
    }
}
