use nfc1_sys::{
	nfc_version,
	str_nfc_baud_rate,
	str_nfc_modulation_type,
	iso14443a_crc as nfc_iso14443a_crc,
	iso14443a_crc_append as nfc_iso14443a_crc_append,
	iso14443b_crc as nfc_iso14443b_crc,
	iso14443b_crc_append as nfc_iso14443b_crc_append,
};
use std::ffi::CStr;
use std::os::raw::c_int;
use std::time::Duration;
use std::string::ToString;
use std::io::{ErrorKind, Error as IoError};

mod target;
mod context;
mod device;

pub use target::Target;
pub use device::Device;
pub use context::Context;
pub use target::info as target_info;

/// Safe error type representing the NFC_E* constants
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
	// rs-nfc1 errors
	Malloc,
	Undefined(c_int),
	UndefinedModulationType,
	NoDeviceFound,

	// libnfc errors
	Io,
	InvalidArgument,
	DeviceNotSupported,
	NoSuchDeviceFound,
	BufferOverflow,
	Timeout,
	OperationAborted,
	NotImplemented,
	TargetReleased,
	RfTransmissionError,
	MifareAuthFailed,
	Soft,
	Chip,
}

impl From<c_int> for Error {
	fn from(input: c_int) -> Self {
		match input {
			nfc1_sys::NFC_EIO => Self::Io,
			nfc1_sys::NFC_EINVARG => Self::InvalidArgument,
			nfc1_sys::NFC_EDEVNOTSUPP => Self::DeviceNotSupported,
			nfc1_sys::NFC_ENOTSUCHDEV => Self::NoSuchDeviceFound,
			nfc1_sys::NFC_EOVFLOW => Self::BufferOverflow,
			nfc1_sys::NFC_ETIMEOUT => Self::Timeout,
			nfc1_sys::NFC_EOPABORTED => Self::OperationAborted,
			nfc1_sys::NFC_ENOTIMPL => Self::NotImplemented,
			nfc1_sys::NFC_ETGRELEASED => Self::TargetReleased,
			nfc1_sys::NFC_ERFTRANS => Self::RfTransmissionError,
			nfc1_sys::NFC_EMFCAUTHFAIL => Self::MifareAuthFailed,
			nfc1_sys::NFC_ESOFT => Self::Soft,
			nfc1_sys::NFC_ECHIP => Self::Chip,
			_ => Self::Undefined(input),
		}
	}
}

impl From<Error> for IoError {
	fn from(input: Error) -> Self {
		match input {
			// rs-nfc1 errors
			Error::Malloc => IoError::from(ErrorKind::Interrupted),
			Error::Undefined(_) => IoError::from(ErrorKind::Other),
			Error::UndefinedModulationType => IoError::from(ErrorKind::InvalidInput),
			Error::NoDeviceFound => IoError::from(ErrorKind::NotFound),

			// libnfc errors
			Error::Io => IoError::from(ErrorKind::Other),
			Error::InvalidArgument => IoError::from(ErrorKind::InvalidInput),
			Error::DeviceNotSupported => IoError::from(ErrorKind::Other),
			Error::NoSuchDeviceFound => IoError::from(ErrorKind::NotFound),
			Error::BufferOverflow => IoError::from(ErrorKind::Other),
			Error::Timeout => IoError::from(ErrorKind::TimedOut),
			Error::OperationAborted => IoError::from(ErrorKind::Interrupted),
			Error::NotImplemented => IoError::from(ErrorKind::Other),
			Error::TargetReleased => IoError::from(ErrorKind::Other),
			Error::RfTransmissionError => IoError::from(ErrorKind::Interrupted),
			Error::MifareAuthFailed => IoError::from(ErrorKind::PermissionDenied),
			Error::Soft => IoError::from(ErrorKind::Other),
			Error::Chip => IoError::from(ErrorKind::Other),
		}
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			// rs-nfc1 errors
			Error::Malloc => write!(f, "Memory allocation error"),
			Error::Undefined(errno) => write!(f, "Unknown libnfc error: {}", errno),
			Error::UndefinedModulationType => write!(f, "Undefined modulation type"),
			Error::NoDeviceFound => write!(f, "No device found"),

			// libnfc errors
			Error::Io => write!(f, "Input/output error, device may not be usable anymore without re-opening it"),
			Error::InvalidArgument => write!(f, "Invalid argument(s)"),
			Error::DeviceNotSupported => write!(f, "Operation not supported by device"),
			Error::NoSuchDeviceFound => write!(f, "No such device found"),
			Error::BufferOverflow => write!(f, "Buffer overflow"),
			Error::Timeout => write!(f, "Operation timed out"),
			Error::OperationAborted => write!(f, "Operation aborted (by user)"),
			Error::NotImplemented => write!(f, "Not (yet) implemented"),
			Error::TargetReleased => write!(f, "Target released"),
			Error::RfTransmissionError => write!(f, "Error during RF transmission"),
			Error::MifareAuthFailed => write!(f, "MIFARE Classic: authentication failed"),
			Error::Soft => write!(f, "Software error (allocation, file/pipe creation, etc.)"),
			Error::Chip => write!(f, "Device internal chip error"),
		}
	}
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn wrap_err(res: c_int) -> Result<()> {
	if res < 0 {
		return Err(res.into());
	}
	Ok(())
}

