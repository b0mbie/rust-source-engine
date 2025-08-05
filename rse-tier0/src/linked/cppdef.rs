use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::RefConst;

use crate::Color;

unsafe extern "C" {
	pub fn Msg(msg: *const c_char, ...);
	pub fn DMsg(group_name: *const c_char, level: c_int, msg: *const c_char, ...);

	pub fn Warning(msg: *const c_char, ...);
	pub fn DWarning(group_name: *const c_char, level: c_int, msg: *const c_char, ...);

	pub fn Log(msg: *const c_char, ...);
	pub fn DLog(group_name: *const c_char, level: c_int, msg: *const c_char, ...);

	pub fn DevMsg(level: c_int, msg: *const c_char, ...);
	pub fn DevWarning(level: c_int, msg: *const c_char, ...);
	pub fn DevLog(level: c_int, msg: *const c_char, ...);
	
	pub fn ConColorMsg(level: c_int, clr: RefConst<Color>, msg: *const c_char, ...);
	pub fn ConMsg(level: c_int, msg: *const c_char, ...);
	pub fn ConWarning(level: c_int, msg: *const c_char, ...);
	pub fn ConLog(level: c_int, msg: *const c_char, ...);
	
	pub fn ConDColorMsg(clr: RefConst<Color>, msg: *const c_char, ...);
	pub fn ConDMsg(msg: *const c_char, ...);
	pub fn ConDWarning(msg: *const c_char, ...);
	pub fn ConDLog(msg: *const c_char, ...);
	
	pub fn NetMsg(level: c_int, msg: *const c_char, ...);
	pub fn NetWarning(level: c_int, msg: *const c_char, ...);
	pub fn NetLog(level: c_int, msg: *const c_char, ...);
}
