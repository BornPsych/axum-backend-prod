use axum::body::Body;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{cookie, Cookies};

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
   ctx?;
    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx{
    type Rejection = Error;

    async  fn from_request_parts(parts: &mut Parts,_state: &S,) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // parse token
        let (user_id, epx, sign) = auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token)?;

    Ok(Ctx::new(user_id))
    }
}


/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Return (user_id, expiratoin, signature)
fn parse_token(token: String) -> Result<(u64, String, String)>{
    let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
		&token
	)
	.ok_or(Error::AuthFailTokenWrongFormat)?;
    
    let user_id:u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

