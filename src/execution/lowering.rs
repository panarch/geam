use super::ExecutionPlan;
use super::function::ExecutableFunction;
use super::table::FunctionTables;
use crate::plan::{
    BoolFunctionReturn, BoolListReturn, BoolReturn, FloatFunctionReturn, FloatListReturn,
    FloatReturn, FunctionExecutionParts, FunctionFunctionReturn, FunctionListReturn, FunctionPlan,
    IntFunctionReturn, IntListReturn, IntReturn, ListFunctionFunctionId, ListFunctionReturn,
    ListListReturn, ModulePlan, NilFunctionReturn, NilListReturn, NilReturn, ReturnExprKind,
    StringFunctionReturn, StringListReturn, StringReturn, TupleFunctionReturn, TupleListReturn,
    TupleReturn,
};

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    let main = parts.main.return_().runtime_id();
    let mut tables = FunctionTableBuilder::default();

    tables.push(parts.main);
    for function in parts.functions {
        tables.push(function);
    }
    for function in parts.anonymous_functions {
        tables.push(function);
    }

    ExecutionPlan {
        module: parts.module,
        source_context: parts.source_context,
        main,
        functions: tables.finish(),
    }
}

#[derive(Default)]
struct FunctionTableBuilder {
    int_functions: Vec<(usize, ExecutableFunction<IntReturn>)>,
    float_functions: Vec<(usize, ExecutableFunction<FloatReturn>)>,
    string_functions: Vec<(usize, ExecutableFunction<StringReturn>)>,
    bool_functions: Vec<(usize, ExecutableFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, ExecutableFunction<NilReturn>)>,
    tuple_functions: Vec<(usize, ExecutableFunction<TupleReturn>)>,
    int_list_functions: Vec<(usize, ExecutableFunction<IntListReturn>)>,
    string_list_functions: Vec<(usize, ExecutableFunction<StringListReturn>)>,
    float_list_functions: Vec<(usize, ExecutableFunction<FloatListReturn>)>,
    bool_list_functions: Vec<(usize, ExecutableFunction<BoolListReturn>)>,
    nil_list_functions: Vec<(usize, ExecutableFunction<NilListReturn>)>,
    tuple_list_functions: Vec<(usize, ExecutableFunction<TupleListReturn>)>,
    list_list_functions: Vec<(usize, ExecutableFunction<ListListReturn>)>,
    function_list_functions: Vec<(usize, ExecutableFunction<FunctionListReturn>)>,
    int_function_functions: Vec<(usize, ExecutableFunction<IntFunctionReturn>)>,
    float_function_functions: Vec<(usize, ExecutableFunction<FloatFunctionReturn>)>,
    string_function_functions: Vec<(usize, ExecutableFunction<StringFunctionReturn>)>,
    bool_function_functions: Vec<(usize, ExecutableFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, ExecutableFunction<NilFunctionReturn>)>,
    tuple_function_functions: Vec<(usize, ExecutableFunction<TupleFunctionReturn>)>,
    int_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    string_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    float_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    bool_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    nil_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    tuple_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    list_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    function_list_function_functions: Vec<(usize, ExecutableFunction<ListFunctionReturn>)>,
    function_function_functions: Vec<(usize, ExecutableFunction<FunctionFunctionReturn>)>,
}

