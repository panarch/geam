mod params;
mod return_body;
mod steps;

pub(crate) use return_body::*;

use crate::plan::{FunctionTemplate, FunctionTemplateId, Param, ParamSlot, Step};
use ecow::EcoString;

pub(crate) struct FunctionDsl {
    name: EcoString,
    params: Vec<Param>,
    captures: Vec<ParamSlot>,
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
        captures: Vec::new(),
        steps: Vec::new(),
        return_: return_.into(),
    }
}

impl FunctionDsl {
    pub(crate) fn capture(mut self, slot: ParamSlot) -> Self {
        self.captures.push(slot);
        self
    }

    pub(crate) fn build(self, id: FunctionTemplateId) -> FunctionTemplate {
        let return_ = self.return_.build();

        FunctionTemplate::with_captures(
            id,
            self.name,
            self.params,
            self.captures,
            self.steps,
            return_,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::function;
    use crate::plan::{
        FunctionFunctionId, FunctionTemplateId, FunctionType, IntFunctionFunctionId, IntLocalId,
        ParamLocal, Step, StepKind, ValueType,
    };
    use crate::planner::dsl::expression::{
        bool_, bool_function_ref, float_function_ref, function_function_ref, int, int_function_ref,
        list, list_function_ref, nil, nil_function_ref, string, string_function_ref,
    };

    #[test]
    fn function_dsl() {
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
                crate::plan::FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
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
                    crate::plan::FunctionFunctionType::new(
                        Vec::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                ),
            )
            .step(Step::evaluate(int(4).into()))
            .evaluate(int(3))
            .build(FunctionTemplateId::new(0));

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
        let list_value_return = function("list_value", list([int(1)], ValueType::Int))
            .build(FunctionTemplateId::new(0));
        let int_return = function("int", int_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionTemplateId::new(1));
        let string_return = function("string", string_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionTemplateId::new(2));
        let float_return = function("float", float_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionTemplateId::new(3));
        let bool_return = function("bool", bool_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionTemplateId::new(4));
        let nil_return = function("nil", nil_function_ref(0, Vec::<ParamLocal>::new()))
            .build(FunctionTemplateId::new(5));
        let list_return = function(
            "list",
            list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int),
        )
        .build(FunctionTemplateId::new(6));
        let return_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_return = function(
            "function",
            function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                return_type.clone(),
            ),
        )
        .build(FunctionTemplateId::new(7));

        assert_eq!(
            list_value_return.return_().value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_eq!(
            int_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        assert_eq!(
            string_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String,))),
        );
        assert_eq!(
            float_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float,))),
        );
        assert_eq!(
            bool_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
        );
        assert_eq!(
            nil_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
        );
        assert_eq!(
            list_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ))),
        );
        assert_eq!(
            function_return.return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(return_type)),
            ))),
        );
    }
}
