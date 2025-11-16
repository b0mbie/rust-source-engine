use ::core::{
	ffi::{
		CStr, c_char, c_int,
	},
	marker::PhantomData,
	ptr::null_mut,
};
use ::rse_cpp::{
	ptr_compat::{
		PointerFrom,
		convert_ref, convert_mut, convert_mut_ptr,
	},
	convert_vt_ref, convert_vt_mut,
	new_vtable_self, vtable_methods,
	this_to_self, this_to_pin_self,
	VtObjectPtr,
	RefConst, RefMut,
	AsObject, VtObject,
	VTablePtr,
};
use ::rse_utl::{
	cppdef::{
		UtlVector, UtlString,
	},
	memory::tier0::Tier0Memory,
	Vector,
};

use crate::{
	cppdef::{
		ConCommand, ConCommandVt, ConCommandExt,
		ConCommandBase as CConCommandBase,
		ConCommandBaseVt, ConCommandBaseVtBase, ConCommandBaseVtExt,
		ConCommandVtBase,
		ConCommandBaseExt as CConCommandBaseExt,
		ConCommandBits,
		CommandCallback, CompletionCallback,
		Command as CCommand,
		CvarDllIdentifier,
	},
	console_base::{
		CvarFlags,
		ConCommandBaseExt,
		RegistrableMut,
	},
};

use super::{
	RawCommand, Suggestions, Invocation,
};

const _: () = {
	const fn assert_unpin<T: Unpin>() {}
	assert_unpin::<ConCommandObject<'_, ()>>()
};

#[repr(C)]
pub struct ConCommandObject<'str, T> {
	con_command: ConCommand,
	pub inner: T,
	_strings: PhantomData<&'str CStr>,
}

impl<'str, T> ConCommandObject<'str, T> {
	pub const fn as_inner(&self) -> &ConCommand {
		&self.con_command
	}

	pub const unsafe fn as_mut_inner(&mut self) -> &mut ConCommand {
		&mut self.con_command
	}

	pub const fn as_base(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.con_command.data.base) }
	}

	pub const fn as_mut_base(&mut self) -> &mut ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_mut(&mut self.con_command.data.base) }
	}

	pub const fn as_base_object(&mut self) -> &mut VtObject<ConCommandBaseVt> {
		convert_vt_mut(
			convert_mut::<_, VtObject<ConCommandVt>>(&mut self.con_command)
		)
	}
}

