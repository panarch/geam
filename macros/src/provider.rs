use crate::path::support_path;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Fields, Ident, Item, ItemStruct, LitStr, Path, Token, Type, Visibility, bracketed};

struct ProviderArguments {
    id: Option<LitStr>,
    package: LitStr,
    initialization: ProviderInitialization,
    modules: Vec<Ident>,
    crate_path: Option<Path>,
}

enum ProviderInitialization {
    Unit,
    Default(Type),
    Configured { state: Type, initialize: Path },
}

#[derive(Default)]
struct PartialProviderArguments {
    id: Option<LitStr>,
    package: Option<LitStr>,
    state: Option<Type>,
    initialize: Option<Path>,
    modules: Option<Vec<Ident>>,
    crate_path: Option<Path>,
}

pub(crate) fn expand(arguments: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let arguments = syn::parse2::<ProviderArguments>(arguments)?;
    let item = syn::parse2::<Item>(item)?;
    let Item::Struct(component) = item else {
        return Err(syn::Error::new_spanned(
            item,
            "`#[geam::provider]` must be applied to a struct",
        ));
    };
    validate_component(&component)?;
    let support = support_path(arguments.crate_path.as_ref())?;
    let ProviderArguments {
        id,
        package,
        initialization,
        modules,
        crate_path: _,
    } = arguments;
    let id = id
        .map(|id| quote!(#id))
        .unwrap_or_else(|| quote!(::core::env!("CARGO_PKG_NAME")));
    let (state, initialization) = match initialization {
        ProviderInitialization::Unit => default_initialization(quote!(()), &support),
        ProviderInitialization::Default(state) => {
            let state = quote!(#state);
            default_initialization(state, &support)
        }
        ProviderInitialization::Configured { state, initialize } => (
            quote!(#state),
            quote! {
                #initialize(configuration).map_err(
                    #support::component_initialization_error::<Self>,
                )
            },
        ),
    };
    let module_count = modules.len();
    let store_fields = modules.iter().map(|module| {
        quote! {
            #module: #module::__GeamStores,
        }
    });

    Ok(quote! {
        #component

        #[doc(hidden)]
        #[derive(Default)]
        pub struct Stores {
            #(#store_fields)*
        }

        impl #support::HostProviderComponent for Component {
            const ID: &'static str = #id;
            type Stores = Stores;
            type RunState = #state;
        }

        impl #support::ProviderPackage for Component {
            const PACKAGE: &'static str = #package;
        }

        impl #support::HostProviderComponentInitialization for Component {
            fn initialize(
                configuration: &#support::HostProviderConfiguration,
            ) -> ::core::result::Result<
                Self::RunState,
                #support::HostProviderInitializationError,
            > {
                #initialization
            }
        }

        impl<Profile> #support::HostProviderComponentRegistration<Profile> for Component
        where
            Profile: #support::HostComponentProfile<Self>,
        {
            fn providers() -> ::core::result::Result<
                ::std::vec::Vec<#support::HostProviderModule<Profile>>,
                #support::HostRegistrationError,
            > {
                let mut providers = ::std::vec::Vec::with_capacity(#module_count);
                #(
                    providers.push(#modules::__geam_provider_module::<Profile>()?);
                )*
                ::core::result::Result::Ok(providers)
            }
        }
    })
}

fn default_initialization(state: TokenStream, support: &TokenStream) -> (TokenStream, TokenStream) {
    let initialization = quote! {
        if configuration.is_empty() {
            ::core::result::Result::Ok(
                <Self::RunState as ::core::default::Default>::default(),
            )
        } else {
            ::core::result::Result::Err(
                #support::HostProviderInitializationError::for_component::<Self>(
                    "provider does not accept configuration",
                ),
            )
        }
    };
    (state, initialization)
}

impl Parse for ProviderArguments {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut partial = PartialProviderArguments::default();

        while !input.is_empty() {
            let field = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            match field.to_string().as_str() {
                "id" => set_once(&mut partial.id, input.parse()?, &field)?,
                "package" => set_once(&mut partial.package, input.parse()?, &field)?,
                "state" => set_once(&mut partial.state, input.parse()?, &field)?,
                "initialize" => set_once(&mut partial.initialize, input.parse()?, &field)?,
                "modules" => {
                    let content;
                    bracketed!(content in input);
                    let modules = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect();
                    set_once(&mut partial.modules, modules, &field)?;
                }
                "crate_path" => set_once(&mut partial.crate_path, input.parse()?, &field)?,
                _ => {
                    return Err(syn::Error::new(
                        field.span(),
                        format!("unknown provider argument `{field}`"),
                    ));
                }
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        let modules = required(partial.modules, "modules")?;
        if modules.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "provider `modules` must not be empty",
            ));
        }
        let mut unique = BTreeSet::new();
        for module in &modules {
            if !unique.insert(module.unraw().to_string()) {
                return Err(syn::Error::new(
                    module.span(),
                    format!("duplicate provider module `{module}`"),
                ));
            }
        }

        let initialization = match (partial.state, partial.initialize) {
            (None, None) => ProviderInitialization::Unit,
            (Some(state), None) => ProviderInitialization::Default(state),
            (Some(state), Some(initialize)) => {
                ProviderInitialization::Configured { state, initialize }
            }
            (None, Some(initialize)) => {
                return Err(syn::Error::new_spanned(
                    initialize,
                    "provider `initialize` requires `state`",
                ));
            }
        };

        Ok(Self {
            id: partial.id,
            package: required(partial.package, "package")?,
            initialization,
            modules,
            crate_path: partial.crate_path,
        })
    }
}

