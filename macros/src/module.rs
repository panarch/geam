use crate::path::support_path;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, FnArg, Ident, Item, ItemFn, ItemMod, LitStr, Meta, Path, ReturnType, Token, Type,
};

struct ModuleArguments {
    path: LitStr,
    crate_path: Option<Path>,
}

#[derive(Default)]
struct PartialModuleArguments {
    path: Option<LitStr>,
    crate_path: Option<Path>,
}

struct FunctionModel {
    ident: Ident,
    argument_types: Vec<Type>,
    return_type: Type,
    has_state: bool,
}

pub(crate) fn expand(arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let arguments = syn::parse2::<ModuleArguments>(arguments)?;
    let mut module = syn::parse2::<ItemMod>(item)?;
    let support = support_path(arguments.crate_path.as_ref())?;
    let inline_module_error =
        syn::Error::new_spanned(&module, "`#[geam::module]` requires an inline Rust module");
    let (_, items) = module.content.as_mut().ok_or(inline_module_error)?;
    let mut functions = Vec::new();
    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        if take_marker(&mut function.attrs, "function")? {
            functions.push(validate_function(function)?);
        }
    }

    let module_path = arguments.path;
    let wrappers = functions.iter().map(|function| {
        let FunctionModel {
            ident,
            argument_types,
            return_type,
            has_state,
        } = function;
        let wrapper = format_ident!("__geam_host_{}", ident.unraw());
        let arguments = (0..argument_types.len())
            .map(|index| format_ident!("__geam_argument_{index}"))
            .collect::<Vec<_>>();
        let call_arguments = has_state
            .then_some(quote!(call.state()))
            .into_iter()
            .chain(arguments.iter().map(|argument| quote!(#argument)))
            .collect::<Vec<_>>();

        quote! {
            fn #wrapper<'__geam_call, Profile>(
                call: #support::HostCall<
                    '__geam_call,
                    Profile,
                    __GeamProvider,
                    #return_type,
                >,
                #(#arguments: #argument_types),*
            ) -> ::core::result::Result<
                #support::HostCallCompletion<'__geam_call, #return_type>,
                #support::HostCallError,
            >
            where
                Profile: #support::HostComponentProfile<super::Component>,
            {
                #[allow(unused_mut)]
                let mut call = call;
                let returned = #ident(#(#call_arguments),*);
                ::core::result::Result::Ok(call.return_value(returned))
            }
        }
    });
    let registrations = functions.iter().map(|function| {
        let ident = &function.ident;
        let name = ident.unraw().to_string();
        let wrapper = format_ident!("__geam_host_{}", ident.unraw());
        let arguments = &function.argument_types;
        let return_type = &function.return_type;
        quote! {
            let provider = provider.with_scoped_function::<
                __GeamProvider,
                (#(#arguments,)*),
                #return_type,
                _,
            >(#name, #wrapper::<Profile>)?;
        }
    });

    items.push(Item::Verbatim(quote! {
        struct __GeamProvider;
    }));
    items.push(Item::Verbatim(quote! {
        impl<Profile> #support::HostProvider<Profile> for __GeamProvider
        where
            Profile: #support::HostComponentProfile<super::Component>,
        {
            type State = <super::Component as #support::HostProviderComponent>::RunState;

            fn project(state: &mut Profile::RunState) -> &mut Self::State {
                <Profile as #support::HostComponentProfile<super::Component>>::component_state(
                    state,
                )
            }
        }
    }));
    for wrapper in wrappers {
        items.push(Item::Verbatim(wrapper));
    }
    let registrar = quote! {
        pub(super) fn __geam_provider_module<Profile>(
            package: &'static str,
        ) -> ::core::result::Result<
            #support::HostProviderModule<Profile>,
            #support::HostRegistrationError,
        >
        where
            Profile: #support::HostComponentProfile<super::Component>,
        {
            let provider = #support::HostProviderModule::<Profile>::new(package, #module_path)?;
            #(#registrations)*
            ::core::result::Result::Ok(provider)
        }
    };
    items.push(Item::Verbatim(registrar));

    Ok(quote!(#module))
}

impl Parse for ModuleArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut partial = PartialModuleArguments::default();
        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            match field.to_string().as_str() {
                "path" => set_once(&mut partial.path, input.parse()?, &field)?,
                "crate_path" => set_once(&mut partial.crate_path, input.parse()?, &field)?,
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        format!("unknown module argument `{field}`"),
                    ));
                }
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            path: partial.path.ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "missing required module argument `path`",
                )
            })?,
            crate_path: partial.crate_path,
        })
    }
}

