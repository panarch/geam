use super::{Decode, NativeCall};
use crate::host::{
    HostCall, HostCallError, HostCallRuntime, HostFailure, HostProfile, HostProvider,
    HostScopedValue, HostType, HostTypeSequence, HostValueArgumentSlot, NativeViewBinding,
};
use crate::runtime::execution::Continuation;
use crate::runtime::{HostCallOrigin, NativeValue};
use std::sync::Arc;

pub(in crate::host) fn start<Profile, Provider, Return, Targets>(
    runtime: &mut dyn HostCallRuntime<Profile>,
    binding: &NativeViewBinding,
    rules: Arc<[Box<Decode<Profile, Provider, Return>>]>,
) -> Result<Continuation, HostCallError>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    let source = runtime.function_token(runtime.capture_tokens()[0]);
    let source = runtime.callable(source);
    let inputs = (0..binding.view.arguments.len())
        .map(|index| {
            let token = runtime.value(HostValueArgumentSlot::new(index));
            NativeValue::from_stored(runtime.retain_stored(HostScopedValue::Value(token)))
        })
        .collect::<Vec<_>>();
    let execution = runtime.execution();
    let binding = binding.clone();
    let origin = HostCallOrigin::host(binding.codec.function());
    Ok(Continuation::new(async move {
        let codec = binding.codec.clone();
        let input_origin = origin.clone();
        let input_rules = Arc::clone(&rules);
        let view = binding.view.clone();
        let converted = execution
            .with_codec(codec, input_origin, move |runtime| {
                let scope = runtime.codec_scope();
                let conversions = scope.function().constructions().natives();
                let mut call = NativeCall::<Profile, Provider, Return, Targets>::new(
                    HostCall::new(runtime),
                    input_rules,
                );
                let inputs = view
                    .arguments
                    .iter()
                    .zip(&inputs)
                    .map(|(id, input)| call.convert_value(conversions, *id, input))
                    .collect::<Option<Vec<_>>>()?;
                Some(call.call.runtime.callback_inputs(inputs.into_boxed_slice()))
            })
            .await?;
        let Some(inputs) = converted else {
            return execution
                .fail_native(
                    HostCallError::from(HostFailure::new(
                        "native function view input does not match its source signature",
                    ))
                    .into(),
                    binding.codec,
                    origin,
                )
                .await;
        };
        let returned = match execution.invoke(source, origin.clone(), inputs).await? {
            Ok(value) => NativeValue::from_stored(value),
            Err(error) => return Ok(Err(error)),
        };
        let codec = binding.codec.clone();
        let result_origin = origin.clone();
        let result = binding.view.return_;
        let converted = execution
            .with_codec(codec, result_origin, move |runtime| {
                let scope = runtime.codec_scope();
                let conversions = scope.function().constructions().natives();
                let mut call = NativeCall::<Profile, Provider, Return, Targets>::new(
                    HostCall::new(runtime),
                    rules,
                );
                let value = call.convert_value(conversions, result, &returned)?;
                Some(call.call.runtime.retain_stored(value))
            })
            .await?;
        match converted {
            Some(value) => Ok(Ok(value)),
            None => {
                execution
                    .fail_native(
                        HostCallError::from(HostFailure::new(
                            "native function view result does not match its target signature",
                        ))
                        .into(),
                        binding.codec,
                        origin,
                    )
                    .await
            }
        }
    }))
}
