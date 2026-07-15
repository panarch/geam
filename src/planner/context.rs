use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
    BitArrayFunctionLocalId, BitArrayListFunctionId, BitArrayListItem, BitArrayListLocalId,
    BitArrayLocalId, BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId,
    BoolFunctionLocalId, BoolListFunctionId, BoolListItem, BoolListLocalId, BoolLocalId,
    CaptureArg, CustomConstructor, CustomConstructorField, CustomExpr, CustomFieldAccess,
    CustomFunctionExpr, CustomFunctionFunctionId, CustomFunctionId, CustomFunctionLocalId,
    CustomListFunctionId, CustomListItem, CustomListLocalId, CustomLocalId, CustomTypeDefinition,
    CustomTypeTemplate, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatFunctionLocalId, FloatListFunctionId, FloatListItem, FloatListLocalId, FloatLocalId,
    FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId,
    FunctionId, FunctionListFunctionId, FunctionListItem, FunctionListLocalId, FunctionPlan,
    FunctionReference, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
    IntFunctionId, IntFunctionLocalId, IntListFunctionId, IntListItem, IntListLocalId, IntLocalId,
    ListExpr, ListFunctionExpr, ListFunctionFunctionId, ListFunctionId, ListFunctionLocal,
    ListListFunctionId, ListListItem, ListListLocalId, ListLocal, ListLocalExpr, LocalId, NilExpr,
    NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilListFunctionId,
    NilListItem, NilListLocalId, NilLocalId, PanicSite, ParamBinding, ParamLocal,
    RuntimeFunctionId, StringExpr, StringFunctionExpr, StringFunctionFunctionId, StringFunctionId,
    StringFunctionLocalId, StringListFunctionId, StringListItem, StringListLocalId, StringLocalId,
    TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId, TupleFunctionLocalId,
    TupleListFunctionId, TupleListItem, TupleListLocalId, TupleLocalId, UtfCodepointExpr,
    UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointFunctionLocalId, UtfCodepointListFunctionId, UtfCodepointListItem,
    UtfCodepointListLocalId, UtfCodepointLocalId, ValueType,
};
use crate::plan::{CustomType, CustomTypeName};
use crate::planner::error::{
    InvalidCustomTypeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use ecow::EcoString;
use gleam_core::type_::{
    PRELUDE_MODULE_NAME, PatternConstructor, Type, TypeVar, ValueConstructor,
    ValueConstructorVariant,
};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

#[derive(Clone)]
pub(super) struct FunctionInfo {
    pub(super) id: FunctionId,
    pub(super) runtime_id: RuntimeFunctionId,
    pub(super) return_type: ValueType,
    pub(super) params: Vec<FunctionParam>,
}

#[derive(Clone)]
pub(super) struct FunctionParam {
    pub(super) local: ParamLocal,
    pub(super) binding: ParamBinding,
    pub(super) label: Option<EcoString>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ResolvedCustomConstructor {
    constructor: CustomConstructor,
    constructor_count: usize,
}

impl ResolvedCustomConstructor {
    pub(super) fn constructor_count(&self) -> usize {
        self.constructor_count
    }

    pub(super) fn into_constructor(self) -> CustomConstructor {
        self.constructor
    }
}

pub(super) struct PlanContext<'a> {
    pub(super) module_name: &'a EcoString,
    current_function: EcoString,
    functions: &'a HashMap<EcoString, FunctionInfo>,
    custom_types: &'a [CustomTypeDefinition],
    anonymous_functions: &'a mut AnonymousFunctions,
    bindings: HashMap<EcoString, LocalBinding>,
    next_int_local: usize,
    next_float_local: usize,
    next_string_local: usize,
    next_bit_array_local: usize,
    next_utf_codepoint_local: usize,
    next_custom_local: usize,
    next_bool_local: usize,
    next_nil_local: usize,
    next_tuple_local: usize,
    next_int_list_local: usize,
    next_string_list_local: usize,
    next_bit_array_list_local: usize,
    next_utf_codepoint_list_local: usize,
    next_custom_list_local: usize,
    next_float_list_local: usize,
    next_bool_list_local: usize,
    next_nil_list_local: usize,
    next_tuple_list_local: usize,
    next_list_list_local: usize,
    next_function_list_local: usize,
    next_int_function_local: usize,
    next_float_function_local: usize,
    next_string_function_local: usize,
    next_bit_array_function_local: usize,
    next_utf_codepoint_function_local: usize,
    next_custom_function_local: usize,
    next_bool_function_local: usize,
    next_nil_function_local: usize,
    next_tuple_function_local: usize,
    next_list_function_local: usize,
    next_function_function_local: usize,
}

#[derive(Clone)]
enum LocalBinding {
    Primitive(LocalId),
    Custom {
        local: CustomLocalId,
        type_: CustomType,
    },
    Tuple {
        local: TupleLocalId,
        type_: Vec<ValueType>,
    },
    List(ListLocal),
    Function(FunctionLocalBinding),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CustomFunctionShape {
    name: CustomTypeName,
    arguments: Vec<bool>,
}

pub(super) struct CaptureBinding {
    name: EcoString,
    binding: LocalBinding,
}

fn instantiate_custom_type_template(
    template: &CustomTypeTemplate,
    custom_type: &CustomType,
) -> Result<ValueType, PlanError> {
    let type_ = match template {
        CustomTypeTemplate::Int => ValueType::Int,
        CustomTypeTemplate::Float => ValueType::Float,
        CustomTypeTemplate::String => ValueType::String,
        CustomTypeTemplate::BitArray => ValueType::BitArray,
        CustomTypeTemplate::UtfCodepoint => ValueType::UtfCodepoint,
        CustomTypeTemplate::Bool => ValueType::Bool,
        CustomTypeTemplate::Nil => ValueType::Nil,
        CustomTypeTemplate::Tuple(elements) => ValueType::Tuple(
            elements
                .iter()
                .map(|element| instantiate_custom_type_template(element, custom_type))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        CustomTypeTemplate::List(element) => ValueType::List(Box::new(
            instantiate_custom_type_template(element, custom_type)?,
        )),
        CustomTypeTemplate::Function { arguments, return_ } => {
            ValueType::Function(Box::new(FunctionType::new(
                arguments
                    .iter()
                    .map(|argument| instantiate_custom_type_template(argument, custom_type))
                    .collect::<Result<Vec<_>, _>>()?,
                instantiate_custom_type_template(return_, custom_type)?,
            )))
        }
        CustomTypeTemplate::Custom { name, arguments } => ValueType::Custom(CustomType::new(
            name.clone(),
            arguments
                .iter()
                .map(|argument| instantiate_custom_type_template(argument, custom_type))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        CustomTypeTemplate::Parameter(parameter) => {
            let Some(type_) = custom_type.arguments().get(parameter.0).cloned() else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: custom_type.type_name().name().clone(),
                        reason: InvalidCustomTypeReason::ParameterType,
                    },
                });
            };
            type_
        }
    };
    Ok(type_)
}

fn invalid_custom_type(type_: &CustomType, reason: InvalidCustomTypeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CustomType {
            name: type_.type_name().name().clone(),
            reason,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FunctionLocalBinding {
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
    Custom {
        local: CustomFunctionLocalId,
        type_: FunctionType,
    },
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
    Function {
        local: FunctionFunctionLocalId,
        type_: FunctionType,
    },
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

    pub(super) fn new_with_custom_types(
        module_name: &'a EcoString,
        functions: &'a HashMap<EcoString, FunctionInfo>,
        custom_types: &'a [CustomTypeDefinition],
        anonymous_functions: &'a mut AnonymousFunctions,
    ) -> Self {
        Self {
            module_name,
            current_function: "main".into(),
            functions,
            custom_types,
            anonymous_functions,
            bindings: HashMap::new(),
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
        }
    }

    pub(super) fn set_current_function(&mut self, name: EcoString) {
        self.current_function = name;
    }

    pub(super) fn panic_site(&self, location: gleam_core::ast::SrcSpan) -> PanicSite {
        PanicSite::new(
            self.module_name.clone(),
            self.current_function.clone(),
            location.into(),
        )
    }

    pub(super) fn define_existing_local(&mut self, name: EcoString, local: LocalId) {
        match local {
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

    pub(super) fn define_param_local(&mut self, name: EcoString, type_: ValueType) -> ParamLocal {
        match type_ {
            ValueType::Int => ParamLocal::int(self.define_int_local(name)),
            ValueType::Float => ParamLocal::float(self.define_float_local(name)),
            ValueType::String => ParamLocal::string(self.define_string_local(name)),
            ValueType::BitArray => ParamLocal::bit_array(self.define_bit_array_local(name)),
            ValueType::UtfCodepoint => {
                ParamLocal::utf_codepoint(self.define_utf_codepoint_local(name))
            }
            ValueType::Custom(type_) => {
                ParamLocal::custom(self.define_custom_local(name, type_.clone()), type_)
            }
            ValueType::Bool => ParamLocal::bool(self.define_bool_local(name)),
            ValueType::Nil => ParamLocal::nil(self.define_nil_local(name)),
            ValueType::Tuple(type_) => {
                ParamLocal::tuple(self.define_tuple_local(name, type_.clone()), type_)
            }
            ValueType::List(element_type) => {
                ParamLocal::list(self.define_list_local(name, *element_type))
            }
            ValueType::Function(type_) => {
                let type_ = *type_;
                match type_.return_() {
                    ValueType::Int => ParamLocal::int_function(
                        self.define_int_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::Float => ParamLocal::float_function(
                        self.define_float_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::String => ParamLocal::string_function(
                        self.define_string_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::BitArray => ParamLocal::bit_array_function(
                        self.define_bit_array_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::UtfCodepoint => ParamLocal::utf_codepoint_function(
                        self.define_utf_codepoint_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::Custom(_) => ParamLocal::custom_function(
                        self.define_custom_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::Bool => ParamLocal::bool_function(
                        self.define_bool_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::Nil => ParamLocal::nil_function(
                        self.define_nil_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::Tuple(_) => ParamLocal::tuple_function(
                        self.define_tuple_function_local(name, type_.clone()),
                        type_,
                    ),
                    ValueType::List(item_type) => {
                        ParamLocal::list_function(self.define_list_function_local(
                            name,
                            type_.clone(),
                            item_type.as_ref().clone(),
                        ))
                    }
                    ValueType::Function(_) => ParamLocal::function_function(
                        self.define_function_function_local(name, type_.clone()),
                        type_,
                    ),
                }
            }
        }
    }

    pub(super) fn define_existing_param(&mut self, name: EcoString, local: &ParamLocal) {
        match local {
            ParamLocal::Int(local) => {
                self.define_existing_local(name, LocalId::Int(*local));
            }
            ParamLocal::Float(local) => {
                self.define_existing_local(name, LocalId::Float(*local));
            }
            ParamLocal::String(local) => {
                self.define_existing_local(name, LocalId::String(*local));
            }
            ParamLocal::BitArray(local) => {
                self.define_existing_local(name, LocalId::BitArray(*local));
            }
            ParamLocal::UtfCodepoint(local) => {
                self.define_existing_local(name, LocalId::UtfCodepoint(*local));
            }
            ParamLocal::Custom { local, type_ } => {
                self.next_custom_local = self.next_custom_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Custom {
                        local: *local,
                        type_: type_.clone(),
                    },
                );
            }
            ParamLocal::Bool(local) => {
                self.define_existing_local(name, LocalId::Bool(*local));
            }
            ParamLocal::Nil(local) => {
                self.define_existing_local(name, LocalId::Nil(*local));
            }
            ParamLocal::Tuple { local, type_ } => {
                self.next_tuple_local = self.next_tuple_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Tuple {
                        local: *local,
                        type_: type_.clone(),
                    },
                );
            }
            ParamLocal::List(local) => {
                self.bump_list_local(local);
                self.bindings
                    .insert(name, LocalBinding::List(local.clone()));
            }
            ParamLocal::IntFunction { local, type_ } => {
                self.next_int_function_local = self.next_int_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Int {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::FloatFunction { local, type_ } => {
                self.next_float_function_local = self.next_float_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Float {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::StringFunction { local, type_ } => {
                self.next_string_function_local = self.next_string_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::String {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::BitArrayFunction { local, type_ } => {
                self.next_bit_array_function_local =
                    self.next_bit_array_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::BitArray {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::UtfCodepointFunction { local, type_ } => {
                self.next_utf_codepoint_function_local =
                    self.next_utf_codepoint_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::UtfCodepoint {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::CustomFunction { local, type_ } => {
                self.next_custom_function_local = self.next_custom_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Custom {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::BoolFunction { local, type_ } => {
                self.next_bool_function_local = self.next_bool_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Bool {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::NilFunction { local, type_ } => {
                self.next_nil_function_local = self.next_nil_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Nil {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::TupleFunction { local, type_ } => {
                self.next_tuple_function_local = self.next_tuple_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Tuple {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
            ParamLocal::ListFunction(local) => {
                self.next_list_function_local =
                    self.next_list_function_local.max(local.index() + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::List(local.clone())),
                );
            }
            ParamLocal::FunctionFunction { local, type_ } => {
                self.next_function_function_local =
                    self.next_function_function_local.max(local.0 + 1);
                self.bindings.insert(
                    name,
                    LocalBinding::Function(FunctionLocalBinding::Function {
                        local: *local,
                        type_: type_.clone(),
                    }),
                );
            }
        }
    }

    pub(super) fn define_int_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Int { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_int_function_local(&mut self) -> IntFunctionLocalId {
        let local = IntFunctionLocalId(self.next_int_function_local);
        self.next_int_function_local += 1;
        local
    }

    pub(super) fn define_string_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::String { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_string_function_local(&mut self) -> StringFunctionLocalId {
        let local = StringFunctionLocalId(self.next_string_function_local);
        self.next_string_function_local += 1;
        local
    }

    pub(super) fn define_bit_array_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::BitArray { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_bit_array_function_local(&mut self) -> BitArrayFunctionLocalId {
        let local = BitArrayFunctionLocalId(self.next_bit_array_function_local);
        self.next_bit_array_function_local += 1;
        local
    }

    pub(super) fn define_utf_codepoint_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> UtfCodepointFunctionLocalId {
        let local = UtfCodepointFunctionLocalId(self.next_utf_codepoint_function_local);
        self.next_utf_codepoint_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::UtfCodepoint { local, type_ }),
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

    pub(super) fn define_custom_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> CustomFunctionLocalId {
        let local = CustomFunctionLocalId(self.next_custom_function_local);
        self.next_custom_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Custom { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_custom_function_local(&mut self) -> CustomFunctionLocalId {
        let local = CustomFunctionLocalId(self.next_custom_function_local);
        self.next_custom_function_local += 1;
        local
    }

    pub(super) fn define_float_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Float { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_float_function_local(&mut self) -> FloatFunctionLocalId {
        let local = FloatFunctionLocalId(self.next_float_function_local);
        self.next_float_function_local += 1;
        local
    }

    pub(super) fn define_bool_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Bool { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_bool_function_local(&mut self) -> BoolFunctionLocalId {
        let local = BoolFunctionLocalId(self.next_bool_function_local);
        self.next_bool_function_local += 1;
        local
    }

    pub(super) fn define_nil_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Nil { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_nil_function_local(&mut self) -> NilFunctionLocalId {
        let local = NilFunctionLocalId(self.next_nil_function_local);
        self.next_nil_function_local += 1;
        local
    }

    pub(super) fn define_tuple_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Tuple { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_tuple_function_local(&mut self) -> TupleFunctionLocalId {
        let local = TupleFunctionLocalId(self.next_tuple_function_local);
        self.next_tuple_function_local += 1;
        local
    }

    pub(super) fn define_list_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionLocal {
        let local =
            ListFunctionLocal::from_item_type(self.next_list_function_local, type_, item_type);
        self.next_list_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::List(local.clone())),
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

    pub(super) fn define_function_function_local(
        &mut self,
        name: EcoString,
        type_: FunctionType,
    ) -> FunctionFunctionLocalId {
        let local = FunctionFunctionLocalId(self.next_function_function_local);
        self.next_function_function_local += 1;
        self.bindings.insert(
            name,
            LocalBinding::Function(FunctionLocalBinding::Function { local, type_ }),
        );
        local
    }

    pub(super) fn define_internal_function_function_local(&mut self) -> FunctionFunctionLocalId {
        let local = FunctionFunctionLocalId(self.next_function_function_local);
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

    pub(super) fn define_custom_local(
        &mut self,
        name: EcoString,
        type_: CustomType,
    ) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
        self.bindings
            .insert(name, LocalBinding::Custom { local, type_ });
        local
    }

    pub(super) fn define_internal_custom_local(&mut self) -> CustomLocalId {
        let local = CustomLocalId(self.next_custom_local);
        self.next_custom_local += 1;
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

    pub(super) fn define_tuple_local(
        &mut self,
        name: EcoString,
        type_: Vec<ValueType>,
    ) -> TupleLocalId {
        let local = TupleLocalId(self.next_tuple_local);
        self.next_tuple_local += 1;
        self.bindings
            .insert(name, LocalBinding::Tuple { local, type_ });
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
        let local = self.next_list_local(element_type);
        self.bindings
            .insert(name, LocalBinding::List(local.clone()));
        local
    }

    pub(super) fn define_list_value(
        &mut self,
        name: EcoString,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        let (local, value) = self.next_list_local_expr(value);
        self.bindings
            .insert(name, LocalBinding::List(local.clone()));
        (local, value)
    }

    pub(super) fn define_internal_list_value(
        &mut self,
        value: ListExpr,
    ) -> (ListLocal, ListLocalExpr) {
        self.next_list_local_expr(value)
    }

    fn define_list_capture_value(&mut self, name: EcoString, source: ListLocal) -> ListLocalExpr {
        match source {
            ListLocal::Int(source) => {
                let local = IntListLocalId(self.next_int_list_local);
                self.next_int_list_local += 1;
                self.bindings
                    .insert(name.clone(), LocalBinding::List(ListLocal::int(local)));
                ListLocalExpr::Int {
                    local,
                    value: crate::plan::IntListExpr::local_get(IntListItem, source, name),
                }
            }
            ListLocal::String(source) => {
                let local = StringListLocalId(self.next_string_list_local);
                self.next_string_list_local += 1;
                self.bindings
                    .insert(name.clone(), LocalBinding::List(ListLocal::string(local)));
                ListLocalExpr::String {
                    local,
                    value: crate::plan::StringListExpr::local_get(StringListItem, source, name),
                }
            }
            ListLocal::BitArray(source) => {
                let local = BitArrayListLocalId(self.next_bit_array_list_local);
                self.next_bit_array_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::bit_array(local)),
                );
                ListLocalExpr::BitArray {
                    local,
                    value: crate::plan::BitArrayListExpr::local_get(BitArrayListItem, source, name),
                }
            }
            ListLocal::UtfCodepoint(source) => {
                let local = UtfCodepointListLocalId(self.next_utf_codepoint_list_local);
                self.next_utf_codepoint_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::utf_codepoint(local)),
                );
                ListLocalExpr::UtfCodepoint {
                    local,
                    value: crate::plan::UtfCodepointListExpr::local_get(
                        UtfCodepointListItem,
                        source,
                        name,
                    ),
                }
            }
            ListLocal::Custom {
                local: source,
                item_type,
            } => {
                let local = CustomListLocalId(self.next_custom_list_local);
                self.next_custom_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::custom(local, item_type.clone())),
                );
                let item = CustomListItem::new(item_type);
                ListLocalExpr::Custom {
                    local,
                    item_type: item.item_type(),
                    value: crate::plan::CustomListExpr::local_get(item, source, name),
                }
            }
            ListLocal::Float(source) => {
                let local = FloatListLocalId(self.next_float_list_local);
                self.next_float_list_local += 1;
                self.bindings
                    .insert(name.clone(), LocalBinding::List(ListLocal::float(local)));
                ListLocalExpr::Float {
                    local,
                    value: crate::plan::FloatListExpr::local_get(FloatListItem, source, name),
                }
            }
            ListLocal::Bool(source) => {
                let local = BoolListLocalId(self.next_bool_list_local);
                self.next_bool_list_local += 1;
                self.bindings
                    .insert(name.clone(), LocalBinding::List(ListLocal::bool(local)));
                ListLocalExpr::Bool {
                    local,
                    value: crate::plan::BoolListExpr::local_get(BoolListItem, source, name),
                }
            }
            ListLocal::Nil(source) => {
                let local = NilListLocalId(self.next_nil_list_local);
                self.next_nil_list_local += 1;
                self.bindings
                    .insert(name.clone(), LocalBinding::List(ListLocal::nil(local)));
                ListLocalExpr::Nil {
                    local,
                    value: crate::plan::NilListExpr::local_get(NilListItem, source, name),
                }
            }
            ListLocal::Tuple {
                local: source,
                item_type,
            } => {
                let local = TupleListLocalId(self.next_tuple_list_local);
                self.next_tuple_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::tuple(local, item_type.clone())),
                );
                let item = TupleListItem::new(item_type);
                ListLocalExpr::Tuple {
                    local,
                    item_type: item.item_type(),
                    value: crate::plan::TupleListExpr::local_get(item, source, name),
                }
            }
            ListLocal::List {
                local: source,
                item_type,
            } => {
                let local = ListListLocalId(self.next_list_list_local);
                self.next_list_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::list(local, item_type.as_ref().clone())),
                );
                let item = ListListItem::new(item_type);
                ListLocalExpr::List {
                    local,
                    item_type: item.item_type(),
                    value: crate::plan::ListListExpr::local_get(item, source, name),
                }
            }
            ListLocal::Function {
                local: source,
                item_type,
            } => {
                let local = FunctionListLocalId(self.next_function_list_local);
                self.next_function_list_local += 1;
                self.bindings.insert(
                    name.clone(),
                    LocalBinding::List(ListLocal::function(local, item_type.clone())),
                );
                let item = FunctionListItem::new(item_type);
                ListLocalExpr::Function {
                    local,
                    item_type: item.item_type(),
                    value: crate::plan::FunctionListExpr::local_get(item, source, name),
                }
            }
        }
    }

    fn next_list_local(&mut self, element_type: ValueType) -> ListLocal {
        match element_type {
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
            LocalBinding::Custom { .. } | LocalBinding::Tuple { .. } | LocalBinding::List(_) => {
                None
            }
            LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_custom_local(
        &self,
        name: &EcoString,
    ) -> Option<(CustomLocalId, CustomType)> {
        match self.bindings.get(name)? {
            LocalBinding::Custom { local, type_ } => Some((*local, type_.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Tuple { .. }
            | LocalBinding::List(_)
            | LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_tuple_local(
        &self,
        name: &EcoString,
    ) -> Option<(TupleLocalId, Vec<ValueType>)> {
        match self.bindings.get(name)? {
            LocalBinding::Tuple { local, type_ } => Some((*local, type_.clone())),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom { .. }
            | LocalBinding::List(_)
            | LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_list_local(&self, name: &EcoString) -> Option<ListLocal> {
        match self.bindings.get(name)? {
            LocalBinding::List(local) => Some(local.clone()),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom { .. }
            | LocalBinding::Tuple { .. }
            | LocalBinding::Function(_) => None,
        }
    }

    pub(super) fn lookup_function(&self, name: &EcoString) -> Option<FunctionInfo> {
        self.functions.get(name).cloned()
    }

    pub(super) fn custom_constructor(
        &self,
        constructor: &ValueConstructor,
    ) -> Result<CustomConstructor, PlanError> {
        let ValueConstructorVariant::Record {
            name,
            module,
            variant_index,
            ..
        } = &constructor.variant
        else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::error::InvalidExpressionShapeKind::RecordConstructor,
                },
            });
        };

        let (field_types, return_type) = match constructor.type_.fn_types() {
            Some(signature) => signature,
            None => (Vec::new(), constructor.type_.clone()),
        };
        let type_ = match ValueType::from_gleam(return_type.as_ref()) {
            Some(ValueType::Custom(type_)) => type_,
            None => {
                return Err(PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::GenericFunction,
                });
            }
            Some(_) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: name.clone(),
                        reason: crate::planner::error::InvalidCustomTypeReason::ConstructorType,
                    },
                });
            }
        };
        let Some(field_types) = field_types
            .into_iter()
            .map(|field| ValueType::from_gleam(field.as_ref()))
            .collect::<Option<Vec<_>>>()
        else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: name.clone(),
                    reason: crate::planner::error::InvalidCustomTypeReason::FieldType,
                },
            });
        };

        self.custom_constructor_from_parts(
            type_,
            name.clone(),
            module,
            usize::from(*variant_index),
            field_types,
        )
        .map(ResolvedCustomConstructor::into_constructor)
    }

    pub(super) fn custom_pattern_constructor(
        &self,
        type_: &Type,
        constructor: &PatternConstructor,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        let type_ = match ValueType::from_gleam(type_) {
            Some(ValueType::Custom(type_)) => type_,
            None => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: constructor.name.clone(),
                        reason: InvalidCustomTypeReason::ConstructorType,
                    },
                });
            }
            Some(_) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: constructor.name.clone(),
                        reason: InvalidCustomTypeReason::ConstructorType,
                    },
                });
            }
        };
        self.custom_constructor_from_parts(
            type_,
            constructor.name.clone(),
            &constructor.module,
            usize::from(constructor.constructor_index),
            field_types,
        )
    }

    pub(super) fn custom_field_access(
        &self,
        source: CustomExpr,
        source_type: &Type,
        index: usize,
        label: Option<EcoString>,
        expected: &ValueType,
    ) -> Result<CustomFieldAccess, PlanError> {
        let custom_type = source.type_();
        let Some(type_definition) = self
            .custom_types
            .iter()
            .find(|definition| definition.name() == custom_type.type_name())
        else {
            return Err(invalid_custom_type(
                custom_type,
                InvalidCustomTypeReason::UnknownDefinition,
            ));
        };
        if type_definition.parameters().len() != custom_type.arguments().len() {
            return Err(invalid_custom_type(
                custom_type,
                InvalidCustomTypeReason::TypeArgumentCount,
            ));
        }

        let inferred_variant = source_type.custom_type_inferred_variant().map(usize::from);
        let constructors = match inferred_variant {
            Some(index) => vec![type_definition.constructor(index).ok_or_else(|| {
                invalid_custom_type(custom_type, InvalidCustomTypeReason::ConstructorIndex)
            })?],
            None => type_definition.constructors().iter().collect(),
        };
        let mut allowed = Vec::with_capacity(constructors.len());
        for constructor in constructors {
            let field = constructor.fields().get(index).ok_or_else(|| {
                invalid_custom_type(custom_type, InvalidCustomTypeReason::FieldIndex)
            })?;
            if field.label() != label.as_ref() {
                return Err(invalid_custom_type(
                    custom_type,
                    InvalidCustomTypeReason::FieldLabel,
                ));
            }
            let actual = instantiate_custom_type_template(field.type_(), custom_type)?;
            if &actual != expected {
                return Err(invalid_custom_type(
                    custom_type,
                    InvalidCustomTypeReason::FieldType,
                ));
            }
            let fields = constructor
                .fields()
                .iter()
                .map(|field| {
                    Ok(CustomConstructorField::new(
                        field.label().cloned(),
                        instantiate_custom_type_template(field.type_(), custom_type)?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            allowed.push(CustomConstructor::new(
                custom_type.clone(),
                constructor.name().clone(),
                constructor.index(),
                fields,
            ));
        }

        Ok(CustomFieldAccess::new(source, index, label, allowed))
    }

    fn custom_constructor_from_parts(
        &self,
        type_: CustomType,
        name: EcoString,
        module: &EcoString,
        variant_index: usize,
        field_types: Vec<ValueType>,
    ) -> Result<ResolvedCustomConstructor, PlanError> {
        if module != type_.type_name().module() {
            return Err(invalid_custom_type(
                &type_,
                InvalidCustomTypeReason::ConstructorModule,
            ));
        }
        let (fields, constructor_count) = if type_.type_name().package().is_empty()
            && module == PRELUDE_MODULE_NAME
            && type_.type_name().module() == PRELUDE_MODULE_NAME
            && type_.type_name().name() == "Result"
        {
            let [ok, error] = type_.arguments() else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::TypeArgumentCount,
                ));
            };
            match variant_index {
                0 if name == "Ok" => (vec![(None, ok.clone())], 2),
                1 if name == "Error" => (vec![(None, error.clone())], 2),
                0 | 1 => {
                    return Err(invalid_custom_type(
                        &type_,
                        InvalidCustomTypeReason::ConstructorName,
                    ));
                }
                _ => {
                    return Err(invalid_custom_type(
                        &type_,
                        InvalidCustomTypeReason::ConstructorIndex,
                    ));
                }
            }
        } else {
            let Some(type_definition) = self
                .custom_types
                .iter()
                .find(|definition| definition.name() == type_.type_name())
            else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::UnknownDefinition,
                ));
            };
            if type_definition.parameters().len() != type_.arguments().len() {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::TypeArgumentCount,
                ));
            }
            let Some(constructor_definition) = type_definition.constructor(variant_index) else {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorIndex,
                ));
            };
            if constructor_definition.name() != &name {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorName,
                ));
            };
            if constructor_definition.fields().len() != field_types.len() {
                return Err(invalid_custom_type(
                    &type_,
                    InvalidCustomTypeReason::ConstructorArity,
                ));
            }
            let mut fields = Vec::with_capacity(constructor_definition.fields().len());
            for field in constructor_definition.fields() {
                fields.push((
                    field.label().cloned(),
                    instantiate_custom_type_template(field.type_(), &type_)?,
                ));
            }
            (fields, type_definition.constructors().len())
        };
        if fields.len() != field_types.len()
            || fields.iter().map(|(_, type_)| type_).ne(field_types.iter())
        {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: type_.type_name().name().clone(),
                    reason: InvalidCustomTypeReason::FieldType,
                },
            });
        }
        let fields = fields
            .into_iter()
            .map(|(label, type_)| CustomConstructorField::new(label, type_))
            .collect();

        Ok(ResolvedCustomConstructor {
            constructor: CustomConstructor::new(type_, name, variant_index, fields),
            constructor_count,
        })
    }

    pub(super) fn contains_function_value(&self, type_: &ValueType) -> Result<bool, PlanError> {
        self.contains_function_value_with_visited(type_, &mut HashSet::new())
    }

    fn contains_function_value_with_visited(
        &self,
        type_: &ValueType,
        visited: &mut HashSet<CustomFunctionShape>,
    ) -> Result<bool, PlanError> {
        match type_ {
            ValueType::Function(_) => Ok(true),
            ValueType::Tuple(elements) => {
                for element in elements {
                    if self.contains_function_value_with_visited(element, visited)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            ValueType::List(element) => self.contains_function_value_with_visited(element, visited),
            ValueType::Custom(type_) => self.custom_type_contains_function(type_, visited),
            ValueType::Int
            | ValueType::Float
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Bool
            | ValueType::Nil => Ok(false),
        }
    }

    fn custom_type_contains_function(
        &self,
        type_: &CustomType,
        visited: &mut HashSet<CustomFunctionShape>,
    ) -> Result<bool, PlanError> {
        let arguments = type_
            .arguments()
            .iter()
            .map(|argument| self.contains_function_value_with_visited(argument, visited))
            .collect::<Result<Vec<_>, _>>()?;
        self.custom_shape_contains_function(type_.type_name(), arguments, visited)
    }

    fn custom_shape_contains_function(
        &self,
        name: &CustomTypeName,
        arguments: Vec<bool>,
        visited: &mut HashSet<CustomFunctionShape>,
    ) -> Result<bool, PlanError> {
        let shape = CustomFunctionShape {
            name: name.clone(),
            arguments,
        };
        if !visited.insert(shape.clone()) {
            return Ok(false);
        }

        let result = if name.package().is_empty()
            && name.module() == PRELUDE_MODULE_NAME
            && name.name() == "Result"
        {
            if shape.arguments.len() != 2 {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: name.name().clone(),
                        reason: InvalidCustomTypeReason::TypeArgumentCount,
                    },
                });
            }
            Ok(shape.arguments.iter().copied().any(|argument| argument))
        } else {
            let Some(definition) = self
                .custom_types
                .iter()
                .find(|definition| definition.name() == name)
            else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: name.name().clone(),
                        reason: InvalidCustomTypeReason::UnknownDefinition,
                    },
                });
            };
            if definition.parameters().len() != shape.arguments.len() {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: name.name().clone(),
                        reason: InvalidCustomTypeReason::TypeArgumentCount,
                    },
                });
            }
            let mut contains_function = false;
            'constructors: for constructor in definition.constructors() {
                for field in constructor.fields() {
                    if self.custom_template_contains_function(
                        field.type_(),
                        name,
                        &shape.arguments,
                        visited,
                    )? {
                        contains_function = true;
                        break 'constructors;
                    }
                }
            }
            Ok(contains_function)
        };

        visited.remove(&shape);
        result
    }

    fn custom_template_contains_function(
        &self,
        template: &CustomTypeTemplate,
        owner: &CustomTypeName,
        arguments: &[bool],
        visited: &mut HashSet<CustomFunctionShape>,
    ) -> Result<bool, PlanError> {
        match template {
            CustomTypeTemplate::Function { .. } => Ok(true),
            CustomTypeTemplate::Tuple(elements) => {
                let mut contains_function = false;
                for element in elements {
                    contains_function |=
                        self.custom_template_contains_function(element, owner, arguments, visited)?;
                }
                Ok(contains_function)
            }
            CustomTypeTemplate::List(element) => {
                self.custom_template_contains_function(element, owner, arguments, visited)
            }
            CustomTypeTemplate::Custom {
                name,
                arguments: templates,
            } => {
                let mut nested_arguments = Vec::with_capacity(templates.len());
                for template in templates {
                    let contains_function = self
                        .custom_template_contains_function(template, owner, arguments, visited)?;
                    nested_arguments.push(contains_function);
                }
                self.custom_shape_contains_function(name, nested_arguments, visited)
            }
            CustomTypeTemplate::Parameter(parameter) => match arguments.get(parameter.0).copied() {
                Some(contains_function) => Ok(contains_function),
                None => Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CustomType {
                        name: owner.name().clone(),
                        reason: InvalidCustomTypeReason::ParameterType,
                    },
                }),
            },
            CustomTypeTemplate::Int
            | CustomTypeTemplate::Float
            | CustomTypeTemplate::String
            | CustomTypeTemplate::BitArray
            | CustomTypeTemplate::UtfCodepoint
            | CustomTypeTemplate::Bool
            | CustomTypeTemplate::Nil => Ok(false),
        }
    }

    pub(super) fn lookup_function_local(&self, name: &EcoString) -> Option<FunctionLocalBinding> {
        match self.bindings.get(name)? {
            LocalBinding::Function(binding) => Some(binding.clone()),
            LocalBinding::Primitive(_)
            | LocalBinding::Custom { .. }
            | LocalBinding::Tuple { .. }
            | LocalBinding::List(_) => None,
        }
    }

    pub(super) fn anonymous_function_error_name(&self) -> EcoString {
        self.anonymous_functions.next_name()
    }

    pub(super) fn reserve_anonymous_function_name(&mut self) -> EcoString {
        self.anonymous_functions.reserve_name()
    }

    pub(super) fn allocate_anonymous_function(
        &mut self,
        name: EcoString,
        return_type: ValueType,
        params: Vec<FunctionParam>,
        runtime_id: RuntimeFunctionId,
    ) -> (EcoString, FunctionInfo) {
        self.anonymous_functions
            .allocate(name, return_type, params, runtime_id)
    }

    pub(super) fn allocate_anonymous_runtime_id(
        &mut self,
        return_type: &ValueType,
    ) -> RuntimeFunctionId {
        self.anonymous_functions.allocate_runtime_id(return_type)
    }

    pub(super) fn push_anonymous_function(&mut self, function: FunctionPlan) {
        self.anonymous_functions.push(function);
    }

    pub(super) fn anonymous_function_context(
        &mut self,
        function_name: EcoString,
    ) -> PlanContext<'_> {
        PlanContext {
            module_name: self.module_name,
            current_function: function_name,
            functions: self.functions,
            custom_types: self.custom_types,
            anonymous_functions: self.anonymous_functions,
            bindings: HashMap::new(),
            next_int_local: 0,
            next_float_local: 0,
            next_string_local: 0,
            next_bit_array_local: 0,
            next_utf_codepoint_local: 0,
            next_custom_local: 0,
            next_bool_local: 0,
            next_nil_local: 0,
            next_tuple_local: 0,
            next_int_list_local: 0,
            next_string_list_local: 0,
            next_bit_array_list_local: 0,
            next_utf_codepoint_list_local: 0,
            next_custom_list_local: 0,
            next_float_list_local: 0,
            next_bool_list_local: 0,
            next_nil_list_local: 0,
            next_tuple_list_local: 0,
            next_list_list_local: 0,
            next_function_list_local: 0,
            next_int_function_local: 0,
            next_float_function_local: 0,
            next_string_function_local: 0,
            next_bit_array_function_local: 0,
            next_utf_codepoint_function_local: 0,
            next_custom_function_local: 0,
            next_bool_function_local: 0,
            next_nil_function_local: 0,
            next_tuple_function_local: 0,
            next_list_function_local: 0,
            next_function_function_local: 0,
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

    pub(super) fn define_captures(&mut self, captures: Vec<CaptureBinding>) -> Vec<CaptureArg> {
        captures
            .into_iter()
            .map(|capture| self.define_capture(capture))
            .collect()
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

    fn define_capture(&mut self, capture: CaptureBinding) -> CaptureArg {
        match capture.binding {
            LocalBinding::Primitive(LocalId::Int(local)) => {
                let target = self.define_int_local(capture.name.clone());
                CaptureArg::int(target, IntExpr::local_get(local, capture.name))
            }
            LocalBinding::Primitive(LocalId::Float(local)) => {
                let target = self.define_float_local(capture.name.clone());
                CaptureArg::float(target, FloatExpr::local_get(local, capture.name))
            }
            LocalBinding::Primitive(LocalId::String(local)) => {
                let target = self.define_string_local(capture.name.clone());
                CaptureArg::string(target, StringExpr::local_get(local, capture.name))
            }
            LocalBinding::Primitive(LocalId::BitArray(local)) => {
                let target = self.define_bit_array_local(capture.name.clone());
                CaptureArg::bit_array(target, BitArrayExpr::local_get(local, capture.name))
            }
            LocalBinding::Primitive(LocalId::UtfCodepoint(local)) => {
                let target = self.define_utf_codepoint_local(capture.name.clone());
                CaptureArg::utf_codepoint(target, UtfCodepointExpr::local_get(local, capture.name))
            }
            LocalBinding::Custom { local, type_ } => {
                let target = self.define_custom_local(capture.name.clone(), type_.clone());
                CaptureArg::custom(target, CustomExpr::local_get(local, capture.name, type_))
            }
            LocalBinding::Primitive(LocalId::Bool(local)) => {
                let target = self.define_bool_local(capture.name.clone());
                CaptureArg::bool(target, BoolExpr::local_get(local, capture.name))
            }
            LocalBinding::Primitive(LocalId::Nil(local)) => {
                let target = self.define_nil_local(capture.name.clone());
                CaptureArg::nil(target, NilExpr::local_get(local, capture.name))
            }
            LocalBinding::Tuple { local, type_ } => {
                let target = self.define_tuple_local(capture.name.clone(), type_.clone());
                CaptureArg::tuple(target, TupleExpr::local_get(local, capture.name, type_))
            }
            LocalBinding::List(local) => {
                CaptureArg::list(self.define_list_capture_value(capture.name, local))
            }
            LocalBinding::Function(FunctionLocalBinding::Int { local, type_ }) => {
                let target = self.define_int_function_local(capture.name.clone(), type_.clone());
                CaptureArg::int_function(
                    target,
                    IntFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::Float { local, type_ }) => {
                let target = self.define_float_function_local(capture.name.clone(), type_.clone());
                CaptureArg::float_function(
                    target,
                    FloatFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::String { local, type_ }) => {
                let target = self.define_string_function_local(capture.name.clone(), type_.clone());
                CaptureArg::string_function(
                    target,
                    StringFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::BitArray { local, type_ }) => {
                let target =
                    self.define_bit_array_function_local(capture.name.clone(), type_.clone());
                CaptureArg::bit_array_function(
                    target,
                    BitArrayFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::UtfCodepoint { local, type_ }) => {
                let target =
                    self.define_utf_codepoint_function_local(capture.name.clone(), type_.clone());
                CaptureArg::utf_codepoint_function(
                    target,
                    UtfCodepointFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::Custom { local, type_ }) => {
                let target = self.define_custom_function_local(capture.name.clone(), type_.clone());
                CaptureArg::custom_function(
                    target,
                    CustomFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::Bool { local, type_ }) => {
                let target = self.define_bool_function_local(capture.name.clone(), type_.clone());
                CaptureArg::bool_function(
                    target,
                    BoolFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::Nil { local, type_ }) => {
                let target = self.define_nil_function_local(capture.name.clone(), type_.clone());
                CaptureArg::nil_function(
                    target,
                    NilFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::Tuple { local, type_ }) => {
                let target = self.define_tuple_function_local(capture.name.clone(), type_.clone());
                CaptureArg::tuple_function(
                    target,
                    TupleFunctionExpr::local_get(local, capture.name, type_),
                )
            }
            LocalBinding::Function(FunctionLocalBinding::List(local)) => {
                let target = self.define_list_function_local(
                    capture.name.clone(),
                    local.type_().clone(),
                    local.item_type(),
                );
                CaptureArg::list_function(target, ListFunctionExpr::local_get(local, capture.name))
            }
            LocalBinding::Function(FunctionLocalBinding::Function { local, type_ }) => {
                let target =
                    self.define_function_function_local(capture.name.clone(), type_.clone());
                CaptureArg::function_function(
                    target,
                    FunctionFunctionExpr::local_get(local, capture.name, type_),
                )
            }
        }
    }
}

pub(in crate::planner) struct AnonymousFunctions {
    next_function_index: usize,
    next_anonymous_index: usize,
    runtime_ids: FunctionRuntimeIds,
    functions: Vec<FunctionPlan>,
}

impl AnonymousFunctions {
    pub(in crate::planner) fn new(
        next_function_index: usize,
        runtime_ids: FunctionRuntimeIds,
    ) -> Self {
        Self {
            next_function_index,
            next_anonymous_index: 0,
            runtime_ids,
            functions: Vec::new(),
        }
    }

    pub(in crate::planner) fn into_functions(self) -> Vec<FunctionPlan> {
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
        return_type: ValueType,
        params: Vec<FunctionParam>,
        runtime_id: RuntimeFunctionId,
    ) -> (EcoString, FunctionInfo) {
        let info = FunctionInfo {
            id: FunctionId::new(self.next_function_index),
            runtime_id,
            return_type,
            params,
        };
        self.next_function_index += 1;
        (name, info)
    }

    fn allocate_runtime_id(&mut self, return_type: &ValueType) -> RuntimeFunctionId {
        self.runtime_ids.next(return_type)
    }

    fn push(&mut self, function: FunctionPlan) {
        self.functions.push(function);
    }
}

impl Default for AnonymousFunctions {
    fn default() -> Self {
        Self::new(0, FunctionRuntimeIds::default())
    }
}

impl FunctionInfo {
    pub(super) fn arity(&self) -> usize {
        self.params.len()
    }

    pub(super) fn return_type(&self) -> ValueType {
        self.return_type.clone()
    }

    pub(super) fn reference(&self) -> FunctionReference {
        FunctionReference::new(self.runtime_id.clone(), self.param_locals())
    }

    pub(super) fn param_locals(&self) -> Vec<ParamLocal> {
        self.params
            .iter()
            .map(|param| param.local.clone())
            .collect()
    }
}

#[derive(Debug, Default)]
pub(in crate::planner) struct FunctionRuntimeIds {
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bit_array: usize,
    next_utf_codepoint: usize,
    next_custom: usize,
    next_bool: usize,
    next_nil: usize,
    next_tuple: usize,
    next_int_list: usize,
    next_string_list: usize,
    next_bit_array_list: usize,
    next_utf_codepoint_list: usize,
    next_custom_list: usize,
    next_float_list: usize,
    next_bool_list: usize,
    next_nil_list: usize,
    next_tuple_list: usize,
    next_list_list: usize,
    next_function_list: usize,
    next_int_function: usize,
    next_float_function: usize,
    next_string_function: usize,
    next_bit_array_function: usize,
    next_utf_codepoint_function: usize,
    next_custom_function: usize,
    next_bool_function: usize,
    next_nil_function: usize,
    next_tuple_function: usize,
    next_int_list_function: usize,
    next_string_list_function: usize,
    next_bit_array_list_function: usize,
    next_utf_codepoint_list_function: usize,
    next_custom_list_function: usize,
    next_float_list_function: usize,
    next_bool_list_function: usize,
    next_nil_list_function: usize,
    next_tuple_list_function: usize,
    next_list_list_function: usize,
    next_function_list_function: usize,
    next_function_function: usize,
}

impl FunctionRuntimeIds {
    pub(in crate::planner) fn next(&mut self, return_type: &ValueType) -> RuntimeFunctionId {
        match return_type {
            ValueType::Int => RuntimeFunctionId::Int(self.next_int_id()),
            ValueType::Float => RuntimeFunctionId::Float(self.next_float_id()),
            ValueType::String => RuntimeFunctionId::String(self.next_string_id()),
            ValueType::BitArray => RuntimeFunctionId::BitArray(self.next_bit_array_id()),
            ValueType::UtfCodepoint => {
                RuntimeFunctionId::UtfCodepoint(self.next_utf_codepoint_id())
            }
            ValueType::Custom(return_type) => RuntimeFunctionId::Custom {
                id: self.next_custom_id(),
                return_type: return_type.clone(),
            },
            ValueType::Bool => RuntimeFunctionId::Bool(self.next_bool_id()),
            ValueType::Nil => RuntimeFunctionId::Nil(self.next_nil_id()),
            ValueType::Tuple(return_type) => RuntimeFunctionId::Tuple {
                id: self.next_tuple_id(),
                return_type: return_type.clone(),
            },
            ValueType::List(return_type) => {
                RuntimeFunctionId::List(self.next_list_id(return_type.as_ref().clone()))
            }
            ValueType::Function(return_type) => self.next_function(return_type.as_ref().clone()),
        }
    }

    pub(super) fn next_function(&mut self, return_type: FunctionType) -> RuntimeFunctionId {
        let id = match return_type.return_() {
            ValueType::Int => FunctionFunctionId::Int(self.next_int_function_id()),
            ValueType::Float => FunctionFunctionId::Float(self.next_float_function_id()),
            ValueType::String => FunctionFunctionId::String(self.next_string_function_id()),
            ValueType::BitArray => FunctionFunctionId::BitArray(self.next_bit_array_function_id()),
            ValueType::UtfCodepoint => {
                FunctionFunctionId::UtfCodepoint(self.next_utf_codepoint_function_id())
            }
            ValueType::Custom(_) => FunctionFunctionId::Custom(self.next_custom_function_id()),
            ValueType::Bool => FunctionFunctionId::Bool(self.next_bool_function_id()),
            ValueType::Nil => FunctionFunctionId::Nil(self.next_nil_function_id()),
            ValueType::Tuple(_) => FunctionFunctionId::Tuple(self.next_tuple_function_id()),
            ValueType::List(item_type) => FunctionFunctionId::List(
                self.next_list_function_id(return_type.clone(), item_type.as_ref().clone()),
            ),
            ValueType::Function(_) => {
                FunctionFunctionId::Function(self.next_function_function_id())
            }
        };

        RuntimeFunctionId::Function { id, return_type }
    }

    pub(in crate::planner) fn next_int_id(&mut self) -> IntFunctionId {
        let id = IntFunctionId(self.next_int);
        self.next_int += 1;
        id
    }

    pub(in crate::planner) fn next_string_id(&mut self) -> StringFunctionId {
        let id = StringFunctionId(self.next_string);
        self.next_string += 1;
        id
    }

    pub(in crate::planner) fn next_bit_array_id(&mut self) -> BitArrayFunctionId {
        let id = BitArrayFunctionId(self.next_bit_array);
        self.next_bit_array += 1;
        id
    }

    pub(in crate::planner) fn next_utf_codepoint_id(&mut self) -> UtfCodepointFunctionId {
        let id = UtfCodepointFunctionId(self.next_utf_codepoint);
        self.next_utf_codepoint += 1;
        id
    }

    pub(in crate::planner) fn next_custom_id(&mut self) -> CustomFunctionId {
        let id = CustomFunctionId(self.next_custom);
        self.next_custom += 1;
        id
    }

    pub(in crate::planner) fn next_float_id(&mut self) -> FloatFunctionId {
        let id = FloatFunctionId(self.next_float);
        self.next_float += 1;
        id
    }

    pub(in crate::planner) fn next_bool_id(&mut self) -> BoolFunctionId {
        let id = BoolFunctionId(self.next_bool);
        self.next_bool += 1;
        id
    }

    pub(in crate::planner) fn next_nil_id(&mut self) -> NilFunctionId {
        let id = NilFunctionId(self.next_nil);
        self.next_nil += 1;
        id
    }

    pub(in crate::planner) fn next_tuple_id(&mut self) -> TupleFunctionId {
        let id = TupleFunctionId(self.next_tuple);
        self.next_tuple += 1;
        id
    }

    pub(in crate::planner) fn next_list_id(&mut self, item_type: ValueType) -> ListFunctionId {
        match item_type {
            ValueType::Int => ListFunctionId::Int(self.next_int_list_id()),
            ValueType::String => ListFunctionId::String(self.next_string_list_id()),
            ValueType::BitArray => ListFunctionId::BitArray(self.next_bit_array_list_id()),
            ValueType::UtfCodepoint => {
                ListFunctionId::UtfCodepoint(self.next_utf_codepoint_list_id())
            }
            ValueType::Custom(item_type) => ListFunctionId::Custom {
                id: self.next_custom_list_id(),
                item_type,
            },
            ValueType::Float => ListFunctionId::Float(self.next_float_list_id()),
            ValueType::Bool => ListFunctionId::Bool(self.next_bool_list_id()),
            ValueType::Nil => ListFunctionId::Nil(self.next_nil_list_id()),
            ValueType::Tuple(item_type) => ListFunctionId::Tuple {
                id: self.next_tuple_list_id(),
                item_type,
            },
            ValueType::List(item_type) => ListFunctionId::List {
                id: self.next_list_list_id(),
                item_type,
            },
            ValueType::Function(item_type) => ListFunctionId::Function {
                id: self.next_function_list_id(),
                item_type: *item_type,
            },
        }
    }

    pub(in crate::planner) fn next_int_list_id(&mut self) -> IntListFunctionId {
        let id = IntListFunctionId(self.next_int_list);
        self.next_int_list += 1;
        id
    }

    pub(in crate::planner) fn next_string_list_id(&mut self) -> StringListFunctionId {
        let id = StringListFunctionId(self.next_string_list);
        self.next_string_list += 1;
        id
    }

    pub(in crate::planner) fn next_bit_array_list_id(&mut self) -> BitArrayListFunctionId {
        let id = BitArrayListFunctionId(self.next_bit_array_list);
        self.next_bit_array_list += 1;
        id
    }

    pub(in crate::planner) fn next_utf_codepoint_list_id(&mut self) -> UtfCodepointListFunctionId {
        let id = UtfCodepointListFunctionId(self.next_utf_codepoint_list);
        self.next_utf_codepoint_list += 1;
        id
    }

    pub(in crate::planner) fn next_custom_list_id(&mut self) -> CustomListFunctionId {
        let id = CustomListFunctionId(self.next_custom_list);
        self.next_custom_list += 1;
        id
    }

    pub(in crate::planner) fn next_float_list_id(&mut self) -> FloatListFunctionId {
        let id = FloatListFunctionId(self.next_float_list);
        self.next_float_list += 1;
        id
    }

    pub(in crate::planner) fn next_bool_list_id(&mut self) -> BoolListFunctionId {
        let id = BoolListFunctionId(self.next_bool_list);
        self.next_bool_list += 1;
        id
    }

    pub(in crate::planner) fn next_nil_list_id(&mut self) -> NilListFunctionId {
        let id = NilListFunctionId(self.next_nil_list);
        self.next_nil_list += 1;
        id
    }

    pub(in crate::planner) fn next_tuple_list_id(&mut self) -> TupleListFunctionId {
        let id = TupleListFunctionId(self.next_tuple_list);
        self.next_tuple_list += 1;
        id
    }

    pub(in crate::planner) fn next_list_list_id(&mut self) -> ListListFunctionId {
        let id = ListListFunctionId(self.next_list_list);
        self.next_list_list += 1;
        id
    }

    pub(in crate::planner) fn next_function_list_id(&mut self) -> FunctionListFunctionId {
        let id = FunctionListFunctionId(self.next_function_list);
        self.next_function_list += 1;
        id
    }

    pub(in crate::planner) fn next_int_function_id(&mut self) -> IntFunctionFunctionId {
        let id = IntFunctionFunctionId(self.next_int_function);
        self.next_int_function += 1;
        id
    }

    pub(in crate::planner) fn next_string_function_id(&mut self) -> StringFunctionFunctionId {
        let id = StringFunctionFunctionId(self.next_string_function);
        self.next_string_function += 1;
        id
    }

    pub(in crate::planner) fn next_bit_array_function_id(&mut self) -> BitArrayFunctionFunctionId {
        let id = BitArrayFunctionFunctionId(self.next_bit_array_function);
        self.next_bit_array_function += 1;
        id
    }

    pub(in crate::planner) fn next_utf_codepoint_function_id(
        &mut self,
    ) -> UtfCodepointFunctionFunctionId {
        let id = UtfCodepointFunctionFunctionId(self.next_utf_codepoint_function);
        self.next_utf_codepoint_function += 1;
        id
    }

    pub(in crate::planner) fn next_custom_function_id(&mut self) -> CustomFunctionFunctionId {
        let id = CustomFunctionFunctionId(self.next_custom_function);
        self.next_custom_function += 1;
        id
    }

    pub(in crate::planner) fn next_float_function_id(&mut self) -> FloatFunctionFunctionId {
        let id = FloatFunctionFunctionId(self.next_float_function);
        self.next_float_function += 1;
        id
    }

    pub(in crate::planner) fn next_bool_function_id(&mut self) -> BoolFunctionFunctionId {
        let id = BoolFunctionFunctionId(self.next_bool_function);
        self.next_bool_function += 1;
        id
    }

    pub(in crate::planner) fn next_nil_function_id(&mut self) -> NilFunctionFunctionId {
        let id = NilFunctionFunctionId(self.next_nil_function);
        self.next_nil_function += 1;
        id
    }

    pub(in crate::planner) fn next_tuple_function_id(&mut self) -> TupleFunctionFunctionId {
        let id = TupleFunctionFunctionId(self.next_tuple_function);
        self.next_tuple_function += 1;
        id
    }

    pub(in crate::planner) fn next_list_function_id(
        &mut self,
        type_: FunctionType,
        item_type: ValueType,
    ) -> ListFunctionFunctionId {
        match item_type {
            ValueType::Int => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_int_list_function,
                    type_,
                    ValueType::Int,
                );
                self.next_int_list_function += 1;
                id
            }
            ValueType::String => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_string_list_function,
                    type_,
                    ValueType::String,
                );
                self.next_string_list_function += 1;
                id
            }
            ValueType::BitArray => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_bit_array_list_function,
                    type_,
                    ValueType::BitArray,
                );
                self.next_bit_array_list_function += 1;
                id
            }
            ValueType::UtfCodepoint => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_utf_codepoint_list_function,
                    type_,
                    ValueType::UtfCodepoint,
                );
                self.next_utf_codepoint_list_function += 1;
                id
            }
            ValueType::Custom(item_type) => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_custom_list_function,
                    type_,
                    ValueType::Custom(item_type),
                );
                self.next_custom_list_function += 1;
                id
            }
            ValueType::Float => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_float_list_function,
                    type_,
                    ValueType::Float,
                );
                self.next_float_list_function += 1;
                id
            }
            ValueType::Bool => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_bool_list_function,
                    type_,
                    ValueType::Bool,
                );
                self.next_bool_list_function += 1;
                id
            }
            ValueType::Nil => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_nil_list_function,
                    type_,
                    ValueType::Nil,
                );
                self.next_nil_list_function += 1;
                id
            }
            item_type @ ValueType::Tuple(_) => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_tuple_list_function,
                    type_,
                    item_type,
                );
                self.next_tuple_list_function += 1;
                id
            }
            item_type @ ValueType::List(_) => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_list_list_function,
                    type_,
                    item_type,
                );
                self.next_list_list_function += 1;
                id
            }
            item_type @ ValueType::Function(_) => {
                let id = ListFunctionFunctionId::from_item_type(
                    self.next_function_list_function,
                    type_,
                    item_type,
                );
                self.next_function_list_function += 1;
                id
            }
        }
    }

    pub(in crate::planner) fn next_function_function_id(&mut self) -> FunctionFunctionFunctionId {
        let id = FunctionFunctionFunctionId(self.next_function_function);
        self.next_function_function += 1;
        id
    }
}

