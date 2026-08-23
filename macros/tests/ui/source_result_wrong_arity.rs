#[geam_macros::module(path = "results", crate_path = geam_core)]
mod results {
    use num_bigint::BigInt;

    #[geam_macros::function]
    fn invalid() -> Result<BigInt> {
        unreachable!()
    }
}

fn main() {}
