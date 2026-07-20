mod constant;
mod custom_type;
mod expression;
mod frame;
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

pub(crate) use expression::custom_constructor_expr;
pub(crate) use expression::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExprKind, BitArrayFunctionExprKind,
    BitArraySegment, BoolCaseBranches, BoolExprKind, BoolFunctionExprKind, CallArgStorage,
    CaptureArg, CustomBoolCaseBranches, CustomCaseBranches, CustomConstruction, CustomExprKind,
    CustomFieldAccess, CustomFunctionExprKind, Endianness, ExprKind, FloatBitSize,
    FloatCaseBranches, FloatExprKind, FloatFunctionExprKind, FunctionExprKind,
    FunctionFunctionCallMismatch, FunctionFunctionExprKind, GenericExpr, GenericExprKind,
    GenericFunctionExpr, GenericFunctionExprKind, IntCaseBranches, IntExprKind,
    IntFunctionExprKind, ListElements, ListFunctionExprKind, NilExprKind, NilFunctionExprKind,
    PanicExpr, PanicExprKind, PotentiallyUninhabitedCallArg, StringCaseBranches, StringEncoding,
    StringExprKind, StringFunctionExprKind, TupleExprKind, TupleFunctionExprKind,
    TypedFunctionExpr, TypedFunctionExprKind, UtfCodepointExprKind, UtfCodepointFunctionExprKind,
};
pub use expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CallArg, CustomExpr,
    CustomFunctionExpr, Expr, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr,
    IntExpr, IntFunctionExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
pub(crate) use expression::{
    BitArrayListExpr, BitArrayListItem, BoolListCaseBranches, BoolListExpr, BoolListItem,
    CustomListExpr, CustomListItem, FloatListExpr, FloatListItem, FunctionListExpr,
    FunctionListItem, GenericListExpr, GenericListItem, IntListExpr, IntListItem, ListCaseBranches,
    ListExpr, ListItem, ListListExpr, ListListItem, ListLocalExpr, ListSpreadConstructionError,
    ListSpreadElements, NilListExpr, NilListItem, ParameterListListExpr, ParameterListListItem,
    StoredListExpr, StringListExpr, StringListItem, TupleListExpr, TupleListItem, TypedListExpr,
    TypedListExprKind, TypedListReturnKind, UtfCodepointListExpr, UtfCodepointListItem,
};
pub(crate) use frame::{FrameLayout, FrameLayoutParts};
#[cfg(test)]
pub(crate) use function::ListReturn;
pub(crate) use function::{
    BitArrayFunctionReturn, BitArrayListReturn, BitArrayReturn, BoolFunctionReturn, BoolListReturn,
    BoolReturn, CapturePosition, CustomFunctionReturn, CustomListReturn, CustomReturn,
    FloatFunctionReturn, FloatListReturn, FloatReturn, FunctionFunctionReturn, FunctionListReturn,
    GenericFunctionReturn, GenericListReturn, GenericReturn, IntFunctionReturn, IntListReturn,
    IntReturn, ListFunctionReturn, ListListReturn, NilFunctionReturn, NilListReturn, NilReturn,
    ParamLocal, ParamPosition, ParamSlot, ParameterListListReturn, ReturnBody, ReturnBodyKind,
    ReturnExprKind, StringFunctionReturn, StringListReturn, StringReturn, TupleFunctionReturn,
    TupleListReturn, TupleReturn, UtfCodepointFunctionReturn, UtfCodepointListReturn,
    UtfCodepointReturn,
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
    FloatFunctionLocalId, FloatListFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionFunctionLocalId, FunctionListFunctionLocalId, FunctionListLocalId,
    FunctionReturnFamily, FunctionTemplateId, GenericFunctionLocalId, GenericListFunctionLocalId,
    GenericListLocalId, GenericLocal, GenericLocalId, IntFunctionLocalId, IntListFunctionLocalId,
    IntListLocalId, IntLocalId, ListFunctionLocal, ListListFunctionLocalId, ListListLocalId,
    ListLocal, LocalId, NilFunctionLocalId, NilListFunctionLocalId, NilListLocalId, NilLocalId,
    StringFunctionLocalId, StringListFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
pub(crate) use id::{
    CustomFunctionLocal, CustomLocal, FunctionFunctionLocal, GenericFunctionLocal,
};
pub(crate) use pattern::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, CustomBindingPattern,
    CustomPattern, PatternBinding, Signedness, TotalBindingPattern, TotalBindingPatternKind,
};
pub(crate) use reference::{
    BitArrayFunctionReference, BoolFunctionReference, CustomFunctionReference,
    FloatFunctionReference, FunctionFunctionReference, FunctionReference, GenericFunctionReference,
    IntFunctionReference, ListFunctionReference, NilFunctionReference, StringFunctionReference,
    TupleFunctionReference, TypedFunctionReference, UtfCodepointFunctionReference,
};
pub use step::Step;
pub(crate) use step::{
    AssertBinding, AssertPattern, AssertSubject, ListAssertPattern, ListAssertTail, StepKind,
    StringAssertBinding,
};
pub use type_scheme::TypeScheme;
#[cfg(test)]
pub(crate) use type_scheme::monomorphic_function_instantiation;
pub(crate) use type_scheme::{FunctionInstantiation, FunctionTemplateSignature, TypeSubstitution};

