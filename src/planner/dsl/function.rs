mod params;
mod return_body;
mod steps;

pub(crate) use return_body::*;

use crate::plan::{FunctionId, FunctionPlan, Param, Step};
use crate::planner::context::FunctionRuntimeIds;
use ecow::EcoString;

pub(crate) struct FunctionDsl {
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: FunctionReturn,
}

pub(crate) fn function(
    name: impl Into<EcoString>,
    return_: impl Into<FunctionReturn>,
) -> FunctionDsl {
    FunctionDsl {
        name: name.into(),
        params: Vec::new(),
        steps: Vec::new(),
        return_: return_.into(),
    }
}

impl FunctionDsl {
    pub(crate) fn build(
        self,
        id: FunctionId,
        runtime_ids: &mut FunctionRuntimeIds,
    ) -> FunctionPlan {
        let return_ = self.return_.build(runtime_ids);

        FunctionPlan::new(id, self.name, self.params, self.steps, return_)
    }
}

#[cfg(test)]
mod tests {
    use super::function;
    use crate::plan::{
        BoolFunctionId, FloatFunctionId, FunctionFunctionId, FunctionId, FunctionType,
        IntFunctionFunctionId, IntFunctionId, IntLocalId, ListFunctionId, NilFunctionId,
        ParamLocal, RuntimeFunctionId, Step, StepKind, StringFunctionId, ValueType,
    };
    use crate::planner::context::FunctionRuntimeIds;
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float_function_ref, function_function_ref, function_ref, int,
        int_function_ref, list, list_function_ref, nil, nil_function_ref, string,
        string_function_ref,
    };

    #[test]
    fn function_dsl() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let function = function("main", int(1))
            .param_int(0, "a")
            .param_string(0, "b")
            .param_float(0, "float")
            .param_bool(0, "c")
            .param_nil(0, "d")
            .param_int_function(0, "f", [ValueType::Int])
            .param_string_function(0, "g", [ValueType::String])
            .param_float_function(0, "float_function", [ValueType::Float])
            .param_bool_function(0, "h", [ValueType::Bool])
            .param_nil_function(0, "i", [ValueType::Nil])
            .param_function_function(
                0,
                "j",
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            )
            .let_int(1, "x", int(2))
            .let_string(1, "y", string("a"))
            .let_bool(1, "z", bool_(true))
            .let_nil(1, "n", nil())
            .let_function_function(
                1,
                "ff",
                crate::planner::dsl::expression::local_function_function(
                    0,
                    "j",
                    FunctionType::new(
                        Vec::new(),
                        ValueType::Function(Box::new(FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::Int,
                        ))),
                    ),
                ),
            )
            .step(Step::evaluate(int(4).into()))
            .evaluate(int(3))
            .build(FunctionId::new(0), &mut runtime_ids);

        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 11);
        assert_eq!(function.steps().len(), 7);
        assert_eq!(
            function.steps()[0].kind(),
            &StepKind::LetInt {
                local: IntLocalId(1),
                name: "x".into(),
                value: int(2).into(),
            },
        );
        assert_eq!(
            function.steps()[6].kind(),
            &StepKind::Evaluate(int(3).into()),
        );
    }

    #[test]
    fn function_dsl_return_function_families() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let list_value_return = function("list_value", list([int(1)], ValueType::Int))
            .build(FunctionId::new(0), &mut runtime_ids);
        let int_return = function("int", int_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(1), &mut runtime_ids);
        let string_return = function("string", string_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(2), &mut runtime_ids);
        let float_return = function("float", float_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(3), &mut runtime_ids);
        let bool_return = function("bool", bool_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(4), &mut runtime_ids);
        let nil_return = function("nil", nil_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionId::new(5), &mut runtime_ids);
        let list_return = function(
            "list",
            list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int),
        )
        .build(FunctionId::new(6), &mut runtime_ids);
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_return = function(
            "function",
            function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                return_type.clone(),
            ),
        )
        .build(FunctionId::new(7), &mut runtime_ids);

        assert_eq!(
            list_value_return.return_().runtime_id(),
            RuntimeFunctionId::List(ListFunctionId::from_item_type(
                0,
                crate::plan::ValueType::Int
            )),
        );
        assert_eq!(
            int_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
        );
        assert_eq!(
            string_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(crate::plan::StringFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::String),
            },
        );
        assert_eq!(
            float_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Float(crate::plan::FloatFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Float),
            },
        );
        assert_eq!(
            bool_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(crate::plan::BoolFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Bool),
            },
        );
        assert_eq!(
            nil_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(crate::plan::NilFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Nil),
            },
        );
        assert_eq!(
            list_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(crate::plan::ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                )),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::Int)),
                ),
            },
        );
        assert_eq!(
            function_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(crate::plan::FunctionFunctionFunctionId(0)),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(return_type)),
                ),
            },
        );
    }

    #[test]
    fn function_dsl_generic_function_return_families() {
        let mut runtime_ids = FunctionRuntimeIds::default();
        let int_return = function(
            "int",
            function_ref(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(0), &mut runtime_ids);
        let string_return = function(
            "string",
            function_ref(
                RuntimeFunctionId::String(StringFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(1), &mut runtime_ids);
        let float_return = function(
            "float",
            function_ref(
                RuntimeFunctionId::Float(FloatFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(2), &mut runtime_ids);
        let bool_return = function(
            "bool",
            function_ref(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(3), &mut runtime_ids);
        let nil_return = function(
            "nil",
            function_ref(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(4), &mut runtime_ids);
        let list_return = function(
            "list",
            function_ref(
                RuntimeFunctionId::List(ListFunctionId::from_item_type(
                    0,
                    crate::plan::ValueType::Int,
                )),
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(5), &mut runtime_ids);
        let function_return = function(
            "function",
            function_ref(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                Vec::<ParamLocal>::new(),
            ),
        )
        .build(FunctionId::new(6), &mut runtime_ids);

        assert_eq!(
            int_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Int),
            },
        );
        assert_eq!(
            string_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::String(crate::plan::StringFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::String),
            },
        );
        assert_eq!(
            float_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Float(crate::plan::FloatFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Float),
            },
        );
        assert_eq!(
            bool_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Bool(crate::plan::BoolFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Bool),
            },
        );
        assert_eq!(
            nil_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Nil(crate::plan::NilFunctionFunctionId(0)),
                return_type: FunctionType::new(Vec::new(), ValueType::Nil),
            },
        );
        assert_eq!(
            list_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(crate::plan::ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                )),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::Int)),
                ),
            },
        );
        assert_eq!(
            function_return.return_().runtime_id(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(crate::plan::FunctionFunctionFunctionId(0)),
                return_type: FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                ),
            },
        );
    }
}
