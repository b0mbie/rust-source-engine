use crate::cppdef::ConCommandBase;

/// # Safety
/// `as_con_command_base` must return a valid, mutable [`ConCommandBase`].
pub unsafe trait AsConCommandBase {
	fn as_con_command_base(&mut self) -> *mut ConCommandBase;
}
