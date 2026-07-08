use super::FrameLayout;
use crate::plan::{
    BoolFunctionExpr, BoolFunctionExprKind, FloatFunctionExpr, FloatFunctionExprKind, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, FunctionFunctionExprKind, IntFunctionExpr,
    IntFunctionExprKind, ListFunctionExpr, ListFunctionExprKind, NilFunctionExpr,
    NilFunctionExprKind, StringFunctionExpr, StringFunctionExprKind, TupleFunctionExpr,
    TupleFunctionExprKind,
};

impl FrameLayout {
    pub(in crate::plan::frame) fn include_function_expr(&mut self, expression: &FunctionExpr) {
        match expression.kind() {
            FunctionExprKind::Int(expression) => self.include_int_function_expr(expression),
            FunctionExprKind::String(expression) => self.include_string_function_expr(expression),
            FunctionExprKind::Float(expression) => self.include_float_function_expr(expression),
            FunctionExprKind::Bool(expression) => self.include_bool_function_expr(expression),
            FunctionExprKind::Nil(expression) => self.include_nil_function_expr(expression),
            FunctionExprKind::Tuple(expression) => self.include_tuple_function_expr(expression),
            FunctionExprKind::List(expression) => self.include_list_function_expr(expression),
            FunctionExprKind::Function(expression) => {
                self.include_function_function_expr(expression);
            }
        }
    }

