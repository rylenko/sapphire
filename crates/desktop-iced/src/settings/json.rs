#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum LoadError {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct Loader<'a> {
	path: &'a str,
}

impl<'a> Loader<'a> {
	#[inline]
	#[must_use]
	pub(crate) fn new(path: &'a str) -> Self {
		Self { path }
	}
}

impl<'a> super::Loader<'a> for Loader<'a> {
	type Error = LoadError;

	async fn load(&mut self) -> Result<super::Settings, Self::Error> {
		Ok(Default::default())
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum SaveError {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) struct Saver<'a> {
	path: &'a str,
}

impl<'a> Saver<'a> {
	#[inline]
	#[must_use]
	pub(crate) fn new(path: &'a str) -> Self {
		Self { path }
	}
}

impl<'a> super::Saver<'a> for Saver<'a> {
	type Error = SaveError;

	async fn save(
		&mut self,
		_settings: &super::Settings,
	) -> Result<(), Self::Error> {
		Ok(())
	}
}
