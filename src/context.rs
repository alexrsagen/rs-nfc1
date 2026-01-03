use crate::{Error, Result, Device};
use nfc1_sys::{nfc_connstring, nfc_context, nfc_context_free, nfc_context_new, nfc_init, nfc_list_devices};
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::sync::LazyLock;

// This allocates memory for the `nfc_drivers` linked list in `libnfc`.
//
// The function `nfc_exit()` would release this memory and set the
// list head to NULL.
//
// We intentionally "leak" this memory (never release it), because:
// - the amount of allocated memory is quite small
// - programs are likely to use `libnfc` throughout their entire lifetime
// - we can avoid adding new parameters to Context::new()
static NFC_DRIVERS: LazyLock<()> = LazyLock::new(|| {
	let mut p = ptr::null_mut();
	unsafe {
		nfc_init(&mut p);
		nfc_context_free(p);
	}
});

pub struct Context {
	pub(crate) ptr: *mut nfc_context,
	drivers: (),
}

unsafe impl Send for Context {}

impl Context {
	pub fn new() -> Result<Self> {
		let ptr = unsafe { nfc_context_new() };
		if ptr.is_null() {
			return Err(Error::Malloc);
		}
		let drivers = *NFC_DRIVERS;
		Ok(Self { ptr, drivers })
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
		unsafe { nfc_context_free(self.ptr); }
	}
}