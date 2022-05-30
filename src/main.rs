use axum::{response::Html, routing::{get, post}, Router, http::StatusCode};
use std::net::SocketAddr;
use tokio::time::{Duration,sleep};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
    .route("/", get(handler))
    .route("/test",get(test))
    .route("/act_start", post(act_start));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn test() -> StatusCode {
    sleep(Duration::from_secs(2)).await;
    StatusCode::OK
}

async fn act_start() -> StatusCode {
    StatusCode::OK
}

async fn act_end() -> StatusCode {
    StatusCode::OK
}

async fn proc_start() -> StatusCode {
    StatusCode::OK
}

async fn proc_end() -> StatusCode {
    StatusCode::OK
}

async fn act_approval() -> StatusCode {
    StatusCode::OK
}

