use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use super::{
    BitArrayExpr, BitArrayFunctionExpr, BitArrayListExpr, BoolExpr, BoolFunctionExpr, BoolListExpr,
    CustomExpr, CustomFunctionExpr, CustomListExpr, FloatExpr, FloatFunctionExpr, FloatListExpr,
    FunctionFunctionExpr, FunctionListExpr, GenericFunctionExpr, IntExpr, IntFunctionExpr,
    IntListExpr, ListListExpr, NeverFunctionExpr, NilExpr, NilFunctionExpr, NilListExpr,
    ParameterListExpr, ParameterListListExpr, StringExpr, StringFunctionExpr, StringListExpr,
    TupleExpr, TupleFunctionExpr, TupleListExpr, UtfCodepointFunctionExpr, UtfCodepointListExpr,
};

pub(crate) struct ConstantId<Value> {
    index: usize,
    value: PhantomData<fn() -> Value>,
}

impl<Value> ConstantId<Value> {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self {
            index,
            value: PhantomData,
        }
    }

    pub(in crate::plan::execution) fn index(self) -> usize {
        self.index
    }
}

impl<Value> Clone for ConstantId<Value> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Value> Copy for ConstantId<Value> {}

impl<Value> std::fmt::Debug for ConstantId<Value> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ConstantId")
            .field(&self.index)
            .finish()
    }
}

impl<Value> PartialEq for ConstantId<Value> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<Value> Eq for ConstantId<Value> {}

impl<Value> Hash for ConstantId<Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

#[derive(Default)]
pub(crate) struct ConstantTable {
    ints: Vec<IntExpr>,
    strings: Vec<StringExpr>,
    bit_arrays: Vec<BitArrayExpr>,
    customs: Vec<CustomExpr>,
    floats: Vec<FloatExpr>,
    bools: Vec<BoolExpr>,
    nils: Vec<NilExpr>,
    tuples: Vec<TupleExpr>,
    parameter_lists: Vec<ParameterListExpr>,
    parameter_list_lists: Vec<ParameterListListExpr>,
    int_lists: Vec<IntListExpr>,
    string_lists: Vec<StringListExpr>,
    bit_array_lists: Vec<BitArrayListExpr>,
    utf_codepoint_lists: Vec<UtfCodepointListExpr>,
    custom_lists: Vec<CustomListExpr>,
    float_lists: Vec<FloatListExpr>,
    bool_lists: Vec<BoolListExpr>,
    nil_lists: Vec<NilListExpr>,
    tuple_lists: Vec<TupleListExpr>,
    list_lists: Vec<ListListExpr>,
    function_lists: Vec<FunctionListExpr>,
    generic_functions: Vec<GenericFunctionExpr>,
    never_functions: Vec<NeverFunctionExpr>,
    int_functions: Vec<IntFunctionExpr>,
    string_functions: Vec<StringFunctionExpr>,
    bit_array_functions: Vec<BitArrayFunctionExpr>,
    utf_codepoint_functions: Vec<UtfCodepointFunctionExpr>,
    custom_functions: Vec<CustomFunctionExpr>,
    float_functions: Vec<FloatFunctionExpr>,
    bool_functions: Vec<BoolFunctionExpr>,
    nil_functions: Vec<NilFunctionExpr>,
    tuple_functions: Vec<TupleFunctionExpr>,
    list_functions: Vec<super::ListFunctionExpr>,
    function_functions: Vec<FunctionFunctionExpr>,
}

pub(crate) trait ConstantExpression: Sized {
    fn values(table: &ConstantTable) -> &[Self];
    fn values_mut(table: &mut ConstantTable) -> &mut Vec<Self>;
}

macro_rules! constant_expression {
    ($expression:ty, $field:ident) => {
        impl ConstantExpression for $expression {
            fn values(table: &ConstantTable) -> &[Self] {
                &table.$field
            }

            fn values_mut(table: &mut ConstantTable) -> &mut Vec<Self> {
                &mut table.$field
            }
        }
    };
}

