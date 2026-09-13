mod runtime;
mod target;

use super::catalog::{Catalog, CatalogError, Function};
use super::local::{LocalError, Locals};
use super::operand::Operand;
use super::type_::{TypeError, Types};
use crate::plan::execution::graph::{ParamLocal, ParamSlot};
use crate::plan::execution::type_::ValueShapeDescriptor;

pub(super) trait Target {
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError>;
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CallError {
    Catalog(CatalogError),
    Type(TypeError),
    Local(LocalError),
    ArgumentCount { expected: usize, found: usize },
    ArgumentType { index: usize },
    ReturnType,
    TargetType,
    UnboundCaptures { count: usize },
    NotFunction,
}

impl Function<'_> {
    pub(super) fn call<'data>(
        &self,
        args: &[ParamLocal],
        output: &ParamSlot,
        locals: &Locals<'data>,
        types: &Types<'data>,
    ) -> Result<(), CallError> {
        self.arguments(args, locals, types)?;
        types.slot(output).map_err(CallError::Type)?;
        if !types.flow(self.return_, output.shape) {
            return Err(CallError::ReturnType);
        }
        Ok(())
    }

    pub(super) fn arguments<'data>(
        &self,
        args: &[ParamLocal],
        locals: &Locals<'data>,
        types: &Types<'data>,
    ) -> Result<(), CallError> {
        if !self.captures.is_empty() {
            return Err(CallError::UnboundCaptures {
                count: self.captures.len(),
            });
        }
        if args.len() != self.parameters.len() {
            return Err(CallError::ArgumentCount {
                expected: self.parameters.len(),
                found: args.len(),
            });
        }
        for (index, ((arg, expected), parameter)) in args
            .iter()
            .zip(self.parameter_shapes)
            .zip(self.parameters)
            .enumerate()
        {
            let arg = arg.read(locals).map_err(CallError::Local)?;
            if !super::local::Address::of(&arg.local)
                .same_family(super::local::Address::of(parameter))
                || !locals.value(arg).flows_to(*expected, types)
            {
                return Err(CallError::ArgumentType { index });
            }
        }
        Ok(())
    }
}

pub(super) fn indirect<'data>(
    function: &dyn Operand,
    args: &[ParamLocal],
    output: &ParamSlot,
    locals: &Locals<'data>,
    types: &Types<'data>,
) -> Result<(), CallError> {
    let return_ = indirect_arguments(function, args, locals, types)?;
    types.slot(output).map_err(CallError::Type)?;
    if !types.flow(return_, output.shape) {
        return Err(CallError::ReturnType);
    }
    Ok(())
}

pub(super) fn indirect_arguments<'data>(
    function: &dyn Operand,
    args: &[ParamLocal],
    locals: &Locals<'data>,
    types: &Types<'data>,
) -> Result<crate::plan::execution::type_::ValueShapeId, CallError> {
    let function = function.read(locals).map_err(CallError::Local)?;
    let ValueShapeDescriptor::Function { arguments, return_ } = function.descriptor else {
        return Err(CallError::NotFunction);
    };
    if args.len() != arguments.len() {
        return Err(CallError::ArgumentCount {
            expected: arguments.len(),
            found: args.len(),
        });
    }
    for (index, (arg, expected)) in args.iter().zip(arguments.iter()).enumerate() {
        let arg = arg.read(locals).map_err(CallError::Local)?;
        if !locals.value(arg).flows_to(*expected, types) {
            return Err(CallError::ArgumentType { index });
        }
    }
    Ok(*return_)
}

