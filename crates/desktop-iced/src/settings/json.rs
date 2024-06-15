/// Settings loading error.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum LoadError {
	/// Failed to deserialize bytes into structure.
	Deserialize(serde_json::Error),
	/// Failed to read a settings from the file.
	Read(std::io::Error),
}

impl From<serde_json::Error> for LoadError {
	#[inline]
	#[must_use]
	fn from(e: serde_json::Error) -> Self {
		Self::Deserialize(e)
	}
}

impl From<std::io::Error> for LoadError {
	#[inline]
	#[must_use]
	fn from(e: std::io::Error) -> Self {
		Self::Read(e)
	}
}

impl core::error::Error for LoadError {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Deserialize(ref e) => Some(e),
			Self::Read(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for LoadError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Deserialize(..) => {
				write!(f, "Failed to deserialize bytes into structure.")
			}
			Self::Read(..) => {
				write!(f, "Failed to read a settings from the file.")
			}
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct Loader<P>
where
	P: AsRef<std::path::Path>,
{
	path: P,
}

impl<P> Loader<P>
where
	P: AsRef<std::path::Path>,
{
	#[inline]
	#[must_use]
	pub(crate) const fn new(path: P) -> Self {
		Self { path }
	}
}

impl<P> super::Loader for Loader<P>
where
	P: AsRef<std::path::Path>,
{
	type Error = LoadError;

	async fn load(&self) -> Result<super::Settings, Self::Error> {
		// Read bytes from a file.
		let bytes = tokio::fs::read(&self.path).await?;
		// Deserialize bytes to settings structure.
		let settings = serde_json::from_slice(&bytes)?;
		Ok(settings)
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum SaveError {
	/// Failed to serialize structure to bytes.
	Serialize(serde_json::Error),
	/// Failed to write a settings to the file.
	Write(std::io::Error),
}

impl From<serde_json::Error> for SaveError {
	#[inline]
	#[must_use]
	fn from(e: serde_json::Error) -> Self {
		Self::Serialize(e)
	}
}

impl From<std::io::Error> for SaveError {
	#[inline]
	#[must_use]
	fn from(e: std::io::Error) -> Self {
		Self::Write(e)
	}
}

impl core::error::Error for SaveError {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Serialize(ref e) => Some(e),
			Self::Write(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for SaveError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Serialize(..) => {
				write!(f, "Failed to serialize settings to bytes.")
			}
			Self::Write(..) => {
				write!(f, "Failed to write a settings to the file.")
			}
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct Saver<P>
where
	P: AsRef<std::path::Path>,
{
	path: P,
}

impl<P> Saver<P>
where
	P: AsRef<std::path::Path>,
{
	#[inline]
	#[must_use]
	pub(crate) const fn new(path: P) -> Self {
		Self { path }
	}
}

impl<P> super::Saver for Saver<P>
where
	P: AsRef<std::path::Path>,
{
	type Error = SaveError;

	async fn save(
		&self,
		settings: &super::Settings,
	) -> Result<(), Self::Error> {
		// Serialize settings to bytes.
		let bytes = serde_json::to_vec_pretty(&settings)?;
		// Write bytes to the path.
		tokio::fs::write(&self.path, &bytes).await?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[tokio::test]
	async fn test_loader_and_saver() {
		// Create test settings.
		let settings =
			crate::settings::Settings::new(1.25, iced::Theme::Light);

		// Build test path to the settings.
		let mut path = std::env::temp_dir();
		path.push("sapphire-settings-test");
		path.set_extension("json");

		// Save the settings.
		let saver = super::Saver::new(&path);
		assert!(crate::settings::Saver::save(&saver, &settings).await.is_ok());

		// Test loader.
		let loader = super::Loader::new(&path);
		assert_eq!(
			crate::settings::Loader::load(&loader).await.unwrap(),
			settings
		);
	}
}
