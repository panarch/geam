use super::{
    BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn, BoolListFunctionFunctionId,
    BoolListFunctionId, BoolReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn,
    FloatListFunctionFunctionId, FloatListFunctionId, FloatReturn, FunctionFunctionFunctionId,
    FunctionFunctionReturn, FunctionListFunctionFunctionId, FunctionListFunctionId, FunctionPlan,
    IntFunctionFunctionId, IntFunctionId, IntFunctionReturn, IntListFunctionFunctionId,
    IntListFunctionId, IntReturn, ListFunctionFunctionId, ListFunctionId, ListFunctionReturn,
    ListListFunctionFunctionId, ListListFunctionId, ListReturn, NilFunctionFunctionId,
    NilFunctionId, NilFunctionReturn, NilListFunctionFunctionId, NilListFunctionId, NilReturn,
    RuntimeFunction, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
    StringFunctionReturn, StringListFunctionFunctionId, StringListFunctionId, StringReturn,
    TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn, TupleListFunctionFunctionId,
    TupleListFunctionId, TupleReturn,
};
use crate::plan::ReturnExprKind;

pub(super) struct RuntimePlan {
    main: RuntimeFunctionId,
    int_functions: Vec<RuntimeFunction<IntReturn>>,
    float_functions: Vec<RuntimeFunction<FloatReturn>>,
    string_functions: Vec<RuntimeFunction<StringReturn>>,
    bool_functions: Vec<RuntimeFunction<BoolReturn>>,
    nil_functions: Vec<RuntimeFunction<NilReturn>>,
    tuple_functions: Vec<RuntimeFunction<TupleReturn>>,
    int_list_functions: Vec<RuntimeFunction<ListReturn>>,
    string_list_functions: Vec<RuntimeFunction<ListReturn>>,
    float_list_functions: Vec<RuntimeFunction<ListReturn>>,
    bool_list_functions: Vec<RuntimeFunction<ListReturn>>,
    nil_list_functions: Vec<RuntimeFunction<ListReturn>>,
    tuple_list_functions: Vec<RuntimeFunction<ListReturn>>,
    list_list_functions: Vec<RuntimeFunction<ListReturn>>,
    function_list_functions: Vec<RuntimeFunction<ListReturn>>,
    int_function_functions: Vec<RuntimeFunction<IntFunctionReturn>>,
    float_function_functions: Vec<RuntimeFunction<FloatFunctionReturn>>,
    string_function_functions: Vec<RuntimeFunction<StringFunctionReturn>>,
    bool_function_functions: Vec<RuntimeFunction<BoolFunctionReturn>>,
    nil_function_functions: Vec<RuntimeFunction<NilFunctionReturn>>,
    tuple_function_functions: Vec<RuntimeFunction<TupleFunctionReturn>>,
    int_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    string_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    float_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    bool_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    nil_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    tuple_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    list_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    function_list_function_functions: Vec<RuntimeFunction<ListFunctionReturn>>,
    function_function_functions: Vec<RuntimeFunction<FunctionFunctionReturn>>,
}

impl RuntimePlan {
    pub(super) fn new(
        main: &FunctionPlan,
        functions: &[FunctionPlan],
        anonymous_functions: &[FunctionPlan],
    ) -> Self {
        let mut runtime = RuntimePlanBuilder::default();
        let main_runtime = main.return_().runtime_id();
        runtime.push(main);

        for function in functions {
            runtime.push(function);
        }
        for function in anonymous_functions {
            runtime.push(function);
        }

        runtime.finish(main_runtime)
    }

    pub(super) fn main(&self) -> RuntimeFunctionId {
        self.main.clone()
    }

    pub(super) fn int_function(&self, id: IntFunctionId) -> &RuntimeFunction<IntReturn> {
        &self.int_functions[id.0]
    }

    pub(super) fn float_function(&self, id: FloatFunctionId) -> &RuntimeFunction<FloatReturn> {
        &self.float_functions[id.0]
    }

    pub(super) fn string_function(&self, id: StringFunctionId) -> &RuntimeFunction<StringReturn> {
        &self.string_functions[id.0]
    }

    pub(super) fn bool_function(&self, id: BoolFunctionId) -> &RuntimeFunction<BoolReturn> {
        &self.bool_functions[id.0]
    }

    pub(super) fn nil_function(&self, id: NilFunctionId) -> &RuntimeFunction<NilReturn> {
        &self.nil_functions[id.0]
    }

    pub(super) fn tuple_function(&self, id: TupleFunctionId) -> &RuntimeFunction<TupleReturn> {
        &self.tuple_functions[id.0]
    }

    pub(super) fn list_function(&self, id: &ListFunctionId) -> &RuntimeFunction<ListReturn> {
        match id {
            ListFunctionId::Int(id) => &self.int_list_functions[id.0],
            ListFunctionId::String(id) => &self.string_list_functions[id.0],
            ListFunctionId::Float(id) => &self.float_list_functions[id.0],
            ListFunctionId::Bool(id) => &self.bool_list_functions[id.0],
            ListFunctionId::Nil(id) => &self.nil_list_functions[id.0],
            ListFunctionId::Tuple { id, .. } => &self.tuple_list_functions[id.0],
            ListFunctionId::List { id, .. } => &self.list_list_functions[id.0],
            ListFunctionId::Function { id, .. } => &self.function_list_functions[id.0],
        }
    }

