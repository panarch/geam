use super::{BinOp, CallArg, Clause, HasLocation, Import, SrcSpan, TypedStatement};
use crate::type_::Type;
use ecow::EcoString;
use num_bigint::BigInt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDefinitions {
    pub imports: Vec<Import>,
    pub custom_types: Vec<super::CustomType<Arc<Type>>>,
    pub type_aliases: Vec<super::TypeAlias<Arc<Type>>>,
    pub functions: Vec<super::TypedFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedExpr {
    Int {
        location: SrcSpan,
        type_: Arc<Type>,
        value: EcoString,
        int_value: BigInt,
    },
    Float {
        location: SrcSpan,
        type_: Arc<Type>,
        value: EcoString,
    },
    String {
        location: SrcSpan,
        type_: Arc<Type>,
        value: EcoString,
    },
    Block {
        location: SrcSpan,
        type_: Arc<Type>,
        statements: Vec<TypedStatement>,
    },
    Var {
        location: SrcSpan,
        type_: Arc<Type>,
        name: EcoString,
    },
    List {
        location: SrcSpan,
        type_: Arc<Type>,
        elements: Vec<Self>,
    },
    Call {
        location: SrcSpan,
        type_: Arc<Type>,
        fun: Box<Self>,
        arguments: Vec<CallArg<Self>>,
        open_parenthesis: u32,
    },
    BinOp {
        location: SrcSpan,
        type_: Arc<Type>,
        operator: BinOp,
        operator_start: u32,
        left: Box<Self>,
        right: Box<Self>,
    },
    Case {
        location: SrcSpan,
        type_: Arc<Type>,
        subjects: Vec<Self>,
        clauses: Vec<Clause<Self, Arc<Type>>>,
    },
    FieldAccess {
        location: SrcSpan,
        type_: Arc<Type>,
        label_location: SrcSpan,
        label: EcoString,
        container: Box<Self>,
    },
    ModuleSelect {
        location: SrcSpan,
        type_: Arc<Type>,
        module_name: EcoString,
        module_alias: EcoString,
        label: EcoString,
    },
    Tuple {
        location: SrcSpan,
        type_: Arc<Type>,
        elements: Vec<Self>,
    },
    TupleIndex {
        location: SrcSpan,
        type_: Arc<Type>,
        index: u64,
        tuple: Box<Self>,
    },
    NegateBool {
        location: SrcSpan,
        type_: Arc<Type>,
        value: Box<Self>,
    },
    NegateInt {
        location: SrcSpan,
        type_: Arc<Type>,
        value: Box<Self>,
    },
}

impl TypedExpr {
    pub fn type_(&self) -> Arc<Type> {
        match self {
            Self::Int { type_, .. }
            | Self::Float { type_, .. }
            | Self::String { type_, .. }
            | Self::Block { type_, .. }
            | Self::Var { type_, .. }
            | Self::List { type_, .. }
            | Self::Call { type_, .. }
            | Self::BinOp { type_, .. }
            | Self::Case { type_, .. }
            | Self::FieldAccess { type_, .. }
            | Self::ModuleSelect { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::TupleIndex { type_, .. }
            | Self::NegateBool { type_, .. }
            | Self::NegateInt { type_, .. } => type_.clone(),
        }
    }
}

impl HasLocation for TypedExpr {
    fn location(&self) -> SrcSpan {
        match self {
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
            | Self::ModuleSelect { location, .. }
            | Self::Tuple { location, .. }
            | Self::TupleIndex { location, .. }
            | Self::NegateBool { location, .. }
            | Self::NegateInt { location, .. } => *location,
        }
    }
}
