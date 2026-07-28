use super::argument::{HostArgument, HostParameter, HostParameterLayout, HostScopedArgument};
use super::return_::{HostFunctionImplementation, HostReturn};
use crate::host::{
    HostAbiType, HostCall, HostCallCompletion, HostCallError, HostFailure, HostProfile,
    HostProvider, HostTypeDescriptor,
};

pub trait HostFunction<Arguments, Return>: Send + Sync + 'static {
    fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile>;
}

pub trait FallibleHostFunction<Arguments, Return>: Send + Sync + 'static {
    fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile>;
}

pub trait ScopedHostFunction<Profile, Provider, Arguments, Return>: Send + Sync + 'static
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    fn register(self) -> HostFunctionRegistration<Profile>;
}

pub struct HostFunctionRegistration<Profile: HostProfile> {
    pub(super) parameters: Box<[HostParameter]>,
    pub(super) parameter_types: Box<[HostTypeDescriptor]>,
    pub(super) return_type: HostTypeDescriptor,
    pub(super) custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
    pub(super) implementation: HostFunctionImplementation<Profile>,
}

macro_rules! host_function {
    () => {
        impl<Function, Return> HostFunction<(), Return> for Function
        where
            Function: Fn() -> Return + Send + Sync + 'static,
            Return: HostReturn,
        {
            fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile> {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation: Return::implementation(move |_, _| Ok(self())),
                }
            }
        }

        impl<Function, Return> FallibleHostFunction<(), Return> for Function
        where
            Function: Fn() -> Result<Return, HostFailure> + Send + Sync + 'static,
            Return: HostReturn,
        {
            fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile> {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation: Return::implementation(move |_, _| {
                        self().map_err(HostCallError::from)
                    }),
                }
            }
        }

        impl<Profile, Provider, Function, Return>
            ScopedHostFunction<Profile, Provider, (), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                    HostCall<'call, Profile, Provider, Return>,
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
        {
            fn register(self) -> HostFunctionRegistration<Profile> {
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    parameter_types: Box::new([]),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation: HostFunctionImplementation::scoped(move |runtime| {
                        self(HostCall::new(runtime)).map(|completion| completion.token)
                    }),
                }
            }
        }
    };
    ($($argument:ident => $slot:ident),+) => {
        impl<Function, Return, $($argument,)*> HostFunction<($($argument,)*), Return> for Function
        where
            Function: Fn($($argument),*) -> Return + Send + Sync + 'static,
            Return: HostReturn,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |_, arguments| {
                    Ok(self($($argument::read(arguments, $slot)),*))
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation,
                }
            }
        }

        impl<Function, Return, $($argument,)*>
            FallibleHostFunction<($($argument,)*), Return> for Function
        where
            Function: Fn($($argument),*) -> Result<Return, HostFailure> + Send + Sync + 'static,
            Return: HostReturn,
            $($argument: HostArgument,)*
        {
            fn register<Profile: HostProfile>(self) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |_, arguments| {
                    self($($argument::read(arguments, $slot)),*).map_err(HostCallError::from)
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostReturn>::descriptor(),
                    custom_schemas: Box::new([]),
                    implementation,
                }
            }
        }

        impl<Profile, Provider, Function, Return, $($argument,)*>
            ScopedHostFunction<Profile, Provider, ($($argument,)*), Return> for Function
        where
            Profile: HostProfile,
            Provider: HostProvider<Profile>,
            Function: for<'call> Fn(
                HostCall<'call, Profile, Provider, Return>,
                    $(<$argument as crate::host::HostType>::Value<'call>),*
                ) -> Result<HostCallCompletion<'call, Return>, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostAbiType,
            $($argument: HostScopedArgument,)*
        {
            fn register(self) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = <$argument as HostScopedArgument>::register(&mut layout);)*
                let mut custom_schemas = Vec::new();
                let mut visited = std::collections::HashSet::new();
                $(<$argument as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );)*
                <Return as HostAbiType>::collect_custom_schemas(
                    &mut custom_schemas,
                    &mut visited,
                );
                let implementation = HostFunctionImplementation::scoped(move |runtime| {
                    let call = HostCall::new(runtime);
                    $(let $slot = <$argument as HostScopedArgument>::read(&call, $slot);)*
                    self(
                        call,
                        $($slot),*
                    )
                    .map(|completion| completion.token)
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    parameter_types: vec![$(<$argument as HostAbiType>::descriptor()),*].into_boxed_slice(),
                    return_type: <Return as HostAbiType>::descriptor(),
                    custom_schemas: custom_schemas.into_boxed_slice(),
                    implementation,
                }
            }
        }
    };
}

