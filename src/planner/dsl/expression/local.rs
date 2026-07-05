use super::{Bool, Float, Int, List, Nil, String, Tuple};
use crate::plan::{
    BoolExpr, BoolLocalId, FloatExpr, FloatLocalId, IntExpr, IntLocalId, ListExpr, ListLocalId,
    NilExpr, NilLocalId, StringExpr, StringLocalId, TupleExpr, TupleLocalId, ValueType,
};
use ecow::EcoString;

pub(crate) fn local_int(index: usize, name: impl Into<EcoString>) -> Int {
    Int(IntExpr::local_get(IntLocalId(index), name.into()))
}

pub(crate) fn local_string(index: usize, name: impl Into<EcoString>) -> String {
    String(StringExpr::local_get(StringLocalId(index), name.into()))
}

pub(crate) fn local_float(index: usize, name: impl Into<EcoString>) -> Float {
    Float(FloatExpr::local_get(FloatLocalId(index), name.into()))
}

pub(crate) fn local_bool(index: usize, name: impl Into<EcoString>) -> Bool {
    Bool(BoolExpr::local_get(BoolLocalId(index), name.into()))
}

pub(crate) fn local_nil(index: usize, name: impl Into<EcoString>) -> Nil {
    Nil(NilExpr::local_get(NilLocalId(index), name.into()))
}

pub(crate) fn local_tuple(
    index: usize,
    name: impl Into<EcoString>,
    type_: impl IntoIterator<Item = ValueType>,
) -> Tuple {
    Tuple(TupleExpr::local_get(
        TupleLocalId(index),
        name.into(),
        type_.into_iter().collect(),
    ))
}

pub(crate) fn local_list(
    index: usize,
    name: impl Into<EcoString>,
    element_type: ValueType,
) -> List {
    List(ListExpr::local_get(
        ListLocalId(index),
        name.into(),
        element_type,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        local_bool, local_float, local_int, local_list, local_nil, local_string, local_tuple,
    };
    use crate::plan::{
        BoolExpr, BoolLocalId, FloatExpr, FloatLocalId, IntExpr, IntLocalId, ListExpr, ListLocalId,
        NilExpr, NilLocalId, StringExpr, StringLocalId, TupleExpr, TupleLocalId, ValueType,
    };

    #[test]
    fn local_helpers_build_local_get_shapes() {
        assert_eq!(
            local_int(0, "x").0,
            IntExpr::local_get(IntLocalId(0), "x".into()),
        );
        assert_eq!(
            local_string(1, "name").0,
            StringExpr::local_get(StringLocalId(1), "name".into()),
        );
        assert_eq!(
            local_float(2, "ratio").0,
            FloatExpr::local_get(FloatLocalId(2), "ratio".into()),
        );
        assert_eq!(
            local_bool(3, "ok").0,
            BoolExpr::local_get(BoolLocalId(3), "ok".into()),
        );
        assert_eq!(
            local_nil(4, "done").0,
            NilExpr::local_get(NilLocalId(4), "done".into()),
        );
        assert_eq!(
            local_tuple(5, "pair", [ValueType::Int, ValueType::String]).0,
            TupleExpr::local_get(
                TupleLocalId(5),
                "pair".into(),
                vec![ValueType::Int, ValueType::String],
            ),
        );
        assert_eq!(
            local_list(6, "values", ValueType::Int).0,
            ListExpr::local_get(ListLocalId(6), "values".into(), ValueType::Int),
        );
    }
}
