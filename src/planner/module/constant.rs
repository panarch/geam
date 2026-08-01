use crate::plan::{
    ConstantBitArraySegment, ConstantListConstructionError, ConstantTemplate, ConstantTemplateId,
    ConstantTemplateSignature, ConstantTemplates, ConstantValue, CustomConstructorRefinement,
    CustomValueShape, Endianness, FloatBitSize, FunctionShape, StringEncoding, TypeScheme,
    ValueShape, ValueType,
};
use crate::planner::context::{AnonymousFunctions, ModuleFunctionTarget, PlanContext};
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    UnsupportedBitArraySegmentReason,
};
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use gleam_core::ast::{
    BitArrayOption, BitArraySegment as GleamBitArraySegment, Constant, TypedModuleConstant,
};
use gleam_core::type_::{PRELUDE_MODULE_NAME, Type, ValueConstructor, ValueConstructorVariant};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub(in crate::planner) struct ConstantSignatures {
    by_name: HashMap<EcoString, usize>,
    entries: Vec<ConstantTemplateSignature>,
}

pub(in crate::planner) struct ConstantDeclarations {
    signatures: ConstantSignatures,
    bodies: ConstantBodies,
}

pub(in crate::planner) struct ConstantBodies {
    module: crate::plan::ModuleId,
    seeds: Vec<ConstantSeed>,
}

struct ConstantSeed {
    id: ConstantTemplateId,
    name: EcoString,
    value: Constant<Arc<Type>>,
    type_parameters: TypeParameterScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConstantStorageFamily {
    GenericList,
    Int,
    IntList,
    String,
    StringList,
    BitArray,
    BitArrayList,
    UtfCodepointList,
    Custom,
    CustomList,
    ExternalList,
    Float,
    FloatList,
    Bool,
    BoolList,
    Nil,
    NilList,
    Tuple,
    TupleList,
    ParameterListList,
    ListList,
    FunctionList,
    GenericFunction,
    IntFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    ExternalFunction,
    FloatFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    ListFunction,
    FunctionFunction,
}

#[derive(Debug, PartialEq, Eq)]
enum ConstantStorageShape {
    Int,
    String,
    BitArray,
    Custom(CustomValueShape),
    Float,
    Bool,
    Nil,
    Tuple(Box<[ValueShape]>),
    List(Box<ValueShape>),
    Function(FunctionShape),
}

#[derive(Default)]
struct ConstantStorageIndices {
    next: HashMap<ConstantStorageFamily, usize>,
}

pub(in crate::planner) fn reserve_constants(
    module: crate::plan::ModuleId,
    constants: Vec<TypedModuleConstant>,
) -> Result<ConstantDeclarations, PlanError> {
    reserve_constants_with_external_types(module, constants, &std::collections::HashSet::new())
}

pub(in crate::planner) fn reserve_constants_with_external_types(
    module: crate::plan::ModuleId,
    constants: Vec<TypedModuleConstant>,
    external_types: &std::collections::HashSet<crate::plan::ExternalTypeName>,
) -> Result<ConstantDeclarations, PlanError> {
    let mut seeds = Vec::with_capacity(constants.len());
    let mut names = HashMap::with_capacity(constants.len());
    let mut signatures = Vec::with_capacity(constants.len());
    let mut storage_indices = ConstantStorageIndices::default();

    for (index, constant) in constants.into_iter().enumerate() {
        let mut type_parameters = TypeParameterScope::default();
        let storage_shape =
            ConstantStorageShape::try_from_shape(ValueShape::from_gleam_in_with_external(
                &constant.type_,
                &mut type_parameters,
                &|name| external_types.contains(name),
            ))
            .ok_or_else(invalid_constant_shape_error)?;
        let storage_index = storage_indices.reserve(storage_shape.family());
        let id = ConstantTemplateId::in_module(module, index);
        let signature = storage_shape.into_signature(id, storage_index, type_parameters.scheme());
        names.insert(constant.name.clone(), index);
        signatures.push(signature);
        seeds.push(ConstantSeed {
            id,
            name: constant.name,
            value: *constant.value,
            type_parameters,
        });
    }

    Ok(ConstantDeclarations {
        signatures: ConstantSignatures {
            by_name: names,
            entries: signatures,
        },
        bodies: ConstantBodies { module, seeds },
    })
}

pub(in crate::planner) fn plan_constant_bodies(
    bodies: ConstantBodies,
    registry: &super::registry::ProgramRegistry,
    anonymous_functions: &mut AnonymousFunctions,
) -> Result<ConstantTemplates, PlanError> {
    let module = bodies.module;
    let mut entries = Vec::with_capacity(bodies.seeds.len());

    for seed in bodies.seeds {
        let signature = registry.constant_signature(seed.id);
        let mut context = PlanContext::new_in_program(module, registry, anonymous_functions);
        context.set_current_function(seed.name.clone());
        context.set_type_parameters(seed.type_parameters);
        let value = plan_value(seed.value, &mut context)?;
        if !value.shape().can_flow_to(signature.shape()) {
            return Err(invalid_expression_type_for_value(
                signature.shape().value_type(),
                value.shape().value_type(),
            ));
        }
        entries.push((ConstantTemplate::new(signature.clone(), seed.name), value));
    }

    Ok(ConstantTemplates::from_module_entries(module, entries))
}

impl ConstantDeclarations {
    pub(in crate::planner) fn into_parts(self) -> (ConstantSignatures, ConstantBodies) {
        (self.signatures, self.bodies)
    }
}

impl ConstantSignatures {
    pub(in crate::planner) fn signature(
        &self,
        name: &EcoString,
    ) -> Option<&ConstantTemplateSignature> {
        Some(&self.entries[*self.by_name.get(name)?])
    }

    pub(in crate::planner) fn get(&self, id: ConstantTemplateId) -> &ConstantTemplateSignature {
        &self.entries[id.index()]
    }

