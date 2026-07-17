mod args;
mod expression;
mod function;
mod return_;
mod step;

use std::borrow::Borrow;

use super::function::{Param, ParamLocal, ReturnExpr};
use super::id::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomListLocalId, CustomLocal, FloatFunctionLocalId,
    FloatListLocalId, FloatLocalId, FunctionListLocalId, GenericFunctionLocal, GenericListLocalId,
    GenericLocal, IntFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, ListLocal, NilFunctionLocalId, NilListLocalId, NilLocalId,
    StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use super::id::{CustomFunctionLocal, FunctionFunctionLocal};
use super::step::Step;
use crate::plan::{CustomType, FunctionType, ValueType};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FrameLayout {
    generics: Vec<GenericLocal>,
    generic_lists: Vec<(GenericListLocalId, crate::plan::TypeParameterId)>,
    ints: usize,
    floats: usize,
    strings: usize,
    bit_arrays: usize,
    utf_codepoints: usize,
    customs: Vec<CustomLocal>,
    bools: usize,
    nils: usize,
    tuples: usize,
    int_lists: usize,
    string_lists: usize,
    bit_array_lists: usize,
    utf_codepoint_lists: usize,
    custom_lists: Vec<CustomType>,
    float_lists: usize,
    bool_lists: usize,
    nil_lists: usize,
    tuple_lists: Vec<Vec<ValueType>>,
    list_lists: Vec<ValueType>,
    function_lists: Vec<FunctionType>,
    int_functions: usize,
    float_functions: usize,
    string_functions: usize,
    bit_array_functions: usize,
    utf_codepoint_functions: usize,
    custom_functions: Vec<CustomFunctionLocal>,
    bool_functions: usize,
    nil_functions: usize,
    tuple_functions: usize,
    list_functions: Vec<ListFunctionLocal>,
    function_functions: Vec<FunctionFunctionLocal>,
    generic_functions: Vec<GenericFunctionLocal>,
}

pub(crate) struct FrameLayoutParts<'a> {
    pub(crate) generics: &'a [GenericLocal],
    pub(crate) generic_lists: &'a [(GenericListLocalId, crate::plan::TypeParameterId)],
    pub(crate) ints: usize,
    pub(crate) floats: usize,
    pub(crate) strings: usize,
    pub(crate) bit_arrays: usize,
    pub(crate) utf_codepoints: usize,
    pub(crate) customs: &'a [CustomLocal],
    pub(crate) bools: usize,
    pub(crate) nils: usize,
    pub(crate) tuples: usize,
    pub(crate) int_lists: usize,
    pub(crate) string_lists: usize,
    pub(crate) bit_array_lists: usize,
    pub(crate) utf_codepoint_lists: usize,
    pub(crate) custom_lists: &'a [CustomType],
    pub(crate) float_lists: usize,
    pub(crate) bool_lists: usize,
    pub(crate) nil_lists: usize,
    pub(crate) tuple_lists: &'a [Vec<ValueType>],
    pub(crate) list_lists: &'a [ValueType],
    pub(crate) function_lists: &'a [FunctionType],
    pub(crate) int_functions: usize,
    pub(crate) float_functions: usize,
    pub(crate) string_functions: usize,
    pub(crate) bit_array_functions: usize,
    pub(crate) utf_codepoint_functions: usize,
    pub(crate) custom_functions: &'a [CustomFunctionLocal],
    pub(crate) bool_functions: usize,
    pub(crate) nil_functions: usize,
    pub(crate) tuple_functions: usize,
    pub(crate) list_functions: &'a [ListFunctionLocal],
    pub(crate) function_functions: &'a [FunctionFunctionLocal],
    pub(crate) generic_functions: &'a [GenericFunctionLocal],
}

