mod capture;
mod type_;

use super::{Callable, CallableContext};
use crate::embedding::binding::Bindings;
use crate::embedding::input::InputConstructions;
use crate::embedding::value::{Arguments, EmbeddingValue};
use crate::embedding::work::ScopeBrand;
use crate::embedding::{
    BindingError, CallError, ExecutionScope, HostPreparationBindings, HostedModuleBindings,
    PreparedHostedModuleBindings,
};
use crate::host::{
    HostCallableSchema, HostFunctionBinding, HostProfile, RegisteredCallableConstruction,
};
use crate::plan::{FunctionType, LibraryCallableSignature, ProfiledHostedLibraryModulePlan};
use crate::runtime::{EmbeddingCallable, EmbeddingInputStorage};
#[doc(hidden)]
pub use capture::NativeCaptures;
use capture::{CaptureInput, CaptureTypes};
use std::marker::PhantomData;
use std::sync::Arc;
#[doc(hidden)]
pub use type_::{NativeArguments, NativeType};

/// Permission to construct one concrete specialization of a registered Rust body.
///
/// Select it before sealing with [`HostedModuleBindings::callable`] or
/// [`PreparedHostedModuleBindings::callable`]. Captures are supplied separately
/// inside the module's execution scope; cloning this selection does not create
/// a function instance. The type parameters describe the selected Rust views.
pub struct NativeCallable<Args, Return, Captures> {
    slot: usize,
    owner: Arc<()>,
    marker: PhantomData<fn(Args, Captures) -> Return>,
}

impl<Args, Return, Captures> Clone for NativeCallable<Args, Return, Captures> {
    fn clone(&self) -> Self {
        Self {
            slot: self.slot,
            owner: Arc::clone(&self.owner),
            marker: PhantomData,
        }
    }
}

impl<Profile: HostProfile> HostedModuleBindings<Profile> {
    /// Selects a concrete native declaration with its default Rust views.
    ///
    /// Named custom and external types default to opaque handles. Use
    /// [`Self::callable_as`] to select a generated nominal handle, `Result`,
    /// `Option`, or explicit work view of the same Gleam type.
    #[allow(clippy::type_complexity)]
    pub fn callable<Schema: HostCallableSchema>(
        &mut self,
    ) -> Result<
        NativeCallable<
            <Schema::Arguments as NativeArguments>::Shape,
            <Schema::Return as NativeType>::Shape,
            <Schema::Captures as NativeCaptures>::Shape,
        >,
        BindingError,
    >
    where
        Schema::Arguments: NativeArguments,
        Schema::Return: NativeType,
        Schema::Captures: NativeCaptures,
    {
        self.callable_as::<Schema, _, _, _>()
    }

    /// Selects Rust argument, return and capture views of a concrete declaration.
    ///
    /// Every view must match the declaration's exact Gleam type. Capture shapes
    /// are recursive pairs `(Head, Tail)` ending in `()`, with no capture-count
    /// limit. This selection grants only the declared function capability.
    #[allow(private_bounds)]
    pub fn callable_as<Schema, Args, Return, Captures>(
        &mut self,
    ) -> Result<NativeCallable<Args, Return, Captures>, BindingError>
    where
        Schema: HostCallableSchema,
        Args: Arguments,
        Return: EmbeddingValue,
        Captures: CaptureTypes,
    {
        bind::<Schema, Args, Return, Captures, _, _>(&mut self.inner)
    }
}

impl HostPreparationBindings {
    /// Includes a concrete native construction using its default Rust views.
    pub fn callable<Schema: HostCallableSchema>(&mut self) -> Result<(), BindingError>
    where
        Schema::Arguments: NativeArguments,
        Schema::Return: NativeType,
        Schema::Captures: NativeCaptures,
    {
        self.callable_as::<Schema,
            <Schema::Arguments as NativeArguments>::Shape,
            <Schema::Return as NativeType>::Shape,
            <Schema::Captures as NativeCaptures>::Shape>()
    }

