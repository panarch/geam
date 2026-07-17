use super::FunctionTemplateId;
use crate::plan::{FunctionShape, TypeParameterId, ValueShape};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScheme {
    parameters: Box<[TypeParameterId]>,
}

impl TypeScheme {
    pub(crate) fn new(parameter_count: usize) -> Self {
        Self {
            parameters: (0..parameter_count)
                .map(TypeParameterId)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn parameters(&self) -> &[TypeParameterId] {
        &self.parameters
    }

    pub(crate) fn is_monomorphic(&self) -> bool {
        self.parameters.is_empty()
    }

    pub(crate) fn try_substitution(&self, arguments: Vec<ValueShape>) -> Option<TypeSubstitution> {
        (arguments.len() == self.parameters.len()).then(|| TypeSubstitution {
            arguments: arguments.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TypeSubstitution {
    arguments: Box<[ValueShape]>,
}

impl TypeSubstitution {
    pub(crate) fn identity(scheme: &TypeScheme) -> Self {
        Self {
            arguments: scheme
                .parameters()
                .iter()
                .copied()
                .map(ValueShape::Parameter)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(crate) fn arguments(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn get(&self, parameter: TypeParameterId) -> &ValueShape {
        &self.arguments[parameter.index()]
    }

    pub(crate) fn substitute(&self, outer: &Self) -> Self {
        Self {
            arguments: self
                .arguments
                .iter()
                .map(|shape| shape.substitute(outer))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FunctionTemplateSignature {
    id: FunctionTemplateId,
    scheme: TypeScheme,
    shape: FunctionShape,
}

impl FunctionTemplateSignature {
    pub(crate) fn new(id: FunctionTemplateId, scheme: TypeScheme, shape: FunctionShape) -> Self {
        Self { id, scheme, shape }
    }

    pub(crate) fn id(&self) -> FunctionTemplateId {
        self.id
    }

    pub(crate) fn scheme(&self) -> &TypeScheme {
        &self.scheme
    }

    pub(crate) fn shape(&self) -> &FunctionShape {
        &self.shape
    }

    pub(crate) fn try_instantiate(
        &self,
        arguments: Vec<ValueShape>,
    ) -> Option<FunctionInstantiation> {
        let substitution = self.scheme.try_substitution(arguments)?;
        Some(FunctionInstantiation {
            template: self.id,
            shape: self.shape.substitute(&substitution),
            substitution,
        })
    }

    pub(crate) fn identity_instantiation(&self) -> FunctionInstantiation {
        let substitution = TypeSubstitution::identity(&self.scheme);
        FunctionInstantiation {
            template: self.id,
            shape: self.shape.clone(),
            substitution,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionInstantiation {
    template: FunctionTemplateId,
    substitution: TypeSubstitution,
    shape: FunctionShape,
}

impl FunctionInstantiation {
    pub(crate) fn template(&self) -> FunctionTemplateId {
        self.template
    }

    pub(crate) fn substitution(&self) -> &TypeSubstitution {
        &self.substitution
    }

    pub(crate) fn shape(&self) -> &FunctionShape {
        &self.shape
    }

    pub(crate) fn substitute(&self, outer: &TypeSubstitution) -> Self {
        Self {
            template: self.template,
            substitution: self.substitution.substitute(outer),
            shape: self.shape.substitute(outer),
        }
    }
}

#[cfg(test)]
pub(crate) fn monomorphic_function_instantiation(
    template: usize,
    shape: FunctionShape,
) -> FunctionInstantiation {
    FunctionTemplateSignature::new(FunctionTemplateId::new(template), TypeScheme::new(0), shape)
        .identity_instantiation()
}

#[cfg(test)]
mod tests {
    use super::{FunctionTemplateSignature, TypeScheme, TypeSubstitution};
    use crate::plan::{FunctionShape, FunctionTemplateId, TypeParameterId, ValueShape};

    #[test]
    fn type_substitution_composes_recursive_parameter_shapes() {
        let inner = TypeScheme::new(2)
            .try_substitution(vec![
                ValueShape::Parameter(TypeParameterId(1)),
                ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(0)))),
            ])
            .expect("two arguments should match the scheme");
        let outer = TypeScheme::new(2)
            .try_substitution(vec![ValueShape::Int, ValueShape::String])
            .expect("two arguments should match the scheme");

        assert_eq!(
            inner.substitute(&outer).arguments(),
            &[
                ValueShape::String,
                ValueShape::List(Box::new(ValueShape::Int)),
            ],
        );
    }

    #[test]
    fn function_template_instantiation_rejects_wrong_argument_count() {
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(3),
            TypeScheme::new(1),
            FunctionShape::new(
                vec![ValueShape::Parameter(TypeParameterId(0))],
                ValueShape::Parameter(TypeParameterId(0)),
            ),
        );

        assert_eq!(signature.try_instantiate(Vec::new()), None);
        assert_eq!(
            signature
                .try_instantiate(vec![ValueShape::Int])
                .expect("one argument should match the scheme")
                .shape(),
            &FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
        );
    }

    #[test]
    fn identity_substitution_preserves_parameter_order() {
        assert_eq!(
            TypeSubstitution::identity(&TypeScheme::new(2)).arguments(),
            &[
                ValueShape::Parameter(TypeParameterId(0)),
                ValueShape::Parameter(TypeParameterId(1)),
            ],
        );
    }
}
