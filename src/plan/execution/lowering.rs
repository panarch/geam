mod expression;
mod frame;
mod id;
mod param;
mod pattern;
mod return_;
mod specialization;
mod step;
mod table;
mod value_type;

use super::ExecutionPlan;
use super::custom_type::CustomTypeTable;
use super::value_shape::ValueShapeTable;
use super::value_type::ListTypeTable;
use crate::plan::{ModulePlan, ValueShape};
use specialization::{
    ConcreteCustomConstructor, ConcreteCustomValueShape, ConcreteFunctionShape,
    ConcreteTypeSubstitution, ConcreteValueShape, SpecializationKey,
};
use std::collections::{HashMap, VecDeque};

struct FunctionTemplates {
    templates: Vec<crate::plan::FunctionTemplate>,
    main: crate::plan::FunctionTemplateId,
}

struct LoweringContext {
    types: value_type::TypeInterner,
    functions: table::FunctionTableBuilder,
    frame_templates: HashMap<crate::plan::FunctionTemplateId, frame::LocalAllocationTemplate>,
    specialization_locals: HashMap<SpecializationKey, frame::LocalAllocationPlan>,
    current_specialization: SpecializationKey,
    next_function_indices: HashMap<table::FunctionTableFamily, usize>,
    specializations: HashMap<SpecializationKey, usize>,
    pending: VecDeque<SpecializationKey>,
    substitution: ConcreteTypeSubstitution,
}

struct TargetLocal {
    index: usize,
    shape: ConcreteValueShape,
    substitution: ConcreteTypeSubstitution,
}

impl TargetLocal {
    fn index(&self) -> usize {
        self.index
    }

    fn shape(&self) -> &ConcreteValueShape {
        &self.shape
    }

    fn substitution(&self) -> &ConcreteTypeSubstitution {
        &self.substitution
    }

    fn custom_shape(&self, shape: &crate::plan::CustomValueShape) -> ConcreteCustomValueShape {
        ConcreteCustomValueShape::instantiate(shape, &self.substitution)
    }

    fn function_shape(&self, shape: &crate::plan::FunctionShape) -> ConcreteFunctionShape {
        ConcreteFunctionShape::instantiate(shape, &self.substitution)
    }
}

#[derive(Clone)]
enum SpecializedFunctionLocal {
    Int {
        local: super::IntFunctionLocalId,
        type_: super::FunctionType,
    },
    Float {
        local: super::FloatFunctionLocalId,
        type_: super::FunctionType,
    },
    String {
        local: super::StringFunctionLocalId,
        type_: super::FunctionType,
    },
    BitArray {
        local: super::BitArrayFunctionLocalId,
        type_: super::FunctionType,
    },
    UtfCodepoint {
        local: super::UtfCodepointFunctionLocalId,
        type_: super::FunctionType,
    },
    Custom(super::CustomFunctionLocal),
    Bool {
        local: super::BoolFunctionLocalId,
        type_: super::FunctionType,
    },
    Nil {
        local: super::NilFunctionLocalId,
        type_: super::FunctionType,
    },
    Tuple {
        local: super::TupleFunctionLocalId,
        type_: super::FunctionType,
    },
    List(super::ListFunctionLocal),
    Function(super::FunctionFunctionLocal),
}

impl LoweringContext {
    fn new(templates: &FunctionTemplates, main: SpecializationKey) -> Self {
        let frame_templates = templates
            .templates
            .iter()
            .map(|template| (template.id(), frame::LocalAllocationTemplate::new(template)))
            .collect::<HashMap<_, _>>();
        let main_locals = frame_templates[&main.template()].specialize(main.substitution());
        let mut specialization_locals = HashMap::new();
        specialization_locals.insert(main.clone(), main_locals);
        Self {
            types: value_type::TypeInterner::new(),
            functions: table::FunctionTableBuilder::default(),
            frame_templates,
            specialization_locals,
            current_specialization: main,
            next_function_indices: HashMap::new(),
            specializations: HashMap::new(),
            pending: VecDeque::new(),
            substitution: ConcreteTypeSubstitution::empty(),
        }
    }