impl FrameLayout {
    pub(crate) fn parts(&self) -> FrameLayoutParts<'_> {
        FrameLayoutParts {
            generics: &self.generics,
            generic_lists: &self.generic_lists,
            ints: self.ints,
            floats: self.floats,
            strings: self.strings,
            bit_arrays: self.bit_arrays,
            utf_codepoints: self.utf_codepoints,
            customs: &self.customs,
            bools: self.bools,
            nils: self.nils,
            tuples: self.tuples,
            int_lists: self.int_lists,
            string_lists: self.string_lists,
            bit_array_lists: self.bit_array_lists,
            utf_codepoint_lists: self.utf_codepoint_lists,
            custom_lists: &self.custom_lists,
            float_lists: self.float_lists,
            bool_lists: self.bool_lists,
            nil_lists: self.nil_lists,
            tuple_lists: &self.tuple_lists,
            list_lists: &self.list_lists,
            function_lists: &self.function_lists,
            int_functions: self.int_functions,
            float_functions: self.float_functions,
            string_functions: self.string_functions,
            bit_array_functions: self.bit_array_functions,
            utf_codepoint_functions: self.utf_codepoint_functions,
            custom_functions: &self.custom_functions,
            bool_functions: self.bool_functions,
            nil_functions: self.nil_functions,
            tuple_functions: self.tuple_functions,
            list_functions: &self.list_functions,
            function_functions: &self.function_functions,
            generic_functions: &self.generic_functions,
        }
    }

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
            ParamLocal::Generic(local) => self.include_generic(*local),
            ParamLocal::Int(local) => self.include_int(*local),
            ParamLocal::Float(local) => self.include_float(*local),
            ParamLocal::String(local) => self.include_string(*local),
            ParamLocal::BitArray(local) => self.include_bit_array(*local),
            ParamLocal::UtfCodepoint(local) => self.include_utf_codepoint(*local),
            ParamLocal::Custom(local) => self.include_custom(local.clone()),
            ParamLocal::Bool(local) => self.include_bool(*local),
            ParamLocal::Nil(local) => self.include_nil(*local),
            ParamLocal::Tuple { local, .. } => self.include_tuple(*local),
            ParamLocal::List(local) => self.include_list(local),
            ParamLocal::IntFunction { local, .. } => self.include_int_function(*local),
            ParamLocal::FloatFunction { local, .. } => self.include_float_function(*local),
            ParamLocal::StringFunction { local, .. } => self.include_string_function(*local),
            ParamLocal::BitArrayFunction { local, .. } => self.include_bit_array_function(*local),
            ParamLocal::UtfCodepointFunction { local, .. } => {
                self.include_utf_codepoint_function(*local)
            }
            ParamLocal::CustomFunction(local) => self.include_custom_function(local.clone()),
            ParamLocal::BoolFunction { local, .. } => self.include_bool_function(*local),
            ParamLocal::NilFunction { local, .. } => self.include_nil_function(*local),
            ParamLocal::TupleFunction { local, .. } => self.include_tuple_function(*local),
            ParamLocal::ListFunction(local) => self.include_list_function(local.clone()),
            ParamLocal::FunctionFunction(local) => self.include_function_function(local.clone()),
            ParamLocal::GenericFunction(local) => self.include_generic_function(local.clone()),
        }
    }

    pub(crate) fn include_generic(&mut self, local: GenericLocal) {
        if !self.generics.contains(&local) {
            self.generics.push(local);
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

    pub(crate) fn include_bit_array(&mut self, local: BitArrayLocalId) {
        self.bit_arrays = self.bit_arrays.max(local.0 + 1);
    }

    pub(crate) fn include_utf_codepoint(&mut self, local: UtfCodepointLocalId) {
        self.utf_codepoints = self.utf_codepoints.max(local.0 + 1);
    }

    pub(crate) fn include_custom(&mut self, local: CustomLocal) {
        if !self.customs.contains(&local) {
            self.customs.push(local);
        }
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
            ListLocal::Generic { local, parameter } => {
                let entry = (*local, *parameter);
                if !self.generic_lists.contains(&entry) {
                    self.generic_lists.push(entry);
                }
            }
            ListLocal::Int(local) => self.include_int_list(*local),
            ListLocal::String(local) => self.include_string_list(*local),
            ListLocal::BitArray(local) => self.include_bit_array_list(*local),
            ListLocal::UtfCodepoint(local) => self.include_utf_codepoint_list(*local),
            ListLocal::Custom { local, item_type } => {
                self.include_custom_list(*local, item_type.clone());
            }
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

    pub(crate) fn include_bit_array_list(&mut self, local: BitArrayListLocalId) {
        self.bit_array_lists = self.bit_array_lists.max(local.0 + 1);
    }

    pub(crate) fn include_utf_codepoint_list(&mut self, local: UtfCodepointListLocalId) {
        self.utf_codepoint_lists = self.utf_codepoint_lists.max(local.0 + 1);
    }

    pub(crate) fn include_custom_list(&mut self, local: CustomListLocalId, item_type: CustomType) {
        if self.custom_lists.len() <= local.0 {
            self.custom_lists.resize(local.0 + 1, item_type.clone());
        }
        self.custom_lists[local.0] = item_type;
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
        item_type: FunctionType,
    ) {
        if self.function_lists.len() <= local.0 {
            self.function_lists
                .resize(local.0 + 1, FunctionType::new(Vec::new(), ValueType::Nil));
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

    pub(crate) fn include_bit_array_function(&mut self, local: BitArrayFunctionLocalId) {
        self.bit_array_functions = self.bit_array_functions.max(local.0 + 1);
    }

    pub(crate) fn include_utf_codepoint_function(&mut self, local: UtfCodepointFunctionLocalId) {
        self.utf_codepoint_functions = self.utf_codepoint_functions.max(local.0 + 1);
    }

    pub(crate) fn include_custom_function(&mut self, local: CustomFunctionLocal) {
        if !self.custom_functions.contains(&local) {
            self.custom_functions.push(local);
        }
    }

    pub(crate) fn include_generic_function(&mut self, local: GenericFunctionLocal) {
        if !self.generic_functions.contains(&local) {
            self.generic_functions.push(local);
        }
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

    pub(crate) fn include_list_function(&mut self, local: ListFunctionLocal) {
        if !self.list_functions.contains(&local) {
            self.list_functions.push(local);
        }
    }

    pub(crate) fn include_function_function(&mut self, local: FunctionFunctionLocal) {
        if !self.function_functions.contains(&local) {
            self.function_functions.push(local);
        }
    }

    #[cfg(test)]
    pub(crate) fn ints(&self) -> usize {
        self.ints
    }

    #[cfg(test)]
    pub(crate) fn floats(&self) -> usize {
        self.floats
    }

    #[cfg(test)]
    pub(crate) fn strings(&self) -> usize {
        self.strings
    }

    #[cfg(test)]
    pub(crate) fn bools(&self) -> usize {
        self.bools
    }

    #[cfg(test)]
    pub(crate) fn nils(&self) -> usize {
        self.nils
    }

    #[cfg(test)]
    pub(crate) fn tuples(&self) -> usize {
        self.tuples
    }

    #[cfg(test)]
    pub(crate) fn int_lists(&self) -> usize {
        self.int_lists
    }

    #[cfg(test)]
    pub(crate) fn string_lists(&self) -> usize {
        self.string_lists
    }

    #[cfg(test)]
    pub(crate) fn float_lists(&self) -> usize {
        self.float_lists
    }

    #[cfg(test)]
    pub(crate) fn bool_lists(&self) -> usize {
        self.bool_lists
    }

    #[cfg(test)]
    pub(crate) fn nil_lists(&self) -> usize {
        self.nil_lists
    }

    #[cfg(test)]
    pub(crate) fn tuple_lists(&self) -> &[Vec<ValueType>] {
        &self.tuple_lists
    }

    #[cfg(test)]
    pub(crate) fn list_lists(&self) -> &[ValueType] {
        &self.list_lists
    }

    #[cfg(test)]
    pub(crate) fn function_lists(&self) -> &[FunctionType] {
        &self.function_lists
    }

    #[cfg(test)]
    pub(crate) fn int_functions(&self) -> usize {
        self.int_functions
    }

    #[cfg(test)]
    pub(crate) fn float_functions(&self) -> usize {
        self.float_functions
    }

    #[cfg(test)]
    pub(crate) fn string_functions(&self) -> usize {
        self.string_functions
    }

    #[cfg(test)]
    pub(crate) fn bool_functions(&self) -> usize {
        self.bool_functions
    }

    #[cfg(test)]
    pub(crate) fn nil_functions(&self) -> usize {
        self.nil_functions
    }

    #[cfg(test)]
    pub(crate) fn tuple_functions(&self) -> usize {
        self.tuple_functions
    }

    #[cfg(test)]
    pub(crate) fn list_functions(&self) -> &[ListFunctionLocal] {
        &self.list_functions
    }

    #[cfg(test)]
    pub(crate) fn function_functions(&self) -> &[FunctionFunctionLocal] {
        &self.function_functions
    }
}

#[cfg(test)]
pub(super) mod test_helpers {
    use crate::plan::{
        BoolFunctionExpr, BoolFunctionReference, BoolLocalId, FloatFunctionExpr,
        FloatFunctionReference, FloatLocalId, FunctionFunctionType, FunctionShape, IntFunctionExpr,
        IntFunctionReference, IntLocalId, NilFunctionExpr, NilFunctionReference, NilLocalId,
        ParamLocal, StringFunctionExpr, StringFunctionReference, StringLocalId, ValueShape,
        monomorphic_function_instantiation,
    };

    pub(super) fn int_function_expr() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
            ),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    pub(super) fn string_function_expr() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::String], ValueShape::String),
            ),
            vec![ParamLocal::string(StringLocalId(0))],
        ))
    }

    pub(super) fn float_function_expr() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::Float], ValueShape::Float),
            ),
            vec![ParamLocal::float(FloatLocalId(0))],
        ))
    }

    pub(super) fn bool_function_expr() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::Bool], ValueShape::Bool),
            ),
            vec![ParamLocal::bool(BoolLocalId(0))],
        ))
    }

    pub(super) fn nil_function_expr() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::Nil], ValueShape::Nil),
            ),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    pub(super) fn function_returning_int_function_type() -> FunctionFunctionType {
        FunctionFunctionType::new(Vec::new(), int_function_expr().type_().clone())
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
            "FrameLayout { generics: [], generic_lists: [], ints: 0, floats: 0, strings: 0, bit_arrays: 0, utf_codepoints: 0, customs: [], bools: 0, nils: 0, tuples: 0, int_lists: 0, string_lists: 0, bit_array_lists: 0, utf_codepoint_lists: 0, custom_lists: [], float_lists: 0, bool_lists: 0, nil_lists: 0, tuple_lists: [], list_lists: [], function_lists: [], int_functions: 0, float_functions: 0, string_functions: 0, bit_array_functions: 0, utf_codepoint_functions: 0, custom_functions: [], bool_functions: 0, nil_functions: 0, tuple_functions: 0, list_functions: [], function_functions: [], generic_functions: [] }",
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
