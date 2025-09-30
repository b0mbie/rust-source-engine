use ::core::ffi::c_int;

// TODO: Make this more usable?

macro_rules! flags {
	{
		for $ty:ty:
		$(
			$(#[$attr:meta])*
			$vis:vis $name:ident = $offset:expr;
		)*
	} => {
		$($vis const $name: $ty = 1 << $offset;)*
	};
}

flags! {
	for c_int:
	pub UNREGISTERED = 0;
	pub DEVELOPMENT_ONLY = 1;
	pub GAMEDLL = 2;
	pub CLIENTDLL = 3;
	pub HIDDEN = 4;

	pub PROTECTED = 5;
	pub SP_ONLY = 6;
	pub ARCHIVE = 7;
	pub NOTIFY = 8;
	pub USERINFO = 9;
	pub CHEAT = 14;

	pub PRINTABLE_ONLY = 10;
	pub UNLOGGED = 11;
	pub NEVER_AS_STRING = 12;

	pub REPLICATED = 13;
	pub DEMO = 16;
	pub DONT_RECORD = 17;
	pub RELOAD_MATERIALS = 20;
	pub RELOAD_TEXTURES = 21;

	pub NOT_CONNECTED = 22;
	pub MATERIAL_SYSTEM_THREAD = 23;
	pub ARCHIVE_XBOX = 24;

	pub ACCESSIBLE_FROM_THREADS = 25;

	pub SERVER_CAN_EXECUTE = 28;
	pub SERVER_CANNOT_QUERY = 29;
	pub CLIENTCMD_CAN_EXECUTE = 30;

	pub EXEC_DESPITE_DEFAULT = 31;

	pub INTERNAL_USE = 13;
	pub ALLOWED_IN_COMPETITIVE = 18;
}

pub const MATERIAL_THREAD_MASK: c_int = RELOAD_MATERIALS | RELOAD_TEXTURES | MATERIAL_SYSTEM_THREAD;
