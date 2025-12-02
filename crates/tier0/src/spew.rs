use ::core::ffi::CStr;

use crate::{
	Level, Color,
};

pub trait Tier0Spew<T> {
	fn msg(&self, t: T);
	fn msg_with(&self, group: &CStr, level: Level, t: T);
	fn warning(&self, t: T);
	fn warning_with(&self, group: &CStr, level: Level, t: T);
	fn log(&self, t: T);
	fn log_with(&self, group: &CStr, level: Level, t: T);
	fn timestamped_log(&self, t: T);
}

pub trait Tier0SpewGroups<T> {
	type DevGroup<'a>: LevelLogger<T> + Logger<T> where Self: 'a;
	fn dev_group(&self) -> Self::DevGroup<'_>;

	type ConGroup<'a>: LevelLogger<T> + Logger<T> + ColorLevelLogger<T> + ColorLogger<T> where Self: 'a;
	fn con_group(&self) -> Self::ConGroup<'_>;

	// Technically a pseudo-group. Valve's tier0 uses the `GROUP_CONSOLE` group for these, just with level 2.
	type DevConGroup<'a>: Logger<T> + ColorLogger<T> where Self: 'a;
	fn dev_con_group(&self) -> Self::DevConGroup<'_>;

	type NetGroup<'a>: LevelLogger<T> where Self: 'a;
	fn net_group(&self) -> Self::NetGroup<'_>;
}

pub trait LevelLogger<T> {
	fn msg_on(&self, level: Level, t: T);
	fn warning_on(&self, level: Level, t: T);
	fn log_on(&self, level: Level, t: T);
}

pub trait ColorLevelLogger<T> {
	fn color_msg_on(&self, level: Level, color: &Color, t: T);
}

pub trait Logger<T> {
	fn msg(&self, t: T);
	fn warning(&self, t: T);
	fn log(&self, t: T);
}

pub trait ColorLogger<T> {
	fn color_msg(&self, color: &Color, t: T);
}
