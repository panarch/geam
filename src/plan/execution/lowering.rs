mod expression;
mod frame;
mod id;
mod param;
mod return_;
mod step;

use super::function::ExecutableFunction;
use super::table::FunctionTables;
use super::{
    BoolFunctionReturn, BoolListReturn, BoolReturn, ExecutionPlan, FloatFunctionReturn,
    FloatListReturn, FloatReturn, FunctionFunctionReturn, FunctionListReturn, IntFunctionReturn,
    IntListReturn, IntReturn, ListFunctionFunctionId, ListFunctionReturn, ListListReturn,
    NilFunctionReturn, NilListReturn, NilReturn, RuntimeFunctionId, StringFunctionReturn,
    StringListReturn, StringReturn, TupleFunctionReturn, TupleListReturn, TupleReturn,
};
use crate::plan::{ModulePlan, module};

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    let mut tables = FunctionTableBuilder::default();
    let main = tables.push(parts.main);

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
    fn push(&mut self, function: module::FunctionPlan) -> RuntimeFunctionId {
        let module::FunctionExecutionParts {
            frame_layout,
            steps,
            return_,
        } = function.into_execution_parts();
        let frame_layout = frame::frame_layout(frame_layout);
        let steps = step::steps(steps);

        match return_.into_kind() {
            module::ReturnExprKind::Int { runtime_id, body } => {
                let id = super::IntFunctionId(runtime_id.0);
                self.int_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::int_return(body)),
                ));
                RuntimeFunctionId::Int(id)
            }
            module::ReturnExprKind::Float { runtime_id, body } => {
                let id = super::FloatFunctionId(runtime_id.0);
                self.float_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::float_return(body)),
                ));
                RuntimeFunctionId::Float(id)
            }
            module::ReturnExprKind::String { runtime_id, body } => {
                let id = super::StringFunctionId(runtime_id.0);
                self.string_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::string_return(body)),
                ));
                RuntimeFunctionId::String(id)
            }
            module::ReturnExprKind::Bool { runtime_id, body } => {
                let id = super::BoolFunctionId(runtime_id.0);
                self.bool_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::bool_return(body)),
                ));
                RuntimeFunctionId::Bool(id)
            }
            module::ReturnExprKind::Nil { runtime_id, body } => {
                let id = super::NilFunctionId(runtime_id.0);
                self.nil_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::nil_return(body)),
                ));
                RuntimeFunctionId::Nil(id)
            }
            module::ReturnExprKind::Tuple {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::TupleFunctionId(runtime_id.0);
                self.tuple_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::tuple_return(body)),
                ));
                RuntimeFunctionId::Tuple {
                    id,
                    return_type: type_,
                }
            }
            module::ReturnExprKind::IntList { runtime_id, body } => {
                let id = super::IntListFunctionId(runtime_id.0);
                self.int_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::int_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Int(id))
            }
            module::ReturnExprKind::StringList { runtime_id, body } => {
                let id = super::StringListFunctionId(runtime_id.0);
                self.string_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::string_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::String(id))
            }
            module::ReturnExprKind::FloatList { runtime_id, body } => {
                let id = super::FloatListFunctionId(runtime_id.0);
                self.float_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::float_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Float(id))
            }
            module::ReturnExprKind::BoolList { runtime_id, body } => {
                let id = super::BoolListFunctionId(runtime_id.0);
                self.bool_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::bool_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Bool(id))
            }
            module::ReturnExprKind::NilList { runtime_id, body } => {
                let id = super::NilListFunctionId(runtime_id.0);
                self.nil_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::nil_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Nil(id))
            }
            module::ReturnExprKind::TupleList {
                runtime_id,
                item_type,
                body,
            } => {
                let id = super::TupleListFunctionId(runtime_id.0);
                self.tuple_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::tuple_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Tuple { id, item_type })
            }
            module::ReturnExprKind::ListList {
                runtime_id,
                item_type,
                body,
            } => {
                let id = super::ListListFunctionId(runtime_id.0);
                self.list_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(frame_layout, steps, return_::list_list_return(body)),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::List { id, item_type })
            }
            module::ReturnExprKind::FunctionList {
                runtime_id,
                item_type,
                body,
            } => {
                let id = super::FunctionListFunctionId(runtime_id.0);
                self.function_list_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::function_list_return(body),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Function { id, item_type })
            }
            module::ReturnExprKind::IntFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::IntFunctionFunctionId(runtime_id.0);
                self.int_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::int_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Int(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::FloatFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::FloatFunctionFunctionId(runtime_id.0);
                self.float_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::float_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Float(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::StringFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::StringFunctionFunctionId(runtime_id.0);
                self.string_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::string_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::String(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::BoolFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::BoolFunctionFunctionId(runtime_id.0);
                self.bool_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::bool_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Bool(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::NilFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::NilFunctionFunctionId(runtime_id.0);
                self.nil_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::nil_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Nil(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::TupleFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::TupleFunctionFunctionId(runtime_id.0);
                self.tuple_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::tuple_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Tuple(id),
                    return_type: type_,
                }
            }
            module::ReturnExprKind::ListFunction { runtime_id, body } => {
                let runtime_id = id::list_function_function_id(runtime_id);
                let return_type = runtime_id.type_().clone();
                self.push_list_function_function(
                    runtime_id.clone(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::list_function_return(body),
                    ),
                );
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::List(runtime_id),
                    return_type,
                }
            }
            module::ReturnExprKind::FunctionFunction {
                runtime_id,
                type_,
                body,
            } => {
                let id = super::FunctionFunctionFunctionId(runtime_id.0);
                self.function_function_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::function_function_return(body),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Function(id),
                    return_type: type_,
                }
            }
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
    use super::super::{
        BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionFunctionId, BoolListFunctionId,
        ExecutionPlan, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionFunctionId,
        FloatListFunctionId, FunctionFunctionFunctionId, FunctionListFunctionFunctionId,
        FunctionListFunctionId, IntFunctionFunctionId, IntFunctionId, IntListFunctionFunctionId,
        IntListFunctionId, ListFunctionFunctionId, ListListFunctionFunctionId, ListListFunctionId,
        NilFunctionFunctionId, NilFunctionId, NilListFunctionFunctionId, NilListFunctionId,
        ReturnBody, ReturnBodyKind, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
        StringListFunctionFunctionId, StringListFunctionId, TupleFunctionFunctionId,
        TupleFunctionId, TupleListFunctionFunctionId, TupleListFunctionId,
    };
    use crate::plan::{FunctionType, SourceContext, ValueType};

    #[test]
    fn lowering_builds_every_typed_function_table() {
        let source = r#"
fn int_value() { 1 }
fn float_value() { 1.0 }
fn string_value() { "one" }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }

fn int_list() { [1] }
fn string_list() { ["one"] }
fn float_list() { [1.0] }
fn bool_list() { [True] }
fn nil_list() { [Nil] }
fn tuple_list() { [#(1)] }
fn list_list() { [[1]] }
fn function_list() { [int_value] }

fn int_function() { int_value }
fn float_function() { float_value }
fn string_function() { string_value }
fn bool_function() { bool_value }
fn nil_function() { nil_value }
fn tuple_function() { tuple_value }
fn int_list_function() { int_list }
fn string_list_function() { string_list }
fn float_list_function() { float_list }
fn bool_list_function() { bool_list }
fn nil_list_function() { nil_list }
fn tuple_list_function() { tuple_list }
fn list_list_function() { list_list }
fn function_list_function() { function_list }
fn function_function() { int_function }

pub fn main() { int_value() }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
        assert_eq!(plan.functions.int_functions.len(), 2);
        assert_eq!(plan.functions.float_functions.len(), 1);
        assert_eq!(plan.functions.string_functions.len(), 1);
        assert_eq!(plan.functions.bool_functions.len(), 1);
        assert_eq!(plan.functions.nil_functions.len(), 1);
        assert_eq!(plan.functions.tuple_functions.len(), 1);
        assert_eq!(plan.functions.int_list_functions.len(), 1);
        assert_eq!(plan.functions.string_list_functions.len(), 1);
        assert_eq!(plan.functions.float_list_functions.len(), 1);
        assert_eq!(plan.functions.bool_list_functions.len(), 1);
        assert_eq!(plan.functions.nil_list_functions.len(), 1);
        assert_eq!(plan.functions.tuple_list_functions.len(), 1);
        assert_eq!(plan.functions.list_list_functions.len(), 1);
        assert_eq!(plan.functions.function_list_functions.len(), 1);
        assert_eq!(plan.functions.int_function_functions.len(), 1);
        assert_eq!(plan.functions.float_function_functions.len(), 1);
        assert_eq!(plan.functions.string_function_functions.len(), 1);
        assert_eq!(plan.functions.bool_function_functions.len(), 1);
        assert_eq!(plan.functions.nil_function_functions.len(), 1);
        assert_eq!(plan.functions.tuple_function_functions.len(), 1);
        assert_eq!(plan.functions.int_list_function_functions.len(), 1);
        assert_eq!(plan.functions.string_list_function_functions.len(), 1);
        assert_eq!(plan.functions.float_list_function_functions.len(), 1);
        assert_eq!(plan.functions.bool_list_function_functions.len(), 1);
        assert_eq!(plan.functions.nil_list_function_functions.len(), 1);
        assert_eq!(plan.functions.tuple_list_function_functions.len(), 1);
        assert_eq!(plan.functions.list_list_function_functions.len(), 1);
        assert_eq!(plan.functions.function_list_function_functions.len(), 1);
        assert_eq!(plan.functions.function_function_functions.len(), 1);

        let _ = expect_expression_return(plan.int_function(IntFunctionId(1)).return_());
        let _ = expect_expression_return(plan.float_function(FloatFunctionId(0)).return_());
        let _ = expect_expression_return(plan.string_function(StringFunctionId(0)).return_());
        let _ = expect_expression_return(plan.bool_function(BoolFunctionId(0)).return_());
        let _ = expect_expression_return(plan.nil_function(NilFunctionId(0)).return_());
        let _ = expect_expression_return(plan.tuple_function(TupleFunctionId(0)).return_());
        let _ = expect_expression_return(plan.int_list_function(IntListFunctionId(0)).return_());
        let _ =
            expect_expression_return(plan.string_list_function(StringListFunctionId(0)).return_());
        let _ =
            expect_expression_return(plan.float_list_function(FloatListFunctionId(0)).return_());
        let _ = expect_expression_return(plan.bool_list_function(BoolListFunctionId(0)).return_());
        let _ = expect_expression_return(plan.nil_list_function(NilListFunctionId(0)).return_());
        let _ =
            expect_expression_return(plan.tuple_list_function(TupleListFunctionId(0)).return_());
        let _ = expect_expression_return(plan.list_list_function(ListListFunctionId(0)).return_());
        let _ = expect_expression_return(
            plan.function_list_function(FunctionListFunctionId(0))
                .return_(),
        );

        let plain_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let _ = expect_expression_return(
            plan.int_function_function(IntFunctionFunctionId(0))
                .return_(),
        );
        let _ = expect_expression_return(
            plan.float_function_function(FloatFunctionFunctionId(0))
                .return_(),
        );
        let _ = expect_expression_return(
            plan.string_function_function(StringFunctionFunctionId(0))
                .return_(),
        );
        let _ = expect_expression_return(
            plan.bool_function_function(BoolFunctionFunctionId(0))
                .return_(),
        );
        let _ = expect_expression_return(
            plan.nil_function_function(NilFunctionFunctionId(0))
                .return_(),
        );
        let _ = expect_expression_return(
            plan.tuple_function_function(TupleFunctionFunctionId(0))
                .return_(),
        );

        let list_function_types = [
            ListFunctionFunctionId::Int {
                id: IntListFunctionFunctionId(0),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
            },
            ListFunctionFunctionId::String {
                id: StringListFunctionFunctionId(0),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::String))),
            },
            ListFunctionFunctionId::Float {
                id: FloatListFunctionFunctionId(0),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Float))),
            },
            ListFunctionFunctionId::Bool {
                id: BoolListFunctionFunctionId(0),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Bool))),
            },
            ListFunctionFunctionId::Nil {
                id: NilListFunctionFunctionId(0),
                type_: FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Nil))),
            },
            ListFunctionFunctionId::Tuple {
                id: TupleListFunctionFunctionId(0),
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                ),
                item_type: vec![ValueType::Int],
            },
            ListFunctionFunctionId::List {
                id: ListListFunctionFunctionId(0),
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                ),
                item_type: Box::new(ValueType::Int),
            },
            ListFunctionFunctionId::Function {
                id: FunctionListFunctionFunctionId(0),
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::List(Box::new(ValueType::Function(Box::new(
                        plain_function_type.clone(),
                    )))),
                ),
                item_type: Box::new(plain_function_type.clone()),
            },
        ];
        for id in &list_function_types {
            let _ = expect_expression_return(plan.list_function_function(id).return_());
        }
        let _ = expect_expression_return(
            plan.function_function_function(FunctionFunctionFunctionId(0))
                .return_(),
        );
    }

    #[test]
    fn lowering_preserves_module_source_context_and_main_runtime() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan =
            crate::plan_module_with_source(typed, SourceContext::new("src/main.gleam", source))
                .expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.module().as_str(), "main");
        let source_context = plan.source_context().expect("source should be preserved");
        assert_eq!(source_context.path(), "src/main.gleam");
        assert_eq!(source_context.source(), source);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn expression_return_fixture_guard_rejects_case_return() {
        let source = r#"
pub fn main() {
  case True {
    True -> 1
    False -> 0
  }
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let _ = expect_expression_return(plan.int_function(IntFunctionId(0)).return_());
    }

    fn expect_expression_return<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }
}