fn set_once<Value>(slot: &mut Option<Value>, value: Value, field: &Ident) -> syn::Result<()> {
    if slot.replace(value).is_some() {
        return Err(syn::Error::new(
            field.span(),
            format!("duplicate module argument `{field}`"),
        ));
    }
    Ok(())
}

fn take_marker(attributes: &mut Vec<Attribute>, name: &str) -> syn::Result<bool> {
    let mut retained = Vec::with_capacity(attributes.len());
    let mut found = false;
    for attribute in std::mem::take(attributes) {
        let is_marker = attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == name);
        if !is_marker {
            retained.push(attribute);
            continue;
        }
        if !matches!(attribute.meta, Meta::Path(_)) {
            return Err(syn::Error::new_spanned(
                attribute,
                format!("`#[geam::{name}]` does not accept arguments"),
            ));
        }
        if found {
            return Err(syn::Error::new_spanned(
                attribute,
                format!("duplicate `#[geam::{name}]` attribute"),
            ));
        }
        found = true;
    }
    *attributes = retained;
    Ok(found)
}

fn validate_function(function: &mut ItemFn) -> syn::Result<FunctionModel> {
    if function.sig.constness.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.constness,
            "provider functions must not be const",
        ));
    }
    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.asyncness,
            "provider functions must not be async",
        ));
    }
    if function.sig.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            function.sig.unsafety,
            "provider functions must be safe",
        ));
    }
    if function.sig.abi.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "provider functions must use the ordinary Rust ABI",
        ));
    }
    if !function.sig.generics.params.is_empty() || function.sig.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "provider functions must not have generics",
        ));
    }
    let ReturnType::Type(_, return_type) = &function.sig.output else {
        return Err(syn::Error::new_spanned(
            &function.sig.output,
            "provider functions require an explicit return type",
        ));
    };
    let return_type = (**return_type).clone();

    let mut has_state = false;
    let mut argument_types = Vec::new();
    for (index, argument) in function.sig.inputs.iter_mut().enumerate() {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(
                argument,
                "provider functions must be free functions",
            ));
        };
        let is_state = take_marker(&mut argument.attrs, "state")?;
        if is_state {
            if index != 0 {
                return Err(syn::Error::new_spanned(
                    argument,
                    "the `#[geam::state]` parameter must be first",
                ));
            }
            let Type::Reference(reference) = argument.ty.as_ref() else {
                return Err(syn::Error::new_spanned(
                    &argument.ty,
                    "the `#[geam::state]` parameter must be `&mut State`",
                ));
            };
            if reference.mutability.is_none() {
                return Err(syn::Error::new_spanned(
                    &argument.ty,
                    "the `#[geam::state]` parameter must be `&mut State`",
                ));
            }
            has_state = true;
        } else {
            argument_types.push((*argument.ty).clone());
        }
    }
    if argument_types.len() > 7 {
        return Err(syn::Error::new_spanned(
            &function.sig.inputs,
            "provider functions support at most seven source arguments",
        ));
    }

    Ok(FunctionModel {
        ident: function.sig.ident.clone(),
        argument_types,
        return_type,
        has_state,
    })
}

#[cfg(test)]
mod tests {
    use super::{ModuleArguments, expand};
    use quote::quote;

    fn expansion_error(item: proc_macro2::TokenStream) -> String {
        expand(quote!(path = "counter", crate_path = geam_core), item)
            .expect_err("module should be rejected")
            .to_string()
    }