host_function!();
host_function!(A0 => a0);
host_function!(A0 => a0, A1 => a1);
host_function!(A0 => a0, A1 => a1, A2 => a2);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4);
host_function!(A0 => a0, A1 => a1, A2 => a2, A3 => a3, A4 => a4, A5 => a5);
host_function!(
    A0 => a0,
    A1 => a1,
    A2 => a2,
    A3 => a3,
    A4 => a4,
    A5 => a5,
    A6 => a6
);

#[cfg(test)]
mod tests {
    use super::{FallibleHostFunction, HostFunction, ScopedHostFunction};
    use crate::BitArrayValue;
    use crate::host::function::HostFunctionImplementation;
    use crate::host::function::argument::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostFailure, HostProvider, HostScopedValue,
        HostTypeDescriptor, expect_value_implementation,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct ScopedProvider;

    type Scoped0 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped1 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped2 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped3 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped4 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped5 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped6 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;
    type Scoped7 = for<'call> fn(
        HostCall<'call, TestHostProfile, ScopedProvider, BigInt>,
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>;

    impl HostProvider<TestHostProfile> for ScopedProvider {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    #[test]
    fn supports_zero_arguments() {
        let registration = <_ as HostFunction<(), BigInt>>::register(|| BigInt::from(7));

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_type, HostTypeDescriptor::Int);
        assert_eq!(
            call_int(&registration.implementation, Vec::new(), Vec::new()),
            BigInt::from(7),
        );
    }

