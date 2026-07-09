use super::{ListItem, ListListExpr};
use crate::plan::{
    BoolExpr, CallArg, FloatExpr, IntExpr, ListFunctionExpr, PanicExpr, Step, StringExpr,
    TupleExpr, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedListExpr<Item: ListItem> {
    pub(super) item: Item,
    pub(super) kind: TypedListExprKind<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedListExprKind<Item: ListItem> {
    Value(Vec<Item::ElementExpr>),
    Spread {
        elements: Vec<Item::ElementExpr>,
        tail: Box<TypedListExpr<Item>>,
    },
    LocalGet {
        local: Item::Local,
        name: EcoString,
    },
    Call {
        function: Item::Function,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<ListFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    ListIndex {
        list: Box<ListListExpr>,
        index: usize,
    },
    DropFirst {
        list: Box<TypedListExpr<Item>>,
        count: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TypedListExpr<Item>>,
        false_: Box<TypedListExpr<Item>>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: Box<TypedListExpr<Item>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TypedListExpr<Item>>,
    },
}

impl<Item: ListItem> TypedListExpr<Item> {
    fn new(item: Item, kind: TypedListExprKind<Item>) -> Self {
        Self { item, kind }
    }

    pub(crate) fn item(&self) -> &Item {
        &self.item
    }

    pub(crate) fn element_type(&self) -> ValueType {
        self.item.value_type()
    }

    pub(crate) fn kind(&self) -> &TypedListExprKind<Item> {
        &self.kind
    }

    pub(super) fn value(item: Item, elements: Vec<Item::ElementExpr>) -> Self {
        Self::new(item, TypedListExprKind::Value(elements))
    }

    pub(super) fn spread(
        item: Item,
        elements: Vec<Item::ElementExpr>,
        tail: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::Spread {
                elements,
                tail: Box::new(tail),
            },
        )
    }

    pub(crate) fn local_get(item: Item, local: Item::Local, name: EcoString) -> Self {
        Self::new(item, TypedListExprKind::LocalGet { local, name })
    }

    pub(super) fn call(item: Item, function: Item::Function, args: Vec<CallArg>) -> Self {
        Self::new(item, TypedListExprKind::Call { function, args })
    }

    pub(super) fn function_call(
        item: Item,
        function: ListFunctionExpr,
        args: Vec<CallArg>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        )
    }

    pub(super) fn tuple_index(item: Item, tuple: TupleExpr, index: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    pub(super) fn list_index(item: Item, list: ListListExpr, index: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    pub(super) fn drop_first(item: Item, list: TypedListExpr<Item>, count: usize) -> Self {
        Self::new(
            item,
            TypedListExprKind::DropFirst {
                list: Box::new(list),
                count,
            },
        )
    }

    pub(super) fn panic(item: Item, panic: PanicExpr) -> Self {
        Self::new(item, TypedListExprKind::Panic(panic))
    }

    pub(super) fn bool_case(
        item: Item,
        subject: BoolExpr,
        true_: TypedListExpr<Item>,
        false_: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(super) fn int_case(
        item: Item,
        subject: IntExpr,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn string_case(
        item: Item,
        subject: StringExpr,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn float_case(
        item: Item,
        subject: FloatExpr,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        Self::new(
            item,
            TypedListExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn block(item: Item, steps: Vec<Step>, return_: TypedListExpr<Item>) -> Self {
        Self::new(
            item,
            TypedListExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }
}

#[cfg(test)]
macro_rules! impl_typed_list_expr_from_facade {
    ($type:ty, $method:ident, $name:literal) => {
        impl From<super::ListExpr> for $type {
            fn from(value: super::ListExpr) -> Self {
                value
                    .$method()
                    .expect(concat!("expected ", $name, " list expression"))
            }
        }
    };
}

#[cfg(test)]
impl_typed_list_expr_from_facade!(super::IntListExpr, into_int, "int");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::StringListExpr, into_string, "string");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::FloatListExpr, into_float, "float");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::BoolListExpr, into_bool, "bool");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::NilListExpr, into_nil, "nil");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::TupleListExpr, into_tuple, "tuple");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::ListListExpr, into_list, "list");
#[cfg(test)]
impl_typed_list_expr_from_facade!(super::FunctionListExpr, into_function, "function");

#[cfg(test)]
mod tests {
    use super::TypedListExprKind;
    use crate::plan::{Expr, FloatExpr, FloatListItem, ListExpr, ValueType};

    #[test]
    fn accessors_report_item_type_and_kind() {
        let expression =
            ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float)
                .into_float()
                .expect("float list");

        assert_eq!(expression.item(), &FloatListItem);
        assert_eq!(expression.element_type(), ValueType::Float);
        assert_eq!(
            expression.kind(),
            &TypedListExprKind::Value(vec![FloatExpr::value(1.5)]),
        );
    }
}
