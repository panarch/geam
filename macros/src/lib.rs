use proc_macro::TokenStream;

mod external;
mod module;
mod path;
mod provider;

#[proc_macro_attribute]
pub fn provider(arguments: TokenStream, item: TokenStream) -> TokenStream {
    provider::expand(arguments.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn module(arguments: TokenStream, item: TokenStream) -> TokenStream {
    module::expand(arguments.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn external(arguments: TokenStream, item: TokenStream) -> TokenStream {
    external::expand(arguments.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn function(_arguments: TokenStream, item: TokenStream) -> TokenStream {
    let mut error = syn::Error::new(
        proc_macro2::Span::call_site(),
        "`#[geam::function]` must be inside a `#[geam::module]` inline module",
    )
    .into_compile_error();
    error.extend(proc_macro2::TokenStream::from(item));
    error.into()
}
