mod custom;

use crate::plan::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CaptureArg, CustomFunctionLocal, CustomFunctionLocalId,
    CustomFunctionType, CustomListLocalId, CustomLocalId, CustomValueShape, ExternalFunctionLocal,
    ExternalFunctionLocalId, ExternalFunctionType, ExternalListLocalId, ExternalLocalId,
    FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionFunctionLocalId, FunctionFunctionType, FunctionListLocalId, FunctionReference,
    FunctionShape, FunctionTemplate, FunctionTemplateSignature, FunctionType, IntFunctionLocalId,
    IntListLocalId, IntLocalId, ListExpr, ListFunctionLocal, ListListLocalId, ListLocal,
    ListLocalExpr, LocalId, ModuleId, NilFunctionLocalId, NilListLocalId, NilLocalId, PanicSite,
    ParamBinding, ParamLocal, ParamSlot, StringFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId, ValueShape, ValueType,
};
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError,
};
use ecow::EcoString;
use gleam_core::type_::Type;
use std::collections::HashMap;

#[cfg(test)]
use crate::plan::CustomTypeDefinition;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) signature: FunctionTemplateSignature,
    pub(super) type_parameters: super::type_parameter::TypeParameterScope,
    pub(super) return_shape: ValueShape,
    pub(super) params: Vec<FunctionParam>,
    pub(super) definition_span: crate::plan::SourceSpan,
}

pub(super) struct ModuleFunctionTarget {
    module: EcoString,
    name: EcoString,
    link: ModuleFunctionLink,
    external: bool,
}

pub(super) struct ValidatedModuleFunctionTarget {
    module: EcoString,
    name: EcoString,
    link: ModuleFunctionLink,
}

#[derive(Clone, Copy)]
enum ModuleFunctionLink {
    Unresolved,
    Resolved(ModuleId),
}

impl ModuleFunctionTarget {
    pub(super) fn direct(
        module: EcoString,
        name: EcoString,
        external: bool,
    ) -> ModuleFunctionTarget {
        ModuleFunctionTarget {
            module,
            name,
            link: ModuleFunctionLink::Unresolved,
            external,
        }
    }

    pub(super) fn selected(
        context: &PlanContext<'_>,
        module: EcoString,
        name: EcoString,
        constructor_module: EcoString,
        constructor_name: EcoString,
        external: bool,
    ) -> Result<ModuleFunctionTarget, PlanError> {
        let linked_module = context.resolve_module_reference(&module, &name)?;
        if constructor_module != module {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module,
                    name,
                    reason: InvalidModuleReferenceReason::FunctionModule {
                        actual: constructor_module,
                    },
                },
            });
        }
        if constructor_name != name {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module,
                    name,
                    reason: InvalidModuleReferenceReason::FunctionName {
                        actual: constructor_name,
                    },
                },
            });
        }
        Ok(ModuleFunctionTarget {
            module,
            name,
            link: ModuleFunctionLink::Resolved(linked_module),
            external,
        })
    }

    pub(super) fn validate_external(
        self,
        context: &PlanContext<'_>,
    ) -> Result<ValidatedModuleFunctionTarget, PlanError> {
        if self.external && !context.executable_external(&self.module, &self.name) {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: self.module,
                    name: self.name,
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            });
        }
        Ok(ValidatedModuleFunctionTarget {
            module: self.module,
            name: self.name,
            link: self.link,
        })
    }
}

impl ValidatedModuleFunctionTarget {
    pub(super) fn module(&self) -> &EcoString {
        &self.module
    }

    pub(super) fn name(&self) -> &EcoString {
        &self.name
    }

    pub(super) fn function_shape(&self, shape: ValueShape) -> Result<FunctionShape, PlanError> {
        let ValueShape::Function(shape) = shape else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: self.module.clone(),
                    name: self.name.clone(),
                    reason: InvalidModuleReferenceReason::FunctionType,
                },
            });
        };
        Ok(*shape)
    }

    pub(super) fn instantiate_reference(
        &self,
        function: &FunctionInfo,
        shape: &FunctionShape,
    ) -> Result<crate::plan::FunctionInstantiation, PlanError> {
        function
            .instantiate(shape)
            .map_err(|_| PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: self.module.clone(),
                    name: self.name.clone(),
                    reason: InvalidModuleReferenceReason::FunctionInstantiation,
                },
            })
    }
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    slot: crate::plan::ParamSlot,
    pub(super) binding: ParamBinding,
    pub(super) label: Option<EcoString>,
}

impl FunctionParam {
    pub(super) fn new(
        local: ParamLocal,
        shape: ValueShape,
        binding: ParamBinding,
        label: Option<EcoString>,
    ) -> Self {
        Self {
            slot: crate::plan::ParamSlot::new(local, shape),
            binding,
            label,
        }
    }

    pub(super) fn local(&self) -> &ParamLocal {
        self.slot.local()
    }

    pub(super) fn shape(&self) -> &ValueShape {
        self.slot.shape()
    }
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    current_function: EcoString,
    registry: RegistryAccess<'a>,
    anonymous_functions: &'a mut AnonymousFunctions,
    bindings: HashMap<EcoString, LocalBinding>,
    next_generic_local: usize,
    next_int_local: usize,
    next_float_local: usize,
    next_string_local: usize,
    next_bit_array_local: usize,
    next_utf_codepoint_local: usize,
    next_custom_local: usize,
    next_external_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
    next_tuple_local: usize,
    next_int_list_local: usize,
    next_string_list_local: usize,
    next_bit_array_list_local: usize,
    next_utf_codepoint_list_local: usize,
    next_custom_list_local: usize,
    next_external_list_local: usize,
    next_float_list_local: usize,
    next_bool_list_local: usize,
    next_nil_list_local: usize,
    next_tuple_list_local: usize,
    next_list_list_local: usize,
    next_function_list_local: usize,
    next_generic_list_local: usize,
    next_int_function_local: usize,
    next_float_function_local: usize,
    next_string_function_local: usize,
    next_bit_array_function_local: usize,
    next_utf_codepoint_function_local: usize,
    next_custom_function_local: usize,
    next_external_function_local: usize,
    next_bool_function_local: usize,
    next_nil_function_local: usize,
    next_tuple_function_local: usize,
    next_list_function_local: usize,
    next_function_function_local: usize,
    next_generic_function_local: usize,
    type_parameters: super::type_parameter::TypeParameterScope,
}

#[derive(Clone, Copy)]
enum RegistryAccess<'a> {
    Program {
        registry: &'a crate::planner::module::registry::ProgramRegistry,
    },
    #[cfg(test)]
    Local {
        functions: &'a HashMap<EcoString, FunctionInfo>,
        custom_types: &'a [CustomTypeDefinition],
    },
}

#[derive(Clone)]
enum LocalBinding {
    Primitive(LocalId),
    Custom(crate::plan::CustomLocal),
    External(crate::plan::ExternalLocal),
    Tuple {
        local: TupleLocalId,
        shape: Box<[ValueShape]>,
    },
    List {
        local: ListLocal,
        item_shape: ValueShape,
    },
    Function {
        binding: FunctionLocalBinding,
        shape: FunctionShape,
    },
}

pub(super) struct CaptureBinding {
    name: EcoString,
    binding: LocalBinding,
}

