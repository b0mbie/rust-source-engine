use ::core::pin::Pin;

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
	fn help(object: Pin<&mut Object>);
	fn add_flags(object: Pin<&mut Object>, flags: CvarFlags);
	fn is_registered(object: Pin<&mut Object>) -> bool;
	fn dll_identifier(object: Pin<&mut Object>) -> CvarDllIdentifier;

	// TODO: Is this useful?
	fn init(object: Pin<&mut Object>) {
		let _ = object;
	}

	/// Allow the implementing type to use the data of `Object`
	/// to properly destroy itself.
	/// 
	/// # Safety
	/// This function must only be called *once* by `Object`
	/// (typically, this would be done in its [`Drop`] implementation).
	unsafe fn drop_with_object(object: &mut Object) {
		let _ = object;
	}
}
