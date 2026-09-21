use super::{CallError, Catalog, Function, Target, Types};
use crate::plan::execution::function::{self, ExecutionGraphProfile};
use crate::plan::execution::graph::{ExternalFunctionCallTarget, ExternalFunctionTarget};
use crate::plan::execution::type_::ValueType;

impl Target for ExternalFunctionTarget {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Value(id) => id.resolve(catalog, types),
            Self::List(id) => id.resolve(catalog, types),
            Self::Function(id) => id.resolve(catalog, types),
            Self::ListFunction { id, type_, list_type } => function::ProfiledListFunctionFunctionId::<function::HostedExecutionGraph>::External { id: *id, type_: type_.clone(), list_type: *list_type }.resolve(catalog, types),
        }
    }
}

impl Target for ExternalFunctionCallTarget {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Function(id) => id.resolve(catalog, types),
            Self::ListFunction { id, type_, list_type } => function::ProfiledListFunctionFunctionId::<function::HostedExecutionGraph>::External { id: *id, type_: type_.clone(), list_type: *list_type }.resolve(catalog, types),
        }
    }
}

impl<Symbolic: Target> Target for function::RuntimeFunctionFunctionTarget<Symbolic> {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Core(id) => id.resolve(catalog, types),
            Self::External(id) => id.resolve(catalog, types),
        }
    }
}

impl<Graph: ExecutionGraphProfile> Target for function::ProfiledRuntimeFunctionId<Graph>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    Graph::RuntimeFunctionFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Core(id) => id.resolve(catalog, types),
            Self::External(id) => id.resolve(catalog, types),
        }
    }
}