constant_expression!(IntExpr, ints);
constant_expression!(StringExpr, strings);
constant_expression!(BitArrayExpr, bit_arrays);
constant_expression!(CustomExpr, customs);
constant_expression!(FloatExpr, floats);
constant_expression!(BoolExpr, bools);
constant_expression!(NilExpr, nils);
constant_expression!(TupleExpr, tuples);
constant_expression!(ParameterListExpr, parameter_lists);
constant_expression!(ParameterListListExpr, parameter_list_lists);
constant_expression!(IntListExpr, int_lists);
constant_expression!(StringListExpr, string_lists);
constant_expression!(BitArrayListExpr, bit_array_lists);
constant_expression!(UtfCodepointListExpr, utf_codepoint_lists);
constant_expression!(CustomListExpr, custom_lists);
constant_expression!(FloatListExpr, float_lists);
constant_expression!(BoolListExpr, bool_lists);
constant_expression!(NilListExpr, nil_lists);
constant_expression!(TupleListExpr, tuple_lists);
constant_expression!(ListListExpr, list_lists);
constant_expression!(FunctionListExpr, function_lists);
constant_expression!(GenericFunctionExpr, generic_functions);
constant_expression!(NeverFunctionExpr, never_functions);
constant_expression!(IntFunctionExpr, int_functions);
constant_expression!(StringFunctionExpr, string_functions);
constant_expression!(BitArrayFunctionExpr, bit_array_functions);
constant_expression!(UtfCodepointFunctionExpr, utf_codepoint_functions);
constant_expression!(CustomFunctionExpr, custom_functions);
constant_expression!(FloatFunctionExpr, float_functions);
constant_expression!(BoolFunctionExpr, bool_functions);
constant_expression!(NilFunctionExpr, nil_functions);
constant_expression!(TupleFunctionExpr, tuple_functions);
constant_expression!(super::ListFunctionExpr, list_functions);
constant_expression!(FunctionFunctionExpr, function_functions);

impl ConstantTable {
    pub(in crate::plan::execution) fn push<Expression: ConstantExpression>(
        &mut self,
        value: Expression,
    ) -> ConstantId<Expression> {
        let values = Expression::values_mut(self);
        let id = ConstantId::new(values.len());
        values.push(value);
        id
    }

    pub(crate) fn get<Expression: ConstantExpression>(
        &self,
        id: ConstantId<Expression>,
    ) -> &Expression {
        &Expression::values(self)[id.index()]
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.ints.len()
            + self.strings.len()
            + self.bit_arrays.len()
            + self.customs.len()
            + self.floats.len()
            + self.bools.len()
            + self.nils.len()
            + self.tuples.len()
            + self.parameter_lists.len()
            + self.parameter_list_lists.len()
            + self.int_lists.len()
            + self.string_lists.len()
            + self.bit_array_lists.len()
            + self.utf_codepoint_lists.len()
            + self.custom_lists.len()
            + self.float_lists.len()
            + self.bool_lists.len()
            + self.nil_lists.len()
            + self.tuple_lists.len()
            + self.list_lists.len()
            + self.function_lists.len()
            + self.generic_functions.len()
            + self.never_functions.len()
            + self.int_functions.len()
            + self.string_functions.len()
            + self.bit_array_functions.len()
            + self.utf_codepoint_functions.len()
            + self.custom_functions.len()
            + self.float_functions.len()
            + self.bool_functions.len()
            + self.nil_functions.len()
            + self.tuple_functions.len()
            + self.list_functions.len()
            + self.function_functions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantId;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct UncomparableValue;

    #[test]
    fn constant_id_traits_depend_only_on_the_typed_index() {
        fn assert_copy<T: Copy>() {}

        assert_copy::<ConstantId<UncomparableValue>>();
        let id = ConstantId::<UncomparableValue>::new(3);
        let copied = id;
        let cloned = <ConstantId<UncomparableValue> as Clone>::clone(&id);
        let different = ConstantId::<UncomparableValue>::new(4);

        assert_eq!(copied, id);
        assert_eq!(cloned, id);
        assert_ne!(id, different);
        assert_eq!(format!("{id:?}"), "ConstantId(3)");

        let mut id_hasher = DefaultHasher::new();
        id.hash(&mut id_hasher);
        let mut copied_hasher = DefaultHasher::new();
        copied.hash(&mut copied_hasher);
        let mut different_hasher = DefaultHasher::new();
        different.hash(&mut different_hasher);

        assert_eq!(id_hasher.finish(), copied_hasher.finish());
        assert_ne!(id_hasher.finish(), different_hasher.finish());
    }
}
