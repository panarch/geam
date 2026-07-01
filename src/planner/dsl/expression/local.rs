use super::{Bool, Int, Nil, String};
use crate::plan::{
    BoolExpr, BoolLocalId, IntExpr, IntLocalId, NilExpr, NilLocalId, StringExpr, StringLocalId,
};
use ecow::EcoString;

pub(crate) fn local_int(index: usize, name: impl Into<EcoString>) -> Int {
    Int(IntExpr::local_get(IntLocalId(index), name.into()))
}

pub(crate) fn local_string(index: usize, name: impl Into<EcoString>) -> String {
    String(StringExpr::local_get(StringLocalId(index), name.into()))
}

pub(crate) fn local_bool(index: usize, name: impl Into<EcoString>) -> Bool {
    Bool(BoolExpr::local_get(BoolLocalId(index), name.into()))
}

pub(crate) fn local_nil(index: usize, name: impl Into<EcoString>) -> Nil {
    Nil(NilExpr::local_get(NilLocalId(index), name.into()))
}

#[cfg(test)]
mod tests {
    use super::{local_bool, local_int, local_nil, local_string};
    use crate::plan::{BoolExprKind, IntExprKind, NilExprKind, StringExprKind};

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
            local_bool(0, "x").0.kind(),
            BoolExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_nil(0, "x").0.kind(),
            NilExprKind::LocalGet { .. },
        ));
    }
}
