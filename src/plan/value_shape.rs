use super::{
    CustomType, CustomTypeName, ExternalValueShape, FunctionType, TypeParameterId, ValueType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CustomConstructorRefinement {
    Any,
    Exact(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomValueShape {
    type_: CustomType,
    arguments: Box<[ValueShape]>,
    constructor: CustomConstructorRefinement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionShape {
    arguments: Box<[ValueShape]>,
    return_: Box<ValueShape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueShape {
    Parameter(TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[ValueShape]>),
    List(Box<ValueShape>),
    Function(Box<FunctionShape>),
    Custom(CustomValueShape),
    External(ExternalValueShape),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueStorageShape {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Box<[ValueShape]>),
    List(Box<ValueShape>),
    Function(Box<FunctionShape>),
    Custom(CustomValueShape),
    External(ExternalValueShape),
}

pub(crate) enum ValueRepresentation {
    Uninhabited(TypeParameterId),
    Stored(ValueStorageShape),
}

impl CustomValueShape {
    pub(crate) fn new(
        name: CustomTypeName,
        arguments: Vec<ValueShape>,
        constructor: CustomConstructorRefinement,
    ) -> Self {
        let type_ = CustomType::new(name, arguments.iter().map(ValueShape::value_type).collect());
        Self {
            type_,
            arguments: arguments.into_boxed_slice(),
            constructor,
        }
    }

    pub(crate) fn any(type_: CustomType) -> Self {
        Self::new(
            type_.type_name().clone(),
            type_
                .arguments()
                .iter()
                .cloned()
                .map(ValueShape::from_value_type)
                .collect(),
            CustomConstructorRefinement::Any,
        )
    }

    pub(crate) fn type_(&self) -> &CustomType {
        &self.type_
    }

    pub(crate) fn type_name(&self) -> &CustomTypeName {
        self.type_.type_name()
    }

    pub(crate) fn arguments(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn constructor(&self) -> CustomConstructorRefinement {
        self.constructor
    }

    pub(crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.type_ != other.type_ {
            return None;
        }

        let arguments = self
            .arguments
            .iter()
            .zip(other.arguments.iter())
            .map(|(left, right)| left.merge(right))
            .collect::<Option<Vec<_>>>()?;
        let constructor = if self.constructor == other.constructor {
            self.constructor
        } else {
            CustomConstructorRefinement::Any
        };

        Some(Self::new(
            self.type_.type_name().clone(),
            arguments,
            constructor,
        ))
    }

    fn refine(&self, other: &Self) -> Option<Self> {
        if self.type_ != other.type_ {
            return None;
        }

        let constructor = match (self.constructor, other.constructor) {
            (left, right) if left == right => left,
            (CustomConstructorRefinement::Any, exact)
            | (exact, CustomConstructorRefinement::Any) => exact,
            (CustomConstructorRefinement::Exact(_), CustomConstructorRefinement::Exact(_)) => {
                return None;
            }
        };
        Some(Self::new(
            self.type_.type_name().clone(),
            self.arguments
                .iter()
                .zip(other.arguments.iter())
                .map(|(left, right)| left.refine(right))
                .collect::<Option<Vec<_>>>()?,
            constructor,
        ))
    }

    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        Self::new(
            self.type_name().clone(),
            self.arguments
                .iter()
                .map(|shape| shape.substitute(substitution))
                .collect(),
            self.constructor,
        )
    }
}

impl FunctionShape {
    pub(crate) fn new(arguments: Vec<ValueShape>, return_: ValueShape) -> Self {
        Self {
            arguments: arguments.into_boxed_slice(),
            return_: Box::new(return_),
        }
    }

    pub(crate) fn from_function_type(type_: FunctionType) -> Self {
        Self::new(
            type_
                .argument_types()
                .iter()
                .cloned()
                .map(ValueShape::from_value_type)
                .collect(),
            ValueShape::from_value_type(type_.return_().clone()),
        )
    }

    pub(crate) fn argument_shapes(&self) -> &[ValueShape] {
        &self.arguments
    }

    pub(crate) fn return_shape(&self) -> &ValueShape {
        &self.return_
    }

    pub(crate) fn type_(&self) -> FunctionType {
        FunctionType::new(
            self.arguments.iter().map(ValueShape::value_type).collect(),
            self.return_.value_type(),
        )
    }

    pub(crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.arguments.len() != other.arguments.len() {
            return None;
        }

        Some(Self::new(
            self.arguments
                .iter()
                .zip(other.arguments.iter())
                .map(|(left, right)| left.refine(right))
                .collect::<Option<Vec<_>>>()?,
            self.return_.merge(&other.return_)?,
        ))
    }

    pub(crate) fn refine(&self, other: &Self) -> Option<Self> {
        if self.arguments.len() != other.arguments.len() {
            return None;
        }
        Some(Self::new(
            self.arguments
                .iter()
                .zip(other.arguments.iter())
                .map(|(left, right)| left.merge(right))
                .collect::<Option<Vec<_>>>()?,
            self.return_.refine(&other.return_)?,
        ))
    }

    pub(crate) fn can_flow_to(&self, target: &Self) -> bool {
        self.arguments.len() == target.arguments.len()
            && self
                .arguments
                .iter()
                .zip(target.arguments.iter())
                .all(|(source, target)| target.can_flow_to(source))
            && self.return_.can_flow_to(&target.return_)
    }

    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        Self::new(
            self.arguments
                .iter()
                .map(|shape| shape.substitute(substitution))
                .collect(),
            self.return_.substitute(substitution),
        )
    }
}

impl ValueShape {
    pub(crate) fn representation(&self) -> ValueRepresentation {
        match self {
            Self::Parameter(parameter) => ValueRepresentation::Uninhabited(*parameter),
            Self::Int => ValueRepresentation::Stored(ValueStorageShape::Int),
            Self::Float => ValueRepresentation::Stored(ValueStorageShape::Float),
            Self::String => ValueRepresentation::Stored(ValueStorageShape::String),
            Self::BitArray => ValueRepresentation::Stored(ValueStorageShape::BitArray),
            Self::UtfCodepoint => ValueRepresentation::Stored(ValueStorageShape::UtfCodepoint),
            Self::Bool => ValueRepresentation::Stored(ValueStorageShape::Bool),
            Self::Nil => ValueRepresentation::Stored(ValueStorageShape::Nil),
            Self::Tuple(elements) => {
                ValueRepresentation::Stored(ValueStorageShape::Tuple(elements.clone()))
            }
            Self::List(item) => ValueRepresentation::Stored(ValueStorageShape::List(item.clone())),
            Self::Function(function) => {
                ValueRepresentation::Stored(ValueStorageShape::Function(function.clone()))
            }
            Self::Custom(custom) => {
                ValueRepresentation::Stored(ValueStorageShape::Custom(custom.clone()))
            }
            Self::External(external) => {
                ValueRepresentation::Stored(ValueStorageShape::External(external.clone()))
            }
        }
    }

    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        match self {
            Self::Parameter(parameter) => substitution.resolve(*parameter),
            Self::Int => Self::Int,
            Self::Float => Self::Float,
            Self::String => Self::String,
            Self::BitArray => Self::BitArray,
            Self::UtfCodepoint => Self::UtfCodepoint,
            Self::Bool => Self::Bool,
            Self::Nil => Self::Nil,
            Self::Tuple(elements) => Self::Tuple(
                elements
                    .iter()
                    .map(|shape| shape.substitute(substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::List(item) => Self::List(Box::new(item.substitute(substitution))),
            Self::Function(function) => Self::Function(Box::new(function.substitute(substitution))),
            Self::Custom(custom) => Self::Custom(custom.substitute(substitution)),
            Self::External(external) => Self::External(external.substitute(substitution)),
        }
    }

    pub(crate) fn from_value_type(type_: ValueType) -> Self {
        match type_ {
            ValueType::Parameter(parameter) => Self::Parameter(parameter),
            ValueType::Int => Self::Int,
            ValueType::Float => Self::Float,
            ValueType::String => Self::String,
            ValueType::BitArray => Self::BitArray,
            ValueType::UtfCodepoint => Self::UtfCodepoint,
            ValueType::Bool => Self::Bool,
            ValueType::Nil => Self::Nil,
            ValueType::Tuple(elements) => Self::Tuple(
                elements
                    .into_iter()
                    .map(Self::from_value_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            ValueType::List(item) => Self::List(Box::new(Self::from_value_type(*item))),
            ValueType::Function(type_) => {
                Self::Function(Box::new(FunctionShape::from_function_type(*type_)))
            }
            ValueType::Custom(type_) => Self::Custom(CustomValueShape::any(type_)),
            ValueType::External(type_) => Self::External(ExternalValueShape::any(type_)),
        }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        match self {
            Self::Parameter(parameter) => ValueType::Parameter(*parameter),
            Self::Int => ValueType::Int,
            Self::Float => ValueType::Float,
            Self::String => ValueType::String,
            Self::BitArray => ValueType::BitArray,
            Self::UtfCodepoint => ValueType::UtfCodepoint,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::Tuple(elements) => {
                ValueType::Tuple(elements.iter().map(Self::value_type).collect())
            }
            Self::List(item) => ValueType::List(Box::new(item.value_type())),
            Self::Function(type_) => ValueType::Function(Box::new(type_.type_())),
            Self::Custom(shape) => ValueType::Custom(shape.type_().clone()),
            Self::External(shape) => ValueType::External(shape.type_().clone()),
        }
    }

    pub(crate) fn merge(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Parameter(left), Self::Parameter(right)) if left == right => {
                Some(Self::Parameter(*left))
            }
            (Self::Int, Self::Int) => Some(Self::Int),
            (Self::Float, Self::Float) => Some(Self::Float),
            (Self::String, Self::String) => Some(Self::String),
            (Self::BitArray, Self::BitArray) => Some(Self::BitArray),
            (Self::UtfCodepoint, Self::UtfCodepoint) => Some(Self::UtfCodepoint),
            (Self::Bool, Self::Bool) => Some(Self::Bool),
            (Self::Nil, Self::Nil) => Some(Self::Nil),
            (Self::Tuple(left), Self::Tuple(right)) if left.len() == right.len() => {
                Some(Self::Tuple(
                    left.iter()
                        .zip(right.iter())
                        .map(|(left, right)| left.merge(right))
                        .collect::<Option<Vec<_>>>()?
                        .into_boxed_slice(),
                ))
            }
            (Self::List(left), Self::List(right)) => Some(Self::List(Box::new(left.merge(right)?))),
            (Self::Function(left), Self::Function(right)) => {
                Some(Self::Function(Box::new(left.merge(right)?)))
            }
            (Self::Custom(left), Self::Custom(right)) => Some(Self::Custom(left.merge(right)?)),
            (Self::External(left), Self::External(right)) => {
                Some(Self::External(left.merge(right)?))
            }
            _ => None,
        }
    }

    pub(crate) fn refine(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Parameter(left), Self::Parameter(right)) if left == right => {
                Some(Self::Parameter(*left))
            }
            (Self::Int, Self::Int) => Some(Self::Int),
            (Self::Float, Self::Float) => Some(Self::Float),
            (Self::String, Self::String) => Some(Self::String),
            (Self::BitArray, Self::BitArray) => Some(Self::BitArray),
            (Self::UtfCodepoint, Self::UtfCodepoint) => Some(Self::UtfCodepoint),
            (Self::Bool, Self::Bool) => Some(Self::Bool),
            (Self::Nil, Self::Nil) => Some(Self::Nil),
            (Self::Tuple(left), Self::Tuple(right)) if left.len() == right.len() => {
                Some(Self::Tuple(
                    left.iter()
                        .zip(right.iter())
                        .map(|(left, right)| left.refine(right))
                        .collect::<Option<Vec<_>>>()?
                        .into_boxed_slice(),
                ))
            }
            (Self::List(left), Self::List(right)) => {
                Some(Self::List(Box::new(left.refine(right)?)))
            }
            (Self::Function(left), Self::Function(right)) => {
                Some(Self::Function(Box::new(left.refine(right)?)))
            }
            (Self::Custom(left), Self::Custom(right)) => Some(Self::Custom(left.refine(right)?)),
            (Self::External(left), Self::External(right)) => {
                Some(Self::External(left.refine(right)?))
            }
            _ => None,
        }
    }

    pub(crate) fn can_flow_to(&self, target: &Self) -> bool {
        match (self, target) {
            (Self::Parameter(source), Self::Parameter(target)) => source == target,
            (Self::Int, Self::Int)
            | (Self::Float, Self::Float)
            | (Self::String, Self::String)
            | (Self::BitArray, Self::BitArray)
            | (Self::UtfCodepoint, Self::UtfCodepoint)
            | (Self::Bool, Self::Bool)
            | (Self::Nil, Self::Nil) => true,
            (Self::Tuple(source), Self::Tuple(target)) => {
                source.len() == target.len()
                    && source
                        .iter()
                        .zip(target.iter())
                        .all(|(source, target)| source.can_flow_to(target))
            }
            (Self::List(source), Self::List(target)) => source.can_flow_to(target),
            (Self::Function(source), Self::Function(target)) => source.can_flow_to(target),
            (Self::Custom(source), Self::Custom(target)) => {
                source.type_ == target.type_
                    && source
                        .arguments
                        .iter()
                        .zip(target.arguments.iter())
                        .all(|(source, target)| source.can_flow_to(target))
                    && match target.constructor {
                        CustomConstructorRefinement::Any => true,
                        CustomConstructorRefinement::Exact(target) => {
                            source.constructor == CustomConstructorRefinement::Exact(target)
                        }
                    }
            }
            (Self::External(source), Self::External(target)) => {
                source.type_() == target.type_()
                    && source
                        .arguments()
                        .iter()
                        .zip(target.arguments().iter())
                        .all(|(source, target)| source.can_flow_to(target))
            }
            _ => false,
        }
    }
}

impl ValueStorageShape {
    pub(crate) fn substitute(&self, substitution: &crate::plan::TypeSubstitution) -> Self {
        match self {
            Self::Int => Self::Int,
            Self::Float => Self::Float,
            Self::String => Self::String,
            Self::BitArray => Self::BitArray,
            Self::UtfCodepoint => Self::UtfCodepoint,
            Self::Bool => Self::Bool,
            Self::Nil => Self::Nil,
            Self::Tuple(elements) => Self::Tuple(
                elements
                    .iter()
                    .map(|shape| shape.substitute(substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::List(item) => Self::List(Box::new(item.substitute(substitution))),
            Self::Function(function) => Self::Function(Box::new(function.substitute(substitution))),
            Self::Custom(custom) => Self::Custom(custom.substitute(substitution)),
            Self::External(external) => Self::External(external.substitute(substitution)),
        }
    }

    pub(crate) fn to_value_shape(&self) -> ValueShape {
        match self {
            Self::Int => ValueShape::Int,
            Self::Float => ValueShape::Float,
            Self::String => ValueShape::String,
            Self::BitArray => ValueShape::BitArray,
            Self::UtfCodepoint => ValueShape::UtfCodepoint,
            Self::Bool => ValueShape::Bool,
            Self::Nil => ValueShape::Nil,
            Self::Tuple(elements) => ValueShape::Tuple(elements.clone()),
            Self::List(item) => ValueShape::List(item.clone()),
            Self::Function(function) => ValueShape::Function(function.clone()),
            Self::Custom(custom) => ValueShape::Custom(custom.clone()),
            Self::External(external) => ValueShape::External(external.clone()),
        }
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.to_value_shape().value_type()
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomConstructorRefinement, CustomValueShape, FunctionShape, ValueShape};
    use crate::plan::{
        CustomTypeName, ExternalTypeName, ExternalValueShape, TypeScheme, TypeSubstitution,
        ValueType,
    };

    fn custom(index: Option<usize>, argument: ValueShape) -> ValueShape {
        ValueShape::Custom(CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            vec![argument],
            index.map_or(
                CustomConstructorRefinement::Any,
                CustomConstructorRefinement::Exact,
            ),
        ))
    }

    fn other_custom(index: Option<usize>, argument: ValueShape) -> ValueShape {
        ValueShape::Custom(CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
            vec![argument],
            index.map_or(
                CustomConstructorRefinement::Any,
                CustomConstructorRefinement::Exact,
            ),
        ))
    }

    #[test]
    fn recursive_shapes_materialize_nominal_value_types() {
        let shape = ValueShape::Function(Box::new(FunctionShape::new(
            vec![ValueShape::List(Box::new(custom(Some(1), ValueShape::Int)))],
            ValueShape::Tuple(vec![custom(Some(2), ValueShape::String)].into_boxed_slice()),
        )));

        assert_eq!(
            shape.value_type(),
            ValueType::Function(Box::new(crate::plan::FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Custom(
                    crate::plan::CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                        vec![ValueType::Int],
                    ),
                )))],
                ValueType::Tuple(vec![ValueType::Custom(crate::plan::CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    vec![ValueType::String],
                ))]),
            ))),
        );
    }

    #[test]
    fn primitive_and_parameter_shapes_survive_identity_operations() {
        let substitution = TypeSubstitution::identity(&TypeScheme::new(0));

        for shape in [
            ValueShape::Int,
            ValueShape::Float,
            ValueShape::String,
            ValueShape::BitArray,
            ValueShape::UtfCodepoint,
            ValueShape::Bool,
            ValueShape::Nil,
        ] {
            assert_eq!(shape.substitute(&substitution), shape);
        }

        let parameter = ValueShape::Parameter(crate::plan::TypeParameterId(0));
        assert_eq!(parameter.merge(&parameter), Some(parameter));
    }

    #[test]
    fn merging_keeps_equal_exact_constructors_and_widens_different_ones() {
        let exact = custom(Some(1), custom(Some(2), ValueShape::Int));
        let same = custom(Some(1), custom(Some(2), ValueShape::Int));
        let different = custom(Some(3), custom(None, ValueShape::Int));

        assert_eq!(exact.merge(&same), Some(exact.clone()));
        assert_eq!(
            exact.merge(&different),
            Some(custom(None, custom(None, ValueShape::Int))),
        );
    }

    #[test]
    fn shape_flow_widens_constructor_refinements_without_narrowing() {
        let exact = custom(Some(1), custom(Some(2), ValueShape::Int));
        let widened_inner = custom(Some(1), custom(None, ValueShape::Int));
        let any = custom(None, custom(None, ValueShape::Int));

        assert!(exact.can_flow_to(&widened_inner));
        assert!(exact.can_flow_to(&any));
        assert!(!any.can_flow_to(&exact));
        assert!(!custom(Some(2), ValueShape::Int).can_flow_to(&custom(Some(1), ValueShape::Int)));
    }

    #[test]
    fn function_shape_flow_is_contravariant_in_arguments_and_covariant_in_returns() {
        let exact = custom(Some(1), ValueShape::Int);
        let any = custom(None, ValueShape::Int);
        let broad_argument_exact_return = ValueShape::Function(Box::new(FunctionShape::new(
            vec![any.clone()],
            exact.clone(),
        )));
        let exact_argument_broad_return = ValueShape::Function(Box::new(FunctionShape::new(
            vec![exact.clone()],
            any.clone(),
        )));

        assert!(broad_argument_exact_return.can_flow_to(&exact_argument_broad_return));
        assert!(!exact_argument_broad_return.can_flow_to(&broad_argument_exact_return));
        assert_eq!(
            broad_argument_exact_return.merge(&exact_argument_broad_return),
            Some(exact_argument_broad_return.clone()),
        );
        assert_eq!(
            broad_argument_exact_return.refine(&exact_argument_broad_return),
            Some(broad_argument_exact_return),
        );
    }

    #[test]
    fn incompatible_recursive_shapes_do_not_merge_or_refine() {
        let boxed_int = custom(Some(0), ValueShape::Int);
        let boxed_string = custom(Some(0), ValueShape::String);
        let other_int = other_custom(Some(0), ValueShape::Int);
        let first = custom(Some(0), ValueShape::Int);
        let second = custom(Some(1), ValueShape::Int);

        assert_eq!(boxed_int.merge(&boxed_string), None);
        assert_eq!(boxed_int.refine(&boxed_string), None);
        assert_eq!(boxed_int.merge(&other_int), None);
        assert_eq!(boxed_int.refine(&other_int), None);
        assert_eq!(first.refine(&second), None);

        let conflicting_function_arguments = (
            custom(
                Some(0),
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![custom(Some(0), ValueShape::Int)],
                    ValueShape::Int,
                ))),
            ),
            custom(
                Some(0),
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![custom(Some(1), ValueShape::Int)],
                    ValueShape::Int,
                ))),
            ),
        );
        assert_eq!(
            conflicting_function_arguments
                .0
                .merge(&conflicting_function_arguments.1),
            None,
        );

        let conflicting_function_returns = (
            custom(
                Some(0),
                ValueShape::Function(Box::new(FunctionShape::new(
                    Vec::new(),
                    custom(Some(0), ValueShape::Int),
                ))),
            ),
            custom(
                Some(0),
                ValueShape::Function(Box::new(FunctionShape::new(
                    Vec::new(),
                    custom(Some(1), ValueShape::Int),
                ))),
            ),
        );
        assert_eq!(
            conflicting_function_returns
                .0
                .refine(&conflicting_function_returns.1),
            None,
        );

        let tuple_int = ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice());
        let tuple_string = ValueShape::Tuple(vec![ValueShape::String].into_boxed_slice());
        let tuple_pair =
            ValueShape::Tuple(vec![ValueShape::Int, ValueShape::Int].into_boxed_slice());
        assert_eq!(tuple_int.merge(&tuple_string), None);
        assert_eq!(tuple_int.refine(&tuple_string), None);
        assert_eq!(tuple_int.merge(&tuple_pair), None);
        assert_eq!(tuple_int.refine(&tuple_pair), None);

        let list_int = ValueShape::List(Box::new(ValueShape::Int));
        let list_string = ValueShape::List(Box::new(ValueShape::String));
        assert_eq!(list_int.merge(&list_string), None);
        assert_eq!(list_int.refine(&list_string), None);

        let one_argument = ValueShape::Function(Box::new(FunctionShape::new(
            vec![ValueShape::Int],
            ValueShape::Int,
        )));
        let two_arguments = ValueShape::Function(Box::new(FunctionShape::new(
            vec![ValueShape::Int, ValueShape::Int],
            ValueShape::Int,
        )));
        let wrong_argument = ValueShape::Function(Box::new(FunctionShape::new(
            vec![ValueShape::String],
            ValueShape::Int,
        )));
        let wrong_return = ValueShape::Function(Box::new(FunctionShape::new(
            vec![ValueShape::Int],
            ValueShape::String,
        )));
        assert_eq!(one_argument.merge(&two_arguments), None);
        assert_eq!(one_argument.refine(&two_arguments), None);
        assert_eq!(one_argument.merge(&wrong_argument), None);
        assert_eq!(one_argument.refine(&wrong_argument), None);
        assert_eq!(one_argument.merge(&wrong_return), None);
        assert_eq!(one_argument.refine(&wrong_return), None);

        let external_name = ExternalTypeName::new("geam".into(), "main".into(), "External".into());
        let external_merge_left = ValueShape::External(ExternalValueShape::new(
            external_name.clone(),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![custom(Some(0), ValueShape::Int)],
                ValueShape::Int,
            )))],
        ));
        let external_merge_right = ValueShape::External(ExternalValueShape::new(
            external_name.clone(),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![custom(Some(1), ValueShape::Int)],
                ValueShape::Int,
            )))],
        ));
        let external_refine_left = ValueShape::External(ExternalValueShape::new(
            external_name.clone(),
            vec![custom(Some(0), ValueShape::Int)],
        ));
        let external_refine_right = ValueShape::External(ExternalValueShape::new(
            external_name,
            vec![custom(Some(1), ValueShape::Int)],
        ));
        assert_eq!(external_merge_left.merge(&external_merge_right), None);
        assert_eq!(external_refine_left.refine(&external_refine_right), None,);

        assert_eq!(ValueShape::Int.merge(&ValueShape::String), None);
        assert_eq!(ValueShape::Int.refine(&ValueShape::String), None);
        assert!(!ValueShape::Int.can_flow_to(&ValueShape::String));
    }

    #[test]
    fn incompatible_function_shapes_do_not_flow() {
        let one_argument = FunctionShape::new(vec![ValueShape::Int], ValueShape::Int);
        let two_arguments =
            FunctionShape::new(vec![ValueShape::Int, ValueShape::Int], ValueShape::Int);
        let wrong_argument = FunctionShape::new(vec![ValueShape::String], ValueShape::Int);
        let wrong_return = FunctionShape::new(vec![ValueShape::Int], ValueShape::String);

        assert!(!one_argument.can_flow_to(&two_arguments));
        assert!(!one_argument.can_flow_to(&wrong_argument));
        assert!(!one_argument.can_flow_to(&wrong_return));
    }
}
