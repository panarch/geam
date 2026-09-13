use super::{CustomValueShape, ExternalTypeId, FunctionShape, ValueShapeId, ValueType};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub arguments: Table<ValueType>,
    pub return_: Node<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomFunctionType {
    pub type_: FunctionType,
    pub arguments: Table<ValueShapeId>,
    pub return_: CustomValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalFunctionType {
    pub type_: FunctionType,
    pub arguments: Table<ValueShapeId>,
    pub return_: ExternalTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericFunctionType {
    pub type_: FunctionType,
    pub shape: FunctionShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionFunctionType {
    pub type_: FunctionType,
    pub arguments: Table<ValueShapeId>,
    pub return_: FunctionShape,
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments: arguments.into(),
            return_: Box::new(return_).into(),
        }
    }

    pub(crate) fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }

    pub(crate) fn return_(&self) -> &ValueType {
        &self.return_
    }
}

impl Explain for FunctionType {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("fn(");
        for (index, argument) in self.argument_types().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            context.write(argument);
        }
        context.push_str(") -> ");
        context.write(self.return_());
    }
}

impl CustomFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: CustomValueShape,
    ) -> Self {
        Self {
            type_,
            arguments: arguments.into(),
            return_,
        }
    }
}

impl ExternalFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: ExternalTypeId,
    ) -> Self {
        Self {
            type_,
            arguments: arguments.into(),
            return_,
        }
    }
}

impl GenericFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        shape: FunctionShape,
    ) -> Self {
        Self { type_, shape }
    }
}

impl FunctionFunctionType {
    pub(in crate::plan::execution) fn from_shapes(
        type_: FunctionType,
        arguments: Vec<ValueShapeId>,
        return_: FunctionShape,
    ) -> Self {
        Self {
            type_,
            arguments: arguments.into(),
            return_,
        }
    }

    #[cfg(test)]
    pub(crate) fn argument_shapes(&self) -> &[ValueShapeId] {
        &self.arguments
    }

    #[cfg(test)]
    pub(crate) fn return_shape(&self) -> &FunctionShape {
        &self.return_
    }
}

impl Emit for FunctionType {
    fn emit(&self, output: &mut Rust) {
        let Self { arguments, return_ } = self;
        output.structure(
            "type_::FunctionType",
            &[("arguments", arguments), ("return_", return_)],
        );
    }
}

impl Emit for CustomFunctionType {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            arguments,
            return_,
        } = self;
        output.structure(
            "type_::CustomFunctionType",
            &[
                ("type_", type_),
                ("arguments", arguments),
                ("return_", return_),
            ],
        );
    }
}

impl Emit for ExternalFunctionType {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            arguments,
            return_,
        } = self;
        output.structure(
            "type_::ExternalFunctionType",
            &[
                ("type_", type_),
                ("arguments", arguments),
                ("return_", return_),
            ],
        );
    }
}

impl Emit for GenericFunctionType {
    fn emit(&self, output: &mut Rust) {
        let Self { type_, shape } = self;
        output.structure(
            "type_::GenericFunctionType",
            &[("type_", type_), ("shape", shape)],
        );
    }
}

impl Emit for FunctionFunctionType {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            arguments,
            return_,
        } = self;
        output.structure(
            "type_::FunctionFunctionType",
            &[
                ("type_", type_),
                ("arguments", arguments),
                ("return_", return_),
            ],
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use super::FunctionType;
    use crate::plan::execution::explain;
    use crate::plan::execution::type_::ValueType;

    #[test]
    fn writes_function_argument_and_return_types() {
        let source = "pub fn main() { 1 }";
        let type_ = FunctionType::new(
            vec![ValueType::Bool, ValueType::Int, ValueType::Bool],
            ValueType::Int,
        );
        let expected = "fn(Bool, Int, Bool) -> Int";

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&type_);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomFunctionType, ExternalFunctionType, FunctionFunctionType, FunctionType,
        GenericFunctionType,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalTypeId, FunctionShape,
        ValueShapeId, ValueType,
    };

    #[test]
    fn emits_function_types_with_borrowed_argument_and_return_nodes() {
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Bool);
        assert_eq!(
            Rust::expression(&type_),
            concat!(
                "data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Bool),}"
            )
        );

        static ARGUMENTS: [ValueType; 1] = [ValueType::Int];
        static RETURN: ValueType = ValueType::Bool;
        let borrowed = FunctionType {
            arguments: Table::Static(&ARGUMENTS),
            return_: Node::Static(&RETURN),
        };
        let retained = borrowed.clone();
        assert!(std::ptr::eq(
            retained.argument_types().as_ptr(),
            ARGUMENTS.as_ptr()
        ));
        assert!(std::ptr::eq(retained.return_(), &RETURN));
        assert_eq!(type_, borrowed);
        assert_eq!(
            format!("{borrowed:?}"),
            "FunctionType { arguments: [Int], return_: Bool }"
        );
    }

    #[test]
    fn emits_nominal_and_refined_function_return_contracts() {
        let custom = CustomFunctionType::from_shapes(
            FunctionType::new(vec![ValueType::Int], ValueType::Custom(CustomTypeId(2))),
            vec![ValueShapeId(3)],
            CustomValueShape::new(CustomTypeId(2), CustomValueShapeId(4)),
        );
        assert_eq!(
            Rust::expression(&custom),
            concat!(
                "data::type_::CustomFunctionType {type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2,),)),},",
                "arguments: data::Storage::Static(&[data::type_::ValueShapeId(3,),]),",
                "return_: data::type_::CustomValueShape {type_id: data::type_::CustomTypeId(2,),",
                "shape_id: data::type_::CustomValueShapeId(4,),},}"
            )
        );
        let external = ExternalFunctionType::from_shapes(
            FunctionType::new(
                vec![ValueType::Bool],
                ValueType::External(ExternalTypeId(2)),
            ),
            vec![ValueShapeId(3)],
            ExternalTypeId(2),
        );
        assert_eq!(
            Rust::expression(&external),
            concat!(
                "data::type_::ExternalFunctionType {type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Bool,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(2,),)),},",
                "arguments: data::Storage::Static(&[data::type_::ValueShapeId(3,),]),",
                "return_: data::type_::ExternalTypeId(2,),}"
            )
        );
        let identity = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let shape = FunctionShape::new(ValueShapeId(7), identity.clone());
        let generic = GenericFunctionType::from_shapes(identity.clone(), shape.clone());
        assert_eq!(
            Rust::expression(&generic),
            concat!(
                "data::type_::GenericFunctionType {type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Int),},",
                "shape: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(7,),",
                "type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Int),},},}"
            )
        );
        let function = FunctionFunctionType::from_shapes(
            FunctionType::new(vec![ValueType::Bool], ValueType::Function(identity)),
            vec![ValueShapeId(3)],
            shape,
        );
        assert_eq!(
            Rust::expression(&function),
            concat!(
                "data::type_::FunctionFunctionType {type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Bool,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Function(",
                "data::type_::FunctionType {arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Int),},)),},",
                "arguments: data::Storage::Static(&[data::type_::ValueShapeId(3,),]),",
                "return_: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(7,),",
                "type_: data::type_::FunctionType {",
                "arguments: data::Storage::Static(&[data::type_::ValueType::Int,]),",
                "return_: data::Storage::Static(&data::type_::ValueType::Int),},},}"
            )
        );
    }
}
