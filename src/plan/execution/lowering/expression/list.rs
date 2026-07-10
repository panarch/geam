use super::super::super as execution;
use super::{
    bool_expr, call_args, float_expr, function_expr, int_expr, list_function_expr, panic_expr,
    string_expr, tuple_expr,
};
use crate::plan::module;

trait LowerListItem: module::ListItem {
    type Execution: execution::ListItem;

    fn lower_item(self) -> Self::Execution;
    fn lower_element(
        element: Self::ElementExpr,
    ) -> <Self::Execution as execution::ListItem>::ElementExpr;
    fn lower_local(local: Self::Local) -> <Self::Execution as execution::ListItem>::Local;
    fn lower_function(
        function: Self::Function,
    ) -> <Self::Execution as execution::ListItem>::Function;
}

pub(in crate::plan::execution::lowering) fn list_expr(
    expression: module::ListExpr,
) -> execution::ListExpr {
    match expression {
        module::ListExpr::Int(expression) => execution::ListExpr::Int(int_list_expr(expression)),
        module::ListExpr::String(expression) => {
            execution::ListExpr::String(string_list_expr(expression))
        }
        module::ListExpr::Float(expression) => {
            execution::ListExpr::Float(float_list_expr(expression))
        }
        module::ListExpr::Bool(expression) => execution::ListExpr::Bool(bool_list_expr(expression)),
        module::ListExpr::Nil(expression) => execution::ListExpr::Nil(nil_list_expr(expression)),
        module::ListExpr::Tuple(expression) => {
            execution::ListExpr::Tuple(tuple_list_expr(expression))
        }
        module::ListExpr::List(expression) => execution::ListExpr::List(list_list_expr(expression)),
        module::ListExpr::Function(expression) => {
            execution::ListExpr::Function(function_list_expr(expression))
        }
    }
}

pub(in crate::plan::execution::lowering) fn int_list_expr(
    expression: module::IntListExpr,
) -> execution::IntListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn string_list_expr(
    expression: module::StringListExpr,
) -> execution::StringListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn float_list_expr(
    expression: module::FloatListExpr,
) -> execution::FloatListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn bool_list_expr(
    expression: module::BoolListExpr,
) -> execution::BoolListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn nil_list_expr(
    expression: module::NilListExpr,
) -> execution::NilListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn tuple_list_expr(
    expression: module::TupleListExpr,
) -> execution::TupleListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn list_list_expr(
    expression: module::ListListExpr,
) -> execution::ListListExpr {
    typed_list_expr(expression)
}

pub(in crate::plan::execution::lowering) fn function_list_expr(
    expression: module::FunctionListExpr,
) -> execution::FunctionListExpr {
    typed_list_expr(expression)
}

fn typed_list_expr<Item>(
    expression: module::TypedListExpr<Item>,
) -> execution::TypedListExpr<Item::Execution>
where
    Item: LowerListItem,
{
    let (item, kind) = expression.into_item_and_kind();
    let item = item.lower_item();
    let kind = typed_list_kind::<Item>(kind);
    execution::TypedListExpr::from_item_and_kind(item, kind)
}

fn typed_list_kind<Item>(
    kind: module::TypedListExprKind<Item>,
) -> execution::TypedListExprKind<Item::Execution>
where
    Item: LowerListItem,
{
    use execution::TypedListExprKind as E;
    use module::TypedListExprKind as M;

    match kind {
        M::Value(elements) => E::Value(elements.into_iter().map(Item::lower_element).collect()),
        M::Spread { elements, tail } => E::Spread {
            elements: elements.into_iter().map(Item::lower_element).collect(),
            tail: Box::new(typed_list_kind::<Item>(*tail)),
        },
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: Item::lower_local(local),
        },
        M::Call { function, args } => E::Call {
            function: Item::lower_function(function),
            args: call_args(args),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(list_function_expr(*function)),
            args: call_args(args),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple)),
            index,
        },
        M::ListIndex(source) => {
            let (list, index) = source.into_parts();
            E::ListIndex(execution::ListIndexSource::from_parts(
                list_list_expr(list),
                index,
            ))
        }
        M::DropFirst { list, count } => E::DropFirst {
            list: Box::new(typed_list_kind::<Item>(*list)),
            count,
        },
        M::Panic(value) => E::Panic(panic_expr(value)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject)),
            true_: Box::new(typed_list_kind::<Item>(*true_)),
            false_: Box::new(typed_list_kind::<Item>(*false_)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps),
            return_: Box::new(typed_list_kind::<Item>(*return_)),
        },
    }
}

