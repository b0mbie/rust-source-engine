use ::core::ffi::CStr;

use crate::GameEvent;

pub trait RecvEvent: NamedEvent + for<'a> TryFrom<GameEvent<'a>> {}
pub trait SendEvent: NamedEvent + for<'a> Into<GameEvent<'a>> {}

pub trait NamedEvent: Event {
	const NAME: &CStr;
}

pub trait Event {
	const IS_SERVER_SIDE: bool;
}
