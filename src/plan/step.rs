use super::expression::{BoolExpr, Expr, IntExpr, NilExpr, StringExpr};
use super::id::{BoolLocalId, IntLocalId, NilLocalId, StringLocalId};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    LetInt {
        local: IntLocalId,
        name: EcoString,
        value: IntExpr,
    },
    LetString {
        local: StringLocalId,
        name: EcoString,
        value: StringExpr,
    },
    LetBool {
        local: BoolLocalId,
        name: EcoString,
        value: BoolExpr,
    },
    LetNil {
        local: NilLocalId,
        name: EcoString,
        value: NilExpr,
    },
    Evaluate(Expr),
}
