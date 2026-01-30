type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
	let connstring = std::env::args().nth(1).ok_or("no connection string provided")?;
	let mut context = nfc1::Context::new()?;
	let mut device = context.open_with_connstring(&connstring)?;
	println!("name: {}", device.name());
	println!("connstring: {}", device.connstring());
	println!("{}", device.get_information_about()?);
	Ok(())
}