    pub(super) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &RuntimeFunction<IntFunctionReturn> {
        &self.int_function_functions[id.0]
    }

    pub(super) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &RuntimeFunction<FloatFunctionReturn> {
        &self.float_function_functions[id.0]
    }

    pub(super) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &RuntimeFunction<StringFunctionReturn> {
        &self.string_function_functions[id.0]
    }

    pub(super) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &RuntimeFunction<BoolFunctionReturn> {
        &self.bool_function_functions[id.0]
    }

    pub(super) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &RuntimeFunction<NilFunctionReturn> {
        &self.nil_function_functions[id.0]
    }

    pub(super) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &RuntimeFunction<TupleFunctionReturn> {
        &self.tuple_function_functions[id.0]
    }

    pub(super) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &RuntimeFunction<ListFunctionReturn> {
        match id {
            ListFunctionFunctionId::Int { id, .. } => &self.int_list_function_functions[id.0],
            ListFunctionFunctionId::String { id, .. } => &self.string_list_function_functions[id.0],
            ListFunctionFunctionId::Float { id, .. } => &self.float_list_function_functions[id.0],
            ListFunctionFunctionId::Bool { id, .. } => &self.bool_list_function_functions[id.0],
            ListFunctionFunctionId::Nil { id, .. } => &self.nil_list_function_functions[id.0],
            ListFunctionFunctionId::Tuple { id, .. } => &self.tuple_list_function_functions[id.0],
            ListFunctionFunctionId::List { id, .. } => &self.list_list_function_functions[id.0],
            ListFunctionFunctionId::Function { id, .. } => {
                &self.function_list_function_functions[id.0]
            }
        }
    }

    pub(super) fn function_function_function(
        &self,
        id: FunctionFunctionFunctionId,
    ) -> &RuntimeFunction<FunctionFunctionReturn> {
        &self.function_function_functions[id.0]
    }
}

#[derive(Default)]
struct RuntimePlanBuilder {
    int_functions: Vec<(usize, RuntimeFunction<IntReturn>)>,
    float_functions: Vec<(usize, RuntimeFunction<FloatReturn>)>,
    string_functions: Vec<(usize, RuntimeFunction<StringReturn>)>,
    bool_functions: Vec<(usize, RuntimeFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, RuntimeFunction<NilReturn>)>,
    tuple_functions: Vec<(usize, RuntimeFunction<TupleReturn>)>,
    int_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    string_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    float_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    bool_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    nil_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    tuple_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    list_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    function_list_functions: Vec<(usize, RuntimeFunction<ListReturn>)>,
    int_function_functions: Vec<(usize, RuntimeFunction<IntFunctionReturn>)>,
    float_function_functions: Vec<(usize, RuntimeFunction<FloatFunctionReturn>)>,
    string_function_functions: Vec<(usize, RuntimeFunction<StringFunctionReturn>)>,
    bool_function_functions: Vec<(usize, RuntimeFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, RuntimeFunction<NilFunctionReturn>)>,
    tuple_function_functions: Vec<(usize, RuntimeFunction<TupleFunctionReturn>)>,
    int_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    string_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    float_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    bool_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    nil_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    tuple_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    list_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    function_list_function_functions: Vec<(usize, RuntimeFunction<ListFunctionReturn>)>,
    function_function_functions: Vec<(usize, RuntimeFunction<FunctionFunctionReturn>)>,
}

impl RuntimePlanBuilder {
    fn push(&mut self, function: &FunctionPlan) {
        runtime_function(function, self);
    }

    fn push_list_function(&mut self, id: &ListFunctionId, function: RuntimeFunction<ListReturn>) {
        match id {
            ListFunctionId::Int(IntListFunctionId(index)) => {
                self.int_list_functions.push((*index, function));
            }
            ListFunctionId::String(StringListFunctionId(index)) => {
                self.string_list_functions.push((*index, function));
            }
            ListFunctionId::Float(FloatListFunctionId(index)) => {
                self.float_list_functions.push((*index, function));
            }
            ListFunctionId::Bool(BoolListFunctionId(index)) => {
                self.bool_list_functions.push((*index, function));
            }
            ListFunctionId::Nil(NilListFunctionId(index)) => {
                self.nil_list_functions.push((*index, function));
            }
            ListFunctionId::Tuple {
                id: TupleListFunctionId(index),
                ..
            } => {
                self.tuple_list_functions.push((*index, function));
            }
            ListFunctionId::List {
                id: ListListFunctionId(index),
                ..
            } => {
                self.list_list_functions.push((*index, function));
            }
            ListFunctionId::Function {
                id: FunctionListFunctionId(index),
                ..
            } => {
                self.function_list_functions.push((*index, function));
            }
        }
    }

