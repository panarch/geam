pub(crate) mod constant;
mod explain;
pub(crate) mod function;
pub(crate) mod graph;
pub(crate) mod host;
mod lowering;
pub(crate) mod runtime;
pub(crate) mod type_;

use self::constant::ProfiledConstantTable;
#[cfg(test)]
use self::constant::{ConstantId, ConstantValue};
#[cfg(test)]
use self::function::ExecutableFunction;
#[cfg(test)]
use self::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionFunctionId, CustomFunctionId,
    CustomListFunctionId, ExecutionBitArrayFunctionBody, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBoolFunctionBody, ExecutionBoolFunctionFunctionBody,
    ExecutionCoreListFunctionFunctionBody, ExecutionCustomFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionFloatFunctionFunctionBody,
    ExecutionFunctionFunctionFunctionBody, ExecutionGenericFunctionFunctionBody,
    ExecutionIntFunctionBody, ExecutionIntFunctionFunctionBody, ExecutionNeverFunctionFunctionBody,
    ExecutionNilFunctionBody, ExecutionNilFunctionFunctionBody,
    ExecutionStringFunctionFunctionBody, ExecutionTupleFunctionBody,
    ExecutionTupleFunctionFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    FloatFunctionFunctionId, FloatListFunctionId, FunctionFunctionFunctionId,
    FunctionListFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionId, ListListFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    NilFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    ProfiledListFunctionFunctionId, StringFunctionFunctionId, StringListFunctionId,
    TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionId,
    UtfCodepointListFunctionId,
};
use self::function::{
    ExecutionGraphProfile, ExecutionProfile, FunctionLabelSource, FunctionTables,
    ProfiledRuntimeFunctionId,
};
#[cfg(test)]
use self::type_::{
    CustomConstructorId, CustomConstructorRefinement, CustomTypeId, CustomValueShape,
    CustomValueShapeId, FunctionListTypeId, FunctionType, ListListTypeId, ListStorageTypeId,
    ListTypeId, TupleListTypeId, ValueShapeId, ValueType,
};
use self::type_::{CustomTypeTable, ExternalTypeTable, ListTypeTable, ValueShapeTable};
use crate::host::HostProfile;
use crate::plan::{HostedModulePlan, ModuleId, ModulePlan, SourceContext};
use ecow::EcoString;
pub use explain::ExecutionPlanExplanation;
pub use host::{HostSpecializationError, HostSpecializationErrorReason};
use std::convert::Infallible;

pub struct ExecutionPlan {
    program: ExecutionProgram<Infallible>,
}

pub(crate) struct LibraryFunctionEntry<Function> {
    function: Function,
    inputs: LibraryInputConstructions,
}

pub(crate) struct LibraryInputConstructions {
    variants: Box<[[type_::CustomConstructorId; 2]]>,
}

pub(crate) struct LibraryFunctionEntries {
    pub(crate) ints: Box<[LibraryFunctionEntry<function::IntFunctionId>]>,
    pub(crate) floats: Box<[LibraryFunctionEntry<function::FloatFunctionId>]>,
    pub(crate) strings: Box<[LibraryFunctionEntry<function::StringFunctionId>]>,
    pub(crate) bit_arrays: Box<[LibraryFunctionEntry<function::BitArrayFunctionId>]>,
    pub(crate) utf_codepoints: Box<[LibraryFunctionEntry<function::UtfCodepointFunctionId>]>,
    pub(crate) customs: Box<[LibraryFunctionEntry<function::CustomFunctionId>]>,
    pub(crate) bools: Box<[LibraryFunctionEntry<function::BoolFunctionId>]>,
    pub(crate) nils: Box<[LibraryFunctionEntry<function::NilFunctionId>]>,
    pub(crate) tuples: Box<[LibraryFunctionEntry<function::TupleFunctionId>]>,
}

