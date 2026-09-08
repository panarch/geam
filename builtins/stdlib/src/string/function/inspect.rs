use crate::string_tree::{StoredStringTree, StringTreePayload};
use ecow::EcoString;

pub(in crate::string) fn do_inspect<Context: crate::storage::StorageContext>(
    inspection: EcoString,
) -> StringTreePayload<Context> {
    StringTreePayload::from_stored(StoredStringTree::text(inspection))
}
