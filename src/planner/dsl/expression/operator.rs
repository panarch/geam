use super::{Bool, Float, Int, String};
use crate::plan::{BoolExpr, Expr, FloatExpr, IntExpr, StringExpr};

pub(crate) fn equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::equal(left.into(), right.into()))
}

pub(crate) fn not_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::not_equal(left.into(), right.into()))
}

impl Int {
    pub(crate) fn add_int(self, right: Self) -> Self {
        Self(IntExpr::add(self.into(), right.into()))
    }

    pub(crate) fn sub_int(self, right: Self) -> Self {
        Self(IntExpr::sub(self.into(), right.into()))
    }

    pub(crate) fn mult_int(self, right: Self) -> Self {
        Self(IntExpr::mult(self.into(), right.into()))
    }

    pub(crate) fn div_int(self, right: Self) -> Self {
        Self(IntExpr::div(self.into(), right.into()))
    }

    pub(crate) fn remainder_int(self, right: Self) -> Self {
        Self(IntExpr::remainder(self.into(), right.into()))
    }

    pub(crate) fn lt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lt_int(self.into(), right.into()))
    }

    pub(crate) fn lte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lte_int(self.into(), right.into()))
    }

    pub(crate) fn gt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gt_int(self.into(), right.into()))
    }

    pub(crate) fn gte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gte_int(self.into(), right.into()))
    }

    pub(crate) fn negate_int(self) -> Self {
        Self(IntExpr::negate(self.into()))
    }
}

impl Float {
    pub(crate) fn add_float(self, right: Self) -> Self {
        Self(FloatExpr::add(self.into(), right.into()))
    }

    pub(crate) fn sub_float(self, right: Self) -> Self {
        Self(FloatExpr::sub(self.into(), right.into()))
    }

    pub(crate) fn mult_float(self, right: Self) -> Self {
        Self(FloatExpr::mult(self.into(), right.into()))
    }

    pub(crate) fn div_float(self, right: Self) -> Self {
        Self(FloatExpr::div(self.into(), right.into()))
    }

    pub(crate) fn lt_float(self, right: Self) -> Bool {
        Bool(BoolExpr::lt_float(self.into(), right.into()))
    }

    pub(crate) fn lte_float(self, right: Self) -> Bool {
        Bool(BoolExpr::lte_float(self.into(), right.into()))
    }

    pub(crate) fn gt_float(self, right: Self) -> Bool {
        Bool(BoolExpr::gt_float(self.into(), right.into()))
    }

    pub(crate) fn gte_float(self, right: Self) -> Bool {
        Bool(BoolExpr::gte_float(self.into(), right.into()))
    }
}

impl String {
    pub(crate) fn concatenate(self, right: Self) -> Self {
        Self(StringExpr::concatenate(self.into(), right.into()))
    }
}

impl Bool {
    pub(crate) fn and_bool(self, right: Self) -> Self {
        Self(BoolExpr::and(self.into(), right.into()))
    }

    pub(crate) fn or_bool(self, right: Self) -> Self {
        Self(BoolExpr::or(self.into(), right.into()))
    }

    pub(crate) fn negate_bool(self) -> Self {
        Self(BoolExpr::not(self.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{equal, not_equal};
    use crate::plan::{BoolExprKind, FloatExprKind, IntExprKind, StringExprKind};
    use crate::planner::dsl::expression::{bool_, float, int, string};

    #[test]
    fn int_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            int(1).add_int(int(2)).0.kind(),
            IntExprKind::Add { .. },
        ));
        assert!(matches!(
            int(1).sub_int(int(2)).0.kind(),
            IntExprKind::Sub { .. },
        ));
        assert!(matches!(
            int(1).mult_int(int(2)).0.kind(),
            IntExprKind::Mult { .. },
        ));
        assert!(matches!(
            int(1).div_int(int(2)).0.kind(),
            IntExprKind::Div { .. },
        ));
        assert!(matches!(
            int(1).remainder_int(int(2)).0.kind(),
            IntExprKind::Remainder { .. },
        ));
        assert!(matches!(
            int(1).negate_int().0.kind(),
            IntExprKind::Negate(_)
        ));
    }

    #[test]
    fn float_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            float(1.0).add_float(float(2.0)).0.kind(),
            FloatExprKind::Add { .. },
        ));
        assert!(matches!(
            float(1.0).sub_float(float(2.0)).0.kind(),
            FloatExprKind::Sub { .. },
        ));
        assert!(matches!(
            float(1.0).mult_float(float(2.0)).0.kind(),
            FloatExprKind::Mult { .. },
        ));
        assert!(matches!(
            float(1.0).div_float(float(2.0)).0.kind(),
            FloatExprKind::Div { .. },
        ));
    }

    #[test]
    fn bool_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            int(1).lt_int(int(2)).0.kind(),
            BoolExprKind::LtInt { .. },
        ));
        assert!(matches!(
            int(1).lte_int(int(2)).0.kind(),
            BoolExprKind::LtEqInt { .. },
        ));
        assert!(matches!(
            int(2).gt_int(int(1)).0.kind(),
            BoolExprKind::GtInt { .. },
        ));
        assert!(matches!(
            int(2).gte_int(int(1)).0.kind(),
            BoolExprKind::GtEqInt { .. },
        ));
        assert!(matches!(
            float(1.0).lt_float(float(2.0)).0.kind(),
            BoolExprKind::LtFloat { .. },
        ));
        assert!(matches!(
            float(1.0).lte_float(float(2.0)).0.kind(),
            BoolExprKind::LtEqFloat { .. },
        ));
        assert!(matches!(
            float(2.0).gt_float(float(1.0)).0.kind(),
            BoolExprKind::GtFloat { .. },
        ));
        assert!(matches!(
            float(2.0).gte_float(float(1.0)).0.kind(),
            BoolExprKind::GtEqFloat { .. },
        ));
        assert!(matches!(
            equal(int(1), int(1)).0.kind(),
            BoolExprKind::Equal { .. },
        ));
        assert!(matches!(
            not_equal(bool_(true), bool_(false)).0.kind(),
            BoolExprKind::NotEqual { .. },
        ));
        assert!(matches!(
            bool_(true).and_bool(bool_(false)).0.kind(),
            BoolExprKind::And { .. },
        ));
        assert!(matches!(
            bool_(true).or_bool(bool_(false)).0.kind(),
            BoolExprKind::Or { .. },
        ));
        assert!(matches!(
            bool_(true).negate_bool().0.kind(),
            BoolExprKind::Not(_)
        ));
    }

    #[test]
    fn string_operator_helpers_build_operator_shapes() {
        assert!(matches!(
            string("a").concatenate(string("b")).0.kind(),
            StringExprKind::Concatenate { .. },
        ));
    }
}
