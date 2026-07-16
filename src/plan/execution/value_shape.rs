use super::{CustomTypeId, FunctionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ValueShapeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomValueShapeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CustomConstructorRefinement {
    Any,
    Exact(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomValueShape {
    type_id: CustomTypeId,
    shape_id: CustomValueShapeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionShape {
    shape_id: ValueShapeId,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomValueShapeDescriptor {
    type_id: CustomTypeId,
    arguments: Box<[ValueShapeId]>,
    constructor: CustomConstructorRefinement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ValueShapeDescriptor {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[ValueShapeId]>),
    List(ValueShapeId),
    Function {
        arguments: Box<[ValueShapeId]>,
        return_: ValueShapeId,
    },
    Custom(CustomValueShapeId),
}

pub(super) struct ValueShapeTable {
    // The runtime trusts lowered refinements, but the execution IR keeps their canonical graph.
    #[cfg_attr(not(test), allow(dead_code))]
    shapes: Vec<ValueShapeDescriptor>,
    shape_types: Vec<super::ValueType>,
    #[cfg_attr(not(test), allow(dead_code))]
    custom_shapes: Vec<CustomValueShapeDescriptor>,
}

impl ValueShapeId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl CustomValueShapeId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    fn index(self) -> usize {
        self.0
    }
}

impl CustomValueShape {
    pub(super) fn new(type_id: CustomTypeId, shape_id: CustomValueShapeId) -> Self {
        Self { type_id, shape_id }
    }

    pub(crate) fn type_id(self) -> CustomTypeId {
        self.type_id
    }

    #[cfg(test)]
    pub(super) fn shape_id(self) -> CustomValueShapeId {
        self.shape_id
    }
}

impl FunctionShape {
    pub(super) fn new(shape_id: ValueShapeId, type_: FunctionType) -> Self {
        Self { shape_id, type_ }
    }

    #[cfg(test)]
    pub(crate) fn shape_id(&self) -> ValueShapeId {
        self.shape_id
    }

    pub(crate) fn type_(&self) -> &FunctionType {
        &self.type_
    }
}

impl ValueShapeTable {
    pub(super) fn new(
        shapes: Vec<ValueShapeDescriptor>,
        shape_types: Vec<super::ValueType>,
        custom_shapes: Vec<CustomValueShapeDescriptor>,
    ) -> Self {
        Self {
            shapes,
            shape_types,
            custom_shapes,
        }
    }

    #[cfg(test)]
    pub(crate) fn get(&self, id: ValueShapeId) -> &ValueShapeDescriptor {
        &self.shapes[id.index()]
    }

    pub(crate) fn value_type(&self, id: ValueShapeId) -> &super::ValueType {
        &self.shape_types[id.index()]
    }

    #[cfg(test)]
    pub(crate) fn custom(&self, id: CustomValueShapeId) -> &CustomValueShapeDescriptor {
        &self.custom_shapes[id.index()]
    }
}

impl CustomValueShapeDescriptor {
    pub(super) fn new(
        type_id: CustomTypeId,
        arguments: Box<[ValueShapeId]>,
        constructor: CustomConstructorRefinement,
    ) -> Self {
        Self {
            type_id,
            arguments,
            constructor,
        }
    }

    #[cfg(test)]
    pub(crate) fn type_id(&self) -> CustomTypeId {
        self.type_id
    }

    #[cfg(test)]
    pub(crate) fn arguments(&self) -> &[ValueShapeId] {
        &self.arguments
    }

    #[cfg(test)]
    pub(crate) fn constructor(&self) -> CustomConstructorRefinement {
        self.constructor
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomConstructorRefinement, CustomValueShapeDescriptor, CustomValueShapeId,
        ValueShapeDescriptor, ValueShapeId, ValueShapeTable,
    };
    use crate::plan::execution::{CustomTypeId, ExecutionPlan, RuntimeFunctionId};

    #[test]
    fn lowering_preserves_exact_and_widened_custom_shapes() {
        let exact = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub fn main() { First(1) }
"#,
        );
        let widened = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub fn main() {
  let flag = True
  case flag { True -> First(1) False -> Second(2) }
}
"#,
        );

        assert_custom_constructor(&exact, main_custom_shape(&exact), Some(0), "exact main");
        assert_custom_constructor(&widened, main_custom_shape(&widened), None, "widened main");
    }

    #[test]
    fn lowering_preserves_nested_shapes_without_splitting_nominal_list_storage() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrapper(a) { Wrapper(value: a) }

pub fn main() {
  let flag = True
  Wrapper(#(
    Wrapper(First(1)),
    [First(2)],
    case flag { True -> [First(3)] False -> [Second(4)] },
    fn() { First(5) },
  ))
}
"#,
        );
        assert_eq!(
            plan.value_shapes.shapes,
            vec![
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(0)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(1)),
                ValueShapeDescriptor::List(ValueShapeId::new(0)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(2)),
                ValueShapeDescriptor::List(ValueShapeId::new(3)),
                ValueShapeDescriptor::Function {
                    arguments: Vec::new().into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Tuple(
                    vec![
                        ValueShapeId::new(1),
                        ValueShapeId::new(2),
                        ValueShapeId::new(4),
                        ValueShapeId::new(5),
                    ]
                    .into_boxed_slice(),
                ),
            ],
        );
        assert_eq!(
            plan.value_shapes.custom_shapes,
            vec![
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    vec![ValueShapeId::new(0)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    vec![ValueShapeId::new(6)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(1),
                ),
            ],
        );
    }

    #[test]
    fn lowering_preserves_refinements_through_projections_calls_and_captures() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrapper(a) { Wrapper(value: a, label: String) }
pub type Factory(a) { Factory(make: fn() -> a, label: String) }

fn direct() { First(3) }

pub fn main() {
  let tuple_value = #(First(1)).0
  let record_value = Wrapper(First(2), "record").value
  let captured = First(4)
  let closure = fn() { captured }
  let constructor = First
  let captured_constructor = fn() { constructor(6) }
  let factory = Factory(fn() { First(5) }, "factory")
  Wrapper(#(
    tuple_value,
    record_value,
    direct(),
    closure(),
    factory.make(),
    captured_constructor(),
  ), "result")
}
"#,
        );
        assert_eq!(
            plan.value_shapes.shapes,
            vec![
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(0)),
                ValueShapeDescriptor::Int,
                ValueShapeDescriptor::Function {
                    arguments: Vec::new().into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Function {
                    arguments: vec![ValueShapeId::new(1)].into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(3)),
                ValueShapeDescriptor::Tuple(
                    vec![
                        ValueShapeId::new(0),
                        ValueShapeId::new(0),
                        ValueShapeId::new(4),
                        ValueShapeId::new(0),
                        ValueShapeId::new(0),
                        ValueShapeId::new(0),
                    ]
                    .into_boxed_slice(),
                ),
            ],
        );
        assert_eq!(
            plan.value_shapes.custom_shapes,
            vec![
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    vec![ValueShapeId::new(0)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    vec![ValueShapeId::new(0)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(3),
                    vec![ValueShapeId::new(5)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
            ],
        );
    }

    #[test]
    fn lowering_preserves_refinements_through_function_returning_function_types() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Boxed(a) { Boxed(value: a) }

pub fn main() { Boxed(fn() { fn() { First(1) } }).value }
"#,
        );
        let main = plan.function_function_function_id(0);
        let type_ = main.type_();
        assert_eq!(type_.argument_shapes(), []);
        let returned = type_.return_shape();
        let (arguments, return_) = function_shape(&plan.value_shapes, returned.shape_id());
        assert_eq!(arguments, &[]);
        let custom = custom_shape(&plan.value_shapes, return_);
        assert_eq!(
            plan.value_shapes.custom(custom).constructor(),
            CustomConstructorRefinement::Exact(0),
        );
    }

    #[test]
    #[should_panic(expected = "expected a function value shape")]
    fn function_shape_fixture_guard_rejects_int_shape() {
        let table = ValueShapeTable::new(
            vec![ValueShapeDescriptor::Int],
            vec![crate::plan::execution::ValueType::Int],
            Vec::new(),
        );

        let _ = function_shape(&table, ValueShapeId::new(0));
    }

    #[test]
    #[should_panic(expected = "expected a custom value shape")]
    fn custom_shape_fixture_guard_rejects_int_shape() {
        let table = ValueShapeTable::new(
            vec![ValueShapeDescriptor::Int],
            vec![crate::plan::execution::ValueType::Int],
            Vec::new(),
        );

        let _ = custom_shape(&table, ValueShapeId::new(0));
    }

    #[test]
    fn execution_shapes_materialize_every_recursive_value_family() {
        let plan = execution_plan(
            r#"
pub type Choice { First Second }
pub type Wrapper(a) { Wrapper(value: a) }

fn identity(value: Wrapper(#(
  Int,
  Float,
  String,
  BitArray,
  UtfCodepoint,
  Bool,
  Nil,
  #(Int),
  List(Int),
  fn(Int) -> String,
  Choice,
))) {
  let _ = value.value
  value
}

pub fn main() -> Wrapper(#(
  Int,
  Float,
  String,
  BitArray,
  UtfCodepoint,
  Bool,
  Nil,
  #(Int),
  List(Int),
  fn(Int) -> String,
  Choice,
)) {
  panic
}
"#,
        );
        let shape = plan
            .custom_function(plan.custom_function_id(1))
            .return_()
            .shape();

        assert_eq!(
            plan.custom_shape_refinement(shape),
            super::CustomConstructorRefinement::Any,
        );
        assert_eq!(
            plan.custom_shape_value_type(shape),
            crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Wrapper".into(),),
                vec![crate::plan::ValueType::Tuple(vec![
                    crate::plan::ValueType::Int,
                    crate::plan::ValueType::Float,
                    crate::plan::ValueType::String,
                    crate::plan::ValueType::BitArray,
                    crate::plan::ValueType::UtfCodepoint,
                    crate::plan::ValueType::Bool,
                    crate::plan::ValueType::Nil,
                    crate::plan::ValueType::Tuple(vec![crate::plan::ValueType::Int]),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                    crate::plan::ValueType::Function(Box::new(crate::plan::FunctionType::new(
                        vec![crate::plan::ValueType::Int],
                        crate::plan::ValueType::String,
                    ))),
                    crate::plan::ValueType::Custom(crate::plan::CustomType::new(
                        crate::plan::CustomTypeName::new(
                            "geam".into(),
                            "main".into(),
                            "Choice".into(),
                        ),
                        Vec::new(),
                    )),
                ])],
            ),
        );
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module)
    }

    fn main_custom_shape(plan: &ExecutionPlan) -> super::CustomValueShapeId {
        let id = plan.custom_function_id(0);
        assert_eq!(plan.main_runtime(), RuntimeFunctionId::Custom(id));
        plan.custom_function(id).return_().shape().shape_id()
    }

    fn function_shape(
        table: &ValueShapeTable,
        shape: ValueShapeId,
    ) -> (&[ValueShapeId], ValueShapeId) {
        match table.get(shape) {
            ValueShapeDescriptor::Function { arguments, return_ } => (arguments, *return_),
            _ => panic!("expected a function value shape"),
        }
    }

    fn custom_shape(table: &ValueShapeTable, shape: ValueShapeId) -> CustomValueShapeId {
        match table.get(shape) {
            ValueShapeDescriptor::Custom(custom) => *custom,
            _ => panic!("expected a custom value shape"),
        }
    }

    fn assert_custom_constructor(
        plan: &ExecutionPlan,
        shape: super::CustomValueShapeId,
        expected: Option<usize>,
        context: &str,
    ) {
        let constructor = plan.value_shapes.custom(shape).constructor();
        assert_eq!(
            constructor,
            expected.map_or(
                CustomConstructorRefinement::Any,
                CustomConstructorRefinement::Exact,
            ),
            "{context}",
        );
    }
}
