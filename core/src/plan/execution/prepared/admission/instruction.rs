mod bit_array;
mod custom;
mod external;
mod function;
mod list;
mod value;

use super::call::{self, CallError, Target};
use super::catalog::Catalog;
use super::constant::{self, ConstantError};
use super::local::{Address, LocalError, Locals};
use super::operand::Operand;
use super::source::{SourceError, Sources};
use super::type_::{Slot, TypeError, Types};
use crate::plan::HostCallSite;
use crate::plan::execution::constant::{ConstantId, ConstantValue, ProfiledConstantTable};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{CustomLocal, ParamLocal, ParamSlot, TupleLocalId};
use crate::plan::execution::type_::{CustomConstructorRefinement, ValueShapeId, ValueType};

pub(super) struct Instructions<'context, 'data, Graph: ExecutionGraphProfile> {
    pub(super) types: &'context Types<'data>,
    pub(super) catalog: &'context Catalog<'data>,
    pub(super) constants: &'data ProfiledConstantTable<Graph>,
    pub(super) sources: &'context Sources<'data>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum InstructionError {
    Integer(super::literal::IntegerError),
    Type(TypeError),
    Local(LocalError),
    Call(CallError),
    Constant(ConstantError),
    Source(SourceError),
    OutputType,
    OperandType,
    Flow,
    TupleIndex { index: usize },
    CustomField { index: usize },
    Arity { expected: usize, found: usize },
    ZeroBitUnit,
    FunctionFamily,
    CaptureTarget { index: usize },
}

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    <Graph::ExternalListInstruction as crate::plan::execution::graph::ExternalListInstructionView>::FunctionLocal: Operand,
{
    pub(super) fn check(&self, instruction: &'data crate::plan::execution::graph::ProfiledInstruction<Graph>, locals: &Locals<'data>) -> Result<(), InstructionError> {
        use crate::plan::execution::graph::{ProfiledInstructionKind as Kind, ExternalInstructionView, ExternalListInstructionView, ExternalFunctionInstructionView};
        let output = &instruction.output;
        match &instruction.kind {
            Kind::Int(value) => self.int(value, output, locals),
            Kind::Float(value) => self.float(value, output, locals),
            Kind::String(value) => self.string(value, output, locals),
            Kind::BitArray(value) => self.bit_array(value, output, locals),
            Kind::UtfCodepoint(value) => self.utf_codepoint(value, output, locals),
            Kind::Custom(value) => self.custom(value, output, locals),
            Kind::External(value) => self.external(value.instruction_ref(), output, locals),
            Kind::ExternalList(value) => self.typed_list(crate::plan::execution::type_::ListStorageTypeId::External(value.type_id()), value.instruction(), output, locals),
            Kind::ExternalFunction(value) => self.external_function(value.instruction(), output, locals),
            Kind::Bool(value) => self.bool(value, output, locals),
            Kind::Nil(value) => self.nil(value, output, locals),
            Kind::Tuple(value) => self.tuple(value, output, locals),
            Kind::List(value) => self.list(value, output, locals),
            Kind::Function(value) => self.function(value, output, locals),
        }
    }
}

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    fn output_type(
        &self,
        output: &ParamSlot,
        expected: &ValueType,
    ) -> Result<(), InstructionError> {
        let output = self.types.slot(output).map_err(InstructionError::Type)?;
        if output.type_ != expected {
            return Err(InstructionError::OutputType);
        }
        Ok(())
    }

    fn flow(&self, source: ValueShapeId, target: ValueShapeId) -> Result<(), InstructionError> {
        if !self
            .types
            .can_flow(source, target)
            .map_err(InstructionError::Type)?
        {
            return Err(InstructionError::Flow);
        }
        Ok(())
    }

    fn constant<Return: ConstantValue>(
        &self,
        id: ConstantId<Return>,
        output: &ParamSlot,
    ) -> Result<(), InstructionError> {
        constant::reference(self.constants, id, output, self.types)
            .map_err(InstructionError::Constant)
    }

    fn call(
        &self,
        function: &dyn Target,
        args: &[ParamLocal],
        site: &HostCallSite,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.sources
            .span(site.module(), site.span())
            .map_err(InstructionError::Source)?;
        function
            .resolve(self.catalog, self.types)
            .map_err(InstructionError::Call)?
            .call(args, output, locals, self.types)
            .map_err(InstructionError::Call)
    }

