mod decode;
mod encode;

pub(super) use decode::decode_to_dynamic;
pub(super) use encode::{
    do_bool, do_float, do_int, do_null, do_object, do_preprocessed_array, do_string, do_to_string,
    to_string_tree,
};

use super::GleamJsonHostProfile;
use crate::HostProvider;
use std::marker::PhantomData;

pub(super) struct JsonProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type State = Profile::RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        state
    }
}
