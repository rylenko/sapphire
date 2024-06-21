/// Macros to easily create formatted error [flash].
///
/// [flash]: Flash
macro_rules! flash_err {
	($($arg:tt)*) => {
		crate::flash::Flash::Err(format!($($arg)*))
	};
}

/// Macros to easily create formatted ok [flash].
///
/// [flash]: Flash
macro_rules! flash_ok {
	($($arg:tt)*) => {
		crate::flash::Flash::Ok(format!($($arg)*))
	};
}

/// Flash message to show to the user.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum Flash {
	Err(String),
	Ok(String),
}

impl Flash {
	const ERR_COLOR: iced::Color =
		iced::Color::from_rgb(164.0 / 255.0, 0.0, 0.0);
	const OK_COLOR: iced::Color =
		iced::Color::from_rgb(0.0, 1.0, 127.0 / 255.0);

	/// Returns [color] of the flash.
	///
	/// [color]: iced::Color
	#[must_use]
	pub(crate) const fn color(&self) -> iced::Color {
		match self {
			Self::Err(..) => Self::ERR_COLOR,
			Self::Ok(..) => Self::OK_COLOR,
		}
	}

	/// Returns string of the flash.
	#[must_use]
	pub(crate) fn as_str(&self) -> &str {
		match self {
			Self::Err(ref s) | Self::Ok(ref s) => s,
		}
	}
}

impl AsRef<str> for Flash {
	#[inline]
	#[must_use]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_flash_err() {
		let flash = flash_err!("Hello, {what}! {}.", 123, what = "world");
		assert_eq!(flash.as_str(), "Hello, world! 123.");
		assert_eq!(flash.color(), super::Flash::Err(String::new()).color());
	}

	#[test]
	fn test_flash_ok() {
		let flash = flash_ok!("Hello, {what}! {}.", 123, what = "world");
		assert_eq!(flash.as_str(), "Hello, world! 123.");
		assert_eq!(flash.color(), super::Flash::Ok(String::new()).color());
	}
}
