use super::super::super as execution;
use super::{
    bit_array_expr, bool_expr, call_args, custom_expr, float_expr, function_expr, int_expr,
    list_function_expr, panic_expr, string_expr, tuple_expr, utf_codepoint_expr,
};
use crate::plan::execution::lowering::LoweringContext;
use crate::plan::module;

trait LowerListItem: module::ListItem {
    type Execution: execution::ListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution;
    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::ElementExpr;
    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::Local;
    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::Function;
}

pub(in crate::plan::execution::lowering) fn list_expr(
    expression: module::ListExpr,
    context: &mut LoweringContext,
) -> execution::ListExpr {
    match expression {
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

pub(in crate::plan::execution::lowering) fn int_list_expr(
    expression: module::IntListExpr,
    context: &mut LoweringContext,
) -> execution::IntListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn string_list_expr(
    expression: module::StringListExpr,
    context: &mut LoweringContext,
) -> execution::StringListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bit_array_list_expr(
    expression: module::BitArrayListExpr,
    context: &mut LoweringContext,
) -> execution::BitArrayListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn utf_codepoint_list_expr(
    expression: module::UtfCodepointListExpr,
    context: &mut LoweringContext,
) -> execution::UtfCodepointListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn custom_list_expr(
    expression: module::CustomListExpr,
    context: &mut LoweringContext,
) -> execution::CustomListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn float_list_expr(
    expression: module::FloatListExpr,
    context: &mut LoweringContext,
) -> execution::FloatListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bool_list_expr(
    expression: module::BoolListExpr,
    context: &mut LoweringContext,
) -> execution::BoolListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn nil_list_expr(
    expression: module::NilListExpr,
    context: &mut LoweringContext,
) -> execution::NilListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn tuple_list_expr(
    expression: module::TupleListExpr,
    context: &mut LoweringContext,
) -> execution::TupleListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn list_list_expr(
    expression: module::ListListExpr,
    context: &mut LoweringContext,
) -> execution::ListListExpr {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn function_list_expr(
    expression: module::FunctionListExpr,
    context: &mut LoweringContext,
) -> execution::FunctionListExpr {
    typed_list_expr(expression, context)
}

fn typed_list_expr<Item>(
    expression: module::TypedListExpr<Item>,
    context: &mut LoweringContext,
) -> execution::TypedListExpr<Item::Execution>
where
    Item: LowerListItem,
{
    let (item, kind) = expression.into_item_and_kind();
    let item = item.lower_item(context);
    let kind = typed_list_kind::<Item>(kind, &item, context);
    execution::TypedListExpr::from_item_and_kind(item, kind)
}

fn typed_list_kind<Item>(
    kind: module::TypedListExprKind<Item>,
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
                .into_iter()
                .map(|element| Item::lower_element(element, context))
                .collect(),
        ),
        M::Spread { elements, tail } => E::Spread {
            elements: elements
                .into_iter()
                .map(|element| Item::lower_element(element, context))
                .collect(),
            tail: Box::new(typed_list_kind::<Item>(*tail, item, context)),
        },
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: Item::lower_local(local, context),
        },
        M::Call { function, args } => E::Call {
            function: Item::lower_function(function, item, context),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(list_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::CustomField(access) => E::CustomField(super::custom_field_access(access, context)),
        M::ListIndex(source) => {
            let (list, index) = source.into_parts();
            E::ListIndex(execution::ListIndexSource::from_parts(
                list_list_expr(list, context),
                index,
            ))
        }
        M::DropFirst { list, count } => E::DropFirst {
            list: Box::new(typed_list_kind::<Item>(*list, item, context)),
            count,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(typed_list_kind::<Item>(*true_, item, context)),
            false_: Box::new(typed_list_kind::<Item>(*false_, item, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch, item, context)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback, item, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch, item, context)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback, item, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, typed_list_kind::<Item>(branch, item, context)))
                .collect(),
            fallback: Box::new(typed_list_kind::<Item>(*fallback, item, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(typed_list_kind::<Item>(*return_, item, context)),
        },
    }
}

pub(in crate::plan::execution::lowering) fn list_local_expr(
    expression: module::ListLocalExpr,
    context: &mut LoweringContext,
) -> execution::ListLocalExpr {
    match expression {
        module::ListLocalExpr::Int { local, value } => execution::ListLocalExpr::Int {
            local: execution::IntListLocalId(local.0),
            value: int_list_expr(value, context),
        },
        module::ListLocalExpr::String { local, value } => execution::ListLocalExpr::String {
            local: execution::StringListLocalId(local.0),
            value: string_list_expr(value, context),
        },
        module::ListLocalExpr::BitArray { local, value } => execution::ListLocalExpr::BitArray {
            local: execution::BitArrayListLocalId(local.0),
            value: bit_array_list_expr(value, context),
        },
        module::ListLocalExpr::UtfCodepoint { local, value } => {
            execution::ListLocalExpr::UtfCodepoint {
                local: execution::UtfCodepointListLocalId(local.0),
                value: utf_codepoint_list_expr(value, context),
            }
        }
        module::ListLocalExpr::Custom {
            local,
            item_type: _,
            value,
        } => execution::ListLocalExpr::Custom {
            local: execution::CustomListLocalId(local.0),
            value: custom_list_expr(value, context),
        },
        module::ListLocalExpr::Float { local, value } => execution::ListLocalExpr::Float {
            local: execution::FloatListLocalId(local.0),
            value: float_list_expr(value, context),
        },
        module::ListLocalExpr::Bool { local, value } => execution::ListLocalExpr::Bool {
            local: execution::BoolListLocalId(local.0),
            value: bool_list_expr(value, context),
        },
        module::ListLocalExpr::Nil { local, value } => execution::ListLocalExpr::Nil {
            local: execution::NilListLocalId(local.0),
            value: nil_list_expr(value, context),
        },
        module::ListLocalExpr::Tuple {
            local,
            item_type: _,
            value,
        } => execution::ListLocalExpr::Tuple {
            local: execution::TupleListLocalId(local.0),
            value: tuple_list_expr(value, context),
        },
        module::ListLocalExpr::List {
            local,
            item_type: _,
            value,
        } => execution::ListLocalExpr::List {
            local: execution::ListListLocalId(local.0),
            value: list_list_expr(value, context),
        },
        module::ListLocalExpr::Function {
            local,
            item_type: _,
            value,
        } => execution::ListLocalExpr::Function {
            local: execution::FunctionListLocalId(local.0),
            value: function_list_expr(value, context),
        },
    }
}

impl LowerListItem for module::IntListItem {
    type Execution = execution::IntListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::IntListItem::new(context.int_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::IntExpr {
        int_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::IntListLocalId {
        execution::IntListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::IntListFunctionId {
        execution::IntListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::StringListItem {
    type Execution = execution::StringListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::StringListItem::new(context.string_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::StringExpr {
        string_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::StringListLocalId {
        execution::StringListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::StringListFunctionId {
        execution::StringListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::BitArrayListItem {
    type Execution = execution::BitArrayListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::BitArrayListItem::new(context.bit_array_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::BitArrayExpr {
        bit_array_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::BitArrayListLocalId {
        execution::BitArrayListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::BitArrayListFunctionId {
        execution::BitArrayListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::UtfCodepointListItem {
    type Execution = execution::UtfCodepointListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::UtfCodepointListItem::new(context.utf_codepoint_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::UtfCodepointExpr {
        utf_codepoint_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::UtfCodepointListLocalId {
        execution::UtfCodepointListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::UtfCodepointListFunctionId {
        execution::UtfCodepointListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::CustomListItem {
    type Execution = execution::CustomListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::CustomListItem::new(context.custom_list_type(self.into_item_type()))
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::CustomExpr {
        custom_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::CustomListLocalId {
        execution::CustomListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::CustomListFunctionId {
        execution::CustomListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::FloatListItem {
    type Execution = execution::FloatListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::FloatListItem::new(context.float_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::FloatExpr {
        float_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::FloatListLocalId {
        execution::FloatListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::FloatListFunctionId {
        execution::FloatListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::BoolListItem {
    type Execution = execution::BoolListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::BoolListItem::new(context.bool_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::BoolExpr {
        bool_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::BoolListLocalId {
        execution::BoolListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::BoolListFunctionId {
        execution::BoolListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::NilListItem {
    type Execution = execution::NilListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::NilListItem::new(context.nil_list_type())
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::NilExpr {
        super::nil_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::NilListLocalId {
        execution::NilListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::NilListFunctionId {
        execution::NilListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::TupleListItem {
    type Execution = execution::TupleListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::TupleListItem::new(context.tuple_list_type(self.into_item_type()))
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::TupleExpr {
        tuple_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::TupleListLocalId {
        execution::TupleListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::TupleListFunctionId {
        execution::TupleListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::ListListItem {
    type Execution = execution::ListListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::ListListItem::new(context.list_list_type(*self.into_item_type()))
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::ListExpr {
        list_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::ListListLocalId {
        execution::ListListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::ListListFunctionId {
        execution::ListListFunctionId::new(function.0, item.type_id())
    }
}

impl LowerListItem for module::FunctionListItem {
    type Execution = execution::FunctionListItem;

    fn lower_item(self, context: &mut LoweringContext) -> Self::Execution {
        execution::FunctionListItem::new(context.function_list_type(self.into_item_type()))
    }

    fn lower_element(
        element: Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> execution::FunctionExpr {
        function_expr(element, context)
    }

    fn lower_local(
        local: Self::Local,
        _context: &mut LoweringContext,
    ) -> execution::FunctionListLocalId {
        execution::FunctionListLocalId(local.0)
    }

    fn lower_function(
        function: Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> execution::FunctionListFunctionId {
        execution::FunctionListFunctionId::new(function.0, item.type_id())
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