impl<'str, T> ConCommandObject<'str, T>
where
	T: RawCommand<'str>,
{
	/// # Safety
	/// `ext` must be properly initialized.
	pub const unsafe fn from_raw(inner: T, ext: ConCommandExt) -> Self {
		Self {
			con_command: ConCommand::new(
				VTablePtr::from_ref(Self::VTABLE),
				ext,
			),
			inner,
			_strings: PhantomData,
		}
	}

	pub const fn new(
		inner: T,
		name: &'str CStr, help: Option<&'str CStr>,
		flags: CvarFlags,
	) -> Self {
		unsafe { Self::from_raw(
			inner,
			ConCommandExt {
				base: CConCommandBaseExt {
					next: null_mut(),
					registered: false,
					name: name.as_ptr(),
					help_string: ::rse_cpp::c_str::opt_c_str_as_ptr(help),
					flags: flags.bits(),
				},
				command_callback: CommandCallback {
					v1: empty_command_callback_v1,
				},
				completion_callback: CompletionCallback { not_used: () },
				bits: ConCommandBits::new(),
			},
		) }
	}

	pub const fn as_registrable(&mut self) -> RegistrableMut {
		convert_mut_ptr(&mut self.con_command)
	}

	const VTABLE: &'static ConCommandVt = &ConCommandVt {
		base: ConCommandBaseVt {
			base: new_vtable_self!(ConCommandBaseVtBase {
				destructor,
				#[cfg(not(windows))]
				destructor_2,
				is_command
			}),
			ext: new_vtable_self!(ConCommandBaseVtExt {
				is_flag_set,
				add_flags,
				get_name,
				get_help_text,
				is_registered,
				get_dll_identifier,
				create_base,
				init
			}),
		},
		con_command: new_vtable_self!(ConCommandVtBase {
			auto_complete_suggest,
			can_auto_complete,
			dispatch
		}),
	};

	vtable_methods! {
		this: VtObjectPtr<ConCommandBaseVtBase>;
		fn destructor() {
			let _ = this;
			// TODO: Destructor?
		}
		#[cfg(not(windows))]
		fn destructor_2() {
			let _ = this;
			// TODO: Destructor?
		}
		fn is_command() -> bool {
			let _ = this;
			true
		}
	}

	vtable_methods! {
		this: VtObjectPtr<ConCommandBaseVt>;
		fn is_flag_set(flag: c_int) -> bool {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::are_flags_set(this, CvarFlags::from_bits_retain(flag))
		}
		fn add_flags(flags: c_int) {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::add_flags(this, CvarFlags::from_bits_retain(flags))
		}
		fn get_name() -> *const c_char {
			let this_pinned = unsafe { this_to_pin_self!(mut this) };
			T::name(this_pinned);
			this_to_self!(ref this).con_command.data.base.name
		}
		fn get_help_text() -> *const c_char {
			let this_pinned = unsafe { this_to_pin_self!(mut this) };
			T::help(this_pinned);
			this_to_self!(ref this).con_command.data.base.help_string
		}
		fn is_registered() -> bool {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::is_registered(this)
		}
		fn get_dll_identifier() -> CvarDllIdentifier {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::dll_identifier(this)
		}
		fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int) {
			let _ = this;
			let _ = name;
			let _ = help_string;
			let _ = flags;
			// Do nothing here. This method is purely for usage in the `ConCommandBase` constructor.
		}
		fn init() {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::init(this)
		}
	}

	vtable_methods! {
		this: VtObjectPtr<ConCommandVt>;
		fn auto_complete_suggest(partial: *const c_char, commands: RefMut<UtlVector<UtlString>>) -> c_int {
			let partial = unsafe { CStr::from_ptr(partial) };
			let suggestions = unsafe {
				Vector::<UtlString, Tier0Memory<UtlString>>::from_mut_ptr(
					commands.cast::<UtlVector<UtlString, Tier0Memory<UtlString>>>().as_ptr()
				)
			};
			let suggestions = unsafe { Suggestions::from_mut(suggestions) };
			let this = unsafe { this_to_pin_self!(mut this) };
			T::auto_complete_suggest(this, partial, suggestions).get()
		}
		fn can_auto_complete() -> bool {
			let this = unsafe { this_to_pin_self!(mut this) };
			T::can_auto_complete(this)
		}
		fn dispatch(command: RefConst<CCommand>) {
			let invocation = unsafe { Invocation::from_ptr(command.as_ptr()) };
			let this = unsafe { this_to_pin_self!(mut this) };
			T::dispatch(this, invocation)
		}
	}
}

unsafe impl<T> PointerFrom<ConCommandObject<'_, T>> for ConCommand {}
unsafe impl<T> PointerFrom<ConCommandObject<'_, T>> for CConCommandBase {}

impl<T> AsObject<ConCommandVt> for ConCommandObject<'_, T> {
	fn as_object(&self) -> &VtObject<ConCommandVt> {
		convert_ref(&self.con_command)
	}
}

impl<T> AsObject<ConCommandBaseVt> for ConCommandObject<'_, T> {
	fn as_object(&self) -> &VtObject<ConCommandBaseVt> {
		convert_vt_ref(
			convert_ref::<_, VtObject<ConCommandVt>>(&self.con_command)
		)
	}
}

unsafe extern "C" fn empty_command_callback_v1() {}
