mod constant;
mod custom_type;
mod expression;
mod external_type;
mod function;
mod id;
mod pattern;
mod reference;
mod step;
mod type_scheme;

use crate::plan::SourceContext;
use ecow::EcoString;

pub(crate) use constant::{
    ConstantBitArrayFunctionInstantiation, ConstantBitArrayListInstantiation,
    ConstantBitArrayReference, ConstantBitArraySegment, ConstantBitArrayValue,
    ConstantBoolFunctionInstantiation, ConstantBoolListInstantiation, ConstantBoolReference,
    ConstantCustomFunctionInstantiation, ConstantCustomListInstantiation, ConstantCustomReference,
    ConstantExternalFunctionInstantiation, ConstantExternalListInstantiation,
    ConstantFloatFunctionInstantiation, ConstantFloatListInstantiation, ConstantFloatReference,
    ConstantFloatValue, ConstantFunctionFunctionInstantiation, ConstantFunctionInstantiation,
    ConstantFunctionListInstantiation, ConstantGenericFunctionInstantiation,
    ConstantGenericListInstantiation, ConstantInstantiation, ConstantIntFunctionInstantiation,
    ConstantIntListInstantiation, ConstantIntReference, ConstantIntValue,
    ConstantListConstructionError, ConstantListFunctionInstantiation, ConstantListInstantiation,
    ConstantListListInstantiation, ConstantNilFunctionInstantiation, ConstantNilListInstantiation,
    ConstantNilReference, ConstantParameterListListInstantiation,
    ConstantStringFunctionInstantiation, ConstantStringListInstantiation, ConstantStringReference,
    ConstantStringValue, ConstantTemplateSignature, ConstantTemplates,
    ConstantTupleFunctionInstantiation, ConstantTupleListInstantiation, ConstantTupleReference,
    ConstantUtfCodepointFunctionInstantiation, ConstantUtfCodepointListInstantiation,
    ConstantValue,
};
pub use constant::{ConstantTemplate, ConstantTemplateId};
pub(crate) use custom_type::{CustomConstructor, CustomConstructorField};
pub use custom_type::{
    CustomConstructorDefinition, CustomFieldDefinition, CustomTypeDefinition,
    CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
};
pub(crate) use external_type::ExternalValueShape;
pub use external_type::{ExternalType, ExternalTypeDefinition, ExternalTypeName};

