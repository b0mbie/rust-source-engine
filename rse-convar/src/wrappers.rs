use ::core::ops::{
	Deref, DerefMut,
};
use ::rse_cpp::transparent_wrapper;

use crate::{
	cppdef::{
		ConCommandBase as CConCommandBase,
		ConVar as CConVar, ConCommand as CConCommand,
	},
	console_base::ConCommandBaseExt,
	variable::ConVarExt,
};

transparent_wrapper! {
	pub struct ConCommandBase for CConCommandBase as "ConCommandBase";
}

impl ConCommandBase {
	pub const fn ext(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.0.data) }
	}

	pub const fn ext_mut(&mut self) -> &mut ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_mut(&mut self.0.data) }
	}

	pub fn is_command(&self) -> bool {
		let object = self.0.as_object();
		let this = object.as_ptr().cast();
		unsafe { (object.vtable().base.is_command)(this) }
	}

	pub fn kind(&self) -> Kind<'_> {
		if self.is_command() {
			unsafe { Kind::Cmd(ConCommand::from_ptr(&self.0 as *const _ as *const _)) }
		} else {
			unsafe { Kind::Var(ConVar::from_ptr(&self.0 as *const _ as *const _)) }
		}
	}

	pub fn kind_mut(&mut self) -> KindMut<'_> {
		if self.is_command() {
			unsafe { KindMut::Cmd(ConCommand::from_mut_ptr(&mut self.0 as *mut _ as *mut _)) }
		} else {
			unsafe { KindMut::Var(ConVar::from_mut_ptr(&mut self.0 as *mut _ as *mut _)) }
		}
	}

	pub fn to_cmd(&self) -> Option<&ConCommand> {
		match self.kind() {
			Kind::Cmd(cmd) => Some(cmd),
			_ => None,
		}
	}

	pub fn to_cmd_mut(&mut self) -> Option<&mut ConCommand> {
		match self.kind_mut() {
			KindMut::Cmd(cmd) => Some(cmd),
			_ => None,
		}
	}

	pub fn to_var(&self) -> Option<&ConVar> {
		match self.kind() {
			Kind::Var(var) => Some(var),
			_ => None,
		}
	}

	pub fn to_var_mut(&mut self) -> Option<&mut ConVar> {
		match self.kind_mut() {
			KindMut::Var(var) => Some(var),
			_ => None,
		}
	}
}

impl Deref for ConCommandBase {
	type Target = ConCommandBaseExt;
	fn deref(&self) -> &Self::Target {
		self.ext()
	}
}
impl DerefMut for ConCommandBase {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ext_mut()
	}
}

mod kind {
	#[derive(Clone, Copy)]
	#[doc(hidden)]
	pub enum KindOf<ConVar, ConCommand> {
		Var(ConVar),
		Cmd(ConCommand),
	}
}

pub type Kind<'a> = kind::KindOf<&'a ConVar, &'a ConCommand>;
pub type KindMut<'a> = kind::KindOf<&'a mut ConVar, &'a mut ConCommand>;

transparent_wrapper! {
	pub struct ConVar for CConVar as "ConVar";
}

impl ConVar {
	pub const fn ext(&self) -> &ConVarExt {
		unsafe { ConVarExt::from_ref(&self.0.data) }
	}

	pub const fn ext_mut(&mut self) -> &mut ConVarExt {
		unsafe { ConVarExt::from_mut(&mut self.0.data) }
	}
}

impl Deref for ConVar {
	type Target = ConVarExt;
	fn deref(&self) -> &Self::Target {
		self.ext()
	}
}
impl DerefMut for ConVar {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ext_mut()
	}
}

transparent_wrapper! {
	pub struct ConCommand for CConCommand as "ConCommand";
}

impl ConCommand {
	pub const fn base(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.0.data.base) }
	}

	pub const fn base_mut(&mut self) -> &mut ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_mut(&mut self.0.data.base) }
	}
}

impl Deref for ConCommand {
	type Target = ConCommandBaseExt;
	fn deref(&self) -> &Self::Target {
		self.base()
	}
}
impl DerefMut for ConCommand {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.base_mut()
	}
}
