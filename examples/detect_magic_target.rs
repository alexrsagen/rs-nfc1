extern crate nfc1 as nfc;
use std::thread::sleep;
use std::time::Duration;

const MAX_FRAME_LEN: usize = 264;
const UNLOCK_1: [u8; 1] = [0x40];
const UNLOCK_2: [u8; 1] = [0x40];

fn main() -> nfc::Result<()> {
    let mut context = nfc::Context::new()?;
    let mut device = context.open()?;

    print!("NFC reader: {} opened\n\n", device.name());

    device.initiator_init()?;

    // Configure the CRC
    device.set_property_bool(nfc::Property::HandleCrc, false)?;
    // Use raw send/receive methods
    device.set_property_bool(nfc::Property::EasyFraming, false)?;
    // Disable 14443-4 autoswitching
    device.set_property_bool(nfc::Property::AutoIso144434, false)?;

    print!("Looking for targets...\n");
    loop {
        match device.initiator_select_passive_target(&nfc::Modulation {
            modulation_type: nfc::ModulationType::Iso14443a,
            baud_rate: nfc::BaudRate::Baud106,
        }) {
            Ok(target) => {
                print!("Target found: {}", target.to_string(false)?);
                match device.initiator_transceive_bits(&UNLOCK_1, 7, MAX_FRAME_LEN) {
                    Ok(rx) => print!("Received bits: {:02X?}\n", rx),
                    Err(err) => {
                        print!(
                            "This is NOT a backdoored rewritable UID chinese card ({:?})\n",
                            err
                        );
                        sleep(Duration::from_secs(5));
                        print!("Looking for targets...\n");
                        continue;
                    }
                };

                match device.initiator_transceive_bytes(
                    &UNLOCK_2,
                    MAX_FRAME_LEN,
                    nfc::Timeout::Default,
                ) {
                    Ok(rx) => {
                        print!("Received bytes: {:02X?}\n", rx);
                        print!("This is backdoored rewritable UID chinese card\n")
                    }
                    Err(err) => print!(
                        "This is NOT a backdoored rewritable UID chinese card ({:?})\n",
                        err
                    ),
                };
            }
            Err(_) => {
                continue;
            }
        }
        sleep(Duration::from_secs(5));
        print!("Looking for targets...\n");
    }
}
