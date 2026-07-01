use super::{
    Bool, BoolFunction, Function, FunctionFunction, Int, IntFunction, IntoParamLocal,
    IntoValueType, Nil, NilFunction, String, StringFunction,
};
use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, FunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr,
    LocalId, NilExpr, NilFunctionExpr, ParamLocal, StringExpr, StringFunctionExpr, ValueType,
};

impl From<Int> for Expr {
    fn from(value: Int) -> Self {
        Self::int(value.into())
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Self::string(value.into())
    }
}

impl From<Bool> for Expr {
    fn from(value: Bool) -> Self {
        Self::bool(value.into())
    }
}

impl From<Nil> for Expr {
    fn from(value: Nil) -> Self {
        Self::nil(value.into())
    }
}

impl From<Function> for Expr {
    fn from(value: Function) -> Self {
        Self::function(value.into())
    }
}

impl From<Int> for IntExpr {
    fn from(value: Int) -> Self {
        value.0
    }
}

impl From<String> for StringExpr {
    fn from(value: String) -> Self {
        value.0
    }
}

impl From<Bool> for BoolExpr {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl From<Nil> for NilExpr {
    fn from(value: Nil) -> Self {
        value.0
    }
}

impl From<Function> for FunctionExpr {
    fn from(value: Function) -> Self {
        value.0
    }
}

impl From<IntFunction> for Function {
    fn from(value: IntFunction) -> Self {
        Function(FunctionExpr::int(value.into()))
    }
}

impl From<IntFunction> for Expr {
    fn from(value: IntFunction) -> Self {
        Self::function(FunctionExpr::int(value.into()))
    }
}

impl From<IntFunction> for FunctionExpr {
    fn from(value: IntFunction) -> Self {
        FunctionExpr::int(value.into())
    }
}

impl From<IntFunction> for IntFunctionExpr {
    fn from(value: IntFunction) -> Self {
        value.0
    }
}

impl From<StringFunction> for StringFunctionExpr {
    fn from(value: StringFunction) -> Self {
        value.0
    }
}

impl From<StringFunction> for Expr {
    fn from(value: StringFunction) -> Self {
        Self::function(FunctionExpr::string(value.into()))
    }
}

impl From<BoolFunction> for BoolFunctionExpr {
    fn from(value: BoolFunction) -> Self {
        value.0
    }
}

impl From<BoolFunction> for Expr {
    fn from(value: BoolFunction) -> Self {
        Self::function(FunctionExpr::bool(value.into()))
    }
}

impl From<NilFunction> for NilFunctionExpr {
    fn from(value: NilFunction) -> Self {
        value.0
    }
}

impl From<NilFunction> for Expr {
    fn from(value: NilFunction) -> Self {
        Self::function(FunctionExpr::nil(value.into()))
    }
}

impl From<FunctionFunction> for Function {
    fn from(value: FunctionFunction) -> Self {
        Function(FunctionExpr::function(value.into()))
    }
}

impl From<FunctionFunction> for Expr {
    fn from(value: FunctionFunction) -> Self {
        Self::function(FunctionExpr::function(value.into()))
    }
}

impl From<FunctionFunction> for FunctionExpr {
    fn from(value: FunctionFunction) -> Self {
        FunctionExpr::function(value.into())
    }
}

impl From<FunctionFunction> for FunctionFunctionExpr {
    fn from(value: FunctionFunction) -> Self {
        value.0
    }
}

impl IntoValueType for ValueType {
    fn into_value_type(self) -> ValueType {
        self
    }
}

impl IntoValueType for LocalId {
    fn into_value_type(self) -> ValueType {
        match self {
            LocalId::Int(_) => ValueType::Int,
            LocalId::String(_) => ValueType::String,
            LocalId::Bool(_) => ValueType::Bool,
            LocalId::Nil(_) => ValueType::Nil,
        }
    }
}

impl IntoValueType for ParamLocal {
    fn into_value_type(self) -> ValueType {
        self.value_type()
    }
}

impl IntoParamLocal for LocalId {
    fn into_param_local(self) -> ParamLocal {
        match self {
            LocalId::Int(local) => ParamLocal::int(local),
            LocalId::String(local) => ParamLocal::string(local),
            LocalId::Bool(local) => ParamLocal::bool(local),
            LocalId::Nil(local) => ParamLocal::nil(local),
        }
    }
}

impl IntoParamLocal for ParamLocal {
    fn into_param_local(self) -> ParamLocal {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{IntoParamLocal, IntoValueType};
    use crate::plan::{
        Expr, ExprKind, FunctionExpr, FunctionExprKind, FunctionFunctionId, FunctionType,
        IntFunctionFunctionId, ParamLocal, ValueType,
    };
    use crate::planner::dsl::expression::{
        Function, bool_, function_function_ref, int, int_function_ref, nil, string,
    };

    #[test]
    fn value_type_conversions() {
        assert_eq!(ValueType::Int.into_value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::int(crate::plan::IntLocalId(0)).into_value_type(),
            ValueType::Int,
        );
        assert_eq!(
            crate::plan::LocalId::Int(crate::plan::IntLocalId(0)).into_value_type(),
            ValueType::Int,
        );
        assert_eq!(
            crate::plan::LocalId::Int(crate::plan::IntLocalId(0)).into_param_local(),
            ParamLocal::int(crate::plan::IntLocalId(0)),
        );
    }

    #[test]
    fn wrapper_conversions_preserve_result_families() {
        assert!(matches!(Expr::from(int(1)).kind(), ExprKind::Int(_)));
        assert!(matches!(
            Expr::from(string("a")).kind(),
            ExprKind::String(_)
        ));
        assert!(matches!(Expr::from(bool_(true)).kind(), ExprKind::Bool(_)));
        assert!(matches!(Expr::from(nil()).kind(), ExprKind::Nil(_)));
        assert!(matches!(
            Expr::from(int_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(Function::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            )))
            .kind(),
            FunctionExprKind::Function(_),
        ));
    }
}
