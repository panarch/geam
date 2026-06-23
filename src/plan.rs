mod expression;
mod frame;
mod function;
mod id;
mod runtime;
mod step;
mod value;

use self::runtime::RuntimePlan;
use ecow::EcoString;
use std::fmt;

pub use expression::{BoolExpr, CallArg, Expr, IntExpr, NilExpr, StringExpr};
pub(crate) use expression::{
    BoolExprKind, CallArgKind, ExprKind, IntExprKind, NilExprKind, StringExprKind,
};
pub(crate) use frame::FrameLayout;
pub(crate) use function::RuntimeFunction;
pub use function::{FunctionPlan, Param};
pub(crate) use id::RuntimeFunctionId;
pub use id::{
    BoolFunctionId, BoolLocalId, FunctionId, IntFunctionId, IntLocalId, LocalId, NilFunctionId,
    NilLocalId, StringFunctionId, StringLocalId,
};
pub use step::Step;
pub(crate) use step::StepKind;
pub use value::{Value, ValueType};

pub struct ExecutionPlan {
    module: EcoString,
    main: FunctionPlan,
    functions: Vec<FunctionPlan>,
    runtime: RuntimePlan,
}

impl ExecutionPlan {
    pub(crate) fn new(module: EcoString, main: FunctionPlan, functions: Vec<FunctionPlan>) -> Self {
        let runtime = RuntimePlan::new(&main, &functions);

        Self {
            module,
            main,
            functions,
            runtime,
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn main_function(&self) -> &FunctionPlan {
        &self.main
    }

    pub fn functions(&self) -> &[FunctionPlan] {
        &self.functions
    }

    pub(crate) fn main_runtime(&self) -> RuntimeFunctionId {
        self.runtime.main()
    }

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &RuntimeFunction<IntExpr> {
        self.runtime.int_function(id)
    }

    pub(crate) fn string_function(&self, id: StringFunctionId) -> &RuntimeFunction<StringExpr> {
        self.runtime.string_function(id)
    }

    pub(crate) fn bool_function(&self, id: BoolFunctionId) -> &RuntimeFunction<BoolExpr> {
        self.runtime.bool_function(id)
    }

    pub(crate) fn nil_function(&self, id: NilFunctionId) -> &RuntimeFunction<NilExpr> {
        self.runtime.nil_function(id)
    }
}

impl fmt::Debug for ExecutionPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExecutionPlan")
            .field("module", &self.module)
            .field("main", &self.main)
            .field("functions", &self.functions)
            .finish()
    }
}

impl PartialEq for ExecutionPlan {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.main == other.main && self.functions == other.functions
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionPlan, Expr, FunctionId, FunctionPlan, IntExpr};
    use num_bigint::BigInt;

    #[test]
    fn execution_plan_accessors() {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            Expr::int(IntExpr::value(BigInt::from(1))),
        );
        let helper = FunctionPlan::new(
            FunctionId::new(1),
            "helper".into(),
            Vec::new(),
            Vec::new(),
            Expr::int(IntExpr::value(BigInt::from(2))),
        );
        let plan = ExecutionPlan::new("main".into(), main, vec![helper]);

        assert_eq!(plan.module(), "main");
        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
    }

    #[test]
    fn execution_plan_debug_surface() {
        let plan = ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                Expr::int(IntExpr::value(BigInt::from(1))),
            ),
            Vec::new(),
        );
        let debug = format!("{plan:?}");

        assert!(debug.contains("ExecutionPlan"));
        assert!(debug.contains("module"));
        assert!(debug.contains("main"));
        assert!(debug.contains("functions"));
        assert!(!debug.contains("runtime"));
    }
}
