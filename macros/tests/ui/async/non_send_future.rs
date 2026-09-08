use geam_core::provider::BigInt;

#[geam_macros::provider(
    package = "non_send_future",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "native", crate_path = geam_core)]
mod native {
    use super::BigInt;

    #[geam_macros::function]
    async fn read() -> BigInt {
        let value = std::rc::Rc::new(42);
        std::future::pending::<()>().await;
        BigInt::from(*value)
    }
}

fn main() {}
