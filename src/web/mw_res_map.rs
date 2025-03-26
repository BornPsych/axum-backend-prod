use std::sync::Arc;

use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use axum::Json;
use axum::body::Body;
use axum::extract::Request;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_reponse_map(
	// ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	debug!(" {:<12} - mw_reponse_map", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let web_error = res.extensions().get::<Arc<web::Error>>();
	let client_status_error = web_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				debug!(" CLIENT ERROR BODY:\n{client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// -- Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	if let Some(ctx) = res.extensions().get::<Option<Ctx>>() {
		// Handle context
		let _ =
			log_request(uuid, req_method, uri, ctx.clone(), web_error, client_error)
				.await;
	}

	error_response.unwrap_or(res)
}