    fn generic_local_index(&self, local: crate::plan::GenericLocalId) -> usize {
        self.local_index(frame::LocalKey::new(frame::LocalKind::Generic, local.0))
    }

    fn generic_list_local_index(&self, local: crate::plan::GenericListLocalId) -> usize {
        self.local_index(frame::LocalKey::new(frame::LocalKind::GenericList, local.0))
    }

    fn generic_function_local_index(&self, local: crate::plan::GenericFunctionLocalId) -> usize {
        self.local_index(frame::LocalKey::new(
            frame::LocalKind::GenericFunction,
            local.0,
        ))
    }

    fn local_index(&self, key: frame::LocalKey) -> usize {
        self.specialization_locals[&self.current_specialization].index(key)
    }

    fn mapped_local(&self, kind: frame::LocalKind, index: usize) -> usize {
        self.local_index(frame::LocalKey::new(kind, index))
    }

    fn target_local(
        &mut self,
        instantiation: &crate::plan::FunctionInstantiation,
        key: frame::LocalKey,
    ) -> TargetLocal {
        let (specialization, shape) =
            SpecializationKey::from_instantiation(instantiation, &self.substitution);
        self.reserve(specialization.clone(), shape);
        let locals = &self.specialization_locals[&specialization];
        TargetLocal {
            index: locals.index(key),
            shape: locals.shape(key).clone(),
            substitution: specialization.substitution().clone(),
        }
    }

    fn current_target(&self, index: usize, shape: ConcreteValueShape) -> TargetLocal {
        TargetLocal {
            index,
            shape,
            substitution: self.substitution.clone(),
        }
    }

    fn current_local_entries(&self) -> &[(frame::LocalKey, ConcreteValueShape)] {
        self.specialization_locals[&self.current_specialization].entries()
    }

    fn concrete_parameter(&self, parameter: crate::plan::TypeParameterId) -> ConcreteValueShape {
        ConcreteValueShape::instantiate(&ValueShape::Parameter(parameter), &self.substitution)
    }

    fn value_type(&mut self, type_: crate::plan::ValueType) -> super::ValueType {
        let shape = ConcreteValueShape::instantiate(
            &ValueShape::from_value_type(type_),
            &self.substitution,
        );
        self.types.value_type(&shape)
    }

    fn custom_value_shape(
        &mut self,
        shape: crate::plan::CustomValueShape,
    ) -> super::CustomValueShape {
        let shape = ConcreteCustomValueShape::instantiate(&shape, &self.substitution);
        self.types.custom_value_shape(&shape)
    }

    fn value_shape(&mut self, shape: crate::plan::ValueShape) -> super::ValueShapeId {
        self.types
            .value_shape(&ConcreteValueShape::instantiate(&shape, &self.substitution))
    }

    fn function_shape(&mut self, shape: crate::plan::FunctionShape) -> super::FunctionShape {
        self.types
            .function_shape(&ConcreteFunctionShape::instantiate(
                &shape,
                &self.substitution,
            ))
    }

    fn function_type(&mut self, type_: crate::plan::FunctionType) -> super::FunctionType {
        let shape = crate::plan::FunctionShape::from_function_type(type_);
        let shape = ConcreteFunctionShape::instantiate(&shape, &self.substitution);
        self.types.function_type(&shape)
    }

    fn custom_function_type(
        &mut self,
        type_: crate::plan::CustomFunctionType,
    ) -> super::CustomFunctionType {
        let substitution = self.substitution.clone();
        self.custom_function_type_with_substitution(&type_, &substitution)
    }

    fn custom_function_type_with_substitution(
        &mut self,
        type_: &crate::plan::CustomFunctionType,
        substitution: &ConcreteTypeSubstitution,
    ) -> super::CustomFunctionType {
        let arguments = type_
            .argument_shapes()
            .iter()
            .map(|shape| ConcreteValueShape::instantiate(shape, substitution))
            .collect::<Vec<_>>();
        let return_ = ConcreteCustomValueShape::instantiate(type_.return_(), substitution);
        self.types.custom_function_type(&arguments, &return_)
    }

