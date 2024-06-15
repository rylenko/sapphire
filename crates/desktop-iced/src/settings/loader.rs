/// [`Loader`] implementation.
pub(crate) type LoaderImpl = super::json::Loader;

/// Trait for settings loaders.
pub(crate) trait Loader {
	type Error;

	/// Creates new settings loader.
	///
	/// TODO: avoid PathBuf allocation.
	fn new(path: impl Into<std::path::PathBuf>) -> Self;

	/// Loads settings to the storage.
	async fn load(&self) -> Result<super::Settings, Self::Error>;
}
