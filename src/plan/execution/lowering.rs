mod expression;
mod frame;
mod id;
mod param;
mod pattern;
mod return_;
mod step;
mod table;
mod value_type;

use super::ExecutionPlan;
use super::custom_type::CustomTypeTable;
use super::value_shape::ValueShapeTable;
use super::value_type::ListTypeTable;
use crate::plan::ModulePlan;

struct LoweringContext {
    types: value_type::TypeInterner,
}

impl LoweringContext {
    fn new() -> Self {
        Self {
            types: value_type::TypeInterner::new(),
        }
    }

    fn value_type(&mut self, type_: crate::plan::ValueType) -> super::ValueType {
        self.types.value_type(type_)
    }

    fn custom_value_shape(
        &mut self,
        shape: crate::plan::CustomValueShape,
    ) -> super::CustomValueShape {
        self.types.custom_value_shape(shape)
    }

    fn value_shape(&mut self, shape: crate::plan::ValueShape) -> super::ValueShapeId {
        self.types.value_shape(shape)
    }

    fn function_shape(&mut self, shape: crate::plan::FunctionShape) -> super::FunctionShape {
        self.types.function_shape(shape)
    }

    fn function_type(&mut self, type_: crate::plan::FunctionType) -> super::FunctionType {
        self.types.function_type(type_)
    }

    fn custom_function_type(
        &mut self,
        type_: crate::plan::CustomFunctionType,
    ) -> super::CustomFunctionType {
        self.types.custom_function_type(type_)
    }

    fn function_function_type(
        &mut self,
        type_: crate::plan::FunctionFunctionType,
    ) -> super::FunctionFunctionType {
        self.types.function_function_type(type_)
    }

    fn int_list_type(&mut self) -> super::IntListTypeId {
        self.types.int_list_type()
    }

    fn string_list_type(&mut self) -> super::StringListTypeId {
        self.types.string_list_type()
    }

    fn bit_array_list_type(&mut self) -> super::BitArrayListTypeId {
        self.types.bit_array_list_type()
    }

    fn utf_codepoint_list_type(&mut self) -> super::UtfCodepointListTypeId {
        self.types.utf_codepoint_list_type()
    }

    fn custom_constructor(
        &mut self,
        constructor: crate::plan::CustomConstructor,
    ) -> super::CustomConstructorId {
        self.types.custom_constructor(constructor)
    }

    fn custom_list_type(&mut self, item: crate::plan::CustomType) -> super::CustomListTypeId {
        self.types.custom_list_type(item)
    }

    fn float_list_type(&mut self) -> super::FloatListTypeId {
        self.types.float_list_type()
    }

    fn bool_list_type(&mut self) -> super::BoolListTypeId {
        self.types.bool_list_type()
    }

    fn nil_list_type(&mut self) -> super::NilListTypeId {
        self.types.nil_list_type()
    }

    fn tuple_list_type(&mut self, item: Vec<crate::plan::ValueType>) -> super::TupleListTypeId {
        self.types.tuple_list_type(item)
    }

    fn list_list_type(&mut self, item: crate::plan::ValueType) -> super::ListListTypeId {
        self.types.list_list_type(item)
    }

    fn function_list_type(&mut self, item: crate::plan::FunctionType) -> super::FunctionListTypeId {
        self.types.function_list_type(item)
    }

    fn into_tables(self) -> (ListTypeTable, CustomTypeTable, ValueShapeTable) {
        self.types.into_tables()
    }
}

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    drop(parts.custom_types);
    let mut context = LoweringContext::new();
    let mut tables = table::FunctionTableBuilder::default();
    let main = tables.push(parts.main, &mut context);

    for function in parts.functions {
        tables.push(function, &mut context);
    }
    for function in parts.anonymous_functions {
        tables.push(function, &mut context);
    }

    let (list_types, custom_types, value_shapes) = context.into_tables();
    ExecutionPlan {
        module: parts.module,
        source_context: parts.source_context,
        main,
        functions: tables.finish(),
        list_types,
        custom_types,
        value_shapes,
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
