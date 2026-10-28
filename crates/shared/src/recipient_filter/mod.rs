use ::core::ffi::c_int;
use ::rse_cpp::{
	transparent_wrapper,
	virtual_call,
};

use crate::cppdef::IRecipientFilter;

pub mod object;

mod iter;
pub use iter::*;
mod impls;
pub use impls::*;

pub type Recipient = c_int;
pub type RecipientSlot = c_int;

pub const RECIPIENT_INVALID: Recipient = -1;

impl RecipientFilter {
	/// Returns `true` if this filter is reliable.
	pub fn is_reliable(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_reliable()) }
	}

	/// Returns `true` if this filter is intended for a sign-on message.
	pub fn is_init_message(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_init_message()) }
	}

	/// Returns the number of recipients in this filter.
	pub fn recipient_count(&self) -> RecipientSlot {
		unsafe { virtual_call!(self.as_inner() => get_recipient_count()) }
	}

	/// Returns the [`Recipient`] in the given `slot`,
	/// or [`RECIPIENT_INVALID`] if `slot` is invalid.
	pub fn recipient_at(&self, slot: RecipientSlot) -> Recipient {
		unsafe { virtual_call!(self.as_inner() => get_recipient_index(slot)) }
	}

	/// Returns an iterator over all [`Recipient`]s in this filter.
	pub fn iter(&self) -> Recipients<'_> {
		Recipients {
			filter: self,
			slots: 0..self.recipient_count(),
		}
	}
}

transparent_wrapper! {
	pub struct RecipientFilter for IRecipientFilter as "IRecipientFilter";
}
