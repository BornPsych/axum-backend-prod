mod error;

pub use self::error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptContent {
	pub content: String, // Clear content
	pub salt: String,    // clear salt
}

pub fn encrypt_into_b64u(
	key: &[u8],
	enc_content: &EncryptContent,
) -> Result<String> {
	let EncryptContent { content, salt } = enc_content;
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	// -- Add content
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// -- Finalize and b64u encode
	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = base64_url::encode(&result_bytes);
	Ok(result)
}

// region -- Test
#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	fn test_encrypt_into_b64u_ok() -> Result<()> {
		todo!()
	}
}

// endregion --- Tests
