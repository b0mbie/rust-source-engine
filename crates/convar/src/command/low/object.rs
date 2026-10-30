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
		convert_mut,
	},
	new_vtable_self, vtable_methods,
	This,
	this_to_self, this_to_pin_self,
	RefConst, RefMut,
	VTablePtr,
};
use ::rse_utl::{
	cppdef::{
		CUtlVector, CUtlString,
	},
	memory::tier0::Tier0Memory,
	Vector,
};

use crate::{
	cppdef::{
		ConCommand, ConCommandVt, ConCommandData,
		ConCommandBase as CConCommandBase, ConCommandBaseVt, ConCommandBaseData,
		ConCommandBits,
		Registrable, RegistrableVt, RegistrableData,
		CommandCallback, CompletionCallback,
		CCommand,
		CvarDllIdentifier,
	},
	console_base::{
		CvarFlags,
		RegistrableExt,
		RegistrableMut, registrable_mut,
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

	/// # Safety
	/// The returned [`ConCommand`]'s public fields must not be mutated
	/// such that they could cause Undefined Behavior when used in conjunction with `T`.
	pub const unsafe fn as_mut_inner(&mut self) -> &mut ConCommand {
		&mut self.con_command
	}

	pub const fn as_registrable(&self) -> &RegistrableExt {
		unsafe { RegistrableExt::from_ref(&self.con_command.data.inherited_con_command_base.inherited_registrable) }
	}

	pub const fn as_mut_registrable(&mut self) -> &mut RegistrableExt {
		unsafe { RegistrableExt::from_mut(&mut self.con_command.data.inherited_con_command_base.inherited_registrable) }
	}

	pub const fn to_registrable_mut(&mut self) -> RegistrableMut {
		registrable_mut::<CConCommandBase>(
			convert_mut(&mut self.con_command)
		)
	}

	pub const fn as_mut_base(&mut self) -> &mut CConCommandBase {
		convert_mut(&mut self.con_command)
	}
}

impl<'str, T> ConCommandObject<'str, T>
where
	T: RawCommand<'str>,
{
	/// # Safety
	/// `ext` must be properly initialized.
	pub const unsafe fn from_raw(inner: T, data: ConCommandData) -> Self {
		Self {
			con_command: ConCommand {
				vtable: VTablePtr::from_ref(Self::VTABLE),
				data,
			},
			inner,
			_strings: PhantomData,
		}
	}

	/// # Safety
	/// `name` must point to a valid C string that lives for `'str`.
	/// `help` must either be null, or point to a valid C string that lives for `'str`.
	/// 
	/// `command_callback`, `completion_callback` and `bits`
	/// must be properly initialized for `T`.
	pub const unsafe fn with_raw_strings(
		inner: T,
		name: *const c_char, help: *const c_char,
		flags: CvarFlags,
		command_callback: CommandCallback,
		completion_callback: CompletionCallback,
		bits: ConCommandBits,
	) -> Self {
		unsafe { Self::from_raw(
			inner,
			ConCommandData {
				inherited_con_command_base: ConCommandBaseData {
					inherited_registrable: RegistrableData {
						next: null_mut(),
						registered: false,
						name,
						help_string: help,
						flags: flags.bits(),
					},
				},
				command_callback,
				completion_callback,
				bits,
			},
		) }
	}

	/// # Safety
	/// `command_callback`, `completion_callback` and `bits`
	/// must be properly initialized for `T`.
	pub const unsafe fn new(
		inner: T,
		name: &'str CStr, help: Option<&'str CStr>,
		flags: CvarFlags,
		command_callback: CommandCallback,
		completion_callback: CompletionCallback,
		bits: ConCommandBits,
	) -> Self {
		unsafe { Self::with_raw_strings(
			inner,
			name.as_ptr(),
			::rse_cpp::c_str::opt_c_str_as_ptr(help),
			flags,
			command_callback, completion_callback,
			bits,
		) }
	}

	const VTABLE: &'static ConCommandVt = &ConCommandVt {
		inherited_con_command_base: ConCommandBaseVt {
			inherited_registrable: new_vtable_self!(RegistrableVt {
				destructors: rse_cpp::Destructors::new(Self::destructor),
				is_command
			}),
			is_flag_set: Self::is_flag_set,
			add_flags: Self::add_flags,
			get_name: Self::get_name,
			get_help_text: Self::get_help_text,
			is_registered: Self::is_registered,
			get_dll_identifier: Self::get_dll_identifier,
			create_base: Self::create_base,
			init: Self::init,
		},
		auto_complete_suggest: Self::auto_complete_suggest,
		can_auto_complete: Self::can_auto_complete,
		dispatch: Self::dispatch,
	};

	vtable_methods! {
		this: This<Registrable>;
		fn destructor() {
			let _ = this;
			// TODO: Destructor?
		}
		fn is_command() -> bool {
			let _ = this;
			true
		}
	}

	vtable_methods! {
		this: This<CConCommandBase>;
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
			unsafe { this_to_self!(ref this) }
				.con_command.data
				.inherited_con_command_base.inherited_registrable
				.name
		}
		fn get_help_text() -> *const c_char {
			let this_pinned = unsafe { this_to_pin_self!(mut this) };
			T::help(this_pinned);
			unsafe { this_to_self!(ref this) }
				.con_command.data
				.inherited_con_command_base.inherited_registrable
				.help_string
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
		this: This<ConCommand>;
		fn auto_complete_suggest(partial: *const c_char, commands: RefMut<CUtlVector<CUtlString>>) -> c_int {
			let partial = unsafe { CStr::from_ptr(partial) };
			let suggestions = unsafe {
				Vector::<CUtlString, Tier0Memory<CUtlString>>::from_mut_ptr(
					commands.cast::<CUtlVector<CUtlString, Tier0Memory<CUtlString>>>().as_ptr()
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
