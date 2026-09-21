use crate::runtime::RetainedCallable;
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedExternalFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedIntFunction,
    EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction,
    EvaluatedTupleFunction, EvaluatedUtfCodepointFunction,
};
use crate::runtime::function::InvocableFunctionValue;

/// An invocable column selected by the admitted embedding signature.
#[derive(Clone)]
pub(crate) struct EmbeddingCallable(pub(in crate::runtime) RetainedCallable);

macro_rules! callable_values {
    ($($kind:ident($type:ty)),+ $(,)?) => {
        enum CallableRef<'value> {
            $($kind(&'value $type),)+
        }

        #[derive(Default)]
        pub(crate) struct CallableView<'value> {
            value: Option<CallableRef<'value>>,
        }

        impl<'value> CallableView<'value> {
            pub(in crate::runtime) fn new(value: &'value EvaluatedFunctionValue) -> Self {
                Self {
                    value: match value.kind() {
                        $(EvaluatedFunctionValueKind::$kind(value) => Some(CallableRef::$kind(value)),)+
                        EvaluatedFunctionValueKind::Generic(_) => None,
                    },
                }
            }

            pub(crate) fn retain(&self) -> EmbeddingCallable {
                // Like the scalar row columns, this slot is read only by its
                // statically selected codec. Symbolic functions have no slot.
                let value = match &self.value.as_slice()[0] {
                    $(CallableRef::$kind(value) => InvocableFunctionValue::$kind((*value).clone()),)+
                };
                EmbeddingCallable(RetainedCallable::new(value))
            }
        }

        impl EmbeddingCallable {
            pub(in crate::runtime) fn from_value(value: EvaluatedFunctionValue) -> Option<Self> {
                let value = match value.into_kind() {
                    $(EvaluatedFunctionValueKind::$kind(value) => InvocableFunctionValue::$kind(value),)+
                    EvaluatedFunctionValueKind::Generic(_) => return None,
                };
                Some(Self(RetainedCallable::new(value)))
            }
        }
    };
}

callable_values!(
    Never(EvaluatedNeverFunction),
    Int(EvaluatedIntFunction),
    Float(EvaluatedFloatFunction),
    String(EvaluatedStringFunction),
    BitArray(EvaluatedBitArrayFunction),
    UtfCodepoint(EvaluatedUtfCodepointFunction),
    Custom(EvaluatedCustomFunction),
    External(EvaluatedExternalFunction),
    Bool(EvaluatedBoolFunction),
    Nil(EvaluatedNilFunction),
    Tuple(EvaluatedTupleFunction),
    List(EvaluatedListFunction),
    Function(EvaluatedFunctionFunction),
);

impl EmbeddingCallable {
    pub(crate) fn construct(
        construction: &crate::plan::execution::host::HostCallableConstruction,
        inputs: Option<crate::runtime::RetainedInputs>,
        storage: &crate::runtime::CaptureStorage,
    ) -> Self {
        let captures = storage.capture(inputs.map_or_else(Vec::new, |inputs| {
            inputs.into_retained().into_captures(&construction.captures)
        }));
        Self(RetainedCallable::new(InvocableFunctionValue::closure(
            construction.target.clone(),
            construction
                .parameters
                .iter()
                .map(|slot| slot.local().clone())
                .collect(),
            captures,
            construction.type_.clone(),
        )))
    }

    pub(crate) async fn invoke<Profile: crate::host::HostProfile>(
        &self,
        context: &crate::runtime::execution::EntryContext<Profile>,
        inputs: crate::runtime::CallbackInputs,
    ) -> Result<super::EmbeddingOutput, crate::embedding::CallError> {
        context
            .invoke(
                self.0.clone(),
                crate::runtime::HostCallOrigin::Entry,
                inputs,
            )
            .await
            .map_err(|_| crate::embedding::CallError::Cancelled)?
            .map(super::EmbeddingOutput::from_value)
            .map_err(crate::embedding::CallError::Execution)
    }
}