#[cfg(test)]
pub(crate) use expression::TypedFunctionExpr;
pub(crate) use expression::custom_constructor_expr;
pub(crate) use expression::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExprKind, BitArrayFunctionExprKind,
    BitArraySegment, BoolCaseBranches, BoolExprKind, BoolFunctionExprKind, CallArgStorage,
    CaptureArg, CustomBoolCaseBranches, CustomCaseBranches, CustomConstruction, CustomExprKind,
    CustomFieldAccess, CustomFunctionExprKind, CustomLocalExpr, Endianness, ExprKind,
    ExternalExprKind, ExternalFunctionExprKind, FloatBitSize, FloatCaseBranches, FloatExprKind,
    FloatFunctionExprKind, FunctionExprKind, FunctionFunctionExprKind, GenericExpr,
    GenericExprKind, GenericFunctionExpr, GenericFunctionExprKind, IntCaseBranches, IntExprKind,
    IntFunctionExprKind, ListElements, ListFunctionExprKind, NilExprKind, NilFunctionExprKind,
    PanicExpr, PanicExprKind, PotentiallyUninhabitedCallArg, StringCaseBranches, StringEncoding,
    StringExprKind, StringFunctionExprKind, TupleExprKind, TupleFunctionExprKind,
    TypedFunctionExprKind, UtfCodepointExprKind, UtfCodepointFunctionExprKind,
};
pub use expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CallArg, CustomExpr,
    CustomFunctionExpr, Expr, ExternalExpr, ExternalFunctionExpr, FloatExpr, FloatFunctionExpr,
    FunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListFunctionExpr, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    UtfCodepointExpr, UtfCodepointFunctionExpr,
};
pub(crate) use expression::{
    BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
    CustomListExpr, CustomListItem, ExternalListExpr, ExternalListItem, FloatListExpr,
    FloatListItem, FunctionListExpr, FunctionListItem, GenericListExpr, GenericListItem,
    IntListExpr, IntListItem, ListCaseBranches, ListExpr, ListItem, ListListExpr, ListListItem,
    ListLocalExpr, ListSpreadConstructionError, ListSpreadElements, NilListExpr, NilListItem,
    ParameterListListExpr, ParameterListListItem, StoredListExpr, StringListExpr, StringListItem,
    TupleListExpr, TupleListItem, TypedListExpr, TypedListExprKind, TypedListReturnKind,
    UtfCodepointListExpr, UtfCodepointListItem,
};
pub(crate) use function::{
    BitArrayFunctionReturn, BitArrayReturn, BoolFunctionReturn, BoolReturn, CapturePosition,
    CustomFunctionReturn, CustomReturn, ExternalFunctionReturn, ExternalReturn,
    FloatFunctionReturn, FloatReturn, FunctionFunctionReturn, GenericFunctionReturn,
    GenericListReturn, GenericReturn, IntFunctionReturn, IntReturn, ListFunctionReturn,
    NilFunctionReturn, NilReturn, ParamLocal, ParamSlot, ParameterListListReturn, ReturnBody,
    ReturnBodyKind, ReturnExprKind, StringFunctionReturn, StringReturn, TupleFunctionReturn,
    TupleReturn, UtfCodepointFunctionReturn, UtfCodepointReturn,
};
#[cfg(test)]
pub(crate) use function::{
    BitArrayListReturn, BoolListReturn, CustomListReturn, FloatListReturn, FunctionListReturn,
    IntListReturn, ListListReturn, ListReturn, NilListReturn, StringListReturn, TupleListReturn,
    UtfCodepointListReturn,
};
pub use function::{FunctionTemplate, Param, ParamBinding, ReturnExpr};
#[cfg(test)]
pub(crate) use id::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    CustomFunctionFunctionId, CustomFunctionId, FloatFunctionFunctionId, FloatFunctionId,
    FunctionFunctionFunctionId, FunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    ListFunctionFunctionId, ListFunctionId, NilFunctionFunctionId, NilFunctionId,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId,
    TupleFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
};
pub use id::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BoolFunctionLocalId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    CustomFunctionLocalId, CustomListFunctionLocalId, CustomListLocalId, CustomLocalId,
    ExternalFunctionLocalId, ExternalListFunctionLocalId, ExternalListLocalId, ExternalLocalId,
    FloatFunctionLocalId, FloatListFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionFunctionLocalId, FunctionListFunctionLocalId, FunctionListLocalId,
    FunctionReturnFamily, FunctionTemplateId, GenericFunctionLocalId, GenericListFunctionLocalId,
    GenericListLocalId, GenericLocal, GenericLocalId, IntFunctionLocalId, IntListFunctionLocalId,
    IntListLocalId, IntLocalId, ListFunctionLocal, ListListFunctionLocalId, ListListLocalId,
    ListLocal, LocalId, ModuleId, NilFunctionLocalId, NilListFunctionLocalId, NilListLocalId,
    NilLocalId, StringFunctionLocalId, StringListFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
pub(crate) use id::{
    CustomFunctionLocal, CustomLocal, ExternalFunctionLocal, ExternalLocal, FunctionFunctionLocal,
    GenericFunctionLocal,
};
pub(crate) use pattern::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, CustomBindingPattern,
    CustomPattern, PatternBinding, Signedness, TotalBindingPattern, TotalBindingPatternKind,
};
pub(crate) use reference::{
    BitArrayFunctionReference, BoolFunctionReference, CustomFunctionReference,
    ExternalFunctionReference, FloatFunctionReference, FunctionFunctionReference,
    FunctionReference, GenericFunctionReference, IntFunctionReference, ListFunctionReference,
    NilFunctionReference, StringFunctionReference, TupleFunctionReference, TypedFunctionReference,
    UtfCodepointFunctionReference,
};
pub use step::Step;
pub(crate) use step::{
    AssertBinding, AssertPattern, AssertSubject, Echo, EchoSubject, ListAssertPattern,
    ListAssertTail, StepKind, StringAssertBinding,
};
pub use type_scheme::TypeScheme;
#[cfg(test)]
pub(crate) use type_scheme::monomorphic_function_instantiation;
pub(crate) use type_scheme::{FunctionInstantiation, FunctionTemplateSignature, TypeSubstitution};

