use super::FrameLayout;
use super::expression::{BoolExpr, FunctionExpr, IntExpr, NilExpr, StringExpr};
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FunctionFunctionLocalId, FunctionId, IntFunctionLocalId,
    IntLocalId, NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
};
use super::step::Step;
use super::value::{FunctionType, ValueType};
use ecow::EcoString;

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
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Int(IntLocalId),
    String(StringLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    IntFunction {
        local: IntFunctionLocalId,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpr {
    kind: ReturnExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ReturnExprKind {
    Int(IntExpr),
    String(StringExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    Function(FunctionExpr),
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
        self.frame_layout
    }
}

impl ReturnExpr {
    pub(crate) fn int(expression: IntExpr) -> Self {
        Self {
            kind: ReturnExprKind::Int(expression),
        }
    }

    pub(crate) fn string(expression: StringExpr) -> Self {
        Self {
            kind: ReturnExprKind::String(expression),
        }
    }

    pub(crate) fn bool(expression: BoolExpr) -> Self {
        Self {
            kind: ReturnExprKind::Bool(expression),
        }
    }

    pub(crate) fn nil(expression: NilExpr) -> Self {
        Self {
            kind: ReturnExprKind::Nil(expression),
        }
    }

    pub(crate) fn function(expression: FunctionExpr) -> Self {
        Self {
            kind: ReturnExprKind::Function(expression),
        }
    }

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Int(_) => ValueType::Int,
            ReturnExprKind::String(_) => ValueType::String,
            ReturnExprKind::Bool(_) => ValueType::Bool,
            ReturnExprKind::Nil(_) => ValueType::Nil,
            ReturnExprKind::Function(expression) => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
        }
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
        self.frame_layout
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}

impl Param {
    pub(crate) fn new(local: ParamLocal, name: EcoString) -> Self {
        Self { local, name }
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }
}

impl ParamLocal {
    pub(crate) fn int(local: IntLocalId) -> Self {
        Self::Int(local)
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

    pub(crate) fn int_function(local: IntFunctionLocalId, type_: FunctionType) -> Self {
        Self::IntFunction { local, type_ }
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

    pub(crate) fn function_function(local: FunctionFunctionLocalId, type_: FunctionType) -> Self {
        Self::FunctionFunction { local, type_ }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::String(_) => ValueType::String,
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::IntFunction { type_, .. }
            | Self::StringFunction { type_, .. }
            | Self::BoolFunction { type_, .. }
            | Self::NilFunction { type_, .. }
            | Self::FunctionFunction { type_, .. } => ValueType::Function(Box::new(type_.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionPlan, Param, ParamLocal, ReturnExpr, RuntimeFunction};
    use crate::plan::{
        BoolExpr, BoolFunctionLocalId, BoolLocalId, FrameLayout, FunctionExpr, FunctionId,
        FunctionType, FunctionValue, IntExpr, IntFunctionId, IntFunctionLocalId, IntLocalId,
        NilExpr, NilFunctionLocalId, RuntimeFunctionId, StringExpr, StringFunctionLocalId,
        ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_plan_accessors() {
        let param = Param::new(ParamLocal::int(IntLocalId(0)), "x".into());
        let return_ = ReturnExpr::int(IntExpr::value(BigInt::from(1)));
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
        assert_eq!(function.params()[0].name(), "x");
        assert_eq!(function.steps(), &[]);
        assert_eq!(
            function.return_(),
            &ReturnExpr::int(IntExpr::value(BigInt::from(1)))
        );
        assert_eq!(function.frame_layout().ints(), 1);
    }

    #[test]
    fn return_expr_value_type() {
        assert_eq!(
            ReturnExpr::int(IntExpr::value(BigInt::from(1))).value_type(),
            ValueType::Int,
        );
        assert_eq!(
            ReturnExpr::string(StringExpr::value("geam".into())).value_type(),
            ValueType::String,
        );
        assert_eq!(
            ReturnExpr::bool(BoolExpr::value(true)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ReturnExpr::nil(NilExpr::value()).value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ReturnExpr::function(FunctionExpr::value(FunctionValue::new(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::new(),
            )))
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
    }

    #[test]
    fn param_name_accessor() {
        let param = Param::new(ParamLocal::int(IntLocalId(0)), "x".into());

        assert_eq!(param.name(), "x");
    }

    #[test]
    fn param_local_value_type() {
        assert_eq!(ParamLocal::int(IntLocalId(0)).value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::string(crate::plan::StringLocalId(0)).value_type(),
            ValueType::String,
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