#[cfg(test)]
mod tests {
    use super::{CallError, Catalog, CatalogError, Locals, Target, Types, indirect};
    use super::{LocalError, TypeError};
    use crate::plan::execution::function::{FunctionTableFamily, IntFunctionId};
    use crate::plan::execution::graph::{
        BoolLocalId, IntInstruction, IntLocalId, ParamLocal, ParamSlot, ProfiledInstructionKind,
    };
    use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId};

    #[test]
    fn validates_real_direct_and_indirect_calls_before_defining_the_result() {
        let source = r#"
fn double(x: Int) { x * 2 }
fn invoke(action: fn(Int) -> Int) { action(20) }
pub fn main() { echo True double(21) + invoke(fn(x) { x + 2 }) }
"#;
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
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
        let mut direct = 0;
        let mut indirect_count = 0;
        let boolean = ParamSlot {
            local: ParamLocal::Bool(BoolLocalId(0)),
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Bool)
                    .unwrap(),
            ),
        };
        for function in plan.program.functions.value_returns.int_functions.iter() {
            for block in function.body().block_graph().blocks() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    match instruction.kind() {
                        ProfiledInstructionKind::Int(IntInstruction::Call {
                            function,
                            args,
                            ..
                        }) => {
                            let target = function.resolve(&catalog, &types).unwrap();
                            target
                                .call(args, instruction.output(), &locals, &types)
                                .unwrap();
                            assert_eq!(
                                target.arguments(&[], &locals, &types),
                                Err(CallError::ArgumentCount {
                                    expected: args.len(),
                                    found: 0
                                })
                            );
                            assert_eq!(
                                target.call(args, &boolean, &locals, &types),
                                Err(CallError::ReturnType)
                            );
                            let invalid = ParamSlot {
                                shape: ValueShapeId(999),
                                ..instruction.output().clone()
                            };
                            assert_eq!(
                                target.call(args, &invalid, &locals, &types),
                                Err(CallError::Type(TypeError::MissingShape { index: 999 })),
                            );
                            assert_eq!(
                                target.call(
                                    &[ParamLocal::Int(IntLocalId(999))],
                                    instruction.output(),
                                    &locals,
                                    &types
                                ),
                                Err(CallError::Local(LocalError::Missing(
                                    IntLocalId(999).into()
                                ))),
                            );
                            direct += 1;
                        }
                        ProfiledInstructionKind::Int(IntInstruction::FunctionCall {
                            function,
                            args,
                            ..
                        }) => {
                            indirect(function, args, instruction.output(), &locals, &types)
                                .unwrap();
                            assert_eq!(
                                indirect(function, &[], instruction.output(), &locals, &types),
                                Err(CallError::ArgumentCount {
                                    expected: args.len(),
                                    found: 0
                                })
                            );
                            assert_eq!(
                                indirect(function, args, &boolean, &locals, &types),
                                Err(CallError::ReturnType)
                            );
                            let invalid = ParamSlot {
                                shape: ValueShapeId(999),
                                ..instruction.output().clone()
                            };
                            assert_eq!(
                                indirect(function, args, &invalid, &locals, &types),
                                Err(CallError::Type(TypeError::MissingShape { index: 999 })),
                            );
                            assert_eq!(
                                indirect(
                                    function,
                                    &[ParamLocal::Int(IntLocalId(999))],
                                    instruction.output(),
                                    &locals,
                                    &types
                                ),
                                Err(CallError::Local(LocalError::Missing(
                                    IntLocalId(999).into()
                                ))),
                            );
                            assert_eq!(
                                indirect(&args[0], args, instruction.output(), &locals, &types),
                                Err(CallError::NotFunction),
                            );
                            indirect_count += 1;
                        }
                        _ => {}
                    }
                    locals.define(instruction.output(), &types).unwrap();
                }
            }
        }
        assert!(direct > 0);
        assert!(indirect_count > 0);
        assert_eq!(
            IntFunctionId(usize::MAX).resolve(&catalog, &types).err(),
            Some(CallError::Catalog(CatalogError::MissingFunction {
                family: FunctionTableFamily::Int,
                index: usize::MAX
            }))
        );
    }

    #[test]
    fn rejects_unbound_captures_without_invoking_the_target() {
        let source = "fn make(x) { fn(y) { x + y } } pub fn main() { make(2)(4) }";
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
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
        let mut captured = 0;
        for index in 0..plan.program.functions.value_returns.int_functions.len() {
            let function = catalog.function(FunctionTableFamily::Int, index).unwrap();
            if !function.captures.is_empty() {
                assert_eq!(
                    function.arguments(&[], &Locals::default(), &types),
                    Err(CallError::UnboundCaptures {
                        count: function.captures.len()
                    })
                );
                captured += 1;
            }
        }
        assert!(captured > 0);
    }

    #[test]
    fn rejects_target_return_metadata_that_disagrees_with_its_typed_table() {
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { #(42, \"text\") }",
        )
        .unwrap();
        let mut plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = std::sync::Arc::get_mut(&mut plan.program.common).unwrap();
        let mut contracts = common.function_parameters.functions.to_vec();
        contracts[0].return_ = ValueShapeId(0);
        std::sync::Arc::get_mut(&mut common.function_parameters)
            .unwrap()
            .functions = contracts.into();
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        assert_eq!(
            crate::plan::execution::function::TupleFunctionId(0)
                .resolve(&catalog, &types)
                .err(),
            Some(CallError::TargetType)
        );
    }
}
