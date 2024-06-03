/*!
Sapphire desktop application built on [`iced`].

TODO: documentation and commnents.
*/

#[derive(Clone, Debug, PartialEq)]
struct Application {
	page: Page,
	settings: Settings,
}

impl Application {
	#[must_use]
	fn create_header(
		&self,
	) -> iced::widget::Column<'static, <Self as iced::Application>::Message> {
		let mut start_button = iced::widget::button(
			iced::widget::text("Start").size(self.settings.scale(11.0)),
		);
		let mut settings_button = iced::widget::button(
			iced::widget::text("Settings").size(self.settings.scale(11.0)),
		);

		match self.page {
			Page::Start => {
				settings_button = settings_button.on_press(
					<Self as iced::Application>::Message::SettingsPage,
				);
			}
			Page::Settings => {
				start_button = start_button
					.on_press(<Self as iced::Application>::Message::StartPage);
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
			.on_press(<Self as iced::Application>::Message::Exit),
		]
		.spacing(self.settings.scale(8.0));
		iced::widget::column![
			row,
			iced::widget::horizontal_rule(self.settings.scale(10.0)),
		]
		.spacing(self.settings.scale(8.0))
	}

	#[must_use]
	fn create_settings_page(
		&self,
	) -> iced::Element<<Self as iced::Application>::Message> {
		let content = iced::widget::column![
			self.create_header(),
			self.create_settings_page_theme_list(),
			self.create_settings_page_iface_slider(),
			self.create_settings_page_restore_defaults_button(),
		]
		.padding(self.settings.scale(10.0))
		.spacing(self.settings.scale(8.0));

		Into::into(content)
	}

	#[must_use]
	fn create_settings_page_iface_slider(
		&self,
	) -> iced::widget::Row<'static, <Self as iced::Application>::Message> {
		iced::widget::row![
			iced::widget::text("Interace scale:")
				.size(self.settings.scale(11.0)),
			iced::widget::slider(
				0.1..=3.0,
				self.settings.iface_scale,
				|scale| { Message::IfaceScale(scale) }
			)
			.step(0.1),
		]
		.spacing(self.settings.scale(8.0))
	}

	#[must_use]
	fn create_settings_page_restore_defaults_button(
		&self,
	) -> iced::widget::Button<'static, <Self as iced::Application>::Message> {
		iced::widget::button(
			iced::widget::text("Restore defaults")
				.size(self.settings.scale(11.0)),
		)
		.on_press(<Self as iced::Application>::Message::DefaultSettings)
	}

	#[must_use]
	fn create_settings_page_theme_list(
		&self,
	) -> iced::widget::PickList<
		iced::Theme,
		&[iced::Theme],
		iced::Theme,
		<Self as iced::Application>::Message,
	> {
		iced::widget::pick_list(
			iced::Theme::ALL,
			Some(Clone::clone(&self.settings.theme)),
			|theme| match theme {
				iced::Theme::CatppuccinFrappe => {
					Message::CatppuccinFrappeTheme
				}
				iced::Theme::CatppuccinLatte => Message::CatppuccinLatteTheme,
				iced::Theme::CatppuccinMacchiato => {
					Message::CatppuccinMacchiatoTheme
				}
				iced::Theme::CatppuccinMocha => Message::CatppuccinMochaTheme,
				iced::Theme::Dark => Message::DarkTheme,
				iced::Theme::Dracula => Message::DraculaTheme,
				iced::Theme::GruvboxDark => Message::GruvboxDarkTheme,
				iced::Theme::GruvboxLight => Message::GruvboxLightTheme,
				iced::Theme::KanagawaDragon => Message::KanagawaDragonTheme,
				iced::Theme::KanagawaLotus => Message::KanagawaLotusTheme,
				iced::Theme::KanagawaWave => Message::KanagawaWaveTheme,
				iced::Theme::Light => Message::LightTheme,
				iced::Theme::Moonfly => Message::MoonflyTheme,
				iced::Theme::Nightfly => Message::NightflyTheme,
				iced::Theme::Nord => Message::NordTheme,
				iced::Theme::SolarizedDark => Message::SolarizedDarkTheme,
				iced::Theme::SolarizedLight => Message::SolarizedLightTheme,
				iced::Theme::TokyoNightLight => Message::TokyoNightLightTheme,
				iced::Theme::TokyoNightStorm => Message::TokyoNightStormTheme,
				iced::Theme::TokyoNight => Message::TokyoNightTheme,
				iced::Theme::Oxocarbon => Message::OxocarbonTheme,
				iced::Theme::Custom(..) => unreachable!(),
			},
		)
		.placeholder("Pick a theme...")
		.text_size(self.settings.scale(11.0))
	}

	#[must_use]
	fn create_start_page(
		&self,
	) -> iced::Element<<Self as iced::Application>::Message> {
		let content = iced::widget::column![
			self.create_header(),
			iced::widget::text("A modern decentralized and private messenger with end-to-end encryption.").size(self.settings.scale(11.0)),
		].padding(self.settings.scale(10.0)).spacing(self.settings.scale(8.0));
		Into::into(content)
	}
}

