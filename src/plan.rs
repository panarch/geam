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

pub(crate) use expression::{
    BoolCaseBranches, BoolExprKind, BoolFunctionExprKind, CallArgKind, CaptureArg, CaptureArgKind,
    ExprKind, FloatCaseBranches, FloatExprKind, FloatFunctionExprKind, FunctionExprKind,
    FunctionFunctionExprKind, IntCaseBranches, IntExprKind, IntFunctionExprKind, ListExprKind,
    ListFunctionExprKind, NilExprKind, NilFunctionExprKind, StringCaseBranches, StringExprKind,
    StringFunctionExprKind, TupleExprKind, TupleFunctionExprKind,
};
pub use expression::{
    BoolExpr, BoolFunctionExpr, CallArg, Expr, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
};
pub(crate) use frame::FrameLayout;
pub(crate) use function::{
    BoolFunctionReturn, BoolReturn, FloatFunctionReturn, FloatReturn, FunctionFunctionReturn,
    IntFunctionReturn, IntReturn, ListFunctionReturn, ListReturn, NilFunctionReturn, NilReturn,
    ParamLocal, ReturnBody, ReturnBodyKind, ReturnExprKind, RuntimeFunction, StringFunctionReturn,
    StringReturn, TupleFunctionReturn, TupleReturn,
};
pub use function::{FunctionPlan, Param, ParamBinding, ReturnExpr};
pub use id::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolLocalId,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionFunctionId, FunctionFunctionLocalId, FunctionId, IntFunctionFunctionId,
    IntFunctionId, IntFunctionLocalId, IntLocalId, ListFunctionFunctionId, ListFunctionId,
    ListFunctionLocalId, ListLocalId, LocalId, NilFunctionFunctionId, NilFunctionId,
    NilFunctionLocalId, NilLocalId, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleLocalId,
};
pub(crate) use id::{FunctionFunctionId, FunctionReturnFamily, RuntimeFunctionId};
pub use step::Step;
pub(crate) use step::StepKind;
pub(crate) use value::{
    BoolFunctionValue, CaptureValue, CaptureValueKind, FloatFunctionValue, FunctionFunctionValue,
    FunctionValueKind, IntFunctionValue, ListFunctionValue, NilFunctionValue, StringFunctionValue,
    TupleFunctionValue,
};
pub use value::{FunctionType, FunctionValue, ListValue, Value, ValueType};

pub struct ExecutionPlan {
    module: EcoString,
    main: FunctionPlan,
    functions: Vec<FunctionPlan>,
    anonymous_functions: Vec<FunctionPlan>,
    runtime: RuntimePlan,
}

impl ExecutionPlan {
    #[cfg(test)]
    pub(crate) fn new(module: EcoString, main: FunctionPlan, functions: Vec<FunctionPlan>) -> Self {
        Self::new_with_anonymous(module, main, functions, Vec::new())
    }