    /// Includes exact Rust views selected with [`HostedModuleBindings::callable_as`].
    #[allow(private_bounds)]
    pub fn callable_as<Schema, Args, Return, Captures>(&mut self) -> Result<(), BindingError>
    where
        Schema: HostCallableSchema,
        Args: Arguments,
        Return: EmbeddingValue,
        Captures: CaptureTypes,
    {
        bind::<Schema, Args, Return, Captures, (), ()>(&mut self.inner).map(|_| ())
    }
}

impl<Profile: HostProfile> PreparedHostedModuleBindings<Profile> {
    /// Checks the same native declaration and default views against admitted data.
    #[allow(clippy::type_complexity)]
    pub fn callable<Schema: HostCallableSchema>(
        &mut self,
    ) -> Result<
        NativeCallable<
            <Schema::Arguments as NativeArguments>::Shape,
            <Schema::Return as NativeType>::Shape,
            <Schema::Captures as NativeCaptures>::Shape,
        >,
        crate::embedding::PreparedError,
    >
    where
        Schema::Arguments: NativeArguments,
        Schema::Return: NativeType,
        Schema::Captures: NativeCaptures,
    {
        self.callable_as::<Schema, _, _, _>()
    }

    /// Selects exactly the declaration and Rust views included during preparation.
    #[allow(private_bounds)]
    pub fn callable_as<Schema, Args, Return, Captures>(
        &mut self,
    ) -> Result<NativeCallable<Args, Return, Captures>, crate::embedding::PreparedError>
    where
        Schema: HostCallableSchema,
        Args: Arguments,
        Return: EmbeddingValue,
        Captures: CaptureTypes,
    {
        signature::<Schema, Args, Return, Captures>()
            .map_err(crate::embedding::PreparedError::from)
            .and_then(|(signature, standard)| {
                self.program.select_callable(
                    &RegisteredCallableConstruction::of::<Schema>(),
                    &signature,
                    &standard,
                )
            })
            .map(|slot| NativeCallable {
                slot,
                owner: Arc::clone(&self.owner),
                marker: PhantomData,
            })
    }
}

fn bind<Schema, Args, Return, Captures, Value, Never>(
    bindings: &mut Bindings<ProfiledHostedLibraryModulePlan<HostFunctionBinding<Value, Never>>>,
) -> Result<NativeCallable<Args, Return, Captures>, BindingError>
where
    Schema: HostCallableSchema,
    Args: Arguments,
    Return: EmbeddingValue,
    Captures: CaptureTypes,
{
    signature::<Schema, Args, Return, Captures>()
        .and_then(|(signature, standard)| {
            bind_signature(
                bindings.plan_mut(),
                RegisteredCallableConstruction::of::<Schema>(),
                signature,
                standard,
            )
        })
        .map(|slot| NativeCallable {
            slot,
            owner: Arc::clone(bindings.owner()),
            marker: PhantomData,
        })
}

fn bind_signature<Value, Never>(
    plan: &mut ProfiledHostedLibraryModulePlan<HostFunctionBinding<Value, Never>>,
    declaration: RegisteredCallableConstruction,
    signature: crate::plan::LibraryNativeSignature,
    standard: Vec<crate::plan::StandardVariant>,
) -> Result<usize, BindingError> {
    for variant in standard {
        let name = variant.type_name();
        if !variant.has_exact_definition(plan.custom_type(&name)) {
            return Err(BindingError::standard_type_mismatch(
                declaration.identity.name,
                name,
            ));
        }
    }
    plan.callable(declaration, signature)
}

fn signature<Schema, Args, Return, Captures>() -> Result<
    (
        crate::plan::LibraryNativeSignature,
        Vec<crate::plan::StandardVariant>,
    ),
    BindingError,
>
where
    Schema: HostCallableSchema,
    Args: Arguments,
    Return: EmbeddingValue,
    Captures: CaptureTypes,
{
    let declaration = RegisteredCallableConstruction::of::<Schema>();
    let arguments = Args::value_types();
    let return_ = Return::value_type();
    let mut captures = Vec::new();
    Captures::types(&mut captures);
    validate_view(declaration, &arguments, &return_, &captures).map(|()| {
        let invocation = LibraryCallableSignature {
            type_: FunctionType::new(arguments, return_),
            input_variants: Args::input_variants(),
            input_lists: Args::input_lists(),
            callables: Return::callables(),
        };
        let mut variants = Vec::new();
        let mut lists = Vec::new();
        let mut standard = Args::standard_variants();
        Return::collect_variants(&mut standard);
        Captures::variants(&mut variants);
        Captures::lists(&mut lists);
        Captures::standard(&mut standard);
        (
            crate::plan::LibraryNativeSignature {
                invocation,
                captures,
                capture_variants: variants,
                capture_lists: lists,
            },
            standard,
        )
    })
}

