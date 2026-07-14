use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
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

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub fn return_(&self) -> &ValueType {
        &self.return_
    }

    pub fn argument_types(&self) -> &[ValueType] {
        &self.arguments
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