    #[test]
    fn supports_zero_argument_fallible_functions() {
        let registration =
            <_ as FallibleHostFunction<(), BigInt>>::register::<TestHostProfile>(|| {
                Err(HostFailure::new("unavailable"))
            });
        let implementation = expect_value_implementation(&registration.implementation);

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_type, HostTypeDescriptor::Int);
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        assert_eq!(
            implementation
                .call(&mut runtime)
                .expect_err("fallible function should preserve its failure")
                .to_string(),
            "unavailable",
        );
    }

    #[test]
    fn supports_every_fallible_argument_arity() {
        let registrations = vec![
            <_ as FallibleHostFunction<(), BigInt>>::register::<TestHostProfile>(|| {
                Ok::<_, HostFailure>(BigInt::from(0))
            }),
            <_ as FallibleHostFunction<((),), BigInt>>::register::<TestHostProfile>(|_: ()| {
                Ok::<_, HostFailure>(BigInt::from(1))
            }),
            <_ as FallibleHostFunction<((), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: ()| Ok::<_, HostFailure>(BigInt::from(2)),
            ),
            <_ as FallibleHostFunction<((), (), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: (), _: ()| Ok::<_, HostFailure>(BigInt::from(3)),
            ),
            <_ as FallibleHostFunction<((), (), (), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: (), _: (), _: ()| Ok::<_, HostFailure>(BigInt::from(4)),
            ),
            <_ as FallibleHostFunction<((), (), (), (), ()), BigInt>>::register::<TestHostProfile>(
                |_: (), _: (), _: (), _: (), _: ()| Ok::<_, HostFailure>(BigInt::from(5)),
            ),
            <_ as FallibleHostFunction<((), (), (), (), (), ()), BigInt>>::register::<
                TestHostProfile,
            >(|_: (), _: (), _: (), _: (), _: (), _: ()| {
                Ok::<_, HostFailure>(BigInt::from(6))
            }),
            <_ as FallibleHostFunction<((), (), (), (), (), (), ()), BigInt>>::register::<
                TestHostProfile,
            >(|_: (), _: (), _: (), _: (), _: (), _: (), _: ()| {
                Ok::<_, HostFailure>(BigInt::from(7))
            }),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            expect_value_implementation(&registration.implementation)
                .call(&mut runtime)
                .expect("fallible callback should succeed");
            assert_eq!(
                runtime.completed(),
                Some(&HostScopedValue::Int(BigInt::from(arity))),
            );
        }
    }

    #[test]
    fn supports_every_scoped_argument_arity() {
        let zero: Scoped0 = |mut call| {
            *call.state() += 1;
            Ok(call.return_value(BigInt::from(0)))
        };
        let one: Scoped1 = |call, _| Ok(call.return_value(BigInt::from(1)));
        let two: Scoped2 = |call, _, _| Ok(call.return_value(BigInt::from(2)));
        let three: Scoped3 = |call, _, _, _| Ok(call.return_value(BigInt::from(3)));
        let four: Scoped4 = |call, _, _, _, _| Ok(call.return_value(BigInt::from(4)));
        let five: Scoped5 = |call, _, _, _, _, _| Ok(call.return_value(BigInt::from(5)));
        let six: Scoped6 = |call, _, _, _, _, _, _| Ok(call.return_value(BigInt::from(6)));
        let seven: Scoped7 = |call, _, _, _, _, _, _, _| Ok(call.return_value(BigInt::from(7)));
        let registrations = vec![
            <_ as ScopedHostFunction<TestHostProfile, ScopedProvider, (), BigInt>>::register(
                zero,
            ),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((),),
                BigInt,
            >>::register(one),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), ()),
                BigInt,
            >>::register(two),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), (), ()),
                BigInt,
            >>::register(three),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), ()),
                BigInt,
            >>::register(four),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), ()),
                BigInt,
            >>::register(five),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), ()),
                BigInt,
            >>::register(six),
            <_ as ScopedHostFunction<
                TestHostProfile,
                ScopedProvider,
                ((), (), (), (), (), (), ()),
                BigInt,
            >>::register(seven),
        ];

        for (arity, registration) in registrations.into_iter().enumerate() {
            assert_eq!(
                registration.parameter_types.as_ref(),
                vec![HostTypeDescriptor::Nil; arity],
            );
            let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                arity,
            );
            let mut state = TestRunState::default();
            let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
            expect_value_implementation(&registration.implementation)
                .call(&mut runtime)
                .expect("scoped callback should succeed");
            assert_eq!(
                runtime.completed(),
                Some(&HostScopedValue::Int(BigInt::from(arity))),
            );
            drop(runtime);
            assert_eq!(state.counter, usize::from(arity == 0));
        }
    }

    #[test]
    fn supports_one_argument() {
        let registration = <_ as HostFunction<(BigInt,), BigInt>>::register(|a: BigInt| a + 1);

        assert_eq!(
            registration.parameter_types.as_ref(),
            [HostTypeDescriptor::Int],
        );
        assert_eq!(
            call_int(&registration.implementation, vec![1.into()], Vec::new()),
            BigInt::from(2),
        );
    }

    #[test]
    fn supports_two_arguments() {
        let registration =
            <_ as HostFunction<(BigInt, BigInt), BigInt>>::register(|a: BigInt, b: BigInt| a - b);

        assert_eq!(
            registration.parameter_types.as_ref(),
            [HostTypeDescriptor::Int, HostTypeDescriptor::Int],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 3.into()],
                Vec::new(),
            ),
            BigInt::from(7),
        );
    }

    #[test]
    fn supports_three_arguments() {
        let registration = <_ as HostFunction<(bool, BigInt, BigInt), BigInt>>::register(
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 20.into()],
                vec![false],
            ),
            BigInt::from(20),
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![10.into(), 20.into()],
                vec![true],
            ),
            BigInt::from(10),
        );
    }

    #[test]
    fn supports_four_arguments() {
        let registration = <_ as HostFunction<(BigInt, bool, BigInt, bool), bool>>::register(
            |left: BigInt, first: bool, right: BigInt, second: bool| {
                left < right && first && !second
            },
        );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
            ],
        );
        assert!(call_bool(
            registration.implementation,
            vec![1.into(), 2.into()],
            vec![true, false],
        ));
    }

    #[test]
    fn supports_five_arguments() {
        let registration =
            <_ as HostFunction<(BigInt, BigInt, BigInt, BigInt, BigInt), BigInt>>::register(
                |a: BigInt, b: BigInt, c: BigInt, d: BigInt, e: BigInt| a + b + c + d + e,
            );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 3.into(), 4.into(), 5.into()],
                Vec::new(),
            ),
            BigInt::from(15),
        );
    }

    #[test]
    fn supports_six_arguments() {
        let registration =
            <_ as HostFunction<(bool, bool, bool, bool, bool, bool), bool>>::register(
                |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool| {
                    a && !b && c && !d && e && !f
                },
            );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Bool,
            ],
        );
        assert!(call_bool(
            registration.implementation,
            Vec::new(),
            vec![true, false, true, false, true, false],
        ));
    }

    #[test]
    fn supports_seven_arguments() {
        let registration = <_ as HostFunction<
            (BigInt, bool, BigInt, bool, BigInt, bool, BigInt),
            BigInt,
        >>::register::<TestHostProfile>(
            |a: BigInt, b: bool, c: BigInt, d: bool, e: BigInt, f: bool, g: BigInt| {
                let c = if b { c } else { BigInt::from(0) };
                let e = if d { e } else { BigInt::from(0) };
                let g = if f { g } else { BigInt::from(0) };
                a + c + e + g
            },
        );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int,
            ],
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 4.into(), 8.into()],
                vec![true, false, true],
            ),
            BigInt::from(11),
        );
        assert_eq!(
            call_int(
                &registration.implementation,
                vec![1.into(), 2.into(), 4.into(), 8.into()],
                vec![false, true, false],
            ),
            BigInt::from(5),
        );
    }

    #[test]
    fn supports_every_scalar_argument_family_in_one_sealed_layout() {
        let registration = <_ as HostFunction<
            (BigInt, f64, EcoString, BitArrayValue, char, bool, ()),
            EcoString,
        >>::register::<TestHostProfile>(
            |int: BigInt,
             float: f64,
             string: EcoString,
             bits: BitArrayValue,
             codepoint: char,
             bool_: bool,
             (): ()| {
                format!(
                    "{int}:{float}:{string}:{}:{codepoint}:{bool_}",
                    bits.bit_len(),
                )
                .into()
            },
        );

        assert_eq!(
            registration.parameter_types.as_ref(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Float,
                HostTypeDescriptor::String,
                HostTypeDescriptor::BitArray,
                HostTypeDescriptor::UtfCodepoint,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Nil,
            ],
        );
        assert_eq!(registration.return_type, HostTypeDescriptor::String);
        let arguments = CallArguments::new(vec![1.into()], vec![true]).with_scalar_values(
            vec![1.5],
            vec!["one".into()],
            vec![BitArrayValue::from_bytes(vec![0xff])],
            vec!['A'],
            1,
        );

        assert_eq!(
            call_string(registration.implementation, arguments),
            EcoString::from("1:1.5:one:8:A:true"),
        );
    }

    #[test]
    #[should_panic(expected = "test function should return Int")]
    fn int_return_shape_guard_is_visible() {
        let registration = <_ as HostFunction<(), bool>>::register(<bool as Default>::default);
        call_int(&registration.implementation, Vec::new(), Vec::new());
    }

    #[test]
    #[should_panic(expected = "test function should return Bool")]
    fn bool_return_shape_guard_is_visible() {
        call_bool(
            <_ as HostFunction<(), BigInt>>::register(<BigInt as Default>::default).implementation,
            Vec::new(),
            Vec::new(),
        );
    }

    #[test]
    #[should_panic(expected = "all-scalar test function should return String")]
    fn string_return_shape_guard_is_visible() {
        call_string(
            <_ as HostFunction<(), BigInt>>::register(<BigInt as Default>::default).implementation,
            CallArguments::new(Vec::new(), Vec::new()),
        );
    }

    fn call_int(
        implementation: &HostFunctionImplementation<TestHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> BigInt {
        let implementation = expect_value_implementation(implementation);
        let arguments = CallArguments::new(ints, bools);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        implementation
            .call(&mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::Int(value)) = runtime.completed() else {
            panic!("test function should return Int");
        };
        value.clone()
    }

    fn call_bool(
        implementation: HostFunctionImplementation<TestHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> bool {
        let implementation = expect_value_implementation(&implementation);
        let arguments = CallArguments::new(ints, bools);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        implementation
            .call(&mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::Bool(value)) = runtime.completed() else {
            panic!("test function should return Bool");
        };
        *value
    }

    fn call_string(
        implementation: HostFunctionImplementation<TestHostProfile>,
        arguments: CallArguments,
    ) -> EcoString {
        let implementation = expect_value_implementation(&implementation);
        let mut state = TestRunState::default();
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        implementation
            .call(&mut runtime)
            .expect("test host function should succeed");
        let Some(HostScopedValue::String(value)) = runtime.completed() else {
            panic!("all-scalar test function should return String");
        };
        value.clone()
    }
}
