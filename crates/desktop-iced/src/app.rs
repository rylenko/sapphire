#[derive(Clone, Debug, PartialEq)]
pub(crate) struct App {
	page: crate::page::Page,
	settings: crate::settings::Settings,
}

impl App {
	#[must_use]
	fn create_header(
		&self,
	) -> iced::widget::Column<'static, crate::message::Message> {
		let mut start_button = iced::widget::button(
			iced::widget::text("Start").size(self.settings.scale(11.0)),
		);
		let mut settings_button = iced::widget::button(
			iced::widget::text("Settings").size(self.settings.scale(11.0)),
		);

		match self.page {
			crate::page::Page::Start => {
				settings_button = settings_button
					.on_press(crate::message::Message::SettingsPage);
			}
			crate::page::Page::Settings => {
				start_button =
					start_button.on_press(crate::message::Message::StartPage);
			}
		}

		let row = iced::widget::row![
			iced::widget::text("Sapphire 🔐")
				.size(self.settings.scale(17.0))
				// To enable emoji support.
				.shaping(iced::widget::text::Shaping::Advanced),
			start_button,
			settings_button,
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
	}

	#[must_use]
	fn create_settings_page_theme_list(
		&self,
	) -> iced::widget::Row<'static, crate::message::Message> {
		let list = iced::widget::pick_list(
			iced::Theme::ALL,
			Some(Clone::clone(&self.settings.theme)),
			|theme| match theme {
				iced::Theme::CatppuccinFrappe => {
					crate::message::Message::CatppuccinFrappeTheme
				}
				iced::Theme::CatppuccinLatte => {
					crate::message::Message::CatppuccinLatteTheme
				}
				iced::Theme::CatppuccinMacchiato => {
					crate::message::Message::CatppuccinMacchiatoTheme
				}
				iced::Theme::CatppuccinMocha => {
					crate::message::Message::CatppuccinMochaTheme
				}
				iced::Theme::Dark => crate::message::Message::DarkTheme,
				iced::Theme::Dracula => crate::message::Message::DraculaTheme,
				iced::Theme::GruvboxDark => {
					crate::message::Message::GruvboxDarkTheme
				}
				iced::Theme::GruvboxLight => {
					crate::message::Message::GruvboxLightTheme
				}
				iced::Theme::KanagawaDragon => {
					crate::message::Message::KanagawaDragonTheme
				}
				iced::Theme::KanagawaLotus => {
					crate::message::Message::KanagawaLotusTheme
				}
				iced::Theme::KanagawaWave => {
					crate::message::Message::KanagawaWaveTheme
				}
				iced::Theme::Light => crate::message::Message::LightTheme,
				iced::Theme::Moonfly => crate::message::Message::MoonflyTheme,
				iced::Theme::Nightfly => {
					crate::message::Message::NightflyTheme
				}
				iced::Theme::Nord => crate::message::Message::NordTheme,
				iced::Theme::SolarizedDark => {
					crate::message::Message::SolarizedDarkTheme
				}
				iced::Theme::SolarizedLight => {
					crate::message::Message::SolarizedLightTheme
				}
				iced::Theme::TokyoNight => {
					crate::message::Message::TokyoNightTheme
				}
				iced::Theme::TokyoNightLight => {
					crate::message::Message::TokyoNightLightTheme
				}
				iced::Theme::TokyoNightStorm => {
					crate::message::Message::TokyoNightStormTheme
				}
				iced::Theme::Oxocarbon => {
					crate::message::Message::OxocarbonTheme
				}
				iced::Theme::Custom(..) => unreachable!(),
			},
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
		]
		.padding(self.settings.scale(10.0))
		.spacing(self.settings.scale(8.0));
		Into::into(content)
	}

	#[must_use]
	fn create_start_page_welcome(&self) -> iced::widget::Text<'static> {
		iced::widget::text(
			"A modern decentralized and private messenger with end-to-end encryption."
		)
		.size(self.settings.scale(11.0))
	}
}

impl iced::Application for App {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = crate::message::Message;
	type Theme = iced::Theme;

	#[inline]
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Self {
				page: crate::page::Page::default(),
				settings: crate::settings::Settings::default(),
			},
			iced::Command::none(),
		)
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
		match message {
			Self::Message::CatppuccinFrappeTheme => {
				self.settings.theme = iced::Theme::CatppuccinFrappe;
			}
			Self::Message::CatppuccinLatteTheme => {
				self.settings.theme = iced::Theme::CatppuccinLatte;
			}
			Self::Message::CatppuccinMacchiatoTheme => {
				self.settings.theme = iced::Theme::CatppuccinMacchiato;
			}
			Self::Message::CatppuccinMochaTheme => {
				self.settings.theme = iced::Theme::CatppuccinMocha;
			}
			Self::Message::DarkTheme => {
				self.settings.theme = iced::Theme::Dark;
			}
			Self::Message::DefaultSettings => {
				self.settings.restore_defaults();
			}
			Self::Message::DraculaTheme => {
				self.settings.theme = iced::Theme::Dracula;
			}
			Self::Message::GruvboxDarkTheme => {
				self.settings.theme = iced::Theme::GruvboxDark;
			}
			Self::Message::GruvboxLightTheme => {
				self.settings.theme = iced::Theme::GruvboxLight;
			}
			Self::Message::KanagawaDragonTheme => {
				self.settings.theme = iced::Theme::KanagawaDragon;
			}
			Self::Message::KanagawaLotusTheme => {
				self.settings.theme = iced::Theme::KanagawaLotus;
			}
			Self::Message::KanagawaWaveTheme => {
				self.settings.theme = iced::Theme::KanagawaWave;
			}
			Self::Message::LightTheme => {
				self.settings.theme = iced::Theme::Light;
			}
			Self::Message::MoonflyTheme => {
				self.settings.theme = iced::Theme::Moonfly;
			}
			Self::Message::NightflyTheme => {
				self.settings.theme = iced::Theme::Nightfly;
			}
			Self::Message::NordTheme => {
				self.settings.theme = iced::Theme::Nord;
			}
			Self::Message::Scale(scale) => self.settings.scale = scale,
			Self::Message::SolarizedDarkTheme => {
				self.settings.theme = iced::Theme::SolarizedDark;
			}
			Self::Message::SolarizedLightTheme => {
				self.settings.theme = iced::Theme::SolarizedLight;
			}
			Self::Message::TokyoNightTheme => {
				self.settings.theme = iced::Theme::TokyoNight;
			}
			Self::Message::TokyoNightLightTheme => {
				self.settings.theme = iced::Theme::TokyoNightLight;
			}
			Self::Message::TokyoNightStormTheme => {
				self.settings.theme = iced::Theme::TokyoNightStorm;
			}
			Self::Message::OxocarbonTheme => {
				self.settings.theme = iced::Theme::Oxocarbon;
			}
			Self::Message::SettingsPage => {
				self.page = crate::page::Page::Settings;
			}
			Self::Message::StartPage => self.page = crate::page::Page::Start,
			Self::Message::Exit => {
				return iced::window::close(iced::window::Id::MAIN)
			}
		}

		iced::Command::none()
	}

	#[must_use]
	fn view(&self) -> iced::Element<Self::Message> {
		match self.page {
			crate::page::Page::Settings => self.create_settings_page(),
			crate::page::Page::Start => self.create_start_page(),
		}
	}
}