pub(super) struct PlannedCaptures {
    captures: Box<[PlannedCapture]>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct PlannedCapture {
    slot: ParamSlot,
    source: CaptureArg,
}

impl PlannedCaptures {
    pub(super) fn into_parts(self) -> (Vec<ParamSlot>, Vec<CaptureArg>) {
        let mut slots = Vec::with_capacity(self.captures.len());
        let mut sources = Vec::with_capacity(self.captures.len());
        for capture in self.captures {
            slots.push(capture.slot);
            sources.push(capture.source);
        }
        (slots, sources)
    }
}

fn invalid_local_shape() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::Invalid,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionLocalBinding {
    Generic(crate::plan::GenericFunctionLocal),
    Int {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    String {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArray {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepoint {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    Custom(CustomFunctionLocal),
    External(ExternalFunctionLocal),
    Float {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    Bool {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    Nil {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    Tuple {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    List(ListFunctionLocal),
    Function(FunctionFunctionLocal),
}

impl<'a> PlanContext<'a> {
    #[cfg(test)]
    pub(super) fn new(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_custom_types(module_name, functions, &[], anonymous_functions)
    }

    #[cfg(test)]
    pub(super) fn new_with_custom_types(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        custom_types: &'a [CustomTypeDefinition],
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_registry(
            module_name,
            RegistryAccess::Local {
                functions,
                custom_types,
            },
            anonymous_functions,
        )
    }

    pub(super) fn new_in_program(
        module: crate::plan::ModuleId,
        registry: &'a crate::planner::module::registry::ProgramRegistry,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self::new_with_registry(
            registry.module_name(module),
            RegistryAccess::Program { registry },
            anonymous_functions,
        )
    }

    fn new_with_registry(
        module_name: &'a EcoString,
        registry: RegistryAccess<'a>,
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self {
            module_name,
            current_function: "main".into(),
            registry,
            anonymous_functions,
            bindings: HashMap::new(),
            next_generic_local: 0,
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_external_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_external_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_generic_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_external_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
            next_generic_function_local: 0,
            type_parameters: super::type_parameter::TypeParameterScope::default(),
        }
    }

    pub(super) fn set_current_function(&mut self, name: EcoString) {
        self.current_function = name;
    }

    pub(super) fn set_type_parameters(
        &mut self,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) {
        self.type_parameters = type_parameters;
    }

    pub(super) fn type_parameters(&self) -> &super::type_parameter::TypeParameterScope {
        &self.type_parameters
    }

    pub(super) fn value_shape(&mut self, type_: &Type) -> ValueShape {
        let registry = self.registry;
        ValueShape::from_gleam_in_with_external(type_, &mut self.type_parameters, &|name| {
            registry.is_external_type(name)
        })
    }

    pub(super) fn value_shape_in_scope(&self, type_: &Type) -> ValueShape {
        let mut type_parameters = self.type_parameters.clone();
        self.value_shape_with_parameters(type_, &mut type_parameters)
    }

    pub(super) fn value_shape_with_parameters(
        &self,
        type_: &Type,
        type_parameters: &mut super::type_parameter::TypeParameterScope,
    ) -> ValueShape {
        ValueShape::from_gleam_in_with_external(type_, type_parameters, &|name| {
            self.registry.is_external_type(name)
        })
    }

    pub(super) fn is_external_type(&self, name: &crate::plan::ExternalTypeName) -> bool {
        self.registry.is_external_type(name)
    }

    pub(super) fn value_type(&mut self, type_: &Type) -> ValueType {
        self.value_shape(type_).value_type()
    }

    pub(super) fn panic_site(&self, location: gleam_core::ast::SrcSpan) -> PanicSite {
        PanicSite::new(
            self.module_name.clone(),
            self.current_function.clone(),
            location.into(),
        )
    }

    pub(super) fn echo_site(&self, location: gleam_core::ast::SrcSpan) -> crate::plan::EchoSite {
        crate::plan::EchoSite::new(
            self.module_name.clone(),
            self.current_function.clone(),
            location.into(),
        )
    }

    pub(super) fn host_call_site(
        &self,
        location: gleam_core::ast::SrcSpan,
    ) -> crate::plan::HostCallSite {
        crate::plan::HostCallSite::new(
            self.module_name.clone(),
            self.current_function.clone(),
            location.into(),
        )
    }

    pub(super) fn define_existing_local(&mut self, name: EcoString, local: LocalId) {
        match local {
            LocalId::Generic(local) => {
                self.next_generic_local = self.next_generic_local.max(local.id().0 + 1);
            }
            LocalId::Int(local) => {
                self.next_int_local = self.next_int_local.max(local.0 + 1);
            }
            LocalId::Float(local) => {
                self.next_float_local = self.next_float_local.max(local.0 + 1);
            }
            LocalId::String(local) => {
                self.next_string_local = self.next_string_local.max(local.0 + 1);
            }
            LocalId::BitArray(local) => {
                self.next_bit_array_local = self.next_bit_array_local.max(local.0 + 1);
            }
            LocalId::UtfCodepoint(local) => {
                self.next_utf_codepoint_local = self.next_utf_codepoint_local.max(local.0 + 1);
            }
            LocalId::Bool(local) => {
                self.next_bool_local = self.next_bool_local.max(local.0 + 1);
            }
            LocalId::Nil(local) => {
                self.next_nil_local = self.next_nil_local.max(local.0 + 1);
            }
        }
        self.bindings.insert(name, LocalBinding::Primitive(local));
    }

    pub(super) fn define_param_local_shape(
        &mut self,
        name: EcoString,
        shape: ValueShape,
    ) -> ParamLocal {
        match shape {
            ValueShape::Parameter(parameter) => {
                ParamLocal::generic(self.define_generic_local(name, parameter))
            }
            ValueShape::Int => ParamLocal::int(self.define_int_local(name)),
            ValueShape::Float => ParamLocal::float(self.define_float_local(name)),
            ValueShape::String => ParamLocal::string(self.define_string_local(name)),
            ValueShape::BitArray => ParamLocal::bit_array(self.define_bit_array_local(name)),
            ValueShape::UtfCodepoint => {
                ParamLocal::utf_codepoint(self.define_utf_codepoint_local(name))
            }
            ValueShape::Custom(shape) => {
                let local = self.define_custom_local_shape(name, shape.clone());
                ParamLocal::custom_shape(local, shape)
            }
            ValueShape::External(shape) => {
                let local = self.define_external_local_shape(name, shape.clone());
                ParamLocal::external_shape(local, shape)
            }
            ValueShape::Bool => ParamLocal::bool(self.define_bool_local(name)),
            ValueShape::Nil => ParamLocal::nil(self.define_nil_local(name)),
            ValueShape::Tuple(shape) => {
                let type_ = shape.iter().map(ValueShape::value_type).collect::<Vec<_>>();
                ParamLocal::tuple(self.define_tuple_local_shape(name, shape), type_)
            }
            ValueShape::List(element) => {
                ParamLocal::list(self.define_list_local_shape(name, *element))
            }
            ValueShape::Function(shape) => {
                let shape = *shape;
                let type_ = shape.type_();
                let return_shape = shape.return_shape().clone();
                match return_shape {
                    ValueShape::Parameter(parameter) => {
                        let type_ = crate::plan::GenericFunctionType::new(
                            shape.argument_shapes().to_vec(),
                            parameter,
                        );
                        ParamLocal::generic_function(
                            self.define_generic_function_local_shape(name, type_, shape),
                        )
                    }
                    ValueShape::Int => ParamLocal::int_function(
                        self.define_int_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Float => ParamLocal::float_function(
                        self.define_float_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::String => ParamLocal::string_function(
                        self.define_string_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::BitArray => ParamLocal::bit_array_function(
                        self.define_bit_array_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::UtfCodepoint => ParamLocal::utf_codepoint_function(
                        self.define_utf_codepoint_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Custom(return_) => {
                        let type_ = CustomFunctionType::from_shapes(
                            shape.argument_shapes().to_vec(),
                            return_.clone(),
                        );
                        ParamLocal::custom_function(
                            self.define_custom_function_local_shape(name, type_, shape),
                        )
                    }
                    ValueShape::External(return_) => {
                        let type_ = ExternalFunctionType::from_shapes(
                            shape.argument_shapes().to_vec(),
                            return_.clone(),
                        );
                        ParamLocal::external_function(
                            self.define_external_function_local_shape(name, type_, shape),
                        )
                    }
                    ValueShape::Bool => ParamLocal::bool_function(
                        self.define_bool_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Nil => ParamLocal::nil_function(
                        self.define_nil_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::Tuple(_) => ParamLocal::tuple_function(
                        self.define_tuple_function_local_shape(name, type_.clone(), shape),
                        type_,
                    ),
                    ValueShape::List(item_shape) => {
                        ParamLocal::list_function(self.define_list_function_local_shape(
                            name,
                            type_.clone(),
                            item_shape.value_type(),
                            shape,
                        ))
                    }
                    ValueShape::Function(return_) => {
                        let type_ = FunctionFunctionType::from_shapes(
                            shape.argument_shapes().to_vec(),
                            return_.as_ref().clone(),
                        );
                        ParamLocal::function_function(
                            self.define_function_function_local_shape(name, type_, shape),
                        )
                    }
                }
            }
        }
    }

    pub(super) fn define_existing_param(
        &mut self,
        name: EcoString,
        local: &ParamLocal,
        shape: ValueShape,
    ) -> Result<(), PlanError> {
        match (local, shape) {
            (ParamLocal::Generic(local), ValueShape::Parameter(parameter))
                if local.parameter() == parameter =>
            {
                self.define_existing_local(name, LocalId::Generic(*local));
            }
            (ParamLocal::Int(local), ValueShape::Int) => {
                self.define_existing_local(name, LocalId::Int(*local));
            }
            (ParamLocal::Float(local), ValueShape::Float) => {
                self.define_existing_local(name, LocalId::Float(*local));
            }
            (ParamLocal::String(local), ValueShape::String) => {
                self.define_existing_local(name, LocalId::String(*local));
            }
            (ParamLocal::BitArray(local), ValueShape::BitArray) => {
                self.define_existing_local(name, LocalId::BitArray(*local));
            }
            (ParamLocal::UtfCodepoint(local), ValueShape::UtfCodepoint) => {
                self.define_existing_local(name, LocalId::UtfCodepoint(*local));
            }
            (ParamLocal::Custom(local), ValueShape::Custom(shape)) if local.shape() == &shape => {
                self.next_custom_local = self.next_custom_local.max(local.id().0 + 1);
                self.bindings
                    .insert(name, LocalBinding::Custom(local.clone()));
            }
            (ParamLocal::External(local), ValueShape::External(shape))
                if local.shape() == &shape =>
            {
                self.next_external_local = self.next_external_local.max(local.id().0 + 1);
                self.bindings
                    .insert(name, LocalBinding::External(local.clone()));
            }
            (ParamLocal::Bool(local), ValueShape::Bool) => {
                self.define_existing_local(name, LocalId::Bool(*local));
            }
            (ParamLocal::Nil(local), ValueShape::Nil) => {
                self.define_existing_local(name, LocalId::Nil(*local));
            }
            (ParamLocal::Tuple { local, type_ }, ValueShape::Tuple(shape))
                if shape
                    .iter()
                    .map(ValueShape::value_type)
                    .eq(type_.iter().cloned()) =>
            {
                self.next_tuple_local = self.next_tuple_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Tuple {
                        local: *local,
                        shape,
                    },
                );
            }
            (ParamLocal::List(local), ValueShape::List(item_shape))
                if item_shape.value_type() == local.item_type() =>
            {
                self.bump_list_local(local);
                self.bindings.insert(
                    name,
                    LocalBinding::List {
                        local: local.clone(),
                        item_shape: *item_shape,
                    },
                );
            }
            (ParamLocal::IntFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_int_function_local = self.next_int_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Int {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::FloatFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_float_function_local = self.next_float_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Float {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::StringFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_string_function_local = self.next_string_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::String {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::BitArrayFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_bit_array_function_local =
                    self.next_bit_array_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::BitArray {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::UtfCodepointFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_utf_codepoint_function_local =
                    self.next_utf_codepoint_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::UtfCodepoint {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::CustomFunction(local), ValueShape::Function(shape))
                if shape.type_() == local.type_().to_function_type() =>
            {
                self.next_custom_function_local =
                    self.next_custom_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Custom(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::ExternalFunction(local), ValueShape::Function(shape))
                if shape.type_() == local.type_().to_function_type() =>
            {
                self.next_external_function_local =
                    self.next_external_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::External(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::BoolFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_bool_function_local = self.next_bool_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Bool {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::NilFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_nil_function_local = self.next_nil_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Nil {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::TupleFunction { local, type_ }, ValueShape::Function(shape))
                if shape.type_() == *type_ =>
            {
                self.next_tuple_function_local = self.next_tuple_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Tuple {
                            local: *local,
                            type_: type_.clone(),
                        },
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::ListFunction(local), ValueShape::Function(shape))
                if shape.type_() == *local.type_() =>
            {
                self.next_list_function_local =
                    self.next_list_function_local.max(local.index() + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::List(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::FunctionFunction(local), ValueShape::Function(shape))
                if shape.type_() == local.type_().to_function_type() =>
            {
                self.next_function_function_local =
                    self.next_function_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Function(local.clone()),
                        shape: *shape,
                    },
                );
            }
            (ParamLocal::GenericFunction(local), ValueShape::Function(shape))
                if local.type_().shape() == *shape =>
            {
                self.next_generic_function_local =
                    self.next_generic_function_local.max(local.id().0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function {
                        binding: FunctionLocalBinding::Generic(local.clone()),
                        shape: *shape,
                    },
                );
            }
            _ => return Err(invalid_local_shape()),
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn define_int_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> IntFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_int_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_int_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Int { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_int_function_local(&mut self) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_string_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> StringFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_string_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_string_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::String { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_string_function_local(&mut self) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_bit_array_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BitArrayFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_bit_array_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_bit_array_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::BitArray { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_bit_array_function_local(&mut self) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        local
    }

    pub(super) fn define_utf_codepoint_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> UtfCodepointFunctionLocalId {
        let local = UtfCodepointFunctionLocalId(self.next_utf_codepoint_function_local);
        self.next_utf_codepoint_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::UtfCodepoint { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_utf_codepoint_function_local(
        &mut self,
    ) -> UtfCodepointFunctionLocalId {
        let local = UtfCodepointFunctionLocalId(self.next_utf_codepoint_function_local);
        self.next_utf_codepoint_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_custom_function_local(
        &mut self,
        name: EcoString,
        type_: CustomFunctionType,
    ) -> CustomFunctionLocal {
        let shape = FunctionShape::new(
            type_.argument_shapes().to_vec(),
            ValueShape::Custom(type_.return_().clone()),
        );
        self.define_custom_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_custom_function_local_shape(
        &mut self,
        name: EcoString,
        type_: CustomFunctionType,
        shape: FunctionShape,
    ) -> CustomFunctionLocal {
        let local = CustomFunctionLocal::new(
            CustomFunctionLocalId(self.next_custom_function_local),
            type_,
        );
        self.next_custom_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Custom(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_custom_function_local(
        &mut self,
        type_: CustomFunctionType,
    ) -> CustomFunctionLocal {
        let local = CustomFunctionLocal::new(
            CustomFunctionLocalId(self.next_custom_function_local),
            type_,
        );
        self.next_custom_function_local += 1;
        local
    }

    pub(super) fn define_external_function_local_shape(
        &mut self,
        name: EcoString,
        type_: ExternalFunctionType,
        shape: FunctionShape,
    ) -> ExternalFunctionLocal {
        let local = ExternalFunctionLocal::new(
            ExternalFunctionLocalId(self.next_external_function_local),
            type_,
        );
        self.next_external_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::External(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_external_function_local(
        &mut self,
        type_: ExternalFunctionType,
    ) -> ExternalFunctionLocal {
        let local = ExternalFunctionLocal::new(
            ExternalFunctionLocalId(self.next_external_function_local),
            type_,
        );
        self.next_external_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_float_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FloatFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_float_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_float_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Float { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_float_function_local(&mut self) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_bool_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BoolFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_bool_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_bool_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Bool { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_bool_function_local(&mut self) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_nil_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> NilFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_nil_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_nil_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Nil { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_nil_function_local(&mut self) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_tuple_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> TupleFunctionLocalId {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_tuple_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_tuple_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        shape: FunctionShape,
    ) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Tuple { local, type_ },
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_tuple_function_local(&mut self) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_list_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionLocal {
        let shape = FunctionShape::from_function_type(type_.clone());
        self.define_list_function_local_shape(name, type_, item_type, shape)
    }

    pub(super) fn define_list_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        item_type: ValueType,
        shape: FunctionShape,
    ) -> ListFunctionLocal {
        let local =
            ListFunctionLocal::from_item_type(self.next_list_function_local, type_, item_type);
        self.next_list_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::List(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_list_function_local(
        &mut self,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionLocal {
        let local =
            ListFunctionLocal::from_item_type(self.next_list_function_local, type_, item_type);
        self.next_list_function_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_function_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionFunctionType,
    ) -> FunctionFunctionLocal {
        let shape = FunctionShape::from_function_type(type_.to_function_type());
        self.define_function_function_local_shape(name, type_, shape)
    }

    pub(super) fn define_function_function_local_shape(
        &mut self,
        name: EcoString,
        type_: FunctionFunctionType,
        shape: FunctionShape,
    ) -> FunctionFunctionLocal {
        let local = FunctionFunctionLocal::new(
            FunctionFunctionLocalId(self.next_function_function_local),
            type_,
        );
        self.next_function_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Function(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_function_function_local(
        &mut self,
        type_: FunctionFunctionType,
    ) -> FunctionFunctionLocal {
        let local = FunctionFunctionLocal::new(
            FunctionFunctionLocalId(self.next_function_function_local),
            type_,
        );
        self.next_function_function_local += 1;
        local
    }

    pub(super) fn define_int_local(&mut self, name: EcoString) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Int(local)));
        local
    }

    pub(super) fn define_generic_local(
        &mut self,
        name: EcoString,
        parameter: crate::plan::TypeParameterId,
    ) -> crate::plan::GenericLocal {
        let local = crate::plan::GenericLocal::new(
            crate::plan::GenericLocalId(self.next_generic_local),
            parameter,
        );
        self.next_generic_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Generic(local)));
        local
    }

    pub(super) fn define_internal_generic_local(
        &mut self,
        parameter: crate::plan::TypeParameterId,
    ) -> crate::plan::GenericLocal {
        let local = crate::plan::GenericLocal::new(
            crate::plan::GenericLocalId(self.next_generic_local),
            parameter,
        );
        self.next_generic_local += 1;
        local
    }

    pub(super) fn define_generic_function_local_shape(
        &mut self,
        name: EcoString,
        type_: crate::plan::GenericFunctionType,
        shape: FunctionShape,
    ) -> crate::plan::GenericFunctionLocal {
        let local = crate::plan::GenericFunctionLocal::new(
            crate::plan::GenericFunctionLocalId(self.next_generic_function_local),
            type_,
        );
        self.next_generic_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function {
                binding: FunctionLocalBinding::Generic(local.clone()),
                shape,
            },
        );
        local
    }

    pub(super) fn define_internal_generic_function_local(
        &mut self,
        type_: crate::plan::GenericFunctionType,
    ) -> crate::plan::GenericFunctionLocal {
        let local = crate::plan::GenericFunctionLocal::new(
            crate::plan::GenericFunctionLocalId(self.next_generic_function_local),
            type_,
        );
        self.next_generic_function_local += 1;
        local
    }

    pub(super) fn define_internal_int_local(&mut self) -> IntLocalId {
        let local = IntLocalId(self.next_int_local);
        self.next_int_local += 1;
        local
    }

    pub(super) fn define_string_local(&mut self, name: EcoString) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::String(local)));
        local
    }

    pub(super) fn define_internal_string_local(&mut self) -> StringLocalId {
        let local = StringLocalId(self.next_string_local);
        self.next_string_local += 1;
        local
    }

    pub(super) fn define_bit_array_local(&mut self, name: EcoString) -> BitArrayLocalId {
        let local = BitArrayLocalId(self.next_bit_array_local);
        self.next_bit_array_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::BitArray(local)));
        local
    }

    pub(super) fn define_internal_bit_array_local(&mut self) -> BitArrayLocalId {
        let local = BitArrayLocalId(self.next_bit_array_local);
        self.next_bit_array_local += 1;
        local
    }

    pub(super) fn define_utf_codepoint_local(&mut self, name: EcoString) -> UtfCodepointLocalId {
        let local = UtfCodepointLocalId(self.next_utf_codepoint_local);
        self.next_utf_codepoint_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::UtfCodepoint(local)));
        local
    }

    pub(super) fn define_internal_utf_codepoint_local(&mut self) -> UtfCodepointLocalId {
        let local = UtfCodepointLocalId(self.next_utf_codepoint_local);
        self.next_utf_codepoint_local += 1;
        local
    }

    pub(super) fn define_custom_local_shape(
        &mut self,
        name: EcoString,
        shape: CustomValueShape,
    ) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Custom(crate::plan::CustomLocal::from_shape(local, shape)),
        );
        local
    }

    pub(super) fn define_internal_custom_local(&mut self) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
        local
    }

    pub(super) fn define_external_local_shape(
        &mut self,
        name: EcoString,
        shape: crate::plan::ExternalValueShape,
    ) -> ExternalLocalId {
        let local = ExternalLocalId(self.next_external_local);
        self.next_external_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::External(crate::plan::ExternalLocal::from_shape(local, shape)),
        );
        local
    }

    pub(super) fn define_internal_external_local(&mut self) -> ExternalLocalId {
        let local = ExternalLocalId(self.next_external_local);
        self.next_external_local += 1;
        local
    }

    pub(super) fn define_float_local(&mut self, name: EcoString) -> FloatLocalId {
        let local = FloatLocalId(self.next_float_local);
        self.next_float_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Float(local)));
        local
    }

    pub(super) fn define_internal_float_local(&mut self) -> FloatLocalId {
        let local = FloatLocalId(self.next_float_local);
        self.next_float_local += 1;
        local
    }

    pub(super) fn define_bool_local(&mut self, name: EcoString) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Bool(local)));
        local
    }

    pub(super) fn define_internal_bool_local(&mut self) -> BoolLocalId {
        let local = BoolLocalId(self.next_bool_local);
        self.next_bool_local += 1;
        local
    }

    pub(super) fn define_nil_local(&mut self, name: EcoString) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        self.bindings
            .insert(name, LocalBinding::Primitive(LocalId::Nil(local)));
        local
    }

    pub(super) fn define_internal_nil_local(&mut self) -> NilLocalId {
        let local = NilLocalId(self.next_nil_local);
        self.next_nil_local += 1;
        local
    }

    #[cfg(test)]
    pub(super) fn define_tuple_local(
        &mut self,
        name: EcoString,
        type_: Vec<ValueType>,
    ) -> TupleLocalId {
        let shape = type_
            .into_iter()
            .map(ValueShape::from_value_type)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.define_tuple_local_shape(name, shape)
    }

    pub(super) fn define_tuple_local_shape(
        &mut self,
        name: EcoString,
        shape: Box<[ValueShape]>,
    ) -> TupleLocalId {
        let local = TupleLocalId(self.next_tuple_local);
        self.next_tuple_local += 1;
        self.bindings
            .insert(name, LocalBinding::Tuple { local, shape });
        local
    }

    pub(super) fn define_internal_tuple_local(&mut self) -> TupleLocalId {
        let local = TupleLocalId(self.next_tuple_local);
        self.next_tuple_local += 1;
        local
    }

    pub(super) fn define_list_local(
        &mut self,
        name: EcoString,
        element_type: ValueType,
    ) -> ListLocal {
        self.define_list_local_shape(name, ValueShape::from_value_type(element_type))
    }

    pub(super) fn define_list_local_shape(
        &mut self,
        name: EcoString,
        item_shape: ValueShape,
    ) -> ListLocal {
        let local = self.next_list_local(item_shape.value_type());
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        local
    }

    pub(super) fn define_list_value(
        &mut self,
        name: EcoString,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        let item_shape = value.item_shape().clone();
        let (local, value) = self.next_list_local_expr(value);
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        (local, value)
    }

    pub(super) fn define_internal_list_value(
        &mut self,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        self.next_list_local_expr(value)
    }

    fn define_list_capture_value(
        &mut self,
        name: EcoString,
        source: ListLocal,
        item_shape: ValueShape,
    ) -> PlannedCapture {
        let local = self.next_list_local(item_shape.value_type());
        self.bindings.insert(
            name,
            LocalBinding::List {
                local: local.clone(),
                item_shape: item_shape.clone(),
            },
        );
        Self::planned_capture(
            ParamLocal::list(local),
            ParamLocal::list(source),
            ValueShape::List(Box::new(item_shape)),
        )
    }

    fn next_list_local(&mut self, element_type: ValueType) -> ListLocal {
        match element_type {
            ValueType::Parameter(parameter) => {
                let local = crate::plan::GenericListLocalId(self.next_generic_list_local);
                self.next_generic_list_local += 1;
                ListLocal::generic(local, parameter)
            }
            ValueType::Int => {
                let local = IntListLocalId(self.next_int_list_local);
                self.next_int_list_local += 1;
                ListLocal::int(local)
            }
            ValueType::String => {
                let local = StringListLocalId(self.next_string_list_local);
                self.next_string_list_local += 1;
                ListLocal::string(local)
            }
            ValueType::BitArray => {
                let local = BitArrayListLocalId(self.next_bit_array_list_local);
                self.next_bit_array_list_local += 1;
                ListLocal::bit_array(local)
            }
            ValueType::UtfCodepoint => {
                let local = UtfCodepointListLocalId(self.next_utf_codepoint_list_local);
                self.next_utf_codepoint_list_local += 1;
                ListLocal::utf_codepoint(local)
            }
            ValueType::Custom(item_type) => {
                let local = CustomListLocalId(self.next_custom_list_local);
                self.next_custom_list_local += 1;
                ListLocal::custom(local, item_type)
            }
            ValueType::External(item_type) => {
                let local = ExternalListLocalId(self.next_external_list_local);
                self.next_external_list_local += 1;
                ListLocal::external(local, item_type)
            }
            ValueType::Float => {
                let local = FloatListLocalId(self.next_float_list_local);
                self.next_float_list_local += 1;
                ListLocal::float(local)
            }
            ValueType::Bool => {
                let local = BoolListLocalId(self.next_bool_list_local);
                self.next_bool_list_local += 1;
                ListLocal::bool(local)
            }
            ValueType::Nil => {
                let local = NilListLocalId(self.next_nil_list_local);
                self.next_nil_list_local += 1;
                ListLocal::nil(local)
            }
            ValueType::Tuple(item_type) => {
                let local = TupleListLocalId(self.next_tuple_list_local);
                self.next_tuple_list_local += 1;
                ListLocal::tuple(local, item_type)
            }
            ValueType::List(item_type) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                ListLocal::list(local, *item_type)
            }
            ValueType::Function(item_type) => {
                let local = FunctionListLocalId(self.next_function_list_local);
                self.next_function_list_local += 1;
                ListLocal::function(local, *item_type)
            }
        }
    }

    fn next_list_local_expr(&mut self, value: ListExpr) -> (ListLocal, ListLocalExpr) {
        match value {
            ListExpr::Generic(value) => {
                let local = crate::plan::GenericListLocalId(self.next_generic_list_local);
                self.next_generic_list_local += 1;
                let parameter = value.item().parameter();
                (
                    ListLocal::generic(local, parameter),
                    ListLocalExpr::Generic {
                        local,
                        parameter,
                        value,
                    },
                )
            }
            ListExpr::ParameterList(value) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                let parameter = value.item().parameter();
                (
                    ListLocal::list(local, ValueType::Parameter(parameter)),
                    ListLocalExpr::ParameterList {
                        local,
                        parameter,
                        value,
                    },
                )
            }
            ListExpr::Int(value) => {
                let local = IntListLocalId(self.next_int_list_local);
                self.next_int_list_local += 1;
                (ListLocal::int(local), ListLocalExpr::Int { local, value })
            }
            ListExpr::String(value) => {
                let local = StringListLocalId(self.next_string_list_local);
                self.next_string_list_local += 1;
                (
                    ListLocal::string(local),
                    ListLocalExpr::String { local, value },
                )
            }
            ListExpr::BitArray(value) => {
                let local = BitArrayListLocalId(self.next_bit_array_list_local);
                self.next_bit_array_list_local += 1;
                (
                    ListLocal::bit_array(local),
                    ListLocalExpr::BitArray { local, value },
                )
            }
            ListExpr::UtfCodepoint(value) => {
                let local = UtfCodepointListLocalId(self.next_utf_codepoint_list_local);
                self.next_utf_codepoint_list_local += 1;
                (
                    ListLocal::utf_codepoint(local),
                    ListLocalExpr::UtfCodepoint { local, value },
                )
            }
            ListExpr::Custom(value) => {
                let local = CustomListLocalId(self.next_custom_list_local);
                self.next_custom_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::custom(local, item_type.clone()),
                    ListLocalExpr::Custom {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::External(value) => {
                let local = ExternalListLocalId(self.next_external_list_local);
                self.next_external_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::external(local, item_type.clone()),
                    ListLocalExpr::External {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::Float(value) => {
                let local = FloatListLocalId(self.next_float_list_local);
                self.next_float_list_local += 1;
                (
                    ListLocal::float(local),
                    ListLocalExpr::Float { local, value },
                )
            }
            ListExpr::Bool(value) => {
                let local = BoolListLocalId(self.next_bool_list_local);
                self.next_bool_list_local += 1;
                (ListLocal::bool(local), ListLocalExpr::Bool { local, value })
            }
            ListExpr::Nil(value) => {
                let local = NilListLocalId(self.next_nil_list_local);
                self.next_nil_list_local += 1;
                (ListLocal::nil(local), ListLocalExpr::Nil { local, value })
            }
            ListExpr::Tuple(value) => {
                let local = TupleListLocalId(self.next_tuple_list_local);
                self.next_tuple_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::tuple(local, item_type.clone()),
                    ListLocalExpr::Tuple {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::List(value) => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::list(local, item_type.as_ref().clone()),
                    ListLocalExpr::List {
                        local,
                        item_type,
                        value,
                    },
                )
            }
            ListExpr::Function(value) => {
                let local = FunctionListLocalId(self.next_function_list_local);
                self.next_function_list_local += 1;
                let item_type = value.item().item_type();
                (
                    ListLocal::function(local, item_type.clone()),
                    ListLocalExpr::Function {
                        local,
                        item_type,
                        value,
                    },
                )
            }
        }
    }

    fn bump_list_local(&mut self, local: &ListLocal) {
        match local {
            ListLocal::Generic { local, .. } => {
                self.next_generic_list_local = self.next_generic_list_local.max(local.0 + 1);
            }
            ListLocal::Int(local) => {
                self.next_int_list_local = self.next_int_list_local.max(local.0 + 1);
            }
            ListLocal::String(local) => {
                self.next_string_list_local = self.next_string_list_local.max(local.0 + 1);
            }
            ListLocal::BitArray(local) => {
                self.next_bit_array_list_local = self.next_bit_array_list_local.max(local.0 + 1);
            }
            ListLocal::UtfCodepoint(local) => {
                self.next_utf_codepoint_list_local =
                    self.next_utf_codepoint_list_local.max(local.0 + 1);
            }
            ListLocal::Custom { local, .. } => {
                self.next_custom_list_local = self.next_custom_list_local.max(local.0 + 1);
            }
            ListLocal::External { local, .. } => {
                self.next_external_list_local = self.next_external_list_local.max(local.0 + 1);
            }
            ListLocal::Float(local) => {
                self.next_float_list_local = self.next_float_list_local.max(local.0 + 1);
            }
            ListLocal::Bool(local) => {
                self.next_bool_list_local = self.next_bool_list_local.max(local.0 + 1);
            }
            ListLocal::Nil(local) => {
                self.next_nil_list_local = self.next_nil_list_local.max(local.0 + 1);
            }
            ListLocal::Tuple { local, .. } => {
                self.next_tuple_list_local = self.next_tuple_list_local.max(local.0 + 1);
            }
            ListLocal::List { local, .. } => {
                self.next_list_list_local = self.next_list_list_local.max(local.0 + 1);
            }
            ListLocal::Function { local, .. } => {
                self.next_function_list_local = self.next_function_list_local.max(local.0 + 1);
            }
        }
    }

    pub(super) fn lookup_local(&self, name: &EcoString) -> Option<(LocalId, ValueType)> {
        match self.bindings.get(name)? {
            LocalBinding::Primitive(local) => Some((*local, local.value_type())),
            LocalBinding::Custom(_)
            | LocalBinding::External(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_custom_local(&self, name: &EcoString) -> Option<crate::plan::CustomLocal> {
        match self.bindings.get(name)? {
            LocalBinding::Custom(local) => Some(local.clone()),
            LocalBinding::Primitive(_)
            | LocalBinding::External(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_tuple_local(
        &self,
        name: &EcoString,
    ) -> Option<(TupleLocalId, Box<[ValueShape]>)> {
        match self.bindings.get(name)? {
            LocalBinding::Tuple { local, shape } => Some((*local, shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::External(_)
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_list_local(&self, name: &EcoString) -> Option<(ListLocal, ValueShape)> {
        match self.bindings.get(name)? {
            LocalBinding::List { local, item_shape } => Some((local.clone(), item_shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::External(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn lookup_external_local(
        &self,
        name: &EcoString,
    ) -> Option<crate::plan::ExternalLocal> {
        match self.bindings.get(name)? {
            LocalBinding::External(local) => Some(local.clone()),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. }
            | LocalBinding::Function { .. } => None,
        }
    }

    pub(super) fn resolve_module_reference(
        &self,
        module: &EcoString,
        name: &EcoString,
    ) -> Result<ModuleId, PlanError> {
        let module_id = match self.registry {
            RegistryAccess::Program { registry } => registry.module_id(module),
            #[cfg(test)]
            RegistryAccess::Local { .. } if module == self.module_name => Some(ModuleId::root()),
            #[cfg(test)]
            RegistryAccess::Local { .. } => None,
        };
        module_id.ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ModuleReference {
                module: module.clone(),
                name: name.clone(),
                reason: InvalidModuleReferenceReason::UnlinkedModule,
            },
        })
    }

    pub(super) fn module_function(
        &self,
        target: &ValidatedModuleFunctionTarget,
    ) -> Result<FunctionInfo, PlanError> {
        let module_id = match target.link {
            ModuleFunctionLink::Unresolved => {
                self.resolve_module_reference(&target.module, &target.name)?
            }
            ModuleFunctionLink::Resolved(module) => module,
        };
        let function = match self.registry {
            RegistryAccess::Program { registry } => registry.function(module_id, &target.name),
            #[cfg(test)]
            RegistryAccess::Local { functions, .. } => functions.get(&target.name).cloned(),
        };
        function.ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ModuleReference {
                module: target.module.clone(),
                name: target.name.clone(),
                reason: InvalidModuleReferenceReason::MissingFunction,
            },
        })
    }

    pub(super) fn module_constant_expr(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Result<crate::plan::Expr, PlanError> {
        self.module_constant_instantiation(module, name, shape)
            .map(crate::plan::module::ConstantTemplates::reference)
    }

    pub(super) fn module_constant_instantiation(
        &self,
        module: &EcoString,
        name: &EcoString,
        shape: &ValueShape,
    ) -> Result<crate::plan::ConstantInstantiation, PlanError> {
        let module_id = self.resolve_module_reference(module, name)?;
        let result = match self.registry {
            RegistryAccess::Program { registry } => {
                registry.constant_instantiation(module_id, name, shape)
            }
            #[cfg(test)]
            RegistryAccess::Local { .. } => Err(
                crate::planner::module::registry::ModuleConstantResolutionError::MissingConstant,
            ),
        };
        result.map_err(|error| {
            let reason = match error {
                crate::planner::module::registry::ModuleConstantResolutionError::MissingConstant => {
                    InvalidModuleReferenceReason::MissingConstant
                }
                crate::planner::module::registry::ModuleConstantResolutionError::Instantiation => {
                    InvalidModuleReferenceReason::ConstantInstantiation
                }
            };
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: module.clone(),
                    name: name.clone(),
                    reason,
                },
            }
        })
    }

    fn executable_external(&self, module: &EcoString, name: &EcoString) -> bool {
        match self.registry {
            RegistryAccess::Program { registry } => registry.executable_external(module, name),
            #[cfg(test)]
            RegistryAccess::Local { .. } => false,
        }
    }

    pub(super) fn lookup_function_local(
        &self,
        name: &EcoString,
    ) -> Option<(FunctionLocalBinding, FunctionShape)> {
        match self.bindings.get(name)? {
            LocalBinding::Function { binding, shape } => Some((binding.clone(), shape.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom(_)
            | LocalBinding::External(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List { .. } => None,
        }
    }

    pub(super) fn anonymous_function_error_name(&self) -> EcoString {
        self.anonymous_functions.next_name()
    }

    pub(super) fn reserve_anonymous_function_name(&mut self) -> EcoString {
        self.anonymous_functions.reserve_name()
    }

    pub(super) fn allocate_anonymous_function_shape(
        &mut self,
        name: EcoString,
        return_shape: ValueShape,
        params: Vec<FunctionParam>,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> (EcoString, FunctionInfo) {
        self.anonymous_functions
            .allocate(name, return_shape, params, type_parameters)
    }

    pub(super) fn push_anonymous_function(&mut self, function: FunctionTemplate) {
        self.anonymous_functions.push(function);
    }

    pub(super) fn anonymous_function_context(
        &mut self,
        function_name: EcoString,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> PlanContext<'_> {
        PlanContext {
            module_name: self.module_name,
            current_function: function_name,
            registry: self.registry,
            anonymous_functions: self.anonymous_functions,
            bindings: HashMap::new(),
            next_generic_local: 0,
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_external_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_external_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_generic_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_external_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
            next_generic_function_local: 0,
            type_parameters,
        }
    }

    pub(super) fn capture_bindings(
        &self,
        names: &[EcoString],
    ) -> Result<Vec<CaptureBinding>, PlanError> {
        names
            .iter()
            .map(|name| {
                let Some(binding) = self.bindings.get(name).cloned() else {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                    });
                };

                Ok(CaptureBinding {
                    name: name.clone(),
                    binding,
                })
            })
            .collect()
    }

    pub(super) fn define_captures(&mut self, captures: Vec<CaptureBinding>) -> PlannedCaptures {
        let captures = captures
            .into_iter()
            .map(|capture| self.define_capture(capture))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        PlannedCaptures { captures }
    }

    pub(super) fn with_local_scope<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let bindings = self.bindings.clone();
        let result = f(self);
        self.bindings = bindings;
        result
    }

    fn define_capture(&mut self, capture: CaptureBinding) -> PlannedCapture {
        let name = capture.name;
        match capture.binding {
            LocalBinding::Primitive(LocalId::Generic(local)) => {
                let target = self.define_generic_local(name, local.parameter());
                Self::planned_capture(
                    ParamLocal::generic(target),
                    ParamLocal::generic(local),
                    ValueShape::Parameter(local.parameter()),
                )
            }
            LocalBinding::Primitive(LocalId::Int(local)) => {
                let target = self.define_int_local(name);
                Self::planned_capture(
                    ParamLocal::int(target),
                    ParamLocal::int(local),
                    ValueShape::Int,
                )
            }
            LocalBinding::Primitive(LocalId::Float(local)) => {
                let target = self.define_float_local(name);
                Self::planned_capture(
                    ParamLocal::float(target),
                    ParamLocal::float(local),
                    ValueShape::Float,
                )
            }
            LocalBinding::Primitive(LocalId::String(local)) => {
                let target = self.define_string_local(name);
                Self::planned_capture(
                    ParamLocal::string(target),
                    ParamLocal::string(local),
                    ValueShape::String,
                )
            }
            LocalBinding::Primitive(LocalId::BitArray(local)) => {
                let target = self.define_bit_array_local(name);
                Self::planned_capture(
                    ParamLocal::bit_array(target),
                    ParamLocal::bit_array(local),
                    ValueShape::BitArray,
                )
            }
            LocalBinding::Primitive(LocalId::UtfCodepoint(local)) => {
                let target = self.define_utf_codepoint_local(name);
                Self::planned_capture(
                    ParamLocal::utf_codepoint(target),
                    ParamLocal::utf_codepoint(local),
                    ValueShape::UtfCodepoint,
                )
            }
            LocalBinding::Custom(local) => {
                let shape = local.shape().clone();
                let target = self.define_custom_local_shape(name, shape.clone());
                Self::planned_capture(
                    ParamLocal::custom_shape(target, shape.clone()),
                    ParamLocal::Custom(local),
                    ValueShape::Custom(shape),
                )
            }
            LocalBinding::External(local) => {
                let shape = local.shape().clone();
                let target = self.define_external_local_shape(name, shape.clone());
                Self::planned_capture(
                    ParamLocal::external_shape(target, shape.clone()),
                    ParamLocal::External(local),
                    ValueShape::External(shape),
                )
            }
            LocalBinding::Primitive(LocalId::Bool(local)) => {
                let target = self.define_bool_local(name);
                Self::planned_capture(
                    ParamLocal::bool(target),
                    ParamLocal::bool(local),
                    ValueShape::Bool,
                )
            }
            LocalBinding::Primitive(LocalId::Nil(local)) => {
                let target = self.define_nil_local(name);
                Self::planned_capture(
                    ParamLocal::nil(target),
                    ParamLocal::nil(local),
                    ValueShape::Nil,
                )
            }
            LocalBinding::Tuple { local, shape } => {
                let type_: Vec<ValueType> = shape.iter().map(ValueShape::value_type).collect();
                let target = self.define_tuple_local_shape(name, shape.clone());
                Self::planned_capture(
                    ParamLocal::tuple(target, type_.clone()),
                    ParamLocal::tuple(local, type_),
                    ValueShape::Tuple(shape),
                )
            }
            LocalBinding::List { local, item_shape } => {
                self.define_list_capture_value(name, local, item_shape)
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Generic(local),
                shape,
            } => {
                let target = self.define_generic_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::generic_function(target),
                    ParamLocal::generic_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Int { local, type_ },
                shape,
            } => {
                let target =
                    self.define_int_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::int_function(target, type_.clone()),
                    ParamLocal::int_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Float { local, type_ },
                shape,
            } => {
                let target =
                    self.define_float_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::float_function(target, type_.clone()),
                    ParamLocal::float_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::String { local, type_ },
                shape,
            } => {
                let target =
                    self.define_string_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::string_function(target, type_.clone()),
                    ParamLocal::string_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::BitArray { local, type_ },
                shape,
            } => {
                let target =
                    self.define_bit_array_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::bit_array_function(target, type_.clone()),
                    ParamLocal::bit_array_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::UtfCodepoint { local, type_ },
                shape,
            } => {
                let target = self.define_utf_codepoint_function_local_shape(
                    name,
                    type_.clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::utf_codepoint_function(target, type_.clone()),
                    ParamLocal::utf_codepoint_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Custom(local),
                shape,
            } => {
                let target = self.define_custom_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::custom_function(target),
                    ParamLocal::custom_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::External(local),
                shape,
            } => {
                let target = self.define_external_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::external_function(target),
                    ParamLocal::external_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Bool { local, type_ },
                shape,
            } => {
                let target =
                    self.define_bool_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::bool_function(target, type_.clone()),
                    ParamLocal::bool_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Nil { local, type_ },
                shape,
            } => {
                let target =
                    self.define_nil_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::nil_function(target, type_.clone()),
                    ParamLocal::nil_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Tuple { local, type_ },
                shape,
            } => {
                let target =
                    self.define_tuple_function_local_shape(name, type_.clone(), shape.clone());
                Self::planned_capture(
                    ParamLocal::tuple_function(target, type_.clone()),
                    ParamLocal::tuple_function(local, type_),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::List(local),
                shape,
            } => {
                let target = self.define_list_function_local_shape(
                    name,
                    local.type_().clone(),
                    local.item_type(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::list_function(target),
                    ParamLocal::list_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
            LocalBinding::Function {
                binding: FunctionLocalBinding::Function(local),
                shape,
            } => {
                let target = self.define_function_function_local_shape(
                    name,
                    local.type_().clone(),
                    shape.clone(),
                );
                Self::planned_capture(
                    ParamLocal::function_function(target),
                    ParamLocal::function_function(local),
                    ValueShape::Function(Box::new(shape)),
                )
            }
        }
    }

    fn planned_capture(local: ParamLocal, source: ParamLocal, shape: ValueShape) -> PlannedCapture {
        PlannedCapture {
            slot: ParamSlot::new(local, shape),
            source: CaptureArg::new(source),
        }
    }
}

impl RegistryAccess<'_> {
    fn is_external_type(&self, name: &crate::plan::ExternalTypeName) -> bool {
        match self {
            Self::Program { registry } => registry.external_type(name).is_some(),
            #[cfg(test)]
            Self::Local { .. } => false,
        }
    }
}

pub(in crate::planner) struct AnonymousFunctions {
    module: crate::plan::ModuleId,
    next_function_index: usize,
    next_anonymous_index: usize,
    functions: Vec<FunctionTemplate>,
}

impl AnonymousFunctions {
    pub(in crate::planner) fn new(next_function_index: usize) -> Self {
        Self::in_module(crate::plan::ModuleId::root(), next_function_index)
    }

    pub(in crate::planner) fn in_module(
        module: crate::plan::ModuleId,
        next_function_index: usize,
    ) -> Self {
        Self {
            module,
            next_function_index,
            next_anonymous_index: 0,
            functions: Vec::new(),
        }
    }

    pub(in crate::planner) fn into_functions(mut self) -> Vec<FunctionTemplate> {
        self.functions.sort_by_key(|function| function.id().index());
        self.functions
    }

    fn next_name(&self) -> EcoString {
        format!("<anonymous:{}>", self.next_anonymous_index).into()
    }

    fn reserve_name(&mut self) -> EcoString {
        let name = self.next_name();
        self.next_anonymous_index += 1;
        name
    }

    fn allocate(
        &mut self,
        name: EcoString,
        return_shape: ValueShape,
        params: Vec<FunctionParam>,
        type_parameters: super::type_parameter::TypeParameterScope,
    ) -> (EcoString, FunctionInfo) {
        let id = crate::plan::FunctionTemplateId::in_module(self.module, self.next_function_index);
        let shape = FunctionShape::new(
            params.iter().map(|param| param.shape().clone()).collect(),
            return_shape.clone(),
        );
        let info = FunctionInfo {
            signature: FunctionTemplateSignature::new(id, type_parameters.scheme(), shape),
            type_parameters,
            return_shape,
            params,
            definition_span: crate::plan::SourceSpan::new(0, 0),
        };
        self.next_function_index += 1;
        (name, info)
    }

    fn push(&mut self, function: FunctionTemplate) {
        self.functions.push(function);
    }
}

impl Default for AnonymousFunctions {
    fn default() -> Self {
        Self::new(0)
    }
}

impl FunctionInfo {
    pub(super) fn arity(&self) -> usize {
        self.params.len()
    }

    pub(super) fn return_shape(&self) -> ValueShape {
        self.return_shape.clone()
    }

    pub(super) fn instantiate(
        &self,
        actual: &FunctionShape,
    ) -> Result<
        crate::plan::FunctionInstantiation,
        super::type_parameter::FunctionInstantiationMismatch,
    > {
        super::type_parameter::instantiate(&self.signature, actual)
    }

    pub(super) fn reference(
        &self,
        instantiation: crate::plan::FunctionInstantiation,
    ) -> FunctionReference {
        FunctionReference::new(instantiation)
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::FunctionLocalBinding;
    use super::{
        AnonymousFunctions, FunctionInfo, ModuleFunctionTarget, PlanContext, PlannedCapture,
    };
    use crate::plan::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CaptureArg, CustomFunctionLocal, CustomFunctionLocalId,
        CustomFunctionType, CustomType, CustomTypeName, ExternalFunctionLocal,
        ExternalFunctionLocalId, ExternalFunctionType, ExternalListLocalId, ExternalLocalId,
        ExternalTypeName, ExternalValueShape, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionType, FunctionListLocalId,
        FunctionShape, FunctionType, GenericFunctionLocal, GenericFunctionLocalId,
        GenericFunctionType, GenericListLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId,
        ListExpr, ListListLocalId, ListLocal, ListLocalExpr, LocalId, NilFunctionLocalId,
        NilListLocalId, NilLocalId, ParamLocal, ParamSlot, StringFunctionLocalId,
        StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        TypeParameterId, UtfCodepointListLocalId, ValueShape, ValueType,
    };
    use crate::planner::{InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use std::collections::HashMap;

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn local_registry_lookups_preserve_missing_and_unlinked_boundaries() {
        let module = EcoString::from("main");
        let mut anonymous = AnonymousFunctions::default();
        let (function_name, function) = anonymous.allocate(
            "present".into(),
            ValueShape::Int,
            Vec::new(),
            Default::default(),
        );
        let function_id = function.signature.id();
        let functions = HashMap::from([(function_name, function)]);
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context.module_constant_expr(&module, &EcoString::from("missing"), &ValueShape::Int,),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "missing".into(),
                    reason: InvalidModuleReferenceReason::MissingConstant,
                },
            }),
        );
        assert_eq!(
            context.resolve_module_reference(&module, &EcoString::from("present")),
            Ok(crate::plan::ModuleId::root()),
        );
        assert_eq!(
            context
                .module_function(
                    &ModuleFunctionTarget::direct(module.clone(), "present".into(), false)
                        .validate_external(&context)
                        .expect("local function target should validate"),
                )
                .map(function_template_id),
            Ok(function_id),
        );
        assert_eq!(
            context
                .module_function(
                    &ModuleFunctionTarget::direct(module.clone(), "missing".into(), false)
                        .validate_external(&context)
                        .expect("local function target should validate"),
                )
                .map(function_template_id),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "missing".into(),
                    reason: InvalidModuleReferenceReason::MissingFunction,
                },
            }),
        );
        assert_eq!(
            context
                .resolve_module_reference(&EcoString::from("other"), &EcoString::from("present"),),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "present".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
    }

    #[test]
    fn module_function_target_preserves_selected_validation_order() {
        let module = EcoString::from("main");
        let mut anonymous = AnonymousFunctions::default();
        let functions = HashMap::new();
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            ModuleFunctionTarget::selected(
                &context,
                module.clone(),
                "run".into(),
                "other".into(),
                "wrong".into(),
                true,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "run".into(),
                    reason: InvalidModuleReferenceReason::FunctionModule {
                        actual: "other".into(),
                    },
                },
            }),
        );
        assert_eq!(
            ModuleFunctionTarget::selected(
                &context,
                module.clone(),
                "run".into(),
                module.clone(),
                "wrong".into(),
                true,
            )
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "run".into(),
                    reason: InvalidModuleReferenceReason::FunctionName {
                        actual: "wrong".into(),
                    },
                },
            }),
        );
        assert_eq!(
            ModuleFunctionTarget::selected(
                &context,
                module.clone(),
                "run".into(),
                module.clone(),
                "run".into(),
                true,
            )
            .and_then(|target| target.validate_external(&context))
            .map(|_| ()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "run".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );
    }

    #[test]
    fn module_function_target_owns_reference_shape_and_instantiation_errors() {
        let module = EcoString::from("main");
        let mut anonymous = AnonymousFunctions::default();
        let (_, function) = anonymous.allocate(
            "run".into(),
            ValueShape::Int,
            Vec::new(),
            Default::default(),
        );
        let functions = HashMap::from([(EcoString::from("run"), function.clone())]);
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let target = ModuleFunctionTarget::direct(module.clone(), "run".into(), false)
            .validate_external(&context)
            .expect("local function target should validate");

        assert_eq!(
            target.function_shape(ValueShape::Int),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "run".into(),
                    reason: InvalidModuleReferenceReason::FunctionType,
                },
            }),
        );
        assert_eq!(
            target.instantiate_reference(
                &function,
                &FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "run".into(),
                    reason: InvalidModuleReferenceReason::FunctionInstantiation,
                },
            }),
        );
    }

    fn function_template_id(function: FunctionInfo) -> crate::plan::FunctionTemplateId {
        function.signature.id()
    }

    #[test]
    fn local_scope_restores_names_after_error_without_reusing_ids() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(context.define_int_local("x".into()), IntLocalId(0));
        let result = context.with_local_scope(|context| {
            assert_eq!(context.define_int_local("x".into()), IntLocalId(1));
            Err::<(), _>(())
        });

        assert_eq!(result, Err(()));
        assert_eq!(
            context.lookup_local(&"x".into()),
            Some((LocalId::Int(IntLocalId(0)), ValueType::Int))
        );
        assert_eq!(context.define_int_local("y".into()), IntLocalId(2));
    }

    #[test]
    fn define_function_local_records_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = int_function_type();

        let local = context.define_int_function_local("f".into(), type_.clone());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Int {
                    local,
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_),
            ))
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn define_external_local_records_its_nominal_shape() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let shape = ExternalValueShape::new(
            ExternalTypeName::new(
                "dependency".into(),
                "dependency/resource".into(),
                "Resource".into(),
            ),
            vec![ValueShape::Int],
        );

        let local = context.define_external_local_shape("resource".into(), shape.clone());
        context.define_int_local("count".into());

        assert_eq!(
            context.lookup_external_local(&"resource".into()),
            Some(crate::plan::ExternalLocal::from_shape(local, shape)),
        );
        assert_eq!(context.lookup_external_local(&"missing".into()), None);
        assert_eq!(context.lookup_external_local(&"count".into()), None);
        assert_eq!(context.lookup_local(&"resource".into()), None);
    }

    #[test]
    fn external_parameters_preserve_scalar_list_and_function_local_sequences() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let shape = ExternalValueShape::new(
            ExternalTypeName::new(
                "dependency".into(),
                "dependency/resource".into(),
                "Resource".into(),
            ),
            vec![ValueShape::Int],
        );
        let function_shape =
            FunctionShape::new(vec![ValueShape::Int], ValueShape::External(shape.clone()));
        let function_type = ExternalFunctionType::from_shapes(
            function_shape.argument_shapes().to_vec(),
            shape.clone(),
        );

        assert_eq!(
            context
                .define_param_local_shape("resource".into(), ValueShape::External(shape.clone()),),
            ParamLocal::external_shape(ExternalLocalId(0), shape.clone()),
        );
        assert_eq!(
            context.define_param_local_shape(
                "resources".into(),
                ValueShape::List(Box::new(ValueShape::External(shape.clone()))),
            ),
            ParamLocal::list(ListLocal::external(
                ExternalListLocalId(0),
                shape.type_().clone(),
            )),
        );
        assert_eq!(
            context.define_param_local_shape(
                "factory".into(),
                ValueShape::Function(Box::new(function_shape.clone())),
            ),
            ParamLocal::external_function(ExternalFunctionLocal::new(
                ExternalFunctionLocalId(0),
                function_type.clone(),
            )),
        );

        context
            .define_existing_param(
                "existing_resource".into(),
                &ParamLocal::external_shape(ExternalLocalId(4), shape.clone()),
                ValueShape::External(shape.clone()),
            )
            .expect("an existing external scalar should retain its nominal shape");
        context
            .define_existing_param(
                "existing_resources".into(),
                &ParamLocal::list(ListLocal::external(
                    ExternalListLocalId(4),
                    shape.type_().clone(),
                )),
                ValueShape::List(Box::new(ValueShape::External(shape.clone()))),
            )
            .expect("an existing external list should retain its nominal shape");
        context
            .define_existing_param(
                "existing_factory".into(),
                &ParamLocal::external_function(ExternalFunctionLocal::new(
                    ExternalFunctionLocalId(4),
                    function_type,
                )),
                ValueShape::Function(Box::new(function_shape.clone())),
            )
            .expect("an existing external function should retain its exact shape");

        assert_eq!(
            context.define_param_local_shape(
                "next_resource".into(),
                ValueShape::External(shape.clone())
            ),
            ParamLocal::external_shape(ExternalLocalId(5), shape.clone()),
        );
        assert_eq!(
            context.define_param_local_shape(
                "next_resources".into(),
                ValueShape::List(Box::new(ValueShape::External(shape.clone()))),
            ),
            ParamLocal::list(ListLocal::external(
                ExternalListLocalId(5),
                shape.type_().clone(),
            )),
        );
        let next_function_type =
            ExternalFunctionType::from_shapes(function_shape.argument_shapes().to_vec(), shape);
        assert_eq!(
            context.define_param_local_shape(
                "next_factory".into(),
                ValueShape::Function(Box::new(function_shape)),
            ),
            ParamLocal::external_function(ExternalFunctionLocal::new(
                ExternalFunctionLocalId(5),
                next_function_type,
            )),
        );
    }

    #[test]
    fn generic_function_params_preserve_parameter_shape_and_local_sequence() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let shape = FunctionShape::new(
            vec![ValueShape::Parameter(parameter)],
            ValueShape::Parameter(parameter),
        );
        let type_ = GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter);
        let local = GenericFunctionLocal::new(GenericFunctionLocalId(0), type_.clone());

        assert_eq!(
            context.define_param_local_shape(
                "function".into(),
                ValueShape::Function(Box::new(shape.clone())),
            ),
            ParamLocal::generic_function(local.clone()),
        );
        assert_eq!(
            context.lookup_function_local(&"function".into()),
            Some((FunctionLocalBinding::Generic(local), shape.clone())),
        );

        let existing = GenericFunctionLocal::new(GenericFunctionLocalId(4), type_.clone());
        context
            .define_existing_param(
                "existing".into(),
                &ParamLocal::generic_function(existing.clone()),
                ValueShape::Function(Box::new(shape.clone())),
            )
            .unwrap();
        assert_eq!(
            context.lookup_function_local(&"existing".into()),
            Some((FunctionLocalBinding::Generic(existing), shape.clone())),
        );
        assert_eq!(
            context.define_generic_function_local_shape("next".into(), type_, shape),
            GenericFunctionLocal::new(
                GenericFunctionLocalId(5),
                GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter,),
            ),
        );
    }

    #[test]
    fn param_local_shapes_preserve_remaining_primitive_and_function_families() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context.define_param_local_shape("float".into(), ValueShape::Float),
            ParamLocal::float(FloatLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("string".into(), ValueShape::String),
            ParamLocal::string(StringLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("bool".into(), ValueShape::Bool),
            ParamLocal::bool(BoolLocalId(0)),
        );
        assert_eq!(
            context.define_param_local_shape("nil".into(), ValueShape::Nil),
            ParamLocal::nil(NilLocalId(0)),
        );

        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        assert_eq!(
            context.define_param_local_shape(
                "float_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    float_type.clone(),
                ))),
            ),
            ParamLocal::float_function(FloatFunctionLocalId(0), float_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "string_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    string_type.clone(),
                ))),
            ),
            ParamLocal::string_function(StringFunctionLocalId(0), string_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "bool_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    bool_type.clone(),
                ))),
            ),
            ParamLocal::bool_function(BoolFunctionLocalId(0), bool_type),
        );
        assert_eq!(
            context.define_param_local_shape(
                "nil_fn".into(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    nil_type.clone(),
                ))),
            ),
            ParamLocal::nil_function(NilFunctionLocalId(0), nil_type),
        );
    }

    #[test]
    fn define_existing_param_records_tuple_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]));

        context
            .define_existing_param(
                "f".into(),
                &ParamLocal::tuple_function(TupleFunctionLocalId(2), type_.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(type_.clone()))),
            )
            .unwrap();

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Tuple {
                    local: TupleFunctionLocalId(2),
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_.clone()),
            )),
        );
        assert_eq!(context.define_tuple_function_local("g".into(), type_).0, 3);
    }

    #[test]
    fn define_existing_param_records_tuple_and_function_family_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bit_array_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type());
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let function_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));

        context
            .define_existing_param(
                "tuple".into(),
                &ParamLocal::tuple(TupleLocalId(2), tuple_type.clone()),
                ValueShape::from_value_type(ValueType::Tuple(tuple_type.clone())),
            )
            .unwrap();
        context
            .define_existing_param(
                "string_fn".into(),
                &ParamLocal::string_function(StringFunctionLocalId(3), string_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(string_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "bit_array_fn".into(),
                &ParamLocal::bit_array_function(BitArrayFunctionLocalId(4), bit_array_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(bit_array_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "custom_fn".into(),
                &ParamLocal::custom_function(CustomFunctionLocal::new(
                    CustomFunctionLocalId(8),
                    custom_function_type.clone(),
                )),
                ValueShape::from_value_type(ValueType::Function(Box::new(
                    custom_function_type.to_function_type(),
                ))),
            )
            .unwrap();
        context
            .define_existing_param(
                "float_fn".into(),
                &ParamLocal::float_function(FloatFunctionLocalId(4), float_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(float_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "bool_fn".into(),
                &ParamLocal::bool_function(BoolFunctionLocalId(5), bool_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(bool_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "nil_fn".into(),
                &ParamLocal::nil_function(NilFunctionLocalId(6), nil_type.clone()),
                ValueShape::from_value_type(ValueType::Function(Box::new(nil_type.clone()))),
            )
            .unwrap();
        context
            .define_existing_param(
                "function_fn".into(),
                &ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(7),
                    function_type.clone(),
                )),
                ValueShape::from_value_type(ValueType::Function(Box::new(
                    function_type.to_function_type(),
                ))),
            )
            .unwrap();

        assert_eq!(
            context.lookup_tuple_local(&"tuple".into()),
            Some((TupleLocalId(2), vec![ValueShape::Int].into_boxed_slice(),)),
        );
        assert_eq!(
            context.lookup_function_local(&"string_fn".into()),
            Some((
                FunctionLocalBinding::String {
                    local: StringFunctionLocalId(3),
                    type_: string_type.clone(),
                },
                FunctionShape::from_function_type(string_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"bit_array_fn".into()),
            Some((
                FunctionLocalBinding::BitArray {
                    local: BitArrayFunctionLocalId(4),
                    type_: bit_array_type.clone(),
                },
                FunctionShape::from_function_type(bit_array_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"custom_fn".into()),
            Some((
                FunctionLocalBinding::Custom(CustomFunctionLocal::new(
                    CustomFunctionLocalId(8),
                    custom_function_type.clone(),
                )),
                FunctionShape::from_function_type(custom_function_type.to_function_type()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"float_fn".into()),
            Some((
                FunctionLocalBinding::Float {
                    local: FloatFunctionLocalId(4),
                    type_: float_type.clone(),
                },
                FunctionShape::from_function_type(float_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"bool_fn".into()),
            Some((
                FunctionLocalBinding::Bool {
                    local: BoolFunctionLocalId(5),
                    type_: bool_type.clone(),
                },
                FunctionShape::from_function_type(bool_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"nil_fn".into()),
            Some((
                FunctionLocalBinding::Nil {
                    local: NilFunctionLocalId(6),
                    type_: nil_type.clone(),
                },
                FunctionShape::from_function_type(nil_type.clone()),
            )),
        );
        assert_eq!(
            context.lookup_function_local(&"function_fn".into()),
            Some((
                FunctionLocalBinding::Function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(7),
                    function_type.clone(),
                )),
                FunctionShape::from_function_type(function_type.to_function_type()),
            )),
        );

        assert_eq!(
            context
                .define_tuple_local("next_tuple".into(), tuple_type)
                .0,
            3
        );
        assert_eq!(
            context
                .define_string_function_local("next_string_fn".into(), string_type)
                .0,
            4,
        );
        assert_eq!(
            context
                .define_bit_array_function_local("next_bit_array_fn".into(), bit_array_type)
                .0,
            5,
        );
        assert_eq!(
            context
                .define_custom_function_local("next_custom_fn".into(), custom_function_type)
                .id(),
            CustomFunctionLocalId(9),
        );
        assert_eq!(
            context.define_internal_custom_function_local(CustomFunctionType::new(
                Vec::new(),
                custom_type(),
            )),
            CustomFunctionLocal::new(
                CustomFunctionLocalId(10),
                CustomFunctionType::new(Vec::new(), custom_type()),
            ),
        );
        assert_eq!(
            context
                .define_float_function_local("next_float_fn".into(), float_type)
                .0,
            5,
        );
        assert_eq!(
            context
                .define_bool_function_local("next_bool_fn".into(), bool_type)
                .0,
            6,
        );
        assert_eq!(
            context
                .define_nil_function_local("next_nil_fn".into(), nil_type)
                .0,
            7,
        );
        assert_eq!(
            context
                .define_function_function_local("next_function_fn".into(), function_type)
                .id(),
            FunctionFunctionLocalId(8),
        );
        assert_eq!(
            context.define_existing_param(
                "mismatch".into(),
                &ParamLocal::int(IntLocalId(0)),
                ValueShape::String,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn define_internal_tuple_local_reserves_id_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];

        assert_eq!(context.define_internal_tuple_local(), TupleLocalId(0));
        assert_eq!(context.lookup_tuple_local(&"<tuple:0>".into()), None);
        assert_eq!(context.lookup_tuple_local(&"<case:tuple:0>".into()), None);
        assert_eq!(
            context.define_tuple_local("tuple".into(), tuple_type.clone()),
            TupleLocalId(1),
        );
        assert_eq!(
            context.lookup_tuple_local(&"tuple".into()),
            Some((TupleLocalId(1), vec![ValueShape::Int].into_boxed_slice())),
        );
    }

    #[test]
    fn define_internal_list_local_reserves_id_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context
                .define_internal_list_value(ListExpr::value(Vec::new(), ValueType::Int))
                .0,
            ListLocal::int(IntListLocalId(0))
        );
        assert_eq!(context.lookup_list_local(&"<list:int:0>".into()), None);
        assert_eq!(context.lookup_list_local(&"<case:list:int:0>".into()), None);
        assert_eq!(
            context.define_list_local("values".into(), ValueType::Int),
            ListLocal::int(IntListLocalId(1)),
        );
        assert_eq!(
            context.lookup_list_local(&"values".into()),
            Some((ListLocal::int(IntListLocalId(1)), ValueShape::Int)),
        );
    }

    #[test]
    fn define_list_capture_value_preserves_every_item_family() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let nested_item_type = Box::new(ValueType::String);
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Bool);

        assert_eq!(
            context.define_list_capture_value(
                "ints".into(),
                ListLocal::int(IntListLocalId(9)),
                ValueShape::Int,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::int(IntListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::int(IntListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "strings".into(),
                ListLocal::string(StringListLocalId(9)),
                ValueShape::String,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::string(
                    StringListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::string(StringListLocalId(9),))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "bit_arrays".into(),
                ListLocal::bit_array(BitArrayListLocalId(9)),
                ValueShape::BitArray,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::bit_array(
                    BitArrayListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::bit_array(
                    BitArrayListLocalId(9),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "utf_codepoints".into(),
                ListLocal::utf_codepoint(UtfCodepointListLocalId(9)),
                ValueShape::UtfCodepoint,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::utf_codepoint(
                    UtfCodepointListLocalId(0),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::utf_codepoint(
                    UtfCodepointListLocalId(9),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "floats".into(),
                ListLocal::float(FloatListLocalId(9)),
                ValueShape::Float,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::float(FloatListLocalId(
                    0
                ),))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::float(FloatListLocalId(9,)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "bools".into(),
                ListLocal::bool(BoolListLocalId(9)),
                ValueShape::Bool,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::bool(BoolListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::bool(BoolListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "nils".into(),
                ListLocal::nil(NilListLocalId(9)),
                ValueShape::Nil,
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::nil(NilListLocalId(0)))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::nil(NilListLocalId(9)))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "tuples".into(),
                ListLocal::tuple(TupleListLocalId(9), tuple_type.clone()),
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::tuple(
                    TupleListLocalId(0),
                    tuple_type.clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::tuple(
                    TupleListLocalId(9),
                    tuple_type.clone(),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "lists".into(),
                ListLocal::list(ListListLocalId(9), nested_item_type.as_ref().clone()),
                ValueShape::List(Box::new(ValueShape::String)),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::list(
                    ListListLocalId(0),
                    nested_item_type.as_ref().clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::list(
                    ListListLocalId(9),
                    nested_item_type.as_ref().clone(),
                ))),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "functions".into(),
                ListLocal::function(FunctionListLocalId(9), function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    function_type.clone(),
                ))),
            ),
            PlannedCapture {
                slot: ParamSlot::from_local(ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(0),
                    function_type.clone(),
                ))),
                source: CaptureArg::new(ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(9),
                    function_type.clone(),
                ))),
            },
        );

        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some((
                ListLocal::function(FunctionListLocalId(0), function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(function_type,))),
            )),
        );
    }

    #[test]
    fn define_list_locals_preserve_item_family_boundaries() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int, ValueType::String];
        let nested_function_type = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let parameter = TypeParameterId(0);

        assert_eq!(
            context.define_list_local("generic".into(), ValueType::Parameter(parameter)),
            ListLocal::generic(GenericListLocalId(0), parameter),
        );

        assert_eq!(
            context.define_list_local("strings".into(), ValueType::String),
            ListLocal::string(StringListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("bit_arrays".into(), ValueType::BitArray),
            ListLocal::bit_array(BitArrayListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("floats".into(), ValueType::Float),
            ListLocal::float(FloatListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("bools".into(), ValueType::Bool),
            ListLocal::bool(BoolListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("nils".into(), ValueType::Nil),
            ListLocal::nil(NilListLocalId(0)),
        );
        assert_eq!(
            context.define_list_local("tuples".into(), ValueType::Tuple(tuple_type.clone())),
            ListLocal::tuple(TupleListLocalId(0), tuple_type.clone()),
        );
        assert_eq!(
            context.define_list_local("lists".into(), ValueType::List(Box::new(ValueType::Float)),),
            ListLocal::list(ListListLocalId(0), ValueType::Float),
        );
        assert_eq!(
            context.define_list_local(
                "functions".into(),
                ValueType::Function(Box::new(nested_function_type.clone())),
            ),
            ListLocal::function(FunctionListLocalId(0), nested_function_type.clone()),
        );

        assert_eq!(
            context.lookup_list_local(&"tuples".into()),
            Some((
                ListLocal::tuple(TupleListLocalId(0), tuple_type),
                ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),),
            )),
        );
        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some((
                ListLocal::function(FunctionListLocalId(0), nested_function_type.clone()),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    nested_function_type,
                ))),
            )),
        );
        assert_eq!(
            context.lookup_list_local(&"generic".into()),
            Some((
                ListLocal::generic(GenericListLocalId(0), parameter),
                ValueShape::Parameter(parameter),
            )),
        );
    }

    #[test]
    fn define_generic_list_value_preserves_parameter_and_typed_local_expression() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let parameter = TypeParameterId(0);
        let value = ListExpr::try_value(Vec::new(), ValueType::Parameter(parameter))
            .expect("an empty generic list has a sealed item parameter");
        let typed_value = value
            .clone()
            .into_generic()
            .expect("a parameter item list has generic list storage");

        assert_eq!(
            context.define_list_value("values".into(), value),
            (
                ListLocal::generic(GenericListLocalId(0), parameter),
                ListLocalExpr::Generic {
                    local: GenericListLocalId(0),
                    parameter,
                    value: typed_value,
                },
            ),
        );
        assert_eq!(
            context.lookup_list_local(&"values".into()),
            Some((
                ListLocal::generic(GenericListLocalId(0), parameter),
                ValueShape::Parameter(parameter),
            )),
        );
    }

    #[test]
    fn define_existing_list_params_bump_their_own_item_family() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let tuple_type = vec![ValueType::Int];
        let nested_function_type = FunctionType::new(Vec::new(), ValueType::Int);

        context
            .define_existing_param(
                "strings".into(),
                &ParamLocal::list(ListLocal::string(StringListLocalId(2))),
                ValueShape::List(Box::new(ValueShape::String)),
            )
            .unwrap();
        context
            .define_existing_param(
                "bit_arrays".into(),
                &ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(2))),
                ValueShape::List(Box::new(ValueShape::BitArray)),
            )
            .unwrap();
        context
            .define_existing_param(
                "floats".into(),
                &ParamLocal::list(ListLocal::float(FloatListLocalId(3))),
                ValueShape::List(Box::new(ValueShape::Float)),
            )
            .unwrap();
        context
            .define_existing_param(
                "bools".into(),
                &ParamLocal::list(ListLocal::bool(BoolListLocalId(4))),
                ValueShape::List(Box::new(ValueShape::Bool)),
            )
            .unwrap();
        context
            .define_existing_param(
                "nils".into(),
                &ParamLocal::list(ListLocal::nil(NilListLocalId(5))),
                ValueShape::List(Box::new(ValueShape::Nil)),
            )
            .unwrap();
        context
            .define_existing_param(
                "tuples".into(),
                &ParamLocal::list(ListLocal::tuple(TupleListLocalId(6), tuple_type.clone())),
                ValueShape::List(Box::new(ValueShape::Tuple(
                    vec![ValueShape::Int].into_boxed_slice(),
                ))),
            )
            .unwrap();
        context
            .define_existing_param(
                "lists".into(),
                &ParamLocal::list(ListLocal::list(ListListLocalId(7), ValueType::Int)),
                ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::Int)))),
            )
            .unwrap();
        context
            .define_existing_param(
                "functions".into(),
                &ParamLocal::list(ListLocal::function(
                    FunctionListLocalId(8),
                    nested_function_type.clone(),
                )),
                ValueShape::List(Box::new(ValueShape::Function(Box::new(
                    FunctionShape::from_function_type(nested_function_type.clone()),
                )))),
            )
            .unwrap();

        assert_eq!(
            context.define_list_local("next_string".into(), ValueType::String),
            ListLocal::string(StringListLocalId(3)),
        );
        assert_eq!(
            context.define_list_local("next_bit_array".into(), ValueType::BitArray),
            ListLocal::bit_array(BitArrayListLocalId(3)),
        );
        assert_eq!(
            context.define_list_local("next_float".into(), ValueType::Float),
            ListLocal::float(FloatListLocalId(4)),
        );
        assert_eq!(
            context.define_list_local("next_bool".into(), ValueType::Bool),
            ListLocal::bool(BoolListLocalId(5)),
        );
        assert_eq!(
            context.define_list_local("next_nil".into(), ValueType::Nil),
            ListLocal::nil(NilListLocalId(6)),
        );
        assert_eq!(
            context.define_list_local("next_tuple".into(), ValueType::Tuple(tuple_type)),
            ListLocal::tuple(TupleListLocalId(7), vec![ValueType::Int]),
        );
        assert_eq!(
            context.define_list_local(
                "next_list".into(),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            ListLocal::list(ListListLocalId(8), ValueType::Int),
        );
        assert_eq!(
            context.define_list_local(
                "next_function".into(),
                ValueType::Function(Box::new(nested_function_type.clone())),
            ),
            ListLocal::function(FunctionListLocalId(9), nested_function_type),
        );
    }

    #[test]
    fn define_internal_primitive_locals_reserve_ids_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(context.define_internal_int_local(), IntLocalId(0));
        assert_eq!(context.define_internal_string_local(), StringLocalId(0));
        assert_eq!(
            context.define_internal_bit_array_local(),
            BitArrayLocalId(0),
        );
        assert_eq!(context.define_internal_float_local(), FloatLocalId(0));
        assert_eq!(context.define_internal_bool_local(), BoolLocalId(0));
        assert_eq!(context.define_internal_nil_local(), NilLocalId(0));
        assert_eq!(context.lookup_local(&"<case:int:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:string:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:bit_array:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:float:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:bool:0>".into()), None);
        assert_eq!(context.lookup_local(&"<case:nil:0>".into()), None);

        assert_eq!(context.define_int_local("int".into()), IntLocalId(1));
        assert_eq!(
            context.define_string_local("string".into()),
            StringLocalId(1),
        );
        assert_eq!(
            context.define_bit_array_local("bit_array".into()),
            BitArrayLocalId(1),
        );
        assert_eq!(context.define_float_local("float".into()), FloatLocalId(1));
        assert_eq!(context.define_bool_local("bool".into()), BoolLocalId(1));
        assert_eq!(context.define_nil_local("nil".into()), NilLocalId(1));
    }

    #[test]
    fn define_internal_function_locals_reserve_ids_without_user_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            context.define_internal_int_function_local(),
            IntFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_string_function_local(),
            StringFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_bit_array_function_local(),
            BitArrayFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_float_function_local(),
            FloatFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_bool_function_local(),
            BoolFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_nil_function_local(),
            NilFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_tuple_function_local(),
            TupleFunctionLocalId(0),
        );
        assert_eq!(
            context.define_internal_list_function_local(
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                ),
                crate::plan::ValueType::Int
            ),
            crate::plan::ListFunctionLocal::from_item_type(
                0,
                crate::plan::FunctionType::new(
                    Vec::new(),
                    crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                ),
                crate::plan::ValueType::Int,
            ),
        );
        assert_eq!(
            context.define_internal_function_function_local(FunctionFunctionType::new(
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            )),
            FunctionFunctionLocal::new(
                FunctionFunctionLocalId(0),
                FunctionFunctionType::new(
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                ),
            ),
        );
        assert_eq!(
            context.lookup_function_local(&"<case:int_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:string_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:bit_array_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:float_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:bool_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:nil_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:tuple_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:list_function:0>".into()),
            None,
        );
        assert_eq!(
            context.lookup_function_local(&"<case:function_function:0>".into()),
            None,
        );

        assert_eq!(
            context.define_int_function_local("f".into(), type_),
            IntFunctionLocalId(1),
        );
    }

    #[test]
    fn function_local_shadows_primitive_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = int_function_type();

        context.define_int_local("f".into());
        context.define_int_function_local("f".into(), type_.clone());

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some((
                FunctionLocalBinding::Int {
                    local: IntFunctionLocalId(0),
                    type_: type_.clone(),
                },
                FunctionShape::from_function_type(type_),
            ))
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn primitive_binding_shadows_function_local() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        context.define_int_function_local("f".into(), int_function_type());
        let local = context.define_int_local("f".into());

        assert_eq!(context.lookup_function_local(&"f".into()), None);
        assert_eq!(
            context.lookup_local(&"f".into()),
            Some((LocalId::Int(local), ValueType::Int))
        );
    }

    #[test]
    fn define_captures_records_float_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Float], ValueType::Float);

        context.define_float_function_local("f".into(), type_.clone());
        let captures = context.capture_bindings(&[EcoString::from("f")]).unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![ParamSlot::from_local(ParamLocal::float_function(
                FloatFunctionLocalId(1),
                type_.clone(),
            ))],
        );
        assert_eq!(
            sources,
            vec![CaptureArg::new(ParamLocal::float_function(
                FloatFunctionLocalId(0),
                type_,
            ))],
        );
    }

    #[test]
    fn define_captures_preserves_bit_array_and_nil_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        context.define_bit_array_local("bits".into());
        context.define_nil_local("nil".into());
        let captures = context
            .capture_bindings(&["bits".into(), "nil".into()])
            .expect("defined locals should be capturable");
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::bit_array(BitArrayLocalId(1))),
                ParamSlot::from_local(ParamLocal::nil(NilLocalId(1))),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::bit_array(BitArrayLocalId(0))),
                CaptureArg::new(ParamLocal::nil(NilLocalId(0))),
            ],
        );
    }

    #[test]
    fn define_captures_records_tuple_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]));

        context.define_tuple_function_local("f".into(), type_.clone());
        let captures = context.capture_bindings(&[EcoString::from("f")]).unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![ParamSlot::from_local(ParamLocal::tuple_function(
                TupleFunctionLocalId(1),
                type_.clone(),
            ))],
        );
        assert_eq!(
            sources,
            vec![CaptureArg::new(ParamLocal::tuple_function(
                TupleFunctionLocalId(0),
                type_,
            ))],
        );
    }

    #[test]
    fn define_captures_records_list_bindings() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let element_type = ValueType::Int;
        let function_type = FunctionType::new(
            vec![ValueType::List(Box::new(element_type.clone()))],
            ValueType::List(Box::new(element_type.clone())),
        );

        context.define_list_local("values".into(), element_type.clone());
        context.define_list_function_local("f".into(), function_type.clone(), element_type);
        let captures = context
            .capture_bindings(&[EcoString::from("values"), EcoString::from("f")])
            .unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::list(ListLocal::int(IntListLocalId(1)))),
                ParamSlot::from_local(ParamLocal::list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        1,
                        function_type.clone(),
                        ValueType::Int,
                    ),
                )),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::list(ListLocal::int(IntListLocalId(0)))),
                CaptureArg::new(ParamLocal::list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        function_type,
                        ValueType::Int,
                    ),
                )),
            ],
        );
    }

    #[test]
    fn define_captures_records_remaining_function_families() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bit_array_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let function_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));

        context.define_string_function_local("string_fn".into(), string_type.clone());
        context.define_bit_array_function_local("bit_array_fn".into(), bit_array_type.clone());
        context.define_bool_function_local("bool_fn".into(), bool_type.clone());
        context.define_nil_function_local("nil_fn".into(), nil_type.clone());
        context.define_function_function_local("function_fn".into(), function_type.clone());
        let captures = context
            .capture_bindings(&[
                "string_fn".into(),
                "bit_array_fn".into(),
                "bool_fn".into(),
                "nil_fn".into(),
                "function_fn".into(),
            ])
            .unwrap();
        let (slots, sources) = context.define_captures(captures).into_parts();

        assert_eq!(
            slots,
            vec![
                ParamSlot::from_local(ParamLocal::string_function(
                    StringFunctionLocalId(1),
                    string_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::bit_array_function(
                    BitArrayFunctionLocalId(1),
                    bit_array_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::bool_function(
                    BoolFunctionLocalId(1),
                    bool_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::nil_function(
                    NilFunctionLocalId(1),
                    nil_type.clone(),
                )),
                ParamSlot::from_local(ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(1),
                    function_type.clone()
                ),)),
            ],
        );
        assert_eq!(
            sources,
            vec![
                CaptureArg::new(ParamLocal::string_function(
                    StringFunctionLocalId(0),
                    string_type,
                )),
                CaptureArg::new(ParamLocal::bit_array_function(
                    BitArrayFunctionLocalId(0),
                    bit_array_type,
                )),
                CaptureArg::new(ParamLocal::bool_function(BoolFunctionLocalId(0), bool_type,)),
                CaptureArg::new(ParamLocal::nil_function(NilFunctionLocalId(0), nil_type)),
                CaptureArg::new(ParamLocal::function_function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    function_type,
                ))),
            ],
        );
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Int)
    }
}
