use ::core::ffi::{
	CStr, c_int,
};
use ::rse_game_interfaces::{
	cppdef::{
		FileSystemSeek, ValidFileHandle,
	},
	BaseFileSystemImpl,
};
use ::std::io::{
	Seek as StdSeek, SeekFrom as StdSeekFrom,
	Result as IoResult, Error as IoError,
	Read, Write,
};

use super::with_fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Seek {
	Head(u64),
	Cur(i64),
	Tail(i64),
}

#[repr(transparent)]
pub struct File {
	raw: ValidFileHandle,
}

impl File {
	/// # Safety
	/// `handle` must've come from the global `IFileSystem` interface.
	pub const unsafe fn from_raw(raw: ValidFileHandle) -> Self {
		Self {
			raw,
		}
	}

	pub fn open(path: &CStr, options: &CStr, path_id: &CStr) -> Option<Self> {
		with_fs(move |fs| unsafe {
			let raw = fs.open(path, options, path_id)?;
			Some(Self::from_raw(raw))
		})
	}

	pub fn seek(&mut self, seek: Seek) {
		with_fs(move |fs| unsafe {
			let (pos, method) = match seek {
				Seek::Head(pos) => (pos as _, FileSystemSeek::Head),
				Seek::Cur(pos) => (pos as _, FileSystemSeek::Current),
				Seek::Tail(pos) => (pos as _, FileSystemSeek::Tail),
			};
			fs.seek(self.raw, pos, method)
		})
	}

	pub fn tell(&self) -> u64 {
		with_fs(move |fs| unsafe {
			fs.tell(self.raw) as _
		})
	}

	pub fn size(&self) -> usize {
		with_fs(move |fs| unsafe {
			fs.size(self.raw) as _
		})
	}

	pub fn flush(&mut self) {
		with_fs(move |fs| unsafe {
			fs.flush(self.raw)
		})
	}

	pub fn read(&mut self, buffer: &mut [u8]) -> Option<usize> {
		with_fs(move |fs| unsafe {
			fs_result_to_n(fs.read(self.raw, buffer))
		})
	}

	pub fn write(&mut self, data: &[u8]) -> Option<usize> {
		with_fs(move |fs| unsafe {
			fs_result_to_n(fs.write(self.raw, data))
		})
	}
}

const fn fs_result_to_n(result: c_int) -> Option<usize> {
	if result >= 0 {
		Some(result as _)
	} else {
		None
	}
}

impl Drop for File {
	fn drop(&mut self) {
		with_fs(move |fs| unsafe { fs.close(self.raw) })
	}
}

impl StdSeek for File {
	fn seek(&mut self, pos: StdSeekFrom) -> IoResult<u64> {
		self.seek(match pos {
			StdSeekFrom::Start(offset) => Seek::Head(offset),
			StdSeekFrom::Current(offset) => Seek::Cur(offset),
			StdSeekFrom::End(offset) => Seek::Tail(offset),
		});
		Ok(self.tell())
	}

	fn seek_relative(&mut self, offset: i64) -> IoResult<()> {
		self.seek(Seek::Cur(offset));
		Ok(())
	}
	
	fn rewind(&mut self) -> IoResult<()> {
		self.seek(Seek::Head(0));
		Ok(())
	}

	fn stream_position(&mut self) -> IoResult<u64> {
		Ok(self.tell())
	}
}

impl Read for File {
	fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
		File::read(self, buf).ok_or_else(move || IoError::other("failed to read into buffer"))
	}
}

impl Write for File {
	fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
		File::write(self, buf).ok_or_else(move || IoError::other("failed to write data"))
	}

	fn flush(&mut self) -> IoResult<()> {
		File::flush(self);
		Ok(())
	}
}