    fn function_function_type(
        &mut self,
        type_: crate::plan::FunctionFunctionType,
    ) -> super::FunctionFunctionType {
        let substitution = self.substitution.clone();
        self.function_function_type_with_substitution(&type_, &substitution)
    }

    fn function_function_type_with_substitution(
        &mut self,
        type_: &crate::plan::FunctionFunctionType,
        substitution: &ConcreteTypeSubstitution,
    ) -> super::FunctionFunctionType {
        let arguments = type_
            .argument_shapes()
            .iter()
            .map(|shape| ConcreteValueShape::instantiate(shape, substitution))
            .collect::<Vec<_>>();
        let return_ = ConcreteFunctionShape::instantiate(type_.return_shape(), substitution);
        self.types.function_function_type(&arguments, &return_)
    }

    fn int_list_type(&mut self) -> super::IntListTypeId {
        self.types.int_list_type()
    }

    fn string_list_type(&mut self) -> super::StringListTypeId {
        self.types.string_list_type()
    }

    fn bit_array_list_type(&mut self) -> super::BitArrayListTypeId {
        self.types.bit_array_list_type()
    }

    fn utf_codepoint_list_type(&mut self) -> super::UtfCodepointListTypeId {
        self.types.utf_codepoint_list_type()
    }

    fn custom_constructor(
        &mut self,
        constructor: crate::plan::CustomConstructor,
    ) -> super::CustomConstructorId {
        self.types
            .custom_constructor(ConcreteCustomConstructor::instantiate(
                constructor,
                &self.substitution,
            ))
    }

    fn custom_list_type(&mut self, item: crate::plan::CustomType) -> super::CustomListTypeId {
        let shape = ConcreteCustomValueShape::instantiate(
            &crate::plan::CustomValueShape::any(item),
            &self.substitution,
        );
        self.types.custom_list_type(&shape)
    }

    fn float_list_type(&mut self) -> super::FloatListTypeId {
        self.types.float_list_type()
    }

    fn bool_list_type(&mut self) -> super::BoolListTypeId {
        self.types.bool_list_type()
    }

    fn nil_list_type(&mut self) -> super::NilListTypeId {
        self.types.nil_list_type()
    }

    fn tuple_list_type(&mut self, item: Vec<crate::plan::ValueType>) -> super::TupleListTypeId {
        let item = item
            .into_iter()
            .map(|type_| {
                ConcreteValueShape::instantiate(
                    &ValueShape::from_value_type(type_),
                    &self.substitution,
                )
            })
            .collect::<Vec<_>>();
        self.types.tuple_list_type(&item)
    }

    fn list_list_type(&mut self, item: crate::plan::ValueType) -> super::ListListTypeId {
        self.types.list_list_type(&ConcreteValueShape::instantiate(
            &ValueShape::from_value_type(item),
            &self.substitution,
        ))
    }

    fn function_list_type(&mut self, item: crate::plan::FunctionType) -> super::FunctionListTypeId {
        let shape = ConcreteFunctionShape::instantiate(
            &crate::plan::FunctionShape::from_function_type(item),
            &self.substitution,
        );
        self.types.function_list_type(&shape)
    }

