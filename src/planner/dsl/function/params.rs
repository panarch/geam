use super::FunctionDsl;
use crate::plan::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    FunctionType, IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, Param,
    ParamLocal, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueType,
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

    pub(crate) fn param_utf_codepoint(mut self, local: usize, name: impl Into<EcoString>) -> Self {
        self.params.push(Param::named(
            ParamLocal::utf_codepoint(UtfCodepointLocalId(local)),
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

    pub(crate) fn param_tuple(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        type_: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::tuple(TupleLocalId(local), type_.into_iter().collect()),
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

    pub(crate) fn param_utf_codepoint_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::utf_codepoint_function(
                UtfCodepointFunctionLocalId(local),
                FunctionType::new(arguments.into_iter().collect(), ValueType::UtfCodepoint),
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

    pub(crate) fn param_tuple_function(
        mut self,
        local: usize,
        name: impl Into<EcoString>,
        arguments: impl IntoIterator<Item = ValueType>,
        return_type: impl IntoIterator<Item = ValueType>,
    ) -> Self {
        self.params.push(Param::named(
            ParamLocal::tuple_function(
                TupleFunctionLocalId(local),
                FunctionType::new(
                    arguments.into_iter().collect(),
                    ValueType::Tuple(return_type.into_iter().collect()),
                ),
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
    use crate::plan::{
        FloatFunctionLocalId, FloatLocalId, FunctionType, ParamLocal, TupleFunctionLocalId,
        TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueType,
    };
    use crate::planner::dsl::function;

    #[test]
    fn float_param_helpers_build_float_local_shapes() {
        let function = function("main", crate::planner::dsl::int(1))
            .param_float(0, "value")
            .param_utf_codepoint(0, "codepoint")
            .param_tuple(0, "pair", [ValueType::Int])
            .param_float_function(0, "callback", [ValueType::Float])
            .param_utf_codepoint_function(0, "codepoint_callback", [ValueType::UtfCodepoint])
            .param_tuple_function(
                0,
                "tuple_callback",
                [ValueType::Tuple(vec![ValueType::Int])],
                [ValueType::String],
            );

        assert_eq!(
            function.params[0].local(),
            &ParamLocal::float(FloatLocalId(0)),
        );
        assert_eq!(
            function.params[1].local(),
            &ParamLocal::utf_codepoint(UtfCodepointLocalId(0)),
        );
        assert_eq!(
            function.params[2].local(),
            &ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int]),
        );
        assert_eq!(
            function.params[3].local(),
            &ParamLocal::float_function(
                FloatFunctionLocalId(0),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
        );
        assert_eq!(
            function.params[4].local(),
            &ParamLocal::utf_codepoint_function(
                UtfCodepointFunctionLocalId(0),
                FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint,),
            ),
        );
        assert_eq!(
            function.params[5].local(),
            &ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                FunctionType::new(
                    vec![ValueType::Tuple(vec![ValueType::Int])],
                    ValueType::Tuple(vec![ValueType::String]),
                ),
            ),
        );
    }
}