    fn indirect(
        &self,
        function: &dyn Operand,
        args: &[ParamLocal],
        site: &HostCallSite,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.sources
            .span(site.module(), site.span())
            .map_err(InstructionError::Source)?;
        call::indirect(function, args, output, locals, self.types).map_err(InstructionError::Call)
    }

    fn tuple_index(
        &self,
        tuple: &TupleLocalId,
        index: usize,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let tuple = locals.tuple(*tuple).map_err(InstructionError::Local)?;
        let source = *tuple
            .elements
            .get(index)
            .ok_or(InstructionError::TupleIndex { index })?;
        if !locals
            .projected_value(output, source)
            .flows_to(output.shape, self.types)
        {
            return Err(InstructionError::Flow);
        }
        Ok(())
    }

    fn custom_field(
        &self,
        source: &CustomLocal,
        index: usize,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let slot = read(source, locals)?;
        let shape = &self.types.shapes.custom_shapes[source.shape.shape_id.0];
        let type_ = &self.types.customs.types[shape.type_id.index()];
        let mut found = false;
        for constructor in type_.constructors.iter() {
            if !locals.allows_constructor(&slot.local, constructor.id.index)
                || matches!(shape.constructor, CustomConstructorRefinement::Exact(selected) if selected != constructor.id.index)
            {
                continue;
            }
            let field = constructor
                .fields
                .get(index)
                .ok_or(InstructionError::CustomField { index })?;
            self.output_type(output, &field.type_)?;
            if !self
                .types
                .can_leave_field(&field.refinement, &shape.arguments, output.shape)
            {
                return Err(InstructionError::Flow);
            }
            found = true;
        }
        if !found {
            return Err(InstructionError::CustomField { index });
        }
        Ok(())
    }

    fn list_index(
        &self,
        list: &(impl Copy + Into<Address>),
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let list = locals.list(*list).map_err(InstructionError::Local)?;
        if !locals
            .projected_value(output, list.item)
            .flows_to(output.shape, self.types)
        {
            return Err(InstructionError::Flow);
        }
        Ok(())
    }
}

fn read<'data>(
    value: &dyn Operand,
    locals: &Locals<'data>,
) -> Result<Slot<'data, 'data>, InstructionError> {
    value.read(locals).map_err(InstructionError::Local)
}

fn pair(
    left: &dyn Operand,
    right: &dyn Operand,
    locals: &Locals<'_>,
) -> Result<(), InstructionError> {
    read(left, locals)?;
    read(right, locals)?;
    Ok(())
}

fn same_type(
    left: &dyn Operand,
    right: &dyn Operand,
    locals: &Locals<'_>,
) -> Result<(), InstructionError> {
    let left = read(left, locals)?;
    let right = read(right, locals)?;
    if left.type_ != right.type_ {
        return Err(InstructionError::OperandType);
    }
    Ok(())
}

