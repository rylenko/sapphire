/// Application event messages.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub(crate) enum Message {
	DefaultSettings,
	Exit,
	Flash(String),
	Page(crate::page::Page),
	SaveSettings,
	Scale(f32),
	Theme(iced::Theme),
}
