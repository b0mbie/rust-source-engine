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
	if cfg!(all(not(any(rust_analyzer, doc)), feature = "link-dll")) {
		println!(
			"cargo:rustc-link-search={}",
			var("VALVE_LIB_PATH")
				.map_err(move |e| format!("`VALVE_LIB_PATH` must be specified, where it contains `tier0` and the like ({e})"))?
		);
		println!("cargo:rustc-link-lib=dylib={}", link_name!("tier0"));
	}
	Ok(())
}
