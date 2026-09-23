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
/// use geam_core::{HostConstruction, HostListType};
/// use num_bigint::BigInt;
///
/// let _ = HostConstruction::<'static, HostListType<BigInt>>::new();
/// ```
///
/// An index selects exactly the type registered at that position.
///
/// ```compile_fail
/// use crate::StringValue;
/// use geam_core::{
///     HostConstruction, HostConstructions, HostListType, HostTypeIndex0, HostTypeList,
///     HostTypeListEnd,
/// };
/// use num_bigint::BigInt;
///
/// type Types = HostTypeList<HostListType<StringValue>, HostTypeListEnd>;
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
/// use geam_core::{HostConstructions, HostTypeIndex0, HostTypeListEnd};
///
/// fn undeclared(constructions: HostConstructions<'_, HostTypeListEnd>) {
///     let _ = constructions.at::<HostTypeIndex0>();
/// }
/// ```
pub struct HostConstructions<'call, Types: HostTypeSequence> {
    callable_base: usize,
    marker: CallScopedMarker<'call, Types>,
}

/// Permission to construct one exact host type during the active host call.
///
/// The token cannot escape the call lifetime that granted it.
///
/// ```compile_fail
/// use crate::StringValue;
/// use geam_core::{
///     HostConstruction, HostConstructions, HostListType, HostTypeIndex0, HostTypeList,
///     HostTypeListEnd,
/// };
///
/// type List = HostListType<StringValue>;
/// type Types = HostTypeList<List, HostTypeListEnd>;
///
/// fn escape<'call>(
///     constructions: HostConstructions<'call, Types>,
/// ) -> HostConstruction<'static, List> {
///     constructions.at::<HostTypeIndex0>()
/// }
/// ```
pub struct HostConstruction<'call, Type: HostType> {
    pub(super) callable_index: usize,
    marker: CallScopedMarker<'call, Type>,
}

impl<'call, Types: HostTypeSequence> HostConstructions<'call, Types> {
    pub(crate) fn new() -> Self {
        Self::with_base(0)
    }

    pub(crate) fn with_base(callable_base: usize) -> Self {
        Self {
            callable_base,
            marker: PhantomData,
        }
    }

    pub(crate) fn callable_base(&self) -> usize {
        self.callable_base
    }

    /// Selects the exact construction type registered at `Index`.
    pub fn at<Index>(&self) -> HostConstruction<'call, <Types as HostTypeAt<Index>>::Type>
    where
        Types: HostTypeAt<Index>,
    {
        HostConstruction {
            callable_index: self.callable_base
                + crate::host::type_::construction_callable_index::<Types, Index>(),
            marker: PhantomData,
        }
    }
}
