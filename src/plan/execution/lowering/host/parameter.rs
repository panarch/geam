use super::super::LoweringContext;
use super::super::local;
use super::super::specialization::StoredValueShape;
use crate::host::HostParameter;
use crate::plan::execution::graph as execution_graph;
use crate::plan::execution::host::{HostCallParameter, HostedFunctionParameters};

pub(super) fn lower_host_parameters(
    shapes: &[StoredValueShape],
    layout: &[HostParameter],
    context: &mut LoweringContext,
) -> HostedFunctionParameters {
    let mut prefix = local::ParameterPrefix::default();
    let parameters = shapes
        .iter()
        .enumerate()
        .map(|(position, shape)| {
            let (index, stored) = prefix.allocate_stored(shape.clone(), &context.representations);
            let entry = local::stored_value_local_at(&stored, index, context);
            let call = host_call_parameter(&stored, index, &layout[position], context);
            (entry, call)
        })
        .collect::<Vec<_>>();
    let (entry, call) = parameters.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
    HostedFunctionParameters::new(entry.into_boxed_slice(), call.into_boxed_slice())
}

fn host_call_parameter(
    shape: &StoredValueShape,
    index: usize,
    parameter: &HostParameter,
    context: &mut LoweringContext,
) -> HostCallParameter {
    match parameter {
        HostParameter::Int(_) => HostCallParameter::Int(execution_graph::IntLocalId(index)),
        HostParameter::Float(_) => HostCallParameter::Float(execution_graph::FloatLocalId(index)),
        HostParameter::String(_) => {
            HostCallParameter::String(execution_graph::StringLocalId(index))
        }
        HostParameter::BitArray(_) => {
            HostCallParameter::BitArray(execution_graph::BitArrayLocalId(index))
        }
        HostParameter::UtfCodepoint(_) => {
            HostCallParameter::UtfCodepoint(execution_graph::UtfCodepointLocalId(index))
        }
        HostParameter::Bool(_) => HostCallParameter::Bool(execution_graph::BoolLocalId(index)),
        HostParameter::Nil(_) => HostCallParameter::Nil(execution_graph::NilLocalId(index)),
        HostParameter::Value(_) => {
            HostCallParameter::Value(local::stored_value_local_at(shape, index, context))
        }
        HostParameter::List(_) => {
            HostCallParameter::List(local::stored_value_local_at(shape, index, context))
        }
        HostParameter::Tuple(_) => {
            HostCallParameter::Tuple(local::stored_value_local_at(shape, index, context))
        }
        HostParameter::Custom(_) => {
            HostCallParameter::Custom(local::stored_value_local_at(shape, index, context))
        }
        HostParameter::External(_) => {
            HostCallParameter::External(local::stored_value_local_at(shape, index, context))
        }
        HostParameter::Function(_) => {
            HostCallParameter::Function(local::stored_value_local_at(shape, index, context))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId};
    use crate::plan::execution::host::HostCallParameter;
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    #[test]
    fn lowers_repeated_mixed_parameters_to_family_local_slots_in_source_order() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/select")
            .expect("host module should be valid")
            .with_function("choose", |first: bool, value: BigInt, second: bool| {
                first && second && value > BigInt::from(0)
            })
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let source = r#"
import host/select

pub fn main() {
  select.choose(True, 1, True)
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
        let function = &execution.host_functions.value_functions()[0];

        assert_eq!(
            function.call_parameters(),
            [
                HostCallParameter::Bool(BoolLocalId(0)),
                HostCallParameter::Int(IntLocalId(0)),
                HostCallParameter::Bool(BoolLocalId(1)),
            ],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(crate::Value::Bool(true)),
        );
    }
}
