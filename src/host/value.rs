use crate::BitArrayValue;
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct HostValueToken {
    pub family: HostValueFamily,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HostValueFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List,
    Tuple,
    Custom,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostListToken {
    Parameter(usize),
    Stored(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostTupleToken(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostCustomToken(pub usize);

/// A call-scoped value whose concrete runtime family is selected by `Type`.
///
/// This handle is used for generic parameters and cannot outlive its active
/// [`crate::HostCall`].
pub struct HostValue<'call, Type> {
    pub(crate) token: HostValueToken,
    marker: PhantomData<&'call Type>,
}

/// A call-scoped Gleam list with the statically declared `Item` ABI type.
pub struct HostList<'call, Item> {
    pub(crate) token: HostListToken,
    marker: PhantomData<&'call Item>,
}

/// A call-scoped Gleam tuple described by a recursive host type sequence.
pub struct HostTuple<'call, Elements> {
    pub(crate) token: HostTupleToken,
    marker: PhantomData<&'call Elements>,
}

/// A call-scoped ordinary Gleam custom value with a validated custom schema.
pub struct HostCustom<'call, Custom> {
    pub(crate) token: HostCustomToken,
    marker: PhantomData<&'call Custom>,
}

/// A typed value completed by one active [`crate::HostCall`].
///
/// The completion cannot be retained beyond the invocation that owns its
/// runtime value tokens.
///
/// ```compile_fail
/// use geam::HostCallCompletion;
/// use num_bigint::BigInt;
///
/// fn escape<'call>(
///     completion: HostCallCompletion<'call, BigInt>,
/// ) -> HostCallCompletion<'static, BigInt> {
///     completion
/// }
/// ```
pub struct HostCallCompletion<'call, Return> {
    pub(crate) token: HostValueToken,
    call: PhantomData<&'call mut ()>,
    return_: PhantomData<fn() -> Return>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum HostScopedValue {
    Int(BigInt),
    Float(f64),
    String(EcoString),
    BitArray(BitArrayValue),
    UtfCodepoint(char),
    Bool(bool),
    Nil,
    Value(HostValueToken),
    List(HostListToken),
    Tuple(HostTupleToken),
    Custom(HostCustomToken),
}

impl<'call, Type> HostValue<'call, Type> {
    pub(crate) fn new(token: HostValueToken) -> Self {
        Self {
            token,
            marker: PhantomData,
        }
    }
}

impl<Type> Clone for HostValue<'_, Type> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Type> Copy for HostValue<'_, Type> {}

impl<'call, Item> HostList<'call, Item> {
    pub(crate) fn new(token: HostListToken) -> Self {
        Self {
            token,
            marker: PhantomData,
        }
    }
}

impl<Item> Clone for HostList<'_, Item> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Item> Copy for HostList<'_, Item> {}

impl<'call, Elements> HostTuple<'call, Elements> {
    pub(crate) fn new(token: HostTupleToken) -> Self {
        Self {
            token,
            marker: PhantomData,
        }
    }
}

impl<Elements> Clone for HostTuple<'_, Elements> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Elements> Copy for HostTuple<'_, Elements> {}

impl<'call, Custom> HostCustom<'call, Custom> {
    pub(crate) fn new(token: HostCustomToken) -> Self {
        Self {
            token,
            marker: PhantomData,
        }
    }
}

impl<Custom> Clone for HostCustom<'_, Custom> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Custom> Copy for HostCustom<'_, Custom> {}

impl<'call, Return> HostCallCompletion<'call, Return> {
    pub(crate) fn new(token: HostValueToken) -> Self {
        Self {
            token,
            call: PhantomData,
            return_: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostCustom, HostCustomToken, HostList, HostListToken, HostTuple, HostTupleToken, HostValue,
        HostValueFamily, HostValueToken,
    };

    #[test]
    fn scoped_handles_preserve_only_the_call_owned_token() {
        fn clone_handle<Handle: Clone>(handle: &Handle) -> Handle {
            handle.clone()
        }

        let value = HostValue::<bool>::new(HostValueToken {
            family: HostValueFamily::Bool,
            index: 1,
        });
        let list = HostList::<bool>::new(HostListToken::Stored(2));
        let tuple = HostTuple::<bool>::new(HostTupleToken(3));
        let custom = HostCustom::<bool>::new(HostCustomToken(4));
        let copied = value;
        let cloned_value = clone_handle(&value);
        let cloned_list = clone_handle(&list);
        let cloned_tuple = clone_handle(&tuple);
        let cloned_custom = clone_handle(&custom);

        assert_eq!(
            value.token,
            HostValueToken {
                family: HostValueFamily::Bool,
                index: 1,
            },
        );
        assert_eq!(copied.token, value.token);
        assert_eq!(cloned_value.token, value.token);
        assert_eq!(list.token, HostListToken::Stored(2));
        assert_eq!(cloned_list.token, list.token);
        assert_eq!(tuple.token, HostTupleToken(3));
        assert_eq!(cloned_tuple.token, tuple.token);
        assert_eq!(custom.token, HostCustomToken(4));
        assert_eq!(cloned_custom.token, custom.token);
    }
}
