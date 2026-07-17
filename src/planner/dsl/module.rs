use crate::plan::{FunctionTemplateId, ModulePlan};
use crate::planner::dsl::function::FunctionDsl;
use ecow::EcoString;

pub(crate) fn module(
    name: impl Into<EcoString>,
    main: FunctionDsl,
    functions: impl IntoIterator<Item = FunctionDsl>,
) -> ModulePlan {
    module_with_anonymous(name, main, functions, [])
}

pub(crate) fn module_with_anonymous(
    name: impl Into<EcoString>,
    main: FunctionDsl,
    functions: impl IntoIterator<Item = FunctionDsl>,
    anonymous_functions: impl IntoIterator<Item = FunctionDsl>,
) -> ModulePlan {
    let main = main.build(FunctionTemplateId::new(0));
    let functions = functions
        .into_iter()
        .enumerate()
        .map(|(index, function)| function.build(FunctionTemplateId::new(index + 1)))
        .collect::<Vec<_>>();
    let next_function_index = functions.len() + 1;
    let anonymous_functions = anonymous_functions
        .into_iter()
        .enumerate()
        .map(|(index, function)| {
            function.build(FunctionTemplateId::new(next_function_index + index))
        })
        .collect();

    ModulePlan::new(name.into(), main, functions).with_anonymous_functions(anonymous_functions)
}

#[cfg(test)]
mod tests {
    use super::module_with_anonymous;
    use crate::planner::dsl::expression::{int, nil};
    use crate::planner::dsl::function::function;

    #[test]
    fn module_dsl() {
        let plan = module_with_anonymous(
            "main",
            function("main", int(1)),
            [function("helper", nil())],
            [function("<anonymous:0>", int(2))],
        );

        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
        assert_eq!(plan.anonymous_functions().len(), 1);
        assert_eq!(plan.anonymous_functions()[0].name(), "<anonymous:0>");
    }
}