fn validate_view(
    declaration: RegisteredCallableConstruction,
    arguments: &[crate::plan::ValueType],
    return_: &crate::plan::ValueType,
    captures: &[crate::plan::ValueType],
) -> Result<(), BindingError> {
    if !declaration
        .arguments
        .iter()
        .map(|type_| type_.resolve(&|_| None))
        .eq(arguments.iter().cloned().map(Some))
        || declaration.return_.resolve(&|_| None) != Some(return_.clone())
        || !declaration
            .captures
            .iter()
            .map(|type_| type_.resolve(&|_| None))
            .eq(captures.iter().cloned().map(Some))
    {
        return Err(BindingError::NativeCallableView {
            package: declaration.identity.package,
            module: declaration.identity.module,
            name: declaration.identity.name,
        });
    }
    Ok(())
}

impl<'scope, 'module: 'scope, Profile: HostProfile> ExecutionScope<'scope, 'module, Profile> {
    /// Creates a fresh native function with immutable captures in declaration order.
    ///
    /// Captures use the selected recursive shape: `(head, tail)`, ending in
    /// `()`. Owned input containers are constructed once; retained values share
    /// storage. Construction executes no application body and schedules no task.
    #[allow(private_bounds, private_interfaces)]
    pub fn construct<Args, Return, Captures, Input>(
        &self,
        declaration: &NativeCallable<Args, Return, Captures>,
        captures: Input,
    ) -> Result<Callable<'scope, Args, Return>, CallError>
    where
        Args: Arguments,
        Return: EmbeddingValue,
        Captures: CaptureInput<Input, ScopeBrand<'scope>>,
    {
        if !Arc::ptr_eq(&declaration.owner, self.owner) {
            return Err(CallError::ForeignFunction);
        }
        if !Captures::owners_match(&captures, self.owner) {
            return Err(CallError::ForeignValue);
        }
        let entry = &self.native_callables[declaration.slot];
        let mut inputs = None;
        Captures::push(
            captures,
            &mut InputConstructions::new(&entry.captures),
            &EmbeddingInputStorage::default(),
            &mut inputs,
        );
        Ok(Callable {
            value: EmbeddingCallable::construct(&entry.construction, inputs, &self.captures),
            context: CallableContext {
                brand: self.brand,
                owner: Arc::clone(self.owner),
                declaration: &entry.invocation,
            },
            marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::signature;
    use crate::embedding::{BigInt, BindingError};
    use crate::{HostCallableSchema, HostReturns, HostTypeList, HostTypeListEnd};

    struct Add;
    impl HostCallableSchema for Add {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "callbacks";
        const NAME: &'static str = "add";
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Return = BigInt;
        type Captures = HostTypeList<BigInt, HostTypeListEnd>;
        type Constructions = HostTypeListEnd;
        type Completion = HostReturns;
    }

    #[test]
    fn native_views_reject_argument_return_and_capture_type_or_count_disagreement() {
        let expected = BindingError::NativeCallableView {
            package: "application".into(),
            module: "callbacks".into(),
            name: "add".into(),
        };
        for result in [
            signature::<Add, (bool,), BigInt, (BigInt, ())>(),
            signature::<Add, (), BigInt, (BigInt, ())>(),
            signature::<Add, (BigInt,), bool, (BigInt, ())>(),
            signature::<Add, (BigInt,), BigInt, (bool, ())>(),
            signature::<Add, (BigInt,), BigInt, ()>(),
            signature::<Add, (BigInt,), BigInt, (BigInt, (BigInt, ()))>(),
        ] {
            assert_eq!(result.err(), Some(expected.clone()));
        }
    }

    struct Profile;
    impl crate::HostProfile for Profile {
        type RunState = std::cell::Cell<usize>;
        type ExternalStores = ();
        type ExecutionState = ();
    }
    impl crate::HostProvider<Profile> for Profile {
        type State = std::cell::Cell<usize>;
        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    fn add<'call>(
        mut call: crate::HostCall<'call, Profile, Profile, BigInt>,
        captures: crate::HostCaptures<'call, HostTypeList<BigInt, HostTypeListEnd>>,
        _: crate::HostConstructions<'call, HostTypeListEnd>,
        value: BigInt,
    ) -> Result<crate::HostCallCompletion<'call, BigInt>, crate::HostCallError> {
        let (offset, ()) = call.captures(captures);
        let state = call.state();
        state.set(state.get() + 1);
        Ok(call.return_value(value + offset))
    }

    #[allow(clippy::type_complexity)]
    fn bindings() -> (
        crate::embedding::HostedModuleBindings<Profile>,
        crate::embedding::Function<
            (crate::embedding::CallableType<(BigInt,), BigInt>,),
            crate::embedding::CallableType<(BigInt,), BigInt>,
        >,
    ) {
        let providers = crate::HostProviderSet::new([])
            .unwrap()
            .with_callable::<Profile, Add, (BigInt,), _>(add)
            .unwrap();
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [crate::PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "library",
                    "library.gleam",
                    "pub fn keep(callback: fn(Int) -> Int) -> fn(Int) -> Int { callback }",
                )],
            )],
            providers,
        )
        .unwrap();
        crate::embedding::HostedModuleBuilder::new(program)
            .unwrap()
            .function(crate::embedding::FunctionDeclaration::new("keep"))
            .unwrap()
    }

    #[test]
    fn construction_and_aliasing_execute_no_body_and_foreign_factories_are_rejected() {
        let (mut own, keep) = bindings();
        let factory = own.callable::<Add>().unwrap();
        let (mut other, foreign_keep) = bindings();
        let foreign_factory = other.callable::<Add>().unwrap();
        let alias = factory.clone();
        let mut module = own.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut calls = std::cell::Cell::new(0);
        host.block_on(
            module.with_execution(&host, &mut calls, &mut drop, async |scope| {
                let function = scope.construct(&alias, (BigInt::from(10), ())).unwrap();
                let retained = function.clone();
                assert_eq!(
                    scope
                        .construct(&foreign_factory, (BigInt::from(100), ()))
                        .err(),
                    Some(crate::embedding::CallError::ForeignFunction)
                );
                assert_eq!(
                    scope.call(&foreign_keep, (&retained,)).await.err(),
                    Some(crate::embedding::CallError::ForeignFunction)
                );
                let returned = scope.call(&keep, (retained,)).await.unwrap();
                assert_eq!(
                    scope.invoke(&returned, (BigInt::from(5),)).await.unwrap(),
                    BigInt::from(15)
                );
            }),
        )
        .unwrap();
        assert_eq!(calls.get(), 1);
    }

    type DeclarationMarker<Args, Return, Captures, Completion> =
        std::marker::PhantomData<fn() -> (Args, Return, Captures, Completion)>;
    struct Declaration<Args, Return, Captures, Completion, const MISSING: bool = false>(
        DeclarationMarker<Args, Return, Captures, Completion>,
    );
    impl<Args, Return, Captures, Completion, const MISSING: bool> HostCallableSchema
        for Declaration<Args, Return, Captures, Completion, MISSING>
    where
        Args: crate::HostTypeSequence,
        Return: crate::HostType,
        Captures: crate::HostTypeSequence,
        Completion: crate::HostCompletion + 'static,
    {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "callbacks";
        const NAME: &'static str = if MISSING { "missing" } else { "add" };
        type Arguments = Args;
        type Return = Return;
        type Captures = Captures;
        type Constructions = HostTypeListEnd;
        type Completion = Completion;
    }

    #[test]
    fn native_standard_views_share_the_declared_result_and_capture_layout() {
        type End = HostTypeListEnd;
        type Choice = crate::provider::ProviderResult<BigInt, BigInt>;
        type Captures = HostTypeList<Choice, End>;
        type Schema = Declaration<End, Choice, Captures, HostReturns>;
        fn choose<'call>(
            mut call: crate::HostCall<'call, Profile, Profile, Choice>,
            captures: crate::HostCaptures<'call, Captures>,
            _: crate::HostConstructions<'call, End>,
        ) -> Result<crate::HostCallCompletion<'call, Choice>, crate::HostCallError> {
            let (value, ()) = call.captures(captures);
            let state = call.state();
            state.set(state.get() + 1);
            Ok(call.return_value(value))
        }
        let typed = crate::compile_typed_host_program(
            "application",
            "library",
            [crate::PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "library",
                    "library.gleam",
                    "pub fn main() -> Result(Int, Int) { Ok(42) }",
                )],
            )],
            crate::HostProviderSet::new([])
                .unwrap()
                .with_callable::<Profile, Schema, (), _>(choose)
                .unwrap(),
        )
        .unwrap();
        let (mut bindings, _) = crate::embedding::HostedModuleBuilder::new(typed)
            .unwrap()
            .function(crate::embedding::FunctionDeclaration::<
                (),
                Result<BigInt, BigInt>,
            >::new("main"))
            .unwrap();
        let factory = bindings
            .callable_as::<Schema, (), Result<BigInt, BigInt>, (Result<BigInt, BigInt>, ())>()
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut state = std::cell::Cell::new(0);
        host.block_on(
            module.with_execution(&host, &mut state, &mut drop, async |scope| {
                for value in [Ok(BigInt::from(42)), Err(BigInt::from(7))] {
                    let callback = scope.construct(&factory, (value.clone(), ())).unwrap();
                    assert_eq!(scope.invoke(&callback, ()).await.unwrap(), value);
                }
            }),
        )
        .unwrap();
        assert_eq!(state.get(), 2);
    }

    #[test]
    fn native_selection_rejects_changed_body_contract_before_creating_a_factory() {
        type One<Type> = HostTypeList<Type, HostTypeListEnd>;
        let (mut bindings, _) = bindings();
        for actual in [
            bindings
                .callable_as::<Add, (bool,), BigInt, (BigInt, ())>()
                .err(),
            bindings
                .callable_as::<Add, (BigInt,), bool, (BigInt, ())>()
                .err(),
            bindings
                .callable_as::<Add, (BigInt,), BigInt, (bool, ())>()
                .err(),
        ] {
            assert_eq!(
                actual,
                Some(BindingError::NativeCallableView {
                    package: "application".into(),
                    module: "callbacks".into(),
                    name: "add".into(),
                })
            );
        }
        assert_eq!(
            bindings
                .callable::<Declaration<One<BigInt>, BigInt, One<BigInt>, HostReturns, true>>()
                .err(),
            Some(BindingError::NativeCallable {
                package: "application".into(),
                module: "callbacks".into(),
                name: "missing".into(),
            })
        );
        type Optional = Declaration<
            HostTypeListEnd,
            crate::provider::ProviderOption<BigInt>,
            HostTypeListEnd,
            HostReturns,
        >;
        assert_eq!(
            bindings
                .callable_as::<Optional, (), Option<BigInt>, ()>()
                .err(),
            Some(BindingError::StandardTypeMismatch {
                name: "add".into(),
                package: "gleam_stdlib".into(),
                module: "gleam/option".into(),
                type_name: "Option".into(),
            }),
        );
        let expected = BindingError::NativeCallable {
            package: "application".into(),
            module: "callbacks".into(),
            name: "add".into(),
        };
        for actual in [
            bindings
                .callable::<Declaration<HostTypeListEnd, BigInt, One<BigInt>, HostReturns>>()
                .err(),
            bindings
                .callable::<Declaration<One<BigInt>, bool, One<BigInt>, HostReturns>>()
                .err(),
            bindings
                .callable::<Declaration<One<BigInt>, BigInt, One<bool>, HostReturns>>()
                .err(),
            bindings
                .callable::<Declaration<One<BigInt>, BigInt, One<BigInt>, crate::HostDiverges>>()
                .err(),
        ] {
            assert_eq!(actual, Some(expected.clone()));
        }
    }
}