impl super::EmbeddingInputValue for EmbeddingCallable {
    type ListType = crate::plan::execution::type_::FunctionListTypeId;

    fn into_input(self) -> super::EmbeddingInput {
        super::EmbeddingInput::function(self.0.into_evaluated())
    }

    fn into_list(
        type_: Self::ListType,
        values: impl ExactSizeIterator<Item = Self>,
        storage: &super::EmbeddingInputStorage,
    ) -> super::EmbeddingListInput {
        super::EmbeddingListInput(
            storage
                .lists()
                .function(
                    type_,
                    values.map(|value| value.0.into_evaluated()).collect(),
                )
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{FunctionType, LibraryEntry, LibraryValueType, ValueType};
    use crate::runtime::{CallbackInputs, RetainedInputs, execution::Domain};
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};
    use std::sync::Arc;

    #[test]
    fn diverging_functions_have_callable_views_but_symbolic_functions_do_not() {
        use crate::plan::execution::function::{FunctionReturnFamily, TupleFunctionId};
        use crate::runtime::evaluated::EvaluatedValue;
        let plan = crate::runtime::plan_src(
            r#"
pub type Empty
fn stop() -> Empty { panic as "stop" }
fn identity(value) { value }
pub fn main() { #(stop, identity, 42) }
"#,
        );
        let mut echo = Vec::new();
        let mut state = crate::runtime::state::RuntimeState::new(&mut echo);
        let values = crate::runtime::function::run_tuple(
            &plan,
            &mut state,
            TupleFunctionId(0),
            crate::runtime::HostCallOrigin::Entry,
            crate::runtime::RetainedValues::empty(),
        )
        .unwrap();
        let functions = values
            .into_iter()
            .filter_map(|value| match value {
                EvaluatedValue::Function(function) => Some(function),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(functions.len(), 2);
        let diverging = &functions[0];
        let symbolic = &functions[1];
        assert_eq!(diverging.kind().family(), FunctionReturnFamily::Never);
        assert_eq!(symbolic.kind().family(), FunctionReturnFamily::Generic);
        let view = super::CallableView::new(diverging);
        assert_eq!(view.retain().0.into_evaluated(), diverging.clone());
        assert_eq!(
            super::EmbeddingCallable::from_value(diverging.clone())
                .unwrap()
                .0
                .into_evaluated(),
            diverging.clone()
        );
        assert!(super::CallableView::new(symbolic).value.is_none());
        assert!(super::EmbeddingCallable::from_value(symbolic.clone()).is_none());
        assert!(echo.is_empty());
    }

    #[test]
    fn a_retained_callable_cannot_restart_after_its_original_domain_closes() {
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
pub fn main() { fn() { echo "called" 42 } }
"#,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let plan = crate::planner::plan_host_library_program(typed).unwrap();
        let root = plan
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap()
            .signature()
            .id();
        let entry = LibraryEntry::new(
            root,
            LibraryValueType::Function(FunctionType::new(vec![], ValueType::Int)),
            vec![],
            vec![],
        );
        let (program, entries, _) =
            crate::plan::execution::HostedProgram::from_library_plan(plan, entry, vec![]).unwrap();
        let plan = Arc::new(program);
        let host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut stores = ();
        let mut echoes = Vec::new();
        let domain = Domain::new(
            plan,
            &host,
            &mut state,
            &mut stores,
            &mut echoes,
            crate::runtime::CaptureStorage::default(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        let retained = host
            .block_on(domain.drive(async {
                let mut output = entries.functions[0]
                    .function
                    .call(&context, RetainedInputs::empty())
                    .await
                    .unwrap();
                let callable = output.take_function();
                let mut output = callable
                    .invoke(&context, CallbackInputs::new())
                    .await
                    .unwrap();
                assert_eq!(output.take_int(), 42.into());
                callable
            }))
            .unwrap();
        assert_eq!(
            host.block_on(retained.invoke(&context, CallbackInputs::new()))
                .err(),
            Some(crate::embedding::CallError::Cancelled)
        );
        assert_eq!(
            echoes
                .iter()
                .map(|echo| echo.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["\"called\""]
        );
    }
    #[test]
    fn direct_and_diverging_calls_reject_foreign_captures_before_source_effects() {
        use crate::plan::execution::function::TupleFunctionId;
        use crate::runtime::{BorrowedValue, EvaluatedValue, HostCallOrigin, RetainedValues};
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
pub type Empty { Again(Empty) }
fn apply(callback: fn() -> Empty) -> Empty { callback() }
fn direct_stop(value: Int) -> value { panic as "direct stopped" }
pub fn main() {
  let offset = 40
  #(fn() { echo "entered" offset + 2 },
    fn() -> Empty { echo "stopped" panic as "stopped" }, apply,
    fn() -> Int { panic as "integer stopped" },
    fn() -> Int { let _ = direct_stop(1) 0 })
}
"#,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = ();
        let mut echo = Vec::new();
        let mut other_state = ();
        let mut other_stores = ();
        let mut other_echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
        );
        let original = domain.context();
        host.block_on(domain.drive(async {
            let values = original
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 5);
            let integer = BorrowedValue::from_value(&values[0]).function();
            let stopped = BorrowedValue::from_value(&values[1]).function();
            let apply = BorrowedValue::from_value(&values[2]).function();
            let failed_integer = BorrowedValue::from_value(&values[3]).function();
            let direct = BorrowedValue::from_value(&values[4]).function();
            assert_eq!(
                direct
                    .invoke(&original, CallbackInputs::new())
                    .await
                    .err()
                    .unwrap()
                    .to_string(),
                "panic: direct stopped"
            );
            assert_eq!(
                failed_integer
                    .invoke(&original, CallbackInputs::new())
                    .await
                    .err()
                    .unwrap()
                    .to_string(),
                "panic: integer stopped"
            );
            assert_eq!(
                integer
                    .invoke(&original, CallbackInputs::new())
                    .await
                    .unwrap()
                    .take_int(),
                42.into()
            );
            let other = Domain::new(
                Arc::clone(plan),
                &host,
                &mut other_state,
                &mut other_stores,
                &mut other_echo,
                captures.clone(),
                Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
            );
            let foreign = other.context();
            other
                .drive(async {
                    assert_eq!(
                        integer.invoke(&foreign, CallbackInputs::new()).await.err(),
                        Some(crate::embedding::CallError::Cancelled)
                    );
                    let foreign_values = foreign
                        .call(
                            TupleFunctionId(0),
                            HostCallOrigin::Entry,
                            RetainedValues::empty(),
                        )
                        .await
                        .unwrap()
                        .unwrap();
                    let foreign_apply = BorrowedValue::from_value(&foreign_values[2]).function();
                    let mut inputs = CallbackInputs::new();
                    inputs.push_value(EvaluatedValue::Function(stopped.0.clone().into_evaluated()));
                    assert_eq!(
                        foreign_apply.invoke(&foreign, inputs).await.err(),
                        Some(crate::embedding::CallError::Cancelled)
                    );
                })
                .await
                .unwrap();
            let mut inputs = CallbackInputs::new();
            inputs.push_value(EvaluatedValue::Function(stopped.0.into_evaluated()));
            assert_eq!(
                apply
                    .invoke(&original, inputs)
                    .await
                    .err()
                    .unwrap()
                    .to_string(),
                "panic: stopped"
            );
            assert_eq!(
                integer
                    .invoke(&original, CallbackInputs::new())
                    .await
                    .unwrap()
                    .take_int(),
                42.into()
            );
        }))
        .unwrap();
        assert!(other_echo.is_empty());
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["\"entered\"", "\"stopped\"", "\"entered\""]
        );
    }
}
