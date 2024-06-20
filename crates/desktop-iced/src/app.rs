/// Desktop application based on [`iced`].
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct App {
	page: crate::page::Page,
	settings: crate::settings::Settings,
	settings_path: std::path::PathBuf,
}

impl App {
	/// Ensures that the file with a valid settings exists. If the file does
	/// not exist or file is invalid, it is created with default settings.
	///
	/// # Return
	///
	/// Settings from the file and path to the settings file.
	fn ensure_settings_file() -> Result<
		(crate::settings::Settings, std::path::PathBuf),
		EnsureSettingsFileError,
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
		let default_settings = crate::settings::Settings::default();
		let saver = crate::settings::Saver::new(&path);
		saver.save(&default_settings)?;

		Ok((default_settings, path))
	}

	#[must_use]
	fn create_header(
		&self,
	) -> iced::widget::Column<'static, crate::message::Message> {
		let row = iced::widget::row![
			iced::widget::text("Sapphire 🔐")
				.size(self.settings.scale(17.0))
				// To enable emoji support.
				.shaping(iced::widget::text::Shaping::Advanced),
			self.create_header_start_button(),
			self.create_header_settings_button(),
			iced::widget::button(
				iced::widget::text("Exit").size(self.settings.scale(11.0))
			)
			.on_press(crate::message::Message::Exit),
		]
		.spacing(self.settings.scale(8.0));
		iced::widget::column![
			row,
			iced::widget::horizontal_rule(self.settings.scale(10.0)),
		]
		.spacing(self.settings.scale(8.0))
	}

	#[must_use]
	fn create_header_settings_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		let mut button = iced::widget::button(
			iced::widget::text("Settings").size(self.settings.scale(11.0)),
		);
		if self.page != crate::page::Page::Settings {
			button = button.on_press(crate::message::Message::Page(
				crate::page::Page::Settings,
			));
		}
		button
	}

	#[must_use]
	fn create_header_start_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		let mut button = iced::widget::button(
			iced::widget::text("Start").size(self.settings.scale(11.0)),
		);
		if self.page != crate::page::Page::Start {
			button = button.on_press(crate::message::Message::Page(
				crate::page::Page::Start,
			));
		}
		button
	}

	#[must_use]
	fn create_settings_page(&self) -> iced::Element<crate::message::Message> {
		let content = iced::widget::column![
			self.create_header(),
			self.create_settings_page_theme_list(),
			self.create_settings_page_scale_slider(),
			iced::widget::row![
				self.create_settings_page_save_button(),
				self.create_settings_page_restore_defaults_button(),
			]
			.spacing(self.settings.scale(8.0)),
		]
		.padding(self.settings.scale(10.0))
		.spacing(self.settings.scale(8.0));

		Into::into(content)
	}

	#[must_use]
	fn create_settings_page_scale_slider(
		&self,
	) -> iced::widget::Row<'static, crate::message::Message> {
		iced::widget::row![
			iced::widget::text("Interface scale:")
				.size(self.settings.scale(15.0)),
			iced::widget::slider(0.1..=3.0, self.settings.scale, |scale| {
				crate::message::Message::Scale(scale)
			})
			.step(0.1),
		]
		.spacing(self.settings.scale(8.0))
	}

	#[must_use]
	fn create_settings_page_restore_defaults_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Restore defaults")
				.size(self.settings.scale(11.0)),
		)
		.on_press(crate::message::Message::DefaultSettings)
	}

	#[must_use]
	fn create_settings_page_save_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Save").size(self.settings.scale(11.0)),
		)
		.on_press(crate::message::Message::SaveSettings)
	}

	#[must_use]
	fn create_settings_page_theme_list(
		&self,
	) -> iced::widget::Row<'static, crate::message::Message> {
		let list = iced::widget::pick_list(
			iced::Theme::ALL,
			Some(Clone::clone(&self.settings.theme)),
			crate::message::Message::Theme,
		)
		.placeholder("Pick a theme...")
		.text_size(self.settings.scale(11.0));
		iced::widget::row![
			iced::widget::text("Theme:").size(self.settings.scale(15.0)),
			list,
		]
		.spacing(self.settings.scale(8.0))
	}

	#[must_use]
	fn create_start_page(&self) -> iced::Element<crate::message::Message> {
		let content = iced::widget::column![
			self.create_header(),
			self.create_start_page_welcome(),
			iced::widget::row![
				self.create_start_page_login_button(),
				self.create_start_page_register_button(),
			]
			.spacing(self.settings.scale(8.0)),
		]
		.padding(self.settings.scale(10.0))
		.spacing(self.settings.scale(8.0));
		Into::into(content)
	}

	#[must_use]
	fn create_start_page_login_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Login").size(self.settings.scale(15.0)),
		)
	}

	#[must_use]
	fn create_start_page_register_button(
		&self,
	) -> iced::widget::Button<'static, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Register").size(self.settings.scale(15.0)),
		)
	}

	#[must_use]
	fn create_start_page_welcome(&self) -> iced::widget::Text<'static> {
		iced::widget::text(
			"A modern decentralized and private messenger with end-to-end encryption."
		)
		.size(self.settings.scale(11.0))
	}

	fn get_settings_save_command(
		&self,
	) -> iced::Command<crate::message::Message> {
		// Clone settings to satisfy lifetimes of command executor.
		let settings = Clone::clone(&self.settings);
		// Clone settings path to satisfy lifetimes of command executor.
		let path = Clone::clone(&self.settings_path);
		// Create saving future.
		let save = async move {
			let saver = crate::settings::Saver::new(path);
			saver.save_async(&settings).await
		};

		iced::Command::perform(save, |result| {
			// TODO: Use Message::Error on error + iced::Command
			result.expect("Failed to save settings.");
			crate::message::Message::None
		})
	}
}

