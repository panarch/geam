mod args;
mod expression;
mod function;
mod return_;
mod step;

use std::borrow::Borrow;

use super::function::{Param, ParamLocal, ReturnExpr};
use super::id::{
    BoolFunctionLocalId, BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId,
    FloatLocalId, FunctionFunctionLocalId, FunctionListLocalId, IntFunctionLocalId, IntListLocalId,
    IntLocalId, ListFunctionLocalId, ListListLocalId, ListLocal, NilFunctionLocalId,
    NilListLocalId, NilLocalId, StringFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
};
use super::step::Step;
use super::value::ValueType;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FrameLayout {
    ints: usize,
    floats: usize,
    strings: usize,
    bools: usize,
    nils: usize,
    tuples: usize,
    int_lists: usize,
    string_lists: usize,
    float_lists: usize,
    bool_lists: usize,
    nil_lists: usize,
    tuple_lists: Vec<Vec<ValueType>>,
    list_lists: Vec<ValueType>,
    function_lists: Vec<super::FunctionType>,
    int_functions: usize,
    float_functions: usize,
    string_functions: usize,
    bool_functions: usize,
    nil_functions: usize,
    tuple_functions: usize,
    list_functions: usize,
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
            ParamLocal::Float(local) => self.include_float(*local),
            ParamLocal::String(local) => self.include_string(*local),
            ParamLocal::Bool(local) => self.include_bool(*local),
            ParamLocal::Nil(local) => self.include_nil(*local),
            ParamLocal::Tuple { local, .. } => self.include_tuple(*local),
            ParamLocal::List(local) => self.include_list(local),
            ParamLocal::IntFunction { local, .. } => self.include_int_function(*local),
            ParamLocal::FloatFunction { local, .. } => self.include_float_function(*local),
            ParamLocal::StringFunction { local, .. } => self.include_string_function(*local),
            ParamLocal::BoolFunction { local, .. } => self.include_bool_function(*local),
            ParamLocal::NilFunction { local, .. } => self.include_nil_function(*local),
            ParamLocal::TupleFunction { local, .. } => self.include_tuple_function(*local),
            ParamLocal::ListFunction { local, .. } => self.include_list_function(*local),
            ParamLocal::FunctionFunction { local, .. } => self.include_function_function(*local),
        }
    }

    pub(crate) fn include_int(&mut self, local: IntLocalId) {
        self.ints = self.ints.max(local.0 + 1);
    }

    pub(crate) fn include_float(&mut self, local: FloatLocalId) {
        self.floats = self.floats.max(local.0 + 1);
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

    pub(crate) fn include_tuple(&mut self, local: TupleLocalId) {
        self.tuples = self.tuples.max(local.0 + 1);
    }

    pub(crate) fn include_list(&mut self, local: impl Borrow<ListLocal>) {
        let local = local.borrow();
        match local {
            ListLocal::Int(local) => self.include_int_list(*local),
            ListLocal::String(local) => self.include_string_list(*local),
            ListLocal::Float(local) => self.include_float_list(*local),
            ListLocal::Bool(local) => self.include_bool_list(*local),
            ListLocal::Nil(local) => self.include_nil_list(*local),
            ListLocal::Tuple { local, item_type } => {
                self.include_tuple_list(*local, item_type.clone());
            }
            ListLocal::List { local, item_type } => {
                self.include_list_list(*local, item_type.as_ref().clone());
            }
            ListLocal::Function { local, item_type } => {
                self.include_function_list(*local, item_type.clone());
            }
        }
    }

    pub(crate) fn include_int_list(&mut self, local: IntListLocalId) {
        self.int_lists = self.int_lists.max(local.0 + 1);
    }

    pub(crate) fn include_string_list(&mut self, local: StringListLocalId) {
        self.string_lists = self.string_lists.max(local.0 + 1);
    }

    pub(crate) fn include_float_list(&mut self, local: FloatListLocalId) {
        self.float_lists = self.float_lists.max(local.0 + 1);
    }

    pub(crate) fn include_bool_list(&mut self, local: BoolListLocalId) {
        self.bool_lists = self.bool_lists.max(local.0 + 1);
    }

    pub(crate) fn include_nil_list(&mut self, local: NilListLocalId) {
        self.nil_lists = self.nil_lists.max(local.0 + 1);
    }

    pub(crate) fn include_tuple_list(
        &mut self,
        local: TupleListLocalId,
        item_type: Vec<ValueType>,
    ) {
        if self.tuple_lists.len() <= local.0 {
            self.tuple_lists.resize(local.0 + 1, Vec::new());
        }
        self.tuple_lists[local.0] = item_type;
    }

    pub(crate) fn include_list_list(&mut self, local: ListListLocalId, item_type: ValueType) {
        if self.list_lists.len() <= local.0 {
            self.list_lists.resize(local.0 + 1, ValueType::Nil);
        }
        self.list_lists[local.0] = item_type;
    }

    pub(crate) fn include_function_list(
        &mut self,
        local: FunctionListLocalId,
        item_type: super::FunctionType,
    ) {
        if self.function_lists.len() <= local.0 {
            self.function_lists.resize(
                local.0 + 1,
                super::FunctionType::new(Vec::new(), ValueType::Nil),
            );
        }
        self.function_lists[local.0] = item_type;
    }

    pub(crate) fn include_int_function(&mut self, local: IntFunctionLocalId) {
        self.int_functions = self.int_functions.max(local.0 + 1);
    }

    pub(crate) fn include_float_function(&mut self, local: FloatFunctionLocalId) {
        self.float_functions = self.float_functions.max(local.0 + 1);
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

    pub(crate) fn include_tuple_function(&mut self, local: TupleFunctionLocalId) {
        self.tuple_functions = self.tuple_functions.max(local.0 + 1);
    }

    pub(crate) fn include_list_function(&mut self, local: ListFunctionLocalId) {
        self.list_functions = self.list_functions.max(local.0 + 1);
    }

    pub(crate) fn include_function_function(&mut self, local: FunctionFunctionLocalId) {
        self.function_functions = self.function_functions.max(local.0 + 1);
    }

    pub(crate) fn ints(&self) -> usize {
        self.ints
    }

    pub(crate) fn floats(&self) -> usize {
        self.floats
    }

    pub(crate) fn strings(&self) -> usize {
        self.strings
    }

    pub(crate) fn bools(&self) -> usize {
        self.bools
    }

    #[cfg(test)]
    pub(crate) fn nils(&self) -> usize {
        self.nils
    }

    pub(crate) fn tuples(&self) -> usize {
        self.tuples
    }

    pub(crate) fn int_lists(&self) -> usize {
        self.int_lists
    }

    pub(crate) fn string_lists(&self) -> usize {
        self.string_lists
    }

    pub(crate) fn float_lists(&self) -> usize {
        self.float_lists
    }

    pub(crate) fn bool_lists(&self) -> usize {
        self.bool_lists
    }

    pub(crate) fn nil_lists(&self) -> usize {
        self.nil_lists
    }

    pub(crate) fn tuple_lists(&self) -> &[Vec<ValueType>] {
        &self.tuple_lists
    }

    pub(crate) fn list_lists(&self) -> &[ValueType] {
        &self.list_lists
    }

    pub(crate) fn function_lists(&self) -> &[super::FunctionType] {
        &self.function_lists
    }

    pub(crate) fn int_functions(&self) -> usize {
        self.int_functions
    }

    pub(crate) fn float_functions(&self) -> usize {
        self.float_functions
    }

    pub(crate) fn string_functions(&self) -> usize {
        self.string_functions
    }

    pub(crate) fn bool_functions(&self) -> usize {
        self.bool_functions
    }

    pub(crate) fn nil_functions(&self) -> usize {
        self.nil_functions
    }

    pub(crate) fn tuple_functions(&self) -> usize {
        self.tuple_functions
    }

    pub(crate) fn list_functions(&self) -> usize {
        self.list_functions
    }

    pub(crate) fn function_functions(&self) -> usize {
        self.function_functions
    }
}

#[cfg(test)]
pub(super) mod test_helpers {
    use crate::plan::{
        BoolFunctionExpr, BoolFunctionId, BoolFunctionValue, BoolLocalId, FloatFunctionExpr,
        FloatFunctionId, FloatFunctionValue, FloatLocalId, FunctionType, IntFunctionExpr,
        IntFunctionId, IntFunctionValue, IntLocalId, NilFunctionExpr, NilFunctionId,
        NilFunctionValue, NilLocalId, ParamLocal, StringFunctionExpr, StringFunctionId,
        StringFunctionValue, StringLocalId, ValueType,
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

    pub(super) fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(FloatLocalId(0))],
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
        BoolLocalId, FloatFunctionLocalId, FloatLocalId, IntFunctionLocalId, IntListLocalId,
        IntLocalId, ListLocal, NilFunctionLocalId, NilLocalId, ParamLocal, StringLocalId,
        TupleFunctionLocalId, TupleListLocalId, TupleLocalId, ValueType,
    };

    #[test]
    fn frame_layout_derived_surface_is_covered() {
        let layout = FrameLayout::default();
        let cloned = clone_value(&layout);

        assert_eq!(layout, cloned);
        assert_eq!(
            format!("{layout:?}"),
            "FrameLayout { ints: 0, floats: 0, strings: 0, bools: 0, nils: 0, tuples: 0, int_lists: 0, string_lists: 0, float_lists: 0, bool_lists: 0, nil_lists: 0, tuple_lists: [], list_lists: [], function_lists: [], int_functions: 0, float_functions: 0, string_functions: 0, bool_functions: 0, nil_functions: 0, tuple_functions: 0, list_functions: 0, function_functions: 0 }",
        );
    }

    fn clone_value<T: Clone>(value: &T) -> T {
        value.clone()
    }

    #[test]
    fn frame_layout_includes_local_ids() {
        let mut layout = FrameLayout::default();

        layout.include_local(&ParamLocal::int(IntLocalId(1)));
        layout.include_local(&ParamLocal::float(FloatLocalId(2)));
        layout.include_local(&ParamLocal::string(StringLocalId(2)));
        layout.include_local(&ParamLocal::bool(BoolLocalId(3)));
        layout.include_local(&ParamLocal::nil(NilLocalId(4)));
        layout.include_local(&ParamLocal::tuple(TupleLocalId(5), vec![ValueType::Int]));
        layout.include_int_function(IntFunctionLocalId(5));
        layout.include_float_function(FloatFunctionLocalId(6));
        layout.include_nil_function(NilFunctionLocalId(7));
        layout.include_local(&ParamLocal::tuple_function(
            TupleFunctionLocalId(8),
            crate::plan::FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
        ));

        assert_eq!(layout.ints(), 2);
        assert_eq!(layout.floats(), 3);
        assert_eq!(layout.strings(), 3);
        assert_eq!(layout.bools(), 4);
        assert_eq!(layout.nils(), 5);
        assert_eq!(layout.tuples(), 6);
        assert_eq!(layout.int_functions(), 6);
        assert_eq!(layout.float_functions(), 7);
        assert_eq!(layout.nil_functions(), 8);
        assert_eq!(layout.tuple_functions(), 9);
    }

    #[test]
    fn frame_layout_preserves_list_local_item_types() {
        let mut layout = FrameLayout::default();

        layout.include_list(ListLocal::int(IntListLocalId(0)));
        layout.include_local(&ParamLocal::list(ListLocal::tuple(
            TupleListLocalId(0),
            vec![ValueType::String],
        )));

        assert_eq!(layout.int_lists(), 1);
        assert_eq!(layout.tuple_lists(), &[vec![ValueType::String]]);
    }
}
