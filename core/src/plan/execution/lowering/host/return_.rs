use super::super::LoweringContext;
use super::super::function;
use super::super::local;
use super::super::specialization::{
    SpecializationKey, SpecializedFunctionShape, StoredValueShape, ValueInhabitation,
};
use crate::plan::execution::function as execution_function;
use crate::plan::execution::function::{
    DirectHostedExecutionProfile, ExecutionFunction, ExecutionFunctionBody, ExecutionNeverFunction,
    ExecutionProfile,
};
use crate::plan::execution::graph as execution_graph;
use crate::plan::execution::host::{
    HostFunctionId, HostNeverFunctionId, HostedExecutionProfile, HostedFunctionTarget,
    TransferHostedExecutionProfile,
};

#[derive(Clone, Copy)]
pub(super) enum HostTargetIndex {
    Value(usize),
    Never(usize),
}

#[derive(Clone, Copy)]
pub(super) struct HostNeverTargetIndex(pub(super) usize);

pub(super) trait LowerHostTarget<Execution: ExecutionProfile> {
    fn lower<Body>(
        self,
        key: &SpecializationKey,
        return_: Body::Return,
    ) -> function::LoweredSpecialization<ExecutionFunction<Execution, Body>>
    where
        Body: ExecutionFunctionBody;
}

pub(super) trait LowerNeverHostTarget<Execution: ExecutionProfile> {
    fn lower_never(
        self,
        key: &SpecializationKey,
    ) -> function::LoweredSpecialization<ExecutionNeverFunction<Execution>>;
}

impl LowerHostTarget<HostedExecutionProfile> for HostTargetIndex {
    fn lower<Body>(
        self,
        key: &SpecializationKey,
        return_: Body::Return,
    ) -> function::LoweredSpecialization<ExecutionFunction<HostedExecutionProfile, Body>>
    where
        Body: ExecutionFunctionBody,
    {
        let target = match self {
            Self::Value(index) => {
                HostedFunctionTarget::value(HostFunctionId::<Body>::new(index, return_))
            }
            Self::Never(index) => HostedFunctionTarget::never(HostNeverFunctionId::new(index)),
        };
        function::lowered_host_function(key, target)
    }
}

impl LowerNeverHostTarget<HostedExecutionProfile> for HostNeverTargetIndex {
    fn lower_never(
        self,
        key: &SpecializationKey,
    ) -> function::LoweredSpecialization<ExecutionNeverFunction<HostedExecutionProfile>> {
        function::lowered_host_function(key, HostNeverFunctionId::new(self.0))
    }
}

impl LowerHostTarget<TransferHostedExecutionProfile> for HostTargetIndex {
    fn lower<Body>(
        self,
        key: &SpecializationKey,
        return_: Body::Return,
    ) -> function::LoweredSpecialization<ExecutionFunction<TransferHostedExecutionProfile, Body>>
    where
        Body: ExecutionFunctionBody,
    {
        let target = match self {
            Self::Value(index) => {
                HostedFunctionTarget::value(HostFunctionId::<Body>::new(index, return_))
            }
            Self::Never(index) => HostedFunctionTarget::never(HostNeverFunctionId::new(index)),
        };
        function::lowered_host_function(key, target)
    }
}

impl LowerNeverHostTarget<TransferHostedExecutionProfile> for HostNeverTargetIndex {
    fn lower_never(
        self,
        key: &SpecializationKey,
    ) -> function::LoweredSpecialization<ExecutionNeverFunction<TransferHostedExecutionProfile>>
    {
        function::lowered_host_function(key, HostNeverFunctionId::new(self.0))
    }
}

pub(super) fn lower_direct_transfer_host_return(
    index: usize,
    key: &SpecializationKey,
    return_: StoredValueShape,
    host_index: usize,
    functions: &mut function::ProfiledFunctionEntries<TransferHostedExecutionProfile>,
    context: &mut LoweringContext,
) {
    lower_host_return(
        index,
        key,
        return_,
        HostTargetIndex::Value(host_index),
        functions,
        context,
    );
}

