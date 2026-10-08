use super::shape::CallLocal;
use crate::plan::execution::graph::{CustomLocal, ParamLocal};
use crate::plan::execution::type_::{
    CustomConstructorId, CustomConstructorRefinement, CustomTypeTable, ValueShapeId,
    ValueShapeTable,
};

/// A local is projected only after its complete constructor refinement is
/// proved to have no fields. The original shape stays in admission metadata.
pub(super) struct CallTypes<'types> {
    pub(super) custom_types: &'types CustomTypeTable,
    pub(super) value_shapes: &'types ValueShapeTable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NullaryLocal {
    pub(super) local: CustomLocal,
    pub(super) arguments: Vec<ValueShapeId>,
    pub(super) constructors: Vec<CustomConstructorId>,
}

impl CallTypes<'_> {
    pub(super) fn local(&self, local: &ParamLocal) -> Option<CallLocal> {
        if let Some(local) = CallLocal::inspect(local) {
            return Some(local);
        }
        let ParamLocal::Custom(local) = local else {
            return None;
        };
        let shape = &self.value_shapes.custom_shapes[local.shape.shape_id.0];
        let type_ = &self.custom_types.types[local.shape.type_id.0];
        let constructors = match shape.constructor {
            CustomConstructorRefinement::Any => {
                if type_.constructor_count != type_.constructors.len()
                    || type_
                        .constructors
                        .iter()
                        .any(|constructor| !constructor.fields.is_empty())
                {
                    return None;
                }
                type_
                    .constructors
                    .iter()
                    .map(|constructor| constructor.id)
                    .collect()
            }
            CustomConstructorRefinement::Exact(index) => {
                let constructor = type_
                    .constructors
                    .iter()
                    .find(|constructor| constructor.id.index == index)?;
                if !constructor.fields.is_empty() {
                    return None;
                }
                vec![constructor.id]
            }
        };
        Some(CallLocal::Nullary(NullaryLocal {
            local: *local,
            arguments: shape.arguments.to_vec(),
            constructors,
        }))
    }
}

impl NullaryLocal {
    pub(super) fn accepts(&self, argument: &Self) -> bool {
        self.local.shape.type_id == argument.local.shape.type_id
            && self.arguments == argument.arguments
            && argument
                .constructors
                .iter()
                .all(|constructor| self.constructors.contains(constructor))
    }
}

#[cfg(test)]
mod tests {
    use super::{CallLocal, CallTypes};
    use crate::host::HostValueLifetime;
    use crate::plan::Text;
    use crate::plan::execution::graph::{
        CustomListLocalId, CustomLocal, CustomLocalId, IntLocalId, ListLocal, ParamLocal,
    };
    use crate::plan::execution::type_::custom::FieldRefinement;
    use crate::plan::execution::type_::{
        CustomConstructorDescriptor, CustomConstructorId, CustomConstructorRefinement,
        CustomFieldDescriptor, CustomListTypeId, CustomTypeDescriptor, CustomTypeId,
        CustomTypeTable, CustomValueShape, CustomValueShapeDescriptor, CustomValueShapeId,
        ListTypeId, NominalTypeMetadata, ValueShapeDescriptor, ValueShapeId, ValueShapeTable,
        ValueType,
    };

