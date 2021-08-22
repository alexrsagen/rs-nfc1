extern crate nfc1 as nfc;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn set_dl533n_red_led(device: &mut nfc::Device, enable: bool) -> nfc::Result<()> {
    if enable {
        // Set P7 bit 1 high
        device.pn53x_write_register(0xfff7, 0b00000001, 0x01)?;
    } else {
        // Set P7 bit 1 low
        device.pn53x_write_register(0xfff7, 0b00000001, 0x00)?;
    }
    Ok(())
}

fn set_dl533n_green_led(device: &mut nfc::Device, enable: bool) -> nfc::Result<()> {
    if enable {
        // Set P3 bit 5 / GPIO P35 high
        device.pn53x_write_register(0xffb0, 0b00100000, 0x20)?;
    } else {
        // Set P3 bit 5 / GPIO P35 low
        device.pn53x_write_register(0xffb0, 0b00100000, 0x00)?;
    }
    Ok(())
}

fn set_dl533n_buzzer(device: &mut nfc::Device, enable: bool) -> nfc::Result<()> {
    if enable {
        // Set P3 bit 4 / GPIO P34 high
        device.pn53x_write_register(0xffb0, 0b00010000, 0x10)?;
    } else {
        // Set P3 bit 4 / GPIO P34 low
        device.pn53x_write_register(0xffb0, 0b00010000, 0x00)?;
    }
    Ok(())
}

fn init_dl533n(device: &mut nfc::Device) -> nfc::Result<()> {
    // Clear P3 reg
    device.pn53x_write_register(0xffb0, 0xff, 0x00)?;
    // Clear P7 reg
    device.pn53x_write_register(0xfff7, 0xff, 0x00)?;
    // bit 'SVDD' must be enabled to control P34
    device.pn53x_write_register(0x6106, 0xff, 0x1b)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut context = nfc::Context::new()?;
    let mut device = context.open()?;

    print!("NFC reader: {} opened\n\n", device.name());

    print!("!!! PLEASE READ CAREFULLY !!!\n");
    print!("If the opened reader is not a DL533N CS, it may be damaged.\n");
    loop {
        print!("Are you completely sure the opened reader is a DL533N CS? [y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.to_ascii_lowercase().trim_end() {
            "n" => {
                print!("Aborted.\n");
                return Ok(());
            }
            "y" => {
                for n in (1..6).rev() {
                    print!("\rPRESS Ctrl-C TO ABORT. Running test in {}...", n);
                    io::stdout().flush()?;
                    sleep(Duration::from_secs(1));
                }
                print!("\rRunning test...                             \n");
                break;
            }
            _ => {
                continue;
            }
        }
    }

    init_dl533n(&mut device)?;
    sleep(Duration::from_millis(50));

    set_dl533n_green_led(&mut device, true)?;
    sleep(Duration::from_millis(1000));
    set_dl533n_green_led(&mut device, false)?;
    sleep(Duration::from_millis(50));

    set_dl533n_red_led(&mut device, true)?;
    sleep(Duration::from_millis(1000));
    set_dl533n_red_led(&mut device, false)?;
    sleep(Duration::from_millis(50));

    set_dl533n_buzzer(&mut device, true)?;
    sleep(Duration::from_millis(1000));
    set_dl533n_buzzer(&mut device, false)?;
    sleep(Duration::from_millis(50));

    Ok(())
}
