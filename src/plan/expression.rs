mod bool;
mod case;
mod function;
mod int;
mod nil;
mod string;

use super::id::{BoolLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId};
use super::value::{Value, ValueType};

pub(crate) use self::case::{BoolCaseBranches, IntCaseBranches};
pub use self::{
    bool::BoolExpr, function::FunctionExpr, int::IntExpr, nil::NilExpr, string::StringExpr,
};
pub(crate) use self::{
    bool::BoolExprKind, function::FunctionExprKind, int::IntExprKind, nil::NilExprKind,
    string::StringExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind {
    Int(IntExpr),
    String(StringExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    Function(FunctionExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    kind: CallArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CallArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
}

impl Expr {
    pub(crate) fn int(expression: IntExpr) -> Self {
        Self {
            kind: ExprKind::Int(expression),
        }
    }

    pub(crate) fn string(expression: StringExpr) -> Self {
        Self {
            kind: ExprKind::String(expression),
        }
    }

    pub(crate) fn bool(expression: BoolExpr) -> Self {
        Self {
            kind: ExprKind::Bool(expression),
        }
    }

    pub(crate) fn nil(expression: NilExpr) -> Self {
        Self {
            kind: ExprKind::Nil(expression),
        }
    }

    pub(crate) fn function(expression: FunctionExpr) -> Self {
        Self {
            kind: ExprKind::Function(expression),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, branches: BoolCaseBranches) -> Self {
        match branches {
            BoolCaseBranches::Int { true_, false_ } => {
                Self::int(IntExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::String { true_, false_ } => {
                Self::string(StringExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Bool { true_, false_ } => {
                Self::bool(BoolExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Nil { true_, false_ } => {
                Self::nil(NilExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Function { true_, false_ } => {
                Self::function(FunctionExpr::bool_case(subject, true_, false_))
            }
        }
    }

    pub(crate) fn int_case(subject: IntExpr, branches: IntCaseBranches) -> Self {
        match branches {
            IntCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Function { clauses, fallback } => {
                Self::function(FunctionExpr::int_case(subject, clauses, fallback))
            }
        }
    }

    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> ExprKind {
        self.kind
    }

    pub(crate) fn into_int(self) -> Result<IntExpr, Self> {
        match self.kind {
            ExprKind::Int(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_string(self) -> Result<StringExpr, Self> {
        match self.kind {
            ExprKind::String(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_bool(self) -> Result<BoolExpr, Self> {
        match self.kind {
            ExprKind::Bool(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub(crate) fn into_function(self) -> Result<FunctionExpr, Self> {
        match self.kind {
            ExprKind::Function(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    #[cfg(test)]
    pub(crate) fn into_nil(self) -> Result<NilExpr, Self> {
        match self.kind {
            ExprKind::Nil(expression) => Ok(expression),
            kind => Err(Self { kind }),
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ExprKind::Int(_) => ValueType::Int,
            ExprKind::String(_) => ValueType::String,
            ExprKind::Bool(_) => ValueType::Bool,
            ExprKind::Nil(_) => ValueType::Nil,
            ExprKind::Function(expression) => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
        }
    }

    pub(crate) fn into_call_arg(self, local: LocalId) -> Result<CallArg, Self> {
        match (local, self.kind) {
            (LocalId::Int(local), ExprKind::Int(value)) => Ok(CallArg::int(local, value)),
            (LocalId::String(local), ExprKind::String(value)) => Ok(CallArg::string(local, value)),
            (LocalId::Bool(local), ExprKind::Bool(value)) => Ok(CallArg::bool(local, value)),
            (LocalId::Nil(local), ExprKind::Nil(value)) => Ok(CallArg::nil(local, value)),
            (_, kind) => Err(Self { kind }),
        }
    }
}

impl CallArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CallArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CallArgKind::String { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CallArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CallArgKind::Nil { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CallArgKind {
        &self.kind
    }
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(value) => Self::int(IntExpr::value(value)),
            Value::String(value) => Self::string(StringExpr::value(value)),
            Value::Bool(value) => Self::bool(BoolExpr::value(value)),
            Value::Nil => Self::nil(NilExpr::value()),
            Value::Function(value) => Self::function(FunctionExpr::value(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolCaseBranches, BoolExpr, CallArgKind, Expr, FunctionExpr, IntCaseBranches, IntExpr,
        NilExpr, StringExpr,
    };
    use crate::plan::{
        BoolLocalId, FunctionType, FunctionValue, IntFunctionId, IntLocalId, LocalId, NilLocalId,
        RuntimeFunctionId, StringLocalId, Value, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn expr_value_shapes() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))),
            Expr::from(Value::Int(BigInt::from(1)))
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into())),
            Expr::from(Value::String("geam".into()))
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)),
            Expr::from(Value::Bool(true))
        );
        assert_eq!(Expr::nil(NilExpr::value()), Expr::from(Value::Nil));
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value())),
            Expr::from(Value::Function(function_value())),
        );
    }

    #[test]
    fn expr_bool_case_shapes() {
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Int {
                    true_: IntExpr::value(BigInt::from(1)),
                    false_: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(BigInt::from(1)),
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::String {
                    true_: StringExpr::value("yes".into()),
                    false_: StringExpr::value("no".into()),
                },
            ),
            Expr::string(StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into()),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Bool {
                    true_: BoolExpr::value(true),
                    false_: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Nil {
                    true_: NilExpr::value(),
                    false_: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::bool_case(
                BoolExpr::value(true),
                NilExpr::value(),
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Function {
                    true_: FunctionExpr::value(function_value()),
                    false_: FunctionExpr::value(function_value()),
                },
            ),
            Expr::function(FunctionExpr::bool_case(
                BoolExpr::value(true),
                FunctionExpr::value(function_value()),
                FunctionExpr::value(function_value()),
            )),
        );
    }

    #[test]
    fn expr_int_case_shapes() {
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Int {
                    clauses: vec![(BigInt::from(1), IntExpr::value(BigInt::from(10)))],
                    fallback: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), IntExpr::value(BigInt::from(10)))],
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::String {
                    clauses: vec![(BigInt::from(1), StringExpr::value("one".into()))],
                    fallback: StringExpr::value("other".into()),
                },
            ),
            Expr::string(StringExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), StringExpr::value("one".into()))],
                StringExpr::value("other".into()),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Bool {
                    clauses: vec![(BigInt::from(1), BoolExpr::value(true))],
                    fallback: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), BoolExpr::value(true))],
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Nil {
                    clauses: vec![(BigInt::from(1), NilExpr::value())],
                    fallback: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), NilExpr::value())],
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Function {
                    clauses: vec![(BigInt::from(1), FunctionExpr::value(function_value()))],
                    fallback: FunctionExpr::value(function_value()),
                },
            ),
            Expr::function(FunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), FunctionExpr::value(function_value()))],
                FunctionExpr::value(function_value()),
            )),
        );
    }

    #[test]
    fn expr_value_type() {
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).value_type(),
            ValueType::Int
        );
        assert_eq!(
            Expr::from(Value::String("geam".into())).value_type(),
            ValueType::String,
        );
        assert_eq!(Expr::from(Value::Bool(true)).value_type(), ValueType::Bool);
        assert_eq!(Expr::from(Value::Nil).value_type(), ValueType::Nil);
        assert_eq!(
            Expr::from(Value::Function(function_value())).value_type(),
            ValueType::Function(Box::new(function_type())),
        );
    }

    #[test]
    fn expr_into_typed_expression() {
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).into_int(),
            Ok(IntExpr::value(BigInt::from(1))),
        );
        assert_eq!(
            Expr::from(Value::String("geam".into())).into_string(),
            Ok(StringExpr::value("geam".into())),
        );
        assert_eq!(
            Expr::from(Value::Bool(true)).into_bool(),
            Ok(BoolExpr::value(true)),
        );
        assert_eq!(Expr::from(Value::Nil).into_nil(), Ok(NilExpr::value()));
        assert_eq!(
            Expr::from(Value::Nil).into_int(),
            Err(Expr::from(Value::Nil)),
        );
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).into_nil(),
            Err(Expr::from(Value::Int(BigInt::from(1)))),
        );
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).into_function(),
            Err(Expr::from(Value::Int(BigInt::from(1)))),
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value())).into_function(),
            Ok(FunctionExpr::value(function_value())),
        );
    }

    #[test]
    fn expr_into_call_arg() {
        assert!(matches!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(LocalId::Int(IntLocalId(0)))
                .expect("int call arg")
                .kind(),
            CallArgKind::Int {
                local: IntLocalId(0),
                ..
            },
        ));
        assert!(matches!(
            Expr::string(StringExpr::value("geam".into()))
                .into_call_arg(LocalId::String(StringLocalId(0)))
                .expect("string call arg")
                .kind(),
            CallArgKind::String {
                local: StringLocalId(0),
                ..
            },
        ));
        assert!(matches!(
            Expr::bool(BoolExpr::value(true))
                .into_call_arg(LocalId::Bool(BoolLocalId(0)))
                .expect("bool call arg")
                .kind(),
            CallArgKind::Bool {
                local: BoolLocalId(0),
                ..
            },
        ));
        assert!(matches!(
            Expr::nil(NilExpr::value())
                .into_call_arg(LocalId::Nil(NilLocalId(0)))
                .expect("nil call arg")
                .kind(),
            CallArgKind::Nil {
                local: NilLocalId(0),
                ..
            },
        ));
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value()))
                .into_call_arg(LocalId::Int(IntLocalId(0))),
            Err(Expr::function(FunctionExpr::value(function_value()))),
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_call_arg(LocalId::Bool(BoolLocalId(0))),
            Err(Expr::int(IntExpr::value(BigInt::from(1)))),
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }
}
