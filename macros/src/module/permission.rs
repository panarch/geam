use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Type;
use syn::ext::IdentExt;

pub(super) fn restore_parameter(type_: &Type) -> syn::Result<Type> {
    let Some((target, _)) = super::type_syntax::collection_item_with_path(type_, "Restore")? else {
        return Err(syn::Error::new_spanned(
            type_,
            "restore parameters require `Restore<T>`",
        ));
    };
    Ok(target)
}

pub(super) fn restoration_host(
    target: &Type,
    return_type: &TokenStream,
    support: &TokenStream,
) -> TokenStream {
    quote!(<#target as #support::ProviderDynamicInput<__GeamProfile, __GeamProvider, #return_type>>::Host)
}

pub(super) fn bindings_type(function: &super::FunctionModel, profile: &TokenStream) -> TokenStream {
    let name = format_ident!("__GeamPermissions_{}", function.ident.unraw());
    let parameters = function.generics.iter().map(|generic| &generic.ident);
    quote!(#name<#profile, #(#parameters),*>)
}

pub(super) struct Bindings<'a> {
    pub(super) function: &'a super::FunctionModel,
    pub(super) requirements: &'a TokenStream,
    pub(super) offset: usize,
    pub(super) count: usize,
    pub(super) bounds: &'a [TokenStream],
    pub(super) support: &'a TokenStream,
    pub(super) flavor: super::InputOwnership,
    pub(super) return_type: &'a TokenStream,
}

impl Bindings<'_> {
    pub(super) fn generate(self) -> TokenStream {
        let Self {
            function,
            requirements,
            offset,
            count,
            bounds,
            support,
            flavor,
            return_type,
        } = self;
        let mode = super::callable::capture_mode(flavor, support);
        let name = format_ident!("__GeamPermissions_{}", function.ident.unraw());
        let parameters = function
            .generics
            .iter()
            .map(|generic| &generic.ident)
            .collect::<Vec<_>>();
        let type_ = bindings_type(function, &quote!(__GeamProfile));
        let selections = function.permissions.iter().enumerate().map(|(position, permission)| {
            let index = super::function::provider_requirement_index(offset + position, count, support);
            match permission {
                super::FunctionPermission::Factory(factory) => {
                    let required = quote!(<#factory as #support::ProviderFactoryCodec<__GeamProfile, #mode>>::Requirements);
                    quote! {
                        impl<__GeamProfile, #(#parameters,)*> #support::ProviderFactoryBinding<#factory, #required> for #type_
                        where
                            __GeamProfile: __GeamModuleProfile,
                            #(#parameters: #support::ProviderValue + 'static,)*
                            #(#bounds,)*
                        {
                            fn select<'call>(proof: &#support::ProviderConstructions<'call, Self::Requirements>) -> #support::ProviderConstructions<'call, #required> {
                                proof.select::<#index>()
                            }
                        }
                    }
                }
                super::FunctionPermission::Restore(target) => {
                    let host = restoration_host(target, return_type, support);
                    quote! {
                        impl<__GeamProfile, #(#parameters,)*> #support::ProviderRestorationBinding<#target, #host> for #type_
                        where
                            __GeamProfile: __GeamModuleProfile,
                            #(#parameters: #support::ProviderValue + 'static,)*
                            #(#bounds,)*
                        {
                            fn restoration<'call>(proof: &#support::ProviderConstructions<'call, Self::Requirements>) -> #support::HostRestoration<'call, #host> {
                                proof.select::<#index>().token().restoration()
                            }
                        }
                    }
                }
            }
        });
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            struct #name<__GeamProfile, #(#parameters,)*>(::core::marker::PhantomData<fn() -> (__GeamProfile, #(#parameters,)*)>);
            impl<__GeamProfile, #(#parameters,)*> #support::ProviderCallBindings for #type_
            where
                __GeamProfile: __GeamModuleProfile,
                #(#parameters: #support::ProviderValue + 'static,)*
                #(#bounds,)*
            { type Requirements = #requirements; type CaptureMode = #mode; }
            #(#selections)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::restore_parameter;
    use crate::module;
    use quote::quote;

    #[test]
    fn restoration_parameters_require_one_explicit_target() {
        assert_eq!(
            restore_parameter(&syn::parse_quote!(Restore<types::Envelope>)).unwrap(),
            syn::parse_quote!(types::Envelope)
        );
        assert_eq!(
            restore_parameter(&syn::parse_quote!(bool))
                .unwrap_err()
                .to_string(),
            "restore parameters require `Restore<T>`"
        );
        assert_eq!(
            restore_parameter(&syn::parse_quote!(Restore<bool, bool>))
                .unwrap_err()
                .to_string(),
            "Restore requires exactly one type argument"
        );
    }

    #[test]
    fn restoration_markers_require_a_mutable_call_and_precede_source_arguments() {
        for arguments in [
            quote!(#[geam::restore] restore: Restore<bool>),
            quote!(#[geam::call] call: &Call<()>, #[geam::restore] restore: Restore<bool>),
            quote!(#[geam::call] call: &mut Call<()>, value: bool, #[geam::restore] restore: Restore<bool>),
        ] {
            let error = module::expand(
                quote!(path = "native", crate_path = geam_core),
                quote!(mod native { #[geam::function] fn restore(#arguments) -> bool { true } }),
            )
            .unwrap_err();
            assert_eq!(
                error.to_string(),
                "permission parameters must follow a mutable Call and precede source arguments"
            );
        }
        let error = module::expand(
            quote!(path = "native", crate_path = geam_core),
            quote!(
                mod native {
                    #[geam::function]
                    fn restore(
                        #[geam::call] call: &mut Call<()>,
                        #[geam::restore] one: Restore<bool>,
                        #[geam::restore] two: Restore<bool>,
                    ) -> bool {
                        true
                    }
                }
            ),
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "duplicate permission declaration parameter"
        );
    }
}
