use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::rse_convar::variable::ChangeVariable;

use super::{
	GenericConVar, CStrLock,
	GetValue,
	ConVarParams, ConVarValue,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct ConVar {
	inner: GenericConVar<DynConVar>,
}

impl ConVar {
	/// # Safety
	/// The [`ConVar`] must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(params: ConVarParams<'static>) -> Self {
		Self {
			inner: unsafe { GenericConVar::new(DynConVar, params) },
		}
	}

	/// # Safety
	/// The [`ConVar`] must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn simple(name: &'static CStr, default: ConVarValue<'static>) -> Self {
		unsafe { Self::new(ConVarParams::simple(name, default)) }
	}

	pub fn value<'a, V: GetValue<'a>>(&'a self) -> V {
		self.inner.value()
	}

	pub fn float(&self) -> c_float {
		self.inner.float()
	}

	pub fn int(&self) -> c_int {
		self.inner.int()
	}

	pub fn c_str(&self) -> CStrLock<'_> {
		self.inner.c_str()
	}

	pub fn register(&self) {
		self.inner.register()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct DynConVar;
impl ChangeVariable for DynConVar {}
