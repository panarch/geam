use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::IntFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use num_bigint::BigInt;

pub(in crate::runtime) fn run_int(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: IntFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<BigInt> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    use crate::frontend::compile_typed_host_program;
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
        HostProviderSet,
    };
    use crate::plan::execution::HostedProgram;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::FunctionTarget;
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::{
        EchoOutput, EchoSink, HostFailure, HostModule, HostedExecution, ModuleSource,
        PackageSource, StatelessHostProfile, Value, compile_typed_module, plan_host_program,
        plan_module, run_main,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    struct TransferProvider;
    #[derive(Default)]
    struct SendEcho {
        outputs: usize,
    }

    impl EchoSink for SendEcho {
        fn emit(&mut self, _output: EchoOutput) {
            self.outputs += 1;
        }
    }

    impl HostProvider<StatelessHostProfile> for TransferProvider {
        type State = ();

        fn project(state: &mut ()) -> &mut Self::State {
            state
        }
    }

    fn transfer_add<'call>(
        mut call: HostCall<'call, StatelessHostProfile, TransferProvider, BigInt>,
        left: BigInt,
        right: BigInt,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let () = *call.state();
        Ok(call.return_value(left + right))
    }

    fn transfer_stop(
        _call: HostCall<'_, StatelessHostProfile, TransferProvider, BigInt>,
        _value: BigInt,
    ) -> Result<Infallible, HostCallError> {
        Err(HostFailure::new("stopped").into())
    }

    fn transfer_program() -> crate::frontend::HostedTypedProgram<StatelessHostProfile> {
        let host = HostProviderModule::new("application", "library")
            .expect("transfer host module")
            .with_scoped_function::<TransferProvider, (BigInt, BigInt), BigInt, _>(
                "add",
                transfer_add,
            )
            .expect("transfer value function")
            .with_scoped_diverging_function::<TransferProvider, (BigInt,), BigInt, _>(
                "stop",
                transfer_stop,
            )
            .expect("transfer diverging function");
        let providers = HostProviderSet::from_providers([host]).expect("transfer providers");
        compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    r#"
@external(erlang, "native", "add")
fn add(left: Int, right: Int) -> Int

@external(erlang, "native", "stop")
fn stop(value: Int) -> Int

pub fn sum(left: Int, right: Int) { echo add(left, right) }
pub fn halt(value: Int) { stop(value) }
"#,
                )],
            )],
            providers,
        )
        .expect("transfer source")
    }

    #[test]
    fn plain_int_function_protocol_executes_graph_entries() {
        let source = r#"
fn increment(value: Int) {
  value + 1
}

pub fn main() {
  increment(41)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::Int(IntFunctionId(1))),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::Int(42.into()))
        );
    }

    #[test]
    fn hosted_int_function_protocol_executes_graph_and_host_entries() {
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
        let source = r#"
import host/math

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  increment(math.add(20, 21))
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
        let mut execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::Int(IntFunctionId(2))),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::Int(IntFunctionId(1))),
            [
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(Value::Int(42.into())),
        );
    }

    #[test]
    fn transfer_int_function_protocol_exposes_graph_and_host_parameters() {
        let program = transfer_program();
        let plan =
            crate::planner::plan_host_library_program(program).expect("transfer library plan");
        let (sum, halt) = {
            let entry = |name: &str| {
                let template = plan
                    .functions()
                    .iter()
                    .find(|function| function.name() == name)
                    .expect("public transfer function")
                    .signature()
                    .id();
                LibraryEntry::new(template, LibraryValueType::Int, Vec::new(), Vec::new())
            };
            (entry("sum"), entry("halt"))
        };
        let (execution, entries) =
            HostedProgram::from_library_plan(plan, sum, vec![halt]).expect("transfer execution");

        let parameter_counts = (0..4)
            .map(|index| {
                execution
                    .function_parameters()
                    .function(&FunctionTarget::Int(IntFunctionId(index)))
                    .len()
            })
            .collect::<Vec<_>>();
        assert_eq!(parameter_counts, [2, 1, 2, 1]);
        let mut state = ();
        let mut stores = ();
        let mut echo = SendEcho::default();
        let host = crate::execution_fixture::TestHost::default();
        let domain = crate::runtime::execution::Domain::new(
            std::sync::Arc::new(execution),
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            std::num::NonZeroUsize::MIN,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let mut values = crate::runtime::graph::RetainedValues::empty();
            values.push_evaluated(crate::runtime::EvaluatedValue::Int(20.into()));
            values.push_evaluated(crate::runtime::EvaluatedValue::Int(22.into()));
            assert_eq!(
                context
                    .call(
                        *entries.ints[0].function(),
                        crate::runtime::HostCallOrigin::Entry,
                        values
                    )
                    .await
                    .expect("entry remains active"),
                Ok(42.into())
            );
            let mut values = crate::runtime::graph::RetainedValues::empty();
            values.push_evaluated(crate::runtime::EvaluatedValue::Int(0.into()));
            let error = context
                .call(
                    *entries.ints[1].function(),
                    crate::runtime::HostCallOrigin::Entry,
                    values,
                )
                .await
                .expect("entry remains active")
                .expect_err("diverging host");
            assert_eq!(
                error.to_string(),
                "host function application::library.stop failed: stopped"
            );
        }))
        .expect("host cleanup");
        assert_eq!(echo.outputs, 1);
    }
}