impl<Graph: ExecutionGraphProfile> Target for function::ProfiledCoreRuntimeFunctionId<Graph>
where
    Graph::ExternalListFunctionId: Target,
    Graph::RuntimeFunctionFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        match self {
            Self::Never(id) => id.resolve(catalog, types),
            Self::Int(id) => id.resolve(catalog, types),
            Self::Float(id) => id.resolve(catalog, types),
            Self::String(id) => id.resolve(catalog, types),
            Self::BitArray(id) => id.resolve(catalog, types),
            Self::UtfCodepoint(id) => id.resolve(catalog, types),
            Self::Custom(id) => id.resolve(catalog, types),
            Self::Bool(id) => id.resolve(catalog, types),
            Self::Nil(id) => id.resolve(catalog, types),
            Self::List(id) => id.resolve(catalog, types),
            Self::Tuple { id, return_type } => {
                let function = id.resolve(catalog, types)?;
                if !matches!(function.return_type, ValueType::Tuple(actual) if actual == return_type)
                {
                    return Err(CallError::TargetType);
                }
                Ok(function)
            }
            Self::Function { id, return_type } => {
                let function = id.resolve(catalog, types)?;
                if !matches!(function.return_type, ValueType::Function(actual) if actual == return_type)
                {
                    return Err(CallError::TargetType);
                }
                Ok(function)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CallError, Catalog, Target, Types, ValueType};
    use crate::plan::execution::function::{
        FunctionTableFamily, ProfiledCoreRuntimeFunctionId as Core,
        ProfiledRuntimeFunctionId as Runtime,
    };
    use crate::plan::execution::type_::FunctionType;

    #[test]
    fn hosted_runtime_entries_resolve_native_values_and_nested_callables() {
        use crate::plan::execution::function::{
            ProfiledListFunctionId, RuntimeFunctionFunctionTarget,
        };
        use crate::plan::execution::graph::{ExternalFunctionCallTarget, ExternalFunctionTarget};
        use crate::plan::execution::prepared::admission::catalog::CatalogError;
        use crate::plan::execution::prepared::admission::tests::lowered_native;
        for (family, expression) in [
            (FunctionTableFamily::Int, "42"),
            (FunctionTableFamily::External, "key()"),
            (FunctionTableFamily::ExternalList, "[key()]"),
            (FunctionTableFamily::ExternalFunction, "fn() { key() }"),
            (
                FunctionTableFamily::ExternalListFunction,
                "fn() { [key()] }",
            ),
        ] {
            let source = format!(
                "pub type Key \
                @external(erlang, \"native\", \"key\") fn key() -> Key \
                pub fn main() {{ {expression} }}"
            );
            let (program, _, _) = lowered_native(&source);
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let expected = catalog.function(family, 0).unwrap();
            let actual = common.main.resolve(&catalog, &types).unwrap();
            assert_eq!(actual.return_, expected.return_);
            assert!(std::ptr::eq(actual.return_type, expected.return_type));
            assert!(std::ptr::eq(actual.parameters, expected.parameters));
            let external = match &common.main {
                Runtime::External(id) => Some(ExternalFunctionTarget::Value(*id)),
                Runtime::Core(Core::List(ProfiledListFunctionId::External(id))) => {
                    Some(ExternalFunctionTarget::List(*id))
                }
                Runtime::Core(Core::Function {
                    id:
                        RuntimeFunctionFunctionTarget::External(ExternalFunctionCallTarget::Function(
                            id,
                        )),
                    ..
                }) => Some(ExternalFunctionTarget::Function(id.clone())),
                Runtime::Core(Core::Function {
                    id:
                        RuntimeFunctionFunctionTarget::External(
                            ExternalFunctionCallTarget::ListFunction {
                                id,
                                type_,
                                list_type,
                            },
                        ),
                    ..
                }) => Some(ExternalFunctionTarget::ListFunction {
                    id: *id,
                    type_: type_.clone(),
                    list_type: *list_type,
                }),
                _ => None,
            };
            assert_eq!(external.is_some(), family != FunctionTableFamily::Int);
            if let Some(target) = external {
                assert_eq!(
                    target.resolve(&catalog, &types).unwrap().return_,
                    expected.return_
                );
                if let ExternalFunctionTarget::Function(id) = target {
                    assert_eq!(
                        id.with_index(999).resolve(&catalog, &types).err(),
                        Some(CallError::Catalog(CatalogError::MissingFunction {
                            family,
                            index: 999
                        }))
                    );
                }
            }
        }
    }

    #[test]
    fn structured_runtime_entries_reject_missing_inner_targets() {
        use crate::plan::execution::function::{
            IntFunctionFunctionId, ProfiledFunctionFunctionId, TupleFunctionId,
        };
        use crate::plan::execution::prepared::admission::catalog::CatalogError;
        use std::convert::Infallible;

        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 42 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let cases = [
            (
                Core::<Infallible>::Tuple {
                    id: TupleFunctionId(99),
                    return_type: vec![ValueType::Int].into(),
                },
                FunctionTableFamily::Tuple,
            ),
            (
                Core::<Infallible>::Function {
                    id: ProfiledFunctionFunctionId::Int(IntFunctionFunctionId(99)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                FunctionTableFamily::IntFunction,
            ),
        ];
        for (target, family) in cases {
            assert_eq!(
                target.resolve(&catalog, &types).err(),
                Some(CallError::Catalog(CatalogError::MissingFunction {
                    family,
                    index: 99
                }))
            );
        }
    }

    #[test]
    fn runtime_entry_wrappers_resolve_the_same_borrowed_family_contracts() {
        let cases = [
            (FunctionTableFamily::Never, "panic"),
            (FunctionTableFamily::Int, "42"),
            (FunctionTableFamily::Float, "1.5"),
            (FunctionTableFamily::String, "\"text\""),
            (FunctionTableFamily::BitArray, "<<42>>"),
            (
                FunctionTableFamily::UtfCodepoint,
                "{ let assert <<point:utf8_codepoint>> = <<65>> point }",
            ),
            (FunctionTableFamily::Custom, "Box(42)"),
            (FunctionTableFamily::Bool, "True"),
            (FunctionTableFamily::Nil, "Nil"),
            (FunctionTableFamily::Tuple, "#(42, True)"),
            (FunctionTableFamily::IntList, "[42]"),
            (FunctionTableFamily::IntFunction, "fn() { 42 }"),
        ];
        for (family, expression) in cases {
            let source = format!("pub type Box(a) {{ Box(a) }} pub fn main() {{ {expression} }}");
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            let expected = catalog.function(family, 0).unwrap();
            for actual in [
                common.main.resolve(&catalog, &types).unwrap(),
                common.main.runtime_id().resolve(&catalog, &types).unwrap(),
            ] {
                assert_eq!(actual.return_, expected.return_);
                assert!(std::ptr::eq(actual.return_type, expected.return_type));
                assert!(std::ptr::eq(actual.parameters, expected.parameters));
                assert!(std::ptr::eq(actual.captures, expected.captures));
            }
        }
    }

    #[test]
    fn entry_wrappers_reject_different_tuple_fields_and_callable_signatures() {
        for expression in ["#(42, True)", "fn() { 42 }", "42"] {
            let source = format!("pub fn main() {{ {expression} }}");
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            let wrong: Option<Runtime<std::convert::Infallible>> = match common.main.clone() {
                Runtime::Core(Core::Tuple { id, .. }) => Some(Runtime::Core(Core::Tuple {
                    id,
                    return_type: vec![ValueType::Bool].into(),
                })),
                Runtime::Core(Core::Function { id, .. }) => Some(Runtime::Core(Core::Function {
                    id,
                    return_type: FunctionType::new(Vec::new(), ValueType::Bool),
                })),
                _ => None,
            };
            assert_eq!(wrong.is_some(), expression != "42");
            let Some(wrong) = wrong else { continue };
            assert_eq!(
                wrong.resolve(&catalog, &types).err(),
                Some(CallError::TargetType)
            );
        }
    }
}
