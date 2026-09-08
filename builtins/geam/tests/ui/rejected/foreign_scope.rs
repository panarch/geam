use geam_core::embedding::{BigInt, ExecutionScope};
use geam_core::host::HostWorkProfile;
use geam_runtime_api::{FutureComponent, embedding::Future};

pub fn foreign<'a, 'b, P>(scope: &mut ExecutionScope<'a, '_, P>, work: Future<'b, BigInt>)
where P: HostWorkProfile<Work = FutureComponent> {
    let _observation = scope.observe(&work);
}
fn main() {}
