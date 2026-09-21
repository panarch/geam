use super::{ProviderConstructionRequirements, ProviderConstructions, ProviderNoConstructions};
use crate::{HostCall, HostCallError, HostProfile, HostProvider, HostType, HostTypeListEnd};
use std::marker::PhantomData;

/// Selects a native callable declaration for an explicitly authorized provider call.
///
/// Receive this parameter with `#[geam::factory]` and pass it to `Call::create`.
/// The declaration alone grants no construction permission.
///
/// ```compile_fail
/// use geam_core::{HostProfile, HostProvider, HostType};
/// use geam_core::provider::{Call, Factory, ProviderActiveCall, ProviderFactoryCodec};
/// fn unregistered<'call, P, H, R, D>(
///     call: &mut Call<H::State, ProviderActiveCall<'call, P, H, R>>,
///     factory: &Factory<D>, captures: D::Captures,
/// ) where P: HostProfile, H: HostProvider<P>, R: HostType, D: ProviderFactoryCodec<P> {
///     let _ = call.create(factory, captures);
/// }
/// ```
pub struct Factory<Declaration> {
    declaration: PhantomData<fn() -> Declaration>,
}

impl<Declaration> Factory<Declaration> {
    #[doc(hidden)]
    pub fn declaration() -> Self {
        Self {
            declaration: PhantomData,
        }
    }
}

/// The exact construction tree owned by one generated provider function.
#[doc(hidden)]
pub trait ProviderFactoryBindings {
    type Requirements: ProviderConstructionRequirements;
    type CaptureMode;
}

/// Selects a declared factory's subtree without granting any additional capability.
#[doc(hidden)]
pub trait ProviderFactoryBinding<Declaration, Requirements>: ProviderFactoryBindings
where
    Requirements: ProviderConstructionRequirements,
{
    fn select<'call>(
        constructions: &ProviderConstructions<'call, Self::Requirements>,
    ) -> ProviderConstructions<'call, Requirements>;
}

/// Declaration-owned capture conversion and native function construction.
#[doc(hidden)]
pub trait ProviderFactoryCodec<Profile: HostProfile, Mode = ProviderOwnedCaptures> {
    type Requirements: ProviderConstructionRequirements;
    type Captures;
    type Output;

    fn create<'call, Provider, Return>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::Requirements>,
        captures: Self::Captures,
    ) -> Result<Self::Output, HostCallError>
    where
        Provider: HostProvider<Profile>,
        Return: HostType;
}

#[doc(hidden)]
pub struct ProviderNoFactories;

/// Capture inputs received during an immediate provider call.
#[doc(hidden)]
pub struct ProviderImmediateCaptures;

/// Capture inputs already owned by a resumable provider operation.
#[doc(hidden)]
pub struct ProviderOwnedCaptures;

impl ProviderFactoryBindings for ProviderNoFactories {
    type Requirements = ProviderNoConstructions;
    type CaptureMode = ProviderOwnedCaptures;
}

pub(super) type FactoryConstructions<Bindings> =
    <<Bindings as ProviderFactoryBindings>::Requirements as ProviderConstructionRequirements>::Types<HostTypeListEnd>;

