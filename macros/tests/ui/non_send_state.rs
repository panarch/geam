use std::rc::Rc;

#[geam_macros::provider(
    package = "non_send_state",
    modules = [state],
    state = Rc<()>,
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "state", crate_path = geam_core)]
mod state {
    #[geam_macros::function]
    fn ready() -> bool { true }
}

fn main() {}