    #[test]
    fn nullary_projection_keeps_exact_sparse_refinements_and_rejects_fieldful_or_incomplete_any() {
        let type_id = CustomTypeId(0);
        let mut custom_types = CustomTypeTable {
            types: vec![CustomTypeDescriptor {
                type_: NominalTypeMetadata {
                    package: Text::Static("application"),
                    module: Text::Static("direction"),
                    name: Text::Static("Direction"),
                    arguments: Vec::new().into(),
                },
                constructor_count: 8,
                native_visible: true,
                lifetime: HostValueLifetime::LoadedOwner,
                constructors: vec![CustomConstructorDescriptor {
                    id: CustomConstructorId { type_id, index: 7 },
                    name: Text::Static("Before"),
                    native_tag: Text::Static("before"),
                    fields: Vec::new().into(),
                }]
                .into(),
            }]
            .into(),
            definitions: Vec::new().into(),
        };
        let value_shapes = ValueShapeTable {
            shapes: vec![ValueShapeDescriptor::Int, ValueShapeDescriptor::Float].into(),
            shape_types: vec![ValueType::Int, ValueType::Float].into(),
            custom_shapes: [
                (CustomConstructorRefinement::Any, Vec::new()),
                (CustomConstructorRefinement::Exact(7), Vec::new()),
                (CustomConstructorRefinement::Exact(0), Vec::new()),
                (CustomConstructorRefinement::Exact(7), vec![ValueShapeId(0)]),
                (CustomConstructorRefinement::Exact(7), vec![ValueShapeId(1)]),
            ]
            .into_iter()
            .map(|(constructor, arguments)| CustomValueShapeDescriptor {
                type_id,
                arguments: arguments.into(),
                constructor,
            })
            .collect::<Vec<_>>()
            .into(),
        };
        let local = |shape| {
            ParamLocal::Custom(CustomLocal {
                id: CustomLocalId(0),
                shape: CustomValueShape {
                    type_id,
                    shape_id: CustomValueShapeId(shape),
                },
            })
        };
        let projection = CallTypes {
            custom_types: &custom_types,
            value_shapes: &value_shapes,
        };
        assert!(
            projection.local(&local(0)).is_none(),
            "unknown constructors cannot be assumed fieldless"
        );
        assert!(
            projection.local(&local(2)).is_none(),
            "a missing sparse constructor is not index zero"
        );
        let exact = nullary(projection.local(&local(1)));
        assert_eq!(
            exact.constructors,
            vec![CustomConstructorId { type_id, index: 7 }]
        );
        let integer = nullary(projection.local(&local(3)));
        let floating = nullary(projection.local(&local(4)));
        assert!(
            !integer.accepts(&floating),
            "nominal identity does not erase type arguments"
        );
        let mut foreign = exact.clone();
        foreign.local.shape.type_id = CustomTypeId(1);
        assert!(!exact.accepts(&foreign));

        let mut complete = custom_types.types[0].clone();
        complete.constructor_count = 1;
        custom_types.types = vec![complete].into();
        let projection = CallTypes {
            custom_types: &custom_types,
            value_shapes: &value_shapes,
        };
        let any = nullary(projection.local(&local(0)));
        assert!(any.accepts(&exact));
        assert!(exact.accepts(&any));

        let mut fieldful = custom_types.types[0].clone();
        let mut constructor = fieldful.constructors[0].clone();
        constructor.fields = vec![CustomFieldDescriptor {
            label: None,
            type_: ValueType::Int,
            shape: ValueShapeId(0),
            refinement: FieldRefinement::Value,
        }]
        .into();
        fieldful.constructors = vec![constructor].into();
        custom_types.types = vec![fieldful].into();
        let projection = CallTypes {
            custom_types: &custom_types,
            value_shapes: &value_shapes,
        };
        assert!(projection.local(&local(0)).is_none());
        assert!(projection.local(&local(1)).is_none());
        assert_eq!(
            projection
                .local(&ParamLocal::Int(IntLocalId(0)))
                .unwrap()
                .canonical(),
            ParamLocal::Int(IntLocalId(0)),
        );
        assert!(
            projection
                .local(&ParamLocal::List(ListLocal::Custom {
                    local: CustomListLocalId(0),
                    type_id: CustomListTypeId {
                        list_type: ListTypeId(0),
                        item_type: type_id
                    }
                }))
                .is_none()
        );
    }
    fn nullary(local: Option<CallLocal>) -> super::NullaryLocal {
        match local {
            Some(CallLocal::Nullary(value)) => value,
            _ => panic!("fixture local must be a fieldless custom value"),
        }
    }

    #[test]
    fn nullary_fixture_guard_rejects_absent_and_scalar_projections() {
        for local in [None, Some(CallLocal::Int(IntLocalId(0)))] {
            assert!(std::panic::catch_unwind(|| nullary(local)).is_err());
        }
    }
}
