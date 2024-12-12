use crate::{
	Error,
	Result,
	Context,
	Target,
	target_info::Dep,
	DepMode,
	Timeout,
	Mode,
	Modulation,
	ModulationType,
	BaudRate,
	Property,
	wrap_err,
	wrap_err_usize,
};
#[cfg(feature = "driver_pn53x_usb")]
use nfc1_sys::{
	pn53x_transceive,
	pn53x_read_register,
	pn53x_write_register,
};
use nfc1_sys::{
	nfc_device,
	nfc_open,
	nfc_close,
	nfc_strerror,
	nfc_device_get_last_error,
	nfc_abort_command,
	nfc_idle,
	nfc_initiator_init,
	nfc_initiator_init_secure_element,
	nfc_initiator_select_passive_target,
	nfc_initiator_list_passive_targets,
	nfc_initiator_poll_target,
	nfc_initiator_select_dep_target,
	nfc_initiator_poll_dep_target,
	nfc_initiator_deselect_target,
	nfc_initiator_transceive_bytes,
	nfc_initiator_transceive_bits,
	nfc_initiator_transceive_bytes_timed,
	nfc_initiator_transceive_bits_timed,
	nfc_initiator_target_is_present,
	nfc_target_init,
	nfc_target_send_bytes,
	nfc_target_receive_bytes,
	nfc_target_send_bits,
	nfc_target_receive_bits,
	nfc_device_get_name,
	nfc_device_get_connstring,
	nfc_device_get_supported_modulation,
	nfc_device_get_supported_baud_rate,
	nfc_device_get_supported_baud_rate_target_mode,
	nfc_device_set_property_int,
	nfc_device_set_property_bool,
	nfc_device_get_information_about,
	nfc_free,
};
use std::time::Duration;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Device {
	ptr: *mut nfc_device,
}

unsafe impl Send for Device {}

impl Device {
	fn new_device(context: &mut Context, connstring: Option<&str>) -> Result<Self> {
		let mut connstring_ptr = ptr::null_mut();
		if let Some(connstring) = connstring {
			let mut connstring_bytes = Vec::from(connstring);
			connstring_bytes.resize(1023, 0);
			let connstring_cstring = unsafe { CString::from_vec_unchecked(connstring_bytes) };
			connstring_ptr = connstring_cstring.into_raw();
		}

		match unsafe { nfc_open(context.ptr, connstring_ptr).as_mut() } {
			Some(ptr) => Ok(Self{ ptr }),
			None => Err(Error::NoDeviceFound)
		}
	}

	pub fn new(context: &mut Context) -> Result<Self> {
		Self::new_device(context, None)
	}

	pub fn new_with_connstring(context: &mut Context, connstring: &str) -> Result<Self> {
		Self::new_device(context, Some(connstring))
	}

	// Error reporting

	pub fn get_last_error_string(&mut self) -> Option<String> {
		let errptr = unsafe { nfc_strerror(self.ptr) };
		if errptr == ptr::null() {
			return None;
		}
		Some(unsafe { CStr::from_ptr(errptr) }.to_string_lossy().into_owned())
	}

	pub fn get_last_error(&mut self) -> Option<Error> {
		let res = unsafe { nfc_device_get_last_error(self.ptr) };
		if res >= 0 {
			return None;
		}
		Some(res.into())
	}

	// NFC Device/Hardware manipulation

