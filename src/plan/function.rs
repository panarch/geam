use super::FrameLayout;
use super::expression::{BoolExpr, IntExpr, NilExpr, StringExpr};
use super::id::{FunctionId, LocalId};
use super::step::Step;
use super::value::ValueType;
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
    local: LocalId,
    name: EcoString,
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

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Int(_) => ValueType::Int,
            ReturnExprKind::String(_) => ValueType::String,
            ReturnExprKind::Bool(_) => ValueType::Bool,
            ReturnExprKind::Nil(_) => ValueType::Nil,
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
    pub(crate) fn new(local: LocalId, name: EcoString) -> Self {
        Self { local, name }
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub(crate) fn local(&self) -> LocalId {
        self.local
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionPlan, Param, ReturnExpr, RuntimeFunction};
    use crate::plan::{
        BoolExpr, FrameLayout, FunctionId, IntExpr, IntLocalId, LocalId, NilExpr, StringExpr,
        ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn function_plan_accessors() {
        let param = Param::new(LocalId::Int(IntLocalId(0)), "x".into());
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
    }

    #[test]
    fn param_name_accessor() {
        let param = Param::new(LocalId::Int(IntLocalId(0)), "x".into());

        assert_eq!(param.name(), "x");
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
