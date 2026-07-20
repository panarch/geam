use super::ListItem;
use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CallArg, FloatExpr, IntExpr, ListFunctionExpr, PanicExpr, Step, StringExpr,
    TupleExpr, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;
use vec1::Vec1;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedListExpr<Item: ListItem> {
    item_shape: crate::plan::ValueShape,
    pub(super) item: Item,
    pub(super) kind: TypedListExprKind<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ListIndexSource<Item: ListItem> {
    list: Box<Item::IndexSource>,
    index: usize,
    result_item: PhantomData<fn() -> Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TypedListExprKind<Item: ListItem> {
    Value(Vec<Item::ElementExpr>),
    Constant(Item::Constant),
    Spread {
        elements: Vec1<Item::ElementExpr>,
        tail: Box<TypedListExprKind<Item>>,
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
    CustomField(CustomFieldAccess),
    ListIndex(ListIndexSource<Item>),
    DropFirst {
        list: Box<TypedListExprKind<Item>>,
        count: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TypedListExprKind<Item>>,
        false_: Box<TypedListExprKind<Item>>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TypedListExprKind<Item>)>,
        fallback: Box<TypedListExprKind<Item>>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TypedListExprKind<Item>>,
    },
}

pub(crate) enum TypedListReturnKind<Item: ListItem> {
    Call {
        function: Item::Function,
        args: Vec<CallArg>,
    },
    BoolCase {
        subject: BoolExpr,
        true_: TypedListExpr<Item>,
        false_: TypedListExpr<Item>,
    },
    IntCase {
        subject: IntExpr,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    },
    StringCase {
        subject: StringExpr,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    },
    FloatCase {
        subject: FloatExpr,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    },
    Block {
        steps: Vec<Step>,
        return_: TypedListExpr<Item>,
    },
    Expr(TypedListExpr<Item>),
}

impl<Item: ListItem> ListIndexSource<Item> {
    pub(super) fn new(list: Item::IndexSource, index: usize) -> Self {
        Self {
            list: Box::new(list),
            index,
            result_item: PhantomData,
        }
    }

    pub(crate) fn list(&self) -> &Item::IndexSource {
        &self.list
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl<Item: ListItem> TypedListExpr<Item> {
    fn new(item: Item, kind: TypedListExprKind<Item>) -> Self {
        let item_shape = crate::plan::ValueShape::from_value_type(item.value_type());
        Self {
            item_shape,
            item,
            kind,
        }
    }

    fn from_shape_item_and_kind(
        item_shape: crate::plan::ValueShape,
        item: Item,
        kind: TypedListExprKind<Item>,
    ) -> Self {
        Self {
            item_shape,
            item,
            kind,
        }
    }

    pub(super) fn constant(
        item_shape: crate::plan::ValueShape,
        item: Item,
        reference: Item::Constant,
    ) -> Self {
        Self::from_shape_item_and_kind(item_shape, item, TypedListExprKind::Constant(reference))
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

    pub(crate) fn item_shape(&self) -> &crate::plan::ValueShape {
        &self.item_shape
    }

    pub(in crate::plan::module) fn with_item_shape(
        mut self,
        item_shape: crate::plan::ValueShape,
    ) -> Self {
        self.item_shape = item_shape;
        self
    }

    pub(crate) fn into_item_and_kind(self) -> (Item, TypedListExprKind<Item>) {
        (self.item, self.kind)
    }

    pub(crate) fn into_shape_item_and_kind(
        self,
    ) -> (crate::plan::ValueShape, Item, TypedListExprKind<Item>) {
        (self.item_shape, self.item, self.kind)
    }

    pub(crate) fn into_return_kind(self) -> TypedListReturnKind<Item> {
        let (item_shape, item, kind) = self.into_shape_item_and_kind();
        match kind {
            TypedListExprKind::Constant(reference) => {
                TypedListReturnKind::Expr(Self::from_shape_item_and_kind(
                    item_shape,
                    item,
                    TypedListExprKind::Constant(reference),
                ))
            }
            TypedListExprKind::Call { function, args } => {
                TypedListReturnKind::Call { function, args }
            }
            TypedListExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => TypedListReturnKind::BoolCase {
                subject: *subject,
                true_: Self::from_shape_item_and_kind(item_shape.clone(), item.clone(), *true_),
                false_: Self::from_shape_item_and_kind(item_shape, item, *false_),
            },
            TypedListExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => TypedListReturnKind::IntCase {
                subject: *subject,
                clauses: clauses
                    .into_iter()
                    .map(|(value, branch)| {
                        (
                            value,
                            Self::from_shape_item_and_kind(
                                item_shape.clone(),
                                item.clone(),
                                branch,
                            ),
                        )
                    })
                    .collect(),
                fallback: Self::from_shape_item_and_kind(item_shape, item, *fallback),
            },
            TypedListExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => TypedListReturnKind::StringCase {
                subject: *subject,
                clauses: clauses
                    .into_iter()
                    .map(|(value, branch)| {
                        (
                            value,
                            Self::from_shape_item_and_kind(
                                item_shape.clone(),
                                item.clone(),
                                branch,
                            ),
                        )
                    })
                    .collect(),
                fallback: Self::from_shape_item_and_kind(item_shape, item, *fallback),
            },
            TypedListExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => TypedListReturnKind::FloatCase {
                subject: *subject,
                clauses: clauses
                    .into_iter()
                    .map(|(value, branch)| {
                        (
                            value,
                            Self::from_shape_item_and_kind(
                                item_shape.clone(),
                                item.clone(),
                                branch,
                            ),
                        )
                    })
                    .collect(),
                fallback: Self::from_shape_item_and_kind(item_shape, item, *fallback),
            },
            TypedListExprKind::Block { steps, return_ } => TypedListReturnKind::Block {
                steps,
                return_: Self::from_shape_item_and_kind(item_shape, item, *return_),
            },
            kind => {
                TypedListReturnKind::Expr(Self::from_shape_item_and_kind(item_shape, item, kind))
            }
        }
    }

    pub(in crate::plan::module) fn value(item: Item, elements: Vec<Item::ElementExpr>) -> Self {
        Self::new(item, TypedListExprKind::Value(elements))
    }

    pub(in crate::plan::module) fn spread(
        elements: Vec1<Item::ElementExpr>,
        tail: TypedListExpr<Item>,
    ) -> Self {
        let (item_shape, item, tail) = tail.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
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

    pub(super) fn custom_field(item: Item, access: CustomFieldAccess) -> Self {
        Self::new(item, TypedListExprKind::CustomField(access))
    }

    pub(super) fn from_list_index(item: Item, source: ListIndexSource<Item>) -> Self {
        Self::new(item, TypedListExprKind::ListIndex(source))
    }

    pub(super) fn drop_first(list: TypedListExpr<Item>, count: usize) -> Self {
        let (item_shape, item, list) = list.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
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
        subject: BoolExpr,
        true_: TypedListExpr<Item>,
        false_: TypedListExpr<Item>,
    ) -> Self {
        let (item_shape, item, true_) = true_.into_shape_item_and_kind();
        let (_, false_) = false_.into_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
            item,
            TypedListExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(super) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| {
                let (_, branch) = branch.into_item_and_kind();
                (pattern, branch)
            })
            .collect();
        let (item_shape, item, fallback) = fallback.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
            item,
            TypedListExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| {
                let (_, branch) = branch.into_item_and_kind();
                (pattern, branch)
            })
            .collect();
        let (item_shape, item, fallback) = fallback.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
            item,
            TypedListExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, TypedListExpr<Item>)>,
        fallback: TypedListExpr<Item>,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| {
                let (_, branch) = branch.into_item_and_kind();
                (pattern, branch)
            })
            .collect();
        let (item_shape, item, fallback) = fallback.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
            item,
            TypedListExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(super) fn block(steps: Vec<Step>, return_: TypedListExpr<Item>) -> Self {
        let (item_shape, item, return_) = return_.into_shape_item_and_kind();
        Self::from_shape_item_and_kind(
            item_shape,
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
impl_typed_list_expr_from_facade!(super::BitArrayListExpr, into_bit_array, "bit array");
#[cfg(test)]
impl_typed_list_expr_from_facade!(
    super::UtfCodepointListExpr,
    into_utf_codepoint,
    "utf codepoint"
);
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
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FloatListItem, IntExpr, ListExpr, TupleExpr, TupleListExpr,
        TupleListItem, ValueType,
    };

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

    #[test]
    fn same_result_children_share_the_root_item() {
        let item = TupleListItem::new(vec![ValueType::Int]);
        let first = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        let second = TupleExpr::value(
            vec![Expr::int(IntExpr::value(2.into()))],
            vec![ValueType::Int],
        );
        let third = TupleExpr::value(
            vec![Expr::int(IntExpr::value(3.into()))],
            vec![ValueType::Int],
        );

        let true_ =
            TupleListExpr::drop_first(TupleListExpr::value(item.clone(), vec![second.clone()]), 0);
        let false_ = TupleListExpr::value(item.clone(), vec![third.clone()]);
        let tail = TupleListExpr::block(
            Vec::new(),
            TupleListExpr::bool_case(BoolExpr::value(true), true_, false_),
        );
        let expression = TupleListExpr::spread(vec1::vec1![first.clone()], tail);

        assert_eq!(expression.item(), &item);
        assert_eq!(
            expression.kind(),
            &TypedListExprKind::Spread {
                elements: vec1::vec1![first],
                tail: Box::new(TypedListExprKind::Block {
                    steps: Vec::new(),
                    return_: Box::new(TypedListExprKind::BoolCase {
                        subject: Box::new(BoolExpr::value(true)),
                        true_: Box::new(TypedListExprKind::DropFirst {
                            list: Box::new(TypedListExprKind::Value(vec![second])),
                            count: 0,
                        }),
                        false_: Box::new(TypedListExprKind::Value(vec![third])),
                    }),
                }),
            },
        );
    }
}