impl<Requirements: ProviderConstructionRequirements> ProviderConstructions<'_, Requirements> {
    /// Converts owned factory captures using the declaration's original provider.
    /// Neither the reborrowed call nor its exact proof can escape this operation.
    #[doc(hidden)]
    pub fn with_call<Profile, Provider, Return, CodecProvider, Output>(
        &self,
        call: &mut HostCall<'_, Profile, Provider, Return>,
        operation: impl for<'codec> FnOnce(
            HostCall<'codec, Profile, CodecProvider, ()>,
            ProviderConstructions<'codec, Requirements>,
        ) -> Output,
    ) -> Output
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostType,
        CodecProvider: HostProvider<Profile>,
    {
        let host = self.host();
        call.with_codec(&host, |call, proof| {
            operation(call, ProviderConstructions::new(&proof))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Factory, ProviderFactoryBinding, ProviderFactoryBindings, ProviderFactoryCodec};
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::provider::{
        Call, ProviderActiveCall, ProviderConstruction, ProviderConstructionIndex0,
        ProviderConstructionIndexNext, ProviderConstructionList, ProviderConstructionRequirements,
        ProviderConstructions, ProviderExecutionCall, ProviderNoConstructions,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema,
        HostCaptures, HostConstructions, HostCreatedFunction, HostFunctionType, HostOwnedCallable,
        HostOwnedCompletion, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
        HostReturns, HostTupleType, HostType, HostTypeList, HostTypeListEnd, ModuleSource,
        PackageSource,
    };
    use num_bigint::BigInt;

    type End = HostTypeListEnd;
    type One<Type> = HostTypeList<Type, End>;
    type Function = HostFunctionType<One<BigInt>, BigInt>;
    type Pair = HostTupleType<HostTypeList<Function, One<Function>>>;
    type Needed = ProviderConstruction<HostCreatedFunction<Add<100>>>;
    type Requirements = ProviderConstructionList<
        ProviderConstruction<HostCreatedFunction<Add<10>>>,
        ProviderConstructionList<Needed, ProviderNoConstructions>,
    >;
    type Constructions = <Requirements as ProviderConstructionRequirements>::Types<End>;
    type Retained = HostOwnedCallable<Profile, Native, One<BigInt>, BigInt, End>;

    struct Profile;
    #[derive(Default)]
    struct State {
        consumer: (),
        creations: usize,
        effects: Vec<usize>,
    }
    impl HostProfile for Profile {
        type RunState = State;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    struct Native;
    impl HostProvider<Profile> for Native {
        type State = State;
        fn project(state: &mut State) -> &mut State {
            state
        }
    }
    struct Consumer;
    impl HostProvider<Profile> for Consumer {
        type State = ();
        fn project(state: &mut State) -> &mut () {
            &mut state.consumer
        }
    }
    struct Bindings;
    impl ProviderFactoryBindings for Bindings {
        type Requirements = Requirements;
        type CaptureMode = super::ProviderOwnedCaptures;
    }
    impl ProviderFactoryBinding<Add<100>, Needed> for Bindings {
        fn select<'call>(
            proof: &ProviderConstructions<'call, Requirements>,
        ) -> ProviderConstructions<'call, Needed> {
            proof.select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>()
        }
    }
    struct Add<const OFFSET: usize>;
    impl<const OFFSET: usize> HostCallableSchema for Add<OFFSET> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = if OFFSET == 10 { "first" } else { "second" };
        type Arguments = One<BigInt>;
        type Return = BigInt;
        type Captures = One<BigInt>;
        type Constructions = End;
        type Completion = HostReturns;
    }
    impl ProviderFactoryCodec<Profile> for Add<100> {
        type Requirements = Needed;
        type Captures = (BigInt,);
        type Output = Retained;
        fn create<'call, Caller, Return>(
            call: &mut HostCall<'call, Profile, Caller, Return>,
            proof: &ProviderConstructions<'call, Needed>,
            captures: (BigInt,),
        ) -> Result<Retained, HostCallError>
        where
            Caller: HostProvider<Profile>,
            Return: HostType,
        {
            proof.with_call::<_, _, _, Native, _>(call, |mut call, proof| {
                call.state().creations += 1;
                let value = call.construct_function(proof.token(), (captures.0, ()));
                Ok(call.owned_callable(value, &HostConstructions::<End>::new()))
            })
        }
    }
    fn add<'call, const OFFSET: usize>(
        mut call: HostCall<'call, Profile, Native, BigInt>,
        captures: HostCaptures<'call, One<BigInt>>,
        _: HostConstructions<'call, End>,
        argument: BigInt,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let (captured, ()) = call.captures(captures);
        call.state().effects.push(OFFSET);
        Ok(call.return_value(captured + argument + OFFSET))
    }
    fn make<'call>(
        call: HostCall<'call, Profile, Consumer, Function>,
        proof: HostConstructions<'call, Constructions>,
    ) -> Result<HostCallCompletion<'call, Function>, HostCallError> {
        let mut call =
            Call::<_, ProviderActiveCall<'_, _, _, _, Bindings>>::from_host_call_with_factories(
                call,
                ProviderConstructions::new(&proof),
            );
        let callback = call
            .create(&Factory::<Add<100>>::declaration(), (7.into(),))
            .unwrap();
        let mut call = call.into_host_call();
        let callback = callback.restore(&mut call).unwrap();
        Ok(call.return_value(callback))
    }
    fn make_owned<'call>(
        call: HostCall<'call, Profile, Consumer, Pair>,
        proof: HostConstructions<'call, Constructions>,
        cancel: bool,
    ) -> Result<HostCallContinuation<'call, Pair>, HostCallError> {
        Ok(call.resume(proof, move |context| Box::pin(async move {
            if cancel {
                assert!(context.execution().unit().expect("active native invocation").cancel());
            }
            let mut call = Call::<_, ProviderExecutionCall<'_, _, _, (), Bindings>>::from_execution_context_with_factories(context);
            let first = call.create(&Factory::<Add<100>>::declaration(), (9.into(),)).await.unwrap();
            let second = call.with_call(|call| call.create(&Factory::<Add<100>>::declaration(), (11.into(),))).await.unwrap().unwrap();
            Ok(HostOwnedCompletion::new(move |mut call, _| {
                let first = first.restore(&mut call).unwrap();
                let second = second.restore(&mut call).unwrap();
                Ok(call.return_tuple((first, (second, ()))))
            }))
        })))
    }

    #[test]
    fn factories_keep_selected_authority_and_the_declarations_provider_across_owned_operations() {
        let provider = HostProviderModule::new("application", "library")
            .unwrap()
            .with_scoped_function_and_constructions::<Consumer, (), Function, Constructions, _>(
                "make", make,
            )
            .unwrap()
            .with_resumable_function::<Consumer, (bool,), Pair, Constructions, _>(
                "make_owned",
                make_owned,
            )
            .unwrap()
            .with_callable::<Native, Add<10>, (BigInt,), _>(add::<10>)
            .unwrap()
            .with_callable::<Native, Add<100>, (BigInt,), _>(add::<100>)
            .unwrap();
        let source = r#"
@external(erlang, "native", "make")
fn make() -> fn(Int) -> Int
@external(erlang, "native", "make_owned")
fn make_owned(cancel: Bool) -> #(fn(Int) -> Int, fn(Int) -> Int)
pub fn cancelled() { make_owned(True) Nil }
pub fn run() {
  let direct = make()
  let #(first, second) = make_owned(False)
  direct(3) + first(4) + second(5)
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "library.gleam", source)],
            )],
            HostProviderSet::from_providers(vec![provider]).unwrap(),
        )
        .unwrap();
        let (mut builder, run) = HostedModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("run"))
            .unwrap();
        let cancelled = builder
            .function(FunctionDeclaration::<(), ()>::new("cancelled"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = State::default();
        assert_eq!(
            std::ptr::from_mut(Consumer::project(&mut state)),
            std::ptr::addr_of_mut!(state.consumer)
        );
        let result = host
            .block_on(
                module.with_execution(&host, &mut state, &mut drop, async |scope| {
                    assert_eq!(
                        scope.call(&cancelled, ()).await,
                        Err(crate::embedding::CallError::Cancelled)
                    );
                    scope.call(&run, ()).await.unwrap()
                }),
            )
            .unwrap();
        assert_eq!(result, BigInt::from(339));
        assert_eq!(state.creations, 3);
        assert_eq!(state.effects, [100, 100, 100]);
    }
}