	#[cfg(feature = "driver_pn53x_usb")]
	pub fn pn53x_transceive(&mut self, tx: &[u8], mut rx_len: usize, timeout: Timeout) -> Result<Vec<u8>> {
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { pn53x_transceive(self.ptr, tx.as_ptr(), tx.len(), rx_buf.as_mut_ptr(), rx_len, timeout.into()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	#[cfg(feature = "driver_pn53x_usb")]
	pub fn pn53x_read_register(&mut self, register_address: u16) -> Result<u8> {
		let mut value = 0u8;
		wrap_err(unsafe { pn53x_read_register(self.ptr, register_address, &mut value) })?;
		Ok(value)
	}

	#[cfg(feature = "driver_pn53x_usb")]
	pub fn pn53x_write_register(&mut self, register_address: u16, symbol_mask: u8, value: u8) -> Result<()> {
		wrap_err(unsafe { pn53x_write_register(self.ptr, register_address, symbol_mask, value) })
	}

	pub fn abort_command(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_abort_command(self.ptr) })
	}

	pub fn idle(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_idle(self.ptr) })
	}

	// NFC initiator: act as "reader"

	pub fn initiator_init(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_initiator_init(self.ptr) })
	}

	pub fn initiator_init_secure_element(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_initiator_init_secure_element(self.ptr) })
	}

	pub fn initiator_select_passive_target_with_init_data(&mut self, modulation: &Modulation, init_data: &[u8]) -> Result<Target> {
		let mut target: nfc1_sys::nfc_target = (&Target::new_iso14443a()).into();
		wrap_err(unsafe { nfc_initiator_select_passive_target(self.ptr, modulation.into(), init_data.as_ptr(), init_data.len(), &mut target) })?;
		target.try_into()
	}

	pub fn initiator_select_passive_target(&mut self, modulation: &Modulation) -> Result<Target> {
		let mut target: nfc1_sys::nfc_target = (&Target::new_iso14443a()).into();
		wrap_err(unsafe { nfc_initiator_select_passive_target(self.ptr, modulation.into(), ptr::null(), 0, &mut target) })?;
		target.try_into()
	}

	pub fn initiator_list_passive_targets(&mut self, modulation: &Modulation, max_len: usize) -> Result<Vec<Target>> {
		let mut targets: Vec<nfc1_sys::nfc_target> = vec![(&Target::new_iso14443a()).into(); max_len];
		wrap_err(unsafe { nfc_initiator_list_passive_targets(self.ptr, modulation.into(), targets.as_mut_ptr(), targets.len()) })?;
		targets.into_iter().map(|target| target.try_into()).collect()
	}

	pub fn initiator_poll_target(&mut self, modulations: &[Modulation], max_polls: u8, poll_period: Duration) -> Result<Target> {
		let mut target: nfc1_sys::nfc_target = (&Target::new_iso14443a()).into();
		let modulations: Vec<nfc1_sys::nfc_modulation> = modulations.iter().map(|modulation| modulation.into()).collect();
		let period = (poll_period.as_millis() as f32 / 150.0).floor().min(255.0) as u8;
		wrap_err(unsafe { nfc_initiator_poll_target(self.ptr, modulations.as_ptr(), modulations.len(), max_polls, period, &mut target) })?;
		target.try_into()
	}

	pub fn initiator_select_dep_target(&mut self, dep_mode: DepMode, baud_rate: BaudRate, initiator: &Dep, timeout: Timeout) -> Result<Target> {
		let mut target: nfc1_sys::nfc_target = (&Target::new_dep()).into();
		let initiator: nfc1_sys::nfc_dep_info = initiator.into();
		wrap_err(unsafe { nfc_initiator_select_dep_target(self.ptr, dep_mode.into(), baud_rate.into(), &initiator, &mut target, timeout.into()) })?;
		target.try_into()
	}

	pub fn initiator_poll_dep_target(&mut self, dep_mode: DepMode, baud_rate: BaudRate, initiator: &Dep, timeout: Timeout) -> Result<Target> {
		let mut target: nfc1_sys::nfc_target = (&Target::new_dep()).into();
		let initiator: nfc1_sys::nfc_dep_info = initiator.into();
		wrap_err(unsafe { nfc_initiator_poll_dep_target(self.ptr, dep_mode.into(), baud_rate.into(), &initiator, &mut target, timeout.into()) })?;
		target.try_into()
	}

	pub fn initiator_deselect_target(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_initiator_deselect_target(self.ptr) })
	}

	pub fn initiator_transceive_bytes(&mut self, tx: &[u8], mut rx_len: usize, timeout: Timeout) -> Result<Vec<u8>> {
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bytes(self.ptr, tx.as_ptr(), tx.len(), rx_buf.as_mut_ptr(), rx_buf.len(), timeout.into()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	pub fn initiator_transceive_bytes_timed(&mut self, tx: &[u8], mut rx_len: usize) -> Result<(Vec<u8>, u32)> {
		let mut rx_buf = vec![0u8; rx_len];
		let mut cycles = 0u32;
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bytes_timed(self.ptr, tx.as_ptr(), tx.len(), rx_buf.as_mut_ptr(), rx_buf.len(), &mut cycles) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok((rx_buf, cycles))
	}

	pub fn initiator_transceive_bits(&mut self, tx: &[u8], tx_bits: usize, mut rx_len: usize) -> Result<Vec<u8>> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bits(self.ptr, tx.as_ptr(), tx_bits, ptr::null(), rx_buf.as_mut_ptr(), rx_buf.len(), ptr::null_mut()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	pub fn initiator_transceive_bits_with_parity(&mut self, tx: &[u8], tx_bits: usize, parity_tx: &[u8], mut rx_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		let mut rx_buf = vec![0u8; rx_len];
		let mut rx_parity_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bits(self.ptr, tx.as_ptr(), tx_bits, parity_tx.as_ptr(), rx_buf.as_mut_ptr(), rx_buf.len(), rx_parity_buf.as_mut_ptr()) })?;
		rx_buf.resize(rx_len, 0u8);
		rx_parity_buf.resize(rx_len, 0u8);
		Ok((rx_buf, rx_parity_buf))
	}

	pub fn initiator_transceive_bits_timed(&mut self, tx: &[u8], tx_bits: usize, mut rx_len: usize) -> Result<(Vec<u8>, u32)> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		let mut rx_buf = vec![0u8; rx_len];
		let mut cycles = 0u32;
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bits_timed(self.ptr, tx.as_ptr(), tx_bits, ptr::null(), rx_buf.as_mut_ptr(), rx_buf.len(), ptr::null_mut(), &mut cycles) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok((rx_buf, cycles))
	}

	pub fn initiator_transceive_bits_with_parity_timed(&mut self, tx: &[u8], tx_bits: usize, parity_tx: &[u8], mut rx_len: usize) -> Result<(Vec<u8>, Vec<u8>, u32)> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		let mut rx_buf = vec![0u8; rx_len];
		let mut rx_parity_buf = vec![0u8; rx_len];
		let mut cycles = 0u32;
		rx_len = wrap_err_usize(unsafe { nfc_initiator_transceive_bits_timed(self.ptr, tx.as_ptr(), tx_bits, parity_tx.as_ptr(), rx_buf.as_mut_ptr(), rx_buf.len(), rx_parity_buf.as_mut_ptr(), &mut cycles) })?;
		rx_buf.resize(rx_len, 0u8);
		rx_parity_buf.resize(rx_len, 0u8);
		Ok((rx_buf, rx_parity_buf, cycles))
	}

	pub fn initiator_target_is_present(&mut self, target: &Target) -> Result<()> {
		let target: nfc1_sys::nfc_target = target.into();
		wrap_err(unsafe { nfc_initiator_target_is_present(self.ptr, &target) })
	}

	pub fn initiator_target_is_present_any(&mut self) -> Result<()> {
		wrap_err(unsafe { nfc_initiator_target_is_present(self.ptr, ptr::null()) })
	}

	// NFC target: act as tag (i.e. MIFARE Classic) or NFC target device.

	pub fn target_init(&mut self, target: &Target, mut rx_len: usize, timeout: Timeout) -> Result<Vec<u8>> {
		let mut target: nfc1_sys::nfc_target = target.into();
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_target_init(self.ptr, &mut target, rx_buf.as_mut_ptr(), rx_buf.len(), timeout.into()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	pub fn target_send_bytes(&mut self, tx: &[u8], timeout: Timeout) -> Result<()> {
		wrap_err(unsafe { nfc_target_send_bytes(self.ptr, tx.as_ptr(), tx.len(), timeout.into()) })
	}

	pub fn target_receive_bytes(&mut self, mut rx_len: usize, timeout: Timeout) -> Result<Vec<u8>> {
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_target_receive_bytes(self.ptr, rx_buf.as_mut_ptr(), rx_buf.len(), timeout.into()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	pub fn target_send_bits(&mut self, tx: &[u8], tx_bits: usize) -> Result<()> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		wrap_err(unsafe { nfc_target_send_bits(self.ptr, tx.as_ptr(), tx_bits, ptr::null_mut()) })
	}

	pub fn target_send_bits_with_parity(&mut self, tx: &[u8], tx_bits: usize, parity_tx: &[u8]) -> Result<()> {
		if tx_bits > tx.len() * 8 {
			return Err(Error::BufferOverflow);
		}
		wrap_err(unsafe { nfc_target_send_bits(self.ptr, tx.as_ptr(), tx_bits, parity_tx.as_ptr()) })
	}

	pub fn target_receive_bits(&mut self, mut rx_len: usize) -> Result<Vec<u8>> {
		let mut rx_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_target_receive_bits(self.ptr, rx_buf.as_mut_ptr(), rx_buf.len(), ptr::null_mut()) })?;
		rx_buf.resize(rx_len, 0u8);
		Ok(rx_buf)
	}

	pub fn target_receive_bits_with_parity(&mut self, mut rx_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
		let mut rx_buf = vec![0u8; rx_len];
		let mut rx_parity_buf = vec![0u8; rx_len];
		rx_len = wrap_err_usize(unsafe { nfc_target_receive_bits(self.ptr, rx_buf.as_mut_ptr(), rx_buf.len(), rx_parity_buf.as_mut_ptr()) })?;
		rx_buf.resize(rx_len, 0u8);
		rx_parity_buf.resize(rx_len, 0u8);
		Ok((rx_buf, rx_parity_buf))
	}

	// Special data accessors

	pub fn name(&mut self) -> &'static str {
		// XXX: Safe because nfc_device_get_name returns a struct member
		// which is guaranteed to be initialized
		unsafe { CStr::from_ptr(nfc_device_get_name(self.ptr)) }.to_str().unwrap()
	}

	pub fn connstring(&mut self) -> &'static str {
		// XXX: Safe because nfc_device_get_connstring returns a struct member
		// which is guaranteed to be initialized
		unsafe { CStr::from_ptr(nfc_device_get_connstring(self.ptr)) }.to_str().unwrap()
	}

	pub fn get_supported_modulation(&mut self, mode: Mode) -> Result<Vec<ModulationType>> {
		let mut supported_mt = MaybeUninit::uninit();
		wrap_err(unsafe { nfc_device_get_supported_modulation(self.ptr, mode.into(), supported_mt.as_mut_ptr()) })?;
		unsafe {
			// XXX: This should be safe, as nfc_device_get_supported_modulation should
			// return a non-zero error code if supported_mt is not set
			let supported_mt_init = supported_mt.assume_init();
			let mut supported_mt_vec = vec![];
			let mut i = 0;
			loop {
				let val = *supported_mt_init.add(i);
				if val == 0 { break; }
				supported_mt_vec.push(val.into());
				i += 1;
			}
			Ok(supported_mt_vec)
		}
	}

	pub fn get_supported_baud_rate(&mut self, mode: Mode, modulation_type: ModulationType) -> Result<Vec<BaudRate>> {
		let mut supported_br = MaybeUninit::uninit();
		match mode {
			Mode::Initiator => wrap_err(unsafe { nfc_device_get_supported_baud_rate(self.ptr, modulation_type.into(), supported_br.as_mut_ptr()) })?,
			Mode::Target => wrap_err(unsafe { nfc_device_get_supported_baud_rate_target_mode(self.ptr, modulation_type.into(), supported_br.as_mut_ptr()) })?,
		}
		unsafe {
			// XXX: This should be safe, as nfc_device_get_supported_baud_rate should
			// return a non-zero error code if supported_br is not set
			let supported_br_init = supported_br.assume_init();
			let mut supported_br_vec = vec![];
			let mut i = 0;
			loop {
				let val = *supported_br_init.add(i);
				if val == 0 { break; }
				supported_br_vec.push(val.into());
				i += 1;
			}
			Ok(supported_br_vec)
		}
	}

	// Properties accessors

	pub fn set_property_int(&mut self, property: Property, value: i32) -> Result<()> {
		wrap_err(unsafe { nfc_device_set_property_int(self.ptr, property.into(), value as c_int) })
	}

	pub fn set_property_bool(&mut self, property: Property, value: bool) -> Result<()> {
		wrap_err(unsafe { nfc_device_set_property_bool(self.ptr, property.into(), value) })
	}

	// Misc. functions

	pub fn get_information_about(&mut self) -> Result<String> {
		let mut strinfo_ptr: *mut c_char = ptr::null_mut();
		wrap_err(unsafe { nfc_device_get_information_about(self.ptr, &mut strinfo_ptr) })?;
		let strinfo = unsafe { CStr::from_ptr(strinfo_ptr) }.to_string_lossy().into_owned();
		unsafe { nfc_free(strinfo_ptr as *mut c_void); }
		Ok(strinfo)
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe { nfc_close(self.ptr); }
	}
}