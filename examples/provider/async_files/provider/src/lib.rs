#[geam::provider(package = "example_async_files", modules = [files])]
pub struct Component;

#[geam::module(path = "example_async_files")]
mod files {
    use geam::provider::StringValue;

    #[geam::function]
    async fn read(path: StringValue) -> Result<StringValue, StringValue> {
        async_fs::read_to_string(path.as_str())
            .await
            .map(StringValue::from)
            .map_err(|error| StringValue::from(error.to_string()))
    }
}
