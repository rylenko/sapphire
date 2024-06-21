/*!
Utils for desktop applications to work with configuration directory.
*/

/// Configuration directory ensure error.
#[derive(Debug)]
#[non_exhaustive]
pub enum EnsureDirError {
	/// Failed to create configuration directory.
	CreateDir(std::io::Error),
	/// Failed to get root configuration directory.
	GetRootDir,
}

impl From<std::io::Error> for EnsureDirError {
	#[inline]
	#[must_use]
	fn from(e: std::io::Error) -> Self {
		Self::CreateDir(e)
	}
}

impl std::error::Error for EnsureDirError {
	#[must_use]
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::CreateDir(ref e) => Some(e),
			Self::GetRootDir => None,
		}
	}
}

impl core::fmt::Display for EnsureDirError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::CreateDir(..) => {
				write!(f, "failed to create configuration directory")
			}
			Self::GetRootDir => {
				write!(f, "failed to get root configuration directory")
			}
		}
	}
}

/// Ensures that the application's configuration directory exists.
///
/// Directory will be recursively created if does not exists.
///
/// # Return
///
/// Path to the confiduration directory.
///
/// # Errors
///
/// See [`EnsureDirError`].
pub fn ensure_dir() -> Result<std::path::PathBuf, EnsureDirError> {
	// Build configuration directory.
	let dir =
		dirs::config_dir().ok_or(EnsureDirError::GetRootDir)?.join("sapphire");

	// Create configuration directory recursively if does not exists.
	std::fs::create_dir_all(&dir)?;
	Ok(dir)
}
