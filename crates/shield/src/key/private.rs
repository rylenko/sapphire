/// User's private key.
#[derive(Clone, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Private([u8; 32]);

// Prohibit the implementation of traits that can reveal private key details.
impl !PartialEq for Private {}
impl !core::fmt::Debug for Private {}
impl !core::fmt::Display for Private {}
impl !core::hash::Hash for Private {}
impl !zerocopy::AsBytes for Private {}