pub(in crate::plan::execution::lowering) fn list_local_expr(
    expression: module::ListLocalExpr,
) -> execution::ListLocalExpr {
    match expression {
        module::ListLocalExpr::Int { local, value } => execution::ListLocalExpr::Int {
            local: execution::IntListLocalId(local.0),
            value: int_list_expr(value),
        },
        module::ListLocalExpr::String { local, value } => execution::ListLocalExpr::String {
            local: execution::StringListLocalId(local.0),
            value: string_list_expr(value),
        },
        module::ListLocalExpr::Float { local, value } => execution::ListLocalExpr::Float {
            local: execution::FloatListLocalId(local.0),
            value: float_list_expr(value),
        },
        module::ListLocalExpr::Bool { local, value } => execution::ListLocalExpr::Bool {
            local: execution::BoolListLocalId(local.0),
            value: bool_list_expr(value),
        },
        module::ListLocalExpr::Nil { local, value } => execution::ListLocalExpr::Nil {
            local: execution::NilListLocalId(local.0),
            value: nil_list_expr(value),
        },
        module::ListLocalExpr::Tuple {
            local,
            item_type,
            value,
        } => execution::ListLocalExpr::Tuple {
            local: execution::TupleListLocalId(local.0),
            item_type,
            value: tuple_list_expr(value),
        },
        module::ListLocalExpr::List {
            local,
            item_type,
            value,
        } => execution::ListLocalExpr::List {
            local: execution::ListListLocalId(local.0),
            item_type,
            value: list_list_expr(value),
        },
        module::ListLocalExpr::Function {
            local,
            item_type,
            value,
        } => execution::ListLocalExpr::Function {
            local: execution::FunctionListLocalId(local.0),
            item_type,
            value: function_list_expr(value),
        },
    }
}

impl LowerListItem for module::IntListItem {
    type Execution = execution::IntListItem;

    fn lower_item(self) -> Self::Execution {
        execution::IntListItem
    }

    fn lower_element(element: Self::ElementExpr) -> execution::IntExpr {
        int_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::IntListLocalId {
        execution::IntListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::IntListFunctionId {
        execution::IntListFunctionId(function.0)
    }
}

impl LowerListItem for module::StringListItem {
    type Execution = execution::StringListItem;

    fn lower_item(self) -> Self::Execution {
        execution::StringListItem
    }

    fn lower_element(element: Self::ElementExpr) -> execution::StringExpr {
        string_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::StringListLocalId {
        execution::StringListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::StringListFunctionId {
        execution::StringListFunctionId(function.0)
    }
}

impl LowerListItem for module::FloatListItem {
    type Execution = execution::FloatListItem;

    fn lower_item(self) -> Self::Execution {
        execution::FloatListItem
    }

    fn lower_element(element: Self::ElementExpr) -> execution::FloatExpr {
        float_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::FloatListLocalId {
        execution::FloatListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::FloatListFunctionId {
        execution::FloatListFunctionId(function.0)
    }
}

impl LowerListItem for module::BoolListItem {
    type Execution = execution::BoolListItem;

    fn lower_item(self) -> Self::Execution {
        execution::BoolListItem
    }

    fn lower_element(element: Self::ElementExpr) -> execution::BoolExpr {
        bool_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::BoolListLocalId {
        execution::BoolListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::BoolListFunctionId {
        execution::BoolListFunctionId(function.0)
    }
}

impl LowerListItem for module::NilListItem {
    type Execution = execution::NilListItem;

    fn lower_item(self) -> Self::Execution {
        execution::NilListItem
    }

    fn lower_element(element: Self::ElementExpr) -> execution::NilExpr {
        super::nil_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::NilListLocalId {
        execution::NilListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::NilListFunctionId {
        execution::NilListFunctionId(function.0)
    }
}

impl LowerListItem for module::TupleListItem {
    type Execution = execution::TupleListItem;

    fn lower_item(self) -> Self::Execution {
        execution::TupleListItem::new(self.into_item_type())
    }

    fn lower_element(element: Self::ElementExpr) -> execution::TupleExpr {
        tuple_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::TupleListLocalId {
        execution::TupleListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::TupleListFunctionId {
        execution::TupleListFunctionId(function.0)
    }
}

impl LowerListItem for module::ListListItem {
    type Execution = execution::ListListItem;

    fn lower_item(self) -> Self::Execution {
        execution::ListListItem::new(self.into_item_type())
    }

    fn lower_element(element: Self::ElementExpr) -> execution::ListExpr {
        list_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::ListListLocalId {
        execution::ListListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::ListListFunctionId {
        execution::ListListFunctionId(function.0)
    }
}

impl LowerListItem for module::FunctionListItem {
    type Execution = execution::FunctionListItem;

    fn lower_item(self) -> Self::Execution {
        execution::FunctionListItem::new(self.into_item_type())
    }

    fn lower_element(element: Self::ElementExpr) -> execution::FunctionExpr {
        function_expr(element)
    }

    fn lower_local(local: Self::Local) -> execution::FunctionListLocalId {
        execution::FunctionListLocalId(local.0)
    }

    fn lower_function(function: Self::Function) -> execution::FunctionListFunctionId {
        execution::FunctionListFunctionId(function.0)
    }
}
