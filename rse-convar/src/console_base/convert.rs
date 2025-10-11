use crate::cppdef::ConCommandBase;

pub type Registrable = *mut ConCommandBase;

/// # Safety
/// `as_registrable` must return a valid, mutable [`ConCommandBase`]
/// that can be registered with the `ICvar` interface.
pub unsafe trait AsRegistrable {
	fn as_registrable(&mut self) -> Registrable;
}
