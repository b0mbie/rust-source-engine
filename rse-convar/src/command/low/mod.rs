use crate::{
	console_base::RawConsoleBase,
	Invocation,
};

mod object;
pub use object::*;

/// # Safety
/// `name` must modify [`ConCommandObject<Self>`] so that the name string is stored inside of it.
/// It is also desirable that the the string doesn't change.
pub unsafe trait RawCommand<'a>
where
	Self: Sized,
	Self: RawConsoleBase<ConCommandObject<'a, Self>>,
{
	fn name(object: &mut ConCommandObject<Self>);
	fn dispatch(object: &mut ConCommandObject<Self>, invocation: &Invocation);
}