impl FunctionTableBuilder {
    fn push(&mut self, function: FunctionPlan) {
        let FunctionExecutionParts {
            frame_layout,
            steps,
            return_,
        } = function.into_execution_parts();

        match return_.into_kind() {
            ReturnExprKind::Int { runtime_id, body } => self.int_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::Float { runtime_id, body } => self.float_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::String { runtime_id, body } => self.string_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::Bool { runtime_id, body } => self.bool_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::Nil { runtime_id, body } => self.nil_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::Tuple {
                runtime_id, body, ..
            } => self.tuple_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::IntList { runtime_id, body } => self.int_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::StringList { runtime_id, body } => self.string_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::FloatList { runtime_id, body } => self.float_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::BoolList { runtime_id, body } => self.bool_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::NilList { runtime_id, body } => self.nil_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::TupleList {
                runtime_id, body, ..
            } => self.tuple_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::ListList {
                runtime_id, body, ..
            } => self.list_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::FunctionList {
                runtime_id, body, ..
            } => self.function_list_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::IntFunction {
                runtime_id, body, ..
            } => self.int_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::FloatFunction {
                runtime_id, body, ..
            } => self.float_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::StringFunction {
                runtime_id, body, ..
            } => self.string_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::BoolFunction {
                runtime_id, body, ..
            } => self.bool_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::NilFunction {
                runtime_id, body, ..
            } => self.nil_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::TupleFunction {
                runtime_id, body, ..
            } => self.tuple_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
            ReturnExprKind::ListFunction {
                runtime_id, body, ..
            } => self.push_list_function_function(
                runtime_id,
                ExecutableFunction::new(frame_layout, steps, body),
            ),
            ReturnExprKind::FunctionFunction {
                runtime_id, body, ..
            } => self.function_function_functions.push((
                runtime_id.0,
                ExecutableFunction::new(frame_layout, steps, body),
            )),
        }
    }

