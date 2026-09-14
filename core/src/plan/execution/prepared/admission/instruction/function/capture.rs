use super::super::local_flow;
use super::InstructionError;
use crate::plan::execution::graph::{self, FunctionCapture, ParamLocal, ParamSlot};
use crate::plan::execution::prepared::admission::{
    local::{Address, Locals},
    operand::Operand,
    type_::{Slot, Types},
};

pub(super) fn admit<'data>(
    types: &Types<'data>,
    capture: &FunctionCapture,
    expected: Option<&ParamSlot>,
    index: usize,
    locals: &Locals<'data>,
) -> Result<(), InstructionError> {
    match capture {
        FunctionCapture::Int { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::Float { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::String { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::BitArray { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::UtfCodepoint { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::Custom { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::External { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::Bool { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::Nil { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::Tuple { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ParameterList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ParameterListList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::IntList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::StringList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::BitArrayList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::UtfCodepointList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::CustomList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ExternalList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::FloatList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::BoolList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::NilList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::TupleList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ListList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::FunctionList { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::IntFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::FloatFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::StringFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::BitArrayFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::UtfCodepointFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::GenericFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::NeverFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::CustomFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ExternalFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::BoolFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::NilFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::TupleFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::ListFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
        FunctionCapture::FunctionFunction { target, source } => {
            capture_values(types, target, source, expected, index, locals)
        }
    }
}

trait Destination {
    fn matches(&self, expected: &ParamLocal) -> bool;
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError>;
}

impl<Id: Copy + Into<Address>> Destination for Id {
    fn matches(&self, expected: &ParamLocal) -> bool {
        (*self).into() == Address::of(expected)
    }
    fn validate(&self, _source: Slot<'_, '_>, _types: &Types<'_>) -> Result<(), InstructionError> {
        Ok(())
    }
}

impl Destination for graph::CustomLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::Custom(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::Custom(*self), source, types)
    }
}

impl Destination for graph::ExternalLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::External(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::External(*self), source, types)
    }
}

impl Destination for graph::GenericFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::GenericFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::GenericFunction(self.clone()), source, types)
    }
}

impl Destination for graph::NeverFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::NeverFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::NeverFunction(self.clone()), source, types)
    }
}

impl Destination for graph::CustomFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::CustomFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::CustomFunction(self.clone()), source, types)
    }
}

impl Destination for graph::ExternalFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::ExternalFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::ExternalFunction(self.clone()), source, types)
    }
}

impl Destination for graph::ListFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::ListFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::ListFunction(self.clone()), source, types)
    }
}

impl Destination for graph::FunctionFunctionLocal {
    fn matches(&self, expected: &ParamLocal) -> bool {
        matches!(expected, ParamLocal::FunctionFunction(local) if local == self)
    }
    fn validate(&self, source: Slot<'_, '_>, types: &Types<'_>) -> Result<(), InstructionError> {
        validate_metadata(&ParamLocal::FunctionFunction(self.clone()), source, types)
    }
}

fn validate_metadata(
    target: &ParamLocal,
    source: Slot<'_, '_>,
    types: &Types<'_>,
) -> Result<(), InstructionError> {
    let type_ = types.local_type(target).map_err(InstructionError::Type)?;
    if source.type_ != &type_ {
        return Err(InstructionError::OperandType);
    }
    Ok(())
}

