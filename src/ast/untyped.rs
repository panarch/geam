use super::{BinOp, CallArg, Clause, HasLocation, SrcSpan, UntypedStatement};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UntypedExpr {
    Int {
        location: SrcSpan,
        value: EcoString,
        int_value: BigInt,
    },
    Float {
        location: SrcSpan,
        value: EcoString,
    },
    String {
        location: SrcSpan,
        value: EcoString,
    },
    Block {
        location: SrcSpan,
        statements: Vec<UntypedStatement>,
    },
    Var {
        location: SrcSpan,
        name: EcoString,
    },
    List {
        location: SrcSpan,
        elements: Vec<Self>,
    },
    Call {
        location: SrcSpan,
        fun: Box<Self>,
        arguments: Vec<CallArg<Self>>,
        open_parenthesis: u32,
    },
    BinOp {
        location: SrcSpan,
        operator: BinOp,
        operator_start: u32,
        left: Box<Self>,
        right: Box<Self>,
    },
    PipeLine {
        expressions: Vec<Self>,
    },
    Case {
        location: SrcSpan,
        subjects: Vec<Self>,
        clauses: Vec<Clause<Self, ()>>,
    },
    FieldAccess {
        location: SrcSpan,
        label_location: SrcSpan,
        label: EcoString,
        container: Box<Self>,
    },
    Tuple {
        location: SrcSpan,
        elements: Vec<Self>,
    },
    TupleIndex {
        location: SrcSpan,
        index: u64,
        tuple: Box<Self>,
    },
    NegateBool {
        location: SrcSpan,
        value: Box<Self>,
    },
    NegateInt {
        location: SrcSpan,
        value: Box<Self>,
    },
}

impl HasLocation for UntypedExpr {
    fn location(&self) -> SrcSpan {
        match self {
            Self::PipeLine { expressions } => match (expressions.first(), expressions.last()) {
                (Some(first), Some(last)) => first.location().merge(&last.location()),
                _ => SrcSpan::default(),
            },
            Self::Int { location, .. }
            | Self::Float { location, .. }
            | Self::String { location, .. }
            | Self::Block { location, .. }
            | Self::Var { location, .. }
            | Self::List { location, .. }
            | Self::Call { location, .. }
            | Self::BinOp { location, .. }
            | Self::Case { location, .. }
            | Self::FieldAccess { location, .. }
            | Self::Tuple { location, .. }
            | Self::TupleIndex { location, .. }
            | Self::NegateBool { location, .. }
            | Self::NegateInt { location, .. } => *location,
        }
    }
}
