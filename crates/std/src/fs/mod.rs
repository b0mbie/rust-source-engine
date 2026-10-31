use ::core::ffi::CStr;
use ::rse_shared::interfaces::file_system::{
	OFileSystem, FileSystem,
};
use ::std::sync::OnceLock;

use crate::plugin::PluginFactories;

mod file;
pub use file::*;
mod find;
pub use find::*;

pub fn find(wildcard: &CStr) -> Option<Find> {
	Find::open(wildcard)
}

pub fn find_in(wildcard: &CStr, path_id: &CStr) -> Option<Find> {
	Find::open_in(wildcard, path_id)
}

pub fn size_of_file(path: &CStr, path_id: &CStr) -> usize {
	with_fs(move |fs| fs.as_base().size_at(path, path_id)) as _
}

static FS: OnceLock<OFileSystem> = OnceLock::new();

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

fn with_fs<F: FnOnce(&FileSystem) -> R, R>(f: F) -> R {
	#[cold]
	const fn not_init() -> ! {
		panic!("filesystem interface used without being initialized")
	}

	match FS.get() {
		Some(fs) => f(fs),
		None => not_init(),
	}
}
