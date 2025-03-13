use axum::{response::IntoResponse, routing::get, Router};



#[tokio::main]
async fn main() {
   let routes_hello = Router::new().route("/hello", get(handler_hello));

   let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
   println!("Running on the server with port 8000");
   axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
}


async fn handler_hello () -> impl IntoResponse{
    println!("this is from Handler function");
    return "hello world".into_response();
}