#[derive(Debug, PartialEq)]
pub struct ModulePlan {
    module: EcoString,
    source_context: Option<SourceContext>,
    custom_types: Vec<CustomTypeDefinition>,
    constants: ConstantTemplates,
    main: FunctionTemplate,
    functions: Vec<FunctionTemplate>,
    anonymous_functions: Vec<FunctionTemplate>,
}

pub(crate) struct ModulePlanParts {
    pub(crate) module: EcoString,
    pub(crate) source_context: Option<SourceContext>,
    pub(crate) custom_types: Vec<CustomTypeDefinition>,
    pub(crate) constants: ConstantTemplates,
    pub(crate) main: FunctionTemplate,
    pub(crate) functions: Vec<FunctionTemplate>,
    pub(crate) anonymous_functions: Vec<FunctionTemplate>,
}

impl ModulePlan {
    pub(crate) fn new(
        module: EcoString,
        main: FunctionTemplate,
        functions: Vec<FunctionTemplate>,
    ) -> Self {
        Self {
            module,
            source_context: None,
            custom_types: Vec::new(),
            constants: ConstantTemplates::empty(),
            main,
            functions,
            anonymous_functions: Vec::new(),
        }
    }

    pub(crate) fn with_anonymous_functions(
        mut self,
        anonymous_functions: Vec<FunctionTemplate>,
    ) -> Self {
        self.anonymous_functions = anonymous_functions;
        self
    }

    pub(crate) fn with_custom_types(mut self, custom_types: Vec<CustomTypeDefinition>) -> Self {
        self.custom_types = custom_types;
        self
    }

    pub(crate) fn with_constants(mut self, constants: ConstantTemplates) -> Self {
        self.constants = constants;
        self
    }

    pub(crate) fn with_source_context(mut self, source_context: SourceContext) -> Self {
        self.source_context = Some(source_context);
        self
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

    pub fn main_function(&self) -> &FunctionTemplate {
        &self.main
    }

    pub fn functions(&self) -> &[FunctionTemplate] {
        &self.functions
    }

    #[cfg(test)]
    pub(crate) fn anonymous_functions(&self) -> &[FunctionTemplate] {
        &self.anonymous_functions
    }

    pub(crate) fn into_parts(self) -> ModulePlanParts {
        ModulePlanParts {
            module: self.module,
            source_context: self.source_context,
            custom_types: self.custom_types,
            constants: self.constants,
            main: self.main,
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
            ConstantTemplateSignature::int(ConstantTemplateId(0), 0, TypeScheme::new(0));
        let constant = ConstantTemplate::new(signature, "answer".into());
        let plan = ModulePlan::new("main".into(), main, vec![helper])
            .with_anonymous_functions(vec![anonymous])
            .with_constants(ConstantTemplates::from_entries(vec![(
                constant.clone(),
                ConstantValue::int(BigInt::from(42)),
            )]));

        assert_eq!(plan.module(), "main");
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
                "ModulePlan {{ module: {:?}, source_context: {:?}, custom_types: {:?}, constants: {:?}, main: {:?}, functions: {:?}, anonymous_functions: {:?} }}",
                plan.module,
                plan.source_context,
                plan.custom_types,
                plan.constants,
                plan.main,
                plan.functions,
                plan.anonymous_functions,
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
