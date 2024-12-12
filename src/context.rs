use crate::{Error, Result, Device};
use nfc1_sys::{nfc_context, nfc_connstring, nfc_init, nfc_exit, nfc_list_devices};
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub struct Context {
	pub(crate) ptr: *mut nfc_context,
}

unsafe impl Send for Context {}

impl Context {
	pub fn new() -> Result<Self> {
		match unsafe {
			let mut p: *mut nfc_context = ptr::null_mut();
			nfc_init(&mut p);
			p.as_mut()
		} {
			Some(ptr) => Ok(Self{ ptr }),
			None => Err(Error::Malloc)
		}
	}

	// NFC Device/Hardware manipulation

	pub fn open(&mut self) -> Result<Device> {
		Device::new(self)
	}

	pub fn open_with_connstring(&mut self, connstring: &str) -> Result<Device> {
		Device::new_with_connstring(self, connstring)
	}

	pub fn list_devices(&mut self, max: usize) -> Result<Vec<String>> {
		let sized_array: nfc_connstring = vec![0 as c_char; 1024].try_into().map_err(|_| Error::Malloc)?;
		let mut connstrings: Vec<nfc_connstring> = vec![sized_array; max];
		let count = unsafe{ nfc_list_devices(self.ptr, connstrings.as_mut_ptr(), connstrings.len()) } as usize;
		connstrings.resize(count, sized_array);
		Ok(connstrings.into_iter().map(|connstring| unsafe { CStr::from_ptr(connstring.as_ptr()) }.to_string_lossy().into_owned()).collect())
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe { nfc_exit(self.ptr); }
	}
}