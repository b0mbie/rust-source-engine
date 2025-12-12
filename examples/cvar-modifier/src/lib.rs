use rse_std::prelude::*;
use rse_std::{
	con::{
		cmd::{
			Partial, Suggestions,
			CmdBuffer, Arg,
		},
		cvar::with_cvars,
		flag::{
			all_flags, str_to_flag, strings_of,
		},
		Registered,
	},
	io::con,
};
use std::convert::Infallible;

fn for_registered<F: FnMut(&Registered) -> bool>(name: &[u8], mut f: F) {
	with_cvars(move |cvars| {
		for reg in cvars.registered() {
			let reg_name = reg.name().to_bytes();
			if (name.is_empty()
			|| reg_name.get(..name.len())
				.is_some_and(move |prefix| prefix.eq_ignore_ascii_case(name)))
			&& !f(reg) {
				break
			}
		}
	});
}

fn suggest(part: Partial<'_>, out: &mut Suggestions) {
	let (i, arg, before) = part.split_last();
	if i == 0 {
		for_registered(arg, move |reg| {
			let mut buf = CmdBuffer::new();
			buf.write(move |f| {
				f.write(before);
				f.write(reg.name().to_bytes());
			});
			out.try_push(buf)
		});
	}
}

fn suggest_vars(part: Partial<'_>, out: &mut Suggestions) {
	let (i, arg, before) = part.split_last();
	if i == 0 {
		for_registered(arg, move |reg| {
			if reg.is_command() {
				return true
			}

			let mut buf = CmdBuffer::new();
			buf.write(move |f| {
				f.write(before);
				f.write(reg.name().to_bytes());
			});
			out.try_push(buf)
		});
	}
}

fn push_flag_suggestions(arg: &[u8], before: &[u8], out: &mut Suggestions) {
	for (flag, ..) in all_flags() {
		let flag = flag.as_bytes();
		if !flag.starts_with(arg) {
			continue
		}

		let mut buf = CmdBuffer::new();
		buf.write(move |f| {
			f.write(before);
			f.write(flag);
		});
		if !out.try_push(buf) { break }
	}
}

fn suggest_flags_for(part: Partial<'_>, out: &mut Suggestions) {
	let (i, arg, before) = part.split_last();
	if i == 0 {
		for_registered(arg, move |reg| {
			let mut buf = CmdBuffer::new();
			buf.write(move |f| {
				f.write(before);
				f.write(reg.name().to_bytes());
			});
			out.try_push(buf)
		});
	} else {
		push_flag_suggestions(arg, before, out)
	}
}

macro_rules! unwrap_flags {
	($rlt:expr) => {
		match $rlt {
			Ok(t) => t,
			Err(e) => {
				con_warn!("Invalid flag: {e:?}");
				return
			}
		}
	};
}

macro_rules! unwrap_reg {
	($name:expr => $opt:expr) => {
		match $opt {
			Some(t) => t,
			None => {
				con_warn!("Unknown console variable or command: {:?}", $name);
				return
			}
		}
	};
}

#[con_command(complete = suggest)]
fn cm_flags(cmd: &Invocation) {
	let [name] = cmd.args() else {
		con_warn!("Expected name of console variable or command");
		return
	};
	let name = name.as_c_str();
	with_cvars(move |cvars| {
		let reg = unwrap_reg!(name => cvars.find(name));
		for flag in strings_of(reg.flags_ref()) {
			con().msg_raw(flag.unwrap_or("<unknown>"));
			con().msg_raw(' ');
		}
		con().msg_raw('\n');
	});
}

fn parse_flags(flags: &[Arg]) -> Result<CvarFlags, &CStr> {
	let mut result = CvarFlags::empty();
	for flag in flags {
		let flag = flag.as_c_str();
		if let Some(flag) = str_to_flag(flag.to_bytes()) {
			result = result.union(flag);
		} else {
			return Err(flag)
		}
	}
	Ok(result)
}

#[con_command(complete = suggest_flags_for)]
fn cm_flags_add(cmd: &Invocation) {
	let [name, flags @ ..] = cmd.args() else {
		con_warn!("Expected name of console variable or command, and the integer flags to add");
		return
	};
	let name = name.as_c_str();
	let flags = unwrap_flags!(parse_flags(flags));

	with_cvars(move |mut cvars| {
		let reg = unwrap_reg!(name => cvars.find_mut(name));
		reg.add_flags(flags);
	});
}

