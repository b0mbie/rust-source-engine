use ::core::ops::{
	Deref, DerefMut,
};
use ::rse_cpp::util::clamp_len_to_c_int;

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

struct Impl<'a> {
	pub recipients: &'a [Recipient],
	pub flags: FilterFlags,
}

impl FilterImpl for Impl<'_> {
	fn is_reliable(&self) -> bool {
		self.flags.contains(FilterFlags::RELIABLE)
	}
	fn is_init_message(&self) -> bool {
		self.flags.contains(FilterFlags::INIT_MESSAGE)
	}
	fn recipient_count(&self) -> RecipientSlot {
		clamp_len_to_c_int(self.recipients.len())
	}
	fn recipient_at(&self, slot: RecipientSlot) -> Recipient {
		if slot >= 0 {
			self.recipients
				.get(slot as usize).copied()
				.unwrap_or(RECIPIENT_INVALID)
		} else {
			RECIPIENT_INVALID
		}
	}
}

#[repr(transparent)]
pub struct SliceFilter<'a> {
	inner: FilterObject<Impl<'a>>,
}

impl Deref for SliceFilter<'_> {
	type Target = RecipientFilter;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl DerefMut for SliceFilter<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a> SliceFilter<'a> {
	pub const fn new(
		recipients: &'a [Recipient],
		reliable: bool, init_message: bool,
	) -> Self {
		Self {
			inner: FilterObject::new(Impl {
				recipients,
				flags: FilterFlags::new(reliable, init_message),
			}),
		}
	}

	pub const fn reliable(recipients: &'a [Recipient]) -> Self {
		Self::new(recipients, true, false)
	}

	pub const fn unreliable(recipients: &'a [Recipient]) -> Self {
		Self::new(recipients, false, false)
	}
}

#[test]
fn slice_filter() {
	let recipients = &[42, 1337, 37];
	let filter = SliceFilter::reliable(recipients);
	assert!(filter.is_reliable());
	assert!(!filter.is_init_message());
	assert_eq!(filter.recipient_count(), recipients.len() as _);
	assert_eq!(filter.recipient_at(-1), RECIPIENT_INVALID);
	assert!(filter.iter().eq(recipients.iter().copied()));
}