fn set_once<Value>(slot: &mut Option<Value>, value: Value, field: &Ident) -> syn::Result<()> {
    if slot.replace(value).is_some() {
        return Err(syn::Error::new(
            field.span(),
            format!("duplicate provider argument `{field}`"),
        ));
    }
    Ok(())
}

fn required<Value>(value: Option<Value>, field: &str) -> syn::Result<Value> {
    value.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("missing required provider argument `{field}`"),
        )
    })
}

fn validate_component(component: &ItemStruct) -> syn::Result<()> {
    if component.ident != "Component" {
        return Err(syn::Error::new(
            component.ident.span(),
            "`#[geam::provider]` must be applied to a struct named `Component`",
        ));
    }
    if !matches!(component.vis, Visibility::Public(_)) {
        return Err(syn::Error::new_spanned(
            &component.vis,
            "provider `Component` must be public",
        ));
    }
    if !matches!(component.fields, Fields::Unit) {
        return Err(syn::Error::new_spanned(
            &component.fields,
            "provider `Component` must be a unit struct",
        ));
    }
    if !component.generics.params.is_empty() || component.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &component.generics,
            "provider `Component` must not have generics",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ProviderArguments, expand};
    use quote::quote;

    fn parse_error(arguments: proc_macro2::TokenStream) -> String {
        syn::parse2::<ProviderArguments>(arguments)
            .err()
            .expect("provider arguments should be rejected")
            .to_string()
    }

    #[test]
    fn provider_arguments_require_package_modules_and_consistent_initialization() {
        assert_eq!(
            parse_error(quote!(id = "counter", modules = [counter])),
            "missing required provider argument `package`",
        );
        assert_eq!(
            parse_error(quote!(package = "counter",)),
            "missing required provider argument `modules`",
        );
        assert_eq!(
            parse_error(quote!(
                package = "counter",
                initialize = initialize,
                modules = [counter]
            )),
            "provider `initialize` requires `state`",
        );
    }

    #[test]
    fn provider_arguments_reject_duplicate_unknown_and_invalid_modules() {
        assert_eq!(
            parse_error(quote!(
                id = "first",
                id = "second",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter]
            )),
            "duplicate provider argument `id`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = []
            )),
            "provider `modules` must not be empty",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter, counter]
            )),
            "duplicate provider module `counter`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter, r#counter]
            )),
            "duplicate provider module `r#counter`",
        );
        assert_eq!(
            parse_error(quote!(unknown = value)),
            "unknown provider argument `unknown`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                state = OtherState,
                initialize = initialize,
                modules = [counter]
            )),
            "duplicate provider argument `state`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                initialize = other,
                modules = [counter]
            )),
            "duplicate provider argument `initialize`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter],
                modules = [other]
            )),
            "duplicate provider argument `modules`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                package = "other",
                state = RunState,
                initialize = initialize,
                modules = [counter]
            )),
            "duplicate provider argument `package`",
        );
        assert_eq!(
            parse_error(quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter],
                crate_path = geam_core,
                crate_path = other_core
            )),
            "duplicate provider argument `crate_path`",
        );
    }

    #[test]
    fn provider_argument_syntax_errors_are_preserved() {
        let cases = [
            (quote!(= "counter"), "expected identifier"),
            (quote!(id "counter"), "expected `=`"),
            (quote!(id = counter), "expected string literal"),
            (
                quote!(id = "counter", package = counter),
                "expected string literal",
            ),
            (
                quote!(id = "counter", state = "RunState"),
                "expected one of: `for`, parentheses, `fn`, `unsafe`, `extern`, identifier, `::`, `<`, `dyn`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime",
            ),
            (
                quote!(id = "counter", initialize = "initialize"),
                "expected identifier",
            ),
            (
                quote!(id = "counter", modules = counter),
                "expected square brackets",
            ),
            (
                quote!(id = "counter", modules = ["counter"]),
                "expected identifier",
            ),
            (
                quote!(id = "counter", crate_path = "geam"),
                "expected identifier",
            ),
            (quote!(id = "counter" package = "counter"), "expected `,`"),
        ];

        for (arguments, expected) in cases {
            assert_eq!(parse_error(arguments), expected);
        }
    }

    #[test]
    fn expansion_preserves_argument_item_and_crate_resolution_errors() {
        assert_eq!(
            expand(
                quote!(id),
                quote!(
                    pub struct Component;
                )
            )
            .expect_err("malformed arguments should fail")
            .to_string(),
            "expected `=`",
        );
        assert_eq!(
            expand(
                quote!(
                    id = "counter",
                    package = "counter",
                    state = RunState,
                    initialize = initialize,
                    modules = [counter],
                    crate_path = geam_core,
                ),
                quote!(pub struct)
            )
            .expect_err("malformed item should fail")
            .to_string(),
            "unexpected end of input, expected identifier",
        );
        let resolution_error = expand(
            quote!(
                id = "counter",
                package = "counter",
                state = RunState,
                initialize = initialize,
                modules = [counter],
            ),
            quote!(
                pub struct Component;
            ),
        )
        .expect_err("missing default crate dependency should fail")
        .to_string();
        assert!(resolution_error.starts_with(
            "could not resolve the `geam` dependency: Could not find `geam` in `dependencies` or `dev-dependencies`"
        ));
    }

    #[test]
    fn provider_target_is_one_public_non_generic_unit_component() {
        let arguments = quote!(
            id = "counter",
            package = "counter",
            state = RunState,
            initialize = initialize,
            modules = [counter],
            crate_path = geam_core,
        );
        assert_eq!(
            expand(
                arguments.clone(),
                quote!(
                    pub enum Component {}
                )
            )
            .expect_err("non-struct component should fail")
            .to_string(),
            "`#[geam::provider]` must be applied to a struct",
        );
        assert_eq!(
            expand(
                arguments.clone(),
                quote!(
                    pub struct Wrong;
                )
            )
            .expect_err("wrong component name should fail")
            .to_string(),
            "`#[geam::provider]` must be applied to a struct named `Component`",
        );
        assert_eq!(
            expand(
                arguments.clone(),
                quote!(
                    struct Component;
                )
            )
            .expect_err("private component should fail")
            .to_string(),
            "provider `Component` must be public",
        );
        assert_eq!(
            expand(
                arguments.clone(),
                quote!(
                    pub struct Component {
                        value: usize,
                    }
                )
            )
            .expect_err("field-backed component should fail")
            .to_string(),
            "provider `Component` must be a unit struct",
        );
        assert_eq!(
            expand(
                arguments,
                quote!(
                    pub struct Component<T>;
                )
            )
            .expect_err("generic component should fail")
            .to_string(),
            "provider `Component` must not have generics",
        );
        let where_only = quote!(
            id = "counter",
            package = "counter",
            state = RunState,
            initialize = initialize,
            modules = [counter],
            crate_path = geam_core,
        );
        assert_eq!(
            expand(
                where_only,
                quote!(
                    pub struct Component
                    where
                        RunState: Send;
                )
            )
            .expect_err("where-constrained component should fail")
            .to_string(),
            "provider `Component` must not have generics",
        );
    }

    #[test]
    fn provider_expansion_preserves_declared_module_order() {
        let expansion = expand(
            quote!(
                id = "ordered",
                package = "sample",
                state = RunState,
                initialize = initialize,
                modules = [first, second],
                crate_path = geam_core,
            ),
            quote!(
                pub struct Component;
            ),
        )
        .expect("provider should expand")
        .to_string();

        let first = expansion
            .find("first :: __geam_provider_module")
            .expect("first registration should be generated");
        let second = expansion
            .find("second :: __geam_provider_module")
            .expect("second registration should be generated");
        assert!(first < second);
        assert!(expansion.contains("pub struct Stores { first : first :: __GeamStores"));
        assert!(expansion.contains("second : second :: __GeamStores"));
        assert!(expansion.contains("type Stores = Stores"));
        assert!(
            expansion
                .contains("impl geam_core :: __macro_support :: ProviderPackage for Component")
        );
        assert!(expansion.contains("const PACKAGE : & 'static str = \"sample\""));
    }

    #[test]
    fn provider_expansion_generates_unit_and_default_initialization() {
        let unit = expand(
            quote!(
                package = "sample",
                modules = [sample],
                crate_path = geam_core,
            ),
            quote!(
                pub struct Component;
            ),
        )
        .expect("minimal provider should expand")
        .to_string();
        assert!(unit.contains("const ID : & 'static str = :: core :: env ! (\"CARGO_PKG_NAME\")"));
        assert!(unit.contains("type RunState = ()"));
        assert!(unit.contains("if configuration . is_empty ()"));
        assert!(unit.contains("< Self :: RunState as :: core :: default :: Default > :: default"));
        assert!(unit.contains("provider does not accept configuration"));

        let default_state = expand(
            quote!(
                package = "sample",
                state = RunState,
                modules = [sample],
                crate_path = geam_core,
            ),
            quote!(
                pub struct Component;
            ),
        )
        .expect("Default-backed provider should expand")
        .to_string();
        assert!(default_state.contains("type RunState = RunState"));
        assert!(default_state.contains("if configuration . is_empty ()"));
    }
}