    fn reserve(
        &mut self,
        key: SpecializationKey,
        shape: ConcreteFunctionShape,
    ) -> super::RuntimeFunctionId {
        self.reserve_locals(&key);
        let index = match self.specializations.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.next_function_index(table::function_table_family(shape.return_()));
                self.specializations.insert(key.clone(), index);
                self.pending.push_back(key);
                index
            }
        };
        table::function_id(&shape, index, &mut self.types)
    }

    fn reserve_index_for(
        &mut self,
        instantiation: &crate::plan::FunctionInstantiation,
        family: table::FunctionTableFamily,
    ) -> usize {
        let (key, _) = SpecializationKey::from_instantiation(instantiation, &self.substitution);
        self.reserve_locals(&key);
        match self.specializations.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.next_function_index(family);
                self.specializations.insert(key.clone(), index);
                self.pending.push_back(key);
                index
            }
        }
    }

    fn reserve_locals(&mut self, key: &SpecializationKey) {
        if !self.specialization_locals.contains_key(key) {
            let locals = self.frame_templates[&key.template()].specialize(key.substitution());
            self.specialization_locals.insert(key.clone(), locals);
        }
    }

    fn next_function_index(&mut self, family: table::FunctionTableFamily) -> usize {
        let next = self.next_function_indices.entry(family).or_default();
        let index = *next;
        *next += 1;
        index
    }

    fn int_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::IntFunctionId {
        super::IntFunctionId(self.reserve_index_for(function, table::FunctionTableFamily::Int))
    }

    fn float_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::FloatFunctionId {
        super::FloatFunctionId(self.reserve_index_for(function, table::FunctionTableFamily::Float))
    }

    fn string_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::StringFunctionId {
        super::StringFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::String),
        )
    }

    fn bit_array_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BitArrayFunctionId {
        super::BitArrayFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::BitArray),
        )
    }

    fn utf_codepoint_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::UtfCodepointFunctionId {
        super::UtfCodepointFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::UtfCodepoint),
        )
    }

    fn custom_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        shape: &ConcreteCustomValueShape,
    ) -> super::CustomFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::Custom);
        super::CustomFunctionId::new(index, self.types.custom_value_shape(shape))
    }

    fn bool_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BoolFunctionId {
        super::BoolFunctionId(self.reserve_index_for(function, table::FunctionTableFamily::Bool))
    }

    fn nil_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::NilFunctionId {
        super::NilFunctionId(self.reserve_index_for(function, table::FunctionTableFamily::Nil))
    }

    fn tuple_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::TupleFunctionId {
        super::TupleFunctionId(self.reserve_index_for(function, table::FunctionTableFamily::Tuple))
    }

    fn list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        item: &ConcreteValueShape,
    ) -> super::ListFunctionId {
        let index = self.reserve_index_for(function, table::list_function_table_family(item));
        table::list_function_id(item, index, &mut self.types)
    }

    fn int_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::IntListFunctionId {
        super::IntListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::IntList),
            self.types.int_list_type(),
        )
    }

    fn string_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::StringListFunctionId {
        super::StringListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::StringList),
            self.types.string_list_type(),
        )
    }

    fn bit_array_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BitArrayListFunctionId {
        super::BitArrayListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::BitArrayList),
            self.types.bit_array_list_type(),
        )
    }

    fn utf_codepoint_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::UtfCodepointListFunctionId {
        super::UtfCodepointListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::UtfCodepointList),
            self.types.utf_codepoint_list_type(),
        )
    }

    fn custom_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::CustomListTypeId,
    ) -> super::CustomListFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::CustomList);
        super::CustomListFunctionId::new(index, type_id)
    }

    fn float_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::FloatListFunctionId {
        super::FloatListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::FloatList),
            self.types.float_list_type(),
        )
    }

    fn bool_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BoolListFunctionId {
        super::BoolListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::BoolList),
            self.types.bool_list_type(),
        )
    }

    fn nil_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::NilListFunctionId {
        super::NilListFunctionId::new(
            self.reserve_index_for(function, table::FunctionTableFamily::NilList),
            self.types.nil_list_type(),
        )
    }

    fn tuple_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::TupleListTypeId,
    ) -> super::TupleListFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::TupleList);
        super::TupleListFunctionId::new(index, type_id)
    }

    fn list_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::ListListTypeId,
    ) -> super::ListListFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::ListList);
        super::ListListFunctionId::new(index, type_id)
    }

    fn function_list_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_id: super::FunctionListTypeId,
    ) -> super::FunctionListFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::FunctionList);
        super::FunctionListFunctionId::new(index, type_id)
    }

    fn function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        return_: &ConcreteFunctionShape,
    ) -> super::FunctionFunctionId {
        let index = self.reserve_index_for(
            function,
            table::function_function_table_family(return_.return_()),
        );
        table::function_function_id(return_, index, &mut self.types)
    }

    fn int_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::IntFunctionFunctionId {
        super::IntFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::IntFunction),
        )
    }

    fn float_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::FloatFunctionFunctionId {
        super::FloatFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::FloatFunction),
        )
    }

    fn string_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::StringFunctionFunctionId {
        super::StringFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::StringFunction),
        )
    }

    fn bit_array_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BitArrayFunctionFunctionId {
        super::BitArrayFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::BitArrayFunction),
        )
    }

    fn utf_codepoint_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::UtfCodepointFunctionFunctionId {
        super::UtfCodepointFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::UtfCodepointFunction),
        )
    }

    fn custom_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::CustomFunctionType,
    ) -> super::CustomFunctionFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::CustomFunction);
        super::CustomFunctionFunctionId::new(index, type_)
    }

    fn bool_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::BoolFunctionFunctionId {
        super::BoolFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::BoolFunction),
        )
    }

    fn nil_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::NilFunctionFunctionId {
        super::NilFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::NilFunction),
        )
    }

    fn tuple_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
    ) -> super::TupleFunctionFunctionId {
        super::TupleFunctionFunctionId(
            self.reserve_index_for(function, table::FunctionTableFamily::TupleFunction),
        )
    }

    fn list_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: &ConcreteFunctionShape,
        item: &ConcreteValueShape,
    ) -> super::ListFunctionFunctionId {
        let index = self.reserve_index_for(
            function,
            table::function_function_table_family(&ConcreteValueShape::List(Box::new(
                item.clone(),
            ))),
        );
        table::list_function_function_id(type_, item, index, &mut self.types)
    }

    fn function_function_function_id(
        &mut self,
        function: &crate::plan::FunctionInstantiation,
        type_: super::FunctionFunctionType,
    ) -> super::FunctionFunctionFunctionId {
        let index = self.reserve_index_for(function, table::FunctionTableFamily::FunctionFunction);
        super::FunctionFunctionFunctionId::new(index, type_)
    }

    fn concrete_value_shape(&self, shape: &ValueShape) -> ConcreteValueShape {
        ConcreteValueShape::instantiate(shape, &self.substitution)
    }

    fn lower_concrete_value_type(&mut self, shape: &ConcreteValueShape) -> super::ValueType {
        self.types.value_type(shape)
    }

    fn lower_concrete_custom_shape(
        &mut self,
        shape: &ConcreteCustomValueShape,
    ) -> super::CustomValueShape {
        self.types.custom_value_shape(shape)
    }

    fn lower_concrete_function_type(
        &mut self,
        shape: &ConcreteFunctionShape,
    ) -> super::FunctionType {
        self.types.function_type(shape)
    }

    fn lower_concrete_function_shape(
        &mut self,
        shape: &ConcreteFunctionShape,
    ) -> super::FunctionShape {
        self.types.function_shape(shape)
    }

    fn concrete_custom_value_shape(
        &self,
        shape: &crate::plan::CustomValueShape,
    ) -> ConcreteCustomValueShape {
        ConcreteCustomValueShape::instantiate(shape, &self.substitution)
    }

    fn concrete_function_shape(&self, shape: &crate::plan::FunctionShape) -> ConcreteFunctionShape {
        ConcreteFunctionShape::instantiate(shape, &self.substitution)
    }

    fn begin(&mut self, key: &SpecializationKey) {
        self.substitution = key.substitution().clone();
        self.current_specialization = key.clone();
    }

    fn specialization_index(&self, key: &SpecializationKey) -> usize {
        self.specializations[key]
    }

    fn into_parts(
        self,
    ) -> (
        table::FunctionTableBuilder,
        ListTypeTable,
        CustomTypeTable,
        ValueShapeTable,
    ) {
        let (list_types, custom_types, value_shapes) = self.types.into_tables();
        (self.functions, list_types, custom_types, value_shapes)
    }
}

