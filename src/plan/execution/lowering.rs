mod expression;
mod frame;
mod id;
mod param;
mod return_;
mod step;
mod table;
mod value_type;

use super::ExecutionPlan;
use super::value_type::ListTypeTable;
use crate::plan::ModulePlan;

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

    fn into_list_type_table(self) -> ListTypeTable {
        self.list_types.into_table()
    }
}

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    let mut context = LoweringContext::default();
    let mut tables = table::FunctionTableBuilder::default();
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
        list_types: context.into_list_type_table(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ExecutionPlan, IntFunctionId, RuntimeFunctionId};
    use crate::plan::SourceContext;

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
