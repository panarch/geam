use crate::plan::{
    ConstantInstantiation, ConstantTemplateSignature, CustomConstructorRefinement,
    FunctionInstantiation, FunctionTemplateSignature, TypeParameterId, TypeScheme, ValueShape,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub(super) struct TypeParameterScope {
    parameters: HashMap<u64, TypeParameterId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionInstantiationMismatch {
    ArgumentCount,
    ArgumentShape,
    ReturnShape,
    UnresolvedParameter,
}

pub(super) fn instantiate(
    signature: &FunctionTemplateSignature,
    actual: &crate::plan::FunctionShape,
) -> Result<FunctionInstantiation, FunctionInstantiationMismatch> {
    if signature.shape().argument_shapes().len() != actual.argument_shapes().len() {
        return Err(FunctionInstantiationMismatch::ArgumentCount);
    }

    if signature.scheme().is_monomorphic() {
        for (template, actual) in signature
            .shape()
            .argument_shapes()
            .iter()
            .zip(actual.argument_shapes())
        {
            match_shape(template, actual, &mut [])
                .ok_or(FunctionInstantiationMismatch::ArgumentShape)?;
        }
        match_shape(
            signature.shape().return_shape(),
            actual.return_shape(),
            &mut [],
        )
        .ok_or(FunctionInstantiationMismatch::ReturnShape)?;
        return Ok(signature.identity_instantiation());
    }

    let mut bindings = vec![None; signature.scheme().parameters().len()];
    for (template, actual) in signature
        .shape()
        .argument_shapes()
        .iter()
        .zip(actual.argument_shapes())
    {
        match_shape(template, actual, &mut bindings)
            .ok_or(FunctionInstantiationMismatch::ArgumentShape)?;
    }
    match_shape(
        signature.shape().return_shape(),
        actual.return_shape(),
        &mut bindings,
    )
    .ok_or(FunctionInstantiationMismatch::ReturnShape)?;

    let arguments = bindings
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(FunctionInstantiationMismatch::UnresolvedParameter)?;
    signature
        .try_instantiate(arguments)
        .ok_or(FunctionInstantiationMismatch::UnresolvedParameter)
}

pub(super) fn instantiate_constant(
    signature: &ConstantTemplateSignature,
    actual: &ValueShape,
) -> Option<ConstantInstantiation> {
    let mut bindings = vec![None; signature.scheme().parameters().len()];
    match_shape(signature.shape(), actual, &mut bindings)?;
    signature.try_instantiate(bindings.into_iter().collect::<Option<Vec<_>>>()?)
}

fn match_shape(
    template: &ValueShape,
    actual: &ValueShape,
    bindings: &mut [Option<ValueShape>],
) -> Option<()> {
    match (template, actual) {
        (ValueShape::Parameter(parameter), actual) => {
            let binding = &mut bindings[parameter.index()];
            *binding = Some(match binding.take() {
                Some(existing) => existing.merge(actual)?,
                None => actual.clone(),
            });
            Some(())
        }
        (ValueShape::Int, ValueShape::Int)
        | (ValueShape::Float, ValueShape::Float)
        | (ValueShape::String, ValueShape::String)
        | (ValueShape::BitArray, ValueShape::BitArray)
        | (ValueShape::UtfCodepoint, ValueShape::UtfCodepoint)
        | (ValueShape::Bool, ValueShape::Bool)
        | (ValueShape::Nil, ValueShape::Nil) => Some(()),
        (ValueShape::Tuple(template), ValueShape::Tuple(actual))
            if template.len() == actual.len() =>
        {
            for (template, actual) in template.iter().zip(actual.iter()) {
                match_shape(template, actual, bindings)?;
            }
            Some(())
        }
        (ValueShape::List(template), ValueShape::List(actual)) => {
            match_shape(template, actual, bindings)
        }
        (ValueShape::Function(template), ValueShape::Function(actual))
            if template.argument_shapes().len() == actual.argument_shapes().len() =>
        {
            for (template, actual) in template
                .argument_shapes()
                .iter()
                .zip(actual.argument_shapes())
            {
                match_shape(template, actual, bindings)?;
            }
            match_shape(template.return_shape(), actual.return_shape(), bindings)
        }
        (ValueShape::Custom(template), ValueShape::Custom(actual))
            if template.type_name() == actual.type_name()
                && template.arguments().len() == actual.arguments().len()
                && !matches!(
                    (template.constructor(), actual.constructor()),
                    (
                        CustomConstructorRefinement::Exact(left),
                        CustomConstructorRefinement::Exact(right)
                    ) if left != right
                ) =>
        {
            for (template, actual) in template.arguments().iter().zip(actual.arguments()) {
                match_shape(template, actual, bindings)?;
            }
            Some(())
        }
        _ => None,
    }
}

impl TypeParameterScope {
    pub(super) fn resolve(&mut self, source_id: u64) -> TypeParameterId {
        let next = TypeParameterId(self.parameters.len());
        *self.parameters.entry(source_id).or_insert(next)
    }

    pub(super) fn scheme(&self) -> TypeScheme {
        TypeScheme::new(self.parameters.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FunctionInstantiationMismatch, TypeParameterScope, instantiate, instantiate_constant,
    };
    use crate::plan::{
        ConstantTemplateId, ConstantTemplateSignature, ConstantValue, CustomConstructorRefinement,
        CustomTypeName, CustomValueShape, FunctionShape, FunctionTemplateId,
        FunctionTemplateSignature, TypeParameterId, TypeScheme, ValueShape,
    };

    fn function_signature(
        parameter_count: usize,
        arguments: Vec<ValueShape>,
        return_: ValueShape,
    ) -> FunctionTemplateSignature {
        FunctionTemplateSignature::new(
            FunctionTemplateId::new(0),
            TypeScheme::new(parameter_count),
            FunctionShape::new(arguments, return_),
        )
    }

    fn custom_shape(
        constructor: CustomConstructorRefinement,
        arguments: Vec<ValueShape>,
    ) -> ValueShape {
        ValueShape::Custom(CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            arguments,
            constructor,
        ))
    }

    #[test]
    fn type_parameter_scope_canonicalizes_source_ids_per_scheme() {
        let mut scope = TypeParameterScope::default();

        assert_eq!(scope.resolve(41), TypeParameterId(0));
        assert_eq!(scope.resolve(7), TypeParameterId(1));
        assert_eq!(scope.resolve(41), TypeParameterId(0));
        assert_eq!(
            scope.scheme().parameters(),
            &[TypeParameterId(0), TypeParameterId(1)],
        );
    }

    #[test]
    fn function_instantiation_reports_each_shape_boundary() {
        let parameter = ValueShape::Parameter(TypeParameterId(0));
        let polymorphic = function_signature(1, vec![parameter.clone()], parameter.clone());

        assert_eq!(
            instantiate(
                &polymorphic,
                &FunctionShape::new(Vec::new(), ValueShape::Int),
            ),
            Err(FunctionInstantiationMismatch::ArgumentCount),
        );

        let argument_constrained = function_signature(1, vec![ValueShape::Int], parameter.clone());
        assert_eq!(
            instantiate(
                &argument_constrained,
                &FunctionShape::new(vec![ValueShape::String], ValueShape::String),
            ),
            Err(FunctionInstantiationMismatch::ArgumentShape),
        );

        assert_eq!(
            instantiate(
                &polymorphic,
                &FunctionShape::new(vec![ValueShape::Int], ValueShape::String),
            ),
            Err(FunctionInstantiationMismatch::ReturnShape),
        );

        let unresolved = function_signature(2, vec![parameter.clone()], parameter.clone());
        assert_eq!(
            instantiate(
                &unresolved,
                &FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
            ),
            Err(FunctionInstantiationMismatch::UnresolvedParameter),
        );

        let monomorphic = function_signature(0, vec![ValueShape::Int], ValueShape::String);
        assert_eq!(
            instantiate(
                &monomorphic,
                &FunctionShape::new(vec![ValueShape::String], ValueShape::String),
            ),
            Err(FunctionInstantiationMismatch::ArgumentShape),
        );
        assert_eq!(
            instantiate(
                &monomorphic,
                &FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
            ),
            Err(FunctionInstantiationMismatch::ReturnShape),
        );
    }

    #[test]
    fn function_instantiation_preserves_recursive_shapes_and_refinements() {
        let parameter = ValueShape::Parameter(TypeParameterId(0));
        let template_custom =
            custom_shape(CustomConstructorRefinement::Any, vec![parameter.clone()]);
        let actual_custom = custom_shape(
            CustomConstructorRefinement::Exact(1),
            vec![ValueShape::List(Box::new(ValueShape::Int))],
        );
        let signature = function_signature(
            1,
            vec![ValueShape::Tuple(
                vec![
                    template_custom,
                    ValueShape::Function(Box::new(FunctionShape::new(
                        vec![parameter.clone()],
                        parameter.clone(),
                    ))),
                ]
                .into_boxed_slice(),
            )],
            parameter,
        );
        let actual = FunctionShape::new(
            vec![ValueShape::Tuple(
                vec![
                    actual_custom,
                    ValueShape::Function(Box::new(FunctionShape::new(
                        vec![ValueShape::List(Box::new(ValueShape::Int))],
                        ValueShape::List(Box::new(ValueShape::Int)),
                    ))),
                ]
                .into_boxed_slice(),
            )],
            ValueShape::List(Box::new(ValueShape::Int)),
        );

        let instantiation = instantiate(&signature, &actual).expect("shape should instantiate");
        assert_eq!(
            instantiation.substitution().arguments(),
            &[ValueShape::List(Box::new(ValueShape::Int))],
        );
        assert_eq!(
            instantiation.shape(),
            &FunctionShape::new(
                vec![ValueShape::Tuple(
                    vec![
                        custom_shape(
                            CustomConstructorRefinement::Any,
                            vec![ValueShape::List(Box::new(ValueShape::Int))],
                        ),
                        ValueShape::Function(Box::new(FunctionShape::new(
                            vec![ValueShape::List(Box::new(ValueShape::Int))],
                            ValueShape::List(Box::new(ValueShape::Int)),
                        ))),
                    ]
                    .into_boxed_slice(),
                )],
                ValueShape::List(Box::new(ValueShape::Int)),
            ),
        );

        let exact_zero = function_signature(
            1,
            vec![custom_shape(
                CustomConstructorRefinement::Exact(0),
                vec![ValueShape::Parameter(TypeParameterId(0))],
            )],
            ValueShape::Parameter(TypeParameterId(0)),
        );
        assert_eq!(
            instantiate(
                &exact_zero,
                &FunctionShape::new(
                    vec![custom_shape(
                        CustomConstructorRefinement::Exact(1),
                        vec![ValueShape::Int],
                    )],
                    ValueShape::Int,
                ),
            ),
            Err(FunctionInstantiationMismatch::ArgumentShape),
        );
    }

    #[test]
    fn constant_instantiation_requires_a_complete_matching_substitution() {
        let signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(0),
            0,
            TypeScheme::new(1),
            ValueShape::Parameter(TypeParameterId(0)),
        );

        let instantiation =
            instantiate_constant(&signature, &ValueShape::List(Box::new(ValueShape::String)))
                .expect("constant should instantiate");
        assert_eq!(
            ConstantValue::reference(instantiation).shape(),
            ValueShape::List(Box::new(ValueShape::String)),
        );

        assert_eq!(instantiate_constant(&signature, &ValueShape::Int), None);

        let unresolved = ConstantTemplateSignature::list(
            ConstantTemplateId::new(1),
            1,
            TypeScheme::new(2),
            ValueShape::Parameter(TypeParameterId(0)),
        );
        assert_eq!(
            instantiate_constant(&unresolved, &ValueShape::List(Box::new(ValueShape::String)),),
            None,
        );
    }

    #[test]
    fn recursive_shape_matching_propagates_nested_mismatches() {
        let parameter = ValueShape::Parameter(TypeParameterId(0));
        let tuple_signature = function_signature(
            1,
            vec![ValueShape::Tuple(
                vec![parameter.clone(), ValueShape::Int].into_boxed_slice(),
            )],
            parameter.clone(),
        );
        assert_eq!(
            instantiate(
                &tuple_signature,
                &FunctionShape::new(
                    vec![ValueShape::Tuple(
                        vec![ValueShape::String, ValueShape::String].into_boxed_slice(),
                    )],
                    ValueShape::String,
                ),
            ),
            Err(FunctionInstantiationMismatch::ArgumentShape),
        );

        let custom_signature = function_signature(
            1,
            vec![custom_shape(
                CustomConstructorRefinement::Any,
                vec![parameter.clone(), ValueShape::Int],
            )],
            parameter,
        );
        assert_eq!(
            instantiate(
                &custom_signature,
                &FunctionShape::new(
                    vec![custom_shape(
                        CustomConstructorRefinement::Exact(0),
                        vec![ValueShape::String, ValueShape::String],
                    )],
                    ValueShape::String,
                ),
            ),
            Err(FunctionInstantiationMismatch::ArgumentShape),
        );
    }
}
