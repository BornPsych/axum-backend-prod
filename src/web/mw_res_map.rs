use crate::log::log_request;
use crate::{Result, ctx::Ctx, web};
use axum::Json;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use uuid::Uuid;
pub async fn mw_reponse_map(
	ctx: Result<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	let uuid = Uuid::new_v4();

	// Get the eventual response error
	let service_error = res.extensions().get::<web::error::Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error":{
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log a server log line
	let client_error = client_status_error.unzip().1;
	let _ =
		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	error_response.unwrap_or(res)
}
