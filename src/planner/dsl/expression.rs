use crate::plan::{BinOp, Expr, FunctionRef, Value};
use crate::planner::dsl::locals::LocalTable;
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::planner) fn int(value: i64) -> ExprBuilder {
    ExprBuilder::Value(Value::Int(BigInt::from(value)))
}

pub(in crate::planner) fn string(value: impl Into<EcoString>) -> ExprBuilder {
    ExprBuilder::Value(Value::String(value.into()))
}

pub(in crate::planner) fn bool_(value: bool) -> ExprBuilder {
    ExprBuilder::Value(Value::Bool(value))
}

pub(in crate::planner) fn nil() -> ExprBuilder {
    ExprBuilder::Value(Value::Nil)
}

pub(in crate::planner) fn local(name: impl Into<EcoString>) -> ExprBuilder {
    ExprBuilder::Local(name.into())
}

pub(in crate::planner) fn call(
    name: impl Into<EcoString>,
    args: impl IntoIterator<Item = ExprBuilder>,
) -> ExprBuilder {
    ExprBuilder::Call {
        name: name.into(),
        args: args.into_iter().collect(),
    }
}

#[derive(Debug, Clone)]
pub(in crate::planner) enum ExprBuilder {
    Value(Value),
    Local(EcoString),
    Call {
        name: EcoString,
        args: Vec<ExprBuilder>,
    },
    BinOp {
        op: BinOp,
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    NegateInt(Box<ExprBuilder>),
    NegateBool(Box<ExprBuilder>),
}

impl ExprBuilder {
    pub(in crate::planner) fn add_int(self, right: Self) -> Self {
        self.bin_op(BinOp::AddInt, right)
    }

    pub(in crate::planner) fn sub_int(self, right: Self) -> Self {
        self.bin_op(BinOp::SubInt, right)
    }

    pub(in crate::planner) fn mult_int(self, right: Self) -> Self {
        self.bin_op(BinOp::MultInt, right)
    }

    pub(in crate::planner) fn lt_int(self, right: Self) -> Self {
        self.bin_op(BinOp::LtInt, right)
    }

    pub(in crate::planner) fn lte_int(self, right: Self) -> Self {
        self.bin_op(BinOp::LtEqInt, right)
    }

    pub(in crate::planner) fn gt_int(self, right: Self) -> Self {
        self.bin_op(BinOp::GtInt, right)
    }

    pub(in crate::planner) fn gte_int(self, right: Self) -> Self {
        self.bin_op(BinOp::GtEqInt, right)
    }

    pub(in crate::planner) fn equal(self, right: Self) -> Self {
        self.bin_op(BinOp::Eq, right)
    }

    pub(in crate::planner) fn not_equal(self, right: Self) -> Self {
        self.bin_op(BinOp::NotEq, right)
    }

    pub(in crate::planner) fn concatenate(self, right: Self) -> Self {
        self.bin_op(BinOp::Concatenate, right)
    }

    pub(in crate::planner) fn negate_int(self) -> Self {
        Self::NegateInt(Box::new(self))
    }

    pub(in crate::planner) fn negate_bool(self) -> Self {
        Self::NegateBool(Box::new(self))
    }

