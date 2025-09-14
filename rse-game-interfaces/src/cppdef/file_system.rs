use ::core::{
	ffi::{
		CStr, c_char, c_int, c_long, c_uint, c_void,
	},
	ptr::null_mut,
};
use ::libc::{
	SEEK_SET, SEEK_CUR, SEEK_END,
};
use ::rse_cpp::{
	vtable, RefMut, RefConst, VtObjectMut,
};
use ::rse_game::cppdef::{
	app_system::AppSystemVt,
	utl::FileNameHandle,
};

pub type FileHandle = *mut c_void;
pub const FILESYSTEM_INVALID_HANDLE: FileHandle = null_mut();
pub type FileFindHandle = c_int;

// TODO: `CUtlBuffer`.
type UtlBuffer = c_void;

#[repr(C)]
pub struct FileSystemVt {
	pub app_system: AppSystemVt,
	pub base_file_system: BaseFileSystemVt,
	pub file_system: FileSystemVtBase,
}

vtable! {
	pub FileSystemVtBase for VtObjectMut<FileSystemVt> {
		pub fn is_steam() -> bool;
		pub fn mount_steam_content(extra_app_id: c_int) -> FileSystemMount;

		pub fn add_search_path(path: *const c_char, path_id: *const c_char, search_path_add: SearchPathAdd);
		pub fn remove_search_path(path: *const c_char, path_id: *const c_char) -> bool;
		pub fn remove_all_search_paths();
		pub fn remove_search_paths(path_id: *const c_char);
		pub fn mark_path_id_by_request_only(path: *const c_char, request_only: bool);
		pub fn relative_path_to_full_path(
			filename: *const c_char, path_id: *const c_char,
			out_dest: *mut c_char, max_len_in_chars: c_int,
			path_filter: PathTypeFilter, out_path_type: *mut PathTypeQuery,
		) -> *const c_char;
		pub fn get_search_path(
			path_id: *const c_char, get_pack_files: bool,
			out_dest: *mut c_char, max_len_in_chars: c_int,
		) -> c_int;
		pub fn add_pack_file(full_path: *const c_char, path_id: *const c_char) -> bool;
		pub fn remove_file(relative_path: *const c_char, path_id: *const c_char);
		pub fn rename_file(old_path: *const c_char, new_path: *const c_char, path_id: *const c_char) -> bool;
		pub fn create_dir_hierarchy(path: *const c_char, path_id: *const c_char);
		pub fn is_directory(filename: *const c_char, path_id: *const c_char) -> bool;
		pub fn file_time_to_string(out_strip: *mut c_char, max_chars_incl_terminator: c_int, file_time: c_long);
		pub fn set_buffer_size(file: FileHandle, bytes: c_uint);
		pub fn is_ok(file: FileHandle) -> bool;
		pub fn end_of_file(file: FileHandle) -> bool;
		pub fn read_line(output: *mut c_char, max_chars: c_int, file: FileHandle) -> *mut c_char;
		pub fn fprintf(file: FileHandle, format: *const c_char, ...) -> c_int;
		
		pub fn load_module(
			filename: *const c_char, path_id: *const c_char,
			validated_dll_only: bool,
		) -> *mut SysModule;
		pub fn unload_module(module: *mut SysModule);

		pub fn find_first(wildcard: *const c_char, out_handle: *mut FileFindHandle) -> *const c_char;
		pub fn find_next(handle: FileFindHandle) -> *const c_char;
		pub fn find_is_directory(handle: FileFindHandle) -> bool;
		pub fn find_close(handle: FileFindHandle);

		pub fn find_first_ex(
			wildcard: *const c_char, path_id: *const c_char,
			out_handle: *mut FileFindHandle,
		);

		pub fn get_local_path(filename: *const c_char, out_dest: *mut c_char, max_len_in_chars: c_int) -> *const c_char;
		pub fn full_path_to_relative_path(
			full_path: *const c_char,
			out_dest: *mut c_char, max_len_in_chars: c_int,
		) -> bool;

		pub fn get_current_directory(out_directory: *mut c_char, max_len: c_int) -> bool;

		pub fn find_or_add_filename(filename: *const c_char) -> FileNameHandle;
		pub fn string(handle: RefConst<FileNameHandle>, out_buf: *mut c_char, buf_len: c_int) -> bool;
		
		// TODO: `IFileSystem::AsyncReadMultiple` and others (79 functions).
	}
}

pub const FILESYSTEM_INTERFACE_VERSION: &CStr = c"VFileSystem022";

// TODO: `CSysModule`.
type SysModule = c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum FileSystemMount {
	Ok = 0,
	Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum SearchPathAdd {
	ToHead,
	ToTail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum PathTypeFilter {
	None = 0,
	CullPack = 1,
	CullNonPack = 2,
}

pub mod path_type_query {
	::rse_cpp::flag_consts! {
		for super::PathTypeQuery:
		/// Path is normal, not pack-based.
		IS_NORMAL = 0x00;
		/// Path is a pack file.
		IS_PACK_FILE = 0x01;
		/// Path is a map pack file.
		IS_MAP_PACK_FILE = 0x02;
		/// Path is the remote filesystem.
		IS_REMOTE = 0x04;
	}
}
pub type PathTypeQuery = u32;

vtable! {
	pub BaseFileSystemVt {
		pub fn read(output: *mut c_void, size: c_int, file: FileHandle) -> c_int;
		pub fn write(input: *const c_void, size: c_int, file: FileHandle) -> c_int;

		pub fn open(filename: *const c_char, options: *const c_char, path_id: *const c_char) -> FileHandle;
		pub fn close(file: FileHandle);

		pub fn seek(file: FileHandle, pos: c_int, seek_type: FileSystemSeek);
		pub fn tell(file: FileHandle) -> c_uint;
		pub fn size(file: FileHandle) -> c_uint;
		pub fn size_at(filename: *const c_char, path_id: *const c_char) -> c_uint;

		pub fn flush(file: FileHandle);
		pub fn precache(filename: *const c_char, path_id: *const c_char) -> bool;

		pub fn file_exists(filename: *const c_char, path_id: *const c_char) -> bool;
		pub fn is_file_writable(filename: *const c_char, path_id: *const c_char) -> bool;
		pub fn set_file_writable(filename: *const c_char,writable: bool, path_id: *const c_char) -> bool;

		pub fn get_file_time(filename: *const c_char, path_id: *const c_char) -> c_long;

		pub fn read_file(
			filename: *const c_char, path: *const c_char,
			buf: RefMut<UtlBuffer>, max_bytes: c_int, starting_byte: c_int, alloc_fn: FsAllocFunc,
		) -> bool;
		pub fn write_file(filename: *const c_char, path: *const c_char, buf: RefMut<UtlBuffer>);
		pub fn unzip_file(filename: *const c_char, path: *const c_char, destination: *const c_char);
	}
}

pub const BASEFILESYSTEM_INTERFACE_VERSION: &CStr = c"VBaseFileSystem011";

pub type FsAllocFunc = unsafe extern "C" fn(filename: *const c_char, bytes: c_uint) -> *mut c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum FileSystemSeek {
	Head = SEEK_SET as _,
	Current = SEEK_CUR as _,
	Tail = SEEK_END as _,
}
