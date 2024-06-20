/*!
Sapphire desktop application built on [`iced`].

TODO: documentation and comments.
TODO: add page::Page enum, page::Start, page::Settings and move logic to them.
*/

mod app;
mod message;
mod page;
mod settings;

fn main() -> Result<(), Box<iced::Error>> {
	<app::App as iced::Application>::run(iced::Settings::default())
		.map_err(Box::new)
}
