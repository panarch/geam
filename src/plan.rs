mod expression;
mod frame;
mod function;
mod id;
mod runtime;
mod source;
mod step;
mod value;

use self::runtime::RuntimePlan;
use ecow::EcoString;
use std::fmt;

pub(crate) use expression::{
    BoolCaseBranches, BoolExprKind, BoolFunctionExprKind, CallArgKind, CaptureArg, CaptureArgKind,
    ExprKind, FloatCaseBranches, FloatExprKind, FloatFunctionExprKind, FunctionExprKind,
    FunctionFunctionExprKind, IntCaseBranches, IntExprKind, IntFunctionExprKind, ListElements,
    ListFunctionExprKind, NilExprKind, NilFunctionExprKind, PanicExpr, PanicExprKind,
    StringCaseBranches, StringExprKind, StringFunctionExprKind, TupleExprKind,
    TupleFunctionExprKind,
};
pub use expression::{
    BoolExpr, BoolFunctionExpr, CallArg, Expr, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListFunctionExpr, NilExpr, NilFunctionExpr,
    StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
};
pub(crate) use expression::{
    BoolListCaseBranches, BoolListExpr, BoolListItem, FloatListExpr, FloatListItem,
    FunctionListExpr, FunctionListItem, IntListExpr, IntListItem, ListCaseBranches, ListExpr,
    ListItem, ListListExpr, ListListItem, ListLocalExpr, ListSpreadElements, NilListExpr,
    NilListItem, StringListExpr, StringListItem, TupleListExpr, TupleListItem, TypedListExpr,
    TypedListExprKind, TypedListReturnKind,
};
pub(crate) use frame::FrameLayout;
pub(crate) use function::{
    BoolFunctionReturn, BoolListReturn, BoolReturn, FloatFunctionReturn, FloatListReturn,
    FloatReturn, FunctionFunctionReturn, FunctionListReturn, IntFunctionReturn, IntListReturn,
    IntReturn, ListFunctionReturn, ListListReturn, ListReturn, NilFunctionReturn, NilListReturn,
    NilReturn, ParamLocal, ReturnBody, ReturnBodyKind, ReturnExprKind, RuntimeFunction,
    StringFunctionReturn, StringListReturn, StringReturn, TupleFunctionReturn, TupleListReturn,
    TupleReturn,
};
pub use function::{FunctionPlan, Param, ParamBinding, ReturnExpr};
pub use id::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolListFunctionFunctionId,
    BoolListFunctionId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatListFunctionFunctionId,
    FloatListFunctionId, FloatListFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionFunctionFunctionId, FunctionFunctionLocalId, FunctionId,
    FunctionListFunctionFunctionId, FunctionListFunctionId, FunctionListFunctionLocalId,
    FunctionListLocalId, FunctionReturnFamily, IntFunctionFunctionId, IntFunctionId,
    IntFunctionLocalId, IntListFunctionFunctionId, IntListFunctionId, IntListFunctionLocalId,
    IntListLocalId, IntLocalId, ListFunctionFunctionId, ListFunctionId, ListFunctionLocal,
    ListListFunctionFunctionId, ListListFunctionId, ListListFunctionLocalId, ListListLocalId,
    ListLocal, LocalId, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    NilListFunctionFunctionId, NilListFunctionId, NilListFunctionLocalId, NilListLocalId,
    NilLocalId, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
    StringListFunctionFunctionId, StringListFunctionId, StringListFunctionLocalId,
    StringListLocalId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleListFunctionFunctionId, TupleListFunctionId,
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId,
};
pub(crate) use id::{FunctionFunctionId, RuntimeFunctionId};
pub use source::{PanicSite, SourceContext, SourceSpan};
pub use step::Step;
pub(crate) use step::{AssertBinding, AssertPattern, ListAssertPattern, ListAssertTail, StepKind};
pub(crate) use value::{
    BoolFunctionValue, CaptureValue, CaptureValueKind, FloatFunctionValue, FunctionFunctionValue,
    FunctionValueKind, IntFunctionValue, ListFunctionValue, ListLocalValue, NilFunctionValue,
    StringFunctionValue, TupleFunctionValue,
};
pub use value::{
    FunctionType, FunctionValue, ListValue, ListValueItemTypeMismatch, Value, ValueType,
};

pub struct ExecutionPlan {
    module: EcoString,
    source_context: Option<SourceContext>,
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
            source_context: None,
            main,
            functions,
            anonymous_functions,
            runtime,
        }
    }

    pub(crate) fn with_source_context(mut self, source_context: SourceContext) -> Self {
        self.source_context = Some(source_context);
        self
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.source_context.as_ref()
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

    pub(crate) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &RuntimeFunction<IntListReturn> {
        self.runtime.int_list_function(id)
    }

    pub(crate) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &RuntimeFunction<StringListReturn> {
        self.runtime.string_list_function(id)
    }

    pub(crate) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &RuntimeFunction<FloatListReturn> {
        self.runtime.float_list_function(id)
    }

    pub(crate) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &RuntimeFunction<BoolListReturn> {
        self.runtime.bool_list_function(id)
    }

    pub(crate) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &RuntimeFunction<NilListReturn> {
        self.runtime.nil_list_function(id)
    }

    pub(crate) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &RuntimeFunction<TupleListReturn> {
        self.runtime.tuple_list_function(id)
    }

    pub(crate) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &RuntimeFunction<ListListReturn> {
        self.runtime.list_list_function(id)
    }

    pub(crate) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &RuntimeFunction<FunctionListReturn> {
        self.runtime.function_list_function(id)
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
        id: &ListFunctionFunctionId,
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
            .field("source_context", &self.source_context)
            .field("main", &self.main)
            .field("functions", &self.functions)
            .field("anonymous_functions", &self.anonymous_functions)
            .finish()
    }
}

impl PartialEq for ExecutionPlan {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
            && self.source_context == other.source_context
            && self.main == other.main
            && self.functions == other.functions
            && self.anonymous_functions == other.anonymous_functions
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionPlan, FunctionId, FunctionPlan, IntExpr, IntFunctionId, ReturnBody, ReturnExpr,
        RuntimeFunctionId, SourceContext,
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
        )
        .with_source_context(SourceContext::new("main.gleam", "pub fn main() { panic }"));
        let debug = format!("{plan:?}");

        assert!(debug.contains("ExecutionPlan"));
        assert!(debug.contains("module"));
        assert!(debug.contains("source_context"));
        assert!(debug.contains("main.gleam"));
        assert!(debug.contains("main"));
        assert!(debug.contains("functions"));
        assert!(debug.contains("anonymous_functions"));
        assert!(!debug.contains("runtime:"));
        assert!(!debug.contains("RuntimePlan"));
    }

    #[test]
    fn execution_plan_equality_includes_source_context() {
        let new_plan = || {
            ExecutionPlan::new(
                "main".into(),
                FunctionPlan::new(
                    FunctionId::new(0),
                    "main".into(),
                    Vec::new(),
                    Vec::new(),
                    ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1))),
                ),
                Vec::new(),
            )
        };

        assert_eq!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan(),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan()
                .with_source_context(SourceContext::new("other.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 2 }")),
        );
    }
}
