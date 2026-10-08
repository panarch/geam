use super::{ProviderCallBindings, ProviderConstructions};
use crate::{HostRestoration, HostType};
use std::marker::PhantomData;

/// Selects an explicitly declared restoration target for a provider call.
///
/// Receive this parameter with `#[geam::restore]`, then pass it to
/// [`super::Call::restore_native`] or [`super::Call::restore_dynamic`]. The
/// declaration alone grants no permission to inspect, construct, or invoke a value.
pub struct Restore<Type> {
    type_: PhantomData<fn() -> Type>,
}

impl<Type> Restore<Type> {
    #[doc(hidden)]
    pub fn declaration() -> Self {
        Self { type_: PhantomData }
    }
}

impl<Type> Copy for Restore<Type> {}

impl<Type> Clone for Restore<Type> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Selects the exact restoration proof sealed into a generated call's registration.
#[doc(hidden)]
pub trait ProviderRestorationBinding<Type, Host: HostType>: ProviderCallBindings {
    fn restoration<'call>(
        permissions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> HostRestoration<'call, Host>;
}

#[cfg(test)]
mod tests {
    use super::Restore;

    #[test]
    fn declaration_is_reusable_without_copying_the_restored_type() {
        struct NonClone;
        fn cloned<Type>(declaration: &Restore<Type>) -> Restore<Type> {
            Clone::clone(declaration)
        }
        let declaration = Restore::<NonClone>::declaration();
        let copied = declaration;
        let cloned = cloned(&declaration);
        assert_eq!(std::mem::size_of_val(&copied), 0);
        assert_eq!(std::mem::size_of_val(&cloned), 0);
    }
}
