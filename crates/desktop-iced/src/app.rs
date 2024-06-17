/// Desktop application based on [`iced`].
///
/// TODO: Load settings before window render.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct App {
	page: crate::page::Page,
	settings: crate::settings::Settings,
}

impl App {
	/// # Commands
	///
	/// 1. Command to async load settings. This command may fail if the file
	/// does not exist or has invalid data. In this case, the application
	/// must set the default settings.
	fn get_init_commands() -> iced::Command<crate::message::Message> {
		iced::Command::batch([Self::get_settings_load_command()])
	}

	fn get_settings_load_command() -> iced::Command<crate::message::Message> {
		// iced::Command's background task to load settings.
		let load_settings = async {
			// TODO: create loader during application initialization.
			let loader =
				crate::settings_json::Loader::new(&*crate::settings::PATH);
			crate::settings::Settings::load(&loader).await
		};
		iced::Command::perform(load_settings, |result| {
			crate::message::Message::Settings(
				// TODO: Use Message::Error on error and then log an
				// error.
				result.expect("Failed to load settings."),
			)
		})
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
		// iced::Command's background task to save settings. Settings must be
		// cloned there.
		let save = async |settings: crate::settings::Settings| {
			// Create settings saver and save current settings.
			// TODO: create saver during application initialization.
			let saver =
				crate::settings_json::Saver::new(&*crate::settings::PATH);
			settings.save(&saver).await
		};
		iced::Command::perform(
			// TODO: use std::sync::Arc<tokio::sync::Mutex<...>> if
			// settings become too big.
			save(Clone::clone(&self.settings)),
			|result| {
				// TODO: Use Message::Error on error + iced::Command
				result.expect("Failed to save settings.");
				crate::message::Message::None
			},
		)
	}
}

impl iced::Application for App {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = crate::message::Message;
	type Theme = iced::Theme;

	#[inline]
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		// Create the aplication.
		let app = Self {
			page: crate::page::Page::default(),
			settings: crate::settings::Settings::default(),
		};
		(app, Self::get_init_commands())
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
			Self::Message::Settings(settings) => self.settings = settings,
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
