use crate::plan::HostCallSite;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, ExecutionGraphProfile, FloatFunctionId, IntFunctionId,
    NilFunctionId, StringFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayInstruction, BlockGraphExitId, BlockId, BoolFunctionLocalId,
    BoolInstruction, FloatFunctionLocalId, FloatInstruction, IntFunctionLocalId, IntInstruction,
    IntLocalId, IntegerOperand, NilFunctionLocalId, NilInstruction, ParamLocal, ProfiledBlockGraph,
    ProfiledInstructionKind, StringFunctionLocalId, StringInstruction, Terminator,
    UtfCodepointFunctionLocalId, UtfCodepointInstruction,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};

/// The canonical callback/native repetition, including its typed producer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeLoopContract {
    pub header: BlockId,
    pub finished: BlockId,
    pub repeat: BlockId,
    pub exit: BlockGraphExitId,
    pub native: NativeLoopTarget,
    pub producer: NativeLoopProducer,
    pub site: HostCallSite,
}

/// Supported scalar targets share one bounded native-loop implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeLoopTarget {
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
}

/// An argument-free producer with one supported scalar return family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeLoopProducer {
    Int(IntFunctionLocalId),
    Float(FloatFunctionLocalId),
    String(StringFunctionLocalId),
    BitArray(BitArrayFunctionLocalId),
    UtfCodepoint(UtfCodepointFunctionLocalId),
    Bool(BoolFunctionLocalId),
    Nil(NilFunctionLocalId),
}

impl Emit for NativeLoopProducer {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(local) => output.call("compiled::NativeLoopProducer::Int", &[local]),
            Self::Float(local) => output.call("compiled::NativeLoopProducer::Float", &[local]),
            Self::String(local) => output.call("compiled::NativeLoopProducer::String", &[local]),
            Self::BitArray(local) => {
                output.call("compiled::NativeLoopProducer::BitArray", &[local])
            }
            Self::UtfCodepoint(local) => {
                output.call("compiled::NativeLoopProducer::UtfCodepoint", &[local])
            }
            Self::Bool(local) => output.call("compiled::NativeLoopProducer::Bool", &[local]),
            Self::Nil(local) => output.call("compiled::NativeLoopProducer::Nil", &[local]),
        }
    }
}

impl NativeLoopTarget {
    pub(crate) fn key(self) -> (usize, usize) {
        match self {
            Self::Int(id) => (0, id.0),
            Self::Float(id) => (1, id.0),
            Self::String(id) => (2, id.0),
            Self::BitArray(id) => (3, id.0),
            Self::UtfCodepoint(id) => (4, id.0),
            Self::Bool(id) => (5, id.0),
            Self::Nil(id) => (6, id.0),
        }
    }
}

impl Emit for NativeLoopTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(id) => output.call("compiled::NativeLoopTarget::Int", &[id]),
            Self::Float(id) => output.call("compiled::NativeLoopTarget::Float", &[id]),
            Self::String(id) => output.call("compiled::NativeLoopTarget::String", &[id]),
            Self::BitArray(id) => output.call("compiled::NativeLoopTarget::BitArray", &[id]),
            Self::UtfCodepoint(id) => {
                output.call("compiled::NativeLoopTarget::UtfCodepoint", &[id])
            }
            Self::Bool(id) => output.call("compiled::NativeLoopTarget::Bool", &[id]),
            Self::Nil(id) => output.call("compiled::NativeLoopTarget::Nil", &[id]),
        }
    }
}

impl NativeLoopContract {
    pub(crate) fn checkpoint(&self) -> super::CompiledCheckpoint {
        // The producer variant fixes its argument-free scalar signature. These
        // are the existing checkpoint columns, not another callback ABI.
        super::CompiledCheckpoint {
            block: self.header,
            instruction: 0,
            ints: 1,
            bools: 0,
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: usize::from(matches!(self.producer, NativeLoopProducer::Int(_))),
            bool_functions: usize::from(matches!(self.producer, NativeLoopProducer::Bool(_))),
        }
    }

