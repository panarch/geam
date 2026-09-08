use geam_core::embedding::{BigInt, ExecutionScope, FunctionDeclaration, HostedModuleBuilder, List};
use geam_core::host::{HostFuturePayload, HostWorkProfile};
use geam_builtin::{FutureComponent, embedding::{Future, FutureType}};

pub fn bind<P: HostWorkProfile<Work = FutureComponent>>(builder: HostedModuleBuilder<P>) {
    type Nested = Result<(List<FutureType<BigInt>>,), ()>;
    let _ = builder.function(FunctionDeclaration::<(), Nested>::new("nested"));
}

pub fn retain<'scope>(work: Future<'scope, BigInt>) -> Future<'scope, BigInt> { work }

pub fn observe<'scope, P>(scope: &mut ExecutionScope<'scope, '_, P>, work: Future<'scope, BigInt>)
where P: HostWorkProfile<Work = FutureComponent> {
    let _observation = scope.observe(&work);
}

pub fn send_only<P>(program: geam_core::frontend::HostedTypedProgram<P>)
where P: HostWorkProfile<RunState = std::cell::Cell<u32>>, P::ExternalStores: Send {
    let _ = HostedModuleBuilder::new(program);
}

pub fn same(left: &HostFuturePayload, right: &HostFuturePayload) -> bool {
    left.same_operation(right)
}
fn main() {}
