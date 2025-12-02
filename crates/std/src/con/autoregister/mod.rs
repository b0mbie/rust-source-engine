use ::core::{
	ptr::null_mut,
	sync::atomic::{
		Ordering, AtomicPtr,
	},
};
use ::rse_convar::{
	cppdef::Registrable,
	console_base::RegistrableMut,
};

use super::cvar::register_raw;

mod macros;
pub use macros::*;

pub fn add_to_register(registrable: &'static dyn AsStaticRegistrable) {
	let mut new_first = registrable.as_static_registrable();
	let old_first = FIRST_REGISTRABLE.swap(null_mut(), Ordering::Acquire);
	unsafe { new_first.as_mut_base().data.next = old_first }
	FIRST_REGISTRABLE.store(new_first.as_ptr().as_ptr(), Ordering::Release);
}

pub fn register_all() {
	let mut current = FIRST_REGISTRABLE.swap(null_mut(), Ordering::SeqCst);
	while let Some(reg) = RegistrableMut::new(current) {
		unsafe {
			current = reg.as_ref().data.next;
			register_raw(reg);
		}
	}
}

static FIRST_REGISTRABLE: AtomicPtr<Registrable> = AtomicPtr::new(null_mut());

#[diagnostic::on_unimplemented(message = "`{Self}` is not a registrable ConVar or ConCommand")]
pub trait AsStaticRegistrable {
	fn as_static_registrable(&'static self) -> StaticRegistrable;
}

impl AsStaticRegistrable for crate::con::cmd::ConCommand {
	fn as_static_registrable(&'static self) -> StaticRegistrable {
		unsafe { StaticRegistrable::new(self.as_registrable()) }
	}
}
impl<T> AsStaticRegistrable for crate::con::cmd::GenericConCommand<'_, T> {
	fn as_static_registrable(&'static self) -> StaticRegistrable {
		unsafe { StaticRegistrable::new(self.as_registrable()) }
	}
}
impl AsStaticRegistrable for crate::con::var::ConVar {
	fn as_static_registrable(&'static self) -> StaticRegistrable {
		unsafe { StaticRegistrable::new(self.as_registrable()) }
	}
}
impl<T> AsStaticRegistrable for crate::con::var::GenericConVar<'_, T> {
	fn as_static_registrable(&'static self) -> StaticRegistrable {
		unsafe { StaticRegistrable::new(self.as_registrable()) }
	}
}
impl<T> AsStaticRegistrable for crate::con::var::TypedConVar<T> {
	fn as_static_registrable(&'static self) -> StaticRegistrable {
		unsafe { StaticRegistrable::new(self.as_registrable()) }
	}
}

#[repr(transparent)]
pub struct StaticRegistrable(RegistrableMut);
unsafe impl Send for StaticRegistrable {}

impl StaticRegistrable {
	/// # Safety
	/// `ptr` must point to a registrable ConVar or ConCommand
	/// that is valid for the `'static` lifetime.
	pub const unsafe fn new(ptr: RegistrableMut) -> Self {
		Self(ptr)
	}

	const unsafe fn as_mut_base(&mut self) -> &mut Registrable {
		unsafe { self.0.as_mut() }
	}

	const fn as_ptr(&mut self) -> RegistrableMut {
		self.0
	}
}
