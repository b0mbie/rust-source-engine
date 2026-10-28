use ::core::ops::Range;

use super::{
	RecipientFilter, RecipientSlot, Recipient,
	RECIPIENT_INVALID,
};

pub struct Recipients<'a> {
	pub(super) filter: &'a RecipientFilter,
	pub(super) slots: Range<RecipientSlot>,
}

impl Iterator for Recipients<'_> {
	type Item = Recipient;
	#[track_caller]
	fn next(&mut self) -> Option<Self::Item> {
		let slot = self.slots.next()?;
		let recipient = self.filter.recipient_at(slot);
		debug_assert_ne!(
			recipient, RECIPIENT_INVALID,
			"`IRecipientFilter` reported incorrect recipient count ({}) - got invalid recipient at [{slot}]",
			self.slots.end,
		);
		Some(recipient)
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.slots.size_hint()
	}
}
