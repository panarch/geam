use super::expression::Expr;
use super::id::{FunctionId, LocalId};
use super::step::Step;
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionPlan {
    pub id: FunctionId,
    pub name: EcoString,
    pub params: Vec<Param>,
    pub steps: Vec<Step>,
    pub return_: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub local: LocalId,
    pub name: EcoString,
}
