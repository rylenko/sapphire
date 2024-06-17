/// Trait for settings loaders.
pub(crate) trait Loader {
	type Error;

	/// Loads settings to the storage.
	async fn load(&self) -> Result<super::Settings, Self::Error>;
}
