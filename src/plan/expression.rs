use super::id::{
    BoolFunctionId, BoolLocalId, IntFunctionId, IntLocalId, LocalId, NilFunctionId, NilLocalId,
    StringFunctionId, StringLocalId,
};
use super::step::Step;
use super::value::{FunctionType, FunctionValue, Value, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

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

#[derive(Debug, Clone, PartialEq)]
pub struct IntExpr {
    kind: IntExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntExprKind {
    Value(BigInt),
    LocalGet {
        local: IntLocalId,
        name: EcoString,
    },
    Call {
        function: IntFunctionId,
        args: Vec<CallArg>,
    },
    Add {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Sub {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Mult {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Div {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Remainder {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Negate(Box<IntExpr>),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<IntExpr>,
        false_: Box<IntExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: Box<IntExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringExpr {
    kind: StringExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringExprKind {
    Value(EcoString),
    LocalGet {
        local: StringLocalId,
        name: EcoString,
    },
    Call {
        function: StringFunctionId,
        args: Vec<CallArg>,
    },
    Concatenate {
        left: Box<StringExpr>,
        right: Box<StringExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<StringExpr>,
        false_: Box<StringExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: Box<StringExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolExpr {
    kind: BoolExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolExprKind {
    Value(bool),
    LocalGet {
        local: BoolLocalId,
        name: EcoString,
    },
    Call {
        function: BoolFunctionId,
        args: Vec<CallArg>,
    },
    Not(Box<BoolExpr>),
    LtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    LtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    GtEqInt {
        left: Box<IntExpr>,
        right: Box<IntExpr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    Or {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BoolExpr>,
        false_: Box<BoolExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: Box<BoolExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BoolExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct NilExpr {
    kind: NilExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilExprKind {
    Value,
    LocalGet {
        local: NilLocalId,
        name: EcoString,
    },
    Call {
        function: NilFunctionId,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<NilExpr>,
        false_: Box<NilExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: Box<NilExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    type_: FunctionType,
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Value(FunctionValue),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FunctionExpr>,
        false_: Box<FunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionExpr)>,
        fallback: Box<FunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionExpr>,
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

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: Expr,
        false_: Expr,
    ) -> Result<Self, Box<(Self, Self)>> {
        match (true_.kind, false_.kind) {
            (ExprKind::Int(true_), ExprKind::Int(false_)) => {
                Ok(Self::int(IntExpr::bool_case(subject, true_, false_)))
            }
            (ExprKind::String(true_), ExprKind::String(false_)) => {
                Ok(Self::string(StringExpr::bool_case(subject, true_, false_)))
            }
            (ExprKind::Bool(true_), ExprKind::Bool(false_)) => {
                Ok(Self::bool(BoolExpr::bool_case(subject, true_, false_)))
            }
            (ExprKind::Nil(true_), ExprKind::Nil(false_)) => {
                Ok(Self::nil(NilExpr::bool_case(subject, true_, false_)))
            }
            (ExprKind::Function(true_), ExprKind::Function(false_)) => {
                FunctionExpr::bool_case(subject, true_, false_)
                    .map(Self::function)
                    .map_err(|branches| {
                        let (true_, false_) = *branches;
                        Box::new((Self::function(true_), Self::function(false_)))
                    })
            }
            (true_, false_) => Err(Box::new((Self { kind: true_ }, Self { kind: false_ }))),
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, Expr)>,
        fallback: Expr,
    ) -> Result<Self, ()> {
        match fallback.kind {
            ExprKind::Int(fallback) => {
                let mut typed_clauses = Vec::with_capacity(clauses.len());
                for (value, clause) in clauses {
                    let ExprKind::Int(clause) = clause.kind else {
                        return Err(());
                    };
                    typed_clauses.push((value, clause));
                }
                Ok(Self::int(IntExpr::int_case(
                    subject,
                    typed_clauses,
                    fallback,
                )))
            }
            ExprKind::String(fallback) => {
                let mut typed_clauses = Vec::with_capacity(clauses.len());
                for (value, clause) in clauses {
                    let ExprKind::String(clause) = clause.kind else {
                        return Err(());
                    };
                    typed_clauses.push((value, clause));
                }
                Ok(Self::string(StringExpr::int_case(
                    subject,
                    typed_clauses,
                    fallback,
                )))
            }
            ExprKind::Bool(fallback) => {
                let mut typed_clauses = Vec::with_capacity(clauses.len());
                for (value, clause) in clauses {
                    let ExprKind::Bool(clause) = clause.kind else {
                        return Err(());
                    };
                    typed_clauses.push((value, clause));
                }
                Ok(Self::bool(BoolExpr::int_case(
                    subject,
                    typed_clauses,
                    fallback,
                )))
            }
            ExprKind::Nil(fallback) => {
                let mut typed_clauses = Vec::with_capacity(clauses.len());
                for (value, clause) in clauses {
                    let ExprKind::Nil(clause) = clause.kind else {
                        return Err(());
                    };
                    typed_clauses.push((value, clause));
                }
                Ok(Self::nil(NilExpr::int_case(
                    subject,
                    typed_clauses,
                    fallback,
                )))
            }
            ExprKind::Function(fallback) => {
                let mut typed_clauses = Vec::with_capacity(clauses.len());
                for (value, clause) in clauses {
                    let ExprKind::Function(clause) = clause.kind else {
                        return Err(());
                    };
                    typed_clauses.push((value, clause));
                }
                Ok(Self::function(FunctionExpr::int_case(
                    subject,
                    typed_clauses,
                    fallback,
                )?))
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

impl IntExpr {
    pub(crate) fn value(value: BigInt) -> Self {
        Self {
            kind: IntExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: IntLocalId, name: EcoString) -> Self {
        Self {
            kind: IntExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: IntFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: IntExprKind::Call { function, args },
        }
    }

    pub(crate) fn add(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Add {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn sub(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Sub {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn mult(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Mult {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn div(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Div {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn remainder(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Remainder {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn negate(value: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Negate(Box::new(value)),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: IntExpr, false_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, IntExpr)>,
        fallback: IntExpr,
    ) -> Self {
        Self {
            kind: IntExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: IntExpr) -> Self {
        Self {
            kind: IntExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &IntExprKind {
        &self.kind
    }
}

impl StringExpr {
    pub(crate) fn value(value: EcoString) -> Self {
        Self {
            kind: StringExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: StringLocalId, name: EcoString) -> Self {
        Self {
            kind: StringExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: StringFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: StringExprKind::Call { function, args },
        }
    }

    pub(crate) fn concatenate(left: StringExpr, right: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Concatenate {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: StringExpr, false_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringExpr)>,
        fallback: StringExpr,
    ) -> Self {
        Self {
            kind: StringExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringExpr) -> Self {
        Self {
            kind: StringExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &StringExprKind {
        &self.kind
    }
}

impl BoolExpr {
    pub(crate) fn value(value: bool) -> Self {
        Self {
            kind: BoolExprKind::Value(value),
        }
    }

    pub(crate) fn local_get(local: BoolLocalId, name: EcoString) -> Self {
        Self {
            kind: BoolExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: BoolFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: BoolExprKind::Call { function, args },
        }
    }

    pub(crate) fn not(value: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Not(Box::new(value)),
        }
    }

    pub(crate) fn lt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn lte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::LtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gt_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn gte_int(left: IntExpr, right: IntExpr) -> Self {
        Self {
            kind: BoolExprKind::GtEqInt {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::Equal {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn not_equal(left: Expr, right: Expr) -> Self {
        Self {
            kind: BoolExprKind::NotEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn and(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::And {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn or(left: BoolExpr, right: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Or {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: BoolExpr, false_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BoolExpr)>,
        fallback: BoolExpr,
    ) -> Self {
        Self {
            kind: BoolExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BoolExpr) -> Self {
        Self {
            kind: BoolExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &BoolExprKind {
        &self.kind
    }
}

impl NilExpr {
    pub(crate) fn value() -> Self {
        Self {
            kind: NilExprKind::Value,
        }
    }

    pub(crate) fn local_get(local: NilLocalId, name: EcoString) -> Self {
        Self {
            kind: NilExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(function: NilFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            kind: NilExprKind::Call { function, args },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: NilExpr, false_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, NilExpr)>,
        fallback: NilExpr,
    ) -> Self {
        Self {
            kind: NilExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: NilExpr) -> Self {
        Self {
            kind: NilExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &NilExprKind {
        &self.kind
    }
}

impl FunctionExpr {
    pub(crate) fn value(value: FunctionValue) -> Self {
        Self {
            type_: value.type_().clone(),
            kind: FunctionExprKind::Value(value),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: FunctionExpr,
        false_: FunctionExpr,
    ) -> Result<Self, Box<(FunctionExpr, FunctionExpr)>> {
        if true_.type_ != false_.type_ {
            return Err(Box::new((true_, false_)));
        }

        Ok(Self {
            type_: true_.type_.clone(),
            kind: FunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        })
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FunctionExpr)>,
        fallback: FunctionExpr,
    ) -> Result<Self, ()> {
        if clauses
            .iter()
            .any(|(_, branch)| branch.type_ != fallback.type_)
        {
            return Err(());
        }

        Ok(Self {
            type_: fallback.type_.clone(),
            kind: FunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        })
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: FunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
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
        BoolExpr, BoolExprKind, CallArgKind, Expr, ExprKind, FunctionExpr, FunctionExprKind,
        IntExpr, IntExprKind, NilExpr, NilExprKind, StringExpr, StringExprKind,
    };
    use crate::plan::{
        BoolLocalId, FunctionLocalId, FunctionType, FunctionValue, IntFunctionId, IntLocalId,
        LocalId, NilLocalId, RuntimeFunctionId, Step, StringLocalId, Value, ValueType,
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
                Expr::int(IntExpr::value(BigInt::from(1))),
                Expr::int(IntExpr::value(BigInt::from(0))),
            ),
            Ok(Expr::int(IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(BigInt::from(1)),
                IntExpr::value(BigInt::from(0)),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::string(StringExpr::value("yes".into())),
                Expr::string(StringExpr::value("no".into())),
            ),
            Ok(Expr::string(StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into()),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::bool(BoolExpr::value(true)),
                Expr::bool(BoolExpr::value(false)),
            ),
            Ok(Expr::bool(BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::nil(NilExpr::value()),
                Expr::nil(NilExpr::value()),
            ),
            Ok(Expr::nil(NilExpr::bool_case(
                BoolExpr::value(true),
                NilExpr::value(),
                NilExpr::value(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::function(FunctionExpr::value(function_value())),
                Expr::function(FunctionExpr::value(function_value())),
            ),
            Ok(Expr::function(
                FunctionExpr::bool_case(
                    BoolExpr::value(true),
                    FunctionExpr::value(function_value()),
                    FunctionExpr::value(function_value()),
                )
                .expect("matching function branch types")
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::int(IntExpr::value(BigInt::from(1))),
                Expr::bool(BoolExpr::value(false)),
            ),
            Err(Box::new((
                Expr::int(IntExpr::value(BigInt::from(1))),
                Expr::bool(BoolExpr::value(false)),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                Expr::function(FunctionExpr::value(function_value())),
                Expr::function(FunctionExpr::value(FunctionValue::new(
                    FunctionType::new(Vec::new(), ValueType::String),
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    Vec::new(),
                ))),
            ),
            Err(Box::new((
                Expr::function(FunctionExpr::value(function_value())),
                Expr::function(FunctionExpr::value(FunctionValue::new(
                    FunctionType::new(Vec::new(), ValueType::String),
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    Vec::new(),
                ))),
            ))),
        );
    }

    #[test]
    fn expr_int_case_shapes() {
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::int(IntExpr::value(BigInt::from(10))))],
                Expr::int(IntExpr::value(BigInt::from(0))),
            ),
            Ok(Expr::int(IntExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), IntExpr::value(BigInt::from(10)))],
                IntExpr::value(BigInt::from(0)),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(
                    BigInt::from(1),
                    Expr::string(StringExpr::value("one".into()))
                )],
                Expr::string(StringExpr::value("other".into())),
            ),
            Ok(Expr::string(StringExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), StringExpr::value("one".into()))],
                StringExpr::value("other".into()),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::bool(BoolExpr::value(true)))],
                Expr::bool(BoolExpr::value(false)),
            ),
            Ok(Expr::bool(BoolExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), BoolExpr::value(true))],
                BoolExpr::value(false),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::nil(NilExpr::value()))],
                Expr::nil(NilExpr::value()),
            ),
            Ok(Expr::nil(NilExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), NilExpr::value())],
                NilExpr::value(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(
                    BigInt::from(1),
                    Expr::function(FunctionExpr::value(function_value()))
                )],
                Expr::function(FunctionExpr::value(function_value())),
            ),
            Ok(Expr::function(
                FunctionExpr::int_case(
                    IntExpr::value(BigInt::from(1)),
                    vec![(BigInt::from(1), FunctionExpr::value(function_value()))],
                    FunctionExpr::value(function_value()),
                )
                .expect("matching function branch types")
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::bool(BoolExpr::value(true)))],
                Expr::int(IntExpr::value(BigInt::from(0))),
            ),
            Err(()),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::int(IntExpr::value(BigInt::from(1))))],
                Expr::string(StringExpr::value("other".into())),
            ),
            Err(()),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::int(IntExpr::value(BigInt::from(1))))],
                Expr::function(FunctionExpr::value(function_value())),
            ),
            Err(()),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(
                    BigInt::from(1),
                    Expr::function(FunctionExpr::value(FunctionValue::new(
                        FunctionType::new(Vec::new(), ValueType::String),
                        RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                        Vec::new(),
                    )))
                )],
                Expr::function(FunctionExpr::value(function_value())),
            ),
            Err(()),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::int(IntExpr::value(BigInt::from(1))))],
                Expr::bool(BoolExpr::value(false)),
            ),
            Err(()),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), Expr::int(IntExpr::value(BigInt::from(1))))],
                Expr::nil(NilExpr::value()),
            ),
            Err(()),
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
    fn typed_expression_kind_accessors() {
        assert!(matches!(
            IntExpr::value(BigInt::from(1)).kind(),
            IntExprKind::Value(_)
        ));
        assert!(matches!(
            StringExpr::value("geam".into()).kind(),
            StringExprKind::Value(_)
        ));
        assert!(matches!(
            BoolExpr::value(true).kind(),
            BoolExprKind::Value(true)
        ));
        assert!(matches!(NilExpr::value().kind(), NilExprKind::Value));
        assert!(matches!(
            IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(1.into()),
                IntExpr::value(0.into())
            )
            .kind(),
            IntExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            IntExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), IntExpr::value(10.into()))],
                IntExpr::value(0.into())
            )
            .kind(),
            IntExprKind::IntCase { .. }
        ));
        assert!(matches!(
            IntExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                IntExpr::value(2.into()),
            )
            .kind(),
            IntExprKind::Block { .. }
        ));
        assert!(matches!(
            StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into())
            )
            .kind(),
            StringExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            StringExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), StringExpr::value("one".into()))],
                StringExpr::value("other".into())
            )
            .kind(),
            StringExprKind::IntCase { .. }
        ));
        assert!(matches!(
            StringExpr::block(
                vec![Step::evaluate(Expr::string(StringExpr::value("a".into())))],
                StringExpr::value("b".into()),
            )
            .kind(),
            StringExprKind::Block { .. }
        ));
        assert!(matches!(
            BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            BoolExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), BoolExpr::value(true))],
                BoolExpr::value(false)
            )
            .kind(),
            BoolExprKind::IntCase { .. }
        ));
        assert!(matches!(
            BoolExpr::and(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            BoolExprKind::And { .. }
        ));
        assert!(matches!(
            BoolExpr::or(BoolExpr::value(true), BoolExpr::value(false)).kind(),
            BoolExprKind::Or { .. }
        ));
        assert!(matches!(
            BoolExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::value(false)))],
                BoolExpr::value(true),
            )
            .kind(),
            BoolExprKind::Block { .. }
        ));
        assert!(matches!(
            NilExpr::bool_case(BoolExpr::value(true), NilExpr::value(), NilExpr::value()).kind(),
            NilExprKind::BoolCase { .. }
        ));
        assert!(matches!(
            NilExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), NilExpr::value())],
                NilExpr::value()
            )
            .kind(),
            NilExprKind::IntCase { .. }
        ));
        assert!(matches!(
            NilExpr::block(
                vec![Step::evaluate(Expr::nil(NilExpr::value()))],
                NilExpr::value(),
            )
            .kind(),
            NilExprKind::Block { .. }
        ));
        assert!(matches!(
            Expr::from(Value::Nil).kind(),
            ExprKind::Nil(NilExpr { .. })
        ));
        assert!(matches!(
            FunctionExpr::value(function_value()).kind(),
            FunctionExprKind::Value(_)
        ));
        assert!(matches!(
            FunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), FunctionExpr::value(function_value()))],
                FunctionExpr::value(function_value()),
            )
            .expect("matching function branch types")
            .kind(),
            FunctionExprKind::IntCase { .. }
        ));
        assert!(matches!(
            FunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                FunctionExpr::value(function_value()),
            )
            .kind(),
            FunctionExprKind::Block { .. }
        ));
        assert_eq!(
            FunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(
                    1.into(),
                    FunctionExpr::value(FunctionValue::new(
                        FunctionType::new(Vec::new(), ValueType::String),
                        RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                        Vec::new(),
                    ))
                )],
                FunctionExpr::value(function_value()),
            ),
            Err(()),
        );
        assert_eq!(
            FunctionExpr::bool_case(
                BoolExpr::value(true),
                FunctionExpr::value(function_value()),
                FunctionExpr::value(FunctionValue::new(
                    FunctionType::new(Vec::new(), ValueType::String),
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    Vec::new(),
                )),
            ),
            Err(Box::new((
                FunctionExpr::value(function_value()),
                FunctionExpr::value(FunctionValue::new(
                    FunctionType::new(Vec::new(), ValueType::String),
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    Vec::new(),
                )),
            ))),
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
                .into_call_arg(LocalId::Function(FunctionLocalId(0))),
            Err(Expr::function(FunctionExpr::value(function_value()))),
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_call_arg(LocalId::Bool(BoolLocalId(0))),
            Err(Expr::int(IntExpr::value(BigInt::from(1)))),
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            function_type(),
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![LocalId::Int(IntLocalId(0))],
        )
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }
}
