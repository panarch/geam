use camino::Utf8PathBuf;
use ecow::EcoString;
use num_bigint::BigInt;

pub type SpannedString = (SrcSpan, EcoString);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SrcSpan {
    pub start: u32,
    pub end: u32,
}

impl SrcSpan {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

pub trait HasLocation {
    fn location(&self) -> SrcSpan;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<Info, Definitions> {
    pub name: EcoString,
    pub path: Utf8PathBuf,
    pub documentation: Vec<EcoString>,
    pub type_info: Info,
    pub definitions: Definitions,
}

pub type SourceModule = Module<(), Vec<SourceDefinition>>;
pub type SourceDefinition = Definition<(), Expr>;
pub type SourceFunction = Function<(), Expr>;
pub type SourceStatement = Statement<(), Expr>;
pub type SourcePattern = Pattern<()>;
pub type SourceClause = Clause<Expr, ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Publicity {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition<TypeT, ExprT> {
    Function(Function<TypeT, ExprT>),
    TypeAlias(TypeAlias<TypeT>),
    CustomType(CustomType<TypeT>),
    Import(Import),
}

impl<T, E> HasLocation for Definition<T, E> {
    fn location(&self) -> SrcSpan {
        match self {
            Self::Function(function) => function.location(),
            Self::TypeAlias(alias) => alias.location,
            Self::CustomType(type_) => type_.location,
            Self::Import(import) => import.location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub location: SrcSpan,
    pub module: EcoString,
    pub alias: Option<SpannedString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias<TypeT> {
    pub location: SrcSpan,
    pub publicity: Publicity,
    pub name: SpannedString,
    pub parameters: Vec<SpannedString>,
    pub alias: TypeAst,
    pub type_: TypeT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomType<TypeT> {
    pub location: SrcSpan,
    pub publicity: Publicity,
    pub name: SpannedString,
    pub parameters: Vec<SpannedString>,
    pub constructors: Vec<RecordConstructor<TypeT>>,
    pub type_: TypeT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordConstructor<TypeT> {
    pub location: SrcSpan,
    pub name: SpannedString,
    pub arguments: Vec<RecordConstructorArg<TypeT>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordConstructorArg<TypeT> {
    pub location: SrcSpan,
    pub label: Option<SpannedString>,
    pub annotation: TypeAst,
    pub type_: TypeT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<TypeT, ExprT> {
    pub location: SrcSpan,
    pub body_start: Option<u32>,
    pub end_position: u32,
    pub name: Option<SpannedString>,
    pub arguments: Vec<Arg<TypeT>>,
    pub body: Vec<Statement<TypeT, ExprT>>,
    pub publicity: Publicity,
    pub return_annotation: Option<TypeAst>,
    pub return_type: TypeT,
}

impl<T, E> HasLocation for Function<T, E> {
    fn location(&self) -> SrcSpan {
        SrcSpan::new(self.location.start, self.end_position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg<TypeT> {
    pub location: SrcSpan,
    pub name: SpannedString,
    pub annotation: Option<TypeAst>,
    pub type_: TypeT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeAst {
    Constructor {
        location: SrcSpan,
        module: Option<(EcoString, SrcSpan)>,
        name: SpannedString,
        arguments: Vec<TypeAst>,
    },
    Fn {
        location: SrcSpan,
        arguments: Vec<TypeAst>,
        return_: Box<TypeAst>,
    },
    Var {
        location: SrcSpan,
        name: EcoString,
    },
    Tuple {
        location: SrcSpan,
        elements: Vec<TypeAst>,
    },
    Hole {
        location: SrcSpan,
        name: EcoString,
    },
}

impl HasLocation for TypeAst {
    fn location(&self) -> SrcSpan {
        match self {
            Self::Constructor { location, .. }
            | Self::Fn { location, .. }
            | Self::Var { location, .. }
            | Self::Tuple { location, .. }
            | Self::Hole { location, .. } => *location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<TypeT, ExprT> {
    Expression(ExprT),
    Assignment(Box<Assignment<TypeT, ExprT>>),
}

impl<T, E: HasLocation> HasLocation for Statement<T, E> {
    fn location(&self) -> SrcSpan {
        match self {
            Self::Expression(expression) => expression.location(),
            Self::Assignment(assignment) => assignment.location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<TypeT, ExprT> {
    pub location: SrcSpan,
    pub pattern: Pattern<TypeT>,
    pub annotation: Option<TypeAst>,
    pub value: ExprT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
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
        statements: Vec<SourceStatement>,
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
        clauses: Vec<SourceClause>,
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

impl HasLocation for Expr {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    And,
    Or,
    Eq,
    NotEq,
    LtInt,
    LtEqInt,
    LtFloat,
    LtEqFloat,
    GtEqInt,
    GtInt,
    GtEqFloat,
    GtFloat,
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MultInt,
    MultFloat,
    DivInt,
    DivFloat,
    RemainderInt,
    Concatenate,
}

impl BinOp {
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::Eq | Self::NotEq => 3,
            Self::LtInt
            | Self::LtEqInt
            | Self::LtFloat
            | Self::LtEqFloat
            | Self::GtEqInt
            | Self::GtInt
            | Self::GtEqFloat
            | Self::GtFloat => 4,
            Self::AddInt | Self::AddFloat | Self::SubInt | Self::SubFloat | Self::Concatenate => 5,
            Self::MultInt
            | Self::MultFloat
            | Self::DivInt
            | Self::DivFloat
            | Self::RemainderInt => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallArg<ValueT> {
    pub location: SrcSpan,
    pub label: Option<SpannedString>,
    pub value: ValueT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<ExprT, TypeT> {
    pub location: SrcSpan,
    pub pattern: Vec<Pattern<TypeT>>,
    pub alternative_patterns: Vec<Vec<Pattern<TypeT>>>,
    pub guard: Option<ClauseGuard<ExprT>>,
    pub then: ExprT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseGuard<ExprT> {
    pub location: SrcSpan,
    pub expression: ExprT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<TypeT> {
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
    Variable {
        location: SrcSpan,
        name: EcoString,
        type_: TypeT,
    },
    Assign {
        name: EcoString,
        location: SrcSpan,
        pattern: Box<Self>,
    },
    Discard {
        name: EcoString,
        location: SrcSpan,
        type_: TypeT,
    },
    List {
        location: SrcSpan,
        elements: Vec<Self>,
        type_: TypeT,
    },
    Constructor {
        location: SrcSpan,
        name_location: SrcSpan,
        name: EcoString,
        arguments: Vec<CallArg<Self>>,
        module: Option<(EcoString, SrcSpan)>,
        type_: TypeT,
    },
    Tuple {
        location: SrcSpan,
        elements: Vec<Self>,
    },
    StringPrefix {
        location: SrcSpan,
        left_location: SrcSpan,
        left_side_assignment: Option<SpannedString>,
        right_location: SrcSpan,
        left_side_string: EcoString,
        right_side_assignment: AssignName,
    },
}

impl<T> HasLocation for Pattern<T> {
    fn location(&self) -> SrcSpan {
        match self {
            Self::Int { location, .. }
            | Self::Float { location, .. }
            | Self::String { location, .. }
            | Self::Variable { location, .. }
            | Self::Assign { location, .. }
            | Self::Discard { location, .. }
            | Self::List { location, .. }
            | Self::Constructor { location, .. }
            | Self::Tuple { location, .. }
            | Self::StringPrefix { location, .. } => *location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignName {
    Variable(SpannedString),
    Discard(SpannedString),
}
