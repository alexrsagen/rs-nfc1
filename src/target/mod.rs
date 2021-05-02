use crate::{Error, Result, Modulation, ModulationType, BaudRate, wrap_err};
use nfc1_sys::str_nfc_target;
use nfc1_sys::nfc_free;
use std::convert::TryFrom;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::ptr;

pub mod info;

/// Safe version of nfc_target
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Target {
	pub target_info: info::TargetInfo,
	pub modulation: Modulation,
}

impl Target {
	pub fn new_iso14443a() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443a(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443a,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_felica() -> Self {
		Self{
			target_info: info::TargetInfo::new_felica(),
			modulation: Modulation{
				modulation_type: ModulationType::Felica,
				baud_rate: BaudRate::Baud212,
			},
		}
	}

	pub fn new_iso14443b() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443b(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443b,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_iso14443bi() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443bi(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443bi,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_iso14443b2sr() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443b2sr(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443b2sr,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_iso14443b2ct() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443b2ct(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443b2ct,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_jewel() -> Self {
		Self{
			target_info: info::TargetInfo::new_jewel(),
			modulation: Modulation{
				modulation_type: ModulationType::Jewel,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_dep() -> Self {
		Self{
			target_info: info::TargetInfo::new_dep(),
			modulation: Modulation{
				modulation_type: ModulationType::Dep,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_barcode() -> Self {
		Self{
			target_info: info::TargetInfo::new_barcode(),
			modulation: Modulation{
				modulation_type: ModulationType::Barcode,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn new_iso14443biclass() -> Self {
		Self{
			target_info: info::TargetInfo::new_iso14443biclass(),
			modulation: Modulation{
				modulation_type: ModulationType::Iso14443biClass,
				baud_rate: BaudRate::Baud106,
			},
		}
	}

	pub fn to_string(&self, verbose: bool) -> Result<String> {
		let target: nfc1_sys::nfc_target = self.into();
		let mut strinfo_ptr: *mut c_char = ptr::null_mut();
		wrap_err(unsafe { str_nfc_target(&mut strinfo_ptr, &target, verbose) })?;
		let strinfo = unsafe { CStr::from_ptr(strinfo_ptr) }.to_string_lossy().into_owned();
		unsafe { nfc_free(strinfo_ptr as *mut c_void); }
		Ok(strinfo)
	}
}

impl From<&Target> for nfc1_sys::nfc_target {
	fn from(input: &Target) -> nfc1_sys::nfc_target {
		nfc1_sys::nfc_target{
			nti: (&input.target_info).into(),
			nm: (&input.modulation).into(),
		}
	}
}

impl TryFrom<nfc1_sys::nfc_target> for Target {
	type Error = Error;
	fn try_from(input: nfc1_sys::nfc_target) -> Result<Target> {
		let modulation = Modulation::from(input.nm);
		match modulation.modulation_type {
			ModulationType::Iso14443a => Ok(Target{
				target_info: info::TargetInfo::from_iso14443a(input.nti),
				modulation,
			}),
			ModulationType::Jewel => Ok(Target{
				target_info: info::TargetInfo::from_jewel(input.nti),
				modulation,
			}),
			ModulationType::Iso14443b => Ok(Target{
				target_info: info::TargetInfo::from_iso14443b(input.nti),
				modulation,
			}),
			ModulationType::Iso14443bi => Ok(Target{
				target_info: info::TargetInfo::from_iso14443bi(input.nti),
				modulation,
			}),
			ModulationType::Iso14443b2sr => Ok(Target{
				target_info: info::TargetInfo::from_iso14443b2sr(input.nti),
				modulation,
			}),
			ModulationType::Iso14443b2ct => Ok(Target{
				target_info: info::TargetInfo::from_iso14443b2ct(input.nti),
				modulation,
			}),
			ModulationType::Felica => Ok(Target{
				target_info: info::TargetInfo::from_felica(input.nti),
				modulation,
			}),
			ModulationType::Dep => Ok(Target{
				target_info: info::TargetInfo::from_dep(input.nti),
				modulation,
			}),
			ModulationType::Barcode => Ok(Target{
				target_info: info::TargetInfo::from_barcode(input.nti),
				modulation,
			}),
			ModulationType::Iso14443biClass => Ok(Target{
				target_info: info::TargetInfo::from_iso14443biclass(input.nti),
				modulation,
			}),
			ModulationType::Undefined => Err(Error::UndefinedModulationType),
		}
	}
}

impl TryFrom<&Modulation> for Target {
	type Error = Error;
	fn try_from(input: &Modulation) -> Result<Self> {
		match input.modulation_type {
			ModulationType::Iso14443a => Ok(Self{
				target_info: info::TargetInfo::new_iso14443a(),
				modulation: input.clone(),
			}),
			ModulationType::Jewel => Ok(Self{
				target_info: info::TargetInfo::new_jewel(),
				modulation: input.clone(),
			}),
			ModulationType::Iso14443b => Ok(Self{
				target_info: info::TargetInfo::new_iso14443b(),
				modulation: input.clone(),
			}),
			ModulationType::Iso14443bi => Ok(Self{
				target_info: info::TargetInfo::new_iso14443bi(),
				modulation: input.clone(),
			}),
			ModulationType::Iso14443b2sr => Ok(Self{
				target_info: info::TargetInfo::new_iso14443b2sr(),
				modulation: input.clone(),
			}),
			ModulationType::Iso14443b2ct => Ok(Self{
				target_info: info::TargetInfo::new_iso14443b2ct(),
				modulation: input.clone(),
			}),
			ModulationType::Felica => Ok(Self{
				target_info: info::TargetInfo::new_felica(),
				modulation: input.clone(),
			}),
			ModulationType::Dep => Ok(Self{
				target_info: info::TargetInfo::new_dep(),
				modulation: input.clone(),
			}),
			ModulationType::Barcode => Ok(Self{
				target_info: info::TargetInfo::new_barcode(),
				modulation: input.clone(),
			}),
			ModulationType::Iso14443biClass => Ok(Self{
				target_info: info::TargetInfo::new_iso14443biclass(),
				modulation: input.clone(),
			}),
			ModulationType::Undefined => Err(Error::UndefinedModulationType),
		}
	}
}
