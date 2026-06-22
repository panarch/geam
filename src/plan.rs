mod expression;
mod function;
mod id;
mod step;
mod value;

use ecow::EcoString;

pub use expression::{BoolExpr, Expr, IntExpr, NilExpr, StringExpr};
pub use function::{FunctionPlan, Param};
pub use id::{BoolLocalId, FunctionId, IntLocalId, LocalId, NilLocalId, StringLocalId};
pub use step::Step;
pub use value::{Value, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePlan {
    pub module: EcoString,
    pub main: FunctionId,
    pub functions: Vec<FunctionPlan>,
}
