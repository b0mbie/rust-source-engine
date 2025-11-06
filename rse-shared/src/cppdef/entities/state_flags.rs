use ::core::ffi::c_int;

use ::rse_cpp::*;

flag_consts! {
	for c_int:
	/// Game DLL sets this when the entity state changes.
	/// Mutually exclusive with `PARTIAL_CHANGE`.
	CHANGED = 1 << 0;
	/// This edict is free for re-use.
	FREE = 1 << 1;
	/// This is a full server entity.
	FULL = 1 << 2;
	/// Always transmit this entity.
	ALWAYS = 1 << 3;
	/// Don't transmit this entity.
	DONT_SEND = 1 << 4;
	/// Always transmit this entity, but cull against PVS.
	ALWAYS_PVS_CHECK = 1 << 5;
	// /// Used by local network backdoor.
	// PENDING_DORMANT_CHECK = 1 << 6;
	// /// This is always set at the same time the entity flag `DIRTY_PVS_INFORMATION` is set,
	// /// but it gets cleared in a different place.
	// DIRTY_PVS_INFORMATION = 1 << 7;
	// /// This is used internally by `edict_t` to remember that it's carrying a "full change list" -
	// /// all its properties might have changed their value.
	// FULL_EDICT_CHANGED = 1 << 8;
}

const TRANSMIT_KIND_MASK: c_int = ALWAYS | DONT_SEND | ALWAYS_PVS_CHECK;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TransmitKind {
	#[default]
	UseShouldTransmit = 0,
	Always = ALWAYS as _,
	AlwaysWithPvs = ALWAYS_PVS_CHECK as _,
	Never = DONT_SEND as _,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StateFlags(pub c_int);
impl StateFlags {
	/// Return a set of bitfields that:
	/// - indicates that the corresponding edict is not free,
	/// - indicates that the coreresponding entity is not a full server entity, and
	/// - stores [`TransmitKind::UseShouldTransmit`].
	pub const fn new() -> Self {
		Self(0)
	}

	/// Return `true` if
	/// the entity state has changed.
	pub const fn has_changed(&self) -> bool {
		test_bits!(self, CHANGED)
	}

	/// Return this set of bitfields with a flag set indicating that
	/// the entity state has changed.
	pub const fn changed(self) -> Self {
		with_bits!(self, CHANGED)
	}

	/// Return `true` if
	/// the corresponding edict is free for re-use.
	pub const fn is_free(&self) -> bool {
		test_bits!(self, FREE)
	}

	/// Return this set of bitfields with a flag set indicating that
	/// the corresponding edict is free for re-use.
	pub const fn free(self) -> Self {
		with_bits!(self, FREE)
	}

	/// Return `true` if
	/// the corresponding entity is a full server entity.
	pub const fn is_full_entity(&self) -> bool {
		test_bits!(self, FULL)
	}

	/// Return this set of bitfields with a flag set indicating that
	/// the corresponding entity is a full server entity.
	pub const fn full_entity(self) -> Self {
		with_bits!(self, FULL)
	}

	/// Return the [`TransmitKind`] stored in the bitfields.
	pub const fn transmit_kind(&self) -> Option<TransmitKind> {
		match self.0 & TRANSMIT_KIND_MASK {
			0 => Some(TransmitKind::UseShouldTransmit),
			ALWAYS => Some(TransmitKind::Always),
			DONT_SEND => Some(TransmitKind::Never),
			ALWAYS_PVS_CHECK => Some(TransmitKind::AlwaysWithPvs),
			_ => None,
		}
	}

	/// Return this set of bitfields with the given [`TransmitKind`],
	pub const fn with_transmit_kind(self, kind: TransmitKind) -> Self {
		Self((self.0 & !TRANSMIT_KIND_MASK) | (kind as c_int))
	}
}