    pub(crate) fn new_with_anonymous(
        module: EcoString,
        main: FunctionPlan,
        functions: Vec<FunctionPlan>,
        anonymous_functions: Vec<FunctionPlan>,
    ) -> Self {
        let runtime = RuntimePlan::new(&main, &functions, &anonymous_functions);

        Self {
            module,
            main,
            functions,
            anonymous_functions,
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

    #[cfg(test)]
    pub(crate) fn anonymous_functions(&self) -> &[FunctionPlan] {
        &self.anonymous_functions
    }

    pub(crate) fn main_runtime(&self) -> RuntimeFunctionId {
        self.runtime.main()
    }

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &RuntimeFunction<IntReturn> {
        self.runtime.int_function(id)
    }

    pub(crate) fn float_function(&self, id: FloatFunctionId) -> &RuntimeFunction<FloatReturn> {
        self.runtime.float_function(id)
    }

    pub(crate) fn string_function(&self, id: StringFunctionId) -> &RuntimeFunction<StringReturn> {
        self.runtime.string_function(id)
    }

    pub(crate) fn bool_function(&self, id: BoolFunctionId) -> &RuntimeFunction<BoolReturn> {
        self.runtime.bool_function(id)
    }

    pub(crate) fn nil_function(&self, id: NilFunctionId) -> &RuntimeFunction<NilReturn> {
        self.runtime.nil_function(id)
    }

    pub(crate) fn tuple_function(&self, id: TupleFunctionId) -> &RuntimeFunction<TupleReturn> {
        self.runtime.tuple_function(id)
    }

    pub(crate) fn list_function(&self, id: ListFunctionId) -> &RuntimeFunction<ListReturn> {
        self.runtime.list_function(id)
    }

    pub(crate) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &RuntimeFunction<IntFunctionReturn> {
        self.runtime.int_function_function(id)
    }

    pub(crate) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &RuntimeFunction<FloatFunctionReturn> {
        self.runtime.float_function_function(id)
    }

    pub(crate) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &RuntimeFunction<StringFunctionReturn> {
        self.runtime.string_function_function(id)
    }

    pub(crate) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &RuntimeFunction<BoolFunctionReturn> {
        self.runtime.bool_function_function(id)
    }

    pub(crate) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &RuntimeFunction<NilFunctionReturn> {
        self.runtime.nil_function_function(id)
    }

    pub(crate) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &RuntimeFunction<TupleFunctionReturn> {
        self.runtime.tuple_function_function(id)
    }

    pub(crate) fn list_function_function(
        &self,
        id: ListFunctionFunctionId,
    ) -> &RuntimeFunction<ListFunctionReturn> {
        self.runtime.list_function_function(id)
    }

    pub(crate) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &RuntimeFunction<FunctionFunctionReturn> {
        self.runtime.function_function_function(id)
    }
}

impl fmt::Debug for ExecutionPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExecutionPlan")
            .field("module", &self.module)
            .field("main", &self.main)
            .field("functions", &self.functions)
            .field("anonymous_functions", &self.anonymous_functions)
            .finish()
    }
}

impl PartialEq for ExecutionPlan {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
            && self.main == other.main
            && self.functions == other.functions
            && self.anonymous_functions == other.anonymous_functions
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionPlan, FunctionId, FunctionPlan, IntExpr, IntFunctionId, ReturnBody, ReturnExpr,
        RuntimeFunctionId,
    };
    use num_bigint::BigInt;

    #[test]
    fn execution_plan_accessors() {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1))),
        );
        let helper = FunctionPlan::new(
            FunctionId::new(1),
            "helper".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(1), IntExpr::value(BigInt::from(2))),
        );
        let anonymous = FunctionPlan::new(
            FunctionId::new(2),
            "<anonymous:0>".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(2), IntExpr::value(BigInt::from(3))),
        );
        let plan =
            ExecutionPlan::new_with_anonymous("main".into(), main, vec![helper], vec![anonymous]);

        assert_eq!(plan.module(), "main");
        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
        assert_eq!(plan.anonymous_functions().len(), 1);
        assert_eq!(plan.anonymous_functions()[0].name(), "<anonymous:0>");
    }

    #[test]
    fn execution_plan_runtime_table_uses_return_expr_ids() {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(1), IntExpr::value(BigInt::from(11))),
        );
        let helper = FunctionPlan::new(
            FunctionId::new(1),
            "helper".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(10))),
        );
        let plan = ExecutionPlan::new("main".into(), main, vec![helper]);

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(1))
        );
        assert_eq!(
            plan.int_function(IntFunctionId(0)).return_(),
            &ReturnBody::expr(IntExpr::value(BigInt::from(10))),
        );
        assert_eq!(
            plan.int_function(IntFunctionId(1)).return_(),
            &ReturnBody::expr(IntExpr::value(BigInt::from(11))),
        );
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
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1))),
            ),
            Vec::new(),
        );
        let debug = format!("{plan:?}");

        assert!(debug.contains("ExecutionPlan"));
        assert!(debug.contains("module"));
        assert!(debug.contains("main"));
        assert!(debug.contains("functions"));
        assert!(debug.contains("anonymous_functions"));
        assert!(!debug.contains("runtime:"));
        assert!(!debug.contains("RuntimePlan"));
    }
}
