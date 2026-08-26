#[geam_macros::module(path = "options", crate_path = geam_core)]
mod options {
    use num_bigint::BigInt;

    #[geam_macros::function]
    fn invalid() -> Option<BigInt, BigInt> {
        unreachable!()
    }
}

fn main() {}
