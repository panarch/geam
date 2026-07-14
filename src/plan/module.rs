mod expression;
mod frame;
mod function;
mod id;
mod pattern;
mod reference;
mod step;

use crate::plan::SourceContext;
use ecow::EcoString;

pub use expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CallArg, Expr, FloatExpr,
    FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr,
    ListFunctionExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr,
    TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
pub(crate) use expression::{
    BitArrayExprKind, BitArrayFunctionExprKind, BitArraySegment, BoolCaseBranches, BoolExprKind,
    BoolFunctionExprKind, CallArgKind, CaptureArg, CaptureArgKind, Endianness, ExprKind,
    FloatBitSize, FloatCaseBranches, FloatExprKind, FloatFunctionExprKind, FunctionExprKind,
    FunctionFunctionExprKind, IntCaseBranches, IntExprKind, IntFunctionExprKind, ListElements,
    ListFunctionExprKind, NilExprKind, NilFunctionExprKind, PanicExpr, PanicExprKind,
    StringCaseBranches, StringEncoding, StringExprKind, StringFunctionExprKind, TupleExprKind,
    TupleFunctionExprKind, UtfCodepointExprKind, UtfCodepointFunctionExprKind,
};
pub(crate) use expression::{
    BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
    FloatListExpr, FloatListItem, FunctionListExpr, FunctionListItem, IntListExpr, IntListItem,
    ListCaseBranches, ListExpr, ListItem, ListListExpr, ListListItem, ListLocalExpr,
    ListSpreadElements, NilListExpr, NilListItem, StringListExpr, StringListItem, TupleListExpr,
    TupleListItem, TypedListExpr, TypedListExprKind, TypedListReturnKind, UtfCodepointListExpr,
    UtfCodepointListItem,
};
pub(crate) use frame::FrameLayout;
#[cfg(test)]
pub(crate) use function::ListReturn;
pub(crate) use function::{
    BitArrayFunctionReturn, BitArrayListReturn, BitArrayReturn, BoolFunctionReturn, BoolListReturn,
    BoolReturn, FloatFunctionReturn, FloatListReturn, FloatReturn, FunctionExecutionParts,
    FunctionFunctionReturn, FunctionListReturn, IntFunctionReturn, IntListReturn, IntReturn,
    ListFunctionReturn, ListListReturn, NilFunctionReturn, NilListReturn, NilReturn, ParamLocal,
    ReturnBody, ReturnBodyKind, ReturnExprKind, StringFunctionReturn, StringListReturn,
    StringReturn, TupleFunctionReturn, TupleListReturn, TupleReturn, UtfCodepointFunctionReturn,
    UtfCodepointListReturn, UtfCodepointReturn,
};
pub use function::{FunctionPlan, Param, ParamBinding, ReturnExpr};
pub use id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId,
    BitArrayListFunctionFunctionId, BitArrayListFunctionId, BitArrayListFunctionLocalId,
    BitArrayListLocalId, BitArrayLocalId, BoolFunctionFunctionId, BoolFunctionId,
    BoolFunctionLocalId, BoolListFunctionFunctionId, BoolListFunctionId, BoolListFunctionLocalId,
    BoolListLocalId, BoolLocalId, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId,
    FloatListFunctionFunctionId, FloatListFunctionId, FloatListFunctionLocalId, FloatListLocalId,
    FloatLocalId, FunctionFunctionFunctionId, FunctionFunctionLocalId, FunctionId,
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
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointFunctionLocalId, UtfCodepointListFunctionFunctionId,
    UtfCodepointListFunctionId, UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
pub(crate) use id::{FunctionFunctionId, RuntimeFunctionId};
pub(crate) use pattern::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, PatternBinding,
    Signedness,
};
pub(crate) use reference::{
    BitArrayFunctionReference, BoolFunctionReference, FloatFunctionReference,
    FunctionFunctionReference, FunctionReference, IntFunctionReference, ListFunctionReference,
    NilFunctionReference, StringFunctionReference, TupleFunctionReference, TypedFunctionReference,
    UtfCodepointFunctionReference,
};
pub use step::Step;
pub(crate) use step::{AssertBinding, AssertPattern, ListAssertPattern, ListAssertTail, StepKind};

#[derive(Debug, PartialEq)]
pub struct ModulePlan {
    module: EcoString,
    source_context: Option<SourceContext>,
    main: FunctionPlan,
    functions: Vec<FunctionPlan>,
    anonymous_functions: Vec<FunctionPlan>,
}

pub(crate) struct ModulePlanParts {
    pub(crate) module: EcoString,
    pub(crate) source_context: Option<SourceContext>,
    pub(crate) main: FunctionPlan,
    pub(crate) functions: Vec<FunctionPlan>,
    pub(crate) anonymous_functions: Vec<FunctionPlan>,
}

impl ModulePlan {
    pub(crate) fn new(module: EcoString, main: FunctionPlan, functions: Vec<FunctionPlan>) -> Self {
        Self {
            module,
            source_context: None,
            main,
            functions,
            anonymous_functions: Vec::new(),
        }
    }

    pub(crate) fn with_anonymous_functions(
        mut self,
        anonymous_functions: Vec<FunctionPlan>,
    ) -> Self {
        self.anonymous_functions = anonymous_functions;
        self
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

    pub(crate) fn into_parts(self) -> ModulePlanParts {
        ModulePlanParts {
            module: self.module,
            source_context: self.source_context,
            main: self.main,
            functions: self.functions,
            anonymous_functions: self.anonymous_functions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModulePlan;
    use crate::plan::{
        FunctionId, FunctionPlan, IntExpr, IntFunctionId, ReturnExpr, SourceContext,
    };
    use num_bigint::BigInt;

    #[test]
    fn module_plan_accessors() {
        let main = function(0, "main", 1);
        let helper = function(1, "helper", 2);
        let anonymous = function(2, "<anonymous:0>", 3);
        let plan = ModulePlan::new("main".into(), main, vec![helper])
            .with_anonymous_functions(vec![anonymous]);

        assert_eq!(plan.module(), "main");
        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
        assert_eq!(plan.anonymous_functions().len(), 1);
        assert_eq!(plan.anonymous_functions()[0].name(), "<anonymous:0>");
        assert_eq!(plan.source_context(), None);
    }

    #[test]
    fn module_plan_debug_surface_contains_only_canonical_plan() {
        let plan = ModulePlan::new("main".into(), function(0, "main", 1), Vec::new())
            .with_source_context(SourceContext::new("main.gleam", "pub fn main() { panic }"));
        let debug = format!("{plan:?}");

        assert_eq!(
            debug,
            format!(
                "ModulePlan {{ module: {:?}, source_context: {:?}, main: {:?}, functions: {:?}, anonymous_functions: {:?} }}",
                plan.module,
                plan.source_context,
                plan.main,
                plan.functions,
                plan.anonymous_functions,
            ),
        );
    }

    #[test]
    fn module_plan_equality_includes_source_context() {
        let new_plan = || ModulePlan::new("main".into(), function(0, "main", 1), Vec::new());

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
        assert_ne!(
            ModulePlan::new("left".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("right".into(), function(0, "main", 1), Vec::new()),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("main".into(), function(0, "main", 2), Vec::new()),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new(
                "main".into(),
                function(0, "main", 1),
                vec![function(1, "helper", 2)],
            ),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new())
                .with_anonymous_functions(vec![function(1, "<anonymous:0>", 2)]),
        );
    }

    fn function(id: usize, name: &str, value: i64) -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(id),
            name.into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(id), IntExpr::value(BigInt::from(value))),
        )
    }
}
