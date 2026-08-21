mod function;
mod schema;

use self::function::{
    BitArrayProvider, base16_decode, base16_encode, base64_encode, bit_array_to_int_and_size,
    bit_size, byte_size, concat, decode64, from_string, pad_to_bytes, slice, unsafe_to_string,
};
use self::schema::{BitArrayList, BitArrayResult, IntPair};
use super::GleamStdlibHostProfile;
use crate::{BitArrayValue, HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/bit_array")
        .and_then(|provider| provider.with_function("from_string", from_string))
        .and_then(|provider| provider.with_function("bit_size", bit_size))
        .and_then(|provider| provider.with_function("byte_size", byte_size))
        .and_then(|provider| provider.with_function("pad_to_bytes", pad_to_bytes))
        .and_then(|provider| {
            provider.with_scoped_function::<
                BitArrayProvider<Profile>,
                (BitArrayValue, BigInt, BigInt),
                BitArrayResult,
                _,
            >("slice", slice::<Profile>)
        })
        .and_then(|provider| {
            provider.with_fallible_function::<(BitArrayValue,), EcoString, _>(
                "unsafe_to_string",
                unsafe_to_string,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                BitArrayProvider<Profile>,
                (BitArrayList,),
                BitArrayValue,
                _,
            >("concat", concat::<Profile>)
        })
        .and_then(|provider| provider.with_function("base64_encode", base64_encode))
        .and_then(|provider| {
            provider
                .with_scoped_function::<BitArrayProvider<Profile>, (EcoString,), BitArrayResult, _>(
                    "decode64",
                    decode64::<Profile>,
                )
        })
        .and_then(|provider| provider.with_function("base16_encode", base16_encode))
        .and_then(|provider| {
            provider
                .with_scoped_function::<BitArrayProvider<Profile>, (EcoString,), BitArrayResult, _>(
                    "base16_decode",
                    base16_decode::<Profile>,
                )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<BitArrayProvider<Profile>, (BitArrayValue,), IntPair, _>(
                    "bit_array_to_int_and_size",
                    bit_array_to_int_and_size::<Profile>,
                )
        })
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;

    const BIT_ARRAY_DECLARATIONS: &str = r#"
@external(erlang, "host", "from_string")
fn from_string(value: String) -> BitArray

@external(erlang, "host", "bit_size")
fn bit_size(value: BitArray) -> Int

@external(erlang, "host", "byte_size")
fn byte_size(value: BitArray) -> Int

@external(erlang, "host", "pad_to_bytes")
fn pad_to_bytes(value: BitArray) -> BitArray

@external(erlang, "host", "slice")
fn slice(value: BitArray, position: Int, length: Int) -> Result(BitArray, Nil)

@external(erlang, "host", "unsafe_to_string")
fn unsafe_to_string(value: BitArray) -> String

@external(erlang, "host", "concat")
fn concat(values: List(BitArray)) -> BitArray

@external(erlang, "host", "base64_encode")
fn base64_encode(value: BitArray, padding: Bool) -> String

@external(erlang, "host", "decode64")
fn decode64(value: String) -> Result(BitArray, Nil)

@external(erlang, "host", "base16_encode")
fn base16_encode(value: BitArray) -> String

@external(erlang, "host", "base16_decode")
fn base16_decode(value: String) -> Result(BitArray, Nil)

@external(erlang, "host", "bit_array_to_int_and_size")
fn bit_array_to_int_and_size(value: BitArray) -> #(Int, Int)
"#;

    #[test]
    fn registers_the_exact_official_bit_array_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official bit array provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/bit_array");
        assert_eq!(provider.external_types().count(), 0);
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "from_string",
                "bit_size",
                "byte_size",
                "pad_to_bytes",
                "slice",
                "unsafe_to_string",
                "concat",
                "base64_encode",
                "decode64",
                "base16_encode",
                "base16_decode",
                "bit_array_to_int_and_size",
            ],
        );
    }

    #[test]
    fn executes_every_bit_array_provider_through_the_hosted_pipeline() {
        let source = format!(
            r#"{BIT_ARRAY_DECLARATIONS}

pub fn main() {{
  assert from_string("AB") == <<65, 66>>
  assert bit_size(<<5:size(3)>>) == 3
  assert byte_size(<<5:size(3)>>) == 1
  assert pad_to_bytes(<<5:size(3)>>) == <<5:size(3), 0:size(5)>>
  assert slice(<<1, 2, 3>>, 1, 1) == Ok(<<2>>)
  assert slice(<<1, 2, 3>>, 2, -1) == Ok(<<2>>)
  assert slice(<<1>>, 0, -1) == Error(Nil)
  assert slice(<<1, 2, 3>>, 4, 1) == Error(Nil)
  assert slice(<<1:size(2)>>, 0, 0) == Error(Nil)
  assert unsafe_to_string(<<65, 66>>) == "AB"
  assert concat([<<1>>, <<2:size(2)>>]) == <<1, 2:size(2)>>
  assert base64_encode(<<5:size(3)>>, True) == "oA=="
  assert base64_encode(<<5:size(3)>>, False) == "oA"
  assert decode64("AQ==") == Ok(<<1>>)
  assert decode64("AB==") == Ok(<<0>>)
  assert byte_size(from_string("aG  \t\nVsbG8=")) == 12
  assert decode64("aG  \t\nVsbG8=") == Ok(<<"hello":utf8>>)
  assert decode64("***=") == Error(Nil)
  assert base16_encode(<<5:size(3)>>) == "A0"
  assert base16_decode("01") == Ok(<<1>>)
  assert base16_decode("GG") == Error(Nil)
  bit_array_to_int_and_size(<<5:size(3)>>)
}}
"#,
        );
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official bit array provider should register");
        let hosts = HostProviderSet::with_providers(
            Vec::<HostModule<GleamStdlibProfile>>::new(),
            [provider],
        )
        .expect("bit array provider module should be unique");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/bit_array",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/bit_array",
                    "src/gleam/bit_array.gleam",
                    source,
                )],
            )],
            hosts,
        )
        .expect("synthetic bit array source should compile");
        let plan = plan_host_program(typed).expect("synthetic bit array source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("bit array execution should seal");
        let value = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("bit array providers should run");

        assert_eq!(value.inspect().to_string(), "#(5, 3)");
    }
}
