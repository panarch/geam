use crate::plan::{
    BoolExpr, Expr, FunctionId, IntExpr, LocalId, NilExpr, StringExpr, Value, ValueType,
};
use crate::planner::dsl::locals::LocalTable;
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

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
    AddInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    SubInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    MultInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    DivInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    RemainderInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    LtInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    LtEqInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    GtInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    GtEqInt {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    Equal {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    NotEqual {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    Concatenate {
        left: Box<ExprBuilder>,
        right: Box<ExprBuilder>,
    },
    NegateInt(Box<ExprBuilder>),
    NegateBool(Box<ExprBuilder>),
}

impl ExprBuilder {
    pub(in crate::planner) fn add_int(self, right: Self) -> Self {
        Self::AddInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn sub_int(self, right: Self) -> Self {
        Self::SubInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn mult_int(self, right: Self) -> Self {
        Self::MultInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn div_int(self, right: Self) -> Self {
        Self::DivInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn remainder_int(self, right: Self) -> Self {
        Self::RemainderInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn lt_int(self, right: Self) -> Self {
        Self::LtInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn lte_int(self, right: Self) -> Self {
        Self::LtEqInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn gt_int(self, right: Self) -> Self {
        Self::GtInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn gte_int(self, right: Self) -> Self {
        Self::GtEqInt {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn equal(self, right: Self) -> Self {
        Self::Equal {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn not_equal(self, right: Self) -> Self {
        Self::NotEqual {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn concatenate(self, right: Self) -> Self {
        Self::Concatenate {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub(in crate::planner) fn negate_int(self) -> Self {
        Self::NegateInt(Box::new(self))
    }

    pub(in crate::planner) fn negate_bool(self) -> Self {
        Self::NegateBool(Box::new(self))
    }

    pub(super) fn build(self, locals: &LocalTable, functions: &FunctionTable) -> Expr {
        match self {
            Self::Value(value) => Expr::from(value),
            Self::Local(name) => build_local(locals.lookup(&name), name),
            Self::Call { name, args } => {
                let function = lookup_function(functions, &name);
                build_call(
                    function.return_type,
                    function.id,
                    args.into_iter()
                        .map(|arg| arg.build(locals, functions))
                        .collect(),
                )
            }
            Self::AddInt { left, right } => Expr::Int(IntExpr::Add {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::SubInt { left, right } => Expr::Int(IntExpr::Sub {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::MultInt { left, right } => Expr::Int(IntExpr::Mult {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::DivInt { left, right } => Expr::Int(IntExpr::Div {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::RemainderInt { left, right } => Expr::Int(IntExpr::Remainder {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::LtInt { left, right } => Expr::Bool(BoolExpr::LtInt {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::LtEqInt { left, right } => Expr::Bool(BoolExpr::LtEqInt {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::GtInt { left, right } => Expr::Bool(BoolExpr::GtInt {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::GtEqInt { left, right } => Expr::Bool(BoolExpr::GtEqInt {
                left: Box::new(left.build_int(locals, functions)),
                right: Box::new(right.build_int(locals, functions)),
            }),
            Self::Equal { left, right } => Expr::Bool(BoolExpr::Equal {
                left: Box::new(left.build(locals, functions)),
                right: Box::new(right.build(locals, functions)),
            }),
            Self::NotEqual { left, right } => Expr::Bool(BoolExpr::NotEqual {
                left: Box::new(left.build(locals, functions)),
                right: Box::new(right.build(locals, functions)),
            }),
            Self::Concatenate { left, right } => Expr::String(StringExpr::Concatenate {
                left: Box::new(left.build_string(locals, functions)),
                right: Box::new(right.build_string(locals, functions)),
            }),
            Self::NegateInt(value) => Expr::Int(IntExpr::Negate(Box::new(
                value.build_int(locals, functions),
            ))),
            Self::NegateBool(value) => {
                Expr::Bool(BoolExpr::Not(Box::new(value.build_bool(locals, functions))))
            }
        }
    }

    pub(super) fn value_type(&self, locals: &LocalTable, functions: &FunctionTable) -> ValueType {
        match self {
            Self::Value(value) => Expr::from(value.clone()).value_type(),
            Self::Local(name) => locals.lookup(name).into(),
            Self::Call { name, .. } => lookup_function(functions, name).return_type,
            Self::AddInt { .. }
            | Self::SubInt { .. }
            | Self::MultInt { .. }
            | Self::DivInt { .. }
            | Self::RemainderInt { .. }
            | Self::NegateInt(_) => ValueType::Int,
            Self::Concatenate { .. } => ValueType::String,
            Self::LtInt { .. }
            | Self::LtEqInt { .. }
            | Self::GtInt { .. }
            | Self::GtEqInt { .. }
            | Self::Equal { .. }
            | Self::NotEqual { .. }
            | Self::NegateBool(_) => ValueType::Bool,
        }
    }

    fn build_int(self, locals: &LocalTable, functions: &FunctionTable) -> IntExpr {
        match self.build(locals, functions) {
            Expr::Int(value) => value,
            expression => panic_type_mismatch(ValueType::Int, expression.value_type()),
        }
    }

    fn build_string(self, locals: &LocalTable, functions: &FunctionTable) -> StringExpr {
        match self.build(locals, functions) {
            Expr::String(value) => value,
            expression => panic_type_mismatch(ValueType::String, expression.value_type()),
        }
    }

    fn build_bool(self, locals: &LocalTable, functions: &FunctionTable) -> BoolExpr {
        match self.build(locals, functions) {
            Expr::Bool(value) => value,
            expression => panic_type_mismatch(ValueType::Bool, expression.value_type()),
        }
    }
}

pub(super) type FunctionTable = HashMap<EcoString, FunctionEntry>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::planner) struct FunctionEntry {
    pub(super) id: FunctionId,
    pub(super) return_type: ValueType,
}

fn lookup_function(functions: &FunctionTable, name: &EcoString) -> FunctionEntry {
    *functions
        .get(name)
        .unwrap_or_else(|| panic!("unknown function `{name}` in planner DSL"))
}

fn build_local(local: LocalId, name: EcoString) -> Expr {
    match local {
        LocalId::Int(local) => Expr::Int(IntExpr::LocalGet { local, name }),
        LocalId::String(local) => Expr::String(StringExpr::LocalGet { local, name }),
        LocalId::Bool(local) => Expr::Bool(BoolExpr::LocalGet { local, name }),
        LocalId::Nil(local) => Expr::Nil(NilExpr::LocalGet { local, name }),
    }
}

fn build_call(return_type: ValueType, function: FunctionId, args: Vec<Expr>) -> Expr {
    match return_type {
        ValueType::Int => Expr::Int(IntExpr::Call { function, args }),
        ValueType::String => Expr::String(StringExpr::Call { function, args }),
        ValueType::Bool => Expr::Bool(BoolExpr::Call { function, args }),
        ValueType::Nil => Expr::Nil(NilExpr::Call { function, args }),
    }
}

fn panic_type_mismatch(expected: ValueType, actual: ValueType) -> ! {
    panic!("planner DSL expected {expected:?} expression, got {actual:?}")
}

impl From<LocalId> for ValueType {
    fn from(local: LocalId) -> Self {
        match local {
            LocalId::Int(_) => Self::Int,
            LocalId::String(_) => Self::String,
            LocalId::Bool(_) => Self::Bool,
            LocalId::Nil(_) => Self::Nil,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{BoolLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId};

    #[test]
    fn int_build() {
        assert_eq!(
            build_expr(int(-1)),
            Expr::Int(IntExpr::Value(BigInt::from(-1)))
        );
    }

    #[test]
    fn string_build() {
        assert_eq!(
            build_expr(string("geam")),
            Expr::String(StringExpr::Value("geam".into()))
        );
    }

    #[test]
    fn bool_build() {
        assert_eq!(build_expr(bool_(true)), Expr::Bool(BoolExpr::Value(true)));
        assert_eq!(build_expr(bool_(false)), Expr::Bool(BoolExpr::Value(false)));
    }

    #[test]
    fn nil_build() {
        assert_eq!(build_expr(nil()), Expr::Nil(NilExpr::Value));
    }

    #[test]
    fn local_build() {
        let mut locals = LocalTable::default();
        locals.define_int("x".into());
        locals.define_string("name".into());
        locals.define_bool("flag".into());
        locals.define_nil("nothing".into());

        assert_eq!(
            local("x").build(&locals, &FunctionTable::default()),
            Expr::Int(IntExpr::LocalGet {
                local: IntLocalId(0),
                name: "x".into(),
            })
        );
        assert_eq!(
            local("name").build(&locals, &FunctionTable::default()),
            Expr::String(StringExpr::LocalGet {
                local: StringLocalId(0),
                name: "name".into(),
            })
        );
        assert_eq!(
            local("flag").build(&locals, &FunctionTable::default()),
            Expr::Bool(BoolExpr::LocalGet {
                local: BoolLocalId(0),
                name: "flag".into(),
            })
        );
        assert_eq!(
            local("nothing").build(&locals, &FunctionTable::default()),
            Expr::Nil(NilExpr::LocalGet {
                local: NilLocalId(0),
                name: "nothing".into(),
            })
        );
    }

    #[test]
    fn call_build() {
        let mut locals = LocalTable::default();
        locals.define_int("x".into());

        assert_eq!(
            call("helper", [local("x"), string("done")]).build(&locals, &function_table()),
            Expr::Int(IntExpr::Call {
                function: FunctionId(1),
                args: vec![
                    Expr::Int(IntExpr::LocalGet {
                        local: IntLocalId(0),
                        name: "x".into(),
                    }),
                    Expr::String(StringExpr::Value("done".into())),
                ],
            })
        );
        assert_eq!(
            call("string_helper", []).build(&locals, &function_table()),
            Expr::String(StringExpr::Call {
                function: FunctionId(2),
                args: vec![],
            })
        );
        assert_eq!(
            call("bool_helper", []).build(&locals, &function_table()),
            Expr::Bool(BoolExpr::Call {
                function: FunctionId(3),
                args: vec![],
            })
        );
        assert_eq!(
            call("nil_helper", []).build(&locals, &function_table()),
            Expr::Nil(NilExpr::Call {
                function: FunctionId(4),
                args: vec![],
            })
        );
    }

    #[test]
    fn expr_builder_value_type_for_local_and_call() {
        let mut locals = LocalTable::default();
        locals.define_string("name".into());

        assert_eq!(
            local("name").value_type(&locals, &function_table()),
            ValueType::String
        );
        assert_eq!(
            call("nil_helper", []).value_type(&locals, &function_table()),
            ValueType::Nil,
        );
    }

    #[test]
    fn local_id_into_value_type() {
        assert_eq!(ValueType::from(LocalId::Int(IntLocalId(0))), ValueType::Int);
        assert_eq!(
            ValueType::from(LocalId::String(StringLocalId(0))),
            ValueType::String,
        );
        assert_eq!(
            ValueType::from(LocalId::Bool(BoolLocalId(0))),
            ValueType::Bool
        );
        assert_eq!(ValueType::from(LocalId::Nil(NilLocalId(0))), ValueType::Nil);
    }

    #[test]
    #[should_panic(expected = "unknown function `missing` in planner DSL")]
    fn call_build_panics_on_missing_function() {
        call("missing", []).build(&LocalTable::default(), &FunctionTable::default());
    }

    #[test]
    fn expr_builder_add_int() {
        assert_eq!(
            build_expr(int(1).add_int(int(2))),
            Expr::Int(IntExpr::Add {
                left: Box::new(IntExpr::Value(BigInt::from(1))),
                right: Box::new(IntExpr::Value(BigInt::from(2))),
            }),
        );
    }

    #[test]
    fn expr_builder_sub_int() {
        assert_eq!(
            build_expr(int(1).sub_int(int(2))),
            Expr::Int(IntExpr::Sub {
                left: Box::new(IntExpr::Value(BigInt::from(1))),
                right: Box::new(IntExpr::Value(BigInt::from(2))),
            }),
        );
    }

    #[test]
    fn expr_builder_mult_int() {
        assert_eq!(
            build_expr(int(1).mult_int(int(2))),
            Expr::Int(IntExpr::Mult {
                left: Box::new(IntExpr::Value(BigInt::from(1))),
                right: Box::new(IntExpr::Value(BigInt::from(2))),
            }),
        );
    }

    #[test]
    fn expr_builder_div_int() {
        assert_eq!(
            build_expr(int(1).div_int(int(2))),
            Expr::Int(IntExpr::Div {
                left: Box::new(IntExpr::Value(BigInt::from(1))),
                right: Box::new(IntExpr::Value(BigInt::from(2))),
            }),
        );
    }

    #[test]
    fn expr_builder_remainder_int() {
        assert_eq!(
            build_expr(int(1).remainder_int(int(2))),
            Expr::Int(IntExpr::Remainder {
                left: Box::new(IntExpr::Value(BigInt::from(1))),
                right: Box::new(IntExpr::Value(BigInt::from(2))),
            }),
        );
    }

    #[test]
    fn expr_builder_lt_int() {
        assert_int_comparison(int(1).lt_int(int(2)), ExpectedComparison::Lt);
    }

    #[test]
    fn expr_builder_lte_int() {
        assert_int_comparison(int(1).lte_int(int(2)), ExpectedComparison::LtEq);
    }

    #[test]
    fn expr_builder_gt_int() {
        assert_int_comparison(int(1).gt_int(int(2)), ExpectedComparison::Gt);
    }

    #[test]
    fn expr_builder_gte_int() {
        assert_int_comparison(int(1).gte_int(int(2)), ExpectedComparison::GtEq);
    }

    #[test]
    fn expr_builder_equal() {
        assert_eq!(
            build_expr(int(1).equal(int(2))),
            Expr::Bool(BoolExpr::Equal {
                left: Box::new(Expr::Int(IntExpr::Value(BigInt::from(1)))),
                right: Box::new(Expr::Int(IntExpr::Value(BigInt::from(2)))),
            }),
        );
    }

    #[test]
    fn expr_builder_not_equal() {
        assert_eq!(
            build_expr(int(1).not_equal(int(2))),
            Expr::Bool(BoolExpr::NotEqual {
                left: Box::new(Expr::Int(IntExpr::Value(BigInt::from(1)))),
                right: Box::new(Expr::Int(IntExpr::Value(BigInt::from(2)))),
            }),
        );
    }

    #[test]
    fn expr_builder_concatenate() {
        assert_eq!(
            build_expr(string("a").concatenate(string("b"))),
            Expr::String(StringExpr::Concatenate {
                left: Box::new(StringExpr::Value("a".into())),
                right: Box::new(StringExpr::Value("b".into())),
            })
        );
    }

    #[test]
    fn expr_builder_negate_int() {
        assert_eq!(
            build_expr(int(1).negate_int()),
            Expr::Int(IntExpr::Negate(Box::new(IntExpr::Value(BigInt::from(1)))))
        );
    }

    #[test]
    fn expr_builder_negate_bool() {
        assert_eq!(
            build_expr(bool_(true).negate_bool()),
            Expr::Bool(BoolExpr::Not(Box::new(BoolExpr::Value(true))))
        );
    }

    #[test]
    #[should_panic(expected = "planner DSL expected Int expression, got String")]
    fn expr_builder_int_operation_panics_on_string_operand() {
        let _ = int(1)
            .add_int(string("bad"))
            .build(&LocalTable::default(), &function_table());
    }

    #[test]
    #[should_panic(expected = "planner DSL expected String expression, got Int")]
    fn expr_builder_concatenate_panics_on_int_operand() {
        let _ = string("a")
            .concatenate(int(1))
            .build(&LocalTable::default(), &function_table());
    }

    #[test]
    #[should_panic(expected = "planner DSL expected Bool expression, got Int")]
    fn expr_builder_negate_bool_panics_on_int_operand() {
        let _ = int(1)
            .negate_bool()
            .build(&LocalTable::default(), &function_table());
    }

    fn assert_int_comparison(builder: ExprBuilder, expected: ExpectedComparison) {
        let left = Box::new(IntExpr::Value(BigInt::from(1)));
        let right = Box::new(IntExpr::Value(BigInt::from(2)));
        let expected = match expected {
            ExpectedComparison::Lt => BoolExpr::LtInt { left, right },
            ExpectedComparison::LtEq => BoolExpr::LtEqInt { left, right },
            ExpectedComparison::Gt => BoolExpr::GtInt { left, right },
            ExpectedComparison::GtEq => BoolExpr::GtEqInt { left, right },
        };

        assert_eq!(build_expr(builder), Expr::Bool(expected));
    }

    enum ExpectedComparison {
        Lt,
        LtEq,
        Gt,
        GtEq,
    }

    fn build_expr(expr: ExprBuilder) -> Expr {
        expr.build(&LocalTable::default(), &function_table())
    }

    fn function_table() -> FunctionTable {
        FunctionTable::from([
            (
                "helper".into(),
                FunctionEntry {
                    id: FunctionId(1),
                    return_type: ValueType::Int,
                },
            ),
            (
                "string_helper".into(),
                FunctionEntry {
                    id: FunctionId(2),
                    return_type: ValueType::String,
                },
            ),
            (
                "bool_helper".into(),
                FunctionEntry {
                    id: FunctionId(3),
                    return_type: ValueType::Bool,
                },
            ),
            (
                "nil_helper".into(),
                FunctionEntry {
                    id: FunctionId(4),
                    return_type: ValueType::Nil,
                },
            ),
        ])
    }
}
