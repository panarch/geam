use super::callable_declarations::{
    Add, AddFunction, Constant, ConstantFunction, End, MAKE_ADDER, MAKE_CONSTANT, MAKE_STOP, One,
    PRODUCE, Stop, T, U, WRAP, Wrap, Wrapped,
};
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostCaptures, HostConstructions,
    HostCreatedFunction, HostFailure, HostProvider, HostProviderModule, HostProviderSet,
    HostTypeIndex0, HostValue, StatelessHostProfile,
};
use num_bigint::BigInt;

struct Provider;
impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

fn constant<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, T>,
    captures: HostCaptures<'call, One<T>>,
    _: HostConstructions<'call, End>,
) -> Result<HostCallCompletion<'call, T>, HostCallError> {
    let (value, ()) = call.captures(captures);
    Ok(call.return_value(value))
}

fn add<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    captures: HostCaptures<'call, One<BigInt>>,
    _: HostConstructions<'call, End>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let (base, ()) = call.captures(captures);
    Ok(call.return_value(super::pricing::add(base, value)))
}

fn make_constant<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, ConstantFunction<T>>,
    constructions: HostConstructions<'call, One<HostCreatedFunction<Constant<T>>>>,
    value: HostValue<'call, T>,
) -> Result<HostCallCompletion<'call, ConstantFunction<T>>, HostCallError> {
    let callback = call.construct_function(constructions.at::<HostTypeIndex0>(), (value, ()));
    Ok(call.return_value(callback))
}

fn make_adder<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, AddFunction>,
    constructions: HostConstructions<'call, One<HostCreatedFunction<Add>>>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, AddFunction>, HostCallError> {
    let callback = call.construct_function(constructions.at::<HostTypeIndex0>(), (value, ()));
    Ok(call.return_value(callback))
}

fn make_wrapper<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, Wrapped<T, U>>,
    constructions: HostConstructions<'call, One<HostCreatedFunction<Wrap<T, U>>>>,
    callback: geam_core::HostCallable<'call, One<T>, U>,
) -> Result<HostCallCompletion<'call, Wrapped<T, U>>, HostCallError> {
    let wrapped = call.construct_function(constructions.at::<HostTypeIndex0>(), (callback, ()));
    Ok(call.return_value(wrapped))
}

fn wrap<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, U>,
    captures: HostCaptures<'call, One<Wrapped<T, U>>>,
    constructions: HostConstructions<'call, End>,
    value: HostValue<'call, T>,
) -> Result<geam_core::HostCallContinuation<'call, U>, HostCallError> {
    type Owned<Type> =
        geam_core::provider::Value<Type, geam_core::provider::ProviderValueContext<Type>>;
    let (callback, ()) = call.captures(captures);
    let callback = call.owned_callable(callback, &constructions);
    let input = Owned::<T>::from_host(&call, value);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let output = callback
                .invoke(
                    &context,
                    move |mut call, _| (input.into_host(&mut call), ()),
                    |call, _, value| Ok(Owned::<U>::from_host(&call, value)),
                )
                .await?;
            Ok(geam_core::HostOwnedCompletion::new(move |mut call, _| {
                let value = output.into_host(&mut call);
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn make_stop<'call>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, ConstantFunction<T>>,
    constructions: HostConstructions<'call, One<HostCreatedFunction<Stop<T>>>>,
) -> Result<HostCallCompletion<'call, ConstantFunction<T>>, HostCallError> {
    let callback = call.construct_function(constructions.at::<HostTypeIndex0>(), ());
    Ok(call.return_value(callback))
}

fn stop<'call>(
    _: HostCall<'call, StatelessHostProfile, Provider, T>,
    _: HostCaptures<'call, End>,
    _: HostConstructions<'call, End>,
) -> Result<HostCallCompletion<'call, T>, HostCallError> {
    Err(HostFailure::new("callable stopped").into())
}

fn produce<'call>(
    _: HostCall<'call, StatelessHostProfile, Provider, T>,
    _: HostConstructions<'call, End>,
) -> Result<HostCallCompletion<'call, T>, HostCallError> {
    Err(HostFailure::new("producer stopped").into())
}

pub(crate) fn implementations() -> HostProviderSet<StatelessHostProfile> {
    HostProviderSet::from_providers([HostProviderModule::new("support", "support")
        .unwrap()
        .with_declared_function::<Provider, _, _, _, _>(MAKE_CONSTANT, make_constant)
        .unwrap()
        .with_declared_function::<Provider, _, _, _, _>(MAKE_ADDER, make_adder)
        .unwrap()
        .with_declared_function::<Provider, _, _, _, _>(WRAP, make_wrapper)
        .unwrap()
        .with_declared_function::<Provider, _, _, _, _>(MAKE_STOP, make_stop)
        .unwrap()
        .with_declared_function::<Provider, _, _, _, _>(PRODUCE, produce)
        .unwrap()
        .with_callable::<Provider, Stop<T>, (), _>(stop)
        .unwrap()
        .with_callable::<Provider, Constant<T>, (), _>(constant)
        .unwrap()
        .with_callable::<Provider, Add, (BigInt,), _>(add)
        .unwrap()
        .with_resumable_callable::<Provider, Wrap<T, U>, (T,), _>(wrap)
        .unwrap()])
    .unwrap()
}
