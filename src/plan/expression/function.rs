mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;

use crate::plan::{FunctionType, FunctionValue, FunctionValueKind};

pub use self::{
    bool::BoolFunctionExpr, float::FloatFunctionExpr, int::IntFunctionExpr, list::ListFunctionExpr,
    nil::NilFunctionExpr, returning_function::FunctionFunctionExpr, string::StringFunctionExpr,
    tuple::TupleFunctionExpr,
};
pub(crate) use self::{
    bool::BoolFunctionExprKind, float::FloatFunctionExprKind, int::IntFunctionExprKind,
    list::ListFunctionExprKind, nil::NilFunctionExprKind,
    returning_function::FunctionFunctionExprKind, string::StringFunctionExprKind,
    tuple::TupleFunctionExprKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    kind: FunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionExprKind {
    Int(IntFunctionExpr),
    String(StringFunctionExpr),
    Float(FloatFunctionExpr),
    Bool(BoolFunctionExpr),
    Nil(NilFunctionExpr),
    Tuple(TupleFunctionExpr),
    List(ListFunctionExpr),
    Function(FunctionFunctionExpr),
}

impl FunctionExpr {
    pub(crate) fn value(value: FunctionValue) -> Self {
        match value.kind() {
            FunctionValueKind::Int(value) => Self::int(IntFunctionExpr::value(value.clone())),
            FunctionValueKind::String(value) => {
                Self::string(StringFunctionExpr::value(value.clone()))
            }
            FunctionValueKind::Float(value) => Self::float(FloatFunctionExpr::value(value.clone())),
            FunctionValueKind::Bool(value) => Self::bool(BoolFunctionExpr::value(value.clone())),
            FunctionValueKind::Nil(value) => Self::nil(NilFunctionExpr::value(value.clone())),
            FunctionValueKind::Tuple(value) => Self::tuple(TupleFunctionExpr::value(value.clone())),
            FunctionValueKind::List(value) => Self::list(ListFunctionExpr::value(value.clone())),
            FunctionValueKind::Function(value) => {
                Self::function(FunctionFunctionExpr::value(value.clone()))
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
        BoolFunctionExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
        IntFunctionExpr, ListFunctionExpr, NilFunctionExpr, StringFunctionExpr, TupleFunctionExpr,
    };
    use crate::plan::{
        BoolFunctionId, BoolFunctionValue, FloatFunctionId, FloatFunctionValue, FunctionFunctionId,
        FunctionFunctionValue, FunctionType, FunctionValue, IntFunctionFunctionId, IntFunctionId,
        IntFunctionValue, ListFunctionId, ListFunctionValue, NilFunctionId, NilFunctionValue,
        ParamLocal, RuntimeFunctionId, StringFunctionId, StringFunctionValue, TupleFunctionId,
        TupleFunctionValue, ValueType,
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
    fn function_expr_value_preserves_runtime_family() {
        assert_eq!(
            FunctionExpr::value(int_runtime_function_value()).kind(),
            &FunctionExprKind::Int(int_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(string_runtime_function_value()).kind(),
            &FunctionExprKind::String(string_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(float_runtime_function_value()).kind(),
            &FunctionExprKind::Float(float_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(bool_runtime_function_value()).kind(),
            &FunctionExprKind::Bool(bool_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(nil_runtime_function_value()).kind(),
            &FunctionExprKind::Nil(nil_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(tuple_runtime_function_value()).kind(),
            &FunctionExprKind::Tuple(tuple_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(list_runtime_function_value()).kind(),
            &FunctionExprKind::List(list_function_value()),
        );
        assert_eq!(
            FunctionExpr::value(function_runtime_function_value()).kind(),
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

    fn int_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        )
    }

    fn string_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![ParamLocal::string(crate::plan::StringLocalId(0))],
        )
    }

    fn float_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Float(FloatFunctionId(0)),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        )
    }

    fn bool_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        )
    }

    fn nil_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
        )
    }

    fn tuple_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
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

    fn list_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::List(ListFunctionId::from_item_type(
                0,
                crate::plan::ValueType::Int,
            )),
            vec![ParamLocal::list(crate::plan::ListLocal::int(
                crate::plan::IntListLocalId(0),
            ))],
        )
    }

    fn function_runtime_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            },
            Vec::new(),
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(crate::plan::IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(crate::plan::StringLocalId(0))],
        ))
    }

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(crate::plan::NilLocalId(0))],
        ))
    }

    fn tuple_function_value() -> TupleFunctionExpr {
        TupleFunctionExpr::value(TupleFunctionValue::new(
            TupleFunctionId(0),
            vec![ParamLocal::tuple(
                crate::plan::TupleLocalId(0),
                vec![ValueType::Int],
            )],
            vec![ValueType::Int],
        ))
    }

    fn list_function_value() -> ListFunctionExpr {
        ListFunctionExpr::value(ListFunctionValue::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::list(crate::plan::ListLocal::int(
                crate::plan::IntListLocalId(0),
            ))],
        ))
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            int_function_type(),
        ))
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
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
