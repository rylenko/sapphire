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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct Saver {
	path: std::path::PathBuf,
}

impl crate::settings::Saver for Saver {
	type Error = SaveError;

	#[inline]
	#[must_use]
	fn new(path: impl Into<std::path::PathBuf>) -> Self {
		Self { path: Into::into(path) }
	}

	async fn save(
		&self,
		settings: &crate::settings::Settings,
	) -> Result<(), Self::Error> {
		// Serialize settings to bytes.
		let bytes = serde_json::to_vec_pretty(&settings)?;
		// Write bytes to the path.
		tokio::fs::write(&self.path, &bytes).await?;
		Ok(())
	}
}
