#[macro_export]
macro_rules! flag_consts {
	{
		for $ty:ty:
		$(
			$(#[$attr:meta])*
			$vis:vis $name:ident = $value:expr;
		)*
	} => {
		$(
			$(#[$attr])*
			$vis const $name: $ty = $value;
		)*
	};
}

#[macro_export]
macro_rules! bit_flag_consts {
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

#[macro_export]
macro_rules! test_bits {
	($self:expr, $bits:expr) => {
		($self.0 & $bits) != 0
	};
}

#[macro_export]
macro_rules! with_bits {
	($self:expr, $bits:expr) => {
		Self($self.0 | $bits)
	};
}
