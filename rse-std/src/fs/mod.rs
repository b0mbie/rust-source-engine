use ::core::ffi::CStr;
use ::rse_game_interfaces::{
	BaseFileSystem, BaseFileSystemImpl,
};
use ::std::sync::OnceLock;

use crate::plugin::PluginFactories;

mod file;
pub use file::*;

pub fn size_of_file(path: &CStr, path_id: &CStr) -> usize {
	with_fs(move |fs| fs.size_at(path, path_id)) as _
}

static FS: OnceLock<BaseFileSystem> = OnceLock::new();

pub(crate) fn attach(factories: PluginFactories) -> bool {
	match factories.create_interface() {
		Ok(iface) => {
			let _ = FS.set(iface);
			true
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
			false
		}
	}
}

fn with_fs<F: FnOnce(&BaseFileSystem) -> R, R>(f: F) -> R {
	#[cold]
	const fn not_init() -> ! {
		panic!("filesystem interface used without being initialized")
	}

	match FS.get() {
		Some(fs) => f(fs),
		None => not_init(),
	}
}
