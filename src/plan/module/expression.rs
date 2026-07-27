mod arg;
mod bit_array;
mod bool;
mod case;
mod custom;
mod custom_field;
mod float;
mod function;
mod generic;
mod int;
mod list;
mod nil;
mod panic;
mod string;
mod tuple;
mod utf_codepoint;

use crate::plan::{Step, ValueShape, ValueType};

pub(crate) use self::case::{
    BoolCaseBranches, FloatCaseBranches, IntCaseBranches, StringCaseBranches,
};
pub(crate) use self::function::TypedFunctionExpr;
pub use self::{
    arg::CallArg,
    bit_array::BitArrayExpr,
    bool::BoolExpr,
    custom::CustomExpr,
    float::FloatExpr,
    function::{
        BitArrayFunctionExpr, BoolFunctionExpr, CustomFunctionExpr, FloatFunctionExpr,
        FunctionExpr, FunctionFunctionExpr, IntFunctionExpr, ListFunctionExpr, NilFunctionExpr,
        StringFunctionExpr, TupleFunctionExpr, UtfCodepointFunctionExpr,
    },
    int::IntExpr,
    nil::NilExpr,
    string::StringExpr,
    tuple::TupleExpr,
    utf_codepoint::UtfCodepointExpr,
};
pub(crate) use self::{
    arg::{CallArgStorage, CaptureArg, PotentiallyUninhabitedCallArg},
    bit_array::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExprKind, BitArraySegment, Endianness,
        FloatBitSize, StringEncoding,
    },
    bool::BoolExprKind,
    custom::{
        CustomBoolCaseBranches, CustomCaseBranches, CustomConstruction, CustomExprKind,
        CustomLocalExpr, custom_constructor_expr,
    },
    custom_field::CustomFieldAccess,
    float::FloatExprKind,
    function::{
        BitArrayFunctionExprKind, BoolFunctionExprKind, CustomFunctionExprKind,
        FloatFunctionExprKind, FunctionExprKind, FunctionFunctionCallMismatch,
        FunctionFunctionExprKind, GenericFunctionExpr, GenericFunctionExprKind,
        IntFunctionExprKind, ListFunctionExprKind, NilFunctionExprKind, StringFunctionExprKind,
        TupleFunctionExprKind, TypedFunctionExprKind, UtfCodepointFunctionExprKind,
    },
    generic::{GenericExpr, GenericExprKind},
    int::IntExprKind,
    list::{
        BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
        CustomListExpr, CustomListItem, FloatListExpr, FloatListItem, FunctionListExpr,
        FunctionListItem, GenericListExpr, GenericListItem, IntListExpr, IntListItem,
        ListCaseBranches, ListElements, ListExpr, ListItem, ListListExpr, ListListItem,
        ListLocalExpr, ListSpreadConstructionError, ListSpreadElements, NilListExpr, NilListItem,
        ParameterListListExpr, ParameterListListItem, StoredListExpr, StringListExpr,
        StringListItem, TupleListExpr, TupleListItem, TypedListExpr, TypedListExprKind,
        TypedListReturnKind, UtfCodepointListExpr, UtfCodepointListItem,
    },
    nil::NilExprKind,
    panic::{PanicExpr, PanicExprKind},
    string::StringExprKind,
    tuple::TupleExprKind,
    utf_codepoint::UtfCodepointExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    shape: crate::plan::ValueShape,
    kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind {
    Generic(GenericExpr),
    Int(IntExpr),
    String(StringExpr),
    BitArray(BitArrayExpr),
    UtfCodepoint(UtfCodepointExpr),
    Custom(CustomExpr),
    Float(FloatExpr),
    Bool(BoolExpr),
    Nil(NilExpr),
    Tuple(TupleExpr),
    List(ListExpr),
    Function(FunctionExpr),
}

impl Expr {
    pub(crate) fn shape(&self) -> &crate::plan::ValueShape {
        &self.shape
    }

    pub(crate) fn with_shape(self, shape: crate::plan::ValueShape) -> Option<Self> {
        let Self {
            shape: current,
            kind,
        } = self;
        match (shape, kind) {
            (crate::plan::ValueShape::Function(shape), ExprKind::Function(expression)) => {
                let expression = expression.with_shape(*shape)?;
                Some(Self {
                    shape: crate::plan::ValueShape::Function(Box::new(expression.shape().clone())),
                    kind: ExprKind::Function(expression),
                })
            }
            (shape, kind) => {
                if shape.value_type() != current.value_type() {
                    return None;
                }
                let shape = current.refine(&shape)?;
                Self {
                    shape: current,
                    kind,
                }
                .with_resolved_shape(shape)
            }
        }
    }

