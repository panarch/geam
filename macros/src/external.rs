use proc_macro2::TokenStream;
use syn::Item;

pub(crate) fn expand(_arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<Item>(item)?;
    let message = if matches!(item, Item::Struct(_)) {
        "`#[geam::external]` must be inside a `#[geam::module]` inline module"
    } else {
        "`#[geam::external]` must be applied to a struct inside a `#[geam::module]` inline module"
    };
    Err(syn::Error::new(proc_macro2::Span::call_site(), message))
}

#[cfg(test)]
mod tests {
    use super::expand;
    use quote::quote;

    #[test]
    fn external_attribute_rejects_standalone_and_non_struct_targets() {
        assert_eq!(
            expand(
                quote!(name = "Metrics"),
                quote!(
                    struct Metrics;
                )
            )
            .expect_err("standalone external structs should fail")
            .to_string(),
            "`#[geam::external]` must be inside a `#[geam::module]` inline module",
        );
        assert_eq!(
            expand(
                quote!(name = "Metrics"),
                quote!(
                    enum Metrics {}
                )
            )
            .expect_err("external enums should fail")
            .to_string(),
            "`#[geam::external]` must be applied to a struct inside a `#[geam::module]` inline module",
        );
        assert_eq!(
            expand(quote!(name = "Metrics"), quote!(struct))
                .expect_err("malformed items should preserve their parser error")
                .to_string(),
            "unexpected end of input, expected identifier",
        );
    }
}
