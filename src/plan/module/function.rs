use super::expression::{
    BitArrayExpr, BitArrayListExpr, BoolExpr, BoolListExpr, CallArg, CustomExpr, CustomListExpr,
    FloatExpr, FloatListExpr, FunctionListExpr, IntExpr, IntListExpr, ListListExpr, NilExpr,
    NilListExpr, StringExpr, StringListExpr, TupleExpr, TupleListExpr, UtfCodepointExpr,
    UtfCodepointListExpr,
};
use super::id::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, CustomLocalId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocal, FunctionTemplateId, GenericFunctionLocal, GenericLocal,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use super::step::Step;
use super::{FrameLayout, FunctionInstantiation, FunctionTemplateSignature, TypeScheme};
use crate::plan::{
    CustomFunctionType, CustomType, FunctionFunctionType, FunctionType, ValueStorageShape,
    ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[cfg(test)]
use super::expression::ListExpr;
#[cfg(test)]
use super::id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    FloatFunctionFunctionId, FloatFunctionId, IntFunctionFunctionId, IntFunctionId,
    ListFunctionFunctionId, NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId,
    StringFunctionId, TupleFunctionFunctionId, TupleFunctionId, UtfCodepointFunctionFunctionId,
};
#[cfg(test)]
use crate::plan::{ValueRepresentation, ValueShape};

