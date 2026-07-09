use super::{
    BoolListExpr, FloatListExpr, FunctionListExpr, IntListExpr, ListExpr, ListListExpr,
    NilListExpr, StringListExpr, TupleListExpr,
};
use crate::plan::ValueType;

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

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ListCaseBranches<Pattern> {
    Int {
        clauses: Vec<(Pattern, IntListExpr)>,
        fallback: IntListExpr,
    },
    String {
        clauses: Vec<(Pattern, StringListExpr)>,
        fallback: StringListExpr,
    },
    Float {
        clauses: Vec<(Pattern, FloatListExpr)>,
        fallback: FloatListExpr,
    },
    Bool {
        clauses: Vec<(Pattern, BoolListExpr)>,
        fallback: BoolListExpr,
    },
    Nil {
        clauses: Vec<(Pattern, NilListExpr)>,
        fallback: NilListExpr,
    },
    Tuple {
        clauses: Vec<(Pattern, TupleListExpr)>,
        fallback: TupleListExpr,
    },
    List {
        clauses: Vec<(Pattern, ListListExpr)>,
        fallback: ListListExpr,
    },
    Function {
        clauses: Vec<(Pattern, FunctionListExpr)>,
        fallback: FunctionListExpr,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListCaseBranchTypeMismatch {
    pub(crate) expected: ValueType,
    pub(crate) actual: ValueType,
}

impl<Pattern> ListCaseBranches<Pattern> {
    pub(crate) fn from_exprs(
        clauses: Vec<(Pattern, ListExpr)>,
        fallback: ListExpr,
    ) -> Result<Self, ListCaseBranchTypeMismatch> {
        match fallback {
            ListExpr::Int(fallback) => Ok(Self::Int {
                clauses: typed_int_clauses(clauses)?,
                fallback,
            }),
            ListExpr::String(fallback) => Ok(Self::String {
                clauses: typed_string_clauses(clauses)?,
                fallback,
            }),
            ListExpr::Float(fallback) => Ok(Self::Float {
                clauses: typed_float_clauses(clauses)?,
                fallback,
            }),
            ListExpr::Bool(fallback) => Ok(Self::Bool {
                clauses: typed_bool_clauses(clauses)?,
                fallback,
            }),
            ListExpr::Nil(fallback) => Ok(Self::Nil {
                clauses: typed_nil_clauses(clauses)?,
                fallback,
            }),
            ListExpr::Tuple(fallback) => Ok(Self::Tuple {
                clauses: typed_tuple_clauses(clauses, fallback.element_type())?,
                fallback,
            }),
            ListExpr::List(fallback) => Ok(Self::List {
                clauses: typed_list_clauses(clauses, fallback.element_type())?,
                fallback,
            }),
            ListExpr::Function(fallback) => Ok(Self::Function {
                clauses: typed_function_clauses(clauses, fallback.element_type())?,
                fallback,
            }),
        }
    }
}

fn list_case_branch_type_mismatch(
    expected: ValueType,
    actual: ValueType,
) -> ListCaseBranchTypeMismatch {
    ListCaseBranchTypeMismatch { expected, actual }
}

fn typed_int_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Result<Vec<(Pattern, IntListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_int() else {
            return Err(list_case_branch_type_mismatch(ValueType::Int, actual));
        };
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_string_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Result<Vec<(Pattern, StringListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_string() else {
            return Err(list_case_branch_type_mismatch(ValueType::String, actual));
        };
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_float_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Result<Vec<(Pattern, FloatListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_float() else {
            return Err(list_case_branch_type_mismatch(ValueType::Float, actual));
        };
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_bool_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Result<Vec<(Pattern, BoolListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_bool() else {
            return Err(list_case_branch_type_mismatch(ValueType::Bool, actual));
        };
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_nil_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
) -> Result<Vec<(Pattern, NilListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_nil() else {
            return Err(list_case_branch_type_mismatch(ValueType::Nil, actual));
        };
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_tuple_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    expected: ValueType,
) -> Result<Vec<(Pattern, TupleListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_tuple() else {
            return Err(list_case_branch_type_mismatch(expected, actual));
        };
        if branch.element_type() != expected {
            return Err(list_case_branch_type_mismatch(
                expected,
                branch.element_type(),
            ));
        }
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_list_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    expected: ValueType,
) -> Result<Vec<(Pattern, ListListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_list() else {
            return Err(list_case_branch_type_mismatch(expected, actual));
        };
        if branch.element_type() != expected {
            return Err(list_case_branch_type_mismatch(
                expected,
                branch.element_type(),
            ));
        }
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

fn typed_function_clauses<Pattern>(
    clauses: Vec<(Pattern, ListExpr)>,
    expected: ValueType,
) -> Result<Vec<(Pattern, FunctionListExpr)>, ListCaseBranchTypeMismatch> {
    let mut typed_clauses = Vec::with_capacity(clauses.len());
    for (pattern, branch) in clauses {
        let actual = branch.element_type();
        let Some(branch) = branch.into_function() else {
            return Err(list_case_branch_type_mismatch(expected, actual));
        };
        if branch.element_type() != expected {
            return Err(list_case_branch_type_mismatch(
                expected,
                branch.element_type(),
            ));
        }
        typed_clauses.push((pattern, branch));
    }
    Ok(typed_clauses)
}

#[cfg(test)]
mod tests {
    use super::{BoolListCaseBranches, ListCaseBranchTypeMismatch, ListCaseBranches};
    use crate::plan::{
        BoolExpr, Expr, FunctionExpr, FunctionType, FunctionValue, IntExpr, ListExpr, NilExpr,
        RuntimeFunctionId, StringExpr, TupleExpr, ValueType,
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
    fn list_case_branches_preserve_all_item_family_metadata() {
        let function_type = FunctionType::new(Vec::new(), ValueType::Bool);

        assert_eq!(
            ListCaseBranches::<BigInt>::from_exprs(
                Vec::new(),
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Ok(ListCaseBranches::String {
                clauses: Vec::new(),
                fallback: ListExpr::value(Vec::new(), ValueType::String)
                    .into_string()
                    .expect("string list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::<BigInt>::from_exprs(
                Vec::new(),
                ListExpr::value(Vec::new(), ValueType::Float),
            ),
            Ok(ListCaseBranches::Float {
                clauses: Vec::new(),
                fallback: ListExpr::value(Vec::new(), ValueType::Float)
                    .into_float()
                    .expect("float list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::<BigInt>::from_exprs(
                Vec::new(),
                ListExpr::value(Vec::new(), ValueType::Bool),
            ),
            Ok(ListCaseBranches::Bool {
                clauses: Vec::new(),
                fallback: ListExpr::value(Vec::new(), ValueType::Bool)
                    .into_bool()
                    .expect("bool list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(
                    BigInt::from(1),
                    ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
                )],
                ListExpr::value(Vec::new(), ValueType::Nil),
            ),
            Ok(ListCaseBranches::Nil {
                clauses: vec![(
                    BigInt::from(1),
                    ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil)
                        .into_nil()
                        .expect("nil list"),
                )],
                fallback: ListExpr::value(Vec::new(), ValueType::Nil)
                    .into_nil()
                    .expect("nil list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
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
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
            ),
            Ok(ListCaseBranches::Tuple {
                clauses: vec![(
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
                fallback: ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int]))
                    .into_tuple()
                    .expect("tuple list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
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
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            ),
            Ok(ListCaseBranches::List {
                clauses: vec![(
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
                fallback: ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String)))
                    .into_list()
                    .expect("list list"),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
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
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(function_type.clone())),
                ),
            ),
            Ok(ListCaseBranches::Function {
                clauses: vec![(
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
                    )
                    .into_function()
                    .expect("function list"),
                )],
                fallback:
                    ListExpr::value(Vec::new(), ValueType::Function(Box::new(function_type)),)
                        .into_function()
                        .expect("function list"),
            }),
        );
    }

    #[test]
    fn list_case_branches_report_item_family_mismatch() {
        let int_to_int = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let int_to_string = FunctionType::new(vec![ValueType::Int], ValueType::String);

        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(
                    BigInt::from(1),
                    ListExpr::value(Vec::new(), ValueType::String)
                )],
                ListExpr::value(Vec::new(), ValueType::Int),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::String),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::String,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::Float),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Float,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::Bool),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Bool,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::Nil),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Nil,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::String])),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::String]),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(
                    BigInt::from(1),
                    ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                )],
                ListExpr::value(Vec::new(), ValueType::Tuple(vec![ValueType::String])),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::String]),
                actual: ValueType::Tuple(vec![ValueType::Int]),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::String)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(
                    BigInt::from(1),
                    ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                )],
                ListExpr::value(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::List(Box::new(ValueType::String)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(BigInt::from(1), ListExpr::value(Vec::new(), ValueType::Int))],
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(int_to_string.clone()))
                ),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Function(Box::new(int_to_string.clone())),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ListCaseBranches::from_exprs(
                vec![(
                    BigInt::from(1),
                    ListExpr::value(
                        Vec::new(),
                        ValueType::Function(Box::new(int_to_int.clone()))
                    ),
                )],
                ListExpr::value(
                    Vec::new(),
                    ValueType::Function(Box::new(int_to_string.clone()))
                ),
            ),
            Err(ListCaseBranchTypeMismatch {
                expected: ValueType::Function(Box::new(int_to_string)),
                actual: ValueType::Function(Box::new(int_to_int)),
            }),
        );
    }
}
