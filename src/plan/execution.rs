pub(crate) mod constant;
mod explain;
pub(crate) mod function;
pub(crate) mod graph;
pub(crate) mod host;
mod lowering;
pub(crate) mod runtime;
pub(crate) mod type_;

use self::constant::ConstantTable;
#[cfg(test)]
use self::constant::{ConstantId, ConstantProgram, ConstantValue};
#[cfg(test)]
use self::function::ExecutableFunction;
#[cfg(test)]
use self::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionBody, BoolFunctionFunctionBody,
    BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionId, CustomFunctionBody,
    CustomFunctionFunctionBody, CustomFunctionFunctionId, CustomFunctionId, CustomListFunctionId,
    FloatFunctionFunctionBody, FloatFunctionFunctionId, FloatListFunctionId,
    FunctionFunctionFunctionBody, FunctionFunctionFunctionId, FunctionListFunctionId,
    GenericFunctionFunctionBody, GenericFunctionFunctionId, IntFunctionBody,
    IntFunctionFunctionBody, IntFunctionFunctionId, IntFunctionId, IntListFunctionId,
    ListFunctionFunctionBody, ListFunctionFunctionId, ListListFunctionId,
    NeverFunctionFunctionBody, NeverFunctionFunctionId, NilFunctionBody, NilFunctionFunctionBody,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionId, ParameterListFunctionId,
    ParameterListListFunctionId, StringFunctionFunctionBody, StringFunctionFunctionId,
    StringListFunctionId, TupleFunctionBody, TupleFunctionFunctionBody, TupleFunctionFunctionId,
    TupleFunctionId, TupleListFunctionId, UtfCodepointFunctionFunctionBody,
    UtfCodepointFunctionFunctionId, UtfCodepointListFunctionId,
};
use self::function::{ExecutionProfile, FunctionLabelSource, FunctionTables, RuntimeFunctionId};
#[cfg(test)]
use self::type_::{
    CustomConstructorId, CustomConstructorRefinement, CustomTypeId, CustomValueShape,
    CustomValueShapeId, FunctionListTypeId, FunctionType, ListListTypeId, ListStorageTypeId,
    ListTypeId, TupleListTypeId, ValueShapeDescriptor, ValueShapeId, ValueType,
};
use self::type_::{CustomTypeTable, ListTypeTable, ValueShapeTable};
use crate::host::HostProfile;
use crate::plan::{HostedModulePlan, ModuleId, ModulePlan, SourceContext};
use ecow::EcoString;
pub use explain::ExecutionPlanExplanation;
pub use host::HostSpecializationError;
use std::convert::Infallible;

pub struct ExecutionPlan {
    program: ExecutionProgram<Infallible>,
}

pub struct HostedExecution<Profile: HostProfile> {
    program: ExecutionProgram<host::HostedExecutionProfile<Profile>>,
    host_functions: host::HostFunctionTables<Profile>,
}

pub(crate) struct ExecutionProgram<Profile: ExecutionProfile> {
    common: ExecutionProgramCommon,
    functions: FunctionTables<Profile>,
}

struct ExecutionProgramCommon {
    root: ModuleId,
    modules: Box<[ExecutionModuleContext]>,
    main: RuntimeFunctionId,
    constants: ConstantTable,
    list_types: ListTypeTable,
    custom_types: CustomTypeTable,
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

    pub fn module(&self) -> &EcoString {
        &self.program.common.modules[self.program.common.root.index()].module
    }

    pub fn source_context(&self) -> Option<&SourceContext> {
        self.program.common.modules[self.program.common.root.index()]
            .source_context
            .as_ref()
    }

    #[cfg(test)]
    pub(crate) fn source_context_for(&self, module: &EcoString) -> Option<&SourceContext> {
        self.program
            .common
            .modules
            .iter()
            .find(|context| &context.module == module)
            .and_then(|context| context.source_context.as_ref())
    }

    pub fn explain(&self) -> ExecutionPlanExplanation<'_> {
        ExecutionPlanExplanation::new(self)
    }
}

