use ::core::{
	ffi::c_int,
	ops::{
		Deref, DerefMut,
	},
};
use ::rse_cpp::{
	new_vtable_self, vtable_methods,
	This, this_to_self,
	VTablePtr,
};

use crate::cppdef::{
	IRecipientFilter, IRecipientFilterVt,
};

use super::{
	RecipientFilter, RecipientSlot, Recipient,
};

/// Rust types which can be used to create [`IRecipientFilter`]s with [`FilterObject`].
pub trait FilterImpl {
	fn is_reliable(&self) -> bool;
	fn is_init_message(&self) -> bool;
	fn recipient_count(&self) -> RecipientSlot;
	fn recipient_at(&self, slot: RecipientSlot) -> Recipient;
}

#[repr(C)]
pub struct FilterObject<T> {
	raw: IRecipientFilter,
	inner: T,
}

impl<T> Deref for FilterObject<T> {
	type Target = RecipientFilter;
	fn deref(&self) -> &Self::Target {
		unsafe { RecipientFilter::from_ref(&self.raw) }
	}
}
impl<T> DerefMut for FilterObject<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { RecipientFilter::from_mut(&mut self.raw) }
	}
}

impl<T: FilterImpl> FilterObject<T> {
	pub const fn new(inner: T) -> Self {
		Self {
			raw: IRecipientFilter {
				vtable: VTablePtr::from_ref(Self::VTABLE),
			},
			inner,
		}
	}

	const VTABLE: &IRecipientFilterVt = &new_vtable_self!(IRecipientFilterVt {
		destructor,
		#[cfg(not(windows))]
		destructor_2,
		is_reliable,
		is_init_message,
		get_recipient_count,
		get_recipient_index,
	});

	vtable_methods! {
		this: This<IRecipientFilter>;
		fn destructor() {
			let _ = this;
			// TODO: Destructor?
		}
		#[cfg(not(windows))]
		fn destructor_2() {
			let _ = this;
			// TODO: Destructor?
		}
		fn is_reliable() -> bool {
			unsafe { this_to_self!(ref this) }.inner.is_reliable()
		}
		fn is_init_message() -> bool {
			unsafe { this_to_self!(ref this) }.inner.is_init_message()
		}
		fn get_recipient_count() -> c_int {
			unsafe { this_to_self!(ref this) }.inner.recipient_count()
		}
		fn get_recipient_index(slot: c_int) -> c_int {
			unsafe { this_to_self!(ref this) }.inner.recipient_at(slot)
		}
	}
}
