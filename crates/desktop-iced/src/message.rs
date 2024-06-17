/// Application update messages.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub(crate) enum Message {
	DefaultSettings,
	Exit,
	//Error(Error),
	None,
	Page(crate::page::Page),
	SaveSettings,
	Scale(f32),
	Settings(crate::settings::Settings),
	Theme(iced::Theme),
}
