use super::type_::NativeType;
use crate::embedding::input::{InputConstructions, ScopedInputValue};
use crate::embedding::value::EmbeddingValue;
use crate::host::{HostTypeList, HostTypeListEnd, HostTypeSequence};
use crate::plan::{LibraryValueType, LibraryVariant, StandardVariant, ValueType};
use crate::runtime::{EmbeddingInputStorage, EmbeddingInputValue, RetainedInputs};
use std::sync::Arc;

#[allow(private_bounds)]
pub trait NativeCaptures: HostTypeSequence {
    type Shape: CaptureTypes;
}

pub(in crate::embedding) trait CaptureTypes {
    fn types(output: &mut Vec<ValueType>);
    fn variants(output: &mut Vec<LibraryVariant>);
    fn lists(output: &mut Vec<LibraryValueType>);
    fn standard(output: &mut Vec<StandardVariant>);
}

pub(in crate::embedding) trait CaptureInput<Input, Scope>:
    CaptureTypes
{
    fn owners_match(input: &Input, owner: &Arc<()>) -> bool;
    fn push(
        input: Input,
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
        output: &mut Option<RetainedInputs>,
    );
}

impl NativeCaptures for HostTypeListEnd {
    type Shape = ();
}
impl<Head: NativeType, Tail: NativeCaptures> NativeCaptures for HostTypeList<Head, Tail> {
    type Shape = (Head::Shape, Tail::Shape);
}

impl CaptureTypes for () {
    fn types(_: &mut Vec<ValueType>) {}
    fn variants(_: &mut Vec<LibraryVariant>) {}
    fn lists(_: &mut Vec<LibraryValueType>) {}
    fn standard(_: &mut Vec<StandardVariant>) {}
}
impl<Scope> CaptureInput<(), Scope> for () {
    fn owners_match(_: &(), _: &Arc<()>) -> bool {
        true
    }
    fn push(
        _: (),
        _: &mut InputConstructions<'_>,
        _: &EmbeddingInputStorage,
        _: &mut Option<RetainedInputs>,
    ) {
    }
}
impl<Head: EmbeddingValue, Tail: CaptureTypes> CaptureTypes for (Head, Tail) {
    fn types(output: &mut Vec<ValueType>) {
        output.push(Head::value_type());
        Tail::types(output);
    }
    fn variants(output: &mut Vec<LibraryVariant>) {
        Head::collect_input_variants(output);
        Tail::variants(output);
    }
    fn lists(output: &mut Vec<LibraryValueType>) {
        Head::collect_lists(output);
        Tail::lists(output);
    }
    fn standard(output: &mut Vec<StandardVariant>) {
        Head::collect_variants(output);
        Tail::standard(output);
    }
}
impl<Head, Tail, HeadInput, TailInput, Scope> CaptureInput<(HeadInput, TailInput), Scope>
    for (Head, Tail)
where
    Head: ScopedInputValue<HeadInput, Scope>,
    Tail: CaptureInput<TailInput, Scope>,
{
    fn owners_match(input: &(HeadInput, TailInput), owner: &Arc<()>) -> bool {
        Head::owners_match(&input.0, owner) && Tail::owners_match(&input.1, owner)
    }
    fn push(
        input: (HeadInput, TailInput),
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
        output: &mut Option<RetainedInputs>,
    ) {
        output
            .get_or_insert_with(RetainedInputs::empty)
            .push_input(Head::into_runtime(input.0, constructions, storage).into_input());
        Tail::push(input.1, constructions, storage, output);
    }
}
