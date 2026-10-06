use super::super::LoweringContext;
use super::super::specialization::{
    SpecializationKey, SpecializedCustomConstructor, SpecializedCustomValueShape,
    SpecializedFunctionShape, SpecializedTypeSubstitution, SpecializedValueShape, StoredValueShape,
    ValueInhabitation,
};
use super::super::{function, local};
use crate::host::{HostTypeDescriptor, RegisteredHostConstructions};
use crate::plan::execution::function::{CoreRuntimeFunctionId, NeverFunctionId, RuntimeFunctionId};
use crate::plan::execution::host::native::views::retained_types;
use crate::plan::execution::host::{
    HostCallParameter, HostNativeView, HostSpecializationError, HostedFunctionParameters,
    NativeConstructor, NativeConversion, NativeConversionId, NativeConversionKind,
    NativeConversions, NativeCustomView, NativeFunctionView,
};
use crate::plan::execution::type_::custom::CustomDefinition;
use crate::plan::execution::type_::metadata::substitute;
use crate::plan::execution::type_::{FunctionMetadata, TypeMetadata};
use crate::plan::{HostFunctionTemplate, ValueType};
use std::collections::{HashMap, VecDeque};

struct NativeSealing<'context> {
    context: &'context mut LoweringContext,
    rules: HashMap<ValueType, usize>,
    customs: HashMap<ValueType, Box<[NativeCustomConstruction]>>,
    ids: HashMap<ValueType, NativeConversionId>,
    pending: VecDeque<SpecializedValueShape>,
    nodes: Vec<NativeConversion>,
    sources: Vec<SpecializedValueShape>,
    position: NativeViewPosition,
    definitions: Vec<CustomDefinition>,
}

pub(in crate::plan::execution::lowering) struct NativeViewDraft {
    pub(super) index: usize,
    pub(super) family: function::FunctionTableFamily,
    pub(super) shape: SpecializedFunctionShape,
    pub(super) parameters: HostedFunctionParameters,
    pub(super) parameter_shapes: Box<[StoredValueShape]>,
    pub(super) captures: Box<[StoredValueShape]>,
    pub(super) operation: HostNativeView,
}

pub(super) struct NativeViewPosition {
    pub(super) parent: usize,
    pub(super) parent_value: bool,
    pub(super) first_value: usize,
    pub(super) first_never: usize,
}

pub(in crate::plan::execution::lowering) struct NativeViewShape {
    pub(in crate::plan::execution::lowering) family: function::FunctionTableFamily,
    pub(in crate::plan::execution::lowering) index: usize,
    pub(in crate::plan::execution::lowering) parameters: Box<[StoredValueShape]>,
    pub(in crate::plan::execution::lowering) return_: SpecializedValueShape,
    pub(in crate::plan::execution::lowering) captures: Box<[StoredValueShape]>,
}

pub(super) struct NativeCustomConstruction {
    constructor: crate::plan::execution::type_::CustomConstructorId,
    tag: ecow::EcoString,
    fields: Box<[SpecializedValueShape]>,
}

impl NativeCustomConstruction {
    pub(super) fn new(
        constructor: crate::plan::execution::type_::CustomConstructorId,
        name: &str,
        fields: Box<[SpecializedValueShape]>,
    ) -> Self {
        Self {
            constructor,
            tag: gleam_compiler_core::strings::to_snake_case(name),
            fields,
        }
    }
}

pub(super) fn seal(
    template: &HostFunctionTemplate,
    constructions: &RegisteredHostConstructions,
    rules: &[HostTypeDescriptor],
    key: &SpecializationKey,
    context: &mut LoweringContext,
    customs: HashMap<ValueType, Box<[NativeCustomConstruction]>>,
    position: NativeViewPosition,
) -> Result<NativeConversions, HostSpecializationError> {
    let targets = constructions.types();
    let sources = constructions.native_sources();
    let instantiate = |descriptor: &HostTypeDescriptor| {
        SpecializedValueShape::instantiate(&descriptor.value_shape(), key.substitution())
    };
    let mut resolved_rules = HashMap::new();
    for (index, rule) in rules.iter().enumerate() {
        let type_ = instantiate(rule).to_module_shape().value_type();
        if resolved_rules.insert(type_.clone(), index).is_some() {
            return Err(HostSpecializationError::conflicting_native_conversions(
                template.package().clone(),
                template.site().module().into(),
                template.site().function().into(),
                crate::plan::FunctionType::new(
                    template
                        .parameters()
                        .iter()
                        .map(|parameter| instantiate(parameter).to_module_shape().value_type())
                        .collect(),
                    instantiate(template.return_type())
                        .to_module_shape()
                        .value_type(),
                ),
                type_,
            ));
        }
    }
    let definitions = context
        .representations
        .definitions()
        .map(CustomDefinition::from_definition)
        .chain([crate::plan::execution::type_::custom::definition::RESULT.clone()])
        .collect::<Vec<_>>();
    let granted = if sources.is_empty() {
        Vec::new()
    } else {
        retained_types(
            sources.iter().chain(targets).map(|descriptor| {
                TypeMetadata::from_public(&instantiate(descriptor).to_module_shape().value_type())
            }),
            |nominal| {
                definitions.iter().find(|definition| {
                    definition.identity()
                        == (
                            nominal.package.as_str(),
                            nominal.module.as_str(),
                            nominal.name.as_str(),
                        )
                })
            },
        )
    };
    let source_shapes = granted
        .iter()
        .map(|metadata| {
            SpecializedValueShape::instantiate(
                &crate::plan::ValueShape::from_value_type(metadata.materialize()),
                &SpecializedTypeSubstitution::empty(),
            )
        })
        .collect();
    let mut sealing = NativeSealing {
        context,
        rules: resolved_rules,
        customs,
        ids: HashMap::new(),
        pending: VecDeque::new(),
        nodes: Vec::new(),
        sources: source_shapes,
        position,
        definitions,
    };
    let roots = targets
        .iter()
        .map(|target| sealing.intern(instantiate(target)))
        .collect();
    while let Some(shape) = sealing.pending.pop_front() {
        let type_ = shape.to_module_shape().value_type();
        let kind = sealing.lower(shape);
        sealing.nodes.push(NativeConversion::new(type_, kind));
    }
    Ok(NativeConversions::new(
        roots,
        sealing.nodes.into_boxed_slice(),
    ))
}

