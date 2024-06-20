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

impl std::error::Error for SaveError {
	#[must_use]
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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
pub(crate) struct Saver<P> {
	path: P,
}

impl<P> Saver<P> {
	/// Creates new saver.
	#[inline]
	#[must_use]
	pub(crate) const fn new(path: P) -> Self {
		Self { path }
	}
}

impl<P> Saver<P>
where
	P: AsRef<std::path::Path>,
{
	/// Saves current settings to the file.
	pub(crate) fn save(
		&self,
		settings: &super::Settings,
	) -> Result<(), SaveError> {
		// Serialize settings to bytes.
		let bytes = serde_json::to_vec_pretty(&settings)?;
		// Write bytes to the path.
		std::fs::write(&self.path, bytes)?;
		Ok(())
	}

	/// Asynchronously saves current settings to the file.
	pub(crate) async fn save_async(
		&self,
		settings: &super::Settings,
	) -> Result<(), SaveError> {
		// Serialize settings to bytes.
		let bytes = serde_json::to_vec_pretty(&settings)?;
		// Write bytes to the path.
		tokio::fs::write(&self.path, &bytes).await?;
		Ok(())
	}
}
