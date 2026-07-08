use super::FrameLayout;
use super::expression::{BoolExpr, CallArg, FloatExpr, IntExpr, NilExpr, StringExpr, TupleExpr};
use super::id::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolLocalId,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionId,
    IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntLocalId, ListFunctionFunctionId,
    ListFunctionId, ListFunctionLocalId, ListLocalId, NilFunctionFunctionId, NilFunctionId,
    NilFunctionLocalId, NilLocalId, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleLocalId,
};
use super::step::Step;
use super::value::{FunctionType, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub struct FunctionPlan {
    id: FunctionId,
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: ReturnExpr,
    frame_layout: FrameLayout,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    local: ParamLocal,
    binding: ParamBinding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamBinding {
    Named(EcoString),
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple {
        local: TupleLocalId,
        type_: Vec<ValueType>,
    },
    List {
        local: ListLocalId,
        element_type: ValueType,
    },
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    StringFunction {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    NilFunction {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    ListFunction {
        local: ListFunctionLocalId,
        type_: FunctionType,
    },
    FunctionFunction {
        local: FunctionFunctionLocalId,
        type_: FunctionType,
    },
}

pub(crate) struct RuntimeFunction<Return> {
    frame_layout: FrameLayout,
    steps: Vec<Step>,
    return_: Return,
}

pub(crate) type IntReturn = ReturnBody<IntExpr, IntFunctionId>;
pub(crate) type FloatReturn = ReturnBody<FloatExpr, FloatFunctionId>;
pub(crate) type StringReturn = ReturnBody<StringExpr, StringFunctionId>;
pub(crate) type BoolReturn = ReturnBody<BoolExpr, BoolFunctionId>;
pub(crate) type NilReturn = ReturnBody<NilExpr, NilFunctionId>;
pub(crate) type TupleReturn = ReturnBody<TupleExpr, TupleFunctionId>;
pub(crate) type ListReturn = ReturnBody<super::ListExpr, ListFunctionId>;
pub(crate) type IntFunctionReturn = ReturnBody<super::IntFunctionExpr, IntFunctionFunctionId>;
pub(crate) type FloatFunctionReturn = ReturnBody<super::FloatFunctionExpr, FloatFunctionFunctionId>;
pub(crate) type StringFunctionReturn =
    ReturnBody<super::StringFunctionExpr, StringFunctionFunctionId>;
pub(crate) type BoolFunctionReturn = ReturnBody<super::BoolFunctionExpr, BoolFunctionFunctionId>;
pub(crate) type NilFunctionReturn = ReturnBody<super::NilFunctionExpr, NilFunctionFunctionId>;
pub(crate) type TupleFunctionReturn = ReturnBody<super::TupleFunctionExpr, TupleFunctionFunctionId>;
pub(crate) type ListFunctionReturn = ReturnBody<super::ListFunctionExpr, ListFunctionFunctionId>;
pub(crate) type FunctionFunctionReturn =
    ReturnBody<super::FunctionFunctionExpr, FunctionFunctionFunctionId>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReturnBody<Expression, Function> {
    kind: ReturnBodyKind<Expression, Function>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ReturnBodyKind<Expression, Function> {
    Expr(Expression),
    TailCall {
        function: Function,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: BoolExpr,
        true_: Box<ReturnBody<Expression, Function>>,
        false_: Box<ReturnBody<Expression, Function>>,
    },
    IntCase {
        subject: IntExpr,
        clauses: Vec<(BigInt, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    FloatCase {
        subject: FloatExpr,
        clauses: Vec<(f64, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    StringCase {
        subject: StringExpr,
        clauses: Vec<(EcoString, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ReturnBody<Expression, Function>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpr {
    kind: ReturnExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ReturnExprKind {
    Int {
        runtime_id: IntFunctionId,
        body: IntReturn,
    },
    Float {
        runtime_id: FloatFunctionId,
        body: FloatReturn,
    },
    String {
        runtime_id: StringFunctionId,
        body: StringReturn,
    },
    Bool {
        runtime_id: BoolFunctionId,
        body: BoolReturn,
    },
    Nil {
        runtime_id: NilFunctionId,
        body: NilReturn,
    },
    Tuple {
        runtime_id: TupleFunctionId,
        type_: Vec<ValueType>,
        body: TupleReturn,
    },
    List {
        runtime_id: ListFunctionId,
        element_type: ValueType,
        body: ListReturn,
    },
    IntFunction {
        runtime_id: IntFunctionFunctionId,
        type_: FunctionType,
        body: IntFunctionReturn,
    },
    FloatFunction {
        runtime_id: FloatFunctionFunctionId,
        type_: FunctionType,
        body: FloatFunctionReturn,
    },
    StringFunction {
        runtime_id: StringFunctionFunctionId,
        type_: FunctionType,
        body: StringFunctionReturn,
    },
    BoolFunction {
        runtime_id: BoolFunctionFunctionId,
        type_: FunctionType,
        body: BoolFunctionReturn,
    },
    NilFunction {
        runtime_id: NilFunctionFunctionId,
        type_: FunctionType,
        body: NilFunctionReturn,
    },
    TupleFunction {
        runtime_id: TupleFunctionFunctionId,
        type_: FunctionType,
        body: TupleFunctionReturn,
    },
    ListFunction {
        runtime_id: ListFunctionFunctionId,
        type_: FunctionType,
        body: ListFunctionReturn,
    },
    FunctionFunction {
        runtime_id: FunctionFunctionFunctionId,
        type_: FunctionType,
        body: FunctionFunctionReturn,
    },
}

impl FunctionPlan {
    pub(crate) fn new(
        id: FunctionId,
        name: EcoString,
        params: Vec<Param>,
        steps: Vec<Step>,
        return_: ReturnExpr,
    ) -> Self {
        let frame_layout = FrameLayout::from_function_parts(&params, &steps, &return_);

        Self {
            id,
            name,
            params,
            steps,
            return_,
            frame_layout,
        }
    }

    pub fn id(&self) -> FunctionId {
        self.id
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn return_(&self) -> &ReturnExpr {
        &self.return_
    }

    pub(crate) fn frame_layout(&self) -> FrameLayout {
        self.frame_layout.clone()
    }
}

impl ReturnExpr {
    #[cfg(test)]
    pub(crate) fn int(runtime_id: IntFunctionId, expression: IntExpr) -> Self {
        Self::int_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn int_body(runtime_id: IntFunctionId, body: IntReturn) -> Self {
        Self {
            kind: ReturnExprKind::Int { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn float(runtime_id: FloatFunctionId, expression: FloatExpr) -> Self {
        Self::float_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn float_body(runtime_id: FloatFunctionId, body: FloatReturn) -> Self {
        Self {
            kind: ReturnExprKind::Float { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn string(runtime_id: StringFunctionId, expression: StringExpr) -> Self {
        Self::string_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn string_body(runtime_id: StringFunctionId, body: StringReturn) -> Self {
        Self {
            kind: ReturnExprKind::String { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool(runtime_id: BoolFunctionId, expression: BoolExpr) -> Self {
        Self::bool_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn bool_body(runtime_id: BoolFunctionId, body: BoolReturn) -> Self {
        Self {
            kind: ReturnExprKind::Bool { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil(runtime_id: NilFunctionId, expression: NilExpr) -> Self {
        Self::nil_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn nil_body(runtime_id: NilFunctionId, body: NilReturn) -> Self {
        Self {
            kind: ReturnExprKind::Nil { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn tuple(runtime_id: TupleFunctionId, expression: TupleExpr) -> Self {
        let type_ = expression.type_().to_vec();
        Self::tuple_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_body(
        runtime_id: TupleFunctionId,
        type_: Vec<ValueType>,
        body: TupleReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::Tuple {
                runtime_id,
                type_,
                body,
            },
        }
    }

    pub(crate) fn list_body(
        runtime_id: ListFunctionId,
        element_type: ValueType,
        body: ListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::List {
                runtime_id,
                element_type,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function(
        runtime_id: IntFunctionFunctionId,
        expression: super::IntFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::int_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn int_function_body(
        runtime_id: IntFunctionFunctionId,
        type_: FunctionType,
        body: IntFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::IntFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn float_function(
        runtime_id: FloatFunctionFunctionId,
        expression: super::FloatFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::float_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn float_function_body(
        runtime_id: FloatFunctionFunctionId,
        type_: FunctionType,
        body: FloatFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FloatFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn string_function(
        runtime_id: StringFunctionFunctionId,
        expression: super::StringFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::string_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn string_function_body(
        runtime_id: StringFunctionFunctionId,
        type_: FunctionType,
        body: StringFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::StringFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool_function(
        runtime_id: BoolFunctionFunctionId,
        expression: super::BoolFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::bool_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn bool_function_body(
        runtime_id: BoolFunctionFunctionId,
        type_: FunctionType,
        body: BoolFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BoolFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil_function(
        runtime_id: NilFunctionFunctionId,
        expression: super::NilFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::nil_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn nil_function_body(
        runtime_id: NilFunctionFunctionId,
        type_: FunctionType,
        body: NilFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::NilFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn tuple_function(
        runtime_id: TupleFunctionFunctionId,
        expression: super::TupleFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::tuple_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_function_body(
        runtime_id: TupleFunctionFunctionId,
        type_: FunctionType,
        body: TupleFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::TupleFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn list_function(
        runtime_id: ListFunctionFunctionId,
        expression: super::ListFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::list_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn list_function_body(
        runtime_id: ListFunctionFunctionId,
        type_: FunctionType,
        body: ListFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::ListFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn function_function(
        runtime_id: FunctionFunctionFunctionId,
        expression: super::FunctionFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::function_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn function_function_body(
        runtime_id: FunctionFunctionFunctionId,
        type_: FunctionType,
        body: FunctionFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FunctionFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Int { .. } => ValueType::Int,
            ReturnExprKind::Float { .. } => ValueType::Float,
            ReturnExprKind::String { .. } => ValueType::String,
            ReturnExprKind::Bool { .. } => ValueType::Bool,
            ReturnExprKind::Nil { .. } => ValueType::Nil,
            ReturnExprKind::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            ReturnExprKind::List { element_type, .. } => {
                ValueType::List(Box::new(element_type.clone()))
            }
            ReturnExprKind::IntFunction { type_, .. }
            | ReturnExprKind::FloatFunction { type_, .. }
            | ReturnExprKind::StringFunction { type_, .. }
            | ReturnExprKind::BoolFunction { type_, .. }
            | ReturnExprKind::NilFunction { type_, .. }
            | ReturnExprKind::TupleFunction { type_, .. }
            | ReturnExprKind::ListFunction { type_, .. }
            | ReturnExprKind::FunctionFunction { type_, .. } => {
                ValueType::Function(Box::new(type_.clone()))
            }
        }
    }

    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        match self.kind() {
            ReturnExprKind::Int { runtime_id, .. } => RuntimeFunctionId::Int(*runtime_id),
            ReturnExprKind::Float { runtime_id, .. } => RuntimeFunctionId::Float(*runtime_id),
            ReturnExprKind::String { runtime_id, .. } => RuntimeFunctionId::String(*runtime_id),
            ReturnExprKind::Bool { runtime_id, .. } => RuntimeFunctionId::Bool(*runtime_id),
            ReturnExprKind::Nil { runtime_id, .. } => RuntimeFunctionId::Nil(*runtime_id),
            ReturnExprKind::Tuple {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Tuple {
                id: *runtime_id,
                return_type: type_.clone(),
            },
            ReturnExprKind::List {
                runtime_id,
                element_type,
                ..
            } => RuntimeFunctionId::List {
                id: *runtime_id,
                return_type: Box::new(element_type.clone()),
            },
            ReturnExprKind::IntFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::FloatFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Float(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::StringFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::BoolFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::NilFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::TupleFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Tuple(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::ListFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::FunctionFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(*runtime_id),
                return_type: type_.clone(),
            },
        }
    }
}

impl<Expression, Function> ReturnBody<Expression, Function> {
    pub(crate) fn expr(expression: Expression) -> Self {
        Self {
            kind: ReturnBodyKind::Expr(expression),
        }
    }

    pub(crate) fn tail_call(function: Function, args: Vec<CallArg>) -> Self {
        Self {
            kind: ReturnBodyKind::TailCall { function, args },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self {
            kind: ReturnBodyKind::BoolCase {
                subject,
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self {
            kind: ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            kind: ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            kind: ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            kind: ReturnBodyKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &ReturnBodyKind<Expression, Function> {
        &self.kind
    }
}

impl<Return> RuntimeFunction<Return> {
    pub(crate) fn new(frame_layout: FrameLayout, steps: Vec<Step>, return_: Return) -> Self {
        Self {
            frame_layout,
            steps,
            return_,
        }
    }

    pub(crate) fn frame_layout(&self) -> FrameLayout {
        self.frame_layout.clone()
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}

impl Param {
    pub(crate) fn named(local: ParamLocal, name: EcoString) -> Self {
        Self {
            local,
            binding: ParamBinding::Named(name),
        }
    }

    pub(crate) fn discard(local: ParamLocal) -> Self {
        Self {
            local,
            binding: ParamBinding::Discard,
        }
    }

    pub fn name(&self) -> Option<&EcoString> {
        match &self.binding {
            ParamBinding::Named(name) => Some(name),
            ParamBinding::Discard => None,
        }
    }

    pub fn binding(&self) -> &ParamBinding {
        &self.binding
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }
}

impl ParamLocal {
    pub(crate) fn int(local: IntLocalId) -> Self {
        Self::Int(local)
    }

    pub(crate) fn float(local: FloatLocalId) -> Self {
        Self::Float(local)
    }

    pub(crate) fn string(local: StringLocalId) -> Self {
        Self::String(local)
    }

    pub(crate) fn bool(local: BoolLocalId) -> Self {
        Self::Bool(local)
    }

    pub(crate) fn nil(local: NilLocalId) -> Self {
        Self::Nil(local)
    }

    pub(crate) fn tuple(local: TupleLocalId, type_: Vec<ValueType>) -> Self {
        Self::Tuple { local, type_ }
    }

    pub(crate) fn list(local: ListLocalId, element_type: ValueType) -> Self {
        Self::List {
            local,
            element_type,
        }
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, type_: FunctionType) -> Self {
        Self::IntFunction { local, type_ }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, type_: FunctionType) -> Self {
        Self::FloatFunction { local, type_ }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, type_: FunctionType) -> Self {
        Self::StringFunction { local, type_ }
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, type_: FunctionType) -> Self {
        Self::BoolFunction { local, type_ }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, type_: FunctionType) -> Self {
        Self::NilFunction { local, type_ }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, type_: FunctionType) -> Self {
        Self::TupleFunction { local, type_ }
    }

    pub(crate) fn list_function(local: ListFunctionLocalId, type_: FunctionType) -> Self {
        Self::ListFunction { local, type_ }
    }

    pub(crate) fn function_function(local: FunctionFunctionLocalId, type_: FunctionType) -> Self {
        Self::FunctionFunction { local, type_ }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            Self::List { element_type, .. } => ValueType::List(Box::new(element_type.clone())),
            Self::IntFunction { type_, .. }
            | Self::FloatFunction { type_, .. }
            | Self::StringFunction { type_, .. }
            | Self::BoolFunction { type_, .. }
            | Self::NilFunction { type_, .. }
            | Self::TupleFunction { type_, .. }
            | Self::ListFunction { type_, .. }
            | Self::FunctionFunction { type_, .. } => ValueType::Function(Box::new(type_.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionPlan, Param, ParamBinding, ParamLocal, ReturnExpr, RuntimeFunction};
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
        BoolFunctionValue, BoolLocalId, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId,
        FloatFunctionId, FloatFunctionLocalId, FloatFunctionValue, FloatLocalId, FrameLayout,
        FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionFunctionLocalId, FunctionFunctionValue, FunctionId, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntLocalId, ListExpr, ListFunctionExpr, ListFunctionFunctionId,
        ListFunctionId, ListFunctionLocalId, ListFunctionValue, ListLocalId, NilExpr,
        NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
        NilFunctionValue, StringExpr, StringFunctionExpr, StringFunctionFunctionId,
        StringFunctionId, StringFunctionLocalId, StringFunctionValue, TupleExpr, TupleFunctionExpr,
        TupleFunctionFunctionId, TupleFunctionId, TupleFunctionLocalId, TupleFunctionValue,
        ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_plan_accessors() {
        let param = Param::named(ParamLocal::int(IntLocalId(0)), "x".into());
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1)));
        let function = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            vec![param],
            Vec::new(),
            return_,
        );

        assert_eq!(function.id(), FunctionId::new(0));
        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 1);
        assert_eq!(function.params()[0].name(), Some(&"x".into()));
        assert_eq!(function.steps(), &[]);
        assert_eq!(
            function.return_(),
            &ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1)))
        );
        assert_eq!(function.frame_layout().ints(), 1);
    }

    #[test]
    fn return_expr_value_type() {
        assert_eq!(
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1))).value_type(),
            ValueType::Int,
        );
        assert_eq!(
            ReturnExpr::string(
                crate::plan::StringFunctionId(0),
                StringExpr::value("geam".into()),
            )
            .value_type(),
            ValueType::String,
        );
        assert_eq!(
            ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(1.0)).value_type(),
            ValueType::Float,
        );
        assert_eq!(
            ReturnExpr::bool(crate::plan::BoolFunctionId(0), BoolExpr::value(true)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ReturnExpr::nil(crate::plan::NilFunctionId(0), NilExpr::value()).value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ReturnExpr::tuple(
                TupleFunctionId(0),
                TupleExpr::value(
                    vec![crate::plan::Expr::int(IntExpr::value(BigInt::from(1)))],
                    vec![ValueType::Int],
                ),
            )
            .value_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ReturnExpr::list_body(
                ListFunctionId(0),
                ValueType::Int,
                super::ReturnBody::expr(ListExpr::value(
                    vec![crate::plan::Expr::int(IntExpr::value(BigInt::from(1)))],
                    ValueType::Int,
                )),
            )
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            ReturnExpr::int_function(
                IntFunctionFunctionId(0),
                IntFunctionExpr::value(IntFunctionValue::new(IntFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        assert_eq!(
            ReturnExpr::string_function(
                StringFunctionFunctionId(0),
                StringFunctionExpr::value(StringFunctionValue::new(
                    StringFunctionId(0),
                    Vec::new(),
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
        );
        assert_eq!(
            ReturnExpr::float_function(
                FloatFunctionFunctionId(0),
                FloatFunctionExpr::value(FloatFunctionValue::new(FloatFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float))),
        );
        assert_eq!(
            ReturnExpr::bool_function(
                BoolFunctionFunctionId(0),
                BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        );
        assert_eq!(
            ReturnExpr::nil_function(
                NilFunctionFunctionId(0),
                NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
        );
        assert_eq!(
            ReturnExpr::tuple_function(
                TupleFunctionFunctionId(0),
                TupleFunctionExpr::value(TupleFunctionValue::new(
                    TupleFunctionId(0),
                    Vec::new(),
                    vec![ValueType::Int],
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            ))),
        );
        assert_eq!(
            ReturnExpr::list_function(
                ListFunctionFunctionId(0),
                ListFunctionExpr::value(ListFunctionValue::new(
                    ListFunctionId(0),
                    Vec::new(),
                    ValueType::Int,
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ))),
        );
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            ReturnExpr::function_function(
                FunctionFunctionFunctionId(0),
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    return_type.clone(),
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(return_type)),
            ))),
        );
    }

    #[test]
    fn param_binding_accessors() {
        let named = Param::named(ParamLocal::int(IntLocalId(0)), "x".into());
        let discard = Param::discard(ParamLocal::int(IntLocalId(1)));

        assert_eq!(named.name(), Some(&"x".into()));
        assert_eq!(named.binding(), &ParamBinding::Named("x".into()));
        assert_eq!(discard.name(), None);
        assert_eq!(discard.binding(), &ParamBinding::Discard);
    }

    #[test]
    fn param_local_value_type() {
        assert_eq!(ParamLocal::int(IntLocalId(0)).value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::string(crate::plan::StringLocalId(0)).value_type(),
            ValueType::String,
        );
        assert_eq!(
            ParamLocal::float(FloatLocalId(0)).value_type(),
            ValueType::Float,
        );
        assert_eq!(
            ParamLocal::bool(BoolLocalId(0)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ParamLocal::nil(crate::plan::NilLocalId(0)).value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ParamLocal::list(ListLocalId(0), ValueType::Int).value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            ParamLocal::int_function(
                IntFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        );
        assert_eq!(
            ParamLocal::string_function(
                StringFunctionLocalId(0),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::String],
                ValueType::String,
            ))),
        );
        assert_eq!(
            ParamLocal::float_function(
                FloatFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Float,
            ))),
        );
        assert_eq!(
            ParamLocal::bool_function(
                BoolFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Bool],
                ValueType::Bool,
            ))),
        );
        assert_eq!(
            ParamLocal::nil_function(
                NilFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Nil],
                ValueType::Nil,
            ))),
        );
        assert_eq!(
            ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                FunctionType::new(
                    vec![ValueType::Tuple(vec![ValueType::Int])],
                    ValueType::Tuple(vec![ValueType::String]),
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Tuple(vec![ValueType::Int])],
                ValueType::Tuple(vec![ValueType::String]),
            ))),
        );
        assert_eq!(
            ParamLocal::list_function(
                ListFunctionLocalId(0),
                FunctionType::new(
                    vec![ValueType::List(Box::new(ValueType::Int))],
                    ValueType::List(Box::new(ValueType::String)),
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::String)),
            ))),
        );
        assert_eq!(
            ParamLocal::function_function(
                FunctionFunctionLocalId(0),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ))),
        );
    }

    #[test]
    fn runtime_function_accessors() {
        let return_ = IntExpr::value(BigInt::from(1));
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        let function = RuntimeFunction::new(layout, Vec::new(), return_);

        assert_eq!(function.frame_layout().ints(), 1);
        assert_eq!(function.steps(), &[]);
        assert_eq!(function.return_(), &IntExpr::value(BigInt::from(1)));
    }
}
