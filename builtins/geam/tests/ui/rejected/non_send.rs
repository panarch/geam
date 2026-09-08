use geam_core::HostProfile;

struct NonSend;
impl HostProfile for NonSend {
    type RunState = std::rc::Rc<()>;
    type ExternalStores = ();
}
fn main() {}