pub(super) fn lower(module_plan: ModulePlan) -> ExecutionPlan {
    let parts = module_plan.into_parts();
    drop(parts.custom_types);
    drop(parts.constants);
    let templates = FunctionTemplates::new(parts.main, parts.functions, parts.anonymous_functions);
    let main_key = SpecializationKey::monomorphic(templates.main);
    let main_shape = ConcreteFunctionShape::instantiate(
        templates.get(templates.main).signature().shape(),
        main_key.substitution(),
    );
    let mut context = LoweringContext::new(&templates, main_key.clone());
    let main = context.reserve(main_key, main_shape);

    for template in &templates.templates {
        if template.scheme().is_monomorphic() && template.id() != templates.main {
            let key = SpecializationKey::monomorphic(template.id());
            let shape = ConcreteFunctionShape::instantiate(
                template.signature().shape(),
                key.substitution(),
            );
            context.reserve(key, shape);
        }
    }

    while let Some(key) = context.pending.pop_front() {
        context.begin(&key);
        table::lower_specialized(templates.get(key.template()), &key, &mut context);
    }

    let (functions, list_types, custom_types, value_shapes) = context.into_parts();
    ExecutionPlan {
        module: parts.module,
        source_context: parts.source_context,
        main,
        functions: functions.finish(),
        list_types,
        custom_types,
        value_shapes,
    }
}

