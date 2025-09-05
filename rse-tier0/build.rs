#![allow(unexpected_cfgs)]

use ::std::env::var;

#[cfg(feature = "link-srv")]
macro_rules! link_name {
	($name:literal) => {
		concat!($name, "_srv")
	};
}
#[cfg(not(feature = "link-srv"))]
macro_rules! link_name {
	($name:literal) => {
		$name
	};
}

fn main() -> Result<(), String> {
	if cfg!(feature = "link-dll") {
		match var("VALVE_LIB_PATH") {
			Ok(path) => {
				println!("cargo:rustc-link-search={path}");
				println!("cargo:rustc-link-lib=dylib={}", link_name!("tier0"));
			}
			Err(..) if cfg!(any(rust_analyzer, feature = "link-optional")) => {}
			Err(error) => {
				return Err(format!("`VALVE_LIB_PATH` must be specified, where it contains `tier0` and the like ({error})"))
			}
		}
	}
	Ok(())
}