    pub(crate) fn with_resolved_shape(self, shape: crate::plan::ValueShape) -> Option<Self> {
        let kind = match (shape.clone(), self.kind) {
            (crate::plan::ValueShape::Parameter(parameter), ExprKind::Generic(expression))
                if expression.parameter() == parameter =>
            {
                ExprKind::Generic(expression)
            }
            (crate::plan::ValueShape::Int, ExprKind::Int(expression)) => ExprKind::Int(expression),
            (crate::plan::ValueShape::String, ExprKind::String(expression)) => {
                ExprKind::String(expression)
            }
            (crate::plan::ValueShape::BitArray, ExprKind::BitArray(expression)) => {
                ExprKind::BitArray(expression)
            }
            (crate::plan::ValueShape::UtfCodepoint, ExprKind::UtfCodepoint(expression)) => {
                ExprKind::UtfCodepoint(expression)
            }
            (crate::plan::ValueShape::Custom(shape), ExprKind::Custom(expression)) => {
                ExprKind::Custom(expression.with_shape(shape))
            }
            (crate::plan::ValueShape::Float, ExprKind::Float(expression)) => {
                ExprKind::Float(expression)
            }
            (crate::plan::ValueShape::Bool, ExprKind::Bool(expression)) => {
                ExprKind::Bool(expression)
            }
            (crate::plan::ValueShape::Nil, ExprKind::Nil(expression)) => ExprKind::Nil(expression),
            (crate::plan::ValueShape::Tuple(shape), ExprKind::Tuple(expression)) => {
                ExprKind::Tuple(expression.with_shape(shape))
            }
            (crate::plan::ValueShape::List(item_shape), ExprKind::List(expression)) => {
                ExprKind::List(expression.with_item_shape(*item_shape))
            }
            (crate::plan::ValueShape::Function(shape), ExprKind::Function(expression)) => {
                ExprKind::Function(expression.with_resolved_shape(*shape)?)
            }
            _ => return None,
        };
        Some(Self { shape, kind })
    }

