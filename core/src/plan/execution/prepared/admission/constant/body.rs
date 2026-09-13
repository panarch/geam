use super::super::block::{BlockError, Blocks};
use super::super::body::{self, BodyError, ExitError};
use super::super::call::Target;
use super::super::instruction::Instructions;
use super::super::operand::Operand;
use crate::plan::execution::constant::{ProfiledConstantProgram, ProfiledConstantTable};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::ExternalListInstructionView;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) struct ConstantBodyError {
    pub family: &'static str,
    pub index: usize,
    pub error: Box<BodyError>,
}

pub(in crate::plan::execution::prepared::admission) fn all<'data, Graph: ExecutionGraphProfile>(
    constants: &'data ProfiledConstantTable<Graph>,
    context: &Instructions<'_, 'data, Graph>,
) -> Result<(), ConstantBodyError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    <Graph::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    let ProfiledConstantTable {
        ints,
        strings,
        bit_arrays,
        customs,
        floats,
        bools,
        nils,
        tuples,
        parameter_lists,
        parameter_list_lists,
        int_lists,
        string_lists,
        bit_array_lists,
        utf_codepoint_lists,
        custom_lists,
        external_lists,
        float_lists,
        bool_lists,
        nil_lists,
        tuple_lists,
        list_lists,
        function_lists,
        functions,
    } = constants;
    table("ints", ints, context)?;
    table("strings", strings, context)?;
    table("bit_arrays", bit_arrays, context)?;
    table("customs", customs, context)?;
    table("floats", floats, context)?;
    table("bools", bools, context)?;
    table("nils", nils, context)?;
    table("tuples", tuples, context)?;
    table("parameter_lists", parameter_lists, context)?;
    table("parameter_list_lists", parameter_list_lists, context)?;
    table("int_lists", int_lists, context)?;
    table("string_lists", string_lists, context)?;
    table("bit_array_lists", bit_array_lists, context)?;
    table("utf_codepoint_lists", utf_codepoint_lists, context)?;
    table("custom_lists", custom_lists, context)?;
    table("external_lists", external_lists, context)?;
    table("float_lists", float_lists, context)?;
    table("bool_lists", bool_lists, context)?;
    table("nil_lists", nil_lists, context)?;
    table("tuple_lists", tuple_lists, context)?;
    table("list_lists", list_lists, context)?;
    table("function_lists", function_lists, context)?;
    table("functions", functions, context)?;
    Ok(())
}

fn table<'data, Return: Operand + 'static, Graph: ExecutionGraphProfile>(
    family: &'static str,
    programs: &'data [ProfiledConstantProgram<Return, Graph>],
    context: &Instructions<'_, 'data, Graph>,
) -> Result<(), ConstantBodyError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    <Graph::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    for (index, program) in programs.iter().enumerate() {
        program_body(program, context).map_err(|error| ConstantBodyError {
            family,
            index,
            error: Box::new(error),
        })?;
    }
    Ok(())
}

fn program_body<'data, Return: Operand + 'static, Graph: ExecutionGraphProfile>(
    program: &'data ProfiledConstantProgram<Return, Graph>,
    context: &Instructions<'_, 'data, Graph>,
) -> Result<(), BodyError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    <Graph::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    context
        .types
        .shape_type(program.shape)
        .map_err(BodyError::Type)?;
    let blocks = Blocks::admit(&program.block_graph).map_err(BodyError::Block)?;
    let count = blocks.entry_block().params().len();
    if count != 0 {
        return Err(BodyError::Block(BlockError::ParameterCount {
            expected: 0,
            found: count,
        }));
    }
    let mut used = vec![false; program.returns.len()];
    body::graph(&blocks, context, &mut |exit, locals| {
        let value = program
            .returns
            .get(exit.index())
            .ok_or(ExitError::Missing {
                index: exit.index(),
            })?;
        used[exit.index()] = true;
        let value = value.read(locals).map_err(ExitError::Local)?;
        body::return_value(locals.value(value), program.shape, context.types)
    })?;
    body::claimed(&used)
}

