macro_rules! channels {
	{
		$(
			$(#[$attr:meta])*
			$name:ident = $value:expr,
		)*
	} => {
		$(
			$(#[$attr])*
			pub const $name: ::core::ffi::c_int = $value;
		)*
	};
}

channels! {
	CHAN_REPLACE = -1,
	CHAN_AUTO = 0,
	CHAN_WEAPON = 1,
	CHAN_VOICE = 2,
	CHAN_ITEM = 3,
	CHAN_BODY = 4,
	CHAN_STREAM = 5,
	CHAN_STATIC = 6,
	CHAN_VOICE2 = 7,
	/// Network voice data.
	CHAN_VOICE_BASE = 8,
	/// Anything greater than or equal to this number is allocated to game code.
	CHAN_USER_BASE = CHAN_VOICE_BASE + 128,
}
