use geam::embedding::BigInt;
use std::cell::Cell;

// Caller-owned state needs Send, but not Sync or Clone.
#[derive(Default)]
pub struct Pricing {
    pub calls: Cell<usize>,
}

pub fn add_offset(state: &mut Pricing, value: BigInt, offset: BigInt) -> BigInt {
    state.calls.set(state.calls.get() + 1);
    value + offset
}
