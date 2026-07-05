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

pub(crate) fn list_spread(
    elements: impl IntoIterator<Item = impl Into<Expr>>,
    tail: List,
    element_type: ValueType,
) -> List {
    List(ListExpr::spread(
        elements.into_iter().map(Into::into).collect(),
        tail.into(),
        element_type,
    ))
}

#[cfg(test)]
mod tests {
    use super::{bool_, float, int, list, list_spread, nil, string, tuple};
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, IntExpr, ListExpr, NilExpr, StringExpr, TupleExpr, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn value_helpers_build_typed_exprs() {
        assert_eq!(int(1).0, IntExpr::value(BigInt::from(1)));
        assert_eq!(string("a").0, StringExpr::value("a".into()));
        assert_eq!(float(1.0).0, FloatExpr::value(1.0));
        assert_eq!(bool_(true).0, BoolExpr::value(true));
        assert_eq!(nil().0, NilExpr::value());

        let tuple_elements = [Expr::from(int(1)), Expr::from(string("one"))];
        assert_eq!(
            tuple(tuple_elements.clone()).0,
            TupleExpr::value(
                tuple_elements.to_vec(),
                vec![ValueType::Int, ValueType::String]
            ),
        );

        assert_eq!(
            list([int(1)], ValueType::Int).0,
            ListExpr::value(vec![Expr::from(int(1))], ValueType::Int),
        );
        assert_eq!(
            list_spread([int(1)], list([int(2)], ValueType::Int), ValueType::Int).0,
            ListExpr::spread(
                vec![Expr::from(int(1))],
                ListExpr::value(vec![Expr::from(int(2))], ValueType::Int),
                ValueType::Int,
            ),
        );
    }
}