pub(crate) fn wrap_err_usize(res: c_int) -> Result<usize> {
	if res < 0 {
		return Err(res.into());
	}
	Ok(res as usize)
}

/// Safe version of the int values used for timeouts in libnfc
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Timeout {
	None,
	Default,
	Duration(Duration),
}

impl From<Timeout> for c_int {
	fn from(input: Timeout) -> Self {
		match input {
			Timeout::None => 0,
			Timeout::Default => -1,
			Timeout::Duration(duration) => duration.as_millis() as c_int,
		}
	}
}

impl From<c_int> for Timeout {
	fn from(input: c_int) -> Self {
		if input < 0 {
			Timeout::Default
		} else {
			match input {
				0 => Timeout::None,
				millis => Timeout::Duration(Duration::from_millis(millis as u64)),
			}
		}
	}
}

/// Safe version of nfc_mode
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
	Target,
	Initiator,
}

impl From<Mode> for nfc1_sys::nfc_mode {
	fn from(input: Mode) -> nfc1_sys::nfc_mode {
		match input {
			Mode::Target => nfc1_sys::nfc_mode_N_TARGET,
			Mode::Initiator => nfc1_sys::nfc_mode_N_INITIATOR,
		}
	}
}

/// Safe version of nfc_baud_rate
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaudRate {
	Baud106,
	Baud212,
	Baud424,
	Baud847,
	Undefined,
}

impl ToString for BaudRate {
	fn to_string(&self) -> String {
		let baud_rate: nfc1_sys::nfc_baud_rate = self.clone().into();
		return unsafe { CStr::from_ptr(str_nfc_baud_rate(baud_rate)) }.to_string_lossy().into_owned();
	}
}

impl From<BaudRate> for nfc1_sys::nfc_baud_rate {
	fn from(input: BaudRate) -> nfc1_sys::nfc_baud_rate {
		match input {
			BaudRate::Baud106 => nfc1_sys::nfc_baud_rate_NBR_106,
			BaudRate::Baud212 => nfc1_sys::nfc_baud_rate_NBR_212,
			BaudRate::Baud424 => nfc1_sys::nfc_baud_rate_NBR_424,
			BaudRate::Baud847 => nfc1_sys::nfc_baud_rate_NBR_847,
			BaudRate::Undefined => nfc1_sys::nfc_baud_rate_NBR_UNDEFINED,
		}
	}
}

impl From<nfc1_sys::nfc_baud_rate> for BaudRate {
	fn from(input: nfc1_sys::nfc_baud_rate) -> Self {
		match input {
			nfc1_sys::nfc_baud_rate_NBR_106 => BaudRate::Baud106,
			nfc1_sys::nfc_baud_rate_NBR_212 => BaudRate::Baud212,
			nfc1_sys::nfc_baud_rate_NBR_424 => BaudRate::Baud424,
			nfc1_sys::nfc_baud_rate_NBR_847 => BaudRate::Baud847,
			_ => BaudRate::Undefined,
		}
	}
}

/// Safe version of nfc_property
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Property {
	TimeoutCommand,
	TimeoutAtr,
	TimeoutCom,
	HandleCrc,
	HandleParity,
	ActivateField,
	ActivateCrypto1,
	InfiniteSelect,
	AcceptInvalidFrames,
	AcceptMultipleFrames,
	AutoIso144434,
	EasyFraming,
	ForceIso14443A,
	ForceIso14443B,
	ForceSpeed106,
}

