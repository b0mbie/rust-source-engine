use ::core::{
	ffi::CStr,
};

use super::{
	Suggestions, Invocation,
};

pub trait DispatchCommand {
	fn dispatch(&mut self, invocation: &Invocation);
	fn can_auto_complete(&mut self) -> bool {
		false
	}
	fn auto_complete(&mut self, partial: &CStr, suggestions: &mut Suggestions) {
		let _ = partial;
		let _ = suggestions;
	}
}
