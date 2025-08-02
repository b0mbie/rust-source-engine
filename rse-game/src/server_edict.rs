use crate::cppdef::entities::Edict;

pub use crate::cppdef::entities::EdictIndex;

/// # Layout
/// This type has the exact same layout as the C++ [`Edict`].
#[derive(Debug)]
#[repr(transparent)]
pub struct ServerEdict(Edict);

impl ServerEdict {
	/// # Safety
	/// `c_edict` must exist in-game.
	pub const unsafe fn from_c_edict(c_edict: &Edict) -> &Self {
		unsafe { &*(c_edict as *const _ as *const Self) }
	}

	/// # Safety
	/// `c_edict` must exist in-game.
	pub const unsafe fn from_c_edict_mut(c_edict: &mut Edict) -> &mut Self {
		unsafe { &mut *(c_edict as *mut _ as *mut Self) }
	}

	pub const fn index(&self) -> EdictIndex {
		self.0.base_edict.edict_index
	}
}
