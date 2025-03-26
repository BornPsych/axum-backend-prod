use std::str::FromStr;

use crate::config;
use crate::crypt::{Error, Result};
use crate::utils::{b64u_decode, b64u_encode};

// region:      -- Token Type

/// String format: `ident_b64u.exp_b64u.sign_b64u`
#[derive(Debug, PartialEq)]
pub struct Token {
	pub ident: String,     // Identifier (username for example)
	pub exp: String,       // Expiration date in Rfc3339
	pub sign_b64u: String, // Signature, base64url encoded.
}

impl FromStr for Token {
	type Err = Error;

	fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
		let splits: Vec<&str> = token_str.split('.').collect();
		if splits.len() != 3 {
			return Err(Error::TokenInvalidFormat);
		}
		let (indent_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);
		Ok(Self {
			ident: b64u_decode(indent_b64u)
				.map_err(|_| Error::TokenCannotDecodeIdent)?,
			exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
			sign_b64u: sign_b64u.to_string(),
		})
	}
}

impl core::fmt::Display for Token {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}.{}.{}",
			b64u_encode(&self.ident),
			b64u_encode(&self.exp),
			self.sign_b64u
		)
	}
}
// endregion    -- Token Type

// region:      -- Web Token Gen and Validation
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
	let config = &config();
	_generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
	let config = &config();
	_validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;
	Ok(())
}

// endregion   -- Web Token Gen and Validation

// region:      -- (private) Token Gen and Validation
fn _generate_token(
	ident: &str,
	duration_sec: f64,
	salt: &str,
	key: &[u8],
) -> Result<Token> {
	todo!()
}

fn _validate_token_sign_and_exp(
	origin_token: &Token,
	salt: &str,
	key: &[u8],
) -> Result<()> {
	todo!()
}

/// Create token signature from token parts and token salt
fn _token_sign_into_b64u(
	ident: &str,
	exp: &str,
	salt: &str,
	key: &[u8],
) -> Result<String> {
	todo!()
}
// endregion    -- (private) Token Gen and Validation

#[cfg(test)]
mod test {
	use super::*;
	use anyhow::{Ok, Result};

	#[test]
	fn test_token_display_ok() -> Result<()> {
		// --Fixture token
		let fx_token_str =
			"ZnhfZGVudC0wMQ.MjAyMy0wNS0xMlQxNTozMDowMHo.some-sign_b64u_encoded";
		let fx_token = Token {
			ident: "fx_dent-01".to_string(),
			exp: "2023-05-12T15:30:00z".to_string(),
			sign_b64u: "some-sign_b64u_encoded".to_string(),
		};

		// -- Exec & check
		assert_eq!(fx_token.to_string(), fx_token_str);
		Ok(())
	}

	#[test]
	fn test_token_from_str_ok() -> Result<()> {
		// --Fixture token
		let fx_token_str =
			"ZnhfZGVudC0wMQ.MjAyMy0wNS0xMlQxNTozMDowMHo.some-sign_b64u_encoded";
		let fx_token = Token {
			ident: "fx_dent-01".to_string(),
			exp: "2023-05-12T15:30:00z".to_string(),
			sign_b64u: "some-sign_b64u_encoded".to_string(),
		};

		// --Exec
		let token: Token = fx_token_str.parse()?;
		assert_eq!(token, fx_token);
		Ok(())
	}
}
