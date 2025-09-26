use crate::cppdef::entities::edict_t;

pub use crate::cppdef::entities::EdictIndex;

/// # Layout
/// This type has the exact same layout as the C++ [`edict_t`].
#[derive(Debug)]
#[repr(transparent)]
pub struct ServerEdict(edict_t);
impl ServerEdict {
	/// # Safety
	/// `c_edict` must exist in-game.
	pub const unsafe fn from_c_edict(c_edict: &edict_t) -> &Self {
		unsafe { &*(c_edict as *const _ as *const Self) }
	}

	/// # Safety
	/// `c_edict` must exist in-game.
	pub const unsafe fn from_c_edict_mut(c_edict: &mut edict_t) -> &mut Self {
		unsafe { &mut *(c_edict as *mut _ as *mut Self) }
	}

	pub const fn as_ptr(&self) -> *const edict_t {
		self as *const _ as *const _
	}

	pub const fn as_mut_ptr(&mut self) -> *mut edict_t {
		self as *mut _ as *mut _
	}

	::rse_cpp::transparent_wrapper_inner_impls!(ServerEdict for edict_t as "edict_t");

	pub const fn index(&self) -> EdictIndex {
		self.0.base_edict.edict_index
	}
}