#[cfg(test)]
impl ExecutionPlan {
    pub(crate) fn main_runtime(&self) -> RuntimeFunctionId {
        self.program.common.main.clone()
    }

    pub(crate) fn constant<Return: ConstantValue>(
        &self,
        id: ConstantId<Return>,
    ) -> &ConstantProgram<Return> {
        self.program.common.constants.get(id)
    }

    pub(crate) fn list_value_type(&self, id: ListTypeId) -> crate::plan::ValueType {
        self.program
            .common
            .list_types
            .list_value_type(id, &self.program.common.custom_types)
    }

    #[cfg(test)]
    pub(crate) fn list_storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.program.common.list_types.storage_type(id)
    }

    pub(crate) fn tuple_list_item_type(&self, id: TupleListTypeId) -> Vec<crate::plan::ValueType> {
        self.program
            .common
            .list_types
            .tuple_item_type(id, &self.program.common.custom_types)
    }

    pub(crate) fn nested_list_item_type(&self, id: ListListTypeId) -> crate::plan::ValueType {
        self.program
            .common
            .list_types
            .nested_list_item_type(id, &self.program.common.custom_types)
    }

    pub(crate) fn function_list_item_type(
        &self,
        id: FunctionListTypeId,
    ) -> crate::plan::FunctionType {
        self.program
            .common
            .list_types
            .function_item_type(id, &self.program.common.custom_types)
    }

    pub(crate) fn function_type(&self, type_: &FunctionType) -> crate::plan::FunctionType {
        self.program
            .common
            .list_types
            .function_type(type_, &self.program.common.custom_types)
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
                .map(|argument| self.value_shape_type(*argument))
                .collect(),
        )
    }

    #[cfg(test)]
    fn value_shape_type(&self, id: ValueShapeId) -> crate::plan::ValueType {
        match self.program.common.value_shapes.get(id) {
            ValueShapeDescriptor::Parameter(parameter) => {
                crate::plan::ValueType::Parameter(*parameter)
            }
            ValueShapeDescriptor::Int => crate::plan::ValueType::Int,
            ValueShapeDescriptor::Float => crate::plan::ValueType::Float,
            ValueShapeDescriptor::String => crate::plan::ValueType::String,
            ValueShapeDescriptor::BitArray => crate::plan::ValueType::BitArray,
            ValueShapeDescriptor::UtfCodepoint => crate::plan::ValueType::UtfCodepoint,
            ValueShapeDescriptor::Bool => crate::plan::ValueType::Bool,
            ValueShapeDescriptor::Nil => crate::plan::ValueType::Nil,
            ValueShapeDescriptor::Tuple(elements) => crate::plan::ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_shape_type(*element))
                    .collect(),
            ),
            ValueShapeDescriptor::List(item) => {
                crate::plan::ValueType::List(Box::new(self.value_shape_type(*item)))
            }
            ValueShapeDescriptor::Function { arguments, return_ } => {
                crate::plan::ValueType::Function(Box::new(crate::plan::FunctionType::new(
                    arguments
                        .iter()
                        .map(|argument| self.value_shape_type(*argument))
                        .collect(),
                    self.value_shape_type(*return_),
                )))
            }
            ValueShapeDescriptor::Custom(custom) => {
                crate::plan::ValueType::Custom(self.custom_shape_type(*custom))
            }
        }
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

    pub(crate) fn int_function(&self, id: IntFunctionId) -> &ExecutableFunction<IntFunctionBody> {
        self.program.functions.int_function(id)
    }

    pub(crate) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionBody> {
        self.program.functions.bit_array_function(id)
    }

    pub(crate) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomFunctionBody> {
        self.program.functions.custom_function(id)
    }

    #[cfg(test)]
    pub(crate) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        self.program.functions.custom_function_id(index)
    }

    pub(crate) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutableFunction<BoolFunctionBody> {
        self.program.functions.bool_function(id)
    }

    pub(crate) fn nil_function(&self, id: NilFunctionId) -> &ExecutableFunction<NilFunctionBody> {
        self.program.functions.nil_function(id)
    }

    pub(crate) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<TupleFunctionBody> {
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
    ) -> &ExecutableFunction<IntFunctionFunctionBody> {
        self.program.functions.int_function_function(id)
    }

    pub(crate) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionFunctionBody> {
        self.program.functions.float_function_function(id)
    }

    pub(crate) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionFunctionBody> {
        self.program.functions.string_function_function(id)
    }

    pub(crate) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionFunctionBody> {
        self.program.functions.bit_array_function_function(id)
    }

    pub(crate) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionFunctionBody> {
        self.program.functions.utf_codepoint_function_function(id)
    }

    pub(crate) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionFunctionBody> {
        self.program.functions.custom_function_function(id)
    }

    pub(crate) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionFunctionBody> {
        self.program.functions.generic_function_function(id)
    }

    pub(crate) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionFunctionBody> {
        self.program.functions.never_function_function(id)
    }

    pub(crate) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionFunctionBody> {
        self.program.functions.bool_function_function(id)
    }

    pub(crate) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionFunctionBody> {
        self.program.functions.nil_function_function(id)
    }

    pub(crate) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionFunctionBody> {
        self.program.functions.tuple_function_function(id)
    }

    pub(crate) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionFunctionBody> {
        self.program.functions.list_function_function(id)
    }

    pub(crate) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionFunctionBody> {
        self.program.functions.function_function_function(id)
    }

    #[cfg(test)]
    pub(crate) fn function_function_function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        self.program.functions.function_function_function_id(index)
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
        })
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
}