fn capture_values<'data>(
    types: &Types<'data>,
    target: &dyn Destination,
    source: &dyn Operand,
    expected: Option<&ParamSlot>,
    index: usize,
    locals: &Locals<'data>,
) -> Result<(), InstructionError> {
    let source = source.read(locals).map_err(InstructionError::Local)?;
    target.validate(source, types)?;
    if let Some(expected) = expected {
        if !target.matches(&expected.local) {
            return Err(InstructionError::CaptureTarget { index });
        }
        local_flow(types, source, expected.shape, locals)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{FunctionCapture, InstructionError, Locals, ParamLocal, ParamSlot, admit};
    use crate::plan::execution::graph::{
        BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, FloatLocalId, IntLocalId,
        NilLocalId, StringLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::admission::instruction::Instructions;
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog,
        local::{Address, LocalError},
        source::Sources,
        type_::{TypeError, Types},
    };
    use crate::plan::execution::type_::{
        CustomConstructorRefinement, CustomValueShape, CustomValueShapeId, ValueShapeDescriptor,
        ValueShapeId, ValueType,
    };

    #[test]
    fn closures_capture_compound_and_callable_values_with_exact_destination_contracts() {
        use crate::plan::execution::graph::{
            FunctionInstructionKind, FunctionTarget, ProfiledInstructionKind,
        };
        use crate::plan::execution::prepared::admission::call::Target;

        let source = r#"
pub type Box(a) { Box(a) }
fn capture(value) { echo 0 fn() { echo value 42 } }
fn unchanged(value) { value }
fn stop(_value: Int) -> a { panic }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<65>> value }
pub fn main() {
  let _ = #(capture(42), capture(1.5), capture("text"), capture(True), capture(Nil),
    capture(<<1>>), capture(codepoint()), capture(#(1, True)), capture(Box(7)))
  let _ = #(capture([42]), capture([1.5]), capture(["text"]), capture([True]), capture([Nil]),
    capture([<<1>>]), capture([codepoint()]), capture([#(1, True)]), capture([Box(7)]),
    capture([]), capture([[]]), capture([[42]]), capture([unchanged]))
  let _ = #(capture(fn() { 42 }), capture(fn() { 1.5 }), capture(fn() { "text" }),
    capture(fn() { True }), capture(fn() { Nil }), capture(fn() { <<1>> }),
    capture(fn() { codepoint() }), capture(fn() { #(1, True) }),
    capture(fn() { Box(7) }), capture(unchanged), capture(stop))
  let _ = #(capture(fn() { [42] }), capture(fn() { [1.5] }), capture(fn() { ["text"] }),
    capture(fn() { [True] }), capture(fn() { [Nil] }), capture(fn() { [<<1>>] }),
    capture(fn() { [codepoint()] }), capture(fn() { [#(1, True)] }),
    capture(fn() { [Box(7)] }), capture(fn() { [] }), capture(fn() { [[]] }),
    capture(fn() { [[42]] }), capture(fn() { [unchanged] }))
  let _ = capture(fn() { unchanged })
  42
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let int_shape = ValueShapeId(
            common
                .value_shapes
                .shape_types
                .iter()
                .position(|type_| type_ == &ValueType::Int)
                .unwrap(),
        );
        let nil_shape = ValueShapeId(
            common
                .value_shapes
                .shape_types
                .iter()
                .position(|type_| type_ == &ValueType::Nil)
                .unwrap(),
        );
        let mut checked = 0;
        for function in plan
            .program
            .functions
            .function_returns
            .int_function_functions
            .iter()
        {
            for block in function.body().function_body().block_graph().blocks() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::Function(value) = instruction.kind()
                        && let FunctionInstructionKind::Closure {
                            target: FunctionTarget::Int(target),
                            captures,
                        } = &value.kind
                    {
                        let declaration = target.resolve(&catalog, &types).unwrap();
                        assert_eq!(captures.len(), 1);
                        assert_eq!(declaration.captures.len(), 1);
                        let capture = &captures[0];
                        let expected = &declaration.captures[0];
                        assert_eq!(
                            admit(context.types, capture, Some(expected), 0, &locals),
                            Ok(())
                        );
                        assert_eq!(admit(context.types, capture, None, 0, &locals), Ok(()));
                        let other = if matches!(expected.local, ParamLocal::Nil(_)) {
                            ParamSlot::new(ParamLocal::Int(IntLocalId(0)), int_shape)
                        } else {
                            ParamSlot::new(ParamLocal::Nil(NilLocalId(0)), nil_shape)
                        };
                        assert_eq!(
                            admit(context.types, capture, Some(&other), 0, &locals),
                            Err(InstructionError::CaptureTarget { index: 0 })
                        );
                        let source = block.params().first().unwrap();
                        assert_eq!(
                            admit(context.types, capture, None, 0, &Locals::default()),
                            Err(InstructionError::Local(LocalError::Missing(Address::of(
                                &source.local
                            ))))
                        );
                        checked += 1;
                    }
                    locals.define(instruction.output(), &types).unwrap();
                }
            }
        }
        assert_eq!(checked, 47);
    }

    #[test]
    fn scalar_captures_preserve_source_family_and_destination_slot() {
        let source = r#"
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<120>>
  #(42, 1.5, "text", <<1>>, point, True, Nil)
}
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let cases = [
            (
                FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(0),
                },
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(1)),
                ValueType::Int,
            ),
            (
                FunctionCapture::Float {
                    target: FloatLocalId(1),
                    source: FloatLocalId(0),
                },
                ParamLocal::Float(FloatLocalId(0)),
                ParamLocal::Float(FloatLocalId(1)),
                ValueType::Float,
            ),
            (
                FunctionCapture::String {
                    target: StringLocalId(1),
                    source: StringLocalId(0),
                },
                ParamLocal::String(StringLocalId(0)),
                ParamLocal::String(StringLocalId(1)),
                ValueType::String,
            ),
            (
                FunctionCapture::BitArray {
                    target: BitArrayLocalId(1),
                    source: BitArrayLocalId(0),
                },
                ParamLocal::BitArray(BitArrayLocalId(0)),
                ParamLocal::BitArray(BitArrayLocalId(1)),
                ValueType::BitArray,
            ),
            (
                FunctionCapture::UtfCodepoint {
                    target: UtfCodepointLocalId(1),
                    source: UtfCodepointLocalId(0),
                },
                ParamLocal::UtfCodepoint(UtfCodepointLocalId(0)),
                ParamLocal::UtfCodepoint(UtfCodepointLocalId(1)),
                ValueType::UtfCodepoint,
            ),
            (
                FunctionCapture::Bool {
                    target: BoolLocalId(1),
                    source: BoolLocalId(0),
                },
                ParamLocal::Bool(BoolLocalId(0)),
                ParamLocal::Bool(BoolLocalId(1)),
                ValueType::Bool,
            ),
            (
                FunctionCapture::Nil {
                    target: NilLocalId(1),
                    source: NilLocalId(0),
                },
                ParamLocal::Nil(NilLocalId(0)),
                ParamLocal::Nil(NilLocalId(1)),
                ValueType::Nil,
            ),
        ];
        let slots = cases
            .iter()
            .map(|(_, local, _, type_)| {
                let shape = common
                    .value_shapes
                    .shape_types
                    .iter()
                    .position(|actual| actual == type_)
                    .unwrap();
                ParamSlot::new(local.clone(), ValueShapeId(shape))
            })
            .collect::<Vec<_>>();
        let mut locals = Locals::default();
        for slot in &slots {
            locals.define(slot, &types).unwrap();
        }
        for (index, ((capture, _, destination, _), slot)) in cases.iter().zip(&slots).enumerate() {
            let expected = ParamSlot::new(destination.clone(), slot.shape);
            assert_eq!(
                admit(context.types, capture, Some(&expected), index, &locals),
                Ok(())
            );
            assert_eq!(admit(context.types, capture, None, index, &locals), Ok(()));
            assert_eq!(
                admit(context.types, capture, Some(slot), index, &locals),
                Err(InstructionError::CaptureTarget { index })
            );
            assert_eq!(
                admit(
                    context.types,
                    capture,
                    Some(&expected),
                    index,
                    &Locals::default()
                ),
                Err(InstructionError::Local(LocalError::Missing(Address::of(
                    &slot.local
                ))))
            );
        }
    }

    #[test]
    fn custom_captures_validate_nominal_identity_and_constructor_flow_separately() {
        let source = "pub type Choice { First Second } pub type Other { Other } pub fn main() { #(First, Second, Other) }";
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let local_shapes = [("Choice", 0), ("Choice", 1), ("Other", 0)]
            .into_iter()
            .enumerate()
            .map(|(index, (name, constructor))| {
                let shape_index = common
                    .value_shapes
                    .custom_shapes
                    .iter()
                    .position(|shape| {
                        common.custom_types.types[shape.type_id.index()]
                            .type_
                            .name
                            .as_ref()
                            == name
                            && shape.constructor == CustomConstructorRefinement::Exact(constructor)
                    })
                    .unwrap();
                let shape = CustomValueShapeId(shape_index);
                let value_shape = common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|actual| actual == &ValueShapeDescriptor::Custom(shape))
                    .unwrap();
                (
                    CustomLocal {
                        id: CustomLocalId(index),
                        shape: CustomValueShape {
                            type_id: common.value_shapes.custom_shapes[shape_index].type_id,
                            shape_id: shape,
                        },
                    },
                    ValueShapeId(value_shape),
                )
            })
            .collect::<Vec<_>>();
        let slots = local_shapes
            .iter()
            .map(|(local, shape)| ParamSlot::new(ParamLocal::Custom(*local), *shape))
            .collect::<Vec<_>>();
        let mut locals = Locals::default();
        for slot in &slots {
            locals.define(slot, &types).unwrap();
        }
        let first = &local_shapes[0].0;
        let second = &local_shapes[1].0;
        let other = &local_shapes[2].0;
        let target = CustomLocal {
            id: CustomLocalId(3),
            shape: first.shape,
        };
        let expected = ParamSlot::new(ParamLocal::Custom(target), slots[0].shape);
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target,
                    source: *first
                },
                Some(&expected),
                0,
                &locals
            ),
            Ok(())
        );
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target,
                    source: *first
                },
                None,
                0,
                &locals
            ),
            Ok(())
        );
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target,
                    source: *second
                },
                Some(&expected),
                0,
                &locals
            ),
            Err(InstructionError::Flow)
        );
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target,
                    source: *other
                },
                Some(&expected),
                0,
                &locals
            ),
            Err(InstructionError::OperandType)
        );
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target,
                    source: *first
                },
                Some(&slots[0]),
                0,
                &locals
            ),
            Err(InstructionError::CaptureTarget { index: 0 })
        );
        let invalid = CustomLocal {
            id: target.id,
            shape: CustomValueShape {
                type_id: target.shape.type_id,
                shape_id: CustomValueShapeId(99),
            },
        };
        assert_eq!(
            admit(
                context.types,
                &FunctionCapture::Custom {
                    target: invalid,
                    source: *first
                },
                None,
                0,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingCustomShape {
                index: 99
            }))
        );
    }
}