#[derive(Debug, PartialEq)]
pub struct ModulePlan {
    root: ModuleId,
    entry: FunctionTemplateId,
    modules: Vec<PlannedModule>,
}

#[derive(Debug, PartialEq)]
pub struct PlannedModule {
    id: ModuleId,
    package: EcoString,
    module: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<CustomTypeDefinition>,
    constants: ConstantTemplates,
    functions: Vec<FunctionTemplate>,
    anonymous_functions: Vec<FunctionTemplate>,
}

pub(crate) struct ModulePlanParts {
    pub(crate) root: ModuleId,
    pub(crate) entry: FunctionTemplateId,
    pub(crate) modules: Vec<PlannedModule>,
}

pub(crate) struct PlannedModuleParts {
    pub(crate) module: EcoString,
    pub(crate) source_context: Option<SourceContext>,
    pub(crate) custom_types: Vec<CustomTypeDefinition>,
    pub(crate) constants: ConstantTemplates,
    pub(crate) functions: Vec<FunctionTemplate>,
    pub(crate) anonymous_functions: Vec<FunctionTemplate>,
}

impl ModulePlan {
    #[cfg(test)]
    pub(crate) fn new(
        module: EcoString,
        main: FunctionTemplate,
        functions: Vec<FunctionTemplate>,
    ) -> Self {
        let root = ModuleId::root();
        let entry = main.id();
        let mut named_functions = Vec::with_capacity(functions.len() + 1);
        named_functions.push(main);
        named_functions.extend(functions);
        Self {
            root,
            entry,
            modules: vec![PlannedModule {
                id: root,
                package: "geam".into(),
                module,
                source_context: None,
                custom_types: Vec::new(),
                constants: ConstantTemplates::empty(),
                functions: named_functions,
                anonymous_functions: Vec::new(),
            }],
        }
    }

