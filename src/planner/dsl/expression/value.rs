use super::{Bool, Int, Nil, String};
use crate::plan::{BoolExpr, IntExpr, NilExpr, StringExpr};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) fn int(value: i64) -> Int {
    Int(IntExpr::value(BigInt::from(value)))
}

pub(crate) fn string(value: impl Into<EcoString>) -> String {
    String(StringExpr::value(value.into()))
}

pub(crate) fn bool_(value: bool) -> Bool {
    Bool(BoolExpr::value(value))
}

pub(crate) fn nil() -> Nil {
    Nil(NilExpr::value())
}

#[cfg(test)]
mod tests {
    use super::{bool_, int, nil, string};
    use crate::plan::{Expr, ExprKind};

    #[test]
    fn value_helpers_build_typed_exprs() {
        assert!(matches!(Expr::from(int(1)).kind(), ExprKind::Int(_)));
        assert!(matches!(
            Expr::from(string("a")).kind(),
            ExprKind::String(_),
        ));
        assert!(matches!(Expr::from(bool_(true)).kind(), ExprKind::Bool(_)));
        assert!(matches!(Expr::from(nil()).kind(), ExprKind::Nil(_)));
    }
}