impl From<Property> for nfc1_sys::nfc_property {
	fn from(input: Property) -> nfc1_sys::nfc_property {
		match input {
			Property::TimeoutCommand => nfc1_sys::nfc_property_NP_TIMEOUT_COMMAND,
			Property::TimeoutAtr => nfc1_sys::nfc_property_NP_TIMEOUT_ATR,
			Property::TimeoutCom => nfc1_sys::nfc_property_NP_TIMEOUT_COM,
			Property::HandleCrc => nfc1_sys::nfc_property_NP_HANDLE_CRC,
			Property::HandleParity => nfc1_sys::nfc_property_NP_HANDLE_PARITY,
			Property::ActivateField => nfc1_sys::nfc_property_NP_ACTIVATE_FIELD,
			Property::ActivateCrypto1 => nfc1_sys::nfc_property_NP_ACTIVATE_CRYPTO1,
			Property::InfiniteSelect => nfc1_sys::nfc_property_NP_INFINITE_SELECT,
			Property::AcceptInvalidFrames => nfc1_sys::nfc_property_NP_ACCEPT_INVALID_FRAMES,
			Property::AcceptMultipleFrames => nfc1_sys::nfc_property_NP_ACCEPT_MULTIPLE_FRAMES,
			Property::AutoIso144434 => nfc1_sys::nfc_property_NP_AUTO_ISO14443_4,
			Property::EasyFraming => nfc1_sys::nfc_property_NP_EASY_FRAMING,
			Property::ForceIso14443A => nfc1_sys::nfc_property_NP_FORCE_ISO14443_A,
			Property::ForceIso14443B => nfc1_sys::nfc_property_NP_FORCE_ISO14443_B,
			Property::ForceSpeed106 => nfc1_sys::nfc_property_NP_FORCE_SPEED_106,
		}
	}
}

/// Safe version of nfc_modulation_type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModulationType {
	Iso14443a,
	Jewel,
	Iso14443b,
	Iso14443bi,
	Iso14443b2sr,
	Iso14443b2ct,
	Felica,
	Dep,
	Barcode,
	Iso14443biClass,
	Undefined,
}

impl ToString for ModulationType {
	fn to_string(&self) -> String {
		let modulation_type: nfc1_sys::nfc_modulation_type = self.clone().into();
		return unsafe { CStr::from_ptr(str_nfc_modulation_type(modulation_type)) }.to_string_lossy().into_owned();
	}
}

impl From<ModulationType> for nfc1_sys::nfc_modulation_type {
	fn from(input: ModulationType) -> nfc1_sys::nfc_modulation_type {
		match input {
			ModulationType::Iso14443a => nfc1_sys::nfc_modulation_type_NMT_ISO14443A,
			ModulationType::Jewel => nfc1_sys::nfc_modulation_type_NMT_JEWEL,
			ModulationType::Iso14443b => nfc1_sys::nfc_modulation_type_NMT_ISO14443B,
			ModulationType::Iso14443bi => nfc1_sys::nfc_modulation_type_NMT_ISO14443BI,
			ModulationType::Iso14443b2sr => nfc1_sys::nfc_modulation_type_NMT_ISO14443B2SR,
			ModulationType::Iso14443b2ct => nfc1_sys::nfc_modulation_type_NMT_ISO14443B2CT,
			ModulationType::Felica => nfc1_sys::nfc_modulation_type_NMT_FELICA,
			ModulationType::Dep => nfc1_sys::nfc_modulation_type_NMT_DEP,
			ModulationType::Barcode => nfc1_sys::nfc_modulation_type_NMT_BARCODE,
			ModulationType::Iso14443biClass => nfc1_sys::nfc_modulation_type_NMT_ISO14443BICLASS,
			ModulationType::Undefined => 0,
		}
	}
}

