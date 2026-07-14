mod bit_array;
mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod utf_codepoint;

use crate::plan::{
    BitArrayFunctionReference, BoolFunctionReference, FloatFunctionReference,
    FunctionFunctionReference, FunctionReference, FunctionType, IntFunctionReference,
    ListFunctionReference, NilFunctionReference, RuntimeFunctionId, StringFunctionReference,
    TupleFunctionReference, UtfCodepointFunctionReference,
};

pub use self::{
    bit_array::BitArrayFunctionExpr, bool::BoolFunctionExpr, float::FloatFunctionExpr,
    int::IntFunctionExpr, list::ListFunctionExpr, nil::NilFunctionExpr,
    returning_function::FunctionFunctionExpr, string::StringFunctionExpr, tuple::TupleFunctionExpr,
    utf_codepoint::UtfCodepointFunctionExpr,
};
pub(crate) use self::{
    bit_array::BitArrayFunctionExprKind, bool::BoolFunctionExprKind, float::FloatFunctionExprKind,
    int::IntFunctionExprKind, list::ListFunctionExprKind, nil::NilFunctionExprKind,
    returning_function::FunctionFunctionExprKind, string::StringFunctionExprKind,
    tuple::TupleFunctionExprKind, utf_codepoint::UtfCodepointFunctionExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    BitArray(BitArrayFunctionExpr),
    UtfCodepoint(UtfCodepointFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    List(ListFunctionExpr),
    Function(FunctionFunctionExpr),
}

impl FunctionExpr {
    pub(crate) fn reference(reference: FunctionReference) -> Self {
        let (runtime_id, params) = reference.into_parts();
        match runtime_id {
            RuntimeFunctionId::Int(id) => Self::int(IntFunctionExpr::reference(
                IntFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::Float(id) => Self::float(FloatFunctionExpr::reference(
                FloatFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::String(id) => Self::string(StringFunctionExpr::reference(
                StringFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::BitArray(id) => Self::bit_array(BitArrayFunctionExpr::reference(
                BitArrayFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::UtfCodepoint(id) => Self::utf_codepoint(
                UtfCodepointFunctionExpr::reference(UtfCodepointFunctionReference::new(id, params)),
            ),
            RuntimeFunctionId::Bool(id) => Self::bool(BoolFunctionExpr::reference(
                BoolFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::Nil(id) => Self::nil(NilFunctionExpr::reference(
                NilFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::Tuple { id, return_type } => Self::tuple(
                TupleFunctionExpr::reference(TupleFunctionReference::new(id, params), return_type),
            ),
            RuntimeFunctionId::List(id) => Self::list(ListFunctionExpr::reference(
                ListFunctionReference::new(id, params),
            )),
            RuntimeFunctionId::Function { id, return_type } => {
                Self::function(FunctionFunctionExpr::reference(
                    FunctionFunctionReference::new(id, params),
                    return_type,
                ))
            }
        }
    }

    pub(crate) fn int(expression: IntFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Int(expression),
        }
    }

    pub(crate) fn string(expression: StringFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::String(expression),
        }
    }

    pub(crate) fn bit_array(expression: BitArrayFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::BitArray(expression),
        }
    }

    pub(crate) fn utf_codepoint(expression: UtfCodepointFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::UtfCodepoint(expression),
        }
    }

    pub(crate) fn float(expression: FloatFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Float(expression),
        }
    }

    pub(crate) fn bool(expression: BoolFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Bool(expression),
        }
    }

    pub(crate) fn nil(expression: NilFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Nil(expression),
        }
    }

    pub(crate) fn tuple(expression: TupleFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Tuple(expression),
        }
    }

    pub(crate) fn list(expression: ListFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::List(expression),
        }
    }

    pub(crate) fn function(expression: FunctionFunctionExpr) -> Self {
        Self {
            kind: FunctionExprKind::Function(expression),
        }
    }

    pub fn type_(&self) -> &FunctionType {
        match &self.kind {
            FunctionExprKind::Int(expression) => expression.type_(),
            FunctionExprKind::String(expression) => expression.type_(),
            FunctionExprKind::BitArray(expression) => expression.type_(),
            FunctionExprKind::UtfCodepoint(expression) => expression.type_(),
            FunctionExprKind::Float(expression) => expression.type_(),
            FunctionExprKind::Bool(expression) => expression.type_(),
            FunctionExprKind::Nil(expression) => expression.type_(),
            FunctionExprKind::Tuple(expression) => expression.type_(),
            FunctionExprKind::List(expression) => expression.type_(),
            FunctionExprKind::Function(expression) => expression.type_(),
        }
    }

    pub(crate) fn kind(&self) -> &FunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_kind(self) -> FunctionExprKind {
        self.kind
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
            &int_function_type(),
        );
        assert_eq!(
            FunctionExpr::string(string_function_value()).type_(),
            &string_function_type(),
        );
        assert_eq!(
            FunctionExpr::bit_array(bit_array_function_value()).type_(),
            &bit_array_function_type(),
        );
        assert_eq!(
            FunctionExpr::utf_codepoint(utf_codepoint_function_value()).type_(),
            &utf_codepoint_function_type(),
        );
        assert_eq!(
            FunctionExpr::float(float_function_value()).type_(),
            &float_function_type(),
        );
        assert_eq!(
            FunctionExpr::bool(bool_function_value()).type_(),
            &bool_function_type()
        );
        assert_eq!(FunctionExpr::nil(nil_function_value()).type_(), &nil_type());
        assert_eq!(
            FunctionExpr::tuple(tuple_function_value()).type_(),
            &tuple_function_type(),
        );
        assert_eq!(
            FunctionExpr::list(list_function_value()).type_(),
            &list_function_type()
        );
        assert_eq!(
            FunctionExpr::function(function_function_value()).type_(),
            &function_function_type(),
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
