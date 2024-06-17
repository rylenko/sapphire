/// Trait for settings savers.
pub(crate) trait Saver {
	type Error;

	/// Creates new settings saver.
	///
	/// TODO: avoid PathBuf allocation.
	fn new(path: impl Into<std::path::PathBuf>) -> Self;

	/// Saves settings to the storage.
	async fn save(
		&self,
		settings: &super::Settings,
	) -> Result<(), Self::Error>;
}
