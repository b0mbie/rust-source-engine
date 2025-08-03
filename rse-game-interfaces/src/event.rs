use ::core::ffi::CStr;

pub trait NamedEvent: Event {
	const NAME: &CStr;
}

pub trait Event {
	const IS_SERVER_SIDE: bool;
}
