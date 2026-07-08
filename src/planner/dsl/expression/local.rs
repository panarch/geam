use super::{Bool, Float, Int, List, Nil, String, Tuple};
use crate::plan::{
    BoolExpr, BoolListLocalId, BoolLocalId, FloatExpr, FloatListLocalId, FloatLocalId,
    FunctionListLocalId, IntExpr, IntListLocalId, IntLocalId, ListExpr, ListListLocalId, ListLocal,
    NilExpr, NilListLocalId, NilLocalId, StringExpr, StringListLocalId, StringLocalId, TupleExpr,
    TupleListLocalId, TupleLocalId, ValueType,
};
use ecow::EcoString;

pub(crate) fn local_int(index: usize, name: impl Into<EcoString>) -> Int {
    Int(IntExpr::local_get(IntLocalId(index), name.into()))
}

pub(crate) fn local_string(index: usize, name: impl Into<EcoString>) -> String {
    String(StringExpr::local_get(StringLocalId(index), name.into()))
}

pub(crate) fn local_float(index: usize, name: impl Into<EcoString>) -> Float {
    Float(FloatExpr::local_get(FloatLocalId(index), name.into()))
}

pub(crate) fn local_bool(index: usize, name: impl Into<EcoString>) -> Bool {
    Bool(BoolExpr::local_get(BoolLocalId(index), name.into()))
}

pub(crate) fn local_nil(index: usize, name: impl Into<EcoString>) -> Nil {
    Nil(NilExpr::local_get(NilLocalId(index), name.into()))
}

pub(crate) fn local_tuple(
    index: usize,
    name: impl Into<EcoString>,
    type_: impl IntoIterator<Item = ValueType>,
) -> Tuple {
    Tuple(TupleExpr::local_get(
        TupleLocalId(index),
        name.into(),
        type_.into_iter().collect(),
    ))
}

pub(crate) fn local_list(
    index: usize,
    name: impl Into<EcoString>,
    element_type: ValueType,
) -> List {
    List(ListExpr::local_get(
        list_local(index, element_type),
        name.into(),
    ))
}

pub(super) fn list_local(index: usize, element_type: ValueType) -> ListLocal {
    match element_type {
        ValueType::Int => ListLocal::int(IntListLocalId(index)),
        ValueType::String => ListLocal::string(StringListLocalId(index)),
        ValueType::Float => ListLocal::float(FloatListLocalId(index)),
        ValueType::Bool => ListLocal::bool(BoolListLocalId(index)),
        ValueType::Nil => ListLocal::nil(NilListLocalId(index)),
        ValueType::Tuple(item_type) => ListLocal::tuple(TupleListLocalId(index), item_type),
        ValueType::List(item_type) => ListLocal::list(ListListLocalId(index), *item_type),
        ValueType::Function(item_type) => {
            ListLocal::function(FunctionListLocalId(index), *item_type)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        local_bool, local_float, local_int, local_list, local_nil, local_string, local_tuple,
    };
    use crate::plan::{
        BoolExpr, BoolListLocalId, BoolLocalId, FloatExpr, FloatListLocalId, FloatLocalId,
        FunctionListLocalId, FunctionType, IntExpr, IntListLocalId, IntLocalId, ListExpr,
        ListListLocalId, ListLocal, NilExpr, NilListLocalId, NilLocalId, StringExpr,
        StringListLocalId, StringLocalId, TupleExpr, TupleListLocalId, TupleLocalId, ValueType,
    };

    #[test]
    fn local_helpers_build_local_get_shapes() {
        assert_eq!(
            local_int(0, "x").0,
            IntExpr::local_get(IntLocalId(0), "x".into()),
        );
        assert_eq!(
            local_string(1, "name").0,
            StringExpr::local_get(StringLocalId(1), "name".into()),
        );
        assert_eq!(
            local_float(2, "ratio").0,
            FloatExpr::local_get(FloatLocalId(2), "ratio".into()),
        );
        assert_eq!(
            local_bool(3, "ok").0,
            BoolExpr::local_get(BoolLocalId(3), "ok".into()),
        );
        assert_eq!(
            local_nil(4, "done").0,
            NilExpr::local_get(NilLocalId(4), "done".into()),
        );
        assert_eq!(
            local_tuple(5, "pair", [ValueType::Int, ValueType::String]).0,
            TupleExpr::local_get(
                TupleLocalId(5),
                "pair".into(),
                vec![ValueType::Int, ValueType::String],
            ),
        );
        assert_eq!(
            local_list(6, "values", ValueType::Int).0,
            ListExpr::local_get(ListLocal::int(IntListLocalId(6)), "values".into()),
        );
        assert_eq!(
            local_list(7, "strings", ValueType::String).0,
            ListExpr::local_get(ListLocal::string(StringListLocalId(7)), "strings".into()),
        );
        assert_eq!(
            local_list(8, "floats", ValueType::Float).0,
            ListExpr::local_get(ListLocal::float(FloatListLocalId(8)), "floats".into()),
        );
        assert_eq!(
            local_list(9, "bools", ValueType::Bool).0,
            ListExpr::local_get(ListLocal::bool(BoolListLocalId(9)), "bools".into()),
        );
        assert_eq!(
            local_list(10, "nils", ValueType::Nil).0,
            ListExpr::local_get(ListLocal::nil(NilListLocalId(10)), "nils".into()),
        );
        assert_eq!(
            local_list(
                11,
                "tuples",
                ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            )
            .0,
            ListExpr::local_get(
                ListLocal::tuple(
                    TupleListLocalId(11),
                    vec![ValueType::Int, ValueType::String]
                ),
                "tuples".into(),
            ),
        );
        assert_eq!(
            local_list(12, "lists", ValueType::List(Box::new(ValueType::Float)),).0,
            ListExpr::local_get(
                ListLocal::list(ListListLocalId(12), ValueType::Float),
                "lists".into(),
            ),
        );
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        assert_eq!(
            local_list(
                13,
                "functions",
                ValueType::Function(Box::new(function_type.clone())),
            )
            .0,
            ListExpr::local_get(
                ListLocal::function(FunctionListLocalId(13), function_type),
                "functions".into(),
            ),
        );
    }
}
