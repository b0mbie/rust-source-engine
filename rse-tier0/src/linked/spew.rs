use ::core::ffi::CStr;

use crate::{
	Tier0Spew, Tier0SpewGroups, CFormattable,
	Level, Color,
};

use super::{
	cppdef::*,
	STR_FORMAT, LinkedTier0,
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

impl<T: CFormattable> Tier0Spew<T> for LinkedTier0 {
	fn msg(&self, t: T) {
		unsafe { Msg(T::FORMAT_STR.as_ptr(), t.into_c_type()) }
	}
	fn msg_with(&self, group: &CStr, level: crate::Level, t: T) {
		unsafe { DMsg(group.as_ptr(), level.0, T::FORMAT_STR.as_ptr(), t.into_c_type()) }	
	}
	fn warning(&self, t: T) {
		unsafe { Warning(T::FORMAT_STR.as_ptr(), t.into_c_type()) }
	}
	fn warning_with(&self, group: &CStr, level: Level, t: T) {
		unsafe { DWarning(group.as_ptr(), level.0, T::FORMAT_STR.as_ptr(), t.into_c_type()) }	
	}
	fn log(&self, t: T) {
		unsafe { Log(T::FORMAT_STR.as_ptr(), t.into_c_type()) }
	}
	fn log_with(&self, group: &CStr, level: Level, t: T) {
		unsafe { DLog(group.as_ptr(), level.0, T::FORMAT_STR.as_ptr(), t.into_c_type()) }	
	}
	fn timestamped_log(&self, t: T) {
		unsafe { COM_TimestampedLog(T::FORMAT_STR.as_ptr(), t.into_c_type()) }
	}
}
impl Tier0Spew<&str> for LinkedTier0 {
	fn msg(&self, s: &str) {
		unsafe { Msg(STR_FORMAT, s.len(), s.as_ptr()) }
	}
	fn msg_with(&self, group: &CStr, level: crate::Level, s: &str) {
		unsafe { DMsg(group.as_ptr(), level.0, STR_FORMAT, s.len(), s.as_ptr()) }	
	}
	fn warning(&self, s: &str) {
		unsafe { Warning(STR_FORMAT, s.len(), s.as_ptr()) }
	}
	fn warning_with(&self, group: &CStr, level: Level, s: &str) {
		unsafe { DWarning(group.as_ptr(), level.0, STR_FORMAT, s.len(), s.as_ptr()) }	
	}
	fn log(&self, s: &str) {
		unsafe { Log(STR_FORMAT, s.len(), s.as_ptr()) }
	}
	fn log_with(&self, group: &CStr, level: Level, s: &str) {
		unsafe { DLog(group.as_ptr(), level.0, STR_FORMAT, s.len(), s.as_ptr()) }	
	}
	fn timestamped_log(&self, s: &str) {
		unsafe { COM_TimestampedLog(STR_FORMAT, s.len(), s.as_ptr()) }
	}
}

impl<T: CFormattable> Tier0SpewGroups<T> for LinkedTier0 {
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
impl Tier0SpewGroups<&str> for LinkedTier0 {
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
		impl<T: CFormattable> crate::LevelLogger<T> for $target {
			fn msg_on(&self, level: Level, t: T) {
				unsafe { $msg(level.0, <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
			fn warning_on(&self, level: Level, t: T) {
				unsafe { $warning(level.0, <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
			fn log_on(&self, level: Level, t: T) {
				unsafe { $log(level.0, <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
		}

		impl crate::LevelLogger<&str> for $target {
			fn msg_on(&self, level: Level, s: &str) {
				unsafe { $msg(level.0, STR_FORMAT, s.len(), s.as_ptr()) }
			}
			fn warning_on(&self, level: Level, s: &str) {
				unsafe { $warning(level.0, STR_FORMAT, s.len(), s.as_ptr()) }
			}
			fn log_on(&self, level: Level, s: &str) {
				unsafe { $log(level.0, STR_FORMAT, s.len(), s.as_ptr()) }
			}
		}
	};
}

macro_rules! impl_color_level_logger {
	($target:ty: $color_msg:expr) => {
		impl<T: CFormattable> crate::ColorLevelLogger<T> for $target {
			fn color_msg_on(&self, level: Level, color: &Color, t: T) {
				unsafe { $color_msg(
					level.0, ::rse_cpp::RefConst::from(color),
					<T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type(),
				) }
			}
		}

		impl crate::ColorLevelLogger<&str> for $target {
			fn color_msg_on(&self, level: Level, color: &Color, s: &str) {
				unsafe { $color_msg(
					level.0, ::rse_cpp::RefConst::from(color),
					STR_FORMAT, s.len(), s.as_ptr(),
				) }
			}
		}
	};
}

macro_rules! impl_logger {
	($target:ty: $msg:expr, $warning:expr, $log:expr $(, $level:expr)?) => {
		impl<T: CFormattable> crate::Logger<T> for $target {
			fn msg(&self, t: T) {
				unsafe { $msg($($level.0,)? <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
			fn warning(&self, t: T) {
				unsafe { $warning($($level.0,)? <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
			fn log(&self, t: T) {
				unsafe { $log($($level.0,)? <T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type()) }
			}
		}

		impl crate::Logger<&str> for $target {
			fn msg(&self, s: &str) {
				unsafe { $msg($($level.0,)? STR_FORMAT, s.len(), s.as_ptr()) }
			}
			fn warning(&self, s: &str) {
				unsafe { $warning($($level.0,)? STR_FORMAT, s.len(), s.as_ptr()) }
			}
			fn log(&self, s: &str) {
				unsafe { $log($($level.0,)? STR_FORMAT, s.len(), s.as_ptr()) }
			}
		}
	};
}

macro_rules! impl_color_logger {
	($target:ty: $color_msg:expr $(, $level:expr)?) => {
		impl<T: CFormattable> crate::ColorLogger<T> for $target {
			fn color_msg(&self, color: &Color, t: T) {
				unsafe { $color_msg(
					$($level.0,)? ::rse_cpp::RefConst::from(color),
					<T as CFormattable>::FORMAT_STR.as_ptr(), t.into_c_type(),
				) }
			}
		}

		impl crate::ColorLogger<&str> for $target {
			fn color_msg(&self, color: &Color, s: &str) {
				unsafe { $color_msg(
					$($level.0,)? ::rse_cpp::RefConst::from(color),
					STR_FORMAT, s.len(), s.as_ptr(),
				) }
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
