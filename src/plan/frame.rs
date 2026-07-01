mod args;
mod expression;
mod function;
mod return_;
mod step;

use super::function::{Param, ParamLocal, ReturnExpr};
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId,
    NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
};
use super::step::Step;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FrameLayout {
    ints: usize,
    strings: usize,
    bools: usize,
    nils: usize,
    int_functions: usize,
    string_functions: usize,
    bool_functions: usize,
    nil_functions: usize,
    function_functions: usize,
}

impl FrameLayout {
    pub(crate) fn from_function_parts(
        params: &[Param],
        steps: &[Step],
        return_: &ReturnExpr,
    ) -> Self {
        let mut layout = Self::default();

        for param in params {
            layout.include_local(param.local());
        }
        layout.include_steps(steps);
        layout.include_return_expr(return_);

        layout
    }

    pub(crate) fn include_local(&mut self, local: &ParamLocal) {
        match local {
            ParamLocal::Int(local) => self.include_int(*local),
            ParamLocal::String(local) => self.include_string(*local),
            ParamLocal::Bool(local) => self.include_bool(*local),
            ParamLocal::Nil(local) => self.include_nil(*local),
            ParamLocal::IntFunction { local, .. } => self.include_int_function(*local),
            ParamLocal::StringFunction { local, .. } => self.include_string_function(*local),
            ParamLocal::BoolFunction { local, .. } => self.include_bool_function(*local),
            ParamLocal::NilFunction { local, .. } => self.include_nil_function(*local),
            ParamLocal::FunctionFunction { local, .. } => self.include_function_function(*local),
        }
    }

    pub(crate) fn include_int(&mut self, local: IntLocalId) {
        self.ints = self.ints.max(local.0 + 1);
    }

    pub(crate) fn include_string(&mut self, local: StringLocalId) {
        self.strings = self.strings.max(local.0 + 1);
    }

    pub(crate) fn include_bool(&mut self, local: BoolLocalId) {
        self.bools = self.bools.max(local.0 + 1);
    }

    pub(crate) fn include_nil(&mut self, local: NilLocalId) {
        self.nils = self.nils.max(local.0 + 1);
    }

    pub(crate) fn include_int_function(&mut self, local: IntFunctionLocalId) {
        self.int_functions = self.int_functions.max(local.0 + 1);
    }

    pub(crate) fn include_string_function(&mut self, local: StringFunctionLocalId) {
        self.string_functions = self.string_functions.max(local.0 + 1);
    }

    pub(crate) fn include_bool_function(&mut self, local: BoolFunctionLocalId) {
        self.bool_functions = self.bool_functions.max(local.0 + 1);
    }

    pub(crate) fn include_nil_function(&mut self, local: NilFunctionLocalId) {
        self.nil_functions = self.nil_functions.max(local.0 + 1);
    }

    pub(crate) fn include_function_function(&mut self, local: FunctionFunctionLocalId) {
        self.function_functions = self.function_functions.max(local.0 + 1);
    }

    pub(crate) fn ints(self) -> usize {
        self.ints
    }

    pub(crate) fn strings(self) -> usize {
        self.strings
    }

    pub(crate) fn bools(self) -> usize {
        self.bools
    }

    #[cfg(test)]
    pub(crate) fn nils(self) -> usize {
        self.nils
    }

    pub(crate) fn int_functions(self) -> usize {
        self.int_functions
    }

    pub(crate) fn string_functions(self) -> usize {
        self.string_functions
    }

    pub(crate) fn bool_functions(self) -> usize {
        self.bool_functions
    }

    pub(crate) fn nil_functions(self) -> usize {
        self.nil_functions
    }

    pub(crate) fn function_functions(self) -> usize {
        self.function_functions
    }
}

#[cfg(test)]
pub(super) mod test_helpers {
    use crate::plan::{
        BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, BoolLocalId, FunctionType,
        IntFunctionExpr, IntFunctionId, IntFunctionValue, IntLocalId, NilFunctionExpr,
        NilFunctionId, NilFunctionValue, NilLocalId, ParamLocal, StringFunctionExpr,
        StringFunctionId, StringFunctionValue, StringLocalId, ValueType,
    };

    pub(super) fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    pub(super) fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    pub(super) fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    pub(super) fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    pub(super) fn function_returning_int_function_type() -> FunctionType {
        FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(int_function_expr().type_().clone())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolLocalId, IntFunctionLocalId, IntLocalId, NilFunctionLocalId, NilLocalId, ParamLocal,
        StringLocalId,
    };

    #[test]
    fn frame_layout_derived_surface_is_covered() {
        let layout = FrameLayout::default();
        let cloned = clone_value(&layout);

        assert_eq!(layout, cloned);
        assert_eq!(
            format!("{layout:?}"),
            "FrameLayout { ints: 0, strings: 0, bools: 0, nils: 0, int_functions: 0, string_functions: 0, bool_functions: 0, nil_functions: 0, function_functions: 0 }",
        );
    }

    fn clone_value<T: Clone>(value: &T) -> T {
        value.clone()
    }

    #[test]
    fn frame_layout_includes_local_ids() {
        let mut layout = FrameLayout::default();

        layout.include_local(&ParamLocal::int(IntLocalId(1)));
        layout.include_local(&ParamLocal::string(StringLocalId(2)));
        layout.include_local(&ParamLocal::bool(BoolLocalId(3)));
        layout.include_local(&ParamLocal::nil(NilLocalId(4)));
        layout.include_int_function(IntFunctionLocalId(5));
        layout.include_nil_function(NilFunctionLocalId(6));

        assert_eq!(layout.ints(), 2);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.bools(), 4);
        assert_eq!(layout.nils(), 5);
        assert_eq!(layout.int_functions(), 6);
        assert_eq!(layout.nil_functions(), 7);
    }
}
