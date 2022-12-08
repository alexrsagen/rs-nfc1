use std::thread::sleep;
use std::time::Duration;
use std::boxed::Box;

use clap::Parser;
use hex_fmt::HexFmt;
use mrtd1::mrz::normalize_mrz_string;
use mrtd1::mrz::borrowed::Mrz;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
	#[clap(long = "reader")]
	reader: Option<String>,

	/// Machine Readable Zone (MRZ) of the Machine Readable Transport Document (MRTD)
	#[clap(long = "mrz", parse(from_str = normalize_mrz_string))]
	mrz: String,
}

fn main() -> BoxResult<()> {
	let args = Args::parse();
	let mrz = Mrz::try_from(args.mrz.as_str())?;

	let mut context = nfc1::Context::new()?;
	let mut device = if let Some(connstring) = &args.reader {
		context.open_with_connstring(connstring)?
	} else {
		context.open()?
	};
	println!("NFC reader: {} opened", device.name());
	device.initiator_init()?;
	device.set_property_bool(nfc1::Property::InfiniteSelect, true)?;

	println!("Looking for targets...");
	'findtarget: loop {
		match device.initiator_select_passive_target(&nfc1::Modulation{
			modulation_type: nfc1::ModulationType::Iso14443a,
			baud_rate: nfc1::BaudRate::Baud106,
		}) {
			Ok(target) => {
				if let nfc1::target_info::TargetInfo::Iso14443a(target_info) = target.target_info {
					println!("Found ISO/IEC 14443-A target: {}", HexFmt(&target_info.uid[..target_info.uid_len]));
					break 'findtarget;
				}
			},
			Err(_) => {}
		}
		match device.initiator_select_passive_target(&nfc1::Modulation{
			modulation_type: nfc1::ModulationType::Iso14443b,
			baud_rate: nfc1::BaudRate::Baud106,
		}) {
			Ok(target) => {
				if let nfc1::target_info::TargetInfo::Iso14443b(target_info) = target.target_info {
					println!("Found ISO/IEC 14443-B target: {}", target_info.card_identifier);
					break 'findtarget;
				}
			},
			Err(_) => {}
		}
		sleep(Duration::from_millis(300));
	}

	// handshake with eMRTD / ePassport
	let mut bac_res = mrtd1::auth::bac::handshake(&mut device, &mrz)?;

	// read files
	// NOTE: fails on first file not found
	for file in &mrtd1::files::FILES {
		let contents = mrtd1::files::read_file(&mut device, &bac_res.ks_mac, &bac_res.ks_enc, &mut bac_res.ssc, file)?;
		println!("{}: {}", file.name, HexFmt(contents));
	}

	Ok(())
}