#[cfg(test)]
mod tests {
    use super::HostedExecution;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        BoolFunctionBody, BoolFunctionId, IntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId};
    use crate::plan::execution::host::{HostFunctionId, HostedFunctionTarget};
    use crate::{
        HostModule, HostProviderSet, ModuleSource, PackageSource, compile_typed_host_program,
        compile_typed_module, plan_host_program, plan_module,
    };
    use num_bigint::BigInt;

    #[test]
    fn plain_execution_program_keeps_host_targets_uninhabited() {
        let typed = compile_typed_module("main", "main.gleam", "pub fn main() { 1 }")
            .expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = super::ExecutionPlan::from_module_plan(plan);
        let function: &super::ExecutableFunction<super::IntFunctionBody> =
            execution.program.functions.int_function(IntFunctionId(0));

        assert_eq!(function.body().block_graph().blocks().len(), 1);
    }

    #[test]
    fn hosted_execution_program_seals_graph_and_host_int_targets() {
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
        let source = r#"
import host/math

pub fn main() {
  let call = fn(left, right) { math.add(left, right) }
  call(1, 2)
}
"#;
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
        let graph: &ValueFunctionEntry<IntFunctionBody, HostedFunctionTarget<IntFunctionBody>> =
            execution.program.functions.int_function(IntFunctionId(0));
        let host: &ValueFunctionEntry<IntFunctionBody, HostedFunctionTarget<IntFunctionBody>> =
            execution.program.functions.int_function(IntFunctionId(2));

        assert_eq!(
            [graph, host].map(|function| match function {
                ValueFunctionEntry::Graph(_) => "graph",
                ValueFunctionEntry::Host(_) => "host",
            }),
            ["graph", "host"],
        );
        assert!(matches!(
            host,
            ValueFunctionEntry::Host(target)
                if *target
                    == HostedFunctionTarget::value(HostFunctionId::new(0, IntLocalId(0)))
        ));
        let implementation = &execution.host_functions.value_functions()[0];
        assert_eq!(implementation.name(), "add");
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(crate::Value::Int(3.into())),
        );
    }

    #[test]
    fn hosted_execution_program_seals_graph_and_host_bool_targets() {
        let predicates = HostModule::new("host_support", "host/predicates")
            .expect("host module should be valid")
            .with_function("is_positive", |value: BigInt| value > BigInt::from(0))
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([predicates]).expect("host modules should be unique");
        let source = r#"
import host/predicates

fn identity(value: Bool) {
  value
}

pub fn main() {
  identity(predicates.is_positive(1))
}
"#;
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
        let main: &ValueFunctionEntry<BoolFunctionBody, HostedFunctionTarget<BoolFunctionBody>> =
            execution.program.functions.bool_function(BoolFunctionId(0));
        let host: &ValueFunctionEntry<BoolFunctionBody, HostedFunctionTarget<BoolFunctionBody>> =
            execution.program.functions.bool_function(BoolFunctionId(1));
        let identity: &ValueFunctionEntry<
            BoolFunctionBody,
            HostedFunctionTarget<BoolFunctionBody>,
        > = execution.program.functions.bool_function(BoolFunctionId(2));

        assert_eq!(
            [main, host, identity].map(|function| match function {
                ValueFunctionEntry::Graph(_) => "graph",
                ValueFunctionEntry::Host(_) => "host",
            }),
            ["graph", "host", "graph"],
        );
        assert!(matches!(
            host,
            ValueFunctionEntry::Host(target)
                if *target
                    == HostedFunctionTarget::value(HostFunctionId::new(0, BoolLocalId(0)))
        ));
        let implementation = &execution.host_functions.value_functions()[0];
        assert_eq!(implementation.name(), "is_positive");
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(crate::Value::Bool(true)),
        );
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

    #[test]
    fn lowering_preserves_public_module_and_source_context() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("sample", "sample.gleam", source)
            .expect("source should compile");
        let context = crate::SourceContext::new("sample.gleam", source);
        let module =
            crate::plan_module_with_source(typed, context.clone()).expect("source should plan");
        let execution = super::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.module(), "sample");
        assert_eq!(execution.source_context(), Some(&context));
    }

    #[test]
    fn lowering_seeds_only_the_root_entry_and_preserves_module_sources() {
        let root_source = "pub fn main() { 7 }";
        let dependency_source = "pub fn main(value: Int) { value }";
        let typed = crate::compile_typed_program(
            "root",
            [
                crate::ModuleSource::new("root", "root.gleam", root_source),
                crate::ModuleSource::new("alpha", "alpha.gleam", dependency_source),
            ],
        )
        .expect("program should compile");
        let module = crate::plan_program(typed).expect("program should plan");
        let execution = super::ExecutionPlan::from_module_plan(module);

        assert_eq!(execution.module(), "root");
        assert_eq!(
            execution.source_context().map(|context| context.source()),
            Some(root_source),
        );
        assert_eq!(
            execution
                .source_context_for(&"alpha".into())
                .map(|context| context.source()),
            Some(dependency_source),
        );
        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Int(7.into())),
        );
        assert_eq!(
            execution.explain().to_string(),
            concat!(
                "module root\n",
                "main int#0\n",
                "\n",
                "function int#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %int#0:shape#0(Int) = int.value 7\n",
                "    return %int#0\n",
            ),
        );
    }

    #[test]
    fn lowering_deduplicates_cross_module_generic_specializations() {
        let typed = crate::compile_typed_program(
            "main",
            [
                crate::ModuleSource::new(
                    "generic",
                    "generic.gleam",
                    "pub fn identity(value: value) { value }",
                ),
                crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import generic

pub fn main() {
  #(
    generic.identity(1),
    generic.identity(2),
    generic.identity("three"),
  )
}
"#,
                ),
            ],
        )
        .expect("generic module program should compile");
        let module = crate::plan_program(typed).expect("generic module program should plan");
        let execution = super::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .int_functions
                .len(),
            1
        );
        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .string_functions
                .len(),
            1
        );
        assert_eq!(
            execution
                .program
                .functions
                .value_returns
                .tuple_functions
                .len(),
            1
        );
        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Tuple(vec![
                crate::Value::Int(1.into()),
                crate::Value::Int(2.into()),
                crate::Value::String("three".into()),
            ])),
        );
    }
}