impl ValueType {
    pub(super) fn from_gleam(type_: &Type) -> Option<Self> {
        match type_ {
            Type::Var { type_ } => match type_.borrow().deref() {
                TypeVar::Link { type_ } => Self::from_gleam(type_.as_ref()),
                TypeVar::Unbound { .. } | TypeVar::Generic { .. } => None,
            },
            Type::Tuple { elements } => Some(Self::Tuple(
                elements
                    .iter()
                    .map(|element| Self::from_gleam(element.as_ref()))
                    .collect::<Option<Vec<_>>>()?,
            )),
            Type::Fn { arguments, return_ } => Some(Self::Function(Box::new(FunctionType::new(
                arguments
                    .iter()
                    .map(|argument| Self::from_gleam(argument.as_ref()))
                    .collect::<Option<Vec<_>>>()?,
                Self::from_gleam(return_.as_ref())?,
            )))),
            Type::Named {
                package,
                module,
                name,
                arguments,
                ..
            } => {
                if type_.is_int() {
                    Some(Self::Int)
                } else if type_.is_float() {
                    Some(Self::Float)
                } else if type_.is_string() {
                    Some(Self::String)
                } else if type_.is_bit_array() {
                    Some(Self::BitArray)
                } else if type_.is_utf_codepoint() {
                    Some(Self::UtfCodepoint)
                } else if type_.is_bool() {
                    Some(Self::Bool)
                } else if type_.is_nil() {
                    Some(Self::Nil)
                } else if let Some(element) = type_.list_type() {
                    Some(Self::List(Box::new(Self::from_gleam(element.as_ref())?)))
                } else {
                    Some(Self::Custom(CustomType::new(
                        CustomTypeName::new(package.clone(), module.clone(), name.clone()),
                        arguments
                            .iter()
                            .map(|argument| Self::from_gleam(argument.as_ref()))
                            .collect::<Option<Vec<_>>>()?,
                    )))
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::FunctionLocalBinding;
    use super::{
        AnonymousFunctions, FunctionInfo, FunctionRuntimeIds, PlanContext,
        ResolvedCustomConstructor, instantiate_custom_type_template,
    };
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionId, BitArrayFunctionLocalId, BitArrayListExpr,
        BitArrayListItem, BitArrayListLocalId, BitArrayLocalId, BoolFunctionExpr,
        BoolFunctionLocalId, BoolListExpr, BoolListItem, BoolListLocalId, BoolLocalId, CaptureArg,
        CustomConstructor, CustomConstructorDefinition, CustomConstructorField,
        CustomFieldDefinition, CustomFunctionLocalId, CustomType, CustomTypeDefinition,
        CustomTypeName, CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
        FloatFunctionExpr, FloatFunctionId, FloatFunctionLocalId, FloatListExpr, FloatListItem,
        FloatListLocalId, FloatLocalId, FunctionFunctionExpr, FunctionFunctionLocalId,
        FunctionListExpr, FunctionListItem, FunctionListLocalId, FunctionType, IntFunctionId,
        IntFunctionLocalId, IntListExpr, IntListItem, IntListLocalId, IntLocalId, ListExpr,
        ListFunctionExpr, ListListExpr, ListListItem, ListListLocalId, ListLocal, ListLocalExpr,
        LocalId, NilFunctionExpr, NilFunctionLocalId, NilListExpr, NilListItem, NilListLocalId,
        NilLocalId, ParamLocal, RuntimeFunctionId, StringFunctionExpr, StringFunctionLocalId,
        StringListExpr, StringListItem, StringListLocalId, StringLocalId, TupleFunctionExpr,
        TupleFunctionLocalId, TupleListExpr, TupleListItem, TupleListLocalId, TupleLocalId,
        UtfCodepointListExpr, UtfCodepointListItem, UtfCodepointListLocalId, ValueType,
    };
    use crate::planner::{InvalidCustomTypeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::Publicity;
    use gleam_core::type_::{
        self, Deprecation, PatternConstructor, ValueConstructor, ValueConstructorVariant,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn custom_constructor_and_equality_metadata_reject_invalid_typed_ast_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let generic_name = CustomTypeName::new("geam".into(), module.clone(), "Generic".into());
        let function_name =
            CustomTypeName::new("geam".into(), module.clone(), "FunctionBox".into());
        let tuple_function_name =
            CustomTypeName::new("geam".into(), module.clone(), "TupleFunctionBox".into());
        let list_function_name =
            CustomTypeName::new("geam".into(), module.clone(), "ListFunctionBox".into());
        let broken_name = CustomTypeName::new("geam".into(), module.clone(), "Broken".into());
        let tuple_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "TupleBroken".into());
        let custom_argument_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "CustomArgumentBroken".into());
        let nested_broken_name =
            CustomTypeName::new("geam".into(), module.clone(), "NestedBroken".into());
        let missing_name = CustomTypeName::new("geam".into(), module.clone(), "Missing".into());
        let recursive_name = CustomTypeName::new("geam".into(), module.clone(), "Recursive".into());
        let definitions = vec![
            CustomTypeDefinition::new(
                generic_name.clone(),
                CustomTypePublicity::Public,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Generic".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        Some("value".into()),
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "FunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Function {
                            arguments: vec![CustomTypeTemplate::Int],
                            return_: Box::new(CustomTypeTemplate::String),
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                tuple_function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "TupleFunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Tuple(vec![
                            CustomTypeTemplate::Int,
                            CustomTypeTemplate::Function {
                                arguments: Vec::new(),
                                return_: Box::new(CustomTypeTemplate::Nil),
                            },
                        ]),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                list_function_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "ListFunctionBox".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Function {
                            arguments: Vec::new(),
                            return_: Box::new(CustomTypeTemplate::Nil),
                        })),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0)],
                vec![CustomConstructorDefinition::new(
                    "Broken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                tuple_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "TupleBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
                            CustomTypeParameterId(0),
                        )]),
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                custom_argument_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "CustomArgumentBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: generic_name.clone(),
                            arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(
                                0,
                            ))],
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                nested_broken_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "NestedBroken".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: missing_name.clone(),
                            arguments: Vec::new(),
                        },
                    )],
                )],
            ),
            CustomTypeDefinition::new(
                recursive_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Recursive".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Custom {
                            name: recursive_name.clone(),
                            arguments: Vec::new(),
                        },
                    )],
                )],
            ),
        ];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);
        let generic_int = CustomType::new(generic_name.clone(), vec![ValueType::Int]);

        for (template, expected) in [
            (CustomTypeTemplate::Float, ValueType::Float),
            (CustomTypeTemplate::BitArray, ValueType::BitArray),
            (CustomTypeTemplate::UtfCodepoint, ValueType::UtfCodepoint),
            (CustomTypeTemplate::Bool, ValueType::Bool),
            (CustomTypeTemplate::Nil, ValueType::Nil),
            (
                CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Int)),
                ValueType::List(Box::new(ValueType::Int)),
            ),
        ] {
            assert_eq!(
                instantiate_custom_type_template(&template, &generic_int),
                Ok(expected),
            );
        }

        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Ok(ResolvedCustomConstructor {
                constructor: CustomConstructor::new(
                    generic_int.clone(),
                    "Generic".into(),
                    0,
                    vec![CustomConstructorField::new(
                        Some("value".into()),
                        ValueType::Int,
                    )],
                ),
                constructor_count: 1,
            }),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::String],
            ),
            Err(invalid_custom_field_error(&generic_int)),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Wrong".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorName,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &"other".into(),
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorModule,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                0,
                Vec::new(),
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorArity,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_int.clone(),
                "Generic".into(),
                &module,
                1,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorIndex,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                CustomType::new(
                    CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
                    Vec::new(),
                ),
                "Missing".into(),
                &module,
                0,
                Vec::new(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Missing".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );
        let generic_without_arguments = CustomType::new(generic_name.clone(), Vec::new());
        assert_eq!(
            context.custom_constructor_from_parts(
                generic_without_arguments.clone(),
                "Generic".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &generic_without_arguments,
                InvalidCustomTypeReason::TypeArgumentCount,
            )),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(generic_without_arguments.clone())),
            Err(invalid_custom_constructor_error(
                &generic_without_arguments,
                InvalidCustomTypeReason::TypeArgumentCount,
            )),
        );

        let result_type = CustomType::new(
            CustomTypeName::new(
                "".into(),
                type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        assert_eq!(
            ValueType::from_gleam(type_::result(type_::int(), type_::string()).as_ref()),
            Some(ValueType::Custom(result_type.clone())),
        );
        let prelude = result_type.type_name().module().clone();
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                1,
                vec![ValueType::String],
            ),
            Ok(ResolvedCustomConstructor {
                constructor: CustomConstructor::new(
                    result_type.clone(),
                    "Error".into(),
                    1,
                    vec![CustomConstructorField::new(None, ValueType::String)],
                ),
                constructor_count: 2,
            }),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                1,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_field_error(&result_type)),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Error".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &result_type,
                InvalidCustomTypeReason::ConstructorName,
            )),
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                result_type.clone(),
                "Ok".into(),
                &prelude,
                2,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &result_type,
                InvalidCustomTypeReason::ConstructorIndex,
            )),
        );
        let malformed_result_type =
            CustomType::new(result_type.type_name().clone(), vec![ValueType::Int]);
        assert_eq!(
            context.custom_constructor_from_parts(
                malformed_result_type.clone(),
                "Ok".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &malformed_result_type,
                InvalidCustomTypeReason::TypeArgumentCount,
            )),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(malformed_result_type.clone())),
            Err(invalid_custom_constructor_error(
                &malformed_result_type,
                InvalidCustomTypeReason::TypeArgumentCount,
            )),
        );
        let non_prelude_result = CustomType::new(
            CustomTypeName::new(
                "other".into(),
                type_::PRELUDE_MODULE_NAME.into(),
                "Result".into(),
            ),
            vec![ValueType::Int, ValueType::String],
        );
        assert_eq!(
            context.custom_constructor_from_parts(
                non_prelude_result.clone(),
                "Ok".into(),
                &prelude,
                0,
                vec![ValueType::Int],
            ),
            Err(invalid_custom_constructor_error(
                &non_prelude_result,
                InvalidCustomTypeReason::UnknownDefinition,
            )),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(non_prelude_result.clone())),
            Err(invalid_custom_constructor_error(
                &non_prelude_result,
                InvalidCustomTypeReason::UnknownDefinition,
            )),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(generic_int)),
            Ok(false),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                recursive_name,
                Vec::new(),
            ))),
            Ok(false),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Tuple(vec![
                ValueType::Int,
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String,))),
            ])),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Tuple(vec![
                ValueType::Int,
                ValueType::String,
            ])),
            Ok(false),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::List(Box::new(ValueType::Function(
                Box::new(FunctionType::new(Vec::new(), ValueType::Nil)),
            )))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                function_name,
                Vec::new(),
            ))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                tuple_function_name,
                Vec::new(),
            ))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                list_function_name,
                Vec::new(),
            ))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                result_type.type_name().clone(),
                vec![
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int,))),
                    ValueType::String,
                ],
            ))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                result_type.type_name().clone(),
                vec![ValueType::Int, ValueType::String],
            ))),
            Ok(false),
        );
        let broken = CustomType::new(broken_name, vec![ValueType::Int]);
        assert_eq!(
            context.custom_constructor_from_parts(
                broken.clone(),
                "Broken".into(),
                &module,
                0,
                vec![ValueType::Int],
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Broken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(broken.clone())),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Broken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                tuple_broken_name,
                Vec::new(),
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "TupleBroken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                custom_argument_broken_name,
                Vec::new(),
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "CustomArgumentBroken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                result_type.type_name().clone(),
                vec![ValueType::String, ValueType::Custom(broken.clone())],
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Broken".into(),
                    reason: InvalidCustomTypeReason::ParameterType,
                },
            }),
        );
        let missing = CustomType::new(missing_name, Vec::new());
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(missing.clone())),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Missing".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Tuple(vec![ValueType::Custom(
                missing.clone(),
            )])),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Missing".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Custom(CustomType::new(
                nested_broken_name,
                Vec::new(),
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Missing".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );

        let missing_parameter = CustomType::new(generic_name, Vec::new());
        let parameter_error = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Generic".into(),
                reason: InvalidCustomTypeReason::ParameterType,
            },
        });
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Tuple(vec![CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(0),
                )]),
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::List(Box::new(CustomTypeTemplate::Parameter(
                    CustomTypeParameterId(0),
                ))),
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Function {
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                    return_: Box::new(CustomTypeTemplate::Int),
                },
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Function {
                    arguments: Vec::new(),
                    return_: Box::new(CustomTypeTemplate::Parameter(CustomTypeParameterId(0))),
                },
                &missing_parameter,
            ),
            parameter_error.clone(),
        );
        assert_eq!(
            instantiate_custom_type_template(
                &CustomTypeTemplate::Custom {
                    name: missing_parameter.type_name().clone(),
                    arguments: vec![CustomTypeTemplate::Parameter(CustomTypeParameterId(0))],
                },
                &missing_parameter,
            ),
            parameter_error,
        );
    }

    #[test]
    fn reject_margin_custom_function_shape_errors_through_module_planning() {
        let mut tuple_parameter = crate::planner::support::compile(
            "pub type Box(value) { Box(#(Int, value)) } fn equal(value: Box(Int)) { value == value } pub fn main() { 0 }",
        );
        let missing_parameter = type_::generic_var(99);
        tuple_parameter.definitions.custom_types[0]
            .typed_parameters
            .push(missing_parameter.clone());
        tuple_parameter.definitions.custom_types[0].constructors[0].arguments[0].type_ =
            type_::tuple(vec![type_::int(), missing_parameter]);
        let type_argument_count_error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Box".into(),
                reason: InvalidCustomTypeReason::TypeArgumentCount,
            },
        };
        assert_eq!(
            crate::planner::plan_module(tuple_parameter),
            Err(type_argument_count_error.clone()),
        );

        let mut custom_argument = crate::planner::support::compile(
            "pub type Wrapper(value) { Wrapper(value) } pub type Box(value) { Box(Wrapper(value)) } fn equal(value: Box(Int)) { value == value } pub fn main() { 0 }",
        );
        let box_type = custom_argument
            .definitions
            .custom_types
            .iter_mut()
            .find(|type_| type_.name == "Box")
            .expect("compiled module should contain Box");
        let missing_parameter = type_::generic_var(99);
        box_type.typed_parameters.push(missing_parameter.clone());
        box_type.constructors[0].arguments[0].type_ = type_::named(
            "main",
            "main",
            "Wrapper",
            Publicity::Public,
            vec![missing_parameter],
        );
        assert_eq!(
            crate::planner::plan_module(custom_argument),
            Err(type_argument_count_error),
        );
    }

    #[test]
    fn custom_constructor_typed_ast_margins_are_exact() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);
        let local = ValueConstructor::local_variable(
            crate::planner::support::dummy_span(),
            gleam_core::type_::error::VariableOrigin::generated(),
            type_::int(),
        );
        assert_eq!(
            context.custom_constructor(&local),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        let invalid_field = ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_: Arc::new(gleam_core::type_::Type::Fn {
                arguments: vec![type_::generic_var(0)],
                return_: type_::result(type_::int(), type_::string()),
            }),
            variant: ValueConstructorVariant::Record {
                name: "Ok".into(),
                arity: 1,
                field_map: None,
                location: crate::planner::support::dummy_span(),
                module: type_::PRELUDE_MODULE_NAME.into(),
                variants_count: 2,
                variant_index: 0,
                documentation: None,
            },
        };
        assert_eq!(
            context.custom_constructor(&invalid_field),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Ok".into(),
                    reason: InvalidCustomTypeReason::FieldType,
                },
            }),
        );
        let invalid_constructor_type = ValueConstructor {
            type_: type_::int(),
            ..invalid_field
        };
        assert_eq!(
            context.custom_constructor(&invalid_constructor_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Ok".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );

        let pattern = PatternConstructor {
            name: "Invalid".into(),
            field_map: None,
            documentation: None,
            module: module.clone(),
            location: crate::planner::support::dummy_span(),
            constructor_index: 0,
        };
        assert_eq!(
            context.custom_pattern_constructor(
                type_::generic_var(0).as_ref(),
                &pattern,
                Vec::new()
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Invalid".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_eq!(
            context.custom_pattern_constructor(type_::int().as_ref(), &pattern, Vec::new()),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Invalid".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
    }

    fn invalid_custom_constructor_error(
        type_: &CustomType,
        reason: InvalidCustomTypeReason,
    ) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: type_.type_name().name().clone(),
                reason,
            },
        }
    }

    fn invalid_custom_field_error(type_: &CustomType) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: type_.type_name().name().clone(),
                reason: InvalidCustomTypeReason::FieldType,
            },
        }
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
            Some(FunctionLocalBinding::Int { local, type_ })
        );
        assert_eq!(context.lookup_local(&"f".into()), None);
    }

    #[test]
    fn define_existing_param_records_tuple_function_binding() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]));

        context.define_existing_param(
            "f".into(),
            &ParamLocal::tuple_function(TupleFunctionLocalId(2), type_.clone()),
        );

        assert_eq!(
            context.lookup_function_local(&"f".into()),
            Some(FunctionLocalBinding::Tuple {
                local: TupleFunctionLocalId(2),
                type_: type_.clone(),
            }),
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
        let custom_function_type = FunctionType::new(Vec::new(), ValueType::Custom(custom_type()));
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );

        context.define_existing_param(
            "tuple".into(),
            &ParamLocal::tuple(TupleLocalId(2), tuple_type.clone()),
        );
        context.define_existing_param(
            "string_fn".into(),
            &ParamLocal::string_function(StringFunctionLocalId(3), string_type.clone()),
        );
        context.define_existing_param(
            "bit_array_fn".into(),
            &ParamLocal::bit_array_function(BitArrayFunctionLocalId(4), bit_array_type.clone()),
        );
        context.define_existing_param(
            "custom_fn".into(),
            &ParamLocal::custom_function(CustomFunctionLocalId(8), custom_function_type.clone()),
        );
        context.define_existing_param(
            "float_fn".into(),
            &ParamLocal::float_function(FloatFunctionLocalId(4), float_type.clone()),
        );
        context.define_existing_param(
            "bool_fn".into(),
            &ParamLocal::bool_function(BoolFunctionLocalId(5), bool_type.clone()),
        );
        context.define_existing_param(
            "nil_fn".into(),
            &ParamLocal::nil_function(NilFunctionLocalId(6), nil_type.clone()),
        );
        context.define_existing_param(
            "function_fn".into(),
            &ParamLocal::function_function(FunctionFunctionLocalId(7), function_type.clone()),
        );

        assert_eq!(
            context.lookup_tuple_local(&"tuple".into()),
            Some((TupleLocalId(2), tuple_type.clone())),
        );
        assert_eq!(
            context.lookup_function_local(&"string_fn".into()),
            Some(FunctionLocalBinding::String {
                local: StringFunctionLocalId(3),
                type_: string_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"bit_array_fn".into()),
            Some(FunctionLocalBinding::BitArray {
                local: BitArrayFunctionLocalId(4),
                type_: bit_array_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"custom_fn".into()),
            Some(FunctionLocalBinding::Custom {
                local: CustomFunctionLocalId(8),
                type_: custom_function_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"float_fn".into()),
            Some(FunctionLocalBinding::Float {
                local: FloatFunctionLocalId(4),
                type_: float_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"bool_fn".into()),
            Some(FunctionLocalBinding::Bool {
                local: BoolFunctionLocalId(5),
                type_: bool_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"nil_fn".into()),
            Some(FunctionLocalBinding::Nil {
                local: NilFunctionLocalId(6),
                type_: nil_type.clone(),
            }),
        );
        assert_eq!(
            context.lookup_function_local(&"function_fn".into()),
            Some(FunctionLocalBinding::Function {
                local: FunctionFunctionLocalId(7),
                type_: function_type.clone(),
            }),
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
                .0,
            9,
        );
        assert_eq!(
            context.define_internal_custom_function_local(),
            CustomFunctionLocalId(10),
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
                .0,
            8,
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
            Some((TupleLocalId(1), tuple_type)),
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
            Some(ListLocal::int(IntListLocalId(1))),
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
            context.define_list_capture_value("ints".into(), ListLocal::int(IntListLocalId(9))),
            ListLocalExpr::Int {
                local: IntListLocalId(0),
                value: IntListExpr::local_get(IntListItem, IntListLocalId(9), "ints".into()),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "strings".into(),
                ListLocal::string(StringListLocalId(9)),
            ),
            ListLocalExpr::String {
                local: StringListLocalId(0),
                value: StringListExpr::local_get(
                    StringListItem,
                    StringListLocalId(9),
                    "strings".into(),
                ),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "bit_arrays".into(),
                ListLocal::bit_array(BitArrayListLocalId(9)),
            ),
            ListLocalExpr::BitArray {
                local: BitArrayListLocalId(0),
                value: BitArrayListExpr::local_get(
                    BitArrayListItem,
                    BitArrayListLocalId(9),
                    "bit_arrays".into(),
                ),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "utf_codepoints".into(),
                ListLocal::utf_codepoint(UtfCodepointListLocalId(9)),
            ),
            ListLocalExpr::UtfCodepoint {
                local: UtfCodepointListLocalId(0),
                value: UtfCodepointListExpr::local_get(
                    UtfCodepointListItem,
                    UtfCodepointListLocalId(9),
                    "utf_codepoints".into(),
                ),
            },
        );
        assert_eq!(
            context
                .define_list_capture_value("floats".into(), ListLocal::float(FloatListLocalId(9))),
            ListLocalExpr::Float {
                local: FloatListLocalId(0),
                value: FloatListExpr::local_get(
                    FloatListItem,
                    FloatListLocalId(9),
                    "floats".into(),
                ),
            },
        );
        assert_eq!(
            context.define_list_capture_value("bools".into(), ListLocal::bool(BoolListLocalId(9))),
            ListLocalExpr::Bool {
                local: BoolListLocalId(0),
                value: BoolListExpr::local_get(BoolListItem, BoolListLocalId(9), "bools".into()),
            },
        );
        assert_eq!(
            context.define_list_capture_value("nils".into(), ListLocal::nil(NilListLocalId(9))),
            ListLocalExpr::Nil {
                local: NilListLocalId(0),
                value: NilListExpr::local_get(NilListItem, NilListLocalId(9), "nils".into()),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "tuples".into(),
                ListLocal::tuple(TupleListLocalId(9), tuple_type.clone()),
            ),
            ListLocalExpr::Tuple {
                local: TupleListLocalId(0),
                item_type: tuple_type.clone(),
                value: TupleListExpr::local_get(
                    TupleListItem::new(tuple_type.clone()),
                    TupleListLocalId(9),
                    "tuples".into(),
                ),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "lists".into(),
                ListLocal::list(ListListLocalId(9), nested_item_type.as_ref().clone()),
            ),
            ListLocalExpr::List {
                local: ListListLocalId(0),
                item_type: nested_item_type.clone(),
                value: ListListExpr::local_get(
                    ListListItem::new(nested_item_type.clone()),
                    ListListLocalId(9),
                    "lists".into(),
                ),
            },
        );
        assert_eq!(
            context.define_list_capture_value(
                "functions".into(),
                ListLocal::function(FunctionListLocalId(9), function_type.clone()),
            ),
            ListLocalExpr::Function {
                local: FunctionListLocalId(0),
                item_type: function_type.clone(),
                value: FunctionListExpr::local_get(
                    FunctionListItem::new(function_type.clone()),
                    FunctionListLocalId(9),
                    "functions".into(),
                ),
            },
        );

        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some(ListLocal::function(FunctionListLocalId(0), function_type)),
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
            Some(ListLocal::tuple(TupleListLocalId(0), tuple_type)),
        );
        assert_eq!(
            context.lookup_list_local(&"functions".into()),
            Some(ListLocal::function(
                FunctionListLocalId(0),
                nested_function_type,
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

        context.define_existing_param(
            "strings".into(),
            &ParamLocal::list(ListLocal::string(StringListLocalId(2))),
        );
        context.define_existing_param(
            "bit_arrays".into(),
            &ParamLocal::list(ListLocal::bit_array(BitArrayListLocalId(2))),
        );
        context.define_existing_param(
            "floats".into(),
            &ParamLocal::list(ListLocal::float(FloatListLocalId(3))),
        );
        context.define_existing_param(
            "bools".into(),
            &ParamLocal::list(ListLocal::bool(BoolListLocalId(4))),
        );
        context.define_existing_param(
            "nils".into(),
            &ParamLocal::list(ListLocal::nil(NilListLocalId(5))),
        );
        context.define_existing_param(
            "tuples".into(),
            &ParamLocal::list(ListLocal::tuple(TupleListLocalId(6), tuple_type.clone())),
        );
        context.define_existing_param(
            "lists".into(),
            &ParamLocal::list(ListLocal::list(ListListLocalId(7), ValueType::Int)),
        );
        context.define_existing_param(
            "functions".into(),
            &ParamLocal::list(ListLocal::function(
                FunctionListLocalId(8),
                nested_function_type.clone(),
            )),
        );

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
            context.define_internal_function_function_local(),
            FunctionFunctionLocalId(0),
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
            Some(FunctionLocalBinding::Int {
                local: IntFunctionLocalId(0),
                type_,
            })
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
        let captures = context.define_captures(captures);

        assert_eq!(
            captures[0],
            CaptureArg::float_function(
                FloatFunctionLocalId(1),
                FloatFunctionExpr::local_get(FloatFunctionLocalId(0), "f".into(), type_),
            ),
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
        let captures = context.define_captures(captures);

        assert_eq!(
            captures[0],
            CaptureArg::tuple_function(
                TupleFunctionLocalId(1),
                TupleFunctionExpr::local_get(TupleFunctionLocalId(0), "f".into(), type_),
            ),
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
        let captures = context.define_captures(captures);

        assert_eq!(
            captures,
            vec![
                CaptureArg::list(ListLocalExpr::Int {
                    local: IntListLocalId(1),
                    value: IntListExpr::local_get(IntListItem, IntListLocalId(0), "values".into(),),
                }),
                CaptureArg::list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        1,
                        function_type.clone(),
                        ValueType::Int,
                    ),
                    ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            0,
                            function_type,
                            ValueType::Int,
                        ),
                        "f".into(),
                    ),
                ),
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
        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
        );

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
        let captures = context.define_captures(captures);

        assert_eq!(
            captures,
            vec![
                CaptureArg::string_function(
                    StringFunctionLocalId(1),
                    StringFunctionExpr::local_get(
                        StringFunctionLocalId(0),
                        "string_fn".into(),
                        string_type,
                    ),
                ),
                CaptureArg::bit_array_function(
                    BitArrayFunctionLocalId(1),
                    BitArrayFunctionExpr::local_get(
                        BitArrayFunctionLocalId(0),
                        "bit_array_fn".into(),
                        bit_array_type,
                    ),
                ),
                CaptureArg::bool_function(
                    BoolFunctionLocalId(1),
                    BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(0),
                        "bool_fn".into(),
                        bool_type
                    ),
                ),
                CaptureArg::nil_function(
                    NilFunctionLocalId(1),
                    NilFunctionExpr::local_get(NilFunctionLocalId(0), "nil_fn".into(), nil_type),
                ),
                CaptureArg::function_function(
                    FunctionFunctionLocalId(1),
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(0),
                        "function_fn".into(),
                        function_type,
                    ),
                ),
            ],
        );
    }

    #[test]
    fn value_type_converts_recursive_list_types() {
        assert_eq!(
            ValueType::from_gleam(type_::bit_array().as_ref()),
            Some(ValueType::BitArray),
        );
        assert_eq!(
            ValueType::from_gleam(type_::utf_codepoint().as_ref()),
            Some(ValueType::UtfCodepoint),
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::named("package", "main", "BitArray", Publicity::Public, Vec::new(),)
                    .as_ref(),
            ),
            Some(ValueType::Custom(CustomType::new(
                crate::plan::CustomTypeName::new(
                    "package".into(),
                    "main".into(),
                    "BitArray".into(),
                ),
                Vec::new(),
            ))),
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::named(
                    "package",
                    "main",
                    "UtfCodepoint",
                    Publicity::Public,
                    Vec::new(),
                )
                .as_ref(),
            ),
            Some(ValueType::Custom(CustomType::new(
                crate::plan::CustomTypeName::new(
                    "package".into(),
                    "main".into(),
                    "UtfCodepoint".into(),
                ),
                Vec::new(),
            ))),
        );
        assert_eq!(
            ValueType::from_gleam(type_::fn_(Vec::new(), type_::list(type_::int())).as_ref()),
            Some(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            )))),
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::fn_(vec![type_::list(type_::int())], type_::int()).as_ref()
            ),
            Some(ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::Int,
            )))),
        );
        assert_eq!(
            ValueType::from_gleam(type_::tuple(vec![type_::list(type_::int())]).as_ref()),
            Some(ValueType::Tuple(vec![ValueType::List(Box::new(
                ValueType::Int
            ))])),
        );
    }

    #[test]
    fn value_type_rejects_unsupported_recursive_member_types() {
        let unsupported = || type_::generic_var(0);

        assert_eq!(ValueType::from_gleam(type_::generic_var(0).as_ref()), None);
        assert_eq!(ValueType::from_gleam(type_::unbound_var(0).as_ref()), None);
        assert_eq!(
            ValueType::from_gleam(type_::tuple(vec![unsupported()]).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::list(unsupported()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::fn_(vec![unsupported()], type_::int()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(type_::fn_(Vec::new(), unsupported()).as_ref()),
            None,
        );
        assert_eq!(
            ValueType::from_gleam(
                type_::named(
                    "geam",
                    "main",
                    "Boxed",
                    Publicity::Public,
                    vec![unsupported()],
                )
                .as_ref(),
            ),
            None,
        );
    }

    #[test]
    fn function_runtime_ids_allocate_by_return_type() {
        let mut ids = FunctionRuntimeIds::default();

        assert_eq!(
            ids.next(&ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Int),
            RuntimeFunctionId::Int(IntFunctionId(1))
        );
        assert_eq!(
            ids.next(&ValueType::Float),
            RuntimeFunctionId::Float(FloatFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::String),
            RuntimeFunctionId::String(crate::plan::StringFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::BitArray),
            RuntimeFunctionId::BitArray(BitArrayFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Custom(custom_type())),
            RuntimeFunctionId::Custom {
                id: crate::plan::CustomFunctionId(0),
                return_type: custom_type(),
            },
        );
        assert_eq!(
            ids.next(&ValueType::Bool),
            RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0))
        );
        assert_eq!(
            ids.next(&ValueType::Nil),
            RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0))
        );
    }

    #[test]
    fn function_runtime_ids_allocate_list_functions_by_item_family() {
        let mut ids = FunctionRuntimeIds::default();

        assert_eq!(
            ids.next_list_id(ValueType::Int),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Int),
        );
        assert_eq!(
            ids.next_list_id(ValueType::String),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::String),
        );
        assert_eq!(
            ids.next_list_id(ValueType::BitArray),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::BitArray),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Custom(custom_type())),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Custom(custom_type()),),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Float),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Float),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Bool),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Bool),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Nil),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Nil),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Tuple(vec![ValueType::Int])),
            crate::plan::ListFunctionId::from_item_type(0, ValueType::Tuple(vec![ValueType::Int])),
        );
        assert_eq!(
            ids.next_list_id(ValueType::List(Box::new(ValueType::Int))),
            crate::plan::ListFunctionId::from_item_type(
                0,
                ValueType::List(Box::new(ValueType::Int))
            ),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Function(Box::new(nested_function_type()))),
            crate::plan::ListFunctionId::from_item_type(
                0,
                ValueType::Function(Box::new(nested_function_type()))
            ),
        );
        assert_eq!(
            ids.next_list_id(ValueType::Int),
            crate::plan::ListFunctionId::from_item_type(1, ValueType::Int),
        );
    }

    #[test]
    fn function_runtime_ids_allocate_list_function_functions_by_item_family() {
        let mut ids = FunctionRuntimeIds::default();

        for item_type in list_item_types() {
            let type_ = list_function_type(item_type.clone());

            assert_eq!(
                ids.next_list_function_id(type_.clone(), item_type.clone()),
                crate::plan::ListFunctionFunctionId::from_item_type(0, type_, item_type),
            );
        }

        assert_eq!(
            ids.next_list_function_id(list_function_type(ValueType::Int), ValueType::Int),
            crate::plan::ListFunctionFunctionId::from_item_type(
                1,
                list_function_type(ValueType::Int),
                ValueType::Int,
            ),
        );
    }

    fn list_item_types() -> Vec<ValueType> {
        vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom_type()),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(nested_function_type())),
        ]
    }

    fn list_function_type(item_type: ValueType) -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::List(Box::new(item_type)))
    }

    fn nested_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::String)
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Int)
    }
}
