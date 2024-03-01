macro_rules! const_assert {
 	($($tt:tt)*) => {
 		const _: () = assert!($($tt)*);
	};
}
