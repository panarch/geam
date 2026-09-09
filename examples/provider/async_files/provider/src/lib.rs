#[geam::provider(package = "example_async_files", modules = [files])]
pub struct Component;

#[geam::module(path = "example_async_files")]
mod files {
    use geam::provider::EcoString;

    #[geam::function]
    async fn read(path: EcoString) -> Result<EcoString, EcoString> {
        async_fs::read_to_string(path.as_str())
            .await
            .map(EcoString::from)
            .map_err(|error| EcoString::from(error.to_string()))
    }
}