#[con_command(complete = suggest_flags_for)]
fn cm_flags_remove(cmd: &Invocation) {
	let [name, flags @ ..] = cmd.args() else {
		con_warn!("Expected name of console variable or command, and the integer flags to remove");
		return
	};
	let name = name.as_c_str();
	let flags = unwrap_flags!(parse_flags(flags));
	
	with_cvars(move |mut cvars| {
		let reg = unwrap_reg!(name => cvars.find_mut(name));
		let reg_flags = reg.flags_mut();
		*reg_flags = reg_flags.difference(flags);
	});
}

macro_rules! unwrap_var {
	($name:expr => $opt:expr) => {
		match $opt {
			Some(t) => t,
			None => {
				con_warn!("Unknown console variable: {:?}", $name);
				return
			}
		}
	};
}

macro_rules! unwrap_lim {
	($rlt:expr) => {
		match $rlt {
			Ok(t) => t,
			Err(e) => {
				con_warn!("Failed to parse value limit: {e}");
				return
			}
		}
	};
}

#[con_command(complete = suggest_vars)]
fn cm_min_set(cmd: &Invocation) {
	let [name, min] = cmd.args() else {
		con_warn!("Expected name of console variable or command, and the minimum value limit to set");
		return
	};
	let min = unwrap_lim!(unwrap_lim!(min.as_c_str().to_str()).parse());
	with_cvars(move |mut cvars| {
		let reg = unwrap_var!(name => cvars.find_var_mut(name.as_c_str()));
		unsafe {
			let data = &mut reg.as_mut_inner().data;
			data.min_value = min;
			data.has_min = true;
		}
	});
}
#[con_command(complete = suggest_vars)]
fn cm_max_set(cmd: &Invocation) {
	let [name, max] = cmd.args() else {
		con_warn!("Expected name of console variable or command, and the maximum value limit to set");
		return
	};
	let max = unwrap_lim!(unwrap_lim!(max.as_c_str().to_str()).parse());
	with_cvars(move |mut cvars| {
		let reg = unwrap_var!(name => cvars.find_var_mut(name.as_c_str()));
		unsafe {
			let data = &mut reg.as_mut_inner().data;
			data.max_value = max;
			data.has_max = true;
		}
	});
}

#[con_command(complete = suggest_vars)]
fn cm_min_clear(cmd: &Invocation) {
	let [name] = cmd.args() else {
		con_warn!("Expected name of console variable or command to clear the minimum value limit of");
		return
	};
	with_cvars(move |mut cvars| {
		let reg = unwrap_var!(name => cvars.find_var_mut(name.as_c_str()));
		unsafe {
			let data = &mut reg.as_mut_inner().data;
			data.has_min = false;
		}
	});
}
#[con_command(complete = suggest_vars)]
fn cm_max_clear(cmd: &Invocation) {
	let [name] = cmd.args() else {
		con_warn!("Expected name of console variable or command to clear the maximum value limit of");
		return
	};
	with_cvars(move |mut cvars| {
		let reg = unwrap_var!(name => cvars.find_var_mut(name.as_c_str()));
		unsafe {
			let data = &mut reg.as_mut_inner().data;
			data.has_max = false;
		}
	});
}

fn suggest_flags(part: Partial<'_>, out: &mut Suggestions) {
	let (_, arg, before) = part.split_last();
	push_flag_suggestions(arg, before, out);
}

#[con_command(complete = suggest_flags)]
fn cm_find_by_flags(cmd: &Invocation) {
	let flags = unwrap_flags!(parse_flags(cmd.args()));
	with_cvars(move |cvars| {
		for reg in cvars.registered() {
			if reg.are_flags_set(flags) {
				con().msg_raw(reg.name());
				con().msg_raw('\n');
			}
		}
	});
}

#[con_command(complete = suggest_flags)]
fn cm_flags_remove_from_all(cmd: &Invocation) {
	let flags = unwrap_flags!(parse_flags(cmd.args()));
	with_cvars(move |mut cvars| {
		for reg in cvars.registered_mut() {
			let reg_flags = reg.flags_mut();
			*reg_flags = reg_flags.difference(flags);
		}
	});
}

impl Plugin for CvarModifier {
	type LoadError = Infallible;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}

struct CvarModifier;
export_plugin!(CvarModifier);
