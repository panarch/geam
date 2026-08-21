use super::{CustomValueShape, ExternalType, ExternalValueShape, ValueShape};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Parameter(TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(Box<ValueType>),
    Function(Box<FunctionType>),
    Custom(CustomType),
    External(ExternalType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeParameterId(pub(crate) usize);

impl TypeParameterId {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomTypeName {
    package: EcoString,
    module: EcoString,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomType {
    name: Box<CustomTypeName>,
    arguments: Box<[ValueType]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionType {
    arguments: Vec<ValueShape>,
    return_: CustomValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ExternalFunctionType {
    arguments: Vec<ValueShape>,
    return_: ExternalValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionFunctionType {
    arguments: Box<[ValueShape]>,
    return_: Box<super::FunctionShape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenericFunctionType {
    arguments: Box<[ValueShape]>,
    return_: TypeParameterId,
}

impl GenericFunctionType {
    pub(crate) fn new(arguments: Vec<ValueShape>, return_: TypeParameterId) -> Self {
        Self {
            arguments: arguments.into_boxed_slice(),
            return_,
        }
    }

    pub(crate) fn return_parameter(&self) -> TypeParameterId {
        self.return_
    }

    pub(crate) fn shape(&self) -> super::FunctionShape {
        super::FunctionShape::new(self.arguments.to_vec(), ValueShape::Parameter(self.return_))
    }
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }

    pub fn return_(&self) -> &ValueType {
        &self.return_
    }
}

impl CustomFunctionType {
    #[cfg(test)]
    pub(crate) fn new(arguments: Vec<ValueType>, return_: CustomType) -> Self {
        Self::from_shapes(
            arguments
                .into_iter()
                .map(ValueShape::from_value_type)
                .collect(),
            CustomValueShape::any(return_),
        )
    }

    pub(crate) fn from_shapes(arguments: Vec<ValueShape>, return_: CustomValueShape) -> Self {
        Self { arguments, return_ }
    }

    pub(crate) fn argument_shapes(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn argument_types(&self) -> Vec<ValueType> {
        self.arguments.iter().map(ValueShape::value_type).collect()
    }

    pub(crate) fn return_(&self) -> &CustomValueShape {
        &self.return_
    }

    pub(crate) fn to_function_type(&self) -> FunctionType {
        FunctionType::new(
            self.argument_types(),
            ValueType::Custom(self.return_.type_().clone()),
        )
    }
}

impl ExternalFunctionType {
    pub(crate) fn from_shapes(arguments: Vec<ValueShape>, return_: ExternalValueShape) -> Self {
        Self { arguments, return_ }
    }

    pub(crate) fn argument_shapes(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn argument_types(&self) -> Vec<ValueType> {
        self.arguments.iter().map(ValueShape::value_type).collect()
    }

    pub(crate) fn return_(&self) -> &ExternalValueShape {
        &self.return_
    }

    pub(crate) fn to_function_type(&self) -> FunctionType {
        FunctionType::new(
            self.argument_types(),
            ValueType::External(self.return_.type_().clone()),
        )
    }
}

impl FunctionFunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: FunctionType) -> Self {
        Self::from_shapes(
            arguments
                .into_iter()
                .map(ValueShape::from_value_type)
                .collect(),
            super::FunctionShape::from_function_type(return_),
        )
    }

    pub(crate) fn from_shapes(arguments: Vec<ValueShape>, return_: super::FunctionShape) -> Self {
        Self {
            arguments: arguments.into_boxed_slice(),
            return_: Box::new(return_),
        }
    }

    pub(crate) fn argument_shapes(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn argument_types(&self) -> Vec<ValueType> {
        self.arguments.iter().map(ValueShape::value_type).collect()
    }

    pub(crate) fn return_shape(&self) -> &super::FunctionShape {
        &self.return_
    }

    pub(crate) fn return_type(&self) -> FunctionType {
        self.return_.type_()
    }

    pub(crate) fn to_function_type(&self) -> FunctionType {
        FunctionType::new(
            self.argument_types(),
            ValueType::Function(Box::new(self.return_type())),
        )
    }
}

impl CustomTypeName {
    pub(crate) fn new(package: EcoString, module: EcoString, name: EcoString) -> Self {
        Self {
            package,
            module,
            name,
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }
}

impl CustomType {
    pub(crate) fn new(name: CustomTypeName, arguments: Vec<ValueType>) -> Self {
        Self {
            name: Box::new(name),
            arguments: arguments.into_boxed_slice(),
        }
    }

    pub fn type_name(&self) -> &CustomTypeName {
        &self.name
    }

    pub fn arguments(&self) -> &[ValueType] {
        &self.arguments
    }
}
