//! Shared static declarations. Preparation never imports the application bodies.
use geam::embedding::{BigInt, BindingError, HostPreparationBindings};
use geam::{
    HostCallableSchema, HostDeclarations, HostRegistrationError, HostReturns, HostTypeList,
    HostTypeListEnd,
};

pub struct AddOffset;
impl HostCallableSchema for AddOffset {
    const PACKAGE: &'static str = "geam_rust_embedding_callables";
    const MODULE: &'static str = "application/callables";
    const NAME: &'static str = "add_offset";
    type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
    type Return = BigInt;
    type Captures = HostTypeList<BigInt, HostTypeListEnd>;
    type Constructions = HostTypeListEnd;
    type Completion = HostReturns;
}

// These two functions are entry points of the independent preparation helper.
#[allow(dead_code)]
pub fn declare(base: HostDeclarations) -> Result<HostDeclarations, HostRegistrationError> {
    base.with_callable::<AddOffset>()
}

#[allow(dead_code)]
pub fn select(bindings: &mut HostPreparationBindings) -> Result<(), BindingError> {
    bindings.callable::<AddOffset>()
}
