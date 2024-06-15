/// [`Saver`] implementation.
pub(crate) type SaverImpl = super::json::Saver;

/// Trait for settings savers.
pub(crate) trait Saver {
	type Error;

	/// Creates new settings saver.
	fn new(path: impl Into<std::path::PathBuf>) -> Self;

	/// Saves settings to the storage.
	async fn save(
		&self,
		settings: &super::Settings,
	) -> Result<(), Self::Error>;
}
