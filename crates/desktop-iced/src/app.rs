/// Desktop application based on [`iced`].
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct App {
	page: crate::page::Page,
	flashes: Vec<crate::flash::Flash>,
	settings: crate::settings::Settings,
	settings_path: std::path::PathBuf,
}

impl App {
	#[must_use]
	fn create_exit_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Exit").size(self.settings.scale(11.0)),
		)
		.on_press(crate::message::Message::Exit)
	}

	#[must_use]
	fn create_flash_row(
		&self,
	) -> Option<iced::widget::Row<'_, crate::message::Message>> {
		self.flashes.first().map(|flash| {
			iced::widget::row![
				iced::widget::text(flash.as_str())
					.size(self.settings.scale(10.0))
					.style(flash.color()),
				iced::widget::button(
					iced::widget::text("✖").size(self.settings.scale(5.0))
				)
				.on_press(crate::message::Message::RemoveFlash),
			]
			.spacing(self.settings.scale(4.0))
		})
	}

	#[must_use]
	fn create_header(
		&self,
	) -> iced::widget::Column<'_, crate::message::Message> {
		// Create row with title and main page buttons.
		let row = iced::widget::row![
			self.create_title(),
			self.create_start_page_button(),
			self.create_settings_page_button(),
			self.create_exit_button(),
		]
		.spacing(self.settings.scale(8.0));

		// Create complete header widget.
		let mut header =
			iced::widget::column![row].spacing(self.settings.scale(8.0));
		// Push flash row.
		header = header.push_maybe(self.create_flash_row());
		// Push horizontal line to split header and other content.
		header = header
			.push(iced::widget::horizontal_rule(self.settings.scale(10.0)));
		header
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
	fn create_settings_page_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
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
	fn create_settings_page_scale_slider(
		&self,
	) -> iced::widget::Row<'_, crate::message::Message> {
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
	) -> iced::widget::Button<'_, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Restore defaults")
				.size(self.settings.scale(11.0)),
		)
		.on_press(crate::message::Message::DefaultSettings)
	}

	#[must_use]
	fn create_settings_page_save_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Save").size(self.settings.scale(11.0)),
		)
		.on_press(crate::message::Message::SaveSettings)
	}

	#[must_use]
	fn create_settings_page_theme_list(
		&self,
	) -> iced::widget::Row<'_, crate::message::Message> {
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
	fn create_start_page_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
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
	fn create_start_page_login_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Login").size(self.settings.scale(15.0)),
		)
	}

	#[must_use]
	fn create_start_page_register_button(
		&self,
	) -> iced::widget::Button<'_, crate::message::Message> {
		iced::widget::button(
			iced::widget::text("Register").size(self.settings.scale(15.0)),
		)
	}

	#[must_use]
	fn create_start_page_welcome(&self) -> iced::widget::Text<'_> {
		iced::widget::text(
			"A modern decentralized and private messenger with end-to-end encryption."
		)
		.size(self.settings.scale(11.0))
	}

	#[must_use]
	fn create_title(&self) -> iced::widget::Text {
		iced::widget::text("Sapphire 🔐")
			.size(self.settings.scale(17.0))
			// To enable emoji support.
			.shaping(iced::widget::text::Shaping::Advanced)
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
			// Ensure that configuration directory exists or return error string.
			desktop_utils::config::ensure_dir().map_err(|e| format!("{e}"))?;

			// Save settings to the path or return error string.
			let saver = crate::settings::Saver::new(path);
			saver.save_async(&settings).await.map_err(|e| format!("{e}"))
		};

		iced::Command::perform(save, |result| match result {
			Ok(()) => crate::message::Message::Flash(flash_ok!(
				"Settings successfully saved."
			)),
			Err(e) => {
				// Log an error.
				//
				// TODO: Use a logger.
				eprintln!("Failed to save the settings: {e:?}");
				// Add an error message to display to user.
				crate::message::Message::Flash(flash_err!(
					"Failed to save the settings: {e}."
				))
			}
		})
	}
}

impl iced::Application for App {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = crate::message::Message;
	type Theme = iced::Theme;

	/// Creates new application.
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		// Ensure that file with a valid settings exists.
		let (settings, settings_path) = crate::settings::ensure_file()
			.expect("Failed to ensure settings file");

		// Initialize the application.
		let app = Self {
			page: crate::page::Page::default(),
			flashes: vec![],
			settings,
			settings_path,
		};
		(app, iced::Command::none())
	}

	/// Returns current theme.
	#[inline]
	#[must_use]
	fn theme(&self) -> Self::Theme {
		Clone::clone(&self.settings.theme)
	}

	/// Returns title of the application.
	#[inline]
	#[must_use]
	fn title(&self) -> String {
		ToOwned::to_owned("Sapphire")
	}

	/// Updates application's state using passed event [message].
	///
	/// [message]: Self::Message
	fn update(
		&mut self,
		message: Self::Message,
	) -> iced::Command<Self::Message> {
		// Commands to execute after update.
		let mut commands = vec![];

		match message {
			Self::Message::DefaultSettings => {
				self.settings.restore_defaults();
			}
			Self::Message::Exit => {
				commands.push(iced::window::close(iced::window::Id::MAIN));
			}
			Self::Message::Flash(flash) => self.flashes.push(flash),
			Self::Message::Page(page) => self.page = page,
			Self::Message::RemoveFlash if !self.flashes.is_empty() => {
				self.flashes.remove(0);
			}
			Self::Message::SaveSettings => {
				commands.push(self.get_settings_save_command());
			}
			Self::Message::Scale(scale) => self.settings.scale = scale,
			Self::Message::Theme(theme) => self.settings.theme = theme,
			_ => {}
		}

		// Execute all collected commands.
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
