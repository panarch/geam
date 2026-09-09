use geam_core::embedding::{BigInt, ExecutionScope, Function, FutureType, List};
use geam_core::host::{HostExternalSchema, HostWorkProfile};
use geam_builtin::FutureComponent;

pub struct Foreign;
impl HostExternalSchema for Foreign {
    const PACKAGE: &'static str = "foreign";
    const MODULE: &'static str = "foreign/work";
    const NAME: &'static str = "Future";
    const PARAMETER_COUNT: usize = 1;
}
type Nested = Result<(List<FutureType<BigInt, Foreign>>,), ()>;
pub fn call<P: HostWorkProfile<Work = FutureComponent>>(
    scope: &mut ExecutionScope<'_, '_, P>,
    function: &Function<(), Nested>,
) {
    let _ = scope.call(function, ());
}
fn main() {}
