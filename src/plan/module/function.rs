use super::FrameLayout;
use super::expression::{
    BitArrayExpr, BitArrayListExpr, BoolExpr, BoolListExpr, CallArg, CustomExpr, CustomListExpr,
    FloatExpr, FloatListExpr, FunctionListExpr, IntExpr, IntListExpr, ListListExpr, NilExpr,
    NilListExpr, StringExpr, StringListExpr, TupleExpr, TupleListExpr, UtfCodepointExpr,
    UtfCodepointListExpr,
};
use super::id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionLocalId,
    BitArrayListFunctionId, BitArrayLocalId, BoolFunctionFunctionId, BoolFunctionId,
    BoolFunctionLocalId, BoolListFunctionId, BoolLocalId, CustomFunctionFunctionId,
    CustomFunctionId, CustomFunctionLocal, CustomListFunctionId, CustomLocal, CustomLocalId,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionLocalId, FloatListFunctionId,
    FloatLocalId, FunctionFunctionFunctionId, FunctionFunctionLocal, FunctionId,
    FunctionListFunctionId, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
    IntListFunctionId, IntLocalId, ListFunctionFunctionId, ListFunctionLocal, ListListFunctionId,
    ListLocal, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilListFunctionId,
    NilLocalId, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
    StringListFunctionId, StringLocalId, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionLocalId, TupleListFunctionId, TupleLocalId, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointFunctionLocalId, UtfCodepointListFunctionId,
    UtfCodepointLocalId,
};
use super::step::Step;
use crate::plan::{CustomFunctionType, CustomType, FunctionFunctionType, FunctionType, ValueType};
use ecow::EcoString;
use num_bigint::BigInt;

#[cfg(test)]
use super::expression::ListExpr;
#[cfg(test)]
use super::id::{FunctionFunctionId, ListFunctionId, RuntimeFunctionId};

