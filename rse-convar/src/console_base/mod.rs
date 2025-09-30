pub use crate::cppdef::CvarDllIdentifier;

mod wrapper;
pub use wrapper::*;

/// # Safety
/// `help` must modify `Object` so that the help text string is stored inside of `Object`.
/// 
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait RawConsoleBase<Object: ?Sized> {
	fn help(object: &mut Object);
	fn dll_identifier(object: &mut Object) -> CvarDllIdentifier;
}
