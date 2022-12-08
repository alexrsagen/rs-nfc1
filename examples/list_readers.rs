type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxResult<T> = Result<T, BoxError>;

fn main() -> BoxResult<()> {
	let mut context = nfc1::Context::new()?;
	let devices = context.list_devices(10)?;
	for device in &devices {
		println!("{}", device)
	}
	Ok(())
}