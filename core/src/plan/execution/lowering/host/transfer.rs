use super::super::function::ProfiledFunctionEntries;
use super::super::specialization::{
    SpecializationKey, SpecializedFunctionShape, ValueInhabitation,
};
use super::super::{LoweredExecution, LoweringCompletion, LoweringContext};
use super::{SealedHostFunctionLowering, SealedHostFunctionRegistry, parameter, return_, sealing};
use crate::host::{HostProfile, RegisteredHostConstructions, TransferHostFunctionImplementation};
use crate::plan::execution::function::RuntimeFunctionId;
use crate::plan::execution::host::{
    HostSpecializationError, HostedFunctionMetadata, TransferHostFunctionTableBuilder,
    TransferHostFunctionTables, TransferHostedExecutionProfile,
};
use crate::plan::execution::{ExecutionModuleContext, ExecutionProgram, ExecutionProgramCommon};
use crate::plan::{HostFunctionTemplate, ModuleId, TransferHostImplementationBinding};
use std::collections::HashMap;
use std::sync::Arc;

pub(super) struct TransferHostFunctionRegistry<Profile: HostProfile> {
    functions: HashMap<crate::plan::FunctionTemplateId, RegisteredFunction<Profile>>,
}

struct RegisteredFunction<Profile: HostProfile> {
    constructions: RegisteredHostConstructions,
    implementation: Arc<TransferHostFunctionImplementation<Profile>>,
}

pub(super) struct TransferHostFunctionLowering<'registry, Profile: HostProfile> {
    registered: &'registry TransferHostFunctionRegistry<Profile>,
    functions: TransferHostFunctionTableBuilder<Profile>,
    additional: ProfiledFunctionEntries<TransferHostedExecutionProfile>,
}