impl<Function> LibraryFunctionEntry<Function> {
    pub(in crate::plan::execution) fn new(
        function: Function,
        inputs: LibraryInputConstructions,
    ) -> Self {
        Self { function, inputs }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn inputs(&self) -> &LibraryInputConstructions {
        &self.inputs
    }
}

impl LibraryInputConstructions {
    pub(in crate::plan::execution) fn new(variants: Vec<[type_::CustomConstructorId; 2]>) -> Self {
        Self {
            variants: variants.into_boxed_slice(),
        }
    }

    pub(crate) fn variants(&self) -> &[[type_::CustomConstructorId; 2]] {
        &self.variants
    }
}

pub struct HostedExecution<Profile: HostProfile> {
    program: ExecutionProgram<host::HostedExecutionProfile>,
    host_functions: host::HostFunctionTables<Profile>,
    external_stores: Profile::ExternalStores,
}

pub(crate) struct ExecutionProgram<Profile: ExecutionProfile> {
    common: ExecutionProgramCommon<Profile::Graph>,
    functions: FunctionTables<Profile>,
}

struct ExecutionProgramCommon<Graph: ExecutionGraphProfile> {
    root: ModuleId,
    modules: Box<[ExecutionModuleContext]>,
    main: ProfiledRuntimeFunctionId<Graph>,
    constants: ProfiledConstantTable<Graph>,
    list_types: ListTypeTable,
    custom_types: CustomTypeTable,
    external_types: ExternalTypeTable,
    value_shapes: ValueShapeTable,
}

struct ExecutionModuleContext {
    module: EcoString,
    source_context: Option<SourceContext>,
}

impl ExecutionModuleContext {
    fn new(module: EcoString, source_context: Option<SourceContext>) -> Self {
        Self {
            module,
            source_context,
        }
    }
}

impl explain::Explain for ExecutionPlan {
    fn write_explanation(&self, context: &mut explain::ExplainContext<'_, '_>) {
        context.push_str("module ");
        context.push_str(self.module());
        context.push_str("\nmain ");
        self.program
            .common
            .main
            .function_label()
            .write(context.output());
        context.push('\n');
        context.write(&self.program.functions);
        context.write(&self.program.common.constants);
    }
}

impl<Profile: HostProfile> explain::Explain for HostedExecution<Profile> {
    fn write_explanation(&self, context: &mut explain::ExplainContext<'_, '_>) {
        context.push_str("module ");
        context.push_str(&self.program.common.modules[self.program.common.root.index()].module);
        context.push_str("\nmain ");
        self.program
            .common
            .main
            .function_label()
            .write(context.output());
        context.push('\n');
        context.write(&function::HostedFunctionTablesExplanation::new(
            &self.program.functions,
            &self.host_functions,
        ));
        context.write(&self.program.common.constants);
    }
}

impl ExecutionPlan {
    pub fn from_module_plan(module_plan: ModulePlan) -> Self {
        Self {
            program: lowering::lower(module_plan),
        }
    }

    pub(crate) fn from_library_plan(
        module_plan: crate::plan::LibraryModulePlan,
        first: crate::plan::LibraryEntry,
        remaining: Vec<crate::plan::LibraryEntry>,
    ) -> (Self, LibraryFunctionEntries) {
        let (program, entries) = lowering::lower_library(module_plan, first, remaining);
        (Self { program }, entries)
    }

    pub fn module(&self) -> &EcoString {
        &self.program.common.modules[self.program.common.root.index()].module
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.program.common.modules[self.program.common.root.index()]
            .source_context
            .as_ref()
    }

    pub fn explain(&self) -> ExecutionPlanExplanation<'_> {
        ExecutionPlanExplanation::new(self)
    }
}

impl<Profile: HostProfile> HostedExecution<Profile> {
    /// Seals all entry-reachable host specializations into executable storage.
    ///
    /// A linked but unused provider does not participate in sealing.
    pub fn try_from_module_plan(
        module_plan: HostedModulePlan<Profile>,
    ) -> Result<Self, HostSpecializationError> {
        let (program, host_functions) = lowering::lower_hosted(module_plan)?;
        Ok(Self {
            program,
            host_functions,
            external_stores: Profile::ExternalStores::default(),
        })
    }

