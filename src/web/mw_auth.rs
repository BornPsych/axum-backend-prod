use std::str::FromStr;

use crate::crypt;
use crate::crypt::token::{Token, validate_web_token};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserBmc, UserForAuth};
use crate::web::{AUTH_TOKEN, set_token_cookie};
use crate::web::{Error, Result};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::http::uri::Authority;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use time::format_description::parse;
use tower_cookies::{Cookie, Cookies};
use tracing::{debug, info};

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

	if ctx_ext_result.is_err()
		&& !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::from(AUTH_TOKEN));
	}

	// store the ctx_ext_result in the request extension
	// (for Ctx extractor).
	req.extensions_mut().insert(ctx_ext_result);

	Ok(next.run(req).await)
}

async fn _ctx_resolve(
	State(mm): State<ModelManager>,
	cookies: &Cookies,
) -> CtxExtResult {
	// -- Get Token String
	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(CtxExtError::TokenNotInCookie)?;
	// -- Parse Token
	let token = token
		.parse::<Token>()
		.map_err(|_| CtxExtError::TokenWrongFormat)?;
	// -- Get UserForAuth
	let user = UserBmc::first_by_username::<UserForAuth>(
		&Ctx::root_ctx(),
		&mm,
		&token.ident,
	)
	.await
	.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
	.ok_or(CtxExtError::UserNotFound)?;
	// -- Validate Token
	validate_web_token(&token, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::FailValidate)?;
	// -- Update Token
	set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
		.map_err(|_| CtxExtError::CannotSetTokenCookie)?;

	// -- Create CtxExtResult
	Ctx::new(user.id).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region:    --- Ctx Extractor
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!(" {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	TokenNotInCookie,
	CtxNotInRequestExt,
	CtxCreateFail(String),
	TokenWrongFormat,
	UserNotFound,
	ModelAccessError(String),
	FailValidate,
	CannotSetTokenCookie,
	CannotCreateFail(String),
}

// endregion: --- Ctx Extractor Result/Error
