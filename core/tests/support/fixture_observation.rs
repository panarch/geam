pub(crate) const ENTRY: &str = "geam_transfer_fixture_entry";

pub(crate) fn source(original: &str) -> String {
    // Observe arbitrary fixture results through one ordinary, typed Nil entry.
    format!("{original}\n\npub fn {ENTRY}() {{\n  echo main()\n  Nil\n}}\n")
}
