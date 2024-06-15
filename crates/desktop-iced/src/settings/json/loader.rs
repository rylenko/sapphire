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

impl<P> crate::settings::Loader for Loader<P>
where
	P: AsRef<std::path::Path>,
{
	type Error = LoadError;

	async fn load(&self) -> Result<crate::settings::Settings, Self::Error> {
		// Read bytes from a file.
		let bytes = tokio::fs::read(&self.path).await?;
		// Deserialize bytes to settings structure.
		let settings = serde_json::from_slice(&bytes)?;
		Ok(settings)
	}
}