#[derive(Debug, PartialEq)]
pub struct FunctionTemplate {
    signature: FunctionTemplateSignature,
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: ReturnExpr,
    frame_layout: FrameLayout,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    local: ParamLocal,
    binding: ParamBinding,
    shape: crate::plan::ValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParamSlot {
    local: ParamLocal,
    shape: crate::plan::ValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamBinding {
    Named(EcoString),
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Generic(GenericLocal),
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Custom(CustomLocal),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple {
        local: TupleLocalId,
        type_: Vec<ValueType>,
    },
    List(ListLocal),
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    StringFunction {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    CustomFunction(CustomFunctionLocal),
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    NilFunction {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    ListFunction(ListFunctionLocal),
    FunctionFunction(FunctionFunctionLocal),
    GenericFunction(GenericFunctionLocal),
}

pub(crate) type GenericReturn = ReturnBody<super::GenericExpr, FunctionInstantiation>;
pub(crate) type IntReturn = ReturnBody<IntExpr, FunctionInstantiation>;
pub(crate) type FloatReturn = ReturnBody<FloatExpr, FunctionInstantiation>;
pub(crate) type StringReturn = ReturnBody<StringExpr, FunctionInstantiation>;
pub(crate) type BitArrayReturn = ReturnBody<BitArrayExpr, FunctionInstantiation>;
pub(crate) type UtfCodepointReturn = ReturnBody<UtfCodepointExpr, FunctionInstantiation>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomReturn {
    signature_shape: crate::plan::CustomValueShape,
    body_shape: crate::plan::CustomValueShape,
    body: ReturnBody<super::CustomExprKind, FunctionInstantiation>,
}
pub(crate) type BoolReturn = ReturnBody<BoolExpr, FunctionInstantiation>;
pub(crate) type NilReturn = ReturnBody<NilExpr, FunctionInstantiation>;
pub(crate) type TupleReturn = ReturnBody<TupleExpr, FunctionInstantiation>;
pub(crate) type GenericListReturn = ReturnBody<super::GenericListExpr, FunctionInstantiation>;
pub(crate) type ParameterListListReturn =
    ReturnBody<super::ParameterListListExpr, FunctionInstantiation>;
pub(crate) type IntListReturn = ReturnBody<IntListExpr, FunctionInstantiation>;
pub(crate) type FloatListReturn = ReturnBody<FloatListExpr, FunctionInstantiation>;
pub(crate) type StringListReturn = ReturnBody<StringListExpr, FunctionInstantiation>;
pub(crate) type BitArrayListReturn = ReturnBody<BitArrayListExpr, FunctionInstantiation>;
pub(crate) type UtfCodepointListReturn = ReturnBody<UtfCodepointListExpr, FunctionInstantiation>;
pub(crate) type CustomListReturn = ReturnBody<CustomListExpr, FunctionInstantiation>;
pub(crate) type BoolListReturn = ReturnBody<BoolListExpr, FunctionInstantiation>;
pub(crate) type NilListReturn = ReturnBody<NilListExpr, FunctionInstantiation>;
pub(crate) type TupleListReturn = ReturnBody<TupleListExpr, FunctionInstantiation>;
pub(crate) type ListListReturn = ReturnBody<ListListExpr, FunctionInstantiation>;
pub(crate) type FunctionListReturn = ReturnBody<FunctionListExpr, FunctionInstantiation>;
pub(crate) type GenericFunctionReturn =
    ReturnBody<super::GenericFunctionExpr, FunctionInstantiation>;
pub(crate) type IntFunctionReturn = ReturnBody<super::IntFunctionExpr, FunctionInstantiation>;
pub(crate) type FloatFunctionReturn = ReturnBody<super::FloatFunctionExpr, FunctionInstantiation>;
pub(crate) type StringFunctionReturn = ReturnBody<super::StringFunctionExpr, FunctionInstantiation>;
pub(crate) type BitArrayFunctionReturn =
    ReturnBody<super::BitArrayFunctionExpr, FunctionInstantiation>;
pub(crate) type UtfCodepointFunctionReturn =
    ReturnBody<super::UtfCodepointFunctionExpr, FunctionInstantiation>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFunctionReturn {
    type_: CustomFunctionType,
    body: ReturnBody<super::CustomFunctionExprKind, FunctionInstantiation>,
}
pub(crate) type BoolFunctionReturn = ReturnBody<super::BoolFunctionExpr, FunctionInstantiation>;
pub(crate) type NilFunctionReturn = ReturnBody<super::NilFunctionExpr, FunctionInstantiation>;
pub(crate) type TupleFunctionReturn = ReturnBody<super::TupleFunctionExpr, FunctionInstantiation>;
pub(crate) type ListFunctionReturn = ReturnBody<super::ListFunctionExpr, FunctionInstantiation>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionFunctionReturn {
    type_: FunctionFunctionType,
    body: ReturnBody<super::FunctionFunctionExprKind, FunctionInstantiation>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListReturn {
    Generic {
        item_parameter: crate::plan::TypeParameterId,
        body: GenericListReturn,
    },
    Int(IntListReturn),
    Float(FloatListReturn),
    String(StringListReturn),
    BitArray(BitArrayListReturn),
    UtfCodepoint(UtfCodepointListReturn),
    Custom {
        item_type: CustomType,
        body: CustomListReturn,
    },
    Bool(BoolListReturn),
    Nil(NilListReturn),
    Tuple {
        item_type: Vec<ValueType>,
        body: TupleListReturn,
    },
    ParameterList {
        item_parameter: crate::plan::TypeParameterId,
        body: ParameterListListReturn,
    },
    List {
        item_shape: ValueStorageShape,
        body: ListListReturn,
    },
    Function {
        item_type: FunctionType,
        body: FunctionListReturn,
    },
}

#[cfg(test)]
impl ListReturn {
    pub(crate) fn expr(expression: ListExpr) -> Self {
        match expression {
            ListExpr::Generic(expression) => Self::Generic {
                item_parameter: expression.item().parameter(),
                body: GenericListReturn::expr(expression),
            },
            ListExpr::Int(expression) => Self::Int(IntListReturn::expr(expression)),
            ListExpr::Float(expression) => Self::Float(FloatListReturn::expr(expression)),
            ListExpr::String(expression) => Self::String(StringListReturn::expr(expression)),
            ListExpr::BitArray(expression) => Self::BitArray(BitArrayListReturn::expr(expression)),
            ListExpr::UtfCodepoint(expression) => {
                Self::UtfCodepoint(UtfCodepointListReturn::expr(expression))
            }
            ListExpr::Custom(expression) => Self::Custom {
                item_type: expression.item().item_type(),
                body: CustomListReturn::expr(expression),
            },
            ListExpr::Bool(expression) => Self::Bool(BoolListReturn::expr(expression)),
            ListExpr::Nil(expression) => Self::Nil(NilListReturn::expr(expression)),
            ListExpr::Tuple(expression) => Self::Tuple {
                item_type: expression.item().item_type(),
                body: TupleListReturn::expr(expression),
            },
            ListExpr::ParameterList(expression) => Self::ParameterList {
                item_parameter: expression.item().parameter(),
                body: ParameterListListReturn::expr(expression),
            },
            ListExpr::List(expression) => Self::List {
                item_shape: expression.item().item_shape().clone(),
                body: ListListReturn::expr(expression),
            },
            ListExpr::Function(expression) => Self::Function {
                item_type: expression.item().item_type(),
                body: FunctionListReturn::expr(expression),
            },
        }
    }

    pub(crate) fn tail_call(
        function: FunctionInstantiation,
        item_type: ValueType,
        args: Vec<CallArg>,
    ) -> Self {
        match item_type {
            ValueType::Parameter(item_parameter) => Self::Generic {
                item_parameter,
                body: GenericListReturn::tail_call(function, args),
            },
            ValueType::Int => Self::Int(IntListReturn::tail_call(function, args)),
            ValueType::Float => Self::Float(FloatListReturn::tail_call(function, args)),
            ValueType::String => Self::String(StringListReturn::tail_call(function, args)),
            ValueType::BitArray => Self::BitArray(BitArrayListReturn::tail_call(function, args)),
            ValueType::UtfCodepoint => {
                Self::UtfCodepoint(UtfCodepointListReturn::tail_call(function, args))
            }
            ValueType::Custom(item_type) => Self::Custom {
                item_type,
                body: CustomListReturn::tail_call(function, args),
            },
            ValueType::Bool => Self::Bool(BoolListReturn::tail_call(function, args)),
            ValueType::Nil => Self::Nil(NilListReturn::tail_call(function, args)),
            ValueType::Tuple(item_type) => Self::Tuple {
                item_type,
                body: TupleListReturn::tail_call(function, args),
            },
            ValueType::List(item_type) => {
                match ValueShape::from_value_type(*item_type).representation() {
                    ValueRepresentation::Uninhabited(item_parameter) => Self::ParameterList {
                        item_parameter,
                        body: ParameterListListReturn::tail_call(function, args),
                    },
                    ValueRepresentation::Stored(item_shape) => Self::List {
                        item_shape,
                        body: ListListReturn::tail_call(function, args),
                    },
                }
            }
            ValueType::Function(item_type) => Self::Function {
                item_type: *item_type,
                body: FunctionListReturn::tail_call(function, args),
            },
        }
    }

    pub(crate) fn try_bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Option<Self> {
        Some(match (true_, false_) {
            (
                Self::Generic {
                    item_parameter: true_parameter,
                    body: true_,
                },
                Self::Generic {
                    item_parameter: false_parameter,
                    body: false_,
                },
            ) if true_parameter == false_parameter => Self::Generic {
                item_parameter: true_parameter,
                body: GenericListReturn::bool_case(subject, true_, false_),
            },
            (Self::Int(true_), Self::Int(false_)) => {
                Self::Int(IntListReturn::bool_case(subject, true_, false_))
            }
            (Self::Float(true_), Self::Float(false_)) => {
                Self::Float(FloatListReturn::bool_case(subject, true_, false_))
            }
            (Self::String(true_), Self::String(false_)) => {
                Self::String(StringListReturn::bool_case(subject, true_, false_))
            }
            (Self::BitArray(true_), Self::BitArray(false_)) => {
                Self::BitArray(BitArrayListReturn::bool_case(subject, true_, false_))
            }
            (Self::UtfCodepoint(true_), Self::UtfCodepoint(false_)) => {
                Self::UtfCodepoint(UtfCodepointListReturn::bool_case(subject, true_, false_))
            }
            (
                Self::Custom {
                    item_type: true_type,
                    body: true_,
                },
                Self::Custom {
                    item_type: false_type,
                    body: false_,
                },
            ) if true_type == false_type => Self::Custom {
                item_type: true_type,
                body: CustomListReturn::bool_case(subject, true_, false_),
            },
            (Self::Bool(true_), Self::Bool(false_)) => {
                Self::Bool(BoolListReturn::bool_case(subject, true_, false_))
            }
            (Self::Nil(true_), Self::Nil(false_)) => {
                Self::Nil(NilListReturn::bool_case(subject, true_, false_))
            }
            (
                Self::Tuple {
                    item_type: true_type,
                    body: true_,
                },
                Self::Tuple {
                    item_type: false_type,
                    body: false_,
                },
            ) if true_type == false_type => Self::Tuple {
                item_type: true_type,
                body: TupleListReturn::bool_case(subject, true_, false_),
            },
            (
                Self::ParameterList {
                    item_parameter: true_parameter,
                    body: true_,
                },
                Self::ParameterList {
                    item_parameter: false_parameter,
                    body: false_,
                },
            ) if true_parameter == false_parameter => Self::ParameterList {
                item_parameter: true_parameter,
                body: ParameterListListReturn::bool_case(subject, true_, false_),
            },
            (
                Self::List {
                    item_shape: true_shape,
                    body: true_,
                },
                Self::List {
                    item_shape: false_shape,
                    body: false_,
                },
            ) if true_shape == false_shape => Self::List {
                item_shape: true_shape,
                body: ListListReturn::bool_case(subject, true_, false_),
            },
            (
                Self::Function {
                    item_type: true_type,
                    body: true_,
                },
                Self::Function {
                    item_type: false_type,
                    body: false_,
                },
            ) if true_type == false_type => Self::Function {
                item_type: true_type,
                body: FunctionListReturn::bool_case(subject, true_, false_),
            },
            _ => return None,
        })
    }

    pub(crate) fn try_int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
            Self::Generic {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Generic {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::Generic {
                    item_parameter,
                    body: GenericListReturn::int_case(subject, clauses, fallback),
                })
            }
            Self::Int(fallback) => Some(Self::Int(IntListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Int(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Float(fallback) => Some(Self::Float(FloatListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Float(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::String(fallback) => Some(Self::String(StringListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::String(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::BitArray(fallback) => Some(Self::BitArray(BitArrayListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::BitArray(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::UtfCodepoint(fallback) => {
                Some(Self::UtfCodepoint(UtfCodepointListReturn::int_case(
                    subject,
                    into_list_return_clauses(clauses, |branch| match branch {
                        Self::UtfCodepoint(branch) => Some(branch),
                        _ => None,
                    })?,
                    fallback,
                )))
            }
            Self::Custom {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Custom {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Custom {
                    item_type,
                    body: CustomListReturn::int_case(subject, clauses, fallback),
                })
            }
            Self::Bool(fallback) => Some(Self::Bool(BoolListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Bool(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Nil(fallback) => Some(Self::Nil(NilListReturn::int_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Nil(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Tuple {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Tuple {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Tuple {
                    item_type,
                    body: TupleListReturn::int_case(subject, clauses, fallback),
                })
            }
            Self::ParameterList {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::ParameterList {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::ParameterList {
                    item_parameter,
                    body: ParameterListListReturn::int_case(subject, clauses, fallback),
                })
            }
            Self::List {
                item_shape,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_shape: branch_shape,
                        body,
                    } if branch_shape == item_shape => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_shape,
                    body: ListListReturn::int_case(subject, clauses, fallback),
                })
            }
            Self::Function {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Function {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Function {
                    item_type,
                    body: FunctionListReturn::int_case(subject, clauses, fallback),
                })
            }
        }
    }

    pub(crate) fn try_float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
            Self::Generic {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Generic {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::Generic {
                    item_parameter,
                    body: GenericListReturn::float_case(subject, clauses, fallback),
                })
            }
            Self::Int(fallback) => Some(Self::Int(IntListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Int(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Float(fallback) => Some(Self::Float(FloatListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Float(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::String(fallback) => Some(Self::String(StringListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::String(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::BitArray(fallback) => Some(Self::BitArray(BitArrayListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::BitArray(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::UtfCodepoint(fallback) => {
                Some(Self::UtfCodepoint(UtfCodepointListReturn::float_case(
                    subject,
                    into_list_return_clauses(clauses, |branch| match branch {
                        Self::UtfCodepoint(branch) => Some(branch),
                        _ => None,
                    })?,
                    fallback,
                )))
            }
            Self::Custom {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Custom {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Custom {
                    item_type,
                    body: CustomListReturn::float_case(subject, clauses, fallback),
                })
            }
            Self::Bool(fallback) => Some(Self::Bool(BoolListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Bool(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Nil(fallback) => Some(Self::Nil(NilListReturn::float_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Nil(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Tuple {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Tuple {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Tuple {
                    item_type,
                    body: TupleListReturn::float_case(subject, clauses, fallback),
                })
            }
            Self::ParameterList {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::ParameterList {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::ParameterList {
                    item_parameter,
                    body: ParameterListListReturn::float_case(subject, clauses, fallback),
                })
            }
            Self::List {
                item_shape,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_shape: branch_shape,
                        body,
                    } if branch_shape == item_shape => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_shape,
                    body: ListListReturn::float_case(subject, clauses, fallback),
                })
            }
            Self::Function {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Function {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Function {
                    item_type,
                    body: FunctionListReturn::float_case(subject, clauses, fallback),
                })
            }
        }
    }

    pub(crate) fn try_string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
            Self::Generic {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Generic {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::Generic {
                    item_parameter,
                    body: GenericListReturn::string_case(subject, clauses, fallback),
                })
            }
            Self::Int(fallback) => Some(Self::Int(IntListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Int(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Float(fallback) => Some(Self::Float(FloatListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Float(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::String(fallback) => Some(Self::String(StringListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::String(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::BitArray(fallback) => Some(Self::BitArray(BitArrayListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::BitArray(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::UtfCodepoint(fallback) => {
                Some(Self::UtfCodepoint(UtfCodepointListReturn::string_case(
                    subject,
                    into_list_return_clauses(clauses, |branch| match branch {
                        Self::UtfCodepoint(branch) => Some(branch),
                        _ => None,
                    })?,
                    fallback,
                )))
            }
            Self::Custom {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Custom {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Custom {
                    item_type,
                    body: CustomListReturn::string_case(subject, clauses, fallback),
                })
            }
            Self::Bool(fallback) => Some(Self::Bool(BoolListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Bool(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Nil(fallback) => Some(Self::Nil(NilListReturn::string_case(
                subject,
                into_list_return_clauses(clauses, |branch| match branch {
                    Self::Nil(branch) => Some(branch),
                    _ => None,
                })?,
                fallback,
            ))),
            Self::Tuple {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Tuple {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Tuple {
                    item_type,
                    body: TupleListReturn::string_case(subject, clauses, fallback),
                })
            }
            Self::ParameterList {
                item_parameter,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::ParameterList {
                        item_parameter: branch_parameter,
                        body,
                    } if branch_parameter == item_parameter => Some(body),
                    _ => None,
                })?;
                Some(Self::ParameterList {
                    item_parameter,
                    body: ParameterListListReturn::string_case(subject, clauses, fallback),
                })
            }
            Self::List {
                item_shape,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_shape: branch_shape,
                        body,
                    } if branch_shape == item_shape => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_shape,
                    body: ListListReturn::string_case(subject, clauses, fallback),
                })
            }
            Self::Function {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::Function {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::Function {
                    item_type,
                    body: FunctionListReturn::string_case(subject, clauses, fallback),
                })
            }
        }
    }

    pub(crate) fn try_block(steps: Vec<Step>, return_: Self) -> Self {
        match return_ {
            Self::Generic {
                item_parameter,
                body,
            } => Self::Generic {
                item_parameter,
                body: GenericListReturn::block(steps, body),
            },
            Self::Int(return_) => Self::Int(IntListReturn::block(steps, return_)),
            Self::Float(return_) => Self::Float(FloatListReturn::block(steps, return_)),
            Self::String(return_) => Self::String(StringListReturn::block(steps, return_)),
            Self::BitArray(return_) => Self::BitArray(BitArrayListReturn::block(steps, return_)),
            Self::UtfCodepoint(return_) => {
                Self::UtfCodepoint(UtfCodepointListReturn::block(steps, return_))
            }
            Self::Custom { item_type, body } => Self::Custom {
                item_type,
                body: CustomListReturn::block(steps, body),
            },
            Self::Bool(return_) => Self::Bool(BoolListReturn::block(steps, return_)),
            Self::Nil(return_) => Self::Nil(NilListReturn::block(steps, return_)),
            Self::Tuple { item_type, body } => Self::Tuple {
                item_type,
                body: TupleListReturn::block(steps, body),
            },
            Self::ParameterList {
                item_parameter,
                body,
            } => Self::ParameterList {
                item_parameter,
                body: ParameterListListReturn::block(steps, body),
            },
            Self::List { item_shape, body } => Self::List {
                item_shape,
                body: ListListReturn::block(steps, body),
            },
            Self::Function { item_type, body } => Self::Function {
                item_type,
                body: FunctionListReturn::block(steps, body),
            },
        }
    }
}

#[cfg(test)]
fn into_list_return_clauses<Pattern, Body>(
    clauses: Vec<(Pattern, ListReturn)>,
    mut into_body: impl FnMut(ListReturn) -> Option<Body>,
) -> Option<Vec<(Pattern, Body)>> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| into_body(branch).map(|branch| (pattern, branch)))
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReturnBody<Expression, Function> {
    kind: ReturnBodyKind<Expression, Function>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ReturnBodyKind<Expression, Function> {
    Expr(Expression),
    TailCall {
        function: Function,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: BoolExpr,
        true_: Box<ReturnBody<Expression, Function>>,
        false_: Box<ReturnBody<Expression, Function>>,
    },
    IntCase {
        subject: IntExpr,
        clauses: Vec<(BigInt, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    FloatCase {
        subject: FloatExpr,
        clauses: Vec<(f64, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    StringCase {
        subject: StringExpr,
        clauses: Vec<(EcoString, ReturnBody<Expression, Function>)>,
        fallback: Box<ReturnBody<Expression, Function>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ReturnBody<Expression, Function>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpr {
    kind: ReturnExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ReturnExprKind {
    Generic {
        parameter: crate::plan::TypeParameterId,
        body: GenericReturn,
    },
    Int {
        body: IntReturn,
    },
    Float {
        body: FloatReturn,
    },
    String {
        body: StringReturn,
    },
    BitArray {
        body: BitArrayReturn,
    },
    UtfCodepoint {
        body: UtfCodepointReturn,
    },
    Custom {
        body: CustomReturn,
    },
    Bool {
        body: BoolReturn,
    },
    Nil {
        body: NilReturn,
    },
    Tuple {
        type_: Vec<ValueType>,
        body: TupleReturn,
    },
    GenericList {
        parameter: crate::plan::TypeParameterId,
        body: GenericListReturn,
    },
    ParameterListList {
        parameter: crate::plan::TypeParameterId,
        body: ParameterListListReturn,
    },
    IntList {
        body: IntListReturn,
    },
    StringList {
        body: StringListReturn,
    },
    BitArrayList {
        body: BitArrayListReturn,
    },
    UtfCodepointList {
        body: UtfCodepointListReturn,
    },
    CustomList {
        item_type: CustomType,
        body: CustomListReturn,
    },
    FloatList {
        body: FloatListReturn,
    },
    BoolList {
        body: BoolListReturn,
    },
    NilList {
        body: NilListReturn,
    },
    TupleList {
        item_type: Vec<ValueType>,
        body: TupleListReturn,
    },
    ListList {
        item_shape: ValueStorageShape,
        body: ListListReturn,
    },
    FunctionList {
        item_type: FunctionType,
        body: FunctionListReturn,
    },
    GenericFunction {
        shape: crate::plan::FunctionShape,
        body: GenericFunctionReturn,
    },
    IntFunction {
        shape: crate::plan::FunctionShape,
        body: IntFunctionReturn,
    },
    FloatFunction {
        shape: crate::plan::FunctionShape,
        body: FloatFunctionReturn,
    },
    StringFunction {
        shape: crate::plan::FunctionShape,
        body: StringFunctionReturn,
    },
    BitArrayFunction {
        shape: crate::plan::FunctionShape,
        body: BitArrayFunctionReturn,
    },
    UtfCodepointFunction {
        shape: crate::plan::FunctionShape,
        body: UtfCodepointFunctionReturn,
    },
    CustomFunction {
        shape: crate::plan::FunctionShape,
        body: CustomFunctionReturn,
    },
    BoolFunction {
        shape: crate::plan::FunctionShape,
        body: BoolFunctionReturn,
    },
    NilFunction {
        shape: crate::plan::FunctionShape,
        body: NilFunctionReturn,
    },
    TupleFunction {
        shape: crate::plan::FunctionShape,
        body: TupleFunctionReturn,
    },
    ListFunction {
        shape: crate::plan::FunctionShape,
        item_type: ValueType,
        body: ListFunctionReturn,
    },
    FunctionFunction {
        shape: crate::plan::FunctionShape,
        body: FunctionFunctionReturn,
    },
}

impl FunctionTemplate {
    #[cfg(test)]
    pub(crate) fn new(
        id: FunctionTemplateId,
        name: EcoString,
        params: Vec<Param>,
        steps: Vec<Step>,
        return_: ReturnExpr,
    ) -> Self {
        let signature = FunctionTemplateSignature::new(
            id,
            TypeScheme::new(0),
            crate::plan::FunctionShape::new(
                params.iter().map(|param| param.shape().clone()).collect(),
                crate::plan::ValueShape::from_value_type(return_.value_type()),
            ),
        );
        Self::from_signature(signature, name, params, steps, return_)
    }

    pub(crate) fn from_signature(
        signature: FunctionTemplateSignature,
        name: EcoString,
        params: Vec<Param>,
        steps: Vec<Step>,
        return_: ReturnExpr,
    ) -> Self {
        let frame_layout = FrameLayout::from_function_parts(&params, &steps, &return_);

        Self {
            signature,
            name,
            params,
            steps,
            return_,
            frame_layout,
        }
    }

    pub fn id(&self) -> FunctionTemplateId {
        self.signature.id()
    }

    pub fn scheme(&self) -> &TypeScheme {
        self.signature.scheme()
    }

    pub(crate) fn signature(&self) -> &FunctionTemplateSignature {
        &self.signature
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn return_(&self) -> &ReturnExpr {
        &self.return_
    }

    #[cfg(test)]
    pub(crate) fn frame_layout(&self) -> FrameLayout {
        self.frame_layout.clone()
    }

    pub(crate) fn execution_frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl ReturnExpr {
    pub(crate) fn generic_body(
        parameter: crate::plan::TypeParameterId,
        body: GenericReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::Generic { parameter, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn int(_runtime_id: IntFunctionId, expression: IntExpr) -> Self {
        Self::int_body(ReturnBody::expr(expression))
    }

    pub(crate) fn int_body(body: IntReturn) -> Self {
        Self {
            kind: ReturnExprKind::Int { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn float(_runtime_id: FloatFunctionId, expression: FloatExpr) -> Self {
        Self::float_body(ReturnBody::expr(expression))
    }

    pub(crate) fn float_body(body: FloatReturn) -> Self {
        Self {
            kind: ReturnExprKind::Float { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn string(_runtime_id: StringFunctionId, expression: StringExpr) -> Self {
        Self::string_body(ReturnBody::expr(expression))
    }

    pub(crate) fn string_body(body: StringReturn) -> Self {
        Self {
            kind: ReturnExprKind::String { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bit_array(_runtime_id: BitArrayFunctionId, expression: BitArrayExpr) -> Self {
        Self::bit_array_body(ReturnBody::expr(expression))
    }

    pub(crate) fn bit_array_body(body: BitArrayReturn) -> Self {
        Self {
            kind: ReturnExprKind::BitArray { body },
        }
    }

    pub(crate) fn utf_codepoint_body(body: UtfCodepointReturn) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepoint { body },
        }
    }

    pub(crate) fn custom_body(body: CustomReturn) -> Self {
        Self {
            kind: ReturnExprKind::Custom { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool(_runtime_id: BoolFunctionId, expression: BoolExpr) -> Self {
        Self::bool_body(ReturnBody::expr(expression))
    }

    pub(crate) fn bool_body(body: BoolReturn) -> Self {
        Self {
            kind: ReturnExprKind::Bool { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil(_runtime_id: NilFunctionId, expression: NilExpr) -> Self {
        Self::nil_body(ReturnBody::expr(expression))
    }

    pub(crate) fn nil_body(body: NilReturn) -> Self {
        Self {
            kind: ReturnExprKind::Nil { body },
        }
    }

    #[cfg(test)]
    pub(crate) fn tuple(_runtime_id: TupleFunctionId, expression: TupleExpr) -> Self {
        Self::tuple_body(expression.type_().to_vec(), ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_body(type_: Vec<ValueType>, body: TupleReturn) -> Self {
        Self {
            kind: ReturnExprKind::Tuple { type_, body },
        }
    }

    pub(crate) fn int_list_body(body: IntListReturn) -> Self {
        Self {
            kind: ReturnExprKind::IntList { body },
        }
    }

    pub(crate) fn generic_list_body(
        parameter: crate::plan::TypeParameterId,
        body: GenericListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::GenericList { parameter, body },
        }
    }

    pub(crate) fn parameter_list_list_body(
        parameter: crate::plan::TypeParameterId,
        body: ParameterListListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::ParameterListList { parameter, body },
        }
    }

    pub(crate) fn string_list_body(body: StringListReturn) -> Self {
        Self {
            kind: ReturnExprKind::StringList { body },
        }
    }

    pub(crate) fn bit_array_list_body(body: BitArrayListReturn) -> Self {
        Self {
            kind: ReturnExprKind::BitArrayList { body },
        }
    }

    pub(crate) fn utf_codepoint_list_body(body: UtfCodepointListReturn) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepointList { body },
        }
    }

    pub(crate) fn custom_list_body(item_type: CustomType, body: CustomListReturn) -> Self {
        Self {
            kind: ReturnExprKind::CustomList { item_type, body },
        }
    }

    pub(crate) fn float_list_body(body: FloatListReturn) -> Self {
        Self {
            kind: ReturnExprKind::FloatList { body },
        }
    }

    pub(crate) fn bool_list_body(body: BoolListReturn) -> Self {
        Self {
            kind: ReturnExprKind::BoolList { body },
        }
    }

    pub(crate) fn nil_list_body(body: NilListReturn) -> Self {
        Self {
            kind: ReturnExprKind::NilList { body },
        }
    }

    pub(crate) fn tuple_list_body(item_type: Vec<ValueType>, body: TupleListReturn) -> Self {
        Self {
            kind: ReturnExprKind::TupleList { item_type, body },
        }
    }

    pub(crate) fn list_list_body(item_shape: ValueStorageShape, body: ListListReturn) -> Self {
        Self {
            kind: ReturnExprKind::ListList { item_shape, body },
        }
    }

    pub(crate) fn function_list_body(item_type: FunctionType, body: FunctionListReturn) -> Self {
        Self {
            kind: ReturnExprKind::FunctionList { item_type, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function(
        _runtime_id: IntFunctionFunctionId,
        expression: super::IntFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::int_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn int_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: IntFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::IntFunction { shape, body },
        }
    }

    pub(crate) fn generic_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: GenericFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::GenericFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function_body(
        _runtime_id: IntFunctionFunctionId,
        type_: FunctionType,
        body: IntFunctionReturn,
    ) -> Self {
        Self::int_function_shape_body(crate::plan::FunctionShape::from_function_type(type_), body)
    }

    #[cfg(test)]
    pub(crate) fn float_function(
        _runtime_id: FloatFunctionFunctionId,
        expression: super::FloatFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::float_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn float_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: FloatFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FloatFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn float_function_body(
        _runtime_id: FloatFunctionFunctionId,
        type_: FunctionType,
        body: FloatFunctionReturn,
    ) -> Self {
        Self::float_function_shape_body(crate::plan::FunctionShape::from_function_type(type_), body)
    }

    #[cfg(test)]
    pub(crate) fn string_function(
        _runtime_id: StringFunctionFunctionId,
        expression: super::StringFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::string_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn string_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: StringFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::StringFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn string_function_body(
        _runtime_id: StringFunctionFunctionId,
        type_: FunctionType,
        body: StringFunctionReturn,
    ) -> Self {
        Self::string_function_shape_body(
            crate::plan::FunctionShape::from_function_type(type_),
            body,
        )
    }

    #[cfg(test)]
    pub(crate) fn bit_array_function(
        _runtime_id: BitArrayFunctionFunctionId,
        expression: super::BitArrayFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::bit_array_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn bit_array_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: BitArrayFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BitArrayFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bit_array_function_body(
        _runtime_id: BitArrayFunctionFunctionId,
        type_: FunctionType,
        body: BitArrayFunctionReturn,
    ) -> Self {
        Self::bit_array_function_shape_body(
            crate::plan::FunctionShape::from_function_type(type_),
            body,
        )
    }

    pub(crate) fn utf_codepoint_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: UtfCodepointFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepointFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn utf_codepoint_function_body(
        _runtime_id: UtfCodepointFunctionFunctionId,
        type_: FunctionType,
        body: UtfCodepointFunctionReturn,
    ) -> Self {
        Self::utf_codepoint_function_shape_body(
            crate::plan::FunctionShape::from_function_type(type_),
            body,
        )
    }

    pub(crate) fn custom_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: CustomFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::CustomFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn custom_function_body(_runtime_index: usize, body: CustomFunctionReturn) -> Self {
        let shape = crate::plan::FunctionShape::new(
            body.type_().argument_shapes().to_vec(),
            crate::plan::ValueShape::Custom(body.type_().return_().clone()),
        );
        Self::custom_function_shape_body(shape, body)
    }

    #[cfg(test)]
    pub(crate) fn bool_function(
        _runtime_id: BoolFunctionFunctionId,
        expression: super::BoolFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::bool_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn bool_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: BoolFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BoolFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool_function_body(
        _runtime_id: BoolFunctionFunctionId,
        type_: FunctionType,
        body: BoolFunctionReturn,
    ) -> Self {
        Self::bool_function_shape_body(crate::plan::FunctionShape::from_function_type(type_), body)
    }

    #[cfg(test)]
    pub(crate) fn nil_function(
        _runtime_id: NilFunctionFunctionId,
        expression: super::NilFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::nil_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn nil_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: NilFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::NilFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil_function_body(
        _runtime_id: NilFunctionFunctionId,
        type_: FunctionType,
        body: NilFunctionReturn,
    ) -> Self {
        Self::nil_function_shape_body(crate::plan::FunctionShape::from_function_type(type_), body)
    }

    #[cfg(test)]
    pub(crate) fn tuple_function(
        _runtime_id: TupleFunctionFunctionId,
        expression: super::TupleFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::tuple_function_shape_body(shape, ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: TupleFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::TupleFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn list_function(
        _runtime_id: ListFunctionFunctionId,
        expression: super::ListFunctionExpr,
    ) -> Self {
        let item_type = expression.return_item_type();
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::list_function_shape_body(shape, item_type, ReturnBody::expr(expression))
    }

    pub(crate) fn list_function_shape_body(
        shape: crate::plan::FunctionShape,
        item_type: ValueType,
        body: ListFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::ListFunction {
                shape,
                item_type,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn list_function_body(
        runtime_id: ListFunctionFunctionId,
        body: ListFunctionReturn,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(runtime_id.type_().clone());
        Self::list_function_shape_body(shape, runtime_id.item_type(), body)
    }

    #[cfg(test)]
    pub(crate) fn function_function(
        _runtime_index: usize,
        expression: super::FunctionFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(expression.type_().clone());
        Self::function_function_shape_body(shape, FunctionFunctionReturn::expr(expression))
    }

    pub(crate) fn function_function_shape_body(
        shape: crate::plan::FunctionShape,
        body: FunctionFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FunctionFunction { shape, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn function_function_body(
        _runtime_index: usize,
        body: FunctionFunctionReturn,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(body.type_().to_function_type());
        Self::function_function_shape_body(shape, body)
    }

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Generic { parameter, .. } => ValueType::Parameter(*parameter),
            ReturnExprKind::Int { .. } => ValueType::Int,
            ReturnExprKind::Float { .. } => ValueType::Float,
            ReturnExprKind::String { .. } => ValueType::String,
            ReturnExprKind::BitArray { .. } => ValueType::BitArray,
            ReturnExprKind::UtfCodepoint { .. } => ValueType::UtfCodepoint,
            ReturnExprKind::Custom { body } => ValueType::Custom(body.shape().type_().clone()),
            ReturnExprKind::Bool { .. } => ValueType::Bool,
            ReturnExprKind::Nil { .. } => ValueType::Nil,
            ReturnExprKind::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            ReturnExprKind::GenericList { parameter, .. } => {
                ValueType::List(Box::new(ValueType::Parameter(*parameter)))
            }
            ReturnExprKind::ParameterListList { parameter, .. } => ValueType::List(Box::new(
                ValueType::List(Box::new(ValueType::Parameter(*parameter))),
            )),
            ReturnExprKind::IntList { .. } => ValueType::List(Box::new(ValueType::Int)),
            ReturnExprKind::StringList { .. } => ValueType::List(Box::new(ValueType::String)),
            ReturnExprKind::BitArrayList { .. } => ValueType::List(Box::new(ValueType::BitArray)),
            ReturnExprKind::UtfCodepointList { .. } => {
                ValueType::List(Box::new(ValueType::UtfCodepoint))
            }
            ReturnExprKind::CustomList { item_type, .. } => {
                ValueType::List(Box::new(ValueType::Custom(item_type.clone())))
            }
            ReturnExprKind::FloatList { .. } => ValueType::List(Box::new(ValueType::Float)),
            ReturnExprKind::BoolList { .. } => ValueType::List(Box::new(ValueType::Bool)),
            ReturnExprKind::NilList { .. } => ValueType::List(Box::new(ValueType::Nil)),
            ReturnExprKind::TupleList { item_type, .. } => {
                ValueType::List(Box::new(ValueType::Tuple(item_type.clone())))
            }
            ReturnExprKind::ListList { item_shape, .. } => {
                ValueType::List(Box::new(ValueType::List(Box::new(item_shape.value_type()))))
            }
            ReturnExprKind::FunctionList { item_type, .. } => {
                ValueType::List(Box::new(ValueType::Function(Box::new(item_type.clone()))))
            }
            ReturnExprKind::GenericFunction { shape, .. }
            | ReturnExprKind::IntFunction { shape, .. }
            | ReturnExprKind::FloatFunction { shape, .. }
            | ReturnExprKind::StringFunction { shape, .. }
            | ReturnExprKind::BitArrayFunction { shape, .. }
            | ReturnExprKind::UtfCodepointFunction { shape, .. }
            | ReturnExprKind::CustomFunction { shape, .. }
            | ReturnExprKind::BoolFunction { shape, .. }
            | ReturnExprKind::NilFunction { shape, .. }
            | ReturnExprKind::TupleFunction { shape, .. }
            | ReturnExprKind::ListFunction { shape, .. }
            | ReturnExprKind::FunctionFunction { shape, .. } => {
                ValueType::Function(Box::new(shape.type_()))
            }
        }
    }
}

impl CustomReturn {
    #[cfg(test)]
    pub(crate) fn expr(expression: CustomExpr) -> Self {
        let shape = expression.shape().clone();
        Self::with_signature_shape(shape, expression)
    }

    pub(crate) fn with_signature_shape(
        signature_shape: crate::plan::CustomValueShape,
        expression: CustomExpr,
    ) -> Self {
        let (body_shape, kind) = expression.into_parts();
        Self {
            signature_shape,
            body_shape,
            body: custom_return_body(kind),
        }
    }

    #[cfg(test)]
    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            signature_shape: return_.signature_shape,
            body_shape: return_.body_shape,
            body: ReturnBody::block(steps, return_.body),
        }
    }

    pub(crate) fn shape(&self) -> &crate::plan::CustomValueShape {
        &self.body_shape
    }

    pub(crate) fn signature_shape(&self) -> &crate::plan::CustomValueShape {
        &self.signature_shape
    }

    pub(crate) fn body(&self) -> &ReturnBody<super::CustomExprKind, FunctionInstantiation> {
        &self.body
    }
}

fn custom_return_body(
    kind: super::CustomExprKind,
) -> ReturnBody<super::CustomExprKind, FunctionInstantiation> {
    use super::CustomExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function, args),
        K::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            *subject,
            custom_return_body(*true_),
            custom_return_body(*false_),
        ),
        K::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_return_body(branch)))
                .collect(),
            custom_return_body(*fallback),
        ),
        K::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_return_body(branch)))
                .collect(),
            custom_return_body(*fallback),
        ),
        K::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_return_body(branch)))
                .collect(),
            custom_return_body(*fallback),
        ),
        K::Block { steps, return_ } => ReturnBody::block(steps, custom_return_body(*return_)),
        kind => ReturnBody::expr(kind),
    }
}

impl CustomFunctionReturn {
    pub(crate) fn expr(expression: super::CustomFunctionExpr) -> Self {
        let (type_, kind) = expression.into_parts();
        Self {
            type_,
            body: custom_function_return_body(kind),
        }
    }

    pub(crate) fn type_(&self) -> &CustomFunctionType {
        &self.type_
    }

    pub(crate) fn kind(
        &self,
    ) -> &ReturnBodyKind<super::CustomFunctionExprKind, FunctionInstantiation> {
        self.body.kind()
    }

    pub(crate) fn body(&self) -> &ReturnBody<super::CustomFunctionExprKind, FunctionInstantiation> {
        &self.body
    }
}

fn custom_function_return_body(
    kind: super::CustomFunctionExprKind,
) -> ReturnBody<super::CustomFunctionExprKind, FunctionInstantiation> {
    use super::CustomFunctionExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function, args),
        K::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            *subject,
            custom_function_return_body(*true_),
            custom_function_return_body(*false_),
        ),
        K::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_return_body(branch)))
                .collect(),
            custom_function_return_body(*fallback),
        ),
        K::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_return_body(branch)))
                .collect(),
            custom_function_return_body(*fallback),
        ),
        K::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, custom_function_return_body(branch)))
                .collect(),
            custom_function_return_body(*fallback),
        ),
        K::Block { steps, return_ } => {
            ReturnBody::block(steps, custom_function_return_body(*return_))
        }
        kind => ReturnBody::expr(kind),
    }
}

impl FunctionFunctionReturn {
    pub(crate) fn expr(expression: super::FunctionFunctionExpr) -> Self {
        let (type_, kind) = expression.into_parts();
        Self {
            type_,
            body: function_function_return_body(kind),
        }
    }

    #[cfg(test)]
    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.body))
            .collect();
        Self {
            type_: fallback.type_,
            body: ReturnBody::int_case(subject, clauses, fallback.body),
        }
    }

    #[cfg(test)]
    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.body))
            .collect();
        Self {
            type_: fallback.type_,
            body: ReturnBody::string_case(subject, clauses, fallback.body),
        }
    }

    #[cfg(test)]
    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            type_: return_.type_,
            body: ReturnBody::block(steps, return_.body),
        }
    }

    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn kind(
        &self,
    ) -> &ReturnBodyKind<super::FunctionFunctionExprKind, FunctionInstantiation> {
        self.body.kind()
    }

    pub(crate) fn body(
        &self,
    ) -> &ReturnBody<super::FunctionFunctionExprKind, FunctionInstantiation> {
        &self.body
    }
}

fn function_function_return_body(
    kind: super::FunctionFunctionExprKind,
) -> ReturnBody<super::FunctionFunctionExprKind, FunctionInstantiation> {
    use super::FunctionFunctionExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function, args),
        K::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            *subject,
            function_function_return_body(*true_),
            function_function_return_body(*false_),
        ),
        K::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, function_function_return_body(branch)))
                .collect(),
            function_function_return_body(*fallback),
        ),
        K::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, function_function_return_body(branch)))
                .collect(),
            function_function_return_body(*fallback),
        ),
        K::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            *subject,
            clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, function_function_return_body(branch)))
                .collect(),
            function_function_return_body(*fallback),
        ),
        K::Block { steps, return_ } => {
            ReturnBody::block(steps, function_function_return_body(*return_))
        }
        kind => ReturnBody::expr(kind),
    }
}

impl<Expression, Function> ReturnBody<Expression, Function> {
    pub(crate) fn expr(expression: Expression) -> Self {
        Self {
            kind: ReturnBodyKind::Expr(expression),
        }
    }

    pub(crate) fn tail_call(function: Function, args: Vec<CallArg>) -> Self {
        Self {
            kind: ReturnBodyKind::TailCall { function, args },
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self {
            kind: ReturnBodyKind::BoolCase {
                subject,
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self {
            kind: ReturnBodyKind::IntCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            kind: ReturnBodyKind::FloatCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            kind: ReturnBodyKind::StringCase {
                subject,
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            kind: ReturnBodyKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn kind(&self) -> &ReturnBodyKind<Expression, Function> {
        &self.kind
    }
}

impl Param {
    #[cfg(test)]
    pub(crate) fn named(local: ParamLocal, name: EcoString) -> Self {
        let shape = local.value_shape();
        Self::named_shape(local, name, shape)
    }

    pub(crate) fn named_shape(
        local: ParamLocal,
        name: EcoString,
        shape: crate::plan::ValueShape,
    ) -> Self {
        Self {
            local,
            binding: ParamBinding::Named(name),
            shape,
        }
    }

    #[cfg(test)]
    pub(crate) fn discard(local: ParamLocal) -> Self {
        let shape = local.value_shape();
        Self::discard_shape(local, shape)
    }

    pub(crate) fn discard_shape(local: ParamLocal, shape: crate::plan::ValueShape) -> Self {
        Self {
            local,
            binding: ParamBinding::Discard,
            shape,
        }
    }

    pub fn name(&self) -> Option<&EcoString> {
        match &self.binding {
            ParamBinding::Named(name) => Some(name),
            ParamBinding::Discard => None,
        }
    }

    pub fn binding(&self) -> &ParamBinding {
        &self.binding
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }

    pub(crate) fn shape(&self) -> &crate::plan::ValueShape {
        &self.shape
    }
}

impl ParamSlot {
    pub(crate) fn new(local: ParamLocal, shape: crate::plan::ValueShape) -> Self {
        Self { local, shape }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }

    pub(crate) fn shape(&self) -> &crate::plan::ValueShape {
        &self.shape
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.shape.value_type()
    }

    #[cfg(test)]
    pub(crate) fn from_local(local: ParamLocal) -> Self {
        let shape = local.value_shape();
        Self::new(local, shape)
    }
}

impl ParamLocal {
    pub(crate) fn generic(local: GenericLocal) -> Self {
        Self::Generic(local)
    }

    pub(crate) fn int(local: IntLocalId) -> Self {
        Self::Int(local)
    }

    pub(crate) fn float(local: FloatLocalId) -> Self {
        Self::Float(local)
    }

    pub(crate) fn string(local: StringLocalId) -> Self {
        Self::String(local)
    }

    pub(crate) fn bit_array(local: BitArrayLocalId) -> Self {
        Self::BitArray(local)
    }

    pub(crate) fn utf_codepoint(local: UtfCodepointLocalId) -> Self {
        Self::UtfCodepoint(local)
    }

    #[cfg(test)]
    pub(crate) fn custom(local: CustomLocalId, type_: CustomType) -> Self {
        Self::Custom(CustomLocal::new(local, type_))
    }

    pub(crate) fn custom_shape(local: CustomLocalId, shape: crate::plan::CustomValueShape) -> Self {
        Self::Custom(CustomLocal::from_shape(local, shape))
    }

    pub(crate) fn bool(local: BoolLocalId) -> Self {
        Self::Bool(local)
    }

    pub(crate) fn nil(local: NilLocalId) -> Self {
        Self::Nil(local)
    }

    pub(crate) fn tuple(local: TupleLocalId, type_: Vec<ValueType>) -> Self {
        Self::Tuple { local, type_ }
    }

    pub(crate) fn list(local: ListLocal) -> Self {
        Self::List(local)
    }

    pub(crate) fn int_function(local: IntFunctionLocalId, type_: FunctionType) -> Self {
        Self::IntFunction { local, type_ }
    }

    pub(crate) fn float_function(local: FloatFunctionLocalId, type_: FunctionType) -> Self {
        Self::FloatFunction { local, type_ }
    }

    pub(crate) fn string_function(local: StringFunctionLocalId, type_: FunctionType) -> Self {
        Self::StringFunction { local, type_ }
    }

    pub(crate) fn bit_array_function(local: BitArrayFunctionLocalId, type_: FunctionType) -> Self {
        Self::BitArrayFunction { local, type_ }
    }

    pub(crate) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    ) -> Self {
        Self::UtfCodepointFunction { local, type_ }
    }

    pub(crate) fn custom_function(local: CustomFunctionLocal) -> Self {
        Self::CustomFunction(local)
    }

    pub(crate) fn bool_function(local: BoolFunctionLocalId, type_: FunctionType) -> Self {
        Self::BoolFunction { local, type_ }
    }

    pub(crate) fn nil_function(local: NilFunctionLocalId, type_: FunctionType) -> Self {
        Self::NilFunction { local, type_ }
    }

    pub(crate) fn tuple_function(local: TupleFunctionLocalId, type_: FunctionType) -> Self {
        Self::TupleFunction { local, type_ }
    }

    pub(crate) fn list_function(local: ListFunctionLocal) -> Self {
        Self::ListFunction(local)
    }

    pub(crate) fn function_function(local: FunctionFunctionLocal) -> Self {
        Self::FunctionFunction(local)
    }

    pub(crate) fn generic_function(local: GenericFunctionLocal) -> Self {
        Self::GenericFunction(local)
    }

    pub(crate) fn value_type(&self) -> ValueType {
        match self {
            Self::Generic(local) => ValueType::Parameter(local.parameter()),
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::String(_) => ValueType::String,
            Self::BitArray(_) => ValueType::BitArray,
            Self::UtfCodepoint(_) => ValueType::UtfCodepoint,
            Self::Custom(local) => ValueType::Custom(local.type_().clone()),
            Self::Bool(_) => ValueType::Bool,
            Self::Nil(_) => ValueType::Nil,
            Self::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            Self::List(local) => local.value_type(),
            Self::IntFunction { type_, .. }
            | Self::FloatFunction { type_, .. }
            | Self::StringFunction { type_, .. }
            | Self::BitArrayFunction { type_, .. }
            | Self::UtfCodepointFunction { type_, .. }
            | Self::BoolFunction { type_, .. }
            | Self::NilFunction { type_, .. }
            | Self::TupleFunction { type_, .. } => ValueType::Function(Box::new(type_.clone())),
            Self::CustomFunction(local) => {
                ValueType::Function(Box::new(local.type_().to_function_type()))
            }
            Self::ListFunction(local) => local.value_type(),
            Self::FunctionFunction(local) => {
                ValueType::Function(Box::new(local.type_().to_function_type()))
            }
            Self::GenericFunction(local) => {
                ValueType::Function(Box::new(local.type_().shape().type_()))
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn value_shape(&self) -> crate::plan::ValueShape {
        match self {
            Self::Generic(local) => crate::plan::ValueShape::Parameter(local.parameter()),
            Self::Custom(local) => crate::plan::ValueShape::Custom(local.shape().clone()),
            Self::CustomFunction(local) => {
                crate::plan::ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                    local.type_().argument_shapes().to_vec(),
                    crate::plan::ValueShape::Custom(local.type_().return_().clone()),
                )))
            }
            Self::GenericFunction(local) => {
                crate::plan::ValueShape::Function(Box::new(local.type_().shape()))
            }
            _ => crate::plan::ValueShape::from_value_type(self.value_type()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListReturn, BoolListReturn, CustomFunctionReturn, CustomListReturn, CustomReturn,
        FloatListReturn, FunctionFunctionReturn, FunctionListReturn, FunctionTemplate,
        GenericFunctionReturn, GenericListReturn, GenericReturn, IntListReturn, ListListReturn,
        ListReturn, NilListReturn, Param, ParamBinding, ParamLocal, ParameterListListReturn,
        ReturnBody, ReturnBodyKind, ReturnExpr, StringListReturn, TupleListReturn,
    };
    use crate::plan::{
        BitArrayExpr, BoolExpr, BoolFunctionLocalId, BoolLocalId, CustomConstructorRefinement,
        CustomExpr, CustomFunctionExpr, CustomFunctionLocal, CustomFunctionLocalId,
        CustomFunctionType, CustomType, CustomTypeName, CustomValueShape, FloatExpr,
        FloatFunctionLocalId, FloatLocalId, FunctionFunctionExpr, FunctionFunctionLocal,
        FunctionFunctionLocalId, FunctionFunctionType, FunctionTemplateId, FunctionType,
        GenericFunctionLocal, GenericFunctionLocalId, GenericFunctionType, GenericLocal,
        GenericLocalId, IntExpr, IntFunctionLocalId, IntListLocalId, IntLocalId, ListExpr,
        ListLocal, NilExpr, NilFunctionLocalId, StringExpr, StringFunctionLocalId, TupleExpr,
        TupleFunctionLocalId, TypeParameterId, UtfCodepointExpr, UtfCodepointListReturn,
        UtfCodepointLocalId, ValueShape, ValueStorageShape, ValueType,
    };
    use num_bigint::BigInt;

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn custom_function_parameter_helpers_preserve_recursive_value_shape() {
        let type_ = custom_type();
        let return_shape = CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let type_ = CustomFunctionType::from_shapes(
            vec![ValueShape::Custom(return_shape.clone())],
            return_shape.clone(),
        );
        let local =
            ParamLocal::custom_function(CustomFunctionLocal::new(CustomFunctionLocalId(0), type_));

        assert_eq!(
            local.value_shape(),
            ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                vec![ValueShape::Custom(return_shape.clone())],
                ValueShape::Custom(return_shape),
            ))),
        );
    }

    #[test]
    fn callable_returns_own_exact_type_around_itemless_bodies() {
        let custom_function_type = CustomFunctionType::new(vec![ValueType::Int], custom_type());
        let custom_function_shape = crate::plan::FunctionShape::new(
            custom_function_type.argument_shapes().to_vec(),
            ValueShape::Custom(custom_function_type.return_().clone()),
        );
        let custom_instantiation = crate::plan::monomorphic_function_instantiation(
            7,
            crate::plan::FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(custom_function_shape)),
            ),
        );
        let custom_return = CustomFunctionReturn::expr(CustomFunctionExpr::block(
            Vec::new(),
            CustomFunctionExpr::call(
                custom_instantiation.clone(),
                Vec::new(),
                custom_function_type.clone(),
            ),
        ));

        assert_eq!(
            custom_return,
            CustomFunctionReturn {
                type_: custom_function_type,
                body: ReturnBody {
                    kind: ReturnBodyKind::Block {
                        steps: Vec::new(),
                        return_: Box::new(ReturnBody {
                            kind: ReturnBodyKind::TailCall {
                                function: custom_instantiation,
                                args: Vec::new(),
                            },
                        }),
                    },
                },
            },
        );

        let returned = FunctionType::new(vec![ValueType::String], ValueType::Int);
        let function_function_type = FunctionFunctionType::new(vec![ValueType::Bool], returned);
        let function_instantiation = crate::plan::monomorphic_function_instantiation(
            9,
            crate::plan::FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(crate::plan::FunctionShape::from_function_type(
                    function_function_type.to_function_type(),
                ))),
            ),
        );
        let function_return = FunctionFunctionReturn::expr(FunctionFunctionExpr::call(
            function_instantiation.clone(),
            Vec::new(),
            function_function_type.clone(),
        ));

        assert_eq!(
            function_return,
            FunctionFunctionReturn {
                type_: function_function_type,
                body: ReturnBody {
                    kind: ReturnBodyKind::TailCall {
                        function: function_instantiation,
                        args: Vec::new(),
                    },
                },
            },
        );
    }

    #[test]
    fn custom_returns_own_one_type_around_itemless_tail_calls() {
        let type_ = custom_type();
        let custom_shape = CustomValueShape::any(type_.clone());
        let function = crate::plan::monomorphic_function_instantiation(
            7,
            crate::plan::FunctionShape::new(Vec::new(), ValueShape::Custom(custom_shape.clone())),
        );
        let body = CustomReturn::expr(CustomExpr::block(
            Vec::new(),
            CustomExpr::bool_case(
                BoolExpr::value(true),
                crate::plan::CustomBoolCaseBranches::try_new(
                    CustomExpr::call(function.clone(), Vec::new(), custom_shape.clone()),
                    CustomExpr::call(function.clone(), Vec::new(), custom_shape.clone()),
                )
                .expect("matching custom branches should be valid"),
            ),
        ));

        assert_eq!(
            body,
            CustomReturn {
                signature_shape: custom_shape.clone(),
                body_shape: custom_shape,
                body: ReturnBody {
                    kind: ReturnBodyKind::Block {
                        steps: Vec::new(),
                        return_: Box::new(ReturnBody {
                            kind: ReturnBodyKind::BoolCase {
                                subject: BoolExpr::value(true),
                                true_: Box::new(ReturnBody {
                                    kind: ReturnBodyKind::TailCall {
                                        function: function.clone(),
                                        args: Vec::new(),
                                    },
                                }),
                                false_: Box::new(ReturnBody {
                                    kind: ReturnBodyKind::TailCall {
                                        function,
                                        args: Vec::new(),
                                    },
                                }),
                            },
                        }),
                    },
                },
            },
        );
    }

    #[test]
    fn function_plan_accessors() {
        let param = Param::named(ParamLocal::int(IntLocalId(0)), "x".into());
        let return_ = ReturnExpr::int_body(ReturnBody::expr(IntExpr::value(BigInt::from(1))));
        let function = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            vec![param],
            Vec::new(),
            return_,
        );

        assert_eq!(function.id(), FunctionTemplateId::new(0));
        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 1);
        assert_eq!(function.params()[0].name(), Some(&"x".into()));
        assert_eq!(function.steps(), &[]);
        assert_eq!(
            function.return_(),
            &ReturnExpr::int_body(ReturnBody::expr(IntExpr::value(BigInt::from(1))))
        );
        assert_eq!(function.frame_layout().ints(), 1);
    }

    #[test]
    fn return_expr_value_type_preserves_parametric_and_compound_families() {
        let parameter = TypeParameterId(0);
        let custom = custom_type();
        let tuple = vec![ValueType::Int, ValueType::String];
        let nested_type = Box::new(ValueType::List(Box::new(ValueType::Bool)));
        let nested_shape = ValueStorageShape::List(Box::new(ValueShape::Bool));
        let function = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let function_shape = crate::plan::FunctionShape::new(
            vec![ValueShape::Parameter(parameter)],
            ValueShape::Parameter(parameter),
        );
        let tail_call = crate::plan::monomorphic_function_instantiation(
            0,
            crate::plan::FunctionShape::new(Vec::new(), ValueShape::Nil),
        );

        let returns = [
            ReturnExpr::generic_body(
                parameter,
                GenericReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::generic_list_body(
                parameter,
                GenericListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::parameter_list_list_body(
                parameter,
                ParameterListListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::string_list_body(StringListReturn::tail_call(
                tail_call.clone(),
                Vec::new(),
            )),
            ReturnExpr::bit_array_list_body(BitArrayListReturn::tail_call(
                tail_call.clone(),
                Vec::new(),
            )),
            ReturnExpr::utf_codepoint_list_body(UtfCodepointListReturn::tail_call(
                tail_call.clone(),
                Vec::new(),
            )),
            ReturnExpr::custom_list_body(
                custom.clone(),
                CustomListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::float_list_body(FloatListReturn::tail_call(tail_call.clone(), Vec::new())),
            ReturnExpr::bool_list_body(BoolListReturn::tail_call(tail_call.clone(), Vec::new())),
            ReturnExpr::tuple_list_body(
                tuple.clone(),
                TupleListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::list_list_body(
                nested_shape,
                ListListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::function_list_body(
                function.clone(),
                FunctionListReturn::tail_call(tail_call.clone(), Vec::new()),
            ),
            ReturnExpr::generic_function_shape_body(
                function_shape.clone(),
                GenericFunctionReturn::tail_call(tail_call, Vec::new()),
            ),
        ];

        assert_eq!(
            returns.map(|return_| return_.value_type()),
            [
                ValueType::Parameter(parameter),
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
                ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Parameter(
                    parameter,
                ))))),
                ValueType::List(Box::new(ValueType::String)),
                ValueType::List(Box::new(ValueType::BitArray)),
                ValueType::List(Box::new(ValueType::UtfCodepoint)),
                ValueType::List(Box::new(ValueType::Custom(custom))),
                ValueType::List(Box::new(ValueType::Float)),
                ValueType::List(Box::new(ValueType::Bool)),
                ValueType::List(Box::new(ValueType::Tuple(tuple))),
                ValueType::List(Box::new(ValueType::List(nested_type))),
                ValueType::List(Box::new(ValueType::Function(Box::new(function)))),
                ValueType::Function(Box::new(function_shape.type_())),
            ],
        );
    }

    #[test]
    fn param_binding_accessors() {
        let named = Param::named(ParamLocal::int(IntLocalId(0)), "x".into());
        let discard = Param::discard(ParamLocal::int(IntLocalId(1)));

        assert_eq!(named.name(), Some(&"x".into()));
        assert_eq!(named.binding(), &ParamBinding::Named("x".into()));
        assert_eq!(discard.name(), None);
        assert_eq!(discard.binding(), &ParamBinding::Discard);
    }

    #[test]
    fn param_local_value_type() {
        let parameter = TypeParameterId(0);
        let generic = ParamLocal::generic(GenericLocal::new(GenericLocalId(0), parameter));
        assert_eq!(generic.value_type(), ValueType::Parameter(parameter));
        assert_eq!(generic.value_shape(), ValueShape::Parameter(parameter));

        let generic_function_type = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let generic_function = ParamLocal::generic_function(GenericFunctionLocal::new(
            GenericFunctionLocalId(0),
            generic_function_type.clone(),
        ));
        assert_eq!(
            generic_function.value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Parameter(parameter),
            ))),
        );
        assert_eq!(
            generic_function.value_shape(),
            ValueShape::Function(Box::new(generic_function_type.shape())),
        );

        assert_eq!(ParamLocal::int(IntLocalId(0)).value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::string(crate::plan::StringLocalId(0)).value_type(),
            ValueType::String,
        );
        assert_eq!(
            ParamLocal::utf_codepoint(UtfCodepointLocalId(0)).value_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ParamLocal::float(FloatLocalId(0)).value_type(),
            ValueType::Float,
        );
        assert_eq!(
            ParamLocal::bool(BoolLocalId(0)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ParamLocal::nil(crate::plan::NilLocalId(0)).value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ParamLocal::list(ListLocal::int(IntListLocalId(0))).value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            ParamLocal::int_function(
                IntFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        );
        assert_eq!(
            ParamLocal::string_function(
                StringFunctionLocalId(0),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::String],
                ValueType::String,
            ))),
        );
        assert_eq!(
            ParamLocal::utf_codepoint_function(
                crate::plan::UtfCodepointFunctionLocalId(0),
                FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::UtfCodepoint],
                ValueType::UtfCodepoint,
            ))),
        );
        assert_eq!(
            ParamLocal::float_function(
                FloatFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Float,
            ))),
        );
        assert_eq!(
            ParamLocal::bool_function(
                BoolFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Bool],
                ValueType::Bool,
            ))),
        );
        assert_eq!(
            ParamLocal::nil_function(
                NilFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Nil],
                ValueType::Nil,
            ))),
        );
        assert_eq!(
            ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                FunctionType::new(
                    vec![ValueType::Tuple(vec![ValueType::Int])],
                    ValueType::Tuple(vec![ValueType::String]),
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Tuple(vec![ValueType::Int])],
                ValueType::Tuple(vec![ValueType::String]),
            ))),
        );
        assert_eq!(
            ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                0,
                FunctionType::new(
                    vec![ValueType::List(Box::new(ValueType::Int))],
                    ValueType::List(Box::new(ValueType::String)),
                ),
                ValueType::String,
            ))
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::String)),
            ))),
        );
        assert_eq!(
            ParamLocal::function_function(FunctionFunctionLocal::new(
                FunctionFunctionLocalId(0),
                FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                ),
            ),)
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ))),
        );
    }

    #[test]
    fn list_return_expr_preserves_item_family() {
        let parameter = crate::plan::TypeParameterId(0);
        let generic = ListExpr::value(Vec::new(), ValueType::Parameter(parameter));
        assert_eq!(
            ListReturn::expr(generic.clone()),
            ListReturn::Generic {
                item_parameter: parameter,
                body: super::GenericListReturn::expr(generic.into_generic().expect("generic list"),),
            },
        );

        let int = ListExpr::value(
            vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
            ValueType::Int,
        );
        assert_eq!(
            ListReturn::expr(int.clone()),
            ListReturn::Int(IntListReturn::expr(int.into_int().expect("int list"))),
        );

        let float = ListExpr::value(
            vec![crate::plan::Expr::float(FloatExpr::value(1.5))],
            ValueType::Float,
        );
        assert_eq!(
            ListReturn::expr(float.clone()),
            ListReturn::Float(FloatListReturn::expr(
                float.into_float().expect("float list")
            )),
        );

        let string = ListExpr::value(
            vec![crate::plan::Expr::string(StringExpr::value("one".into()))],
            ValueType::String,
        );
        assert_eq!(
            ListReturn::expr(string.clone()),
            ListReturn::String(StringListReturn::expr(
                string.into_string().expect("string list"),
            )),
        );

        let bit_array = ListExpr::value(
            vec![crate::plan::Expr::bit_array(
                BitArrayExpr::value(Vec::new()),
            )],
            ValueType::BitArray,
        );
        assert_eq!(
            ListReturn::expr(bit_array.clone()),
            ListReturn::BitArray(BitArrayListReturn::expr(
                bit_array.into_bit_array().expect("bit array list"),
            )),
        );

        let utf_codepoint = ListExpr::value(
            vec![crate::plan::Expr::utf_codepoint(
                UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into()),
            )],
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ListReturn::expr(utf_codepoint.clone()),
            ListReturn::UtfCodepoint(UtfCodepointListReturn::expr(
                utf_codepoint
                    .into_utf_codepoint()
                    .expect("UTF codepoint list"),
            )),
        );

        let custom_type = custom_type();
        let custom = ListExpr::value(Vec::new(), ValueType::Custom(custom_type.clone()));
        assert_eq!(
            ListReturn::expr(custom.clone()),
            ListReturn::Custom {
                item_type: custom_type,
                body: CustomListReturn::expr(custom.into_custom().expect("custom list")),
            },
        );

        let bool_ = ListExpr::value(
            vec![crate::plan::Expr::bool(BoolExpr::value(true))],
            ValueType::Bool,
        );
        assert_eq!(
            ListReturn::expr(bool_.clone()),
            ListReturn::Bool(BoolListReturn::expr(bool_.into_bool().expect("bool list"))),
        );

        let nil = ListExpr::value(
            vec![crate::plan::Expr::nil(NilExpr::value())],
            ValueType::Nil,
        );
        assert_eq!(
            ListReturn::expr(nil.clone()),
            ListReturn::Nil(NilListReturn::expr(nil.into_nil().expect("nil list"))),
        );

        let tuple = ListExpr::value(
            vec![crate::plan::Expr::tuple(TupleExpr::value(
                vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
                vec![ValueType::Int],
            ))],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListReturn::expr(tuple.clone()),
            ListReturn::Tuple {
                item_type: vec![ValueType::Int],
                body: TupleListReturn::expr(tuple.into_tuple().expect("tuple list")),
            },
        );

        let nested = ListExpr::value(
            vec![crate::plan::Expr::list(ListExpr::value(
                vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
                ValueType::Int,
            ))],
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            ListReturn::expr(nested.clone()),
            ListReturn::List {
                item_shape: ValueStorageShape::Int,
                body: ListListReturn::expr(nested.into_list().expect("nested list")),
            },
        );

        let parameter = TypeParameterId(0);
        let parameter_nested = ListExpr::value(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Parameter(parameter))),
        );
        assert_eq!(
            ListReturn::expr(parameter_nested.clone()),
            ListReturn::ParameterList {
                item_parameter: parameter,
                body: ParameterListListReturn::expr(
                    parameter_nested
                        .into_parameter_list()
                        .expect("parameter-list list"),
                ),
            },
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_instantiation = crate::plan::monomorphic_function_instantiation(
            0,
            crate::plan::FunctionShape::from_function_type(function_type.clone()),
        );
        let function = ListExpr::value(
            vec![crate::plan::Expr::function(
                crate::plan::FunctionExpr::reference(crate::plan::FunctionReference::new(
                    function_instantiation,
                    Vec::new(),
                )),
            )],
            ValueType::Function(Box::new(function_type.clone())),
        );
        assert_eq!(
            ListReturn::expr(function.clone()),
            ListReturn::Function {
                item_type: function_type,
                body: FunctionListReturn::expr(function.into_function().expect("function list")),
            },
        );
    }

    #[test]
    fn list_return_tail_call_preserves_item_family() {
        fn tail_call_function(item_type: ValueType) -> crate::plan::FunctionInstantiation {
            crate::plan::monomorphic_function_instantiation(
                0,
                crate::plan::FunctionShape::new(
                    Vec::new(),
                    ValueShape::List(Box::new(ValueShape::from_value_type(item_type))),
                ),
            )
        }

        let parameter = crate::plan::TypeParameterId(0);
        let function = tail_call_function(ValueType::Parameter(parameter));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::Parameter(parameter),
                Vec::new(),
            ),
            ListReturn::Generic {
                item_parameter: parameter,
                body: super::GenericListReturn::tail_call(function, Vec::new()),
            },
        );

        let function = tail_call_function(ValueType::Int);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::Int, Vec::new()),
            ListReturn::Int(IntListReturn::tail_call(function, Vec::new())),
        );

        let function = tail_call_function(ValueType::Float);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::Float, Vec::new()),
            ListReturn::Float(FloatListReturn::tail_call(function, Vec::new())),
        );

        let function = tail_call_function(ValueType::String);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::String, Vec::new()),
            ListReturn::String(StringListReturn::tail_call(function, Vec::new())),
        );

        let function = tail_call_function(ValueType::BitArray);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::BitArray, Vec::new()),
            ListReturn::BitArray(BitArrayListReturn::tail_call(function, Vec::new())),
        );

        let function = tail_call_function(ValueType::UtfCodepoint);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::UtfCodepoint, Vec::new()),
            ListReturn::UtfCodepoint(UtfCodepointListReturn::tail_call(function, Vec::new())),
        );

        let custom_type = custom_type();
        let function = tail_call_function(ValueType::Custom(custom_type.clone()));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::Custom(custom_type.clone()),
                Vec::new(),
            ),
            ListReturn::Custom {
                item_type: custom_type,
                body: CustomListReturn::tail_call(function, Vec::new()),
            },
        );

        let function = tail_call_function(ValueType::Bool);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::Bool, Vec::new()),
            ListReturn::Bool(BoolListReturn::tail_call(function, Vec::new())),
        );

        let function = tail_call_function(ValueType::Nil);
        assert_eq!(
            ListReturn::tail_call(function.clone(), ValueType::Nil, Vec::new()),
            ListReturn::Nil(NilListReturn::tail_call(function, Vec::new())),
        );

        let tuple_type = vec![ValueType::Int];
        let function = tail_call_function(ValueType::Tuple(tuple_type.clone()));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::Tuple(tuple_type.clone()),
                Vec::new(),
            ),
            ListReturn::Tuple {
                item_type: tuple_type,
                body: TupleListReturn::tail_call(function, Vec::new()),
            },
        );

        let list_type = Box::new(ValueType::Int);
        let function = tail_call_function(ValueType::List(list_type.clone()));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::List(list_type.clone()),
                Vec::new(),
            ),
            ListReturn::List {
                item_shape: ValueStorageShape::Int,
                body: ListListReturn::tail_call(function, Vec::new()),
            },
        );

        let parameter = TypeParameterId(0);
        let parameter_list_type = Box::new(ValueType::Parameter(parameter));
        let function = tail_call_function(ValueType::List(parameter_list_type.clone()));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::List(parameter_list_type),
                Vec::new(),
            ),
            ListReturn::ParameterList {
                item_parameter: parameter,
                body: ParameterListListReturn::tail_call(function, Vec::new()),
            },
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function = tail_call_function(ValueType::Function(Box::new(function_type.clone())));
        assert_eq!(
            ListReturn::tail_call(
                function.clone(),
                ValueType::Function(Box::new(function_type.clone())),
                Vec::new(),
            ),
            ListReturn::Function {
                item_type: function_type,
                body: FunctionListReturn::tail_call(function, Vec::new()),
            },
        );
    }

    #[test]
    fn list_return_cases_and_block_preserve_typed_body() {
        let true_ = IntListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
                ValueType::Int,
            )
            .into_int()
            .expect("int list"),
        );
        let false_ = IntListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::int(IntExpr::value(2.into()))],
                ValueType::Int,
            )
            .into_int()
            .expect("int list"),
        );
        assert_eq!(
            ListReturn::try_bool_case(
                BoolExpr::value(true),
                ListReturn::Int(true_.clone()),
                ListReturn::Int(false_.clone()),
            ),
            Some(ListReturn::Int(IntListReturn::bool_case(
                BoolExpr::value(true),
                true_,
                false_,
            ))),
        );

        let fallback = StringListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::string(StringExpr::value(
                    "fallback".into(),
                ))],
                ValueType::String,
            )
            .into_string()
            .expect("string list"),
        );
        let branch = StringListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::string(StringExpr::value(
                    "branch".into(),
                ))],
                ValueType::String,
            )
            .into_string()
            .expect("string list"),
        );
        assert_eq!(
            ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), ListReturn::String(branch.clone()))],
                ListReturn::String(fallback.clone()),
            ),
            Some(ListReturn::String(StringListReturn::int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), branch)],
                fallback,
            ))),
        );

        let fallback = FloatListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::float(FloatExpr::value(1.5))],
                ValueType::Float,
            )
            .into_float()
            .expect("float list"),
        );
        let branch = FloatListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::float(FloatExpr::value(2.5))],
                ValueType::Float,
            )
            .into_float()
            .expect("float list"),
        );
        assert_eq!(
            ListReturn::try_string_case(
                StringExpr::value("key".into()),
                vec![("key".into(), ListReturn::Float(branch.clone()))],
                ListReturn::Float(fallback.clone()),
            ),
            Some(ListReturn::Float(FloatListReturn::string_case(
                StringExpr::value("key".into()),
                vec![("key".into(), branch)],
                fallback,
            ))),
        );

        let fallback = BoolListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::bool(BoolExpr::value(false))],
                ValueType::Bool,
            )
            .into_bool()
            .expect("bool list"),
        );
        let branch = BoolListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::bool(BoolExpr::value(true))],
                ValueType::Bool,
            )
            .into_bool()
            .expect("bool list"),
        );
        assert_eq!(
            ListReturn::try_float_case(
                FloatExpr::value(1.5),
                vec![(1.5, ListReturn::Bool(branch.clone()))],
                ListReturn::Bool(fallback.clone()),
            ),
            Some(ListReturn::Bool(BoolListReturn::float_case(
                FloatExpr::value(1.5),
                vec![(1.5, branch)],
                fallback,
            ))),
        );

        let return_ = NilListReturn::expr(
            ListExpr::value(
                vec![crate::plan::Expr::nil(NilExpr::value())],
                ValueType::Nil,
            )
            .into_nil()
            .expect("nil list"),
        );
        assert_eq!(
            ListReturn::try_block(
                Vec::<crate::plan::Step>::new(),
                ListReturn::Nil(return_.clone()),
            ),
            ListReturn::Nil(NilListReturn::block(
                Vec::<crate::plan::Step>::new(),
                return_,
            )),
        );
    }

    #[test]
    fn list_return_case_rejects_mismatched_item_families() {
        assert_eq!(
            ListReturn::try_bool_case(
                BoolExpr::value(true),
                ListReturn::expr(ListExpr::value(
                    vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
                    ValueType::Int,
                )),
                ListReturn::expr(ListExpr::value(
                    vec![crate::plan::Expr::string(StringExpr::value("wrong".into()))],
                    ValueType::String,
                )),
            ),
            None,
        );
        assert_eq!(
            ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(
                    BigInt::from(1),
                    ListReturn::expr(ListExpr::value(
                        vec![crate::plan::Expr::string(StringExpr::value("wrong".into()))],
                        ValueType::String,
                    )),
                )],
                ListReturn::expr(ListExpr::value(
                    vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
                    ValueType::Int,
                )),
            ),
            None,
        );
        assert_eq!(
            ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(
                    BigInt::from(1),
                    ListReturn::expr(ListExpr::value(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::String)),
                    )),
                )],
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::Tuple(vec![ValueType::Int])
                )),
            ),
            None,
        );
        assert_eq!(
            ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(
                    BigInt::from(1),
                    ListReturn::expr(ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            ValueType::Bool
                        ))),
                    )),
                )],
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::String)),
                )),
            ),
            None,
        );
        assert_eq!(
            ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(
                    BigInt::from(1),
                    ListReturn::expr(ListExpr::value(
                        Vec::new(),
                        ValueType::Tuple(vec![ValueType::Int])
                    )),
                )],
                ListReturn::expr(ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
                )),
            ),
            None,
        );
    }

    #[test]
    fn return_body_kind_accessor_exposes_exact_shape() {
        let expression = ListExpr::value(
            vec![crate::plan::Expr::int(IntExpr::value(1.into()))],
            ValueType::Int,
        )
        .into_int()
        .expect("int list");
        let body = IntListReturn::expr(expression.clone());
        assert_eq!(body.kind(), &ReturnBodyKind::Expr(expression));
    }

    #[test]
    fn list_return_case_helpers_preserve_all_item_families() {
        let item_types = vec![
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(1)))),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        ];

        for item_type in item_types {
            let true_ = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let false_ = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let bool_case = ListReturn::try_bool_case(BoolExpr::value(true), true_, false_);
            assert_eq!(
                bool_case.as_ref().map(list_return_item_type),
                Some(item_type.clone())
            );

            let branch = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let fallback = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let int_case = ListReturn::try_int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), branch)],
                fallback,
            );
            assert_eq!(
                int_case.as_ref().map(list_return_item_type),
                Some(item_type.clone())
            );

            let branch = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let fallback = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let float_case =
                ListReturn::try_float_case(FloatExpr::value(1.5), vec![(1.5, branch)], fallback);
            assert_eq!(
                float_case.as_ref().map(list_return_item_type),
                Some(item_type.clone()),
            );

            let branch = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let fallback = ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone()));
            let string_case = ListReturn::try_string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), branch)],
                fallback,
            );
            assert_eq!(
                string_case.as_ref().map(list_return_item_type),
                Some(item_type.clone()),
            );

            let block = ListReturn::try_block(
                Vec::<crate::plan::Step>::new(),
                ListReturn::expr(ListExpr::value(Vec::new(), item_type.clone())),
            );
            assert_eq!(list_return_item_type(&block), item_type);
        }
    }

    #[test]
    fn list_return_case_helpers_reject_clause_mismatch_for_all_item_families() {
        fn empty_return(item_type: ValueType) -> ListReturn {
            ListReturn::expr(ListExpr::value(Vec::new(), item_type))
        }

        let item_types = vec![
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(1)))),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        ];

        for item_type in item_types {
            let mismatched_type = if item_type == ValueType::Int {
                ValueType::String
            } else {
                ValueType::Int
            };

            assert_eq!(
                ListReturn::try_int_case(
                    IntExpr::value(1.into()),
                    vec![(BigInt::from(1), empty_return(mismatched_type.clone()))],
                    empty_return(item_type.clone()),
                ),
                None,
            );
            assert_eq!(
                ListReturn::try_float_case(
                    FloatExpr::value(1.5),
                    vec![(1.5, empty_return(mismatched_type.clone()))],
                    empty_return(item_type.clone()),
                ),
                None,
            );
            assert_eq!(
                ListReturn::try_string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), empty_return(mismatched_type))],
                    empty_return(item_type),
                ),
                None,
            );
        }
    }

    #[test]
    fn list_return_case_helpers_reject_nested_item_metadata_mismatch() {
        fn empty_return(item_type: ValueType) -> ListReturn {
            ListReturn::expr(ListExpr::value(Vec::new(), item_type))
        }

        fn assert_case_helpers_reject(branch_type: ValueType, fallback_type: ValueType) {
            assert_eq!(
                ListReturn::try_bool_case(
                    BoolExpr::value(true),
                    empty_return(branch_type.clone()),
                    empty_return(fallback_type.clone()),
                ),
                None,
            );
            assert_eq!(
                ListReturn::try_int_case(
                    IntExpr::value(1.into()),
                    vec![(BigInt::from(1), empty_return(branch_type.clone()))],
                    empty_return(fallback_type.clone()),
                ),
                None,
            );
            assert_eq!(
                ListReturn::try_float_case(
                    FloatExpr::value(1.5),
                    vec![(1.5, empty_return(branch_type.clone()))],
                    empty_return(fallback_type.clone()),
                ),
                None,
            );
            assert_eq!(
                ListReturn::try_string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), empty_return(branch_type))],
                    empty_return(fallback_type),
                ),
                None,
            );
        }

        assert_case_helpers_reject(
            ValueType::Tuple(vec![ValueType::String]),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_case_helpers_reject(
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_case_helpers_reject(
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
    }

    fn list_return_item_type(return_: &ListReturn) -> ValueType {
        match return_ {
            ListReturn::Generic { item_parameter, .. } => ValueType::Parameter(*item_parameter),
            ListReturn::Int(_) => ValueType::Int,
            ListReturn::Float(_) => ValueType::Float,
            ListReturn::String(_) => ValueType::String,
            ListReturn::BitArray(_) => ValueType::BitArray,
            ListReturn::UtfCodepoint(_) => ValueType::UtfCodepoint,
            ListReturn::Custom { item_type, .. } => ValueType::Custom(item_type.clone()),
            ListReturn::Bool(_) => ValueType::Bool,
            ListReturn::Nil(_) => ValueType::Nil,
            ListReturn::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            ListReturn::ParameterList { item_parameter, .. } => {
                ValueType::List(Box::new(ValueType::Parameter(*item_parameter)))
            }
            ListReturn::List { item_shape, .. } => {
                ValueType::List(Box::new(item_shape.value_type()))
            }
            ListReturn::Function { item_type, .. } => {
                ValueType::Function(Box::new(item_type.clone()))
            }
        }
    }
}
