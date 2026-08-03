use crate::plan::{TypeParameterId, ValueShape, ValueType};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalTypeDefinition {
    name: ExternalTypeName,
    parameters: Box<[TypeParameterId]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalType {
    name: Box<ExternalTypeName>,
    arguments: Box<[ValueType]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalTypeName {
    package: EcoString,
    module: EcoString,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ExternalValueShape {
    type_: ExternalType,
    arguments: Box<[ValueShape]>,
}

impl ExternalTypeDefinition {
    pub(crate) fn new(name: ExternalTypeName, parameter_count: usize) -> Self {
        Self {
            name,
            parameters: (0..parameter_count)
                .map(TypeParameterId)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn name(&self) -> &ExternalTypeName {
        &self.name
    }

    pub fn parameters(&self) -> &[TypeParameterId] {
        &self.parameters
    }
}

impl ExternalType {
    pub(crate) fn new(name: ExternalTypeName, arguments: Vec<ValueType>) -> Self {
        Self {
            name: Box::new(name),
            arguments: arguments.into_boxed_slice(),
        }
    }

    pub fn type_name(&self) -> &ExternalTypeName {
        &self.name
    }

    pub fn arguments(&self) -> &[ValueType] {
        &self.arguments
    }
}

impl ExternalTypeName {
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

impl ExternalValueShape {
    pub(crate) fn new(name: ExternalTypeName, arguments: Vec<ValueShape>) -> Self {
        Self {
            type_: ExternalType::new(name, arguments.iter().map(ValueShape::value_type).collect()),
            arguments: arguments.into_boxed_slice(),
        }
    }

    pub(crate) fn any(type_: ExternalType) -> Self {
        Self::new(
            type_.type_name().clone(),
            type_
                .arguments()
                .iter()
                .cloned()
                .map(ValueShape::from_value_type)
                .collect(),
        )
    }

    pub(crate) fn type_(&self) -> &ExternalType {
        &self.type_
    }

    pub(crate) fn type_name(&self) -> &ExternalTypeName {
        self.type_.type_name()
    }

    pub(crate) fn arguments(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.type_ != other.type_ {
            return None;
        }
        Some(Self::new(
            self.type_name().clone(),
            self.arguments
                .iter()
                .zip(other.arguments.iter())
                .map(|(left, right)| left.merge(right))
                .collect::<Option<Vec<_>>>()?,
        ))
    }

    pub(crate) fn refine(&self, other: &Self) -> Option<Self> {
        if self.type_ != other.type_ {
            return None;
        }
        Some(Self::new(
            self.type_name().clone(),
            self.arguments
                .iter()
                .zip(other.arguments.iter())
                .map(|(left, right)| left.refine(right))
                .collect::<Option<Vec<_>>>()?,
        ))
    }

    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        Self::new(
            self.type_name().clone(),
            self.arguments
                .iter()
                .map(|shape| shape.substitute(substitution))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ExternalType, ExternalTypeDefinition, ExternalTypeName, ExternalValueShape};
    use crate::plan::{
        CustomConstructorRefinement, CustomTypeName, CustomValueShape, FunctionShape,
        TypeParameterId, TypeSubstitution, ValueShape, ValueType,
    };

    #[test]
    fn external_type_metadata_preserves_nominal_identity_and_parameters() {
        let name = ExternalTypeName::new("domain".into(), "domain/box".into(), "Box".into());
        let type_ = ExternalType::new(name.clone(), vec![ValueType::Int]);
        let definition = ExternalTypeDefinition::new(name.clone(), 2);

        assert_eq!(name.package(), "domain");
        assert_eq!(name.module(), "domain/box");
        assert_eq!(name.name(), "Box");
        assert_eq!(type_.type_name(), &name);
        assert_eq!(type_.arguments(), [ValueType::Int]);
        assert_eq!(definition.name(), &name);
        assert_eq!(
            definition.parameters(),
            [TypeParameterId(0), TypeParameterId(1)],
        );
    }

    #[test]
    fn external_value_shape_merges_refines_and_substitutes_exact_arguments() {
        let name = ExternalTypeName::new("domain".into(), "domain/box".into(), "Box".into());
        let parameter = ExternalValueShape::new(
            name.clone(),
            vec![ValueShape::Parameter(TypeParameterId(0))],
        );
        let same = parameter.clone();
        let other = ExternalValueShape::new(
            ExternalTypeName::new("domain".into(), "domain/other".into(), "Box".into()),
            vec![ValueShape::Parameter(TypeParameterId(0))],
        );
        let substituted =
            parameter.substitute(&TypeSubstitution::from_arguments(vec![ValueShape::Bool]));
        let type_ = ExternalType::new(name, vec![ValueType::Parameter(TypeParameterId(0))]);

        assert_eq!(ExternalValueShape::any(type_), parameter);
        assert_eq!(
            parameter.type_().arguments(),
            [ValueType::Parameter(TypeParameterId(0))]
        );
        assert_eq!(
            parameter.arguments(),
            [ValueShape::Parameter(TypeParameterId(0))]
        );
        assert_eq!(parameter.merge(&same), Some(parameter.clone()));
        assert_eq!(parameter.refine(&same), Some(parameter.clone()));
        assert_eq!(parameter.merge(&other), None);
        assert_eq!(parameter.refine(&other), None);
        assert_eq!(substituted.type_().arguments(), [ValueType::Bool]);
        assert_eq!(substituted.arguments(), [ValueShape::Bool]);
    }

    #[test]
    fn external_value_shape_rejects_incompatible_nested_refinements() {
        let name = ExternalTypeName::new("domain".into(), "domain/box".into(), "Box".into());
        let custom = |constructor| {
            ValueShape::Custom(CustomValueShape::new(
                CustomTypeName::new("domain".into(), "domain/item".into(), "Item".into()),
                Vec::new(),
                CustomConstructorRefinement::Exact(constructor),
            ))
        };
        let merge_left = ExternalValueShape::new(
            name.clone(),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![custom(0)],
                ValueShape::Int,
            )))],
        );
        let merge_right = ExternalValueShape::new(
            name.clone(),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![custom(1)],
                ValueShape::Int,
            )))],
        );
        let refine_left = ExternalValueShape::new(name.clone(), vec![custom(0)]);
        let refine_right = ExternalValueShape::new(name, vec![custom(1)]);

        assert_eq!(merge_left.type_(), merge_right.type_());
        assert_eq!(merge_left.merge(&merge_right), None);
        assert_eq!(refine_left.type_(), refine_right.type_());
        assert_eq!(refine_left.refine(&refine_right), None);
    }
}
