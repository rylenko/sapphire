/// Draft for the [`State`] and [`Recv`], so as not to corrupt them due to
/// errors.
///
/// [`State`]: super::state::State
/// [`Recv`]: super::recv::Recv
pub(super) trait Draft {
	fn commit_draft(&mut self, draft: Self);

	#[must_use]
	fn create_draft(&self) -> Self;
}
