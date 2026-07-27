use super::HostValueType;
use super::argument::{HostArgument, HostParameter, HostParameterLayout};
use super::return_::{HostFunctionImplementation, HostReturn};

pub trait HostFunction<Arguments, Return>: Send + Sync + 'static {
    fn register(self) -> HostFunctionRegistration;
}

pub struct HostFunctionRegistration {
    pub(super) parameters: Box<[HostParameter]>,
    pub(super) return_: HostValueType,
    pub(super) implementation: HostFunctionImplementation,
}

macro_rules! host_function {
    () => {
        impl<Function, Return> HostFunction<(), Return> for Function
        where
            Function: Fn() -> Return + Send + Sync + 'static,
            Return: HostReturn,
        {
            fn register(self) -> HostFunctionRegistration {
                HostFunctionRegistration {
                    parameters: Box::new([]),
                    return_: Return::type_(),
                    implementation: Return::implementation(move |_| self()),
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
            fn register(self) -> HostFunctionRegistration {
                let mut layout = HostParameterLayout::default();
                $(let $slot = layout.register::<$argument>();)*
                let implementation = Return::implementation(move |arguments| {
                    self($($argument::read(arguments, $slot)),*)
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
    use super::HostFunction;
    use crate::host::function::argument::{
        HostBoolArgumentSlot, HostCallArguments, HostIntArgumentSlot,
    };
    use crate::host::function::{HostFunctionImplementation, HostParameter, HostValueType};
    use num_bigint::BigInt;

    struct Arguments {
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    }

    impl HostCallArguments for Arguments {
        fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
            self.ints[slot.index()].clone()
        }

        fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
            self.bools[slot.index()]
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
        >>::register(
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

    fn parameter_types(parameters: &[HostParameter]) -> Vec<HostValueType> {
        parameters
            .iter()
            .map(|parameter| parameter.type_())
            .collect()
    }

    fn call_int(
        implementation: HostFunctionImplementation,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> BigInt {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("test function should return Int");
        };
        implementation.call(&Arguments { ints, bools })
    }

    fn call_bool(
        implementation: HostFunctionImplementation,
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    ) -> bool {
        let HostFunctionImplementation::Bool(implementation) = implementation else {
            panic!("test function should return Bool");
        };
        implementation.call(&Arguments { ints, bools })
    }
}
