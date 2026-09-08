use geam_core::embedding::{BigInt, FunctionDeclaration, WorkModuleBuilder, FutureType, List};
use geam_core::host::{HostExternalSchema, HostWorkProfile};
use geam_runtime_api::FutureComponent;

pub struct Foreign;
impl HostExternalSchema for Foreign {
    const PACKAGE: &'static str = "foreign";
    const MODULE: &'static str = "foreign/work";
    const NAME: &'static str = "Future";
    const PARAMETER_COUNT: usize = 1;
}
pub fn bind<P: HostWorkProfile<Work = FutureComponent>>(builder: WorkModuleBuilder<P>) {
    type Nested = Result<(List<FutureType<BigInt, Foreign>>,), ()>;
    let _ = builder.function(FunctionDeclaration::<(), Nested>::new("wrong"));
}
fn main() {}
