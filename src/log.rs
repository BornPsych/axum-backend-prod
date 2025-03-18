use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Error, Result, ctx::Ctx, web::error::ClientError};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{Value, json};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Result<Ctx>,
	service_error: Option<&Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	let error_type = service_error.map(|se| se.as_ref().to_string());

	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	// Create a request log line
	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: timestamp.to_string(),
		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		user_id: ctx.ok().map(|c| c.user_id()),
		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		error_type,
		error_data,
	};
	println!("Log line {}", json!(log_line));

	// Now we can send to cloud watch or other services
	Ok(())
}

#[derive(Serialize)]
#[skip_serializing_none]
struct RequestLogLine {
	// user and contest attributes
	uuid: String,
	timestamp: String,

	// http request attributes
	req_path: String,
	req_method: String,

	user_id: Option<u64>,

	// Error attributes
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}
