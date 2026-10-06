#[geam::provider(package = "raw_string_values_fixture", modules = [native])]
pub struct Component;

#[geam::module(path = "raw_string_values_fixture")]
mod native {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use geam::provider::{BigInt, StringValue, Value};
    use sha1::{Digest, Sha1};

    #[geam::custom(input = ShaHashInput)]
    #[allow(dead_code)]
    pub enum ShaHash {
        Sha1,
    }

    #[geam::function]
    fn identity<Item>(value: Value<Item>) -> Value<Item> {
        value
    }

    #[geam::function]
    fn crypto_hash(hash: ShaHashInput, data: StringValue) -> StringValue {
        match hash {
            ShaHashInput::Sha1 => StringValue::from_bytes(Sha1::digest(data.as_bytes()).to_vec()),
        }
    }

    #[geam::function]
    fn base64_encode(data: StringValue) -> StringValue {
        STANDARD.encode(data.as_bytes()).into()
    }

    #[geam::function]
    fn sample(index: BigInt) -> Result<StringValue, ()> {
        let index = usize::try_from(index).map_err(|_| ())?;
        let bytes = match index {
            0 => vec![],
            1 => vec![0],
            2 => vec![0x80],
            3 => vec![0xff, 0xfe],
            4 => vec![0xc3],
            5 => "é🙂".as_bytes().to_vec(),
            _ => return Err(()),
        };
        Ok(StringValue::from_bytes(bytes))
    }

    #[geam::function]
    fn byte_slice(value: StringValue, start: BigInt, length: BigInt) -> Result<StringValue, ()> {
        let start = usize::try_from(start).map_err(|_| ())?;
        let length = usize::try_from(length).map_err(|_| ())?;
        let end = start.checked_add(length).ok_or(())?;
        value.get(start..end).ok_or(())
    }

    #[geam::function]
    fn bytes(value: StringValue) -> Vec<BigInt> {
        value.as_bytes().iter().copied().map(BigInt::from).collect()
    }
}