impl iced::Application for App {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = crate::message::Message;
	type Theme = iced::Theme;

	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		// Ensure that file with a valid settings exists.
		let (settings, settings_path) = Self::ensure_settings_file()
			.expect("Failed to ensure settings file.");

		// Initialize the application.
		let app = Self {
			page: crate::page::Page::default(),
			settings,
			settings_path,
		};
		(app, iced::Command::none())
	}

	#[inline]
	#[must_use]
	fn theme(&self) -> Self::Theme {
		Clone::clone(&self.settings.theme)
	}

	#[inline]
	#[must_use]
	fn title(&self) -> String {
		ToOwned::to_owned("Sapphire")
	}

	fn update(
		&mut self,
		message: Self::Message,
	) -> iced::Command<Self::Message> {
		let mut commands = vec![];
		match message {
			Self::Message::DefaultSettings => {
				self.settings.restore_defaults();
			}
			Self::Message::Exit => {
				commands.push(iced::window::close(iced::window::Id::MAIN));
			}
			Self::Message::None => {}
			Self::Message::Page(page) => self.page = page,
			Self::Message::SaveSettings => {
				commands.push(self.get_settings_save_command());
			}
			Self::Message::Scale(scale) => self.settings.scale = scale,
			Self::Message::Theme(theme) => self.settings.theme = theme,
		}
		iced::Command::batch(commands)
	}

	#[must_use]
	fn view(&self) -> iced::Element<Self::Message> {
		match self.page {
			crate::page::Page::Settings => self.create_settings_page(),
			crate::page::Page::Start => self.create_start_page(),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum EnsureSettingsFileError {
	/// Failed to ensure that configuration directory exists.
	EnsureConfigDir(desktop_utils::config::EnsureDirError),
	/// Failed to save the default settings.
	SaveDefaults(Box<crate::settings::SaveError>),
}

impl From<crate::settings::SaveError> for EnsureSettingsFileError {
	#[inline]
	#[must_use]
	fn from(e: crate::settings::SaveError) -> Self {
		Self::SaveDefaults(Box::new(e))
	}
}

impl From<desktop_utils::config::EnsureDirError> for EnsureSettingsFileError {
	#[inline]
	#[must_use]
	fn from(e: desktop_utils::config::EnsureDirError) -> Self {
		Self::EnsureConfigDir(e)
	}
}

impl std::error::Error for EnsureSettingsFileError {
	#[must_use]
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::EnsureConfigDir(ref e) => Some(e),
			Self::SaveDefaults(e_boxed) => Some(e_boxed.as_ref()),
		}
	}
}

impl core::fmt::Display for EnsureSettingsFileError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::EnsureConfigDir(..) => {
				write!(
					f,
					"Failed to ensure that configuration directory exists."
				)
			}
			Self::SaveDefaults(..) => {
				write!(f, "Failed to save default settings.")
			}
		}
	}
}
