mod single_user;
pub use single_user::*;
mod slice;
pub use slice::*;

rse_cpp::bitflags! {
	struct FilterFlags: u8 {
		const RELIABLE = 1 << 0;
		const INIT_MESSAGE = 1 << 1;
	}
}

impl FilterFlags {
	pub const fn new(reliable: bool, init_message: bool) -> Self {
		let mut this = Self::empty();
		if reliable {
			this = this.union(Self::RELIABLE)
		}
		if init_message {
			this = this.union(Self::INIT_MESSAGE)
		}
		this
	}
}