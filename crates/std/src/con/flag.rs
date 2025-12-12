use ::core::str::from_utf8_unchecked;
use ::bitflags::iter::IterNames;
use ::phf::map::Entries;

use super::CvarFlags;

macro_rules! flags {
	{
		$($byte_str:literal <=> $flags:ident,)*
	} => {
		const STR_TO_FLAG: ::phf::Map<&[u8], super::CvarFlags> = ::phf::phf_map! {
			$($byte_str => super::CvarFlags::$flags,)*
		};

		$(const _: () = {
			if ::core::str::from_utf8($byte_str).is_err() {
				panic!("byte string name is not valid UTF-8")
			}
		};)*

		fn flag_name_to_str(name: &str) -> Option<&'static str> {
			Some(match name {
				$(stringify!($flags) => const {
					match ::core::str::from_utf8($byte_str) {
						Ok(t) => t,
						Err(..) => unreachable!(),
					}
				})*
				_ => return None,
			})
		}
	};
}

pub fn all_flags() -> Flags {
	Flags {
		entries: STR_TO_FLAG.entries(),
	}
}

#[repr(transparent)]
pub struct Flags {
	entries: Entries<'static, &'static [u8], CvarFlags>,
}

impl Iterator for Flags {
	type Item = (&'static str, CvarFlags);
	fn next(&mut self) -> Option<Self::Item> {
		let (name, flags) = self.entries.next()?;
		let name = unsafe { from_utf8_unchecked(name) };
		Some((name, *flags))
	}
}

pub fn str_to_flag(s: &[u8]) -> Option<CvarFlags> {
	STR_TO_FLAG.get(s).copied()
}

pub const fn strings_of(flags: &CvarFlags) -> Iter {
	Iter {
		names: flags.iter_names(),
	}
}

#[repr(transparent)]
pub struct Iter {
	names: IterNames<CvarFlags>,
}

impl Iterator for Iter {
	type Item = Option<&'static str>;
	fn next(&mut self) -> Option<Self::Item> {
		let (name, ..) = self.names.next()?;
		Some(flag_name_to_str(name))
	}
}

flags! {
	b"game" <=> GAMEDLL,
	b"client" <=> CLIENTDLL,
	b"archive" <=> ARCHIVE,
	b"notify" <=> NOTIFY,
	b"singleplayer" <=> SP_ONLY,
	b"notconnected" <=> NOT_CONNECTED,
	b"cheat" <=> CHEAT,
	b"replicated" <=> REPLICATED,
	b"server_can_execute" <=> SERVER_CAN_EXECUTE,
	b"clientcmd_can_execute" <=> CLIENTCMD_CAN_EXECUTE,

	b"dev" <=> DEVELOPMENT_ONLY,
	b"hidden" <=> HIDDEN,
	b"protected" <=> PROTECTED,
	b"userinfo" <=> USERINFO,
	b"printable_only" <=> PRINTABLE_ONLY,
	b"unlogged" <=> UNLOGGED,
	b"never_as_string" <=> NEVER_AS_STRING,
	b"demo" <=> DEMO,
	b"dont_record" <=> DONT_RECORD,
	b"reload_materials" <=> RELOAD_MATERIALS,
	b"reload_textures" <=> RELOAD_TEXTURES,
	b"material_system_thread" <=> MATERIAL_SYSTEM_THREAD,
	b"archive_xbox" <=> ARCHIVE_XBOX,
	b"accessible_from_threads" <=> ACCESSIBLE_FROM_THREADS,
	b"server_cannot_query" <=> SERVER_CANNOT_QUERY,
	b"exec_despite_default" <=> EXEC_DESPITE_DEFAULT,
	b"internal" <=> INTERNAL_USE,
	b"competitive" <=> ALLOWED_IN_COMPETITIVE,
}
