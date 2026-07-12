mod expression;
mod frame;
mod id;
mod param;
mod return_;
mod step;
mod value_type;

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

#[derive(Default)]
struct LoweringContext {
    list_types: value_type::ListTypeInterner,
}

impl LoweringContext {
    fn value_type(&mut self, type_: crate::plan::ValueType) -> super::ValueType {
        self.list_types.value_type(type_)
    }

    fn function_type(&mut self, type_: crate::plan::FunctionType) -> super::FunctionType {
        self.list_types.function_type(type_)
    }

    fn int_list_type(&mut self) -> super::IntListTypeId {
        self.list_types.int_list_type()
    }

    fn string_list_type(&mut self) -> super::StringListTypeId {
        self.list_types.string_list_type()
    }

    fn float_list_type(&mut self) -> super::FloatListTypeId {
        self.list_types.float_list_type()
    }

    fn bool_list_type(&mut self) -> super::BoolListTypeId {
        self.list_types.bool_list_type()
    }

    fn nil_list_type(&mut self) -> super::NilListTypeId {
        self.list_types.nil_list_type()
    }

    fn tuple_list_type(&mut self, item: Vec<crate::plan::ValueType>) -> super::TupleListTypeId {
        self.list_types.tuple_list_type(item)
    }

    fn list_list_type(&mut self, item: crate::plan::ValueType) -> super::ListListTypeId {
        self.list_types.list_list_type(item)
    }

    fn function_list_type(&mut self, item: crate::plan::FunctionType) -> super::FunctionListTypeId {
        self.list_types.function_list_type(item)
    }

