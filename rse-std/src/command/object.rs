use ::rse_convar::{
	cppdef::ConCommandBase,
	console_base::AsRegistrable,
	command::{
		low::ConCommandObject,
		Command,
	},
};

use super::StdCommand;

#[repr(transparent)]
pub struct ConCommand<T> {
	con_command: ConCommandObject<'static, StdCommand<T>>,
}

impl<T> ConCommand<T>
where
	T: Command,
{
	pub const fn new(inner: T) -> Self {
		Self {
			con_command: ConCommandObject::new(
				StdCommand::new(inner),
				T::NAME, T::HELP, T::FLAGS,
			)
		}
	}
}

unsafe impl<T> AsRegistrable for ConCommand<T> {
	fn as_registrable(&mut self) -> *mut ConCommandBase {
		self.con_command.as_registrable()
	}
}