impl NativeSealing<'_> {
    fn intern(&mut self, shape: SpecializedValueShape) -> NativeConversionId {
        let type_ = shape.to_module_shape().value_type();
        if let Some(id) = self.ids.get(&type_) {
            return *id;
        }
        let id = NativeConversionId::new(self.ids.len());
        self.ids.insert(type_, id);
        self.pending.push_back(shape);
        id
    }

    fn lower(&mut self, shape: SpecializedValueShape) -> NativeConversionKind {
        if let Some(rule) = self.rules.get(&shape.to_module_shape().value_type()) {
            return NativeConversionKind::External { rule: *rule };
        }
        match shape {
            SpecializedValueShape::Int => NativeConversionKind::Int,
            SpecializedValueShape::Float => NativeConversionKind::Float,
            SpecializedValueShape::String => NativeConversionKind::String,
            SpecializedValueShape::BitArray => NativeConversionKind::BitArray,
            SpecializedValueShape::UtfCodepoint => NativeConversionKind::UtfCodepoint,
            SpecializedValueShape::Bool => NativeConversionKind::Bool,
            SpecializedValueShape::Nil => NativeConversionKind::Nil,
            SpecializedValueShape::Tuple(items) => NativeConversionKind::Tuple(
                items
                    .into_vec()
                    .into_iter()
                    .map(|item| self.intern(item))
                    .collect(),
            ),
            SpecializedValueShape::List(item) => {
                let storage = self.context.types.list_type(&item);
                NativeConversionKind::List {
                    storage,
                    item: self.intern(*item),
                }
            }
            SpecializedValueShape::Custom(custom) => {
                let nominal = ValueType::Custom(custom.to_module_shape().type_().clone());
                if let Some(constructors) = self.customs.remove(&nominal) {
                    self.custom(constructors)
                } else {
                    self.custom_views(custom)
                }
            }
            SpecializedValueShape::Function(shape) => self.function_views(*shape),
            SpecializedValueShape::Parameter(_) | SpecializedValueShape::External(_) => {
                NativeConversionKind::Exact
            }
        }
    }

    fn function_views(&mut self, target: SpecializedFunctionShape) -> NativeConversionKind {
        let mut views = Vec::new();
        for source in self.sources.clone() {
            let SpecializedValueShape::Function(source) = source else {
                continue;
            };
            if *source == target || source.arguments().len() != target.arguments().len() {
                continue;
            }
            let Some(parameters) = target
                .arguments()
                .iter()
                .map(
                    |shape| match self.context.representations.inhabitation(shape) {
                        ValueInhabitation::Inhabited(shape) => Some(shape),
                        ValueInhabitation::Uninhabited(_) => None,
                    },
                )
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            if source
                .arguments()
                .iter()
                .any(|shape| !self.context.representations.is_inhabited(shape))
            {
                continue;
            }
            let return_ = match self.context.representations.inhabitation(target.return_()) {
                ValueInhabitation::Inhabited(shape) => Some(shape),
                ValueInhabitation::Uninhabited(_) => None,
            };
            let capture = StoredValueShape::Function(source.clone());
            let arguments = source
                .arguments()
                .iter()
                .cloned()
                .map(|argument| self.intern(argument))
                .collect();
            let result = self.intern(target.return_().clone());
            let family = return_
                .as_ref()
                .map(|return_| {
                    function::stored_function_table_family(return_, &self.context.representations)
                })
                .unwrap_or(function::FunctionTableFamily::Never);
            let index = self.context.next_function_index(family);
            let runtime_target = return_
                .as_ref()
                .map(|return_| {
                    function::function_id(
                        return_,
                        index,
                        &mut self.context.types,
                        &self.context.representations,
                    )
                })
                .unwrap_or_else(|| {
                    RuntimeFunctionId::Core(CoreRuntimeFunctionId::Never(NeverFunctionId(index)))
                });
            let mut prefix = local::ParameterPrefix::default();
            let slots = local::parameter_slots(&parameters, &mut prefix, self.context);
            let captures =
                local::parameter_slots(std::slice::from_ref(&capture), &mut prefix, self.context);
            let source_metadata = FunctionMetadata::from_public(&source.to_module_shape().type_());
            let host_value = return_.is_some();
            let host = if host_value {
                self.position.first_value
            } else {
                self.position.first_never
            } + self
                .context
                .native_views
                .iter()
                .filter(|view| {
                    self.context
                        .representations
                        .is_inhabited(view.shape.return_())
                        == host_value
                })
                .count();
            views.push(NativeFunctionView {
                source: source_metadata.clone(),
                target: runtime_target,
                type_: self.context.lower_concrete_function_type(&target),
                captures: captures.clone().into(),
                host,
                host_value,
            });
            self.context.native_views.push(NativeViewDraft {
                index,
                family,
                shape: target.clone(),
                parameters: HostedFunctionParameters {
                    call: slots
                        .into_iter()
                        .map(|slot| HostCallParameter::Value(slot.local))
                        .collect(),
                    captures: captures.into(),
                },
                parameter_shapes: parameters.into_boxed_slice(),
                captures: Box::new([capture]),
                operation: HostNativeView {
                    parent: self.position.parent,
                    parent_value: self.position.parent_value,
                    source: source_metadata,
                    arguments,
                    return_: result,
                },
            });
        }
        if views.is_empty() {
            NativeConversionKind::Exact
        } else {
            NativeConversionKind::Function(views.into())
        }
    }

    fn custom(&mut self, constructors: Box<[NativeCustomConstruction]>) -> NativeConversionKind {
        NativeConversionKind::Custom(
            constructors
                .into_iter()
                .map(|constructor| {
                    let fields = constructor
                        .fields
                        .into_iter()
                        .map(|field| self.intern(field))
                        .collect();
                    NativeConstructor::new(constructor.constructor, constructor.tag, fields)
                })
                .collect(),
        )
    }

    fn custom_views(&mut self, target: SpecializedCustomValueShape) -> NativeConversionKind {
        let target_type = target.to_module_shape().type_().clone();
        let name = target_type.type_name();
        // A retained custom view needs both a declaration and an explicit target
        // grant. Otherwise the custom remains an exact opaque value.
        let Some(definition) = self.definitions.iter().find(|definition| {
            definition.identity()
                == (
                    name.package().as_str(),
                    name.module().as_str(),
                    name.name().as_str(),
                )
                && self.sources.iter().any(|shape| {
                    shape.to_module_shape().value_type() == ValueType::Custom(target_type.clone())
                })
        }) else {
            return NativeConversionKind::Exact;
        };
        let sources = self
            .sources
            .iter()
            .filter_map(|shape| {
                let SpecializedValueShape::Custom(source) = shape else {
                    return None;
                };
                let source_type = source.to_module_shape().type_().clone();
                (source_type.type_name() == target_type.type_name() && source_type != target_type)
                    .then_some(source_type)
            })
            .collect::<Vec<_>>();
        if sources.is_empty() {
            return NativeConversionKind::Exact;
        }
        let definition = definition.clone();
        let arguments = target_type
            .arguments()
            .iter()
            .map(TypeMetadata::from_public)
            .collect::<Vec<_>>();
        let mut constructors = Vec::new();
        for (index, constructor) in definition.constructors.iter().enumerate() {
            let fields = constructor
                .fields
                .iter()
                .map(|field| {
                    crate::plan::CustomConstructorField::new(
                        field.label.as_ref().map(|label| label.as_str().into()),
                        substitute(&field.type_, &arguments).materialize(),
                    )
                })
                .collect::<Vec<_>>();
            let native_fields = fields
                .iter()
                .map(|field| {
                    SpecializedValueShape::instantiate(
                        &crate::plan::ValueShape::from_value_type(field.type_().clone()),
                        &SpecializedTypeSubstitution::empty(),
                    )
                })
                .collect::<Vec<_>>();
            let specialized = SpecializedCustomConstructor::instantiate(
                crate::plan::CustomConstructor::new(
                    target_type.clone(),
                    constructor.name.as_str().into(),
                    index,
                    fields,
                ),
                &SpecializedTypeSubstitution::empty(),
                &self.context.representations,
            );
            let constructor_id = self.context.types.custom_constructor(specialized);
            let fields = native_fields
                .into_iter()
                .map(|field| self.intern(field))
                .collect();
            constructors.push(NativeConstructor::new(
                constructor_id,
                gleam_compiler_core::strings::to_snake_case(&constructor.name),
                fields,
            ));
        }
        NativeConversionKind::CustomView(
            sources
                .into_iter()
                .map(|source| NativeCustomView {
                    source: TypeMetadata::from_public(&ValueType::Custom(source)),
                    constructors: constructors.clone().into(),
                })
                .collect(),
        )
    }
}
