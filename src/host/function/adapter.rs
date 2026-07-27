use super::HostValueType;
use super::argument::{HostArgument, HostParameter, HostParameterLayout};
use super::return_::{HostFunctionImplementation, HostReturn};
use crate::host::{HostCall, HostCallError, HostFailure, HostProfile, HostProvider};

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
    pub(super) return_: HostValueType,
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
                    return_: Return::type_(),
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
                    return_: Return::type_(),
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
                    &mut HostCall<'call, Profile, Provider>,
                ) -> Result<Return, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostReturn,
        {
            fn register(self) -> HostFunctionRegistration<Profile> {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    return_: Return::type_(),
                    implementation: Return::implementation(move |state, _| {
                        self(&mut HostCall::new(state))
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
                    return_: Return::type_(),
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
                    return_: Return::type_(),
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
                    &mut HostCall<'call, Profile, Provider>,
                    $($argument),*
                ) -> Result<Return, HostCallError>
                + Send
                + Sync
                + 'static,
            Return: HostReturn,
            $($argument: HostArgument,)*
        {
            fn register(self) -> HostFunctionRegistration<Profile> {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |state, arguments| {
                    self(
                        &mut HostCall::new(state),
                        $($argument::read(arguments, $slot)),*
                    )
                });
                HostFunctionRegistration {
                    parameters: layout.finish(),
                    return_: Return::type_(),
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
    use crate::host::function::argument::CallArguments;
    use crate::host::function::{
        HostFunctionImplementation, HostIntFunction, HostParameter, HostStringFunction,
        HostValueType,
    };
    use crate::host::{HostCall, HostFailure, HostProfile, HostProvider, StatelessHostProfile};
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct Profile;

    struct Counter;

    impl HostProfile for Profile {
        type RunState = usize;
    }

    impl HostProvider<Profile> for Counter {
        type State = usize;

        fn project(state: &mut usize) -> &mut Self::State {
            state
        }
    }

    #[test]
    fn supports_zero_arguments() {
        let registration = <_ as HostFunction<(), BigInt>>::register(|| BigInt::from(7));

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_, HostValueType::Int);
        assert_eq!(
            call_int(registration.implementation, Vec::new(), Vec::new()),
            BigInt::from(7),
        );
    }

    #[test]
    fn supports_zero_argument_fallible_functions() {
        let registration =
            <_ as FallibleHostFunction<(), BigInt>>::register::<StatelessHostProfile>(|| {
                Err(HostFailure::new("unavailable"))
            });
        let implementation = int_function(registration.implementation);

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_, HostValueType::Int);
        assert_eq!(
            implementation
                .call(&mut (), &CallArguments::new(Vec::new(), Vec::new()))
                .expect_err("fallible function should preserve its failure")
                .to_string(),
            "unavailable",
        );
    }

    #[test]
    fn supports_zero_argument_scoped_functions() {
        let registration = <_ as ScopedHostFunction<Profile, Counter, (), BigInt>>::register(
            |call: &mut HostCall<'_, Profile, Counter>| {
                *call.state() += 1;
                Ok(BigInt::from(*call.state()))
            },
        );
        let implementation = int_function(registration.implementation);
        let mut state = 41;

        assert_eq!(registration.parameters.as_ref(), []);
        assert_eq!(registration.return_, HostValueType::Int);
        assert_eq!(
            implementation.call(&mut state, &CallArguments::new(Vec::new(), Vec::new())),
            Ok(BigInt::from(42)),
        );
        assert_eq!(state, 42);
    }

    #[test]
    fn supports_one_argument() {
        let registration = <_ as HostFunction<(BigInt,), BigInt>>::register(|a: BigInt| a + 1);

        assert_eq!(
            parameter_types(&registration.parameters),
            [HostValueType::Int]
        );
        assert_eq!(
            call_int(registration.implementation, vec![1.into()], Vec::new()),
            BigInt::from(2),
        );
    }

    #[test]
    fn supports_two_arguments() {
        let registration =
            <_ as HostFunction<(BigInt, BigInt), BigInt>>::register(|a: BigInt, b: BigInt| a - b);

        assert_eq!(
            parameter_types(&registration.parameters),
            [HostValueType::Int, HostValueType::Int],
        );
        assert_eq!(
            call_int(
                registration.implementation,
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
            parameter_types(&registration.parameters),
            [HostValueType::Bool, HostValueType::Int, HostValueType::Int],
        );
        assert_eq!(
            call_int(
                registration.implementation.clone(),
                vec![10.into(), 20.into()],
                vec![false],
            ),
            BigInt::from(20),
        );
        assert_eq!(
            call_int(
                registration.implementation,
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
            parameter_types(&registration.parameters),
            [
                HostValueType::Int,
                HostValueType::Bool,
                HostValueType::Int,
                HostValueType::Bool,
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
            parameter_types(&registration.parameters),
            [HostValueType::Int; 5],
        );
        assert_eq!(
            call_int(
                registration.implementation,
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
            parameter_types(&registration.parameters),
            [HostValueType::Bool; 6],
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
        >>::register::<StatelessHostProfile>(
            |a: BigInt, b: bool, c: BigInt, d: bool, e: BigInt, f: bool, g: BigInt| {
                let c = if b { c } else { BigInt::from(0) };
                let e = if d { e } else { BigInt::from(0) };
                let g = if f { g } else { BigInt::from(0) };
                a + c + e + g
            },
        );

        assert_eq!(
            parameter_types(&registration.parameters),
            [
                HostValueType::Int,
                HostValueType::Bool,
                HostValueType::Int,
                HostValueType::Bool,
                HostValueType::Int,
                HostValueType::Bool,
                HostValueType::Int,
            ],
        );
        assert_eq!(
            call_int(
                registration.implementation.clone(),
                vec![1.into(), 2.into(), 4.into(), 8.into()],
                vec![true, false, true],
            ),
            BigInt::from(11),
        );
        assert_eq!(
            call_int(
                registration.implementation,
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
        >>::register::<StatelessHostProfile>(
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
            parameter_types(&registration.parameters),
            [
                HostValueType::Int,
                HostValueType::Float,
                HostValueType::String,
                HostValueType::BitArray,
                HostValueType::UtfCodepoint,
                HostValueType::Bool,
                HostValueType::Nil,
            ],
        );
        assert_eq!(registration.return_, HostValueType::String);
        let implementation = string_implementation(registration.implementation);
        let arguments = CallArguments::new(vec![1.into()], vec![true]).with_scalar_values(
            vec![1.5],
            vec!["one".into()],
            vec![BitArrayValue::from_bytes(vec![0xff])],
            vec!['A'],
            1,
        );

        assert_eq!(
            implementation.call(&mut (), &arguments),
            Ok(EcoString::from("1:1.5:one:8:A:true")),
        );
    }

    #[test]
    #[should_panic(expected = "test function should return Int")]
    fn int_return_shape_guard_is_visible() {
        call_int(
            <_ as HostFunction<(), bool>>::register(<bool as Default>::default).implementation,
            Vec::new(),
            Vec::new(),
        );
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
        string_implementation(
            <_ as HostFunction<(), BigInt>>::register(<BigInt as Default>::default).implementation,
        );
    }

    fn parameter_types(parameters: &[HostParameter]) -> Vec<HostValueType> {
        parameters
            .iter()
            .map(|parameter| parameter.type_())
            .collect()
    }

    fn call_int(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> BigInt {
        int_function(implementation)
            .call(&mut (), &CallArguments::new(ints, bools))
            .expect("test host function should succeed")
    }

    fn call_bool(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> bool {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("test function should return Bool");
        };
        implementation
            .call(&mut (), &CallArguments::new(ints, bools))
            .expect("test host function should succeed")
    }

    fn string_implementation(
        implementation: HostFunctionImplementation<StatelessHostProfile>,
    ) -> HostStringFunction<StatelessHostProfile> {
        let HostFunctionImplementation::String(implementation) = implementation else {
            panic!("all-scalar test function should return String");
        };
        implementation
    }

    fn int_function<Profile: HostProfile>(
        implementation: HostFunctionImplementation<Profile>,
    ) -> HostIntFunction<Profile> {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("test function should return Int");
        };
        implementation
    }
}
