use super::{CustomTypeId, ExternalTypeId, FunctionType};

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
    Parameter(crate::plan::TypeParameterId),
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
    External(ExternalTypeId),
}

pub(crate) struct ValueShapeTable {
    // The runtime trusts lowered refinements, but the execution IR keeps their canonical graph.
    #[cfg_attr(not(test), allow(dead_code))]
    shapes: Vec<ValueShapeDescriptor>,
    shape_types: Vec<super::ValueType>,
    #[cfg_attr(not(test), allow(dead_code))]
    custom_shapes: Vec<CustomValueShapeDescriptor>,
}

impl ValueShapeId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl CustomValueShapeId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    #[cfg(test)]
    fn index(self) -> usize {
        self.0
    }
}

impl CustomValueShape {
    pub(in crate::plan::execution) fn new(
        type_id: CustomTypeId,
        shape_id: CustomValueShapeId,
    ) -> Self {
        Self { type_id, shape_id }
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn shape_id(self) -> CustomValueShapeId {
        self.shape_id
    }
}

impl FunctionShape {
    pub(in crate::plan::execution) fn new(shape_id: ValueShapeId, type_: FunctionType) -> Self {
        Self { shape_id, type_ }
    }

    #[cfg(test)]
    pub(crate) fn shape_id(&self) -> ValueShapeId {
        self.shape_id
    }
}

impl ValueShapeTable {
    pub(in crate::plan::execution) fn new(
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

    pub(crate) fn value_type(&self, id: ValueShapeId) -> &super::ValueType {
        &self.shape_types[id.index()]
    }

    #[cfg(test)]
    pub(crate) fn get(&self, id: ValueShapeId) -> &ValueShapeDescriptor {
        &self.shapes[id.index()]
    }

    #[cfg(test)]
    pub(crate) fn custom(&self, id: CustomValueShapeId) -> &CustomValueShapeDescriptor {
        &self.custom_shapes[id.index()]
    }
}

impl CustomValueShapeDescriptor {
    pub(in crate::plan::execution) fn new(
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
    use crate::plan::execution::ExecutionPlan;

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
  let _ = identity
  panic
}
"#,
        );
        let shape = plan
            .custom_function(plan.custom_function_id(1))
            .body()
            .signature_shape();

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

    #[test]
    fn execution_shapes_materialize_unresolved_phantom_parameters() {
        let plan = execution_plan(
            r#"
pub type Phantom(value) { Phantom }
pub fn main() { Phantom }
"#,
        );
        let shape_id = plan
            .custom_function(plan.custom_function_id(0))
            .body()
            .body_shape()
            .shape_id();
        let shape = super::CustomValueShape::new(
            plan.program.common.value_shapes.custom(shape_id).type_id(),
            shape_id,
        );

        assert_eq!(
            plan.custom_shape_value_type(&shape),
            crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Phantom".into(),),
                vec![crate::plan::ValueType::Parameter(
                    crate::plan::TypeParameterId(0),
                )],
            ),
        );
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module)
    }
}
