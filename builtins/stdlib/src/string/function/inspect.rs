use crate::string_tree::{StoredStringTree, StringTreePayload};
use ecow::EcoString;

pub(in crate::string) fn do_inspect(inspection: EcoString) -> StringTreePayload {
    StringTreePayload::from_stored(StoredStringTree::text(inspection))
}
