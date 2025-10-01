use ::core::{
	ffi::{
		CStr, c_char, c_int,
	},
	marker::PhantomData,
};
use ::rse_cpp::{
	ptr_compat::{
		PointerFrom,
		convert_ref, convert_mut,
	},
	convert_vt_ref, convert_vt_mut,
	new_vtable_self, vtable_methods, this_to_self,
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
		ConCommandBaseVt, ConCommandVtBase,
		ConCommandBaseExt as CConCommandBaseExt,
		ConCommandBits,
		CompletionArray,
		CommandCallback, CompletionCallback,
		Command as CCommand,
		CvarDllIdentifier,
	},
	console_base::ConCommandBaseExt,
	Invocation,
};

use super::{
	RawCommand, Suggestions,
};

#[repr(C)]
pub struct ConCommandObject<'a, T> {
	con_command: ConCommand,
	pub inner: T,
	_strings: PhantomData<&'a CStr>,
}

impl<'a, T> ConCommandObject<'a, T> {
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

impl<'a, T> ConCommandObject<'a, T>
where
	T: RawCommand<'a>,
{
	pub const fn new(
		inner: T,
		name: &'a CStr, help: Option<&'a CStr>,
	) -> Self {
		Self {
			con_command: ConCommand::new(
				unsafe { VTablePtr::new_unchecked(Self::VTABLE as *const _ as *mut _) },
				ConCommandExt {
					base: CConCommandBaseExt {
						next: None,
						registered: false,
						name: name.as_ptr(),
						help_string: crate::util::c_str_ptr(help),
						// TODO: Flags.
						flags: 0,
					},
					command_callback: CommandCallback {
						v1: invalid_command_callback,
					},
					completion_callback: CompletionCallback {
						function: invalid_complete_callback,
					},
					bits: ConCommandBits::new(),
				},
			),
			inner,
			_strings: PhantomData,
		}
	}

	const VTABLE: &'static ConCommandVt = &ConCommandVt {
		base: new_vtable_self!(ConCommandBaseVt {
			destructor,
			#[cfg(not(windows))]
			destructor_2,
			is_command,
			is_flag_set,
			add_flags,
			get_name,
			get_help_text,
			is_registered,
			get_dll_identifier,
			create_base,
			init
		}),
		con_command: new_vtable_self!(ConCommandVtBase {
			auto_complete_suggest,
			can_auto_complete,
			dispatch
		}),
	};

	vtable_methods! {
		this: VtObjectPtr<ConCommandBaseVt>;
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
		fn is_flag_set(flag: c_int) -> bool {
			this_to_self!(ref this).as_base().is_flag_set(flag)
		}
		fn add_flags(flags: c_int) {
			this_to_self!(mut this).as_mut_base().add_flags(flags)
		}
		fn get_name() -> *const c_char {
			let this = this_to_self!(mut this);
			T::name(this);
			this.con_command.data.base.help_string
		}
		fn get_help_text() -> *const c_char {
			let this = this_to_self!(mut this);
			T::help(this);
			this.con_command.data.base.help_string
		}
		fn is_registered() -> bool {
			this_to_self!(ref this).as_base().is_registered()
		}
		fn get_dll_identifier() -> CvarDllIdentifier {
			T::dll_identifier(this_to_self!(mut this))
		}
		fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int) {
			let _ = this;
			let _ = name;
			let _ = help_string;
			let _ = flags;
			// Do nothing here. This method is purely for usage in the `ConCommandBase` constructor.
		}
		fn init() {
			T::init(this_to_self!(mut this))
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
			let suggestions = Suggestions::wrap(suggestions);
			T::auto_complete_suggest(this_to_self!(mut this), partial, suggestions).get()
		}
		fn can_auto_complete() -> bool {
			T::can_auto_complete(this_to_self!(mut this))
		}
		fn dispatch(command: RefConst<CCommand>) {
			let invocation = unsafe { Invocation::from_ptr(command.as_ptr()) };
			T::dispatch(this_to_self!(mut this), invocation)
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

unsafe extern "C" fn invalid_command_callback() {}
unsafe extern "C" fn invalid_complete_callback(
	partial: *const c_char,
	out_commands: *mut CompletionArray,
) -> c_int {
	let _ = partial;
	let _ = out_commands;
	0
}