pub(super) fn lower_host_return<Execution, Target>(
    index: usize,
    key: &SpecializationKey,
    return_: StoredValueShape,
    specialization: Target,
    functions: &mut function::ProfiledFunctionEntries<Execution>,
    context: &mut LoweringContext,
) where
    Execution: DirectHostedExecutionProfile,
    Target: LowerHostTarget<Execution> + Copy,
{
    use execution_function::ListFunctionId as L;
    use execution_function::RuntimeListFunctionId as R;

    match return_ {
        StoredValueShape::Int => {
            let return_ = execution_graph::IntLocalId(0);
            functions.int.push((
                index,
                lowered_host_target::<execution_function::IntFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Float => {
            let return_ = execution_graph::FloatLocalId(0);
            functions.float.push((
                index,
                lowered_host_target::<execution_function::FloatFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::String => {
            let return_ = execution_graph::StringLocalId(0);
            functions.string.push((
                index,
                lowered_host_target::<execution_function::StringFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::BitArray => {
            let return_ = execution_graph::BitArrayLocalId(0);
            functions.bit_array.push((
                index,
                lowered_host_target::<execution_function::BitArrayFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::UtfCodepoint => {
            let return_ = execution_graph::UtfCodepointLocalId(0);
            functions.utf_codepoint.push((
                index,
                lowered_host_target::<execution_function::UtfCodepointFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Custom(shape) => {
            let return_ = execution_graph::CustomLocal::new(
                execution_graph::CustomLocalId(0),
                context.lower_concrete_custom_shape(&shape),
            );
            functions.custom.push((
                index,
                lowered_host_target::<execution_function::CustomFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::External(shape) => {
            let return_ = execution_graph::ExternalLocal::new(
                execution_graph::ExternalLocalId(0),
                context.lower_concrete_external_type(&shape),
            );
            functions.external.push((
                index,
                lowered_host_target::<execution_function::ExternalFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Bool => {
            let return_ = execution_graph::BoolLocalId(0);
            functions.bool.push((
                index,
                lowered_host_target::<execution_function::BoolFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Nil => {
            let return_ = execution_graph::NilLocalId(0);
            functions.nil.push((
                index,
                lowered_host_target::<execution_function::NilFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::Tuple(_) => {
            let return_ = execution_graph::TupleLocalId(0);
            functions.tuple.push((
                index,
                lowered_host_target::<execution_function::TupleFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        StoredValueShape::List(item) => {
            match function::list_function_id(&item, index, &mut context.types) {
                R::Core(function) => match function {
                    L::Parameter(id) => functions.parameter_list.push((
                        id,
                        lowered_host_target::<execution_function::ParameterListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::ParameterListLocalId(0),
                        ),
                    )),
                    L::ParameterList(id) => functions.parameter_list_list.push((
                        id,
                        lowered_host_target::<
                            execution_function::ParameterListListFunctionBody,
                            _,
                            _,
                        >(
                            key,
                            specialization,
                            execution_graph::ParameterListListLocalId(0),
                        ),
                    )),
                    L::Int(id) => functions.int_list.push((
                        id,
                        lowered_host_target::<execution_function::IntListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::IntListLocalId(0),
                        ),
                    )),
                    L::String(id) => functions.string_list.push((
                        id,
                        lowered_host_target::<execution_function::StringListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::StringListLocalId(0),
                        ),
                    )),
                    L::BitArray(id) => functions.bit_array_list.push((
                        id,
                        lowered_host_target::<execution_function::BitArrayListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::BitArrayListLocalId(0),
                        ),
                    )),
                    L::UtfCodepoint(id) => {
                        functions.utf_codepoint_list.push((
                            id,
                            lowered_host_target::<
                                execution_function::UtfCodepointListFunctionBody,
                                _,
                                _,
                            >(
                                key,
                                specialization,
                                execution_graph::UtfCodepointListLocalId(0),
                            ),
                        ))
                    }
                    L::Custom(id) => functions.custom_list.push((
                        id,
                        lowered_host_target::<execution_function::CustomListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::CustomListLocalId(0),
                        ),
                    )),
                    L::Float(id) => functions.float_list.push((
                        id,
                        lowered_host_target::<execution_function::FloatListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::FloatListLocalId(0),
                        ),
                    )),
                    L::Bool(id) => functions.bool_list.push((
                        id,
                        lowered_host_target::<execution_function::BoolListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::BoolListLocalId(0),
                        ),
                    )),
                    L::Nil(id) => functions.nil_list.push((
                        id,
                        lowered_host_target::<execution_function::NilListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::NilListLocalId(0),
                        ),
                    )),
                    L::Tuple(id) => functions.tuple_list.push((
                        id,
                        lowered_host_target::<execution_function::TupleListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::TupleListLocalId(0),
                        ),
                    )),
                    L::List(id) => functions.list_list.push((
                        id,
                        lowered_host_target::<execution_function::ListListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::ListListLocalId(0),
                        ),
                    )),
                    L::Function(id) => functions.function_list.push((
                        id,
                        lowered_host_target::<execution_function::FunctionListFunctionBody, _, _>(
                            key,
                            specialization,
                            execution_graph::FunctionListLocalId(0),
                        ),
                    )),
                },
                R::External(id) => functions.external_list.push((
                    id,
                    lowered_host_target::<execution_function::ExternalListFunctionBody, _, _>(
                        key,
                        specialization,
                        execution_graph::ExternalListLocalId(0),
                    ),
                )),
            }
        }
        StoredValueShape::Function(function) => {
            lower_host_function_return(index, key, &function, specialization, functions, context)
        }
    }
}

fn lower_host_function_return<Execution, Target>(
    index: usize,
    key: &SpecializationKey,
    function: &SpecializedFunctionShape,
    specialization: Target,
    functions: &mut function::ProfiledFunctionEntries<Execution>,
    context: &mut LoweringContext,
) where
    Execution: DirectHostedExecutionProfile,
    Target: LowerHostTarget<Execution> + Copy,
{
    use local::SpecializedFunctionLocal as F;

    match local::function_local_at(function, 0, context) {
        F::Generic(return_) => functions.generic_function_functions.push((
            index,
            lowered_host_target::<execution_function::GenericFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Never(return_) => functions.never_function_functions.push((
            index,
            lowered_host_target::<execution_function::NeverFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Int { local: return_, .. } => functions.int_function_functions.push((
            index,
            lowered_host_target::<execution_function::IntFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Float { local: return_, .. } => functions.float_function_functions.push((
            index,
            lowered_host_target::<execution_function::FloatFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::String { local: return_, .. } => functions.string_function_functions.push((
            index,
            lowered_host_target::<execution_function::StringFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::BitArray { local: return_, .. } => functions.bit_array_function_functions.push((
            index,
            lowered_host_target::<execution_function::BitArrayFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::UtfCodepoint { local: return_, .. } => {
            functions.utf_codepoint_function_functions.push((
                index,
                lowered_host_target::<execution_function::UtfCodepointFunctionFunctionBody, _, _>(
                    key,
                    specialization,
                    return_,
                ),
            ));
        }
        F::Custom(return_) => functions.custom_function_functions.push((
            index,
            lowered_host_target::<execution_function::CustomFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::External(return_) => functions.external_function_functions.push((
            index,
            lowered_host_target::<execution_function::ExternalFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Bool { local: return_, .. } => functions.bool_function_functions.push((
            index,
            lowered_host_target::<execution_function::BoolFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Nil { local: return_, .. } => functions.nil_function_functions.push((
            index,
            lowered_host_target::<execution_function::NilFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::Tuple { local: return_, .. } => functions.tuple_function_functions.push((
            index,
            lowered_host_target::<execution_function::TupleFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
        F::List(return_) => {
            use execution_graph::ListFunctionLocal as L;

            match return_ {
                return_ @ L::Parameter { .. } => {
                    functions.parameter_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ))
                }
                return_ @ L::ParameterList { .. } => {
                    functions.parameter_list_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ))
                }
                return_ @ L::Int { .. } => functions.int_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::String { .. } => functions.string_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::BitArray { .. } => {
                    functions.bit_array_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ));
                }
                return_ @ L::UtfCodepoint { .. } => {
                    functions.utf_codepoint_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ));
                }
                return_ @ L::Custom { .. } => functions.custom_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::External { .. } => {
                    functions.external_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::ExternalListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ));
                }
                return_ @ L::Float { .. } => functions.float_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::Bool { .. } => functions.bool_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::Nil { .. } => functions.nil_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::Tuple { .. } => functions.tuple_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::List { .. } => functions.list_list_function_functions.push((
                    index,
                    lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                        key,
                        specialization,
                        return_,
                    ),
                )),
                return_ @ L::Function { .. } => {
                    functions.function_list_function_functions.push((
                        index,
                        lowered_host_target::<execution_function::CoreListFunctionFunctionBody, _, _>(
                            key,
                            specialization,
                            return_,
                        ),
                    ));
                }
            }
        }
        F::Function(return_) => functions.function_function_functions.push((
            index,
            lowered_host_target::<execution_function::FunctionFunctionFunctionBody, _, _>(
                key,
                specialization,
                return_,
            ),
        )),
    }
}

fn lowered_host_target<Body, Execution, Target>(
    key: &SpecializationKey,
    specialization: Target,
    return_: Body::Return,
) -> function::LoweredSpecialization<ExecutionFunction<Execution, Body>>
where
    Body: ExecutionFunctionBody,
    Execution: DirectHostedExecutionProfile,
    Target: LowerHostTarget<Execution>,
{
    specialization.lower::<Body>(key, return_)
}

pub(super) fn lower_uninhabited_never_return<Execution, Target>(
    index: usize,
    key: &SpecializationKey,
    target: Target,
    functions: &mut function::ProfiledFunctionEntries<Execution>,
) where
    Execution: DirectHostedExecutionProfile,
    Target: LowerNeverHostTarget<Execution>,
{
    functions.never.push((index, target.lower_never(key)));
}

pub(super) fn lower_direct_transfer_never_return(
    index: usize,
    key: &SpecializationKey,
    return_: &ValueInhabitation,
    host_index: usize,
    functions: &mut function::ProfiledFunctionEntries<TransferHostedExecutionProfile>,
    context: &mut LoweringContext,
) {
    let target = HostTargetIndex::Never(host_index);
    match return_ {
        ValueInhabitation::Inhabited(return_) => {
            lower_host_return(index, key, return_.clone(), target, functions, context)
        }
        ValueInhabitation::Uninhabited(_) => {
            lower_uninhabited_never_return(index, key, HostNeverTargetIndex(host_index), functions)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::{
        BoolFunctionBody, BoolFunctionId, IntFunctionBody, IntFunctionId, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId};
    use crate::plan::execution::host::{HostFunctionId, HostedFunctionTarget};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;

    #[test]
    fn inserts_int_graph_and_host_targets_in_the_int_return_family() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")])
        .expect("host modules should be unique");
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
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(crate::Value::Int(3.into())),
        );
    }

    #[test]
    fn inserts_bool_graph_and_host_targets_in_the_bool_return_family() {
        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/predicates")
            .expect("host module should be valid")
            .with_function("is_positive", |value: BigInt| value > BigInt::from(0))
            .expect("host function should be valid")])
        .expect("host modules should be unique");
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
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(crate::Value::Bool(true)),
        );
    }
}
