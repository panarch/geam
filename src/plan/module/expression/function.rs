mod bit_array;
mod bool;
mod custom;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod typed;
mod utf_codepoint;

use crate::plan::{
    BitArrayFunctionReference, BoolFunctionReference, CustomFieldAccess, CustomFunctionReference,
    FloatFunctionReference, FunctionFunctionReference, FunctionReference, FunctionShape,
    FunctionType, IntFunctionReference, ListFunctionReference, NilFunctionReference,
    RuntimeFunctionId, StringFunctionReference, TupleFunctionReference,
    UtfCodepointFunctionReference, ValueShape,
};

pub use self::{
    bit_array::BitArrayFunctionExpr, bool::BoolFunctionExpr, custom::CustomFunctionExpr,
    float::FloatFunctionExpr, int::IntFunctionExpr, list::ListFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr, tuple::TupleFunctionExpr,
    utf_codepoint::UtfCodepointFunctionExpr,
};
pub(crate) use self::{
    bit_array::BitArrayFunctionExprKind,
    bool::BoolFunctionExprKind,
    custom::CustomFunctionExprKind,
    float::FloatFunctionExprKind,
    int::IntFunctionExprKind,
    list::ListFunctionExprKind,
    nil::NilFunctionExprKind,
    returning_function::{FunctionFunctionCallMismatch, FunctionFunctionExprKind},
    string::StringFunctionExprKind,
    tuple::TupleFunctionExprKind,
    typed::TypedFunctionExpr,
    utf_codepoint::UtfCodepointFunctionExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    shape: crate::plan::FunctionShape,
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    BitArray(BitArrayFunctionExpr),
    UtfCodepoint(UtfCodepointFunctionExpr),
    Custom(CustomFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    List(ListFunctionExpr),
    Function(FunctionFunctionExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedFunctionExprKind {
    Int(TypedFunctionExpr<IntFunctionExpr>),
    String(TypedFunctionExpr<StringFunctionExpr>),
    BitArray(TypedFunctionExpr<BitArrayFunctionExpr>),
    UtfCodepoint(TypedFunctionExpr<UtfCodepointFunctionExpr>),
    Custom(TypedFunctionExpr<CustomFunctionExpr>),
    Float(TypedFunctionExpr<FloatFunctionExpr>),
    Bool(TypedFunctionExpr<BoolFunctionExpr>),
    Nil(TypedFunctionExpr<NilFunctionExpr>),
    Tuple(TypedFunctionExpr<TupleFunctionExpr>),
    List(TypedFunctionExpr<ListFunctionExpr>),
    Function(TypedFunctionExpr<FunctionFunctionExpr>),
}

impl FunctionExpr {
    fn new(kind: FunctionExprKind) -> Self {
        let shape = match &kind {
            FunctionExprKind::Int(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::String(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::BitArray(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::UtfCodepoint(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::Custom(expression) => crate::plan::FunctionShape::new(
                expression.custom_function_type().argument_shapes().to_vec(),
                crate::plan::ValueShape::Custom(
                    expression.custom_function_type().return_().clone(),
                ),
            ),
            FunctionExprKind::Float(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::Bool(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::Nil(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::Tuple(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::List(expression) => {
                crate::plan::FunctionShape::from_function_type(expression.type_().clone())
            }
            FunctionExprKind::Function(expression) => crate::plan::FunctionShape::new(
                expression
                    .function_function_type()
                    .argument_shapes()
                    .to_vec(),
                crate::plan::ValueShape::Function(Box::new(
                    expression.function_function_type().return_shape().clone(),
                )),
            ),
        };
        Self { shape, kind }
    }

    pub(crate) fn custom_field_shape(access: CustomFieldAccess, shape: FunctionShape) -> Self {
        let type_ = shape.type_();
        match shape.return_shape().clone() {
            ValueShape::Int => {
                Self::int_with_shape(IntFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::String => {
                Self::string_with_shape(StringFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::BitArray => {
                Self::bit_array_with_shape(BitArrayFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::UtfCodepoint => Self::utf_codepoint_with_shape(
                UtfCodepointFunctionExpr::custom_field(access, type_),
                shape,
            ),
            ValueShape::Custom(return_shape) => Self::custom(CustomFunctionExpr::custom_field(
                access,
                crate::plan::CustomFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    return_shape,
                ),
            )),
            ValueShape::Float => {
                Self::float_with_shape(FloatFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::Bool => {
                Self::bool_with_shape(BoolFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::Nil => {
                Self::nil_with_shape(NilFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::Tuple(_) => {
                Self::tuple_with_shape(TupleFunctionExpr::custom_field(access, type_), shape)
            }
            ValueShape::List(item_shape) => Self::list_with_shape(
                ListFunctionExpr::custom_field(access, type_, item_shape.value_type()),
                shape,
            ),
            ValueShape::Function(return_shape) => Self::function_with_shape(
                FunctionFunctionExpr::custom_field(
                    access,
                    crate::plan::FunctionFunctionType::from_shapes(
                        shape.argument_shapes().to_vec(),
                        *return_shape,
                    ),
                ),
                shape,
            ),
        }
    }

    pub(crate) fn reference(reference: FunctionReference) -> Self {
        let (runtime_id, params) = reference.into_parts();
        match runtime_id {
            RuntimeFunctionId::Int(id) => Self::int(IntFunctionExpr::reference(
                IntFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::Float(id) => Self::float(FloatFunctionExpr::reference(
                FloatFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::String(id) => Self::string(StringFunctionExpr::reference(
                StringFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::BitArray(id) => Self::bit_array(BitArrayFunctionExpr::reference(
                BitArrayFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::UtfCodepoint(id) => {
                Self::utf_codepoint(UtfCodepointFunctionExpr::reference(
                    UtfCodepointFunctionReference::from_slots(id, params),
                ))
            }
            RuntimeFunctionId::Custom(id) => Self::custom(CustomFunctionExpr::reference(
                CustomFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::Bool(id) => Self::bool(BoolFunctionExpr::reference(
                BoolFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::Nil(id) => Self::nil(NilFunctionExpr::reference(
                NilFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::Tuple { id, return_type } => {
                Self::tuple(TupleFunctionExpr::reference(
                    TupleFunctionReference::from_slots(id, params),
                    return_type,
                ))
            }
            RuntimeFunctionId::List(id) => Self::list(ListFunctionExpr::reference(
                ListFunctionReference::from_slots(id, params),
            )),
            RuntimeFunctionId::Function { id, return_type } => {
                Self::function(FunctionFunctionExpr::reference(
                    FunctionFunctionReference::from_slots(id, params),
                    return_type,
                ))
            }
        }
    }

    pub(crate) fn int(expression: IntFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Int(expression))
    }

    pub(crate) fn int_with_shape(
        expression: IntFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Int(expression), shape)
    }

    pub(crate) fn string(expression: StringFunctionExpr) -> Self {
        Self::new(FunctionExprKind::String(expression))
    }

    pub(crate) fn string_with_shape(
        expression: StringFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::String(expression), shape)
    }

    pub(crate) fn bit_array(expression: BitArrayFunctionExpr) -> Self {
        Self::new(FunctionExprKind::BitArray(expression))
    }

    pub(crate) fn bit_array_with_shape(
        expression: BitArrayFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::BitArray(expression), shape)
    }

    pub(crate) fn utf_codepoint(expression: UtfCodepointFunctionExpr) -> Self {
        Self::new(FunctionExprKind::UtfCodepoint(expression))
    }

    pub(crate) fn utf_codepoint_with_shape(
        expression: UtfCodepointFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::UtfCodepoint(expression), shape)
    }

    pub(crate) fn custom(expression: CustomFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Custom(expression))
    }

    pub(crate) fn float(expression: FloatFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Float(expression))
    }

    pub(crate) fn float_with_shape(
        expression: FloatFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Float(expression), shape)
    }

    pub(crate) fn bool(expression: BoolFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Bool(expression))
    }

    pub(crate) fn bool_with_shape(
        expression: BoolFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Bool(expression), shape)
    }

    pub(crate) fn nil(expression: NilFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Nil(expression))
    }

    pub(crate) fn nil_with_shape(
        expression: NilFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Nil(expression), shape)
    }

    pub(crate) fn tuple(expression: TupleFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Tuple(expression))
    }

    pub(crate) fn tuple_with_shape(
        expression: TupleFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Tuple(expression), shape)
    }

    pub(crate) fn list(expression: ListFunctionExpr) -> Self {
        Self::new(FunctionExprKind::List(expression))
    }

    pub(crate) fn list_with_shape(
        expression: ListFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::List(expression), shape)
    }

    pub(crate) fn function(expression: FunctionFunctionExpr) -> Self {
        Self::new(FunctionExprKind::Function(expression))
    }

    pub(crate) fn function_with_shape(
        expression: FunctionFunctionExpr,
        shape: crate::plan::FunctionShape,
    ) -> Self {
        Self::with_typed_shape(FunctionExprKind::Function(expression), shape)
    }

    fn with_typed_shape(kind: FunctionExprKind, shape: crate::plan::FunctionShape) -> Self {
        Self { shape, kind }
    }

    pub(crate) fn block(steps: Vec<crate::plan::Step>, return_: Self) -> Self {
        let Self { shape, kind } = return_;
        let kind = match kind {
            FunctionExprKind::Int(return_) => {
                FunctionExprKind::Int(IntFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::String(return_) => {
                FunctionExprKind::String(StringFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::BitArray(return_) => {
                FunctionExprKind::BitArray(BitArrayFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::UtfCodepoint(return_) => {
                FunctionExprKind::UtfCodepoint(UtfCodepointFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Custom(return_) => {
                FunctionExprKind::Custom(CustomFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Float(return_) => {
                FunctionExprKind::Float(FloatFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Bool(return_) => {
                FunctionExprKind::Bool(BoolFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Nil(return_) => {
                FunctionExprKind::Nil(NilFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Tuple(return_) => {
                FunctionExprKind::Tuple(TupleFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::List(return_) => {
                FunctionExprKind::List(ListFunctionExpr::block(steps, return_))
            }
            FunctionExprKind::Function(return_) => {
                FunctionExprKind::Function(FunctionFunctionExpr::block(steps, return_))
            }
        };
        Self { shape, kind }
    }

    pub fn type_(&self) -> FunctionType {
        self.shape.type_()
    }

    pub(crate) fn shape(&self) -> &crate::plan::FunctionShape {
        &self.shape
    }

    pub(crate) fn with_shape(self, shape: crate::plan::FunctionShape) -> Option<Self> {
        if shape.type_() != self.type_() {
            return None;
        }
        if !self.shape.can_flow_to(&shape) {
            return None;
        }

        Some(self)
    }

    pub(crate) fn with_resolved_shape(self, shape: crate::plan::FunctionShape) -> Option<Self> {
        if shape.type_() != self.type_() {
            return None;
        }

        Some(self.set_resolved_shape(shape))
    }

    fn set_resolved_shape(mut self, shape: crate::plan::FunctionShape) -> Self {
        self.kind = match (self.kind, shape.return_shape().clone()) {
            (FunctionExprKind::Custom(expression), crate::plan::ValueShape::Custom(return_)) => {
                FunctionExprKind::Custom(expression.with_type(
                    crate::plan::CustomFunctionType::from_shapes(
                        shape.argument_shapes().to_vec(),
                        return_,
                    ),
                ))
            }
            (
                FunctionExprKind::Function(expression),
                crate::plan::ValueShape::Function(return_),
            ) => FunctionExprKind::Function(expression.with_type(
                crate::plan::FunctionFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    *return_,
                ),
            )),
            (kind, _) => kind,
        };
        self.shape = shape;
        self
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> FunctionExprKind {
        self.kind
    }

    pub(crate) fn into_parts(self) -> (crate::plan::FunctionShape, FunctionExprKind) {
        (self.shape, self.kind)
    }

    pub(crate) fn into_typed_kind(self) -> TypedFunctionExprKind {
        let Self { shape, kind } = self;
        match kind {
            FunctionExprKind::Int(expression) => {
                TypedFunctionExprKind::Int(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::String(expression) => {
                TypedFunctionExprKind::String(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::BitArray(expression) => {
                TypedFunctionExprKind::BitArray(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::UtfCodepoint(expression) => {
                TypedFunctionExprKind::UtfCodepoint(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Custom(expression) => {
                TypedFunctionExprKind::Custom(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Float(expression) => {
                TypedFunctionExprKind::Float(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Bool(expression) => {
                TypedFunctionExprKind::Bool(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Nil(expression) => {
                TypedFunctionExprKind::Nil(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Tuple(expression) => {
                TypedFunctionExprKind::Tuple(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::List(expression) => {
                TypedFunctionExprKind::List(TypedFunctionExpr::new(shape, expression))
            }
            FunctionExprKind::Function(expression) => {
                TypedFunctionExprKind::Function(TypedFunctionExpr::new(shape, expression))
            }
        }
    }

    pub(crate) fn into_typed_int(self) -> Option<TypedFunctionExpr<IntFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_string(self) -> Option<TypedFunctionExpr<StringFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_bit_array(self) -> Option<TypedFunctionExpr<BitArrayFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::BitArray(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_utf_codepoint(
        self,
    ) -> Option<TypedFunctionExpr<UtfCodepointFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::UtfCodepoint(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_custom(self) -> Option<TypedFunctionExpr<CustomFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Custom(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_float(self) -> Option<TypedFunctionExpr<FloatFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_bool(self) -> Option<TypedFunctionExpr<BoolFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_nil(self) -> Option<TypedFunctionExpr<NilFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_tuple(self) -> Option<TypedFunctionExpr<TupleFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_list(self) -> Option<TypedFunctionExpr<ListFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_typed_function(self) -> Option<TypedFunctionExpr<FunctionFunctionExpr>> {
        match self.into_typed_kind() {
            TypedFunctionExprKind::Function(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_int(self) -> Option<IntFunctionExpr> {
        match self.kind {
            FunctionExprKind::Int(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<StringFunctionExpr> {
        match self.kind {
            FunctionExprKind::String(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bit_array(self) -> Option<BitArrayFunctionExpr> {
        match self.kind {
            FunctionExprKind::BitArray(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_utf_codepoint(self) -> Option<UtfCodepointFunctionExpr> {
        match self.kind {
            FunctionExprKind::UtfCodepoint(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_custom(self) -> Option<CustomFunctionExpr> {
        match self.kind {
            FunctionExprKind::Custom(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<FloatFunctionExpr> {
        match self.kind {
            FunctionExprKind::Float(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<BoolFunctionExpr> {
        match self.kind {
            FunctionExprKind::Bool(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_nil(self) -> Option<NilFunctionExpr> {
        match self.kind {
            FunctionExprKind::Nil(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_tuple(self) -> Option<TupleFunctionExpr> {
        match self.kind {
            FunctionExprKind::Tuple(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_list(self) -> Option<ListFunctionExpr> {
        match self.kind {
            FunctionExprKind::List(expression) => Some(expression),
            _ => None,
        }
    }

    pub(crate) fn into_function(self) -> Option<FunctionFunctionExpr> {
        match self.kind {
            FunctionExprKind::Function(expression) => Some(expression),
            _ => None,
        }
    }
}

impl From<IntFunctionExpr> for FunctionExpr {
    fn from(expression: IntFunctionExpr) -> Self {
        Self::int(expression)
    }
}

impl From<StringFunctionExpr> for FunctionExpr {
    fn from(expression: StringFunctionExpr) -> Self {
        Self::string(expression)
    }
}

impl From<BitArrayFunctionExpr> for FunctionExpr {
    fn from(expression: BitArrayFunctionExpr) -> Self {
        Self::bit_array(expression)
    }
}

impl From<UtfCodepointFunctionExpr> for FunctionExpr {
    fn from(expression: UtfCodepointFunctionExpr) -> Self {
        Self::utf_codepoint(expression)
    }
}

impl From<FloatFunctionExpr> for FunctionExpr {
    fn from(expression: FloatFunctionExpr) -> Self {
        Self::float(expression)
    }
}

impl From<BoolFunctionExpr> for FunctionExpr {
    fn from(expression: BoolFunctionExpr) -> Self {
        Self::bool(expression)
    }
}

impl From<NilFunctionExpr> for FunctionExpr {
    fn from(expression: NilFunctionExpr) -> Self {
        Self::nil(expression)
    }
}

impl From<TupleFunctionExpr> for FunctionExpr {
    fn from(expression: TupleFunctionExpr) -> Self {
        Self::tuple(expression)
    }
}

impl From<ListFunctionExpr> for FunctionExpr {
    fn from(expression: ListFunctionExpr) -> Self {
        Self::list(expression)
    }
}

impl From<FunctionFunctionExpr> for FunctionExpr {
    fn from(expression: FunctionFunctionExpr) -> Self {
        Self::function(expression)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayFunctionExpr, BoolFunctionExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind,
        FunctionFunctionExpr, IntFunctionExpr, ListFunctionExpr, NilFunctionExpr,
        StringFunctionExpr, TupleFunctionExpr, UtfCodepointFunctionExpr,
    };
    use crate::plan::{
        BitArrayFunctionId, BitArrayFunctionReference, BoolFunctionId, BoolFunctionReference,
        FloatFunctionId, FloatFunctionReference, FunctionFunctionId, FunctionFunctionReference,
        FunctionReference, FunctionType, IntFunctionFunctionId, IntFunctionId,
        IntFunctionReference, ListFunctionId, ListFunctionReference, NilFunctionId,
        NilFunctionReference, ParamLocal, RuntimeFunctionId, StringFunctionId,
        StringFunctionReference, TupleFunctionId, TupleFunctionReference, UtfCodepointFunctionId,
        UtfCodepointFunctionReference, ValueType,
    };

    #[test]
    fn function_expr_kind_accessors() {
        assert_eq!(
            FunctionExpr::int(int_function_value()).kind(),
            &FunctionExprKind::Int(int_function_value()),
        );
        assert_eq!(
            FunctionExpr::string(string_function_value()).kind(),
            &FunctionExprKind::String(string_function_value()),
        );
        assert_eq!(
            FunctionExpr::bit_array(bit_array_function_value()).kind(),
            &FunctionExprKind::BitArray(bit_array_function_value()),
        );
        assert_eq!(
            FunctionExpr::utf_codepoint(utf_codepoint_function_value()).kind(),
            &FunctionExprKind::UtfCodepoint(utf_codepoint_function_value()),
        );
        assert_eq!(
            FunctionExpr::float(float_function_value()).kind(),
            &FunctionExprKind::Float(float_function_value()),
        );
        assert_eq!(
            FunctionExpr::bool(bool_function_value()).kind(),
            &FunctionExprKind::Bool(bool_function_value()),
        );
        assert_eq!(
            FunctionExpr::nil(nil_function_value()).kind(),
            &FunctionExprKind::Nil(nil_function_value()),
        );
        assert_eq!(
            FunctionExpr::tuple(tuple_function_value()).kind(),
            &FunctionExprKind::Tuple(tuple_function_value()),
        );
        assert_eq!(
            FunctionExpr::list(list_function_value()).kind(),
            &FunctionExprKind::List(list_function_value()),
        );
        assert_eq!(
            FunctionExpr::function(function_function_value()).kind(),
            &FunctionExprKind::Function(function_function_value()),
        );
    }

    #[test]
    fn typed_function_family_conversions_reject_other_return_families() {
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_typed_bit_array(),
            None,
        );
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_typed_utf_codepoint(),
            None,
        );
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_typed_custom(),
            None,
        );
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_typed_float(),
            None,
        );
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_typed_list(),
            None,
        );
    }

    #[test]
    fn function_expr_reference_preserves_runtime_family() {
        assert_eq!(
            FunctionExpr::reference(int_function_reference()).kind(),
            &FunctionExprKind::Int(int_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(string_function_reference()).kind(),
            &FunctionExprKind::String(string_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(bit_array_function_reference()).kind(),
            &FunctionExprKind::BitArray(bit_array_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(utf_codepoint_function_reference()).kind(),
            &FunctionExprKind::UtfCodepoint(utf_codepoint_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(float_function_reference()).kind(),
            &FunctionExprKind::Float(float_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(bool_function_reference()).kind(),
            &FunctionExprKind::Bool(bool_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(nil_function_reference()).kind(),
            &FunctionExprKind::Nil(nil_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(tuple_function_reference()).kind(),
            &FunctionExprKind::Tuple(tuple_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(list_function_reference()).kind(),
            &FunctionExprKind::List(list_function_value()),
        );
        assert_eq!(
            FunctionExpr::reference(function_function_reference()).kind(),
            &FunctionExprKind::Function(function_function_value()),
        );
    }

    #[test]
    fn function_expr_type_accessors() {
        assert_eq!(
            FunctionExpr::int(int_function_value()).type_(),
            int_function_type(),
        );
        assert_eq!(
            FunctionExpr::string(string_function_value()).type_(),
            string_function_type(),
        );
        assert_eq!(
            FunctionExpr::bit_array(bit_array_function_value()).type_(),
            bit_array_function_type(),
        );
        assert_eq!(
            FunctionExpr::utf_codepoint(utf_codepoint_function_value()).type_(),
            utf_codepoint_function_type(),
        );
        assert_eq!(
            FunctionExpr::float(float_function_value()).type_(),
            float_function_type(),
        );
        assert_eq!(
            FunctionExpr::bool(bool_function_value()).type_(),
            bool_function_type()
        );
        assert_eq!(FunctionExpr::nil(nil_function_value()).type_(), nil_type());
        assert_eq!(
            FunctionExpr::tuple(tuple_function_value()).type_(),
            tuple_function_type(),
        );
        assert_eq!(
            FunctionExpr::list(list_function_value()).type_(),
            list_function_type()
        );
        assert_eq!(
            FunctionExpr::function(function_function_value()).type_(),
            function_function_type(),
        );
    }

    #[test]
    fn function_expr_typed_conversions() {
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_int(),
            Some(int_function_value()),
        );
        assert_eq!(
            FunctionExpr::string(string_function_value()).into_string(),
            Some(string_function_value()),
        );
        assert_eq!(
            FunctionExpr::bit_array(bit_array_function_value()).into_bit_array(),
            Some(bit_array_function_value()),
        );
        assert_eq!(
            FunctionExpr::utf_codepoint(utf_codepoint_function_value()).into_utf_codepoint(),
            Some(utf_codepoint_function_value()),
        );
        assert_eq!(
            FunctionExpr::float(float_function_value()).into_float(),
            Some(float_function_value()),
        );
        assert_eq!(
            FunctionExpr::bool(bool_function_value()).into_bool(),
            Some(bool_function_value()),
        );
        assert_eq!(
            FunctionExpr::nil(nil_function_value()).into_nil(),
            Some(nil_function_value()),
        );
        assert_eq!(
            FunctionExpr::tuple(tuple_function_value()).into_tuple(),
            Some(tuple_function_value()),
        );
        assert_eq!(
            FunctionExpr::list(list_function_value()).into_list(),
            Some(list_function_value()),
        );
        assert_eq!(
            FunctionExpr::function(function_function_value()).into_function(),
            Some(function_function_value()),
        );

        assert_eq!(
            FunctionExpr::string(string_function_value()).into_int(),
            None
        );
        assert_eq!(FunctionExpr::int(int_function_value()).into_string(), None,);
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_bit_array(),
            None
        );
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_utf_codepoint(),
            None,
        );
        assert_eq!(FunctionExpr::int(int_function_value()).into_custom(), None,);
        assert_eq!(FunctionExpr::int(int_function_value()).into_float(), None);
        assert_eq!(FunctionExpr::int(int_function_value()).into_bool(), None);
        assert_eq!(FunctionExpr::int(int_function_value()).into_nil(), None);
        assert_eq!(FunctionExpr::int(int_function_value()).into_tuple(), None);
        assert_eq!(FunctionExpr::int(int_function_value()).into_list(), None);
        assert_eq!(
            FunctionExpr::int(int_function_value()).into_function(),
            None,
        );

        assert_eq!(
            FunctionExpr::from(int_function_value()),
            FunctionExpr::int(int_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(string_function_value()),
            FunctionExpr::string(string_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(bit_array_function_value()),
            FunctionExpr::bit_array(bit_array_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(utf_codepoint_function_value()),
            FunctionExpr::utf_codepoint(utf_codepoint_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(float_function_value()),
            FunctionExpr::float(float_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(bool_function_value()),
            FunctionExpr::bool(bool_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(nil_function_value()),
            FunctionExpr::nil(nil_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(tuple_function_value()),
            FunctionExpr::tuple(tuple_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(list_function_value()),
            FunctionExpr::list(list_function_value()),
        );
        assert_eq!(
            FunctionExpr::from(function_function_value()),
            FunctionExpr::function(function_function_value()),
        );
    }

    fn int_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        )
    }

    fn string_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![ParamLocal::string(crate::plan::StringLocalId(0))],
        )
    }

    fn bit_array_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::BitArray(BitArrayFunctionId(0)),
            vec![ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
        )
    }

    fn utf_codepoint_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(0)),
            vec![ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                0,
            ))],
        )
    }

    fn float_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Float(FloatFunctionId(0)),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        )
    }

    fn bool_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        )
    }

    fn nil_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
        )
    }

    fn tuple_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Tuple {
                id: TupleFunctionId(0),
                return_type: vec![ValueType::Int],
            },
            vec![ParamLocal::tuple(
                crate::plan::TupleLocalId(0),
                vec![ValueType::Int],
            )],
        )
    }

    fn list_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::List(ListFunctionId::from_item_type(
                0,
                crate::plan::ValueType::Int,
            )),
            vec![ParamLocal::list(crate::plan::ListLocal::int(
                crate::plan::IntListLocalId(0),
            ))],
        )
    }

    fn function_function_reference() -> FunctionReference {
        FunctionReference::new(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            },
            Vec::new(),
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(
            IntFunctionId(0),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(
            StringFunctionId(0),
            vec![ParamLocal::string(crate::plan::StringLocalId(0))],
        ))
    }

    fn bit_array_function_value() -> BitArrayFunctionExpr {
        BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            BitArrayFunctionId(0),
            vec![ParamLocal::bit_array(crate::plan::BitArrayLocalId(0))],
        ))
    }

    fn utf_codepoint_function_value() -> UtfCodepointFunctionExpr {
        UtfCodepointFunctionExpr::reference(UtfCodepointFunctionReference::new(
            UtfCodepointFunctionId(0),
            vec![ParamLocal::utf_codepoint(crate::plan::UtfCodepointLocalId(
                0,
            ))],
        ))
    }

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
        ))
    }

    fn tuple_function_value() -> TupleFunctionExpr {
        TupleFunctionExpr::reference(
            TupleFunctionReference::new(
                TupleFunctionId(0),
                vec![ParamLocal::tuple(
                    crate::plan::TupleLocalId(0),
                    vec![ValueType::Int],
                )],
            ),
            vec![ValueType::Int],
        )
    }

    fn list_function_value() -> ListFunctionExpr {
        ListFunctionExpr::reference(ListFunctionReference::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::list(crate::plan::ListLocal::int(
                crate::plan::IntListLocalId(0),
            ))],
        ))
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
            ),
            int_function_type(),
        )
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn bit_array_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray)
    }

    fn utf_codepoint_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint)
    }

    fn float_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn bool_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(vec![ValueType::Int])],
            ValueType::Tuple(vec![ValueType::Int]),
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::List(Box::new(ValueType::Int))],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn function_function_type() -> FunctionType {
        FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        )
    }
}
