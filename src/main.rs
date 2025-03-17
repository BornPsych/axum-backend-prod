#![allow(unused)]

use axum::{
    extract::{Path, Query}, middleware, response::{IntoResponse, Response}, routing::{get, get_service}, Json, Router
};
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

pub use self::error::{Error, Result};
mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise NOdeCOntroller
    let mc = ModelController::new().await?;

    let routes_api = web::routes_ticket::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(route_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Running on the server with port 8000");
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{user_id}", get(handler_hello2))
}

async fn main_response_mapper(res: Response) -> Response {
    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error=res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());
    
    let error_response = client_status_error.as_ref().map(|(status_code, client_error)|{
        let client_error_body = json!({
            "error":{
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });

        (*status_code, Json(client_error_body)).into_response()

    });

    println!(" ->> server log line {uuid} - Error:{service_error:?}");
    
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap();
    let response = format!("this is hello sting with params {}", name).into_response();
    println!("this is from Handler function {params:?}");
    return response;
}

async fn handler_hello2(Path(user_id): Path<String>) -> impl IntoResponse {
    let response = format!("this is hello sting with path {user_id}").into_response();
    println!("this is from Handler function {user_id}");
    return response;
}