    pub(crate) fn int(expression: IntExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::Int,
            kind: ExprKind::Int(expression),
        }
    }

    pub(crate) fn generic(expression: GenericExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::Parameter(expression.parameter()),
            kind: ExprKind::Generic(expression),
        }
    }

    pub(crate) fn string(expression: StringExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::String,
            kind: ExprKind::String(expression),
        }
    }

    pub(crate) fn bit_array(expression: BitArrayExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::BitArray,
            kind: ExprKind::BitArray(expression),
        }
    }

    pub(crate) fn utf_codepoint(expression: UtfCodepointExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::UtfCodepoint,
            kind: ExprKind::UtfCodepoint(expression),
        }
    }

    pub(crate) fn custom(expression: CustomExpr) -> Self {
        let shape = crate::plan::ValueShape::Custom(expression.shape().clone());
        Self {
            shape,
            kind: ExprKind::Custom(expression),
        }
    }

    pub(crate) fn float(expression: FloatExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::Float,
            kind: ExprKind::Float(expression),
        }
    }

    pub(crate) fn bool(expression: BoolExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::Bool,
            kind: ExprKind::Bool(expression),
        }
    }

    pub(crate) fn nil(expression: NilExpr) -> Self {
        Self {
            shape: crate::plan::ValueShape::Nil,
            kind: ExprKind::Nil(expression),
        }
    }

    pub(crate) fn tuple(expression: TupleExpr) -> Self {
        let shape = crate::plan::ValueShape::Tuple(expression.shape().to_vec().into_boxed_slice());
        Self {
            shape,
            kind: ExprKind::Tuple(expression),
        }
    }

    pub(crate) fn list(expression: ListExpr) -> Self {
        let shape = crate::plan::ValueShape::List(Box::new(expression.item_shape().clone()));
        Self {
            shape,
            kind: ExprKind::List(expression),
        }
    }

    pub(crate) fn function(expression: FunctionExpr) -> Self {
        let shape = crate::plan::ValueShape::Function(Box::new(expression.shape().clone()));
        Self {
            shape,
            kind: ExprKind::Function(expression),
        }
    }

    #[cfg(test)]
    pub(crate) fn call(function: crate::plan::FunctionInstantiation, args: Vec<CallArg>) -> Self {
        Self::call_at(function, args, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn call_at(
        function: crate::plan::FunctionInstantiation,
        args: Vec<CallArg>,
        site: crate::plan::HostCallSite,
    ) -> Self {
        match function.shape().return_shape().clone() {
            ValueShape::Parameter(parameter) => {
                Self::generic(GenericExpr::call_at(parameter, function, args, site))
            }
            ValueShape::Int => Self::int(IntExpr::call_at(function, args, site)),
            ValueShape::String => Self::string(StringExpr::call_at(function, args, site)),
            ValueShape::BitArray => Self::bit_array(BitArrayExpr::call_at(function, args, site)),
            ValueShape::UtfCodepoint => {
                Self::utf_codepoint(UtfCodepointExpr::call_at(function, args, site))
            }
            ValueShape::Custom(shape) => Self::custom(CustomExpr::call(function, args, shape)),
            ValueShape::Float => Self::float(FloatExpr::call_at(function, args, site)),
            ValueShape::Bool => Self::bool(BoolExpr::call_at(function, args, site)),
            ValueShape::Nil => Self::nil(NilExpr::call_at(function, args, site)),
            ValueShape::Tuple(shape) => {
                let expression = TupleExpr::call(
                    function,
                    args,
                    shape.iter().map(ValueShape::value_type).collect(),
                );
                Self {
                    shape: ValueShape::Tuple(shape),
                    kind: ExprKind::Tuple(expression),
                }
            }
            ValueShape::List(item_shape) => {
                Self::list(ListExpr::call(function, args, (*item_shape).clone()))
            }
            ValueShape::Function(shape) => {
                Self::function(FunctionExpr::call(function, args, (*shape).clone()))
            }
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        let Self { shape, kind } = return_;
        let kind = match kind {
            ExprKind::Generic(return_) => ExprKind::Generic(GenericExpr::block(steps, return_)),
            ExprKind::Int(return_) => ExprKind::Int(IntExpr::block(steps, return_)),
            ExprKind::String(return_) => ExprKind::String(StringExpr::block(steps, return_)),
            ExprKind::BitArray(return_) => ExprKind::BitArray(BitArrayExpr::block(steps, return_)),
            ExprKind::UtfCodepoint(return_) => {
                ExprKind::UtfCodepoint(UtfCodepointExpr::block(steps, return_))
            }
            ExprKind::Custom(return_) => ExprKind::Custom(CustomExpr::block(steps, return_)),
            ExprKind::Float(return_) => ExprKind::Float(FloatExpr::block(steps, return_)),
            ExprKind::Bool(return_) => ExprKind::Bool(BoolExpr::block(steps, return_)),
            ExprKind::Nil(return_) => ExprKind::Nil(NilExpr::block(steps, return_)),
            ExprKind::Tuple(return_) => ExprKind::Tuple(TupleExpr::block(steps, return_)),
            ExprKind::List(return_) => ExprKind::List(ListExpr::block(steps, return_)),
            ExprKind::Function(return_) => ExprKind::Function(FunctionExpr::block(steps, return_)),
        };
        Self { shape, kind }
    }

    pub(crate) fn custom_field_shape(access: CustomFieldAccess, shape: ValueShape) -> Self {
        match shape {
            ValueShape::Parameter(parameter) => {
                Self::generic(GenericExpr::custom_field(parameter, access))
            }
            ValueShape::Int => Self::int(IntExpr::custom_field(access)),
            ValueShape::String => Self::string(StringExpr::custom_field(access)),
            ValueShape::BitArray => Self::bit_array(BitArrayExpr::custom_field(access)),
            ValueShape::UtfCodepoint => Self::utf_codepoint(UtfCodepointExpr::custom_field(access)),
            ValueShape::Custom(shape) => {
                Self::custom(CustomExpr::custom_field_shape(access, shape))
            }
            ValueShape::Float => Self::float(FloatExpr::custom_field(access)),
            ValueShape::Bool => Self::bool(BoolExpr::custom_field(access)),
            ValueShape::Nil => Self::nil(NilExpr::custom_field(access)),
            ValueShape::Tuple(shape) => {
                let type_ = shape.iter().map(ValueShape::value_type).collect();
                Self::tuple(TupleExpr::custom_field(access, type_).with_shape(shape))
            }
            ValueShape::List(item_shape) => {
                let item_type = item_shape.value_type();
                Self::list(ListExpr::custom_field(access, item_type).with_item_shape(*item_shape))
            }
            ValueShape::Function(shape) => {
                Self::function(FunctionExpr::custom_field_shape(access, *shape))
            }
        }
    }

    pub(crate) fn tuple_index_shape(tuple: TupleExpr, index: usize, shape: ValueShape) -> Self {
        match shape {
            ValueShape::Parameter(parameter) => {
                Self::generic(GenericExpr::tuple_index(parameter, tuple, index))
            }
            ValueShape::Int => Self::int(IntExpr::tuple_index(tuple, index)),
            ValueShape::String => Self::string(StringExpr::tuple_index(tuple, index)),
            ValueShape::BitArray => Self::bit_array(BitArrayExpr::tuple_index(tuple, index)),
            ValueShape::UtfCodepoint => {
                Self::utf_codepoint(UtfCodepointExpr::tuple_index(tuple, index))
            }
            ValueShape::Custom(shape) => {
                Self::custom(CustomExpr::tuple_index_shape(tuple, index, shape))
            }
            ValueShape::Float => Self::float(FloatExpr::tuple_index(tuple, index)),
            ValueShape::Bool => Self::bool(BoolExpr::tuple_index(tuple, index)),
            ValueShape::Nil => Self::nil(NilExpr::tuple_index(tuple, index)),
            ValueShape::Tuple(shape) => {
                let type_ = shape.iter().map(ValueShape::value_type).collect();
                Self::tuple(TupleExpr::tuple_index(tuple, index, type_).with_shape(shape))
            }
            ValueShape::List(item_shape) => {
                let item_type = item_shape.value_type();
                Self::list(
                    ListExpr::tuple_index(tuple, index, item_type).with_item_shape(*item_shape),
                )
            }
            ValueShape::Function(shape) => {
                Self::function(FunctionExpr::tuple_index_shape(tuple, index, *shape))
            }
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, branches: BoolCaseBranches) -> Self {
        match branches {
            BoolCaseBranches::Int { true_, false_ } => {
                Self::int(IntExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::String { true_, false_ } => {
                Self::string(StringExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::BitArray { true_, false_ } => {
                Self::bit_array(BitArrayExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::UtfCodepoint { true_, false_ } => {
                Self::utf_codepoint(UtfCodepointExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Custom(branches) => {
                Self::custom(CustomExpr::bool_case(subject, branches))
            }
            BoolCaseBranches::Float { true_, false_ } => {
                Self::float(FloatExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Bool { true_, false_ } => {
                Self::bool(BoolExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Nil { true_, false_ } => {
                Self::nil(NilExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::Tuple { true_, false_ } => {
                Self::tuple(TupleExpr::bool_case(subject, true_, false_))
            }
            BoolCaseBranches::List(branches) => Self::list(ListExpr::bool_case(subject, branches)),
            BoolCaseBranches::IntFunction { true_, false_ } => Self::function(FunctionExpr::int(
                IntFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::StringFunction { true_, false_ } => Self::function(
                FunctionExpr::string(StringFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::BitArrayFunction { true_, false_ } => Self::function(
                FunctionExpr::bit_array(BitArrayFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::UtfCodepointFunction { true_, false_ } => {
                Self::function(FunctionExpr::utf_codepoint(
                    UtfCodepointFunctionExpr::bool_case(subject, true_, false_),
                ))
            }
            BoolCaseBranches::CustomFunction { true_, false_ } => Self::function(
                FunctionExpr::custom(CustomFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::FloatFunction { true_, false_ } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::BoolFunction { true_, false_ } => Self::function(FunctionExpr::bool(
                BoolFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::NilFunction { true_, false_ } => Self::function(FunctionExpr::nil(
                NilFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::TupleFunction { true_, false_ } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::bool_case(subject, true_, false_)),
            ),
            BoolCaseBranches::ListFunction { true_, false_ } => Self::function(FunctionExpr::list(
                ListFunctionExpr::bool_case(subject, true_, false_),
            )),
            BoolCaseBranches::FunctionFunction { true_, false_ } => Self::function(
                FunctionExpr::function(FunctionFunctionExpr::bool_case(subject, true_, false_)),
            ),
        }
    }

    pub(crate) fn int_case(subject: IntExpr, branches: IntCaseBranches) -> Self {
        match branches {
            IntCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::BitArray { clauses, fallback } => {
                Self::bit_array(BitArrayExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::utf_codepoint(UtfCodepointExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Custom(branches) => {
                Self::custom(CustomExpr::int_case(subject, branches))
            }
            IntCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::int_case(subject, clauses, fallback))
            }
            IntCaseBranches::List(branches) => Self::list(ListExpr::int_case(subject, branches)),
            IntCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::BitArrayFunction { clauses, fallback } => Self::function(
                FunctionExpr::bit_array(BitArrayFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::UtfCodepointFunction { clauses, fallback } => {
                Self::function(FunctionExpr::utf_codepoint(
                    UtfCodepointFunctionExpr::int_case(subject, clauses, fallback),
                ))
            }
            IntCaseBranches::CustomFunction { clauses, fallback } => Self::function(
                FunctionExpr::custom(CustomFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::int_case(subject, clauses, fallback)),
            ),
            IntCaseBranches::FunctionFunction { clauses, fallback } => Self::function(
                FunctionExpr::function(FunctionFunctionExpr::int_case(subject, clauses, fallback)),
            ),
        }
    }

    pub(crate) fn string_case(subject: StringExpr, branches: StringCaseBranches) -> Self {
        match branches {
            StringCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::BitArray { clauses, fallback } => {
                Self::bit_array(BitArrayExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::utf_codepoint(UtfCodepointExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Custom(branches) => {
                Self::custom(CustomExpr::string_case(subject, branches))
            }
            StringCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::string_case(subject, clauses, fallback))
            }
            StringCaseBranches::List(branches) => {
                Self::list(ListExpr::string_case(subject, branches))
            }
            StringCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::BitArrayFunction { clauses, fallback } => {
                Self::function(FunctionExpr::bit_array(BitArrayFunctionExpr::string_case(
                    subject, clauses, fallback,
                )))
            }
            StringCaseBranches::UtfCodepointFunction { clauses, fallback } => {
                Self::function(FunctionExpr::utf_codepoint(
                    UtfCodepointFunctionExpr::string_case(subject, clauses, fallback),
                ))
            }
            StringCaseBranches::CustomFunction { clauses, fallback } => Self::function(
                FunctionExpr::custom(CustomFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::string_case(subject, clauses, fallback)),
            ),
            StringCaseBranches::FunctionFunction { clauses, fallback } => {
                Self::function(FunctionExpr::function(FunctionFunctionExpr::string_case(
                    subject, clauses, fallback,
                )))
            }
        }
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: FloatCaseBranches) -> Self {
        match branches {
            FloatCaseBranches::Int { clauses, fallback } => {
                Self::int(IntExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::String { clauses, fallback } => {
                Self::string(StringExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::BitArray { clauses, fallback } => {
                Self::bit_array(BitArrayExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::UtfCodepoint { clauses, fallback } => {
                Self::utf_codepoint(UtfCodepointExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Custom(branches) => {
                Self::custom(CustomExpr::float_case(subject, branches))
            }
            FloatCaseBranches::Float { clauses, fallback } => {
                Self::float(FloatExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Bool { clauses, fallback } => {
                Self::bool(BoolExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Nil { clauses, fallback } => {
                Self::nil(NilExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::Tuple { clauses, fallback } => {
                Self::tuple(TupleExpr::float_case(subject, clauses, fallback))
            }
            FloatCaseBranches::List(branches) => {
                Self::list(ListExpr::float_case(subject, branches))
            }
            FloatCaseBranches::IntFunction { clauses, fallback } => Self::function(
                FunctionExpr::int(IntFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::StringFunction { clauses, fallback } => Self::function(
                FunctionExpr::string(StringFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::BitArrayFunction { clauses, fallback } => {
                Self::function(FunctionExpr::bit_array(BitArrayFunctionExpr::float_case(
                    subject, clauses, fallback,
                )))
            }
            FloatCaseBranches::UtfCodepointFunction { clauses, fallback } => {
                Self::function(FunctionExpr::utf_codepoint(
                    UtfCodepointFunctionExpr::float_case(subject, clauses, fallback),
                ))
            }
            FloatCaseBranches::CustomFunction { clauses, fallback } => Self::function(
                FunctionExpr::custom(CustomFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::FloatFunction { clauses, fallback } => Self::function(
                FunctionExpr::float(FloatFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::BoolFunction { clauses, fallback } => Self::function(
                FunctionExpr::bool(BoolFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::NilFunction { clauses, fallback } => Self::function(
                FunctionExpr::nil(NilFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::TupleFunction { clauses, fallback } => Self::function(
                FunctionExpr::tuple(TupleFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::ListFunction { clauses, fallback } => Self::function(
                FunctionExpr::list(ListFunctionExpr::float_case(subject, clauses, fallback)),
            ),
            FloatCaseBranches::FunctionFunction { clauses, fallback } => {
                Self::function(FunctionExpr::function(FunctionFunctionExpr::float_case(
                    subject, clauses, fallback,
                )))
            }
        }
    }

    pub(crate) fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> ExprKind {
        self.kind
    }

    pub(crate) fn into_int(self) -> Option<IntExpr> {
        match self.kind {
            ExprKind::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringExpr> {
        match self.kind {
            ExprKind::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bit_array(self) -> Option<BitArrayExpr> {
        match self.kind {
            ExprKind::BitArray(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_utf_codepoint(self) -> Option<UtfCodepointExpr> {
        match self.kind {
            ExprKind::UtfCodepoint(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_custom(self) -> Option<CustomExpr> {
        match self.kind {
            ExprKind::Custom(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatExpr> {
        match self.kind {
            ExprKind::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolExpr> {
        match self.kind {
            ExprKind::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleExpr> {
        match self.kind {
            ExprKind::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_list(self) -> Option<ListExpr> {
        match self.kind {
            ExprKind::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionExpr> {
        match self.kind {
            ExprKind::Function(expression) => Some(expression),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn into_nil(self) -> Option<NilExpr> {
        match self.kind {
            ExprKind::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ExprKind::Generic(expression) => ValueType::Parameter(expression.parameter()),
            ExprKind::Int(_) => ValueType::Int,
            ExprKind::String(_) => ValueType::String,
            ExprKind::BitArray(_) => ValueType::BitArray,
            ExprKind::UtfCodepoint(_) => ValueType::UtfCodepoint,
            ExprKind::Custom(expression) => ValueType::Custom(expression.type_().clone()),
            ExprKind::Float(_) => ValueType::Float,
            ExprKind::Bool(_) => ValueType::Bool,
            ExprKind::Nil(_) => ValueType::Nil,
            ExprKind::Tuple(expression) => ValueType::Tuple(expression.type_().to_vec()),
            ExprKind::List(expression) => {
                ValueType::List(Box::new(expression.element_type().clone()))
            }
            ExprKind::Function(expression) => {
                ValueType::Function(Box::new(expression.type_().clone()))
            }
        }
    }

    pub(crate) fn value_shape(&self) -> &crate::plan::ValueShape {
        &self.shape
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoolCaseBranches, BoolExpr, BoolFunctionExpr, BoolListCaseBranches, CustomExpr, Expr,
        FloatCaseBranches, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr,
        IntCaseBranches, IntExpr, IntFunctionExpr, ListCaseBranches, ListExpr, ListFunctionExpr,
        NilExpr, NilFunctionExpr, StringCaseBranches, StringExpr, StringFunctionExpr, TupleExpr,
        UtfCodepointExpr,
    };
    use crate::plan::{
        BoolFunctionReference, CustomConstructorRefinement, CustomLocal, CustomType,
        CustomTypeName, CustomValueShape, FloatFunctionReference, FunctionFunctionReference,
        FunctionInstantiation, FunctionReference, FunctionShape, FunctionType,
        IntFunctionReference, ListFunctionReference, NilFunctionReference, StringFunctionReference,
        UtfCodepointLocalId, ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn expression_shape_updates_reject_incompatible_value_families() {
        let expression = Expr::int(IntExpr::value(BigInt::from(1)));

        assert_eq!(expression.clone().with_shape(ValueShape::String), None);
        assert_eq!(expression.with_resolved_shape(ValueShape::String), None);

        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Choice".into()),
            Vec::new(),
        );
        let first = CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let second = CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(1),
        );
        let expression = Expr::custom(CustomExpr::local_get(
            CustomLocal::from_shape(crate::plan::CustomLocalId(0), first),
            "choice".into(),
        ));

        assert_eq!(expression.with_shape(ValueShape::Custom(second)), None);

        let expression = Expr::function(FunctionExpr::int(int_function_expr()));
        let shape = ValueShape::Function(Box::new(crate::plan::FunctionShape::from_function_type(
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        )));

        assert_eq!(
            expression.clone().with_resolved_shape(shape.clone()),
            Some(expression),
        );

        let expression = Expr::function(FunctionExpr::int(int_function_expr()));
        assert_eq!(
            expression.with_shape(ValueShape::Function(Box::new(
                crate::plan::FunctionShape::from_function_type(FunctionType::new(
                    vec![ValueType::String],
                    ValueType::String,
                )),
            ))),
            None,
        );
    }

    #[test]
    fn expr_bool_case_shapes() {
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Int {
                    true_: IntExpr::value(BigInt::from(1)),
                    false_: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::bool_case(
                BoolExpr::value(true),
                IntExpr::value(BigInt::from(1)),
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::String {
                    true_: StringExpr::value("yes".into()),
                    false_: StringExpr::value("no".into()),
                },
            ),
            Expr::string(StringExpr::bool_case(
                BoolExpr::value(true),
                StringExpr::value("yes".into()),
                StringExpr::value("no".into()),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Float {
                    true_: FloatExpr::value(1.5),
                    false_: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::bool_case(
                BoolExpr::value(true),
                FloatExpr::value(1.5),
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Bool {
                    true_: BoolExpr::value(true),
                    false_: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::bool_case(
                BoolExpr::value(true),
                BoolExpr::value(true),
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::Nil {
                    true_: NilExpr::value(),
                    false_: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::bool_case(
                BoolExpr::value(true),
                NilExpr::value(),
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::List(BoolListCaseBranches::Int {
                    true_: list_expr()
                        .into_int()
                        .expect("test list expression should be List(Int)"),
                    false_: list_expr()
                        .into_int()
                        .expect("test list expression should be List(Int)"),
                }),
            ),
            Expr::list(ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Int {
                    true_: list_expr()
                        .into_int()
                        .expect("test list expression should be List(Int)"),
                    false_: list_expr()
                        .into_int()
                        .expect("test list expression should be List(Int)"),
                },
            )),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::IntFunction {
                    true_: int_function_expr(),
                    false_: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_expr(),
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::StringFunction {
                    true_: string_function_expr(),
                    false_: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                string_function_expr(),
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::FloatFunction {
                    true_: float_function_expr(),
                    false_: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::bool_case(
                BoolExpr::value(true),
                float_function_expr(),
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::BoolFunction {
                    true_: bool_function_expr(),
                    false_: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::bool_case(
                BoolExpr::value(true),
                bool_function_expr(),
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::ListFunction {
                    true_: list_function_expr(),
                    false_: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::bool_case(
                BoolExpr::value(true),
                list_function_expr(),
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::NilFunction {
                    true_: nil_function_expr(),
                    false_: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::bool_case(
                BoolExpr::value(true),
                nil_function_expr(),
                nil_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_int_case_shapes() {
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Int {
                    clauses: vec![(BigInt::from(1), IntExpr::value(BigInt::from(10)))],
                    fallback: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), IntExpr::value(BigInt::from(10)))],
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::String {
                    clauses: vec![(BigInt::from(1), StringExpr::value("one".into()))],
                    fallback: StringExpr::value("other".into()),
                },
            ),
            Expr::string(StringExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), StringExpr::value("one".into()))],
                StringExpr::value("other".into()),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Float {
                    clauses: vec![(BigInt::from(1), FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Bool {
                    clauses: vec![(BigInt::from(1), BoolExpr::value(true))],
                    fallback: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), BoolExpr::value(true))],
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::Nil {
                    clauses: vec![(BigInt::from(1), NilExpr::value())],
                    fallback: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), NilExpr::value())],
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::List(
                    ListCaseBranches::from_exprs(vec![(BigInt::from(1), list_expr())], list_expr())
                        .expect("list case branches"),
                ),
            ),
            Expr::list(ListExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                ListCaseBranches::from_exprs(vec![(BigInt::from(1), list_expr())], list_expr())
                    .expect("list case branches"),
            )),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::IntFunction {
                    clauses: vec![(BigInt::from(1), int_function_expr())],
                    fallback: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), int_function_expr())],
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::StringFunction {
                    clauses: vec![(BigInt::from(1), string_function_expr())],
                    fallback: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), string_function_expr())],
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::FloatFunction {
                    clauses: vec![(BigInt::from(1), float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), float_function_expr())],
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::BoolFunction {
                    clauses: vec![(BigInt::from(1), bool_function_expr())],
                    fallback: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), bool_function_expr())],
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::ListFunction {
                    clauses: vec![(BigInt::from(1), list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::int_case(
                IntExpr::value(BigInt::from(1)),
                IntCaseBranches::NilFunction {
                    clauses: vec![(BigInt::from(1), nil_function_expr())],
                    fallback: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), nil_function_expr())],
                nil_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_float_case_shapes() {
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Int {
                    clauses: vec![(1.0, IntExpr::value(BigInt::from(10)))],
                    fallback: IntExpr::value(BigInt::from(0)),
                },
            ),
            Expr::int(IntExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, IntExpr::value(BigInt::from(10)))],
                IntExpr::value(BigInt::from(0)),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::String {
                    clauses: vec![(1.0, StringExpr::value("one".into()))],
                    fallback: StringExpr::value("other".into()),
                },
            ),
            Expr::string(StringExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, StringExpr::value("one".into()))],
                StringExpr::value("other".into()),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Float {
                    clauses: vec![(1.0, FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Bool {
                    clauses: vec![(1.0, BoolExpr::value(true))],
                    fallback: BoolExpr::value(false),
                },
            ),
            Expr::bool(BoolExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, BoolExpr::value(true))],
                BoolExpr::value(false),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::Nil {
                    clauses: vec![(1.0, NilExpr::value())],
                    fallback: NilExpr::value(),
                },
            ),
            Expr::nil(NilExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, NilExpr::value())],
                NilExpr::value(),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::List(
                    ListCaseBranches::from_exprs(vec![(1.0, list_expr())], list_expr())
                        .expect("list case branches"),
                ),
            ),
            Expr::list(ListExpr::float_case(
                FloatExpr::value(1.0),
                ListCaseBranches::from_exprs(vec![(1.0, list_expr())], list_expr())
                    .expect("list case branches"),
            )),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::IntFunction {
                    clauses: vec![(1.0, int_function_expr())],
                    fallback: int_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, int_function_expr())],
                int_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::StringFunction {
                    clauses: vec![(1.0, string_function_expr())],
                    fallback: string_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, string_function_expr())],
                string_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::FloatFunction {
                    clauses: vec![(1.0, float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_function_expr())],
                float_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::BoolFunction {
                    clauses: vec![(1.0, bool_function_expr())],
                    fallback: bool_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, bool_function_expr())],
                bool_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::NilFunction {
                    clauses: vec![(1.0, nil_function_expr())],
                    fallback: nil_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, nil_function_expr())],
                nil_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::ListFunction {
                    clauses: vec![(1.0, list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::float_case(
                FloatExpr::value(1.0),
                FloatCaseBranches::FunctionFunction {
                    clauses: vec![(1.0, function_function_expr())],
                    fallback: function_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, function_function_expr())],
                function_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_string_case_shapes() {
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::Float {
                    clauses: vec![("one".into(), FloatExpr::value(1.5))],
                    fallback: FloatExpr::value(0.5),
                },
            ),
            Expr::float(FloatExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), FloatExpr::value(1.5))],
                FloatExpr::value(0.5),
            )),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::List(
                    ListCaseBranches::from_exprs(vec![("one".into(), list_expr())], list_expr())
                        .expect("list case branches"),
                ),
            ),
            Expr::list(ListExpr::string_case(
                StringExpr::value("one".into()),
                ListCaseBranches::from_exprs(vec![("one".into(), list_expr())], list_expr())
                    .expect("list case branches"),
            )),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::ListFunction {
                    clauses: vec![("one".into(), list_function_expr())],
                    fallback: list_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), list_function_expr())],
                list_function_expr(),
            ))),
        );
        assert_eq!(
            Expr::string_case(
                StringExpr::value("one".into()),
                StringCaseBranches::FloatFunction {
                    clauses: vec![("one".into(), float_function_expr())],
                    fallback: float_function_expr(),
                },
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), float_function_expr())],
                float_function_expr(),
            ))),
        );
    }

    #[test]
    fn expr_value_type() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).value_type(),
            ValueType::Int
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into())).value_type(),
            ValueType::String,
        );
        assert_eq!(
            Expr::float(FloatExpr::value(1.5)).value_type(),
            ValueType::Float
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)).value_type(),
            ValueType::Bool
        );
        assert_eq!(Expr::nil(NilExpr::value()).value_type(), ValueType::Nil);
        assert_eq!(
            Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                vec![ValueType::Int],
            ))
            .value_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                ValueType::Int,
            ))
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            Expr::function(FunctionExpr::reference(function_value())).value_type(),
            ValueType::Function(Box::new(function_type())),
        );
    }

    #[test]
    fn expr_into_typed_expression() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_int(),
            Some(IntExpr::value(BigInt::from(1))),
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into())).into_string(),
            Some(StringExpr::value("geam".into())),
        );
        let codepoint = UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into());
        assert_eq!(
            Expr::utf_codepoint(codepoint.clone()).into_utf_codepoint(),
            Some(codepoint),
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_utf_codepoint(),
            None,
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_custom(),
            None,
        );
        assert_eq!(
            Expr::float(FloatExpr::value(1.5)).into_float(),
            Some(FloatExpr::value(1.5)),
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_float(),
            None
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)).into_bool(),
            Some(BoolExpr::value(true)),
        );
        assert_eq!(
            Expr::nil(NilExpr::value()).into_nil(),
            Some(NilExpr::value())
        );
        assert_eq!(
            Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                vec![ValueType::Int],
            ))
            .into_tuple(),
            Some(TupleExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                vec![ValueType::Int],
            )),
        );
        assert_eq!(
            Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                ValueType::Int,
            ))
            .into_list(),
            Some(ListExpr::value(
                vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                ValueType::Int,
            )),
        );
        assert_eq!(Expr::int(IntExpr::value(BigInt::from(1))).into_list(), None);
        assert_eq!(Expr::nil(NilExpr::value()).into_int(), None);
        assert_eq!(Expr::int(IntExpr::value(BigInt::from(1))).into_nil(), None);
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1))).into_function(),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::reference(function_value())).into_function(),
            Some(FunctionExpr::reference(function_value())),
        );
    }

    fn function_value() -> FunctionReference {
        FunctionReference::new(instantiation(function_type()))
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(instantiation(function_type())))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(instantiation(
            FunctionType::new(vec![ValueType::String], ValueType::String),
        )))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(instantiation(
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
        )))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(instantiation(
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
        )))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(instantiation(FunctionType::new(
            vec![ValueType::Nil],
            ValueType::Nil,
        ))))
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            ValueType::Int,
        )
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::reference(
            ListFunctionReference::new(instantiation(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::Int)),
            ))),
            ValueType::Int,
        )
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(instantiation(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(function_type())),
            ))),
            function_type(),
        )
    }

    fn instantiation(type_: FunctionType) -> FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::from_function_type(type_))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }
}
