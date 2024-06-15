/// Trait for settings savers.
pub(crate) trait Saver {
	type Error;

	/// Saves settings to the storage.
	async fn save(
		&self,
		settings: &super::Settings,
	) -> Result<(), Self::Error>;
}
