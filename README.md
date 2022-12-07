# nfc1
[![Crates.io](https://img.shields.io/crates/v/nfc1)](https://crates.io/crates/nfc1)

High-level safe Rust bindings for libnfc.

This crate provides a safe wrapper around [`nfc1-sys`](https://github.com/alexrsagen/rs-nfc1-sys).

In contrast to [`nfc`](https://github.com/dsgriffin/nfc), this crate additionally provides:
- Extra safety
	- No exposed raw pointers
	- No `.unwrap()` where the it is not guaranteed to succeed
	- Enums for well-known constants
- `Result<T, Error>` for methods which can fail
- Everything  [`nfc1-sys`](https://github.com/alexrsagen/rs-nfc1-sys) provides, which [`nfc-sys`](https://github.com/dsgriffin/nfc-sys) does not
	- Some internal methods exposed (such as `pn53x_*`, which are useful for accessing manufacturer-specific features in NFC devices)

## Usage
Add `nfc1` as a dependency in your project's `Cargo.toml` file:
```toml
[dependencies]
nfc1 = "0.5"
```

Import the `nfc1` crate in your project, then you can use all the wrapped functions from `libnfc`.

See the [`libnfc` wiki](https://github.com/nfc-tools/libnfc/wiki) or [`libnfc` 1.8.0 examples](https://github.com/nfc-tools/libnfc/tree/libnfc-1.8.0/examples) for information on how to use `libnfc`.

### Usage example
```rust
fn main() -> nfc1::Result<()> {
	println!("libnfc v{}", nfc1::version());

	let mut context = nfc1::Context::new()?;
	let mut device = context.open()?;

	println!("NFC device {:?} opened through connection {:?}", device.name(), device.connstring());
	println!("- Initiator modulations: {:?}", device.get_supported_modulation(nfc1::Mode::Initiator)?);
	println!("- Target modulations: {:?}", device.get_supported_modulation(nfc1::Mode::Target)?);

	Ok(())
}
```
