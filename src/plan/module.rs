use super::{FunctionPlan, SourceContext};
use ecow::EcoString;

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