    pub(crate) fn try_from_library_plan(
        module_plan: crate::plan::HostedLibraryModulePlan<Profile>,
        first: crate::plan::LibraryEntry,
        remaining: Vec<crate::plan::LibraryEntry>,
    ) -> Result<(Self, LibraryFunctionEntries), HostSpecializationError> {
        let (program, host_functions, entries) =
            lowering::lower_hosted_library(module_plan, first, remaining)?;
        Ok((
            Self {
                program,
                host_functions,
                external_stores: Profile::ExternalStores::default(),
            },
            entries,
        ))
    }

    pub fn run_main(
        &self,
        state: &mut Profile::RunState,
        echo: &mut dyn crate::EchoSink,
    ) -> Result<crate::Value, crate::ExecutionError> {
        crate::runtime::run_hosted_main(self, state, echo)
    }

    pub fn explain(&self) -> ExecutionPlanExplanation<'_> {
        ExecutionPlanExplanation::new_hosted(self)
    }

    pub(crate) fn host_value_function<Body>(
        &self,
        id: &host::HostFunctionId<Body>,
    ) -> &host::HostedValueFunction<Profile>
    where
        Body: function::ExecutionFunctionBody,
    {
        self.host_functions.value(id)
    }

    pub(crate) fn host_never_function(
        &self,
        id: host::HostNeverFunctionId,
    ) -> &host::HostedNeverFunction<Profile> {
        self.host_functions.never(id)
    }

    pub(crate) fn external_stores(&self) -> &Profile::ExternalStores {
        &self.external_stores
    }
}

#[cfg(test)]
impl ExecutionPlan {
    pub(crate) fn main_runtime(&self) -> self::function::RuntimeFunctionId {
        self.program.common.main.runtime_id()
    }

    pub(crate) fn constant<Return: ConstantValue>(
        &self,
        id: ConstantId<Return>,
    ) -> &self::constant::ProfiledConstantProgram<Return, Infallible> {
        self.program.common.constants.get(id)
    }

    pub(crate) fn list_value_type(&self, id: ListTypeId) -> crate::plan::ValueType {
        self.program.common.list_types.list_value_type(
            id,
            &self.program.common.custom_types,
            &self.program.common.external_types,
        )
    }

