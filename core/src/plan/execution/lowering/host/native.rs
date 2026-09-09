use super::super::LoweringContext;
use super::super::specialization::{SpecializationKey, SpecializedValueShape};
use crate::host::HostTypeDescriptor;
use crate::plan::execution::host::{
    HostSpecializationError, NativeConstructor, NativeConversion, NativeConversionId,
    NativeConversionKind, NativeConversions,
};
use crate::plan::{HostFunctionTemplate, ValueType};
use std::collections::{HashMap, VecDeque};

struct NativeSealing<'context> {
    context: &'context mut LoweringContext,
    rules: HashMap<ValueType, usize>,
    customs: HashMap<ValueType, Box<[NativeCustomConstruction]>>,
    ids: HashMap<ValueType, NativeConversionId>,
    pending: VecDeque<SpecializedValueShape>,
    nodes: Vec<NativeConversion>,
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
    targets: &[HostTypeDescriptor],
    rules: &[HostTypeDescriptor],
    key: &SpecializationKey,
    context: &mut LoweringContext,
    customs: HashMap<ValueType, Box<[NativeCustomConstruction]>>,
) -> Result<NativeConversions, HostSpecializationError> {
    let instantiate = |descriptor: &HostTypeDescriptor| {
        SpecializedValueShape::instantiate(&descriptor.value_shape(), key.substitution())
    };
    let mut resolved_rules = HashMap::new();
    for (index, rule) in rules.iter().enumerate() {
        let type_ = instantiate(rule).to_module_shape().value_type();
        if resolved_rules.insert(type_.clone(), index).is_some() {
            return Err(HostSpecializationError::conflicting_native_conversions(
                template.package().clone(),
                template.site().module().clone(),
                template.site().function().clone(),
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
    let mut sealing = NativeSealing {
        context,
        rules: resolved_rules,
        customs,
        ids: HashMap::new(),
        pending: VecDeque::new(),
        nodes: Vec::new(),
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
                self.custom(ValueType::Custom(custom.to_module_shape().type_().clone()))
            }
            SpecializedValueShape::Parameter(_)
            | SpecializedValueShape::Function(_)
            | SpecializedValueShape::External(_) => NativeConversionKind::Exact,
        }
    }

    fn custom(&mut self, type_: ValueType) -> NativeConversionKind {
        let Some(constructors) = self.customs.remove(&type_) else {
            return NativeConversionKind::Exact;
        };
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
}
