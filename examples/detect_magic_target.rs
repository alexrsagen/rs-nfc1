use std::thread::sleep;
use std::time::Duration;

const MAX_FRAME_LEN: usize = 264;
const UNLOCK_1: [u8; 1] = [0x40];
const UNLOCK_2: [u8; 1] = [0x40];

fn main() -> nfc1::Result<()> {
	let mut context = nfc1::Context::new()?;
	let mut device = context.open()?;
	print!("NFC reader: {} opened\n\n", device.name());
	device.initiator_init()?;

	// Configure the CRC
	device.set_property_bool(nfc1::Property::HandleCrc, false)?;
	// Use raw send/receive methods
	device.set_property_bool(nfc1::Property::EasyFraming, false)?;
	// Disable 14443-4 autoswitching
	device.set_property_bool(nfc1::Property::AutoIso144434, false)?;

	loop {
		println!("Looking for targets...\n");
		match device.initiator_select_passive_target(&nfc1::Modulation{
			modulation_type: nfc1::ModulationType::Iso14443a,
			baud_rate: nfc1::BaudRate::Baud106,
		}) {
			Ok(target) => {
				print!("Target found: {}", target.to_string(false)?);
				match device.initiator_transceive_bits(&UNLOCK_1, 7, MAX_FRAME_LEN) {
					Ok(rx) => print!("Received bits: {:02X?}\n", rx),
					Err(err) => {
						print!("This is NOT a backdoored rewritable UID chinese card ({:?})\n", err);
						sleep(Duration::from_secs(5));
						print!("Looking for targets...\n");
						continue;
					},
				};

				match device.initiator_transceive_bytes(&UNLOCK_2, MAX_FRAME_LEN, nfc1::Timeout::Default){
					Ok(rx) => {
						print!("Received bytes: {:02X?}\n", rx);
						print!("This is a backdoored rewritable UID chinese card\n")
					},
					Err(err) => print!("This is NOT a backdoored rewritable UID chinese card ({:?})\n", err),
				};
			},
			Err(_) => { continue; }
		}
		sleep(Duration::from_secs(5));
	}
}