fn local_flow<'data>(
    types: &Types<'data>,
    source: Slot<'data, 'data>,
    target: ValueShapeId,
    locals: &Locals<'data>,
) -> Result<(), InstructionError> {
    if !locals.value(source).flows_to(target, types) {
        return Err(InstructionError::Flow);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{InstructionError, Instructions, LocalError, Locals, TypeError, pair, same_type};
    use crate::plan::execution::graph::{
        BoolLocalId, IntLocalId, ParamLocal, ParamSlot, TupleLocalId,
    };
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog, source::Sources, type_::Types,
    };
    use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId, ValueType};

    #[test]
    fn operands_and_container_projections_validate_raw_links_before_execution() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { #(42, True, #(21), [42]) }",
        )
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let integer = ParamSlot {
            local: ParamLocal::Int(IntLocalId(0)),
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Int)
                    .unwrap(),
            ),
        };
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
        let tuple = ParamSlot {
            local: ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: vec![ValueType::Int].into(),
            },
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| {
                        shape == &ValueShapeDescriptor::Tuple(vec![integer.shape].into())
                    })
                    .unwrap(),
            ),
        };
        let list = ParamSlot {
            local: ParamLocal::List(crate::plan::execution::graph::ListLocal::Int {
                local: crate::plan::execution::graph::IntListLocalId(0),
                type_id: crate::plan::execution::type_::IntListTypeId {
                    list_type: crate::plan::execution::type_::ListTypeId(0),
                },
            }),
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::List(integer.shape))
                    .unwrap(),
            ),
        };
        let mut locals = Locals::default();
        for slot in [&integer, &boolean, &tuple, &list] {
            locals.define(slot, &types).unwrap();
        }
        let missing_site = crate::plan::HostCallSite::from_static(
            "missing",
            "main",
            crate::plan::SourceSpan::new(0, 0),
        );
        let source_error =
            InstructionError::Source(super::SourceError::MissingModule("missing".into()));
        let target = crate::plan::execution::function::IntFunctionId(99);
        assert_eq!(
            context.call(&target, &[], &missing_site, &integer, &locals),
            Err(source_error)
        );
        assert_eq!(
            context.indirect(&integer.local, &[], &missing_site, &integer, &locals),
            Err(InstructionError::Source(super::SourceError::MissingModule(
                "missing".into()
            )))
        );
        let valid_site = crate::plan::HostCallSite::from_static(
            "example",
            "main",
            crate::plan::SourceSpan::new(0, 0),
        );
        assert_eq!(
            context.call(&target, &[], &valid_site, &integer, &locals),
            Err(InstructionError::Call(super::CallError::Catalog(
                super::super::catalog::CatalogError::MissingFunction {
                    family: crate::plan::execution::function::FunctionTableFamily::Int,
                    index: 99,
                }
            )))
        );
        assert_eq!(pair(&integer.local, &boolean.local, &locals), Ok(()));
        assert_eq!(same_type(&integer.local, &integer.local, &locals), Ok(()));
        assert_eq!(
            same_type(&integer.local, &boolean.local, &locals),
            Err(InstructionError::OperandType),
        );
        let missing = ParamLocal::Int(IntLocalId(99));
        for (left, right) in [(&missing, &integer.local), (&integer.local, &missing)] {
            assert_eq!(
                pair(left, right, &locals),
                Err(InstructionError::Local(LocalError::Missing(
                    IntLocalId(99).into()
                ))),
            );
            assert_eq!(
                same_type(left, right, &locals),
                Err(InstructionError::Local(LocalError::Missing(
                    IntLocalId(99).into()
                ))),
            );
        }
        assert_eq!(context.flow(integer.shape, integer.shape), Ok(()));
        assert_eq!(
            context.flow(integer.shape, boolean.shape),
            Err(InstructionError::Flow)
        );
        for (source, target) in [
            (ValueShapeId(99), integer.shape),
            (integer.shape, ValueShapeId(99)),
        ] {
            assert_eq!(
                context.flow(source, target),
                Err(InstructionError::Type(TypeError::MissingShape {
                    index: 99
                })),
            );
        }
        assert_eq!(
            context.tuple_index(&TupleLocalId(0), 0, &integer, &locals),
            Ok(())
        );
        assert_eq!(
            context.tuple_index(&TupleLocalId(99), 0, &integer, &locals),
            Err(InstructionError::Local(LocalError::Missing(
                TupleLocalId(99).into()
            ))),
        );
        assert_eq!(
            context.tuple_index(&TupleLocalId(0), 1, &integer, &locals),
            Err(InstructionError::TupleIndex { index: 1 }),
        );
        assert_eq!(
            context.tuple_index(&TupleLocalId(0), 0, &boolean, &locals),
            Err(InstructionError::Flow),
        );
        let invalid = ParamSlot {
            shape: ValueShapeId(99),
            ..integer.clone()
        };
        assert_eq!(
            context.int(
                &crate::plan::execution::graph::IntInstruction::TupleIndex {
                    tuple: TupleLocalId(0),
                    index: 0,
                },
                &invalid,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 99
            })),
        );
        use crate::plan::execution::graph::IntListLocalId;
        assert_eq!(
            context.list_index(&IntListLocalId(0), &integer, &locals),
            Ok(())
        );
        assert_eq!(
            context.list_index(&IntListLocalId(0), &boolean, &locals),
            Err(InstructionError::Flow)
        );
        assert_eq!(
            context.int(
                &crate::plan::execution::graph::IntInstruction::ListIndex {
                    list: IntListLocalId(0),
                    index: 0,
                },
                &invalid,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 99
            }))
        );
        assert_eq!(
            context.list_index(&IntListLocalId(99), &integer, &locals),
            Err(InstructionError::Local(LocalError::Missing(
                IntListLocalId(99).into()
            )))
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Int(42.into()),
                crate::Value::Bool(true),
                crate::Value::Tuple(vec![crate::Value::Int(21.into())]),
                crate::Value::List(crate::ListValue::int(vec![42.into()])),
            ]),
        );
    }
}
