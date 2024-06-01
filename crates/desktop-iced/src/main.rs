/*!
Sapphire desktop application built on [`iced`].

TODO: documentation and commnents.
*/

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Application {
	theme: Theme,
}

impl Application {
	#[must_use]
	fn create_header(
		&self,
	) -> iced::widget::Row<'static, <Self as iced::Application>::Message> {
		iced::widget::row![
			Self::create_header_title(),
			self.create_header_theme_button(),
			Self::create_header_exit_button(),
		]
		.padding(10)
		.spacing(8)
	}

	#[must_use]
	fn create_header_exit_button(
	) -> iced::widget::Button<'static, <Self as iced::Application>::Message> {
		iced::widget::button(iced::widget::text("Exit").size(10))
			.on_press(<Self as iced::Application>::Message::Exit)
	}

	#[must_use]
	fn create_header_theme_button(
		&self,
	) -> iced::widget::Button<'static, <Self as iced::Application>::Message> {
		match self.theme {
			Theme::Light => {
				iced::widget::button(iced::widget::text("Nord theme").size(10))
					.on_press(<Self as iced::Application>::Message::NordTheme)
			}
			Theme::Nord => iced::widget::button(
				iced::widget::text("Light theme").size(10),
			)
			.on_press(<Self as iced::Application>::Message::LightTheme),
		}
	}

	#[must_use]
	fn create_header_title() -> iced::widget::Text<'static> {
		iced::widget::text("Sapphire 💎")
			.size(15)
			// To enable emoji support.
			.shaping(iced::widget::text::Shaping::Advanced)
	}
}

impl iced::Application for Application {
	type Executor = iced::executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = iced::Theme;

	#[inline]
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(Self { theme: Theme::Nord }, iced::Command::none())
	}

	#[inline]
	#[must_use]
	fn theme(&self) -> Self::Theme {
		match self.theme {
			Theme::Light => Self::Theme::Light,
			Theme::Nord => Self::Theme::Nord,
		}
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
			Message::Exit => iced::window::close(iced::window::Id::MAIN),
			Message::LightTheme => {
				self.theme = Theme::Light;
				iced::Command::none()
			}
			Message::NordTheme => {
				self.theme = Theme::Nord;
				iced::Command::none()
			}
		}
	}

	#[must_use]
	fn view(&self) -> iced::Element<Self::Message> {
		Into::into(self.create_header())
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
enum Message {
	Exit,
	LightTheme,
	NordTheme,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
enum Theme {
	Light,
	Nord,
}

fn main() -> Result<(), Box<iced::Error>> {
	<Application as iced::Application>::run(iced::Settings::default())
		// Wrap into box because error type is too large.
		.map_err(Box::new)
}
