use super::{Bool, Float, Int, List, Nil, String, Tuple};
use crate::plan::ValueType;
use crate::plan::{BoolExpr, Expr, FloatExpr, IntExpr, ListExpr, NilExpr, StringExpr, TupleExpr};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) fn int(value: i64) -> Int {
    Int(IntExpr::value(BigInt::from(value)))
}

pub(crate) fn string(value: impl Into<EcoString>) -> String {
    String(StringExpr::value(value.into()))
}

pub(crate) fn float(value: f64) -> Float {
    Float(FloatExpr::value(value))
}

pub(crate) fn bool_(value: bool) -> Bool {
    Bool(BoolExpr::value(value))
}

pub(crate) fn nil() -> Nil {
    Nil(NilExpr::value())
}

pub(crate) fn tuple(elements: impl IntoIterator<Item = impl Into<Expr>>) -> Tuple {
    let elements = elements.into_iter().map(Into::into).collect::<Vec<_>>();
    let type_ = elements.iter().map(Expr::value_type).collect();

    Tuple(TupleExpr::value(elements, type_))
}

pub(crate) fn list(
    elements: impl IntoIterator<Item = impl Into<Expr>>,
    element_type: ValueType,
) -> List {
    List(ListExpr::value(
        elements.into_iter().map(Into::into).collect(),
        element_type,
    ))
}

#[cfg(test)]
mod tests {
    use super::{bool_, float, int, list, nil, string, tuple};
    use crate::plan::ValueType;
    use crate::plan::{Expr, ExprKind};

    #[test]
    fn value_helpers_build_typed_exprs() {
        assert!(matches!(Expr::from(int(1)).kind(), ExprKind::Int(_)));
        assert!(matches!(
            Expr::from(string("a")).kind(),
            ExprKind::String(_),
        ));
        assert!(matches!(Expr::from(float(1.0)).kind(), ExprKind::Float(_)));
        assert!(matches!(Expr::from(bool_(true)).kind(), ExprKind::Bool(_)));
        assert!(matches!(Expr::from(nil()).kind(), ExprKind::Nil(_)));
        assert!(matches!(
            Expr::from(tuple([Expr::from(int(1)), Expr::from(string("one"))])).kind(),
            ExprKind::Tuple(_),
        ));
        assert!(matches!(
            Expr::from(list([int(1)], ValueType::Int)).kind(),
            ExprKind::List(_),
        ));
    }
}
