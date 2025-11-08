use rse_std::prelude::*;
use rse_std::fs::File;
use std::borrow::Cow;

struct FileSystem;
impl Plugin for FileSystem {
	type LoadError = Cow<'static, str>;
	fn load(factories: PluginFactories) -> Result<Self, Self::LoadError> {
		let _ = factories;
	
		let size = rse_std::fs::size_of_file(c"gameinfo.txt", c"GAME");
		con_msg!("Size of `gameinfo.txt`: {size}B");

		let mut buffer = vec![0; size];
		let mut gameinfo = File::open(c"gameinfo.txt", c"rb", c"GAME")
			.ok_or("couldn't open `gameinfo.txt` for reading")?;
		let n_read = gameinfo.read(&mut buffer)
			.ok_or("couldn't read `gameinfo.txt` to the end")?;

		if n_read != size {
			return Err(Cow::Owned(format!("read {n_read} bytes from `gameinfo.txt` when expecting to read {size} bytes")))
		}

		if gameinfo.read(&mut [0]) != Some(0) {
			return Err(Cow::Borrowed("file handle for `gameinfo.txt` had unexpected leftover data"))
		}

		Ok(Self)
	}
	fn description(&mut self) -> &CStr {
		plugin_description!()
	}
}
export_plugin!(FileSystem);