    fn bin_op(self, op: BinOp, right: Self) -> Self {
        Self::BinOp {
            op,
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(super) fn build(self, locals: &LocalTable) -> Expr {
        match self {
            Self::Value(value) => Expr::Value(value),
            Self::Local(name) => Expr::LocalGet {
                local: locals.lookup(&name),
                name,
            },
            Self::Call { name, args } => Expr::Call {
                function: FunctionRef::Local(name),
                args: args.into_iter().map(|arg| arg.build(locals)).collect(),
            },
            Self::BinOp { op, left, right } => Expr::BinOp {
                op,
                left: Box::new(left.build(locals)),
                right: Box::new(right.build(locals)),
            },
            Self::NegateInt(value) => Expr::NegateInt(Box::new(value.build(locals))),
            Self::NegateBool(value) => Expr::NegateBool(Box::new(value.build(locals))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::LocalId;

    #[test]
    fn int_build() {
        assert_eq!(
            build_expr(int(-1)),
            Expr::Value(Value::Int(BigInt::from(-1)))
        );
    }

    #[test]
    fn string_build() {
        assert_eq!(
            build_expr(string("geam")),
            Expr::Value(Value::String("geam".into()))
        );
    }

    #[test]
    fn bool_build() {
        assert_eq!(build_expr(bool_(true)), Expr::Value(Value::Bool(true)));
        assert_eq!(build_expr(bool_(false)), Expr::Value(Value::Bool(false)));
    }

    #[test]
    fn nil_build() {
        assert_eq!(build_expr(nil()), Expr::Value(Value::Nil));
    }

    #[test]
    fn local_build() {
        let mut locals = LocalTable::default();
        locals.define("x".into());

        assert_eq!(
            local("x").build(&locals),
            Expr::LocalGet {
                local: LocalId(0),
                name: "x".into(),
            }
        );
    }

    #[test]
    fn call_build() {
        let mut locals = LocalTable::default();
        locals.define("x".into());

        assert_eq!(
            call("helper", [local("x"), string("done")]).build(&locals),
            Expr::Call {
                function: FunctionRef::Local("helper".into()),
                args: vec![
                    Expr::LocalGet {
                        local: LocalId(0),
                        name: "x".into(),
                    },
                    Expr::Value(Value::String("done".into())),
                ],
            }
        );
    }

    #[test]
    fn expr_builder_add_int() {
        assert_int_binop(int(1).add_int(int(2)), BinOp::AddInt);
    }

    #[test]
    fn expr_builder_sub_int() {
        assert_int_binop(int(1).sub_int(int(2)), BinOp::SubInt);
    }

    #[test]
    fn expr_builder_mult_int() {
        assert_int_binop(int(1).mult_int(int(2)), BinOp::MultInt);
    }

    #[test]
    fn expr_builder_lt_int() {
        assert_int_binop(int(1).lt_int(int(2)), BinOp::LtInt);
    }

    #[test]
    fn expr_builder_lte_int() {
        assert_int_binop(int(1).lte_int(int(2)), BinOp::LtEqInt);
    }

    #[test]
    fn expr_builder_gt_int() {
        assert_int_binop(int(1).gt_int(int(2)), BinOp::GtInt);
    }

    #[test]
    fn expr_builder_gte_int() {
        assert_int_binop(int(1).gte_int(int(2)), BinOp::GtEqInt);
    }

    #[test]
    fn expr_builder_equal() {
        assert_int_binop(int(1).equal(int(2)), BinOp::Eq);
    }

    #[test]
    fn expr_builder_not_equal() {
        assert_int_binop(int(1).not_equal(int(2)), BinOp::NotEq);
    }

    #[test]
    fn expr_builder_concatenate() {
        assert_eq!(
            build_expr(string("a").concatenate(string("b"))),
            Expr::BinOp {
                op: BinOp::Concatenate,
                left: Box::new(Expr::Value(Value::String("a".into()))),
                right: Box::new(Expr::Value(Value::String("b".into()))),
            }
        );
    }

    #[test]
    fn expr_builder_negate_int() {
        assert_eq!(
            build_expr(int(1).negate_int()),
            Expr::NegateInt(Box::new(Expr::Value(Value::Int(BigInt::from(1)))))
        );
    }

    #[test]
    fn expr_builder_negate_bool() {
        assert_eq!(
            build_expr(bool_(true).negate_bool()),
            Expr::NegateBool(Box::new(Expr::Value(Value::Bool(true))))
        );
    }

    fn assert_int_binop(builder: ExprBuilder, op: BinOp) {
        assert_eq!(
            build_expr(builder),
            Expr::BinOp {
                op,
                left: Box::new(Expr::Value(Value::Int(BigInt::from(1)))),
                right: Box::new(Expr::Value(Value::Int(BigInt::from(2)))),
            }
        );
    }

    fn build_expr(expr: ExprBuilder) -> Expr {
        expr.build(&LocalTable::default())
    }
}
