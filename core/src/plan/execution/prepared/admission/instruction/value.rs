use super::super::local::Locals;
use super::local_flow;
use super::{InstructionError, Instructions, pair, read, same_type};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BoolInstruction, FloatInstruction, IntInstruction, NilInstruction, ParamSlot,
    StringInstruction, TupleInstruction, UtfCodepointInstruction,
};
use crate::plan::execution::type_::{ValueShapeDescriptor, ValueType};

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn int(
        &self,
        instruction: &IntInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::Int)?;
        match instruction {
            IntInstruction::Value(value) => {
                super::super::literal::integer(value).map_err(InstructionError::Integer)
            }
            IntInstruction::Constant(id) => self.constant(*id, output),
            IntInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            IntInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            IntInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            IntInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            IntInstruction::ListIndex { list, index: _ } => self.list_index(list, output, locals),
            IntInstruction::Add { left, right }
            | IntInstruction::Sub { left, right }
            | IntInstruction::Mult { left, right }
            | IntInstruction::Div { left, right }
            | IntInstruction::Remainder { left, right } => pair(left, right, locals),
            IntInstruction::Negate(value) => read(value, locals).map(|_| ()),
        }
    }

    pub(super) fn float(
        &self,
        instruction: &FloatInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::Float)?;
        match instruction {
            FloatInstruction::Value(_) => Ok(()),
            FloatInstruction::Constant(id) => self.constant(*id, output),
            FloatInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            FloatInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            FloatInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            FloatInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            FloatInstruction::ListIndex { list, index: _ } => self.list_index(list, output, locals),
            FloatInstruction::Add { left, right }
            | FloatInstruction::Sub { left, right }
            | FloatInstruction::Mult { left, right }
            | FloatInstruction::Div { left, right } => pair(left, right, locals),
        }
    }

    pub(super) fn string(
        &self,
        instruction: &StringInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::String)?;
        match instruction {
            StringInstruction::Value(_) => Ok(()),
            StringInstruction::Constant(id) => self.constant(*id, output),
            StringInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            StringInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            StringInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            StringInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            StringInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
            StringInstruction::Concatenate { left, right } => pair(left, right, locals),
            StringInstruction::DropPrefix { value, prefix: _ } => read(value, locals).map(|_| ()),
        }
    }

    pub(super) fn bool(
        &self,
        instruction: &BoolInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::Bool)?;
        match instruction {
            BoolInstruction::Value(_) => Ok(()),
            BoolInstruction::Constant(id) => self.constant(*id, output),
            BoolInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            BoolInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            BoolInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            BoolInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            BoolInstruction::ListIndex { list, index: _ } => self.list_index(list, output, locals),
            BoolInstruction::Not(value) => read(value, locals).map(|_| ()),
            BoolInstruction::LtInt { left, right }
            | BoolInstruction::LtEqInt { left, right }
            | BoolInstruction::GtInt { left, right }
            | BoolInstruction::GtEqInt { left, right } => pair(left, right, locals),
            BoolInstruction::LtFloat { left, right }
            | BoolInstruction::LtEqFloat { left, right }
            | BoolInstruction::GtFloat { left, right }
            | BoolInstruction::GtEqFloat { left, right } => pair(left, right, locals),
            BoolInstruction::Equal { left, right } | BoolInstruction::NotEqual { left, right } => {
                same_type(left, right, locals)
            }
            BoolInstruction::StringStartsWith { value, prefix: _ } => {
                read(value, locals).map(|_| ())
            }
            BoolInstruction::ListLengthEquals { value, length: _ }
            | BoolInstruction::ListLengthAtLeast { value, length: _ } => {
                read(value, locals).map(|_| ())
            }
        }
    }

    pub(super) fn nil(
        &self,
        instruction: &NilInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::Nil)?;
        match instruction {
            NilInstruction::Value => Ok(()),
            NilInstruction::Constant(id) => self.constant(*id, output),
            NilInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            NilInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            NilInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            NilInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            NilInstruction::ListIndex { list, index: _ } => self.list_index(list, output, locals),
        }
    }

    pub(super) fn utf_codepoint(
        &self,
        instruction: &UtfCodepointInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::UtfCodepoint)?;
        match instruction {
            UtfCodepointInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            UtfCodepointInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            UtfCodepointInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            UtfCodepointInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            UtfCodepointInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
        }
    }

    pub(super) fn tuple(
        &self,
        instruction: &TupleInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let admitted = self.types.slot(output).map_err(InstructionError::Type)?;
        let ValueShapeDescriptor::Tuple(expected) = admitted.descriptor else {
            return Err(InstructionError::OutputType);
        };
        match instruction {
            TupleInstruction::Value(elements) => {
                if elements.len() != expected.len() {
                    return Err(InstructionError::Arity {
                        expected: expected.len(),
                        found: elements.len(),
                    });
                }
                for (element, target) in elements.iter().zip(expected.iter()) {
                    local_flow(self.types, read(element, locals)?, *target, locals)?;
                }
                Ok(())
            }
            TupleInstruction::Constant(id) => self.constant(*id, output),
            TupleInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            TupleInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            TupleInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            TupleInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            TupleInstruction::ListIndex { list, index: _ } => self.list_index(list, output, locals),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{catalog::Catalog, source::Sources, type_::Types};
    use super::{InstructionError, Instructions, IntInstruction, Locals};
    use crate::plan::execution::function::FunctionBodyOwner;
    use crate::plan::execution::graph::{IntLocalId, ParamLocal, ProfiledInstructionKind};
    use std::convert::Infallible;

    #[test]
    fn validates_scalar_calls_arithmetic_comparisons_and_aggregate_projections() {
        let source = r#"
pub type Box { Box(Int, String, Float, Bool, Nil) }
pub type Wrapped(a) { Wrapped(a) }
const decimal_value = 1.5
const text_value = "constant"
const boolean_value = True
const nil_value = Nil
fn apply(callback: fn() -> a) -> a { callback() }
fn integer(x) { let _ = #(x / 2, x % 2, -x, x - 2) x + 1 }
fn decimal(x) { let _ = #(x +. 1.0, x -. 1.0, x *. 2.0) x /. 2.0 }
fn text(x) { x <> "!" }
fn boolean(x) { !x }
fn nothing() { Nil }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<65>> value }
fn first_float(values: List(Float)) { case values { [value, ..] -> value _ -> 0.0 } }
fn first_bool(values: List(Bool)) { case values { [value, ..] -> value _ -> False } }
fn first_nil(values: List(Nil)) { case values { [value, ..] -> value _ -> Nil } }
fn first_point(values: List(UtfCodepoint)) { case values { [value, ..] -> value _ -> panic } }
fn first_tuple(values: List(#(Int, Bool))) { case values { [value, ..] -> value _ -> #(0, False) } }
fn strip_prefix(value: String) { case value { "pre-" <> tail -> tail _ -> "" } }
pub fn main() {
  let Box(a, b, c, d, e) = Box(20, "text", 4.0, True, Nil)
  let tuple = #(a, b, c, d, e)
  let Wrapped(point) = Wrapped(codepoint())
  let point_tuple = #(point)
  let assert [from_points] = [point]
  let Wrapped(nested) = Wrapped(#(point_tuple, point))
  let assert [from_tuples] = [nested]
  let assert [from_floats] = [c]
  let assert [from_bools] = [d]
  let assert [from_nils] = [e]
  let assert "pre-" <> suffix = "pre-text"
  let _ = #(point_tuple.0, from_points, from_tuples.0, from_floats, from_bools, from_nils, suffix)
  let _ = #(first_float([c]), first_bool([d]), first_nil([e]), first_point([point]),
    first_tuple([#(a, d)]), strip_prefix("pre-text"))
  let _ = #(a < 2, a <= 2, a >= 2, a != 20, c <=. 10.0, c >. 10.0, c >=. 10.0)
  let _ = #(decimal_value, text_value, boolean_value, nil_value,
    apply(fn() { 1.5 }), apply(fn() { "text" }), apply(fn() { True }),
    apply(fn() { Nil }), apply(codepoint), apply(fn() { #(42, True) }))
  #(integer(tuple.0) * 2, text(tuple.1), decimal(tuple.2), boolean(tuple.3),
    nothing(), tuple.4, a > 2, c <. 10.0, a == 20)
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
        let mut counts = [0; 7];
        for function in plan.program.functions.value_returns.int_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan.program.functions.value_returns.float_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan.program.functions.value_returns.string_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan.program.functions.value_returns.bool_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan.program.functions.value_returns.nil_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan.program.functions.value_returns.tuple_functions.iter() {
            check_body(&context, function.body(), &mut counts);
        }
        for function in plan
            .program
            .functions
            .value_returns
            .utf_codepoint_functions
            .iter()
        {
            check_body(&context, function.body(), &mut counts);
        }
        assert!(counts.iter().all(|count| *count > 0), "{counts:?}");
    }

    fn check_body<'data>(
        context: &Instructions<'_, 'data, Infallible>,
        body: &'data impl FunctionBodyOwner<Graph = Infallible>,
        counts: &mut [usize; 7],
    ) {
        for block in body.function_body().block_graph().blocks() {
            let mut locals = Locals::default();
            for slot in block.params() {
                locals.define(slot, context.types).unwrap();
            }
            for instruction in block.instructions() {
                let output = instruction.output();
                let checked = match instruction.kind() {
                    ProfiledInstructionKind::Int(value) => {
                        counts[0] += 1;
                        context.int(value, output, &locals)
                    }
                    ProfiledInstructionKind::Float(value) => {
                        counts[1] += 1;
                        context.float(value, output, &locals)
                    }
                    ProfiledInstructionKind::String(value) => {
                        counts[2] += 1;
                        context.string(value, output, &locals)
                    }
                    ProfiledInstructionKind::Bool(value) => {
                        counts[3] += 1;
                        context.bool(value, output, &locals)
                    }
                    ProfiledInstructionKind::Nil(value) => {
                        counts[4] += 1;
                        context.nil(value, output, &locals)
                    }
                    ProfiledInstructionKind::Tuple(value) => {
                        counts[5] += 1;
                        context.tuple(value, output, &locals)
                    }
                    ProfiledInstructionKind::UtfCodepoint(value) => {
                        counts[6] += 1;
                        context.utf_codepoint(value, output, &locals)
                    }
                    _ => Ok(()),
                };
                checked.unwrap();
                locals.define(output, context.types).unwrap();
            }
        }
    }

    #[test]
    fn rejects_wrong_opcode_family_and_tuple_index_before_running_instructions() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { let value = #(42, True) value.0 }",
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
        let graph = plan.program.functions.value_returns.int_functions[0]
            .body()
            .block_graph();
        let block = graph.block(graph.entry);
        let mut locals = Locals::default();
        let mut checked = 0;
        for instruction in block.instructions() {
            if let ProfiledInstructionKind::Int(IntInstruction::TupleIndex { tuple, index: _ }) =
                instruction.kind()
            {
                assert_eq!(
                    context.int(
                        &IntInstruction::TupleIndex {
                            tuple: *tuple,
                            index: 2
                        },
                        instruction.output(),
                        &locals
                    ),
                    Err(InstructionError::TupleIndex { index: 2 })
                );
                let boolean = block
                    .instructions()
                    .iter()
                    .find(|value| matches!(value.output().local, ParamLocal::Bool(_)))
                    .unwrap();
                assert_eq!(
                    context.int(
                        &IntInstruction::Negate(IntLocalId(0)),
                        boolean.output(),
                        &locals
                    ),
                    Err(InstructionError::OutputType)
                );
                checked += 1;
            }
            locals.define(instruction.output(), &types).unwrap();
        }
        assert_eq!(checked, 1);
        let int_output = block.instructions()[0].output();
        assert_eq!(int_output.local, ParamLocal::Int(IntLocalId(0)));
        use crate::plan::execution::graph::{
            BoolInstruction, FloatInstruction, NilInstruction, StringInstruction, TupleInstruction,
            TupleLocalId, UtfCodepointInstruction,
        };
        for result in [
            context.float(&FloatInstruction::Value(0.0), int_output, &locals),
            context.string(
                &StringInstruction::Value("text".into()),
                int_output,
                &locals,
            ),
            context.bool(&BoolInstruction::Value(false), int_output, &locals),
            context.nil(&NilInstruction::Value, int_output, &locals),
            context.utf_codepoint(
                &UtfCodepointInstruction::TupleIndex {
                    tuple: TupleLocalId(0),
                    index: 0,
                },
                int_output,
                &locals,
            ),
            context.tuple(
                &TupleInstruction::Value(Vec::new().into()),
                int_output,
                &locals,
            ),
        ] {
            assert_eq!(result, Err(InstructionError::OutputType));
        }
        let tuple_output = block
            .instructions()
            .iter()
            .find_map(|instruction| {
                matches!(
                    instruction.kind(),
                    ProfiledInstructionKind::Tuple(TupleInstruction::Value(_))
                )
                .then_some(instruction.output())
            })
            .unwrap();
        assert_eq!(
            context.tuple(
                &TupleInstruction::Value(Vec::new().into()),
                tuple_output,
                &locals
            ),
            Err(InstructionError::Arity {
                expected: 2,
                found: 0
            }),
        );
        use crate::plan::execution::prepared::admission::{local::LocalError, type_::TypeError};
        use crate::plan::execution::type_::ValueShapeId;
        let mut invalid = tuple_output.clone();
        invalid.shape = ValueShapeId(999);
        assert_eq!(
            context.tuple(
                &TupleInstruction::Value(Vec::new().into()),
                &invalid,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 999
            }))
        );
        let boolean = block
            .instructions()
            .iter()
            .find(|value| matches!(value.output().local, ParamLocal::Bool(_)))
            .unwrap()
            .output()
            .local
            .clone();
        assert_eq!(
            context.tuple(
                &TupleInstruction::Value(
                    vec![ParamLocal::Int(IntLocalId(99)), boolean.clone()].into()
                ),
                tuple_output,
                &locals
            ),
            Err(InstructionError::Local(LocalError::Missing(
                IntLocalId(99).into()
            )))
        );
        assert_eq!(
            context.tuple(
                &TupleInstruction::Value(vec![boolean.clone(), boolean].into()),
                tuple_output,
                &locals
            ),
            Err(InstructionError::Flow)
        );
    }
}
