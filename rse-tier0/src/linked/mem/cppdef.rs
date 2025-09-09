use ::core::ffi::{
	c_char, c_int, c_long, c_uint, c_void,
};
use ::rse_cpp::{
	RefMut, VtObjectMut, vtable,
};

unsafe extern "C" {
	pub static mut g_pMemAlloc: VtObjectMut<MemAllocVt>;
}

vtable! {
	pub MemAllocVt {
		pub fn alloc(size: usize) -> *mut c_void;
		pub fn realloc(mem: *mut c_void, new_size: usize) -> *mut c_void;
		pub fn free(mem: *mut c_void);
		pub fn expand_no_longer_supported(mem: *mut c_void, size: usize) -> *mut c_void;

		pub fn debug_alloc(
			size: usize,
			filename: *const c_char, line: c_int,
		) -> *mut c_void;
		pub fn debug_realloc(
			mem: *mut c_void, new_size: usize,
			filename: *const c_char, line: c_int,
		) -> *mut c_void;
		pub fn debug_free(
			mem: *mut c_void,
			filename: *const c_char, line: c_int,
		);
		pub fn debug_expand_no_longer_supported(
			mem: *mut c_void, size: usize,
			filename: *const c_char, line: c_int,
		) -> *mut c_void;

		pub fn get_size(mem: *mut c_void) -> usize;

		pub fn push_alloc_dbg_info(filename: *const c_char, line: c_int);
		pub fn pop_alloc_dbg_info();

		pub fn crt_set_break_alloc(new_break_alloc: c_long) -> c_long;
		pub fn crt_set_report_mode(report_type: c_int, report_mode: c_int) -> c_int;
		pub fn crt_is_valid_heap_pointer(mem: *const c_void) -> c_int;
		pub fn crt_is_valid_pointer(mem: *const c_void) -> c_int;
		pub fn crt_check_memory() -> c_int;
		pub fn crt_set_dbg_flag(flag: c_int) -> c_int;
		pub fn crt_mem_checkpoint(state: *mut CrtMemState) -> c_int;

		pub fn dump_stats();
		pub fn dump_stats_file_base(file_base: *const c_char);

		pub fn crt_set_report_file(rpt_type: c_int, file: *mut c_void) -> c_void;
		pub fn crt_set_report_hook(new_hook: *mut c_void);
		pub fn crt_dbg_report(
			rpt_type: c_int, file: *const c_char, line: c_int, module: *const c_char, msg: *const c_char,
		) -> c_int;
		
		pub fn heapchk() -> c_int;
		pub fn is_debug_heap () -> bool;

		pub fn get_actual_dbg_info(filename: RefMut<*const c_char>, line: RefMut<c_int>);
		pub fn register_allocation(
			filename: *const c_char, line: c_int,
			logical_size: c_int, actual_size: c_int,
			time: c_uint,
		);
		pub fn register_deallocation(
			filename: *const c_char, line: c_int,
			logical_size: c_int, actual_size: c_int,
			time: c_uint,
		);

		pub fn get_version() -> c_int;

		pub fn compact_heap();

		pub fn set_alloc_fail_handler(handler: MemAllocFailHandler) -> MemAllocFailHandler;

		pub fn dump_block_stats(mem: *mut c_void);

		#[cfg(feature = "memtest")]
		pub fn set_stats_extra_info(map_name: *const c_char, comment: *const c_char);

		// "I'm sure this is completely thread safe!" Brian Deen 7/19/2012.
		pub fn memory_alloc_failed() -> usize;

		pub fn get_debug_info_size() -> u32;
		pub fn save_debug_info(debug_info: *mut c_void);
		pub fn restore_debug_info(debug_info: *const c_void);
		pub fn init_debug_info(debug_info: *mut c_void, root_filename: *const c_char, line: c_int);

		pub fn global_memory_status(used_memory: *mut usize, free_memory: *mut usize);
	}
}

pub type MemAllocFailHandler = unsafe extern "C-unwind" fn(usize) -> usize;

// Windows thing.
pub type CrtMemState = c_void;
