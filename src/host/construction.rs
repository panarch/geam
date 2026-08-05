use crate::host::{HostType, HostTypeAt, HostTypeSequence};
use std::marker::PhantomData;

type CallScopedMarker<'call, Type> = PhantomData<fn(&'call ()) -> (&'call (), Type)>;

/// Call-scoped construction capabilities registered for one host function.
///
/// Select an exact registered type with [`HostConstructions::at`] and pass the
/// resulting token to the corresponding [`crate::HostCall`] construction
/// method.
///
/// Construction tokens cannot be created directly.
///
/// ```compile_fail
/// use geam::{HostConstruction, HostListType};
/// use num_bigint::BigInt;
///
/// let _ = HostConstruction::<'static, HostListType<BigInt>>::new();
/// ```
///
/// An index selects exactly the type registered at that position.
///
/// ```compile_fail
/// use ecow::EcoString;
/// use geam::{
///     HostConstruction, HostConstructions, HostListType, HostTypeIndex0, HostTypeList,
///     HostTypeListEnd,
/// };
/// use num_bigint::BigInt;
///
/// type Types = HostTypeList<HostListType<EcoString>, HostTypeListEnd>;
///
/// fn wrong<'call>(
///     constructions: HostConstructions<'call, Types>,
/// ) -> HostConstruction<'call, HostListType<BigInt>> {
///     constructions.at::<HostTypeIndex0>()
/// }
/// ```
///
/// An unregistered position cannot be selected.
///
/// ```compile_fail
/// use geam::{HostConstructions, HostTypeIndex0, HostTypeListEnd};
///
/// fn undeclared(constructions: HostConstructions<'_, HostTypeListEnd>) {
///     let _ = constructions.at::<HostTypeIndex0>();
/// }
/// ```
pub struct HostConstructions<'call, Types: HostTypeSequence> {
    marker: CallScopedMarker<'call, Types>,
}

/// Permission to construct one exact host type during the active host call.
///
/// The token cannot escape the call lifetime that granted it.
///
/// ```compile_fail
/// use ecow::EcoString;
/// use geam::{
///     HostConstruction, HostConstructions, HostListType, HostTypeIndex0, HostTypeList,
///     HostTypeListEnd,
/// };
///
/// type List = HostListType<EcoString>;
/// type Types = HostTypeList<List, HostTypeListEnd>;
///
/// fn escape<'call>(
///     constructions: HostConstructions<'call, Types>,
/// ) -> HostConstruction<'static, List> {
///     constructions.at::<HostTypeIndex0>()
/// }
/// ```
pub struct HostConstruction<'call, Type: HostType> {
    marker: CallScopedMarker<'call, Type>,
}

impl<'call, Types: HostTypeSequence> HostConstructions<'call, Types> {
    pub(crate) fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }

    /// Selects the exact construction type registered at `Index`.
    pub fn at<Index>(&self) -> HostConstruction<'call, <Types as HostTypeAt<Index>>::Type>
    where
        Types: HostTypeAt<Index>,
    {
        HostConstruction {
            marker: PhantomData,
        }
    }
}
