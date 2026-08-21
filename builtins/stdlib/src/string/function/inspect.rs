use super::super::schema::InspectValue;
use super::StringProvider;
use crate::GleamStdlibHostProfile;
use crate::string_tree::{StoredStringTree, StringTree, StringTreePayload};
use crate::{HostCall, HostCallCompletion, HostCallError, HostValue};

pub(in crate::string) fn do_inspect<'call, Profile>(
    mut call: HostCall<'call, Profile, StringProvider<Profile>, StringTree>,
    value: HostValue<'call, InspectValue>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let inspection = call.inspect::<InspectValue>(value);
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::text(inspection),
    });
    Ok(call.return_value(value))
}
