use super::super::type_::{TypeError, Types};
use crate::plan::execution::function::function_return::body::{
    ProfiledCustomFunctionFunctionBody, ProfiledExternalFunctionFunctionBody,
    ProfiledFunctionFunctionFunctionBody, TypedFunctionBody,
};
use crate::plan::execution::function::value_return::body::{
    ProfiledCustomFunctionBody, ProfiledExternalFunctionBody,
};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionBodyOwner, ProfiledFunctionBody,
};
use crate::plan::execution::graph::{
    CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId, CustomFunctionLocal,
    CustomFunctionLocalId, ExternalFunctionLocal, ExternalFunctionLocalId, ExternalLocal,
    ExternalLocalId, FunctionFunctionLocal, ParamLocal, ParamSlot,
};
use crate::plan::execution::type_::{
    CustomValueShape, FunctionShape, ValueShapeDescriptor, ValueShapeId,
};

pub(in crate::plan::execution::prepared::admission) trait Contract:
    FunctionBodyOwner
{
    fn check_contract(&self, return_: ValueShapeId, types: &Types<'_>)
    -> Result<(), ContractError>;
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) enum ContractError {
    Type(TypeError),
    ReturnShape,
    BodyShape,
}

impl<Return, TailCall, Graph: ExecutionGraphProfile> Contract
    for ProfiledFunctionBody<Return, TailCall, Graph>
{
    fn check_contract(&self, _: ValueShapeId, _: &Types<'_>) -> Result<(), ContractError> {
        Ok(())
    }
}

impl<Body: Contract> Contract for TypedFunctionBody<Body> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        function_shape(&self._shape, return_, types)?;
        self.body.check_contract(return_, types)
    }
}

impl<Graph: ExecutionGraphProfile> Contract for ProfiledCustomFunctionBody<Graph> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        let ValueShapeDescriptor::Custom(id) = types.shape(return_).map_err(ContractError::Type)?
        else {
            return Err(ContractError::ReturnShape);
        };
        let return_shape = CustomValueShape {
            type_id: types.shapes.custom_shapes[id.0].type_id,
            shape_id: *id,
        };
        if !types
            .can_flow_custom(return_shape, self._signature_shape)
            .map_err(ContractError::Type)?
        {
            return Err(ContractError::ReturnShape);
        }
        if !types
            .can_flow_custom(self._body_shape, return_shape)
            .map_err(ContractError::Type)?
        {
            return Err(ContractError::BodyShape);
        }
        Ok(())
    }
}

impl<Graph: ExecutionGraphProfile> Contract for ProfiledExternalFunctionBody<Graph> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        for type_id in [self._signature_type, self._body_type] {
            let local = ParamLocal::External(ExternalLocal {
                id: ExternalLocalId(0),
                type_id,
            });
            types
                .slot(&ParamSlot {
                    local,
                    shape: return_,
                })
                .map_err(ContractError::Type)?;
        }
        Ok(())
    }
}

impl<Graph: ExecutionGraphProfile> Contract for ProfiledCustomFunctionFunctionBody<Graph> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        function_shape(&self._shape, return_, types)?;
        let local = ParamLocal::CustomFunction(CustomFunctionLocal {
            id: CustomFunctionLocalId(0),
            type_: self._type.clone(),
        });
        types
            .slot(&ParamSlot {
                local,
                shape: self._shape.shape_id,
            })
            .map_err(ContractError::Type)?;
        Ok(())
    }
}

impl<Graph: ExecutionGraphProfile> Contract for ProfiledExternalFunctionFunctionBody<Graph> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        function_shape(&self._shape, return_, types)?;
        let local = ParamLocal::ExternalFunction(ExternalFunctionLocal {
            id: ExternalFunctionLocalId(0),
            type_: self._type.clone(),
        });
        types
            .slot(&ParamSlot {
                local,
                shape: self._shape.shape_id,
            })
            .map_err(ContractError::Type)?;
        Ok(())
    }
}

impl<Graph: ExecutionGraphProfile> Contract for ProfiledFunctionFunctionFunctionBody<Graph> {
    fn check_contract(
        &self,
        return_: ValueShapeId,
        types: &Types<'_>,
    ) -> Result<(), ContractError> {
        function_shape(&self._shape, return_, types)?;
        let local =
            ParamLocal::FunctionFunction(FunctionFunctionLocal::Core(CoreFunctionFunctionLocal {
                id: CoreFunctionFunctionLocalId(0),
                type_: self._type.clone(),
            }));
        types
            .slot(&ParamSlot {
                local,
                shape: self._shape.shape_id,
            })
            .map_err(ContractError::Type)?;
        Ok(())
    }
}