    pub(crate) fn inspect<Graph: ExecutionGraphProfile>(
        graph: &ProfiledBlockGraph<Graph>,
    ) -> Option<Self> {
        let header = graph.entry();
        let block = graph.block(header);
        let [counter, producer] = block.params() else {
            return None;
        };
        if counter.local != ParamLocal::Int(IntLocalId(0)) {
            return None;
        }
        let (producer_id, producer_type) = match &producer.local {
            ParamLocal::IntFunction {
                local: IntFunctionLocalId(0),
                type_,
            } => (NativeLoopProducer::Int(IntFunctionLocalId(0)), type_),
            ParamLocal::FloatFunction {
                local: FloatFunctionLocalId(0),
                type_,
            } => (NativeLoopProducer::Float(FloatFunctionLocalId(0)), type_),
            ParamLocal::StringFunction {
                local: StringFunctionLocalId(0),
                type_,
            } => (NativeLoopProducer::String(StringFunctionLocalId(0)), type_),
            ParamLocal::BitArrayFunction {
                local: BitArrayFunctionLocalId(0),
                type_,
            } => (
                NativeLoopProducer::BitArray(BitArrayFunctionLocalId(0)),
                type_,
            ),
            ParamLocal::UtfCodepointFunction {
                local: UtfCodepointFunctionLocalId(0),
                type_,
            } => (
                NativeLoopProducer::UtfCodepoint(UtfCodepointFunctionLocalId(0)),
                type_,
            ),
            ParamLocal::BoolFunction {
                local: BoolFunctionLocalId(0),
                type_,
            } => (NativeLoopProducer::Bool(BoolFunctionLocalId(0)), type_),
            ParamLocal::NilFunction {
                local: NilFunctionLocalId(0),
                type_,
            } => (NativeLoopProducer::Nil(NilFunctionLocalId(0)), type_),
            _ => return None,
        };
        if !producer_type.arguments.is_empty() {
            return None;
        }
        let [callback, native] = block.instructions() else {
            return None;
        };
        let callback = callback.value()?;
        // Admission has already sealed dense SSA locals and call arity. The
        // only function parameter is the argument-free producer, so a first
        // typed FunctionCall necessarily invokes that parameter.
        if !matches!(
            &callback.kind,
            ProfiledInstructionKind::Int(IntInstruction::FunctionCall { .. })
                | ProfiledInstructionKind::Float(FloatInstruction::FunctionCall { .. })
                | ProfiledInstructionKind::String(StringInstruction::FunctionCall { .. })
                | ProfiledInstructionKind::BitArray(BitArrayInstruction::FunctionCall { .. })
                | ProfiledInstructionKind::UtfCodepoint(
                    UtfCodepointInstruction::FunctionCall { .. }
                )
                | ProfiledInstructionKind::Bool(BoolInstruction::FunctionCall { .. })
                | ProfiledInstructionKind::Nil(NilInstruction::FunctionCall { .. })
        ) {
            return None;
        }
        let native = native.value()?;
        let (target, args, site) = match &native.kind {
            ProfiledInstructionKind::Int(IntInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::Int(*function), args, site),
            ProfiledInstructionKind::Float(FloatInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::Float(*function), args, site),
            ProfiledInstructionKind::String(StringInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::String(*function), args, site),
            ProfiledInstructionKind::BitArray(BitArrayInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::BitArray(*function), args, site),
            ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::UtfCodepoint(*function), args, site),
            ProfiledInstructionKind::Bool(BoolInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::Bool(*function), args, site),
            ProfiledInstructionKind::Nil(NilInstruction::Call {
                function,
                args,
                site,
            }) => (NativeLoopTarget::Nil(*function), args, site),
            _ => return None,
        };
        if args.as_ref() != [callback.output.local.clone()] {
            return None;
        }
        let Terminator::IntSwitch(switch) = block.terminator() else {
            return None;
        };
        let [(literal, done)] = switch.clauses() else {
            return None;
        };
        if switch.subject() != IntLocalId(0)
            || literal.materialize() != 1.into()
            || done.args.as_ref() != [native.output.local.clone()]
        {
            return None;
        }
        let finished = done.target;
        let repeat = switch.fallback.target;
        if switch.fallback.args.as_ref() != [counter.local.clone(), producer.local.clone()] {
            return None;
        }
        let done = graph.block(finished);
        // Typed graph construction, and function admission for external
        // artifacts, already seal SSA outputs and dense edge parameters.
        // The one result edge above fixes the finished block's scalar slot;
        // the two-argument backedge fixes the repeat block's parameter layout.
        if !done.instructions().is_empty() {
            return None;
        }
        let Terminator::Exit(exit) = done.terminator() else {
            return None;
        };
        let loop_ = graph.block(repeat);
        let [subtract] = loop_.instructions() else {
            return None;
        };
        let subtract = subtract.value()?;
        if !matches!(
            &subtract.kind,
            ProfiledInstructionKind::Int(IntInstruction::Sub {
                left: IntegerOperand::Local(IntLocalId(0)),
                right: IntegerOperand::Immediate(1)
            })
        ) {
            return None;
        }
        let Terminator::Jump(jump) = loop_.terminator() else {
            return None;
        };
        if jump.edge.target != header
            || jump.edge.args.as_ref() != [ParamLocal::Int(IntLocalId(1)), producer.local.clone()]
            || graph.blocks().len() != 3
        {
            return None;
        }
        Some(Self {
            header,
            finished,
            repeat,
            exit: *exit,
            native: target,
            producer: producer_id,
            site: site.clone(),
        })
    }
}