    fn push_list_function_function(
        &mut self,
        id: &ListFunctionFunctionId,
        function: RuntimeFunction<ListFunctionReturn>,
    ) {
        match id {
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(index),
                ..
            } => {
                self.int_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(index),
                ..
            } => {
                self.string_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(index),
                ..
            } => {
                self.float_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(index),
                ..
            } => {
                self.bool_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(index),
                ..
            } => {
                self.nil_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(index),
                ..
            } => {
                self.tuple_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(index),
                ..
            } => {
                self.list_list_function_functions.push((*index, function));
            }
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(index),
                ..
            } => {
                self.function_list_function_functions
                    .push((*index, function));
            }
        }
    }

    fn finish(self, main: RuntimeFunctionId) -> RuntimePlan {
        RuntimePlan {
            main,
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

fn runtime_function(function: &FunctionPlan, runtime_functions: &mut RuntimePlanBuilder) {
    match function.return_().kind() {
        ReturnExprKind::Int { runtime_id, body } => {
            runtime_functions.int_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Float { runtime_id, body } => {
            runtime_functions.float_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::String { runtime_id, body } => {
            runtime_functions.string_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Bool { runtime_id, body } => {
            runtime_functions.bool_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Nil { runtime_id, body } => {
            runtime_functions.nil_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::Tuple {
            runtime_id, body, ..
        } => {
            runtime_functions.tuple_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::List {
            runtime_id, body, ..
        } => {
            runtime_functions.push_list_function(
                runtime_id,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            );
        }
        ReturnExprKind::IntFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.int_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::FloatFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.float_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::StringFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.string_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::BoolFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.bool_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::NilFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.nil_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::TupleFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.tuple_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
        ReturnExprKind::ListFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.push_list_function_function(
                runtime_id,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            );
        }
        ReturnExprKind::FunctionFunction {
            runtime_id, body, ..
        } => {
            runtime_functions.function_function_functions.push((
                runtime_id.0,
                RuntimeFunction::new(
                    function.frame_layout(),
                    function.steps().to_vec(),
                    body.clone(),
                ),
            ));
        }
    }
}

fn sort_functions<Return>(
    mut functions: Vec<(usize, RuntimeFunction<Return>)>,
) -> Vec<RuntimeFunction<Return>> {
    functions.sort_by_key(|(index, _)| *index);
    functions
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::RuntimePlan;
    use crate::plan::{
        FunctionId, FunctionPlan, FunctionType, IntExpr, IntFunctionId, ListElements, ListExpr,
        ListFunctionExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionValue, ParamLocal,
        ReturnBody, ReturnExpr, ValueType,
    };

    #[test]
    fn runtime_plan_stores_list_returning_functions_by_item_family() {
        for (function_index, item_type) in list_item_types().into_iter().enumerate() {
            let runtime_id = ListFunctionId::from_item_type(0, item_type.clone());
            let return_ = ReturnBody::expr(empty_list_expr(item_type));
            let function = FunctionPlan::new(
                FunctionId::new(function_index + 1),
                format!("list_{function_index}").into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_body(runtime_id.clone(), return_.clone()),
            );

            let runtime = RuntimePlan::new(&main_function(), &[function], &[]);

            assert_eq!(runtime.list_function(&runtime_id).return_(), &return_);
        }
    }

    #[test]
    fn runtime_plan_stores_list_function_returning_functions_by_item_family() {
        for (function_index, item_type) in list_item_types().into_iter().enumerate() {
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
            let return_ = ReturnBody::expr(ListFunctionExpr::value(returned_function));
            let function = FunctionPlan::new(
                FunctionId::new(function_index + 1),
                format!("list_function_{function_index}").into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::list_function_body(runtime_id.clone(), return_.clone()),
            );

            let runtime = RuntimePlan::new(&main_function(), &[function], &[]);

            assert_eq!(
                runtime.list_function_function(&runtime_id).return_(),
                &return_
            );
        }
    }

    fn main_function() -> FunctionPlan {
        FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
        )
    }

    fn list_item_types() -> Vec<ValueType> {
        vec![
            ValueType::Int,
            ValueType::String,
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::String,
            ))),
        ]
    }

    fn empty_list_expr(item_type: ValueType) -> ListExpr {
        match item_type {
            ValueType::Int => ListExpr::from_elements(ListElements::Int(Vec::new())),
            ValueType::String => ListExpr::from_elements(ListElements::String(Vec::new())),
            ValueType::Float => ListExpr::from_elements(ListElements::Float(Vec::new())),
            ValueType::Bool => ListExpr::from_elements(ListElements::Bool(Vec::new())),
            ValueType::Nil => ListExpr::from_elements(ListElements::Nil(Vec::new())),
            ValueType::Tuple(item_type) => ListExpr::from_elements(ListElements::Tuple {
                item_type,
                values: Vec::new(),
            }),
            ValueType::List(item_type) => ListExpr::from_elements(ListElements::List {
                item_type,
                values: Vec::new(),
            }),
            ValueType::Function(item_type) => ListExpr::from_elements(ListElements::Function {
                item_type: *item_type,
                values: Vec::new(),
            }),
        }
    }
}