    pub(crate) fn from_modules(
        root: ModuleId,
        entry: FunctionTemplateId,
        modules: Vec<PlannedModule>,
    ) -> Self {
        Self {
            root,
            entry,
            modules,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_anonymous_functions(
        mut self,
        anonymous_functions: Vec<FunctionTemplate>,
    ) -> Self {
        self.root_module_mut().anonymous_functions = anonymous_functions;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_custom_types(mut self, custom_types: Vec<CustomTypeDefinition>) -> Self {
        self.root_module_mut().custom_types = custom_types;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_constants(mut self, constants: ConstantTemplates) -> Self {
        self.root_module_mut().constants = constants;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_source_context(mut self, source_context: SourceContext) -> Self {
        self.root_module_mut().source_context = Some(source_context);
        self
    }

    pub fn root(&self) -> ModuleId {
        self.root
    }

    pub fn entry(&self) -> FunctionTemplateId {
        self.entry
    }

    pub fn modules(&self) -> &[PlannedModule] {
        &self.modules
    }

    pub fn module(&self) -> &EcoString {
        self.root_module().module()
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.root_module().source_context()
    }

    pub fn custom_types(&self) -> &[CustomTypeDefinition] {
        self.root_module().custom_types()
    }

    pub fn constants(&self) -> &[ConstantTemplate] {
        self.root_module().constants()
    }

    pub fn main_function(&self) -> &FunctionTemplate {
        &self.root_module().functions[self.entry.index()]
    }

    pub fn functions(&self) -> &[FunctionTemplate] {
        &self.root_module().functions[1..]
    }

    #[cfg(test)]
    pub(crate) fn anonymous_functions(&self) -> &[FunctionTemplate] {
        &self.root_module().anonymous_functions
    }

    pub(crate) fn into_parts(self) -> ModulePlanParts {
        ModulePlanParts {
            root: self.root,
            entry: self.entry,
            modules: self.modules,
        }
    }

    fn root_module(&self) -> &PlannedModule {
        &self.modules[self.root.index()]
    }

    #[cfg(test)]
    fn root_module_mut(&mut self) -> &mut PlannedModule {
        &mut self.modules[self.root.index()]
    }
}

impl PlannedModule {
    pub(crate) fn new(id: ModuleId, package: EcoString, parts: PlannedModuleParts) -> Self {
        let PlannedModuleParts {
            module,
            source_context,
            custom_types,
            constants,
            functions,
            anonymous_functions,
        } = parts;
        Self {
            id,
            package,
            module,
            source_context,
            custom_types,
            constants,
            functions,
            anonymous_functions,
        }
    }

    pub fn id(&self) -> ModuleId {
        self.id
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.source_context.as_ref()
    }

    pub fn custom_types(&self) -> &[CustomTypeDefinition] {
        &self.custom_types
    }

    pub fn constants(&self) -> &[ConstantTemplate] {
        self.constants.headers()
    }

    pub fn functions(&self) -> &[FunctionTemplate] {
        &self.functions
    }

    pub(crate) fn into_parts(self) -> PlannedModuleParts {
        PlannedModuleParts {
            module: self.module,
            source_context: self.source_context,
            custom_types: self.custom_types,
            constants: self.constants,
            functions: self.functions,
            anonymous_functions: self.anonymous_functions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModulePlan;
    use crate::plan::{
        ConstantTemplate, ConstantTemplateId, ConstantTemplateSignature, ConstantTemplates,
        ConstantValue, FunctionTemplate, FunctionTemplateId, IntExpr, IntFunctionId, ReturnExpr,
        SourceContext, TypeScheme,
    };
    use num_bigint::BigInt;

    #[test]
    fn module_plan_accessors() {
        let main = function(0, "main", 1);
        let helper = function(1, "helper", 2);
        let anonymous = function(2, "<anonymous:0>", 3);
        let signature =
            ConstantTemplateSignature::int(ConstantTemplateId::new(0), 0, TypeScheme::new(0));
        let constant = ConstantTemplate::new(signature, "answer".into());
        let plan = ModulePlan::new("main".into(), main, vec![helper])
            .with_anonymous_functions(vec![anonymous])
            .with_constants(ConstantTemplates::from_entries(vec![(
                constant.clone(),
                ConstantValue::int(BigInt::from(42)),
            )]));

        assert_eq!(plan.module(), "main");
        assert_eq!(plan.modules()[0].package(), "geam");
        assert_eq!(plan.main_function().name(), "main");
        assert_eq!(plan.functions().len(), 1);
        assert_eq!(plan.functions()[0].name(), "helper");
        assert_eq!(plan.anonymous_functions().len(), 1);
        assert_eq!(plan.anonymous_functions()[0].name(), "<anonymous:0>");
        assert_eq!(plan.constants(), &[constant]);
        assert_eq!(plan.constants()[0].scheme().parameters(), &[]);
        assert_eq!(plan.source_context(), None);
    }

    #[test]
    fn module_plan_debug_surface_contains_only_canonical_plan() {
        let plan = ModulePlan::new("main".into(), function(0, "main", 1), Vec::new())
            .with_source_context(SourceContext::new("main.gleam", "pub fn main() { panic }"));
        let debug = format!("{plan:?}");

        assert_eq!(
            debug,
            format!(
                "ModulePlan {{ root: {:?}, entry: {:?}, modules: {:?} }}",
                plan.root, plan.entry, plan.modules,
            ),
        );
    }

    #[test]
    fn module_plan_equality_includes_source_context() {
        let new_plan = || ModulePlan::new("main".into(), function(0, "main", 1), Vec::new());

        assert_eq!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan(),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan()
                .with_source_context(SourceContext::new("other.gleam", "pub fn main() { 1 }")),
        );
        assert_ne!(
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 1 }")),
            new_plan().with_source_context(SourceContext::new("main.gleam", "pub fn main() { 2 }")),
        );
        assert_ne!(
            ModulePlan::new("left".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("right".into(), function(0, "main", 1), Vec::new()),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("main".into(), function(0, "main", 2), Vec::new()),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new(
                "main".into(),
                function(0, "main", 1),
                vec![function(1, "helper", 2)],
            ),
        );
        assert_ne!(
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new()),
            ModulePlan::new("main".into(), function(0, "main", 1), Vec::new())
                .with_anonymous_functions(vec![function(1, "<anonymous:0>", 2)]),
        );
    }

    fn function(id: usize, name: &str, value: i64) -> FunctionTemplate {
        FunctionTemplate::new(
            FunctionTemplateId::new(id),
            name.into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(id), IntExpr::value(BigInt::from(value))),
        )
    }
}
