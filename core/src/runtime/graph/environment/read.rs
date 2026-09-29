use super::BlockEnvironment;
use crate::plan::execution::graph::{
    FunctionFunctionLocal, ListFunctionLocal, ListLocal, ParamLocal,
};
use crate::runtime::evaluated::{EvaluatedFunctionRef, EvaluatedListRef, EvaluatedValueRef};
use crate::runtime::state::list::StoredListValueRef;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn value_ref(&self, local: &ParamLocal) -> EvaluatedValueRef<'_> {
        match local {
            ParamLocal::Int(local) => EvaluatedValueRef::Int(&self.values.ints[local.0]),
            ParamLocal::Float(local) => EvaluatedValueRef::Float(self.values.floats[local.0]),
            ParamLocal::String(local) => EvaluatedValueRef::String(&self.values.strings[local.0]),
            ParamLocal::BitArray(local) => {
                EvaluatedValueRef::BitArray(&self.values.bit_arrays[local.0])
            }
            ParamLocal::UtfCodepoint(local) => {
                EvaluatedValueRef::UtfCodepoint(self.values.utf_codepoints[local.0])
            }
            ParamLocal::Custom(local) => {
                EvaluatedValueRef::Custom(&self.values.customs[local.id().0])
            }
            ParamLocal::External(local) => {
                EvaluatedValueRef::External(&self.values.externals[local.id().0])
            }
            ParamLocal::Bool(local) => EvaluatedValueRef::Bool(self.values.bools[local.0]),
            ParamLocal::Nil(_) => EvaluatedValueRef::Nil,
            ParamLocal::Tuple { local, .. } => {
                EvaluatedValueRef::Tuple(&self.values.tuples[local.0])
            }
            ParamLocal::List(local) => self.list_value_ref(local),
            ParamLocal::IntFunction { local, .. } => {
                EvaluatedValueRef::Function(EvaluatedFunctionRef::Int(self.int_function(*local)))
            }
            ParamLocal::FloatFunction { local, .. } => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::Float(self.float_function(*local)),
            ),
            ParamLocal::StringFunction { local, .. } => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::String(self.string_function(*local)),
            ),
            ParamLocal::BitArrayFunction { local, .. } => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::BitArray(self.bit_array_function(*local)),
            ),
            ParamLocal::UtfCodepointFunction { local, .. } => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::UtfCodepoint(self.utf_codepoint_function(*local)),
            ),
            ParamLocal::BoolFunction { local, .. } => {
                EvaluatedValueRef::Function(EvaluatedFunctionRef::Bool(self.bool_function(*local)))
            }
            ParamLocal::NilFunction { local, .. } => {
                EvaluatedValueRef::Function(EvaluatedFunctionRef::Nil(self.nil_function(*local)))
            }
            ParamLocal::TupleFunction { local, .. } => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::Tuple(self.tuple_function(*local)),
            ),
            ParamLocal::GenericFunction(local) => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::Generic(&self.values.generic_functions[local.id().0]),
            ),
            ParamLocal::NeverFunction(local) => {
                EvaluatedValueRef::Function(EvaluatedFunctionRef::Never(self.never_function(local)))
            }
            ParamLocal::CustomFunction(local) => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::Custom(self.custom_function(local)),
            ),
            ParamLocal::ExternalFunction(local) => EvaluatedValueRef::Function(
                EvaluatedFunctionRef::External(self.external_function(local)),
            ),
            ParamLocal::ListFunction(local) => {
                EvaluatedValueRef::Function(self.list_function_ref(local))
            }
            ParamLocal::FunctionFunction(local) => EvaluatedValueRef::Function(match local {
                FunctionFunctionLocal::Core(local) => {
                    EvaluatedFunctionRef::CoreFunction(self.core_function_function(local))
                }
                FunctionFunctionLocal::External(local) => {
                    EvaluatedFunctionRef::ExternalFunction(self.external_function_function(local))
                }
            }),
        }
    }

    pub(in crate::runtime::graph) fn list_len(&self, local: &ListLocal) -> usize {
        match local {
            ListLocal::Parameter { .. } => 0,
            ListLocal::ParameterList { local, .. } => {
                self.values.parameter_list_lists[local.0].len()
            }
            ListLocal::Int { local, .. } => self.values.int_lists[local.0].values().len(),
            ListLocal::String { local, .. } => self.values.string_lists[local.0].values().len(),
            ListLocal::BitArray { local, .. } => {
                self.values.bit_array_lists[local.0].values().len()
            }
            ListLocal::UtfCodepoint { local, .. } => {
                self.values.utf_codepoint_lists[local.0].values().len()
            }
            ListLocal::Custom { local, .. } => self.values.custom_lists[local.0].values().len(),
            ListLocal::External { local, .. } => self.values.external_lists[local.0].values().len(),
            ListLocal::Float { local, .. } => self.values.float_lists[local.0].values().len(),
            ListLocal::Bool { local, .. } => self.values.bool_lists[local.0].values().len(),
            ListLocal::Nil { local, .. } => self.values.nil_lists[local.0].len(),
            ListLocal::Tuple { local, .. } => self.values.tuple_lists[local.0].values().len(),
            ListLocal::List { local, .. } => self.values.list_lists[local.0].values().len(),
            ListLocal::Function { local, .. } => self.values.function_lists[local.0].values().len(),
        }
    }

    fn list_value_ref(&self, local: &ListLocal) -> EvaluatedValueRef<'_> {
        let handle = match local {
            ListLocal::Parameter { local, .. } => {
                return EvaluatedValueRef::ParameterList(self.values.parameter_lists[local.0]);
            }
            ListLocal::ParameterList { local, .. } => {
                StoredListValueRef::ParameterList(&self.values.parameter_list_lists[local.0])
            }
            ListLocal::Int { local, .. } => {
                StoredListValueRef::Int(&self.values.int_lists[local.0])
            }
            ListLocal::String { local, .. } => {
                StoredListValueRef::String(&self.values.string_lists[local.0])
            }
            ListLocal::BitArray { local, .. } => {
                StoredListValueRef::BitArray(&self.values.bit_array_lists[local.0])
            }
            ListLocal::UtfCodepoint { local, .. } => {
                StoredListValueRef::UtfCodepoint(&self.values.utf_codepoint_lists[local.0])
            }
            ListLocal::Custom { local, .. } => {
                StoredListValueRef::Custom(&self.values.custom_lists[local.0])
            }
            ListLocal::External { local, .. } => {
                StoredListValueRef::External(&self.values.external_lists[local.0])
            }
            ListLocal::Float { local, .. } => {
                StoredListValueRef::Float(&self.values.float_lists[local.0])
            }
            ListLocal::Bool { local, .. } => {
                StoredListValueRef::Bool(&self.values.bool_lists[local.0])
            }
            ListLocal::Nil { local, .. } => {
                StoredListValueRef::Nil(&self.values.nil_lists[local.0])
            }
            ListLocal::Tuple { local, .. } => {
                StoredListValueRef::Tuple(&self.values.tuple_lists[local.0])
            }
            ListLocal::List { local, .. } => {
                StoredListValueRef::List(&self.values.list_lists[local.0])
            }
            ListLocal::Function { local, .. } => {
                StoredListValueRef::Function(&self.values.function_lists[local.0])
            }
        };
        EvaluatedValueRef::List(EvaluatedListRef::new(handle))
    }

    fn list_function_ref(&self, local: &ListFunctionLocal) -> EvaluatedFunctionRef<'_> {
        match local {
            ListFunctionLocal::Parameter { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.parameter_list_functions[local.0])
            }
            ListFunctionLocal::ParameterList { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.parameter_list_list_functions[local.0])
            }
            ListFunctionLocal::Int { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.int_list_functions[local.0])
            }
            ListFunctionLocal::String { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.string_list_functions[local.0])
            }
            ListFunctionLocal::BitArray { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.bit_array_list_functions[local.0])
            }
            ListFunctionLocal::UtfCodepoint { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.utf_codepoint_list_functions[local.0])
            }
            ListFunctionLocal::Custom { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.custom_list_functions[local.0])
            }
            ListFunctionLocal::External { local, .. } => {
                EvaluatedFunctionRef::ExternalList(&self.values.external_list_functions[local.0])
            }
            ListFunctionLocal::Float { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.float_list_functions[local.0])
            }
            ListFunctionLocal::Bool { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.bool_list_functions[local.0])
            }
            ListFunctionLocal::Nil { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.nil_list_functions[local.0])
            }
            ListFunctionLocal::Tuple { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.tuple_list_functions[local.0])
            }
            ListFunctionLocal::List { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.list_list_functions[local.0])
            }
            ListFunctionLocal::Function { local, .. } => {
                EvaluatedFunctionRef::List(&self.values.function_list_functions[local.0])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockEnvironment;
    use crate::execution_fixture::TestHost;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use crate::plan::execution::function::{TupleFunctionId, ValueFunctionEntry};
    use crate::plan::execution::graph::{IntLocalId, ParamLocal, TupleLocalId};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::ValueType;
    use crate::runtime::evaluated::{
        EvaluatedFunctionRef, EvaluatedValue, EvaluatedValueRef, value_refs_equal,
    };
    use crate::runtime::execution::Domain;
    use crate::runtime::graph::environment::RetainedValues;
    use crate::runtime::{HostCallOrigin, RuntimeListStorage};
    use crate::work_fixture::WorkComponent;
    use crate::{
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };
    use num_bigint::BigInt;
    use std::ptr;
    use std::sync::Arc;

    #[test]
    fn read_inputs_borrow_typed_slots_until_the_environment_is_consumed() {
        let mut inputs = RetainedValues::empty();
        inputs.push_int(BigInt::from(1_u64) << 160);
        inputs.push_tuple(vec![EvaluatedValue::String("kept".into())]);
        let environment = BlockEnvironment::from_retained(inputs);
        assert!(matches!(
            environment.value_ref(&ParamLocal::Int(IntLocalId(0))),
            EvaluatedValueRef::Int(number) if ptr::eq(number, &environment.values.ints[0])
        ));
        let borrowed = environment.value_ref(&ParamLocal::Tuple {
            local: TupleLocalId(0),
            type_: vec![ValueType::String].into(),
        });
        assert!(matches!(
            &borrowed,
            EvaluatedValueRef::Tuple(fields)
                if ptr::eq(fields.as_ptr(), environment.values.tuples[0].as_ptr())
        ));
        let retained = borrowed.retain();
        drop(environment);
        assert_eq!(
            retained,
            EvaluatedValue::Tuple(vec![EvaluatedValue::String("kept".into())])
        );
    }

    #[test]
    fn source_values_project_from_every_typed_input_slot_and_retain_their_identity() {
        for (type_, sample) in [
            ("Int", "1208925819614629174706176"),
            ("Float", "1.5"),
            ("String", "\"retained string\""),
            ("BitArray", "<<1:3>>"),
            (
                "UtfCodepoint",
                "{ let assert <<c:utf8_codepoint>> = <<65>> c }",
            ),
            ("Box", "Box(42)"),
            ("work.Work(Int)", "work.ready(42)"),
            ("Bool", "True"),
            ("Nil", "Nil"),
            ("#(Int, String)", "#(42, \"field\")"),
            ("List(a)", "[]"),
            ("List(List(a))", "[[]]"),
            ("List(Int)", "[42]"),
            ("List(String)", "[\"field\"]"),
            ("List(BitArray)", "[<<1:3>>]"),
            (
                "List(UtfCodepoint)",
                "{ let assert <<c:utf8_codepoint>> = <<65>> [c] }",
            ),
            ("List(Box)", "[Box(42)]"),
            ("List(work.Work(Int))", "[work.ready(42)]"),
            ("List(Float)", "[1.5]"),
            ("List(Bool)", "[True]"),
            ("List(Nil)", "[Nil]"),
            ("List(#(Int))", "[#(42)]"),
            ("List(List(Int))", "[[42]]"),
            ("List(fn() -> Int)", "[fn() { 42 }]"),
            ("fn(a) -> a", "identity"),
            ("fn(Int) -> a", "stop"),
            ("fn() -> Int", "fn() { 42 }"),
            ("fn() -> Float", "fn() { 1.5 }"),
            ("fn() -> String", "fn() { captured }"),
            ("fn() -> BitArray", "fn() { <<1:3>> }"),
            (
                "fn() -> UtfCodepoint",
                "fn() { let assert <<c:utf8_codepoint>> = <<65>> c }",
            ),
            ("fn() -> Box", "fn() { Box(42) }"),
            ("fn() -> work.Work(Int)", "fn() { work.ready(42) }"),
            ("fn() -> Bool", "fn() { True }"),
            ("fn() -> Nil", "fn() { Nil }"),
            ("fn() -> #(Int)", "fn() { #(42) }"),
            ("fn() -> List(a)", "fn() { [] }"),
            ("fn() -> List(List(a))", "fn() { [[]] }"),
            ("fn() -> List(Int)", "fn() { [42] }"),
            ("fn() -> List(String)", "fn() { [captured] }"),
            ("fn() -> List(BitArray)", "fn() { [<<1:3>>] }"),
            (
                "fn() -> List(UtfCodepoint)",
                "fn() { let assert <<c:utf8_codepoint>> = <<65>> [c] }",
            ),
            ("fn() -> List(Box)", "fn() { [Box(42)] }"),
            ("fn() -> List(work.Work(Int))", "fn() { [work.ready(42)] }"),
            ("fn() -> List(Float)", "fn() { [1.5] }"),
            ("fn() -> List(Bool)", "fn() { [True] }"),
            ("fn() -> List(Nil)", "fn() { [Nil] }"),
            ("fn() -> List(#(Int))", "fn() { [#(42)] }"),
            ("fn() -> List(List(Int))", "fn() { [[42]] }"),
            ("fn() -> List(fn() -> Int)", "fn() { [fn() { 42 }] }"),
            ("fn() -> fn() -> Int", "fn() { fn() { 42 } }"),
            (
                "fn() -> fn() -> work.Work(Int)",
                "fn() { fn() { work.ready(42) } }",
            ),
        ] {
            let source = format!(
                r#"
import fixture/work
pub type Box {{ Box(Int) }}
fn identity(value) {{ value }}
fn stop(_value: Int) -> a {{ panic }}
fn input(value: {type_}) {{ #(value) }}
pub fn main() {{
  let captured = "captured string"
  let original = {sample}
  let returned = input(original)
  #(input, returned.0)
}}
"#
            );
            let typed = compile_typed_host_program(
                "application",
                "main",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "fixture/work.gleam",
                            WorkComponent::SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("main", "main.gleam", &source)],
                    ),
                ],
                HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                    .unwrap(),
            )
            .expect(&source);
            let mut execution =
                HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
            let (plan, stores, captures) = execution.parts_mut();
            let host = TestHost::default();
            let mut state = ();
            assert!(ptr::eq(Profile::component_state(&mut state), &state));
            let mut echo = Vec::new();
            let domain = Domain::new(
                Arc::clone(plan),
                &host,
                &mut state,
                stores,
                &mut echo,
                captures.clone(),
                Domain::<Profile>::DEFAULT_BUDGET,
            );
            let context = domain.context();
            host.block_on(domain.drive(async {
                let values = context
                    .call(
                        TupleFunctionId(0),
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(values.len(), 2, "{source}");
                let original = &values[1];
                let nested = EvaluatedValueRef::from(original).retain();
                assert_eq!(&nested, original, "{source}");
                assert!(matches!(
                    EvaluatedValueRef::from(&values[0]),
                    EvaluatedValueRef::Function(EvaluatedFunctionRef::Tuple(input)) if matches!(
                        plan.tuple_function(input.runtime_id()),
                        ValueFunctionEntry::Graph(function) if {
                            let graph = function.body().block_graph();
                            let parameters = graph.block(graph.entry()).params();
                            assert_eq!(parameters.len(), 1, "{source}");
                            let mut inputs = RetainedValues::empty();
                            inputs.push_evaluated(original.clone());
                            let environment = BlockEnvironment::from_retained(inputs);
                            let local = parameters[0].local();
                            let borrowed = environment.value_ref(local);
                            let owned = environment.value(local);
                            let owned_ref = EvaluatedValueRef::from(&owned);
                            let storage = RuntimeListStorage::default();
                            assert!(value_refs_equal(&storage, &borrowed, &borrowed), "{source}");
                            assert!(value_refs_equal(&storage, &borrowed, &owned_ref), "{source}");
                            assert!(value_refs_equal(&storage, &owned_ref, &borrowed), "{source}");
                            if let ParamLocal::List(local) = local {
                                assert_eq!(environment.list_len(local), usize::from(type_ != "List(a)"), "{source}");
                            }
                            let retained = borrowed.retain();
                            assert_eq!(&retained, original, "{source}");
                            drop(environment);
                            &retained == original
                        }
                    )
                ), "{source}");
            }))
            .unwrap();
            assert!(echo.is_empty());
        }
    }

    struct Profile;

    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }

    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }

    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
}
