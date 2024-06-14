pub(crate) mod json;

/// Trait for settings loaders.
pub(crate) trait Loader {
	type Error;

	/// Loads settings to the storage.
	async fn load(&mut self) -> Result<Settings, Self::Error>;
}

/// Trait for settings savers.
pub(crate) trait Saver {
	type Error;

	/// Saves settings to the storage.
	async fn save(&mut self, settings: &Settings) -> Result<(), Self::Error>;
}

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
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Settings {
	#[serde(default = "Settings::default_scale")]
	pub(crate) scale: f32,
	#[serde(default = "Settings::default_theme", with = "theme_serde")]
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
		let theme = match string.as_str() {
			"CatppuccinFrappe" => iced::Theme::CatppuccinFrappe,
			"CatppuccinLatte" => iced::Theme::CatppuccinLatte,
			"CatppuccinMacchiato" => iced::Theme::CatppuccinMacchiato,
			"CatppuccinMocha" => iced::Theme::CatppuccinMocha,
			"Dark" => iced::Theme::Dark,
			"Dracula" => iced::Theme::Dracula,
			"GruvboxDark" => iced::Theme::GruvboxDark,
			"GruvboxLight" => iced::Theme::GruvboxLight,
			"KanagawaDragon" => iced::Theme::KanagawaDragon,
			"KanagawaLotus" => iced::Theme::KanagawaLotus,
			"KanagawaWave" => iced::Theme::KanagawaWave,
			"Light" => iced::Theme::Light,
			"Moonfly" => iced::Theme::Moonfly,
			"Nightfly" => iced::Theme::Nightfly,
			"Nord" => iced::Theme::Nord,
			"SolarizedDark" => iced::Theme::SolarizedDark,
			"SolarizedLight" => iced::Theme::SolarizedLight,
			"TokyoNight" => iced::Theme::TokyoNight,
			"TokyoNightLight" => iced::Theme::TokyoNightLight,
			"TokyoNightStorm" => iced::Theme::TokyoNightStorm,
			"Oxocarbon" => iced::Theme::Oxocarbon,
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
	fn test_serde_defaults() -> serde_json::Result<()> {
		const STR: &str = "{}";

		// Deserialize default settings struct from the string.
		let settings: super::Settings = serde_json::from_str(STR)?;
		assert!(
			(settings.scale - super::Settings::default_scale()).abs()
				< f32::EPSILON
		);
		assert_eq!(settings.theme, super::Settings::default_theme());
		Ok(())
	}

	#[test]
	fn test_serde_full() -> serde_json::Result<()> {
		const STR: &str = "{\"scale\":1.5,\"theme\":\"Dark\"}";

		// Deserialize settings struct from the string.
		let settings: super::Settings = serde_json::from_str(STR)?;
		assert!((settings.scale - 1.5).abs() < f32::EPSILON);
		assert_eq!(settings.theme, iced::Theme::Dark);

		// Serialize settings to the string.
		let string = serde_json::to_string(&settings)?;
		assert_eq!(string, STR);
		Ok(())
	}

	#[test]
	fn test_serde_invalid_theme() -> serde_json::Result<()> {
		const STR: &str = "{\"theme\":\"InvalidTheme\"}";

		// Deserialize settings struct from the string with an invalid theme.
		let settings: super::Settings = serde_json::from_str(STR)?;
		assert_eq!(settings.theme, super::Settings::default_theme());
		Ok(())
	}
}
