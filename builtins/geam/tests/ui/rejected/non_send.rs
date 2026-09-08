use geam_core::{embedding::WorkModuleBuilder, host::HostWorkProfile};

pub fn non_send<P>(program: geam_core::frontend::TransferHostedTypedProgram<P>)
where P: HostWorkProfile<RunState = std::rc::Rc<()>>, P::ExternalStores: Send {
    let _ = WorkModuleBuilder::new(program);
}
fn main() {}
