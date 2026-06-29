use super::FrameLayout;
use super::expression::{BoolExpr, IntExpr, NilExpr, StringExpr};
use super::id::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolLocalId,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionId,
    IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntLocalId, NilFunctionFunctionId,
    NilFunctionId, NilFunctionLocalId, NilLocalId, RuntimeFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringFunctionLocalId, StringLocalId,
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
    Int {
        runtime_id: IntFunctionId,
        expression: IntExpr,
    },
    String {
        runtime_id: StringFunctionId,
        expression: StringExpr,
    },
    Bool {
        runtime_id: BoolFunctionId,
        expression: BoolExpr,
    },
    Nil {
        runtime_id: NilFunctionId,
        expression: NilExpr,
    },
    IntFunction {
        runtime_id: IntFunctionFunctionId,
        expression: super::IntFunctionExpr,
    },
    StringFunction {
        runtime_id: StringFunctionFunctionId,
        expression: super::StringFunctionExpr,
    },
    BoolFunction {
        runtime_id: BoolFunctionFunctionId,
        expression: super::BoolFunctionExpr,
    },
    NilFunction {
        runtime_id: NilFunctionFunctionId,
        expression: super::NilFunctionExpr,
    },
    FunctionFunction {
        runtime_id: FunctionFunctionFunctionId,
        expression: super::FunctionFunctionExpr,
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
        self.frame_layout
    }
}

impl ReturnExpr {
    pub(crate) fn int(runtime_id: IntFunctionId, expression: IntExpr) -> Self {
        Self {
            kind: ReturnExprKind::Int {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn string(runtime_id: StringFunctionId, expression: StringExpr) -> Self {
        Self {
            kind: ReturnExprKind::String {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn bool(runtime_id: BoolFunctionId, expression: BoolExpr) -> Self {
        Self {
            kind: ReturnExprKind::Bool {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn nil(runtime_id: NilFunctionId, expression: NilExpr) -> Self {
        Self {
            kind: ReturnExprKind::Nil {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn int_function(
        runtime_id: IntFunctionFunctionId,
        expression: super::IntFunctionExpr,
    ) -> Self {
        Self {
            kind: ReturnExprKind::IntFunction {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn string_function(
        runtime_id: StringFunctionFunctionId,
        expression: super::StringFunctionExpr,
    ) -> Self {
        Self {
            kind: ReturnExprKind::StringFunction {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn bool_function(
        runtime_id: BoolFunctionFunctionId,
        expression: super::BoolFunctionExpr,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BoolFunction {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn nil_function(
        runtime_id: NilFunctionFunctionId,
        expression: super::NilFunctionExpr,
    ) -> Self {
        Self {
            kind: ReturnExprKind::NilFunction {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn function_function(
        runtime_id: FunctionFunctionFunctionId,
        expression: super::FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FunctionFunction {
                runtime_id,
                expression,
            },
        }
    }

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Int { .. } => ValueType::Int,
            ReturnExprKind::String { .. } => ValueType::String,
            ReturnExprKind::Bool { .. } => ValueType::Bool,
            ReturnExprKind::Nil { .. } => ValueType::Nil,
            ReturnExprKind::IntFunction { expression, .. } => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
            ReturnExprKind::StringFunction { expression, .. } => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
            ReturnExprKind::BoolFunction { expression, .. } => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
            ReturnExprKind::NilFunction { expression, .. } => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
            ReturnExprKind::FunctionFunction { expression, .. } => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
        }
    }

    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        match self.kind() {
            ReturnExprKind::Int { runtime_id, .. } => RuntimeFunctionId::Int(*runtime_id),
            ReturnExprKind::String { runtime_id, .. } => RuntimeFunctionId::String(*runtime_id),
            ReturnExprKind::Bool { runtime_id, .. } => RuntimeFunctionId::Bool(*runtime_id),
            ReturnExprKind::Nil { runtime_id, .. } => RuntimeFunctionId::Nil(*runtime_id),
            ReturnExprKind::IntFunction {
                runtime_id,
                expression,
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(*runtime_id),
                return_type: expression.type_().clone(),
            },
            ReturnExprKind::StringFunction {
                runtime_id,
                expression,
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(*runtime_id),
                return_type: expression.type_().clone(),
            },
            ReturnExprKind::BoolFunction {
                runtime_id,
                expression,
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(*runtime_id),
                return_type: expression.type_().clone(),
            },
            ReturnExprKind::NilFunction {
                runtime_id,
                expression,
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(*runtime_id),
                return_type: expression.type_().clone(),
            },
            ReturnExprKind::FunctionFunction {
                runtime_id,
                expression,
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(*runtime_id),
                return_type: expression.type_().clone(),
            },
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
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
        BoolFunctionValue, BoolLocalId, FrameLayout, FunctionFunctionExpr,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionValue, FunctionId,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntFunctionValue, IntLocalId, NilExpr, NilFunctionExpr,
        NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilFunctionValue, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
        StringFunctionValue, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_plan_accessors() {
        let param = Param::new(ParamLocal::int(IntLocalId(0)), "x".into());
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
        assert_eq!(function.params()[0].name(), "x");
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
            ReturnExpr::bool(crate::plan::BoolFunctionId(0), BoolExpr::value(true)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ReturnExpr::nil(crate::plan::NilFunctionId(0), NilExpr::value()).value_type(),
            ValueType::Nil,
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
