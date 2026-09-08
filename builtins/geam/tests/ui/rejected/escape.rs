use geam_core::embedding::BigInt;
use geam_runtime_api::embedding::Future;

pub fn escape<'scope>(work: Future<'scope, BigInt>) -> Future<'static, BigInt> {
    work
}
fn main() {}
