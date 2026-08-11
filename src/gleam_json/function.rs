mod decode;
mod encode;

pub(super) use decode::decode_to_dynamic;
pub(super) use encode::{
    do_bool, do_float, do_int, do_null, do_object, do_preprocessed_array, do_string, do_to_string,
    to_string_tree,
};

use super::schema::JsonSchema;
use super::storage::JsonStorage;
use super::{GleamJsonHostProfile, json_state};
use crate::gleam_stdlib::{
    DictExternalStorage, DictSchema, DynamicExternalStorage, DynamicSchema,
    StringTreeExternalStorage, StringTreeSchema,
};
use crate::{HostExternalBinding, HostProvider};
use std::marker::PhantomData;

pub(super) struct JsonProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type State = ();

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        json_state::<Profile>(state)
    }
}

impl<Profile> HostExternalBinding<Profile, JsonSchema> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type Storage = JsonStorage;
}

impl<Profile> HostExternalBinding<Profile, DynamicSchema> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type Storage = DynamicExternalStorage;
}

impl<Profile> HostExternalBinding<Profile, DictSchema> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type Storage = DictExternalStorage;
}

impl<Profile> HostExternalBinding<Profile, StringTreeSchema> for JsonProvider<Profile>
where
    Profile: GleamJsonHostProfile,
{
    type Storage = StringTreeExternalStorage;
}
