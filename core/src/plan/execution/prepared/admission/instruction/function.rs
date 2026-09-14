mod capture;

use super::{InstructionError, Instructions};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionReturnFamily, GenericCallableId,
};
use crate::plan::execution::graph::{
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget, ParamLocal,
    ParamSlot,
};
use crate::plan::execution::prepared::admission::{
    call::Target, catalog::Function, local::Locals, type_::Types,
};
use crate::plan::execution::type_::{
    CustomConstructorId, CustomConstructorRefinement, FunctionType, ValueShapeDescriptor,
    ValueShapeId, ValueType,
};

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn function(
        &self,
        instruction: &FunctionInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        header(self.types, &instruction.type_, instruction.family, output)?;
        match &instruction.kind {
            FunctionInstructionKind::Reference(target) => {
                self.function_value(target, &[], instruction.family, output, locals)
            }
            FunctionInstructionKind::Closure { target, captures } => {
                self.function_value(target, captures, instruction.family, output, locals)
            }
            FunctionInstructionKind::Constructor(id) => self.constructor_function(*id, output),
            FunctionInstructionKind::Constant(id) => self.constant(*id, output),
            FunctionInstructionKind::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            FunctionInstructionKind::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            FunctionInstructionKind::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            FunctionInstructionKind::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            FunctionInstructionKind::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
        }
    }

    fn function_shape(
        &self,
        id: ValueShapeId,
    ) -> Result<(&'data [ValueShapeId], ValueShapeId), InstructionError> {
        let ValueShapeDescriptor::Function { arguments, return_ } =
            self.types.shape(id).map_err(InstructionError::Type)?
        else {
            return Err(InstructionError::OutputType);
        };
        Ok((arguments, *return_))
    }

    fn function_value(
        &self,
        target: &FunctionTarget,
        captures: &[FunctionCapture],
        family: FunctionReturnFamily,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let (actual, function) = match target {
            FunctionTarget::Generic(id) => {
                if family != FunctionReturnFamily::Generic {
                    return Err(InstructionError::FunctionFamily);
                }
                match id {
                    GenericCallableId::Function {
                        // Only symbolic slots retain this identity; there is no
                        // executable body to resolve.
                        template: _,
                        substitution,
                    } => {
                        for shape in substitution.iter() {
                            self.types.shape(*shape).map_err(InstructionError::Type)?;
                        }
                    }
                    GenericCallableId::Constructor(id) => {
                        self.types
                            .constructor(*id)
                            .map_err(InstructionError::Type)?;
                    }
                }
                for (index, capture) in captures.iter().enumerate() {
                    capture::admit(self.types, capture, None, index, locals)?;
                }
                return Ok(());
            }
            FunctionTarget::Never(id) => (
                FunctionReturnFamily::Never,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Int(id) => (
                FunctionReturnFamily::Int,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Float(id) => (
                FunctionReturnFamily::Float,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::String(id) => (
                FunctionReturnFamily::String,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::BitArray(id) => (
                FunctionReturnFamily::BitArray,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::UtfCodepoint(id) => (
                FunctionReturnFamily::UtfCodepoint,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Custom(id) => (
                FunctionReturnFamily::Custom,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Bool(id) => (
                FunctionReturnFamily::Bool,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Nil(id) => (
                FunctionReturnFamily::Nil,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Tuple(id) => (
                FunctionReturnFamily::Tuple,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::List(id) => (
                FunctionReturnFamily::List,
                id.resolve(self.catalog, self.types),
            ),
            FunctionTarget::Function(id) => (
                FunctionReturnFamily::Function,
                id.resolve(self.catalog, self.types),
            ),
        };
        if actual != family {
            return Err(InstructionError::FunctionFamily);
        }
        self.bound_function(
            function.map_err(InstructionError::Call)?,
            captures,
            output,
            locals,
        )
    }

    pub(super) fn bound_function(
        &self,
        function: Function<'data>,
        captures: &[FunctionCapture],
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let (arguments, return_) = self.function_shape(output.shape)?;
        if arguments.len() != function.parameter_shapes.len() {
            return Err(InstructionError::Arity {
                expected: function.parameter_shapes.len(),
                found: arguments.len(),
            });
        }
        for (argument, target) in arguments.iter().zip(function.parameter_shapes) {
            self.flow(*argument, *target)?;
        }
        self.flow(function.return_, return_)?;
        if captures.len() != function.captures.len() {
            return Err(InstructionError::Arity {
                expected: function.captures.len(),
                found: captures.len(),
            });
        }
        for (index, (capture, expected)) in captures.iter().zip(function.captures).enumerate() {
            capture::admit(self.types, capture, Some(expected), index, locals)?;
        }
        Ok(())
    }

    fn constructor_function(
        &self,
        id: CustomConstructorId,
        output: &ParamSlot,
    ) -> Result<(), InstructionError> {
        let (arguments, return_) = self.function_shape(output.shape)?;
        let ValueShapeDescriptor::Custom(shape) = &self.types.shapes.shapes[return_.index()] else {
            return Err(InstructionError::OutputType);
        };
        let shape = &self.types.shapes.custom_shapes[shape.0];
        if shape.type_id != id.type_id
            || matches!(shape.constructor, CustomConstructorRefinement::Exact(index) if index != id.index)
        {
            return Err(InstructionError::OutputType);
        }
        let constructor = self
            .types
            .constructor_descriptor(id)
            .map_err(InstructionError::Type)?;
        if arguments.len() != constructor.fields.len() {
            return Err(InstructionError::Arity {
                expected: constructor.fields.len(),
                found: arguments.len(),
            });
        }
        for (argument, field) in arguments.iter().zip(constructor.fields.iter()) {
            if self.types.shapes.shape_types[argument.index()] != field.type_ {
                return Err(InstructionError::OperandType);
            }
            if !self
                .types
                .can_enter_field(*argument, &field.refinement, &shape.arguments)
            {
                return Err(InstructionError::Flow);
            }
        }
        Ok(())
    }
}

pub(super) fn header(
    types: &Types<'_>,
    expected: &FunctionType,
    family: FunctionReturnFamily,
    output: &ParamSlot,
) -> Result<(), InstructionError> {
    let admitted = types.slot(output).map_err(InstructionError::Type)?;
    let actual = match &output.local {
        ParamLocal::GenericFunction(_) => FunctionReturnFamily::Generic,
        ParamLocal::NeverFunction(_) => FunctionReturnFamily::Never,
        ParamLocal::IntFunction { .. } => FunctionReturnFamily::Int,
        ParamLocal::FloatFunction { .. } => FunctionReturnFamily::Float,
        ParamLocal::StringFunction { .. } => FunctionReturnFamily::String,
        ParamLocal::BitArrayFunction { .. } => FunctionReturnFamily::BitArray,
        ParamLocal::UtfCodepointFunction { .. } => FunctionReturnFamily::UtfCodepoint,
        ParamLocal::CustomFunction(_) => FunctionReturnFamily::Custom,
        ParamLocal::ExternalFunction(_) => FunctionReturnFamily::External,
        ParamLocal::BoolFunction { .. } => FunctionReturnFamily::Bool,
        ParamLocal::NilFunction { .. } => FunctionReturnFamily::Nil,
        ParamLocal::TupleFunction { .. } => FunctionReturnFamily::Tuple,
        ParamLocal::ListFunction(_) => FunctionReturnFamily::List,
        ParamLocal::FunctionFunction(_) => FunctionReturnFamily::Function,
        _ => return Err(InstructionError::OutputType),
    };
    if !matches!(admitted.type_, ValueType::Function(actual) if actual == expected) {
        return Err(InstructionError::OutputType);
    }
    if actual != family {
        return Err(InstructionError::FunctionFamily);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        FunctionInstructionKind, FunctionReturnFamily, InstructionError, Instructions, Locals,
        header,
    };
    use crate::plan::execution::graph::{ProfiledBlockGraph, ProfiledInstructionKind};
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog, source::Sources, type_::Types,
    };
    use std::convert::Infallible;

    #[test]
    fn function_values_preserve_every_origin_including_symbolic_captures() {
        use crate::plan::execution::function::{
            FunctionTableFamily, GenericCallableId, IntFunctionId,
        };
        use crate::plan::execution::graph::{FunctionTarget, IntLocalId};
        use crate::plan::execution::prepared::admission::{
            call::CallError, catalog::CatalogError, local::LocalError, type_::TypeError,
        };
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId, ValueShapeId};

        let source = r#"
pub type Box(a) { Box(value: a) }
fn pass(value) { value }
fn increment(value: Int) { value + 1 }
fn point() { let assert <<value:utf8_codepoint>> = <<65>> value }
const saved = increment
fn create() { increment }
pub fn main() {
  let prefix = "captured"
  let symbolic = fn(value) { #(value, prefix) }
  let constructor: fn(Int) -> Box(Int) = Box
  let tuple = pass(#(increment))
  let boxed = pass(Box(increment))
  let values = pass([increment])
  let factory = pass(create)
  let first = case values { [first, ..] -> first [] -> increment }
  echo #(saved, create(), factory(), tuple.0, boxed.value, first,
    symbolic, constructor, Box, fn(value: Float) { value +. 1.0 }, point)
  Nil
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let mut origins = [false; 9];
        for function in plan.program.functions.value_returns.nil_functions.iter() {
            for block in function.body().block_graph().blocks() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::Function(value) = &instruction.kind {
                        assert_eq!(
                            context.function(value, &instruction.output, &locals),
                            Ok(())
                        );
                        origins[match &value.kind {
                            FunctionInstructionKind::Reference(_) => 0,
                            FunctionInstructionKind::Closure { .. } => 1,
                            FunctionInstructionKind::Constructor(_) => 2,
                            FunctionInstructionKind::Constant(_) => 3,
                            FunctionInstructionKind::Call { .. } => 4,
                            FunctionInstructionKind::FunctionCall { .. } => 5,
                            FunctionInstructionKind::TupleIndex { .. } => 6,
                            FunctionInstructionKind::CustomField { .. } => 7,
                            FunctionInstructionKind::ListIndex { .. } => 8,
                        }] = true;
                        if let FunctionInstructionKind::Reference(target)
                        | FunctionInstructionKind::Closure { target, .. } = &value.kind
                        {
                            assert_eq!(
                                context.function_value(
                                    target,
                                    &[],
                                    FunctionReturnFamily::Bool,
                                    &instruction.output,
                                    &locals,
                                ),
                                Err(InstructionError::FunctionFamily),
                            );
                        }
                    }
                    locals.define(&instruction.output, &types).unwrap();
                }
            }
        }
        assert_eq!(origins, [true; 9]);
        let output = &plan.program.functions.value_returns.nil_functions[0]
            .body()
            .block_graph()
            .instructions[0]
            .output;
        let locals = Locals::default();
        assert_eq!(
            context.function_value(
                &FunctionTarget::Int(IntFunctionId(999)),
                &[],
                FunctionReturnFamily::Int,
                output,
                &locals,
            ),
            Err(InstructionError::Call(CallError::Catalog(
                CatalogError::MissingFunction {
                    family: FunctionTableFamily::Int,
                    index: 999,
                }
            ))),
        );
        let symbolic = FunctionTarget::Generic(GenericCallableId::Function {
            template: 0,
            substitution: vec![ValueShapeId(999)].into(),
        });
        assert_eq!(
            context.function_value(
                &symbolic,
                &[],
                FunctionReturnFamily::Generic,
                output,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 999
            })),
        );
        let constructor =
            FunctionTarget::Generic(GenericCallableId::Constructor(CustomConstructorId {
                type_id: CustomTypeId(999),
                index: 0,
            }));
        assert_eq!(
            context.function_value(
                &constructor,
                &[],
                FunctionReturnFamily::Generic,
                output,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingCustom {
                index: 999
            })),
        );
        let symbolic = FunctionTarget::Generic(GenericCallableId::Function {
            template: 0,
            substitution: vec![output.shape].into(),
        });
        assert_eq!(
            context.function_value(
                &symbolic,
                &[crate::plan::execution::graph::FunctionCapture::Int {
                    target: IntLocalId(0),
                    source: IntLocalId(999),
                }],
                FunctionReturnFamily::Generic,
                output,
                &locals,
            ),
            Err(InstructionError::Local(LocalError::Missing(
                IntLocalId(999).into()
            ))),
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Nil
        );
    }

    #[test]
    fn bound_functions_validate_output_variance_arity_and_capture_sources() {
        use super::{FunctionCapture, FunctionTarget, ParamSlot};
        use crate::plan::execution::graph::IntLocalId;
        use crate::plan::execution::prepared::admission::{
            call::Target, local::LocalError, type_::TypeError,
        };
        use crate::plan::execution::type_::{ValueShapeId, ValueType};

        let source = r#"
fn increment(value: Int) { value + 1 }
fn offset() { 20 }
pub fn main() {
  let offset = case offset() { 20 -> 20 _ -> 0 }
  echo #(fn(_value: Int) { True }, fn() { 42 }, increment, fn(_value: Bool) { 42 },
    fn(value: Int) { value + offset })
  Nil
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let graph = plan.program.functions.value_returns.nil_functions[0]
            .body()
            .block_graph();
        let references = graph
            .instructions
            .iter()
            .filter_map(|instruction| match &instruction.kind {
                ProfiledInstructionKind::Function(value) => match value.kind {
                    FunctionInstructionKind::Reference(FunctionTarget::Int(target)) => {
                        Some((target, &instruction.output))
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>();
        let (target, ordinary_output) = references[0];
        let mut checked = [false; 4];
        let mut captured = 0;
        for block in graph.blocks() {
            let mut locals = Locals::default();
            for slot in block.params() {
                locals.define(slot, &types).unwrap();
            }
            for instruction in block.instructions() {
                if let ProfiledInstructionKind::Function(value) = &instruction.kind {
                    let (index, expected) =
                        match (value.type_.argument_types(), value.type_.return_()) {
                            ([], _) => (
                                0,
                                Err(InstructionError::Arity {
                                    expected: 1,
                                    found: 0,
                                }),
                            ),
                            ([ValueType::Bool], _) => (1, Err(InstructionError::Flow)),
                            (_, ValueType::Bool) => (2, Err(InstructionError::Flow)),
                            _ => (3, Ok(())),
                        };
                    assert_eq!(
                        context.bound_function(
                            target.resolve(&catalog, &types).unwrap(),
                            &[],
                            &instruction.output,
                            &locals
                        ),
                        expected
                    );
                    checked[index] = true;
                    if let FunctionInstructionKind::Closure {
                        target: FunctionTarget::Int(closure),
                        captures,
                    } = &value.kind
                        && !captures.is_empty()
                    {
                        assert_eq!(captures.len(), 1);
                        captured += 1;
                        assert_eq!(
                            context.bound_function(
                                closure.resolve(&catalog, &types).unwrap(),
                                &[FunctionCapture::Int {
                                    target: IntLocalId(1),
                                    source: IntLocalId(999)
                                }],
                                &instruction.output,
                                &locals
                            ),
                            Err(InstructionError::Local(LocalError::Missing(
                                IntLocalId(999).into()
                            )))
                        );
                    }
                }
                locals.define(&instruction.output, &types).unwrap();
            }
        }
        assert_eq!(checked, [true; 4]);
        assert_eq!(captured, 1);
        let scalar = common
            .value_shapes
            .shape_types
            .iter()
            .position(|type_| type_ == &ValueType::Int)
            .unwrap();
        for (shape, expected) in [
            (
                ValueShapeId(999),
                InstructionError::Type(TypeError::MissingShape { index: 999 }),
            ),
            (ValueShapeId(scalar), InstructionError::OutputType),
        ] {
            let output = ParamSlot {
                local: ordinary_output.local.clone(),
                shape,
            };
            assert_eq!(
                context.bound_function(
                    target.resolve(&catalog, &types).unwrap(),
                    &[],
                    &output,
                    &Locals::default()
                ),
                Err(expected)
            );
        }
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Nil
        );
    }

    #[test]
    fn function_headers_and_constructor_links_reject_different_valid_signatures() {
        use super::{
            CustomConstructorId, FunctionType, ParamLocal, ParamSlot, ValueShapeId, ValueType,
        };
        use crate::plan::execution::graph::IntLocalId;
        use crate::plan::execution::type_::CustomTypeId;
        let source = r#"
pub type Box(a) { Box(a) }
pub type Other { Other(Int) }
pub fn main() {
  echo #(42, fn(x: Int) { x }, fn() { Box(42) }, fn(_x: String) { Box(42) },
    fn(x: Int) { Box(x) }, fn(x: Int) { Other(x) })
  Nil
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let custom = CustomTypeId(
            common
                .custom_types
                .types
                .iter()
                .position(|value| value.type_.name.as_str() == "Box")
                .unwrap(),
        );
        let constructor = CustomConstructorId {
            type_id: custom,
            index: 0,
        };
        let scalar = ParamSlot::new(
            ParamLocal::Int(IntLocalId(0)),
            ValueShapeId(
                types
                    .shape_types()
                    .iter()
                    .position(|value| value == &ValueType::Int)
                    .unwrap(),
            ),
        );
        let mut seen = [false; 5];
        for instruction in plan.program.functions.value_returns.nil_functions[0]
            .body()
            .block_graph()
            .blocks()
            .flat_map(|block| block.instructions())
        {
            if let ProfiledInstructionKind::Function(value) = &instruction.kind {
                let output = &instruction.output;
                assert_eq!(
                    header(context.types, &value.type_, value.family, output),
                    Ok(())
                );
                assert_eq!(
                    header(context.types, &value.type_, value.family, &scalar),
                    Err(InstructionError::OutputType)
                );
                assert_eq!(
                    header(
                        context.types,
                        &FunctionType::new(vec![], ValueType::Bool),
                        value.family,
                        output
                    ),
                    Err(InstructionError::OutputType)
                );
                let wrong = ParamSlot {
                    local: output.local.clone(),
                    shape: ValueShapeId(999),
                };
                assert_eq!(
                    context.constructor_function(constructor, &wrong),
                    Err(InstructionError::Type(
                        super::super::super::type_::TypeError::MissingShape { index: 999 }
                    ))
                );
                assert_eq!(
                    header(context.types, &value.type_, value.family, &wrong),
                    Err(InstructionError::Type(
                        super::super::super::type_::TypeError::MissingShape { index: 999 }
                    ))
                );
                let (index, expected) = match value.type_.return_() {
                    ValueType::Int => (0, Err(InstructionError::OutputType)),
                    ValueType::Custom(id) if *id != custom => {
                        (1, Err(InstructionError::OutputType))
                    }
                    _ if value.type_.argument_types().is_empty() => (
                        2,
                        Err(InstructionError::Arity {
                            expected: 1,
                            found: 0,
                        }),
                    ),
                    _ if value.type_.argument_types() == [ValueType::String] => {
                        (3, Err(InstructionError::OperandType))
                    }
                    _ => (4, Ok(())),
                };
                assert_eq!(context.constructor_function(constructor, output), expected);
                if index == 4 {
                    assert_eq!(
                        context.constructor_function(
                            CustomConstructorId {
                                index: 1,
                                ..constructor
                            },
                            output
                        ),
                        Err(InstructionError::OutputType)
                    );
                }
                seen[index] = true;
            }
        }
        assert_eq!(seen, [true; 5]);
    }

    #[test]
    fn constructor_functions_require_materialized_fields_and_generic_refinements() {
        use super::{
            CustomConstructorId, CustomConstructorRefinement, FunctionType, ParamLocal, ParamSlot,
            ValueShapeDescriptor, ValueShapeId, ValueType,
        };
        use crate::plan::execution::graph::{CustomFunctionLocal, CustomFunctionLocalId};
        use crate::plan::execution::type_::{
            CustomFunctionType, CustomValueShape, CustomValueShapeId, ValueShapeTable,
        };

        let source = r#"
pub type Choice { First Second }
pub type Box(a) { Box(a) Missing(a) }
pub fn main() { #(Box(First), fn(x: Choice) { Box(x) }, Second) }
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
        let outputs = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .block_graph()
            .instructions
            .iter()
            .filter_map(|instruction| match &instruction.output.local {
                ParamLocal::Custom(local) => Some((&instruction.output, local)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let (boxed, boxed_local) = outputs
            .iter()
            .find(|(_, local)| {
                common.custom_types.types[local.shape.type_id.index()]
                    .type_
                    .name
                    .as_str()
                    == "Box"
            })
            .unwrap();
        let descriptor = &common.value_shapes.custom_shapes[boxed_local.shape.shape_id.0];
        let argument_type = types.shape_type(descriptor.arguments[0]).unwrap();
        let argument = ValueShapeId(
            types
                .shape_types()
                .iter()
                .enumerate()
                .find(|(index, type_)| {
                    *type_ == argument_type && types.is_nominal_shape(ValueShapeId(*index))
                })
                .unwrap()
                .0,
        );
        let function_type = FunctionType::new(
            vec![argument_type.clone()],
            ValueType::Custom(descriptor.type_id),
        );

        for (constructor, expected) in [
            (0, InstructionError::Flow),
            (
                1,
                InstructionError::Type(super::super::super::type_::TypeError::MissingConstructor {
                    type_index: descriptor.type_id.index(),
                    index: 1,
                }),
            ),
        ] {
            let mut custom_shapes = common.value_shapes.custom_shapes.to_vec();
            let shape_id = CustomValueShapeId(custom_shapes.len());
            custom_shapes.push(crate::plan::execution::type_::CustomValueShapeDescriptor {
                constructor: CustomConstructorRefinement::Exact(constructor),
                ..descriptor.clone()
            });
            let mut shapes = common.value_shapes.shapes.to_vec();
            let return_ = ValueShapeId(shapes.len());
            shapes.push(ValueShapeDescriptor::Custom(shape_id));
            let shape = ValueShapeId(shapes.len());
            shapes.push(ValueShapeDescriptor::Function {
                arguments: vec![argument].into(),
                return_,
            });
            let mut shape_types = common.value_shapes.shape_types.to_vec();
            shape_types.extend([
                types.shape_type(boxed.shape).unwrap().clone(),
                ValueType::Function(function_type.clone()),
            ]);
            let raw = ValueShapeTable {
                shapes: shapes.into(),
                shape_types: shape_types.into(),
                custom_shapes: custom_shapes.into(),
            };
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &raw,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            let output = ParamSlot {
                shape,
                local: ParamLocal::CustomFunction(CustomFunctionLocal {
                    id: CustomFunctionLocalId(0),
                    type_: CustomFunctionType {
                        type_: function_type.clone(),
                        arguments: vec![argument].into(),
                        return_: CustomValueShape {
                            type_id: descriptor.type_id,
                            shape_id,
                        },
                    },
                }),
            };
            types.slot(&output).unwrap();
            assert_eq!(
                context.constructor_function(
                    CustomConstructorId {
                        type_id: descriptor.type_id,
                        index: constructor
                    },
                    &output
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn validates_function_values_and_capture_layouts_in_real_graphs() {
        let source = r#"
pub type Box(a) { Box(value: a) }
fn make(offset) { fn(value) { offset + value } }
fn increment(value) { value + 1 }
pub fn main() {
  let constructor = Box
  let closure = make(20)
  let reference = increment
  #(constructor(True), closure(22), fn(x) { x }, reference(41))
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let mut counts = [0; 3];
        for function in plan.program.functions.value_returns.int_functions.iter() {
            check_body(&context, function.body().block_graph(), &mut counts);
        }
        for function in plan.program.functions.value_returns.tuple_functions.iter() {
            check_body(&context, function.body().block_graph(), &mut counts);
        }
        for function in plan
            .program
            .functions
            .function_returns
            .int_function_functions
            .iter()
        {
            check_body(
                &context,
                function.body().function_body().block_graph(),
                &mut counts,
            );
        }
        assert_eq!(counts, [1, 1, 1]);
    }

    fn check_body<'data>(
        context: &Instructions<'_, 'data, Infallible>,
        body: &'data ProfiledBlockGraph<Infallible>,
        counts: &mut [usize; 3],
    ) {
        for block in body.blocks() {
            let mut locals = Locals::default();
            for slot in block.params() {
                locals.define(slot, context.types).unwrap();
            }
            for instruction in block.instructions() {
                assert_eq!(context.check(instruction, &locals), Ok(()));
                if let ProfiledInstructionKind::Function(value) = instruction.kind() {
                    let mut invalid = value.clone();
                    assert_ne!(value.family, FunctionReturnFamily::Bool);
                    invalid.family = FunctionReturnFamily::Bool;
                    assert_eq!(
                        context.function(&invalid, instruction.output(), &locals),
                        Err(InstructionError::FunctionFamily)
                    );
                    match &value.kind {
                        FunctionInstructionKind::Reference(_) => counts[0] += 1,
                        FunctionInstructionKind::Closure { target, captures }
                            if !captures.is_empty() =>
                        {
                            counts[1] += 1;
                            invalid.family = value.family;
                            invalid.kind = FunctionInstructionKind::Reference(target.clone());
                            assert_eq!(
                                context.function(&invalid, instruction.output(), &locals),
                                Err(InstructionError::Arity {
                                    expected: captures.len(),
                                    found: 0
                                })
                            );
                        }
                        FunctionInstructionKind::Constructor(_) => counts[2] += 1,
                        _ => {}
                    }
                }
                locals.define(instruction.output(), context.types).unwrap();
            }
        }
    }
}
