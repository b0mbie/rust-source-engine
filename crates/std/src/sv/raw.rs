use ::rse_cpp::{
	AsObject, VtObject,
};
use ::rse_server::VEngineServer;

use crate::{
	plugin::PluginFactories,
	raw::global_interface::GlobalInterface,
};

pub use ::rse_server::{
	cppdef::VEngineServerVt,
	VEngineServerImpl,
};

/// Tries to initialize the `IVEngineServer` functions in this module.
/// 
/// # Safety
/// This function must be called from the main thread.
pub(crate) unsafe fn attach(factories: PluginFactories) -> bool {
	let result = factories
		.create_interface().map(Inner::V23)
		;
	match result {
		Ok(inner) => {
			unsafe { SERVER.inspect_unchecked(move |sv| *sv = inner) }
			true
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
			false
		}
	}
}

#[cold]
const fn not_init() -> ! {
	panic!("server interface used without being initialized")
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
pub unsafe fn inspect<R, F: FnOnce(Option<&mut Server>) -> R>(f: F) -> R {
	let f = move |inner: Option<&mut Inner>| {
		let sv = inner.map(move |inner| {
			match Server::from_mut(inner) {
				Some(sv) => sv,
				None => not_init(),
			}
		});
		f(sv)
	};
	unsafe { SERVER.inspect(f) }
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
/// 
/// Additionally, the operations performed on the interface
/// *must* support multi-threading.
pub unsafe fn inspect_mt<R, F: FnOnce(&Server) -> R>(f: F) -> R {
	let f = move |inner: &Inner| {
		match Server::from_ref(inner) {
			Some(sv) => f(sv),
			None => not_init(),
		}
	};
	unsafe { SERVER.inspect_mt(f) }
}

#[repr(transparent)]
pub struct Server {
	// INVARIANT: This value is never `Inner::None`.
	inner: Inner,
}

impl Server {
	pub fn as_v23(&self) -> &VtObject<VEngineServerVt> {
		match self.inner {
			Inner::V23(ref sv) => sv.as_object(),
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

impl AsObject<VEngineServerVt> for Server {
	fn as_object(&self) -> &VtObject<VEngineServerVt> {
		self.as_v23()
	}
}

enum Inner {
	None,
	V23(VEngineServer),
}

static SERVER: GlobalInterface<Inner> = GlobalInterface::new(Inner::None);
