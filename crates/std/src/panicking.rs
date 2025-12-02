use std::{
	backtrace::{
		Backtrace, BacktraceStatus,
	},
	panic::{
		PanicHookInfo, set_hook,
	},
	thread::current as current_thread,
};

use crate::io::con;

pub fn install_panic_hook() {
	set_hook(Box::new(on_panic))
}

fn on_panic(info: &PanicHookInfo<'_>) {
	let backtrace = Backtrace::capture();

	let (file, line, col) = {
		info.location().map(move |loc| (loc.file(), loc.line(), loc.column())).unwrap_or(("<somewhere>", 0, 0))
	};
	
	let payload = info.payload();

	let thread = current_thread();
	let name = thread.name().unwrap_or("<unnamed>");

	let con = con();
	{
		con.warn_raw("Rust thread '");
		con.warn_raw(name);
		con.warn_raw("' panicked at ");
		con.warn_raw(file);
		con.warn_raw(":");
		con.warn_raw(line);
		con.warn_raw(":");
		con.warn_raw(col);
		con.warn_raw(":\n");
	}
	if let Some(&s) = payload.downcast_ref::<&'static str>() {
		con.warn_raw(s);
		con.warn_raw("\n");
	} else if let Some(s) = payload.downcast_ref::<String>() {
		con.warn_raw(s.as_str());
		con.warn_raw("\n");
	}

	match backtrace.status() {
		BacktraceStatus::Captured => con.msg(format_args!("{backtrace}")),
		BacktraceStatus::Disabled => {
			con.warn_raw("note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n");
		}
		_ => {}
	}
}
