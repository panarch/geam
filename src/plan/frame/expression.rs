use super::FrameLayout;
use crate::plan::{
    BoolExpr, BoolExprKind, Expr, ExprKind, FloatExpr, FloatExprKind, IntExpr, IntExprKind,
    ListElements, ListExpr, ListItem, NilExpr, NilExprKind, PanicExpr, StringExpr, StringExprKind,
    TupleExpr, TupleExprKind, TypedListExpr, TypedListExprKind,
};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_expr(&mut self, expression: &Expr) {
        match expression.kind() {
            ExprKind::Int(expression) => self.include_int_expr(expression),
            ExprKind::String(expression) => self.include_string_expr(expression),
            ExprKind::Float(expression) => self.include_float_expr(expression),
            ExprKind::Bool(expression) => self.include_bool_expr(expression),
            ExprKind::Nil(expression) => self.include_nil_expr(expression),
            ExprKind::Tuple(expression) => self.include_tuple_expr(expression),
            ExprKind::List(expression) => self.include_list_expr(expression),
            ExprKind::Function(expression) => self.include_function_expr(expression),
        }
    }

    pub(in crate::plan::frame) fn include_panic_expr(&mut self, expression: &PanicExpr) {
        if let Some(message) = expression.message() {
            self.include_string_expr(message);
        }
    }

    pub(in crate::plan::frame) fn include_int_expr(&mut self, expression: &IntExpr) {
        match expression.kind() {
            IntExprKind::Value(_) => {}
            IntExprKind::Panic(panic) => self.include_panic_expr(panic),
            IntExprKind::LocalGet { local, .. } => self.include_int(*local),
            IntExprKind::Call { args, .. } => self.include_call_args(args),
            IntExprKind::FunctionCall { function, args } => {
                self.include_int_function_expr(function);
                self.include_call_args(args);
            }
            IntExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            IntExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            IntExprKind::Add { left, right }
            | IntExprKind::Sub { left, right }
            | IntExprKind::Mult { left, right }
            | IntExprKind::Div { left, right }
            | IntExprKind::Remainder { left, right } => {
                self.include_int_expr(left);
                self.include_int_expr(right);
            }
            IntExprKind::Negate(value) => self.include_int_expr(value),
            IntExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_expr(true_);
                self.include_int_expr(false_);
            }
            IntExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_expr(branch);
                }
                self.include_int_expr(fallback);
            }
            IntExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_expr(&mut self, expression: &StringExpr) {
        match expression.kind() {
            StringExprKind::Value(_) => {}
            StringExprKind::Panic(panic) => self.include_panic_expr(panic),
            StringExprKind::LocalGet { local, .. } => self.include_string(*local),
            StringExprKind::Call { args, .. } => self.include_call_args(args),
            StringExprKind::FunctionCall { function, args } => {
                self.include_string_function_expr(function);
                self.include_call_args(args);
            }
            StringExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            StringExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            StringExprKind::Concatenate { left, right } => {
                self.include_string_expr(left);
                self.include_string_expr(right);
            }
            StringExprKind::DropPrefix { value, .. } => self.include_string_expr(value),
            StringExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_expr(true_);
                self.include_string_expr(false_);
            }
            StringExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_expr(branch);
                }
                self.include_string_expr(fallback);
            }
            StringExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_expr(&mut self, expression: &BoolExpr) {
        match expression.kind() {
            BoolExprKind::Value(_) => {}
            BoolExprKind::Panic(panic) => self.include_panic_expr(panic),
            BoolExprKind::LocalGet { local, .. } => self.include_bool(*local),
            BoolExprKind::Call { args, .. } => self.include_call_args(args),
            BoolExprKind::FunctionCall { function, args } => {
                self.include_bool_function_expr(function);
                self.include_call_args(args);
            }
            BoolExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            BoolExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            BoolExprKind::Not(value) => self.include_bool_expr(value),
            BoolExprKind::LtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::LtEqInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::GtEqInt { left, right } => self.include_int_binary_expr(left, right),
            BoolExprKind::LtFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::LtEqFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::GtFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::GtEqFloat { left, right } => self.include_float_binary_expr(left, right),
            BoolExprKind::Equal { left, right } => self.include_binary_expr(left, right),
            BoolExprKind::NotEqual { left, right } => self.include_binary_expr(left, right),
            BoolExprKind::StringStartsWith { value, .. } => self.include_string_expr(value),
            BoolExprKind::ListLengthEquals { value, .. }
            | BoolExprKind::ListLengthAtLeast { value, .. } => self.include_list_expr(value),
            BoolExprKind::And { left, right } => self.include_bool_binary_expr(left, right),
            BoolExprKind::Or { left, right } => self.include_bool_binary_expr(left, right),
            BoolExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_expr(true_);
                self.include_bool_expr(false_);
            }
            BoolExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_expr(branch);
                }
                self.include_bool_expr(fallback);
            }
            BoolExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_expr(return_);
            }
        }
    }

    fn include_int_binary_expr(&mut self, left: &IntExpr, right: &IntExpr) {
        self.include_int_expr(left);
        self.include_int_expr(right);
    }

    fn include_float_binary_expr(&mut self, left: &FloatExpr, right: &FloatExpr) {
        self.include_float_expr(left);
        self.include_float_expr(right);
    }

    fn include_binary_expr(&mut self, left: &Expr, right: &Expr) {
        self.include_expr(left);
        self.include_expr(right);
    }

    fn include_bool_binary_expr(&mut self, left: &BoolExpr, right: &BoolExpr) {
        self.include_bool_expr(left);
        self.include_bool_expr(right);
    }

    pub(in crate::plan::frame) fn include_nil_expr(&mut self, expression: &NilExpr) {
        match expression.kind() {
            NilExprKind::Value => {}
            NilExprKind::Panic(panic) => self.include_panic_expr(panic),
            NilExprKind::LocalGet { local, .. } => self.include_nil(*local),
            NilExprKind::Call { args, .. } => self.include_call_args(args),
            NilExprKind::FunctionCall { function, args } => {
                self.include_nil_function_expr(function);
                self.include_call_args(args);
            }
            NilExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            NilExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            NilExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_expr(true_);
                self.include_nil_expr(false_);
            }
            NilExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_expr(branch);
                }
                self.include_nil_expr(fallback);
            }
            NilExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_float_expr(&mut self, expression: &FloatExpr) {
        match expression.kind() {
            FloatExprKind::Value(_) => {}
            FloatExprKind::Panic(panic) => self.include_panic_expr(panic),
            FloatExprKind::LocalGet { local, .. } => self.include_float(*local),
            FloatExprKind::Call { args, .. } => self.include_call_args(args),
            FloatExprKind::FunctionCall { function, args } => {
                self.include_float_function_expr(function);
                self.include_call_args(args);
            }
            FloatExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            FloatExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            FloatExprKind::Add { left, right }
            | FloatExprKind::Sub { left, right }
            | FloatExprKind::Mult { left, right }
            | FloatExprKind::Div { left, right } => {
                self.include_float_expr(left);
                self.include_float_expr(right);
            }
            FloatExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_float_expr(true_);
                self.include_float_expr(false_);
            }
            FloatExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_expr(branch);
                }
                self.include_float_expr(fallback);
            }
            FloatExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_float_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_tuple_expr(&mut self, expression: &TupleExpr) {
        match expression.kind() {
            TupleExprKind::Value(elements) => {
                for element in elements {
                    self.include_expr(element);
                }
            }
            TupleExprKind::Panic(panic) => self.include_panic_expr(panic),
            TupleExprKind::LocalGet { local, .. } => self.include_tuple(*local),
            TupleExprKind::Call { args, .. } => self.include_call_args(args),
            TupleExprKind::FunctionCall { function, args } => {
                self.include_tuple_function_expr(function);
                self.include_call_args(args);
            }
            TupleExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            TupleExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            TupleExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_tuple_expr(true_);
                self.include_tuple_expr(false_);
            }
            TupleExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_expr(branch);
                }
                self.include_tuple_expr(fallback);
            }
            TupleExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_tuple_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_list_expr(&mut self, expression: &ListExpr) {
        match expression {
            ListExpr::Int(expression) => self.include_typed_list_expr(expression),
            ListExpr::String(expression) => self.include_typed_list_expr(expression),
            ListExpr::Float(expression) => self.include_typed_list_expr(expression),
            ListExpr::Bool(expression) => self.include_typed_list_expr(expression),
            ListExpr::Nil(expression) => self.include_typed_list_expr(expression),
            ListExpr::Tuple(expression) => self.include_typed_list_expr(expression),
            ListExpr::List(expression) => self.include_typed_list_expr(expression),
            ListExpr::Function(expression) => self.include_typed_list_expr(expression),
        }
    }

    pub(in crate::plan::frame) fn include_typed_list_expr<Item: ListItem>(
        &mut self,
        expression: &TypedListExpr<Item>,
    ) {
        match expression.kind() {
            TypedListExprKind::Value(elements) => {
                self.include_typed_list_elements(expression.item(), elements)
            }
            TypedListExprKind::Panic(panic) => self.include_panic_expr(panic),
            TypedListExprKind::Spread { elements, tail } => {
                self.include_typed_list_elements(expression.item(), elements);
                self.include_typed_list_expr(tail);
            }
            TypedListExprKind::LocalGet { local, .. } => {
                self.include_list(expression.item().local_to_facade(local.clone()));
            }
            TypedListExprKind::Call { args, .. } => self.include_call_args(args),
            TypedListExprKind::FunctionCall { function, args } => {
                self.include_list_function_expr(function);
                self.include_call_args(args);
            }
            TypedListExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            TypedListExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            TypedListExprKind::DropFirst { list, .. } => self.include_typed_list_expr(list),
            TypedListExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_typed_list_expr(true_);
                self.include_typed_list_expr(false_);
            }
            TypedListExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_expr(branch);
                }
                self.include_typed_list_expr(fallback);
            }
            TypedListExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_expr(branch);
                }
                self.include_typed_list_expr(fallback);
            }
            TypedListExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_typed_list_expr(branch);
                }
                self.include_typed_list_expr(fallback);
            }
            TypedListExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_typed_list_expr(return_);
            }
        }
    }

    fn include_typed_list_elements<Item: ListItem>(
        &mut self,
        item: &Item,
        elements: &[Item::ElementExpr],
    ) {
        let elements = Item::elements_to_facade(item.clone(), elements.to_vec());
        self.include_list_elements(&elements);
    }

    fn include_list_elements(&mut self, elements: &ListElements) {
        match elements {
            ListElements::Int(values) => {
                for value in values {
                    self.include_int_expr(value);
                }
            }
            ListElements::String(values) => {
                for value in values {
                    self.include_string_expr(value);
                }
            }
            ListElements::Float(values) => {
                for value in values {
                    self.include_float_expr(value);
                }
            }
            ListElements::Bool(values) => {
                for value in values {
                    self.include_bool_expr(value);
                }
            }
            ListElements::Nil(values) => {
                for value in values {
                    self.include_nil_expr(value);
                }
            }
            ListElements::Tuple { values, .. } => {
                for value in values {
                    self.include_tuple_expr(value);
                }
            }
            ListElements::List { values, .. } => {
                for value in values {
                    self.include_list_expr(value);
                }
            }
            ListElements::Function { values, .. } => {
                for value in values {
                    self.include_function_expr(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolListCaseBranches, BoolListLocalId, BoolLocalId, CallArg, Expr, FloatExpr,
        FloatFunctionId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionType,
        IntExpr, IntFunctionId, IntListLocalId, IntLocalId, ListExpr, ListFunctionExpr,
        ListFunctionId, ListListLocalId, ListLocal, NilExpr, NilListLocalId, NilLocalId, PanicExpr,
        PanicSite, ReturnExpr, Step, StringExpr, StringListLocalId, StringLocalId, TupleExpr,
        TupleFunctionExpr, TupleFunctionId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        ValueType,
    };

    #[test]
    fn frame_layout_includes_list_projection_expression_dependencies() {
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::list_index(
                ListExpr::local_get(ListLocal::int(IntListLocalId(0)), "int_list".into())
                    .into_int()
                    .expect("int list local should build an IntListExpr"),
                0,
            ))),
            Step::evaluate(Expr::string(StringExpr::list_index(
                ListExpr::local_get(
                    ListLocal::string(StringListLocalId(0)),
                    "string_list".into(),
                )
                .into_string()
                .expect("string list local should build a StringListExpr"),
                0,
            ))),
            Step::evaluate(Expr::float(FloatExpr::list_index(
                ListExpr::local_get(ListLocal::float(FloatListLocalId(0)), "float_list".into())
                    .into_float()
                    .expect("float list local should build a FloatListExpr"),
                0,
            ))),
            Step::evaluate(Expr::bool(BoolExpr::list_index(
                ListExpr::local_get(ListLocal::bool(BoolListLocalId(0)), "bool_list".into())
                    .into_bool()
                    .expect("bool list local should build a BoolListExpr"),
                0,
            ))),
            Step::evaluate(Expr::nil(NilExpr::list_index(
                ListExpr::local_get(ListLocal::nil(NilListLocalId(0)), "nil_list".into())
                    .into_nil()
                    .expect("nil list local should build a NilListExpr"),
                0,
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::list_index(
                ListExpr::local_get(
                    ListLocal::tuple(TupleListLocalId(0), tuple_type()),
                    "tuple_list".into(),
                )
                .into_tuple()
                .expect("tuple list local should build a TupleListExpr"),
                0,
                tuple_type(),
            ))),
            Step::evaluate(Expr::list(ListExpr::list_index(
                ListExpr::local_get(
                    ListLocal::list(ListListLocalId(0), ValueType::Int),
                    "nested_list".into(),
                )
                .into_list()
                .expect("nested list local should build a ListListExpr"),
                0,
            ))),
            Step::evaluate(Expr::list(ListExpr::drop_first(
                ListExpr::local_get(ListLocal::int(IntListLocalId(1)), "drop_list".into()),
                1,
            ))),
            Step::evaluate(Expr::bool(BoolExpr::list_length_equals(
                ListExpr::local_get(ListLocal::int(IntListLocalId(2)), "exact_list".into()),
                2,
            ))),
            Step::evaluate(Expr::bool(BoolExpr::list_length_at_least(
                ListExpr::local_get(ListLocal::int(IntListLocalId(3)), "minimum_list".into()),
                1,
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.int_lists(), 4);
        assert_eq!(layout.string_lists(), 1);
        assert_eq!(layout.float_lists(), 1);
        assert_eq!(layout.bool_lists(), 1);
        assert_eq!(layout.nil_lists(), 1);
        assert_eq!(layout.tuple_lists(), &[tuple_type()]);
        assert_eq!(layout.list_lists(), &[ValueType::Int]);
    }

    #[test]
    fn frame_layout_includes_bool_operator_families() {
        let steps = vec![Step::evaluate(Expr::bool(BoolExpr::and(
            BoolExpr::and(
                BoolExpr::and(
                    BoolExpr::lte_int(
                        IntExpr::local_get(IntLocalId(1), "lte_left".into()),
                        IntExpr::local_get(IntLocalId(2), "lte_right".into()),
                    ),
                    BoolExpr::gt_int(
                        IntExpr::local_get(IntLocalId(3), "gt_left".into()),
                        IntExpr::local_get(IntLocalId(4), "gt_right".into()),
                    ),
                ),
                BoolExpr::and(
                    BoolExpr::gte_int(
                        IntExpr::local_get(IntLocalId(5), "gte_left".into()),
                        IntExpr::local_get(IntLocalId(6), "gte_right".into()),
                    ),
                    BoolExpr::not_equal(
                        Expr::int(IntExpr::local_get(IntLocalId(7), "not_equal_left".into())),
                        Expr::int(IntExpr::local_get(IntLocalId(8), "not_equal_right".into())),
                    ),
                ),
            ),
            BoolExpr::and(
                BoolExpr::lte_float(
                    FloatExpr::local_get(FloatLocalId(0), "float_lte_left".into()),
                    FloatExpr::local_get(FloatLocalId(1), "float_lte_right".into()),
                ),
                BoolExpr::gt_float(
                    FloatExpr::local_get(FloatLocalId(2), "float_gt_left".into()),
                    FloatExpr::local_get(FloatLocalId(3), "float_gt_right".into()),
                ),
            ),
        )))];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 9);
        assert_eq!(layout.floats(), 4);
    }

    #[test]
    fn frame_layout_includes_string_prefix_expression_dependencies() {
        let steps = vec![
            Step::evaluate(Expr::bool(BoolExpr::string_starts_with(
                StringExpr::local_get(StringLocalId(0), "prefix_subject".into()),
                "pre".into(),
            ))),
            Step::evaluate(Expr::string(StringExpr::drop_prefix(
                StringExpr::local_get(StringLocalId(1), "suffix_subject".into()),
                "pre".into(),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.strings(), 2);
    }

    #[test]
    fn frame_layout_includes_panic_message_dependencies() {
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::panic(panic_message(0)))),
            Step::evaluate(Expr::string(StringExpr::panic(panic_message(1)))),
            Step::evaluate(Expr::float(FloatExpr::panic(panic_message(2)))),
            Step::evaluate(Expr::bool(BoolExpr::panic(panic_message(3)))),
            Step::evaluate(Expr::nil(NilExpr::panic(panic_message(4)))),
            Step::evaluate(Expr::tuple(TupleExpr::panic(
                panic_message(5),
                vec![ValueType::Int],
            ))),
            Step::evaluate(Expr::list(ListExpr::panic(
                panic_message(6),
                ValueType::Int,
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 0);
        assert_eq!(layout.floats(), 0);
        assert_eq!(layout.strings(), 7);
        assert_eq!(layout.bools(), 0);
        assert_eq!(layout.nils(), 0);
        assert_eq!(layout.tuples(), 0);
        assert_eq!(layout.int_lists(), 0);
        assert_eq!(layout.string_lists(), 0);
        assert_eq!(layout.float_lists(), 0);
        assert_eq!(layout.bool_lists(), 0);
        assert_eq!(layout.nil_lists(), 0);
        assert_eq!(layout.tuple_lists(), &[] as &[Vec<ValueType>]);
        assert_eq!(layout.list_lists(), &[] as &[ValueType]);
    }

    #[test]
    fn frame_layout_includes_primitive_case_and_block_families() {
        let steps = vec![
            Step::evaluate(Expr::int(IntExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(6), "int_bool_case_subject".into()),
                IntExpr::local_get(IntLocalId(7), "int_bool_true".into()),
                IntExpr::local_get(IntLocalId(8), "int_bool_false".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::int_case(
                IntExpr::local_get(IntLocalId(9), "int_case_subject".into()),
                vec![
                    (
                        0.into(),
                        IntExpr::local_get(IntLocalId(10), "int_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        IntExpr::local_get(IntLocalId(11), "int_branch_one".into()),
                    ),
                ],
                IntExpr::local_get(IntLocalId(12), "int_fallback".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::string_case(
                StringExpr::local_get(StringLocalId(6), "int_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        IntExpr::local_get(IntLocalId(15), "int_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        IntExpr::local_get(IntLocalId(16), "int_string_branch_two".into()),
                    ),
                ],
                IntExpr::local_get(IntLocalId(17), "int_string_fallback".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::float_case(
                FloatExpr::local_get(FloatLocalId(18), "int_float_case_subject".into()),
                vec![(
                    1.0,
                    IntExpr::local_get(IntLocalId(18), "int_float_branch".into()),
                )],
                IntExpr::local_get(IntLocalId(19), "int_float_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(0), "string_case_subject".into()),
                StringExpr::local_get(StringLocalId(1), "string_true".into()),
                StringExpr::local_get(StringLocalId(2), "string_false".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::int_case(
                IntExpr::local_get(IntLocalId(13), "string_case_subject".into()),
                vec![
                    (
                        0.into(),
                        StringExpr::local_get(StringLocalId(3), "string_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        StringExpr::local_get(StringLocalId(4), "string_branch_one".into()),
                    ),
                ],
                StringExpr::local_get(StringLocalId(5), "string_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::string_case(
                StringExpr::local_get(StringLocalId(7), "string_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        StringExpr::local_get(StringLocalId(8), "string_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        StringExpr::local_get(StringLocalId(9), "string_string_branch_two".into()),
                    ),
                ],
                StringExpr::local_get(StringLocalId(10), "string_string_fallback".into()),
            ))),
            Step::evaluate(Expr::string(StringExpr::float_case(
                FloatExpr::local_get(FloatLocalId(19), "string_float_case_subject".into()),
                vec![(
                    1.0,
                    StringExpr::local_get(StringLocalId(13), "string_float_branch".into()),
                )],
                StringExpr::local_get(StringLocalId(14), "string_float_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::int_case(
                IntExpr::local_get(IntLocalId(3), "bool_case_subject".into()),
                vec![(
                    0.into(),
                    BoolExpr::local_get(BoolLocalId(4), "bool_branch".into()),
                )],
                BoolExpr::local_get(BoolLocalId(5), "bool_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::string_case(
                StringExpr::local_get(StringLocalId(11), "bool_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        BoolExpr::local_get(BoolLocalId(8), "bool_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        BoolExpr::local_get(BoolLocalId(9), "bool_string_branch_two".into()),
                    ),
                ],
                BoolExpr::local_get(BoolLocalId(10), "bool_string_fallback".into()),
            ))),
            Step::evaluate(Expr::bool(BoolExpr::float_case(
                FloatExpr::local_get(FloatLocalId(20), "bool_float_case_subject".into()),
                vec![(
                    1.0,
                    BoolExpr::local_get(BoolLocalId(11), "bool_float_branch".into()),
                )],
                BoolExpr::local_get(BoolLocalId(12), "bool_float_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(7), "nil_bool_case_subject".into()),
                NilExpr::local_get(NilLocalId(1), "nil_bool_true".into()),
                NilExpr::local_get(NilLocalId(2), "nil_bool_false".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::int_case(
                IntExpr::local_get(IntLocalId(14), "nil_case_subject".into()),
                vec![
                    (
                        0.into(),
                        NilExpr::local_get(NilLocalId(3), "nil_branch_zero".into()),
                    ),
                    (
                        1.into(),
                        NilExpr::local_get(NilLocalId(4), "nil_branch_one".into()),
                    ),
                ],
                NilExpr::local_get(NilLocalId(5), "nil_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::string_case(
                StringExpr::local_get(StringLocalId(12), "nil_string_case_subject".into()),
                vec![
                    (
                        "one".into(),
                        NilExpr::local_get(NilLocalId(6), "nil_string_branch_one".into()),
                    ),
                    (
                        "two".into(),
                        NilExpr::local_get(NilLocalId(7), "nil_string_branch_two".into()),
                    ),
                ],
                NilExpr::local_get(NilLocalId(8), "nil_string_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::float_case(
                FloatExpr::local_get(FloatLocalId(21), "nil_float_case_subject".into()),
                vec![(
                    1.0,
                    NilExpr::local_get(NilLocalId(9), "nil_float_branch".into()),
                )],
                NilExpr::local_get(NilLocalId(10), "nil_float_fallback".into()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::list_index(
                ListExpr::local_get(
                    ListLocal::nil(NilListLocalId(0)),
                    "nil_list_index_subject".into(),
                )
                .into_nil()
                .expect("nil list local should build a NilListExpr"),
                0,
            ))),
            Step::evaluate(Expr::float(FloatExpr::add(
                FloatExpr::local_get(FloatLocalId(22), "float_add_left".into()),
                FloatExpr::sub(
                    FloatExpr::local_get(FloatLocalId(23), "float_sub_left".into()),
                    FloatExpr::mult(
                        FloatExpr::local_get(FloatLocalId(24), "float_mult_left".into()),
                        FloatExpr::div(
                            FloatExpr::local_get(FloatLocalId(25), "float_div_left".into()),
                            FloatExpr::local_get(FloatLocalId(26), "float_div_right".into()),
                        ),
                    ),
                ),
            ))),
            Step::evaluate(Expr::float(FloatExpr::call(
                FloatFunctionId(0),
                vec![CallArg::float(
                    FloatLocalId(0),
                    FloatExpr::local_get(FloatLocalId(27), "float_call_arg".into()),
                )],
            ))),
            Step::evaluate(Expr::float(FloatExpr::function_call(
                crate::plan::FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(0),
                    "float_function".into(),
                    super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                ),
                vec![CallArg::float(
                    FloatLocalId(1),
                    FloatExpr::local_get(FloatLocalId(28), "float_function_call_arg".into()),
                )],
            ))),
            Step::evaluate(Expr::float(FloatExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(13), "float_bool_case_subject".into()),
                FloatExpr::local_get(FloatLocalId(29), "float_bool_true".into()),
                FloatExpr::local_get(FloatLocalId(30), "float_bool_false".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::int_case(
                IntExpr::local_get(IntLocalId(20), "float_int_case_subject".into()),
                vec![(
                    1.into(),
                    FloatExpr::local_get(FloatLocalId(31), "float_int_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(32), "float_int_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::string_case(
                StringExpr::local_get(StringLocalId(15), "float_string_case_subject".into()),
                vec![(
                    "one".into(),
                    FloatExpr::local_get(FloatLocalId(33), "float_string_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(34), "float_string_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::float_case(
                FloatExpr::local_get(FloatLocalId(35), "float_case_subject".into()),
                vec![(
                    1.0,
                    FloatExpr::local_get(FloatLocalId(36), "float_branch".into()),
                )],
                FloatExpr::local_get(FloatLocalId(37), "float_fallback".into()),
            ))),
            Step::evaluate(Expr::float(FloatExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(21),
                    "float_block_step".into(),
                )))],
                FloatExpr::local_get(FloatLocalId(38), "float_block_return".into()),
            ))),
            Step::evaluate(Expr::int(IntExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::int(IntExpr::local_get(
                        IntLocalId(25),
                        "int_tuple_index_value".into(),
                    ))],
                    vec![ValueType::Int],
                ),
                0,
            ))),
            Step::evaluate(Expr::string(StringExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::string(StringExpr::local_get(
                        StringLocalId(18),
                        "string_tuple_index_value".into(),
                    ))],
                    vec![ValueType::String],
                ),
                0,
            ))),
            Step::evaluate(Expr::float(FloatExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::float(FloatExpr::local_get(
                        FloatLocalId(40),
                        "float_tuple_index_value".into(),
                    ))],
                    vec![ValueType::Float],
                ),
                0,
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::value(
                vec![
                    Expr::int(IntExpr::local_get(IntLocalId(22), "tuple_int".into())),
                    Expr::string(StringExpr::local_get(
                        StringLocalId(16),
                        "tuple_string".into(),
                    )),
                ],
                vec![ValueType::Int, ValueType::String],
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::local_get(
                TupleLocalId(0),
                "tuple_local".into(),
                vec![ValueType::Int],
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::call(
                TupleFunctionId(0),
                vec![CallArg::tuple(
                    TupleLocalId(1),
                    TupleExpr::local_get(TupleLocalId(2), "tuple_call_arg".into(), tuple_type()),
                )],
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::function_call(
                TupleFunctionExpr::local_get(
                    TupleFunctionLocalId(0),
                    "tuple_function".into(),
                    tuple_function_type(),
                ),
                vec![CallArg::tuple(
                    TupleLocalId(3),
                    TupleExpr::local_get(
                        TupleLocalId(4),
                        "tuple_function_call_arg".into(),
                        tuple_type(),
                    ),
                )],
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::tuple_index(
                TupleExpr::local_get(
                    TupleLocalId(5),
                    "tuple_index_subject".into(),
                    vec![ValueType::Tuple(tuple_type())],
                ),
                0,
                tuple_type(),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(14), "tuple_bool_case_subject".into()),
                TupleExpr::local_get(TupleLocalId(6), "tuple_bool_true".into(), tuple_type()),
                TupleExpr::local_get(TupleLocalId(7), "tuple_bool_false".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::int_case(
                IntExpr::local_get(IntLocalId(23), "tuple_int_case_subject".into()),
                vec![(
                    1.into(),
                    TupleExpr::local_get(TupleLocalId(8), "tuple_int_branch".into(), tuple_type()),
                )],
                TupleExpr::local_get(TupleLocalId(9), "tuple_int_fallback".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::string_case(
                StringExpr::local_get(StringLocalId(17), "tuple_string_case_subject".into()),
                vec![(
                    "one".into(),
                    TupleExpr::local_get(
                        TupleLocalId(10),
                        "tuple_string_branch".into(),
                        tuple_type(),
                    ),
                )],
                TupleExpr::local_get(
                    TupleLocalId(11),
                    "tuple_string_fallback".into(),
                    tuple_type(),
                ),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::float_case(
                FloatExpr::local_get(FloatLocalId(39), "tuple_float_case_subject".into()),
                vec![(
                    1.0,
                    TupleExpr::local_get(
                        TupleLocalId(12),
                        "tuple_float_branch".into(),
                        tuple_type(),
                    ),
                )],
                TupleExpr::local_get(
                    TupleLocalId(13),
                    "tuple_float_fallback".into(),
                    tuple_type(),
                ),
            ))),
            Step::evaluate(Expr::tuple(TupleExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(24),
                    "tuple_block_step".into(),
                )))],
                TupleExpr::local_get(TupleLocalId(14), "tuple_block_return".into(), tuple_type()),
            ))),
            Step::evaluate(Expr::nil(NilExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(6),
                    "nil_block_step".into(),
                )))],
                NilExpr::local_get(NilLocalId(0), "nil_return".into()),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 26);
        assert_eq!(layout.floats(), 41);
        assert_eq!(layout.strings(), 19);
        assert_eq!(layout.bools(), 15);
        assert_eq!(layout.nils(), 11);
        assert_eq!(layout.tuples(), 15);
        assert_eq!(layout.nil_lists(), 1);
        assert_eq!(layout.float_functions(), 1);
        assert_eq!(layout.tuple_functions(), 1);
    }

    #[test]
    fn frame_layout_includes_list_expression_families() {
        let steps = vec![
            Step::evaluate(Expr::list(ListExpr::value(
                vec![Expr::int(IntExpr::local_get(
                    IntLocalId(0),
                    "value_element".into(),
                ))],
                list_type(),
            ))),
            Step::evaluate(Expr::list(ListExpr::spread(
                vec![Expr::int(IntExpr::local_get(
                    IntLocalId(3),
                    "spread_element".into(),
                ))],
                ListExpr::local_get(ListLocal::int(IntListLocalId(13)), "spread_tail".into()),
                list_type(),
            ))),
            Step::evaluate(Expr::list(ListExpr::local_get(
                ListLocal::int(IntListLocalId(0)),
                "local".into(),
            ))),
            Step::evaluate(Expr::list(ListExpr::call(
                ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                vec![CallArg::list(
                    ListLocal::int(IntListLocalId(0)),
                    ListExpr::local_get(ListLocal::int(IntListLocalId(1)), "call_arg".into()),
                )],
            ))),
            Step::evaluate(Expr::list(ListExpr::function_call(
                ListFunctionExpr::local_get(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        crate::plan::FunctionType::new(
                            Vec::new(),
                            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                        ),
                        crate::plan::ValueType::Int,
                    ),
                    "callee".into(),
                ),
                vec![CallArg::list(
                    ListLocal::int(IntListLocalId(1)),
                    ListExpr::local_get(
                        ListLocal::int(IntListLocalId(2)),
                        "function_call_arg".into(),
                    ),
                )],
            ))),
            Step::evaluate(Expr::list(ListExpr::tuple_index(
                TupleExpr::value(
                    vec![Expr::list(ListExpr::local_get(
                        ListLocal::int(IntListLocalId(3)),
                        "tuple_element".into(),
                    ))],
                    vec![ValueType::List(Box::new(list_type()))],
                ),
                0,
                list_type(),
            ))),
            Step::evaluate(Expr::list(ListExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(0), "bool_subject".into()),
                BoolListCaseBranches::Int {
                    true_: ListExpr::local_get(
                        ListLocal::int(IntListLocalId(4)),
                        "bool_true".into(),
                    )
                    .into_int()
                    .expect("bool true branch should be List(Int)"),
                    false_: ListExpr::local_get(
                        ListLocal::int(IntListLocalId(5)),
                        "bool_false".into(),
                    )
                    .into_int()
                    .expect("bool false branch should be List(Int)"),
                },
            ))),
            Step::evaluate(Expr::list(ListExpr::int_case(
                IntExpr::local_get(IntLocalId(1), "int_subject".into()),
                vec![(
                    1.into(),
                    ListExpr::local_get(ListLocal::int(IntListLocalId(6)), "int_branch".into()),
                )],
                ListExpr::local_get(ListLocal::int(IntListLocalId(7)), "int_fallback".into()),
            ))),
            Step::evaluate(Expr::list(ListExpr::string_case(
                StringExpr::local_get(StringLocalId(0), "string_subject".into()),
                vec![(
                    "hit".into(),
                    ListExpr::local_get(ListLocal::int(IntListLocalId(8)), "string_branch".into()),
                )],
                ListExpr::local_get(ListLocal::int(IntListLocalId(9)), "string_fallback".into()),
            ))),
            Step::evaluate(Expr::list(ListExpr::float_case(
                FloatExpr::local_get(FloatLocalId(0), "float_subject".into()),
                vec![(
                    1.0,
                    ListExpr::local_get(ListLocal::int(IntListLocalId(10)), "float_branch".into()),
                )],
                ListExpr::local_get(ListLocal::int(IntListLocalId(11)), "float_fallback".into()),
            ))),
            Step::evaluate(Expr::list(ListExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(2),
                    "block_step".into(),
                )))],
                ListExpr::local_get(ListLocal::int(IntListLocalId(12)), "block_return".into()),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 4);
        assert_eq!(layout.floats(), 1);
        assert_eq!(layout.strings(), 1);
        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.int_lists(), 14);
        assert_eq!(layout.list_functions().len(), 1);
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int]
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Tuple(tuple_type())],
            ValueType::Tuple(tuple_type()),
        )
    }

    fn list_type() -> ValueType {
        ValueType::Int
    }

    fn panic_message(index: usize) -> PanicExpr {
        PanicExpr::panic_at(
            Some(StringExpr::local_get(
                StringLocalId(index),
                format!("panic_message_{index}").into(),
            )),
            PanicSite::unknown(),
        )
    }
}
