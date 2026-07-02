use super::FunctionDsl;
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    FunctionType, IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, Param,
    ParamLocal, StringFunctionLocalId, StringLocalId, ValueType,
};
use ecow::EcoString;

impl FunctionDsl {
    pub(crate) fn param_int(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::int(IntLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn discard_int_param(mut self, local: usize) -> Self {
        self.params
            .push(Param::discard(ParamLocal::int(IntLocalId(local))));
        self
    }

    pub(crate) fn param_string(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::string(StringLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_float(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::float(FloatLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::bool(BoolLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_nil(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::nil(NilLocalId(local)),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_int_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::int_function(
                IntFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Int),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_string_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::string_function(
                StringFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::String),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_float_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::float_function(
                FloatFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Float),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_bool_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::bool_function(
                BoolFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Bool),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_nil_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::nil_function(
                NilFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::Nil),
            ),
            name.into(),
        ));
        self
    }

    pub(crate) fn param_function_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        type_: FunctionType,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::function_function(FunctionFunctionLocalId(local), type_),
            name.into(),
        ));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{FloatFunctionLocalId, FloatLocalId, FunctionType, ParamLocal, ValueType};
    use crate::planner::dsl::function;

    #[test]
    fn float_param_helpers_build_float_local_shapes() {
        let function = function("main", crate::planner::dsl::int(1))
            .param_float(0, "value")
            .param_float_function(0, "callback", [ValueType::Float]);

        assert_eq!(
            function.params[0].local(),
            &ParamLocal::float(FloatLocalId(0)),
        );
        assert_eq!(
            function.params[1].local(),
            &ParamLocal::float_function(
                FloatFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );
    }
}
