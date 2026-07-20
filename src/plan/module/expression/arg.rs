use super::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    CustomLocalExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionFunctionExpr, GenericExpr, GenericFunctionExpr, IntExpr, IntFunctionExpr, ListExpr,
    ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
    TupleExpr, TupleFunctionExpr, TypedFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use crate::plan::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomFunctionLocalId, CustomLocalId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocal, FunctionFunctionLocalId, GenericFunctionLocal, GenericLocal,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal, NilFunctionLocalId, NilLocalId,
    ParamLocal, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    kind: CallArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CallArgKind {
    Parametric {
        slot: crate::plan::ParamSlot,
        value: Box<Expr>,
    },
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
    GenericFunction {
        local: GenericFunctionLocal,
        value: FunctionExpr,
    },
}

pub(crate) enum CallArgStorage<'a> {
    Stored(crate::plan::ValueStorageShape),
    PotentiallyUninhabited(PotentiallyUninhabitedCallArg<'a>),
}

pub(crate) enum PotentiallyUninhabitedCallArg<'a> {
    Generic(&'a GenericExpr),
    Tuple(&'a TupleExpr),
    Custom(&'a CustomExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaptureArg {
    kind: CaptureArgKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureArgKind {
    Generic {
        local: GenericLocal,
        value: GenericExpr,
    },
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
    GenericFunction {
        local: GenericFunctionLocal,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    },
}

impl Expr {
    pub(crate) fn into_call_arg(self, local: &ParamLocal) -> Option<CallArg> {
        let Self { shape, kind } = self;
        match (local, kind) {
            (ParamLocal::Generic(local), kind) => Some(CallArg::parametric(
                crate::plan::ParamSlot::new(
                    ParamLocal::Generic(*local),
                    crate::plan::ValueShape::Parameter(local.parameter()),
                ),
                Expr { shape, kind },
            )),
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
            (
                ParamLocal::List(ListLocal::Generic { local, parameter }),
                ExprKind::List(ListExpr::Generic(value)),
            ) if value.item().parameter() == *parameter => {
                Some(CallArg::list(ListLocalExpr::Generic {
                    local: *local,
                    parameter: *parameter,
                    value,
                }))
            }
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
                ExprKind::List(ListExpr::ParameterList(value)),
            ) if item_type.as_ref() == &ValueType::Parameter(value.item().parameter()) => {
                Some(CallArg::list(ListLocalExpr::ParameterList {
                    local: *local,
                    parameter: value.item().parameter(),
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
            (ParamLocal::GenericFunction(local), ExprKind::Function(value)) => {
                Some(CallArg::generic_function_expr(local.clone(), value))
            }
            _ => None,
        }
    }
}

impl CallArg {
    pub(crate) fn parametric(slot: crate::plan::ParamSlot, value: Expr) -> Self {
        Self {
            kind: CallArgKind::Parametric {
                slot,
                value: Box::new(value),
            },
        }
    }

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

    fn generic_function_expr(local: GenericFunctionLocal, value: FunctionExpr) -> Self {
        Self {
            kind: CallArgKind::GenericFunction { local, value },
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

    pub(crate) fn storage(&self) -> CallArgStorage<'_> {
        use crate::plan::ValueStorageShape as S;

        match &self.kind {
            CallArgKind::Parametric { value, .. } => match value.kind() {
                ExprKind::Generic(value) => CallArgStorage::PotentiallyUninhabited(
                    PotentiallyUninhabitedCallArg::Generic(value),
                ),
                ExprKind::Tuple(value) => CallArgStorage::PotentiallyUninhabited(
                    PotentiallyUninhabitedCallArg::Tuple(value),
                ),
                ExprKind::Custom(value) => CallArgStorage::PotentiallyUninhabited(
                    PotentiallyUninhabitedCallArg::Custom(value),
                ),
                ExprKind::Int(_) => CallArgStorage::Stored(S::Int),
                ExprKind::Float(_) => CallArgStorage::Stored(S::Float),
                ExprKind::String(_) => CallArgStorage::Stored(S::String),
                ExprKind::BitArray(_) => CallArgStorage::Stored(S::BitArray),
                ExprKind::UtfCodepoint(_) => CallArgStorage::Stored(S::UtfCodepoint),
                ExprKind::Bool(_) => CallArgStorage::Stored(S::Bool),
                ExprKind::Nil(_) => CallArgStorage::Stored(S::Nil),
                ExprKind::List(value) => {
                    CallArgStorage::Stored(S::List(Box::new(value.item_shape().clone())))
                }
                ExprKind::Function(value) => {
                    CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
                }
            },
            CallArgKind::Tuple { value, .. } => {
                CallArgStorage::PotentiallyUninhabited(PotentiallyUninhabitedCallArg::Tuple(value))
            }
            CallArgKind::Custom(value) => CallArgStorage::PotentiallyUninhabited(
                PotentiallyUninhabitedCallArg::Custom(value.value()),
            ),
            CallArgKind::Int { .. } => CallArgStorage::Stored(S::Int),
            CallArgKind::Float { .. } => CallArgStorage::Stored(S::Float),
            CallArgKind::String { .. } => CallArgStorage::Stored(S::String),
            CallArgKind::BitArray { .. } => CallArgStorage::Stored(S::BitArray),
            CallArgKind::UtfCodepoint { .. } => CallArgStorage::Stored(S::UtfCodepoint),
            CallArgKind::Bool { .. } => CallArgStorage::Stored(S::Bool),
            CallArgKind::Nil { .. } => CallArgStorage::Stored(S::Nil),
            CallArgKind::List(value) => {
                CallArgStorage::Stored(S::List(Box::new(value.item_shape().clone())))
            }
            CallArgKind::IntFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::StringFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::BitArrayFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::UtfCodepointFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::CustomFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::FloatFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::BoolFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::NilFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::TupleFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::ListFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::FunctionFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
            CallArgKind::GenericFunction { value, .. } => {
                CallArgStorage::Stored(S::Function(Box::new(value.shape().clone())))
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn parameter_shape(&self) -> crate::plan::ValueShape {
        match &self.kind {
            CallArgKind::Parametric { slot, .. } => slot.shape().clone(),
            CallArgKind::Int { .. } => crate::plan::ValueShape::Int,
            CallArgKind::Float { .. } => crate::plan::ValueShape::Float,
            CallArgKind::String { .. } => crate::plan::ValueShape::String,
            CallArgKind::BitArray { .. } => crate::plan::ValueShape::BitArray,
            CallArgKind::UtfCodepoint { .. } => crate::plan::ValueShape::UtfCodepoint,
            CallArgKind::Custom(value) => {
                crate::plan::ValueShape::Custom(value.local().shape().clone())
            }
            CallArgKind::Bool { .. } => crate::plan::ValueShape::Bool,
            CallArgKind::Nil { .. } => crate::plan::ValueShape::Nil,
            CallArgKind::Tuple { value, .. } => {
                crate::plan::ValueShape::Tuple(value.shape().to_vec().into_boxed_slice())
            }
            CallArgKind::List(value) => {
                crate::plan::ValueShape::List(Box::new(value.item_shape().clone()))
            }
            CallArgKind::IntFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::StringFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::BitArrayFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::UtfCodepointFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::CustomFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::FloatFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::BoolFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::NilFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::TupleFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::ListFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::FunctionFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
            CallArgKind::GenericFunction { value, .. } => {
                crate::plan::ValueShape::Function(Box::new(value.shape().clone()))
            }
        }
    }
}

impl CaptureArg {
    pub(crate) fn generic(local: GenericLocal, value: GenericExpr) -> Self {
        Self {
            kind: CaptureArgKind::Generic { local, value },
        }
    }

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

    pub(crate) fn generic_function_expr(
        local: GenericFunctionLocal,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    ) -> Self {
        Self {
            kind: CaptureArgKind::GenericFunction { local, value },
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
}

#[cfg(test)]
mod tests {
    use super::{CallArg, CaptureArg, CaptureArgKind, CustomLocalExpr, TypedFunctionExpr};
    use crate::plan::module::{GenericListExpr, GenericListItem};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayFunctionReference,
        BitArrayListLocalId, BitArrayLocalId, BoolExpr, BoolFunctionExpr, BoolFunctionLocalId,
        BoolFunctionReference, BoolListLocalId, BoolLocalId, CustomExpr, CustomFunctionExpr,
        CustomFunctionLocal, CustomFunctionLocalId, CustomFunctionType, CustomLocal, CustomLocalId,
        CustomType, CustomTypeName, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionLocalId,
        FloatFunctionReference, FloatListLocalId, FloatLocalId, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionReference,
        FunctionFunctionType, FunctionListLocalId, FunctionReference, FunctionShape, FunctionType,
        GenericExpr, GenericFunctionExpr, GenericFunctionLocal, GenericFunctionLocalId,
        GenericFunctionType, GenericLocal, GenericLocalId, IntExpr, IntFunctionExpr,
        IntFunctionLocalId, IntFunctionReference, IntListLocalId, IntLocalId, ListExpr,
        ListFunctionExpr, ListFunctionReference, ListListLocalId, ListLocal, ListLocalExpr,
        NilExpr, NilFunctionExpr, NilFunctionLocalId, NilFunctionReference, NilListLocalId,
        NilLocalId, PanicExpr, PanicSite, ParamLocal, StringExpr, StringFunctionExpr,
        StringFunctionLocalId, StringFunctionReference, StringListLocalId, StringLocalId,
        TupleExpr, TupleFunctionExpr, TupleFunctionLocalId, TupleFunctionReference,
        TupleListLocalId, TupleLocalId, TypeParameterId, UtfCodepointExpr,
        UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueShape,
        ValueType, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn generic_arguments_preserve_parameter_and_callable_shapes() {
        let parameter = TypeParameterId(0);
        let local = GenericLocal::new(GenericLocalId(0), parameter);
        let value = GenericExpr::local_get(local, "value".into());
        let call = Expr::generic(value.clone())
            .into_call_arg(&ParamLocal::generic(local))
            .expect("a generic parameter should accept its generic expression");
        assert_eq!(
            call,
            CallArg::parametric(
                crate::plan::ParamSlot::new(
                    ParamLocal::generic(local),
                    ValueShape::Parameter(parameter),
                ),
                Expr::generic(value.clone()),
            ),
        );
        assert_eq!(call.parameter_shape(), ValueShape::Parameter(parameter));
        assert_eq!(
            CaptureArg::generic(local, value.clone()).kind(),
            &CaptureArgKind::Generic { local, value },
        );

        let function_type = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let function_local =
            GenericFunctionLocal::new(GenericFunctionLocalId(0), function_type.clone());
        let function = GenericFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            function_type.clone(),
        );
        let function_facade = FunctionExpr::generic(function.clone());
        let call = Expr::function(function_facade.clone())
            .into_call_arg(&ParamLocal::generic_function(function_local.clone()))
            .expect("a generic callable should accept its exact function expression");
        assert_eq!(
            call.kind(),
            &super::CallArgKind::GenericFunction {
                local: function_local.clone(),
                value: function_facade,
            },
        );
        assert_eq!(
            call.parameter_shape(),
            ValueShape::Function(Box::new(function_type.shape())),
        );
        assert_eq!(
            CaptureArg::generic_function_expr(
                function_local.clone(),
                TypedFunctionExpr::new(function_type.shape(), function.clone()),
            )
            .kind(),
            &CaptureArgKind::GenericFunction {
                local: function_local,
                value: TypedFunctionExpr::new(function_type.shape(), function),
            },
        );
    }

    #[test]
    fn call_arg_parameter_shapes_preserve_compound_family_metadata() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());

        assert_eq!(
            CallArg::bit_array(BitArrayLocalId(0), BitArrayExpr::value(Vec::new()))
                .parameter_shape(),
            ValueShape::BitArray,
        );
        assert_eq!(
            CallArg::utf_codepoint(UtfCodepointLocalId(0), UtfCodepointExpr::panic(panic()))
                .parameter_shape(),
            ValueShape::UtfCodepoint,
        );

        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom = CustomExpr::panic(panic(), custom_type.clone());
        assert_eq!(
            CallArg::custom(CustomLocalExpr::from_parts(
                CustomLocal::new(CustomLocalId(0), custom_type.clone()),
                custom,
            ))
            .parameter_shape(),
            ValueShape::Custom(crate::plan::CustomValueShape::any(custom_type.clone())),
        );

        let bit_array_function_type =
            FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray);
        assert_eq!(
            CallArg::bit_array_function(
                BitArrayFunctionLocalId(0),
                BitArrayFunctionExpr::panic(panic(), bit_array_function_type.clone()),
            )
            .parameter_shape(),
            ValueShape::Function(Box::new(FunctionShape::from_function_type(
                bit_array_function_type,
            ))),
        );

        let utf_codepoint_function_type =
            FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint);
        let utf_codepoint = Expr::function(FunctionExpr::utf_codepoint(
            UtfCodepointFunctionExpr::panic(panic(), utf_codepoint_function_type.clone()),
        ))
        .into_call_arg(&ParamLocal::utf_codepoint_function(
            UtfCodepointFunctionLocalId(0),
            utf_codepoint_function_type.clone(),
        ))
        .expect("a UTF codepoint callable should accept its exact function expression");
        assert_eq!(
            utf_codepoint.parameter_shape(),
            ValueShape::Function(Box::new(FunctionShape::from_function_type(
                utf_codepoint_function_type,
            ))),
        );

        let custom_function_type =
            CustomFunctionType::new(vec![ValueType::Int], custom_type.clone());
        let custom_function = Expr::function(FunctionExpr::custom(CustomFunctionExpr::panic(
            panic(),
            custom_function_type.clone(),
        )))
        .into_call_arg(&ParamLocal::custom_function(CustomFunctionLocal::new(
            CustomFunctionLocalId(0),
            custom_function_type.clone(),
        )))
        .expect("a custom callable should accept its exact function expression");
        assert_eq!(
            custom_function.parameter_shape(),
            ValueShape::Function(Box::new(FunctionShape::new(
                custom_function_type.argument_shapes().to_vec(),
                ValueShape::Custom(custom_function_type.return_().clone()),
            ))),
        );
    }

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
            instantiation(bit_array_function_type.clone()),
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
        let parameter = crate::plan::TypeParameterId(0);
        let generic = GenericListExpr::local_get(
            GenericListItem::new(parameter),
            crate::plan::GenericListLocalId(0),
            "generic".into(),
        );
        assert_eq!(
            Expr::list(crate::plan::ListExpr::Generic(generic.clone())).into_call_arg(
                &ParamLocal::list(ListLocal::generic(
                    crate::plan::GenericListLocalId(1),
                    parameter,
                )),
            ),
            Some(CallArg::list(ListLocalExpr::Generic {
                local: crate::plan::GenericListLocalId(1),
                parameter,
                value: generic,
            })),
        );

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

        let parameter = crate::plan::TypeParameterId(0);
        let parameter_list = crate::plan::ParameterListListExpr::local_get(
            crate::plan::ParameterListListItem::new(parameter),
            ListListLocalId(7),
            "parameter_lists".into(),
        );
        assert_eq!(
            Expr::list(ListExpr::ParameterList(parameter_list.clone())).into_call_arg(
                &ParamLocal::list(ListLocal::list(
                    ListListLocalId(8),
                    ValueType::Parameter(parameter),
                )),
            ),
            Some(CallArg::list(ListLocalExpr::ParameterList {
                local: ListListLocalId(8),
                parameter,
                value: parameter_list,
            })),
        );

        let function_item_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function = ListExpr::value(
            vec![Expr::function(FunctionExpr::reference(
                FunctionReference::new(
                    instantiation(FunctionType::new(Vec::new(), ValueType::Int)),
                    Vec::new(),
                ),
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
                FunctionReference::new(
                    instantiation(FunctionType::new(Vec::new(), ValueType::String)),
                    Vec::new(),
                ),
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
            instantiation(function_type()),
            vec![ParamLocal::int(IntLocalId(0))],
        )
    }

    fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(
            instantiation(function_type()),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(
            instantiation(string_function_type()),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            instantiation(FunctionType::new(vec![ValueType::Float], ValueType::Float)),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(
            instantiation(FunctionType::new(vec![ValueType::Bool], ValueType::Bool)),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            instantiation(FunctionType::new(vec![ValueType::Nil], ValueType::Nil)),
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
        TupleFunctionExpr::reference(TupleFunctionReference::new(
            instantiation(FunctionType::new(
                vec![ValueType::Tuple(vec![ValueType::Int])],
                ValueType::Tuple(vec![ValueType::Int]),
            )),
            vec![ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])],
        ))
    }

    fn list_function_expr() -> ListFunctionExpr {
        ListFunctionExpr::reference(
            ListFunctionReference::new(
                instantiation(list_function_type()),
                vec![ParamLocal::list(ListLocal::int(IntListLocalId(0)))],
            ),
            ValueType::Int,
        )
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
                instantiation(FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(function_type())),
                )),
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

    fn instantiation(type_: FunctionType) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::from_function_type(type_))
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
