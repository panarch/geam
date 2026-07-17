use super::super::super as execution;
use super::{
    bit_array_expr, bool_expr, call_args, custom_expr, float_expr, function_expr,
    generic_bit_array_expr, generic_bool_expr, generic_float_expr, generic_int_expr,
    generic_nil_expr, generic_string_expr, generic_utf_codepoint_expr, int_expr,
    list_function_expr, panic_expr, string_expr, tuple_expr, utf_codepoint_expr,
};
use crate::plan::execution::lowering::LoweringContext;
use crate::plan::module;

trait LowerListItem: module::ListItem<Function = module::FunctionInstantiation> {
    type Execution: execution::ListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution;
    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::ElementExpr;
    fn lower_local(
        local: &Self::Local,
        _context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::Local;
    fn lower_function(
        function: &Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::Function;
}

pub(in crate::plan::execution::lowering) fn list_expr(
    expression: &module::ListExpr,
    context: &mut LoweringContext,
) -> execution::ListExpr {
    match expression {
        module::ListExpr::Generic(expression) => generic_list_expr(expression, context),
        module::ListExpr::Int(expression) => {
            execution::ListExpr::Int(int_list_expr(expression, context))
        }
        module::ListExpr::String(expression) => {
            execution::ListExpr::String(string_list_expr(expression, context))
        }
        module::ListExpr::BitArray(expression) => {
            execution::ListExpr::BitArray(bit_array_list_expr(expression, context))
        }
        module::ListExpr::UtfCodepoint(expression) => {
            execution::ListExpr::UtfCodepoint(utf_codepoint_list_expr(expression, context))
        }
        module::ListExpr::Custom(expression) => {
            execution::ListExpr::Custom(custom_list_expr(expression, context))
        }
        module::ListExpr::Float(expression) => {
            execution::ListExpr::Float(float_list_expr(expression, context))
        }
        module::ListExpr::Bool(expression) => {
            execution::ListExpr::Bool(bool_list_expr(expression, context))
        }
        module::ListExpr::Nil(expression) => {
            execution::ListExpr::Nil(nil_list_expr(expression, context))
        }
        module::ListExpr::Tuple(expression) => {
            execution::ListExpr::Tuple(tuple_list_expr(expression, context))
        }
        module::ListExpr::List(expression) => {
            execution::ListExpr::List(list_list_expr(expression, context))
        }
        module::ListExpr::Function(expression) => {
            execution::ListExpr::Function(function_list_expr(expression, context))
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_list_expr(
    expression: &module::GenericListExpr,
    context: &mut LoweringContext,
) -> execution::ListExpr {
    match context.concrete_parameter(expression.item().parameter()) {
        super::super::specialization::ConcreteValueShape::Int => {
            execution::ListExpr::Int(generic_int_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::String => {
            execution::ListExpr::String(generic_string_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::BitArray => {
            execution::ListExpr::BitArray(generic_bit_array_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::UtfCodepoint => {
            execution::ListExpr::UtfCodepoint(generic_utf_codepoint_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::Custom(shape) => {
            execution::ListExpr::Custom(generic_custom_list_expr(expression, &shape, context))
        }
        super::super::specialization::ConcreteValueShape::Float => {
            execution::ListExpr::Float(generic_float_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::Bool => {
            execution::ListExpr::Bool(generic_bool_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::Nil => {
            execution::ListExpr::Nil(generic_nil_list_expr(expression, context))
        }
        super::super::specialization::ConcreteValueShape::Tuple(elements) => {
            execution::ListExpr::Tuple(generic_tuple_list_expr(expression, &elements, context))
        }
        super::super::specialization::ConcreteValueShape::List(item) => {
            execution::ListExpr::List(generic_nested_list_expr(expression, &item, context))
        }
        super::super::specialization::ConcreteValueShape::Function(function) => {
            execution::ListExpr::Function(generic_function_list_expr(
                expression, &function, context,
            ))
        }
    }
}

pub(in crate::plan::execution::lowering) fn int_list_expr(
    expression: &module::IntListExpr,
    context: &mut LoweringContext,
) -> execution::IntListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn string_list_expr(
    expression: &module::StringListExpr,
    context: &mut LoweringContext,
) -> execution::StringListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bit_array_list_expr(
    expression: &module::BitArrayListExpr,
    context: &mut LoweringContext,
) -> execution::BitArrayListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn utf_codepoint_list_expr(
    expression: &module::UtfCodepointListExpr,
    context: &mut LoweringContext,
) -> execution::UtfCodepointListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn custom_list_expr(
    expression: &module::CustomListExpr,
    context: &mut LoweringContext,
) -> execution::CustomListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn float_list_expr(
    expression: &module::FloatListExpr,
    context: &mut LoweringContext,
) -> execution::FloatListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bool_list_expr(
    expression: &module::BoolListExpr,
    context: &mut LoweringContext,
) -> execution::BoolListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn nil_list_expr(
    expression: &module::NilListExpr,
    context: &mut LoweringContext,
) -> execution::NilListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn tuple_list_expr(
    expression: &module::TupleListExpr,
    context: &mut LoweringContext,
) -> execution::TupleListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn list_list_expr(
    expression: &module::ListListExpr,
    context: &mut LoweringContext,
) -> execution::ListListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn function_list_expr(
    expression: &module::FunctionListExpr,
    context: &mut LoweringContext,
) -> execution::FunctionListExpr {
    typed_list_expr(expression, context)
}

macro_rules! primitive_generic_list_expr {
    (
        $lower:ident,
        $result:ty,
        $item:ident,
        $type_id:ident,
        $element:ident,
        $local:ident,
        $function:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericListExpr,
            context: &mut LoweringContext,
        ) -> $result {
            let item = execution::$item::new(context.$type_id());
            generic_typed_list_expr(
                expression,
                item,
                $element,
                |index| execution::$local(index),
                |function, _, context| context.$function(function),
                context,
            )
        }
    };
}

primitive_generic_list_expr!(
    generic_int_list_expr,
    execution::IntListExpr,
    IntListItem,
    int_list_type,
    generic_int_expr,
    IntListLocalId,
    int_list_function_id
);
primitive_generic_list_expr!(
    generic_string_list_expr,
    execution::StringListExpr,
    StringListItem,
    string_list_type,
    generic_string_expr,
    StringListLocalId,
    string_list_function_id
);
primitive_generic_list_expr!(
    generic_bit_array_list_expr,
    execution::BitArrayListExpr,
    BitArrayListItem,
    bit_array_list_type,
    generic_bit_array_expr,
    BitArrayListLocalId,
    bit_array_list_function_id
);
primitive_generic_list_expr!(
    generic_utf_codepoint_list_expr,
    execution::UtfCodepointListExpr,
    UtfCodepointListItem,
    utf_codepoint_list_type,
    generic_utf_codepoint_expr,
    UtfCodepointListLocalId,
    utf_codepoint_list_function_id
);
primitive_generic_list_expr!(
    generic_float_list_expr,
    execution::FloatListExpr,
    FloatListItem,
    float_list_type,
    generic_float_expr,
    FloatListLocalId,
    float_list_function_id
);
primitive_generic_list_expr!(
    generic_bool_list_expr,
    execution::BoolListExpr,
    BoolListItem,
    bool_list_type,
    generic_bool_expr,
    BoolListLocalId,
    bool_list_function_id
);
primitive_generic_list_expr!(
    generic_nil_list_expr,
    execution::NilListExpr,
    NilListItem,
    nil_list_type,
    generic_nil_expr,
    NilListLocalId,
    nil_list_function_id
);

pub(in crate::plan::execution::lowering) fn generic_tuple_list_expr(
    expression: &module::GenericListExpr,
    elements: &[super::super::specialization::ConcreteValueShape],
    context: &mut LoweringContext,
) -> execution::TupleListExpr {
    let item = execution::TupleListItem::new(
        context.tuple_list_type(
            elements
                .iter()
                .map(super::super::specialization::ConcreteValueShape::value_type)
                .collect(),
        ),
    );
    generic_typed_list_expr(
        expression,
        item,
        |element, context| super::generic_tuple_expr(element, elements, context),
        execution::TupleListLocalId,
        |function, item, context| context.tuple_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_custom_list_expr(
    expression: &module::GenericListExpr,
    shape: &super::super::specialization::ConcreteCustomValueShape,
    context: &mut LoweringContext,
) -> execution::CustomListExpr {
    let item = execution::CustomListItem::new(
        context.custom_list_type(shape.to_module_shape().type_().clone()),
    );
    generic_typed_list_expr(
        expression,
        item,
        |element, context| super::generic_custom_expr(element, shape, context),
        execution::CustomListLocalId,
        |function, item, context| context.custom_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_nested_list_expr(
    expression: &module::GenericListExpr,
    item_shape: &super::super::specialization::ConcreteValueShape,
    context: &mut LoweringContext,
) -> execution::ListListExpr {
    let item = execution::ListListItem::new(context.list_list_type(item_shape.value_type()));
    generic_typed_list_expr(
        expression,
        item,
        |element, context| super::generic_list_value_expr(element, item_shape, context),
        execution::ListListLocalId,
        |function, item, context| context.list_list_function_id(function, item.type_id()),
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_function_list_expr(
    expression: &module::GenericListExpr,
    function_shape: &super::super::specialization::ConcreteFunctionShape,
    context: &mut LoweringContext,
) -> execution::FunctionListExpr {
    let item = execution::FunctionListItem::new(
        context.function_list_type(function_shape.to_module_shape().type_()),
    );
    generic_typed_list_expr(
        expression,
        item,
        |element, context| super::generic_function_value_expr(element, function_shape, context),
        execution::FunctionListLocalId,
        |function, item, context| context.function_list_function_id(function, item.type_id()),
        context,
    )
}

fn generic_typed_list_expr<Item>(
    expression: &module::GenericListExpr,
    item: Item,
    lower_element: impl Copy + Fn(&module::GenericExpr, &mut LoweringContext) -> Item::ElementExpr,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut LoweringContext,
    ) -> Item::Function,
    context: &mut LoweringContext,
) -> execution::TypedListExpr<Item>
where
    Item: execution::ListItem,
{
    let kind = generic_typed_list_kind(
        expression.kind(),
        &item,
        lower_element,
        lower_local,
        lower_function,
        context,
    );
    execution::TypedListExpr::from_item_and_kind(item, kind)
}

fn generic_typed_list_kind<Item>(
    kind: &module::TypedListExprKind<module::GenericListItem>,
    item: &Item,
    lower_element: impl Copy + Fn(&module::GenericExpr, &mut LoweringContext) -> Item::ElementExpr,
    lower_local: impl Copy + Fn(usize) -> Item::Local,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &Item,
        &mut LoweringContext,
    ) -> Item::Function,
    context: &mut LoweringContext,
) -> execution::TypedListExprKind<Item>
where
    Item: execution::ListItem,
{
    use execution::TypedListExprKind as E;
    use module::TypedListExprKind as M;

    match kind {
        M::Value(elements) => E::Value(
            elements
                .iter()
                .map(|element| lower_element(element, context))
                .collect(),
        ),
        M::Spread { elements, tail } => E::Spread {
            elements: elements
                .iter()
                .map(|element| lower_element(element, context))
                .collect(),
            tail: Box::new(generic_typed_list_kind(
                tail,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: lower_local(context.generic_list_local_index(*local)),
        },
        M::Call { function, args } => E::Call {
            function: lower_function(function, item, context),
            args: super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(list_function_expr(function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex(source) => E::ListIndex(execution::ListIndexSource::from_parts(
            list_list_expr(source.list(), context),
            source.index(),
        )),
        M::DropFirst { list, count } => E::DropFirst {
            list: Box::new(generic_typed_list_kind(
                list,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
            count: *count,
        },
        M::Panic(panic) => E::Panic(panic_expr(panic, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(subject, context)),
            true_: Box::new(generic_typed_list_kind(
                true_,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
            false_: Box::new(generic_typed_list_kind(
                false_,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_typed_list_kind(
                            branch,
                            item,
                            lower_element,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_typed_list_kind(
                fallback,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        generic_typed_list_kind(
                            branch,
                            item,
                            lower_element,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_typed_list_kind(
                fallback,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        generic_typed_list_kind(
                            branch,
                            item,
                            lower_element,
                            lower_local,
                            lower_function,
                            context,
                        ),
                    )
                })
                .collect(),
            fallback: Box::new(generic_typed_list_kind(
                fallback,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(generic_typed_list_kind(
                return_,
                item,
                lower_element,
                lower_local,
                lower_function,
                context,
            )),
        },
    }
}

fn typed_list_expr<Item>(
    expression: &module::TypedListExpr<Item>,
    context: &mut LoweringContext,
) -> execution::TypedListExpr<Item::Execution>
where
    Item: LowerListItem,
{
    let item = expression.item().lower_item(context);
    let kind = typed_list_kind::<Item>(expression.kind(), &item, context);
    execution::TypedListExpr::from_item_and_kind(item, kind)
}

fn typed_list_kind<Item>(
    kind: &module::TypedListExprKind<Item>,
    item: &Item::Execution,
    context: &mut LoweringContext,
) -> execution::TypedListExprKind<Item::Execution>
where
    Item: LowerListItem,
{
    use execution::TypedListExprKind as E;
    use module::TypedListExprKind as M;

    match kind {
        M::Value(elements) => E::Value(
            elements
                .iter()
                .map(|element| Item::lower_element(element, context))
                .collect(),
        ),
        M::Spread { elements, tail } => E::Spread {
            elements: elements
                .iter()
                .map(|element| Item::lower_element(element, context))
                .collect(),
            tail: Box::new(typed_list_kind::<Item>(tail, item, context)),
        },
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: Item::lower_local(local, context),
        },
        M::Call { function, args } => E::Call {
            function: Item::lower_function(function, item, context),
            args: super::direct_call_args(function, args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(list_function_expr(function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(tuple, context)),
            index: *index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex(source) => E::ListIndex(execution::ListIndexSource::from_parts(
            list_list_expr(source.list(), context),
            source.index(),
        )),
        M::DropFirst { list, count } => E::DropFirst {
            list: Box::new(typed_list_kind::<Item>(list, item, context)),
            count: *count,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(subject, context)),
            true_: Box::new(typed_list_kind::<Item>(true_, item, context)),
            false_: Box::new(typed_list_kind::<Item>(false_, item, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        typed_list_kind::<Item>(branch, item, context),
                    )
                })
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(fallback, item, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        typed_list_kind::<Item>(branch, item, context),
                    )
                })
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(fallback, item, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(subject, context)),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| (*pattern, typed_list_kind::<Item>(branch, item, context)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(fallback, item, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(typed_list_kind::<Item>(return_, item, context)),
        },
    }
}

pub(in crate::plan::execution::lowering) fn list_local_expr(
    expression: &module::ListLocalExpr,
    context: &mut LoweringContext,
) -> execution::ListLocalExpr {
    let index = context.local_index(super::list_local_expr_key(expression));
    list_local_expr_at(index, expression, context)
}

pub(super) fn list_local_expr_at(
    index: usize,
    expression: &module::ListLocalExpr,
    context: &mut LoweringContext,
) -> execution::ListLocalExpr {
    match expression {
        module::ListLocalExpr::Generic { value, .. } => {
            specialized_list_local_expr(index, generic_list_expr(value, context))
        }
        module::ListLocalExpr::Int { value, .. } => execution::ListLocalExpr::Int {
            local: execution::IntListLocalId(index),
            value: int_list_expr(value, context),
        },
        module::ListLocalExpr::String { value, .. } => execution::ListLocalExpr::String {
            local: execution::StringListLocalId(index),
            value: string_list_expr(value, context),
        },
        module::ListLocalExpr::BitArray { value, .. } => execution::ListLocalExpr::BitArray {
            local: execution::BitArrayListLocalId(index),
            value: bit_array_list_expr(value, context),
        },
        module::ListLocalExpr::UtfCodepoint { value, .. } => {
            execution::ListLocalExpr::UtfCodepoint {
                local: execution::UtfCodepointListLocalId(index),
                value: utf_codepoint_list_expr(value, context),
            }
        }
        module::ListLocalExpr::Custom { value, .. } => execution::ListLocalExpr::Custom {
            local: execution::CustomListLocalId(index),
            value: custom_list_expr(value, context),
        },
        module::ListLocalExpr::Float { value, .. } => execution::ListLocalExpr::Float {
            local: execution::FloatListLocalId(index),
            value: float_list_expr(value, context),
        },
        module::ListLocalExpr::Bool { value, .. } => execution::ListLocalExpr::Bool {
            local: execution::BoolListLocalId(index),
            value: bool_list_expr(value, context),
        },
        module::ListLocalExpr::Nil { value, .. } => execution::ListLocalExpr::Nil {
            local: execution::NilListLocalId(index),
            value: nil_list_expr(value, context),
        },
        module::ListLocalExpr::Tuple { value, .. } => execution::ListLocalExpr::Tuple {
            local: execution::TupleListLocalId(index),
            value: tuple_list_expr(value, context),
        },
        module::ListLocalExpr::List { value, .. } => execution::ListLocalExpr::List {
            local: execution::ListListLocalId(index),
            value: list_list_expr(value, context),
        },
        module::ListLocalExpr::Function { value, .. } => execution::ListLocalExpr::Function {
            local: execution::FunctionListLocalId(index),
            value: function_list_expr(value, context),
        },
    }
}

pub(super) fn specialized_list_local_expr(
    index: usize,
    value: execution::ListExpr,
) -> execution::ListLocalExpr {
    match value {
        execution::ListExpr::Int(value) => execution::ListLocalExpr::Int {
            local: execution::IntListLocalId(index),
            value,
        },
        execution::ListExpr::String(value) => execution::ListLocalExpr::String {
            local: execution::StringListLocalId(index),
            value,
        },
        execution::ListExpr::BitArray(value) => execution::ListLocalExpr::BitArray {
            local: execution::BitArrayListLocalId(index),
            value,
        },
        execution::ListExpr::UtfCodepoint(value) => execution::ListLocalExpr::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(index),
            value,
        },
        execution::ListExpr::Custom(value) => execution::ListLocalExpr::Custom {
            local: execution::CustomListLocalId(index),
            value,
        },
        execution::ListExpr::Float(value) => execution::ListLocalExpr::Float {
            local: execution::FloatListLocalId(index),
            value,
        },
        execution::ListExpr::Bool(value) => execution::ListLocalExpr::Bool {
            local: execution::BoolListLocalId(index),
            value,
        },
        execution::ListExpr::Nil(value) => execution::ListLocalExpr::Nil {
            local: execution::NilListLocalId(index),
            value,
        },
        execution::ListExpr::Tuple(value) => execution::ListLocalExpr::Tuple {
            local: execution::TupleListLocalId(index),
            value,
        },
        execution::ListExpr::List(value) => execution::ListLocalExpr::List {
            local: execution::ListListLocalId(index),
            value,
        },
        execution::ListExpr::Function(value) => execution::ListLocalExpr::Function {
            local: execution::FunctionListLocalId(index),
            value,
        },
    }
}

impl LowerListItem for module::IntListItem {
    type Execution = execution::IntListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::IntListItem::new(context.int_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::IntExpr {
        int_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::IntListLocalId {
        execution::IntListLocalId(
            context.mapped_local(super::super::frame::LocalKind::IntList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::IntListFunctionId {
        context.int_list_function_id(function)
    }
}

impl LowerListItem for module::StringListItem {
    type Execution = execution::StringListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::StringListItem::new(context.string_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::StringExpr {
        string_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::StringListLocalId {
        execution::StringListLocalId(
            context.mapped_local(super::super::frame::LocalKind::StringList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::StringListFunctionId {
        context.string_list_function_id(function)
    }
}

impl LowerListItem for module::BitArrayListItem {
    type Execution = execution::BitArrayListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::BitArrayListItem::new(context.bit_array_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::BitArrayExpr {
        bit_array_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::BitArrayListLocalId {
        execution::BitArrayListLocalId(
            context.mapped_local(super::super::frame::LocalKind::BitArrayList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::BitArrayListFunctionId {
        context.bit_array_list_function_id(function)
    }
}

impl LowerListItem for module::UtfCodepointListItem {
    type Execution = execution::UtfCodepointListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::UtfCodepointListItem::new(context.utf_codepoint_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::UtfCodepointExpr {
        utf_codepoint_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::UtfCodepointListLocalId {
        execution::UtfCodepointListLocalId(
            context.mapped_local(super::super::frame::LocalKind::UtfCodepointList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::UtfCodepointListFunctionId {
        context.utf_codepoint_list_function_id(function)
    }
}

impl LowerListItem for module::CustomListItem {
    type Execution = execution::CustomListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::CustomListItem::new(context.custom_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::CustomExpr {
        custom_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::CustomListLocalId {
        execution::CustomListLocalId(
            context.mapped_local(super::super::frame::LocalKind::CustomList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::CustomListFunctionId {
        context.custom_list_function_id(function, item.type_id())
    }
}

impl LowerListItem for module::FloatListItem {
    type Execution = execution::FloatListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::FloatListItem::new(context.float_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::FloatExpr {
        float_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::FloatListLocalId {
        execution::FloatListLocalId(
            context.mapped_local(super::super::frame::LocalKind::FloatList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::FloatListFunctionId {
        context.float_list_function_id(function)
    }
}

impl LowerListItem for module::BoolListItem {
    type Execution = execution::BoolListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::BoolListItem::new(context.bool_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::BoolExpr {
        bool_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::BoolListLocalId {
        execution::BoolListLocalId(
            context.mapped_local(super::super::frame::LocalKind::BoolList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::BoolListFunctionId {
        context.bool_list_function_id(function)
    }
}

impl LowerListItem for module::NilListItem {
    type Execution = execution::NilListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::NilListItem::new(context.nil_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::NilExpr {
        super::nil_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::NilListLocalId {
        execution::NilListLocalId(
            context.mapped_local(super::super::frame::LocalKind::NilList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::NilListFunctionId {
        context.nil_list_function_id(function)
    }
}

impl LowerListItem for module::TupleListItem {
    type Execution = execution::TupleListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::TupleListItem::new(context.tuple_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::TupleExpr {
        tuple_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::TupleListLocalId {
        execution::TupleListLocalId(
            context.mapped_local(super::super::frame::LocalKind::TupleList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::TupleListFunctionId {
        context.tuple_list_function_id(function, item.type_id())
    }
}

impl LowerListItem for module::ListListItem {
    type Execution = execution::ListListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::ListListItem::new(context.list_list_type(*self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::ListExpr {
        list_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::ListListLocalId {
        execution::ListListLocalId(
            context.mapped_local(super::super::frame::LocalKind::ListList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::ListListFunctionId {
        context.list_list_function_id(function, item.type_id())
    }
}

impl LowerListItem for module::FunctionListItem {
    type Execution = execution::FunctionListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::FunctionListItem::new(context.function_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::FunctionExpr {
        function_expr(element, context)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::FunctionListLocalId {
        execution::FunctionListLocalId(
            context.mapped_local(super::super::frame::LocalKind::FunctionList, local.0),
        )
    }

    fn lower_function(
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> execution::FunctionListFunctionId {
        context.function_list_function_id(function, item.type_id())
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        ExecutionPlan, IntListFunctionId, ListFunctionId, ListItem, ListLocalExpr, ReturnBody,
        ReturnBodyKind, RuntimeFunctionId, Step, StepKind, TypedListExpr, TypedListExprKind,
    };

    #[test]
    fn lowering_derives_nested_list_index_result_from_parent_type() {
        let source = r#"
pub fn main() {
  case [[1]] {
    [first, ..] -> first
    _ -> []
  }
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let main = expect_int_list_main(&plan);
        let (_, return_) = expect_block(plan.int_list_function(main).return_());
        let true_ = expect_bool_case_true(return_);
        let (steps, _) = expect_block(true_);
        let value = expect_int_list_binding(&steps[0]);
        let source = expect_list_index(value);

        assert_eq!(
            source.list().item().type_id().item_type(),
            value.item().type_id().list_type(),
        );
        assert_eq!(source.index(), 0);
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) main function")]
    fn int_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_int_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a block return body")]
    fn block_return_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_block(plan.int_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a Bool case return body")]
    fn bool_case_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_bool_case_true(plan.int_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) binding step")]
    fn int_list_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(Int) { let value = 1 [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_int_list_binding(&plan.int_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a list-index expression")]
    fn list_index_fixture_guard_rejects_list_value() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let value = expect_expression(plan.int_list_function(main).return_());
        let _ = expect_list_index(value);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn expression_return_fixture_guard_rejects_block_return() {
        let plan = execution_plan(
            r#"
pub fn main() {
  case [[1]] {
    [first, ..] -> first
    _ -> []
  }
}
"#,
        );
        let main = expect_int_list_main(&plan);
        let _ = expect_expression(plan.int_list_function(main).return_());
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_int_list_main(plan: &ExecutionPlan) -> IntListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::Int(main)) => main,
            _ => panic!("expected a List(Int) main function"),
        }
    }

    fn expect_block<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> (&[Step], &ReturnBody<Expression, Function>) {
        match body.kind() {
            ReturnBodyKind::Block { steps, return_ } => (steps, return_),
            _ => panic!("expected a block return body"),
        }
    }

    fn expect_bool_case_true<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &ReturnBody<Expression, Function> {
        match body.kind() {
            ReturnBodyKind::BoolCase { true_, .. } => true_,
            _ => panic!("expected a Bool case return body"),
        }
    }

    fn expect_int_list_binding(step: &Step) -> &TypedListExpr<crate::plan::execution::IntListItem> {
        match step.kind() {
            StepKind::LetList {
                value: ListLocalExpr::Int { value, .. },
            } => value,
            _ => panic!("expected a List(Int) binding step"),
        }
    }

    fn expect_list_index<Item: ListItem>(
        expression: &TypedListExpr<Item>,
    ) -> &crate::plan::execution::ListIndexSource<Item> {
        match expression.kind() {
            TypedListExprKind::ListIndex(source) => source,
            _ => panic!("expected a list-index expression"),
        }
    }

    fn expect_expression<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }
}