impl From<nfc1_sys::nfc_modulation_type> for ModulationType {
	fn from(input: nfc1_sys::nfc_modulation_type) -> Self {
		match input {
			nfc1_sys::nfc_modulation_type_NMT_ISO14443A => ModulationType::Iso14443a,
			nfc1_sys::nfc_modulation_type_NMT_JEWEL => ModulationType::Jewel,
			nfc1_sys::nfc_modulation_type_NMT_ISO14443B => ModulationType::Iso14443b,
			nfc1_sys::nfc_modulation_type_NMT_ISO14443BI => ModulationType::Iso14443bi,
			nfc1_sys::nfc_modulation_type_NMT_ISO14443B2SR => ModulationType::Iso14443b2sr,
			nfc1_sys::nfc_modulation_type_NMT_ISO14443B2CT => ModulationType::Iso14443b2ct,
			nfc1_sys::nfc_modulation_type_NMT_FELICA => ModulationType::Felica,
			nfc1_sys::nfc_modulation_type_NMT_DEP => ModulationType::Dep,
			nfc1_sys::nfc_modulation_type_NMT_BARCODE => ModulationType::Barcode,
			nfc1_sys::nfc_modulation_type_NMT_ISO14443BICLASS => ModulationType::Iso14443biClass,
			_ => ModulationType::Undefined,
		}
	}
}

/// Safe version of nfc_modulation
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Modulation {
	pub modulation_type: ModulationType,
	pub baud_rate: BaudRate,
}

impl From<&Modulation> for nfc1_sys::nfc_modulation {
	fn from(input: &Modulation) -> nfc1_sys::nfc_modulation {
		nfc1_sys::nfc_modulation {
			nmt: input.modulation_type.into(),
			nbr: input.baud_rate.into(),
		}
	}
}

impl From<nfc1_sys::nfc_modulation> for Modulation {
	fn from(input: nfc1_sys::nfc_modulation) -> Self {
		Self{
			modulation_type: input.nmt.into(),
			baud_rate: input.nbr.into(),
		}
	}
}

/// Safe version of nfc_dep_mode
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DepMode {
	Undefined,
	Passive,
	Active,
}

impl From<DepMode> for nfc1_sys::nfc_baud_rate {
	fn from(input: DepMode) -> nfc1_sys::nfc_baud_rate {
		match input {
			DepMode::Undefined => nfc1_sys::nfc_dep_mode_NDM_UNDEFINED,
			DepMode::Passive => nfc1_sys::nfc_dep_mode_NDM_PASSIVE,
			DepMode::Active => nfc1_sys::nfc_dep_mode_NDM_ACTIVE,
		}
	}
}

impl From<nfc1_sys::nfc_baud_rate> for DepMode {
	fn from(input: nfc1_sys::nfc_baud_rate) -> DepMode {
		match input {
			nfc1_sys::nfc_dep_mode_NDM_PASSIVE => DepMode::Passive,
			nfc1_sys::nfc_dep_mode_NDM_ACTIVE => DepMode::Active,
			_ => DepMode::Undefined,
		}
	}
}

// Misc. functions

pub fn iso14443a_crc(data: &mut [u8]) -> Vec<u8> {
	let mut crc = vec![0u8; 2];
	unsafe { nfc_iso14443a_crc(data.as_mut_ptr(), data.len(), crc.as_mut_ptr()) };
	crc
}

pub fn iso14443a_crc_append(data: &mut Vec<u8>) {
	unsafe { nfc_iso14443a_crc_append(data.as_mut_ptr(), data.len()) }
}

pub fn iso14443b_crc(data: &mut [u8]) -> Vec<u8> {
	let mut crc = vec![0u8; 2];
	unsafe { nfc_iso14443b_crc(data.as_mut_ptr(), data.len(), crc.as_mut_ptr()) };
	crc
}

pub fn iso14443b_crc_append(data: &mut Vec<u8>) {
	unsafe { nfc_iso14443b_crc_append(data.as_mut_ptr(), data.len()) }
}

pub fn iso14443a_locate_historical_bytes(ats: &[u8]) -> Option<&[u8]> {
	if ats.len() != 0 {
		let mut offset = 1;
		if ats[0] & 0x10 != 0 { // TA
			offset += 1;
		}
		if ats[0] & 0x20 != 0 { // TB
			offset += 1;
		}
		if ats[0] & 0x40 != 0 { // TC
			offset += 1;
		}
		if ats.len() > offset {
			// *pszTk = (szAts - offset);
			return Some(&ats[offset..]);
		}
	}
	None
}

pub fn version() -> &'static str {
	// XXX: Safe because nfc_version returns a constant string (#define)
	unsafe { CStr::from_ptr(nfc_version()) }.to_str().unwrap()
}
