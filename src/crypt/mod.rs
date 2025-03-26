mod error;
pub mod pwd;
pub mod token;
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

	#[test]
	fn test_encrypt_into_b64u_ok() -> Result<()> {
		// -- Setup and fixture
		let mut fx_key = [0u8; 64];
		rand::rng().fill_bytes(&mut fx_key);

		let enc_content = EncryptContent {
			content: "this is content".to_string(),
			salt: "can be anything".to_string(),
		};

		// -- Exec
		let fx_res = encrypt_into_b64u(&fx_key, &enc_content)?;

		// -- Check
		let res = encrypt_into_b64u(&fx_key, &enc_content)?;

		assert_eq!(res, fx_res);

		Ok(())
	}
}

// endregion --- Tests
