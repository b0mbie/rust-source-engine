use ::core::ffi::CStr;

use crate::{
	console_base::{
		RawConsoleBase,
		CvarFlags,
	},
	Invocation,
};

pub use ::rse_utl::CString;

mod object;
pub use object::*;
mod suggestion_count;
pub use suggestion_count::*;
mod suggestions;
pub use suggestions::*;

/// # Safety
/// `name` must modify [`ConCommandObject<Self>`] so that the name string is stored inside of it.
/// It is also desirable that the string doesn't change.
/// 
/// `auto_complete_suggest` must return a [`SuggestionCount`] that is less than or equal to
/// the number of suggestion strings pushed.
pub unsafe trait RawCommand<'a>
where
	Self: Sized,
	Self: RawConsoleBase<ConCommandObject<'a, Self>>,
{
	fn name(object: &mut ConCommandObject<'a, Self>);
	fn dispatch(object: &mut ConCommandObject<'a, Self>, invocation: &Invocation);
	fn can_auto_complete(object: &mut ConCommandObject<'a, Self>) -> bool;
	fn auto_complete_suggest(
		object: &mut ConCommandObject<'a, Self>,
		partial: &CStr,
		suggestions: &mut Suggestions,
	) -> SuggestionCount;

	fn is_flag_set(object: &mut ConCommandObject<'a, Self>, flag: CvarFlags) -> bool {
		object.as_base().is_flag_set(flag)
	}
}
