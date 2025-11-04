pub use crate::cppdef::CvarDllIdentifier;

mod ext;
pub use ext::*;
mod flags;
pub use flags::*;
mod registrable;
pub use registrable::*;

/// # Safety
/// The following functions must modify `Object`
/// so that the corresponding value is stored inside of `Object`:
/// - `help`
/// - `add_flags`
/// 
/// `is_registered` must return a value that is stored inside of `Object`.
/// 
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait RawConsoleBase<Object: ?Sized> {
	fn help(object: &mut Object);
	fn add_flags(object: &mut Object, flags: CvarFlags);
	fn is_registered(object: &mut Object) -> bool;
	fn dll_identifier(object: &mut Object) -> CvarDllIdentifier;

	fn init(object: &mut Object) {
		let _ = object;
	}
}
