mod bool;
mod case;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;

use super::function::ParamLocal;
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocalId, ListLocalId, NilFunctionLocalId,
    NilLocalId, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
};
use super::value::{Value, ValueType};

pub(crate) use self::case::{
    BoolCaseBranches, FloatCaseBranches, IntCaseBranches, StringCaseBranches,
};
pub use self::{
    bool::BoolExpr,
    float::FloatExpr,
    function::{
        BoolFunctionExpr, FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntFunctionExpr,
        ListFunctionExpr, NilFunctionExpr, StringFunctionExpr, TupleFunctionExpr,
    },
    int::IntExpr,
    list::ListExpr,
    nil::NilExpr,
    string::StringExpr,
    tuple::TupleExpr,
};
pub(crate) use self::{
    bool::BoolExprKind,
    float::FloatExprKind,
    function::{
        BoolFunctionExprKind, FloatFunctionExprKind, FunctionExprKind, FunctionFunctionExprKind,
        IntFunctionExprKind, ListFunctionExprKind, NilFunctionExprKind, StringFunctionExprKind,
        TupleFunctionExprKind,
    },
    int::IntExprKind,
    list::ListExprKind,
    nil::NilExprKind,
    string::StringExprKind,
    tuple::TupleExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind {
    Int(IntExpr),
    String(StringExpr),
    Float(FloatExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    Tuple(TupleExpr),
    List(ListExpr),
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
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List {
        local: ListLocalId,
        value: ListExpr,
    },
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocalId,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    kind: CaptureArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List {
        local: ListLocalId,
        value: ListExpr,
    },
    IntFunction {
        local: IntFunctionLocalId,
        value: IntFunctionExpr,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    ListFunction {
        local: ListFunctionLocalId,
        value: ListFunctionExpr,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
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

    pub(crate) fn float(expression: FloatExpr) -> Self {
        Self {
            kind: ExprKind::Float(expression),
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

    pub(crate) fn tuple(expression: TupleExpr) -> Self {
        Self {
            kind: ExprKind::Tuple(expression),
        }
    }

    pub(crate) fn list(expression: ListExpr) -> Self {
        Self {
            kind: ExprKind::List(expression),
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
            BoolCaseBranches::Float { true_, false_ } => {
                Self::float(FloatExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Bool { true_, false_ } => {
                Self::bool(BoolExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Nil { true_, false_ } => {
                Self::nil(NilExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Tuple { true_, false_ } => {
                Self::tuple(TupleExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::List { true_, false_ } => {
                Self::list(ListExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::IntFunction { true_, false_ } => Self::function(FunctionExpr::int(
                IntFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::StringFunction { true_, false_ } => Self::function(
                FunctionExpr::string(StringFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::FloatFunction { true_, false_ } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::BoolFunction { true_, false_ } => Self::function(FunctionExpr::bool(
                BoolFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::NilFunction { true_, false_ } => Self::function(FunctionExpr::nil(
                NilFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::TupleFunction { true_, false_ } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::ListFunction { true_, false_ } => Self::function(FunctionExpr::list(
                ListFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::FunctionFunction { true_, false_ } => Self::function(
                FunctionExpr::function(FunctionFunctionExpr::bool_case(subject, true_, false_)),
            ),
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
            IntCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::List { clauses, fallback } => {
                Self::list(ListExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::FunctionFunction { clauses, fallback } => Self::function(
                FunctionExpr::function(FunctionFunctionExpr::int_case(subject, clauses, fallback)),
            ),
        }
    }

    pub(crate) fn string_case(subject: StringExpr, branches: StringCaseBranches) -> Self {
        match branches {
            StringCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::List { clauses, fallback } => {
                Self::list(ListExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::FunctionFunction { clauses, fallback } => {
                Self::function(FunctionExpr::function(FunctionFunctionExpr::string_case(
                    subject, clauses, fallback,
                )))
            }
        }
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: FloatCaseBranches) -> Self {
        match branches {
            FloatCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::List { clauses, fallback } => {
                Self::list(ListExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::FunctionFunction { clauses, fallback } => {
                Self::function(FunctionExpr::function(FunctionFunctionExpr::float_case(
                    subject, clauses, fallback,
                )))
            }
        }
    }

    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> ExprKind {
        self.kind
    }

    pub(crate) fn into_int(self) -> Option<IntExpr> {
        match self.kind {
            ExprKind::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringExpr> {
        match self.kind {
            ExprKind::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatExpr> {
        match self.kind {
            ExprKind::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolExpr> {
        match self.kind {
            ExprKind::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleExpr> {
        match self.kind {
            ExprKind::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn into_list(self) -> Option<ListExpr> {
        match self.kind {
            ExprKind::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionExpr> {
        match self.kind {
            ExprKind::Function(expression) => Some(expression),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn into_nil(self) -> Option<NilExpr> {
        match self.kind {
            ExprKind::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ExprKind::Int(_) => ValueType::Int,
            ExprKind::String(_) => ValueType::String,
            ExprKind::Float(_) => ValueType::Float,
            ExprKind::Bool(_) => ValueType::Bool,
            ExprKind::Nil(_) => ValueType::Nil,
            ExprKind::Tuple(expression) => ValueType::Tuple(expression.type_().to_vec()),
            ExprKind::List(expression) => {
                ValueType::List(Box::new(expression.element_type().clone()))
            }
            ExprKind::Function(expression) => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
        }
    }

    pub(crate) fn into_call_arg(self, local: &ParamLocal) -> Option<CallArg> {
        match (local, self.kind) {
            (ParamLocal::Int(local), ExprKind::Int(value)) => Some(CallArg::int(*local, value)),
            (ParamLocal::String(local), ExprKind::String(value)) => {
                Some(CallArg::string(*local, value))
            }
            (ParamLocal::Float(local), ExprKind::Float(value)) => {
                Some(CallArg::float(*local, value))
            }
            (ParamLocal::Bool(local), ExprKind::Bool(value)) => Some(CallArg::bool(*local, value)),
            (ParamLocal::Nil(local), ExprKind::Nil(value)) => Some(CallArg::nil(*local, value)),
            (
                ParamLocal::Tuple {
                    local,
                    type_: expected,
                },
                ExprKind::Tuple(value),
            ) if value.type_() == expected => Some(CallArg::tuple(*local, value)),
            (
                ParamLocal::List {
                    local,
                    element_type: expected,
                },
                ExprKind::List(value),
            ) if value.element_type() == expected => Some(CallArg::list(*local, value)),
            (
                ParamLocal::IntFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_int()
                .map(|value| CallArg::int_function(*local, value)),
            (
                ParamLocal::StringFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_string()
                .map(|value| CallArg::string_function(*local, value)),
            (
                ParamLocal::FloatFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_float()
                .map(|value| CallArg::float_function(*local, value)),
            (
                ParamLocal::BoolFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_bool()
                .map(|value| CallArg::bool_function(*local, value)),
            (
                ParamLocal::NilFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_nil()
                .map(|value| CallArg::nil_function(*local, value)),
            (
                ParamLocal::TupleFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_tuple()
                .map(|value| CallArg::tuple_function(*local, value)),
            (
                ParamLocal::ListFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_list()
                .map(|value| CallArg::list_function(*local, value)),
            (
                ParamLocal::FunctionFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == expected => value
                .into_function()
                .map(|value| CallArg::function_function(*local, value)),
            _ => None,
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

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CallArgKind::Float { local, value },
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

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CallArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(local: ListLocalId, value: ListExpr) -> Self {
        Self {
            kind: CallArgKind::List { local, value },
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::StringFunction { local, value },
        }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function(local: ListFunctionLocalId, value: ListFunctionExpr) -> Self {
        Self {
            kind: CallArgKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: CallArgKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CallArgKind {
        &self.kind
    }
}

impl CaptureArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CaptureArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CaptureArgKind::String { local, value },
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CaptureArgKind::Float { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CaptureArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CaptureArgKind::Nil { local, value },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CaptureArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(local: ListLocalId, value: ListExpr) -> Self {
        Self {
            kind: CaptureArgKind::List { local, value },
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::StringFunction { local, value },
        }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function(local: ListFunctionLocalId, value: ListFunctionExpr) -> Self {
        Self {
            kind: CaptureArgKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: CaptureArgKind::FunctionFunction { local, value },
        }
    }

    pub(crate) fn kind(&self) -> &CaptureArgKind {
        &self.kind
    }
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        match value {
            Value::Int(value) => Self::int(IntExpr::value(value)),
            Value::String(value) => Self::string(StringExpr::value(value)),
            Value::Float(value) => Self::float(FloatExpr::value(value)),
            Value::Bool(value) => Self::bool(BoolExpr::value(value)),
            Value::Nil => Self::nil(NilExpr::value()),
            Value::Tuple(value) => Self::tuple(TupleExpr::value(
                value.iter().cloned().map(Self::from).collect(),
                value.iter().map(Value::value_type).collect(),
            )),
            Value::List(value) => Self::list(ListExpr::value(
                value.values().iter().cloned().map(Self::from).collect(),
                value.element_type().clone(),
            )),
            Value::Function(value) => Self::function(FunctionExpr::value(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolCaseBranches, BoolExpr, BoolFunctionExpr, CallArg, Expr, FloatCaseBranches, FloatExpr,
        FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntCaseBranches, IntExpr,
        IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, StringCaseBranches,
        StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionValue, BoolLocalId, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionValue, FloatLocalId, FunctionFunctionId, FunctionFunctionValue, FunctionType,
        FunctionValue, IntFunctionFunctionId, IntFunctionId, IntFunctionValue, IntLocalId,
        ListFunctionId, ListFunctionLocalId, ListFunctionValue, ListLocalId, ListValue,
        NilFunctionId, NilFunctionValue, NilLocalId, ParamLocal, RuntimeFunctionId,
        StringFunctionId, StringFunctionValue, StringLocalId, TupleFunctionId,
        TupleFunctionLocalId, TupleFunctionValue, TupleLocalId, Value, ValueType,
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
            Expr::float(FloatExpr::value(1.5)),
            Expr::from(Value::Float(1.5))
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)),
            Expr::from(Value::Bool(true))
        );
        assert_eq!(Expr::nil(NilExpr::value()), Expr::from(Value::Nil));
        assert_eq!(
            Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                vec![ValueType::Int],
            )),
            Expr::from(Value::Tuple(vec![Value::Int(BigInt::from(1))])),
        );
        assert_eq!(
            Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                ValueType::Int,
            )),
            Expr::from(Value::List(ListValue::new(
                ValueType::Int,
                vec![Value::Int(BigInt::from(1))],
            ))),
        );
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
                BoolCaseBranches::Float {
                    true_: FloatExpr::value(1.5),
                    false_: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::bool_case(
                BoolExpr::value(true),
                FloatExpr::value(1.5),
                FloatExpr::value(0.5),
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
                BoolCaseBranches::List {
                    true_: list_expr(),
                    false_: list_expr(),
                },
            ),
            Expr::list(ListExpr::bool_case(
                BoolExpr::value(true),
                list_expr(),
                list_expr(),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::IntFunction {
                    true_: int_function_expr(),
                    false_: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_expr(),
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::StringFunction {
                    true_: string_function_expr(),
                    false_: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                string_function_expr(),
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::FloatFunction {
                    true_: float_function_expr(),
                    false_: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::bool_case(
                BoolExpr::value(true),
                float_function_expr(),
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::BoolFunction {
                    true_: bool_function_expr(),
                    false_: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::bool_case(
                BoolExpr::value(true),
                bool_function_expr(),
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::ListFunction {
                    true_: list_function_expr(),
                    false_: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::bool_case(
                BoolExpr::value(true),
                list_function_expr(),
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::NilFunction {
                    true_: nil_function_expr(),
                    false_: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::bool_case(
                BoolExpr::value(true),
                nil_function_expr(),
                nil_function_expr(),
            ))),
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
                IntCaseBranches::Float {
                    clauses: vec![(BigInt::from(1), FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
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
                IntCaseBranches::List {
                    clauses: vec![(BigInt::from(1), list_expr())],
                    fallback: list_expr(),
                },
            ),
            Expr::list(ListExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), list_expr())],
                list_expr(),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::IntFunction {
                    clauses: vec![(BigInt::from(1), int_function_expr())],
                    fallback: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), int_function_expr())],
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::StringFunction {
                    clauses: vec![(BigInt::from(1), string_function_expr())],
                    fallback: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), string_function_expr())],
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::FloatFunction {
                    clauses: vec![(BigInt::from(1), float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), float_function_expr())],
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::BoolFunction {
                    clauses: vec![(BigInt::from(1), bool_function_expr())],
                    fallback: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), bool_function_expr())],
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::ListFunction {
                    clauses: vec![(BigInt::from(1), list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::NilFunction {
                    clauses: vec![(BigInt::from(1), nil_function_expr())],
                    fallback: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), nil_function_expr())],
                nil_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_float_case_shapes() {
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Int {
                    clauses: vec![(1.0, IntExpr::value(BigInt::from(10)))],
                    fallback: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, IntExpr::value(BigInt::from(10)))],
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::String {
                    clauses: vec![(1.0, StringExpr::value("one".into()))],
                    fallback: StringExpr::value("other".into()),
                },
            ),
            Expr::string(StringExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, StringExpr::value("one".into()))],
                StringExpr::value("other".into()),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Float {
                    clauses: vec![(1.0, FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Bool {
                    clauses: vec![(1.0, BoolExpr::value(true))],
                    fallback: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, BoolExpr::value(true))],
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Nil {
                    clauses: vec![(1.0, NilExpr::value())],
                    fallback: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, NilExpr::value())],
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::List {
                    clauses: vec![(1.0, list_expr())],
                    fallback: list_expr(),
                },
            ),
            Expr::list(ListExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, list_expr())],
                list_expr(),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::IntFunction {
                    clauses: vec![(1.0, int_function_expr())],
                    fallback: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, int_function_expr())],
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::StringFunction {
                    clauses: vec![(1.0, string_function_expr())],
                    fallback: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, string_function_expr())],
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::FloatFunction {
                    clauses: vec![(1.0, float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_function_expr())],
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::BoolFunction {
                    clauses: vec![(1.0, bool_function_expr())],
                    fallback: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, bool_function_expr())],
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::NilFunction {
                    clauses: vec![(1.0, nil_function_expr())],
                    fallback: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, nil_function_expr())],
                nil_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::ListFunction {
                    clauses: vec![(1.0, list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::FunctionFunction {
                    clauses: vec![(1.0, function_function_expr())],
                    fallback: function_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, function_function_expr())],
                function_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_string_case_shapes() {
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::Float {
                    clauses: vec![("one".into(), FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::List {
                    clauses: vec![("one".into(), list_expr())],
                    fallback: list_expr(),
                },
            ),
            Expr::list(ListExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), list_expr())],
                list_expr(),
            )),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::ListFunction {
                    clauses: vec![("one".into(), list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::FloatFunction {
                    clauses: vec![("one".into(), float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), float_function_expr())],
                float_function_expr(),
            ))),
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
        assert_eq!(Expr::from(Value::Float(1.5)).value_type(), ValueType::Float);
        assert_eq!(Expr::from(Value::Bool(true)).value_type(), ValueType::Bool);
        assert_eq!(Expr::from(Value::Nil).value_type(), ValueType::Nil);
        assert_eq!(
            Expr::from(Value::Tuple(vec![Value::Int(BigInt::from(1))])).value_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            Expr::from(Value::List(ListValue::new(
                ValueType::Int,
                vec![Value::Int(BigInt::from(1))],
            )))
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            Expr::from(Value::Function(function_value())).value_type(),
            ValueType::Function(Box::new(function_type())),
        );
    }

    #[test]
    fn expr_into_typed_expression() {
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).into_int(),
            Some(IntExpr::value(BigInt::from(1))),
        );
        assert_eq!(
            Expr::from(Value::String("geam".into())).into_string(),
            Some(StringExpr::value("geam".into())),
        );
        assert_eq!(
            Expr::from(Value::Float(1.5)).into_float(),
            Some(FloatExpr::value(1.5)),
        );
        assert_eq!(Expr::from(Value::Int(BigInt::from(1))).into_float(), None);
        assert_eq!(
            Expr::from(Value::Bool(true)).into_bool(),
            Some(BoolExpr::value(true)),
        );
        assert_eq!(Expr::from(Value::Nil).into_nil(), Some(NilExpr::value()));
        assert_eq!(
            Expr::from(Value::Tuple(vec![Value::Int(BigInt::from(1))])).into_tuple(),
            Some(TupleExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                vec![ValueType::Int],
            )),
        );
        assert_eq!(
            Expr::from(Value::List(ListValue::new(
                ValueType::Int,
                vec![Value::Int(BigInt::from(1))],
            )))
            .into_list(),
            Some(ListExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                ValueType::Int,
            )),
        );
        assert_eq!(Expr::from(Value::Int(BigInt::from(1))).into_list(), None);
        assert_eq!(Expr::from(Value::Nil).into_int(), None);
        assert_eq!(Expr::from(Value::Int(BigInt::from(1))).into_nil(), None);
        assert_eq!(
            Expr::from(Value::Int(BigInt::from(1))).into_function(),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value())).into_function(),
            Some(FunctionExpr::value(function_value())),
        );
    }

    #[test]
    fn expr_into_call_arg() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            Some(CallArg::int(IntLocalId(0), IntExpr::value(BigInt::from(1)),)),
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into()))
                .into_call_arg(&ParamLocal::string(StringLocalId(0))),
            Some(CallArg::string(
                StringLocalId(0),
                StringExpr::value("geam".into()),
            )),
        );
        assert_eq!(
            Expr::float(FloatExpr::value(1.5)).into_call_arg(&ParamLocal::float(FloatLocalId(0))),
            Some(CallArg::float(FloatLocalId(0), FloatExpr::value(1.5))),
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)).into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            Some(CallArg::bool(BoolLocalId(0), BoolExpr::value(true))),
        );
        assert_eq!(
            Expr::nil(NilExpr::value()).into_call_arg(&ParamLocal::nil(NilLocalId(0))),
            Some(CallArg::nil(NilLocalId(0), NilExpr::value())),
        );
        assert_eq!(
            tuple_expr().into_call_arg(&ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])),
            Some(CallArg::tuple(
                TupleLocalId(0),
                TupleExpr::value(
                    vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                    vec![ValueType::Int],
                ),
            )),
        );
        assert_eq!(
            Expr::list(list_expr())
                .into_call_arg(&ParamLocal::list(ListLocalId(0), ValueType::Int)),
            Some(CallArg::list(ListLocalId(0), list_expr())),
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value())).into_call_arg(
                &ParamLocal::int_function(
                    crate::plan::IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
            ),
            Some(CallArg::int_function(
                crate::plan::IntFunctionLocalId(0),
                int_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(string_function_expr())).into_call_arg(
                &ParamLocal::string_function(
                    crate::plan::StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                )
            ),
            Some(CallArg::string_function(
                crate::plan::StringFunctionLocalId(0),
                string_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::float(float_function_expr())).into_call_arg(
                &ParamLocal::float_function(
                    FloatFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Float], ValueType::Float),
                )
            ),
            Some(CallArg::float_function(
                FloatFunctionLocalId(0),
                float_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(bool_function_expr())).into_call_arg(
                &ParamLocal::bool_function(
                    crate::plan::BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                )
            ),
            Some(CallArg::bool_function(
                crate::plan::BoolFunctionLocalId(0),
                bool_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(nil_function_expr())).into_call_arg(
                &ParamLocal::nil_function(
                    crate::plan::NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                )
            ),
            Some(CallArg::nil_function(
                crate::plan::NilFunctionLocalId(0),
                nil_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::tuple(tuple_function_expr())).into_call_arg(
                &ParamLocal::tuple_function(TupleFunctionLocalId(0), tuple_function_type())
            ),
            Some(CallArg::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::list(list_function_expr())).into_call_arg(
                &ParamLocal::list_function(ListFunctionLocalId(0), list_function_type())
            ),
            Some(CallArg::list_function(
                ListFunctionLocalId(0),
                list_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::function(function_function_expr())).into_call_arg(
                &ParamLocal::function_function(
                    crate::plan::FunctionFunctionLocalId(0),
                    function_function_type(),
                )
            ),
            Some(CallArg::function_function(
                crate::plan::FunctionFunctionLocalId(0),
                function_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                function_type(),
            )))
            .into_call_arg(&ParamLocal::int_function(
                crate::plan::IntFunctionLocalId(0),
                function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(malformed_bool_function_expr(
                string_function_type(),
            )))
            .into_call_arg(&ParamLocal::string_function(
                crate::plan::StringFunctionLocalId(0),
                string_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(malformed_nil_function_expr(
                bool_function_type(),
            )))
            .into_call_arg(&ParamLocal::bool_function(
                crate::plan::BoolFunctionLocalId(0),
                bool_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                function_function_type(),
            )))
            .into_call_arg(&ParamLocal::function_function(
                crate::plan::FunctionFunctionLocalId(0),
                function_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                nil_function_type(),
            )))
            .into_call_arg(&ParamLocal::nil_function(
                crate::plan::NilFunctionLocalId(0),
                nil_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                tuple_function_type(),
            )))
            .into_call_arg(&ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::value(function_value()))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            None,
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            None,
        );
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(IntLocalId(0))],
        )
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn tuple_expr() -> Expr {
        Expr::tuple(TupleExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            vec![ValueType::Int],
        ))
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            ValueType::Int,
        )
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])],
            vec![ValueType::Int],
        ))
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId(0),
            vec![ParamLocal::list(ListLocalId(0), ValueType::Int)],
            ValueType::Int,
        ))
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            function_type(),
        ))
    }

    fn malformed_int_function_expr(type_: FunctionType) -> IntFunctionExpr {
        IntFunctionExpr::local_get(crate::plan::IntFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_string_function_expr(type_: FunctionType) -> StringFunctionExpr {
        StringFunctionExpr::local_get(crate::plan::StringFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_bool_function_expr(type_: FunctionType) -> BoolFunctionExpr {
        BoolFunctionExpr::local_get(crate::plan::BoolFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_nil_function_expr(type_: FunctionType) -> NilFunctionExpr {
        NilFunctionExpr::local_get(crate::plan::NilFunctionLocalId(0), "f".into(), type_)
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn bool_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(vec![ValueType::Int])],
            ValueType::Tuple(vec![ValueType::Int]),
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::List(Box::new(ValueType::Int))],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn function_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Function(Box::new(function_type())))
    }
}
