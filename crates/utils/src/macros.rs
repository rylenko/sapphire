/// Asserts expression during compile time.
#[macro_export]
macro_rules! const_assert {
	($name:ident, $($tt:tt)*) => {
		const $name: () = assert!($($tt)*);
	};
}
