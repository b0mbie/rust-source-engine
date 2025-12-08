use ::atomic::*;
use ::rse_convar::{
	cvar::Cvar as ICvar,
	console_base::CvarDllIdentifier,
};
use ::rse_cpp::{
	AsObject, VtObject,
};

use crate::{
	plugin::PluginFactories,
	raw::exclusive::UnsafeExclusive,
	thread::on_main_thread,
};

pub use ::rse_convar::{
	cppdef::{
		CvarVt, CvarVtBase,
	},
	cvar::CvarImpl,
};

/// # Safety
/// This function must be called from the main thread.
/// 
/// A call to this function must eventually be followed by a call to [`detach`].
pub(crate) unsafe fn attach(factories: PluginFactories) {
	let result = factories
		.create_interface().map(Inner::V4)
		;
	match result {
		Ok(mut inner) => {
			match inner {
				Inner::None => unreachable!(),
				Inner::V4(ref mut iface) => unsafe { set_dll_identifier(iface.allocate_dll_identifier()) }
			}
			unsafe { CVAR.inspect(move |cvar| *cvar = inner) };
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
		}
	}
}

/// # Safety
/// This function must be called from the main thread.
pub(crate) unsafe fn detach() {
	let f = move |inner: &mut Inner| {
		let dll_id = dll_identifier();
		match inner {
			Inner::None => {}
			Inner::V4(cvar) => unsafe { cvar.unregister_all(dll_id) }
		}
		reset_dll_identifier();
		*inner = Inner::None;
	};
	unsafe { CVAR.inspect(f) }
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
pub unsafe fn inspect<R, F: FnOnce(Option<&mut Cvar>) -> R>(f: F) -> R {
	if on_main_thread() {
		unsafe { CVAR.inspect(move |inner| f(Cvar::from_mut(inner))) }
	} else {
		f(None)
	}
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
pub unsafe fn inspect_unchecked<R, F: FnOnce(Option<&mut Cvar>) -> R>(f: F) -> R {
	let f = move |inner: &mut Inner| f(Cvar::from_mut(inner));
	unsafe { CVAR.inspect(f) }
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
/// 
/// Additionally, the operations performed on the interface
/// *must* support multi-threading.
pub unsafe fn inspect_mt<R, F: FnOnce(Option<&Cvar>) -> R>(f: F) -> R {
	let f = move |inner: &mut Inner| f(Cvar::from_ref(inner));
	unsafe { CVAR.inspect(f) }
}

#[repr(transparent)]
pub struct Cvar {
	// INVARIANT: This value is never `Inner::None`.
	inner: Inner,
}

impl Cvar {
	pub fn as_v4(&self) -> &VtObject<CvarVt> {
		match self.inner {
			Inner::V4(ref sv) => sv.as_object(),
			_ => unsafe { ::core::hint::unreachable_unchecked() },
		}
	}

	const fn from_mut(inner: &mut Inner) -> Option<&mut Self> {
		match inner {
			Inner::None => None,
			// SAFETY: `Self` is a transparent wrapper for `Inner`.
			inner => unsafe { Some(&mut *(inner as *mut Inner as *mut Self)) }
		}
	}

	const fn from_ref(inner: &Inner) -> Option<&Self> {
		match inner {
			Inner::None => None,
			// SAFETY: `Self` is a transparent wrapper for `Inner`.
			inner => unsafe { Some(&*(inner as *const Inner as *const Self)) }
		}
	}
}

impl AsObject<CvarVt> for Cvar {
	fn as_object(&self) -> &VtObject<CvarVt> {
		self.as_v4()
	}
}

enum Inner {
	None,
	V4(ICvar),
}

static CVAR: UnsafeExclusive<Inner> = UnsafeExclusive::new(Inner::None);

const FIRST_INIT_DLL_ID: CvarDllIdentifier = 0;
const UNINIT_DLL_ID: CvarDllIdentifier = FIRST_INIT_DLL_ID - 1;

static DLL_IDENTIFIER: Atomic<CvarDllIdentifier> = Atomic::new(UNINIT_DLL_ID);

pub fn dll_identifier() -> CvarDllIdentifier {
	DLL_IDENTIFIER.load(Ordering::Relaxed)
}

pub fn reset_dll_identifier() {
	unsafe { set_dll_identifier(UNINIT_DLL_ID) }
}

/// # Safety
/// `dll_identifier` must be a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe fn set_dll_identifier(dll_id: CvarDllIdentifier) {
	DLL_IDENTIFIER.store(dll_id, Ordering::Relaxed);
}
