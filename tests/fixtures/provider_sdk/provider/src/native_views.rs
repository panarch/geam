use super::{Component, Provider};
use geam::host::native::{NativeCall, NativeRules};
use geam::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostFailure,
    HostProviderModule, HostRegistrationError, HostTypeIndex0, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostValue,
};

type Source = HostTypeParameter<1>;
type Return = HostTypeParameter<0>;
type One<Type> = HostTypeList<Type, HostTypeListEnd>;

pub(super) fn register<Profile: HostComponentProfile<Component>>(
    provider: HostProviderModule<Profile>,
) -> Result<HostProviderModule<Profile>, HostRegistrationError> {
    provider
        .with_native_function::<Provider, (Source,), Return, One<Return>, _>(
            "view",
            NativeRules::default().retained_views::<One<Source>>(),
            view::<Profile>,
        )
        .and_then(|provider| {
            provider.with_native_function::<Provider, (Source,), Return, One<Return>, _>(
                "exact",
                NativeRules::default(),
                view::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<Provider, (Return, Source), bool, _>(
                "same_native",
                same::<Profile>,
            )
        })
}

fn view<'call, Profile: HostComponentProfile<Component>>(
    mut call: NativeCall<'call, Profile, Provider, Return, One<Return>>,
    source: HostValue<'call, Source>,
) -> Result<HostCallCompletion<'call, Return>, HostCallError> {
    let source = call.source::<Source>(source);
    let value = call
        .convert::<HostTypeIndex0>(&source)
        .ok_or_else(|| HostFailure::new("incompatible retained view"))?;
    Ok(call.finish(value))
}

fn same<'call, Profile: HostComponentProfile<Component>>(
    call: HostCall<'call, Profile, Provider, bool>,
    left: HostValue<'call, Return>,
    right: HostValue<'call, Source>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let left = call.native_value::<Return>(left);
    let right = call.native_value::<Source>(right);
    let equal = call.native_equal(&left, &right);
    let same_hash = call.native_hash(&left) == call.native_hash(&right);
    Ok(call.return_value(equal && same_hash))
}
