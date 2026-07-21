use super::{Expr, ExprKind, GenericExpr, TupleExpr};
use crate::plan::ParamLocal;

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    local: ParamLocal,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum CallArgStorage<'a> {
    Stored,
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
            ExprKind::Int(_)
            | ExprKind::Float(_)
            | ExprKind::String(_)
            | ExprKind::BitArray(_)
            | ExprKind::UtfCodepoint(_)
            | ExprKind::Bool(_)
            | ExprKind::Nil(_)
            | ExprKind::List(_)
            | ExprKind::Function(_) => CallArgStorage::Stored,
        }
    }

    #[cfg(test)]
    pub(crate) fn parameter_shape(&self) -> crate::plan::ValueShape {
        self.value.shape().clone()
    }
}

impl CaptureArg {
    pub(crate) fn new(local: ParamLocal) -> Self {
        Self { local }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }
}

#[cfg(test)]
mod tests {
    use super::{CallArg, CallArgStorage, CaptureArg, PotentiallyUninhabitedCallArg};
    use crate::plan::{
        CustomConstructorRefinement, CustomExpr, CustomLocal, CustomLocalId, CustomTypeName,
        CustomValueShape, Expr, FunctionExpr, FunctionReference, FunctionShape, IntExpr, ListExpr,
        TupleExpr, TypeParameterId, ValueShape, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn call_arguments_own_expressions_and_capture_arguments_own_typed_locals() {
        let value = Expr::int(IntExpr::value(BigInt::from(1)));
        let local = crate::plan::ParamLocal::int(crate::plan::IntLocalId(2));

        assert_eq!(CallArg::new(value.clone()).value(), &value);
        assert_eq!(CaptureArg::new(local.clone()).local(), &local);
        assert_eq!(CallArg::new(value).parameter_shape(), ValueShape::Int);
    }

    #[test]
    fn call_argument_storage_distinguishes_stored_values() {
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

        assert_eq!(list.storage(), CallArgStorage::Stored);
        assert_eq!(function.storage(), CallArgStorage::Stored);
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

        let tuple = TupleExpr::value(
            vec![Expr::generic(generic)],
            vec![crate::plan::ValueType::Parameter(parameter)],
        );
        let argument = CallArg::new(Expr::tuple(tuple.clone()));
        assert_eq!(
            argument.storage(),
            CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Tuple(&tuple)),
        );

        let custom = CustomExpr::local_get(
            CustomLocal::from_shape(
                CustomLocalId(0),
                CustomValueShape::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    vec![ValueShape::Parameter(parameter)],
                    CustomConstructorRefinement::Exact(0),
                ),
            ),
            "boxed".into(),
        );
        let argument = CallArg::new(Expr::custom(custom.clone()));
        assert_eq!(
            argument.storage(),
            CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Custom(&custom)),
        );
    }
}