impl Emit for NativeLoopContract {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::NativeLoopContract",
            &[
                ("header", &self.header),
                ("finished", &self.finished),
                ("repeat", &self.repeat),
                ("exit", &self.exit),
                ("native", &self.native),
                ("producer", &self.producer),
                ("site", &self.site),
            ],
        );
    }
}
#[cfg(test)]
mod tests {
    use super::{NativeLoopContract, NativeLoopProducer, NativeLoopTarget};
    use crate::plan::HostCallSite;
    use crate::plan::SourceSpan;
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
        StringFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BoolFunctionLocalId, FloatFunctionLocalId, IntFunctionLocalId,
        NilFunctionLocalId, StringFunctionLocalId, UtfCodepointFunctionLocalId,
    };
    use crate::plan::execution::graph::{
        Block, BlockGraphExitId, BlockId, ProfiledBlockGraph, Terminator,
    };
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn only_the_original_callback_native_and_counter_cycle_is_selected() {
        use crate::host::HostProviderSet;
        use crate::plan::execution::function::{ExecutionFunctionEntry, ExecutionFunctionRef};
        use crate::{
            BitArrayValue, HostProviderModule, ModuleSource, PackageSource, StatelessHostProfile,
            StringValue,
        };
        use num_bigint::BigInt;
        let providers =
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(BigInt,), BigInt, _>("observe", std::convert::identity)
            .unwrap()
            .with_function::<(BigInt, BigInt), BigInt, _>("two", |value: BigInt, _: BigInt| value)
            .unwrap()
            .with_function::<(BigInt,), bool, _>("predicate", |_: BigInt| true)
            .unwrap()
            .with_function::<(f64,), f64, _>("float", std::convert::identity)
            .unwrap()
            .with_function::<(StringValue,), StringValue, _>("string", std::convert::identity)
            .unwrap()
            .with_function::<(BitArrayValue,), BitArrayValue, _>("bits", std::convert::identity)
            .unwrap()
            .with_function::<(char,), char, _>("codepoint", std::convert::identity)
            .unwrap()
            .with_function::<(bool,), bool, _>("bool", std::convert::identity)
            .unwrap()
            .with_function::<((),), (), _>("nil", std::convert::identity)
            .unwrap()])
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    r#"
@external(erlang, "native", "observe")
fn observe(value: Int) -> Int
@external(erlang, "native", "two")
fn two(value: Int, counter: Int) -> Int
@external(erlang, "native", "predicate")
fn predicate(value: Int) -> Bool
fn canonical(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> canonical(counter - 1, producer) }
}
fn argument(counter: Int, producer: fn(Int) -> Int) {
  let returned = observe(producer(counter))
  case counter { 1 -> returned _ -> argument(counter - 1, producer) }
}
fn literal(counter: Int, producer: fn() -> Int) {
  let returned = observe(7)
  case counter { 1 -> returned _ -> literal(counter - 1, producer) }
}
fn arithmetic(counter: Int, producer: fn() -> Int) {
  let returned = producer() + 1
  case counter { 1 -> returned _ -> arithmetic(counter - 1, producer) }
}
fn variadic(counter: Int, producer: fn() -> Int) {
  let returned = two(producer(), counter)
  case counter { 1 -> returned _ -> variadic(counter - 1, producer) }
}
fn result_switch(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case returned { 1 -> returned _ -> result_switch(counter - 1, producer) }
}
fn two_finishes(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned 2 -> returned _ -> two_finishes(counter - 1, producer) }
}
fn zero_finish(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 0 -> returned _ -> zero_finish(counter - 1, producer) }
}
fn original_return(counter: Int, producer: fn() -> Int) {
  let original = producer()
  let returned = observe(original)
  case counter { 1 -> original _ -> original_return(counter - 1, producer) }
}
fn boolean_switch(counter: Int, producer: fn() -> Int) {
  let returned = predicate(producer())
  case returned { True -> returned False -> boolean_switch(counter - 1, producer) }
}
fn finish_arithmetic(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned + 1 _ -> finish_arithmetic(counter - 1, producer) }
}
fn terminal_math(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> counter - 1 }
}
fn decrement_two(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> decrement_two(counter - 2, producer) }
}
fn echo_tail(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> { echo counter echo_tail(counter, producer) } }
}
fn echo_and_subtract(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> { echo counter echo_and_subtract(counter - 1, producer) } }
}
fn another_target(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> canonical(counter - 1, producer) }
}
fn producer_only(counter: Int, producer: fn() -> Int) {
  let returned = producer()
  case counter { 1 -> returned _ -> producer_only(counter - 1, producer) }
}
fn arithmetic_input(counter: Int, producer: fn() -> Int) {
  let returned = observe({ let adjusted = counter + 1 adjusted * 2 })
  case counter { 1 -> returned _ -> arithmetic_input(counter - 1, producer) }
}
fn arithmetic_result(counter: Int, producer: fn() -> Int) {
  let returned = { let value = producer() value + 1 } * 2
  case counter { 1 -> returned _ -> arithmetic_result(counter - 1, producer) }
}
fn finish_switch(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter {
    1 -> case returned { 1 -> returned _ -> returned + 1 }
    _ -> finish_switch(counter - 1, producer)
  }
}
fn decrement_region(counter: Int, producer: fn() -> Int) {
  let returned = observe(producer())
  case counter { 1 -> returned _ -> decrement_region(counter - 1 - 1, producer) }
}
@external(erlang, "native", "float")
fn float(value: Float) -> Float
@external(erlang, "native", "string")
fn string(value: String) -> String
@external(erlang, "native", "bits")
fn bits(value: BitArray) -> BitArray
@external(erlang, "native", "codepoint")
fn codepoint(value: UtfCodepoint) -> UtfCodepoint
@external(erlang, "native", "bool")
fn bool(value: Bool) -> Bool
@external(erlang, "native", "nil")
fn nil(value: Nil) -> Nil
fn float_loop(counter: Int, producer: fn() -> Float) {
  let returned = float(producer())
  case counter { 1 -> returned _ -> float_loop(counter - 1, producer) }
}
fn string_loop(counter: Int, producer: fn() -> String) {
  let returned = string(producer())
  case counter { 1 -> returned _ -> string_loop(counter - 1, producer) }
}
fn bits_loop(counter: Int, producer: fn() -> BitArray) {
  let returned = bits(producer())
  case counter { 1 -> returned _ -> bits_loop(counter - 1, producer) }
}
fn codepoint_loop(counter: Int, producer: fn() -> UtfCodepoint) {
  let returned = codepoint(producer())
  case counter { 1 -> returned _ -> codepoint_loop(counter - 1, producer) }
}
fn bool_loop(counter: Int, producer: fn() -> Bool) {
  let returned = bool(producer())
  case counter { 1 -> returned _ -> bool_loop(counter - 1, producer) }
}
fn nil_loop(counter: Int, producer: fn() -> Nil) {
  let returned = nil(producer())
  case counter { 1 -> returned _ -> nil_loop(counter - 1, producer) }
}
pub fn main() {
  #(canonical, argument, literal, arithmetic, variadic, result_switch,
    two_finishes, zero_finish, original_return, boolean_switch,
    finish_arithmetic, terminal_math, decrement_two, echo_tail,
    echo_and_subtract, another_target, float_loop, string_loop, bits_loop,
    codepoint_loop, bool_loop, nil_loop, producer_only, arithmetic_input,
    arithmetic_result, finish_switch, decrement_region)
}
"#,
                )],
            )],
            providers,
        )
        .unwrap();
        let execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let functions = &execution.execution.program.functions;
        // Every source body must be reachable from the selected root;
        // otherwise absence from the specialization is not rejection evidence.
        assert_eq!(functions.value_returns.int_functions.len(), 22);
        assert_eq!(functions.value_returns.bool_functions.len(), 4);
        let names = functions
            .value_returns
            .int_functions
            .iter()
            .filter_map(|entry| {
                let ExecutionFunctionRef::Graph(entry) = entry.as_ref() else {
                    return None;
                };
                NativeLoopContract::inspect(entry.body().block_graph())
                    .map(|contract| contract.site.function().to_string())
            })
            .collect::<Vec<_>>();
        assert_eq!(names, ["canonical"]);
        let selected = functions
            .value_returns
            .int_functions
            .iter()
            .filter_map(|entry| {
                let ExecutionFunctionRef::Graph(entry) = entry.as_ref() else {
                    return None;
                };
                let graph = entry.body().block_graph();
                NativeLoopContract::inspect(graph).map(|contract| (graph, contract))
            })
            .collect::<Vec<_>>();
        assert_eq!(selected.len(), 1);
        let (graph, contract) = &selected[0];
        // A same-shaped self-edge is a valid typed two-parameter transfer,
        // but it repeats subtraction without re-entering the producer/native.
        // This selector boundary is inspected only; it is not executed.
        let blocks = graph
            .blocks()
            .map(|block| {
                let mut terminator = block.terminator().clone();
                if let Terminator::Jump(jump) = &mut terminator {
                    jump.edge.target = contract.repeat;
                }
                Block::new(
                    block.params().to_vec(),
                    block.instructions().to_vec(),
                    terminator,
                )
            })
            .collect();
        assert_eq!(
            NativeLoopContract::inspect(&ProfiledBlockGraph::from_parts(graph.entry(), blocks)),
            None
        );
        let names = functions
            .value_returns
            .bool_functions
            .iter()
            .filter_map(|entry| {
                let ExecutionFunctionRef::Graph(entry) = entry.as_ref() else {
                    return None;
                };
                NativeLoopContract::inspect(entry.body().block_graph())
                    .map(|contract| contract.site.function().to_string())
            })
            .collect::<Vec<_>>();
        assert_eq!(names, ["bool_loop"]);
        macro_rules! selected {
            ($table:ident, $name:literal) => {
                assert_eq!(functions.value_returns.$table.len(), 2);
                let names = functions
                    .value_returns
                    .$table
                    .iter()
                    .filter_map(|entry| {
                        let ExecutionFunctionRef::Graph(entry) = entry.as_ref() else {
                            return None;
                        };
                        NativeLoopContract::inspect(entry.body().block_graph())
                            .map(|contract| contract.site.function().to_string())
                    })
                    .collect::<Vec<_>>();
                assert_eq!(names, [$name]);
            };
        }
        selected!(float_functions, "float_loop");
        selected!(string_functions, "string_loop");
        selected!(bit_array_functions, "bits_loop");
        selected!(utf_codepoint_functions, "codepoint_loop");
        selected!(nil_functions, "nil_loop");
    }

    #[test]
    fn every_scalar_target_emits_its_typed_family_and_index() {
        for (target, expected) in [
            (
                NativeLoopTarget::Int(IntFunctionId(3)),
                "data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(3))",
            ),
            (
                NativeLoopTarget::Float(FloatFunctionId(4)),
                "data::compiled::NativeLoopTarget::Float(data::function::FloatFunctionId(4))",
            ),
            (
                NativeLoopTarget::String(StringFunctionId(5)),
                "data::compiled::NativeLoopTarget::String(data::function::StringFunctionId(5))",
            ),
            (
                NativeLoopTarget::BitArray(BitArrayFunctionId(6)),
                "data::compiled::NativeLoopTarget::BitArray(data::function::BitArrayFunctionId(6))",
            ),
            (
                NativeLoopTarget::UtfCodepoint(UtfCodepointFunctionId(7)),
                "data::compiled::NativeLoopTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(7))",
            ),
            (
                NativeLoopTarget::Bool(BoolFunctionId(8)),
                "data::compiled::NativeLoopTarget::Bool(data::function::BoolFunctionId(8))",
            ),
            (
                NativeLoopTarget::Nil(NilFunctionId(9)),
                "data::compiled::NativeLoopTarget::Nil(data::function::NilFunctionId(9))",
            ),
        ] {
            assert_eq!(Rust::expression(&target), expected);
        }
    }

    #[test]
    fn every_scalar_producer_emits_its_argument_free_typed_local() {
        for (producer, expected) in [
            (
                NativeLoopProducer::Int(IntFunctionLocalId(0)),
                "data::compiled::NativeLoopProducer::Int(data::graph::IntFunctionLocalId(0))",
            ),
            (
                NativeLoopProducer::Float(FloatFunctionLocalId(1)),
                "data::compiled::NativeLoopProducer::Float(data::graph::FloatFunctionLocalId(1))",
            ),
            (
                NativeLoopProducer::String(StringFunctionLocalId(2)),
                "data::compiled::NativeLoopProducer::String(data::graph::StringFunctionLocalId(2))",
            ),
            (
                NativeLoopProducer::BitArray(BitArrayFunctionLocalId(3)),
                "data::compiled::NativeLoopProducer::BitArray(data::graph::BitArrayFunctionLocalId(3))",
            ),
            (
                NativeLoopProducer::UtfCodepoint(UtfCodepointFunctionLocalId(4)),
                "data::compiled::NativeLoopProducer::UtfCodepoint(data::graph::UtfCodepointFunctionLocalId(4))",
            ),
            (
                NativeLoopProducer::Bool(BoolFunctionLocalId(5)),
                "data::compiled::NativeLoopProducer::Bool(data::graph::BoolFunctionLocalId(5))",
            ),
            (
                NativeLoopProducer::Nil(NilFunctionLocalId(6)),
                "data::compiled::NativeLoopProducer::Nil(data::graph::NilFunctionLocalId(6))",
            ),
        ] {
            assert_eq!(Rust::expression(&producer), expected);
        }
    }

    #[test]
    fn native_loop_contract_emits_its_complete_control_and_source_identity() {
        let contract = NativeLoopContract {
            header: BlockId(0),
            finished: BlockId(1),
            repeat: BlockId(2),
            exit: BlockGraphExitId(0),
            native: NativeLoopTarget::Int(IntFunctionId(3)),
            producer: NativeLoopProducer::Int(IntFunctionLocalId(0)),
            site: HostCallSite::new("example".into(), "cycle".into(), SourceSpan::new(42, 66)),
        };
        assert_eq!(
            Rust::expression(&contract),
            r#"data::compiled::NativeLoopContract {
    header: data::graph::BlockId(0),
    finished: data::graph::BlockId(1),
    repeat: data::graph::BlockId(2),
    exit: data::graph::BlockGraphExitId(0),
    native: data::compiled::NativeLoopTarget::Int(data::function::IntFunctionId(3)),
    producer: data::compiled::NativeLoopProducer::Int(data::graph::IntFunctionLocalId(0)),
    site: data::source::HostCallSite::from_static("example", "cycle", data::source::SourceSpan::new(42, 66)),
}"#
        );
    }
}
