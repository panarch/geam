use super::super::constant::ConstantTailCall;
use super::super::function::FunctionEntry;
use super::super::graph::{
    Edge, FunctionGraph, MatchEdge, MatchEdgeArgument, NeverCallTarget, SourceStopKind, Terminator,
};
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionReturn, CustomListFunctionId, CustomReturn,
    ExecutionPlan, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId,
    FunctionFunctionReturn, FunctionListFunctionId, GenericFunctionFunctionId, Instruction,
    IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListFunctionFunctionId,
    ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId,
    NilFunctionId, NilListFunctionId, ParamSlot, ParameterListFunctionId,
    ParameterListListFunctionId, StringFunctionFunctionId, StringFunctionId, StringListFunctionId,
    TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId, TypedFunctionReturn,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use super::instruction::write_instruction;
use super::label::FunctionLabel;
use super::pattern::write_pattern;
use super::value::{ExplainLocal, write_locals, write_slots};

trait TailFunctionIndex {
    fn tail_function_index(&self) -> usize;
}

impl TailFunctionIndex for usize {
    fn tail_function_index(&self) -> usize {
        *self
    }
}

impl TailFunctionIndex for ConstantTailCall {
    fn tail_function_index(&self) -> usize {
        match *self {}
    }
}

macro_rules! tuple_index {
    ($($type_:ty),+ $(,)?) => {
        $(
            impl TailFunctionIndex for $type_ {
                fn tail_function_index(&self) -> usize {
                    self.0
                }
            }
        )+
    };
}

tuple_index!(
    NeverFunctionId,
    IntFunctionId,
    FloatFunctionId,
    StringFunctionId,
    BitArrayFunctionId,
    UtfCodepointFunctionId,
    BoolFunctionId,
    NilFunctionId,
    TupleFunctionId,
    IntFunctionFunctionId,
    FloatFunctionFunctionId,
    StringFunctionFunctionId,
    BitArrayFunctionFunctionId,
    UtfCodepointFunctionFunctionId,
    BoolFunctionFunctionId,
    NilFunctionFunctionId,
    TupleFunctionFunctionId,
);

macro_rules! indexed {
    ($($type_:ty),+ $(,)?) => {
        $(
            impl TailFunctionIndex for $type_ {
                fn tail_function_index(&self) -> usize {
                    self.index()
                }
            }
        )+
    };
}

indexed!(
    ParameterListFunctionId,
    IntListFunctionId,
    StringListFunctionId,
    BitArrayListFunctionId,
    UtfCodepointListFunctionId,
    CustomListFunctionId,
    FloatListFunctionId,
    BoolListFunctionId,
    NilListFunctionId,
    TupleListFunctionId,
    ParameterListListFunctionId,
    ListListFunctionId,
    FunctionListFunctionId,
    GenericFunctionFunctionId,
    NeverFunctionFunctionId,
);

impl TailFunctionIndex for ListFunctionFunctionId {
    fn tail_function_index(&self) -> usize {
        match self {
            Self::Parameter { id, .. } => id.0,
            Self::ParameterList { id, .. } => id.0,
            Self::Int { id, .. } => id.0,
            Self::String { id, .. } => id.0,
            Self::BitArray { id, .. } => id.0,
            Self::UtfCodepoint { id, .. } => id.0,
            Self::Custom { id, .. } => id.0,
            Self::Float { id, .. } => id.0,
            Self::Bool { id, .. } => id.0,
            Self::Nil { id, .. } => id.0,
            Self::Tuple { id, .. } => id.0,
            Self::List { id, .. } => id.0,
            Self::Function { id, .. } => id.0,
        }
    }
}

pub(super) trait ExplainedGraph {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot];

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot];

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    );
}

impl<Return, TailCall> ExplainedGraph for FunctionGraph<Return, TailCall>
where
    Return: ExplainLocal,
    TailCall: TailFunctionIndex,
{
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        entry.params(self)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        entry.captures(self)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        output.push_str("  entry b");
        output.push_str(&self.entry().index().to_string());
        output.push_str(" params=");
        write_slots(output, plan, entry_params);
        output.push_str(" captures=");
        write_slots(output, plan, entry_captures);
        output.push('\n');

        for (index, block) in self.blocks().iter().enumerate() {
            write_block(output, plan, index, block.params(), block.instructions());
            output.push_str("    ");
            write_terminator(output, block.terminator(), family);
            output.push('\n');
        }
    }
}

