mod loader;
mod saver;

pub(crate) use {loader::Loader, saver::Saver};

#[cfg(test)]
mod tests {
	#[tokio::test]
	async fn test_loader_and_saver() {
		// Create test settings.
		let settings =
			crate::settings::Settings::new(1.25, iced::Theme::Light);
		// Build test path to the settings.
		let path = std::env::temp_dir().join("sapphire-settings-test");

		// Save the settings.
		let saver: super::saver::Saver = crate::settings::Saver::new(&path);
		assert!(crate::settings::Saver::save(&saver, &settings).await.is_ok());

		// Test loader.
		let loader: super::loader::Loader =
			crate::settings::Loader::new(&path);
		assert_eq!(
			crate::settings::Loader::load(&loader).await.unwrap(),
			settings
		);
	}
}