    #[cfg(test)]
    fn empty() -> Self {
        Self::default()
    }
}

impl ConstantStorageShape {
    fn try_from_shape(shape: ValueShape) -> Option<Self> {
        Some(match shape {
            ValueShape::Parameter(_) | ValueShape::UtfCodepoint | ValueShape::External(_) => {
                return None;
            }
            ValueShape::Int => Self::Int,
            ValueShape::String => Self::String,
            ValueShape::BitArray => Self::BitArray,
            ValueShape::Custom(shape) => Self::Custom(shape),
            ValueShape::Float => Self::Float,
            ValueShape::Bool => Self::Bool,
            ValueShape::Nil => Self::Nil,
            ValueShape::Tuple(shape) => Self::Tuple(shape),
            ValueShape::List(item) => Self::List(item),
            ValueShape::Function(shape) => Self::Function(*shape),
        })
    }

    fn family(&self) -> ConstantStorageFamily {
        match self {
            Self::Int => ConstantStorageFamily::Int,
            Self::String => ConstantStorageFamily::String,
            Self::BitArray => ConstantStorageFamily::BitArray,
            Self::Custom(_) => ConstantStorageFamily::Custom,
            Self::Float => ConstantStorageFamily::Float,
            Self::Bool => ConstantStorageFamily::Bool,
            Self::Nil => ConstantStorageFamily::Nil,
            Self::Tuple(_) => ConstantStorageFamily::Tuple,
            Self::Function(shape) => match shape.return_shape() {
                ValueShape::Parameter(_) => ConstantStorageFamily::GenericFunction,
                ValueShape::Int => ConstantStorageFamily::IntFunction,
                ValueShape::String => ConstantStorageFamily::StringFunction,
                ValueShape::BitArray => ConstantStorageFamily::BitArrayFunction,
                ValueShape::UtfCodepoint => ConstantStorageFamily::UtfCodepointFunction,
                ValueShape::Custom(_) => ConstantStorageFamily::CustomFunction,
                ValueShape::External(_) => ConstantStorageFamily::ExternalFunction,
                ValueShape::Float => ConstantStorageFamily::FloatFunction,
                ValueShape::Bool => ConstantStorageFamily::BoolFunction,
                ValueShape::Nil => ConstantStorageFamily::NilFunction,
                ValueShape::Tuple(_) => ConstantStorageFamily::TupleFunction,
                ValueShape::List(_) => ConstantStorageFamily::ListFunction,
                ValueShape::Function(_) => ConstantStorageFamily::FunctionFunction,
            },
            Self::List(item) => match item.as_ref() {
                ValueShape::Parameter(_) => ConstantStorageFamily::GenericList,
                ValueShape::Int => ConstantStorageFamily::IntList,
                ValueShape::String => ConstantStorageFamily::StringList,
                ValueShape::BitArray => ConstantStorageFamily::BitArrayList,
                ValueShape::UtfCodepoint => ConstantStorageFamily::UtfCodepointList,
                ValueShape::Custom(_) => ConstantStorageFamily::CustomList,
                ValueShape::External(_) => ConstantStorageFamily::ExternalList,
                ValueShape::Float => ConstantStorageFamily::FloatList,
                ValueShape::Bool => ConstantStorageFamily::BoolList,
                ValueShape::Nil => ConstantStorageFamily::NilList,
                ValueShape::Tuple(_) => ConstantStorageFamily::TupleList,
                ValueShape::List(item) => match item.representation() {
                    crate::plan::ValueRepresentation::Uninhabited(_) => {
                        ConstantStorageFamily::ParameterListList
                    }
                    crate::plan::ValueRepresentation::Stored(_) => ConstantStorageFamily::ListList,
                },
                ValueShape::Function(_) => ConstantStorageFamily::FunctionList,
            },
        }
    }