    #[cfg(test)]
    pub(crate) fn list_storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.program.common.list_types.storage_type(id)
    }

    pub(crate) fn tuple_list_item_type(&self, id: TupleListTypeId) -> Vec<crate::plan::ValueType> {
        self.program.common.list_types.tuple_item_type(
            id,
            &self.program.common.custom_types,
            &self.program.common.external_types,
        )
    }

    pub(crate) fn nested_list_item_type(&self, id: ListListTypeId) -> crate::plan::ValueType {
        self.program.common.list_types.nested_list_item_type(
            id,
            &self.program.common.custom_types,
            &self.program.common.external_types,
        )
    }

    pub(crate) fn function_list_item_type(
        &self,
        id: FunctionListTypeId,
    ) -> crate::plan::FunctionType {
        self.program.common.list_types.function_item_type(
            id,
            &self.program.common.custom_types,
            &self.program.common.external_types,
        )
    }

    pub(crate) fn function_type(&self, type_: &FunctionType) -> crate::plan::FunctionType {
        self.program.common.list_types.function_type(
            type_,
            &self.program.common.custom_types,
            &self.program.common.external_types,
        )
    }

    pub(crate) fn custom_value_type(&self, id: CustomTypeId) -> crate::plan::CustomType {
        self.program.common.custom_types.value_type(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_shape_refinement(
        &self,
        shape: &CustomValueShape,
    ) -> CustomConstructorRefinement {
        self.program
            .common
            .value_shapes
            .custom(shape.shape_id())
            .constructor()
    }

    #[cfg(test)]
    pub(crate) fn custom_shape_value_type(
        &self,
        shape: &CustomValueShape,
    ) -> crate::plan::CustomType {
        self.custom_shape_type(shape.shape_id())
    }

    #[cfg(test)]
    fn custom_shape_type(&self, id: CustomValueShapeId) -> crate::plan::CustomType {
        let shape = self.program.common.value_shapes.custom(id);
        let nominal = self.program.common.custom_types.value_type(shape.type_id());
        crate::plan::CustomType::new(
            nominal.type_name().clone(),
            shape
                .arguments()
                .iter()
                .map(|argument| {
                    self.program.common.list_types.value_type(
                        self.program.common.value_shapes.value_type(*argument),
                        &self.program.common.custom_types,
                        &self.program.common.external_types,
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn shape_value_type(&self, id: ValueShapeId) -> ValueType {
        self.program.common.value_shapes.value_type(id).clone()
    }

    pub(crate) fn custom_constructor(
        &self,
        id: CustomConstructorId,
    ) -> &type_::CustomConstructorDescriptor {
        self.program.common.custom_types.constructor(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_constructor_id(
        &self,
        type_index: usize,
        constructor_index: usize,
    ) -> CustomConstructorId {
        self.program
            .common
            .custom_types
            .constructor_id(type_index, constructor_index)
    }

    pub(crate) fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutableFunction<ExecutionIntFunctionBody<Infallible>> {
        self.program.functions.int_function(id)
    }

    pub(crate) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<ExecutionBitArrayFunctionBody<Infallible>> {
        self.program.functions.bit_array_function(id)
    }

    pub(crate) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<ExecutionCustomFunctionBody<Infallible>> {
        self.program.functions.custom_function(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        self.program.functions.custom_function_id(index)
    }

    pub(crate) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutableFunction<ExecutionBoolFunctionBody<Infallible>> {
        self.program.functions.bool_function(id)
    }

    pub(crate) fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutableFunction<ExecutionNilFunctionBody<Infallible>> {
        self.program.functions.nil_function(id)
    }

    pub(crate) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<ExecutionTupleFunctionBody<Infallible>> {
        self.program.functions.tuple_function(id)
    }

    #[cfg(test)]
    pub(crate) fn parameter_list_function_id(&self, index: usize) -> ParameterListFunctionId {
        self.program.functions.parameter_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn parameter_list_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListListFunctionId {
        self.program
            .functions
            .parameter_list_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn int_list_function_id(&self, index: usize) -> IntListFunctionId {
        self.program.functions.int_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn string_list_function_id(&self, index: usize) -> StringListFunctionId {
        self.program.functions.string_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn bit_array_list_function_id(&self, index: usize) -> BitArrayListFunctionId {
        self.program.functions.bit_array_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn utf_codepoint_list_function_id(
        &self,
        index: usize,
    ) -> UtfCodepointListFunctionId {
        self.program.functions.utf_codepoint_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn custom_list_function_id(&self, index: usize) -> CustomListFunctionId {
        self.program.functions.custom_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn float_list_function_id(&self, index: usize) -> FloatListFunctionId {
        self.program.functions.float_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn bool_list_function_id(&self, index: usize) -> BoolListFunctionId {
        self.program.functions.bool_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn nil_list_function_id(&self, index: usize) -> NilListFunctionId {
        self.program.functions.nil_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn tuple_list_function_id(&self, index: usize) -> TupleListFunctionId {
        self.program.functions.tuple_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn list_list_function_id(&self, index: usize) -> ListListFunctionId {
        self.program.functions.list_list_function_id(index)
    }

    #[cfg(test)]
    pub(crate) fn function_list_function_id(&self, index: usize) -> FunctionListFunctionId {
        self.program.functions.function_list_function_id(index)
    }

    pub(crate) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionIntFunctionFunctionBody<Infallible>> {
        self.program.functions.int_function_function(id)
    }

    pub(crate) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionFloatFunctionFunctionBody<Infallible>> {
        self.program.functions.float_function_function(id)
    }

    pub(crate) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionStringFunctionFunctionBody<Infallible>> {
        self.program.functions.string_function_function(id)
    }

    pub(crate) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionBitArrayFunctionFunctionBody<Infallible>> {
        self.program.functions.bit_array_function_function(id)
    }

    pub(crate) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionUtfCodepointFunctionFunctionBody<Infallible>> {
        self.program.functions.utf_codepoint_function_function(id)
    }

    pub(crate) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionCustomFunctionFunctionBody<Infallible>> {
        self.program.functions.custom_function_function(id)
    }

    pub(crate) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionGenericFunctionFunctionBody<Infallible>> {
        self.program.functions.generic_function_function(id)
    }

    pub(crate) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionNeverFunctionFunctionBody<Infallible>> {
        self.program.functions.never_function_function(id)
    }

    pub(crate) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionBoolFunctionFunctionBody<Infallible>> {
        self.program.functions.bool_function_function(id)
    }

    pub(crate) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionNilFunctionFunctionBody<Infallible>> {
        self.program.functions.nil_function_function(id)
    }

    pub(crate) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionTupleFunctionFunctionBody<Infallible>> {
        self.program.functions.tuple_function_function(id)
    }

    pub(crate) fn core_list_function_function(
        &self,
        id: &ProfiledListFunctionFunctionId<Infallible>,
    ) -> &ExecutableFunction<ExecutionCoreListFunctionFunctionBody<Infallible>> {
        self.program.functions.core_list_function_function(id)
    }

    pub(crate) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<ExecutionFunctionFunctionFunctionBody<Infallible>> {
        self.program.functions.function_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn function_function_function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        self.program.functions.function_function_function_id(index)
    }
}

#[cfg(test)]
mod tests {
    use super::HostedExecution;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::{
        HostModule, HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
        compile_typed_module, plan_host_program, plan_module,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn plain_execution_program_keeps_host_targets_uninhabited() {
        let typed = compile_typed_module("main", "main.gleam", "pub fn main() { 1 }")
            .expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = super::ExecutionPlan::from_module_plan(plan);
        let function: &super::ExecutableFunction<
            super::function::ExecutionIntFunctionBody<Infallible>,
        > = execution.program.functions.int_function(IntFunctionId(0));

        assert_eq!(function.body().block_graph().blocks().len(), 1);
    }

    #[test]
    fn exposes_the_root_module_and_source_context() {
        let source = "pub fn main() { 1 }";
        let typed =
            compile_typed_module("sample", "sample.gleam", source).expect("source should compile");
        let context = crate::SourceContext::new("sample.gleam", source);
        let module =
            crate::plan_module_with_source(typed, context.clone()).expect("source should plan");
        let execution = super::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.module(), "sample");
        assert_eq!(execution.source_context(), Some(&context));
    }

    #[test]
    fn writes_the_complete_execution_plan() {
        let source = "pub fn main() { 1 }";
        let expected = concat!(
            "module main\n",
            "main int#0\n",
            "\n",
            "function int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
        );

        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(plan);
        });
    }

    #[test]
    fn writes_the_complete_hosted_execution_plan() {
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
        let source = "import host/math\npub fn main() { math.add(1, 2) }";
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        let expected = concat!(
            "module main\n",
            "main int#0\n",
            "\nfunction int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    %int#1:shape#0(Int) = int.value 2\n",
            "    tail int#1 args=[%int#0, %int#1]\n",
            "\nfunction int#1\n",
            "  host host_support::host/math.add signature=fn(Int, Int) -> Int\n",
        );

        assert_eq!(execution.explain().to_string(), expected);
    }
}
