use super::super::graph::{FunctionGraph, Terminator};
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionReturn, CustomListFunctionId, CustomReturn,
    FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionId, ListFunctionFunctionId, ListListFunctionId, NeverFunctionFunctionId,
    NeverFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringFunctionFunctionId,
    StringFunctionId, StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId,
    TupleListFunctionId, TypedFunctionReturn, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use super::label::FunctionLabel;

trait TailFunctionIndex {
    fn tail_function_index(&self) -> usize;
}

impl TailFunctionIndex for usize {
    fn tail_function_index(&self) -> usize {
        *self
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
    fn write_topology(&self, output: &mut String, family: &'static str);
}

impl<Return, TailCall> ExplainedGraph for FunctionGraph<Return, TailCall>
where
    TailCall: TailFunctionIndex,
{
    fn write_topology(&self, output: &mut String, family: &'static str) {
        output.push_str("  graph entry=b");
        output.push_str(&self.entry().index().to_string());
        output.push('\n');

        for (index, block) in self.blocks().iter().enumerate() {
            output.push_str("  b");
            output.push_str(&index.to_string());
            output.push_str(" instructions=");
            output.push_str(&block.instructions().len().to_string());
            output.push(' ');
            write_terminator(output, block.terminator(), family);
        }
    }
}

impl<Body> ExplainedGraph for TypedFunctionReturn<Body>
where
    Body: ExplainedGraph,
{
    fn write_topology(&self, output: &mut String, family: &'static str) {
        self.body().write_topology(output, family);
    }
}

impl ExplainedGraph for CustomReturn {
    fn write_topology(&self, output: &mut String, family: &'static str) {
        self.body().write_topology(output, family);
    }
}

impl ExplainedGraph for CustomFunctionReturn {
    fn write_topology(&self, output: &mut String, family: &'static str) {
        self.body().write_topology(output, family);
    }
}

impl ExplainedGraph for FunctionFunctionReturn {
    fn write_topology(&self, output: &mut String, family: &'static str) {
        self.body().write_topology(output, family);
    }
}

fn write_terminator<Return, TailCall>(
    output: &mut String,
    terminator: &Terminator<Return, TailCall>,
    family: &'static str,
) where
    TailCall: TailFunctionIndex,
{
    match terminator {
        Terminator::Jump(edge) => {
            output.push_str("jump b");
            output.push_str(&edge.target().index().to_string());
            output.push('\n');
        }
        Terminator::BoolBranch { true_, false_, .. } => {
            output.push_str("branch bool true=b");
            output.push_str(&true_.target().index().to_string());
            output.push_str(" false=b");
            output.push_str(&false_.target().index().to_string());
            output.push('\n');
        }
        Terminator::IntSwitch {
            clauses, fallback, ..
        } => {
            output.push_str("switch int");
            for (pattern, edge) in clauses {
                output.push(' ');
                output.push_str(&pattern.to_string());
                output.push_str("->b");
                output.push_str(&edge.target().index().to_string());
            }
            write_fallback(output, fallback.target().index());
        }
        Terminator::FloatSwitch {
            clauses, fallback, ..
        } => {
            output.push_str("switch float");
            for (pattern, edge) in clauses {
                output.push(' ');
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->b");
                output.push_str(&edge.target().index().to_string());
            }
            write_fallback(output, fallback.target().index());
        }
        Terminator::StringSwitch {
            clauses, fallback, ..
        } => {
            output.push_str("switch string");
            for (pattern, edge) in clauses {
                output.push(' ');
                output.push_str(&format!("{pattern:?}"));
                output.push_str("->b");
                output.push_str(&edge.target().index().to_string());
            }
            write_fallback(output, fallback.target().index());
        }
        Terminator::Match {
            success, failure, ..
        } => {
            output.push_str("match success=b");
            output.push_str(&success.target().index().to_string());
            output.push_str(" failure=b");
            output.push_str(&failure.target().index().to_string());
            output.push('\n');
        }
        Terminator::Return(_) => output.push_str("return\n"),
        Terminator::TailCall { function, args } => {
            output.push_str("tail ");
            FunctionLabel::new(family, function.tail_function_index()).push_to(output);
            output.push_str(" args=");
            output.push_str(&args.len().to_string());
            output.push('\n');
        }
        Terminator::SourceStop { .. } => output.push_str("source_stop\n"),
        Terminator::LetAssertPanic { .. } => output.push_str("let_assert_panic\n"),
        Terminator::NeverCall { .. } => output.push_str("never_call\n"),
    }
}

fn write_fallback(output: &mut String, fallback: usize) {
    output.push_str(" fallback=b");
    output.push_str(&fallback.to_string());
    output.push('\n');
}
