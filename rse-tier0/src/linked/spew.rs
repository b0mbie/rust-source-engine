use ::core::ffi::CStr;
use ::printf::IntoFormattable;

use crate::{
	Tier0Spew, Tier0SpewGroups,
	Level, Color,
};

use super::call_printf;

use super::{
	cppdef::*,
	LinkedTier0,
};

pub const fn dev() -> LinkedTier0Dev {
	LinkedTier0Dev
}

pub const fn con() -> LinkedTier0Con {
	LinkedTier0Con
}

pub const fn dev_con() -> LinkedTier0DevCon {
	LinkedTier0DevCon
}

impl<T: IntoFormattable> Tier0Spew<T> for LinkedTier0 {
	fn msg(&self, t: T) {
		unsafe { call_printf!(t => Msg) }
	}
	fn msg_with(&self, group: &CStr, level: crate::Level, t: T) {
		unsafe { call_printf!(t => DMsg, group.as_ptr(), level.0) }
	}
	fn warning(&self, t: T) {
		unsafe { call_printf!(t => Warning) }
	}
	fn warning_with(&self, group: &CStr, level: Level, t: T) {
		unsafe { call_printf!(t => DWarning, group.as_ptr(), level.0) }
	}
	fn log(&self, t: T) {
		unsafe { call_printf!(t => Log) }
	}
	fn log_with(&self, group: &CStr, level: Level, t: T) {
		unsafe { call_printf!(t => DLog, group.as_ptr(), level.0) }
	}
	fn timestamped_log(&self, t: T) {
		unsafe { call_printf!(t => COM_TimestampedLog) }
	}
}

impl<T: IntoFormattable> Tier0SpewGroups<T> for LinkedTier0 {
	type DevGroup<'a> = LinkedTier0Dev;
	fn dev_group(&self) -> Self::DevGroup<'_> {
		LinkedTier0Dev
	}

	type ConGroup<'a> = LinkedTier0Con;
	fn con_group(&self) -> Self::ConGroup<'_> {
		LinkedTier0Con
	}

	type DevConGroup<'a> = LinkedTier0DevCon;
	fn dev_con_group(&self) -> Self::DevConGroup<'_> {
		LinkedTier0DevCon
	}

	type NetGroup<'a> = LinkedTier0Net;
	fn net_group(&self) -> Self::NetGroup<'_> {
		LinkedTier0Net
	}
}

macro_rules! impl_level_logger {
	($target:ty: $msg:expr, $warning:expr, $log:expr) => {
		impl<T: IntoFormattable> crate::LevelLogger<T> for $target {
			fn msg_on(&self, level: Level, t: T) {
				unsafe { call_printf!(t => $msg, level.0) }
			}
			fn warning_on(&self, level: Level, t: T) {
				unsafe { call_printf!(t => $warning, level.0) }
			}
			fn log_on(&self, level: Level, t: T) {
				unsafe { call_printf!(t => $log, level.0) }
			}
		}
	};
}

macro_rules! impl_color_level_logger {
	($target:ty: $color_msg:expr) => {
		impl<T: IntoFormattable> crate::ColorLevelLogger<T> for $target {
			fn color_msg_on(&self, level: Level, color: &Color, t: T) {
				unsafe { call_printf!(t => $color_msg, level.0, ::rse_cpp::RefConst::from(color)) }
			}
		}
	};
}

macro_rules! impl_logger {
	($target:ty: $msg:expr, $warning:expr, $log:expr $(, $level:expr)?) => {
		impl<T: IntoFormattable> crate::Logger<T> for $target {
			fn msg(&self, t: T) {
				unsafe { call_printf!(t => $msg $(, $level.0)?) }
			}
			fn warning(&self, t: T) {
				unsafe { call_printf!(t => $warning $(, $level.0)?) }
			}
			fn log(&self, t: T) {
				unsafe { call_printf!(t => $log $(, $level.0)?) }
			}
		}
	};
}

macro_rules! impl_color_logger {
	($target:ty: $color_msg:expr $(, $level:expr)?) => {
		impl<T: IntoFormattable> crate::ColorLogger<T> for $target {
			fn color_msg(&self, color: &Color, t: T) {
				unsafe { call_printf!(t => $color_msg, $($level.0,)? ::rse_cpp::RefConst::from(color)) }
			}
		}
	};
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0Dev;
impl_level_logger!(LinkedTier0Dev: DevMsg, DevWarning, DevLog);
impl_logger!(LinkedTier0Dev: DevMsg, DevWarning, DevLog, Level::DEFAULT);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0Con;
impl_level_logger!(LinkedTier0Con: ConMsg, ConWarning, ConLog);
impl_logger!(LinkedTier0Con: ConMsg, ConWarning, ConLog, Level::DEFAULT);
impl_color_level_logger!(LinkedTier0Con: ConColorMsg);
impl_color_logger!(LinkedTier0Con: ConColorMsg, Level::DEFAULT);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0DevCon;
impl_logger!(LinkedTier0DevCon: ConDMsg, ConDWarning, ConDLog);
impl_color_logger!(LinkedTier0DevCon: ConDColorMsg);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0Net;
impl_level_logger!(LinkedTier0Net: NetMsg, NetWarning, NetLog);
