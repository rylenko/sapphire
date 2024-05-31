/*!
Sapphire desktop application built on [`iced`].
*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
enum Message {
	Exit,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Application;

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
		let column = iced::widget::column![
			iced::widget::text("Sapphire 💎")
				.size(15)
				.shaping(iced::widget::text::Shaping::Advanced)
				.horizontal_alignment(iced::alignment::Horizontal::Center),
			iced::widget::button("Exit").on_press(Self::Message::Exit)
		]
		.padding(10);
		Into::into(column)
	}
}

fn main() -> iced::Result {
	<Application as iced::Application>::run(iced::Settings::default())
}
