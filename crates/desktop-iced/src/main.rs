/*!
Sapphire desktop application built on [`iced`].

TODO: documentation and commnents.
*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
enum Message {
	Exit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Application;

impl Application {
	#[must_use]
	fn create_header(
	) -> iced::widget::Row<'static, <Self as iced::Application>::Message> {
		iced::widget::row![
			Self::create_header_exit_button(),
			Self::create_header_title(),
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
	#[must_use]
	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(Self {}, iced::Command::none())
	}

	#[inline]
	#[must_use]
	fn theme(&self) -> Self::Theme {
		Self::Theme::Nord
	}

	#[inline]
	#[must_use]
	fn title(&self) -> String {
		ToOwned::to_owned("Sapphire")
	}

	#[must_use]
	fn update(
		&mut self,
		message: Self::Message,
	) -> iced::Command<Self::Message> {
		match message {
			Message::Exit => iced::window::close(iced::window::Id::MAIN),
		}
	}

	#[must_use]
	fn view(&self) -> iced::Element<Self::Message> {
		Into::into(Self::create_header())
	}
}

fn main() -> iced::Result {
	<Application as iced::Application>::run(iced::Settings::default())
}