impl<Profile: HostProfile> TransferHostFunctionRegistry<Profile> {
    pub(super) fn new(bindings: Vec<TransferHostImplementationBinding<Profile>>) -> Self {
        Self {
            functions: bindings
                .into_iter()
                .map(|binding| {
                    let (template, constructions, implementation) = binding.into_parts();
                    (
                        template,
                        RegisteredFunction {
                            constructions,
                            implementation,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl<Profile: HostProfile> SealedHostFunctionRegistry for TransferHostFunctionRegistry<Profile> {
    type Execution = TransferHostedExecutionProfile;
    type Tables = TransferHostFunctionTables<Profile>;
    type Programs = ExecutionProgram<TransferHostedExecutionProfile>;
    type Lowered = LoweredExecution<TransferHostedExecutionProfile>;
    type Error = HostSpecializationError;
    type Lowering<'registry>
        = TransferHostFunctionLowering<'registry, Profile>
    where
        Self: 'registry;

    fn lowering(&self) -> Self::Lowering<'_> {
        TransferHostFunctionLowering {
            registered: self,
            functions: TransferHostFunctionTableBuilder::new(),
            additional: ProfiledFunctionEntries::default(),
        }
    }

    fn assemble(
        &self,
        root: ModuleId,
        modules: Box<[ExecutionModuleContext]>,
        main: RuntimeFunctionId,
        lowered: Box<Self::Lowered>,
    ) -> Self::Programs {
        let LoweredExecution {
            constants,
            functions,
            function_parameters,
            list_types,
            custom_types,
            external_types,
            value_shapes,
        } = *lowered;
        ExecutionProgram {
            common: Arc::new(ExecutionProgramCommon {
                root,
                modules,
                main,
                constants,
                function_parameters: Arc::new(function_parameters),
                list_types,
                custom_types,
                external_types,
                value_shapes,
            }),
            functions,
        }
    }
}

impl<Profile: HostProfile> SealedHostFunctionLowering
    for TransferHostFunctionLowering<'_, Profile>
{
    type Execution = TransferHostedExecutionProfile;
    type Tables = TransferHostFunctionTables<Profile>;
    type Lowered = LoweredExecution<TransferHostedExecutionProfile>;
    type Error = HostSpecializationError;

    fn lower_specialized(
        &mut self,
        template: &HostFunctionTemplate,
        key: &SpecializationKey,
        context: &mut LoweringContext,
    ) -> Result<(), Self::Error> {
        let index = context.specialization_index(key);
        let shape =
            SpecializedFunctionShape::instantiate(template.signature().shape(), key.substitution());
        let parameters = context.specialization_parameters(key).to_vec();
        let type_arguments = key
            .substitution()
            .arguments()
            .iter()
            .map(|argument| argument.to_module_shape().value_type())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let registered = &self.registered.functions[&template.id()];
        let return_shape = context.representations.inhabitation(shape.return_());
        let constructions =
            sealing::seal_host_types(template, &registered.constructions, key, context);
        let parameters = parameter::lower_host_parameters(&parameters, template.layout(), context);
        let type_ = context.lower_concrete_function_type(&shape);
        let metadata = Arc::new(HostedFunctionMetadata::new(
            template.package().clone(),
            template.site().clone(),
            shape.to_module_shape().type_(),
            type_arguments,
            parameters,
            constructions,
            type_,
        ));
        match registered.implementation.as_ref() {
            TransferHostFunctionImplementation::Value(function) => {
                let ValueInhabitation::Inhabited(return_shape) = return_shape else {
                    return Err(HostSpecializationError::undetermined_return_storage(
                        template.package().clone(),
                        template.site().module().clone(),
                        template.site().function().clone(),
                        shape.to_module_shape().type_(),
                    ));
                };
                sealing::seal_callbacks(template, key, &shape, &context.representations, true)?;
                let host_index = self.functions.push_scoped_value(metadata, function);
                return_::lower_direct_transfer_host_return(
                    index,
                    key,
                    return_shape,
                    host_index,
                    &mut self.additional,
                    context,
                );
            }
            TransferHostFunctionImplementation::Never(function) => {
                sealing::seal_callbacks(template, key, &shape, &context.representations, false)?;
                let host_index = self.functions.push_scoped_never(metadata, function);
                return_::lower_direct_transfer_never_return(
                    index,
                    key,
                    &return_shape,
                    host_index,
                    &mut self.additional,
                    context,
                );
            }
        }
        Ok(())
    }

    fn finish(self, context: LoweringContext) -> (LoweringCompletion<Self::Lowered>, Self::Tables) {
        let lowered = context.finish_hosted(self.additional);
        let functions = self.functions.finish();
        (lowered, functions)
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{FunctionDeclaration, WorkModuleBuilder, with_execution_scope};
    use crate::frontend::{TransferHostedTypedProgram, compile_typed_transfer_host_program};
    use crate::host::{
        AsyncHostCallError, AsyncHostComponentProfile, HostCallCompletion, HostCallable,
        HostCustomConstructorListEnd, HostCustomSchema, HostCustomType, HostFunctionType,
        HostFutureStore, HostProfile, HostProvider, HostType, HostTypeList, HostTypeListEnd,
        HostTypeParameter, TransferHostCall, TransferHostProviderModule, TransferHostProviderSet,
    };
    use crate::work_fixture::WorkComponent;
    use crate::{HostFailure, HostSpecializationErrorReason, ModuleSource, PackageSource};
    use futures_util::FutureExt;
    use num_bigint::BigInt;
    use std::convert::Infallible;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl AsyncHostComponentProfile<WorkComponent> for Profile {
        fn component_async_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    fn program(
        source: &str,
        provider: TransferHostProviderModule<Profile>,
    ) -> TransferHostedTypedProgram<Profile> {
        compile_typed_transfer_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            TransferHostProviderSet::new([provider]).expect("provider set"),
        )
        .expect("valid source")
    }

    struct NeverSchema;
    impl HostCustomSchema for NeverSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Never";
        const PARAMETER_COUNT: usize = 0;
        type Constructors = HostCustomConstructorListEnd;
    }
    type Never = HostCustomType<NeverSchema>;
    type Generic = HostTypeParameter<0>;
    type CallbackArgs = HostTypeList<Generic, HostTypeListEnd>;
    type Callback = HostFunctionType<CallbackArgs, BigInt>;

    fn accept_never<'call, Value: HostType>(
        mut call: TransferHostCall<'call, Profile, Provider, BigInt>,
        _: Value::Value<'call>,
    ) -> Result<HostCallCompletion<'call, BigInt>, AsyncHostCallError> {
        assert_eq!(call.state(), &());
        Ok(call.return_value(1.into()))
    }

    fn produce<'call>(
        _: TransferHostCall<'call, Profile, Provider, Generic>,
    ) -> Result<HostCallCompletion<'call, Generic>, AsyncHostCallError> {
        Err(HostFailure::new("native producer failed").into())
    }

    fn accept_callback<'call>(
        call: TransferHostCall<'call, Profile, Provider, BigInt>,
        _: HostCallable<'call, CallbackArgs, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, AsyncHostCallError> {
        Ok(call.return_value(1.into()))
    }

    fn diverge_callback<'call>(
        _: TransferHostCall<'call, Profile, Provider, BigInt>,
        _: HostCallable<'call, CallbackArgs, BigInt>,
    ) -> Result<Infallible, AsyncHostCallError> {
        Err(HostFailure::new("native callback failed").into())
    }

