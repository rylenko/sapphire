/// Settings of the desktop application.
///
/// You can create [new] settings, get [default] settings, [restore default]
/// settings after changing, [serialize] or [deserialize] settings. All
/// settings are available directly as structure fields.
///
/// [new]: Self::new
/// [default]: Default::default
/// [restore default]: Self::restore_defaults
/// [serialize]: serde::Serialize
/// [deserialize]: serde::Deserialize
///
/// TODO: Add SettingsStorage trait and SettingsFileStorage implementation?
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Settings {
	#[serde(default = "Settings::default_scale")]
	pub(crate) scale: f32,
	#[serde(with = "theme_serde")]
	pub(crate) theme: iced::Theme,
}

impl Settings {
	/// Creates new settings.
	#[inline]
	#[must_use]
	pub(crate) fn new(scale: f32, theme: iced::Theme) -> Self {
		Self { scale, theme }
	}

	/// Default interface scale.
	#[inline]
	#[must_use]
	fn default_scale() -> f32 {
		1.0
	}

	/// Default theme.
	#[inline]
	#[must_use]
	fn default_theme() -> iced::Theme {
		iced::Theme::Dark
	}

	/// Restores default settings.
	#[inline]
	pub(crate) fn restore_defaults(&mut self) {
		*self = Self::default();
	}

	/// Scales passed size to interface size using coefficient from the
	/// settings.
	#[must_use]
	#[inline]
	pub(crate) fn scale(&self, size: f32) -> f32 {
		size * self.scale
	}
}

impl Default for Settings {
	#[inline]
	#[must_use]
	fn default() -> Self {
		Self::new(Self::default_scale(), Self::default_theme())
	}
}

/// Module to serialize and deserialize [themes].
///
/// [themes]: iced::Theme
mod theme_serde {
	/// Deserializes theme using a [`String`] from passed [deserializer].
	///
	/// [deserializer]: serde::Deserializer
	pub(super) fn deserialize<'de, D>(
		deserializer: D,
	) -> Result<iced::Theme, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// Deserialize a string using accepted deserializer.
		let string: String = serde::Deserialize::deserialize(deserializer)?;

		// Get theme using deserialized string.
		let theme = match &string {
			_ => super::Settings::default_theme(),
		};
		Ok(theme)
	}

	/// Serializes [theme] to the string using passed [serializer].
	///
	/// [theme]: iced::Theme
	/// [serializer]: serde::Serializer
	pub(super) fn serialize<S>(
		theme: &iced::Theme,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		// Format theme variant to string.
		let string = format!("{theme}");
		// Serialize theme string using accepted serializer.
		serializer.serialize_str(&string)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_serde() {
		todo!("Write serialization and deserialization tests.");
	}
}
