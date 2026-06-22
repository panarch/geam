use crate::plan::{ExecutionPlan, FunctionId};
use crate::planner::dsl::function::FunctionDsl;
use ecow::EcoString;

pub(crate) fn module(
    name: impl Into<EcoString>,
    main: FunctionDsl,
    functions: impl IntoIterator<Item = FunctionDsl>,
) -> ExecutionPlan {
    ExecutionPlan::new(
        name.into(),
        main.build(FunctionId::new(0)),
        functions
            .into_iter()
            .enumerate()
            .map(|(index, function)| function.build(FunctionId::new(index + 1)))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::module;
    use crate::planner::dsl::expression::{int, nil};
    use crate::planner::dsl::function::function;

    #[test]
    fn module_dsl() {
        let plan = module(
            "main",
            function("main", int(1)),
            [function("helper", nil())],
        );

        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
    }
}
