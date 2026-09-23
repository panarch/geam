use super::{Domain, Profile};
use crate::provider::{
    Call, Factory, ProviderConstruction, ProviderConstructionIndex0, ProviderConstructionList,
    ProviderConstructionRequirements, ProviderConstructions, ProviderExecutionCall,
    ProviderFactoryBinding, ProviderFactoryBindings, ProviderFactoryCodec, ProviderNoConstructions,
    ProviderOwnedCaptures,
};
use crate::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallableSchema,
    HostCaptures, HostConstructions, HostCreatedFunction, HostFunctionType, HostOwnedCallable,
    HostOwnedCompletion, HostProvider, HostProviderModule, HostProviderSet, HostReturns, HostType,
    HostTypeList, HostTypeListEnd, ModuleSource, PackageSource,
};
use num_bigint::BigInt;
use std::cell::Cell;
use std::future::Future;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

type End = HostTypeListEnd;
type One<T> = HostTypeList<T, End>;
type Function = HostFunctionType<End, BigInt>;
type Needed = ProviderConstruction<HostCreatedFunction<Increment>>;
type Requirements = ProviderConstructionList<Needed, ProviderNoConstructions>;
type Constructions = <Requirements as ProviderConstructionRequirements>::Types<End>;
type Retained = HostOwnedCallable<Profile, Provider, End, BigInt, End>;

struct Provider;
impl HostProvider<Profile> for Provider {
    type State = Cell<usize>;
    fn project(state: &mut Self::State) -> &mut Self::State {
        state
    }
}

struct Increment;
impl HostCallableSchema for Increment {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "increment";
    type Arguments = End;
    type Return = BigInt;
    type Captures = One<BigInt>;
    type Constructions = End;
    type Completion = HostReturns;
}

struct Bindings;
impl ProviderFactoryBindings for Bindings {
    type Requirements = Requirements;
    type CaptureMode = ProviderOwnedCaptures;
}
impl ProviderFactoryBinding<Increment, Needed> for Bindings {
    fn select<'call>(
        proof: &ProviderConstructions<'call, Requirements>,
    ) -> ProviderConstructions<'call, Needed> {
        proof.select::<ProviderConstructionIndex0>()
    }
}
impl ProviderFactoryCodec<Profile> for Increment {
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
        if captures.0 < BigInt::from(0) {
            return Err(crate::HostFailure::new("invalid capture").into());
        }
        proof.with_call::<_, _, _, Provider, _>(call, |mut call, proof| {
            assert_eq!(call.state().get(), 1);
            call.state().set(2);
            let function = call.construct_function(proof.token(), (captures.0, ()));
            Ok(call.owned_callable(function, &HostConstructions::<End>::new()))
        })
    }
}

fn increment<'call>(
    mut call: HostCall<'call, Profile, Provider, BigInt>,
    captures: HostCaptures<'call, One<BigInt>>,
    _: HostConstructions<'call, End>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    assert_eq!(call.state().get(), 2);
    call.state().set(3);
    let (value, ()) = call.captures(captures);
    Ok(call.return_value(value + 1))
}

fn issue<'call>(
    call: HostCall<'call, Profile, Provider, Function>,
    constructions: HostConstructions<'call, Constructions>,
    value: BigInt,
) -> Result<HostCallContinuation<'call, Function>, HostCallError> {
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            context.with_state(|state| state.set(1)).await.unwrap();
            let mut call = Call::<_, ProviderExecutionCall<'_, _, _, (), Bindings>>::from_execution_context_with_factories(context);
            let callback = call.create(&Factory::<Increment>::declaration(), (value,)).await?;
            Ok(HostOwnedCompletion::new(move |mut call, _| {
                let function = callback.restore(&mut call).unwrap();
                Ok(call.return_value(function))
            }))
        })
    }))
}

#[test]
fn owned_factory_requests_preserve_capture_errors_and_service_shutdown() {
    use crate::plan::{LibraryEntry, LibraryValueType};
    use crate::runtime::{HostCallOrigin, RetainedValues};
    let typed = crate::compile_typed_host_program(
        "app", "main",
        [PackageSource::new("app", Vec::<&str>::new(), [ModuleSource::new(
            "main", "main.gleam",
            "@external(erlang, \"native\", \"issue\") fn issue(value: Int) -> fn() -> Int\npub fn main(value: Int) { issue(value)() }",
        )])],
        HostProviderSet::from_providers([HostProviderModule::new("app", "main").unwrap()
            .with_resumable_function::<Provider, (BigInt,), Function, Constructions, _>("issue", issue).unwrap()
            .with_callable::<Provider, Increment, (), _>(increment).unwrap()]).unwrap(),
    ).unwrap();
    let library = crate::planner::plan_host_library_program(typed).unwrap();
    let main = library
        .functions()
        .iter()
        .find(|function| function.name() == "main")
        .unwrap()
        .signature()
        .id();
    let (plan, entries, _) = crate::plan::execution::HostedProgram::from_library_plan(
        library,
        LibraryEntry::new(main, LibraryValueType::Int, vec![], vec![]),
        vec![],
    )
    .unwrap();
    let plan = Arc::new(plan);
    let host = crate::execution_fixture::TestHost::default();
    for (close, input) in [(false, 41), (false, -1), (true, 41)] {
        let mut state = Cell::new(0);
        let mut stores = Cell::new(());
        let mut echo = Vec::new();
        let mut domain = Domain::new(
            Arc::clone(&plan),
            &host,
            &mut state,
            &mut stores,
            &mut echo,
            Default::default(),
            NonZeroUsize::MIN,
        );
        let context = domain.context();
        let mut arguments = RetainedValues::empty();
        arguments.push_evaluated(crate::runtime::EvaluatedValue::Int(input.into()));
        let mut call = Box::pin(context.call(
            *entries.ints[0].function(),
            HostCallOrigin::Entry,
            arguments,
        ));
        if close {
            let mut cx = Context::from_waker(Waker::noop());
            assert!(call.as_mut().poll(&mut cx).is_pending());
            while domain.state.get() == 0 {
                domain.service(&mut cx);
                assert!(
                    host.poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                        .is_pending()
                );
            }
            assert_eq!(domain.state.get(), 1);
            assert!(call.as_mut().poll(&mut cx).is_pending());
            // Observe the real request-service shutdown before the later unit
            // cancellation step drops this native continuation.
            domain.work.close();
            assert!(
                host.poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                    .is_pending()
            );
            assert_eq!(
                call.as_mut().poll(&mut cx),
                Poll::Ready(Err(crate::runtime::work::Cancelled))
            );
            drop(domain);
            assert!(
                host.poll(std::pin::pin!(std::future::pending::<()>()).as_mut())
                    .is_pending()
            );
        } else {
            let result = host.block_on(domain.drive(call)).unwrap().unwrap();
            assert_eq!(
                result.map_err(|error| error.to_string()),
                if input < 0 {
                    Err("host function app::main.issue failed: invalid capture".into())
                } else {
                    Ok(BigInt::from(42))
                },
            );
        }
        assert_eq!(state.get(), if close || input < 0 { 1 } else { 3 });
        assert!(echo.is_empty());
    }
}