impl FunctionTemplates {
    fn new(
        main: crate::plan::FunctionTemplate,
        functions: Vec<crate::plan::FunctionTemplate>,
        anonymous_functions: Vec<crate::plan::FunctionTemplate>,
    ) -> Self {
        let main_id = main.id();
        let mut templates = Vec::with_capacity(1 + functions.len() + anonymous_functions.len());
        templates.push(main);
        templates.extend(functions);
        templates.extend(anonymous_functions);
        templates.sort_by_key(|template| template.id().index());
        Self {
            templates,
            main: main_id,
        }
    }

    fn get(&self, id: crate::plan::FunctionTemplateId) -> &crate::plan::FunctionTemplate {
        &self.templates[id.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ExecutionPlan, IntFunctionId, RuntimeFunctionId};
    use crate::Value;
    use crate::plan::SourceContext;
    use num_bigint::BigInt;

    #[test]
    fn lowering_preserves_module_source_context_and_main_runtime() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan =
            crate::plan_module_with_source(typed, SourceContext::new("src/main.gleam", source))
                .expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.module().as_str(), "main");
        let source_context = plan.source_context().expect("source should be preserved");
        assert_eq!(source_context.path(), "src/main.gleam");
        assert_eq!(source_context.source(), source);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
    }

    #[test]
    fn lowering_reserves_locals_for_zero_argument_generic_specializations() {
        let source = r#"
type Box(value) {
  Box
}

fn make() -> Box(value) {
  Box
}

pub fn main() {
  let make_int: fn() -> Box(Int) = make
  case make_int() {
    Box -> 1
  }
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Int(BigInt::from(1)),
        );
    }

    #[test]
    fn lowering_assigns_generic_specializations_by_first_use_and_deduplicates_them() {
        let source = r#"
fn first(value: value) -> value {
  let first_marker = "first"
  value
}

fn second(value: value) -> value {
  let second_marker = 2
  value
}

pub fn main() {
  #(second(1), first(2), second(3), first("four"))
}
"#;
        let typed = crate::compile_typed_module("main", "src/main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.functions.int_functions.len(), 2);
        assert_eq!(plan.functions.string_functions.len(), 1);
        assert_eq!(plan.functions.tuple_functions.len(), 1);
        assert_eq!(plan.int_function(IntFunctionId(0)).frame_layout().ints(), 2);
        assert_eq!(
            plan.int_function(IntFunctionId(0)).frame_layout().strings(),
            0,
        );
        assert_eq!(plan.int_function(IntFunctionId(1)).frame_layout().ints(), 1);
        assert_eq!(
            plan.int_function(IntFunctionId(1)).frame_layout().strings(),
            1,
        );
        assert_eq!(
            crate::run_main(&plan).expect("source should execute"),
            Value::Tuple(vec![
                Value::Int(BigInt::from(1)),
                Value::Int(BigInt::from(2)),
                Value::Int(BigInt::from(3)),
                Value::String("four".into()),
            ]),
        );
    }
}
