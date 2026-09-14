use super::{CustomTypeId, ExternalTypeId, FunctionType};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueShapeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomValueShapeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CustomConstructorRefinement {
    Any,
    Exact(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomValueShape {
    pub type_id: CustomTypeId,
    pub shape_id: CustomValueShapeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionShape {
    pub shape_id: ValueShapeId,
    pub type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomValueShapeDescriptor {
    pub type_id: CustomTypeId,
    pub arguments: Table<ValueShapeId>,
    pub constructor: CustomConstructorRefinement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueShapeDescriptor {
    Parameter(crate::plan::TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Table<ValueShapeId>),
    List(ValueShapeId),
    Function {
        arguments: Table<ValueShapeId>,
        return_: ValueShapeId,
    },
    Custom(CustomValueShapeId),
    External(ExternalTypeId),
}

pub struct ValueShapeTable {
    // The runtime trusts lowered refinements, but the execution IR keeps their canonical graph.
    pub shapes: Table<ValueShapeDescriptor>,
    pub shape_types: Table<super::ValueType>,
    pub custom_shapes: Table<CustomValueShapeDescriptor>,
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
            shapes: shapes.into(),
            shape_types: shape_types.into(),
            custom_shapes: custom_shapes.into(),
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
            arguments: arguments.into(),
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

impl Emit for ValueShapeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::ValueShapeId", &[field_0]);
    }
}

impl Emit for CustomValueShapeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::CustomValueShapeId", &[field_0]);
    }
}

impl Emit for CustomConstructorRefinement {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Any => output.path("type_::CustomConstructorRefinement::Any"),
            Self::Exact(field_0) => {
                output.call("type_::CustomConstructorRefinement::Exact", &[field_0])
            }
        }
    }
}

impl Emit for CustomValueShape {
    fn emit(&self, output: &mut Rust) {
        let Self { type_id, shape_id } = self;
        output.structure(
            "type_::CustomValueShape",
            &[("type_id", type_id), ("shape_id", shape_id)],
        );
    }
}

impl Emit for FunctionShape {
    fn emit(&self, output: &mut Rust) {
        let Self { shape_id, type_ } = self;
        output.structure(
            "type_::FunctionShape",
            &[("shape_id", shape_id), ("type_", type_)],
        );
    }
}

impl Emit for CustomValueShapeDescriptor {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_id,
            arguments,
            constructor,
        } = self;
        output.structure(
            "type_::CustomValueShapeDescriptor",
            &[
                ("type_id", type_id),
                ("arguments", arguments),
                ("constructor", constructor),
            ],
        );
    }
}

impl Emit for ValueShapeDescriptor {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(field_0) => {
                output.call("type_::ValueShapeDescriptor::Parameter", &[field_0])
            }
            Self::Int => output.path("type_::ValueShapeDescriptor::Int"),
            Self::Float => output.path("type_::ValueShapeDescriptor::Float"),
            Self::String => output.path("type_::ValueShapeDescriptor::String"),
            Self::BitArray => output.path("type_::ValueShapeDescriptor::BitArray"),
            Self::UtfCodepoint => output.path("type_::ValueShapeDescriptor::UtfCodepoint"),
            Self::Bool => output.path("type_::ValueShapeDescriptor::Bool"),
            Self::Nil => output.path("type_::ValueShapeDescriptor::Nil"),
            Self::Tuple(field_0) => output.call("type_::ValueShapeDescriptor::Tuple", &[field_0]),
            Self::List(field_0) => output.call("type_::ValueShapeDescriptor::List", &[field_0]),
            Self::Function { arguments, return_ } => output.structure(
                "type_::ValueShapeDescriptor::Function",
                &[("arguments", arguments), ("return_", return_)],
            ),
            Self::Custom(field_0) => output.call("type_::ValueShapeDescriptor::Custom", &[field_0]),
            Self::External(field_0) => {
                output.call("type_::ValueShapeDescriptor::External", &[field_0])
            }
        }
    }
}

impl Emit for ValueShapeTable {
    fn emit(&self, output: &mut Rust) {
        let Self {
            shapes,
            shape_types,
            custom_shapes,
        } = self;
        output.structure(
            "type_::ValueShapeTable",
            &[
                ("shapes", shapes),
                ("shape_types", shape_types),
                ("custom_shapes", custom_shapes),
            ],
        );
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