fn function_shape(
    shape: &FunctionShape,
    return_: ValueShapeId,
    types: &Types<'_>,
) -> Result<(), ContractError> {
    types.function_shape(shape).map_err(ContractError::Type)?;
    if !types
        .can_flow(shape.shape_id, return_)
        .map_err(ContractError::Type)?
    {
        return Err(ContractError::ReturnShape);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Contract, ContractError, FunctionShape, Types, function_shape};
    use crate::plan::execution::function::FunctionTableFamily;
    use crate::plan::execution::prepared::admission::catalog::Catalog;
    use crate::plan::execution::type_::ValueType;

    #[test]
    fn constructor_refined_callables_can_flow_to_the_declared_return_but_not_the_reverse() {
        let source = r#"
pub type Box(a) { Box(a) }
pub fn main() { echo 42 fn() { Box(42) } }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
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
        let function = catalog
            .function(FunctionTableFamily::CustomFunction, 0)
            .unwrap();
        let body = plan
            .program
            .functions
            .function_returns
            .custom_function_functions[0]
            .body();
        assert_eq!(
            types.equivalent(body._shape.shape_id, function.return_),
            Ok(false)
        );
        assert_eq!(
            types.can_flow(body._shape.shape_id, function.return_),
            Ok(true)
        );
        assert_eq!(body.check_contract(function.return_, &types), Ok(()));
        let advertised = FunctionShape {
            type_: body._shape.type_.clone(),
            shape_id: function.return_,
        };
        assert_eq!(
            function.return_type,
            &ValueType::Function(advertised.type_.clone())
        );
        assert_eq!(
            function_shape(&advertised, body._shape.shape_id, &types),
            Err(ContractError::ReturnShape)
        );
    }

    #[test]
    fn custom_bodies_keep_signature_body_and_catalog_shapes_consistent() {
        use crate::plan::execution::prepared::admission::{tests::owned_mut, type_::TypeError};
        use crate::plan::execution::type_::{
            CustomValueShape, CustomValueShapeId, ValueShapeDescriptor, ValueShapeId,
        };

        let source = r#"
pub type Choice { First(Int) Second }
pub type Other { Other }
pub fn main() { let _ = #(Second, Other) First(42) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let mut plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let return_ = common.function_parameters.functions
            [common.function_parameters.families[FunctionTableFamily::Custom as usize].start]
            .return_;
        let int = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|value| matches!(value, ValueShapeDescriptor::Int))
                .unwrap(),
        );
        let (other_index, other) = common
            .value_shapes
            .custom_shapes
            .iter()
            .enumerate()
            .find(|(_, value)| {
                common.custom_types.types[value.type_id.index()]
                    .type_
                    .name
                    .as_str()
                    == "Other"
            })
            .unwrap();
        let other = CustomValueShape {
            type_id: other.type_id,
            shape_id: CustomValueShapeId(other_index),
        };
        let body = &mut owned_mut(
            &mut owned_mut(&mut plan.program.functions)
                .value_returns
                .custom_functions,
        )[0]
        .body;
        assert_eq!(body.check_contract(return_, &types), Ok(()));
        assert_eq!(
            body.check_contract(ValueShapeId(999), &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        assert_eq!(
            body.check_contract(int, &types),
            Err(ContractError::ReturnShape)
        );
        let signature = body._signature_shape;
        let actual = body._body_shape;
        body._signature_shape = other;
        assert_eq!(
            body.check_contract(return_, &types),
            Err(ContractError::ReturnShape)
        );
        body._signature_shape = CustomValueShape {
            shape_id: CustomValueShapeId(999),
            ..signature
        };
        assert_eq!(
            body.check_contract(return_, &types),
            Err(ContractError::Type(TypeError::MissingCustomShape {
                index: 999
            }))
        );
        body._signature_shape = signature;
        body._body_shape = other;
        assert_eq!(
            body.check_contract(return_, &types),
            Err(ContractError::BodyShape)
        );
        body._body_shape = CustomValueShape {
            shape_id: CustomValueShapeId(999),
            ..actual
        };
        assert_eq!(
            body.check_contract(return_, &types),
            Err(ContractError::Type(TypeError::MissingCustomShape {
                index: 999
            }))
        );
        body._body_shape = actual;
        assert_eq!(body.check_contract(return_, &types), Ok(()));
    }

    #[test]
    fn callable_body_metadata_is_checked_before_the_enclosed_graph() {
        use crate::plan::execution::prepared::admission::{tests::owned_mut, type_::TypeError};
        use crate::plan::execution::type_::{FunctionType, ValueShapeId};

        let source = r#"
pub type Box(a) { Box(a) }
fn integer() { fn() { 42 } }
fn custom() { fn() { Box(42) } }
fn nested() { fn() { fn() { 42 } } }
pub fn main() { #(integer(), custom(), nested()) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let mut plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let tables = owned_mut(&mut plan.program.functions);
        let int = &mut owned_mut(&mut tables.function_returns.int_function_functions)[0].body;
        let original = int._shape.clone();
        assert_eq!(int.check_contract(original.shape_id, &types), Ok(()));
        int._shape.shape_id = ValueShapeId(999);
        assert_eq!(
            int.check_contract(original.shape_id, &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        int._shape = original;
        assert_eq!(
            int.check_contract(ValueShapeId(999), &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );

        let custom = &mut owned_mut(&mut tables.function_returns.custom_function_functions)[0].body;
        let original = custom._shape.clone();
        assert_eq!(custom.check_contract(original.shape_id, &types), Ok(()));
        custom._shape.shape_id = ValueShapeId(999);
        assert_eq!(
            custom.check_contract(original.shape_id, &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        custom._shape = original;
        custom._type.type_ = FunctionType::new(Vec::new(), ValueType::Bool);
        assert_eq!(
            custom.check_contract(custom._shape.shape_id, &types),
            Err(ContractError::Type(TypeError::LocalTypeMismatch))
        );

        let nested =
            &mut owned_mut(&mut tables.function_returns.function_function_functions)[0].body;
        let original = nested._shape.clone();
        assert_eq!(nested.check_contract(original.shape_id, &types), Ok(()));
        nested._shape.shape_id = ValueShapeId(999);
        assert_eq!(
            nested.check_contract(original.shape_id, &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        nested._shape = original;
        nested._type.type_ = FunctionType::new(Vec::new(), ValueType::Bool);
        assert_eq!(
            nested.check_contract(nested._shape.shape_id, &types),
            Err(ContractError::Type(TypeError::LocalTypeMismatch))
        );
    }

    #[test]
    fn external_body_contracts_validate_nominal_storage_and_callable_shape() {
        use crate::plan::execution::function::ValueFunctionEntry;
        use crate::plan::execution::prepared::admission::{
            tests::{lowered_native, owned_mut},
            type_::TypeError,
        };
        use crate::plan::execution::type_::{ExternalTypeId, FunctionType, ValueShapeId};
        let source = r#"
pub type Key
@external(erlang, "native", "key") fn key() -> Key
fn value() { key() }
pub fn main() { value }
"#;
        let (mut program, _, _) = lowered_native(source);
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let tables = owned_mut(&mut program.functions);
        let bodies = owned_mut(&mut tables.value_returns.external_functions)
            .iter_mut()
            .filter(|entry| matches!(entry, ValueFunctionEntry::Graph(_)))
            .collect::<Vec<_>>();
        assert_eq!(bodies.len(), 1);
        let body = graph_body(bodies.into_iter().next().unwrap());
        let return_ = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| {
                    shape
                        == &crate::plan::execution::type_::ValueShapeDescriptor::External(
                            ExternalTypeId(0),
                        )
                })
                .unwrap(),
        );
        assert_eq!(body.check_contract(return_, &types), Ok(()));
        body._body_type = ExternalTypeId(999);
        assert_eq!(
            body.check_contract(return_, &types),
            Err(ContractError::Type(TypeError::MissingExternal {
                index: 999
            }))
        );
        body._body_type = ExternalTypeId(0);
        let bodies = owned_mut(&mut tables.function_returns.external_function_functions);
        assert_eq!(bodies.len(), 1);
        let body = graph_body(&mut bodies[0]);
        let shape = body._shape.clone();
        assert_eq!(body.check_contract(shape.shape_id, &types), Ok(()));
        body._shape.shape_id = ValueShapeId(999);
        assert_eq!(
            body.check_contract(shape.shape_id, &types),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        body._shape = shape;
        body._type.type_ = FunctionType::new(Vec::new(), ValueType::Bool);
        assert_eq!(
            body.check_contract(body._shape.shape_id, &types),
            Err(ContractError::Type(TypeError::LocalTypeMismatch))
        );
    }

    fn graph_body<Body, Host>(
        entry: &mut crate::plan::execution::function::ValueFunctionEntry<Body, Host>,
    ) -> &mut Body {
        match entry {
            crate::plan::execution::function::ValueFunctionEntry::Graph(function) => {
                &mut crate::plan::execution::prepared::admission::tests::owned_mut(function).body
            }
            crate::plan::execution::function::ValueFunctionEntry::Host(_) => {
                panic!("expected a graph body in this fixture")
            }
        }
    }

    #[test]
    #[should_panic(expected = "expected a graph body in this fixture")]
    fn graph_fixture_guard_rejects_a_registered_native_target() {
        use crate::plan::execution::function::ValueFunctionEntry;
        use crate::plan::execution::prepared::admission::tests::{lowered_native, owned_mut};
        let (mut program, _, _) = lowered_native(
            "pub type Key\n@external(erlang, \"native\", \"key\") fn key() -> Key\npub fn main() { key() }",
        );
        let tables = owned_mut(&mut program.functions);
        let (mut graphs, mut hosts): (Vec<_>, Vec<_>) =
            owned_mut(&mut tables.value_returns.external_functions)
                .iter_mut()
                .partition(|entry| matches!(entry, ValueFunctionEntry::Graph(_)));
        assert_eq!((graphs.len(), hosts.len()), (1, 1));
        graph_body(graphs.pop().unwrap());
        graph_body(hosts.pop().unwrap());
    }
}