#[cfg(test)]
mod tests {
    use super::{
        BlockError, BodyError, ConstantBodyError, ExitError, Instructions, all, program_body,
    };
    use crate::host::{HostProviderSet, StatelessHostProfile};
    use crate::plan::execution::constant::ProfiledConstantTable;
    use crate::plan::execution::function::HostedExecutionGraph;
    use crate::plan::execution::graph::BlockId;
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog,
        source::Sources,
        tests::{lowered_native, owned_mut},
        type_::Types,
    };

    #[test]
    fn constant_admission_checks_shape_parameters_and_return_ownership() {
        use crate::plan::execution::constant::ProfiledConstantProgram;
        use crate::plan::execution::graph::{
            BlockGraphExitId, IntInstruction, IntLocalId, ParamLocal, ParamSlot, ProfiledBlock,
            ProfiledBlockGraph, ProfiledInstruction, ProfiledInstructionKind, Terminator,
        };
        use crate::plan::execution::prepared::admission::{
            instruction::InstructionError, local::LocalError, type_::TypeError,
        };
        use crate::plan::execution::type_::{ValueShapeId, ValueType};

        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    "const saved = 42 pub fn main() { #(saved, True) }",
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let (program, _) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(typed).unwrap(),
        )
        .unwrap();
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
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let int = common.constants.ints[0].shape;
        let boolean = ValueShapeId(
            common
                .value_shapes
                .shape_types
                .iter()
                .position(|type_| type_ == &ValueType::Bool)
                .unwrap(),
        );
        let slot = ParamSlot::new(ParamLocal::Int(IntLocalId(0)), int);
        let literal = ProfiledInstruction::<HostedExecutionGraph> {
            output: slot.clone(),
            kind: ProfiledInstructionKind::Int(IntInstruction::Value(
                num_bigint::BigInt::from(42).into(),
            )),
        };
        let cases = [
            (
                int,
                Vec::new(),
                vec![literal.clone()],
                vec![IntLocalId(0)],
                Ok(()),
            ),
            (
                ValueShapeId(99),
                Vec::new(),
                vec![literal.clone()],
                vec![IntLocalId(0)],
                Err(BodyError::Type(TypeError::MissingShape { index: 99 })),
            ),
            (
                int,
                vec![slot.clone()],
                Vec::new(),
                vec![IntLocalId(0)],
                Err(BodyError::Block(BlockError::ParameterCount {
                    expected: 0,
                    found: 1,
                })),
            ),
            (
                int,
                Vec::new(),
                vec![literal.clone()],
                Vec::new(),
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::Missing { index: 0 },
                }),
            ),
            (
                int,
                Vec::new(),
                vec![literal.clone()],
                vec![IntLocalId(99)],
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::Local(LocalError::Missing(IntLocalId(99).into())),
                }),
            ),
            (
                boolean,
                Vec::new(),
                vec![literal.clone()],
                vec![IntLocalId(0)],
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::ReturnType,
                }),
            ),
            (
                int,
                Vec::new(),
                vec![literal],
                vec![IntLocalId(0), IntLocalId(0)],
                Err(BodyError::UnclaimedExit { index: 1 }),
            ),
            (
                int,
                Vec::new(),
                vec![ProfiledInstruction {
                    output: slot,
                    kind: ProfiledInstructionKind::Int(IntInstruction::Negate(IntLocalId(99))),
                }],
                vec![IntLocalId(0)],
                Err(BodyError::Instruction {
                    block: 0,
                    index: 0,
                    error: InstructionError::Local(LocalError::Missing(IntLocalId(99).into())),
                }),
            ),
        ];
        for (shape, parameters, instructions, returns, expected) in cases {
            let input = ProfiledConstantProgram {
                block_graph: ProfiledBlockGraph::from_parts(
                    BlockId(0),
                    vec![ProfiledBlock::new(
                        parameters,
                        instructions,
                        Terminator::Exit(BlockGraphExitId(0)),
                    )],
                ),
                returns: returns.into(),
                shape,
            };
            assert_eq!(program_body(&input, &context), expected);
        }
    }

    #[test]
    fn every_constant_family_checks_its_body_before_loading() {
        type Mutation = fn(&mut ProfiledConstantTable<HostedExecutionGraph>);
        let cases: &[(&str, &str, Mutation)] = &[
            ("ints", "const saved = 42", |table| {
                owned_mut(&mut table.ints)[0].block_graph.entry = BlockId(99)
            }),
            ("strings", "const saved = \"text\"", |table| {
                owned_mut(&mut table.strings)[0].block_graph.entry = BlockId(99)
            }),
            ("bit_arrays", "const saved = <<1, 2>>", |table| {
                owned_mut(&mut table.bit_arrays)[0].block_graph.entry = BlockId(99)
            }),
            ("customs", "const saved = Box(42)", |table| {
                owned_mut(&mut table.customs)[0].block_graph.entry = BlockId(99)
            }),
            ("floats", "const saved = 1.5", |table| {
                owned_mut(&mut table.floats)[0].block_graph.entry = BlockId(99)
            }),
            ("bools", "const saved = True", |table| {
                owned_mut(&mut table.bools)[0].block_graph.entry = BlockId(99)
            }),
            ("nils", "const saved = Nil", |table| {
                owned_mut(&mut table.nils)[0].block_graph.entry = BlockId(99)
            }),
            ("tuples", "const saved = #(42, True)", |table| {
                owned_mut(&mut table.tuples)[0].block_graph.entry = BlockId(99)
            }),
            ("parameter_lists", "const saved = []", |table| {
                owned_mut(&mut table.parameter_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("parameter_list_lists", "const saved = [[]]", |table| {
                owned_mut(&mut table.parameter_list_lists)[0]
                    .block_graph
                    .entry = BlockId(99)
            }),
            ("int_lists", "const saved = [42]", |table| {
                owned_mut(&mut table.int_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("string_lists", "const saved = [\"text\"]", |table| {
                owned_mut(&mut table.string_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("bit_array_lists", "const saved = [<<1>>]", |table| {
                owned_mut(&mut table.bit_array_lists)[0].block_graph.entry = BlockId(99)
            }),
            (
                "utf_codepoint_lists",
                "const saved: List(UtfCodepoint) = []",
                |table| {
                    owned_mut(&mut table.utf_codepoint_lists)[0]
                        .block_graph
                        .entry = BlockId(99)
                },
            ),
            ("custom_lists", "const saved = [Box(42)]", |table| {
                owned_mut(&mut table.custom_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("external_lists", "const saved: List(Key) = []", |table| {
                owned_mut(&mut table.external_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("float_lists", "const saved = [1.5]", |table| {
                owned_mut(&mut table.float_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("bool_lists", "const saved = [True]", |table| {
                owned_mut(&mut table.bool_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("nil_lists", "const saved = [Nil]", |table| {
                owned_mut(&mut table.nil_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("tuple_lists", "const saved = [#(42, True)]", |table| {
                owned_mut(&mut table.tuple_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("list_lists", "const saved = [[42]]", |table| {
                owned_mut(&mut table.list_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("function_lists", "const saved = [increment]", |table| {
                owned_mut(&mut table.function_lists)[0].block_graph.entry = BlockId(99)
            }),
            ("functions", "const saved = increment", |table| {
                owned_mut(&mut table.functions)[0].block_graph.entry = BlockId(99)
            }),
        ];
        for (family, declaration, mutate) in cases {
            let source = format!(
                "pub type Box(a) {{ Box(a) }} pub type Key\n\
                 fn increment(value: Int) {{ value + 1 }}\n\
                 @external(erlang, \"native\", \"key\") fn key() -> Key\n\
                 {declaration}\npub fn main() {{ echo saved key() }}"
            );
            let (program, _, _) = lowered_native(&source);
            let mut common = std::sync::Arc::try_unwrap(program.common).ok().unwrap();
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            assert_eq!(
                all(
                    &common.constants,
                    &Instructions {
                        types: &types,
                        catalog: &catalog,
                        sources: &sources,
                        constants: &common.constants,
                    }
                ),
                Ok(()),
                "{family}"
            );

            mutate(owned_mut(&mut common.constants));
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            assert_eq!(
                all(
                    &common.constants,
                    &Instructions {
                        types: &types,
                        catalog: &catalog,
                        sources: &sources,
                        constants: &common.constants,
                    }
                ),
                Err(ConstantBodyError {
                    family,
                    index: 0,
                    error: Box::new(BodyError::Block(BlockError::Missing { index: 99 }))
                }),
                "{family}"
            );
        }
    }
}
