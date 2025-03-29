use crate::crypt::{EncryptContent, pwd};
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::user::{UserBmc, UserForLogin};
use crate::web::{self, Error, Result, remove_token_cookie};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use tracing_subscriber::field::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.route("/api/logoff", post(api_logoff_handler))
		.with_state(mm)
}

async fn api_login_handler(
	mm: State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!(" {:<12} - api_login_handler", "HANDLER");

	let LoginPayload {
		username,
		pwd: pwd_clear,
	} = payload;
	let root_ctx = Ctx::root_ctx();

	// -- Get the user
	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;
	let user_id = user.id;

	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	pwd::validate_pwd(
		&EncryptContent {
			salt: user.pwd_salt.to_string(),
			content: pwd_clear.clone(),
		},
		&pwd,
	)
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// -- Set web token
	web::set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");

	let should_logoff = payload.logoff;
	if should_logoff {
		remove_token_cookie(&cookies)?;
	}

	let body = Json(json!({
		"result":{
			"logged_off":should_logoff
		}
	}));
	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
	logoff: bool,
}
