use super::{Bool, Float, Function, FunctionFunction, Int, IntFunction, Nil, String};
use crate::plan::{
    BoolExpr, BoolFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind,
    FunctionFunctionExpr, IntExpr, IntFunctionExpr, NilExpr, NilFunctionExpr, Step, StringExpr,
    StringFunctionExpr,
};

pub(crate) fn block_int(steps: impl IntoIterator<Item = Step>, return_: Int) -> Int {
    Int(IntExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_string(steps: impl IntoIterator<Item = Step>, return_: String) -> String {
    String(StringExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_float(steps: impl IntoIterator<Item = Step>, return_: Float) -> Float {
    Float(FloatExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_bool(steps: impl IntoIterator<Item = Step>, return_: Bool) -> Bool {
    Bool(BoolExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_nil(steps: impl IntoIterator<Item = Step>, return_: Nil) -> Nil {
    Nil(NilExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_function(steps: Vec<Step>, return_: Function) -> Function {
    Function(match FunctionExpr::from(return_).into_kind() {
        FunctionExprKind::Int(return_) => FunctionExpr::int(IntFunctionExpr::block(steps, return_)),
        FunctionExprKind::String(return_) => {
            FunctionExpr::string(StringFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Float(return_) => {
            FunctionExpr::float(FloatFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Bool(return_) => {
            FunctionExpr::bool(BoolFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Nil(return_) => FunctionExpr::nil(NilFunctionExpr::block(steps, return_)),
        FunctionExprKind::Function(return_) => {
            FunctionExpr::function(FunctionFunctionExpr::block(steps, return_))
        }
    })
}

pub(crate) fn block_function_function(
    steps: impl IntoIterator<Item = Step>,
    return_: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_int_function(
    steps: impl IntoIterator<Item = Step>,
    return_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        block_bool, block_float, block_function, block_function_function, block_int,
        block_int_function, block_nil, block_string,
    };
    use crate::plan::{
        BoolExprKind, FloatExprKind, FunctionExpr, FunctionExprKind, FunctionFunctionId,
        FunctionType, IntExprKind, IntFunctionExprKind, IntFunctionFunctionId, NilExprKind,
        ParamLocal, RuntimeFunctionId, StringExprKind, ValueType,
    };
    use crate::planner::dsl::expression::{
        Function, bool_, float, function_function_ref, function_ref, int, int_function_ref,
        let_bool_step, let_int_step, let_nil_step, let_string_step, local_bool, local_int,
        local_nil, local_string, nil, string,
    };

    #[test]
    fn primitive_block_helpers_build_block_shapes() {
        assert!(matches!(
            block_int([let_int_step(0, "x", int(1))], local_int(0, "x"))
                .0
                .kind(),
            IntExprKind::Block { .. },
        ));
        assert!(matches!(
            block_string([let_string_step(0, "x", string("a"))], local_string(0, "x"))
                .0
                .kind(),
            StringExprKind::Block { .. },
        ));
        assert!(matches!(
            block_float([], float(1.0)).0.kind(),
            FloatExprKind::Block { .. },
        ));
        assert!(matches!(
            block_bool([let_bool_step(0, "x", bool_(true))], local_bool(0, "x"))
                .0
                .kind(),
            BoolExprKind::Block { .. },
        ));
        assert!(matches!(
            block_nil([let_nil_step(0, "x", nil())], local_nil(0, "x"))
                .0
                .kind(),
            NilExprKind::Block { .. },
        ));
    }

    #[test]
    fn function_block_helpers_preserve_return_family() {
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::String(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Float(crate::plan::FloatFunctionId(0)),
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Float(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                    [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Bool(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                function_ref(
                    RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                    [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Nil(_),
        ));
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert!(matches!(
            FunctionExpr::from(block_function(
                vec![],
                Function::from(function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
            ))
            .kind(),
            FunctionExprKind::Function(_),
        ));
        assert!(matches!(
            block_int_function(
                [],
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .0
            .kind(),
            IntFunctionExprKind::Block { .. },
        ));
        assert_eq!(
            block_function_function(
                [],
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                ),
            )
            .0
            .type_(),
            &FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type)),
            ),
        );
    }
}
