use ::core::ffi::CStr;

use crate::{
	Level, Color,
};

pub trait Tier0Spew<T> {
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

pub trait Tier0 {
	fn msg(&self, s: &CStr);
	fn msg_with(&self, group: &CStr, level: Level, s: &CStr);
	fn warning(&self, s: &CStr);
	fn warning_with(&self, group: &CStr, level: Level, s: &CStr);
	fn log(&self, s: &CStr);
	fn log_with(&self, group: &CStr, level: Level, s: &CStr);
}

pub trait LevelLogger<T> {
	fn msg_on(&self, level: Level, t: T);
	fn warning_on(&self, level: Level, t: T);
	fn log_on(&self, level: Level, t: T);
}
impl<L: for<'a> LevelLogger<&'a str>> LevelLogger<char> for L {
	fn msg_on(&self, level: Level, c: char) {
		let mut buffer = [0; 4];
		L::msg_on(self, level, c.encode_utf8(&mut buffer))
	}
	fn warning_on(&self, level: Level, c: char) {
		let mut buffer = [0; 4];
		L::warning_on(self, level, c.encode_utf8(&mut buffer))
	}
	fn log_on(&self, level: Level, c: char) {
		let mut buffer = [0; 4];
		L::log_on(self, level, c.encode_utf8(&mut buffer))
	}
}

pub trait ColorLevelLogger<T> {
	fn color_msg_on(&self, level: Level, color: &Color, t: T);
}
impl<L: for<'a> ColorLevelLogger<&'a str>> ColorLevelLogger<char> for L {
	fn color_msg_on(&self, level: Level, color: &Color, c: char) {
		let mut buffer = [0; 4];
		L::color_msg_on(self, level, color, c.encode_utf8(&mut buffer))
	}
}

pub trait Logger<T> {
	fn msg(&self, t: T);
	fn warning(&self, t: T);
	fn log(&self, t: T);
}
impl<L: for<'a> Logger<&'a str>> Logger<char> for L {
	fn msg(&self, c: char) {
		let mut buffer = [0; 4];
		L::msg(self, c.encode_utf8(&mut buffer))
	}
	fn warning(&self, c: char) {
		let mut buffer = [0; 4];
		L::warning(self, c.encode_utf8(&mut buffer))
	}
	fn log(&self, c: char) {
		let mut buffer = [0; 4];
		L::log(self, c.encode_utf8(&mut buffer))
	}
}

pub trait ColorLogger<T> {
	fn color_msg(&self, color: &Color, t: T);
}
impl<L: for<'a> ColorLogger<&'a str>> ColorLogger<char> for L {
	fn color_msg(&self, color: &Color, c: char) {
		let mut buffer = [0; 4];
		L::color_msg(self, color, c.encode_utf8(&mut buffer))
	}
}
