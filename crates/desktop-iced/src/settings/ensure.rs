#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum EnsureFileError {
	/// Failed to ensure that configuration directory exists.
	EnsureConfigDir(desktop_utils::config::EnsureDirError),
	/// Failed to save the default settings.
	SaveDefaults(Box<crate::settings::SaveError>),
}

impl From<crate::settings::SaveError> for EnsureFileError {
	#[inline]
	#[must_use]
	fn from(e: crate::settings::SaveError) -> Self {
		Self::SaveDefaults(Box::new(e))
	}
}

impl From<desktop_utils::config::EnsureDirError> for EnsureFileError {
	#[inline]
	#[must_use]
	fn from(e: desktop_utils::config::EnsureDirError) -> Self {
		Self::EnsureConfigDir(e)
	}
}

impl std::error::Error for EnsureFileError {
	#[must_use]
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::EnsureConfigDir(ref e) => Some(e),
			Self::SaveDefaults(e_boxed) => Some(e_boxed.as_ref()),
		}
	}
}

impl core::fmt::Display for EnsureFileError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::EnsureConfigDir(..) => {
				write!(
					f,
					"failed to ensure that configuration directory exists"
				)
			}
			Self::SaveDefaults(..) => {
				write!(f, "failed to save default settings")
			}
		}
	}
}

/// Ensures that the file with a valid settings exists. If the file does
/// not exist or file is invalid, it is created with default settings.
///
/// # Return
///
/// Settings from the file and path to the settings file.
pub(crate) fn ensure_file() -> Result<
	(super::model::Settings, std::path::PathBuf),
	EnsureFileError,
> {
	// Build settings path.
	let path = desktop_utils::config::ensure_dir()?.join("desktop-iced");

	// Try to load settings from the file if exists.
	if path.exists() {
		let loader = crate::settings::Loader::new(&path);
		if let Ok(settings) = loader.load() {
			return Ok((settings, path));
		}
	}

	// Create a file with the default settings if file is invalid or does
	// not exists.
	let default_settings = super::model::Settings::default();
	let saver = super::saver::Saver::new(&path);
	saver.save(&default_settings)?;

	Ok((default_settings, path))
}