    fn finish(self) -> super::ListTypeTable {
        self.list_types.into_table()
    }
}

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    let mut context = LoweringContext::default();
    let mut tables = FunctionTableBuilder::default();
    let main = tables.push(parts.main, &mut context);

    for function in parts.functions {
        tables.push(function, &mut context);
    }
    for function in parts.anonymous_functions {
        tables.push(function, &mut context);
    }

    ExecutionPlan {
        module: parts.module,
        source_context: parts.source_context,
        main,
        functions: tables.finish(),
        list_types: context.finish(),
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
    int_list_functions: Vec<(super::IntListFunctionId, ExecutableFunction<IntListReturn>)>,
    string_list_functions: Vec<(
        super::StringListFunctionId,
        ExecutableFunction<StringListReturn>,
    )>,
    float_list_functions: Vec<(
        super::FloatListFunctionId,
        ExecutableFunction<FloatListReturn>,
    )>,
    bool_list_functions: Vec<(
        super::BoolListFunctionId,
        ExecutableFunction<BoolListReturn>,
    )>,
    nil_list_functions: Vec<(super::NilListFunctionId, ExecutableFunction<NilListReturn>)>,
    tuple_list_functions: Vec<(
        super::TupleListFunctionId,
        ExecutableFunction<TupleListReturn>,
    )>,
    list_list_functions: Vec<(
        super::ListListFunctionId,
        ExecutableFunction<ListListReturn>,
    )>,
    function_list_functions: Vec<(
        super::FunctionListFunctionId,
        ExecutableFunction<FunctionListReturn>,
    )>,
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
    fn push(
        &mut self,
        function: module::FunctionPlan,
        context: &mut LoweringContext,
    ) -> RuntimeFunctionId {
        let module::FunctionExecutionParts {
            frame_layout,
            steps,
            return_,
        } = function.into_execution_parts();
        let frame_layout = frame::frame_layout(frame_layout, context);
        let steps = step::steps(steps, context);

        match return_.into_kind() {
            module::ReturnExprKind::Int { runtime_id, body } => {
                let id = super::IntFunctionId(runtime_id.0);
                self.int_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::int_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Int(id)
            }
            module::ReturnExprKind::Float { runtime_id, body } => {
                let id = super::FloatFunctionId(runtime_id.0);
                self.float_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::float_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Float(id)
            }
            module::ReturnExprKind::String { runtime_id, body } => {
                let id = super::StringFunctionId(runtime_id.0);
                self.string_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::string_return(body, context),
                    ),
                ));
                RuntimeFunctionId::String(id)
            }
            module::ReturnExprKind::Bool { runtime_id, body } => {
                let id = super::BoolFunctionId(runtime_id.0);
                self.bool_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::bool_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Bool(id)
            }
            module::ReturnExprKind::Nil { runtime_id, body } => {
                let id = super::NilFunctionId(runtime_id.0);
                self.nil_functions.push((
                    runtime_id.0,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::nil_return(body, context),
                    ),
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
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::tuple_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Tuple {
                    id,
                    return_type: type_
                        .into_iter()
                        .map(|type_| context.value_type(type_))
                        .collect(),
                }
            }
            module::ReturnExprKind::IntList { runtime_id, body } => {
                let id = super::IntListFunctionId::new(runtime_id.0, context.int_list_type());
                self.int_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::int_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Int(id))
            }
            module::ReturnExprKind::StringList { runtime_id, body } => {
                let id = super::StringListFunctionId::new(runtime_id.0, context.string_list_type());
                self.string_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::string_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::String(id))
            }
            module::ReturnExprKind::FloatList { runtime_id, body } => {
                let id = super::FloatListFunctionId::new(runtime_id.0, context.float_list_type());
                self.float_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::float_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Float(id))
            }
            module::ReturnExprKind::BoolList { runtime_id, body } => {
                let id = super::BoolListFunctionId::new(runtime_id.0, context.bool_list_type());
                self.bool_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::bool_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Bool(id))
            }
            module::ReturnExprKind::NilList { runtime_id, body } => {
                let id = super::NilListFunctionId::new(runtime_id.0, context.nil_list_type());
                self.nil_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::nil_list_return(body, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Nil(id))
            }
            module::ReturnExprKind::TupleList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.tuple_list_type(item_type);
                let id = super::TupleListFunctionId::new(runtime_id.0, type_id);
                self.tuple_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::tuple_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Tuple(id))
            }
            module::ReturnExprKind::ListList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.list_list_type(*item_type);
                let id = super::ListListFunctionId::new(runtime_id.0, type_id);
                self.list_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::list_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::List(id))
            }
            module::ReturnExprKind::FunctionList {
                runtime_id,
                item_type,
                body,
            } => {
                let type_id = context.function_list_type(item_type);
                let id = super::FunctionListFunctionId::new(runtime_id.0, type_id);
                self.function_list_functions.push((
                    id,
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::function_list_return(body, type_id, context),
                    ),
                ));
                RuntimeFunctionId::List(super::ListFunctionId::Function(id))
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
                        return_::int_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Int(id),
                    return_type: context.function_type(type_),
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
                        return_::float_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Float(id),
                    return_type: context.function_type(type_),
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
                        return_::string_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::String(id),
                    return_type: context.function_type(type_),
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
                        return_::bool_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Bool(id),
                    return_type: context.function_type(type_),
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
                        return_::nil_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Nil(id),
                    return_type: context.function_type(type_),
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
                        return_::tuple_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Tuple(id),
                    return_type: context.function_type(type_),
                }
            }
            module::ReturnExprKind::ListFunction { runtime_id, body } => {
                let runtime_id = id::list_function_function_id(runtime_id, context);
                let return_type = runtime_id.type_().clone();
                self.push_list_function_function(
                    runtime_id.clone(),
                    ExecutableFunction::new(
                        frame_layout,
                        steps,
                        return_::list_function_return(body, context),
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
                        return_::function_function_return(body, context),
                    ),
                ));
                RuntimeFunctionId::Function {
                    id: super::FunctionFunctionId::Function(id),
                    return_type: context.function_type(type_),
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
            int_list_functions: sort_list_functions(self.int_list_functions, |id| id.index()),
            string_list_functions: sort_list_functions(self.string_list_functions, |id| id.index()),
            float_list_functions: sort_list_functions(self.float_list_functions, |id| id.index()),
            bool_list_functions: sort_list_functions(self.bool_list_functions, |id| id.index()),
            nil_list_functions: sort_list_functions(self.nil_list_functions, |id| id.index()),
            tuple_list_functions: sort_list_functions(self.tuple_list_functions, |id| id.index()),
            list_list_functions: sort_list_functions(self.list_list_functions, |id| id.index()),
            function_list_functions: sort_list_functions(self.function_list_functions, |id| {
                id.index()
            }),
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

fn sort_list_functions<Id, Return>(
    mut functions: Vec<(Id, ExecutableFunction<Return>)>,
    index: impl Fn(&Id) -> usize,
) -> Vec<(Id, ExecutableFunction<Return>)> {
    functions.sort_by_key(|(id, _)| index(id));
    functions
}

#[cfg(test)]
mod tests {
    use super::super::{ExecutionPlan, IntFunctionId, RuntimeFunctionId};
    use crate::plan::SourceContext;

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
}
