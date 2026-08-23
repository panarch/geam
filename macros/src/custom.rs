use proc_macro2::TokenStream;
use syn::Item;

pub(crate) fn expand(_arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<Item>(item)?;
    let message = if matches!(item, Item::Enum(_)) {
        "`#[geam::custom]` must be inside a `#[geam::module]` inline module"
    } else {
        "`#[geam::custom]` must be applied to an enum inside a `#[geam::module]` inline module"
    };
    Err(syn::Error::new(proc_macro2::Span::call_site(), message))
}

#[cfg(test)]
mod tests {
    use super::expand;
    use quote::quote;

    #[test]
    fn custom_attribute_rejects_standalone_and_non_enum_targets() {
        assert_eq!(
            expand(
                quote!(input = StatusInput),
                quote! {
                    enum Status {}
                },
            )
            .expect_err("standalone custom enums should fail")
            .to_string(),
            "`#[geam::custom]` must be inside a `#[geam::module]` inline module",
        );
        assert_eq!(
            expand(
                quote!(input = StatusInput),
                quote! {
                    struct Status;
                },
            )
            .expect_err("custom structs should fail")
            .to_string(),
            "`#[geam::custom]` must be applied to an enum inside a `#[geam::module]` inline module",
        );
        assert_eq!(
            expand(quote!(input = StatusInput), quote!(enum))
                .expect_err("malformed items should preserve their parser error")
                .to_string(),
            "unexpected end of input, expected identifier",
        );
    }
}
