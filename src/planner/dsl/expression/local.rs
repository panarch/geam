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
        BoolExprKind, FloatExprKind, IntExprKind, ListExprKind, NilExprKind, StringExprKind,
        TupleExprKind, ValueType,
    };

    #[test]
    fn local_helpers_build_local_get_shapes() {
        assert!(matches!(
            local_int(0, "x").0.kind(),
            IntExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            local_string(0, "x").0.kind(),
            StringExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_float(0, "x").0.kind(),
            FloatExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_bool(0, "x").0.kind(),
            BoolExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_nil(0, "x").0.kind(),
            NilExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_tuple(0, "x", [ValueType::Int]).0.kind(),
            TupleExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_list(0, "x", ValueType::Int).0.kind(),
            ListExprKind::LocalGet { .. },
        ));
    }
}