    #[test]
    fn module_arguments_require_one_path_and_reject_unknown_fields() {
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!())
                .err()
                .expect("missing path should fail")
                .to_string(),
            "missing required module argument `path`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(path = "first", path = "second"))
                .err()
                .expect("duplicate path should fail")
                .to_string(),
            "duplicate module argument `path`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(other = "counter"))
                .err()
                .expect("unknown field should fail")
                .to_string(),
            "unknown module argument `other`",
        );
        assert_eq!(
            syn::parse2::<ModuleArguments>(quote!(
                path = "counter",
                crate_path = geam_core,
                crate_path = other_core
            ))
            .err()
            .expect("duplicate crate path should fail")
            .to_string(),
            "duplicate module argument `crate_path`",
        );
    }

    #[test]
    fn module_argument_syntax_errors_are_preserved() {
        let cases = [
            (quote!(= "counter"), "expected identifier"),
            (quote!(path "counter"), "expected `=`"),
            (quote!(path = counter), "expected string literal"),
            (
                quote!(path = "counter", crate_path = "geam"),
                "expected identifier",
            ),
            (
                quote!(path = "counter" crate_path = geam_core),
                "expected `,`",
            ),
        ];

        for (arguments, expected) in cases {
            assert_eq!(
                syn::parse2::<ModuleArguments>(arguments)
                    .err()
                    .expect("module argument syntax should fail")
                    .to_string(),
                expected,
            );
        }
    }

    #[test]
    fn expansion_preserves_argument_item_and_crate_resolution_errors() {
        assert_eq!(
            expand(
                quote!(path),
                quote!(
                    mod counter {}
                )
            )
            .expect_err("malformed arguments should fail")
            .to_string(),
            "expected `=`",
        );
        assert_eq!(
            expand(quote!(path = "counter"), quote!(mod))
                .expect_err("malformed module should fail")
                .to_string(),
            "unexpected end of input, expected identifier",
        );
        let resolution_error = expand(
            quote!(path = "counter"),
            quote!(
                mod counter {}
            ),
        )
        .expect_err("missing default crate dependency should fail")
        .to_string();
        assert!(resolution_error.starts_with(
            "could not resolve the `geam` dependency: Could not find `geam` in `dependencies` or `dev-dependencies`"
        ));
    }

    #[test]
    fn module_must_be_inline() {
        assert_eq!(
            expansion_error(quote!(
                pub struct Counter;
            )),
            "expected `mod`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter;
            )),
            "`#[geam::module]` requires an inline Rust module",
        );
    }

    #[test]
    fn function_forms_are_rejected_at_the_declaration_owner() {
        let cases = [
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        const fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not be const",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        async fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not be async",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        unsafe fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must be safe",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        extern "C" fn next() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must use the ordinary Rust ABI",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next<T>() -> bool {
                            true
                        }
                    }
                ),
                "provider functions must not have generics",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next() -> bool
                        where
                            bool: Copy,
                        {
                            true
                        }
                    }
                ),
                "provider functions must not have generics",
            ),
            (
                quote!(
                    mod counter {
                        #[geam::function]
                        fn next() {}
                    }
                ),
                "provider functions require an explicit return type",
            ),
        ];

        for (item, expected) in cases {
            assert_eq!(expansion_error(item), expected);
        }
    }

    #[test]
    fn state_injection_is_unique_mutable_and_first() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(label: String, #[geam::state] state: &mut RunState) -> String {
                        label
                    }
                }
            )),
            "the `#[geam::state]` parameter must be first",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::state] state: RunState) -> String {
                        String::new()
                    }
                }
            )),
            "the `#[geam::state]` parameter must be `&mut State`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::state] state: &RunState) -> String {
                        String::new()
                    }
                }
            )),
            "the `#[geam::state]` parameter must be `&mut State`",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(#[geam::state(value)] state: &mut RunState) -> String {
                        String::new()
                    }
                }
            )),
            "`#[geam::state]` does not accept arguments",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(
                        #[geam::state]
                        #[geam::state]
                        state: &mut RunState,
                    ) -> String {
                        String::new()
                    }
                }
            )),
            "duplicate `#[geam::state]` attribute",
        );
    }

    #[test]
    fn function_marker_and_arity_diagnostics_are_exact() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function(value)]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "`#[geam::function]` does not accept arguments",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    #[geam::function]
                    fn next() -> bool {
                        true
                    }
                }
            )),
            "duplicate `#[geam::function]` attribute",
        );
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(
                        a: bool,
                        b: bool,
                        c: bool,
                        d: bool,
                        e: bool,
                        f: bool,
                        g: bool,
                        h: bool,
                    ) -> bool {
                        a
                    }
                }
            )),
            "provider functions support at most seven source arguments",
        );
    }

    #[test]
    fn receiver_parameters_are_rejected_as_non_free_function_shape() {
        assert_eq!(
            expansion_error(quote!(
                mod counter {
                    #[geam::function]
                    fn next(self) -> bool {
                        true
                    }
                }
            )),
            "provider functions must be free functions",
        );
    }

    #[test]
    fn expansion_preserves_lexical_function_order_and_ordinary_helpers() {
        let expansion = expand(
            quote!(path = "counter", crate_path = geam_core),
            quote! {
                mod counter {
                    const ENABLED: bool = true;

                    #[inline]
                    fn helper(value: bool) -> bool { value }

                    #[geam::function]
                    fn first(value: bool) -> bool { helper(value) }

                    #[geam::function]
                    fn second(value: bool) -> bool { value }

                    #[geam::function]
                    fn stateful(#[geam::state] state: &mut RunState, value: bool) -> bool { value }
                }
            },
        )
        .expect("module should expand")
        .to_string();

        assert!(expansion.contains("fn helper"));
        assert!(expansion.contains("const ENABLED"));
        assert!(expansion.contains("stateful (call . state ()"));
        let first = expansion
            .find("\"first\"")
            .expect("first registration should exist");
        let second = expansion
            .find("\"second\"")
            .expect("second registration should exist");
        assert!(first < second);
    }
}
