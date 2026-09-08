use geam_core::embedding::BigInt;
use geam_builtin::embedding::Future;

pub fn escape<'scope>(work: Future<'scope, BigInt>) -> Future<'static, BigInt> {
    work
}
fn main() {}
