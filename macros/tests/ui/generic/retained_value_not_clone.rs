use geam_core::provider::advanced::{Index0, Retained};

struct Payload {
    value: Retained<Payload, Index0>,
}

fn duplicate(payload: &Payload) {
    let _ = payload.value.clone();
}

fn main() {}
