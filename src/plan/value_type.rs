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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomFunctionType {
    arguments: Vec<ValueType>,
    return_: CustomType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionFunctionType {
    arguments: Vec<ValueType>,
    return_: Box<FunctionType>,
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

impl CustomFunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: CustomType) -> Self {
        Self { arguments, return_ }
    }

    pub(crate) fn return_(&self) -> &CustomType {
        &self.return_
    }

    pub(crate) fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }

    pub(crate) fn to_function_type(&self) -> FunctionType {
        FunctionType::new(
            self.arguments.clone(),
            ValueType::Custom(self.return_.clone()),
        )
    }
}

impl FunctionFunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: FunctionType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub(crate) fn return_(&self) -> &FunctionType {
        &self.return_
    }

    pub(crate) fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }

    pub(crate) fn to_function_type(&self) -> FunctionType {
        FunctionType::new(
            self.arguments.clone(),
            ValueType::Function(self.return_.clone()),
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