    fn into_signature(
        self,
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
    ) -> ConstantTemplateSignature {
        match self {
            Self::Int => ConstantTemplateSignature::int(id, storage_index, scheme),
            Self::String => ConstantTemplateSignature::string(id, storage_index, scheme),
            Self::BitArray => ConstantTemplateSignature::bit_array(id, storage_index, scheme),
            Self::Custom(shape) => {
                ConstantTemplateSignature::custom(id, storage_index, scheme, shape)
            }
            Self::Float => ConstantTemplateSignature::float(id, storage_index, scheme),
            Self::Bool => ConstantTemplateSignature::bool(id, storage_index, scheme),
            Self::Nil => ConstantTemplateSignature::nil(id, storage_index, scheme),
            Self::Tuple(shape) => {
                ConstantTemplateSignature::tuple(id, storage_index, scheme, shape)
            }
            Self::List(item_shape) => {
                ConstantTemplateSignature::list(id, storage_index, scheme, *item_shape)
            }
            Self::Function(shape) => {
                ConstantTemplateSignature::function(id, storage_index, scheme, shape)
            }
        }
    }
}

impl ConstantStorageIndices {
    fn reserve(&mut self, family: ConstantStorageFamily) -> usize {
        let next = self.next.entry(family).or_default();
        let index = *next;
        *next += 1;
        index
    }
}

fn plan_value(
    value: Constant<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<ConstantValue, PlanError> {
    let shape = context.value_shape(value.type_().as_ref());
    match value {
        Constant::Int { int_value, .. } => Ok(ConstantValue::int(int_value)),
        Constant::Float { float_value, .. } => Ok(ConstantValue::float(float_value.value())),
        Constant::String { value, .. } => Ok(ConstantValue::string(value)),
        Constant::StringConcatenation { left, right, .. } => {
            let left = into_string(plan_value(*left, context)?)?;
            let right = into_string(plan_value(*right, context)?)?;
            Ok(ConstantValue::string_concatenation(left, right))
        }
        Constant::Tuple { elements, .. } => {
            let ValueShape::Tuple(element_shapes) = shape else {
                return Err(invalid_expression_type_for_value(
                    ValueType::Tuple(Vec::new()),
                    shape.value_type(),
                ));
            };
            if elements.len() != element_shapes.len() {
                return Err(invalid_constant_shape_error());
            }
            let mut planned_elements = Vec::with_capacity(elements.len());
            for (element, expected) in elements.into_iter().zip(&element_shapes) {
                let element = plan_value(element, context)?;
                require_shape(&element, expected)?;
                planned_elements.push(element);
            }
            Ok(ConstantValue::tuple(
                element_shapes,
                planned_elements.into_boxed_slice(),
            ))
        }
        Constant::List { elements, tail, .. } => {
            let ValueShape::List(item_shape) = shape else {
                return Err(invalid_expression_type_for_value(
                    ValueType::List(Box::new(ValueType::Nil)),
                    shape.value_type(),
                ));
            };
            let elements = elements
                .into_iter()
                .map(|element| plan_value(element, context))
                .collect::<Result<Vec<_>, _>>()?;
            let tail = tail.map(|tail| plan_value(*tail, context)).transpose()?;
            match ConstantValue::try_list(*item_shape, elements, tail) {
                Ok(value) => Ok(value),
                Err(ConstantListConstructionError::TypeMismatch { expected, actual }) => {
                    Err(invalid_expression_type_for_value(expected, actual))
                }
                Err(ConstantListConstructionError::SpreadWithoutElements) => {
                    Err(invalid_constant_shape_error())
                }
            }
        }
        Constant::BitArray { segments, .. } => {
            let segments = segments
                .into_iter()
                .map(|segment| plan_bit_array_segment(segment, context))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConstantValue::bit_array(segments.into_boxed_slice()))
        }
        Constant::Var {
            constructor, name, ..
        } => plan_var(
            name,
            constructor.map(|constructor| *constructor),
            shape,
            context,
        ),
        Constant::Record {
            arguments,
            record_constructor,
            ..
        } => plan_record(
            arguments,
            record_constructor.map(|constructor| *constructor),
            shape,
            context,
        ),
        Constant::RecordUpdate { .. } => Err(invalid_expression_shape_error(
            InvalidExpressionShapeKind::RecordUpdate,
        )),
        Constant::Todo { .. } | Constant::Invalid { .. } => Err(invalid_constant_shape_error()),
    }
}

fn plan_var(
    name: EcoString,
    constructor: Option<ValueConstructor>,
    shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<ConstantValue, PlanError> {
    let Some(constructor) = constructor else {
        return Err(invalid_constant_shape_error());
    };

    match &constructor.variant {
        ValueConstructorVariant::ModuleConstant { module, .. } => {
            let instantiation = context.module_constant_instantiation(module, &name, &shape)?;
            Ok(ConstantValue::reference(instantiation))
        }
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } => {
            let target = ModuleFunctionTarget::direct(
                module.clone(),
                name.clone(),
                external_erlang.is_some() || external_javascript.is_some(),
            )
            .validate_external(context)?;
            let actual = target.function_shape(shape.clone())?;
            let function = context.module_function(&target)?;
            let instantiation = target.instantiate_reference(&function, &actual)?;
            Ok(ConstantValue::function(
                actual,
                function.reference(instantiation),
            ))
        }
        ValueConstructorVariant::Record { .. } => {
            plan_record(None, Some(constructor), shape, context)
        }
        ValueConstructorVariant::LocalVariable { .. } => Err(invalid_constant_shape_error()),
    }
}

fn plan_record(
    arguments: Option<Vec<gleam_core::ast::CallArg<Constant<Arc<Type>>>>>,
    constructor: Option<ValueConstructor>,
    shape: ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<ConstantValue, PlanError> {
    let Some(constructor) = constructor else {
        return Err(invalid_expression_shape_error(
            InvalidExpressionShapeKind::RecordConstructor,
        ));
    };
    let ValueConstructorVariant::Record {
        name,
        module,
        arity,
        ..
    } = &constructor.variant
    else {
        return Err(invalid_constant_shape_error());
    };

    if module == PRELUDE_MODULE_NAME && *arity == 0 {
        return match (name.as_str(), &shape) {
            ("True", ValueShape::Bool) => Ok(ConstantValue::bool(true)),
            ("False", ValueShape::Bool) => Ok(ConstantValue::bool(false)),
            ("Nil", ValueShape::Nil) => Ok(ConstantValue::nil()),
            _ => Err(invalid_expression_shape_error(
                InvalidExpressionShapeKind::PreludeConstructor,
            )),
        };
    }
    if module == PRELUDE_MODULE_NAME && !matches!(name.as_str(), "Ok" | "Error") {
        return Err(invalid_expression_shape_error(
            InvalidExpressionShapeKind::RecordConstructor,
        ));
    }
    let constructor = context.custom_constructor(&constructor)?;

    let Some(arguments) = arguments else {
        return match shape {
            ValueShape::Custom(custom) if constructor.fields().is_empty() => {
                Ok(ConstantValue::custom(
                    exact_constructor_shape(custom, constructor.index()),
                    constructor,
                    Vec::new().into_boxed_slice(),
                ))
            }
            ValueShape::Function(shape) if !constructor.fields().is_empty() => {
                let ValueShape::Custom(return_) = shape.return_shape().clone() else {
                    return Err(invalid_expression_shape_error(
                        InvalidExpressionShapeKind::RecordConstructor,
                    ));
                };
                Ok(ConstantValue::constructor_function(
                    *shape,
                    return_,
                    constructor,
                ))
            }
            _ => Err(invalid_expression_shape_error(
                InvalidExpressionShapeKind::RecordConstructor,
            )),
        };
    };

    if arguments.len() != constructor.fields().len() {
        return Err(invalid_expression_shape_error(
            InvalidExpressionShapeKind::RecordConstructor,
        ));
    }
    let mut fields = Vec::with_capacity(arguments.len());
    for (argument, field) in arguments.into_iter().zip(constructor.fields()) {
        if let Some(label) = &argument.label
            && field.label() != Some(label)
        {
            return Err(invalid_expression_shape_error(
                InvalidExpressionShapeKind::RecordConstructor,
            ));
        }
        let value = plan_value(argument.value, context)?;
        if value.shape().value_type() != *field.type_() {
            return Err(invalid_expression_type_for_value(
                field.type_().clone(),
                value.shape().value_type(),
            ));
        }
        fields.push(value);
    }
    let ValueShape::Custom(custom) = shape else {
        return Err(invalid_expression_shape_error(
            InvalidExpressionShapeKind::RecordConstructor,
        ));
    };
    Ok(ConstantValue::custom(
        exact_constructor_shape(custom, constructor.index()),
        constructor,
        fields.into_boxed_slice(),
    ))
}

fn exact_constructor_shape(shape: CustomValueShape, index: usize) -> CustomValueShape {
    CustomValueShape::new(
        shape.type_name().clone(),
        shape.arguments().to_vec(),
        CustomConstructorRefinement::Exact(index),
    )
}

fn plan_bit_array_segment(
    segment: GleamBitArraySegment<Constant<Arc<Type>>, Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<ConstantBitArraySegment, PlanError> {
    let value = plan_value(*segment.value, context)?;
    let options = static_segment_options(segment.options)?;
    let site = context.panic_site(segment.location);
    let kind = match options.kind {
        Some(kind) => kind,
        None => match value.shape().value_type() {
            ValueType::Int => StaticSegmentKind::Int,
            ValueType::Float => StaticSegmentKind::Float,
            _ => return Err(invalid_bit_array_option_error()),
        },
    };

    match kind {
        StaticSegmentKind::Int => {
            let bit_size = match options.size {
                Some(size) => fixed_bit_size(size, options.unit)?,
                None => 8 * usize::from(options.unit),
            };
            Ok(ConstantBitArraySegment::Int {
                value: into_int(value)?,
                bit_size,
                endianness: options.endianness,
            })
        }
        StaticSegmentKind::Float => {
            let bit_size = match options.size {
                Some(size) => float_bit_size(fixed_bit_size(size, options.unit)?)?,
                None => FloatBitSize::SixtyFour,
            };
            Ok(ConstantBitArraySegment::Float {
                value: into_float(value)?,
                bit_size,
                endianness: options.endianness,
            })
        }
        StaticSegmentKind::Bits => match options.size {
            Some(size) => Ok(ConstantBitArraySegment::SizedBits {
                value: into_bit_array(value)?,
                bit_size: fixed_bit_size(size, options.unit)?,
                site,
            }),
            None => Ok(ConstantBitArraySegment::Bits(into_bit_array(value)?)),
        },
        StaticSegmentKind::String(encoding) => {
            if options.size.is_some() || options.unit != 1 {
                return Err(invalid_bit_array_option_error());
            }
            Ok(ConstantBitArraySegment::String {
                value: into_string(value)?,
                encoding: encoding_with_endianness(encoding, options.endianness),
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StaticSegmentKind {
    Int,
    Float,
    Bits,
    String(StringEncoding),
}

#[derive(Debug, PartialEq, Eq)]
struct StaticSegmentOptions {
    kind: Option<StaticSegmentKind>,
    endianness: Endianness,
    size: Option<BigInt>,
    unit: u8,
}

fn static_segment_options(
    options: Vec<BitArrayOption<Constant<Arc<Type>>>>,
) -> Result<StaticSegmentOptions, PlanError> {
    let mut planned = StaticSegmentOptions {
        kind: None,
        endianness: Endianness::Big,
        size: None,
        unit: 1,
    };
    for option in options {
        match option {
            BitArrayOption::Int { .. } => {
                set_segment_kind(&mut planned.kind, StaticSegmentKind::Int)?
            }
            BitArrayOption::Float { .. } => {
                set_segment_kind(&mut planned.kind, StaticSegmentKind::Float)?
            }
            BitArrayOption::Bits { .. } => {
                set_segment_kind(&mut planned.kind, StaticSegmentKind::Bits)?
            }
            BitArrayOption::Utf8 { .. } => set_segment_kind(
                &mut planned.kind,
                StaticSegmentKind::String(StringEncoding::Utf8),
            )?,
            BitArrayOption::Utf16 { .. } => set_segment_kind(
                &mut planned.kind,
                StaticSegmentKind::String(StringEncoding::Utf16(Endianness::Big)),
            )?,
            BitArrayOption::Utf32 { .. } => set_segment_kind(
                &mut planned.kind,
                StaticSegmentKind::String(StringEncoding::Utf32(Endianness::Big)),
            )?,
            BitArrayOption::Utf8Codepoint { .. }
            | BitArrayOption::Utf16Codepoint { .. }
            | BitArrayOption::Utf32Codepoint { .. } => {
                return Err(invalid_bit_array_option_error());
            }
            BitArrayOption::Big { .. } => planned.endianness = Endianness::Big,
            BitArrayOption::Little { .. } => planned.endianness = Endianness::Little,
            BitArrayOption::Unit { value, .. } => planned.unit = value,
            BitArrayOption::Size { value, .. } => {
                let Constant::Int { int_value, .. } = *value else {
                    return Err(invalid_bit_array_option_error());
                };
                planned.size = Some(int_value);
            }
            BitArrayOption::Native { .. } => {
                return Err(PlanError::UnsupportedBitArraySegment {
                    reason: UnsupportedBitArraySegmentReason::NativeEndianness,
                });
            }
            BitArrayOption::Bytes { .. }
            | BitArrayOption::Signed { .. }
            | BitArrayOption::Unsigned { .. } => {
                return Err(invalid_bit_array_option_error());
            }
        }
    }
    Ok(planned)
}

fn set_segment_kind(
    current: &mut Option<StaticSegmentKind>,
    kind: StaticSegmentKind,
) -> Result<(), PlanError> {
    if current.replace(kind).is_some() {
        return Err(invalid_bit_array_option_error());
    }
    Ok(())
}

fn encoding_with_endianness(encoding: StringEncoding, endianness: Endianness) -> StringEncoding {
    match encoding {
        StringEncoding::Utf8 => StringEncoding::Utf8,
        StringEncoding::Utf16(_) => StringEncoding::Utf16(endianness),
        StringEncoding::Utf32(_) => StringEncoding::Utf32(endianness),
    }
}

fn fixed_bit_size(value: BigInt, unit: u8) -> Result<usize, PlanError> {
    let value = if value < BigInt::from(0) {
        BigInt::from(0)
    } else {
        value
    };
    match usize::try_from(value * BigInt::from(unit)) {
        Ok(bit_size) => Ok(bit_size),
        Err(_) => Err(PlanError::UnsupportedBitArraySegment {
            reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
        }),
    }
}

fn float_bit_size(bit_size: usize) -> Result<FloatBitSize, PlanError> {
    match bit_size {
        16 => Ok(FloatBitSize::Sixteen),
        32 => Ok(FloatBitSize::ThirtyTwo),
        64 => Ok(FloatBitSize::SixtyFour),
        _ => Err(invalid_bit_array_option_error()),
    }
}

fn require_shape(value: &ConstantValue, expected: &ValueShape) -> Result<(), PlanError> {
    if value.shape().can_flow_to(expected) {
        Ok(())
    } else {
        Err(invalid_expression_type_for_value(
            expected.value_type(),
            value.shape().value_type(),
        ))
    }
}

fn into_int(value: ConstantValue) -> Result<crate::plan::ConstantIntValue, PlanError> {
    let actual = value.shape().value_type();
    match value.into_int() {
        Some(value) => Ok(value),
        None => Err(invalid_expression_type_for_value(ValueType::Int, actual)),
    }
}

fn into_float(value: ConstantValue) -> Result<crate::plan::ConstantFloatValue, PlanError> {
    let actual = value.shape().value_type();
    match value.into_float() {
        Some(value) => Ok(value),
        None => Err(invalid_expression_type_for_value(ValueType::Float, actual)),
    }
}

fn into_string(value: ConstantValue) -> Result<crate::plan::ConstantStringValue, PlanError> {
    let actual = value.shape().value_type();
    match value.into_string() {
        Some(value) => Ok(value),
        None => Err(invalid_expression_type_for_value(ValueType::String, actual)),
    }
}

fn into_bit_array(value: ConstantValue) -> Result<crate::plan::ConstantBitArrayValue, PlanError> {
    let actual = value.shape().value_type();
    match value.into_bit_array() {
        Some(value) => Ok(value),
        None => Err(invalid_expression_type_for_value(
            ValueType::BitArray,
            actual,
        )),
    }
}

fn invalid_bit_array_option_error() -> PlanError {
    invalid_expression_shape_error(InvalidExpressionShapeKind::BitArraySegmentOption)
}

fn invalid_constant_shape_error() -> PlanError {
    invalid_expression_shape_error(InvalidExpressionShapeKind::Invalid)
}

fn invalid_expression_shape_error(kind: InvalidExpressionShapeKind) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape { kind },
    }
}

fn invalid_expression_type_for_value(expected: ValueType, actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::from_value_type(expected),
            actual: InvalidExpressionType::from_value_type(actual),
        },
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{
        ConstantSignatures, ConstantStorageFamily, ConstantStorageIndices, ConstantStorageShape,
        StaticSegmentKind, StaticSegmentOptions, plan_bit_array_segment, plan_value,
        static_segment_options,
    };
    use crate::plan::{
        ConstantBitArraySegment, CustomType, CustomTypeName, CustomValueShape, Endianness,
        FunctionShape, FunctionTemplateId, FunctionTemplateSignature, IntLocalId, PanicSite,
        ParamBinding, ParamLocal, SourceSpan, StringEncoding, TypeParameterId, TypeScheme,
        ValueShape,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, FunctionParam, PlanContext};
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidModuleReferenceReason,
        InvalidTypedAstReason, PlanError, UnsupportedBitArraySegmentReason,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::type_parameter::TypeParameterScope;
    use ecow::EcoString;
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        BitArrayOption, BitArraySegment, CallArg, Constant, Publicity, RecordBeingUpdated,
    };
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{self, Deprecation, ValueConstructor, ValueConstructorVariant};
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn reject_margin_constant_declared_shape_and_uninhabited_storage() {
        assert_eq!(
            ConstantStorageShape::try_from_shape(ValueShape::Parameter(
                crate::plan::TypeParameterId(0),
            )),
            None,
        );
        assert_eq!(
            ConstantStorageShape::try_from_shape(ValueShape::UtfCodepoint),
            None,
        );

        let signatures = ConstantSignatures::empty();
        assert_eq!(signatures.signature(&"missing".into()), None);

        let mut module = compile("const value = 1\npub fn main() { value }");
        module.definitions.constants[0].type_ = type_::string();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        let mut module = compile("const value = 1\npub fn main() { value }");
        module.definitions.constants[0].type_ = type_::generic_var(0);
        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn constant_storage_families_preserve_exact_list_item_partitions() {
        let parameter = TypeParameterId(0);
        let custom = CustomValueShape::any(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        ));
        let function = FunctionShape::new(Vec::new(), ValueShape::Int);
        let cases = [
            (
                ValueShape::Parameter(parameter),
                ConstantStorageFamily::GenericList,
            ),
            (ValueShape::Int, ConstantStorageFamily::IntList),
            (ValueShape::String, ConstantStorageFamily::StringList),
            (ValueShape::BitArray, ConstantStorageFamily::BitArrayList),
            (
                ValueShape::UtfCodepoint,
                ConstantStorageFamily::UtfCodepointList,
            ),
            (
                ValueShape::Custom(custom),
                ConstantStorageFamily::CustomList,
            ),
            (ValueShape::Float, ConstantStorageFamily::FloatList),
            (ValueShape::Bool, ConstantStorageFamily::BoolList),
            (ValueShape::Nil, ConstantStorageFamily::NilList),
            (
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                ConstantStorageFamily::TupleList,
            ),
            (
                ValueShape::List(Box::new(ValueShape::Int)),
                ConstantStorageFamily::ListList,
            ),
            (
                ValueShape::Function(Box::new(function)),
                ConstantStorageFamily::FunctionList,
            ),
        ];

        for (item, expected) in cases {
            let storage = ConstantStorageShape::try_from_shape(ValueShape::List(Box::new(item)))
                .expect("every list item shape has a constant storage family");
            assert_eq!(storage.family(), expected);
        }
    }

    #[test]
    fn constant_storage_families_preserve_function_return_partitions() {
        let parameter = TypeParameterId(0);
        let custom = CustomValueShape::any(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        ));
        let nested_function = FunctionShape::new(Vec::new(), ValueShape::Int);
        let cases = [
            (
                ValueShape::Parameter(parameter),
                ConstantStorageFamily::GenericFunction,
            ),
            (ValueShape::Int, ConstantStorageFamily::IntFunction),
            (ValueShape::String, ConstantStorageFamily::StringFunction),
            (
                ValueShape::BitArray,
                ConstantStorageFamily::BitArrayFunction,
            ),
            (
                ValueShape::UtfCodepoint,
                ConstantStorageFamily::UtfCodepointFunction,
            ),
            (
                ValueShape::Custom(custom),
                ConstantStorageFamily::CustomFunction,
            ),
            (ValueShape::Float, ConstantStorageFamily::FloatFunction),
            (ValueShape::Bool, ConstantStorageFamily::BoolFunction),
            (ValueShape::Nil, ConstantStorageFamily::NilFunction),
            (
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                ConstantStorageFamily::TupleFunction,
            ),
            (
                ValueShape::List(Box::new(ValueShape::Int)),
                ConstantStorageFamily::ListFunction,
            ),
            (
                ValueShape::Function(Box::new(nested_function)),
                ConstantStorageFamily::FunctionFunction,
            ),
        ];

        for (return_shape, expected) in cases {
            let storage = ConstantStorageShape::try_from_shape(ValueShape::Function(Box::new(
                FunctionShape::new(Vec::new(), return_shape),
            )))
            .expect("every function return shape has a constant storage family");
            assert_eq!(storage.family(), expected);
        }
    }

    #[test]
    fn constant_storage_indices_are_independent_per_function_return_family() {
        let mut indices = ConstantStorageIndices::default();

        assert_eq!(indices.reserve(ConstantStorageFamily::IntFunction), 0);
        assert_eq!(indices.reserve(ConstantStorageFamily::StringFunction), 0);
        assert_eq!(indices.reserve(ConstantStorageFamily::IntFunction), 1);
        assert_eq!(indices.reserve(ConstantStorageFamily::StringFunction), 1);
    }

    #[test]
    fn reject_margin_constant_container_shapes() {
        let invalid_shape = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        };
        let invalid = || Constant::Invalid {
            location: dummy_span(),
            type_: type_::int(),
            extra_information: None,
        };

        assert_eq!(
            plan_fixture(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(invalid()),
                right: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "right".into(),
                }),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "left".into(),
                }),
                right: Box::new(invalid()),
            }),
            Err(invalid_shape.clone()),
        );

        assert_eq!(
            plan_fixture(Constant::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_fixture(Constant::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::tuple(vec![type_::int()]),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::Tuple {
                location: dummy_span(),
                elements: vec![invalid()],
                type_: type_::tuple(vec![type_::int()]),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::Tuple {
                location: dummy_span(),
                elements: vec![int(1)],
                type_: type_::tuple(vec![type_::string()]),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_fixture(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::int(),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_fixture(Constant::List {
                location: dummy_span(),
                elements: vec![invalid()],
                type_: type_::list(type_::int()),
                tail: None,
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::List {
                location: dummy_span(),
                elements: vec![int(1)],
                type_: type_::list(type_::int()),
                tail: Some(Box::new(invalid())),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::List {
                location: dummy_span(),
                elements: vec![int(1)],
                type_: type_::list(type_::string()),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_fixture(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::list(type_::int()),
                tail: Some(Box::new(Constant::List {
                    location: dummy_span(),
                    elements: vec![int(1)],
                    type_: type_::list(type_::int()),
                    tail: None,
                })),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::RecordUpdate {
                location: dummy_span(),
                constructor_location: dummy_span(),
                module: None,
                name: "Record".into(),
                record: RecordBeingUpdated {
                    base: Box::new(int(1)),
                    location: dummy_span(),
                },
                arguments: Vec::new(),
                type_: type_::int(),
                field_map: Inferred::Unknown,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordUpdate,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_constant_reference_and_constructor_shapes() {
        let invalid_shape = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        };
        let record_shape = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordConstructor,
            },
        };

        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "missing".into(),
                constructor: None,
                type_: type_::int(),
            }),
            Err(invalid_shape.clone()),
        );

        assert_eq!(constant_var_constructor(&int(1)), None);

        let alias_module =
            compile("const first = 1\nconst second = first\npub fn main() { second }");
        let constructor = constant_var_constructor(&alias_module.definitions.constants[1].value)
            .expect("second should be a module constant reference");
        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "first".into(),
                constructor: Some(Box::new(constructor)),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "first".into(),
                    reason: InvalidModuleReferenceReason::MissingConstant,
                },
            }),
        );

        let function_module = compile(
            "fn identity(value: Int) { value }\nconst function = identity\npub fn main() { 1 }",
        );
        let function_constructor =
            constant_var_constructor(&function_module.definitions.constants[0].value)
                .expect("function should be a module function reference");
        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "identity".into(),
                constructor: Some(Box::new(function_constructor.clone())),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionType,
                },
            }),
        );

        let external_function_module = compile(
            r#"
@external(erlang, "external", "identity")
fn identity(value: Int) -> Int

const function = identity

pub fn main() {
  1
}
"#,
        );
        let external_constructor =
            constant_var_constructor(&external_function_module.definitions.constants[0].value)
                .expect("function should be an external module function reference");
        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "identity".into(),
                constructor: Some(Box::new(external_constructor)),
                type_: type_::fn_(vec![type_::int()], type_::int()),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );

        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::Int);
        let functions = HashMap::from([(
            EcoString::from("identity"),
            FunctionInfo {
                signature: FunctionTemplateSignature::new(
                    FunctionTemplateId::new(0),
                    TypeScheme::new(0),
                    function_shape,
                ),
                type_parameters: TypeParameterScope::default(),
                return_shape: ValueShape::Int,
                params: vec![FunctionParam::new(
                    ParamLocal::int(IntLocalId(0)),
                    ValueShape::Int,
                    ParamBinding::Named("value".into()),
                    None,
                )],
                definition_span: crate::plan::SourceSpan::new(0, 0),
            },
        )]);
        let module_name = EcoString::from("main");
        let mut anonymous_functions = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);
        assert_eq!(
            super::plan_var(
                "identity".into(),
                Some(function_constructor),
                ValueShape::Function(Box::new(FunctionShape::new(
                    vec![ValueShape::String],
                    ValueShape::String,
                ))),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionInstantiation,
                },
            }),
        );

        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "local".into(),
                constructor: Some(Box::new(ValueConstructor::local_variable(
                    dummy_span(),
                    VariableOrigin::generated(),
                    type_::int(),
                ))),
                type_: type_::int(),
            }),
            Err(invalid_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "True".into(),
                constructor: Some(Box::new(record_constructor(
                    "True",
                    "gleam",
                    0,
                    type_::bool(),
                ))),
                type_: type_::bool(),
            }),
            Ok(crate::plan::ConstantValue::bool(true)),
        );

        assert_eq!(
            plan_fixture(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "Broken".into(),
                arguments: None,
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: None,
            }),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "Broken".into(),
                arguments: None,
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: Some(Box::new(ValueConstructor::local_variable(
                    dummy_span(),
                    VariableOrigin::generated(),
                    type_::int(),
                ))),
            }),
            Err(invalid_shape),
        );
        assert_eq!(
            plan_fixture(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "Other".into(),
                constructor: Some(Box::new(record_constructor(
                    "Other",
                    "gleam",
                    0,
                    type_::bool(),
                ))),
                type_: type_::bool(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        );
        assert_eq!(
            plan_fixture(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "External".into(),
                arguments: None,
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: Some(Box::new(record_constructor(
                    "External",
                    "other",
                    0,
                    type_::int(),
                ))),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "External".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![int_argument(None)]),
                record_constructor(
                    "Other",
                    "gleam",
                    1,
                    type_::fn_(vec![type_::int()], type_::bool()),
                ),
                type_::bool(),
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(Vec::new()),
                record_constructor("Missing", "main", 0, type_::int()),
                type_::int(),
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Missing".into(),
                    reason: crate::planner::InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_constant_record_payload_shapes() {
        let record_shape = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordConstructor,
            },
        };
        let result_type = type_::result(type_::int(), type_::string());
        let result_constructor = || {
            record_constructor(
                "Ok",
                "gleam",
                1,
                type_::fn_(vec![type_::int()], result_type.clone()),
            )
        };

        assert_eq!(
            plan_fixture(record_constant(
                None,
                result_constructor(),
                result_type.clone()
            )),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                None,
                result_constructor(),
                type_::fn_(vec![type_::int()], type_::int()),
            )),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(Vec::new()),
                result_constructor(),
                result_type.clone(),
            )),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![int_argument(None), int_argument(None)]),
                result_constructor(),
                result_type.clone(),
            )),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![int_argument(Some("wrong".into()))]),
                result_constructor(),
                result_type.clone(),
            )),
            Err(record_shape.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::String {
                        location: dummy_span(),
                        value: "wrong".into(),
                    },
                    implicit: None,
                }]),
                result_constructor(),
                result_type.clone(),
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::Invalid {
                        location: dummy_span(),
                        type_: type_::int(),
                        extra_information: None,
                    },
                    implicit: None,
                }]),
                result_constructor(),
                result_type.clone(),
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![int_argument(None)]),
                result_constructor(),
                type_::int(),
            )),
            Err(record_shape.clone()),
        );

        let arity_mismatch = record_constructor(
            "Ok",
            "gleam",
            2,
            type_::fn_(vec![type_::int()], result_type.clone()),
        );
        assert_eq!(
            plan_fixture(record_constant(
                Some(vec![int_argument(None), int_argument(None)]),
                arity_mismatch,
                result_type,
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Result".into(),
                    reason: crate::planner::InvalidCustomTypeReason::ConstructorArity,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_constant_bit_array_segment_shapes() {
        let invalid_option = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::BitArraySegmentOption,
            },
        };

        for (value, options, expected) in [
            (
                Constant::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                    extra_information: None,
                },
                Vec::new(),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                Constant::String {
                    location: dummy_span(),
                    value: "value".into(),
                },
                Vec::new(),
                invalid_option.clone(),
            ),
            (
                Constant::String {
                    location: dummy_span(),
                    value: "value".into(),
                },
                vec![
                    BitArrayOption::Utf8 {
                        location: dummy_span(),
                    },
                    size(int(8)),
                ],
                invalid_option.clone(),
            ),
            (
                int(1),
                vec![BitArrayOption::Utf8Codepoint {
                    location: dummy_span(),
                }],
                invalid_option.clone(),
            ),
            (
                int(1),
                vec![BitArrayOption::Size {
                    location: dummy_span(),
                    value: Box::new(Constant::String {
                        location: dummy_span(),
                        value: "wrong".into(),
                    }),
                    short_form: false,
                }],
                invalid_option.clone(),
            ),
            (
                int(1),
                vec![
                    BitArrayOption::Int {
                        location: dummy_span(),
                    },
                    BitArrayOption::Int {
                        location: dummy_span(),
                    },
                ],
                invalid_option.clone(),
            ),
            (
                int(1),
                vec![
                    BitArrayOption::Float {
                        location: dummy_span(),
                    },
                    size(int(24)),
                ],
                invalid_option.clone(),
            ),
            (
                Constant::String {
                    location: dummy_span(),
                    value: "wrong".into(),
                },
                vec![BitArrayOption::Int {
                    location: dummy_span(),
                }],
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Int,
                        actual: InvalidExpressionType::String,
                    },
                },
            ),
            (
                Constant::String {
                    location: dummy_span(),
                    value: "wrong".into(),
                },
                vec![BitArrayOption::Float {
                    location: dummy_span(),
                }],
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Float,
                        actual: InvalidExpressionType::String,
                    },
                },
            ),
            (
                int(1),
                vec![
                    BitArrayOption::Bits {
                        location: dummy_span(),
                    },
                    size(int(8)),
                ],
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::BitArray,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                int(1),
                vec![BitArrayOption::Bits {
                    location: dummy_span(),
                }],
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::BitArray,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                int(1),
                vec![BitArrayOption::Utf8 {
                    location: dummy_span(),
                }],
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::String,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
        ] {
            assert_eq!(plan_fixture(bit_array(value, options)), Err(expected));
        }

        for option in [
            BitArrayOption::Bytes {
                location: dummy_span(),
            },
            BitArrayOption::Signed {
                location: dummy_span(),
            },
            BitArrayOption::Unsigned {
                location: dummy_span(),
            },
        ] {
            assert_eq!(
                plan_fixture(bit_array(int(1), vec![option])),
                Err(invalid_option.clone())
            );
        }

        assert_eq!(
            plan_fixture(bit_array(
                int(1),
                vec![BitArrayOption::Native {
                    location: dummy_span(),
                }],
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
        assert_eq!(
            plan_fixture(bit_array(
                Constant::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                vec![
                    BitArrayOption::Float {
                        location: dummy_span(),
                    },
                    size(Constant::Int {
                        location: dummy_span(),
                        value: "too large".into(),
                        int_value: BigInt::from(usize::MAX) + BigInt::from(1_u8),
                    }),
                ],
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_fixture(bit_array(
                Constant::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                },
                vec![
                    BitArrayOption::Bits {
                        location: dummy_span(),
                    },
                    size(Constant::Int {
                        location: dummy_span(),
                        value: "too large".into(),
                        int_value: BigInt::from(usize::MAX) + BigInt::from(1_u8),
                    }),
                ],
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );
        assert_eq!(
            plan_fixture(bit_array(
                int(1),
                vec![size(Constant::Int {
                    location: dummy_span(),
                    value: "too large".into(),
                    int_value: BigInt::from(usize::MAX) + BigInt::from(1_u8),
                })],
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::SizeOutOfRange,
            }),
        );

        for options in [
            vec![BitArrayOption::Utf16 {
                location: dummy_span(),
            }],
            vec![BitArrayOption::Utf32 {
                location: dummy_span(),
            }],
        ] {
            assert_eq!(
                plan_fixture(bit_array(
                    Constant::String {
                        location: dummy_span(),
                        value: "A".into(),
                    },
                    options,
                ))
                .map(|value| value.shape()),
                Ok(ValueShape::BitArray),
            );
        }

        assert_eq!(
            plan_fixture(bit_array(
                Constant::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                Vec::new(),
            ))
            .map(|value| value.shape()),
            Ok(ValueShape::BitArray),
        );

        for (option, kind) in [
            (
                BitArrayOption::Utf8 {
                    location: dummy_span(),
                },
                StaticSegmentKind::String(StringEncoding::Utf8),
            ),
            (
                BitArrayOption::Utf16 {
                    location: dummy_span(),
                },
                StaticSegmentKind::String(StringEncoding::Utf16(Endianness::Big)),
            ),
            (
                BitArrayOption::Utf32 {
                    location: dummy_span(),
                },
                StaticSegmentKind::String(StringEncoding::Utf32(Endianness::Big)),
            ),
        ] {
            assert_eq!(
                static_segment_options(vec![option]),
                Ok(StaticSegmentOptions {
                    kind: Some(kind),
                    endianness: Endianness::Big,
                    size: None,
                    unit: 1,
                }),
            );
        }

        for options in [
            vec![
                BitArrayOption::Utf8 {
                    location: dummy_span(),
                },
                BitArrayOption::Utf8 {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf16 {
                    location: dummy_span(),
                },
                BitArrayOption::Utf16 {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Utf32 {
                    location: dummy_span(),
                },
                BitArrayOption::Utf32 {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Float {
                    location: dummy_span(),
                },
                BitArrayOption::Float {
                    location: dummy_span(),
                },
            ],
            vec![
                BitArrayOption::Bits {
                    location: dummy_span(),
                },
                BitArrayOption::Bits {
                    location: dummy_span(),
                },
            ],
        ] {
            assert_eq!(static_segment_options(options), Err(invalid_option.clone()));
        }
    }

    #[test]
    fn plan_negative_sized_bits_constant_preserves_the_exact_zero_bit_segment() {
        let empty_bits = plan_fixture(Constant::BitArray {
            location: dummy_span(),
            segments: Vec::new(),
        })
        .expect("an empty BitArray constant should plan")
        .into_bit_array()
        .expect("the planned constant should preserve its BitArray family");
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);

        assert_eq!(
            plan_bit_array_segment(
                BitArraySegment {
                    location: dummy_span(),
                    type_: type_::bit_array(),
                    value: Box::new(Constant::BitArray {
                        location: dummy_span(),
                        segments: Vec::new(),
                    }),
                    options: vec![
                        BitArrayOption::Bits {
                            location: dummy_span(),
                        },
                        size(int(-1)),
                    ],
                },
                &mut context,
            ),
            Ok(ConstantBitArraySegment::SizedBits {
                value: empty_bits,
                bit_size: 0,
                site: PanicSite::new("main".into(), "main".into(), SourceSpan::new(0, 0),),
            }),
        );
    }

    fn plan_fixture(
        value: Constant<Arc<type_::Type>>,
    ) -> Result<crate::plan::ConstantValue, PlanError> {
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);
        plan_value(value, &mut context)
    }

    fn int(value: i64) -> Constant<Arc<type_::Type>> {
        Constant::Int {
            location: dummy_span(),
            value: value.to_string().into(),
            int_value: value.into(),
        }
    }

    fn size(value: Constant<Arc<type_::Type>>) -> BitArrayOption<Constant<Arc<type_::Type>>> {
        BitArrayOption::Size {
            location: dummy_span(),
            value: Box::new(value),
            short_form: false,
        }
    }

    fn bit_array(
        value: Constant<Arc<type_::Type>>,
        options: Vec<BitArrayOption<Constant<Arc<type_::Type>>>>,
    ) -> Constant<Arc<type_::Type>> {
        Constant::BitArray {
            location: dummy_span(),
            segments: vec![BitArraySegment {
                location: dummy_span(),
                type_: value.type_(),
                value: Box::new(value),
                options,
            }],
        }
    }

    fn record_constructor(
        name: &str,
        module: &str,
        arity: u16,
        type_: Arc<type_::Type>,
    ) -> ValueConstructor {
        ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            variant: ValueConstructorVariant::Record {
                name: name.into(),
                arity,
                field_map: None,
                location: dummy_span(),
                module: module.into(),
                variants_count: 1,
                variant_index: 0,
                documentation: None,
            },
            type_,
        }
    }

    fn constant_var_constructor(value: &Constant<Arc<type_::Type>>) -> Option<ValueConstructor> {
        match value {
            Constant::Var { constructor, .. } => constructor.as_deref().cloned(),
            _ => None,
        }
    }

    fn int_argument(label: Option<EcoString>) -> CallArg<Constant<Arc<type_::Type>>> {
        CallArg {
            label,
            location: dummy_span(),
            value: int(1),
            implicit: None,
        }
    }

    fn record_constant(
        arguments: Option<Vec<CallArg<Constant<Arc<type_::Type>>>>>,
        constructor: ValueConstructor,
        type_: Arc<type_::Type>,
    ) -> Constant<Arc<type_::Type>> {
        Constant::Record {
            location: dummy_span(),
            module: None,
            name: "Ok".into(),
            arguments,
            type_,
            field_map: Inferred::Unknown,
            record_constructor: Some(Box::new(constructor)),
        }
    }
}