fn write_block(
    output: &mut String,
    plan: &ExecutionPlan,
    index: usize,
    params: &[ParamSlot],
    instructions: &[Instruction],
) {
    output.push_str("  block b");
    output.push_str(&index.to_string());
    output.push_str(" params=");
    write_slots(output, plan, params);
    output.push('\n');
    for instruction in instructions {
        write_instruction(output, plan, instruction);
    }
}

impl<Body> ExplainedGraph for TypedFunctionReturn<Body>
where
    Body: ExplainedGraph,
{
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for CustomReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for CustomFunctionReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

impl ExplainedGraph for FunctionFunctionReturn {
    fn entry_params<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_params(entry)
    }

    fn entry_captures<'a>(&'a self, entry: &FunctionEntry) -> &'a [ParamSlot] {
        self.body().entry_captures(entry)
    }

    fn write_complete(
        &self,
        output: &mut String,
        plan: &ExecutionPlan,
        family: &'static str,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
    ) {
        self.body()
            .write_complete(output, plan, family, entry_params, entry_captures);
    }
}

fn write_terminator<Return, TailCall>(
    output: &mut String,
    terminator: &Terminator<Return, TailCall>,
    family: &'static str,
) where
    Return: ExplainLocal,
    TailCall: TailFunctionIndex,
{
    match terminator {
        Terminator::Jump(edge) => {
            output.push_str("jump ");
            write_edge(output, edge);
        }
        Terminator::BoolBranch {
            subject,
            true_,
            false_,
        } => {
            output.push_str("branch ");
            subject.write_local(output);
            output.push_str(" true=");
            write_edge(output, true_);
            output.push_str(" false=");
            write_edge(output, false_);
        }
        Terminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.int ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&pattern.to_string());
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.float ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => {
            output.push_str("switch.string ");
            subject.write_local(output);
            output.push_str(" clauses=[");
            for (index, (pattern, edge)) in clauses.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->");
                write_edge(output, edge);
            }
            output.push_str("] fallback=");
            write_edge(output, fallback);
        }
        Terminator::Match {
            subject,
            pattern,
            success,
            failure,
        } => {
            output.push_str("match ");
            subject.write_local(output);
            output.push_str(" pattern=");
            write_pattern(output, pattern);
            output.push_str(" success=");
            write_match_edge(output, success);
            output.push_str(" failure=");
            write_edge(output, failure);
        }
        Terminator::Return(value) => {
            output.push_str("return ");
            value.write_local(output);
        }
        Terminator::TailCall { function, args } => {
            output.push_str("tail ");
            FunctionLabel::new(family, function.tail_function_index()).push_to(output);
            output.push_str(" args=");
            write_locals(output, args);
        }
        Terminator::SourceStop { kind, message, .. } => {
            output.push_str("source_stop kind=");
            output.push_str(source_stop_kind(*kind));
            output.push_str(" message=");
            match message {
                Some(message) => message.write_local(output),
                None => output.push_str("none"),
            }
        }
        Terminator::LetAssertPanic {
            subject, message, ..
        } => {
            output.push_str("let_assert_panic subject=");
            subject.write_local(output);
            output.push_str(" message=");
            match message {
                Some(message) => message.write_local(output),
                None => output.push_str("none"),
            }
        }
        Terminator::NeverCall { function, args } => {
            output.push_str("never_call ");
            match function {
                NeverCallTarget::Direct(function) => {
                    FunctionLabel::new("never", function.0).push_to(output);
                }
                NeverCallTarget::Value(function) => function.write_local(output),
            }
            output.push_str(" args=");
            write_locals(output, args);
        }
    }
}

fn write_edge(output: &mut String, edge: &Edge) {
    output.push('b');
    output.push_str(&edge.target().index().to_string());
    output.push('(');
    for (index, argument) in edge.args().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        argument.write_local(output);
    }
    output.push(')');
}

fn write_match_edge(output: &mut String, edge: &MatchEdge) {
    output.push('b');
    output.push_str(&edge.target().index().to_string());
    output.push('(');
    for (index, argument) in edge.args().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        match argument {
            MatchEdgeArgument::Binding(binding) => {
                output.push_str("binding#");
                output.push_str(&binding.to_string());
            }
            MatchEdgeArgument::Value(value) => value.write_local(output),
        }
    }
    output.push(')');
}

fn source_stop_kind(kind: SourceStopKind) -> &'static str {
    match kind {
        SourceStopKind::Panic => "panic",
        SourceStopKind::Todo => "todo",
        SourceStopKind::Assert => "assert",
        SourceStopKind::EmptyFunction => "empty_function",
        SourceStopKind::EmptyBlock => "empty_block",
        SourceStopKind::IncompleteUse => "incomplete_use",
    }
}
