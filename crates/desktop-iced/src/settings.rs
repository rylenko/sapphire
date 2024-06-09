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

mod theme_serde {
	pub(super) fn deserialize<'de, D>(
		deserializer: D,
	) -> Result<iced::Theme, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string: String = serde::Deserialize::deserialize(deserializer)?;
		let theme = match &string {
			_ => super::Settings::default_theme(),
		};
		Ok(theme)
	}

	pub(super) fn serialize<S>(
		theme: &iced::Theme,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let string = format!("{}", theme);
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
