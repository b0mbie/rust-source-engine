#[macro_export]
macro_rules! dev_msg {
	() => {
		$crate::msg!($crate::linked::dev())
	};

	($($arg:tt)+) => {
		$crate::msg!($crate::linked::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! dev_warn {
	() => {
		$crate::warn!($crate::linked::dev())
	};

	($($arg:tt)+) => {
		$crate::warn!($crate::linked::dev(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_msg {
	() => {
		$crate::msg!($crate::linked::con())
	};

	($($arg:tt)+) => {
		$crate::msg!($crate::linked::con(), $($arg)+)
	};
}

#[macro_export]
macro_rules! con_warn {
	() => {
		$crate::warn!($crate::linked::con())
	};

	($($arg:tt)+) => {
		$crate::warn!($crate::linked::con(), $($arg)+)
	};
}