impl iced::Application for Application {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = iced::Theme;

	#[inline]
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Self { page: Page::Start, settings: Settings::default() },
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
			Message::CatppuccinFrappeTheme => {
				self.settings.theme = iced::Theme::CatppuccinFrappe;
			}
			Message::CatppuccinLatteTheme => {
				self.settings.theme = iced::Theme::CatppuccinLatte;
			}
			Message::CatppuccinMacchiatoTheme => {
				self.settings.theme = iced::Theme::CatppuccinMacchiato;
			}
			Message::CatppuccinMochaTheme => {
				self.settings.theme = iced::Theme::CatppuccinMocha;
			}
			Message::DarkTheme => {
				self.settings.theme = iced::Theme::Dark;
			}
			Message::DefaultSettings => {
				self.settings.restore_defaults();
			}
			Message::DraculaTheme => {
				self.settings.theme = iced::Theme::Dracula;
			}
			Message::GruvboxDarkTheme => {
				self.settings.theme = iced::Theme::GruvboxDark;
			}
			Message::GruvboxLightTheme => {
				self.settings.theme = iced::Theme::GruvboxLight;
			}
			Message::IfaceScale(scale) => self.settings.iface_scale = scale,
			Message::KanagawaDragonTheme => {
				self.settings.theme = iced::Theme::KanagawaDragon;
			}
			Message::KanagawaLotusTheme => {
				self.settings.theme = iced::Theme::KanagawaLotus;
			}
			Message::KanagawaWaveTheme => {
				self.settings.theme = iced::Theme::KanagawaWave;
			}
			Message::LightTheme => {
				self.settings.theme = iced::Theme::Light;
			}
			Message::MoonflyTheme => {
				self.settings.theme = iced::Theme::Moonfly;
			}
			Message::NightflyTheme => {
				self.settings.theme = iced::Theme::Nightfly;
			}
			Message::NordTheme => {
				self.settings.theme = iced::Theme::Nord;
			}
			Message::SolarizedDarkTheme => {
				self.settings.theme = iced::Theme::SolarizedDark;
			}
			Message::SolarizedLightTheme => {
				self.settings.theme = iced::Theme::SolarizedLight;
			}
			Message::TokyoNightLightTheme => {
				self.settings.theme = iced::Theme::TokyoNightLight;
			}
			Message::TokyoNightStormTheme => {
				self.settings.theme = iced::Theme::TokyoNightStorm;
			}
			Message::TokyoNightTheme => {
				self.settings.theme = iced::Theme::TokyoNight;
			}
			Message::OxocarbonTheme => {
				self.settings.theme = iced::Theme::Oxocarbon;
			}
			Message::SettingsPage => self.page = Page::Settings,
			Message::StartPage => self.page = Page::Start,
			Message::Exit => {
				return iced::window::close(iced::window::Id::MAIN)
			}
		}

		iced::Command::none()
	}

	#[must_use]
	fn view(&self) -> iced::Element<Self::Message> {
		match self.page {
			Page::Settings => self.create_settings_page(),
			Page::Start => self.create_start_page(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
enum Message {
	CatppuccinFrappeTheme,
	CatppuccinLatteTheme,
	CatppuccinMacchiatoTheme,
	CatppuccinMochaTheme,
	DarkTheme,
	DefaultSettings,
	DraculaTheme,
	GruvboxDarkTheme,
	GruvboxLightTheme,
	IfaceScale(f32),
	KanagawaDragonTheme,
	KanagawaLotusTheme,
	KanagawaWaveTheme,
	LightTheme,
	MoonflyTheme,
	NightflyTheme,
	NordTheme,
	SolarizedDarkTheme,
	SolarizedLightTheme,
	TokyoNightLightTheme,
	TokyoNightStormTheme,
	TokyoNightTheme,
	OxocarbonTheme,
	SettingsPage,
	StartPage,
	Exit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
enum Page {
	Settings,
	Start,
}

/// TODO: Serialize them to the file and load this file at startup.
#[derive(Clone, Debug, PartialEq)]
struct Settings {
	theme: iced::Theme,
	iface_scale: f32,
}

impl Settings {
	/// Restores default settings.
	#[inline]
	fn restore_defaults(&mut self) {
		*self = Self::default();
	}

	/// Scales passed size to interface size using the coefficient set on the
	/// settings page.
	#[must_use]
	#[inline]
	fn scale(&self, size: f32) -> f32 {
		size * self.iface_scale
	}
}

impl Default for Settings {
	#[inline]
	#[must_use]
	fn default() -> Self {
		Self { theme: iced::Theme::Dark, iface_scale: 1.0 }
	}
}

fn main() -> Result<(), Box<iced::Error>> {
	<Application as iced::Application>::run(iced::Settings::default())
		// Wrap into box because error type is too large.
		.map_err(Box::new)
}