#[derive(Debug, PartialEq)]
pub struct FunctionPlan {
    id: FunctionId,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamBinding {
    Named(EcoString),
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
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
}

pub(crate) struct FunctionExecutionParts {
    pub(crate) frame_layout: FrameLayout,
    pub(crate) steps: Vec<Step>,
    pub(crate) return_: ReturnExpr,
}

pub(crate) type IntReturn = ReturnBody<IntExpr, IntFunctionId>;
pub(crate) type FloatReturn = ReturnBody<FloatExpr, FloatFunctionId>;
pub(crate) type StringReturn = ReturnBody<StringExpr, StringFunctionId>;
pub(crate) type BitArrayReturn = ReturnBody<BitArrayExpr, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn = ReturnBody<UtfCodepointExpr, UtfCodepointFunctionId>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomReturn {
    type_: CustomType,
    body: ReturnBody<super::CustomExprKind, usize>,
}
pub(crate) type BoolReturn = ReturnBody<BoolExpr, BoolFunctionId>;
pub(crate) type NilReturn = ReturnBody<NilExpr, NilFunctionId>;
pub(crate) type TupleReturn = ReturnBody<TupleExpr, TupleFunctionId>;
pub(crate) type IntListReturn = ReturnBody<IntListExpr, IntListFunctionId>;
pub(crate) type FloatListReturn = ReturnBody<FloatListExpr, FloatListFunctionId>;
pub(crate) type StringListReturn = ReturnBody<StringListExpr, StringListFunctionId>;
pub(crate) type BitArrayListReturn = ReturnBody<BitArrayListExpr, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListReturn =
    ReturnBody<UtfCodepointListExpr, UtfCodepointListFunctionId>;
pub(crate) type CustomListReturn = ReturnBody<CustomListExpr, CustomListFunctionId>;
pub(crate) type BoolListReturn = ReturnBody<BoolListExpr, BoolListFunctionId>;
pub(crate) type NilListReturn = ReturnBody<NilListExpr, NilListFunctionId>;
pub(crate) type TupleListReturn = ReturnBody<TupleListExpr, TupleListFunctionId>;
pub(crate) type ListListReturn = ReturnBody<ListListExpr, ListListFunctionId>;
pub(crate) type FunctionListReturn = ReturnBody<FunctionListExpr, FunctionListFunctionId>;
pub(crate) type IntFunctionReturn = ReturnBody<super::IntFunctionExpr, IntFunctionFunctionId>;
pub(crate) type FloatFunctionReturn = ReturnBody<super::FloatFunctionExpr, FloatFunctionFunctionId>;
pub(crate) type StringFunctionReturn =
    ReturnBody<super::StringFunctionExpr, StringFunctionFunctionId>;
pub(crate) type BitArrayFunctionReturn =
    ReturnBody<super::BitArrayFunctionExpr, BitArrayFunctionFunctionId>;
pub(crate) type UtfCodepointFunctionReturn =
    ReturnBody<super::UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFunctionReturn {
    type_: CustomFunctionType,
    body: ReturnBody<super::CustomFunctionExprKind, usize>,
}
pub(crate) type BoolFunctionReturn = ReturnBody<super::BoolFunctionExpr, BoolFunctionFunctionId>;
pub(crate) type NilFunctionReturn = ReturnBody<super::NilFunctionExpr, NilFunctionFunctionId>;
pub(crate) type TupleFunctionReturn = ReturnBody<super::TupleFunctionExpr, TupleFunctionFunctionId>;
pub(crate) type ListFunctionReturn = ReturnBody<super::ListFunctionExpr, ListFunctionFunctionId>;
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionFunctionReturn {
    type_: FunctionFunctionType,
    body: ReturnBody<super::FunctionFunctionExprKind, usize>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListReturn {
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
    List {
        item_type: Box<ValueType>,
        body: ListListReturn,
    },
    Function {
        item_type: FunctionType,
        body: FunctionListReturn,
    },
}

#[cfg(test)]
impl ListReturn {
    #[cfg(test)]
    pub(crate) fn expr(expression: ListExpr) -> Self {
        match expression {
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
            ListExpr::List(expression) => Self::List {
                item_type: expression.item().item_type(),
                body: ListListReturn::expr(expression),
            },
            ListExpr::Function(expression) => Self::Function {
                item_type: expression.item().item_type(),
                body: FunctionListReturn::expr(expression),
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn tail_call(function: ListFunctionId, args: Vec<CallArg>) -> Self {
        match function {
            ListFunctionId::Int(function) => Self::Int(IntListReturn::tail_call(function, args)),
            ListFunctionId::Float(function) => {
                Self::Float(FloatListReturn::tail_call(function, args))
            }
            ListFunctionId::String(function) => {
                Self::String(StringListReturn::tail_call(function, args))
            }
            ListFunctionId::BitArray(function) => {
                Self::BitArray(BitArrayListReturn::tail_call(function, args))
            }
            ListFunctionId::UtfCodepoint(function) => {
                Self::UtfCodepoint(UtfCodepointListReturn::tail_call(function, args))
            }
            ListFunctionId::Custom { id, item_type } => Self::Custom {
                item_type,
                body: CustomListReturn::tail_call(id, args),
            },
            ListFunctionId::Bool(function) => Self::Bool(BoolListReturn::tail_call(function, args)),
            ListFunctionId::Nil(function) => Self::Nil(NilListReturn::tail_call(function, args)),
            ListFunctionId::Tuple { id, item_type } => Self::Tuple {
                item_type,
                body: TupleListReturn::tail_call(id, args),
            },
            ListFunctionId::List { id, item_type } => Self::List {
                item_type,
                body: ListListReturn::tail_call(id, args),
            },
            ListFunctionId::Function { id, item_type } => Self::Function {
                item_type,
                body: FunctionListReturn::tail_call(id, args),
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn try_bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Option<Self> {
        Some(match (true_, false_) {
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
                Self::List {
                    item_type: true_type,
                    body: true_,
                },
                Self::List {
                    item_type: false_type,
                    body: false_,
                },
            ) if true_type == false_type => Self::List {
                item_type: true_type,
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

    #[cfg(test)]
    pub(crate) fn try_int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
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
            Self::List {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_type,
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

    #[cfg(test)]
    pub(crate) fn try_float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
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
            Self::List {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_type,
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

    #[cfg(test)]
    pub(crate) fn try_string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Option<Self> {
        match fallback {
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
            Self::List {
                item_type,
                body: fallback,
            } => {
                let clauses = into_list_return_clauses(clauses, |branch| match branch {
                    Self::List {
                        item_type: branch_type,
                        body,
                    } if branch_type == item_type => Some(body),
                    _ => None,
                })?;
                Some(Self::List {
                    item_type,
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

    #[cfg(test)]
    pub(crate) fn try_block(steps: Vec<Step>, return_: Self) -> Self {
        match return_ {
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
            Self::List { item_type, body } => Self::List {
                item_type,
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
    Int {
        runtime_id: IntFunctionId,
        body: IntReturn,
    },
    Float {
        runtime_id: FloatFunctionId,
        body: FloatReturn,
    },
    String {
        runtime_id: StringFunctionId,
        body: StringReturn,
    },
    BitArray {
        runtime_id: BitArrayFunctionId,
        body: BitArrayReturn,
    },
    UtfCodepoint {
        runtime_id: UtfCodepointFunctionId,
        body: UtfCodepointReturn,
    },
    Custom {
        runtime_id: CustomFunctionId,
        body: CustomReturn,
    },
    Bool {
        runtime_id: BoolFunctionId,
        body: BoolReturn,
    },
    Nil {
        runtime_id: NilFunctionId,
        body: NilReturn,
    },
    Tuple {
        runtime_id: TupleFunctionId,
        type_: Vec<ValueType>,
        body: TupleReturn,
    },
    IntList {
        runtime_id: IntListFunctionId,
        body: IntListReturn,
    },
    StringList {
        runtime_id: StringListFunctionId,
        body: StringListReturn,
    },
    BitArrayList {
        runtime_id: BitArrayListFunctionId,
        body: BitArrayListReturn,
    },
    UtfCodepointList {
        runtime_id: UtfCodepointListFunctionId,
        body: UtfCodepointListReturn,
    },
    CustomList {
        runtime_id: CustomListFunctionId,
        item_type: CustomType,
        body: CustomListReturn,
    },
    FloatList {
        runtime_id: FloatListFunctionId,
        body: FloatListReturn,
    },
    BoolList {
        runtime_id: BoolListFunctionId,
        body: BoolListReturn,
    },
    NilList {
        runtime_id: NilListFunctionId,
        body: NilListReturn,
    },
    TupleList {
        runtime_id: TupleListFunctionId,
        item_type: Vec<ValueType>,
        body: TupleListReturn,
    },
    ListList {
        runtime_id: ListListFunctionId,
        item_type: Box<ValueType>,
        body: ListListReturn,
    },
    FunctionList {
        runtime_id: FunctionListFunctionId,
        item_type: FunctionType,
        body: FunctionListReturn,
    },
    IntFunction {
        runtime_id: IntFunctionFunctionId,
        type_: FunctionType,
        body: IntFunctionReturn,
    },
    FloatFunction {
        runtime_id: FloatFunctionFunctionId,
        type_: FunctionType,
        body: FloatFunctionReturn,
    },
    StringFunction {
        runtime_id: StringFunctionFunctionId,
        type_: FunctionType,
        body: StringFunctionReturn,
    },
    BitArrayFunction {
        runtime_id: BitArrayFunctionFunctionId,
        type_: FunctionType,
        body: BitArrayFunctionReturn,
    },
    UtfCodepointFunction {
        runtime_id: UtfCodepointFunctionFunctionId,
        type_: FunctionType,
        body: UtfCodepointFunctionReturn,
    },
    CustomFunction {
        runtime_id: CustomFunctionFunctionId,
        body: CustomFunctionReturn,
    },
    BoolFunction {
        runtime_id: BoolFunctionFunctionId,
        type_: FunctionType,
        body: BoolFunctionReturn,
    },
    NilFunction {
        runtime_id: NilFunctionFunctionId,
        type_: FunctionType,
        body: NilFunctionReturn,
    },
    TupleFunction {
        runtime_id: TupleFunctionFunctionId,
        type_: FunctionType,
        body: TupleFunctionReturn,
    },
    ListFunction {
        runtime_id: ListFunctionFunctionId,
        body: ListFunctionReturn,
    },
    FunctionFunction {
        runtime_id: FunctionFunctionFunctionId,
        body: FunctionFunctionReturn,
    },
}

impl FunctionPlan {
    pub(crate) fn new(
        id: FunctionId,
        name: EcoString,
        params: Vec<Param>,
        steps: Vec<Step>,
        return_: ReturnExpr,
    ) -> Self {
        let frame_layout = FrameLayout::from_function_parts(&params, &steps, &return_);

        Self {
            id,
            name,
            params,
            steps,
            return_,
            frame_layout,
        }
    }

    pub fn id(&self) -> FunctionId {
        self.id
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

    pub(crate) fn into_execution_parts(self) -> FunctionExecutionParts {
        FunctionExecutionParts {
            frame_layout: self.frame_layout,
            steps: self.steps,
            return_: self.return_,
        }
    }
}

impl ReturnExpr {
    #[cfg(test)]
    pub(crate) fn int(runtime_id: IntFunctionId, expression: IntExpr) -> Self {
        Self::int_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn int_body(runtime_id: IntFunctionId, body: IntReturn) -> Self {
        Self {
            kind: ReturnExprKind::Int { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn float(runtime_id: FloatFunctionId, expression: FloatExpr) -> Self {
        Self::float_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn float_body(runtime_id: FloatFunctionId, body: FloatReturn) -> Self {
        Self {
            kind: ReturnExprKind::Float { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn string(runtime_id: StringFunctionId, expression: StringExpr) -> Self {
        Self::string_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn string_body(runtime_id: StringFunctionId, body: StringReturn) -> Self {
        Self {
            kind: ReturnExprKind::String { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bit_array(runtime_id: BitArrayFunctionId, expression: BitArrayExpr) -> Self {
        Self::bit_array_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn bit_array_body(runtime_id: BitArrayFunctionId, body: BitArrayReturn) -> Self {
        Self {
            kind: ReturnExprKind::BitArray { runtime_id, body },
        }
    }

    pub(crate) fn utf_codepoint_body(
        runtime_id: UtfCodepointFunctionId,
        body: UtfCodepointReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepoint { runtime_id, body },
        }
    }

    pub(crate) fn custom_body(runtime_index: usize, body: CustomReturn) -> Self {
        let runtime_id = CustomFunctionId::new(runtime_index, body.type_().clone());
        Self {
            kind: ReturnExprKind::Custom { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool(runtime_id: BoolFunctionId, expression: BoolExpr) -> Self {
        Self::bool_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn bool_body(runtime_id: BoolFunctionId, body: BoolReturn) -> Self {
        Self {
            kind: ReturnExprKind::Bool { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil(runtime_id: NilFunctionId, expression: NilExpr) -> Self {
        Self::nil_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn nil_body(runtime_id: NilFunctionId, body: NilReturn) -> Self {
        Self {
            kind: ReturnExprKind::Nil { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn tuple(runtime_id: TupleFunctionId, expression: TupleExpr) -> Self {
        let type_ = expression.type_().to_vec();
        Self::tuple_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_body(
        runtime_id: TupleFunctionId,
        type_: Vec<ValueType>,
        body: TupleReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::Tuple {
                runtime_id,
                type_,
                body,
            },
        }
    }

    pub(crate) fn int_list_body(runtime_id: IntListFunctionId, body: IntListReturn) -> Self {
        Self {
            kind: ReturnExprKind::IntList { runtime_id, body },
        }
    }

    pub(crate) fn string_list_body(
        runtime_id: StringListFunctionId,
        body: StringListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::StringList { runtime_id, body },
        }
    }

    pub(crate) fn bit_array_list_body(
        runtime_id: BitArrayListFunctionId,
        body: BitArrayListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BitArrayList { runtime_id, body },
        }
    }

    pub(crate) fn utf_codepoint_list_body(
        runtime_id: UtfCodepointListFunctionId,
        body: UtfCodepointListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepointList { runtime_id, body },
        }
    }

    pub(crate) fn custom_list_body(
        runtime_id: CustomListFunctionId,
        item_type: CustomType,
        body: CustomListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::CustomList {
                runtime_id,
                item_type,
                body,
            },
        }
    }

    pub(crate) fn float_list_body(runtime_id: FloatListFunctionId, body: FloatListReturn) -> Self {
        Self {
            kind: ReturnExprKind::FloatList { runtime_id, body },
        }
    }

    pub(crate) fn bool_list_body(runtime_id: BoolListFunctionId, body: BoolListReturn) -> Self {
        Self {
            kind: ReturnExprKind::BoolList { runtime_id, body },
        }
    }

    pub(crate) fn nil_list_body(runtime_id: NilListFunctionId, body: NilListReturn) -> Self {
        Self {
            kind: ReturnExprKind::NilList { runtime_id, body },
        }
    }

    pub(crate) fn tuple_list_body(
        runtime_id: TupleListFunctionId,
        item_type: Vec<ValueType>,
        body: TupleListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::TupleList {
                runtime_id,
                item_type,
                body,
            },
        }
    }

    pub(crate) fn list_list_body(
        runtime_id: ListListFunctionId,
        item_type: Box<ValueType>,
        body: ListListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::ListList {
                runtime_id,
                item_type,
                body,
            },
        }
    }

    pub(crate) fn function_list_body(
        runtime_id: FunctionListFunctionId,
        item_type: FunctionType,
        body: FunctionListReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FunctionList {
                runtime_id,
                item_type,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function(
        runtime_id: IntFunctionFunctionId,
        expression: super::IntFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::int_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn int_function_body(
        runtime_id: IntFunctionFunctionId,
        type_: FunctionType,
        body: IntFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::IntFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn float_function(
        runtime_id: FloatFunctionFunctionId,
        expression: super::FloatFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::float_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn float_function_body(
        runtime_id: FloatFunctionFunctionId,
        type_: FunctionType,
        body: FloatFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::FloatFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn string_function(
        runtime_id: StringFunctionFunctionId,
        expression: super::StringFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::string_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn string_function_body(
        runtime_id: StringFunctionFunctionId,
        type_: FunctionType,
        body: StringFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::StringFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn bit_array_function(
        runtime_id: BitArrayFunctionFunctionId,
        expression: super::BitArrayFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::bit_array_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn bit_array_function_body(
        runtime_id: BitArrayFunctionFunctionId,
        type_: FunctionType,
        body: BitArrayFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BitArrayFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    pub(crate) fn utf_codepoint_function_body(
        runtime_id: UtfCodepointFunctionFunctionId,
        type_: FunctionType,
        body: UtfCodepointFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::UtfCodepointFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    pub(crate) fn custom_function_body(runtime_index: usize, body: CustomFunctionReturn) -> Self {
        let runtime_id = CustomFunctionFunctionId::new(runtime_index, body.type_().clone());
        Self {
            kind: ReturnExprKind::CustomFunction { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn bool_function(
        runtime_id: BoolFunctionFunctionId,
        expression: super::BoolFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::bool_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn bool_function_body(
        runtime_id: BoolFunctionFunctionId,
        type_: FunctionType,
        body: BoolFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::BoolFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn nil_function(
        runtime_id: NilFunctionFunctionId,
        expression: super::NilFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::nil_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn nil_function_body(
        runtime_id: NilFunctionFunctionId,
        type_: FunctionType,
        body: NilFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::NilFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn tuple_function(
        runtime_id: TupleFunctionFunctionId,
        expression: super::TupleFunctionExpr,
    ) -> Self {
        let type_ = expression.type_().clone();
        Self::tuple_function_body(runtime_id, type_, ReturnBody::expr(expression))
    }

    pub(crate) fn tuple_function_body(
        runtime_id: TupleFunctionFunctionId,
        type_: FunctionType,
        body: TupleFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::TupleFunction {
                runtime_id,
                type_,
                body,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn list_function(
        runtime_id: ListFunctionFunctionId,
        expression: super::ListFunctionExpr,
    ) -> Self {
        Self::list_function_body(runtime_id, ReturnBody::expr(expression))
    }

    pub(crate) fn list_function_body(
        runtime_id: ListFunctionFunctionId,
        body: ListFunctionReturn,
    ) -> Self {
        Self {
            kind: ReturnExprKind::ListFunction { runtime_id, body },
        }
    }

    #[cfg(test)]
    pub(crate) fn function_function(
        runtime_index: usize,
        expression: super::FunctionFunctionExpr,
    ) -> Self {
        Self::function_function_body(runtime_index, FunctionFunctionReturn::expr(expression))
    }

    pub(crate) fn function_function_body(
        runtime_index: usize,
        body: FunctionFunctionReturn,
    ) -> Self {
        let runtime_id = FunctionFunctionFunctionId::new(runtime_index, body.type_().clone());
        Self {
            kind: ReturnExprKind::FunctionFunction { runtime_id, body },
        }
    }

    pub(crate) fn kind(&self) -> &ReturnExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> ReturnExprKind {
        self.kind
    }

    pub fn value_type(&self) -> ValueType {
        match self.kind() {
            ReturnExprKind::Int { .. } => ValueType::Int,
            ReturnExprKind::Float { .. } => ValueType::Float,
            ReturnExprKind::String { .. } => ValueType::String,
            ReturnExprKind::BitArray { .. } => ValueType::BitArray,
            ReturnExprKind::UtfCodepoint { .. } => ValueType::UtfCodepoint,
            ReturnExprKind::Custom { runtime_id, .. } => {
                ValueType::Custom(runtime_id.return_type().clone())
            }
            ReturnExprKind::Bool { .. } => ValueType::Bool,
            ReturnExprKind::Nil { .. } => ValueType::Nil,
            ReturnExprKind::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
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
            ReturnExprKind::ListList { item_type, .. } => {
                ValueType::List(Box::new(ValueType::List(item_type.clone())))
            }
            ReturnExprKind::FunctionList { item_type, .. } => {
                ValueType::List(Box::new(ValueType::Function(Box::new(item_type.clone()))))
            }
            ReturnExprKind::IntFunction { type_, .. }
            | ReturnExprKind::FloatFunction { type_, .. }
            | ReturnExprKind::StringFunction { type_, .. }
            | ReturnExprKind::BitArrayFunction { type_, .. }
            | ReturnExprKind::UtfCodepointFunction { type_, .. }
            | ReturnExprKind::BoolFunction { type_, .. }
            | ReturnExprKind::NilFunction { type_, .. }
            | ReturnExprKind::TupleFunction { type_, .. } => {
                ValueType::Function(Box::new(type_.clone()))
            }
            ReturnExprKind::CustomFunction { runtime_id, .. } => {
                ValueType::Function(Box::new(runtime_id.type_().to_function_type()))
            }
            ReturnExprKind::ListFunction { runtime_id, .. } => {
                ValueType::Function(Box::new(runtime_id.type_().clone()))
            }
            ReturnExprKind::FunctionFunction { runtime_id, .. } => {
                ValueType::Function(Box::new(runtime_id.type_().to_function_type()))
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        match self.kind() {
            ReturnExprKind::Int { runtime_id, .. } => RuntimeFunctionId::Int(*runtime_id),
            ReturnExprKind::Float { runtime_id, .. } => RuntimeFunctionId::Float(*runtime_id),
            ReturnExprKind::String { runtime_id, .. } => RuntimeFunctionId::String(*runtime_id),
            ReturnExprKind::BitArray { runtime_id, .. } => RuntimeFunctionId::BitArray(*runtime_id),
            ReturnExprKind::UtfCodepoint { runtime_id, .. } => {
                RuntimeFunctionId::UtfCodepoint(*runtime_id)
            }
            ReturnExprKind::Custom { runtime_id, .. } => {
                RuntimeFunctionId::Custom(runtime_id.clone())
            }
            ReturnExprKind::Bool { runtime_id, .. } => RuntimeFunctionId::Bool(*runtime_id),
            ReturnExprKind::Nil { runtime_id, .. } => RuntimeFunctionId::Nil(*runtime_id),
            ReturnExprKind::Tuple {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Tuple {
                id: *runtime_id,
                return_type: type_.clone(),
            },
            ReturnExprKind::IntList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::Int(*runtime_id))
            }
            ReturnExprKind::StringList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::String(*runtime_id))
            }
            ReturnExprKind::BitArrayList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::BitArray(*runtime_id))
            }
            ReturnExprKind::UtfCodepointList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::UtfCodepoint(*runtime_id))
            }
            ReturnExprKind::CustomList {
                runtime_id,
                item_type,
                ..
            } => RuntimeFunctionId::List(ListFunctionId::Custom {
                id: *runtime_id,
                item_type: item_type.clone(),
            }),
            ReturnExprKind::FloatList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::Float(*runtime_id))
            }
            ReturnExprKind::BoolList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::Bool(*runtime_id))
            }
            ReturnExprKind::NilList { runtime_id, .. } => {
                RuntimeFunctionId::List(ListFunctionId::Nil(*runtime_id))
            }
            ReturnExprKind::TupleList {
                runtime_id,
                item_type,
                ..
            } => RuntimeFunctionId::List(ListFunctionId::Tuple {
                id: *runtime_id,
                item_type: item_type.clone(),
            }),
            ReturnExprKind::ListList {
                runtime_id,
                item_type,
                ..
            } => RuntimeFunctionId::List(ListFunctionId::List {
                id: *runtime_id,
                item_type: item_type.clone(),
            }),
            ReturnExprKind::FunctionList {
                runtime_id,
                item_type,
                ..
            } => RuntimeFunctionId::List(ListFunctionId::Function {
                id: *runtime_id,
                item_type: item_type.clone(),
            }),
            ReturnExprKind::IntFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::FloatFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Float(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::StringFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::BitArrayFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::BitArray(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::UtfCodepointFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::UtfCodepoint(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::CustomFunction { runtime_id, .. } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Custom(runtime_id.clone()),
                return_type: runtime_id.type_().to_function_type(),
            },
            ReturnExprKind::BoolFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::NilFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::TupleFunction {
                runtime_id, type_, ..
            } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Tuple(*runtime_id),
                return_type: type_.clone(),
            },
            ReturnExprKind::ListFunction { runtime_id, .. } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(runtime_id.clone()),
                return_type: runtime_id.type_().clone(),
            },
            ReturnExprKind::FunctionFunction { runtime_id, .. } => RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(runtime_id.clone()),
                return_type: runtime_id.type_().to_function_type(),
            },
        }
    }
}

impl CustomReturn {
    pub(crate) fn expr(expression: CustomExpr) -> Self {
        let (type_, kind) = expression.into_parts();
        Self {
            type_,
            body: custom_return_body(kind),
        }
    }

    #[cfg(test)]
    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            type_: return_.type_,
            body: ReturnBody::block(steps, return_.body),
        }
    }

    pub(crate) fn type_(&self) -> &CustomType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &ReturnBody<super::CustomExprKind, usize> {
        &self.body
    }

    pub(crate) fn into_parts(self) -> (CustomType, ReturnBody<super::CustomExprKind, usize>) {
        (self.type_, self.body)
    }
}

fn custom_return_body(kind: super::CustomExprKind) -> ReturnBody<super::CustomExprKind, usize> {
    use super::CustomExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function.index(), args),
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

    pub(crate) fn kind(&self) -> &ReturnBodyKind<super::CustomFunctionExprKind, usize> {
        self.body.kind()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CustomFunctionType,
        ReturnBody<super::CustomFunctionExprKind, usize>,
    ) {
        (self.type_, self.body)
    }
}

fn custom_function_return_body(
    kind: super::CustomFunctionExprKind,
) -> ReturnBody<super::CustomFunctionExprKind, usize> {
    use super::CustomFunctionExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function.index(), args),
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
    pub(crate) fn tail_call(function: FunctionFunctionFunctionId, args: Vec<CallArg>) -> Self {
        Self {
            type_: function.type_().clone(),
            body: ReturnBody::tail_call(function.index(), args),
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

    pub(crate) fn kind(&self) -> &ReturnBodyKind<super::FunctionFunctionExprKind, usize> {
        self.body.kind()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        FunctionFunctionType,
        ReturnBody<super::FunctionFunctionExprKind, usize>,
    ) {
        (self.type_, self.body)
    }
}

fn function_function_return_body(
    kind: super::FunctionFunctionExprKind,
) -> ReturnBody<super::FunctionFunctionExprKind, usize> {
    use super::FunctionFunctionExprKind as K;

    match kind {
        K::Call { function, args } => ReturnBody::tail_call(function.index(), args),
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

    pub(crate) fn into_kind(self) -> ReturnBodyKind<Expression, Function> {
        self.kind
    }
}

impl Param {
    pub(crate) fn named(local: ParamLocal, name: EcoString) -> Self {
        Self {
            local,
            binding: ParamBinding::Named(name),
        }
    }

    pub(crate) fn discard(local: ParamLocal) -> Self {
        Self {
            local,
            binding: ParamBinding::Discard,
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
}

impl ParamLocal {
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

    pub(crate) fn custom(local: CustomLocalId, type_: CustomType) -> Self {
        Self::Custom(CustomLocal::new(local, type_))
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

    pub(crate) fn value_type(&self) -> ValueType {
        match self {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListReturn, BoolListReturn, CustomFunctionReturn, CustomListReturn, CustomReturn,
        FloatListReturn, FunctionFunctionReturn, FunctionListReturn, FunctionPlan, IntListReturn,
        ListListReturn, ListReturn, NilListReturn, Param, ParamBinding, ParamLocal, ReturnBody,
        ReturnBodyKind, ReturnExpr, StringListReturn, TupleListReturn,
    };
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
        BitArrayFunctionReference, BitArrayListFunctionId, BoolExpr, BoolFunctionExpr,
        BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId, BoolFunctionReference,
        BoolLocalId, CustomExpr, CustomFunctionExpr, CustomFunctionFunctionId, CustomFunctionId,
        CustomFunctionReference, CustomFunctionType, CustomListFunctionId, CustomLocalId,
        CustomType, CustomTypeName, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId,
        FloatFunctionId, FloatFunctionLocalId, FloatFunctionReference, FloatListFunctionId,
        FloatLocalId, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionReference,
        FunctionFunctionType, FunctionId, FunctionListFunctionId, FunctionType, IntExpr,
        IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId,
        IntFunctionReference, IntListFunctionId, IntListLocalId, IntLocalId, ListExpr,
        ListFunctionExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionReference,
        ListListFunctionId, ListLocal, NilExpr, NilFunctionExpr, NilFunctionFunctionId,
        NilFunctionId, NilFunctionLocalId, NilFunctionReference, NilListFunctionId,
        RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionFunctionId,
        StringFunctionId, StringFunctionLocalId, StringFunctionReference, StringListFunctionId,
        TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
        TupleFunctionLocalId, TupleFunctionReference, TupleListFunctionId, UtfCodepointExpr,
        UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
        UtfCodepointFunctionReference, UtfCodepointFunctionReturn, UtfCodepointListFunctionId,
        UtfCodepointListReturn, UtfCodepointLocalId, UtfCodepointReturn, ValueType,
    };
    use num_bigint::BigInt;

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn callable_returns_own_exact_type_around_itemless_bodies() {
        let custom_function_type = CustomFunctionType::new(vec![ValueType::Int], custom_type());
        let custom_return = CustomFunctionReturn::expr(CustomFunctionExpr::block(
            Vec::new(),
            CustomFunctionExpr::call(
                CustomFunctionFunctionId::new(7, custom_function_type.clone()),
                Vec::new(),
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
                                function: 7,
                                args: Vec::new(),
                            },
                        }),
                    },
                },
            },
        );

        let returned = FunctionType::new(vec![ValueType::String], ValueType::Int);
        let function_function_type = FunctionFunctionType::new(vec![ValueType::Bool], returned);
        let function_return = FunctionFunctionReturn::expr(FunctionFunctionExpr::call(
            FunctionFunctionFunctionId::new(9, function_function_type.clone()),
            Vec::new(),
        ));

        assert_eq!(
            function_return,
            FunctionFunctionReturn {
                type_: function_function_type,
                body: ReturnBody {
                    kind: ReturnBodyKind::TailCall {
                        function: 9,
                        args: Vec::new(),
                    },
                },
            },
        );
    }

    #[test]
    fn custom_returns_own_one_type_around_itemless_tail_calls() {
        let type_ = custom_type();
        let function = CustomFunctionId::new(7, type_.clone());
        let body = CustomReturn::expr(CustomExpr::block(
            Vec::new(),
            CustomExpr::bool_case(
                BoolExpr::value(true),
                CustomExpr::call(function.clone(), Vec::new()),
                CustomExpr::call(function, Vec::new()),
            ),
        ));

        assert_eq!(
            body,
            CustomReturn {
                type_,
                body: ReturnBody {
                    kind: ReturnBodyKind::Block {
                        steps: Vec::new(),
                        return_: Box::new(ReturnBody {
                            kind: ReturnBodyKind::BoolCase {
                                subject: BoolExpr::value(true),
                                true_: Box::new(ReturnBody {
                                    kind: ReturnBodyKind::TailCall {
                                        function: 7,
                                        args: Vec::new(),
                                    },
                                }),
                                false_: Box::new(ReturnBody {
                                    kind: ReturnBodyKind::TailCall {
                                        function: 7,
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
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1)));
        let function = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            vec![param],
            Vec::new(),
            return_,
        );

        assert_eq!(function.id(), FunctionId::new(0));
        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 1);
        assert_eq!(function.params()[0].name(), Some(&"x".into()));
        assert_eq!(function.steps(), &[]);
        assert_eq!(
            function.return_(),
            &ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1)))
        );
        assert_eq!(function.frame_layout().ints(), 1);
    }

    #[test]
    fn return_expr_value_type() {
        assert_eq!(
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(BigInt::from(1))).value_type(),
            ValueType::Int,
        );
        assert_eq!(
            ReturnExpr::string(
                crate::plan::StringFunctionId(0),
                StringExpr::value("geam".into()),
            )
            .value_type(),
            ValueType::String,
        );
        assert_eq!(
            ReturnExpr::bit_array(BitArrayFunctionId(0), BitArrayExpr::value(Vec::new()))
                .value_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            ReturnExpr::utf_codepoint_body(
                UtfCodepointFunctionId(0),
                UtfCodepointReturn::expr(UtfCodepointExpr::local_get(
                    UtfCodepointLocalId(0),
                    "codepoint".into(),
                )),
            )
            .value_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(1.0)).value_type(),
            ValueType::Float,
        );
        assert_eq!(
            ReturnExpr::bool(crate::plan::BoolFunctionId(0), BoolExpr::value(true)).value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ReturnExpr::nil(crate::plan::NilFunctionId(0), NilExpr::value()).value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ReturnExpr::tuple(
                TupleFunctionId(0),
                TupleExpr::value(
                    vec![crate::plan::Expr::int(IntExpr::value(BigInt::from(1)))],
                    vec![ValueType::Int],
                ),
            )
            .value_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ReturnExpr::int_list_body(
                IntListFunctionId(0),
                IntListReturn::expr(
                    ListExpr::value(
                        vec![crate::plan::Expr::int(IntExpr::value(BigInt::from(1)))],
                        ValueType::Int,
                    )
                    .into_int()
                    .expect("expression should be List(Int)"),
                ),
            )
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            ReturnExpr::int_function(
                IntFunctionFunctionId(0),
                IntFunctionExpr::reference(IntFunctionReference::new(IntFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        assert_eq!(
            ReturnExpr::string_function(
                StringFunctionFunctionId(0),
                StringFunctionExpr::reference(StringFunctionReference::new(
                    StringFunctionId(0),
                    Vec::new(),
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
        );
        assert_eq!(
            ReturnExpr::bit_array_function(
                BitArrayFunctionFunctionId(0),
                BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
                    BitArrayFunctionId(0),
                    Vec::new(),
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(
                FunctionType::new(Vec::new(), ValueType::BitArray,)
            )),
        );
        assert_eq!(
            ReturnExpr::utf_codepoint_function_body(
                UtfCodepointFunctionFunctionId(0),
                FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                UtfCodepointFunctionReturn::expr(UtfCodepointFunctionExpr::reference(
                    UtfCodepointFunctionReference::new(UtfCodepointFunctionId(0), Vec::new()),
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::UtfCodepoint,
            ))),
        );
        assert_eq!(
            ReturnExpr::float_function(
                FloatFunctionFunctionId(0),
                FloatFunctionExpr::reference(FloatFunctionReference::new(
                    FloatFunctionId(0),
                    Vec::new()
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float))),
        );
        assert_eq!(
            ReturnExpr::bool_function(
                BoolFunctionFunctionId(0),
                BoolFunctionExpr::reference(BoolFunctionReference::new(
                    BoolFunctionId(0),
                    Vec::new()
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        );
        assert_eq!(
            ReturnExpr::nil_function(
                NilFunctionFunctionId(0),
                NilFunctionExpr::reference(NilFunctionReference::new(NilFunctionId(0), Vec::new())),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
        );
        assert_eq!(
            ReturnExpr::tuple_function(
                TupleFunctionFunctionId(0),
                TupleFunctionExpr::reference(
                    TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                    vec![ValueType::Int],
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            ))),
        );
        assert_eq!(
            ReturnExpr::list_function(
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                ),
                ListFunctionExpr::reference(ListFunctionReference::new(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new()
                )),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ))),
        );
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            ReturnExpr::function_function(
                0,
                FunctionFunctionExpr::reference(
                    FunctionFunctionReference::new(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::new(),
                    ),
                    return_type.clone(),
                ),
            )
            .value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(return_type)),
            ))),
        );
    }

    #[test]
    fn list_return_expr_preserves_value_type_and_runtime_id_by_item_family() {
        let tuple_item = vec![ValueType::Int];
        let list_item = Box::new(ValueType::Int);
        let function_item = FunctionType::new(Vec::new(), ValueType::Int);
        let custom_item = custom_type();
        let expressions = vec![
            (
                ReturnExpr::int_list_body(
                    IntListFunctionId(1),
                    IntListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Int)
                            .into_int()
                            .expect("expression should be List(Int)"),
                    ),
                ),
                ValueType::Int,
                ListFunctionId::Int(IntListFunctionId(1)),
            ),
            (
                ReturnExpr::string_list_body(
                    StringListFunctionId(2),
                    StringListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::String)
                            .into_string()
                            .expect("expression should be List(String)"),
                    ),
                ),
                ValueType::String,
                ListFunctionId::String(StringListFunctionId(2)),
            ),
            (
                ReturnExpr::bit_array_list_body(
                    BitArrayListFunctionId(9),
                    BitArrayListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::BitArray)
                            .into_bit_array()
                            .expect("expression should be List(BitArray)"),
                    ),
                ),
                ValueType::BitArray,
                ListFunctionId::BitArray(BitArrayListFunctionId(9)),
            ),
            (
                ReturnExpr::utf_codepoint_list_body(
                    UtfCodepointListFunctionId(10),
                    UtfCodepointListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::UtfCodepoint)
                            .into_utf_codepoint()
                            .expect("expression should be List(UtfCodepoint)"),
                    ),
                ),
                ValueType::UtfCodepoint,
                ListFunctionId::UtfCodepoint(UtfCodepointListFunctionId(10)),
            ),
            (
                ReturnExpr::custom_list_body(
                    CustomListFunctionId(11),
                    custom_item.clone(),
                    CustomListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Custom(custom_item.clone()))
                            .into_custom()
                            .expect("expression should be List(Custom)"),
                    ),
                ),
                ValueType::Custom(custom_item.clone()),
                ListFunctionId::Custom {
                    id: CustomListFunctionId(11),
                    item_type: custom_item,
                },
            ),
            (
                ReturnExpr::float_list_body(
                    FloatListFunctionId(3),
                    FloatListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Float)
                            .into_float()
                            .expect("expression should be List(Float)"),
                    ),
                ),
                ValueType::Float,
                ListFunctionId::Float(FloatListFunctionId(3)),
            ),
            (
                ReturnExpr::bool_list_body(
                    crate::plan::BoolListFunctionId(4),
                    BoolListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Bool)
                            .into_bool()
                            .expect("expression should be List(Bool)"),
                    ),
                ),
                ValueType::Bool,
                ListFunctionId::Bool(crate::plan::BoolListFunctionId(4)),
            ),
            (
                ReturnExpr::nil_list_body(
                    NilListFunctionId(5),
                    NilListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Nil)
                            .into_nil()
                            .expect("expression should be List(Nil)"),
                    ),
                ),
                ValueType::Nil,
                ListFunctionId::Nil(NilListFunctionId(5)),
            ),
            (
                ReturnExpr::tuple_list_body(
                    TupleListFunctionId(6),
                    tuple_item.clone(),
                    TupleListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::Tuple(tuple_item.clone()))
                            .into_tuple()
                            .expect("expression should be List(Tuple)"),
                    ),
                ),
                ValueType::Tuple(tuple_item.clone()),
                ListFunctionId::Tuple {
                    id: TupleListFunctionId(6),
                    item_type: tuple_item,
                },
            ),
            (
                ReturnExpr::list_list_body(
                    ListListFunctionId(7),
                    list_item.clone(),
                    ListListReturn::expr(
                        ListExpr::value(Vec::new(), ValueType::List(list_item.clone()))
                            .into_list()
                            .expect("expression should be List(List)"),
                    ),
                ),
                ValueType::List(list_item.clone()),
                ListFunctionId::List {
                    id: ListListFunctionId(7),
                    item_type: list_item,
                },
            ),
            (
                ReturnExpr::function_list_body(
                    FunctionListFunctionId(8),
                    function_item.clone(),
                    FunctionListReturn::expr(
                        ListExpr::value(
                            Vec::new(),
                            ValueType::Function(Box::new(function_item.clone())),
                        )
                        .into_function()
                        .expect("expression should be List(Function)"),
                    ),
                ),
                ValueType::Function(Box::new(function_item.clone())),
                ListFunctionId::Function {
                    id: FunctionListFunctionId(8),
                    item_type: function_item,
                },
            ),
        ];

        for (expression, item_type, runtime_id) in expressions {
            assert_eq!(
                expression.value_type(),
                ValueType::List(Box::new(item_type)),
            );
            assert_eq!(expression.runtime_id(), RuntimeFunctionId::List(runtime_id),);
        }
    }

    #[test]
    fn return_expr_preserves_runtime_id_for_non_list_families() {
        let tuple_type = vec![ValueType::Int];
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let custom_type = custom_type();
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type.clone());
        let expressions = vec![
            (
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(1.into())),
                RuntimeFunctionId::Int(IntFunctionId(0)),
            ),
            (
                ReturnExpr::float(FloatFunctionId(1), FloatExpr::value(1.0)),
                RuntimeFunctionId::Float(FloatFunctionId(1)),
            ),
            (
                ReturnExpr::string(StringFunctionId(2), StringExpr::value("value".into())),
                RuntimeFunctionId::String(StringFunctionId(2)),
            ),
            (
                ReturnExpr::bit_array(BitArrayFunctionId(11), BitArrayExpr::value(Vec::new())),
                RuntimeFunctionId::BitArray(BitArrayFunctionId(11)),
            ),
            (
                ReturnExpr::utf_codepoint_body(
                    UtfCodepointFunctionId(12),
                    UtfCodepointReturn::expr(UtfCodepointExpr::local_get(
                        UtfCodepointLocalId(0),
                        "codepoint".into(),
                    )),
                ),
                RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(12)),
            ),
            (
                ReturnExpr::custom_body(
                    14,
                    CustomReturn::expr(CustomExpr::local_get(
                        crate::plan::CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                        "custom".into(),
                    )),
                ),
                RuntimeFunctionId::Custom(CustomFunctionId::new(14, custom_type.clone())),
            ),
            (
                ReturnExpr::bool(BoolFunctionId(3), BoolExpr::value(true)),
                RuntimeFunctionId::Bool(BoolFunctionId(3)),
            ),
            (
                ReturnExpr::nil(NilFunctionId(4), NilExpr::value()),
                RuntimeFunctionId::Nil(NilFunctionId(4)),
            ),
            (
                ReturnExpr::tuple(
                    TupleFunctionId(5),
                    TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        tuple_type.clone(),
                    ),
                ),
                RuntimeFunctionId::Tuple {
                    id: TupleFunctionId(5),
                    return_type: tuple_type,
                },
            ),
            (
                ReturnExpr::int_function(
                    IntFunctionFunctionId(6),
                    IntFunctionExpr::reference(IntFunctionReference::new(
                        IntFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(6)),
                    return_type: int_function_type.clone(),
                },
            ),
            (
                ReturnExpr::float_function(
                    FloatFunctionFunctionId(7),
                    FloatFunctionExpr::reference(FloatFunctionReference::new(
                        FloatFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Float(FloatFunctionFunctionId(7)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Float),
                },
            ),
            (
                ReturnExpr::string_function(
                    StringFunctionFunctionId(8),
                    StringFunctionExpr::reference(StringFunctionReference::new(
                        StringFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::String(StringFunctionFunctionId(8)),
                    return_type: FunctionType::new(Vec::new(), ValueType::String),
                },
            ),
            (
                ReturnExpr::bit_array_function(
                    BitArrayFunctionFunctionId(12),
                    BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
                        BitArrayFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(12)),
                    return_type: FunctionType::new(Vec::new(), ValueType::BitArray),
                },
            ),
            (
                ReturnExpr::utf_codepoint_function_body(
                    UtfCodepointFunctionFunctionId(13),
                    FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                    UtfCodepointFunctionReturn::expr(UtfCodepointFunctionExpr::reference(
                        UtfCodepointFunctionReference::new(UtfCodepointFunctionId(0), Vec::new()),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(13)),
                    return_type: FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                },
            ),
            (
                ReturnExpr::custom_function_body(
                    14,
                    CustomFunctionReturn::expr(CustomFunctionExpr::reference(
                        CustomFunctionReference::new(
                            CustomFunctionId::new(0, custom_type),
                            Vec::new(),
                        ),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Custom(CustomFunctionFunctionId::new(
                        14,
                        custom_function_type.clone(),
                    )),
                    return_type: custom_function_type.to_function_type(),
                },
            ),
            (
                ReturnExpr::bool_function(
                    BoolFunctionFunctionId(9),
                    BoolFunctionExpr::reference(BoolFunctionReference::new(
                        BoolFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Bool(BoolFunctionFunctionId(9)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Bool),
                },
            ),
            (
                ReturnExpr::nil_function(
                    NilFunctionFunctionId(10),
                    NilFunctionExpr::reference(NilFunctionReference::new(
                        NilFunctionId(0),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Nil(NilFunctionFunctionId(10)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Nil),
                },
            ),
            (
                ReturnExpr::tuple_function(
                    TupleFunctionFunctionId(11),
                    TupleFunctionExpr::reference(
                        TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                        vec![ValueType::Int],
                    ),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Tuple(TupleFunctionFunctionId(11)),
                    return_type: FunctionType::new(
                        Vec::new(),
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                },
            ),
            (
                ReturnExpr::list_function(
                    ListFunctionFunctionId::from_item_type(
                        12,
                        FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                        ValueType::Int,
                    ),
                    ListFunctionExpr::reference(ListFunctionReference::new(
                        ListFunctionId::Int(IntListFunctionId(0)),
                        Vec::new(),
                    )),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                        12,
                        FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                        ValueType::Int,
                    )),
                    return_type: FunctionType::new(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                },
            ),
            (
                ReturnExpr::function_function(
                    13,
                    FunctionFunctionExpr::reference(
                        FunctionFunctionReference::new(
                            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                            Vec::new(),
                        ),
                        int_function_type.clone(),
                    ),
                ),
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Function(FunctionFunctionFunctionId::new(
                        13,
                        FunctionFunctionType::new(Vec::new(), int_function_type.clone()),
                    )),
                    return_type: FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(int_function_type)),
                    ),
                },
            ),
        ];

        for (expression, runtime_id) in expressions {
            assert_eq!(expression.runtime_id(), runtime_id);
        }
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
                item_type: Box::new(ValueType::Int),
                body: ListListReturn::expr(nested.into_list().expect("nested list")),
            },
        );

        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function = ListExpr::value(
            vec![crate::plan::Expr::function(
                crate::plan::FunctionExpr::reference(crate::plan::FunctionReference::new(
                    crate::plan::RuntimeFunctionId::Int(IntFunctionId(0)),
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
        assert_eq!(
            ListReturn::tail_call(ListFunctionId::Int(IntListFunctionId(0)), Vec::new()),
            ListReturn::Int(IntListReturn::tail_call(IntListFunctionId(0), Vec::new())),
        );
        assert_eq!(
            ListReturn::tail_call(ListFunctionId::Float(FloatListFunctionId(0)), Vec::new()),
            ListReturn::Float(FloatListReturn::tail_call(
                FloatListFunctionId(0),
                Vec::new()
            )),
        );
        assert_eq!(
            ListReturn::tail_call(ListFunctionId::String(StringListFunctionId(0)), Vec::new()),
            ListReturn::String(StringListReturn::tail_call(
                StringListFunctionId(0),
                Vec::new(),
            )),
        );
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::BitArray(BitArrayListFunctionId(0)),
                Vec::new(),
            ),
            ListReturn::BitArray(BitArrayListReturn::tail_call(
                BitArrayListFunctionId(0),
                Vec::new(),
            )),
        );
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::UtfCodepoint(UtfCodepointListFunctionId(0)),
                Vec::new(),
            ),
            ListReturn::UtfCodepoint(UtfCodepointListReturn::tail_call(
                UtfCodepointListFunctionId(0),
                Vec::new(),
            )),
        );
        let custom_type = custom_type();
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::Custom {
                    id: CustomListFunctionId(0),
                    item_type: custom_type.clone(),
                },
                Vec::new(),
            ),
            ListReturn::Custom {
                item_type: custom_type,
                body: CustomListReturn::tail_call(CustomListFunctionId(0), Vec::new()),
            },
        );
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::Bool(crate::plan::BoolListFunctionId(0)),
                Vec::new()
            ),
            ListReturn::Bool(BoolListReturn::tail_call(
                crate::plan::BoolListFunctionId(0),
                Vec::new(),
            )),
        );
        assert_eq!(
            ListReturn::tail_call(ListFunctionId::Nil(NilListFunctionId(0)), Vec::new()),
            ListReturn::Nil(NilListReturn::tail_call(NilListFunctionId(0), Vec::new())),
        );
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::Tuple {
                    id: TupleListFunctionId(0),
                    item_type: vec![ValueType::Int],
                },
                Vec::new(),
            ),
            ListReturn::Tuple {
                item_type: vec![ValueType::Int],
                body: TupleListReturn::tail_call(TupleListFunctionId(0), Vec::new()),
            },
        );
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::List {
                    id: ListListFunctionId(0),
                    item_type: Box::new(ValueType::Int),
                },
                Vec::new(),
            ),
            ListReturn::List {
                item_type: Box::new(ValueType::Int),
                body: ListListReturn::tail_call(ListListFunctionId(0), Vec::new()),
            },
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            ListReturn::tail_call(
                ListFunctionId::Function {
                    id: FunctionListFunctionId(0),
                    item_type: function_type.clone(),
                },
                Vec::new(),
            ),
            ListReturn::Function {
                item_type: function_type,
                body: FunctionListReturn::tail_call(FunctionListFunctionId(0), Vec::new()),
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
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
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
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
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
            ListReturn::Int(_) => ValueType::Int,
            ListReturn::Float(_) => ValueType::Float,
            ListReturn::String(_) => ValueType::String,
            ListReturn::BitArray(_) => ValueType::BitArray,
            ListReturn::UtfCodepoint(_) => ValueType::UtfCodepoint,
            ListReturn::Custom { item_type, .. } => ValueType::Custom(item_type.clone()),
            ListReturn::Bool(_) => ValueType::Bool,
            ListReturn::Nil(_) => ValueType::Nil,
            ListReturn::Tuple { item_type, .. } => ValueType::Tuple(item_type.clone()),
            ListReturn::List { item_type, .. } => ValueType::List(item_type.clone()),
            ListReturn::Function { item_type, .. } => {
                ValueType::Function(Box::new(item_type.clone()))
            }
        }
    }
}
