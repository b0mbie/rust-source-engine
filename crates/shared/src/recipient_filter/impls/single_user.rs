use ::core::ops::{
	Deref, DerefMut,
};

use super::{
	super::{
		object::{
			FilterImpl, FilterObject,
		},
		RecipientFilter, RecipientSlot,
		Recipient, RECIPIENT_INVALID,
	},
	FilterFlags,
};

struct Impl {
	pub recipient: Recipient,
	pub flags: FilterFlags,
}

impl FilterImpl for Impl {
	fn is_reliable(&self) -> bool {
		self.flags.contains(FilterFlags::RELIABLE)
	}
	fn is_init_message(&self) -> bool {
		self.flags.contains(FilterFlags::INIT_MESSAGE)
	}
	fn recipient_count(&self) -> RecipientSlot {
		1
	}
	fn recipient_at(&self, slot: RecipientSlot) -> Recipient {
		if slot == 0 {self.recipient} else {RECIPIENT_INVALID}
	}
}

#[repr(transparent)]
pub struct SingleUserFilter {
	inner: FilterObject<Impl>,
}

impl Deref for SingleUserFilter {
	type Target = RecipientFilter;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl DerefMut for SingleUserFilter {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl SingleUserFilter {
	pub const fn new(recipient: Recipient, reliable: bool, init_message: bool) -> Self {
		Self {
			inner: FilterObject::new(Impl {
				recipient,
				flags: FilterFlags::new(reliable, init_message),
			}),
		}
	}

	pub const fn reliable(recipient: Recipient) -> Self {
		Self::new(recipient, true, false)
	}

	pub const fn unreliable(recipient: Recipient) -> Self {
		Self::new(recipient, false, false)
	}
}

#[test]
fn single_user_filter() {
	let recipient = 42;
	let filter = SingleUserFilter::reliable(recipient);
	assert!(filter.is_reliable());
	assert!(!filter.is_init_message());
	assert_eq!(filter.recipient_count(), 1);
	assert_eq!(filter.recipient_at(0), recipient);
	assert_eq!(filter.recipient_at(-1), RECIPIENT_INVALID);
	assert_eq!(filter.recipient_at(1), RECIPIENT_INVALID);
	assert!(filter.iter().eq(core::iter::once(recipient)));
}
