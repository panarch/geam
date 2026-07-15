use super::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    CustomLocalExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, ListExpr, ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr,
    StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr, TypedFunctionExpr,
    UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use crate::plan::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomFunctionLocalId, CustomLocalId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocal, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId,
    ListFunctionLocal, ListLocal, NilFunctionLocalId, NilLocalId, ParamLocal,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    kind: CallArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CallArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    Custom(CustomLocalExpr),
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List(ListLocalExpr),
    IntFunction {
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    kind: CaptureArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureArgKind {
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    Custom(CustomLocalExpr),
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List(ListLocalExpr),
    IntFunction {
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    },
}

impl Expr {
    pub(crate) fn into_call_arg(self, local: &ParamLocal) -> Option<CallArg> {
        match (local, self.kind) {
            (ParamLocal::Int(local), ExprKind::Int(value)) => Some(CallArg::int(*local, value)),
            (ParamLocal::String(local), ExprKind::String(value)) => {
                Some(CallArg::string(*local, value))
            }
            (ParamLocal::BitArray(local), ExprKind::BitArray(value)) => {
                Some(CallArg::bit_array(*local, value))
            }
            (ParamLocal::UtfCodepoint(local), ExprKind::UtfCodepoint(value)) => {
                Some(CallArg::utf_codepoint(*local, value))
            }
            (ParamLocal::Custom(local), ExprKind::Custom(value))
                if local.type_() == value.type_() =>
            {
                Some(CallArg::custom(CustomLocalExpr::from_parts(
                    local.clone(),
                    value,
                )))
            }
            (ParamLocal::Float(local), ExprKind::Float(value)) => {
                Some(CallArg::float(*local, value))
            }
            (ParamLocal::Bool(local), ExprKind::Bool(value)) => Some(CallArg::bool(*local, value)),
            (ParamLocal::Nil(local), ExprKind::Nil(value)) => Some(CallArg::nil(*local, value)),
            (
                ParamLocal::Tuple {
                    local,
                    type_: expected,
                },
                ExprKind::Tuple(value),
            ) if value.type_() == expected => Some(CallArg::tuple(*local, value)),
            (ParamLocal::List(ListLocal::Int(local)), ExprKind::List(ListExpr::Int(value))) => {
                Some(CallArg::list(ListLocalExpr::Int {
                    local: *local,
                    value,
                }))
            }
            (
                ParamLocal::List(ListLocal::String(local)),
                ExprKind::List(ListExpr::String(value)),
            ) => Some(CallArg::list(ListLocalExpr::String {
                local: *local,
                value,
            })),
            (
                ParamLocal::List(ListLocal::BitArray(local)),
                ExprKind::List(ListExpr::BitArray(value)),
            ) => Some(CallArg::list(ListLocalExpr::BitArray {
                local: *local,
                value,
            })),
            (
                ParamLocal::List(ListLocal::UtfCodepoint(local)),
                ExprKind::List(ListExpr::UtfCodepoint(value)),
            ) => Some(CallArg::list(ListLocalExpr::UtfCodepoint {
                local: *local,
                value,
            })),
            (
                ParamLocal::List(ListLocal::Custom { local, item_type }),
                ExprKind::List(ListExpr::Custom(value)),
            ) if value.item().item_type() == item_type.clone() => {
                Some(CallArg::list(ListLocalExpr::Custom {
                    local: *local,
                    item_type: item_type.clone(),
                    value,
                }))
            }
            (ParamLocal::List(ListLocal::Float(local)), ExprKind::List(ListExpr::Float(value))) => {
                Some(CallArg::list(ListLocalExpr::Float {
                    local: *local,
                    value,
                }))
            }
            (ParamLocal::List(ListLocal::Bool(local)), ExprKind::List(ListExpr::Bool(value))) => {
                Some(CallArg::list(ListLocalExpr::Bool {
                    local: *local,
                    value,
                }))
            }
            (ParamLocal::List(ListLocal::Nil(local)), ExprKind::List(ListExpr::Nil(value))) => {
                Some(CallArg::list(ListLocalExpr::Nil {
                    local: *local,
                    value,
                }))
            }
            (
                ParamLocal::List(ListLocal::Tuple { local, item_type }),
                ExprKind::List(ListExpr::Tuple(value)),
            ) if value.item().item_type() == item_type.clone() => {
                Some(CallArg::list(ListLocalExpr::Tuple {
                    local: *local,
                    item_type: item_type.clone(),
                    value,
                }))
            }
            (
                ParamLocal::List(ListLocal::List { local, item_type }),
                ExprKind::List(ListExpr::List(value)),
            ) if value.item().item_type() == item_type.clone() => {
                Some(CallArg::list(ListLocalExpr::List {
                    local: *local,
                    item_type: item_type.clone(),
                    value,
                }))
            }
            (
                ParamLocal::List(ListLocal::Function { local, item_type }),
                ExprKind::List(ListExpr::Function(value)),
            ) if value.item().item_type() == item_type.clone() => {
                Some(CallArg::list(ListLocalExpr::Function {
                    local: *local,
                    item_type: item_type.clone(),
                    value,
                }))
            }
            (
                ParamLocal::IntFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_int()
                .map(|value| CallArg::int_function_expr(*local, value)),
            (
                ParamLocal::StringFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_string()
                .map(|value| CallArg::string_function_expr(*local, value)),
            (
                ParamLocal::BitArrayFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_bit_array()
                .map(|value| CallArg::bit_array_function_expr(*local, value)),
            (
                ParamLocal::UtfCodepointFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_utf_codepoint()
                .map(|value| CallArg::utf_codepoint_function_expr(*local, value)),
            (ParamLocal::CustomFunction(local), ExprKind::Function(value))
                if value.type_() == local.type_().to_function_type() =>
            {
                value
                    .into_typed_custom()
                    .map(|value| CallArg::custom_function_expr(local.clone(), value))
            }
            (
                ParamLocal::FloatFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_float()
                .map(|value| CallArg::float_function_expr(*local, value)),
            (
                ParamLocal::BoolFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_bool()
                .map(|value| CallArg::bool_function_expr(*local, value)),
            (
                ParamLocal::NilFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_nil()
                .map(|value| CallArg::nil_function_expr(*local, value)),
            (
                ParamLocal::TupleFunction {
                    local,
                    type_: expected,
                },
                ExprKind::Function(value),
            ) if value.type_() == *expected => value
                .into_typed_tuple()
                .map(|value| CallArg::tuple_function_expr(*local, value)),
            (ParamLocal::ListFunction(local), ExprKind::Function(value))
                if value.type_() == *local.type_() =>
            {
                value
                    .into_typed_list()
                    .map(|value| CallArg::list_function_expr(local.clone(), value))
            }
            (ParamLocal::FunctionFunction(local), ExprKind::Function(value))
                if value.type_() == local.type_().to_function_type() =>
            {
                value
                    .into_typed_function()
                    .map(|value| CallArg::function_function_expr(local.clone(), value))
            }
            _ => None,
        }
    }
}

impl CallArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CallArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CallArgKind::String { local, value },
        }
    }

    pub(crate) fn bit_array(local: BitArrayLocalId, value: BitArrayExpr) -> Self {
        Self {
            kind: CallArgKind::BitArray { local, value },
        }
    }

    pub(crate) fn utf_codepoint(local: UtfCodepointLocalId, value: UtfCodepointExpr) -> Self {
        Self {
            kind: CallArgKind::UtfCodepoint { local, value },
        }
    }

    pub(crate) fn custom(binding: CustomLocalExpr) -> Self {
        Self {
            kind: CallArgKind::Custom(binding),
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CallArgKind::Float { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CallArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CallArgKind::Nil { local, value },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CallArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(value: ListLocalExpr) -> Self {
        Self {
            kind: CallArgKind::List(value),
        }
    }

    fn int_function_expr(
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::IntFunction { local, value },
        }
    }

    fn string_function_expr(
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::StringFunction { local, value },
        }
    }

    fn bit_array_function_expr(
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::BitArrayFunction { local, value },
        }
    }

    fn utf_codepoint_function_expr(
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::UtfCodepointFunction { local, value },
        }
    }

    fn custom_function_expr(
        local: CustomFunctionLocal,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::CustomFunction { local, value },
        }
    }

    fn float_function_expr(
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::FloatFunction { local, value },
        }
    }

    fn bool_function_expr(
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::BoolFunction { local, value },
        }
    }

    fn nil_function_expr(
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::NilFunction { local, value },
        }
    }

    fn tuple_function_expr(
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::TupleFunction { local, value },
        }
    }

    fn list_function_expr(
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::ListFunction { local, value },
        }
    }

    fn function_function_expr(
        local: FunctionFunctionLocal,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    ) -> Self {
        Self {
            kind: CallArgKind::FunctionFunction { local, value },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::int_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::string_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::bit_array_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn custom_function(local: CustomFunctionLocal, value: CustomFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::new(
            value.custom_function_type().argument_shapes().to_vec(),
            crate::plan::ValueShape::Custom(value.custom_function_type().return_().clone()),
        );
        Self::custom_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::float_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::bool_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::nil_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::tuple_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn list_function(local: ListFunctionLocal, value: ListFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::list_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn function_function(
        local: FunctionFunctionLocal,
        value: FunctionFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_());
        Self::function_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    pub(crate) fn kind(&self) -> &CallArgKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> CallArgKind {
        self.kind
    }
}

impl CaptureArg {
    pub(crate) fn int(local: IntLocalId, value: IntExpr) -> Self {
        Self {
            kind: CaptureArgKind::Int { local, value },
        }
    }

    pub(crate) fn string(local: StringLocalId, value: StringExpr) -> Self {
        Self {
            kind: CaptureArgKind::String { local, value },
        }
    }

    pub(crate) fn bit_array(local: BitArrayLocalId, value: BitArrayExpr) -> Self {
        Self {
            kind: CaptureArgKind::BitArray { local, value },
        }
    }

    pub(crate) fn utf_codepoint(local: UtfCodepointLocalId, value: UtfCodepointExpr) -> Self {
        Self {
            kind: CaptureArgKind::UtfCodepoint { local, value },
        }
    }

    pub(crate) fn custom(local: CustomLocalId, value: CustomExpr) -> Self {
        Self {
            kind: CaptureArgKind::Custom(CustomLocalExpr::from_value(local, value)),
        }
    }

    pub(crate) fn float(local: FloatLocalId, value: FloatExpr) -> Self {
        Self {
            kind: CaptureArgKind::Float { local, value },
        }
    }

    pub(crate) fn bool(local: BoolLocalId, value: BoolExpr) -> Self {
        Self {
            kind: CaptureArgKind::Bool { local, value },
        }
    }

    pub(crate) fn nil(local: NilLocalId, value: NilExpr) -> Self {
        Self {
            kind: CaptureArgKind::Nil { local, value },
        }
    }

    pub(crate) fn tuple(local: TupleLocalId, value: TupleExpr) -> Self {
        Self {
            kind: CaptureArgKind::Tuple { local, value },
        }
    }

    pub(crate) fn list(value: ListLocalExpr) -> Self {
        Self {
            kind: CaptureArgKind::List(value),
        }
    }

    pub(crate) fn int_function_expr(
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::IntFunction { local, value },
        }
    }

    pub(crate) fn string_function_expr(
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::StringFunction { local, value },
        }
    }

    pub(crate) fn bit_array_function_expr(
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::BitArrayFunction { local, value },
        }
    }

    pub(crate) fn utf_codepoint_function_expr(
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::UtfCodepointFunction { local, value },
        }
    }

    pub(crate) fn custom_function_expr(
        local: CustomFunctionLocalId,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    ) -> Self {
        let local =
            CustomFunctionLocal::new(local, value.expression().custom_function_type().clone());
        Self {
            kind: CaptureArgKind::CustomFunction { local, value },
        }
    }

    pub(crate) fn float_function_expr(
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::FloatFunction { local, value },
        }
    }

    pub(crate) fn bool_function_expr(
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::BoolFunction { local, value },
        }
    }

    pub(crate) fn nil_function_expr(
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::NilFunction { local, value },
        }
    }

    pub(crate) fn tuple_function_expr(
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::TupleFunction { local, value },
        }
    }

    pub(crate) fn list_function_expr(
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::ListFunction { local, value },
        }
    }

    pub(crate) fn function_function_expr(
        local: FunctionFunctionLocalId,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    ) -> Self {
        let local =
            FunctionFunctionLocal::new(local, value.expression().function_function_type().clone());
        Self {
            kind: CaptureArgKind::FunctionFunction { local, value },
        }
    }

    #[cfg(test)]
    pub(crate) fn int_function(local: IntFunctionLocalId, value: IntFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::int_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn string_function(local: StringFunctionLocalId, value: StringFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::string_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::bit_array_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::utf_codepoint_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn custom_function(local: CustomFunctionLocalId, value: CustomFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::new(
            value.custom_function_type().argument_shapes().to_vec(),
            crate::plan::ValueShape::Custom(value.custom_function_type().return_().clone()),
        );
        Self::custom_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn float_function(local: FloatFunctionLocalId, value: FloatFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::float_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn bool_function(local: BoolFunctionLocalId, value: BoolFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::bool_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn nil_function(local: NilFunctionLocalId, value: NilFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::nil_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn tuple_function(local: TupleFunctionLocalId, value: TupleFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::tuple_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn list_function(local: ListFunctionLocal, value: ListFunctionExpr) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::list_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn function_function(
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_());
        Self::function_function_expr(local, TypedFunctionExpr::new(shape, value))
    }

    pub(crate) fn kind(&self) -> &CaptureArgKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> CaptureArgKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{CallArg, CaptureArg, CaptureArgKind, CustomLocalExpr, TypedFunctionExpr};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionId, BitArrayFunctionLocalId,
        BitArrayFunctionReference, BitArrayListLocalId, BitArrayLocalId, BoolExpr,
        BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionReference,
        BoolListLocalId, BoolLocalId, CustomExpr, CustomFunctionExpr, CustomFunctionLocal,
        CustomFunctionLocalId, CustomFunctionType, CustomLocal, CustomLocalId, CustomType,
        CustomTypeName, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId,
        FloatFunctionReference, FloatListLocalId, FloatLocalId, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionLocal, FunctionFunctionLocalId,
        FunctionFunctionReference, FunctionFunctionType, FunctionListLocalId, FunctionReference,
        FunctionShape, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntFunctionLocalId, IntFunctionReference, IntListLocalId, IntLocalId,
        ListExpr, ListFunctionExpr, ListFunctionId, ListFunctionReference, ListListLocalId,
        ListLocal, ListLocalExpr, NilExpr, NilFunctionExpr, NilFunctionId, NilFunctionLocalId,
        NilFunctionReference, NilListLocalId, NilLocalId, PanicExpr, PanicSite, ParamLocal,
        RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionId, StringFunctionLocalId,
        StringFunctionReference, StringListLocalId, StringLocalId, TupleExpr, TupleFunctionExpr,
        TupleFunctionId, TupleFunctionLocalId, TupleFunctionReference, TupleListLocalId,
        TupleLocalId, ValueShape, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn into_call_arg_preserves_param_family() {
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            Some(CallArg::int(IntLocalId(0), IntExpr::value(BigInt::from(1)),)),
        );
        assert_eq!(
            Expr::string(StringExpr::value("geam".into()))
                .into_call_arg(&ParamLocal::string(StringLocalId(0))),
            Some(CallArg::string(
                StringLocalId(0),
                StringExpr::value("geam".into()),
            )),
        );
        assert_eq!(
            Expr::bit_array(BitArrayExpr::value(Vec::new()))
                .into_call_arg(&ParamLocal::bit_array(BitArrayLocalId(0))),
            Some(CallArg::bit_array(
                BitArrayLocalId(0),
                BitArrayExpr::value(Vec::new()),
            )),
        );
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_value = CustomExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            custom_type.clone(),
        );
        assert_eq!(
            Expr::custom(custom_value.clone())
                .into_call_arg(&ParamLocal::custom(CustomLocalId(0), custom_type.clone(),)),
            Some(CallArg::custom(CustomLocalExpr::from_parts(
                CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                custom_value.clone(),
            ))),
        );
        assert_eq!(
            Expr::custom(custom_value).into_call_arg(&ParamLocal::custom(
                CustomLocalId(0),
                CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
                    Vec::new(),
                ),
            )),
            None,
        );
        assert_eq!(
            Expr::float(FloatExpr::value(1.5)).into_call_arg(&ParamLocal::float(FloatLocalId(0))),
            Some(CallArg::float(FloatLocalId(0), FloatExpr::value(1.5))),
        );
        assert_eq!(
            Expr::bool(BoolExpr::value(true)).into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            Some(CallArg::bool(BoolLocalId(0), BoolExpr::value(true))),
        );
        assert_eq!(
            Expr::nil(NilExpr::value()).into_call_arg(&ParamLocal::nil(NilLocalId(0))),
            Some(CallArg::nil(NilLocalId(0), NilExpr::value())),
        );
        assert_eq!(
            tuple_expr().into_call_arg(&ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])),
            Some(CallArg::tuple(
                TupleLocalId(0),
                TupleExpr::value(
                    vec![Expr::int(IntExpr::value(BigInt::from(1)))],
                    vec![ValueType::Int],
                ),
            )),
        );
        assert_eq!(
            Expr::list(list_expr())
                .into_call_arg(&ParamLocal::list(ListLocal::int(IntListLocalId(0)))),
            Some(CallArg::list(ListLocalExpr::Int {
                local: IntListLocalId(0),
                value: list_expr().into_int().expect("expected int list"),
            })),
        );
        assert_eq!(
            Expr::function(FunctionExpr::reference(function_value())).into_call_arg(
                &ParamLocal::int_function(
                    IntFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                )
            ),
            Some(CallArg::int_function(
                IntFunctionLocalId(0),
                int_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(string_function_expr())).into_call_arg(
                &ParamLocal::string_function(
                    StringFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::String], ValueType::String),
                )
            ),
            Some(CallArg::string_function(
                StringFunctionLocalId(0),
                string_function_expr(),
            )),
        );
        let bit_array_function_type =
            FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray);
        let bit_array_function = BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            BitArrayFunctionId(0),
            vec![ParamLocal::bit_array(BitArrayLocalId(0))],
        ));
        assert_eq!(
            Expr::function(FunctionExpr::bit_array(bit_array_function.clone())).into_call_arg(
                &ParamLocal::bit_array_function(
                    BitArrayFunctionLocalId(0),
                    bit_array_function_type,
                ),
            ),
            Some(CallArg::bit_array_function(
                BitArrayFunctionLocalId(0),
                bit_array_function,
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::float(float_function_expr())).into_call_arg(
                &ParamLocal::float_function(
                    FloatFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Float], ValueType::Float),
                )
            ),
            Some(CallArg::float_function(
                FloatFunctionLocalId(0),
                float_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(bool_function_expr())).into_call_arg(
                &ParamLocal::bool_function(
                    BoolFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                )
            ),
            Some(CallArg::bool_function(
                BoolFunctionLocalId(0),
                bool_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(nil_function_expr())).into_call_arg(
                &ParamLocal::nil_function(
                    NilFunctionLocalId(0),
                    FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
                )
            ),
            Some(CallArg::nil_function(
                NilFunctionLocalId(0),
                nil_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::tuple(tuple_function_expr())).into_call_arg(
                &ParamLocal::tuple_function(TupleFunctionLocalId(0), tuple_function_type())
            ),
            Some(CallArg::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::list(list_function_expr())).into_call_arg(
                &ParamLocal::list_function(crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    list_function_type(),
                    ValueType::Int,
                ))
            ),
            Some(CallArg::list_function(
                crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    list_function_type(),
                    ValueType::Int,
                ),
                list_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::function(function_function_expr())).into_call_arg(
                &ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    exact_function_function_type(),
                ))
            ),
            Some(CallArg::function_function(
                FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    exact_function_function_type(),
                ),
                function_function_expr(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                function_type(),
            )))
            .into_call_arg(&ParamLocal::int_function(
                IntFunctionLocalId(0),
                function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::bool(malformed_bool_function_expr(
                string_function_type(),
            )))
            .into_call_arg(&ParamLocal::string_function(
                StringFunctionLocalId(0),
                string_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::nil(malformed_nil_function_expr(
                bool_function_type(),
            )))
            .into_call_arg(&ParamLocal::bool_function(
                BoolFunctionLocalId(0),
                bool_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                function_function_type(),
            )))
            .into_call_arg(&ParamLocal::function_function(
                FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    exact_function_function_type(),
                )
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::int(malformed_int_function_expr(
                nil_function_type(),
            )))
            .into_call_arg(&ParamLocal::nil_function(
                NilFunctionLocalId(0),
                nil_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::string(malformed_string_function_expr(
                tuple_function_type(),
            )))
            .into_call_arg(&ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                tuple_function_type(),
            )),
            None,
        );
        assert_eq!(
            Expr::function(FunctionExpr::reference(function_value()))
                .into_call_arg(&ParamLocal::int(IntLocalId(0))),
            None,
        );
        assert_eq!(
            Expr::int(IntExpr::value(BigInt::from(1)))
                .into_call_arg(&ParamLocal::bool(BoolLocalId(0))),
            None,
        );
    }

    #[test]
    fn callable_capture_args_derive_the_local_type_from_the_value() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_function_type =
            CustomFunctionType::new(vec![ValueType::Int], custom_type.clone());
        let custom_value = CustomFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            custom_function_type.clone(),
        );
        let custom_shape = FunctionShape::new(
            custom_function_type.argument_shapes().to_vec(),
            ValueShape::Custom(custom_function_type.return_().clone()),
        );
        assert_eq!(
            CaptureArg::custom_function(CustomFunctionLocalId(2), custom_value.clone()).kind(),
            &CaptureArgKind::CustomFunction {
                local: CustomFunctionLocal::new(CustomFunctionLocalId(2), custom_function_type,),
                value: TypedFunctionExpr::new(custom_shape, custom_value),
            },
        );

        let function_function_type = FunctionFunctionType::new(
            vec![ValueType::String],
            FunctionType::new(vec![ValueType::Bool], ValueType::Int),
        );
        let function_value = FunctionFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            function_function_type.clone(),
        );
        let function_shape = FunctionShape::from_function_type(function_value.type_());
        assert_eq!(
            CaptureArg::function_function(FunctionFunctionLocalId(3), function_value.clone())
                .kind(),
            &CaptureArgKind::FunctionFunction {
                local: FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(3),
                    function_function_type,
                ),
                value: TypedFunctionExpr::new(function_shape, function_value),
            },
        );
    }

    #[test]
    fn custom_capture_arg_derives_the_local_type_from_the_value() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let value = CustomExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            custom_type.clone(),
        );

        assert_eq!(
            CaptureArg::custom(CustomLocalId(3), value.clone()).kind(),
            &CaptureArgKind::Custom(CustomLocalExpr::from_parts(
                CustomLocal::new(CustomLocalId(3), custom_type),
                value,
            )),
        );
    }

    #[test]
    fn callable_call_args_require_the_exact_callable_type() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_function_type =
            CustomFunctionType::new(vec![ValueType::Int], custom_type.clone());
        let custom_value = CustomFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            custom_function_type.clone(),
        );
        assert_eq!(
            Expr::function(FunctionExpr::custom(custom_value.clone())).into_call_arg(
                &ParamLocal::custom_function(CustomFunctionLocal::new(
                    CustomFunctionLocalId(4),
                    custom_function_type.clone(),
                )),
            ),
            Some(CallArg::custom_function(
                CustomFunctionLocal::new(CustomFunctionLocalId(4), custom_function_type),
                custom_value.clone(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::custom(custom_value)).into_call_arg(
                &ParamLocal::custom_function(CustomFunctionLocal::new(
                    CustomFunctionLocalId(4),
                    CustomFunctionType::new(vec![ValueType::String], custom_type),
                )),
            ),
            None,
        );

        let function_function_type = FunctionFunctionType::new(
            vec![ValueType::Bool],
            FunctionType::new(vec![ValueType::Int], ValueType::String),
        );
        let function_value = FunctionFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            function_function_type.clone(),
        );
        assert_eq!(
            Expr::function(FunctionExpr::function(function_value.clone())).into_call_arg(
                &ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(5),
                    function_function_type.clone(),
                )),
            ),
            Some(CallArg::function_function(
                FunctionFunctionLocal::new(FunctionFunctionLocalId(5), function_function_type,),
                function_value.clone(),
            )),
        );
        assert_eq!(
            Expr::function(FunctionExpr::function(function_value)).into_call_arg(
                &ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(5),
                    FunctionFunctionType::new(
                        vec![ValueType::Bool],
                        FunctionType::new(vec![ValueType::Float], ValueType::String),
                    ),
                )),
            ),
            None,
        );
    }

    #[test]
    fn into_call_arg_preserves_list_param_family() {
        let int = ListExpr::value(vec![Expr::int(IntExpr::value(1.into()))], ValueType::Int);
        assert_eq!(
            Expr::list(int.clone())
                .into_call_arg(&ParamLocal::list(ListLocal::int(IntListLocalId(0),))),
            Some(CallArg::list(ListLocalExpr::Int {
                local: IntListLocalId(0),
                value: int.into_int().expect("expected int list"),
            })),
        );

        let string = ListExpr::value(
            vec![Expr::string(StringExpr::value("one".into()))],
            ValueType::String,
        );
        assert_eq!(
            Expr::list(string.clone())
                .into_call_arg(&ParamLocal::list(ListLocal::string(StringListLocalId(1),))),
            Some(CallArg::list(ListLocalExpr::String {
                local: StringListLocalId(1),
                value: string.into_string().expect("expected string list"),
            })),
        );

        let bit_array = ListExpr::value(
            vec![Expr::bit_array(BitArrayExpr::value(Vec::new()))],
            ValueType::BitArray,
        );
        assert_eq!(
            Expr::list(bit_array.clone()).into_call_arg(&ParamLocal::list(ListLocal::bit_array(
                BitArrayListLocalId(2)
            ),)),
            Some(CallArg::list(ListLocalExpr::BitArray {
                local: BitArrayListLocalId(2),
                value: bit_array.into_bit_array().expect("expected bit array list"),
            })),
        );

        let float = ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float);
        assert_eq!(
            Expr::list(float.clone())
                .into_call_arg(&ParamLocal::list(ListLocal::float(FloatListLocalId(2),))),
            Some(CallArg::list(ListLocalExpr::Float {
                local: FloatListLocalId(2),
                value: float.into_float().expect("expected float list"),
            })),
        );

        let bool_ = ListExpr::value(vec![Expr::bool(BoolExpr::value(true))], ValueType::Bool);
        assert_eq!(
            Expr::list(bool_.clone())
                .into_call_arg(&ParamLocal::list(ListLocal::bool(BoolListLocalId(3),))),
            Some(CallArg::list(ListLocalExpr::Bool {
                local: BoolListLocalId(3),
                value: bool_.into_bool().expect("expected bool list"),
            })),
        );

        let nil = ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil);
        assert_eq!(
            Expr::list(nil.clone())
                .into_call_arg(&ParamLocal::list(ListLocal::nil(NilListLocalId(4),))),
            Some(CallArg::list(ListLocalExpr::Nil {
                local: NilListLocalId(4),
                value: nil.into_nil().expect("expected nil list"),
            })),
        );

        let tuple_item_type = vec![ValueType::Int];
        let tuple = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(2.into()))],
                tuple_item_type.clone(),
            ))],
            ValueType::Tuple(tuple_item_type.clone()),
        );
        assert_eq!(
            Expr::list(tuple.clone()).into_call_arg(&ParamLocal::list(ListLocal::tuple(
                TupleListLocalId(5),
                tuple_item_type.clone(),
            ))),
            Some(CallArg::list(ListLocalExpr::Tuple {
                local: TupleListLocalId(5),
                item_type: tuple_item_type,
                value: tuple.into_tuple().expect("expected tuple list"),
            })),
        );

        let nested_item_type = ValueType::Int;
        let nested = ListExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::value(3.into()))],
                nested_item_type.clone(),
            ))],
            ValueType::List(Box::new(nested_item_type.clone())),
        );
        assert_eq!(
            Expr::list(nested.clone()).into_call_arg(&ParamLocal::list(ListLocal::list(
                ListListLocalId(6),
                nested_item_type.clone(),
            ))),
            Some(CallArg::list(ListLocalExpr::List {
                local: ListListLocalId(6),
                item_type: Box::new(nested_item_type),
                value: nested.into_list().expect("expected nested list"),
            })),
        );

        let function_item_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function = ListExpr::value(
            vec![Expr::function(FunctionExpr::reference(
                FunctionReference::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new()),
            ))],
            ValueType::Function(Box::new(function_item_type.clone())),
        );
        assert_eq!(
            Expr::list(function.clone()).into_call_arg(&ParamLocal::list(ListLocal::function(
                FunctionListLocalId(7),
                function_item_type.clone(),
            ))),
            Some(CallArg::list(ListLocalExpr::Function {
                local: FunctionListLocalId(7),
                item_type: function_item_type,
                value: function.into_function().expect("expected function list"),
            })),
        );
    }

    #[test]
    fn into_call_arg_rejects_list_param_nested_metadata_mismatch() {
        let tuple = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![Expr::string(StringExpr::value("wrong".into()))],
                vec![ValueType::String],
            ))],
            ValueType::Tuple(vec![ValueType::String]),
        );
        assert_eq!(
            Expr::list(tuple).into_call_arg(&ParamLocal::list(ListLocal::tuple(
                TupleListLocalId(0),
                vec![ValueType::Int],
            ))),
            None,
        );

        let nested = ListExpr::value(
            vec![Expr::list(ListExpr::value(
                vec![Expr::string(StringExpr::value("wrong".into()))],
                ValueType::String,
            ))],
            ValueType::List(Box::new(ValueType::String)),
        );
        assert_eq!(
            Expr::list(nested).into_call_arg(&ParamLocal::list(ListLocal::list(
                ListListLocalId(0),
                ValueType::Int,
            ))),
            None,
        );

        let function = ListExpr::value(
            vec![Expr::function(FunctionExpr::reference(
                FunctionReference::new(RuntimeFunctionId::String(StringFunctionId(0)), Vec::new()),
            ))],
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
        );
        assert_eq!(
            Expr::list(function).into_call_arg(&ParamLocal::list(ListLocal::function(
                FunctionListLocalId(0),
                FunctionType::new(Vec::new(), ValueType::Int),
            ))),
            None,
        );
    }

    fn function_value() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(IntLocalId(0))],
        )
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn tuple_expr() -> Expr {
        Expr::tuple(TupleExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            vec![ValueType::Int],
        ))
    }

    fn list_expr() -> ListExpr {
        ListExpr::value(
            vec![Expr::int(IntExpr::value(BigInt::from(1)))],
            ValueType::Int,
        )
    }

    fn tuple_function_expr() -> TupleFunctionExpr {
        TupleFunctionExpr::reference(
            TupleFunctionReference::new(
                TupleFunctionId(0),
                vec![ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])],
            ),
            vec![ValueType::Int],
        )
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::reference(ListFunctionReference::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::list(ListLocal::int(IntListLocalId(0)))],
        ))
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::List(Box::new(ValueType::Int))],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
            ),
            function_type(),
        )
    }

    fn malformed_int_function_expr(type_: FunctionType) -> IntFunctionExpr {
        IntFunctionExpr::local_get(IntFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_string_function_expr(type_: FunctionType) -> StringFunctionExpr {
        StringFunctionExpr::local_get(StringFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_bool_function_expr(type_: FunctionType) -> BoolFunctionExpr {
        BoolFunctionExpr::local_get(BoolFunctionLocalId(0), "f".into(), type_)
    }

    fn malformed_nil_function_expr(type_: FunctionType) -> NilFunctionExpr {
        NilFunctionExpr::local_get(NilFunctionLocalId(0), "f".into(), type_)
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn bool_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(vec![ValueType::Int])],
            ValueType::Tuple(vec![ValueType::Int]),
        )
    }

    fn function_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Function(Box::new(function_type())))
    }

    fn exact_function_function_type() -> FunctionFunctionType {
        FunctionFunctionType::new(Vec::new(), function_type())
    }
}