    fn push_list_function_function(
        &mut self,
        id: ListFunctionFunctionId,
        function: ExecutableFunction<ListFunctionReturn>,
    ) {
        match id {
            ListFunctionFunctionId::Int { id, .. } => {
                self.int_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::String { id, .. } => {
                self.string_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Float { id, .. } => {
                self.float_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Bool { id, .. } => {
                self.bool_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Nil { id, .. } => {
                self.nil_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Tuple { id, .. } => {
                self.tuple_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::List { id, .. } => {
                self.list_list_function_functions.push((id.0, function));
            }
            ListFunctionFunctionId::Function { id, .. } => {
                self.function_list_function_functions.push((id.0, function));
            }
        }
    }

    fn finish(self) -> FunctionTables {
        FunctionTables {
            int_functions: sort_functions(self.int_functions),
            float_functions: sort_functions(self.float_functions),
            string_functions: sort_functions(self.string_functions),
            bool_functions: sort_functions(self.bool_functions),
            nil_functions: sort_functions(self.nil_functions),
            tuple_functions: sort_functions(self.tuple_functions),
            int_list_functions: sort_functions(self.int_list_functions),
            string_list_functions: sort_functions(self.string_list_functions),
            float_list_functions: sort_functions(self.float_list_functions),
            bool_list_functions: sort_functions(self.bool_list_functions),
            nil_list_functions: sort_functions(self.nil_list_functions),
            tuple_list_functions: sort_functions(self.tuple_list_functions),
            list_list_functions: sort_functions(self.list_list_functions),
            function_list_functions: sort_functions(self.function_list_functions),
            int_function_functions: sort_functions(self.int_function_functions),
            float_function_functions: sort_functions(self.float_function_functions),
            string_function_functions: sort_functions(self.string_function_functions),
            bool_function_functions: sort_functions(self.bool_function_functions),
            nil_function_functions: sort_functions(self.nil_function_functions),
            tuple_function_functions: sort_functions(self.tuple_function_functions),
            int_list_function_functions: sort_functions(self.int_list_function_functions),
            string_list_function_functions: sort_functions(self.string_list_function_functions),
            float_list_function_functions: sort_functions(self.float_list_function_functions),
            bool_list_function_functions: sort_functions(self.bool_list_function_functions),
            nil_list_function_functions: sort_functions(self.nil_list_function_functions),
            tuple_list_function_functions: sort_functions(self.tuple_list_function_functions),
            list_list_function_functions: sort_functions(self.list_list_function_functions),
            function_list_function_functions: sort_functions(self.function_list_function_functions),
            function_function_functions: sort_functions(self.function_function_functions),
        }
    }
}

fn sort_functions<Return>(
    mut functions: Vec<(usize, ExecutableFunction<Return>)>,
) -> Vec<ExecutableFunction<Return>> {
    functions.sort_by_key(|(index, _)| *index);
    functions
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::lower;
    use crate::plan::{
        BoolListFunctionId, BoolListReturn, FloatListFunctionId, FloatListReturn, FunctionId,
        FunctionListFunctionId, FunctionListReturn, FunctionPlan, FunctionType, IntExpr,
        IntFunctionId, IntListFunctionId, IntListReturn, IntLocalId, ListExpr, ListFunctionExpr,
        ListFunctionFunctionId, ListFunctionId, ListFunctionValue, ListListFunctionId,
        ListListReturn, ModulePlan, NilListFunctionId, NilListReturn, ParamLocal, ReturnBody,
        ReturnExpr, RuntimeFunctionId, SourceContext, Step, StringListFunctionId, StringListReturn,
        TupleListFunctionId, TupleListReturn, ValueType,
    };

    #[test]
    fn lowering_stores_functions_by_runtime_id_order() {
        let main = function(0, "main", 1, 11);
        let helper = function(1, "helper", 0, 10);
        let plan = lower(ModulePlan::new("main".into(), main, vec![helper]));

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(1))
        );
        assert_eq!(
            plan.int_function(IntFunctionId(0)).return_(),
            &ReturnBody::expr(IntExpr::value(10.into())),
        );
        assert_eq!(
            plan.int_function(IntFunctionId(1)).return_(),
            &ReturnBody::expr(IntExpr::value(11.into())),
        );
    }

    #[test]
    fn lowering_moves_source_context_and_anonymous_functions() {
        let source_context = SourceContext::new("main.gleam", "pub fn main() { 1 }");
        let module_plan = ModulePlan::new("main".into(), function(0, "main", 0, 1), Vec::new())
            .with_anonymous_functions(vec![function(1, "<anonymous:0>", 1, 2)])
            .with_source_context(source_context.clone());

        let plan = lower(module_plan);

        assert_eq!(plan.module(), "main");
        assert_eq!(plan.source_context(), Some(&source_context));
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
        assert_eq!(
            plan.int_function(IntFunctionId(1)).return_(),
            &ReturnBody::expr(IntExpr::value(2.into())),
        );
    }

    #[test]
    fn lowering_preserves_function_frame_steps_and_return_body() {
        let step = Step::let_int(IntLocalId(0), "value".into(), IntExpr::value(1.into()));
        let return_ = ReturnBody::expr(IntExpr::local_get(IntLocalId(0), "value".into()));
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            vec![step.clone()],
            ReturnExpr::int_body(IntFunctionId(0), return_.clone()),
        );

        let plan = lower(ModulePlan::new("main".into(), main, Vec::new()));
        let function = plan.int_function(IntFunctionId(0));

        assert_eq!(function.frame_layout().ints(), 1);
        assert_eq!(function.steps(), &[step]);
        assert_eq!(function.return_(), &return_);
    }

    #[test]
    fn lowering_stores_every_list_return_family() {
        let tuple_item = vec![ValueType::Int];
        let list_item = Box::new(ValueType::Int);
        let function_item = FunctionType::new(Vec::new(), ValueType::Int);

        let int_body = IntListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::Int)
                .into_int()
                .expect("expression should be List(Int)"),
        );
        let float_body = FloatListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::Float)
                .into_float()
                .expect("expression should be List(Float)"),
        );
        let string_body = StringListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::String)
                .into_string()
                .expect("expression should be List(String)"),
        );
        let bool_body = BoolListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::Bool)
                .into_bool()
                .expect("expression should be List(Bool)"),
        );
        let nil_body = NilListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::Nil)
                .into_nil()
                .expect("expression should be List(Nil)"),
        );
        let tuple_body = TupleListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::Tuple(tuple_item.clone()))
                .into_tuple()
                .expect("expression should be List(Tuple)"),
        );
        let list_body = ListListReturn::expr(
            ListExpr::value(Vec::new(), ValueType::List(list_item.clone()))
                .into_list()
                .expect("expression should be List(List)"),
        );
        let function_body = FunctionListReturn::expr(
            ListExpr::value(
                Vec::new(),
                ValueType::Function(Box::new(function_item.clone())),
            )
            .into_function()
            .expect("expression should be List(Function)"),
        );

        let functions = vec![
            FunctionPlan::new(
                FunctionId::new(1),
                "int_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int_list_body(IntListFunctionId(0), int_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(2),
                "float_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::float_list_body(FloatListFunctionId(0), float_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(3),
                "string_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::string_list_body(StringListFunctionId(0), string_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(4),
                "bool_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::bool_list_body(BoolListFunctionId(0), bool_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(5),
                "nil_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::nil_list_body(NilListFunctionId(0), nil_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(6),
                "tuple_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::tuple_list_body(TupleListFunctionId(0), tuple_item, tuple_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(7),
                "list_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_list_body(ListListFunctionId(0), list_item, list_body.clone()),
            ),
            FunctionPlan::new(
                FunctionId::new(8),
                "function_list".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::function_list_body(
                    FunctionListFunctionId(0),
                    function_item,
                    function_body.clone(),
                ),
            ),
        ];
        let plan = lower(ModulePlan::new(
            "main".into(),
            function(0, "main", 0, 0),
            functions,
        ));

        assert_eq!(
            plan.int_list_function(IntListFunctionId(0)).return_(),
            &int_body
        );
        assert_eq!(
            plan.float_list_function(FloatListFunctionId(0)).return_(),
            &float_body,
        );
        assert_eq!(
            plan.string_list_function(StringListFunctionId(0)).return_(),
            &string_body,
        );
        assert_eq!(
            plan.bool_list_function(BoolListFunctionId(0)).return_(),
            &bool_body,
        );
        assert_eq!(
            plan.nil_list_function(NilListFunctionId(0)).return_(),
            &nil_body
        );
        assert_eq!(
            plan.tuple_list_function(TupleListFunctionId(0)).return_(),
            &tuple_body,
        );
        assert_eq!(
            plan.list_list_function(ListListFunctionId(0)).return_(),
            &list_body,
        );
        assert_eq!(
            plan.function_list_function(FunctionListFunctionId(0))
                .return_(),
            &function_body,
        );
    }

    #[test]
    fn lowering_stores_list_function_returns_by_item_family() {
        let item_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        ];
        let mut expected = Vec::new();
        let mut functions = Vec::new();

        for (index, item_type) in item_types.into_iter().enumerate() {
            let returned_function_id = ListFunctionId::from_item_type(0, item_type.clone());
            let returned_function = ListFunctionValue::new(
                returned_function_id,
                vec![ParamLocal::int(crate::plan::IntLocalId(0))],
            );
            let function_type = FunctionType::new(
                vec![ValueType::Int],
                ValueType::List(Box::new(item_type.clone())),
            );
            let runtime_id = ListFunctionFunctionId::from_item_type(0, function_type, item_type);
            let body = ReturnBody::expr(ListFunctionExpr::value(returned_function));
            functions.push(FunctionPlan::new(
                FunctionId::new(index + 1),
                format!("list_function_{index}").into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_function_body(runtime_id.clone(), body.clone()),
            ));
            expected.push((runtime_id, body));
        }

        let plan = lower(ModulePlan::new(
            "main".into(),
            function(0, "main", 0, 0),
            functions,
        ));

        for (runtime_id, body) in expected {
            assert_eq!(plan.list_function_function(&runtime_id).return_(), &body);
        }
    }

    fn function(id: usize, name: &str, runtime_id: usize, value: i64) -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(id),
            name.into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(runtime_id), IntExpr::value(value.into())),
        )
    }
}
