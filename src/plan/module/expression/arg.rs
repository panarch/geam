use super::{Expr, ExprKind, GenericExpr, TupleExpr};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    value: Expr,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum CallArgStorage<'a> {
    Stored(crate::plan::ValueStorageShape),
    PotentiallyUninhabited(PotentiallyUninhabitedCallArg<'a>),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum PotentiallyUninhabitedCallArg<'a> {
    Generic(&'a GenericExpr),
    Tuple(&'a TupleExpr),
    Custom(&'a super::CustomExpr),
}

impl CallArg {
    pub(crate) fn new(value: Expr) -> Self {
        Self { value }
    }

    pub(crate) fn value(&self) -> &Expr {
        &self.value
    }

    pub(crate) fn storage(&self) -> CallArgStorage<'_> {
        use crate::plan::ValueStorageShape as S;

        match self.value.kind() {
            ExprKind::Generic(value) => CallArgStorage::PotentiallyUninhabited(
                PotentiallyUninhabitedCallArg::Generic(value),
            ),
            ExprKind::Tuple(value) => {
                CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Tuple(value))
            }
            ExprKind::Custom(value) => {
                CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Custom(value))
            }
            ExprKind::Int(_) => CallArgStorage::Stored(S::Int),
            ExprKind::Float(_) => CallArgStorage::Stored(S::Float),
            ExprKind::String(_) => CallArgStorage::Stored(S::String),
            ExprKind::BitArray(_) => CallArgStorage::Stored(S::BitArray),
            ExprKind::UtfCodepoint(_) => CallArgStorage::Stored(S::UtfCodepoint),
            ExprKind::Bool(_) => CallArgStorage::Stored(S::Bool),
            ExprKind::Nil(_) => CallArgStorage::Stored(S::Nil),
            ExprKind::List(value) => {
                CallArgStorage::Stored(S::List(Box::new(value.item_shape().clone())))
            }
            ExprKind::Function(value) => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn parameter_shape(&self) -> crate::plan::ValueShape {
        self.value.shape().clone()
    }
}

impl CaptureArg {
    pub(crate) fn new(value: Expr) -> Self {
        Self { value }
    }

    pub(crate) fn value(&self) -> &Expr {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::{CallArg, CallArgStorage, CaptureArg, PotentiallyUninhabitedCallArg};
    use crate::plan::{
        Expr, FunctionExpr, FunctionReference, FunctionShape, IntExpr, ListExpr, TypeParameterId,
        ValueShape, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn call_and_capture_arguments_only_own_source_expressions() {
        let value = Expr::int(IntExpr::value(BigInt::from(1)));

        assert_eq!(CallArg::new(value.clone()).value(), &value);
        assert_eq!(CaptureArg::new(value.clone()).value(), &value);
        assert_eq!(CallArg::new(value).parameter_shape(), ValueShape::Int);
    }

    #[test]
    fn call_argument_storage_preserves_exact_source_shape() {
        let item_shape = ValueShape::Parameter(TypeParameterId(0));
        let list = CallArg::new(Expr::list(
            ListExpr::try_value(
                Vec::new(),
                crate::plan::ValueType::Parameter(TypeParameterId(0)),
            )
            .expect("an empty parameter list has no mismatching element"),
        ));
        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::Int);
        let function = CallArg::new(Expr::function(FunctionExpr::reference(
            FunctionReference::new(monomorphic_function_instantiation(
                0,
                function_shape.clone(),
            )),
        )));

        assert_eq!(
            list.storage(),
            CallArgStorage::Stored(crate::plan::ValueStorageShape::List(Box::new(item_shape))),
        );
        assert_eq!(
            function.storage(),
            CallArgStorage::Stored(crate::plan::ValueStorageShape::Function(Box::new(
                function_shape,
            ))),
        );
    }

    #[test]
    fn potentially_uninhabited_arguments_keep_their_typed_source() {
        let parameter = TypeParameterId(0);
        let generic = crate::plan::GenericExpr::panic(
            parameter,
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
        );
        let argument = CallArg::new(Expr::generic(generic.clone()));

        assert_eq!(
            argument.storage(),
            CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Generic(
                &generic,
            )),
        );
    }
}