    pub(in crate::plan::frame) fn include_int_function_expr(
        &mut self,
        expression: &IntFunctionExpr,
    ) {
        match expression.kind() {
            IntFunctionExprKind::Value(_) => {}
            IntFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            IntFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            IntFunctionExprKind::LocalGet { local, .. } => self.include_int_function(*local),
            IntFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            IntFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            IntFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            IntFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            IntFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_int_function_expr(true_);
                self.include_int_function_expr(false_);
            }
            IntFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_expr(branch);
                }
                self.include_int_function_expr(fallback);
            }
            IntFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_expr(branch);
                }
                self.include_int_function_expr(fallback);
            }
            IntFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_int_function_expr(branch);
                }
                self.include_int_function_expr(fallback);
            }
            IntFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_int_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_float_function_expr(
        &mut self,
        expression: &FloatFunctionExpr,
    ) {
        match expression.kind() {
            FloatFunctionExprKind::Value(_) => {}
            FloatFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            FloatFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            FloatFunctionExprKind::LocalGet { local, .. } => self.include_float_function(*local),
            FloatFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FloatFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            FloatFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            FloatFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            FloatFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_float_function_expr(true_);
                self.include_float_function_expr(false_);
            }
            FloatFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_expr(branch);
                }
                self.include_float_function_expr(fallback);
            }
            FloatFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_expr(branch);
                }
                self.include_float_function_expr(fallback);
            }
            FloatFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_float_function_expr(branch);
                }
                self.include_float_function_expr(fallback);
            }
            FloatFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_float_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_string_function_expr(
        &mut self,
        expression: &StringFunctionExpr,
    ) {
        match expression.kind() {
            StringFunctionExprKind::Value(_) => {}
            StringFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            StringFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            StringFunctionExprKind::LocalGet { local, .. } => {
                self.include_string_function(*local);
            }
            StringFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            StringFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            StringFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            StringFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            StringFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_string_function_expr(true_);
                self.include_string_function_expr(false_);
            }
            StringFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_expr(branch);
                }
                self.include_string_function_expr(fallback);
            }
            StringFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_expr(branch);
                }
                self.include_string_function_expr(fallback);
            }
            StringFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_string_function_expr(branch);
                }
                self.include_string_function_expr(fallback);
            }
            StringFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_string_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_bool_function_expr(
        &mut self,
        expression: &BoolFunctionExpr,
    ) {
        match expression.kind() {
            BoolFunctionExprKind::Value(_) => {}
            BoolFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            BoolFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            BoolFunctionExprKind::LocalGet { local, .. } => self.include_bool_function(*local),
            BoolFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            BoolFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            BoolFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            BoolFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            BoolFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_bool_function_expr(true_);
                self.include_bool_function_expr(false_);
            }
            BoolFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_expr(branch);
                }
                self.include_bool_function_expr(fallback);
            }
            BoolFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_expr(branch);
                }
                self.include_bool_function_expr(fallback);
            }
            BoolFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_bool_function_expr(branch);
                }
                self.include_bool_function_expr(fallback);
            }
            BoolFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_bool_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_nil_function_expr(
        &mut self,
        expression: &NilFunctionExpr,
    ) {
        match expression.kind() {
            NilFunctionExprKind::Value(_) => {}
            NilFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            NilFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            NilFunctionExprKind::LocalGet { local, .. } => self.include_nil_function(*local),
            NilFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            NilFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            NilFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            NilFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            NilFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_nil_function_expr(true_);
                self.include_nil_function_expr(false_);
            }
            NilFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_expr(branch);
                }
                self.include_nil_function_expr(fallback);
            }
            NilFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_expr(branch);
                }
                self.include_nil_function_expr(fallback);
            }
            NilFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_nil_function_expr(branch);
                }
                self.include_nil_function_expr(fallback);
            }
            NilFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_nil_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_function_function_expr(
        &mut self,
        expression: &FunctionFunctionExpr,
    ) {
        match expression.kind() {
            FunctionFunctionExprKind::Value(_) => {}
            FunctionFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            FunctionFunctionExprKind::Closure { captures, .. } => {
                self.include_capture_args(captures);
            }
            FunctionFunctionExprKind::LocalGet { local, .. } => {
                self.include_function_function(*local);
            }
            FunctionFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            FunctionFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            FunctionFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            FunctionFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            FunctionFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_function_function_expr(true_);
                self.include_function_function_expr(false_);
            }
            FunctionFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_expr(branch);
                }
                self.include_function_function_expr(fallback);
            }
            FunctionFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_expr(branch);
                }
                self.include_function_function_expr(fallback);
            }
            FunctionFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_function_function_expr(branch);
                }
                self.include_function_function_expr(fallback);
            }
            FunctionFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_function_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_tuple_function_expr(
        &mut self,
        expression: &TupleFunctionExpr,
    ) {
        match expression.kind() {
            TupleFunctionExprKind::Value(_) => {}
            TupleFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            TupleFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            TupleFunctionExprKind::LocalGet { local, .. } => self.include_tuple_function(*local),
            TupleFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            TupleFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            TupleFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            TupleFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            TupleFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_tuple_function_expr(true_);
                self.include_tuple_function_expr(false_);
            }
            TupleFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_expr(branch);
                }
                self.include_tuple_function_expr(fallback);
            }
            TupleFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_expr(branch);
                }
                self.include_tuple_function_expr(fallback);
            }
            TupleFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_tuple_function_expr(branch);
                }
                self.include_tuple_function_expr(fallback);
            }
            TupleFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_tuple_function_expr(return_);
            }
        }
    }

    pub(in crate::plan::frame) fn include_list_function_expr(
        &mut self,
        expression: &ListFunctionExpr,
    ) {
        match expression.kind() {
            ListFunctionExprKind::Value(_) => {}
            ListFunctionExprKind::Panic(panic) => self.include_panic_expr(panic),
            ListFunctionExprKind::Closure { captures, .. } => self.include_capture_args(captures),
            ListFunctionExprKind::LocalGet { local, .. } => {
                self.include_list_function(local.clone());
            }
            ListFunctionExprKind::Call { args, .. } => self.include_call_args(args),
            ListFunctionExprKind::FunctionCall { function, args, .. } => {
                self.include_function_function_expr(function);
                self.include_call_args(args);
            }
            ListFunctionExprKind::TupleIndex { tuple, .. } => self.include_tuple_expr(tuple),
            ListFunctionExprKind::ListIndex { list, .. } => self.include_typed_list_expr(list),
            ListFunctionExprKind::BoolCase {
                subject,
                true_,
                false_,
            } => {
                self.include_bool_expr(subject);
                self.include_list_function_expr(true_);
                self.include_list_function_expr(false_);
            }
            ListFunctionExprKind::IntCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_int_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_expr(branch);
                }
                self.include_list_function_expr(fallback);
            }
            ListFunctionExprKind::StringCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_string_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_expr(branch);
                }
                self.include_list_function_expr(fallback);
            }
            ListFunctionExprKind::FloatCase {
                subject,
                clauses,
                fallback,
            } => {
                self.include_float_expr(subject);
                for (_, branch) in clauses {
                    self.include_list_function_expr(branch);
                }
                self.include_list_function_expr(fallback);
            }
            ListFunctionExprKind::Block { steps, return_ } => {
                self.include_steps(steps);
                self.include_list_function_expr(return_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameLayout;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, CallArg, CaptureArg, Expr,
        FloatExpr, FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionLocalId, FunctionListLocalId,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntLocalId, ListExpr, ListFunctionExpr, ListFunctionFunctionId,
        ListFunctionId, ListLocal, NilFunctionExpr, NilFunctionId, NilFunctionLocalId, PanicExpr,
        PanicSite, ReturnExpr, Step, StringExpr, StringFunctionExpr, StringFunctionId,
        StringFunctionLocalId, StringLocalId, TupleExpr, TupleFunctionExpr,
        TupleFunctionFunctionId, TupleFunctionId, TupleFunctionLocalId, TupleLocalId, ValueType,
    };

    #[test]
    fn frame_layout_includes_function_expression_nested_locals() {
        let nested_block = IntFunctionExpr::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(4),
                "value".into(),
            )))],
            super::super::test_helpers::int_function_expr(),
        );
        let nested_case = IntFunctionExpr::int_case(
            IntExpr::local_get(IntLocalId(3), "subject".into()),
            vec![(1.into(), nested_block)],
            super::super::test_helpers::int_function_expr(),
        );
        let function_case = IntFunctionExpr::bool_case(
            BoolExpr::local_get(crate::plan::BoolLocalId(2), "flag".into()),
            super::super::test_helpers::int_function_expr(),
            nested_case,
        );
        let steps = vec![Step::evaluate(Expr::function(FunctionExpr::int(
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::function(FunctionExpr::int(
                    function_case,
                )))],
                super::super::test_helpers::int_function_expr(),
            ),
        )))];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 5);
        assert_eq!(layout.bools(), 3);
    }

    #[test]
    fn frame_layout_includes_function_panic_message_dependencies() {
        let int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let tuple_type = FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::int(IntFunctionExpr::panic(
                panic_message(0),
                int_type,
            )))),
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::panic(panic_message(1), string_type),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::panic(panic_message(2), float_type),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(BoolFunctionExpr::panic(
                panic_message(3),
                bool_type,
            )))),
            Step::evaluate(Expr::function(FunctionExpr::nil(NilFunctionExpr::panic(
                panic_message(4),
                nil_type,
            )))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::panic(panic_message(5), tuple_type),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(ListFunctionExpr::panic(
                panic_message(6),
                list_type,
                ValueType::Int,
            )))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::panic(panic_message(7), function_type),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 0);
        assert_eq!(layout.floats(), 0);
        assert_eq!(layout.strings(), 8);
        assert_eq!(layout.bools(), 0);
        assert_eq!(layout.nils(), 0);
        assert_eq!(layout.tuples(), 0);
        assert_eq!(layout.int_lists(), 0);
        assert_eq!(layout.function_lists(), &[] as &[FunctionType]);
        assert_eq!(layout.int_functions(), 0);
        assert_eq!(layout.float_functions(), 0);
        assert_eq!(layout.string_functions(), 0);
        assert_eq!(layout.bool_functions(), 0);
        assert_eq!(layout.nil_functions(), 0);
        assert_eq!(layout.tuple_functions(), 0);
        assert_eq!(layout.list_functions().len(), 0);
        assert_eq!(layout.function_functions(), 0);
    }

    #[test]
    fn frame_layout_includes_function_list_index_dependencies() {
        let int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let tuple_type = FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );
        let function_list = |local, name: &str, type_: FunctionType| {
            ListExpr::local_get(
                ListLocal::function(FunctionListLocalId(local), type_),
                name.into(),
            )
            .into_function()
            .expect("function list local should build a FunctionListExpr")
        };
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::int(
                IntFunctionExpr::list_index(
                    function_list(0, "int_list", int_type.clone()),
                    0,
                    int_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::list_index(
                    function_list(1, "string_list", string_type.clone()),
                    0,
                    string_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::list_index(
                    function_list(2, "float_list", float_type.clone()),
                    0,
                    float_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::list_index(
                    function_list(3, "bool_list", bool_type.clone()),
                    0,
                    bool_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::list_index(
                    function_list(4, "nil_list", nil_type.clone()),
                    0,
                    nil_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::list_index(
                    function_list(5, "tuple_list", tuple_type.clone()),
                    0,
                    tuple_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::list_index(
                    function_list(6, "list_list", list_type.clone()),
                    0,
                    list_type,
                    ValueType::Int,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::list_index(
                    function_list(7, "function_list", function_type.clone()),
                    0,
                    function_type,
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(
            layout.function_lists(),
            &[
                FunctionType::new(Vec::new(), ValueType::Int),
                FunctionType::new(Vec::new(), ValueType::String),
                FunctionType::new(Vec::new(), ValueType::Float),
                FunctionType::new(Vec::new(), ValueType::Bool),
                FunctionType::new(Vec::new(), ValueType::Nil),
                FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                ),
            ],
        );
    }

    #[test]
    fn frame_layout_includes_function_local_storage() {
        let steps = vec![Step::let_int_function(
            IntFunctionLocalId(1),
            "f".into(),
            IntFunctionExpr::local_get(
                IntFunctionLocalId(2),
                "g".into(),
                super::super::test_helpers::int_function_expr()
                    .type_()
                    .clone(),
            ),
        )];
        let return_ = ReturnExpr::int(
            IntFunctionId(0),
            IntExpr::function_call(
                IntFunctionExpr::local_get(
                    IntFunctionLocalId(3),
                    "h".into(),
                    super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                ),
                Vec::new(),
            ),
        );

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.int_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_function_expression_return_families() {
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::local_get(
                    StringFunctionLocalId(1),
                    "string".into(),
                    super::super::test_helpers::string_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::local_get(
                    FloatFunctionLocalId(4),
                    "float".into(),
                    super::super::test_helpers::float_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(2),
                    "bool".into(),
                    super::super::test_helpers::bool_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::local_get(
                    NilFunctionLocalId(3),
                    "nil".into(),
                    super::super::test_helpers::nil_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.string_functions(), 2);
        assert_eq!(layout.float_functions(), 5);
        assert_eq!(layout.bool_functions(), 3);
        assert_eq!(layout.nil_functions(), 4);
    }

    #[test]
    fn frame_layout_includes_function_expression_call_and_closure_families() {
        let string_type = super::super::test_helpers::string_function_expr()
            .type_()
            .clone();
        let float_type = super::super::test_helpers::float_function_expr()
            .type_()
            .clone();
        let bool_type = super::super::test_helpers::bool_function_expr()
            .type_()
            .clone();
        let nil_type = super::super::test_helpers::nil_function_expr()
            .type_()
            .clone();
        let function_type = super::super::test_helpers::function_returning_int_function_type();

        let string_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(string_type.clone())),
        );
        let float_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(float_type.clone())),
        );
        let bool_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(bool_type.clone())),
        );
        let nil_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(nil_type.clone())),
        );
        let function_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(function_type.clone())),
        );

        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::closure(
                    StringFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(30), "string_closure_capture".into()),
                    )],
                    string_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(20),
                        "string_callee".into(),
                        string_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(31), "string_call_arg".into()),
                    )],
                    string_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::closure(
                    FloatFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(38), "float_closure_capture".into()),
                    )],
                    float_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(24),
                        "float_callee".into(),
                        float_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(39), "float_call_arg".into()),
                    )],
                    float_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::closure(
                    BoolFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(32), "bool_closure_capture".into()),
                    )],
                    bool_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(21),
                        "bool_callee".into(),
                        bool_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(33), "bool_call_arg".into()),
                    )],
                    bool_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(NilFunctionExpr::closure(
                NilFunctionId(1),
                Vec::new(),
                vec![CaptureArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(34), "nil_closure_capture".into()),
                )],
                nil_type.clone(),
            )))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(22),
                        "nil_callee".into(),
                        nil_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(35), "nil_call_arg".into()),
                    )],
                    nil_type,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(36), "function_closure_capture".into()),
                    )],
                    function_type.clone(),
                    super::super::test_helpers::int_function_expr()
                        .type_()
                        .clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(23),
                        "function_callee".into(),
                        function_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(37), "function_call_arg".into()),
                    )],
                    function_type,
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 40);
        assert_eq!(layout.function_functions(), 25);
    }

    #[test]
    fn frame_layout_includes_tuple_function_expression_families() {
        let tuple_function_type = tuple_function_type();
        let tuple_function_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(tuple_function_type.clone())),
        );
        let tuple_type = tuple_type();
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::closure(
                    TupleFunctionId(1),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(10),
                        IntExpr::local_get(IntLocalId(10), "closure_capture".into()),
                    )],
                    tuple_function_type.clone(),
                    tuple_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::local_get(
                    TupleFunctionLocalId(2),
                    "local_function".into(),
                    tuple_function_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::call(
                    TupleFunctionFunctionId(0),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(11), "direct_arg".into()),
                    )],
                    tuple_function_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(3),
                        "callee".into(),
                        tuple_function_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(12), "function_arg".into()),
                    )],
                    tuple_function_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::tuple_index(
                    TupleExpr::local_get(TupleLocalId(1), "tuple".into(), tuple_type.clone()),
                    0,
                    tuple_function_type.clone(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::bool_case(
                    BoolExpr::local_get(crate::plan::BoolLocalId(4), "flag".into()),
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(5),
                        "true_branch".into(),
                        tuple_function_type.clone(),
                    ),
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(6),
                        "false_branch".into(),
                        tuple_function_type.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::int_case(
                    IntExpr::local_get(IntLocalId(13), "int_subject".into()),
                    vec![(
                        1.into(),
                        TupleFunctionExpr::local_get(
                            TupleFunctionLocalId(7),
                            "int_branch".into(),
                            tuple_function_type.clone(),
                        ),
                    )],
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(8),
                        "int_fallback".into(),
                        tuple_function_type.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::string_case(
                    StringExpr::local_get(crate::plan::StringLocalId(9), "string_subject".into()),
                    vec![(
                        "hit".into(),
                        TupleFunctionExpr::local_get(
                            TupleFunctionLocalId(10),
                            "string_branch".into(),
                            tuple_function_type.clone(),
                        ),
                    )],
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(11),
                        "string_fallback".into(),
                        tuple_function_type.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(2), "float_subject".into()),
                    vec![(
                        1.0,
                        TupleFunctionExpr::local_get(
                            TupleFunctionLocalId(12),
                            "float_branch".into(),
                            tuple_function_type.clone(),
                        ),
                    )],
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(13),
                        "float_fallback".into(),
                        tuple_function_type.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::local_get(
                        IntLocalId(14),
                        "block_step".into(),
                    )))],
                    TupleFunctionExpr::local_get(
                        TupleFunctionLocalId(14),
                        "block_return".into(),
                        tuple_function_type,
                    ),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 15);
        assert_eq!(layout.floats(), 3);
        assert_eq!(layout.strings(), 10);
        assert_eq!(layout.bools(), 5);
        assert_eq!(layout.tuples(), 2);
        assert_eq!(layout.function_functions(), 4);
        assert_eq!(layout.tuple_functions(), 15);
    }

    #[test]
    fn frame_layout_includes_list_function_expression_families() {
        let list_function_type = list_function_type();
        let list_function_callee_type = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(list_function_type.clone())),
        );
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::closure(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new(),
                    vec![CaptureArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(15), "closure_capture".into()),
                    )],
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::local_get(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        crate::plan::FunctionType::new(
                            Vec::new(),
                            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                        ),
                        crate::plan::ValueType::Int,
                    ),
                    "local_function".into(),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(ListFunctionExpr::call(
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                    ),
                    crate::plan::ValueType::Int,
                ),
                vec![CallArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(16), "direct_arg".into()),
                )],
            )))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::function_call(
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(0),
                        "callee".into(),
                        list_function_callee_type,
                    ),
                    vec![CallArg::int(
                        IntLocalId(0),
                        IntExpr::local_get(IntLocalId(17), "function_arg".into()),
                    )],
                    list_function_type.clone(),
                    ValueType::Int,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::tuple_index(
                    TupleExpr::local_get(
                        TupleLocalId(0),
                        "tuple".into(),
                        vec![ValueType::Function(Box::new(list_function_type.clone()))],
                    ),
                    0,
                    list_function_type.clone(),
                    ValueType::Int,
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::bool_case(
                    BoolExpr::local_get(crate::plan::BoolLocalId(0), "flag".into()),
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            1,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "true_branch".into(),
                    ),
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            2,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "false_branch".into(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::int_case(
                    IntExpr::local_get(IntLocalId(18), "int_subject".into()),
                    vec![(
                        1.into(),
                        ListFunctionExpr::local_get(
                            crate::plan::ListFunctionLocal::from_item_type(
                                3,
                                crate::plan::FunctionType::new(
                                    Vec::new(),
                                    crate::plan::ValueType::List(Box::new(
                                        crate::plan::ValueType::Int,
                                    )),
                                ),
                                crate::plan::ValueType::Int,
                            ),
                            "int_branch".into(),
                        ),
                    )],
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            4,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "int_fallback".into(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::string_case(
                    StringExpr::local_get(crate::plan::StringLocalId(0), "string_subject".into()),
                    vec![(
                        "hit".into(),
                        ListFunctionExpr::local_get(
                            crate::plan::ListFunctionLocal::from_item_type(
                                5,
                                crate::plan::FunctionType::new(
                                    Vec::new(),
                                    crate::plan::ValueType::List(Box::new(
                                        crate::plan::ValueType::Int,
                                    )),
                                ),
                                crate::plan::ValueType::Int,
                            ),
                            "string_branch".into(),
                        ),
                    )],
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            6,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "string_fallback".into(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(
                ListFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(0), "float_subject".into()),
                    vec![(
                        1.0,
                        ListFunctionExpr::local_get(
                            crate::plan::ListFunctionLocal::from_item_type(
                                7,
                                crate::plan::FunctionType::new(
                                    Vec::new(),
                                    crate::plan::ValueType::List(Box::new(
                                        crate::plan::ValueType::Int,
                                    )),
                                ),
                                crate::plan::ValueType::Int,
                            ),
                            "float_branch".into(),
                        ),
                    )],
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            8,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "float_fallback".into(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::list(ListFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(19),
                    "block_step".into(),
                )))],
                ListFunctionExpr::local_get(
                    crate::plan::ListFunctionLocal::from_item_type(
                        9,
                        crate::plan::FunctionType::new(
                            Vec::new(),
                            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                        ),
                        crate::plan::ValueType::Int,
                    ),
                    "block_return".into(),
                ),
            )))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.ints(), 20);
        assert_eq!(layout.floats(), 1);
        assert_eq!(layout.strings(), 1);
        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.tuples(), 1);
        assert_eq!(layout.function_functions(), 1);
        assert_eq!(layout.list_functions().len(), 10);
    }

    #[test]
    fn frame_layout_includes_function_expression_float_case_families() {
        let int_function_type = super::super::test_helpers::int_function_expr()
            .type_()
            .clone();
        let string_function_type = super::super::test_helpers::string_function_expr()
            .type_()
            .clone();
        let float_function_type = super::super::test_helpers::float_function_expr()
            .type_()
            .clone();
        let bool_function_type = super::super::test_helpers::bool_function_expr()
            .type_()
            .clone();
        let nil_function_type = super::super::test_helpers::nil_function_expr()
            .type_()
            .clone();
        let function_function_type = super::super::test_helpers::int_function_expr()
            .type_()
            .clone();

        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::int(
                IntFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(0), "int_subject".into()),
                    vec![(
                        1.0,
                        IntFunctionExpr::local_get(
                            IntFunctionLocalId(0),
                            "int_branch".into(),
                            int_function_type.clone(),
                        ),
                    )],
                    IntFunctionExpr::local_get(
                        IntFunctionLocalId(1),
                        "int_fallback".into(),
                        int_function_type,
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::string(
                StringFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(1), "string_subject".into()),
                    vec![(
                        1.0,
                        StringFunctionExpr::local_get(
                            StringFunctionLocalId(0),
                            "string_branch".into(),
                            string_function_type.clone(),
                        ),
                    )],
                    StringFunctionExpr::local_get(
                        StringFunctionLocalId(1),
                        "string_fallback".into(),
                        string_function_type,
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(2), "float_subject".into()),
                    vec![(
                        1.0,
                        FloatFunctionExpr::local_get(
                            FloatFunctionLocalId(0),
                            "float_branch".into(),
                            float_function_type.clone(),
                        ),
                    )],
                    FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(1),
                        "float_fallback".into(),
                        float_function_type,
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::bool(
                BoolFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(3), "bool_subject".into()),
                    vec![(
                        1.0,
                        BoolFunctionExpr::local_get(
                            BoolFunctionLocalId(0),
                            "bool_branch".into(),
                            bool_function_type.clone(),
                        ),
                    )],
                    BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(1),
                        "bool_fallback".into(),
                        bool_function_type,
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::nil(
                NilFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(4), "nil_subject".into()),
                    vec![(
                        1.0,
                        NilFunctionExpr::local_get(
                            NilFunctionLocalId(0),
                            "nil_branch".into(),
                            nil_function_type.clone(),
                        ),
                    )],
                    NilFunctionExpr::local_get(
                        NilFunctionLocalId(1),
                        "nil_fallback".into(),
                        nil_function_type,
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::float_case(
                    FloatExpr::local_get(crate::plan::FloatLocalId(5), "function_subject".into()),
                    vec![(
                        1.0,
                        FunctionFunctionExpr::local_get(
                            FunctionFunctionLocalId(0),
                            "function_branch".into(),
                            function_function_type.clone(),
                        ),
                    )],
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(1),
                        "function_fallback".into(),
                        function_function_type,
                    ),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.floats(), 6);
        assert_eq!(layout.int_functions(), 2);
        assert_eq!(layout.string_functions(), 2);
        assert_eq!(layout.float_functions(), 2);
        assert_eq!(layout.bool_functions(), 2);
        assert_eq!(layout.nil_functions(), 2);
        assert_eq!(layout.function_functions(), 2);
    }

    #[test]
    fn frame_layout_includes_float_function_expression_subject_case_families() {
        let type_ = super::super::test_helpers::float_function_expr()
            .type_()
            .clone();
        let steps = vec![
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::bool_case(
                    BoolExpr::local_get(crate::plan::BoolLocalId(4), "flag".into()),
                    FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(8),
                        "bool_true".into(),
                        type_.clone(),
                    ),
                    FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(9),
                        "bool_false".into(),
                        type_.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::int_case(
                    IntExpr::local_get(IntLocalId(5), "int_subject".into()),
                    vec![(
                        1.into(),
                        FloatFunctionExpr::local_get(
                            FloatFunctionLocalId(10),
                            "int_branch".into(),
                            type_.clone(),
                        ),
                    )],
                    FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(11),
                        "int_fallback".into(),
                        type_.clone(),
                    ),
                ),
            ))),
            Step::evaluate(Expr::function(FunctionExpr::float(
                FloatFunctionExpr::string_case(
                    StringExpr::local_get(crate::plan::StringLocalId(6), "string_subject".into()),
                    vec![(
                        "hit".into(),
                        FloatFunctionExpr::local_get(
                            FloatFunctionLocalId(12),
                            "string_branch".into(),
                            type_.clone(),
                        ),
                    )],
                    FloatFunctionExpr::local_get(
                        FloatFunctionLocalId(13),
                        "string_fallback".into(),
                        type_,
                    ),
                ),
            ))),
        ];
        let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

        let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

        assert_eq!(layout.bools(), 5);
        assert_eq!(layout.ints(), 6);
        assert_eq!(layout.strings(), 7);
        assert_eq!(layout.float_functions(), 14);
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int, ValueType::String]
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Tuple(tuple_type()))
    }

    fn list_type() -> ValueType {
        ValueType::Int
    }

    fn panic_message(index: usize) -> PanicExpr {
        PanicExpr::panic_at(
            Some(StringExpr::local_get(
                StringLocalId(index),
                format!("function_panic_message_{index}").into(),
            )),
            PanicSite::unknown(),
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::List(Box::new(list_type())))
    }
}
