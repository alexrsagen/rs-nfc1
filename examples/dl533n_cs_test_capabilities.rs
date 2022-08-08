use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
	let mut context = nfc1::Context::new()?;
	let mut device = dl533n_connect(&mut context)?;

	for n in (1..6).rev() {
		print!("\rPRESS Ctrl-C TO ABORT. Running test in {}...", n);
		stdout().flush()?;
		sleep(Duration::from_secs(1));
	}
	print!("\rRunning test...                             \n");

	println!("- Blink green LED");
	dl533n_set_green_led(&mut device, true)?;
	sleep(Duration::from_millis(1000));
	dl533n_set_green_led(&mut device, false)?;
	sleep(Duration::from_millis(50));

	println!("- Blink red LED");
	dl533n_set_red_led(&mut device, true)?;
	sleep(Duration::from_millis(1000));
	dl533n_set_red_led(&mut device, false)?;
	sleep(Duration::from_millis(50));

	println!("- Beep buzzer");
	dl533n_buzz(&mut device, Duration::from_millis(1000))?;
	sleep(Duration::from_millis(50));

	Ok(())
}

fn dl533n_set_red_led(device: &mut nfc1::Device, enable: bool) -> nfc1::Result<()> {
	if enable {
		// Set P7 bit 1 high
		device.pn53x_write_register(0xfff7, 0b00000001, 0x01)?;
	} else {
		// Set P7 bit 1 low
		device.pn53x_write_register(0xfff7, 0b00000001, 0x00)?;
	}
	Ok(())
}

fn dl533n_set_green_led(device: &mut nfc1::Device, enable: bool) -> nfc1::Result<()> {
	if enable {
		// Set P3 bit 5 / GPIO P35 high
		device.pn53x_write_register(0xffb0, 0b00100000, 0x20)?;
	} else {
		// Set P3 bit 5 / GPIO P35 low
		device.pn53x_write_register(0xffb0, 0b00100000, 0x00)?;
	}
	Ok(())
}

fn dl533n_set_buzzer(device: &mut nfc1::Device, enable: bool) -> nfc1::Result<()> {
	if enable {
		// Set P3 bit 4 / GPIO P34 high
		device.pn53x_write_register(0xffb0, 0b00010000, 0x10)?;
	} else {
		// Set P3 bit 4 / GPIO P34 low
		device.pn53x_write_register(0xffb0, 0b00010000, 0x00)?;
	}
	Ok(())
}

fn dl533n_buzz(device: &mut nfc1::Device, duration: Duration) -> nfc1::Result<()> {
	dl533n_set_buzzer(device, true)?;
	sleep(duration);
	dl533n_set_buzzer(device, false)?;
	Ok(())
}

fn dl533n_init(device: &mut nfc1::Device) -> nfc1::Result<()> {
	// Clear P3 reg
	device.pn53x_write_register(0xffb0, 0xff, 0x00)?;
	// Clear P7 reg
	device.pn53x_write_register(0xfff7, 0xff, 0x00)?;
	// bit 'SVDD' must be enabled to control P34
	device.pn53x_write_register(0x6106, 0xff, 0x1b)?;
	Ok(())
}

fn dl533n_connect<'a>(context: &'a mut nfc1::Context) -> BoxResult<nfc1::Device<'a>> {
	let mut device = context.open()?;

	print!("NFC reader: {} opened\n\n", device.name());
	print!("!!! PLEASE READ CAREFULLY !!!\n");
	print!("If the opened reader is not a DL533N CS, it may be damaged.\n");
	loop {
		print!("Are you completely sure the opened reader is a DL533N CS? [y/n]: ");
		stdout().flush()?;

		let mut input = String::new();
		stdin().read_line(&mut input)?;
		match input.to_ascii_lowercase().trim_end() {
			"n" => return Err("Aborted".into()),
			"y" => break,
			_ => continue,
		}
	}

	dl533n_init(&mut device)?;
	sleep(Duration::from_millis(50));

	Ok(device)
}