    fn diverge<'call, Return: HostType>(
        _: TransferHostCall<'call, Profile, Provider, Return>,
    ) -> Result<Infallible, AsyncHostCallError> {
        Err(HostFailure::new("native execution stopped").into())
    }

    #[test]
    fn erases_an_uninhabited_provider_specialization_before_direct_execution() {
        let host = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (Never,), BigInt, _>(
                "accept_never",
                accept_never::<Never>,
            )
            .expect("Never argument");
        let source = r#"
pub type Never
@external(erlang, "native", "accept_never")
fn accept_never(value: Never) -> Int
pub fn run() { let _ = accept_never 42 }
"#;
        let (bindings, run) = WorkModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("binding");
        let mut module = bindings
            .seal()
            .expect("uninhabited specialization is erased");
        let mut state = ();
        let mut echo = drop;
        with_execution_scope(async |guard| {
            assert_eq!(
                module
                    .attach(guard, &mut state, &mut echo)
                    .call(&run, ())
                    .expect("direct entry"),
                BigInt::from(42)
            );
        })
        .now_or_never()
        .expect("direct call is immediate");
    }

    #[test]
    fn rejects_a_value_producer_with_unresolved_return_storage_at_sealing() {
        let host = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
            .expect("generic producer");
        let source = r#"
@external(erlang, "native", "produce")
fn produce() -> value
pub fn run() { let _ = produce 42 }
"#;
        let (bindings, _) = WorkModuleBuilder::new(program(source, host))
            .expect("valid plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("binding");
        let error = bindings
            .seal()
            .err()
            .expect("unresolved producer cannot seal");
        assert_eq!(error.function(), "produce");
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UndeterminedReturnStorage
        );
    }

    #[test]
    fn rejects_symbolic_callback_arguments_for_value_and_diverging_providers() {
        let source = r#"
@external(erlang, "native", "accept")
fn accept(callback: fn(value) -> Int) -> Int
fn generic(_value) { 1 }
pub fn run() { accept(generic) }
"#;
        let value = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (Callback,), BigInt, _>("accept", accept_callback)
            .expect("value callback");
        let diverging = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_diverging_function::<Provider, (Callback,), BigInt, _>(
                "accept",
                diverge_callback,
            )
            .expect("diverging callback");
        for host in [value, diverging] {
            let (bindings, _) = WorkModuleBuilder::new(program(source, host))
                .expect("valid plan")
                .function(FunctionDeclaration::<(), BigInt>::new("run"))
                .expect("binding");
            let error = bindings
                .seal()
                .err()
                .expect("symbolic callback cannot be invoked");
            assert_eq!(error.function(), "accept");
            assert_eq!(
                error.reason(),
                &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                    callback: crate::FunctionType::new(
                        vec![crate::ValueType::Parameter(crate::plan::TypeParameterId(0))],
                        crate::ValueType::Int,
                    ),
                }
            );
        }
    }

    #[test]
    fn inhabited_specializations_execute_the_registered_value_and_diverging_callbacks() {
        let host = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>(
                "accept_value",
                accept_never::<BigInt>,
            )
            .expect("inhabited value")
            .with_scoped_function::<Provider, (), Generic, _>("produce", produce)
            .expect("generic producer")
            .with_scoped_function::<Provider, (Callback,), BigInt, _>("accept", accept_callback)
            .expect("inhabited callback")
            .with_scoped_diverging_function::<Provider, (Callback,), BigInt, _>(
                "stop",
                diverge_callback,
            )
            .expect("inhabited diverging callback");
        let source = r#"
@external(erlang, "native", "accept_value")
fn accept_value(value: Int) -> Int
@external(erlang, "native", "produce")
fn produce() -> value
@external(erlang, "native", "accept")
fn accept(callback: fn(value) -> Int) -> Int
@external(erlang, "native", "stop")
fn stop(callback: fn(value) -> Int) -> Int
fn concrete(value: Int) { value + 1 }
pub fn accepted() { accept_value(42) + accept(concrete) }
pub fn failed() -> Int { produce() }
pub fn stopped() { stop(concrete) }
"#;
        let (mut bindings, accepted) = WorkModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("accepted"))
            .expect("value binding");
        let failed = bindings
            .function(FunctionDeclaration::<(), BigInt>::new("failed"))
            .expect("concrete producer");
        let stopped = bindings
            .function(FunctionDeclaration::<(), BigInt>::new("stopped"))
            .expect("diverging provider");
        let mut module = bindings.seal().expect("inhabited source types");
        let mut state = ();
        assert!(std::ptr::eq(
            <WorkComponent as HostProvider<Profile>>::project(&mut state),
            &state
        ));
        let mut echo = drop;
        with_execution_scope(async |guard| {
            let mut execution = module.attach(guard, &mut state, &mut echo);
            assert_eq!(
                execution.call(&accepted, ()).expect("accepted values"),
                BigInt::from(2)
            );
            assert_eq!(
                execution
                    .call(&failed, ())
                    .expect_err("native producer fails")
                    .to_string(),
                "host function application::library.produce failed: native producer failed"
            );
            assert_eq!(
                execution
                    .call(&stopped, ())
                    .expect_err("native callback fails")
                    .to_string(),
                "host function application::library.stop failed: native callback failed"
            );
        })
        .now_or_never()
        .expect("direct executions");
    }

    #[test]
    fn a_diverging_native_never_return_keeps_the_host_failure_boundary() {
        let host = TransferHostProviderModule::new_for_profile("application", "library")
            .expect("provider")
            .with_scoped_diverging_function::<Provider, (), Never, _>("stop", diverge::<Never>)
            .expect("uninhabited return");
        let source = r#"
pub type Never
@external(erlang, "native", "stop")
fn stop() -> Never
pub fn run() { let _ = stop() 42 }
"#;
        let (bindings, run) = WorkModuleBuilder::new(program(source, host))
            .expect("plan")
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .expect("root");
        let mut module = bindings
            .seal()
            .expect("diverging target has no returned value");
        let mut state = ();
        let mut echo = drop;
        with_execution_scope(async |guard| {
            assert_eq!(
                module
                    .attach(guard, &mut state, &mut echo)
                    .call(&run, ())
                    .expect_err("no fabricated Never value")
                    .to_string(),
                "host function application::library.stop failed: native execution stopped"
            );
        })
        .now_or_never()
        .expect("direct failure");
    }
}
