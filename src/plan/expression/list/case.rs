use super::{
    BoolListExpr, FloatListExpr, FunctionListExpr, FunctionListItem, IntListExpr, ListExpr,
    ListListExpr, ListListItem, NilListExpr, StringListExpr, TupleListExpr, TupleListItem,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BoolListCaseBranches {
    Int {
        true_: IntListExpr,
        false_: IntListExpr,
    },
    String {
        true_: StringListExpr,
        false_: StringListExpr,
    },
    Float {
        true_: FloatListExpr,
        false_: FloatListExpr,
    },
    Bool {
        true_: BoolListExpr,
        false_: BoolListExpr,
    },
    Nil {
        true_: NilListExpr,
        false_: NilListExpr,
    },
    Tuple {
        true_: TupleListExpr,
        false_: TupleListExpr,
    },
    List {
        true_: ListListExpr,
        false_: ListListExpr,
    },
    Function {
        true_: FunctionListExpr,
        false_: FunctionListExpr,
    },
}

pub(super) fn into_int_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, IntListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_int()
                    .expect("list case branches must be List(Int)"),
            )
        })
        .collect()
}

pub(super) fn into_string_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, StringListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_string()
                    .expect("list case branches must be List(String)"),
            )
        })
        .collect()
}

pub(super) fn into_float_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, FloatListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_float()
                    .expect("list case branches must be List(Float)"),
            )
        })
        .collect()
}

pub(super) fn into_bool_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, BoolListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_bool()
                    .expect("list case branches must be List(Bool)"),
            )
        })
        .collect()
}

pub(super) fn into_nil_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Vec<(Pattern, NilListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            (
                pattern,
                branch
                    .into_nil()
                    .expect("list case branches must be List(Nil)"),
            )
        })
        .collect()
}

pub(super) fn into_tuple_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &TupleListItem,
) -> Vec<(Pattern, TupleListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_tuple()
                .expect("list case branches must be List(tuple)");
            assert_eq!(
                &branch.item, item,
                "tuple list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

pub(super) fn into_list_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &ListListItem,
) -> Vec<(Pattern, ListListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_list()
                .expect("list case branches must be List(list)");
            assert_eq!(
                &branch.item, item,
                "nested list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

pub(super) fn into_function_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    item: &FunctionListItem,
) -> Vec<(Pattern, FunctionListExpr)> {
    clauses
        .into_iter()
        .map(|(pattern, branch)| {
            let branch = branch
                .into_function()
                .expect("list case branches must be List(function)");
            assert_eq!(
                &branch.item, item,
                "function list branch item types must match"
            );
            (pattern, branch)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        BoolListCaseBranches, into_function_clauses, into_list_clauses, into_nil_clauses,
        into_tuple_clauses,
    };
    use crate::plan::{
        BoolExpr, Expr, FunctionExpr, FunctionType, FunctionValue, IntExpr, ListExpr, ListListItem,
        NilExpr, RuntimeFunctionId, StringExpr, TupleExpr, TupleListItem, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn bool_case_branches_dispatch_every_non_int_item_family() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::String {
                    true_: ListExpr::value(Vec::new(), ValueType::String)
                        .into_string()
                        .expect("string list"),
                    false_: ListExpr::value(Vec::new(), ValueType::String)
                        .into_string()
                        .expect("string list"),
                },
            )
            .element_type(),
            ValueType::String,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Float {
                    true_: ListExpr::value(Vec::new(), ValueType::Float)
                        .into_float()
                        .expect("float list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Float)
                        .into_float()
                        .expect("float list"),
                },
            )
            .element_type(),
            ValueType::Float,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Bool {
                    true_: ListExpr::value(Vec::new(), ValueType::Bool)
                        .into_bool()
                        .expect("bool list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Bool)
                        .into_bool()
                        .expect("bool list"),
                },
            )
            .element_type(),
            ValueType::Bool,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Nil {
                    true_: ListExpr::value(Vec::new(), ValueType::Nil)
                        .into_nil()
                        .expect("nil list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Nil)
                        .into_nil()
                        .expect("nil list"),
                },
            )
            .element_type(),
            ValueType::Nil,
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Tuple {
                    true_: ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
                        .into_tuple()
                        .expect("tuple list"),
                    false_: ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
                        .into_tuple()
                        .expect("tuple list"),
                },
            )
            .element_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::List {
                    true_: ListExpr::value(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::String)),
                    )
                    .into_list()
                    .expect("list list"),
                    false_: ListExpr::value(
                        Vec::new(),
                        ValueType::List(Box::new(ValueType::String)),
                    )
                    .into_list()
                    .expect("list list"),
                },
            )
            .element_type(),
            ValueType::List(Box::new(ValueType::String)),
        );
        assert_eq!(
            ListExpr::bool_case(
                BoolExpr::value(true),
                BoolListCaseBranches::Function {
                    true_: ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(function_type.clone())),
                    )
                    .into_function()
                    .expect("function list"),
                    false_: ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(function_type.clone())),
                    )
                    .into_function()
                    .expect("function list"),
                },
            )
            .element_type(),
            ValueType::Function(Box::new(function_type)),
        );
    }

    #[test]
    fn clause_narrowing_preserves_tuple_list_and_function_item_metadata() {
        let tuple_item = TupleListItem::new(vec![ValueType::Int]);
        let list_item = ListListItem::new(Box::new(ValueType::String));
        let function_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let function_item = crate::plan::FunctionListItem::new(function_type.clone());

        assert_eq!(
            into_nil_clauses(vec![(
                BigInt::from(1),
                ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
            )]),
            vec![(
                BigInt::from(1),
                ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil)
                    .into_nil()
                    .expect("nil list"),
            )],
        );
        assert_eq!(
            into_tuple_clauses(
                vec![(
                    "tuple",
                    ListExpr::value(
                        vec![Expr::tuple(TupleExpr::value(
                            vec![Expr::int(IntExpr::value(1.into()))],
                            vec![ValueType::Int],
                        ))],
                        ValueType::Tuple(vec![ValueType::Int]),
                    ),
                )],
                &tuple_item,
            ),
            vec![(
                "tuple",
                ListExpr::value(
                    vec![Expr::tuple(TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Int],
                    ))],
                    ValueType::Tuple(vec![ValueType::Int]),
                )
                .into_tuple()
                .expect("tuple list"),
            )],
        );
        assert_eq!(
            into_list_clauses(
                vec![(
                    "list",
                    ListExpr::value(
                        vec![Expr::list(ListExpr::value(
                            vec![Expr::string(StringExpr::value("one".into()))],
                            ValueType::String,
                        ))],
                        ValueType::List(Box::new(ValueType::String)),
                    ),
                )],
                &list_item,
            ),
            vec![(
                "list",
                ListExpr::value(
                    vec![Expr::list(ListExpr::value(
                        vec![Expr::string(StringExpr::value("one".into()))],
                        ValueType::String,
                    ))],
                    ValueType::List(Box::new(ValueType::String)),
                )
                .into_list()
                .expect("list list"),
            )],
        );
        assert_eq!(
            into_function_clauses(
                vec![(
                    "function",
                    ListExpr::value(
                        vec![Expr::function(FunctionExpr::value(FunctionValue::new(
                            RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                            Vec::new(),
                        )))],
                        ValueType::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            ValueType::Bool,
                        ))),
                    ),
                )],
                &function_item,
            ),
            vec![(
                "function",
                ListExpr::value(
                    vec![Expr::function(FunctionExpr::value(FunctionValue::new(
                        RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                        Vec::new(),
                    )))],
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
                )
                .into_function()
                .expect("function list"),
            )],
        );
    }
}
