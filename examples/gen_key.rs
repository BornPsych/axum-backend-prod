use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
	let mut key = [0u8; 64]; // 512 bits = 64 bytes

	rand::rng().fill_bytes(&mut key);
	println!("\nGenerate key for HMAC:\n{key:?}");

	let b64u = base64_url::encode(&key);
	println!("\n Key b64u encode:\n{b64u}");
	Ok(())
}
